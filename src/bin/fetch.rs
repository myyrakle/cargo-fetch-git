#[path="./../lib/mod.rs"]
mod lib;

fn main(){
    let args = lib::get_args();
    println!("{:?}", args);
}