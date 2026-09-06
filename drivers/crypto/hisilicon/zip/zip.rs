//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/hisilicon/zip/zip.h
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
// Copyright (c) 2019 HiSilicon Limited.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hisi_zip_error_type {
// negative compression
    HZIP_NC_ERR = 0x0d,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_zip_dfx {
    pub send_cnt: core::sync::atomic::AtomicI64,
    pub recv_cnt: core::sync::atomic::AtomicI64,
    pub send_busy_cnt: core::sync::atomic::AtomicI64,
    pub err_bd_cnt: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_zip {
    pub qm: hisi_qm,
    pub ctrl: *mut hisi_zip_ctrl,
    pub dfx: hisi_zip_dfx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_zip_sqe {
    pub consumed: u32,
    pub produced: u32,
    pub comp_data_length: u32,
//
// status: 0~7 bits
// rsvd: 8~31 bits
//
    pub dw3: u32,
    pub input_data_length: u32,
    pub dw5: u32,
    pub dw6: u32,
//
// in_sge_data_offset: 0~23 bits
// rsvd: 24~27 bits
// sqe_type: 29~31 bits
//
    pub dw7: u32,
//
// out_sge_data_offset: 0~23 bits
// rsvd: 24~31 bits
//
    pub dw8: u32,
//
// request_type: 0~7 bits
// buffer_type: 8~11 bits
// rsvd: 13~31 bits
//
    pub dw9: u32,
    pub dw10: u32,
    pub dw11: u32,
    pub dw12: u32,
// tag: in sqe type 0
    pub dw13: u32,
    pub dest_avail_out: u32,
    pub dw15: u32,
    pub dw16: u32,
    pub dw17: u32,
    pub source_addr_l: u32,
    pub source_addr_h: u32,
    pub dest_addr_l: u32,
    pub dest_addr_h: u32,
    pub dw22: u32,
    pub dw23: u32,
    pub dw24: u32,
    pub dw25: u32,
// tag: in sqe type 3
    pub dw26: u32,
    pub dw27: u32,
    pub rsvd1: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zip_cap_table_type {
    QM_RAS_NFE_TYPE,
    QM_RAS_NFE_RESET,
    QM_RAS_CE_TYPE,
    ZIP_RAS_NFE_TYPE,
    ZIP_RAS_NFE_RESET,
    ZIP_RAS_CE_TYPE,
    ZIP_CORE_INFO,
    ZIP_CORE_EN,
    ZIP_DRV_ALG_BITMAP_TB,
    ZIP_ALG_BITMAP,
    ZIP_CORE1_BITMAP,
    ZIP_CORE2_BITMAP,
    ZIP_CORE3_BITMAP,
    ZIP_CORE4_BITMAP,
    ZIP_CORE5_BITMAP,
}

extern "C" {
    pub fn zip_create_qps(qps: *mut hisi_qp, qp_num: c_int, node: c_int, alg_type: *mut u8) -> c_int;
}
extern "C" {
    pub fn hisi_zip_register_to_crypto(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hisi_zip_unregister_from_crypto(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_zip_alg_support(qm: *mut hisi_qm, alg: u32) -> bool;
}
extern "C" {
    pub fn hisi_dae_set_user_domain(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hisi_dae_set_alg(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hisi_dae_hw_error_disable(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_dae_hw_error_enable(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_dae_open_axi_master_ooo(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_dae_close_axi_master_ooo(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hisi_dae_dev_is_abnormal(qm: *mut hisi_qm) -> bool;
}
extern "C" {
    pub fn hisi_dae_get_err_result(qm: *mut hisi_qm) -> acc_err_result;
}
