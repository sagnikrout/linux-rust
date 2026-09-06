//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/mrp.h
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

pub const MRP_END_MARK: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrp_pdu_hdr {
    pub version: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrp_msg_hdr {
    pub attrtype: u8,
    pub attrlen: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrp_vecattr_hdr {
    pub lenflags: __be16,
    pub firstattrvalue: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_vecattr_event {
    MRP_VECATTR_EVENT_NEW,
    MRP_VECATTR_EVENT_JOIN_IN,
    MRP_VECATTR_EVENT_IN,
    MRP_VECATTR_EVENT_JOIN_MT,
    MRP_VECATTR_EVENT_MT,
    MRP_VECATTR_EVENT_LV,
    __MRP_VECATTR_EVENT_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrp_skb_cb {
    pub mh: *mut mrp_msg_hdr,
    pub vah: *mut mrp_vecattr_hdr,
    pub attrvalue: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_applicant_state {
    MRP_APPLICANT_INVALID,
    MRP_APPLICANT_VO,
    MRP_APPLICANT_VP,
    MRP_APPLICANT_VN,
    MRP_APPLICANT_AN,
    MRP_APPLICANT_AA,
    MRP_APPLICANT_QA,
    MRP_APPLICANT_LA,
    MRP_APPLICANT_AO,
    MRP_APPLICANT_QO,
    MRP_APPLICANT_AP,
    MRP_APPLICANT_QP,
    __MRP_APPLICANT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_event {
    MRP_EVENT_NEW,
    MRP_EVENT_JOIN,
    MRP_EVENT_LV,
    MRP_EVENT_TX,
    MRP_EVENT_R_NEW,
    MRP_EVENT_R_JOIN_IN,
    MRP_EVENT_R_IN,
    MRP_EVENT_R_JOIN_MT,
    MRP_EVENT_R_MT,
    MRP_EVENT_R_LV,
    MRP_EVENT_R_LA,
    MRP_EVENT_REDECLARE,
    MRP_EVENT_PERIODIC,
    __MRP_EVENT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_tx_action {
    MRP_TX_ACTION_NONE,
    MRP_TX_ACTION_S_NEW,
    MRP_TX_ACTION_S_JOIN_IN,
    MRP_TX_ACTION_S_JOIN_IN_OPTIONAL,
    MRP_TX_ACTION_S_IN_OPTIONAL,
    MRP_TX_ACTION_S_LV,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrp_attr {
    pub node: rb_node,
    pub state: mrp_applicant_state,
    pub type: u8,
    pub len: u8,
    pub value: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mrp_applications {
    MRP_APPLICATION_MVRP,
    __MRP_APPLICATION_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrp_application {
    pub type: mrp_applications,
    pub maxattr: c_uint,
    pub pkttype: packet_type,
    pub group_address: [c_uchar; ETH_ALEN],
    pub version: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrp_applicant {
    pub app: *mut mrp_application,
    pub dev: *mut net_device,
    pub join_timer: timer_list,
    pub periodic_timer: timer_list,
    pub lock: spinlock_t,
    pub queue: sk_buff_head,
    pub pdu: *mut sk_buff,
    pub mad: rb_root,
    pub rcu: rcu_head,
    pub active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrp_port {
    pub 1]: *mut *mut mrp_applicant __rcu applicants[MRP_APPLICATION_MAX +,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn mrp_register_application(app: *mut mrp_application) -> c_int;
}
extern "C" {
    pub fn mrp_unregister_application(app: *mut mrp_application);
}
extern "C" {
    pub fn mrp_init_applicant(dev: *mut net_device, app: *mut mrp_application) -> c_int;
}
extern "C" {
    pub fn mrp_uninit_applicant(dev: *mut net_device, app: *mut mrp_application);
}
