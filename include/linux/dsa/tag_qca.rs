//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dsa/tag_qca.h
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

pub const QCA_HDR_LEN: c_int = 2;
pub const QCA_HDR_VERSION: c_uint = 0x2;

// Packet type for recv
pub const QCA_HDR_RECV_TYPE_NORMAL: c_uint = 0x0;
pub const QCA_HDR_RECV_TYPE_MIB: c_uint = 0x1;
pub const QCA_HDR_RECV_TYPE_RW_REG_ACK: c_uint = 0x2;

// Packet type for xmit
pub const QCA_HDR_XMIT_TYPE_NORMAL: c_uint = 0x0;
pub const QCA_HDR_XMIT_TYPE_RW_REG: c_uint = 0x1;
// Check code for a valid mgmt packet. Switch will ignore the packet
// with this wrong.
//
pub const QCA_HDR_MGMT_CHECK_CODE_VAL: c_uint = 0x5;
// Specific define for in-band MDIO read/write with Ethernet packet

// Special struct emulating a Ethernet header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca_mgmt_ethhdr {
    pub /: *mut *mut __le32 command; / command bit 31:0,
    pub /: *mut *mut __le32 seq; / seq 63:32,
    pub /: *mut *mut __le32 mdio_data; / first 4byte mdio,
    pub /: *mut *mut __be16 hdr; / qca hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdio_cmd {
    MDIO_WRITE = 0x0,
    MDIO_READ
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_ethhdr {
    pub /: *mut *mut __le32 data[3]; / first 3 mib counter,
    pub /: *mut *mut __be16 hdr; / qca hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca_tagger_data {
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,
}
