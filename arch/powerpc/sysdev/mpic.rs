//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/sysdev/mpic.h
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
// Copyright 2006-2007, Michael Ellerman, IBM Corporation.
//

extern "C" {
    pub fn mpic_msi_reserve_hwirq(mpic: *mut mpic, hwirq: irq_hw_number_t);
}
extern "C" {
    pub fn mpic_msi_init_allocator(mpic: *mut mpic) -> int __init;
}
extern "C" {
    pub fn mpic_u3msi_init(mpic: *mut mpic) -> int __init;
}

extern "C" {
    pub fn mpic_pasemi_msi_init(mpic: *mut mpic) -> int __init;
}

extern "C" {
    pub fn mpic_set_irq_type(d: *mut irq_data, flow_type: c_uint) -> c_int;
}
extern "C" {
    pub fn mpic_set_vector(virq: c_uint, vector: c_uint);
}
extern "C" {
    pub fn mpic_reset_core(cpu: c_int);
}

extern "C" {
    pub fn mpic_map_error_int(mpic: *mut mpic, virq: c_uint, hw: irq_hw_number_t) -> c_int;
}
extern "C" {
    pub fn mpic_err_int_init(mpic: *mut mpic, irqnum: irq_hw_number_t) -> void __init;
}
extern "C" {
    pub fn mpic_setup_error_int(mpic: *mut mpic, intvec: c_int) -> int __init;
}

