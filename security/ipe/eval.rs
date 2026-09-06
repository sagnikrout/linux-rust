//! Automatically rewritten from C Header to Rust Module
//! Source: security/ipe/eval.h
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
// Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_superblock {
    pub initramfs: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_bdev {

    pub dm_verity_signed: bool,

    pub root_hash: *mut digest_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_inode {
    pub fs_verity_signed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipe_eval_ctx {
    pub op: ipe_op_type,
    pub hook: ipe_hook_type,
    pub file: *const file,
    pub initramfs: bool,

    pub ipe_bdev: *const ipe_bdev,

    pub ino: *const inode,

    pub ipe_inode: *const ipe_inode,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipe_match {
    IPE_MATCH_RULE = 0,
    IPE_MATCH_TABLE,
    IPE_MATCH_GLOBAL,
    __IPE_MATCH_MAX
}

extern "C" {
    pub fn ipe_evaluate_event(ctx: *const *const ipe_eval_ctx) -> c_int;
}
