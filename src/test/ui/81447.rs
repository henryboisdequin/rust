mod some_module {
    pub struct Test<T: ?Sized>(T);
}

use some_module::Test;

struct TestBuilder;

impl TestBuilder {
    fn build(self) -> Test {
        Test(self) 
        // ^~ERROR cannot initialize a tuple struct which contains private fields
    }
}

fn main() {}
