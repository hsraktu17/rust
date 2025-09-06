use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    
    let result = parse_name(&args);
    
    match result {
        Ok(name) => println!("Hello, {}!", name),
        Err(err) => eprintln!("error: {}", err),
    }
}

fn parse_name(args: &[String]) -> Result<String, &'static str> {
    let mut iter = args.iter();

    // Skip program name
    iter.next();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--name" => {
                if let Some(value) = iter.next() {
                    return Ok(value.clone());
                } else {
                    return Err("No value provided after --name");
                }
            }
            _ => continue,
        }
    }

    Err("Expected --name argument")
}
