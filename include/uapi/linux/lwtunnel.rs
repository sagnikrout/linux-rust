//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/lwtunnel.h
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
pub enum lwtunnel_encap_types {
    LWTUNNEL_ENCAP_NONE,
    LWTUNNEL_ENCAP_MPLS,
    LWTUNNEL_ENCAP_IP,
    LWTUNNEL_ENCAP_ILA,
    LWTUNNEL_ENCAP_IP6,
    LWTUNNEL_ENCAP_SEG6,
    LWTUNNEL_ENCAP_BPF,
    LWTUNNEL_ENCAP_SEG6_LOCAL,
    LWTUNNEL_ENCAP_RPL,
    LWTUNNEL_ENCAP_IOAM6,
    LWTUNNEL_ENCAP_XFRM,
    __LWTUNNEL_ENCAP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lwtunnel_ip_t {
    LWTUNNEL_IP_UNSPEC,
    LWTUNNEL_IP_ID,
    LWTUNNEL_IP_DST,
    LWTUNNEL_IP_SRC,
    LWTUNNEL_IP_TTL,
    LWTUNNEL_IP_TOS,
    LWTUNNEL_IP_FLAGS,
    LWTUNNEL_IP_PAD,
    LWTUNNEL_IP_OPTS,
    __LWTUNNEL_IP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lwtunnel_ip6_t {
    LWTUNNEL_IP6_UNSPEC,
    LWTUNNEL_IP6_ID,
    LWTUNNEL_IP6_DST,
    LWTUNNEL_IP6_SRC,
    LWTUNNEL_IP6_HOPLIMIT,
    LWTUNNEL_IP6_TC,
    LWTUNNEL_IP6_FLAGS,
    LWTUNNEL_IP6_PAD,
    LWTUNNEL_IP6_OPTS,
    __LWTUNNEL_IP6_MAX,
}

pub const LWT_BPF_MAX_HEADROOM: c_int = 256;

