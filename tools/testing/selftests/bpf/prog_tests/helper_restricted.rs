//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/helper_restricted.c
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

#[no_mangle]
pub unsafe extern "C" fn test_helper_restricted() {
    void test_helper_restricted(void)
    {
    let mut prog_i: c_int = 0, prog_cnt;
    do {
    struct test_helper_restricted *test;
    int err;
    test = test_helper_restricted__open();
    if (!ASSERT_OK_PTR(test, "open"))
    return;
    prog_cnt = test.skeleton.prog_cnt;
    for (int j = 0; j < prog_cnt; ++j) {
    struct bpf_program *prog = *test.skeleton.progs[j].prog;
    bpf_program__set_autoload(prog, true);
    }
    err = test_helper_restricted__load(test);
    ASSERT_ERR(err, "load_should_fail");
    test_helper_restricted__destroy(test);
    } while (++prog_i < prog_cnt);
    }
