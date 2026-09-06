//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ip6_tunnel.h
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

pub const IPV6_TLV_TNL_ENCAP_LIMIT: c_int = 4;
pub const IPV6_DEFAULT_TNL_ENCAP_LIMIT: c_int = 4;
// don't add encapsulation limit if one isn't present in inner packet
pub const IP6_TNL_F_IGN_ENCAP_LIMIT: c_uint = 0x1;
// copy the traffic class field from the inner packet
pub const IP6_TNL_F_USE_ORIG_TCLASS: c_uint = 0x2;
// copy the flowlabel from the inner packet
pub const IP6_TNL_F_USE_ORIG_FLOWLABEL: c_uint = 0x4;
// being used for Mobile IPv6
pub const IP6_TNL_F_MIP6_DEV: c_uint = 0x8;
// copy DSCP from the outer packet
pub const IP6_TNL_F_RCV_DSCP_COPY: c_uint = 0x10;
// copy fwmark from inner packet
pub const IP6_TNL_F_USE_ORIG_FWMARK: c_uint = 0x20;
// allow remote endpoint on the local node
pub const IP6_TNL_F_ALLOW_LOCAL_REMOTE: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl_parm {
    pub /: *mut *mut char name[IFNAMSIZ]; / name of tunnel device,
    pub /: *mut *mut int link; / ifindex of underlying L2 interface,
    pub /: *mut *mut __u8 proto; / tunnel protocol,
    pub /: *mut *mut __u8 encap_limit; / encapsulation limit for tunnel,
    pub /: *mut *mut __u8 hop_limit; / hop limit for tunnel,
    pub /: *mut *mut __be32 flowinfo; / traffic class and flowlabel for tunnel,
    pub /: *mut *mut __u32 flags; / tunnel flags,
    pub /: *mut *mut in6_addr laddr; / local tunnel end-point address,
    pub /: *mut *mut in6_addr raddr; / remote tunnel end-point address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl_parm2 {
    pub /: *mut *mut char name[IFNAMSIZ]; / name of tunnel device,
    pub /: *mut *mut int link; / ifindex of underlying L2 interface,
    pub /: *mut *mut __u8 proto; / tunnel protocol,
    pub /: *mut *mut __u8 encap_limit; / encapsulation limit for tunnel,
    pub /: *mut *mut __u8 hop_limit; / hop limit for tunnel,
    pub /: *mut *mut __be32 flowinfo; / traffic class and flowlabel for tunnel,
    pub /: *mut *mut __u32 flags; / tunnel flags,
    pub /: *mut *mut in6_addr laddr; / local tunnel end-point address,
    pub /: *mut *mut in6_addr raddr; / remote tunnel end-point address,
    pub i_flags: __be16,
    pub o_flags: __be16,
    pub i_key: __be32,
    pub o_key: __be32,
}
