//! Automatically rewritten from C to Rust
//! Source: lib/math/tests/polynomial_kunit.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only

#[repr(C)]
#[derive(Copy, Clone)]
pub struct polynomial_test_param {
    pub poly: *const polynomial,
    pub data: c_long,
    pub expected: c_long,
    pub name: *const c_char,
}

// f(x) = 5
    static const struct polynomial poly_constant = {
    .total_divider = 1,
    .terms = {
    {0, 5, 1, 1},
    }
    };
// f(x) = 2x^2 + 3x + 5
    static const struct polynomial poly_simple = {
    .total_divider = 1,
    .terms = {
    {2, 2, 1, 1},
    {1, 3, 1, 1},
    {0, 5, 1, 1},
    }
    };
// f(x) = -5x + 100
    static const struct polynomial poly_negative_coef = {
    .total_divider = 1,
    .terms = {
    {1, -5, 1, 1},
    {0, 100, 1, 1},
    }
    };
// f(x) = (150x + 50) / 10
    static const struct polynomial poly_total_divider = {
    .total_divider = 10,
    .terms = {
    {1, 150, 1, 1},
    {0,  50, 1, 1},
    }
    };
//
// f(x) = x / 2
// divider=2 applied once per multiply: mult_frac(coef, data, 2) = coef*data/2
//
    static const struct polynomial poly_step_divider = {
    .total_divider = 1,
    .terms = {
    {1, 1, 2, 1},
    {0, 0, 1, 1},
    }
    };
//
// f(x) = (100/500) * x^2 = 0.2 * x^2
// Encoded as coef=100, divider=10, divider_leftover=5:
// denom = 10^2 * 5 = 500
//
    static const struct polynomial poly_leftover = {
    .total_divider = 1,
    .terms = {
    {2, 100, 10, 5},
    {0,   0,  1, 1},
    }
    };
//
// f(x) = 2x^3  (single high-degree term, no constant)
// Used to exercise the power loop alone.
//
    static const struct polynomial poly_cubic = {
    .total_divider = 1,
    .terms = {
    {3, 2, 1, 1},
    {0, 0, 1, 1},
    }
    };
//
// f(x) = 4x + 1  with a zero-coefficient quadratic term.
// The deg-2 term contributes nothing regardless of input.
//
    static const struct polynomial poly_zero_coef = {
    .total_divider = 1,
    .terms = {
    {2, 0, 1, 1},
    {1, 4, 1, 1},
    {0, 1, 1, 1},
    }
    };
