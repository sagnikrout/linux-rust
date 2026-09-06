//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_tunnel.h
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
pub struct ip_tunnel_parm {
    pub name: [c_char; IFNAMSIZ],
    pub link: c_int,
    pub i_flags: __be16,
    pub o_flags: __be16,
    pub i_key: __be32,
    pub o_key: __be32,
    pub iph: iphdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tunnel_encap_types {
    TUNNEL_ENCAP_NONE,
    TUNNEL_ENCAP_FOU,
    TUNNEL_ENCAP_GUE,
    TUNNEL_ENCAP_MPLS,
}

// SIT-mode i_flags
pub const SIT_ISATAP: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_prl {
    pub addr: __be32,
    pub flags: __u16,
    pub __reserved: __u16,
    pub datalen: __u32,
    pub __reserved2: __u32,
// data follows
}

// PRL flags
pub const PRL_DEFAULT: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_6rd {
    pub prefix: in6_addr,
    pub relay_prefix: __be32,
    pub prefixlen: __u16,
    pub relay_prefixlen: __u16,
}

// VTI-mode i_flags

// Historically, tunnel flags have been defined as __be16 and now there are
// no free bits left. It is strongly advised to switch the already existing
// userspace code to the new *_BIT definitions from down below, as __be16
// can't be simply cast to a wider type on LE systems. All new flags and
// code must use *_BIT only.
//

// Flags starting from here are not available via the old UAPI
