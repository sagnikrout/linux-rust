//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/irq_remapping.h
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
// Copyright (C) 2012 Advanced Micro Devices, Inc.
// Author: Joerg Roedel <joerg.roedel@amd.com>
//
// This header file contains the interface of the interrupt remapping code to
// the x86 interrupt management code.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irq_remap_cap {
    IRQ_POSTING_CAP = 0,
}

//
// This is mainly used to communicate information back-and-forth
// between SVM and IOMMU for setting up and tearing down posted
// interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_iommu_pi_data {
    pub /: *mut *mut u64 vapic_addr; / Physical address of the vCPU's vAPIC.,
    pub ga_tag: u32,
    pub /: *mut *mut u32 vector; / Guest vector of the interrupt,
    pub cpu: c_int,
    pub ga_log_intr: bool,
    pub is_guest_mode: bool,
    pub ir_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_iommu_pi_data {
    pub /: *mut *mut u64 pi_desc_addr; / Physical address of PI Descriptor,
    pub /: *mut *mut u32 vector; / Guest vector of the interrupt,
}

extern "C" {
    pub fn irq_remapping_cap(cap: irq_remap_cap) -> bool;
}
extern "C" {
    pub fn set_irq_remapping_broken();
}
extern "C" {
    pub fn irq_remapping_prepare() -> c_int;
}
extern "C" {
    pub fn irq_remapping_enable() -> c_int;
}
extern "C" {
    pub fn irq_remapping_disable();
}
extern "C" {
    pub fn irq_remapping_reenable(_arg: c_int) -> c_int;
}
extern "C" {
    pub fn irq_remap_enable_fault_handling() -> c_int;
}
extern "C" {
    pub fn panic_if_irq_remap(msg: *const c_char);
}
// Get parent irqdomain for interrupt remapping irqdomain

extern "C" {
    pub fn intel_ack_posted_msi_irq(irqd: *mut irq_data);
}

