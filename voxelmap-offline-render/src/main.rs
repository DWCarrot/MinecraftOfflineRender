mod loader;
mod framework;



fn main() {
    println!("Hello, world!");

    use std::io::Read;
    use std::fs::File;
    use std::convert::TryFrom;
    let mut ifile = File::open("F:\\data\\RAW\\tmp\\0,0\\key").unwrap();
    let mut buf = String::new();
    ifile.read_to_string(&mut buf).unwrap();
    for line in buf.lines() {
        let loader::KeyLine { id, name, state } = loader::KeyLine::try_from(line).unwrap();
        let state: Vec<String> = loader::SplitIter::from(state).map(|s| s.to_string()).collect();
        println!("[{}] {} {:?}", id, name, state);
    }

}

