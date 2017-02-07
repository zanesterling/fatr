macro_rules! errorf {
    ($fmt:expr, $($arg:expr),*) => {
        From::from(format!(
            $fmt,
            $( $arg ),*
        ));
    }
}

macro_rules! expect_args {
    ( $args:expr, $num_args:expr ) => {
        if $args.len() < $num_args {
            return Err(errorf!(
                "expected {} args, got {}",
                $num_args, $args.len()
            ))
        }
    }
}
