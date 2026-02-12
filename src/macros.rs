//! This module defines various macros that enable specific code.
// Theses macros are inspired by [tokio::macros::cfg](https://github.com/tokio-rs/tokio/blob/master/tokio/src/macros/cfg.rs)

/// Enables test specific code.
macro_rules! cfg_test_util {
    ($($item: item)*)=> {
        $(
            #[cfg(feature = "test-util")]
            #[cfg_attr(docsrs, doc(cfg(feature = "test-util")))]
            $item
        )*
    }
}

/// Enables test specific code.
macro_rules! cfg_test {
    ($($item: item)*)=> {
        $(
            #[cfg(test)]
            #[cfg(feature = "test-util")]
            $item
        )*
    }
}
