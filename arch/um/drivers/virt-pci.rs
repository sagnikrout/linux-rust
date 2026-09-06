//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/virt-pci.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct um_pci_device {
    pub ops: *const um_pci_ops,
// for now just standard BARs
    pub resptr: [u8; PCI_STD_NUM_BARS],
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct um_pci_ops {
    pub size): unsigned int offset, int,
    pub val): int size, unsigned long,
    pub size): unsigned int offset, int,
    pub val): unsigned int offset, int size, unsigned long,
    pub size): unsigned int offset, int,
    pub size): *const *const unsigned int offset, void buffer, int,
    pub size): unsigned int offset, u8 value, int,
}

extern "C" {
    pub fn um_pci_device_register(dev: *mut um_pci_device) -> c_int;
}
extern "C" {
    pub fn um_pci_device_unregister(dev: *mut um_pci_device);
}
extern "C" {
    pub fn um_pci_platform_device_register(dev: *mut um_pci_device) -> c_int;
}
extern "C" {
    pub fn um_pci_platform_device_unregister(dev: *mut um_pci_device);
}
