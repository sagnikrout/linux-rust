//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rxe/rxe_task.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//
// data structure to describe a 'task' which is a short
// function that returns 0 as long as it needs to be
// called again.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_task {
    pub work: work_struct,
    pub state: c_int,
    pub lock: spinlock_t,
    pub qp: *mut rxe_qp,
    pub qp): *mut *mut int (func)(struct rxe_qp,
    pub ret: c_int,
    pub num_sched: c_long,
    pub num_done: c_long,
}

extern "C" {
    pub fn rxe_alloc_wq() -> c_int;
}
extern "C" {
    pub fn rxe_destroy_wq();
}
//
// init rxe_task structure
// qp  => parameter to pass to func
// func => function to call until it returns != 0
//
// cleanup task
extern "C" {
    pub fn rxe_cleanup_task(task: *mut rxe_task);
}
extern "C" {
    pub fn rxe_sched_task(task: *mut rxe_task);
}
// keep a task from scheduling
extern "C" {
    pub fn rxe_disable_task(task: *mut rxe_task);
}
// allow task to run
extern "C" {
    pub fn rxe_enable_task(task: *mut rxe_task);
}
