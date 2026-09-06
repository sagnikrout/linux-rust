//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/send_signal_sched_switch.c
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
unsafe extern "C" fn sigusr1_handler(signum: c_int) {
    static void sigusr1_handler(int signum)
    {
    }
pub const THREAD_COUNT: c_int = 100;
    static void *worker(void *p)
    {
    int i;
    for ( i = 0; i < 1000; i++)
    usleep(1);
    return core::ptr::null_mut();
    }
// NOTE: cause events loss
#[no_mangle]
pub unsafe extern "C" fn serial_test_send_signal_sched_switch() {
    void serial_test_send_signal_sched_switch(void)
    {
    struct test_send_signal_kern *skel;
    pthread_t threads[THREAD_COUNT];
    let mut duration: u32 = 0;
    int i, err;
    signal(SIGUSR1, sigusr1_handler);
    skel = test_send_signal_kern__open_and_load();
    if (CHECK(!skel, "skel_open_and_load", "skeleton open_and_load failed\n"))
    return;
    skel.bss.pid = getpid();
    skel.bss.sig = SIGUSR1;
    err = test_send_signal_kern__attach(skel);
    if (CHECK(err, "skel_attach", "skeleton attach failed\n"))
    goto destroy_skel;
    for (i = 0; i < THREAD_COUNT; i++) {
    err = pthread_create(threads + i, core::ptr::null_mut(), worker, core::ptr::null_mut());
    if (CHECK(err, "pthread_create", "Error creating thread, %s\n",
    strerror(errno)))
    goto destroy_skel;
    }
    for (i = 0; i < THREAD_COUNT; i++)
    pthread_join(threads[i], core::ptr::null_mut());
    destroy_skel:
    test_send_signal_kern__destroy(skel);
    }
