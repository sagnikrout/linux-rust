//! Automatically rewritten from C to Rust
//! Source: tools/build/feature/test-all.c
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
// test-all.c: Try to build all the main testcases at once.
//
// A well-configured system will have all the prereqs installed, so we can speed
// up auto-detection on such systems.
//
// Quirk: Python headers cannot be in arbitrary places, so keep this testcase at
// the top:
//

//
// Disable babeltrace2-ctf-writer check for test-all, because the requested
// library version is not released yet in most distributions. Will
// reenable later.
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    main_test_libpython();
    main_test_hello();
    main_test_libelf();
    main_test_gettid();
    main_test_glibc();
    main_test_libdw();
    main_test_eventfd();
    main_test_libelf_getphdrnum();
    main_test_libelf_gelf_getnote();
    main_test_libelf_getshdrstrndx();
    main_test_libslang();
    main_test_backtrace();
    main_test_libnuma();
    main_test_numa_num_possible_cpus();
    main_test_timerfd();
    main_test_stackprotector_all();
    main_test_zlib();
    main_test_pthread_attr_setaffinity_np();
    main_test_pthread_barrier();
    main_test_lzma();
    main_test_bpf();
    main_test_scandirat();
    main_test_sched_getcpu();
    main_test_sdt();
    main_test_setns();
    main_test_libaio();
    main_test_reallocarray();
    main_test_libzstd();
    main_test_libtraceevent();
    main_test_libopenssl();
    return 0;
    }
