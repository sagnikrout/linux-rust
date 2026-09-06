//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/xen/gntdev-common.h
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
// Common functionality of grant device.
//
// Copyright (c) 2006-2007, D G Murray.
// (c) 2009 Gerd Hoffmann <kraxel@redhat.com>
// (c) 2018 Oleksandr Andrushchenko, EPAM Systems Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gntdev_priv {
// Maps with visible offsets in the file descriptor.
    pub maps: list_head,
// lock protects maps and freeable_maps.
    pub lock: mutex,
// Free instances of struct gntdev_copy_batch.
    pub batch: *mut gntdev_copy_batch,
    pub batch_lock: mutex,

// Device for which DMA memory is allocated.
    pub dma_dev: *mut device,

    pub dmabuf_priv: *mut gntdev_dmabuf_priv,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gntdev_unmap_notify {
    pub flags: c_int,
// Address relative to the start of the gntdev_grant_map.
    pub addr: c_int,
    pub event: evtchn_port_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gntdev_grant_map {
    pub in_use: core::sync::atomic::AtomicI32,
    pub notifier: mmu_interval_notifier,
    pub notifier_init: bool,
    pub next: list_head,
    pub index: c_int,
    pub count: c_int,
    pub flags: c_int,
    pub users: refcount_t,
    pub notify: gntdev_unmap_notify,
    pub grants: *mut ioctl_gntdev_grant_ref,
    pub map_ops: *mut gnttab_map_grant_ref,
    pub unmap_ops: *mut gnttab_unmap_grant_ref,
    pub kmap_ops: *mut gnttab_map_grant_ref,
    pub kunmap_ops: *mut gnttab_unmap_grant_ref,
    pub being_removed: *mut bool,
    pub pages: *mut page,
    pub pages_vm_start: c_ulong,

//
// If dmabuf_vaddr is not NULL then this mapping is backed by DMA
// capable memory.
//
    pub dma_dev: *mut device,
// Flags used to create this DMA buffer: GNTDEV_DMA_FLAG_XXX.
    pub dma_flags: c_int,
    pub dma_vaddr: *mut c_void,
    pub dma_bus_addr: dma_addr_t,
// Needed to avoid allocation in gnttab_dma_free_pages().
    pub frames: *mut xen_pfn_t,

// Number of live grants
    pub live_grants: core::sync::atomic::AtomicI32,
// Needed to avoid allocation in __unmap_grant_pages
    pub unmap_data: gntab_unmap_queue_data,
}

extern "C" {
    pub fn gntdev_add_map(priv: *mut gntdev_priv, add: *mut gntdev_grant_map);
}
extern "C" {
    pub fn gntdev_put_map(priv: *mut gntdev_priv, map: *mut gntdev_grant_map);
}
extern "C" {
    pub fn gntdev_test_page_count(count: c_uint) -> bool;
}
extern "C" {
    pub fn gntdev_map_grant_pages(map: *mut gntdev_grant_map) -> c_int;
}
