#[allow(unused)]
mod expand {
    use {{ crate_name }}::DeriveMacroName;

    #[derive(DeriveMacroName)]
    pub struct ExampleStruct {
        name: String,
    }

    #[derive(DeriveMacroName)]
    pub enum ExampleEnum {
        Variant,
    }
}

fn main() {}
