mod app;

fn main() {
    let input = std::env::args().last().unwrap();

    println!("{:?}", input);


    app::_cipher(&*input);
}