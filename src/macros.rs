//! This module defines various macros that enable specific code.

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
