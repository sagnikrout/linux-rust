//! Automatically rewritten from C to Rust
//! Source: tools/testing/radix-tree/iteration_check_2.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// iteration_check_2.c: Check that deleting a tagged entry doesn't cause
// an RCU walker to finish early.
// Copyright (c) 2020 Oracle
// Author: Matthew Wilcox <willy@infradead.org>
//

    static volatile bool test_complete;
    static void *iterator(void *arg)
    {
    XA_STATE(xas, arg, 0);
    void *entry;
    rcu_register_thread();
    while (!test_complete) {
    xas_set(&xas, 0);
    rcu_read_lock();
    xas_for_each_marked(&xas, entry, ULONG_MAX, XA_MARK_0)
    ;
    rcu_read_unlock();
    assert(xas.xa_index >= 100);
    }
    rcu_unregister_thread();
    return core::ptr::null_mut();
    }
    static void *throbber(void *arg)
    {
    struct xarray *xa = arg;
    rcu_register_thread();
    while (!test_complete) {
    int i;
    for (i = 0; i < 100; i++) {
    xa_store(xa, i, xa_mk_value(i), GFP_KERNEL);
    xa_set_mark(xa, i, XA_MARK_0);
    }
    for (i = 0; i < 100; i++)
    xa_erase(xa, i);
    }
    rcu_unregister_thread();
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn iteration_test2(test_duration: unsigned) {
    void iteration_test2(unsigned test_duration)
    {
    pthread_t threads[2];
    DEFINE_XARRAY(array);
    int i;
    printv(1, "Running iteration test 2 for %d seconds\n", test_duration);
    test_complete = false;
    xa_store(&array, 100, xa_mk_value(100), GFP_KERNEL);
    xa_set_mark(&array, 100, XA_MARK_0);
    if (pthread_create(&threads[0], core::ptr::null_mut(), iterator, &array)) {
    perror("create iterator thread");
    exit(1);
    }
    if (pthread_create(&threads[1], core::ptr::null_mut(), throbber, &array)) {
    perror("create throbber thread");
    exit(1);
    }
    sleep(test_duration);
    test_complete = true;
    for (i = 0; i < 2; i++) {
    if (pthread_join(threads[i], core::ptr::null_mut())) {
    perror("pthread_join");
    exit(1);
    }
    }
    xa_destroy(&array);
    }
