//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/filter.h
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
//
// Linux Socket Filter Data Structures
//

//
// Current version of the filter code architecture.
//
pub const BPF_MAJOR_VERSION: c_int = 1;
pub const BPF_MINOR_VERSION: c_int = 1;
//
// Try and keep these values and structures similar to BSD, especially
// the BPF code definitions which need to match so you can share filters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_filter {
    pub /: *mut *mut __u16 code; / Actual filter code,
    pub /: *mut *mut __u8 jt; / Jump true,
    pub /: *mut *mut __u8 jf; / Jump false,
    pub /: *mut *mut __u32 k; / Generic multiuse field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_fprog {
    pub /: *mut *mut unsigned short len; / Number of filter blocks,
    pub filter: *mut sock_filter __user,
}

// ret - BPF_K and BPF_X also apply

pub const BPF_A: c_uint = 0x10;
// misc

pub const BPF_TAX: c_uint = 0x00;
pub const BPF_TXA: c_uint = 0x80;
//
// Macros for filter block array initializers.
//

//
// Number of scratch memory words for: BPF_ST and BPF_STX
//
pub const BPF_MEMWORDS: c_int = 16;
// RATIONALE. Negative offsets are invalid in BPF.
//

pub const SKF_AD_PROTOCOL: c_int = 0;
pub const SKF_AD_PKTTYPE: c_int = 4;
pub const SKF_AD_IFINDEX: c_int = 8;
pub const SKF_AD_NLATTR: c_int = 12;
pub const SKF_AD_NLATTR_NEST: c_int = 16;
pub const SKF_AD_MARK: c_int = 20;
pub const SKF_AD_QUEUE: c_int = 24;
pub const SKF_AD_HATYPE: c_int = 28;
pub const SKF_AD_RXHASH: c_int = 32;
pub const SKF_AD_CPU: c_int = 36;
pub const SKF_AD_ALU_XOR_X: c_int = 40;
pub const SKF_AD_VLAN_TAG: c_int = 44;
pub const SKF_AD_VLAN_TAG_PRESENT: c_int = 48;
pub const SKF_AD_PAY_OFFSET: c_int = 52;
pub const SKF_AD_RANDOM: c_int = 56;
pub const SKF_AD_VLAN_TPID: c_int = 60;
pub const SKF_AD_MAX: c_int = 64;

