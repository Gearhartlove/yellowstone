use std::borrow::Borrow;

// pub fn reallocate(mut chunk: &Chunk, new_size: usize) -> Option<Chunk>{
//     if new_size == 0 {
//         drop(chunk);
//         return None;
//     }
//
//     // chunk.code.resize(new_size, 0);
//     // do I need to handle not being able to allocate memory?
//     // https://craftinginterpreters.com/chunks-of-bytecode.html : "Because Computers are finite lumps of matter ..."
//     // my attempt: check to see if the capacity is zero
//     if chunk.code.capacity() == 0 {
//         panic!("Ran out of memory while allocating a chunk");
//     }
//     // return Some(chunk);
// }