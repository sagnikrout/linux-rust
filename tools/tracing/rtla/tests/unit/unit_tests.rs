//! Automatically rewritten from C to Rust
//! Source: tools/tracing/rtla/tests/unit/unit_tests.c
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


// SPDX-License-Identifier: GPL-2.0
// Macro flag: #define _GNU_SOURCE

    Suite *utils_suite(void);
    Suite *actions_suite(void);
    Suite *osnoise_top_cli_suite(void);
    Suite *osnoise_hist_cli_suite(void);
    Suite *timerlat_top_cli_suite(void);
    Suite *timerlat_hist_cli_suite(void);
    Suite *cli_opt_callback_suite(void);
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int num_failed;
    SRunner *sr;
    in_unit_test = true;
    sr = srunner_create(utils_suite());
    srunner_add_suite(sr, cli_opt_callback_suite());
    srunner_add_suite(sr, actions_suite());
    srunner_add_suite(sr, osnoise_top_cli_suite());
    srunner_add_suite(sr, osnoise_hist_cli_suite());
    srunner_add_suite(sr, timerlat_top_cli_suite());
    srunner_add_suite(sr, timerlat_hist_cli_suite());
    srunner_run_all(sr, CK_VERBOSE);
    num_failed = srunner_ntests_failed(sr);
    srunner_free(sr);
    return (num_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
    }
