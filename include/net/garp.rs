//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/garp.h
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

pub const GARP_PROTOCOL_ID: c_uint = 0x1;
pub const GARP_END_MARK: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct garp_pdu_hdr {
    pub protocol: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct garp_msg_hdr {
    pub attrtype: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_attr_event {
    GARP_LEAVE_ALL,
    GARP_JOIN_EMPTY,
    GARP_JOIN_IN,
    GARP_LEAVE_EMPTY,
    GARP_LEAVE_IN,
    GARP_EMPTY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct garp_attr_hdr {
    pub len: u8,
    pub event: u8,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct garp_skb_cb {
    pub cur_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_applicant_state {
    GARP_APPLICANT_INVALID,
    GARP_APPLICANT_VA,
    GARP_APPLICANT_AA,
    GARP_APPLICANT_QA,
    GARP_APPLICANT_LA,
    GARP_APPLICANT_VP,
    GARP_APPLICANT_AP,
    GARP_APPLICANT_QP,
    GARP_APPLICANT_VO,
    GARP_APPLICANT_AO,
    GARP_APPLICANT_QO,
    __GARP_APPLICANT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_event {
    GARP_EVENT_REQ_JOIN,
    GARP_EVENT_REQ_LEAVE,
    GARP_EVENT_R_JOIN_IN,
    GARP_EVENT_R_JOIN_EMPTY,
    GARP_EVENT_R_EMPTY,
    GARP_EVENT_R_LEAVE_IN,
    GARP_EVENT_R_LEAVE_EMPTY,
    GARP_EVENT_TRANSMIT_PDU,
    __GARP_EVENT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_action {
    GARP_ACTION_NONE,
    GARP_ACTION_S_JOIN_IN,
    GARP_ACTION_S_LEAVE_EMPTY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct garp_attr {
    pub node: rb_node,
    pub state: garp_applicant_state,
    pub type: u8,
    pub dlen: u8,
    pub data: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum garp_applications {
    GARP_APPLICATION_GVRP,
    __GARP_APPLICATION_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct garp_application {
    pub type: garp_applications,
    pub maxattr: c_uint,
    pub proto: stp_proto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct garp_applicant {
    pub app: *mut garp_application,
    pub dev: *mut net_device,
    pub join_timer: timer_list,
    pub lock: spinlock_t,
    pub queue: sk_buff_head,
    pub pdu: *mut sk_buff,
    pub gid: rb_root,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct garp_port {
    pub 1]: *mut *mut garp_applicant __rcu applicants[GARP_APPLICATION_MAX +,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn garp_register_application(app: *mut garp_application) -> c_int;
}
extern "C" {
    pub fn garp_unregister_application(app: *mut garp_application);
}
extern "C" {
    pub fn garp_init_applicant(dev: *mut net_device, app: *mut garp_application) -> c_int;
}
