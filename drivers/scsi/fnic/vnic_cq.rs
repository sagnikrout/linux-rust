//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/vnic_cq.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

//
// These defines avoid symbol clash between fnic and enic (Cisco 10G Eth
// Driver) when both are built with CONFIG options =y
//

// Completion queue control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_cq_ctrl {
    pub /: *mut *mut u64 ring_base; / 0x00,
    pub /: *mut *mut u32 ring_size; / 0x08,
    pub pad0: u32,
    pub /: *mut *mut u32 flow_control_enable; / 0x10,
    pub pad1: u32,
    pub /: *mut *mut u32 color_enable; / 0x18,
    pub pad2: u32,
    pub /: *mut *mut u32 cq_head; / 0x20,
    pub pad3: u32,
    pub /: *mut *mut u32 cq_tail; / 0x28,
    pub pad4: u32,
    pub /: *mut *mut u32 cq_tail_color; / 0x30,
    pub pad5: u32,
    pub /: *mut *mut u32 interrupt_enable; / 0x38,
    pub pad6: u32,
    pub /: *mut *mut u32 cq_entry_enable; / 0x40,
    pub pad7: u32,
    pub /: *mut *mut u32 cq_message_enable; / 0x48,
    pub pad8: u32,
    pub /: *mut *mut u32 interrupt_offset; / 0x50,
    pub pad9: u32,
    pub /: *mut *mut u64 cq_message_addr; / 0x58,
    pub pad10: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_cq {
    pub index: c_uint,
    pub vdev: *mut vnic_dev,
    pub /: *mut *mut *mut vnic_cq_ctrl __iomem ctrl; / memory-mapped,
    pub ring: vnic_dev_ring,
    pub to_clean: c_uint,
    pub last_color: c_uint,
}

extern "C" {
    pub fn vnic_cq_free(cq: *mut vnic_cq);
}
extern "C" {
    pub fn vnic_cq_clean(cq: *mut vnic_cq);
}
