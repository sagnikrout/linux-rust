//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/core/mad_priv.h
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


//
// Copyright (c) 2004, 2005, Voltaire, Inc. All rights reserved.
// Copyright (c) 2005 Intel Corporation. All rights reserved.
// Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
// Copyright (c) 2009 HNR Consulting. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// QP and CQ parameters
pub const IB_MAD_QP_SEND_SIZE: c_int = 128;
pub const IB_MAD_QP_RECV_SIZE: c_int = 512;
pub const IB_MAD_QP_MIN_SIZE: c_int = 64;
pub const IB_MAD_QP_MAX_SIZE: c_int = 8192;
pub const IB_MAD_SEND_REQ_MAX_SG: c_int = 2;
pub const IB_MAD_RECV_REQ_MAX_SG: c_int = 1;
pub const IB_MAD_SEND_Q_PSN: c_int = 0;
// Registration table sizes
pub const MAX_MGMT_CLASS: c_int = 80;
pub const MAX_MGMT_VERSION: c_uint = 0x83;
pub const MAX_MGMT_OUI: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_list_head {
    pub list: list_head,
    pub cqe: ib_cqe,
    pub mad_queue: *mut ib_mad_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_private_header {
    pub mad_list: ib_mad_list_head,
    pub recv_wc: ib_mad_recv_wc,
    pub wc: ib_wc,
    pub mapping: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_private {
    pub header: ib_mad_private_header,
    pub mad_size: usize,
    pub grh: ib_grh,
    pub mad: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_rmpp_segment {
    pub list: list_head,
    pub num: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_agent_private {
    pub agent: ib_mad_agent,
    pub reg_req: *mut ib_mad_reg_req,
    pub qp_info: *mut ib_mad_qp_info,
    pub lock: spinlock_t,
    pub send_list: list_head,
    pub sol_fc_send_count: c_uint,
    pub wait_list: list_head,
    pub sol_fc_wait_count: c_uint,
    pub timed_work: delayed_work,
    pub timeout: c_ulong,
    pub local_list: list_head,
    pub local_work: work_struct,
    pub rmpp_list: list_head,
    pub sol_fc_max: c_uint,
    pub backlog_list: list_head,
    pub refcount: refcount_t,
    pub comp: completion,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_snoop_private {
    pub agent: ib_mad_agent,
    pub qp_info: *mut ib_mad_qp_info,
    pub snoop_index: c_int,
    pub mad_snoop_flags: c_int,
    pub comp: completion,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_mad_state {
// MAD is in the making and is not yet in any list
    IB_MAD_STATE_INIT,
// MAD is in backlog list
    IB_MAD_STATE_QUEUED,
//
// MAD was sent to the QP and is waiting for completion
// notification in send list.
//
    IB_MAD_STATE_SEND_START,
//
// MAD send completed successfully, waiting for a response
// in wait list.
//
    IB_MAD_STATE_WAIT_RESP,
//
// Response came early, before send completion notification,
// in send list.
//
    IB_MAD_STATE_EARLY_RESP,
// MAD was canceled while in wait or send list
    IB_MAD_STATE_CANCELED,
// MAD processing completed, MAD in no list
    IB_MAD_STATE_DONE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_send_wr_private {
    pub mad_list: ib_mad_list_head,
    pub agent_list: list_head,
    pub mad_agent_priv: *mut ib_mad_agent_private,
    pub send_buf: ib_mad_send_buf,
    pub header_mapping: u64,
    pub payload_mapping: u64,
    pub send_wr: ib_ud_wr,
    pub sg_list: [ib_sge; IB_MAD_SEND_REQ_MAX_SG],
    pub tid: __be64,
    pub timeout: c_ulong,
    pub max_retries: c_int,
    pub retries_left: c_int,
    pub retry: c_int,
// RMPP control
    pub rmpp_list: list_head,
    pub last_ack_seg: *mut ib_rmpp_segment,
    pub cur_seg: *mut ib_rmpp_segment,
    pub last_ack: c_int,
    pub seg_num: c_int,
    pub newwin: c_int,
    pub pad: c_int,
    pub state: ib_mad_state,
// Solicited MAD flow control
    pub is_solicited_fc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_local_private {
    pub completion_list: list_head,
    pub mad_priv: *mut ib_mad_private,
    pub recv_mad_agent: *mut ib_mad_agent_private,
    pub mad_send_wr: *mut ib_mad_send_wr_private,
    pub return_wc_byte_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_mgmt_method_table {
    pub agent: [*mut ib_mad_agent_private; IB_MGMT_MAX_METHODS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_mgmt_class_table {
    pub method_table: [*mut ib_mad_mgmt_method_table; MAX_MGMT_CLASS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_mgmt_vendor_class {
    pub oui: [u8; MAX_MGMT_OUI][3],
    pub method_table: [*mut ib_mad_mgmt_method_table; MAX_MGMT_OUI],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_mgmt_vendor_class_table {
    pub vendor_class: [*mut ib_mad_mgmt_vendor_class; MAX_MGMT_VENDOR_RANGE2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_mgmt_version_table {
    pub class: *mut ib_mad_mgmt_class_table,
    pub vendor: *mut ib_mad_mgmt_vendor_class_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_queue {
    pub lock: spinlock_t,
    pub list: list_head,
    pub count: c_int,
    pub max_active: c_int,
    pub qp_info: *mut ib_mad_qp_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_qp_info {
    pub port_priv: *mut ib_mad_port_private,
    pub qp: *mut ib_qp,
    pub send_queue: ib_mad_queue,
    pub recv_queue: ib_mad_queue,
    pub overflow_list: list_head,
    pub snoop_lock: spinlock_t,
    pub snoop_table: *mut ib_mad_snoop_private,
    pub snoop_table_size: c_int,
    pub snoop_count: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mad_port_private {
    pub port_list: list_head,
    pub device: *mut ib_device,
    pub port_num: c_int,
    pub cq: *mut ib_cq,
    pub pd: *mut ib_pd,
    pub reg_lock: spinlock_t,
    pub version: [ib_mad_mgmt_version_table; MAX_MGMT_VERSION],
    pub wq: *mut workqueue_struct,
    pub qp_info: [ib_mad_qp_info; IB_MAD_QPS_CORE],
}

extern "C" {
    pub fn ib_send_mad(mad_send_wr: *mut ib_mad_send_wr_private) -> c_int;
}
extern "C" {
    pub fn ib_mark_mad_done(mad_send_wr: *mut ib_mad_send_wr_private);
}
