//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_ipv6/ip6t_REJECT.h
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
pub enum ip6t_reject_with {
    IP6T_ICMP6_NO_ROUTE,
    IP6T_ICMP6_ADM_PROHIBITED,
    IP6T_ICMP6_NOT_NEIGHBOUR,
    IP6T_ICMP6_ADDR_UNREACH,
    IP6T_ICMP6_PORT_UNREACH,
    IP6T_ICMP6_ECHOREPLY,
    IP6T_TCP_RESET,
    IP6T_ICMP6_POLICY_FAIL,
    IP6T_ICMP6_REJECT_ROUTE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6t_reject_info {
    pub /: *mut *mut __u32 with; / reject type,
}
