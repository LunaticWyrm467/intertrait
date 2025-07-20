use portable_intertrait::*;

struct Data;

#[cast_to]
impl Data {
    fn hello() {
        println!("hello!");
    }
}

fn main() {
    let _ = Data;
}
