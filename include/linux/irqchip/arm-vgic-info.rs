//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/arm-vgic-info.h
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
// include/linux/irqchip/arm-vgic-info.h
//
// Copyright (C) 2016 ARM Limited, All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gic_type {
// Full GICv2
    GIC_V2,
// Full GICv3, optionally with v2 compat
    GIC_V3,
// Full GICv5, optionally with v3 compat
    GIC_V5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gic_kvm_info {
// GIC type
    pub type: gic_type,
// Virtual CPU interface
    pub vcpu: resource,
// GICv2 GICC VA
    pub gicc_base: *mut void __iomem,
// Interrupt number
    pub maint_irq: c_uint,
// No interrupt mask, no need to use the above field
    pub no_maint_irq_mask: bool,
// Virtual control interface
    pub vctrl: resource,
// vlpi support
    pub has_v4: bool,
// rvpeid support
    pub has_v4_1: bool,
// Deactivation impared, subpar stuff
    pub no_hw_deactivation: bool,
}

extern "C" {
    pub fn vgic_set_kvm_info(info: *const gic_kvm_info);
}

