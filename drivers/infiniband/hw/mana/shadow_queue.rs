//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mana/shadow_queue.h
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
// Copyright (c) 2024, Microsoft Corporation. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shadow_wqe_header {
    pub opcode: u16,
    pub error_code: u16,
    pub posted_wqe_size: u32,
    pub wr_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ud_rq_shadow_wqe {
    pub header: shadow_wqe_header,
    pub byte_len: u32,
    pub src_qpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ud_sq_shadow_wqe {
    pub header: shadow_wqe_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shadow_queue {
// Unmasked producer index, Incremented on wqe posting
    pub prod_idx: u64,
// Unmasked consumer index, Incremented on cq polling
    pub cons_idx: u64,
// Unmasked index of next-to-complete (from HW) shadow WQE
    pub next_to_complete_idx: u64,
// queue size in wqes
    pub length: u32,
// distance between elements in bytes
    pub stride: u32,
// ring buffer holding wqes
    pub buffer: *mut c_void,
}

extern "C" {
    pub fn shadow_queue_get_element(_arg: queue, _arg: queue->prod_idx) -> return;
}
extern "C" {
    pub fn shadow_queue_get_element(_arg: queue, _arg: queue->cons_idx) -> return;
}
extern "C" {
    pub fn shadow_queue_get_element(_arg: queue, _arg: queue->next_to_complete_idx) -> return;
}
