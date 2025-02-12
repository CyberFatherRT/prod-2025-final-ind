#[macro_export]
macro_rules! map_vec {
    ($vec:ident, $field:ident) => {
        $vec.iter().map(|c| c.$field.clone()).collect::<Vec<_>>()
    };
}
