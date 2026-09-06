//! Automatically rewritten from C to Rust
//! Source: rust/helpers/binder.c
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
// Copyright (C) 2025 Google LLC.
//

#[no_mangle]
pub unsafe extern "C" fn rust_helper_list_lru_count(lru: *mut list_lru) -> __rust_helper unsigned long {
    __rust_helper unsigned long rust_helper_list_lru_count(struct list_lru *lru)
    {
    return list_lru_count(lru);
    }
    __rust_helper unsigned long rust_helper_list_lru_walk(struct list_lru *lru,
    list_lru_walk_cb isolate,
    void *cb_arg,
    unsigned long nr_to_walk)
    {
    return list_lru_walk(lru, isolate, cb_arg, nr_to_walk);
    }
    __rust_helper void rust_helper_init_task_work(struct callback_head *twork,
    task_work_func_t func)
    {
    init_task_work(twork, func);
    }