//
// f(x) = 9  with total_divider = 0.
// The implementation treats 0 as 1 via `total_divider ?: 1`, so the
// result must equal the constant term unchanged.
//
    static const struct polynomial poly_zero_total_divider = {
    .total_divider = 0,
    .terms = {
    {0, 9, 1, 1},
    }
    };
    static const struct polynomial_test_param test_params[] = {
    {
    .poly     = &poly_constant,
    .data     = 0,
    .expected = 5,
    .name     = "Constant polynomial at x=0",
    },
    {
    .poly     = &poly_constant,
    .data     = 42,
    .expected = 5,
    .name     = "Constant polynomial is independent of input",
    },
    {
    .poly     = &poly_simple,
    .data     = 0,
    .expected = 5,	/* zero input collapses all power terms */
    .name     = "Zero input yields constant term only",
    },
    {
    .poly     = &poly_simple,
    .data     = 10,
    .expected = 235,	/* 2*100 + 3*10 + 5 */
    .name     = "Simple quadratic at x=10",
    },
    {
    .poly     = &poly_negative_coef,
    .data     = 10,
    .expected = 50,		/* -5*10 + 100 */
    .name     = "Negative coefficient at x=10",
    },
    {
    .poly     = &poly_negative_coef,
    .data     = 20,
    .expected = 0,		/* -5*20 + 100 = 0 */
    .name     = "Negative coefficient result is zero",
    },
    {
    .poly     = &poly_total_divider,
    .data     = 3,
    .expected = 50,		/* (150*3 + 50) / 10 = 500/10 */
    .name     = "total_divider scales the final sum",
    },
    {
    .poly     = &poly_step_divider,
    .data     = 100,
    .expected = 50,		/* 1*100/2 */
    .name     = "Per-step divider halves input",
    },
    {
    .poly     = &poly_leftover,
    .data     = 30,
    .expected = 180,	/* 100*30^2 / (10^2 * 5) = 90000/500 */
    .name     = "divider_leftover with quadratic term",
    },
// Boundary: unit and negative-unit input
    {
//
// data=1: each mult_frac(tmp, 1, divider) strips one factor of
// divider from coef per degree, so coef is left-shifted right
// until intermediate precision is exhausted.
// 2*1 + 3*1 + 5 = 10
//
    .poly     = &poly_simple,
    .data     = 1,
    .expected = 10,
    .name     = "Boundary: data=1 (unit input)",
    },
    {
//
// data=-1: even degrees produce positive contributions,
// odd degrees produce negative ones.
// 2*(-1)^2 + 3*(-1) + 5 = 2 - 3 + 5 = 4
//
    .poly     = &poly_simple,
    .data     = -1,
    .expected = 4,
    .name     = "Boundary: data=-1 (negative unit input)",
    },
// Boundary: negative non-trivial input
    {
//
// 2*(-3)^2 + 3*(-3) + 5 = 18 - 9 + 5 = 14
// Verifies sign handling for negative data across all degrees.
//
    .poly     = &poly_simple,
    .data     = -3,
    .expected = 14,
    .name     = "Boundary: negative data with quadratic",
    },
// Boundary: total_divider = 0 is treated as 1
    {
    .poly     = &poly_zero_total_divider,
    .data     = 42,
    .expected = 9,
    .name     = "Boundary: total_divider=0 defaults to 1",
    },
// Boundary: zero-coefficient high-degree term
    {
//
// The deg-2 term has coef=0, so it contributes 0 regardless
// of data. Result: 0 + 4*10 + 1 = 41
//
    .poly     = &poly_zero_coef,
    .data     = 10,
    .expected = 41,
    .name     = "Boundary: zero-coefficient term is inert",
    },
// Boundary: single high-degree term, no constant
    {
// 2 * 5^3 = 250; also verifies the loop terminates on deg-0
    .poly     = &poly_cubic,
    .data     = 5,
    .expected = 250,
    .name     = "Boundary: single cubic term",
    },
    {
// 2 * (-2)^3 = -16; odd power preserves sign of negative data
    .poly     = &poly_cubic,
    .data     = -2,
    .expected = -16,
    .name     = "Boundary: single cubic term, negative data",
    },
    };
#[no_mangle]
unsafe extern "C" fn get_desc(param: *const polynomial_test_param, desc: *mut c_char) {
    static void get_desc(const struct polynomial_test_param *param, char *desc)
    {
    strscpy(desc, param.name, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(polynomial, test_params, get_desc);
#[no_mangle]
unsafe extern "C" fn polynomial_calc_test(test: *mut kunit) {
    static void polynomial_calc_test(struct kunit *test)
    {
    const struct polynomial_test_param *param = test.param_value;
    KUNIT_EXPECT_EQ(test, polynomial_calc(param.poly, param.data),
    param.expected);
    }
    static struct kunit_case polynomial_test_cases[] = {
    KUNIT_CASE_PARAM(polynomial_calc_test, polynomial_gen_params),
    {}
    };
    static struct kunit_suite polynomial_test_suite = {
    .name = "math-polynomial",
    .test_cases = polynomial_test_cases,
    };
    kunit_test_suites(&polynomial_test_suite);
    MODULE_DESCRIPTION("math.polynomial_calc KUnit test suite");
    MODULE_LICENSE("GPL");
