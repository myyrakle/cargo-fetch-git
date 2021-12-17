pub fn get_first_argument()-> Option<String> {
    std::env::args().nth(1)
}