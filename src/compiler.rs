#![allow(warnings)]
use crate::chunk::OpCode::OP_PRINT;
use crate::chunk::{Chunk, OpCode};
use crate::debug::disassemble_chunk;
use crate::error::InterpretError;
use crate::scanner::TokenKind::*;
use crate::scanner::{Scanner, Token, TokenKind};
use crate::value::{allocate_object, Value};
use anyhow::Result;

const DEBUG_PRINT_CODE: bool = false;

/// For a given chunk, scans each token and then parses the token's scanned. The compiler evaluates
/// whether grammar rules are followed, as well as correct evaluation of precedence levels.
pub fn compile(source: &String) -> Result<Chunk> {
    let mut current_chunk = Chunk::default();
    let mut scanner = Scanner::from(source);
    let mut parser = Parser::new(&mut current_chunk);
    let mut current = Compiler::new();
    parser.advance(&mut scanner); // Q; 'primes the pump' > ? do I need
    while !parser.match_token(TOKEN_EOF, &mut scanner) {
        parser.declaration(&mut scanner, &mut current);
    }
    parser.end_compiler();
    match parser.had_error {
        true => Err(InterpretError::COMPILE_ERROR)?,
        false => Ok(current_chunk),
    }
}

#[allow(non_camel_case_types)]
#[derive(PartialOrd, PartialEq)]
enum Precedence {
    PREC_NONE,
    PREC_ASSIGNMENT, // =
    PREC_OR,         // or
    PREC_AND,        // and
    PREC_EQUALITY,   // == !=
    PREC_COMPARISON, // < > <= >=
    PREC_TERM,       // + -
    PREC_FACTOR,     // * /
    PREC_UNARY,      // ! -
    PREC_CALL,       // . ()
    PREC_PRIMARY,
}

impl Precedence {
    fn get_enum(prec: usize) -> Self {
        match prec {
            0 => Precedence::PREC_NONE,
            1 => Precedence::PREC_ASSIGNMENT,
            2 => Precedence::PREC_OR,
            3 => Precedence::PREC_AND,
            4 => Precedence::PREC_EQUALITY,
            5 => Precedence::PREC_COMPARISON,
            6 => Precedence::PREC_TERM,
            7 => Precedence::PREC_FACTOR,
            8 => Precedence::PREC_UNARY,
            9 => Precedence::PREC_CALL,
            _ => Precedence::PREC_PRIMARY,
        }
    }
}

enum ErrorAt {
    Current,
    Before,
}

struct ParseRule<'function, 'source, 'chunk> {
    prefix: Option<
        &'function dyn Fn(
            &mut Parser<'source, 'chunk>,
            &mut Scanner<'source>,
            &mut Compiler<'source>,
            bool,
        ),
    >,
    infix: Option<
        &'function dyn Fn(
            &mut Parser<'source, 'chunk>,
            &mut Scanner<'source>,
            &mut Compiler<'source>,
            bool,
        ),
    >,
    precedence: Precedence,
}

struct Local<'source> {
    pub name: &'source str,
    pub depth: usize,
    pub initialized: bool,
    pub index: Option<usize>,
}

impl<'source> Local<'source> {
    pub fn new(name: &'source str, depth: usize, i: usize) -> Self {
        Local {
            name,
            depth,
            initialized: false,
            index: Some(i),
        }
    }

    pub fn initialize(&mut self) {
        assert!(!self.initialized);

        self.initialized = true;
    }
}

struct Compiler<'source> {
    locals: Vec<Option<Local<'source>>>,
    scope_depth: usize,
}

impl<'source> Compiler<'source> {
    const MAX_LOCALS: usize = 256;

