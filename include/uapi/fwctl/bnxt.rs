//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/fwctl/bnxt.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (c) 2026, Broadcom Inc
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fwctl_bnxt_commands {
    FWCTL_BNXT_INLINE_COMMANDS = 0,
    FWCTL_BNXT_QUERY_COMMANDS,
    FWCTL_BNXT_SEND_COMMANDS,
    FWCTL_BNXT_DMA_COMMANDS,
}

//
// struct fwctl_info_bnxt - ioctl(FWCTL_INFO) out_device_data
// @uctx_caps: The command capabilities driver accepts.
//
// Return basic information about the FW interface available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_info_bnxt {
    pub uctx_caps: __u32,
}

pub const FWCTL_BNXT_MAX_DMABUF: c_uint = 0x10000   /* 64 KiB */;
