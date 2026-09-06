//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_bridge/ebt_stp.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const EBT_STP_TYPE: c_uint = 0x0001;
pub const EBT_STP_FLAGS: c_uint = 0x0002;
pub const EBT_STP_ROOTPRIO: c_uint = 0x0004;
pub const EBT_STP_ROOTADDR: c_uint = 0x0008;
pub const EBT_STP_ROOTCOST: c_uint = 0x0010;
pub const EBT_STP_SENDERPRIO: c_uint = 0x0020;
pub const EBT_STP_SENDERADDR: c_uint = 0x0040;
pub const EBT_STP_PORT: c_uint = 0x0080;
pub const EBT_STP_MSGAGE: c_uint = 0x0100;
pub const EBT_STP_MAXAGE: c_uint = 0x0200;
pub const EBT_STP_HELLOTIME: c_uint = 0x0400;
pub const EBT_STP_FWDD: c_uint = 0x0800;
pub const EBT_STP_MASK: c_uint = 0x0fff;
pub const EBT_STP_CONFIG_MASK: c_uint = 0x0ffe;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_stp_config_info {
    pub flags: __u8,
    pub root_priou: __u16 root_priol,,
    pub root_addrmsk: [char root_addr[6],; 6],
    pub root_costu: __u32 root_costl,,
    pub sender_priou: __u16 sender_priol,,
    pub sender_addrmsk: [char sender_addr[6],; 6],
    pub portu: __u16 portl,,
    pub msg_ageu: __u16 msg_agel,,
    pub max_ageu: __u16 max_agel,,
    pub hello_timeu: __u16 hello_timel,,
    pub forward_delayu: __u16 forward_delayl,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_stp_info {
    pub type: __u8,
    pub config: ebt_stp_config_info,
    pub bitmask: __u16,
    pub invflags: __u16,
}