    /// Instantiate a new compiler for local variables. Push a None onto the array b/c
    /// that slot will be used to determine if their are no local variables in scope.
    fn new() -> Self {
        let v: Vec<Option<Local<'source>>> = vec![None];

        Compiler {
            locals: v,
            scope_depth: 0,
        }
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    /// Decrement the current scope by one. For every variable in the current scope, remove them from the list of locals
    /// and emit an OP_POP OpCode to ???. TODO: understand the OP_POP here.
    fn end_scope(&mut self, parser: &mut Parser) {
        self.scope_depth -= 1;
        let mut pop_count = 0;

        if self.locals.len() > 1 {
            // let mut remove_count = 0;

            for l in self.locals.iter().rev().flatten() {
                let consider_depth = l.depth;
                // Remove the local from the locals register if it's scope is greater than
                // the current scope.
                if consider_depth > self.scope_depth {
                    parser.emit_byte(OpCode::OP_POP);
                    pop_count += 1;
                } else {
                    break;
                }
            }
        }

        for _ in 0..pop_count {
            self.locals.pop();
        }
    }

    /// Add the name of a local to the local list in the Compiler. Only add to the list if their is the MAX
    /// amount of locals have not alread been defined.
    fn add_local(&mut self, name: &'source str, i: usize) {
        if self.locals.len() < Compiler::MAX_LOCALS {
            let depth = self.scope_depth;
            let local = Local::new(name, depth, i);

            self.locals.push(Some(local));
        } else {
            panic!("Too many local variables in a function.");
        }
    }

    /// Walk the list of locals that are currently in scope. If one has the same name as the
    /// identifier token, the identifier must refer to that variable.
    // fn resolve_local(&mut self, name: &'source str) -> usize {
    //     for i in (0..self.locals.len()).rev() {
    //         if let Some(l) = self.locals.get(i as usize) {
    //             if let Some(l) = l {
    //                 if l.name == name && l.initialized {//&& l.depth != 0 {
    //                     return i;
    //                 }
    //             }
    //         }
    //     }
    //     0
    // }

    fn resolve_local(&mut self, name: &'source str) -> usize {
        // check if variable is a local
        for l in self.locals.iter().rev().flatten() {
            if l.name == name {
                return l.index.unwrap();
            }
        }

        usize::MAX
    }

    /// Initialize the most recently added local variable.
    fn initialize_new_variable(&mut self) {
        let last = self.locals.last_mut().unwrap().as_mut().unwrap();
        last.initialize();
    }
}

// ########################################################################################################
// Functions to match each expression type
// ########################################################################################################

/// ParseRule for parethesis.
fn grouping<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    scanner: &mut Scanner<'source>,
    current: &mut Compiler<'source>,
    _can_assign: bool,
) {
    parser.expression(scanner, current);
    parser.consume(
        TokenKind::TOKEN_RIGHT_PAREN,
        "Expect ')' after expression.",
        scanner,
    );
}

/// Parse rule for print statement.
fn printing<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    scanner: &mut Scanner<'source>,
    current: &mut Compiler<'source>,
    _can_assign: bool,
) {
    parser.expression(scanner, current);
}

/// Parse rule for numbers.
fn number<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    _scanner: &mut Scanner<'source>,
    _current: &mut Compiler<'source>,
    _can_assign: bool,
) {
    let value = parser
        .previous
        .as_ref()
        .unwrap()
        .slice
        .parse::<f32>()
        .unwrap();
    parser.emit_constant(Value::number_value(value));
}

/// Parse rule for strings.
fn string<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    _scanner: &mut Scanner<'source>,
    _current: &mut Compiler<'source>,
    _can_assign: bool,
) {
    let slice = parser.previous.as_ref().unwrap().slice;
    let len = slice.len();
    let string = slice[1..len - 1].to_string();
    let string_obj = allocate_object(string);
    parser.emit_constant(string_obj);
}

