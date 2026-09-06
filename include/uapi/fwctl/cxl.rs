//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/fwctl/cxl.h
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
// Copyright (c) 2024-2025 Intel Corporation
//
// These are definitions for the mailbox command interface of CXL subsystem.
//

//
// struct fwctl_rpc_cxl - ioctl(FWCTL_RPC) input for CXL
// @opcode: CXL mailbox command opcode
// @flags: Flags for the command (input).
// @op_size: Size of input payload.
// @reserved1: Reserved. Must be 0s.
// @get_sup_feats_in: Get Supported Features input
// @get_feat_in: Get Feature input
// @set_feat_in: Set Feature input
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_rpc_cxl {
    pub opcode: __u32,
    pub flags: __u32,
    pub op_size: __u32,
    pub reserved1: __u32,
    pub get_sup_feats_in: cxl_mbox_get_sup_feats_in,
    pub get_feat_in: cxl_mbox_get_feat_in,
    pub set_feat_in: cxl_mbox_set_feat_in,
}

//
// struct fwctl_rpc_cxl_out - ioctl(FWCTL_RPC) output for CXL
// @size: Size of the output payload
// @retval: Return value from device
// @get_sup_feats_out: Get Supported Features output
// @payload: raw byte stream of payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_rpc_cxl_out {
    pub size: __u32,
    pub retval: __u32,
    pub get_sup_feats_out: cxl_mbox_get_sup_feats_out,
    pub payload): __DECLARE_FLEX_ARRAY(__u8,,
}
