mod app;

fn main() {
    let mut _count = 0;
    let args: Vec<String> = std::env::args().collect();

    let mut arr: Vec<&str> = vec![];

    for argument in args.iter() {
        arr.push(&*argument)
    }

    if args.len() > 2 {
        let index = arr.iter().position(|&str| str == "file").unwrap();
        if index == 1 {
            app::_cipher_file(args.last().unwrap());
        }
    } else if args.len() == 2 {
        app::_cipher_txt(args.last().unwrap());
    }
}
