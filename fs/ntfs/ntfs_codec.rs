//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/ntfs_codec.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Transparent compression codec interface.
//
// Copyright (c) 2026 LG Electronics Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ntfs_codec_id {
    NTFS_CODEC_LZNT1,

    NTFS_CODEC_XPRESS4K,
    NTFS_CODEC_XPRESS8K,
    NTFS_CODEC_XPRESS16K,
    NTFS_CODEC_LZX32K,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_codec_ops {
    pub id: ntfs_codec_id,
    pub name: *const c_char,
    pub chunk_size): *mut *mut size_t (scratch_size)(u32,
    pub chunk_size): u32,
    pub initialized_size): loff_t i_size, s64,
    pub outbuf): *const *const char inbuf, int bufsize, char,
}