/// Parse rule for binary operations.
fn binary<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    scanner: &mut Scanner<'source>,
    current: &mut Compiler<'source>,
    _can_assign: bool,
) {
    let operator_type = parser.previous.as_ref().unwrap().kind;
    let rule = get_rule(operator_type);
    let prec = rule.precedence as usize;
    parser.parse_precedence(Precedence::get_enum(prec), scanner, current);

    match operator_type {
        TokenKind::TOKEN_BANG_EQUAL => parser.emit_bytes(OpCode::OP_EQUAL, OpCode::OP_NOT),
        TokenKind::TOKEN_EQUAL_EQUAL => parser.emit_byte(OpCode::OP_EQUAL),
        TokenKind::TOKEN_GREATER => parser.emit_byte(OpCode::OP_GREATER),
        TokenKind::TOKEN_GREATER_EQUAL => parser.emit_bytes(OpCode::OP_LESS, OpCode::OP_NOT),
        TokenKind::TOKEN_LESS => parser.emit_byte(OpCode::OP_LESS),
        TokenKind::TOKEN_LESS_EQUAL => parser.emit_bytes(OpCode::OP_GREATER, OpCode::OP_NOT),
        TokenKind::TOKEN_PLUS => parser.emit_byte(OpCode::OP_ADD),
        TokenKind::TOKEN_MINUS => parser.emit_byte(OpCode::OP_SUBTRACT),
        TokenKind::TOKEN_STAR => parser.emit_byte(OpCode::OP_MULTIPLY),
        TokenKind::TOKEN_SLASH => parser.emit_byte(OpCode::OP_DIVIDE),
        _ => {}
    }
}

/// Parse rule for literals.
fn literal<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    _scanner: &mut Scanner<'source>,
    _current: &mut Compiler<'source>,
    _can_assign: bool,
) {
    match parser.previous.as_ref().unwrap().kind {
        TOKEN_TRUE => parser.emit_byte(OpCode::OP_TRUE),
        TOKEN_FALSE => parser.emit_byte(OpCode::OP_FALSE),
        TOKEN_NIL => parser.emit_byte(OpCode::OP_NIL),
        _ => {} // unreachable
    }
}

/// Parse rule for unary Operations.
fn unary<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    scanner: &mut Scanner<'source>,
    current: &mut Compiler<'source>,
    _can_assign: bool,
) {
    let operator_type = parser.previous.as_ref().unwrap().kind;

    // Compile the operand
    parser.parse_precedence(Precedence::PREC_UNARY, scanner, current);

    // Emit the operator instruction
    match operator_type {
        TokenKind::TOKEN_MINUS => {
            parser.emit_byte(OpCode::OP_NEGATE);
        }
        TokenKind::TOKEN_BANG => {
            parser.emit_byte(OpCode::OP_NOT);
        }
        _ => {}
    }
}

/// Parse rule for variables.
fn variable<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    scanner: &mut Scanner<'source>,
    current: &mut Compiler<'source>, // TODO: update every rule to add the compiler to it
    can_assign: bool,
) {
    let (get_op, set_op) = {
        let prev = parser.previous.as_ref().unwrap();
        let prev_word = <&str>::clone(&prev.slice);

        let idx = current.resolve_local(prev_word); // TODO: how to get name?
        if idx != usize::MAX {
            (OpCode::OP_GET_LOCAL(idx), OpCode::OP_SET_LOCAL(idx))
        } else {
            let idx = parser.identifier_constant_prev();
            (OpCode::OP_GET_GLOBAL(idx), OpCode::OP_SET_GLOBAL(idx))
        }
    };

    // TODO: do I need this?
    // let index = parser.identifier_constant_prev();

    // Set variable
    if can_assign && parser.match_token(TOKEN_EQUAL, scanner) {
        parser.expression(scanner, current);
        parser.emit_byte(set_op);
    }
    // Get variable
    else {
        parser.emit_byte(get_op);
    }
}

