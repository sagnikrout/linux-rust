//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_mbox.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

// identifies if a segment belongs to a message or to a response. A VF is only
// expected to send messages and receive responses. PF driver could receive
// messages and send responses.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox_msg_direction_type {
    MBOX_MSG_SEND = 0,
    MBOX_MSG_RESP = 1,
}

// Indicates if mbox message expects a response (ack) or not
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox_msg_ack_type {
    MBOX_MSG_ACK    = 0,
    MBOX_MSG_NO_ACK = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox_msg_data_type {
    MBOX_MSG_DATA_INLINE = 0,
    MBOX_MSG_DATA_DMA    = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox_msg_src_type {
    MBOX_MSG_FROM_MBOX = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox_msg_aeq_type {
    MBOX_MSG_AEQ_FOR_EVENT = 0,
    MBOX_MSG_AEQ_FOR_MBOX  = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_msg_info {
    pub msg_id: u8,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_msg_desc {
    pub msg: *mut u8,
    pub msg_len: __le16,
    pub seq_id: u8,
    pub mod: u8,
    pub cmd: __le16,
    pub msg_info: mbox_msg_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_msg_channel {
    pub resp_msg: hinic3_msg_desc,
    pub recv_msg: hinic3_msg_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_send_mbox {
    pub data: *mut u8 __iomem,
    pub wb_vaddr: *mut c_void,
    pub wb_paddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox_event_state {
    MBOX_EVENT_START   = 0,
    MBOX_EVENT_FAIL    = 1,
    MBOX_EVENT_SUCCESS = 2,
    MBOX_EVENT_TIMEOUT = 3,
    MBOX_EVENT_END     = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_dma_msg {
    pub xor: __le32,
    pub dma_addr_high: __le32,
    pub dma_addr_low: __le32,
    pub msg_len: __le32,
    pub rsvd: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_dma_queue {
    pub dma_buf_vaddr: *mut c_void,
    pub dma_buf_paddr: dma_addr_t,
    pub depth: u16,
    pub prod_idx: u16,
    pub cons_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_mbox {
    pub hwdev: *mut hinic3_hwdev,
// lock for send mbox message and ack message
    pub mbox_send_lock: mutex,
// lock for send message transmission.
// The lock hierarchy is mbox_send_lock -> mbox_seg_send_lock.
//
    pub mbox_seg_send_lock: mutex,
    pub send_mbox: hinic3_send_mbox,
    pub sync_msg_queue: mbox_dma_queue,
    pub async_msg_queue: mbox_dma_queue,
    pub workq: *mut workqueue_struct,
// driver and MGMT CPU
    pub mgmt_msg: hinic3_msg_channel,
// VF to PF
    pub func_msg: *mut hinic3_msg_channel,
    pub send_msg_id: u8,
    pub event_flag: mbox_event_state,
// lock for mbox event flag
    pub mbox_lock: spinlock_t,
}

extern "C" {
    pub fn hinic3_init_mbox(hwdev: *mut hinic3_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic3_free_mbox(hwdev: *mut hinic3_hwdev);
}
