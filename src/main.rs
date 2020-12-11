pub mod text_scanner;

use text_scanner::scan;

fn main() {
    let n: i32 = scan();
    println!("{}", n);
}
