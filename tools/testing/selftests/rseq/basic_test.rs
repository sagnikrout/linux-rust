//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/rseq/basic_test.c
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


// SPDX-License-Identifier: LGPL-2.1
//
// Basic test coverage for critical regions and rseq_current_cpu().
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn test_cpu_pointer() {
    void test_cpu_pointer(void)
    {
    cpu_set_t affinity, test_affinity;
    int i;
    sched_getaffinity(0, sizeof(affinity), &affinity);
    CPU_ZERO(&test_affinity);
    for (i = 0; i < CPU_SETSIZE; i++) {
    if (CPU_ISSET(i, &affinity)) {
    int node;
    CPU_SET(i, &test_affinity);
    sched_setaffinity(0, sizeof(test_affinity),
    &test_affinity);
    assert(sched_getcpu() == i);
    assert(rseq_current_cpu() == i);
    assert(rseq_current_cpu_raw() == i);
    assert(rseq_cpu_start() == i);
    node = rseq_fallback_current_node();
    assert(rseq_current_node_id() == node);
    CPU_CLR(i, &test_affinity);
    }
    }
    sched_setaffinity(0, sizeof(affinity), &affinity);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    if (rseq_register_current_thread()) {
    fprintf(stderr, "Error: rseq_register_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    goto init_thread_error;
    }
    printf("testing current cpu\n");
    test_cpu_pointer();
    if (rseq_unregister_current_thread()) {
    fprintf(stderr, "Error: rseq_unregister_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    goto init_thread_error;
    }
    return 0;
    init_thread_error:
    return -1;
    }
