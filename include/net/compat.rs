//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/compat.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_msghdr {
    pub /: *mut *mut *mut compat_uptr_t msg_name; / void,
    pub msg_namelen: compat_int_t,
    pub /: *mut *mut *mut compat_uptr_t msg_iov; / struct compat_iovec,
    pub msg_iovlen: compat_size_t,
    pub /: *mut *mut *mut compat_uptr_t msg_control; / void,
    pub msg_controllen: compat_size_t,
    pub msg_flags: compat_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_mmsghdr {
    pub msg_hdr: compat_msghdr,
    pub msg_len: compat_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_cmsghdr {
    pub cmsg_len: compat_size_t,
    pub cmsg_level: compat_int_t,
    pub cmsg_type: compat_int_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_rtentry {
    pub rt_pad1: u32,
    pub /: *mut *mut sockaddr rt_dst; / target address,
    pub /: *mut *mut sockaddr rt_gateway; / gateway addr (RTF_GATEWAY),
    pub /: *mut *mut sockaddr rt_genmask; / target network mask (IP),
    pub rt_flags: c_ushort,
    pub rt_pad2: c_short,
    pub rt_pad3: u32,
    pub rt_tos: c_uchar,
    pub rt_class: c_uchar,
    pub rt_pad4: c_short,
    pub /: *mut *mut short rt_metric; / +1 for binary compatibility!,
    pub /: *mut *mut compat_uptr_t rt_dev; / forcing the device at add,
    pub /: *mut *mut u32 rt_mtu; / per route MTU/Window,
    pub /: *mut *mut u32 rt_window; / Window clamping,
    pub /: *mut *mut unsigned short rt_irtt; / Initial RTT,
}

extern "C" {
    pub fn put_cmsg_compat(msghdr*: *mut struct, _arg: c_int, _arg: c_int, _arg: c_int, : *mut c_void) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_group_req {
    pub gr_interface: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_group_source_req {
    pub gsr_interface: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_group_filter {
    pub gf_interface_aux: __u32,
    pub gf_fmode_aux: __u32,
    pub gf_numsrc_aux: __u32,
    pub __packed: },
    pub gf_interface: __u32,
    pub gf_fmode: __u32,
    pub gf_numsrc: __u32,
    pub __packed: },
}
