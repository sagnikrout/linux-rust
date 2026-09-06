//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/vnic_dev.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

//
// These defines avoid symbol clash between fnic and enic (Cisco 10G Eth
// Driver) when both are built with CONFIG options =y
//

pub const VNIC_PADDR_TARGET: c_uint = 0x0000000000000000ULL;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vnic_dev_intr_mode {
    VNIC_DEV_INTR_MODE_UNKNOWN,
    VNIC_DEV_INTR_MODE_INTX,
    VNIC_DEV_INTR_MODE_MSI,
    VNIC_DEV_INTR_MODE_MSIX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_dev_bar {
    pub vaddr: *mut void __iomem,
    pub bus_addr: dma_addr_t,
    pub len: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_dev_ring {
    pub descs: *mut c_void,
    pub size: usize,
    pub base_addr: dma_addr_t,
    pub base_align: usize,
    pub descs_unaligned: *mut c_void,
    pub size_unaligned: usize,
    pub base_addr_unaligned: dma_addr_t,
    pub desc_size: c_uint,
    pub desc_count: c_uint,
    pub desc_avail: c_uint,
}

extern "C" {
    pub fn vnic_dev_clear_desc_ring(ring: *mut vnic_dev_ring);
}
extern "C" {
    pub fn vnic_dev_stats_clear(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_cmd_init(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_stats_dump(vdev: *mut vnic_dev, stats: *mut vnic_stats) -> c_int;
}
extern "C" {
    pub fn vnic_dev_hang_notify(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_add_addr(vdev: *mut vnic_dev, addr: *mut u8);
}
extern "C" {
    pub fn vnic_dev_del_addr(vdev: *mut vnic_dev, addr: *mut u8);
}
extern "C" {
    pub fn vnic_dev_mac_addr(vdev: *mut vnic_dev, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn vnic_dev_notify_set(vdev: *mut vnic_dev, intr: u16) -> c_int;
}
extern "C" {
    pub fn vnic_dev_notify_unset(vdev: *mut vnic_dev);
}
extern "C" {
    pub fn vnic_dev_link_status(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_port_speed(vdev: *mut vnic_dev) -> u32;
}
extern "C" {
    pub fn vnic_dev_msg_lvl(vdev: *mut vnic_dev) -> u32;
}
extern "C" {
    pub fn vnic_dev_mtu(vdev: *mut vnic_dev) -> u32;
}
extern "C" {
    pub fn vnic_dev_link_down_cnt(vdev: *mut vnic_dev) -> u32;
}
extern "C" {
    pub fn vnic_dev_close(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_enable(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_disable(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_open(vdev: *mut vnic_dev, arg: c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_open_done(vdev: *mut vnic_dev, done: *mut c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_init(vdev: *mut vnic_dev, arg: c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_soft_reset(vdev: *mut vnic_dev, arg: c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_soft_reset_done(vdev: *mut vnic_dev, done: *mut c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_get_intr_mode(vdev: *mut vnic_dev) -> vnic_dev_intr_mode;
}
extern "C" {
    pub fn vnic_dev_unregister(vdev: *mut vnic_dev);
}
