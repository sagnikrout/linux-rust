//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedf/drv_fcoe_fw_funcs.h
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
// QLogic FCoE Offload Driver
// Copyright (c) 2016-2018 Cavium Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_task_params {
// Output parameter [set/filled by the HSI function]
    pub context: *mut fcoe_task_context,
// Output parameter [set/filled by the HSI function]
    pub sqe: *mut fcoe_wqe,
    pub task_type: fcoe_task_type,
    pub /: *mut *mut u32 tx_io_size; / in bytes,
    pub /: *mut *mut u32 rx_io_size; / in bytes,
    pub conn_cid: u32,
    pub itid: u16,
    pub cq_rss_number: u8,
// Whether it's Tape device or not (0=Disk, 1=Tape)
    pub is_tape_device: u8,
}

//
// @brief init_initiator_rw_fcoe_task - Initializes FCoE task context for
// read/write task types and init fcoe_sqe
//
// @param task_params - Pointer to task parameters struct
// @param sgl_task_params - Pointer to SGL task params
// @param sense_data_buffer_phys_addr - Pointer to sense data buffer
// @param task_retry_id - retry identification - Used only for Tape device
// @param fcp_cmnd_payload - FCP CMD Payload
//
// @brief init_initiator_midpath_fcoe_task - Initializes FCoE task context for
// midpath/unsolicited task types and init fcoe_sqe
//
// @param task_params - Pointer to task parameters struct
// @param mid_path_fc_header - FC header
// @param tx_sgl_task_params - Pointer to Tx SGL task params
// @param rx_sgl_task_params - Pointer to Rx SGL task params
// @param fw_to_place_fc_header	- Indication if the FW will place the FC header
// in addition to the data arrives.
//
// @brief init_initiator_abort_fcoe_task - Initializes FCoE task context for
// abort task types and init fcoe_sqe
//
// @param task_params - Pointer to task parameters struct
//
extern "C" {
    pub fn init_initiator_abort_fcoe_task(task_params: *mut fcoe_task_params) -> c_int;
}
//
// @brief init_initiator_cleanup_fcoe_task - Initializes FCoE task context for
// cleanup task types and init fcoe_sqe
//
// @param task_params - Pointer to task parameters struct
//
extern "C" {
    pub fn init_initiator_cleanup_fcoe_task(task_params: *mut fcoe_task_params) -> c_int;
}
//
// @brief init_initiator_cleanup_fcoe_task - Initializes FCoE task context for
// sequence recovery task types and init fcoe_sqe
//
// @param task_params - Pointer to task parameters struct
// @param desired_offset - The desired offest the task will be re-sent from
//
