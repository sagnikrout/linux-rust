//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/osdep.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2015 - 2021 Intel Corporation

pub const STATS_TIMER_DELAY: c_int = 60000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_dma_info {
    pub dmaaddrs: *mut dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_dma_mem {
    pub va: *mut c_void,
    pub pa: dma_addr_t,
    pub size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_virt_mem {
    pub va: *mut c_void,
    pub size: u32,
    pub __packed: },
    pub irdma_sc_vsi: struct,
    pub irdma_sc_dev: struct,
    pub irdma_sc_qp: struct,
    pub irdma_puda_buf: struct,
    pub irdma_puda_cmpl_info: struct,
    pub irdma_update_sds_info: struct,
    pub irdma_hmc_fcn_info: struct,
    pub irdma_manage_vf_pble_info: struct,
    pub irdma_hw: struct,
    pub irdma_pci_f: struct,
    pub dev): *mut *mut ib_device to_ibdev(irdma_sc_dev,
    pub qp): *mut *mut void irdma_ieq_mpa_crc_ae(struct irdma_sc_dev dev, struct irdma_sc_qp,
    pub dev): *mut irdma_status_code irdma_vf_wait_vchnl_resp(struct irdma_sc_dev,
    pub dev): *mut bool irdma_vf_clear_to_send(struct irdma_sc_dev,
    pub dev): *mut void irdma_add_dev_ref(struct irdma_sc_dev,
    pub dev): *mut void irdma_put_dev_ref(struct irdma_sc_dev,
    pub val): *const *const int irdma_ieq_check_mpacrc(void addr, u32 len, u32,
    pub buf): *mut irdma_puda_buf,
    pub qp): *mut void irdma_send_ieq_ack(struct irdma_sc_qp,
    pub seqnum): u32,
    pub buf): *mut irdma_puda_buf,
    pub info): *mut irdma_update_sds_info,
    pub pmf_idx): *mut u16,
    pub mem): *mut irdma_dma_mem,
    pub dev): *mut *mut void irdma_remove_cqp_head(struct irdma_sc_dev,
    pub term_len): u8,
    pub timeout_occurred): *mut *mut void irdma_terminate_done(struct irdma_sc_qp qp, int,
    pub qp): *mut void irdma_terminate_start_timer(struct irdma_sc_qp,
    pub qp): *mut void irdma_terminate_del_timer(struct irdma_sc_qp,
    pub vsi): *mut void irdma_hw_stats_start_timer(struct irdma_sc_vsi,
    pub vsi): *mut void irdma_hw_stats_stop_timer(struct irdma_sc_vsi,
    pub val): *mut *mut void wr32(struct irdma_hw hw, u32 reg, u32,
    pub reg): *mut *mut u32 rd32(struct irdma_hw hw, u32,
    pub reg): *mut *mut u64 rd64(struct irdma_hw hw, u32,
    pub pg_cnt): u32,
    pub pg_cnt): *mut *mut *mut void irdma_unmap_vm_page_list(struct irdma_hw hw, dma_addr_t pg_dma, u32,
