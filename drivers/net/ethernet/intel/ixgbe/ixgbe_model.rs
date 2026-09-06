//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_model.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mat_field {
    pub off: c_uint,
    pub m): u32 val, u32,
    pub type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_jump_table {
    pub mat: *mut ixgbe_mat_field,
    pub input: *mut ixgbe_fdir_filter,
    pub mask: *mut ixgbe_atr_input,
    pub link_hdl: u32,
    pub child_loc_map: [c_ulong; 32],
}

pub const IXGBE_MAX_HW_ENTRIES: c_int = 2045;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_nexthdr {
// offset, shift, and mask of position to next header
    pub o: c_uint,
    pub s: u32,
    pub m: u32,
// match criteria to make this jump
    pub off: c_uint,
    pub val: u32,
    pub mask: u32,
// location of jump to make
    pub jump: *mut ixgbe_mat_field,
}
