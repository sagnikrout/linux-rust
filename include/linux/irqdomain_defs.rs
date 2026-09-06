//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqdomain_defs.h
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
//
// Should several domains have the same device node, but serve
// different purposes (for example one domain is for PCI/MSI, and the
// other for wired IRQs), they can be distinguished using a
// bus-specific token. Most domains are expected to only carry
// DOMAIN_BUS_ANY.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irq_domain_bus_token {
    DOMAIN_BUS_ANY		= 0,
    DOMAIN_BUS_WIRED,
    DOMAIN_BUS_GENERIC_MSI,
    DOMAIN_BUS_PCI_MSI,
    DOMAIN_BUS_PLATFORM_MSI,
    DOMAIN_BUS_NEXUS,
    DOMAIN_BUS_IPI,
    DOMAIN_BUS_TI_SCI_INTA_MSI,
    DOMAIN_BUS_WAKEUP,
    DOMAIN_BUS_VMD_MSI,
    DOMAIN_BUS_PCI_DEVICE_MSI,
    DOMAIN_BUS_PCI_DEVICE_MSIX,
    DOMAIN_BUS_DMAR,
    DOMAIN_BUS_AMDVI,
    DOMAIN_BUS_DEVICE_MSI,
    DOMAIN_BUS_WIRED_TO_MSI,
}
