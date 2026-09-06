//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/amd/ae4dma/ae4dma.h
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
// AMD AE4DMA driver
//
// Copyright (c) 2024, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Basavaraj Natikar <Basavaraj.Natikar@amd.com>
//

pub const MAX_AE4_HW_QUEUES: c_int = 16;
pub const AE4_DESC_COMPLETED: c_uint = 0x03;
pub const AE4_MAX_IDX_OFF: c_uint = 0x08;
pub const AE4_RD_IDX_OFF: c_uint = 0x0c;
pub const AE4_WR_IDX_OFF: c_uint = 0x10;
pub const AE4_INTR_STS_OFF: c_uint = 0x14;
pub const AE4_Q_BASE_L_OFF: c_uint = 0x18;
pub const AE4_Q_BASE_H_OFF: c_uint = 0x1c;
pub const AE4_Q_SZ: c_uint = 0x20;
pub const AE4_DMA_VERSION: c_int = 4;
pub const CMD_AE4_DESC_DW0_VAL: c_int = 2;
pub const AE4_TIME_OUT: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ae4_msix {
    pub msix_count: c_int,
    pub msix_entry: [msix_entry; MAX_AE4_HW_QUEUES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ae4_cmd_queue {
    pub ae4: *mut ae4_device,
    pub cmd_q: pt_cmd_queue,
    pub cmd: list_head,
// protect command operations
    pub cmd_lock: mutex,
    pub p_work: delayed_work,
    pub pws: *mut workqueue_struct,
    pub cmp: completion,
    pub q_w: wait_queue_head_t,
    pub intr_cnt: core::sync::atomic::AtomicI64,
    pub done_cnt: core::sync::atomic::AtomicI64,
    pub q_cmd_count: u64,
    pub dridx: u32,
    pub tail_wi: u32,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dwou {
    pub dw0: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dword0 {
    pub byte0: u8,
    pub byte1: u8,
    pub timestamp: u16,
    pub dws: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dword1 {
    pub status: u8,
    pub err_code: u8,
    pub desc_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ae4dma_desc {
    pub dwouv: dwou,
    pub dw1: dword1,
    pub length: u32,
    pub rsvd: u32,
    pub src_hi: u32,
    pub src_lo: u32,
    pub dst_hi: u32,
    pub dst_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ae4_device {
    pub pt: pt_device,
    pub ae4_msix: *mut ae4_msix,
    pub ae4cmd_q: [ae4_cmd_queue; MAX_AE4_HW_QUEUES],
    pub ae4_irq: [c_uint; MAX_AE4_HW_QUEUES],
    pub cmd_q_count: c_uint,
}

extern "C" {
    pub fn ae4_core_init(ae4: *mut ae4_device) -> c_int;
}
extern "C" {
    pub fn ae4_destroy_work(ae4: *mut ae4_device);
}
extern "C" {
    pub fn ae4_check_status_error(ae4cmd_q: *mut ae4_cmd_queue, idx: c_int);
}
