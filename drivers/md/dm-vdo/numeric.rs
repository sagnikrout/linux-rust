//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/numeric.h
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
// Copyright 2023 Red Hat
//

//
// These utilities encode or decode a number from an offset in a larger data buffer and then
// advance the offset pointer to the next field in the buffer.
//
// decoded = get_unaligned_le64(buffer + *offset);
// offset += sizeof(s64);
// decoded = get_unaligned_le64(buffer + *offset);
// offset += sizeof(u64);
// decoded = get_unaligned_le32(buffer + *offset);
// offset += sizeof(s32);
// decoded = get_unaligned_le32(buffer + *offset);
// offset += sizeof(u32);
// decoded = get_unaligned_le16(buffer + *offset);
// offset += sizeof(u16);
