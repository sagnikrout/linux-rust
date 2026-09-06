//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/hisilicon/qm_common.h
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
// Copyright (c) 2022 HiSilicon Limited.
pub const QM_DBG_READ_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_cqe {
    pub rsvd0: __le32,
    pub cmd_id: __le16,
    pub rsvd1: __le16,
    pub sq_head: __le16,
    pub sq_num: __le16,
    pub rsvd2: __le16,
    pub w7: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_eqe {
    pub dw0: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_aeqe {
    pub dw0: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_sqc {
    pub head: __le16,
    pub tail: __le16,
    pub base_l: __le32,
    pub base_h: __le32,
    pub dw3: __le32,
    pub w8: __le16,
    pub rsvd0: __le16,
    pub pasid: __le16,
    pub w11: __le16,
    pub cq_num: __le16,
    pub w13: __le16,
    pub rsvd1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_cqc {
    pub head: __le16,
    pub tail: __le16,
    pub base_l: __le32,
    pub base_h: __le32,
    pub dw3: __le32,
    pub w8: __le16,
    pub rsvd0: __le16,
    pub pasid: __le16,
    pub w11: __le16,
    pub dw6: __le32,
    pub rsvd1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_eqc {
    pub head: __le16,
    pub tail: __le16,
    pub base_l: __le32,
    pub base_h: __le32,
    pub dw3: __le32,
    pub rsvd: [__le32; 2],
    pub dw6: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_aeqc {
    pub head: __le16,
    pub tail: __le16,
    pub base_l: __le32,
    pub base_h: __le32,
    pub dw3: __le32,
    pub rsvd: [__le32; 2],
    pub dw6: __le32,
}

extern "C" {
    pub fn qm_set_and_get_xqc(qm: *mut hisi_qm, cmd: u8, xqc: *mut c_void, qp_id: u32, op: bool) -> c_int;
}
extern "C" {
    pub fn hisi_qm_show_last_dfx_regs(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_qm_set_algqos_init(qm: *mut hisi_qm);
}
