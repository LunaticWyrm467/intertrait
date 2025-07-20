use portable_intertrait::cast::*;
use portable_intertrait::*;

mod test_module {
    pub use portable_intertrait;
}

struct Data;

trait Source: CastFrom {}

trait Greet {
    fn greet(&self);
}

impl Greet for Data {
    fn greet(&self) {
        println!("Hello");
    }
}

impl Source for Data {}

castable_to!( crate = test_module::portable_intertrait | Data => Source, Greet );

#[test]
fn test_cast_to_on_struct() {
    let data = Data;
    let source: &dyn Source = &data;
    let greet = source.cast::<dyn Greet>();
    greet.unwrap().greet();
}
