//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedi/qedi_fw_iscsi.h
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
// QLogic iSCSI Offload Driver
// Copyright (c) 2016 Cavium Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_task_params {
    pub context: *mut iscsi_task_context,
    pub sqe: *mut iscsi_wqe,
    pub tx_io_size: u32,
    pub rx_io_size: u32,
    pub conn_icid: u16,
    pub itid: u16,
    pub cq_rss_number: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_conn_params {
    pub first_burst_length: u32,
    pub max_send_pdu_length: u32,
    pub max_burst_length: u32,
    pub initial_r2t: bool,
    pub immediate_data: bool,
}

// @brief init_initiator_read_iscsi_task - initializes iSCSI Initiator Read
// task context.
//
// @param task_params	  - Pointer to task parameters struct
// @param conn_params	  - Connection Parameters
// @param cmd_params	  - command specific parameters
// @param cmd_pdu_header  - PDU Header Parameters
// @param sgl_task_params - Pointer to SGL task params
// @param dif_task_params - Pointer to DIF parameters struct
//
// @brief init_initiator_login_request_task - initializes iSCSI Initiator Login
// Request task context.
//
// @param task_params		  - Pointer to task parameters struct
// @param login_req_pdu_header    - PDU Header Parameters
// @param tx_sgl_task_params	  - Pointer to SGL task params
// @param rx_sgl_task_params	  - Pointer to SGL task params
//
// @brief init_initiator_nop_out_task - initializes iSCSI Initiator NOP Out
// task context.
//
// @param task_params		- Pointer to task parameters struct
// @param nop_out_pdu_header    - PDU Header Parameters
// @param tx_sgl_task_params	- Pointer to SGL task params
// @param rx_sgl_task_params	- Pointer to SGL task params
//
// @brief init_initiator_logout_request_task - initializes iSCSI Initiator
// Logout Request task context.
//
// @param task_params		- Pointer to task parameters struct
// @param logout_pdu_header  - PDU Header Parameters
// @param tx_sgl_task_params	- Pointer to SGL task params
// @param rx_sgl_task_params	- Pointer to SGL task params
//
// @brief init_initiator_tmf_request_task - initializes iSCSI Initiator TMF
// task context.
//
// @param task_params	- Pointer to task parameters struct
// @param tmf_pdu_header - PDU Header Parameters
//
// @brief init_initiator_text_request_task - initializes iSCSI Initiator Text
// Request task context.
//
// @param task_params		     - Pointer to task parameters struct
// @param text_request_pdu_header    - PDU Header Parameters
// @param tx_sgl_task_params	     - Pointer to Tx SGL task params
// @param rx_sgl_task_params	     - Pointer to Rx SGL task params
//
// @brief init_cleanup_task - initializes Clean task (SQE)
//
// @param task_params - Pointer to task parameters struct
//
extern "C" {
    pub fn init_cleanup_task(task_params: *mut iscsi_task_params) -> c_int;
}
