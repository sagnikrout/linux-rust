//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mailbox/mtk-vcp-mailbox.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2025 MediaTek Inc.
//
pub const MTK_VCP_MBOX_SLOT_MAX_SIZE: c_uint = 0x100 /* mbox max slot size */;
//
// struct mtk_ipi_info - mailbox message info for mtk-vcp-mailbox
// @msg: The share buffer between IPC and mailbox driver
// @len: Message length
// @id: This is for identification purposes and not actually used
// by the mailbox hardware.
// @index: The signal number of the mailbox message.
// @slot_ofs: Data slot offset.
// @irq_status: Captures incoming signals for the RX path.
//
// It is used between IPC with mailbox driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_ipi_info {
    pub msg: *mut c_void,
    pub len: u32,
    pub id: u32,
    pub index: u32,
    pub slot_ofs: u32,
    pub irq_status: u32,
}
