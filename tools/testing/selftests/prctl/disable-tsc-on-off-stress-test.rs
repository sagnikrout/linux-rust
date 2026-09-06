//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/prctl/disable-tsc-on-off-stress-test.c
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
// Tests if the control register is updated correctly
// when set with prctl()
//
// Warning: this test will cause a very high load for a few seconds
//

// Get/set the process' ability to use the timestamp counter instruction

pub const PR_GET_TSC: c_int = 25;
pub const PR_SET_TSC: c_int = 26;

// snippet from wikipedia :-)
#[no_mangle]
unsafe extern "C" fn rdtsc() -> u64 {
    static uint64_t rdtsc(void)
    {
    uint32_t lo, hi;
// We cannot use "=A", since this would use %rax on x86_64
    __asm__ __volatile__ ("rdtsc" : "=a" (lo), "=d" (hi));
    return (uint64_t)hi << 32 | lo;
    }
    let mut should_segv: c_int = 0;
#[no_mangle]
unsafe extern "C" fn sigsegv_cb(sig: c_int) {
    static void sigsegv_cb(int sig)
    {
    if (!should_segv)
    {
    fprintf(stderr, "FATAL ERROR, rdtsc() failed while enabled\n");
    exit(0);
    }
    if (prctl(PR_SET_TSC, PR_TSC_ENABLE) < 0)
    {
    perror("prctl");
    exit(0);
    }
    should_segv = 0;
    rdtsc();
    }
#[no_mangle]
unsafe extern "C" fn task() {
    static void task(void)
    {
    signal(SIGSEGV, sigsegv_cb);
    alarm(10);
    for(;;)
    {
    rdtsc();
    if (should_segv)
    {
    fprintf(stderr, "FATAL ERROR, rdtsc() succeeded while disabled\n");
    exit(0);
    }
    if (prctl(PR_SET_TSC, PR_TSC_SIGSEGV) < 0)
    {
    perror("prctl");
    exit(0);
    }
    should_segv = 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut n_tasks: c_int = 100, i;
    fprintf(stderr, "[No further output means we're all right]\n");
    for (i=0; i<n_tasks; i++)
    if (fork() == 0)
    task();
    for (i=0; i<n_tasks; i++)
    wait(core::ptr::null_mut());
    exit(0);
    }
