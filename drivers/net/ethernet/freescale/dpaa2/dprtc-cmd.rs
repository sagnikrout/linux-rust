//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dprtc-cmd.h
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
// Copyright 2013-2016 Freescale Semiconductor Inc.
// Copyright 2016-2018 NXP
//
// Command versioning
pub const DPRTC_CMD_BASE_VERSION: c_int = 1;
pub const DPRTC_CMD_VERSION_2: c_int = 2;
pub const DPRTC_CMD_ID_OFFSET: c_int = 4;

// Command IDs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprtc_cmd_open {
    pub dprtc_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprtc_cmd_get_irq {
    pub pad: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprtc_cmd_set_irq_enable {
    pub en: u8,
    pub pad: [u8; 3],
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprtc_rsp_get_irq_enable {
    pub en: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprtc_cmd_set_irq_mask {
    pub mask: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprtc_rsp_get_irq_mask {
    pub mask: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprtc_cmd_get_irq_status {
    pub status: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprtc_rsp_get_irq_status {
    pub status: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprtc_cmd_clear_irq_status {
    pub status: __le32,
    pub irq_index: u8,
}

