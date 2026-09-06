//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/vhost_user.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Vhost-user protocol
// Message flags

// Feature bits
pub const VHOST_USER_F_PROTOCOL_FEATURES: c_int = 30;
// Protocol feature bits
pub const VHOST_USER_PROTOCOL_F_MQ: c_int = 0;
pub const VHOST_USER_PROTOCOL_F_REPLY_ACK: c_int = 3;
pub const VHOST_USER_PROTOCOL_F_SLAVE_REQ: c_int = 5;
pub const VHOST_USER_PROTOCOL_F_CONFIG: c_int = 9;
pub const VHOST_USER_PROTOCOL_F_INBAND_NOTIFICATIONS: c_int = 14;
// Vring state index masks
pub const VHOST_USER_VRING_INDEX_MASK: c_uint = 0xff;

// Supported version
pub const VHOST_USER_VERSION: c_int = 1;
// Supported transport features

// Supported protocol features

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vhost_user_request {
    VHOST_USER_GET_FEATURES = 1,
    VHOST_USER_SET_FEATURES = 2,
    VHOST_USER_SET_OWNER = 3,
    VHOST_USER_RESET_OWNER = 4,
    VHOST_USER_SET_MEM_TABLE = 5,
    VHOST_USER_SET_LOG_BASE = 6,
    VHOST_USER_SET_LOG_FD = 7,
    VHOST_USER_SET_VRING_NUM = 8,
    VHOST_USER_SET_VRING_ADDR = 9,
    VHOST_USER_SET_VRING_BASE = 10,
    VHOST_USER_GET_VRING_BASE = 11,
    VHOST_USER_SET_VRING_KICK = 12,
    VHOST_USER_SET_VRING_CALL = 13,
    VHOST_USER_SET_VRING_ERR = 14,
    VHOST_USER_GET_PROTOCOL_FEATURES = 15,
    VHOST_USER_SET_PROTOCOL_FEATURES = 16,
    VHOST_USER_GET_QUEUE_NUM = 17,
    VHOST_USER_SET_VRING_ENABLE = 18,
    VHOST_USER_SEND_RARP = 19,
    VHOST_USER_NET_SEND_MTU = 20,
    VHOST_USER_SET_SLAVE_REQ_FD = 21,
    VHOST_USER_IOTLB_MSG = 22,
    VHOST_USER_SET_VRING_ENDIAN = 23,
    VHOST_USER_GET_CONFIG = 24,
    VHOST_USER_SET_CONFIG = 25,
    VHOST_USER_VRING_KICK = 35,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vhost_user_slave_request {
    VHOST_USER_SLAVE_IOTLB_MSG = 1,
    VHOST_USER_SLAVE_CONFIG_CHANGE_MSG = 2,
    VHOST_USER_SLAVE_VRING_HOST_NOTIFIER_MSG = 3,
    VHOST_USER_SLAVE_VRING_CALL = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_user_header {
//
// Use enum vhost_user_request for outgoing messages,
// uses enum vhost_user_slave_request for incoming ones.
//
    pub request: u32,
    pub flags: u32,
    pub size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_user_config {
    pub offset: u32,
    pub size: u32,
    pub flags: u32,
    pub /: *mut *mut u8 payload[]; / Variable length,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_user_vring_state {
    pub index: u32,
    pub num: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_user_vring_addr {
    pub index: u32,
    pub flags: u32,
    pub log: u64 desc, used, avail,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_user_mem_region {
    pub guest_addr: u64,
    pub size: u64,
    pub user_addr: u64,
    pub mmap_offset: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_user_mem_regions {
    pub num: u32,
    pub padding: u32,
    pub /: *mut *mut vhost_user_mem_region regions[2]; / Currently supporting 2,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union vhost_user_payload {
    pub integer: u64,
    pub config: vhost_user_config,
    pub vring_state: vhost_user_vring_state,
    pub vring_addr: vhost_user_vring_addr,
    pub mem_regions: vhost_user_mem_regions,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_user_msg {
    pub header: vhost_user_header,
    pub payload: vhost_user_payload,
    pub __packed: },
