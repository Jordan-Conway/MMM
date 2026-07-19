pub mod data;
mod ui;

fn main() {
    println!("Hello World");
    match ui::run() {
        Ok(_) => println!("Program ran successfully"),
        Err(err) => {
            println!("Program encountered error: {:?}", err);
        }
    }
}
