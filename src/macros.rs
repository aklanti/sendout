//! Internal macros for conditional compilation
// These macros are inspired by [tokio::macros::cfg](https://github.com/tokio-rs/tokio/blob/master/tokio/src/macros/cfg.rs)

/// Compiles the block only when the `test-util` feature is enabled
macro_rules! cfg_test_util {
    ($($item: item)*)=> {
        $(
            #[cfg(feature = "test-util")]
            $item
        )*
    }
}

/// Compiles the block only when running tests that require the `test-util` feature
macro_rules! cfg_test {
    ($($item: item)*)=> {
        $(
            #[cfg(test)]
            #[cfg(feature = "test-util")]
            #[cfg_attr(docsrs, doc(cfg(feature = "test-util")))]
            $item
        )*
    }
}
