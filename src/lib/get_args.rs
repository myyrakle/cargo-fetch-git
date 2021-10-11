pub fn get_args(): Vec<String> {
    std::env::args().iter().collect()
}