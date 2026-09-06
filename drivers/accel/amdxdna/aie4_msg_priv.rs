//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/aie4_msg_priv.h
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
// Copyright (C) 2026, Advanced Micro Devices, Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aie4_msg_opcode {
    AIE4_MSG_OP_SUSPEND                          = 0x10003,
    AIE4_MSG_OP_ATTACH_WORK_BUFFER               = 0x1000D,

    AIE4_MSG_OP_CREATE_VFS                       = 0x20001,
    AIE4_MSG_OP_DESTROY_VFS                      = 0x20002,

    AIE4_MSG_OP_CREATE_PARTITION                 = 0x30001,
    AIE4_MSG_OP_DESTROY_PARTITION                = 0x30002,
    AIE4_MSG_OP_CREATE_HW_CONTEXT                = 0x30003,
    AIE4_MSG_OP_DESTROY_HW_CONTEXT               = 0x30004,
    AIE4_MSG_OP_AIE_TILE_INFO                    = 0x30006,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aie4_msg_status {
    AIE4_MSG_STATUS_SUCCESS = 0x0,
    AIE4_MSG_STATUS_ERROR = 0x1,
    AIE4_MSG_STATUS_NOTSUPP = 0x2,
    MAX_AIE4_MSG_STATUS_CODE = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_suspend_req {
    pub rsvd: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_suspend_resp {
    pub status: aie4_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_create_vfs_req {
    pub vf_cnt: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_create_vfs_resp {
    pub status: aie4_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_destroy_vfs_req {
    pub rsvd: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_destroy_vfs_resp {
    pub status: aie4_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_create_partition_req {
    pub partition_col_start: __u32,
    pub partition_col_count: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_create_partition_resp {
    pub status: aie4_msg_status,
    pub partition_id: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_destroy_partition_req {
    pub partition_id: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_destroy_partition_resp {
    pub status: aie4_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_create_hw_context_req {
    pub partition_id: __u32,
    pub request_num_tiles: __u32,
    pub hsa_addr_high: __u32,
    pub hsa_addr_low: __u32,

    pub pasid: __u32,
    pub priority_band: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_create_hw_context_resp {
    pub status: aie4_msg_status,
    pub hw_context_id: __u32,
    pub doorbell_offset: __u32,
    pub job_complete_msix_idx: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_destroy_hw_context_req {
    pub hw_context_id: __u32,
    pub resvd1: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_destroy_hw_context_resp {
    pub status: aie4_msg_status,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_tile_info {
    pub size: __u32,
    pub major: __u16,
    pub minor: __u16,
    pub cols: __u16,
    pub rows: __u16,
    pub core_rows: __u16,
    pub mem_rows: __u16,
    pub shim_rows: __u16,
    pub core_row_start: __u16,
    pub mem_row_start: __u16,
    pub shim_row_start: __u16,
    pub core_dma_channels: __u16,
    pub mem_dma_channels: __u16,
    pub shim_dma_channels: __u16,
    pub core_locks: __u16,
    pub mem_locks: __u16,
    pub shim_locks: __u16,
    pub core_events: __u16,
    pub mem_events: __u16,
    pub shim_events: __u16,
    pub resvd: __u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_aie4_tile_info_req {
    pub resvd: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_aie4_tile_info_resp {
    pub status: aie4_msg_status,
    pub info: aie4_tile_info,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_attach_work_buffer_req {
    pub buff_addr: __u64,
    pub reserved: __u32,
    pub buff_size: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie4_msg_attach_work_buffer_resp {
    pub status: aie4_msg_status,
    pub __packed: },
