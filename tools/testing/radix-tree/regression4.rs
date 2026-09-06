//! Automatically rewritten from C to Rust
//! Source: tools/testing/radix-tree/regression4.c
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

    static pthread_barrier_t worker_barrier;
    static int obj0, obj1;
    static RADIX_TREE(mt_tree, GFP_KERNEL);
    static void *reader_fn(void *arg)
    {
    int i;
    void *entry;
    rcu_register_thread();
    pthread_barrier_wait(&worker_barrier);
    for (i = 0; i < 1000000; i++) {
    rcu_read_lock();
    entry = radix_tree_lookup(&mt_tree, 0);
    rcu_read_unlock();
    if (entry != &obj0) {
    printf("iteration %d bad entry = %p\n", i, entry);
    abort();
    }
    }
    rcu_unregister_thread();
    return core::ptr::null_mut();
    }
    static void *writer_fn(void *arg)
    {
    int i;
    rcu_register_thread();
    pthread_barrier_wait(&worker_barrier);
    for (i = 0; i < 1000000; i++) {
    radix_tree_insert(&mt_tree, 1, &obj1);
    radix_tree_delete(&mt_tree, 1);
    }
    rcu_unregister_thread();
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn regression4_test() {
    void regression4_test(void)
    {
    pthread_t reader, writer;
    printv(1, "regression test 4 starting\n");
    radix_tree_insert(&mt_tree, 0, &obj0);
    pthread_barrier_init(&worker_barrier, core::ptr::null_mut(), 2);
    if (pthread_create(&reader, core::ptr::null_mut(), reader_fn, core::ptr::null_mut()) ||
    pthread_create(&writer, core::ptr::null_mut(), writer_fn, core::ptr::null_mut())) {
    perror("pthread_create");
    exit(1);
    }
    if (pthread_join(reader, core::ptr::null_mut()) || pthread_join(writer, core::ptr::null_mut())) {
    perror("pthread_join");
    exit(1);
    }
    printv(1, "regression test 4 passed\n");
    }
