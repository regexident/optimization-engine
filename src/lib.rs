//! **Optimization Engine** is a framework for **fast** and **accurate** embedded nonconvex optimization.
//!
//! # About Optimization Engine
//!
//!
//! Its core functionality (including all numerical routines) is written in [Rust](https://www.rust-lang.org/).
//!
//! **Optimization Engine** can be used on PCs (all OSs are supported) and on embedded devices
//! (e.g., Raspberry Pi, Atom, Odroid, etc).
//!
//! Note that this is the **API documentation** of **Optimization Engine**; to get started,
//! you would rather check out the [documentation]().
//!
//!
//!
//!

extern crate num;

pub mod constraints;
pub mod core;
pub mod lipschitz_estimator;
pub mod matrix_operations;

/* ---------------------------------------------------------------------------- */
/*          TESTS                                                               */
/* ---------------------------------------------------------------------------- */
#[cfg(test)]
mod mocks;

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use core::Optimizer;

    use std::num::NonZeroUsize;

    #[test]
    fn t_access() {
        let radius = 0.2;
        let box_constraints = constraints::Ball2::new_at_origin_with_radius(radius);
        let problem = core::Problem::new(
            box_constraints,
            super::mocks::my_gradient,
            super::mocks::my_cost,
        );
        let gamma = 0.1;
        let tolerance = 1e-6;

        let mut fbs_cache =
            core::fbs::FBSCache::new(NonZeroUsize::new(2).unwrap(), gamma, tolerance);
        let mut fbs_engine = core::fbs::FBSEngine::new(problem, &mut fbs_cache);
        let mut u = [0.0; 2];
        let mut optimizer = core::fbs::FBSOptimizer::new(&mut fbs_engine);

        let status = optimizer.solve(&mut u);

        assert!(status.has_converged());
        assert!(status.get_norm_fpr() < tolerance);
        assert!((-0.14896 - u[0]).abs() < 1e-4);
        assert!((0.13346 - u[1]).abs() < 1e-4);
    }
}
