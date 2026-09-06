//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/hvcserver.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// hvcserver.h
// Copyright (C) 2004 Ryan S Arnold, IBM Corporation
//
// PPC64 virtual I/O console server support.
//

// Converged Location Code length
pub const HVCS_CLC_LENGTH: c_int = 79;
//
// hvcs_partner_info - an element in a list of partner info
// @node: list_head denoting this partner_info struct's position in the list of
// partner info.
// @unit_address: The partner unit address of this entry.
// @partition_ID: The partner partition ID of this entry.
// @location_code: The converged location code of this entry + 1 char for the
// null-term.
//
// This structure outlines the format that partner info is presented to a caller
// of the hvcs partner info fetching functions.  These are strung together into
// a list using linux kernel lists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvcs_partner_info {
    pub node: list_head,
    pub unit_address: u32,
    pub partition_ID: u32,
    pub /: *mut *mut char location_code[HVCS_CLC_LENGTH + 1]; / CLC + 1 null-term char,
}

extern "C" {
    pub fn hvcs_free_partner_info(head: *mut list_head) -> c_int;
}
extern "C" {
    pub fn hvcs_free_connection(unit_address: u32) -> c_int;
}

