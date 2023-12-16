pub fn str_to_u32_or_panic (str: &str) -> u32 {
    str.parse::<u32>().unwrap_or_else(|x| panic!("{}", x))
}
