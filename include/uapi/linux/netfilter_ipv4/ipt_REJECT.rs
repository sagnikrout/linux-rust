//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_ipv4/ipt_REJECT.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipt_reject_with {
    IPT_ICMP_NET_UNREACHABLE,
    IPT_ICMP_HOST_UNREACHABLE,
    IPT_ICMP_PROT_UNREACHABLE,
    IPT_ICMP_PORT_UNREACHABLE,
    IPT_ICMP_ECHOREPLY,
    IPT_ICMP_NET_PROHIBITED,
    IPT_ICMP_HOST_PROHIBITED,
    IPT_TCP_RESET,
    IPT_ICMP_ADMIN_PROHIBITED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_reject_info {
    pub /: *mut *mut ipt_reject_with with; / reject type,
}
