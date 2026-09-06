//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/pci/vfio_pci_priv.h
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

// Special capability IDs predefined access
pub const PCI_CAP_ID_INVALID: c_uint = 0xFF	/* default raw access */;
pub const PCI_CAP_ID_INVALID_VIRT: c_uint = 0xFE	/* default virt access */;
// Cap maximum number of ioeventfds per device (arbitrary)
pub const VFIO_PCI_IOEVENTFD_MAX: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_pci_ioeventfd {
    pub next: list_head,
    pub vdev: *mut vfio_pci_core_device,
    pub virqfd: *mut virqfd,
    pub addr: *mut void __iomem,
    pub data: u64,
    pub pos: loff_t,
    pub bar: c_int,
    pub count: c_int,
    pub test_mem: bool,
}

extern "C" {
    pub fn vfio_pci_intx_mask(vdev: *mut vfio_pci_core_device) -> bool;
}
extern "C" {
    pub fn vfio_pci_intx_unmask(vdev: *mut vfio_pci_core_device);
}

extern "C" {
    pub fn vfio_pci_init_perm_bits() -> c_int;
}
extern "C" {
    pub fn vfio_pci_uninit_perm_bits();
}
extern "C" {
    pub fn vfio_config_init(vdev: *mut vfio_pci_core_device) -> c_int;
}
extern "C" {
    pub fn vfio_config_free(vdev: *mut vfio_pci_core_device);
}
extern "C" {
    pub fn vfio_pci_zap_and_down_write_memory_lock(vdev: *mut vfio_pci_core_device);
}
extern "C" {
    pub fn vfio_pci_memory_lock_and_enable(vdev: *mut vfio_pci_core_device) -> u16;
}

extern "C" {
    pub fn vfio_pci_is_intel_display(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn vfio_pci_igd_init(vdev: *mut vfio_pci_core_device) -> c_int;
}

extern "C" {
    pub fn vfio_pci_zdev_open_device(vdev: *mut vfio_pci_core_device) -> c_int;
}
extern "C" {
    pub fn vfio_pci_zdev_close_device(vdev: *mut vfio_pci_core_device);
}

extern "C" {
    pub fn vfio_pci_dma_buf_cleanup(vdev: *mut vfio_pci_core_device);
}
extern "C" {
    pub fn vfio_pci_dma_buf_move(vdev: *mut vfio_pci_core_device, revoked: bool);
}

