//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/aie4_pci.h
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
// Copyright (C) 2026, Advanced Micro Devices, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cert_comp {
    pub ndev: *mut amdxdna_dev_hdl,
    pub msix_idx: u32,
    pub irq: c_int,
    pub kref: kref,
    pub waitq: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_hwctx_priv {
    pub umq_bo: *mut amdxdna_gem_obj,
    pub umq_read_index: *mut u64,
    pub umq_write_index: *mut u64,
    pub cert_comp: *mut cert_comp,
    pub hw_ctx_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_dev_priv {
    pub npufw_path: *const c_char,
    pub certfw_path: *const c_char,
    pub mbox_bar: u32,
    pub mbox_rbuf_bar: u32,
    pub mbox_info_off: u64,
    pub doorbell_off: u32,
    pub psp_regs_off: [aie_bar_off_pair; PSP_MAX_REGS],
    pub smu_regs_off: [aie_bar_off_pair; SMU_MAX_REGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_dev_hdl {
    pub aie: aie_device,
    pub priv: *const amdxdna_dev_priv,
    pub mbox_base: *mut void __iomem,
    pub rbuf_base: *mut void __iomem,
    pub mbox: *mut mailbox,
    pub partition_id: u32,
    pub /: *mut *mut xarray cert_comp_xa; / device level indexed by msix id,
    pub operations*/: *mut *mut mutex cert_comp_lock; / protects cert_comp,
    pub work_buf: *mut c_void,
    pub work_buf_addr: dma_addr_t,
    pub work_buf_size: u32,
}

// aie4_message.c
extern "C" {
    pub fn aie4_suspend_fw(ndev: *mut amdxdna_dev_hdl) -> c_int;
}
extern "C" {
    pub fn aie4_attach_work_buffer(ndev: *mut amdxdna_dev_hdl) -> c_int;
}
// aie4_ctx.c
extern "C" {
    pub fn aie4_hwctx_init(hwctx: *mut amdxdna_hwctx) -> c_int;
}
extern "C" {
    pub fn aie4_hwctx_fini(hwctx: *mut amdxdna_hwctx);
}
extern "C" {
    pub fn aie4_cmd_wait(hwctx: *mut amdxdna_hwctx, seq: u64, timeout: u32) -> c_int;
}
extern "C" {
    pub fn aie4_hwctx_valid_doorbell(client: *mut amdxdna_client, vm_pgoff: u32) -> c_int;
}
// aie4_sriov.c

extern "C" {
    pub fn aie4_sriov_configure(xdna: *mut amdxdna_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn aie4_sriov_stop(ndev: *mut amdxdna_dev_hdl) -> c_int;
}

