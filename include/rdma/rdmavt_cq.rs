//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/rdmavt_cq.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2016 - 2018 Intel Corporation.
//

//
// Define an ib_cq_notify value that is not valid so we know when CQ
// notifications are armed.
//

//
// Define read macro that apply smp_load_acquire memory barrier
// when reading indice of circular buffer that mmaped to user space.
//

//
// Define write macro that uses smp_store_release memory barrier
// when writing indice of circular buffer that mmaped to user space.
//

//
// This structure is used to contain the head pointer, tail pointer,
// and completion queue entries as a single memory allocation so
// it can be mmap'ed into user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_k_cq_wc {
    pub /: *mut *mut u32 head; / index of next entry to fill,
    pub /: *mut *mut u32 tail; / index of next ib_poll_cq() entry,
    pub kqueue: [ib_wc; ],
}

//
// The completion queue structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_cq {
    pub ibcq: ib_cq,
    pub comptask: work_struct,
    pub /: *mut *mut spinlock_t lock; / protect changes in this struct,
    pub notify: u8,
    pub triggered: u8,
    pub cq_full: u8,
    pub comp_vector_cpu: c_int,
    pub rdi: *mut rvt_dev_info,
    pub queue: *mut rvt_cq_wc,
    pub ip: *mut rvt_mmap_info,
    pub kqueue: *mut rvt_k_cq_wc,
}

extern "C" {
    pub fn container_of(_arg: ibcq, rvt_cq: struct, _arg: ibcq) -> return;
}
extern "C" {
    pub fn rvt_cq_enter(cq: *mut rvt_cq, entry: *mut ib_wc, solicited: bool) -> bool;
}
