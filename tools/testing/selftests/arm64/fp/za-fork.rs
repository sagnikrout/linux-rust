//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/fp/za-fork.c
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
//
// Copyright (C) 2022 ARM Limited.
// Original author: Mark Brown <broonie@kernel.org>
//
// SPDX-License-Identifier: GPL-2.0-only

pub const EXPECTED_TESTS: c_int = 1;
    int fork_test(void);
    int verify_fork(void);
//
// If we fork the value in the parent should be unchanged and the
// child should start with the same value.  This is called from the
// fork_test() asm function.
//
#[no_mangle]
pub unsafe extern "C" fn fork_test_c() -> c_int {
    int fork_test_c(void)
    {
    pid_t newpid, waiting;
    int child_status, parent_result;
    newpid = fork();
    if (newpid == 0) {
// In child
    if (!verify_fork()) {
    ksft_print_msg("ZA state invalid in child\n");
    exit(0);
    } else {
    exit(1);
    }
    }
    if (newpid < 0) {
    ksft_print_msg("fork() failed: %d\n", newpid);
    return 0;
    }
    parent_result = verify_fork();
    if (!parent_result)
    ksft_print_msg("ZA state invalid in parent\n");
    for (;;) {
    waiting = waitpid(newpid, &child_status, 0);
    if (waiting < 0) {
    if (errno == EINTR)
    continue;
    ksft_print_msg("waitpid() failed: %d\n", errno);
    return 0;
    }
    if (waiting != newpid) {
    ksft_print_msg("waitpid() returned wrong PID\n");
    return 0;
    }
    if (!WIFEXITED(child_status)) {
    ksft_print_msg("child did not exit\n");
    return 0;
    }
    return WEXITSTATUS(child_status) && parent_result;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int ret, i;
    ksft_print_header();
    ksft_set_plan(EXPECTED_TESTS);
    ksft_print_msg("PID: %d\n", getpid());
//
// This test is run with nolibc which doesn't support hwcap and
// it's probably disproportionate to implement so instead check
// for the default vector length configuration in /proc.
//
    ret = open("/proc/sys/abi/sme_default_vector_length", O_RDONLY, 0);
    if (ret >= 0) {
    ksft_test_result(fork_test(), "fork_test\n");
    } else {
    ksft_print_msg("SME not supported\n");
    for (i = 0; i < EXPECTED_TESTS; i++) {
    ksft_test_result_skip("fork_test\n");
    }
    }
    ksft_finished();
    return 0;
    }