// Parse rule for and logical operator
fn and_<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    scanner: &mut Scanner<'source>,
    current: &mut Compiler<'source>, // TODO: update every rule to add the compiler to it
    can_assign: bool,
) {
    let end_jump = parser.emit_jump_if_false();
    parser.emit_byte(OpCode::OP_POP);
    parser.parse_precedence(Precedence::PREC_AND, scanner, current);
    parser.patch_jump(end_jump);
}

fn or_<'source, 'chunk>(
    parser: &mut Parser<'source, 'chunk>,
    scanner: &mut Scanner<'source>,
    current: &mut Compiler<'source>, // TODO: update every rule to add the compiler to it
    can_assign: bool,
) {
    let else_jump = parser.emit_jump_if_false();
    let end_jump = parser.emit_jump();
    parser.patch_jump(else_jump);
    parser.emit_byte(OpCode::OP_POP);
    parser.parse_precedence(Precedence::PREC_OR, scanner, current);
    parser.patch_jump(end_jump);
}

/// Looks at the previos and current token generates opcodes from the inputs.
struct Parser<'source, 'chunk> {
    current: Option<Token<'source>>,
    previous: Option<Token<'source>>,
    had_error: bool,
    panic_mode: bool,
    compiling_chunk: &'chunk mut Chunk,
}

impl<'source, 'chunk> Parser<'source, 'chunk> {
    fn new(compiling_chunk: &'chunk mut Chunk) -> Self {
        Parser {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
            compiling_chunk,
        }
    }

    fn advance(&mut self, scanner: &mut Scanner<'source>) {
        self.previous = self.current.take();

        loop {
            let token = scanner.scan_token();

            // TODO: fix scanner bandade
            if token.kind != TokenKind::TOKEN_NUMBER
                && token.kind != TokenKind::TOKEN_STRING
                && token.kind != TokenKind::TOKEN_IDENTIFIER
                && token.kind != TokenKind::TOKEN_NIL
                && token.kind != TokenKind::TOKEN_TRUE
                && token.kind != TokenKind::TOKEN_FALSE
                && token.kind != TokenKind::TOKEN_ASSERT_EQ
            {
                scanner.advance();
            }

            self.current = Some(token);

            if self.current.as_ref().unwrap().kind != TokenKind::TOKEN_ERROR {
                break;
            } else {
                self.error_at(ErrorAt::Current);
            }
        }
    }

    fn error_at(&mut self, error_at: ErrorAt) {
        let token = match error_at {
            ErrorAt::Current => self.current.as_ref().unwrap(),
            ErrorAt::Before => self.previous.as_ref().unwrap(),
        };

        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if token.kind == TokenKind::TOKEN_EOF {
            eprintln!(" at end");
        } else {
            eprint!(" at '{}'", token.slice);
        }

        eprintln!(": {}", token.slice);
        self.had_error = true;
    }

