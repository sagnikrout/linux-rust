//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/irqchip/irq-riscv-aplic-main.h
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
// Copyright (C) 2021 Western Digital Corporation or its affiliates.
// Copyright (C) 2022 Ventana Micro Systems Inc.
//

pub const APLIC_DEFAULT_PRIORITY: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aplic_msicfg {
    pub base_ppn: phys_addr_t,
    pub hhxs: u32,
    pub hhxw: u32,
    pub lhxs: u32,
    pub lhxw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aplic_src_ctrl {
    pub sourcecfg: u32,
    pub target: u32,
    pub ie: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aplic_saved_regs {
    pub domaincfg: u32,

    pub msiaddr: u32,
    pub msiaddrh: u32,

    pub srcs: *mut aplic_src_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aplic_priv {
    pub head: list_head,
    pub genpd_nb: notifier_block,
    pub saved_hw_regs: aplic_saved_regs,
    pub dev: *mut device,
    pub gsi_base: u32,
    pub nr_irqs: u32,
    pub nr_idcs: u32,
    pub acpi_aplic_id: u32,
    pub regs: *mut void __iomem,
    pub msicfg: aplic_msicfg,
}

extern "C" {
    pub fn aplic_irq_unmask(d: *mut irq_data);
}
extern "C" {
    pub fn aplic_irq_mask(d: *mut irq_data);
}
extern "C" {
    pub fn aplic_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int;
}
extern "C" {
    pub fn aplic_init_hw_global(priv: *mut aplic_priv, msi_mode: bool);
}
extern "C" {
    pub fn aplic_setup_priv(priv: *mut aplic_priv, dev: *mut device, regs: *mut void __iomem) -> c_int;
}
extern "C" {
    pub fn aplic_direct_restore_states(priv: *mut aplic_priv);
}
extern "C" {
    pub fn aplic_direct_setup(dev: *mut device, regs: *mut void __iomem) -> c_int;
}

extern "C" {
    pub fn aplic_msi_setup(dev: *mut device, regs: *mut void __iomem) -> c_int;
}

