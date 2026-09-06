//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_ipv4/ipt_CLUSTERIP.h
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

// Macro flag: #define _IPT_CLUSTERIP_H_target

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clusterip_hashmode {
    CLUSTERIP_HASHMODE_SIP = 0,
    CLUSTERIP_HASHMODE_SIP_SPT,
    CLUSTERIP_HASHMODE_SIP_SPT_DPT,
}

pub const CLUSTERIP_MAX_NODES: c_int = 16;
pub const CLUSTERIP_FLAG_NEW: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_clusterip_tgt_info {
    pub flags: __u32,
// only relevant for new ones
    pub clustermac: [__u8; ETH_ALEN],
    pub num_total_nodes: __u16,
    pub num_local_nodes: __u16,
    pub local_nodes: [__u16; CLUSTERIP_MAX_NODES],
    pub hash_mode: __u32,
    pub hash_initval: __u32,
// Used internally by the kernel
    pub config: *mut clusterip_config,
}
