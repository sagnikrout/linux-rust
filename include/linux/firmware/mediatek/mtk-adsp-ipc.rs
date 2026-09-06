//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/mediatek/mtk-adsp-ipc.h
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
// Copyright (c) 2022 MediaTek Inc.
//

pub const MTK_ADSP_IPC_REQ: c_int = 0;
pub const MTK_ADSP_IPC_RSP: c_int = 1;
pub const MTK_ADSP_IPC_OP_REQ: c_uint = 0x1;
pub const MTK_ADSP_IPC_OP_RSP: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_adsp_ipc_ops {
    pub ipc): *mut *mut void (handle_reply)(struct mtk_adsp_ipc,
    pub ipc): *mut *mut void (handle_request)(struct mtk_adsp_ipc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_adsp_chan {
    pub ipc: *mut mtk_adsp_ipc,
    pub cl: mbox_client,
    pub ch: *mut mbox_chan,
    pub name: *mut c_char,
    pub idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_adsp_ipc {
    pub chans: [mtk_adsp_chan; MTK_ADSP_MBOX_NUM],
    pub dev: *mut device,
    pub ops: *const mtk_adsp_ipc_ops,
    pub private_data: *mut c_void,
}

extern "C" {
    pub fn mtk_adsp_ipc_send(ipc: *mut mtk_adsp_ipc, idx: c_uint, op: u32) -> c_int;
}
