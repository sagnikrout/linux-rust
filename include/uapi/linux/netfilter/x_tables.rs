//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/x_tables.h
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

pub const XT_FUNCTION_MAXNAMELEN: c_int = 30;
pub const XT_EXTENSION_MAXNAMELEN: c_int = 29;
pub const XT_TABLE_MAXNAMELEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_entry_match {
    pub match_size: __u16,
// Used by userspace
    pub name: [c_char; XT_EXTENSION_MAXNAMELEN],
    pub revision: __u8,
    pub user: },
    pub match_size: __u16,
// Used inside the kernel
    pub match: *mut xt_match,
    pub kernel: },
// Total length
    pub match_size: __u16,
    pub u: },
    pub data: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_entry_target {
    pub target_size: __u16,
// Used by userspace
    pub name: [c_char; XT_EXTENSION_MAXNAMELEN],
    pub revision: __u8,
    pub user: },
    pub target_size: __u16,
// Used inside the kernel
    pub target: *mut xt_target,
    pub kernel: },
// Total length
    pub target_size: __u16,
    pub u: },
    pub data: [c_uchar; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_standard_target {
    pub target: xt_entry_target,
    pub verdict: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_error_target {
    pub target: xt_entry_target,
    pub errorname: [c_char; XT_FUNCTION_MAXNAMELEN],
}

// The argument to IPT_SO_GET_REVISION_*.  Returns highest revision
// kernel supports, if >= revision.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_get_revision {
    pub name: [c_char; XT_EXTENSION_MAXNAMELEN],
    pub revision: __u8,
}

// CONTINUE verdict for targets
pub const XT_CONTINUE: c_uint = 0xFFFFFFFF;
// For standard target

// this is a dummy structure to find out the alignment requirement for a struct
// containing all the fundamental data types that are used in ipt_entry,
// ip6t_entry and arpt_entry.  This sucks, and it is a hack.  It will be my
// personal pleasure to remove it -HW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xt_align {
    pub u8: __u8,
    pub u16: __u16,
    pub u32: __u32,
    pub u64: __u64,
}

// Standard return verdict, or do jump.

// Error verdict.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_counters {
    pub /: *mut *mut __u64 pcnt, bcnt; / Packet and byte counters,
}

// The argument to IPT_SO_ADD_COUNTERS.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_counters_info {
// Which table.
    pub name: [c_char; XT_TABLE_MAXNAMELEN],
    pub num_counters: c_uint,
// The counters (actually `number' of these).
    pub counters: [xt_counters; ],
}

pub const XT_INV_PROTO: c_uint = 0x40	/* Invert the sense of PROTO. */;
// fn returns 0 to continue iteration

// fn returns 0 to continue iteration

// fn returns 0 to continue iteration

// pos is normally a struct ipt_entry/ip6t_entry/etc.

// can only be xt_entry_match, so no use of typeof here

