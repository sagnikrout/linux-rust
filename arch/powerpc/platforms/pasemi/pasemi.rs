//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/pasemi/pasemi.h
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
    pub fn pas_get_boot_time() -> time64_t;
}
extern "C" {
    pub fn pas_pci_init();
}
extern "C" {
    pub fn pas_pci_dma_dev_setup(dev: *mut pci_dev);
}
extern "C" {
    pub fn pasemi_pci_getcfgaddr(dev: *mut pci_dev, offset: c_int) -> *mut void __iomem __init;
}
extern "C" {
    pub fn pasemi_map_registers() -> void __init;
}
// Power savings modes, implemented in asm
extern "C" {
    pub fn idle_spin();
}
extern "C" {
    pub fn idle_doze();
}
// Restore astate to last set

extern "C" {
    pub fn check_astate() -> c_int;
}
extern "C" {
    pub fn restore_astate(cpu: c_int);
}

// Always return >0 so we never power save

