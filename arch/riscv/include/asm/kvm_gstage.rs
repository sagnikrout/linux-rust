//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/kvm_gstage.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
// Copyright (c) 2025 Ventana Micro Systems Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_gstage {
    pub kvm: *mut kvm,
    pub flags: c_ulong,

    pub vmid: c_ulong,
    pub pgd: *mut pgd_t,
    pub pgd_levels: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_gstage_mapping {
    pub addr: gpa_t,
    pub pte: pte_t,
    pub level: u32,
}

pub const kvm_riscv_gstage_index_bits: c_int = 9;

pub const kvm_riscv_gstage_index_bits: c_int = 10;

pub const kvm_riscv_gstage_pgd_xbits: c_int = 2;

extern "C" {
    pub fn BIT_ULL(_arg: kvm_riscv_gstage_gpa_bits(pgd_levels)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_riscv_gstage_op {
    GSTAGE_OP_NOP = 0,	/* Nothing */
    GSTAGE_OP_CLEAR,	/* Clear/Unmap */
    GSTAGE_OP_WP,		/* Write-protect */
}

extern "C" {
    pub fn kvm_riscv_gstage_wp_range(gstage: *mut kvm_gstage, start: gpa_t, end: gpa_t) -> bool;
}
extern "C" {
    pub fn kvm_riscv_gstage_mode_detect();
}
