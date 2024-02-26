use std::fs;

pub fn parse(args: &[String]) -> String {
    if args.len() > 2 {
        eprintln!("Too many arguments!");
        std::process::exit(1);
    } else if args.len() < 2 {
        eprintln!("Not enough arguments!");
        std::process::exit(1);
    }

    let path = args[1].clone();
    path
}

pub fn run(path: String) {
    let contents = fs::read_dir(path);

    match contents {
        Ok(contents) => {
            for element in contents {
                println!("{:?}", element);
            }
        },

        Err(_) => {
            eprintln!("No such directory!");
            std::process::exit(1);
        },
    }
}