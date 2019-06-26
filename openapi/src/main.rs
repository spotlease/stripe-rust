mod spec;

fn main() {
    match spec::parse() {
        Ok(spec) => println!("Success - {:#?}", spec),
        Err(error) => println!("Error - {:?}", error)
    }
}