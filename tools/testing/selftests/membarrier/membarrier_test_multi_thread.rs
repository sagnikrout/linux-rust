//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/membarrier/membarrier_test_multi_thread.c
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

    static int thread_ready, thread_quit;
    static pthread_mutex_t test_membarrier_thread_mutex =
    PTHREAD_MUTEX_INITIALIZER;
    static pthread_cond_t test_membarrier_thread_cond =
    PTHREAD_COND_INITIALIZER;
    void *test_membarrier_thread(void *arg)
    {
    pthread_mutex_lock(&test_membarrier_thread_mutex);
    thread_ready = 1;
    pthread_cond_broadcast(&test_membarrier_thread_cond);
    pthread_mutex_unlock(&test_membarrier_thread_mutex);
    pthread_mutex_lock(&test_membarrier_thread_mutex);
    while (!thread_quit)
    pthread_cond_wait(&test_membarrier_thread_cond,
    &test_membarrier_thread_mutex);
    pthread_mutex_unlock(&test_membarrier_thread_mutex);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn test_mt_membarrier() -> c_int {
    static int test_mt_membarrier(void)
    {
    int i;
    pthread_t test_thread;
    pthread_create(&test_thread, core::ptr::null_mut(),
    test_membarrier_thread, core::ptr::null_mut());
    pthread_mutex_lock(&test_membarrier_thread_mutex);
    while (!thread_ready)
    pthread_cond_wait(&test_membarrier_thread_cond,
    &test_membarrier_thread_mutex);
    pthread_mutex_unlock(&test_membarrier_thread_mutex);
    test_membarrier_fail();
    test_membarrier_success();
    pthread_mutex_lock(&test_membarrier_thread_mutex);
    thread_quit = 1;
    pthread_cond_broadcast(&test_membarrier_thread_cond);
    pthread_mutex_unlock(&test_membarrier_thread_mutex);
    pthread_join(test_thread, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    ksft_print_header();
    ksft_set_plan(16);
    test_membarrier_query();
// Multi-threaded
    test_mt_membarrier();
    ksft_exit_pass();
    }
