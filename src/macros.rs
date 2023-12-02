#[macro_export]
macro_rules! inputfile {
    ($n:expr) => {
        include_str!(concat!("inputs/", $n))
    };
}
