#[path="./../lib/mod.rs"]
mod lib;

fn main(){
    let url = lib::get_first_argument();
    println!("{:?}", url);
}