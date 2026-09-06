//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vdpa/pds/vdpa_dev.h
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
// Copyright(c) 2023 Advanced Micro Devices, Inc

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_vq_info {
    pub ready: bool,
    pub desc_addr: u64,
    pub avail_addr: u64,
    pub used_addr: u64,
    pub q_len: u32,
    pub qid: u16,
    pub irq: c_int,
    pub irq_name: [c_char; 32],
    pub notify: *mut void __iomem,
    pub notify_pa: dma_addr_t,
    pub doorbell: u64,
    pub avail_idx: u16,
    pub used_idx: u16,
    pub event_cb: vdpa_callback,
    pub pdsv: *mut pds_vdpa_device,
}

pub const PDS_VDPA_MAX_QUEUES: c_int = 65;
pub const PDS_VDPA_MAX_QLEN: c_int = 32768;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vdpa_device {
    pub vdpa_dev: vdpa_device,
    pub vdpa_aux: *mut pds_vdpa_aux,
    pub vqs: [pds_vdpa_vq_info; PDS_VDPA_MAX_QUEUES],
    pub /: *mut *mut u64 supported_features; / supported device features,
    pub /: *mut *mut u64 negotiated_features; / negotiated features,
    pub /: *mut *mut u8 vdpa_index; / rsvd for future subdevice use,
    pub /: *mut *mut u8 num_vqs; / num vqs in use,
    pub /: *mut *mut u8 mac[ETH_ALEN]; / mac selected when the device was added,
    pub config_cb: vdpa_callback,
    pub nb: notifier_block,
}

pub const PDS_VDPA_PACKED_INVERT_IDX: c_uint = 0x8000;
extern "C" {
    pub fn pds_vdpa_release_irqs(pdsv: *mut pds_vdpa_device);
}
extern "C" {
    pub fn pds_vdpa_get_mgmt_info(vdpa_aux: *mut pds_vdpa_aux) -> c_int;
}
