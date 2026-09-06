//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/virtio_pci_modern.h
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
// struct virtio_pci_modern_device - info for modern PCI virtio
// @pci_dev:	    Ptr to the PCI device struct
// @common:	    Position of the common capability in the PCI config
// @device:	    Device-specific data (non-legacy mode)
// @notify_base:    Base of vq notifications (non-legacy mode)
// @notify_pa:	    Physical base of vq notifications
// @isr:	    Where to read and clear interrupt
// @notify_len:	    So we can sanity-check accesses
// @device_len:	    So we can sanity-check accesses
// @notify_map_cap: Capability for when we need to map notifications per-vq
// @notify_offset_multiplier: Multiply queue_notify_off by this value
// (non-legacy mode).
// @modern_bars:    Bitmask of BARs
// @id:		    Device and vendor id
// @device_id_check: Callback defined before vp_modern_probe() to be used to
// verify the PCI device is a vendor's expected device rather
// than the standard virtio PCI device
// Returns the found device id or ERRNO
// @dma_mask:	    Optional mask instead of the traditional DMA_BIT_MASK(64),
// for vendor devices with DMA space address limitations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_modern_device {
    pub pci_dev: *mut pci_dev,
    pub common: *mut virtio_pci_common_cfg __iomem,
    pub device: *mut void __iomem,
    pub notify_base: *mut void __iomem,
    pub notify_pa: resource_size_t,
    pub isr: *mut u8 __iomem,
    pub notify_len: usize,
    pub device_len: usize,
    pub common_len: usize,
    pub notify_map_cap: c_int,
    pub notify_offset_multiplier: u32,
    pub modern_bars: c_int,
    pub id: virtio_device_id,
    pub pdev): *mut *mut int (device_id_check)(struct pci_dev,
    pub dma_mask: u64,
}

//
// Type-safe wrappers for io accesses.
// Use these to enforce at compile time the following spec requirement:
//
// The driver MUST access each field using the “natural” access
// method, i.e. 32-bit accesses for 32-bit fields, 16-bit accesses
// for 16-bit fields and 8-bit accesses for 8-bit fields.
//
extern "C" {
    pub fn ioread8(_arg: addr) -> return;
}
extern "C" {
    pub fn ioread16(_arg: addr) -> return;
}
extern "C" {
    pub fn ioread32(_arg: addr) -> return;
}
extern "C" {
    pub fn vp_modern_generation(mdev: *mut virtio_pci_modern_device) -> u32;
}
extern "C" {
    pub fn vp_modern_get_status(mdev: *mut virtio_pci_modern_device) -> u8;
}
extern "C" {
    pub fn vp_modern_get_num_queues(mdev: *mut virtio_pci_modern_device) -> u16;
}
extern "C" {
    pub fn vp_modern_probe(mdev: *mut virtio_pci_modern_device) -> c_int;
}
extern "C" {
    pub fn vp_modern_remove(mdev: *mut virtio_pci_modern_device);
}
extern "C" {
    pub fn vp_modern_get_queue_reset(mdev: *mut virtio_pci_modern_device, index: u16) -> c_int;
}
extern "C" {
    pub fn vp_modern_set_queue_reset(mdev: *mut virtio_pci_modern_device, index: u16);
}
extern "C" {
    pub fn vp_modern_avq_num(mdev: *mut virtio_pci_modern_device) -> u16;
}
extern "C" {
    pub fn vp_modern_avq_index(mdev: *mut virtio_pci_modern_device) -> u16;
}
