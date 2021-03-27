fn foo() {}

async fn bar() -> std::io::Result<()> {
    foo().await; //~ ERROR `()` is not a future
    std::io::Result::Ok(())
}

fn main() {}
