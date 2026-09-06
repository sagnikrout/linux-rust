//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/remoteproc/ti_k3_common.h
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
// TI K3 Remote Processor(s) driver common code
//
// Refactored out of ti_k3_r5_remoteproc.c, ti_k3_dsp_remoteproc.c and
// ti_k3_m4_remoteproc.c.
//
// ti_k3_r5_remoteproc.c:
// Copyright (C) 2017-2022 Texas Instruments Incorporated - https://www.ti.com
// Suman Anna <s-anna@ti.com>
//
// ti_k3_dsp_remoteproc.c:
// Copyright (C) 2018-2022 Texas Instruments Incorporated - https://www.ti.com
// Suman Anna <s-anna@ti.com>
//
// ti_k3_m4_remoteproc.c:
// Copyright (C) 2021-2024 Texas Instruments Incorporated - https://www.ti.com
// Hari Nagalla <hnagalla@ti.com>
//

//
// struct k3_rproc_mem - internal memory structure
// @cpu_addr: MPU virtual address of the memory region
// @bus_addr: Bus address used to access the memory region
// @dev_addr: Device address of the memory region from remote processor view
// @size: Size of the memory region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_rproc_mem {
    pub cpu_addr: *mut void __iomem,
    pub bus_addr: phys_addr_t,
    pub dev_addr: u32,
    pub size: usize,
}

//
// struct k3_rproc_mem_data - memory definitions for a remote processor
// @name: name for this memory entry
// @dev_addr: device address for the memory entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_rproc_mem_data {
    pub name: *const c_char,
    pub dev_addr: u32,
}

//
// struct k3_rproc_dev_data - device data structure for a remote processor
// @mems: pointer to memory definitions for a remote processor
// @num_mems: number of memory regions in @mems
// @boot_align_addr: boot vector address alignment granularity
// @uses_lreset: flag to denote the need for local reset management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_rproc_dev_data {
    pub mems: *const k3_rproc_mem_data,
    pub num_mems: u32,
    pub boot_align_addr: u32,
    pub uses_lreset: bool,
}

//
// struct k3_rproc - k3 remote processor driver structure
// @dev: cached device pointer
// @rproc: remoteproc device handle
// @mem: internal memory regions data
// @num_mems: number of internal memory regions
// @rmem: reserved memory regions data
// @num_rmems: number of reserved memory regions
// @reset: reset control handle
// @data: pointer to DSP-specific device data
// @tsp: TI-SCI processor control handle
// @ti_sci: TI-SCI handle
// @ti_sci_id: TI-SCI device identifier
// @mbox: mailbox channel handle
// @client: mailbox client to request the mailbox channel
// @priv: void pointer to carry any private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_rproc {
    pub dev: *mut device,
    pub rproc: *mut rproc,
    pub mem: *mut k3_rproc_mem,
    pub num_mems: c_int,
    pub rmem: *mut k3_rproc_mem,
    pub num_rmems: c_int,
    pub reset: *mut reset_control,
    pub data: *const k3_rproc_dev_data,
    pub tsp: *mut ti_sci_proc,
    pub ti_sci: *const ti_sci_handle,
    pub ti_sci_id: u32,
    pub mbox: *mut mbox_chan,
    pub client: mbox_client,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn k3_rproc_mbox_callback(client: *mut mbox_client, data: *mut c_void);
}
extern "C" {
    pub fn k3_rproc_kick(rproc: *mut rproc, vqid: c_int);
}
extern "C" {
    pub fn k3_rproc_reset(kproc: *mut k3_rproc) -> c_int;
}
extern "C" {
    pub fn k3_rproc_release(kproc: *mut k3_rproc) -> c_int;
}
extern "C" {
    pub fn k3_rproc_request_mbox(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn k3_rproc_prepare(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn k3_rproc_unprepare(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn k3_rproc_start(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn k3_rproc_stop(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn k3_rproc_attach(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn k3_rproc_detach(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn k3_mem_release(data: *mut c_void);
}
extern "C" {
    pub fn k3_reserved_mem_init(kproc: *mut k3_rproc) -> c_int;
}
extern "C" {
    pub fn k3_release_tsp(data: *mut c_void);
}
