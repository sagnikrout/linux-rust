//! Automatically rewritten from C to Rust
//! Source: lib/test_ref_tracker.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Reference tracker self test.
//
// Copyright (c) 2021 Eric Dumazet <edumazet@google.com>
//

    static struct ref_tracker_dir ref_dir;
    static struct ref_tracker *tracker[20];

    alloctest_ref_tracker_alloc##X(struct ref_tracker_dir *dir, 	\
    struct ref_tracker **trackerp)	\
    {								\
    ref_tracker_alloc(dir, trackerp, GFP_KERNEL);		\
    }
    TRT_ALLOC(1)
    TRT_ALLOC(2)
    TRT_ALLOC(3)
    TRT_ALLOC(4)
    TRT_ALLOC(5)
    TRT_ALLOC(6)
    TRT_ALLOC(7)
    TRT_ALLOC(8)
    TRT_ALLOC(9)
    TRT_ALLOC(10)
    TRT_ALLOC(11)
    TRT_ALLOC(12)
    TRT_ALLOC(13)
    TRT_ALLOC(14)
    TRT_ALLOC(15)
    TRT_ALLOC(16)
    TRT_ALLOC(17)
    TRT_ALLOC(18)
    TRT_ALLOC(19)

    static noinline void
    alloctest_ref_tracker_free(struct ref_tracker_dir *dir,
    struct ref_tracker **trackerp)
    {
    ref_tracker_free(dir, trackerp);
    }
    static struct timer_list test_ref_tracker_timer;
    let mut test_ref_timer_done: static atomic_t = ATOMIC_INIT(0);
#[no_mangle]
unsafe extern "C" fn test_ref_tracker_timer_func(t: *mut timer_list) {
    static void test_ref_tracker_timer_func(struct timer_list *t)
    {
    ref_tracker_alloc(&ref_dir, &tracker[0], GFP_ATOMIC);
    atomic_set(&test_ref_timer_done, 1);
    }
#[no_mangle]
unsafe extern "C" fn test_ref_tracker_init() -> int __init {
    static int __init test_ref_tracker_init(void)
    {
    int i;
    ref_tracker_dir_init(&ref_dir, 100, "selftest");
    timer_setup(&test_ref_tracker_timer, test_ref_tracker_timer_func, 0);
    mod_timer(&test_ref_tracker_timer, jiffies + 1);
    alloctest_ref_tracker_alloc1(&ref_dir, &tracker[1]);
    alloctest_ref_tracker_alloc2(&ref_dir, &tracker[2]);
    alloctest_ref_tracker_alloc3(&ref_dir, &tracker[3]);
    alloctest_ref_tracker_alloc4(&ref_dir, &tracker[4]);
    alloctest_ref_tracker_alloc5(&ref_dir, &tracker[5]);
    alloctest_ref_tracker_alloc6(&ref_dir, &tracker[6]);
    alloctest_ref_tracker_alloc7(&ref_dir, &tracker[7]);
    alloctest_ref_tracker_alloc8(&ref_dir, &tracker[8]);
    alloctest_ref_tracker_alloc9(&ref_dir, &tracker[9]);
    alloctest_ref_tracker_alloc10(&ref_dir, &tracker[10]);
    alloctest_ref_tracker_alloc11(&ref_dir, &tracker[11]);
    alloctest_ref_tracker_alloc12(&ref_dir, &tracker[12]);
    alloctest_ref_tracker_alloc13(&ref_dir, &tracker[13]);
    alloctest_ref_tracker_alloc14(&ref_dir, &tracker[14]);
    alloctest_ref_tracker_alloc15(&ref_dir, &tracker[15]);
    alloctest_ref_tracker_alloc16(&ref_dir, &tracker[16]);
    alloctest_ref_tracker_alloc17(&ref_dir, &tracker[17]);
    alloctest_ref_tracker_alloc18(&ref_dir, &tracker[18]);
    alloctest_ref_tracker_alloc19(&ref_dir, &tracker[19]);
// free all trackers but first 0 and 1.
    for (i = 2; i < ARRAY_SIZE(tracker); i++)
    alloctest_ref_tracker_free(&ref_dir, &tracker[i]);
// Attempt to free an already freed tracker.
    alloctest_ref_tracker_free(&ref_dir, &tracker[2]);
    while (!atomic_read(&test_ref_timer_done))
    msleep(1);
// This should warn about tracker[0] & tracker[1] being not freed.
    ref_tracker_dir_exit(&ref_dir);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_ref_tracker_exit() -> void __exit {
    static void __exit test_ref_tracker_exit(void)
    {
    }
    module_init(test_ref_tracker_init);
    module_exit(test_ref_tracker_exit);
    MODULE_DESCRIPTION("Reference tracker self test");
    MODULE_LICENSE("GPL v2");
