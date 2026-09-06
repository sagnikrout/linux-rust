//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/enic_mbox.h
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
// Copyright 2025 Cisco Systems, Inc.  All rights reserved.

//
// Mailbox protocol for PF-VF communication over the admin channel.
//
// Even numbers are requests, odd numbers are replies/acks.
// The prefix indicates the initiator: VF_ = VF-initiated, PF_ = PF-initiated.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enic_mbox_msg_type {
    ENIC_MBOX_VF_CAPABILITY_REQUEST		= 0,
    ENIC_MBOX_VF_CAPABILITY_REPLY		= 1,
    ENIC_MBOX_VF_REGISTER_REQUEST		= 2,
    ENIC_MBOX_VF_REGISTER_REPLY		= 3,
    ENIC_MBOX_VF_UNREGISTER_REQUEST		= 4,
    ENIC_MBOX_VF_UNREGISTER_REPLY		= 5,
    ENIC_MBOX_PF_LINK_STATE_NOTIF		= 6,
    ENIC_MBOX_PF_LINK_STATE_ACK		= 7,
    ENIC_MBOX_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_mbox_hdr {
    pub src_vnic_id: __le16,
    pub dst_vnic_id: __le16,
    pub msg_type: u8,
    pub flags: u8,
    pub msg_len: __le16,
    pub msg_num: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_mbox_generic_reply {
    pub ret_major: __le16,
    pub ret_minor: __le16,
}

// ENIC_MBOX_VF_CAPABILITY_REQUEST / _REPLY
pub const ENIC_MBOX_CAP_VERSION_0: c_int = 0;
pub const ENIC_MBOX_CAP_VERSION_1: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_mbox_vf_capability_msg {
    pub version: __le32,
    pub reserved: [__le32; 32],
}

// The embedded enic_mbox_generic_reply has 2-byte alignment, but the
// __le32 members give this struct 4-byte natural alignment.  Receive
// buffers come from kmalloc (>= 8-byte aligned), so there is no
// misaligned access risk when casting from the receive buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_mbox_vf_capability_reply_msg {
    pub reply: enic_mbox_generic_reply,
    pub version: __le32,
    pub reserved: [__le32; 32],
}

// ENIC_MBOX_VF_REGISTER / _UNREGISTER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_mbox_vf_register_reply_msg {
    pub reply: enic_mbox_generic_reply,
}

// ENIC_MBOX_PF_LINK_STATE_NOTIF / _ACK
pub const ENIC_MBOX_LINK_STATE_DISABLE: c_int = 0;
pub const ENIC_MBOX_LINK_STATE_ENABLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_mbox_pf_link_state_notif_msg {
    pub link_state: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enic_mbox_pf_link_state_ack_msg {
    pub ack: enic_mbox_generic_reply,
}

pub const ENIC_MBOX_DST_PF: c_uint = 0xFFFF;
extern "C" {
    pub fn enic_mbox_init(enic: *mut enic);
}
extern "C" {
    pub fn enic_mbox_send_link_state(enic: *mut enic, vf_id: u16, link_state: u32) -> c_int;
}
extern "C" {
    pub fn enic_mbox_vf_capability_check(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_mbox_vf_register(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_mbox_vf_unregister(enic: *mut enic) -> c_int;
}
