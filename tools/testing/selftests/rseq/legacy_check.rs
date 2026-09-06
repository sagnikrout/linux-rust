//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/rseq/legacy_check.c
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

    FIXTURE(legacy)
    {
    };
    let mut cpu_id_in_sigfn: static int = -1;
#[no_mangle]
unsafe extern "C" fn sigfn(sig: c_int) {
    static void sigfn(int sig)
    {
    struct rseq_abi *rs = rseq_get_abi();
    cpu_id_in_sigfn = rs.cpu_id_start;
    }
    FIXTURE_SETUP(legacy)
    {
    let mut res: c_int = __rseq_register_current_thread(true, true);
    switch (res) {
    case -ENOSYS:
    SKIP(return, "RSEQ not enabled\n");
    case -EBUSY:
    SKIP(return, "GLIBC owns RSEQ. Disable GLIBC RSEQ registration\n");
    default:
    ASSERT_EQ(res, 0);
    }
    ASSERT_NE(signal(SIGUSR1, sigfn), SIG_ERR);
    }
    FIXTURE_TEARDOWN(legacy)
    {
    }
    TEST_F(legacy, legacy_test)
    {
    struct rseq_abi *rs = rseq_get_abi();
    ASSERT_NE(rs, core::ptr::null_mut());
// Overwrite rs::cpu_id_start
    rs.cpu_id_start = -1;
    sleep(1);
    ASSERT_NE(rs.cpu_id_start, -1);
    rs.cpu_id_start = -1;
    ASSERT_EQ(raise(SIGUSR1), 0);
    ASSERT_NE(rs.cpu_id_start, -1);
    ASSERT_NE(cpu_id_in_sigfn, -1);
    }
    TEST_HARNESS_MAIN
