//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_tcpudp.h
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

// TCP matching stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_tcp {
    pub /: *mut *mut __u16 spts[2]; / Source port range.,
    pub /: *mut *mut __u16 dpts[2]; / Destination port range.,
    pub non-zero*/: *mut *mut __u8 option; / TCP Option iff,
    pub /: *mut *mut __u8 flg_mask; / TCP flags mask byte,
    pub /: *mut *mut __u8 flg_cmp; / TCP flags compare byte,
    pub /: *mut *mut __u8 invflags; / Inverse flags,
}

// Values for "inv" field in struct ipt_tcp.
pub const XT_TCP_INV_SRCPT: c_uint = 0x01	/* Invert the sense of source ports. */;
pub const XT_TCP_INV_DSTPT: c_uint = 0x02	/* Invert the sense of dest ports. */;
pub const XT_TCP_INV_FLAGS: c_uint = 0x04	/* Invert the sense of TCP flags. */;
pub const XT_TCP_INV_OPTION: c_uint = 0x08	/* Invert the sense of option test. */;
pub const XT_TCP_INV_MASK: c_uint = 0x0F	/* All possible flags. */;
// UDP matching stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_udp {
    pub /: *mut *mut __u16 spts[2]; / Source port range.,
    pub /: *mut *mut __u16 dpts[2]; / Destination port range.,
    pub /: *mut *mut __u8 invflags; / Inverse flags,
}

// Values for "invflags" field in struct ipt_udp.
pub const XT_UDP_INV_SRCPT: c_uint = 0x01	/* Invert the sense of source ports. */;
pub const XT_UDP_INV_DSTPT: c_uint = 0x02	/* Invert the sense of dest ports. */;
pub const XT_UDP_INV_MASK: c_uint = 0x03	/* All possible flags. */;
