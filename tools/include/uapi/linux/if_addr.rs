//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/if_addr.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifaddrmsg {
    pub ifa_family: __u8,
    pub /: *mut *mut __u8 ifa_prefixlen; / The prefix length,
    pub /: *mut *mut __u8 ifa_flags; / Flags,
    pub /: *mut *mut __u8 ifa_scope; / Address scope,
    pub /: *mut *mut __u32 ifa_index; / Link index,
}

//
// Important comment:
// IFA_ADDRESS is prefix address, rather than local interface address.
// It makes no difference for normally configured broadcast interfaces,
// but for point-to-point IFA_ADDRESS is DESTINATION address,
// local address is supplied in IFA_LOCAL attribute.
//
// IFA_FLAGS is a u32 attribute that extends the u8 field ifa_flags.
// If present, the value from struct ifaddrmsg will be ignored.
//

// ifa_flags
pub const IFA_F_SECONDARY: c_uint = 0x01;

pub const IFA_F_NODAD: c_uint = 0x02;
pub const IFA_F_OPTIMISTIC: c_uint = 0x04;
pub const IFA_F_DADFAILED: c_uint = 0x08;
pub const IFA_F_HOMEADDRESS: c_uint = 0x10;
pub const IFA_F_DEPRECATED: c_uint = 0x20;
pub const IFA_F_TENTATIVE: c_uint = 0x40;
pub const IFA_F_PERMANENT: c_uint = 0x80;
pub const IFA_F_MANAGETEMPADDR: c_uint = 0x100;
pub const IFA_F_NOPREFIXROUTE: c_uint = 0x200;
pub const IFA_F_MCAUTOJOIN: c_uint = 0x400;
pub const IFA_F_STABLE_PRIVACY: c_uint = 0x800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifa_cacheinfo {
    pub ifa_prefered: __u32,
    pub ifa_valid: __u32,
    pub /: *mut *mut __u32 cstamp; / created timestamp, hundredths of seconds,
    pub /: *mut *mut __u32 tstamp; / updated timestamp, hundredths of seconds,
}

// backwards compatibility for userspace

// ifa_proto
pub const IFAPROT_UNSPEC: c_int = 0;

