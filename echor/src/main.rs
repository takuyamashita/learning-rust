use clap::App;

fn main() {
    let _matches = App::new("MyApp")
        .version("1.0")
        .author("Me")
        .about("Does awesome things")
        .get_matches();
}
