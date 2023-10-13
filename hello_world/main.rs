fn main() {
    println!("Hello, world!");                 // println! calls a Rust macro
}


// Note1: to compile this program, we neet to run the Rust compiler:
//       'rustc main.rs'
//       This will output a binary executable 'main'
//      
// Note2: to run this compiled program we type './main'
//
// Note3: Rust is an ahead-of-time compiled language, which means that it will
//        run even on machines that don't have Rust installed.
