//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_synproxy.h
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
pub struct synproxy_stats {
    pub syn_received: c_uint,
    pub cookie_invalid: c_uint,
    pub cookie_valid: c_uint,
    pub cookie_retrans: c_uint,
    pub conn_reopened: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synproxy_net {
    pub tmpl: *mut nf_conn,
    pub stats: *mut synproxy_stats __percpu,
    pub hook_ref4: c_uint,
    pub hook_ref6: c_uint,
}

extern "C" {
    pub fn net_generic(_arg: net, _arg: synproxy_net_id) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synproxy_options {
    pub options: u8,
    pub wscale: u8,
    pub mss_option: u16,
    pub mss_encode: u16,
    pub tsval: u32,
    pub tsecr: u32,
}

extern "C" {
    pub fn nf_synproxy_ipv4_init(snet: *mut synproxy_net, net: *mut net) -> c_int;
}
extern "C" {
    pub fn nf_synproxy_ipv4_fini(snet: *mut synproxy_net, net: *mut net);
}

extern "C" {
    pub fn nf_synproxy_ipv6_init(snet: *mut synproxy_net, net: *mut net) -> c_int;
}
extern "C" {
    pub fn nf_synproxy_ipv6_fini(snet: *mut synproxy_net, net: *mut net);
}

