struct Foo {}

impl Foo {
    fn foo(&self) {
        bar(self);
        //~^ ERROR cannot find function `bar` in this scope

        bar(&&self, 102);
        //~^ ERROR cannot find function `bar` in this scope

        bar(&mut self, 102, &"str");
        //~^ ERROR cannot find function `bar` in this scope

        bar();
        //~^ ERROR cannot find function `bar` in this scope

        self.bar();
        //~^ ERROR no method named `bar` found for reference `&Foo` in the current scope
    }
}

fn main() {}
