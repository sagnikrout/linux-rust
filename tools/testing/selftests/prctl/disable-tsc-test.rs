//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/prctl/disable-tsc-test.c
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
//
// Tests for prctl(PR_GET_TSC, ...) / prctl(PR_SET_TSC, ...)
//
// Basic test to test behaviour of PR_GET_TSC and PR_SET_TSC
//

// Get/set the process' ability to use the timestamp counter instruction

pub const PR_GET_TSC: c_int = 25;
pub const PR_SET_TSC: c_int = 26;

    const char *tsc_names[] =
    {
    [0] = "[not set]",
    [PR_TSC_ENABLE] = "PR_TSC_ENABLE",
    [PR_TSC_SIGSEGV] = "PR_TSC_SIGSEGV",
    };
#[no_mangle]
unsafe extern "C" fn rdtsc() -> u64 {
    static uint64_t rdtsc(void)
    {
    uint32_t lo, hi;
// We cannot use "=A", since this would use %rax on x86_64
    __asm__ __volatile__ ("rdtsc" : "=a" (lo), "=d" (hi));
    return (uint64_t)hi << 32 | lo;
    }
#[no_mangle]
unsafe extern "C" fn sigsegv_cb(sig: c_int) {
    static void sigsegv_cb(int sig)
    {
    let mut tsc_val: c_int = 0;
    printf("[ SIG_SEGV ]\n");
    printf("prctl(PR_GET_TSC, &tsc_val); ");
    fflush(stdout);
    if ( prctl(PR_GET_TSC, &tsc_val) == -1)
    perror("prctl");
    printf("tsc_val == %s\n", tsc_names[tsc_val]);
    printf("prctl(PR_SET_TSC, PR_TSC_ENABLE)\n");
    fflush(stdout);
    if ( prctl(PR_SET_TSC, PR_TSC_ENABLE) == -1)
    perror("prctl");
    printf("rdtsc() == ");
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut tsc_val: c_int = 0;
    signal(SIGSEGV, sigsegv_cb);
    printf("rdtsc() == %llu\n", (unsigned long long)rdtsc());
    printf("prctl(PR_GET_TSC, &tsc_val); ");
    fflush(stdout);
    if ( prctl(PR_GET_TSC, &tsc_val) == -1)
    perror("prctl");
    printf("tsc_val == %s\n", tsc_names[tsc_val]);
    printf("rdtsc() == %llu\n", (unsigned long long)rdtsc());
    printf("prctl(PR_SET_TSC, PR_TSC_ENABLE)\n");
    fflush(stdout);
    if ( prctl(PR_SET_TSC, PR_TSC_ENABLE) == -1)
    perror("prctl");
    printf("rdtsc() == %llu\n", (unsigned long long)rdtsc());
    printf("prctl(PR_SET_TSC, PR_TSC_SIGSEGV)\n");
    fflush(stdout);
    if ( prctl(PR_SET_TSC, PR_TSC_SIGSEGV) == -1)
    perror("prctl");
    printf("rdtsc() == ");
    fflush(stdout);
    printf("%llu\n", (unsigned long long)rdtsc());
    fflush(stdout);
    exit(EXIT_SUCCESS);
    }
