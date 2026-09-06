//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/hisilicon/hpre/hpre.h
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

pub const HPRE_PF_DEF_Q_NUM: c_int = 64;
pub const HPRE_PF_DEF_Q_BASE: c_int = 0;
//
// type used in qm sqc DW6.
// 0 - Algorithm which has been supported in V2, like RSA, DH and so on;
// 1 - ECC algorithm in V3.
//
pub const HPRE_V2_ALG_TYPE: c_int = 0;
pub const HPRE_V3_ECC_ALG_TYPE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hpre_ctrl_dbgfs_file {
    HPRE_CLEAR_ENABLE,
    HPRE_CLUSTER_CTRL,
    HPRE_DEBUG_FILE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hpre_dfx_dbgfs_file {
    HPRE_SEND_CNT,
    HPRE_RECV_CNT,
    HPRE_SEND_FAIL_CNT,
    HPRE_SEND_BUSY_CNT,
    HPRE_OVER_THRHLD_CNT,
    HPRE_OVERTIME_THRHLD,
    HPRE_INVALID_REQ_CNT,
    HPRE_DFX_FILE_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre_debugfs_file {
    pub index: c_int,
    pub type: hpre_ctrl_dbgfs_file,
    pub lock: spinlock_t,
    pub debug: *mut hpre_debug,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre_dfx {
    pub value: core::sync::atomic::AtomicI64,
    pub type: hpre_dfx_dbgfs_file,
}

//
// One HPRE controller has one PF and multiple VFs, some global configurations
// which PF has need this structure.
// Just relevant for PF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre_debug {
    pub dfx: [hpre_dfx; HPRE_DFX_FILE_NUM],
    pub files: [hpre_debugfs_file; HPRE_DEBUGFS_FILE_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre {
    pub qm: hisi_qm,
    pub debug: hpre_debug,
    pub status: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hpre_alg_type {
    HPRE_ALG_NC_NCRT = 0x0,
    HPRE_ALG_NC_CRT = 0x1,
    HPRE_ALG_KG_STD = 0x2,
    HPRE_ALG_KG_CRT = 0x3,
    HPRE_ALG_DH_G2 = 0x4,
    HPRE_ALG_DH = 0x5,
    HPRE_ALG_ECC_MUL = 0xD,
// shared by x25519 and x448, but x448 is not supported now
    HPRE_ALG_CURVE25519_MUL = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpre_sqe {
    pub dw0: __le32,
    pub task_len1: __u8,
    pub task_len2: __u8,
    pub mrttest_num: __u8,
    pub resv1: __u8,
    pub key: __le64,
    pub in: __le64,
    pub out: __le64,
    pub tag: __le64,
pub const _HPRE_SQE_ALIGN_EXT: c_int = 6;
    pub rsvd1: [__le32; _HPRE_SQE_ALIGN_EXT],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hpre_cap_table_type {
    QM_RAS_NFE_TYPE = 0x0,
    QM_RAS_NFE_RESET,
    QM_RAS_CE_TYPE,
    HPRE_RAS_NFE_TYPE,
    HPRE_RAS_NFE_RESET,
    HPRE_RAS_CE_TYPE,
    HPRE_CORE_INFO,
    HPRE_CORE_EN,
    HPRE_DRV_ALG_BITMAP,
    HPRE_ALG_BITMAP,
    HPRE_CORE1_BITMAP_CAP,
    HPRE_CORE2_BITMAP_CAP,
    HPRE_CORE3_BITMAP_CAP,
    HPRE_CORE4_BITMAP_CAP,
    HPRE_CORE5_BITMAP_CAP,
    HPRE_CORE6_BITMAP_CAP,
    HPRE_CORE7_BITMAP_CAP,
    HPRE_CORE8_BITMAP_CAP,
    HPRE_CORE9_BITMAP_CAP,
    HPRE_CORE10_BITMAP_CAP,
}

extern "C" {
    pub fn hpre_algs_register(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hpre_algs_unregister(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hpre_check_alg_support(qm: *mut hisi_qm, alg: u32) -> bool;
}
