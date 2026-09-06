//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_bridge.h
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
// bridge-specific defines for netfilter.
//

// Bridge Hooks
// After promisc drops, checksum checks.
pub const NF_BR_PRE_ROUTING: c_int = 0;
// If the packet is destined for this box.
pub const NF_BR_LOCAL_IN: c_int = 1;
// If the packet is destined for another interface.
pub const NF_BR_FORWARD: c_int = 2;
// Packets coming from a local process.
pub const NF_BR_LOCAL_OUT: c_int = 3;
// Packets about to hit the wire.
pub const NF_BR_POST_ROUTING: c_int = 4;
// Not really a hook, but used for the ebtables broute table
pub const NF_BR_BROUTING: c_int = 5;
pub const NF_BR_NUMHOOKS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_br_hook_priorities {
    NF_BR_PRI_FIRST = __KERNEL_INT_MIN,
    NF_BR_PRI_NAT_DST_BRIDGED = -300,
    NF_BR_PRI_FILTER_BRIDGED = -200,
    NF_BR_PRI_BRNF = 0,
    NF_BR_PRI_NAT_DST_OTHER = 100,
    NF_BR_PRI_FILTER_OTHER = 200,
    NF_BR_PRI_NAT_SRC = 300,
    NF_BR_PRI_LAST = __KERNEL_INT_MAX,
}
