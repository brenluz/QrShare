use std::env;
use std::path::PathBuf;

struct CLi {
    path: PathBuf,
}

fn main() {
    let path = env::args().nth(1).expect("Please provide a file path as the first argument");
    
    let args = CLi {
        path: PathBuf::from(path),
    };

    println!("Provided file path: {:?}", args.path);
}
