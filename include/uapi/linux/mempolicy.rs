//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mempolicy.h
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
// NUMA memory policies for Linux.
// Copyright 2003,2004 Andi Kleen SuSE Labs
//

//
// Both the MPOL_* mempolicy mode and the MPOL_F_* optional mode flags are
// passed by the user to either set_mempolicy() or mbind() in an 'int' actual.
// The MPOL_MODE_FLAGS macro determines the legal set of optional mode flags.
//
// Policies
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mempolicy_mode {
    MPOL_DEFAULT,
    MPOL_PREFERRED,
    MPOL_BIND,
    MPOL_INTERLEAVE,
    MPOL_LOCAL,
    MPOL_PREFERRED_MANY,
    MPOL_WEIGHTED_INTERLEAVE,
    MPOL_MAX,	/* always last member of enum */
}

// Flags for set_mempolicy

//
// MPOL_MODE_FLAGS is the union of all possible optional mode flags passed to
// either set_mempolicy() or mbind().
//

// Whether the nodemask is specified by users

// Flags for get_mempolicy

// Flags for mbind

//
// Internal flags that share the struct mempolicy flags word with
// "mode flags".  These flags are allocated from bit 0 up, as they
// are never OR'ed into the mode in mempolicy API arguments.
//

//
// Enabling zone reclaim means the page allocator will attempt to fulfill
// the allocation request on the current node by triggering reclaim and
// trying to shrink the current node.
// Fallback allocations on the next candidates in the zonelist are considered
// when reclaim fails to free up enough memory in the current node/zone.
//
// These bit locations are exposed in the vm.zone_reclaim_mode sysctl.
// New bits are OK, but existing bits should not be changed.
//

