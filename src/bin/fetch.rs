#[path="./../lib/mod.rs"]
mod lib;

fn main(){
    const args = lib::get_args();
    println!("{}", args);
}