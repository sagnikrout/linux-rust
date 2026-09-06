//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/irq.h
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
// Copyright (C) 2012 Regents of the University of California
// Copyright (C) 2017 SiFive
//

extern "C" {
    pub fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: c_int);
}

extern "C" {
    pub fn riscv_set_intc_hwnode_fn((*fn)(void): *mut fwnode_handle);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum riscv_irqchip_type {
    ACPI_RISCV_IRQCHIP_INTC		= 0x00,
    ACPI_RISCV_IRQCHIP_IMSIC	= 0x01,
    ACPI_RISCV_IRQCHIP_PLIC		= 0x02,
    ACPI_RISCV_IRQCHIP_APLIC	= 0x03,
    ACPI_RISCV_IRQCHIP_SMSI		= 0x04,
}

extern "C" {
    pub fn acpi_rintc_index_to_hartid(index: u32) -> c_ulong;
}
extern "C" {
    pub fn acpi_rintc_ext_parent_to_hartid(plic_id: c_uint, ctxt_idx: c_uint) -> c_ulong;
}
extern "C" {
    pub fn acpi_rintc_get_plic_nr_contexts(plic_id: c_uint) -> c_uint;
}
extern "C" {
    pub fn acpi_rintc_get_plic_context(plic_id: c_uint, ctxt_idx: c_uint) -> c_uint;
}
extern "C" {
    pub fn acpi_rintc_get_imsic_mmio_info(index: u32, res: *mut resource) -> int __init;
}
extern "C" {
    pub fn riscv_acpi_update_gsi_range(gsi_base: u32, nr_irqs: u32) -> c_int;
}

