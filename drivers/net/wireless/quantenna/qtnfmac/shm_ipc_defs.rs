//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/shm_ipc_defs.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2015-2016 Quantenna Communications. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qtnf_shm_ipc_region_flags {
    QTNF_SHM_IPC_NEW_DATA		= BIT(0),
    QTNF_SHM_IPC_ACK		= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_shm_ipc_region_header {
    pub flags: __le32,
    pub data_len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union qtnf_shm_ipc_region_headroom {
    pub hdr: qtnf_shm_ipc_region_header,
    pub headroom: [u8; QTN_IPC_REG_HDR_SZ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_shm_ipc_region {
    pub headroom: qtnf_shm_ipc_region_headroom,
    pub data: [u8; QTN_IPC_MAX_DATA_SZ],
    pub __packed: },