    /// Compare the current token's kind to the given 'kind' ; if they are the same, advance. Otherwise
    /// print and return an error.
    fn consume(&mut self, kind: TokenKind, message: &'source str, scanner: &mut Scanner<'source>) {
        let current = self.current.as_ref().unwrap();
        if current.kind == kind {
            self.advance(scanner);
        } else {
            eprintln!("{}", message);
            self.error_at(ErrorAt::Current);
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        return self.current.as_ref().unwrap().kind == kind;
    }

    fn match_token(&mut self, kind: TokenKind, scanner: &mut Scanner<'source>) -> bool {
        if !self.check(kind) {
            return false;
        }

        self.advance(scanner);
        true
    }

    fn expression(&mut self, scanner: &mut Scanner<'source>, current: &mut Compiler<'source>) {
        self.parse_precedence(Precedence::PREC_ASSIGNMENT, scanner, current);
    }

    fn block(&mut self, scanner: &mut Scanner<'source>, current: &mut Compiler<'source>) {
        loop {
            if !self.check(TOKEN_RIGHT_BRACE) && !self.check(TOKEN_EOF) {
                self.declaration(scanner, current);
            } else {
                break;
            }
        }

        self.consume(TOKEN_RIGHT_BRACE, "Expect '}' after block.", scanner);
    }

    fn print_statement(&mut self, scanner: &mut Scanner<'source>, current: &mut Compiler<'source>) {
        self.expression(scanner, current);
        self.consume(TOKEN_SEMICOLON, "Expect ';' after value.", scanner);
        self.emit_byte(OP_PRINT);
    }

    fn while_statement(&mut self, scanner: &mut Scanner<'source>, current: &mut Compiler<'source>) {
        let loop_start = self.compiling_chunk.code.len();
        self.consume(TOKEN_LEFT_PAREN, "Expect '(' after 'while'.", scanner);
        self.expression(scanner, current);
        self.consume(TOKEN_RIGHT_PAREN, "Expect ')' after 'while'.", scanner);
        let exit_jump = self.emit_jump_if_false();
        self.emit_byte(OpCode::OP_POP);
        self.statement(scanner, current);
        // loop construct
        self.emit_loop(loop_start);
        self.patch_jump(exit_jump);
        self.emit_byte(OpCode::OP_POP);
    }

    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OpCode::OP_LOOP);
        let offset = self.compiling_chunk.code.len() - loop_start + 1;
        self.emit_byte(OpCode::OP_JUMP_AMOUNT(offset));
    }

    fn assert_eq_statement(
        &mut self,
        scanner: &mut Scanner<'source>,
        current: &mut Compiler<'source>,
    ) {
        self.consume(
            TOKEN_LEFT_PAREN,
            "Expect '(' after assert statement.",
            scanner,
        );
        // Left side of the assert.
        self.expression(scanner, current);
        self.consume(TOKEN_COMMA, "Expect ',' after expression.", scanner);
        // Right side of the assert.
        self.expression(scanner, current);
        self.consume(TOKEN_RIGHT_PAREN, "Expect ')' after statement.", scanner);
        self.consume(TOKEN_SEMICOLON, "Expect ';' after statement.", scanner);
        self.emit_byte(OpCode::OP_ASSERT_EQ);
    }

    /// An expression followed by a semicolon. How you write an expression in a context where a statement is
    /// expected.
    fn expression_statement(
        &mut self,
        scanner: &mut Scanner<'source>,
        current: &mut Compiler<'source>,
    ) {
        self.expression(scanner, current);
    }

    fn for_statement(&mut self, scanner: &mut Scanner<'source>, current: &mut Compiler<'source>) {
        current.begin_scope();
        self.consume(TOKEN_LEFT_PAREN, "Expect '(' after 'for'.", scanner);
        if self.match_token(TOKEN_SEMICOLON, scanner) {
            // No initializer
        } else if self.match_token(TOKEN_VAR, scanner) {
            self.var_declaration(scanner, current)
        } else {
            self.expression_statement(scanner, current)
        }

        let mut loop_start = self.compiling_chunk.code.len();
        let mut exit_jump = 0;
        if !self.match_token(TOKEN_SEMICOLON, scanner) {
            self.expression(scanner, current);
            self.consume(TOKEN_SEMICOLON, "Expect ';' after loop condition.", scanner);

            // Jump out of the loop if the condition is false.
            exit_jump = self.emit_jump_if_false();
            self.emit_byte(OpCode::OP_POP);
        }

        if !self.match_token(TOKEN_RIGHT_PAREN, scanner) {
            let body_jump = self.emit_jump();
            let increment_start = self.compiling_chunk.code.len();
            self.expression(scanner, current);
            self.emit_byte(OpCode::OP_POP);
            self.consume(TOKEN_RIGHT_PAREN, "Expect ')' after for clauses.", scanner);

            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);
        }

        self.statement(scanner, current);
        self.emit_loop(loop_start);

        if exit_jump == 0 {
            // fix this zero and negative one problem
            self.patch_jump(exit_jump);
            self.emit_byte(OpCode::OP_POP); // Condition
        }

        current.end_scope(self);
    }

    fn if_statement(&mut self, scanner: &mut Scanner<'source>, current: &mut Compiler<'source>) {
        self.consume(TOKEN_LEFT_PAREN, "Expect '(' after 'if'.", scanner);
        self.expression(scanner, current);
        self.consume(TOKEN_RIGHT_PAREN, "Expect ')' after condition", scanner);

        let then_jump: usize = self.emit_jump_if_false();
        self.emit_byte(OpCode::OP_POP);

        self.statement(scanner, current);

        let else_jump: usize = self.emit_jump();
        self.emit_byte(OpCode::OP_POP);

        self.patch_jump(then_jump);

        if self.match_token(TOKEN_ELSE, scanner) {
            self.statement(scanner, current)
        }

        self.patch_jump(else_jump);
    }

    fn declaration(&mut self, scanner: &mut Scanner<'source>, current: &mut Compiler<'source>) {
        if self.match_token(TOKEN_VAR, scanner) {
            self.var_declaration(scanner, current);
        } else {
            self.statement(scanner, current);
        }

        // TODO: test synchoronize
        if self.panic_mode {
            self.synchronize(scanner);
        }
    }

    fn var_declaration(&mut self, scanner: &mut Scanner<'source>, current: &mut Compiler<'source>) {
        let chunk_val_index: usize = self.parse_variable("Expect variable name.", scanner, current);

        if self.match_token(TOKEN_EQUAL, scanner) {
            self.expression(scanner, current);
        } else {
            self.emit_byte(OpCode::OP_NIL)
        }
        self.consume(
            TOKEN_SEMICOLON,
            "Expect ';' after variable declaration.",
            scanner,
        );

        self.define_variable(chunk_val_index, current);
    }

    /// Continue to advance the scanner until a strong token is recognized.
    fn synchronize(&mut self, scanner: &mut Scanner<'source>) {
        self.panic_mode = false;

        loop {
            if let Some(current) = &self.current {
                match current.kind {
                    TOKEN_EOF => {
                        break;
                    }
                    TOKEN_CLASS => {
                        return;
                    }
                    TOKEN_FUN => {
                        return;
                    }
                    TOKEN_VAR => {
                        return;
                    }
                    TOKEN_FOR => {
                        return;
                    }
                    TOKEN_IF => {
                        return;
                    }
                    TOKEN_WHILE => {
                        return;
                    }
                    TOKEN_PRINT => {
                        return;
                    }
                    TOKEN_RETURN => {
                        return;
                    }
                    TOKEN_ASSERT_EQ => {
                        return;
                    }
                    _ => {} // do nothing
                }
                self.advance(scanner);
            }
        }
    }

    fn statement(&mut self, scanner: &mut Scanner<'source>, current: &mut Compiler<'source>) {
        if let Some(t) = &self.current {
            match t.kind {
                TOKEN_PRINT => {
                    self.print_statement(scanner, current);
                }
                TOKEN_IF => {
                    self.advance(scanner);
                    self.if_statement(scanner, current);
                }
                TOKEN_ASSERT_EQ => {
                    self.advance(scanner);
                    self.assert_eq_statement(scanner, current);
                }
                TOKEN_LEFT_BRACE => {
                    self.advance(scanner);
                    current.begin_scope();
                    self.block(scanner, current);
                    current.end_scope(self);
                }
                TOKEN_WHILE => {
                    self.advance(scanner);
                    self.while_statement(scanner, current);
                }
                TOKEN_FOR => {
                    self.advance(scanner);
                    self.for_statement(scanner, current);
                }
                _ => self.expression_statement(scanner, current),
            }
        }
    }

    fn parse_precedence(
        &mut self,
        precedence: Precedence,
        scanner: &mut Scanner<'source>,
        current: &mut Compiler<'source>,
    ) {
        self.advance(scanner);
        let prev_kind = self.previous.as_ref().unwrap().kind;

        let prefix_rule = get_rule(prev_kind).prefix;
        if let Some(rule) = prefix_rule {
            let can_assign: bool = precedence <= Precedence::PREC_ASSIGNMENT;
            rule(self, scanner, current, can_assign);

            while precedence <= get_rule(self.current.as_ref().unwrap().kind).precedence {
                self.advance(scanner);

                let prev = self.previous.as_ref().unwrap().kind;
                let infix_rule = get_rule(prev).infix;
                if let Some(rule) = infix_rule {
                    rule(self, scanner, current, can_assign);
                }

                if can_assign && self.match_token(TOKEN_EQUAL, scanner) {
                    eprint!("Invalid assignment target.");
                }
            }
        } else {
            //eprint!("Expect expression.");
        }
    }

    /// Outputs the bytecode instruction that defines the new variable and stores its initial value.
    fn define_variable(&mut self, index: usize, current: &mut Compiler) {
        if current.scope_depth > 0 {
            current.initialize_new_variable();
            return;
        }

        self.emit_byte(OpCode::OP_DEFINE_GLOBAL(index));
    }

    fn parse_variable(
        &mut self,
        error_message: &'static str,
        scanner: &mut Scanner<'source>,
        current: &mut Compiler<'source>,
    ) -> usize {
        self.consume(TOKEN_IDENTIFIER, error_message, scanner);

        self.declare_variable(current);
        if current.scope_depth > 0 {
            return 0;
        }

        self.identifier_constant_prev()
    }

    fn identifier_constant_prev(&mut self) -> usize {
        let token = self.previous.as_ref().unwrap();
        let value = allocate_object(token.slice.to_string());

        self.compiling_chunk.add_constant(value)
    }

    pub fn declare_variable(&self, current: &mut Compiler<'source>) {
        // look for global variables, instead of local variables
        if current.scope_depth == 0 {
            return;
        }

        let prev = self.previous.as_ref().unwrap();
        let prev_name = <&str>::clone(&prev.slice);

        let mut to_remove: Option<usize> = None;

        // Iterate through the list of locals *in the current depth* , if their is a variable that
        // matches the name of the most recent variable, remove it (variable shadowing effect).
        // Else add it to as a new local variable at the end.
        for (i, local) in current.locals.iter().rev().enumerate() {
            if let Some(l) = local {
                // If not in the current depth, stop comparing variables.
                if l.depth != 0 && l.depth < current.scope_depth {
                    break;
                }

                // Remove the local that is overshaddowed (it will never be refenced again),
                // and add the new one at the end of the scope.
                if prev_name == l.name {
                    to_remove = Some(current.locals.len() - i - 1);
                    break;
                }
            }
        }

        // Revome the variable if it is no longer needed
        if let Some(i) = to_remove {
            current.locals.remove(i);
        }

        // Add local variable to the list of local variables.
        current.add_local(prev_name, self.compiling_chunk.constants.len());
    }

    fn end_compiler(&mut self) {
        if DEBUG_PRINT_CODE && self.had_error {
            disassemble_chunk(self.compiling_chunk, "code");
        }
        self.emit_return() // todo: should this be commented?
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OP_RETURN);
    }

    fn emit_constant(&mut self, value: Value) {
        // check to make sure I don't have the max
        // constants in a chunk
        let constant_value = value.clone();
        let size = self.compiling_chunk.add_constant(constant_value);
        if size > 256 {
            eprint!("Too many constants in one chunk.");
        }
        self.emit_byte(OpCode::OP_CONSTANT(value))
    }

    /// Goes back into the bytecode and replaces the operand at the given
    /// location with the calculated jump offset.
    fn patch_jump(&mut self, offset: usize) {
        let jump = if offset == 0 {
            self.compiling_chunk.code.len() - 2
        } else {
            self.compiling_chunk.code.len() - offset - 1
        };

        // Replace placeholder jump with OP_JUMP
        // where to remove?
        let placeholder = self.compiling_chunk.code.remove(offset);
        assert_eq!(OpCode::OP_PLACEHOLDER_JUMP_AMOUNT, placeholder);
        self.compiling_chunk
            .code
            .insert(offset, OpCode::OP_JUMP_AMOUNT(jump));
    }

    fn emit_byte(&mut self, opcode: OpCode) {
        let line = self.previous.as_ref().unwrap().line as usize;
        self.compiling_chunk.write_chunk(opcode, line);
    }

    fn emit_bytes(&mut self, opcode1: OpCode, opcode2: OpCode) {
        self.emit_byte(opcode1);
        self.emit_byte(opcode2);
    }

    fn emit_jump_if_false(&mut self) -> usize {
        self.emit_byte(OpCode::OP_JUMP_IF_FALSE);
        self.emit_byte(OpCode::OP_PLACEHOLDER_JUMP_AMOUNT);
        self.compiling_chunk.code.len() - 1
    }

    fn emit_jump(&mut self) -> usize {
        self.emit_byte(OpCode::OP_JUMP);
        self.emit_byte(OpCode::OP_PLACEHOLDER_JUMP_AMOUNT);
        self.compiling_chunk.code.len() - 1
    }
}

