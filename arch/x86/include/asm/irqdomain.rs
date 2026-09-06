//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/irqdomain.h
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

extern "C" {
    pub fn x86_fwspec_is_ioapic(fwspec: *mut irq_fwspec) -> c_int;
}
extern "C" {
    pub fn x86_fwspec_is_hpet(fwspec: *mut irq_fwspec) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ioapic_domain_type {
    IOAPIC_DOMAIN_INVALID,
    IOAPIC_DOMAIN_LEGACY,
    IOAPIC_DOMAIN_STRICT,
    IOAPIC_DOMAIN_DYNAMIC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioapic_domain_cfg {
    pub type: ioapic_domain_type,
    pub ops: *const irq_domain_ops,
    pub dev: *mut device_node,
}

extern "C" {
    pub fn mp_irqdomain_ioapic_idx(domain: *mut irq_domain) -> c_int;
}

extern "C" {
    pub fn x86_create_pci_msi_domain();
}

