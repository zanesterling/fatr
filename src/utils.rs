macro_rules! errorf {
    ($fmt:expr, $($arg:expr),*) => {
        From::from(format!(
            $fmt,
            $( $arg ),*
        ));
    }
}
