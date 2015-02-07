use std::old_io::fs::File;
use std::old_io::fs;
use std::env;
use std::str;

fn main() {
    let mut path = Path::new(match env::var("CARGO_MANIFEST_DIR") {
        Some(val) => val.into_string().unwrap(),
        None => unreachable!()
    });

    let home = Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().into_string().unwrap());
    let path = Path::new(env::var("OUT_DIR").unwrap().into_string().unwrap()).join("run.rs");

    let mut outfile = File::create(&path).unwrap();
    let f = &mut outfile;
    let mut names = Vec::new();

    writeln!(f, "
        use std::env::Args;
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
        fn run(mut args: Args) {{
            let project = match args.next() {{
              Some(p) => p.into_string().unwrap(),
              None => \"\".to_string()
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
