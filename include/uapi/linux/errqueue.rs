//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/errqueue.h
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

// RFC 4884: return offset to extension struct + validation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_ee_data_rfc4884 {
    pub len: __u16,
    pub flags: __u8,
    pub reserved: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_extended_err {
    pub ee_errno: __u32,
    pub ee_origin: __u8,
    pub ee_type: __u8,
    pub ee_code: __u8,
    pub ee_pad: __u8,
    pub ee_info: __u32,
    pub ee_data: __u32,
    pub ee_rfc4884: sock_ee_data_rfc4884,
}

pub const SO_EE_ORIGIN_NONE: c_int = 0;
pub const SO_EE_ORIGIN_LOCAL: c_int = 1;
pub const SO_EE_ORIGIN_ICMP: c_int = 2;
pub const SO_EE_ORIGIN_ICMP6: c_int = 3;
pub const SO_EE_ORIGIN_TXSTATUS: c_int = 4;
pub const SO_EE_ORIGIN_ZEROCOPY: c_int = 5;
pub const SO_EE_ORIGIN_TXTIME: c_int = 6;

pub const SO_EE_CODE_ZEROCOPY_COPIED: c_int = 1;
pub const SO_EE_CODE_TXTIME_INVALID_PARAM: c_int = 1;
pub const SO_EE_CODE_TXTIME_MISSED: c_int = 2;
pub const SO_EE_RFC4884_FLAG_INVALID: c_int = 1;
//
// struct scm_timestamping - timestamps exposed through cmsg
//
// The timestamping interfaces SO_TIMESTAMPING, MSG_TSTAMP_
// communicate network timestamps by passing this struct in a cmsg with
// recvmsg(). See Documentation/networking/timestamping.rst for details.
// User space sees a timespec definition that matches either
// __kernel_timespec or __kernel_old_timespec, in the kernel we
// require two structure definitions to provide both.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_timestamping {
    pub ts: [__kernel_old_timespec; 3],    pub ts: [timespec; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_timestamping64 {
    pub ts: [__kernel_timespec; 3],
}

// The type of scm_timestamping, passed in sock_extended_err ee_info.
// This defines the type of ts[0]. For SCM_TSTAMP_SND only, if ts[0]
// is zero, then this is a hardware timestamp and recorded in ts[2].
//
