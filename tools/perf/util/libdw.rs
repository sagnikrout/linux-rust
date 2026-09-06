//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/libdw.h
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

//
// libdw__addr2line - Convert address to source location using libdw
// @addr: Address to resolve
// @file: Pointer to return filename (caller must free)
// @line_nr: Pointer to return line number
// @dso: The dso struct
// @unwind_inlines: Whether to unwind inline function calls
// @node: Inline node list to append to
// @sym: The symbol associated with the address
//
// This function initializes a Dwfl context for the DSO if not already present,
// finds the source line information for the given address, and optionally
// resolves inline function call chains.
//
// Returns 1 on success (found), 0 on failure (not found).
//
// dso__free_libdw - Free libdw resources associated with the DSO
// @dso: The dso to free resources for
//
// This function cleans up the Dwfl context used for addr2line lookups.
//
extern "C" {
    pub fn dso__free_libdw(dso: *mut dso);
}

