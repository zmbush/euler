use std::old_io::fs::File;
use std::old_io::fs;
use std::env;
use std::str;

fn main() {
    let mut path = Path::new(match env::var("CARGO_MANIFEST_DIR") {
        Some(val) => val.into_string().unwrap(),
        None => unreachable!()
    });

    let home = path.clone();

    path.push("src");
    path.push("main.rs");

    let mut outfile = File::create(&path).unwrap();
    let f = &mut outfile;
    let mut names = Vec::new();

    writeln!(f, "
        #[macro_use] extern crate libeuler;
        use std::os;
    ");

    for path in fs::readdir(&home).unwrap().iter() {
        if path.filename().unwrap().starts_with(b"sol_") {
            let name = str::from_utf8(path.filename().unwrap()).unwrap();
            names.push(format!("{}", name));
        }
    }

    names.sort();

    for name in names.iter() {
        writeln!(f, "mod {name} {{
            pub fn run() {{ self::main(); }}
            include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{name}/src/main.rs\"));
        }}", name = name);
    }

    writeln!(f, "
        fn main() {{
            let args = os::args();

            let project = if args.len() < 2 {{
              \"\".to_string()
            }} else {{
                args[1].clone()
            }};
    ");

    for name in names.iter() {
        writeln!(f, "
            if project.len() <= 0 || project == \"{name}\".to_string() {{
                println!(\"Running project {name}\");
                {name}::run();
            }}
        ", name = name);
    }

    writeln!(f, "
        }}
    ");
}
