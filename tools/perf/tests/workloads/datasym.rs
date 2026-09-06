//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/datasym.c
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct buf {
    pub data1: c_char,
    pub reserved: [c_char; 55],
    pub data2: c_char,
    pub __attribute__((aligned(64))): },
// volatile to try to avoid the compiler seeing reserved as unused.
    static volatile struct buf workload_datasym_buf1 = {
// to have this in the data section
    .reserved[0] = 1,
}

    static volatile sig_atomic_t done;
#[no_mangle]
unsafe extern "C" fn sighandler(__maybe_unused: int sig) {
    static void sighandler(int sig __maybe_unused)
    {
    done = 1;
    }
#[no_mangle]
unsafe extern "C" fn datasym(argc: c_int, argv: *const c_char) -> c_int {
    static int datasym(int argc, const char **argv)
    {
    let mut sec: c_int = 1;
    if (argc > 0)
    sec = atoi(argv[0]);
    signal(SIGINT, sighandler);
    signal(SIGALRM, sighandler);
    alarm(sec);
    while (!done) {
    workload_datasym_buf1.data1++;
    if (workload_datasym_buf1.data1 == 123) {
//
// Add some 'noise' in the loop to work around errata
// 1694299 on Arm N1.
//
// Bias exists in SPE sampling which can cause the load
// and store instructions to be skipped entirely. This
// comes and goes randomly depending on the offset the
// linker places the datasym loop at in the Perf binary.
// With an extra branch in the middle of the loop that
// isn't always taken, the instruction stream is no
// longer a continuous repeating pattern that interacts
// badly with the bias.
//
    workload_datasym_buf1.data1++;
    }
    workload_datasym_buf1.data2 += workload_datasym_buf1.data1;
    }
    return 0;
    }
    DEFINE_WORKLOAD(datasym);
