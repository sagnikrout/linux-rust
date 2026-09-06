//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/mana/hw_channel.h
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
// Copyright (c) 2021, Microsoft Corporation.
pub const DEFAULT_LOG2_THROTTLING_FOR_ERROR_EQ: c_int = 4;
pub const HW_CHANNEL_MAX_REQUEST_SIZE: c_uint = 0x1000;
pub const HW_CHANNEL_MAX_RESPONSE_SIZE: c_uint = 0x1000;
pub const HW_CHANNEL_VF_BOOTSTRAP_QUEUE_DEPTH: c_int = 1;
pub const HWC_INIT_DATA_CQID: c_int = 1;
pub const HWC_INIT_DATA_RQID: c_int = 2;
pub const HWC_INIT_DATA_SQID: c_int = 3;
pub const HWC_INIT_DATA_QUEUE_DEPTH: c_int = 4;
pub const HWC_INIT_DATA_MAX_REQUEST: c_int = 5;
pub const HWC_INIT_DATA_MAX_RESPONSE: c_int = 6;
pub const HWC_INIT_DATA_MAX_NUM_CQS: c_int = 7;
pub const HWC_INIT_DATA_PDID: c_int = 8;
pub const HWC_INIT_DATA_GPA_MKEY: c_int = 9;
pub const HWC_INIT_DATA_PF_DEST_RQ_ID: c_int = 10;
pub const HWC_INIT_DATA_PF_DEST_CQ_ID: c_int = 11;
pub const HWC_DATA_CFG_HWC_TIMEOUT: c_int = 1;
pub const HWC_DATA_HW_LINK_CONNECT: c_int = 2;
pub const HWC_DATA_HW_LINK_DISCONNECT: c_int = 3;
pub const HW_CHANNEL_WAIT_RESOURCE_TIMEOUT_MS: c_int = 30000;
// Structures labeled with "HW DATA" are exchanged with the hardware. All of
// them are naturally aligned and hence don't need __packed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union hwc_init_eq_id_db {
    pub as_uint32: u32,
    pub 16: u32 eq_id :,
    pub 16: u32 doorbell :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hwc_init_type_data {
    pub as_uint32: u32,
    pub 24: u32 value :,
    pub 8: u32 type :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hwc_init_soc_service_type {
    pub as_uint32: u32,
    pub 28: u32 value :,
    pub 4: u32 type :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwc_rx_oob {
    pub 6: u32 type :,
    pub 1: u32 eom :,
    pub 1: u32 som :,
    pub 8: u32 vendor_err :,
    pub 16: u32 reserved1 :,
    pub 24: u32 src_virt_wq :,
    pub 8: u32 src_vfid :,
    pub reserved2: u32,
    pub wqe_addr_low: u32,
    pub wqe_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwc_tx_oob {
    pub reserved1: u32,
    pub reserved2: u32,
    pub 24: u32 vrq_id :,
    pub 8: u32 dest_vfid :,
    pub 24: u32 vrcq_id :,
    pub 8: u32 reserved3 :,
    pub 24: u32 vscq_id :,
    pub 1: u32 loopback :,
    pub 1: u32 lso_override:,
    pub 1: u32 dest_pf :,
    pub 5: u32 reserved4 :,
    pub 24: u32 vsq_id :,
    pub 8: u32 reserved5 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwc_work_request {
    pub buf_va: *mut c_void,
    pub buf_sge_addr: *mut c_void,
    pub buf_len: u32,
    pub msg_size: u32,
    pub wqe_req: gdma_wqe_request,
    pub tx_oob: hwc_tx_oob,
    pub sge: gdma_sge,
}

// hwc_dma_buf represents the array of in-flight WQEs.
// mem_info as know as the GDMA mapped memory is partitioned and used by
// in-flight WQEs.
// The number of WQEs is determined by the number of in-flight messages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwc_dma_buf {
    pub mem_info: gdma_mem_info,
    pub gpa_mkey: u32,
    pub num_reqs: u32,
    pub __counted_by(num_reqs): hwc_work_request reqs[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwc_cq {
    pub hwc: *mut hw_channel_context,
    pub gdma_cq: *mut gdma_queue,
    pub gdma_eq: *mut gdma_queue,
    pub comp_buf: *mut gdma_comp,
    pub queue_depth: u16,
    pub rx_event_handler: *mut hwc_rx_event_handler_t,
    pub rx_event_ctx: *mut c_void,
    pub tx_event_handler: *mut hwc_tx_event_handler_t,
    pub tx_event_ctx: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwc_wq {
    pub hwc: *mut hw_channel_context,
    pub gdma_wq: *mut gdma_queue,
    pub msg_buf: *mut hwc_dma_buf,
    pub queue_depth: u16,
    pub hwc_cq: *mut hwc_cq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwc_caller_ctx {
    pub comp_event: completion,
    pub output_buf: *mut c_void,
    pub output_buflen: u32,
    pub /: *mut *mut u32 error; / Linux error code,
    pub status_code: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_channel_context {
    pub gdma_dev: *mut gdma_dev,
    pub dev: *mut device,
    pub num_inflight_msg: u16,
    pub max_req_msg_size: u32,
    pub hwc_init_q_depth_max: u16,
    pub hwc_init_max_req_msg_size: u32,
    pub hwc_init_max_resp_msg_size: u32,
    pub hwc_init_eqe_comp: completion,
    pub rxq: *mut hwc_wq,
    pub txq: *mut hwc_wq,
    pub cq: *mut hwc_cq,
    pub sema: semaphore,
    pub inflight_msg_res: gdma_resource,
    pub pf_dest_vrq_id: u32,
    pub pf_dest_vrcq_id: u32,
    pub hwc_timeout: u32,
    pub caller_ctx: *mut hwc_caller_ctx,
}

extern "C" {
    pub fn mana_hwc_create_channel(gc: *mut gdma_context) -> c_int;
}
extern "C" {
    pub fn mana_hwc_destroy_channel(gc: *mut gdma_context);
}
