//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_io.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_db_type {
    HINIC_DB_CMDQ_TYPE,
    HINIC_DB_SQ_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_io_path {
    HINIC_CTRL_PATH,
    HINIC_DATA_PATH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_free_db_area {
    pub db_idx: [c_int; HINIC_DB_MAX_AREAS],
    pub alloc_pos: c_int,
    pub return_pos: c_int,
    pub num_free: c_int,
// Lock for getting db area
    pub idx_lock: semaphore,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_nic_cfg {
// lock for getting nic cfg
    pub cfg_mutex: mutex,
    pub pause_set: bool,
    pub auto_neg: u32,
    pub rx_pause: u32,
    pub tx_pause: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_func_to_io {
    pub hwif: *mut hinic_hwif,
    pub hwdev: *mut hinic_hwdev,
    pub global_qpn: u16,
    pub ceqs: hinic_ceqs,
    pub wqs: hinic_wqs,
    pub sq_wq: *mut hinic_wq,
    pub rq_wq: *mut hinic_wq,
    pub qps: *mut hinic_qp,
    pub max_qps: u16,
    pub sq_depth: u16,
    pub rq_depth: u16,
    pub sq_db: *mut void __iomem,
    pub db_base: *mut void __iomem,
    pub ci_addr_base: *mut c_void,
    pub ci_dma_base: dma_addr_t,
    pub free_db_area: hinic_free_db_area,
    pub cmdq_db_area: [*mut void __iomem; HINIC_MAX_CMDQ_TYPES],
    pub cmdqs: hinic_cmdqs,
    pub max_vfs: u16,
    pub vf_infos: *mut vf_data_storage,
    pub link_status: u8,
    pub nic_cfg: hinic_nic_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_wq_page_size {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub func_idx: u16,
    pub ppf_idx: u8,
    pub page_size: u8,
    pub rsvd1: u32,
}

extern "C" {
    pub fn hinic_io_free(func_to_io: *mut hinic_func_to_io);
}
