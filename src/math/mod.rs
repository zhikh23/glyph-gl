pub mod matrices;
pub mod vectors;

#[cfg(test)]
#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $eps:expr) => {
        assert!(
            ($left - $right).abs() < $eps,
            "assertion failed: {} != {} (eps: {})",
            $left,
            $right,
            $eps
        )
    };
}
