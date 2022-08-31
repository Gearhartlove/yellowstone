# yellowstone
Homemade Virtual Machine 

Grammer: 

```ebnf
expression    ->  literal
              |   unary
              |   binary
              |   grouping  ;
              
literal       ->  NUMBER | STRING | "true" | false | "nil"  ;
grouping      ->  "(" expression ")"  ;
unary         ->  ( "-" | "!" ) expression  ;
binary        ->  expression operator expression  ;
operator      ->  "==" | ">=" | "<=" | "<" | ">" | "=" | "-"  | "+"  | "*"  | "/"  ;


```
