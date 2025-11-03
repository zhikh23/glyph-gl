pub mod matrices;
pub mod transformations;
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

#[cfg(test)]
#[macro_export]
macro_rules! assert_matrix4_approx_eq {
    ($left:expr, $right:expr, $eps:expr) => {
        for i in 0..4 {
            for j in 0..4 {
                assert!(
                    ($left[i][j] - $right[i][j]).abs() < $eps,
                    "assertion failed: lhs[{i}][{j}] != rhs[{j}][{i}] <=> {} != {} (eps={})",
                    $left[i][j],
                    $right[i][j],
                    $eps,
                );
            }
        }
    };
}
