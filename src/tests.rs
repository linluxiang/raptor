use raptor_core::abstracts::*;

// ![IGrain]
pub trait Echo: IGrain {
    // ![grain]
    fn echo(&self) -> str;
}

async fn test_call_a_grain() {
    let context = Context {};
    let instance = context.create_proxy(String::from("1"));
    instance.echo().await?
}
