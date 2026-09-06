//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vdpa/octeon_ep/octep_vdpa.h
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
// Copyright (C) 2024 Marvell.
//

pub const OCTEP_VDPA_DEVID_CN106K_PF: c_uint = 0xb900;
pub const OCTEP_VDPA_DEVID_CN106K_VF: c_uint = 0xb903;
pub const OCTEP_VDPA_DEVID_CN105K_PF: c_uint = 0xba00;
pub const OCTEP_VDPA_DEVID_CN105K_VF: c_uint = 0xba03;
pub const OCTEP_VDPA_DEVID_CN103K_PF: c_uint = 0xbd00;
pub const OCTEP_VDPA_DEVID_CN103K_VF: c_uint = 0xbd03;
pub const OCTEP_HW_MBOX_BAR: c_int = 0;
pub const OCTEP_HW_CAPS_BAR: c_int = 4;
pub const OCTEP_DEV_READY_SIGNATURE: c_uint = 0xBABABABA;

pub const OCTEP_FW_READY_SIGNATURE0: c_uint = 0xFEEDFEED;
pub const OCTEP_FW_READY_SIGNATURE1: c_uint = 0x3355ffaa;
pub const OCTEP_MAX_CB_INTR: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_vdpa_dev_status {
    OCTEP_VDPA_DEV_STATUS_INVALID,
    OCTEP_VDPA_DEV_STATUS_ALLOC,
    OCTEP_VDPA_DEV_STATUS_WAIT_FOR_BAR_INIT,
    OCTEP_VDPA_DEV_STATUS_INIT,
    OCTEP_VDPA_DEV_STATUS_READY,
    OCTEP_VDPA_DEV_STATUS_ADDED,
    OCTEP_VDPA_DEV_STATUS_REMOVED,
    OCTEP_VDPA_DEV_STATUS_UNINIT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_vdpa_dev_event_state {
    OCTEP_VDPA_DEV_NO_EVENT,
    OCTEP_VDPA_DEV_NEW_EVENT,
    OCTEP_VDPA_DEV_EVENT_ACTIVE,
    OCTEP_VDPA_DEV_EVENT_DONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_vdpa_dev_event {
    OCTEP_VDPA_DEV_EVENT_NONE,
    OCTEP_VDPA_DEV_EVENT_ACK,
    OCTEP_VDPA_DEV_EVENT_NACK,
    OCTEP_VDPA_DEV_ADD_EVENT,
    OCTEP_VDPA_DEV_DEL_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vring_info {
    pub cb: vdpa_callback,
    pub notify_addr: *mut void __iomem,
    pub cb_notify_addr: *mut void __iomem,
    pub notify_pa: phys_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_pci_vndr_cfg_type {
    OCTEP_PCI_VNDR_CFG_TYPE_VIRTIO_ID,
    OCTEP_PCI_VNDR_CFG_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_pci_vndr_data {
    pub hdr: virtio_pci_vndr_data,
    pub id: u8,
    pub bar: u8,
    pub data: u64,
    pub offset: u32,
    pub length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_hw {
    pub pdev: *mut pci_dev,
    pub base: [*mut u8 __iomem; PCI_STD_NUM_BARS],
    pub common_cfg: *mut virtio_pci_common_cfg __iomem,
    pub dev_cfg: *mut u8 __iomem,
    pub isr: *mut u8 __iomem,
    pub notify_base: *mut void __iomem,
    pub notify_base_pa: phys_addr_t,
    pub notify_off_multiplier: u32,
    pub notify_bar: u8,
    pub vqs: *mut octep_vring_info,
    pub config_cb: vdpa_callback,
    pub features: u64,
    pub nr_vring: u16,
    pub config_size: u32,
    pub requested_irqs: c_int,
    pub nb_irqs: c_int,
    pub irqs: *mut c_int,
    pub dev_id: u8,
}

extern "C" {
    pub fn octep_hw_get_status(oct_hw: *mut octep_hw) -> u8;
}
extern "C" {
    pub fn octep_hw_set_status(dev: *mut octep_hw, status: u8);
}
extern "C" {
    pub fn octep_hw_reset(oct_hw: *mut octep_hw);
}
extern "C" {
    pub fn octep_write_queue_select(oct_hw: *mut octep_hw, queue_id: u16);
}
extern "C" {
    pub fn octep_notify_queue(oct_hw: *mut octep_hw, qid: u16);
}
extern "C" {
    pub fn octep_read_dev_config(oct_hw: *mut octep_hw, offset: u64, dst: *mut c_void, length: c_int);
}
extern "C" {
    pub fn octep_set_vq_num(oct_hw: *mut octep_hw, qid: u16, num: u32);
}
extern "C" {
    pub fn octep_set_vq_ready(oct_hw: *mut octep_hw, qid: u16, ready: bool);
}
extern "C" {
    pub fn octep_get_vq_ready(oct_hw: *mut octep_hw, qid: u16) -> bool;
}
extern "C" {
    pub fn octep_set_vq_state(oct_hw: *mut octep_hw, qid: u16, state: *const vdpa_vq_state) -> c_int;
}
extern "C" {
    pub fn octep_get_vq_state(oct_hw: *mut octep_hw, qid: u16, state: *mut vdpa_vq_state) -> c_int;
}
extern "C" {
    pub fn octep_get_vq_size(oct_hw: *mut octep_hw) -> u16;
}
extern "C" {
    pub fn octep_hw_caps_read(oct_hw: *mut octep_hw, pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn octep_hw_get_dev_features(oct_hw: *mut octep_hw) -> u64;
}
extern "C" {
    pub fn octep_hw_set_drv_features(oct_hw: *mut octep_hw, features: u64);
}
extern "C" {
    pub fn octep_hw_get_drv_features(oct_hw: *mut octep_hw) -> u64;
}
extern "C" {
    pub fn octep_verify_features(features: u64) -> c_int;
}