// NOTE: not calling the function here, instead
// returning the reference to the function to be called
// in some other scope
fn get_rule<'function, 'source, 'chunk>(kind: TokenKind) -> ParseRule<'function, 'source, 'chunk> {
    match kind {
        TOKEN_LEFT_PAREN => ParseRule {
            prefix: Some(&grouping),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_RIGHT_PAREN => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_LEFT_BRACE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_RIGHT_BRACE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_COMMA => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_DOT => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_MINUS => ParseRule {
            prefix: Some(&unary),
            infix: Some(&binary),
            precedence: Precedence::PREC_TERM,
        },
        TOKEN_PLUS => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_TERM,
        },
        TOKEN_SEMICOLON => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_SLASH => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_FACTOR,
        },
        TOKEN_STAR => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_FACTOR,
        },
        TOKEN_BANG => ParseRule {
            prefix: Some(&unary),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_BANG_EQUAL => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_EQUALITY,
        },
        TOKEN_EQUAL => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_EQUAL_EQUAL => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_EQUALITY,
        },
        TOKEN_GREATER => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_COMPARISON,
        },
        TOKEN_GREATER_EQUAL => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_COMPARISON,
        },
        TOKEN_LESS => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_COMPARISON,
        },
        TOKEN_LESS_EQUAL => ParseRule {
            prefix: None,
            infix: Some(&binary),
            precedence: Precedence::PREC_COMPARISON,
        },
        TOKEN_IDENTIFIER => ParseRule {
            prefix: Some(&variable),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_STRING => ParseRule {
            prefix: Some(&string),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_NUMBER => ParseRule {
            prefix: Some(&number),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_AND => ParseRule {
            prefix: None,
            infix: Some(&and_),
            precedence: Precedence::PREC_AND,
        },
        TOKEN_CLASS => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_ELSE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_FALSE => ParseRule {
            prefix: Some(&literal),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_FOR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_FUN => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_IF => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_NIL => ParseRule {
            prefix: Some(&literal),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_OR => ParseRule {
            prefix: None,
            infix: Some(&or_),
            precedence: Precedence::PREC_OR,
        },
        TOKEN_PRINT => ParseRule {
            prefix: Some(&printing),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_RETURN => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_SUPER => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_THIS => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_TRUE => ParseRule {
            prefix: Some(&literal),
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        // statement not an expression
        TOKEN_VAR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_WHILE => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_ERROR => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        TOKEN_EOF => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
        // statement not an expression
        TOKEN_ASSERT_EQ => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PREC_NONE,
        },
    }
}
