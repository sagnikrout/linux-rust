//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_conntrack.h
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


// SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note
// Header file for kernel module to match connection tracking information.
// GPL (C) 2001  Marc Boucher (marc@mbsi.ca).
//

// flags, invflags:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_conntrack_mtinfo1 {
    pub origsrc_mask: nf_inet_addr origsrc_addr,,
    pub origdst_mask: nf_inet_addr origdst_addr,,
    pub replsrc_mask: nf_inet_addr replsrc_addr,,
    pub repldst_mask: nf_inet_addr repldst_addr,,
    pub expires_max: __u32 expires_min,,
    pub l4proto: __u16,
    pub origdst_port: __be16 origsrc_port,,
    pub repldst_port: __be16 replsrc_port,,
    pub invert_flags: __u16 match_flags,,
    pub status_mask: __u8 state_mask,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_conntrack_mtinfo2 {
    pub origsrc_mask: nf_inet_addr origsrc_addr,,
    pub origdst_mask: nf_inet_addr origdst_addr,,
    pub replsrc_mask: nf_inet_addr replsrc_addr,,
    pub repldst_mask: nf_inet_addr repldst_addr,,
    pub expires_max: __u32 expires_min,,
    pub l4proto: __u16,
    pub origdst_port: __be16 origsrc_port,,
    pub repldst_port: __be16 replsrc_port,,
    pub invert_flags: __u16 match_flags,,
    pub status_mask: __u16 state_mask,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_conntrack_mtinfo3 {
    pub origsrc_mask: nf_inet_addr origsrc_addr,,
    pub origdst_mask: nf_inet_addr origdst_addr,,
    pub replsrc_mask: nf_inet_addr replsrc_addr,,
    pub repldst_mask: nf_inet_addr repldst_addr,,
    pub expires_max: __u32 expires_min,,
    pub l4proto: __u16,
    pub origdst_port: __u16 origsrc_port,,
    pub repldst_port: __u16 replsrc_port,,
    pub invert_flags: __u16 match_flags,,
    pub status_mask: __u16 state_mask,,
    pub origdst_port_high: __u16 origsrc_port_high,,
    pub repldst_port_high: __u16 replsrc_port_high,,
}
