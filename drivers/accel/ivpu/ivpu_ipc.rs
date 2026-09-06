//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_ipc.h
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
// Copyright (C) 2020-2024 Intel Corporation
//

// VPU FW boot notification
pub const IVPU_IPC_CHAN_BOOT_MSG: c_uint = 0x3ff;
pub const IVPU_IPC_BOOT_MSG_DATA_ADDR: c_uint = 0x424f4f54;
// The alignment to be used for IPC Buffers and IPC Data.
pub const IVPU_IPC_ALIGNMENT: c_int = 64;
pub const IVPU_IPC_HDR_FREE: c_int = 0;
pub const IVPU_IPC_HDR_ALLOCATED: c_int = 1;
//
// struct ivpu_ipc_hdr - The IPC message header structure, exchanged
// with the VPU device firmware.
// @data_addr: The VPU address of the payload (JSM message)
// @data_size: The size of the payload.
// @channel: The channel used.
// @src_node: The Node ID of the sender.
// @dst_node: The Node ID of the intended receiver.
// @status: IPC buffer usage status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_ipc_hdr {
    pub data_addr: u32,
    pub data_size: u32,
    pub channel: u16,
    pub src_node: u8,
    pub dst_node: u8,
    pub status: u8,
    pub __aligned(IVPU_IPC_ALIGNMENT): } __packed,
    pub jsm_msg): *mut vpu_jsm_msg,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_ipc_rx_msg {
    pub link: list_head,
    pub ipc_hdr: *mut ivpu_ipc_hdr,
    pub jsm_msg: *mut vpu_jsm_msg,
    pub callback: ivpu_ipc_rx_callback_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_ipc_consumer {
    pub link: list_head,
    pub channel: u32,
    pub tx_vpu_addr: u32,
    pub request_id: u32,
    pub aborted: bool,
    pub rx_callback: ivpu_ipc_rx_callback_t,
    pub /: *mut *mut spinlock_t rx_lock; / Protects rx_msg_list and aborted,
    pub rx_msg_list: list_head,
    pub rx_msg_wq: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_ipc_info {
    pub mm_tx: *mut gen_pool,
    pub mem_tx: *mut ivpu_bo,
    pub mem_rx: *mut ivpu_bo,
    pub rx_msg_cache: *mut kmem_cache,
    pub rx_msg_count: core::sync::atomic::AtomicI32,
    pub /: *mut *mut spinlock_t cons_lock; / Protects cons_list and cb_msg_list,
    pub cons_list: list_head,
    pub cb_msg_list: list_head,
    pub request_id: core::sync::atomic::AtomicI32,
    pub /: *mut *mut mutex lock; / Lock on status,
    pub on: bool,
}

extern "C" {
    pub fn ivpu_ipc_init(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_ipc_fini(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_ipc_enable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_ipc_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_ipc_reset(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_ipc_irq_handler(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_ipc_irq_thread_handler(irq: c_int, ptr: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ivpu_ipc_consumer_del(vdev: *mut ivpu_device, cons: *mut ivpu_ipc_consumer);
}
