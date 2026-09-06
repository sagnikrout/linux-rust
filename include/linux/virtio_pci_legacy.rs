//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/virtio_pci_legacy.h
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
pub struct virtio_pci_legacy_device {
    pub pci_dev: *mut pci_dev,
// Where to read and clear interrupt
    pub isr: *mut u8 __iomem,
// The IO mapping for the PCI config space (legacy mode only)
    pub ioaddr: *mut void __iomem,
    pub id: virtio_device_id,
}

extern "C" {
    pub fn vp_legacy_get_features(ldev: *mut virtio_pci_legacy_device) -> u64;
}
extern "C" {
    pub fn vp_legacy_get_driver_features(ldev: *mut virtio_pci_legacy_device) -> u64;
}
extern "C" {
    pub fn vp_legacy_get_status(ldev: *mut virtio_pci_legacy_device) -> u8;
}
extern "C" {
    pub fn vp_legacy_probe(ldev: *mut virtio_pci_legacy_device) -> c_int;
}
extern "C" {
    pub fn vp_legacy_remove(ldev: *mut virtio_pci_legacy_device);
}
