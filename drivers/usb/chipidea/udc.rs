//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/chipidea/udc.h
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
// udc.h - ChipIdea UDC structures
//
// Copyright (C) 2008 Chipidea - MIPS Technologies, Inc. All rights reserved.
//
// Author: David Lopo
//

pub const CTRL_PAYLOAD_MAX: c_int = 64;

// DMA layout of transfer descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_hw_td {
// 0
    pub next: __le32,

// 1
    pub token: __le32,

// 2
    pub page: [__le32; 5],
// C attribute field omitted
// DMA layout of queue heads
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_hw_qh {
// 0
    pub cap: __le32,

// 1
    pub curr: __le32,
// 2 - 8
    pub td: ci_hw_td,
// 9
    pub RESERVED: __le32,
    pub setup: usb_ctrlrequest,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct td_node {
    pub td: list_head,
    pub dma: dma_addr_t,
    pub ptr: *mut ci_hw_td,
    pub td_remaining_size: c_int,
}

//
// struct ci_hw_req - usb request representation
// @req: request structure for gadget drivers
// @queue: link to QH list
// @tds: link to TD list
// @sgt: hold original sglist when bounce sglist
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_hw_req {
    pub req: usb_request,
    pub queue: list_head,
    pub tds: list_head,
    pub sgt: sg_table,
}

extern "C" {
    pub fn ci_hdrc_gadget_init(ci: *mut ci_hdrc) -> c_int;
}
extern "C" {
    pub fn ci_hdrc_gadget_destroy(ci: *mut ci_hdrc);
}

