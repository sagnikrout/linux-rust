//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/dtc/libfdt/libfdt_internal.h
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//
// libfdt - Flat Device Tree manipulation
// Copyright (C) 2006 David Gibson, IBM Corporation.
//

extern "C" {
    pub fn fdt_ro_probe_(fdt: *const c_void) -> i32;
}

extern "C" {
    pub fn fdt_check_node_offset_(fdt: *const c_void, offset: c_int) -> c_int;
}
extern "C" {
    pub fn fdt_check_prop_offset_(fdt: *const c_void, offset: c_int) -> c_int;
}
extern "C" {
    pub fn fdt_find_string_len_(_arg: strtab, _arg: tabsize, _arg: s, _arg: strlen(s)) -> return;
}
extern "C" {
    pub fn fdt_node_end_offset_(fdt: *mut c_void, nodeoffset: c_int) -> c_int;
}
//
// Internal helpers to access structural elements of the device tree
// blob (rather than for example reading integers from within property
// values).  We assume that we are either given a naturally aligned
// address for the platform or if we are not, we are on a platform
// where unaligned memory reads will be handled in a graceful manner.
// If not the external helpers fdtXX_ld() from libfdt.h can be used
// instead.
//
extern "C" {
    pub fn fdt32_to_cpu(_arg: *mut p) -> return;
}
extern "C" {
    pub fn fdt64_to_cpu(_arg: *mut p) -> return;
}

//
// Checking controls
//

pub const FDT_ASSUME_MASK: c_int = 0;

//
// Defines assumptions which can be enabled. Each of these can be enabled
// individually. For maximum safety, don't enable any assumptions!
//
// For minimal code size and no safety, use ASSUME_PERFECT at your own risk.
// You should have another method of validating the device tree, such as a
// signature or hash check before using libfdt.
//
// For situations where security is not a concern it may be safe to enable
// ASSUME_PERFECT.
//
// This does essentially no checks. Only the latest device-tree
// version is correctly handled. Inconsistencies or errors in the device
// tree may cause undefined behaviour or crashes. Invalid parameters
// passed to libfdt may do the same.
//
// If an error occurs when modifying the tree it may leave the tree in
// an intermediate (but valid) state. As an example, adding a property
// where there is insufficient space may result in the property name
// being added to the string table even though the property itself is
// not added to the struct section.
//
// Only use this if you have a fully validated device tree with
// the latest supported version and wish to minimise code size.
//
// This assumes that the device tree is sane. i.e. header metadata
// and basic hierarchy are correct.
//
// With this assumption enabled, normal device trees produced by libfdt
// and the compiler should be handled safely. Malicious device trees and
// complete garbage may cause libfdt to behave badly or crash. Truncated
// device trees (e.g. those only partially loaded) can also cause
// problems.
//
// Note: Only checks that relate exclusively to the device tree itself
// (not the parameters passed to libfdt) are disabled by this
// assumption. This includes checking headers, tags and the like.
//
// This builds on ASSUME_VALID_DTB and further assumes that libfdt
// functions are called with valid parameters, i.e. not trigger
// FDT_ERR_BADOFFSET or offsets that are out of bounds. It disables any
// extensive checking of parameters and the device tree, making various
// assumptions about correctness.
//
// It doesn't make sense to enable this assumption unless
// ASSUME_VALID_DTB is also enabled.
//
// This disables checks for device-tree version and removes all code
// which handles older versions.
//
// Only enable this if you know you have a device tree with the latest
// version.
//
// This assumes that it is OK for a failed addition to the device tree,
// due to lack of space or some other problem, to skip any rollback
// steps (such as dropping the property name from the string table).
// This is safe to enable in most circumstances, even though it may
// leave the tree in a sub-optimal state.
//
// This assumes that the device tree components appear in a 'convenient'
// order, i.e. the memory reservation block first, then the structure
// block and finally the string block.
//
// This order is not specified by the device-tree specification,
// but is expected by libfdt. The device-tree compiler always created
// device trees with this order.
//
// This assumption disables a check in fdt_open_into() and removes the
// ability to fix the problem there. This is safe if you know that the
// device tree is correctly ordered. See fdt_blocks_misordered_().
//
// This assumes that libfdt itself does not have any internal bugs. It
// drops certain checks that should never be needed unless libfdt has an
// undiscovered bug.
//
// This can generally be considered safe to enable.
//
// can_assume_() - check if a particular assumption is enabled
//
// @mask: Mask to check (ASSUME_...)
// @return true if that assumption is enabled, else false
//
// helper macros for checking assumptions

