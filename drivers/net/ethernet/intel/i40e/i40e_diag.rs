//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_diag.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

// forward-declare the HW struct for the compiler
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_lb_mode {
    I40E_LB_MODE_NONE       = 0x0,
    I40E_LB_MODE_PHY_LOCAL  = I40E_AQ_LB_PHY_LOCAL,
    I40E_LB_MODE_PHY_REMOTE = I40E_AQ_LB_PHY_REMOTE,
    I40E_LB_MODE_MAC_LOCAL  = I40E_AQ_LB_MAC_LOCAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_diag_reg_test_info {
    pub /: *mut *mut u32 offset; / the base register,
    pub /: *mut *mut u32 mask; / bits that can be tested,
    pub /: *mut *mut u32 elements; / number of elements if array,
    pub /: *mut *mut u32 stride; / bytes between each element,
}

extern "C" {
    pub fn i40e_diag_reg_test(hw: *mut i40e_hw) -> c_int;
}
extern "C" {
    pub fn i40e_diag_eeprom_test(hw: *mut i40e_hw) -> c_int;
}
