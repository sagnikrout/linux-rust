//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/mm/pf_in.h
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
// Fault Injection Test harness (FI)
// Copyright (C) Intel Crop.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reason_type {
    NOT_ME,	/* page fault is not in regions */
    NOTHING,	/* access others point in regions */
    REG_READ,	/* read from addr to reg */
    REG_WRITE,	/* write from reg to addr */
    IMM_WRITE,	/* write from imm to addr */
    OTHERS	/* Other instructions can not intercept */
}

extern "C" {
    pub fn get_ins_type(ins_addr: c_ulong) -> reason_type;
}
extern "C" {
    pub fn get_ins_mem_width(ins_addr: c_ulong) -> c_uint;
}
extern "C" {
    pub fn get_ins_reg_val(ins_addr: c_ulong, regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn get_ins_imm_val(ins_addr: c_ulong) -> c_ulong;
}
