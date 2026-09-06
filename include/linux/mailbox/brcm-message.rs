//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mailbox/brcm-message.h
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
// Copyright (C) 2016 Broadcom
//
// Common header for Broadcom mailbox messages which is shared across
// Broadcom SoCs and Broadcom mailbox client drivers.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcm_message_type {
    BRCM_MESSAGE_UNKNOWN = 0,
    BRCM_MESSAGE_BATCH,
    BRCM_MESSAGE_SPU,
    BRCM_MESSAGE_SBA,
    BRCM_MESSAGE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_sba_command {
    pub cmd: u64,
    pub cmd_dma: *mut u64,
    pub cmd_dma_addr: dma_addr_t,

    pub flags: u64,
    pub resp: dma_addr_t,
    pub resp_len: usize,
    pub data: dma_addr_t,
    pub data_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_message {
    pub type: brcm_message_type,
    pub msgs: *mut brcm_message,
    pub msgs_queued: c_uint,
    pub msgs_count: c_uint,
    pub batch: },
    pub src: *mut scatterlist,
    pub dst: *mut scatterlist,
    pub spu: },
    pub cmds: *mut brcm_sba_command,
    pub cmds_count: c_uint,
    pub sba: },
}
