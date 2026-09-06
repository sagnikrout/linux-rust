//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/vnic_dev.h
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
// Copyright 2008-2010 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

pub const VNIC_PADDR_TARGET: c_uint = 0x0000000000000000ULL;

pub const VNIC_DESC_SIZE_ALIGN: c_int = 16;
pub const VNIC_DESC_COUNT_ALIGN: c_int = 32;
pub const VNIC_DESC_BASE_ALIGN: c_int = 512;
pub const VNIC_DESC_MAX_COUNT: c_int = 4096;
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vnic_proxy_type {
    PROXY_NONE,
    PROXY_BY_BDF,
    PROXY_BY_INDEX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_res {
    pub vaddr: *mut void __iomem,
    pub bus_addr: dma_addr_t,
    pub count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_intr_coal_timer_info {
    pub mul: u32,
    pub div: u32,
    pub max_usec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_dev {
    pub priv: *mut c_void,
    pub pdev: *mut pci_dev,
    pub res: [vnic_res; RES_TYPE_MAX],
    pub intr_mode: vnic_dev_intr_mode,
    pub devcmd: *mut vnic_devcmd __iomem,
    pub notify: *mut vnic_devcmd_notify,
    pub notify_copy: vnic_devcmd_notify,
    pub notify_pa: dma_addr_t,
    pub notify_sz: u32,
    pub linkstatus_pa: dma_addr_t,
    pub stats: *mut vnic_stats,
    pub stats_pa: dma_addr_t,
    pub fw_info: *mut vnic_devcmd_fw_info,
    pub fw_info_pa: dma_addr_t,
    pub proxy: vnic_proxy_type,
    pub proxy_index: u32,
    pub args: [u64; VNIC_DEVCMD_NARGS],
    pub intr_coal_timer_info: vnic_intr_coal_timer_info,
    pub devcmd2: *mut devcmd2_controller,
    pub wait): c_int,
}

extern "C" {
    pub fn vnic_dev_clear_desc_ring(ring: *mut vnic_dev_ring);
}
extern "C" {
    pub fn vnic_dev_cmd_proxy_by_index_start(vdev: *mut vnic_dev, index: u16);
}
extern "C" {
    pub fn vnic_dev_cmd_proxy_end(vdev: *mut vnic_dev);
}
extern "C" {
    pub fn vnic_dev_stats_dump(vdev: *mut vnic_dev, stats: *mut vnic_stats) -> c_int;
}
extern "C" {
    pub fn vnic_dev_hang_notify(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_add_addr(vdev: *mut vnic_dev, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn vnic_dev_del_addr(vdev: *mut vnic_dev, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn vnic_dev_get_mac_addr(vdev: *mut vnic_dev, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn vnic_dev_notify_set(vdev: *mut vnic_dev, intr: u16) -> c_int;
}
extern "C" {
    pub fn vnic_dev_notify_unset(vdev: *mut vnic_dev) -> c_int;
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
    pub fn vnic_dev_close(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_enable_wait(vdev: *mut vnic_dev) -> c_int;
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
    pub fn vnic_dev_deinit(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_intr_coal_timer_info_default(vdev: *mut vnic_dev);
}
extern "C" {
    pub fn vnic_dev_intr_coal_timer_info(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_hang_reset(vdev: *mut vnic_dev, arg: c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_soft_reset(vdev: *mut vnic_dev, arg: c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_hang_reset_done(vdev: *mut vnic_dev, done: *mut c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_soft_reset_done(vdev: *mut vnic_dev, done: *mut c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_get_intr_mode(vdev: *mut vnic_dev) -> vnic_dev_intr_mode;
}
extern "C" {
    pub fn vnic_dev_intr_coal_timer_usec_to_hw(vdev: *mut vnic_dev, usec: u32) -> u32;
}
extern "C" {
    pub fn vnic_dev_intr_coal_timer_hw_to_usec(vdev: *mut vnic_dev, hw_cycles: u32) -> u32;
}
extern "C" {
    pub fn vnic_dev_get_intr_coal_timer_max(vdev: *mut vnic_dev) -> u32;
}
extern "C" {
    pub fn vnic_dev_unregister(vdev: *mut vnic_dev);
}
extern "C" {
    pub fn vnic_dev_init_prov2(vdev: *mut vnic_dev, buf: *mut u8, len: u32) -> c_int;
}
extern "C" {
    pub fn vnic_dev_enable2(vdev: *mut vnic_dev, active: c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_enable2_done(vdev: *mut vnic_dev, status: *mut c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_deinit_done(vdev: *mut vnic_dev, status: *mut c_int) -> c_int;
}
extern "C" {
    pub fn vnic_dev_set_mac_addr(vdev: *mut vnic_dev, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn vnic_devcmd_init(vdev: *mut vnic_dev) -> c_int;
}
extern "C" {
    pub fn vnic_dev_overlay_offload_ctrl(vdev: *mut vnic_dev, overlay: u8, config: u8) -> c_int;
}
extern "C" {
    pub fn vnic_dev_capable_rss_hash_type(vdev: *mut vnic_dev, rss_hash_type: *mut u8) -> c_int;
}
