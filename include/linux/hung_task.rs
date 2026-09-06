//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hung_task.h
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
// Detect Hung Task: detecting tasks stuck in D state
//
// Copyright (C) 2025 Tongcheng Travel (www.ly.com)
// Author: Lance Yang <mingzhe.yang@ly.com>
//

//
// @blocker: Combines lock address and blocking type.
//
// Since lock pointers are at least 4-byte aligned(32-bit) or 8-byte
// aligned(64-bit). This leaves the 2 least bits (LSBs) of the pointer
// always zero. So we can use these bits to encode the specific blocking
// type.
//
// Note that on architectures where this is not guaranteed, or for any
// unaligned lock, this tracking mechanism is silently skipped for that
// lock.
//
// Type encoding:
// 00 - Blocked on mutex			(BLOCKER_TYPE_MUTEX)
// 01 - Blocked on semaphore			(BLOCKER_TYPE_SEM)
// 10 - Blocked on rw-semaphore as READER	(BLOCKER_TYPE_RWSEM_READER)
// 11 - Blocked on rw-semaphore as WRITER	(BLOCKER_TYPE_RWSEM_WRITER)
//
pub const BLOCKER_TYPE_MUTEX: c_uint = 0x00UL;
pub const BLOCKER_TYPE_SEM: c_uint = 0x01UL;
pub const BLOCKER_TYPE_RWSEM_READER: c_uint = 0x02UL;
pub const BLOCKER_TYPE_RWSEM_WRITER: c_uint = 0x03UL;
pub const BLOCKER_TYPE_MASK: c_uint = 0x03UL;

//
// If the lock pointer matches the BLOCKER_TYPE_MASK, return
// without writing anything.
//
// hung_task_get_blocker_type - Extracts blocker type from encoded blocker
// address.
//
// @blocker: Blocker pointer with encoded type (via LSB bits)
//
// Returns: BLOCKER_TYPE_MUTEX, BLOCKER_TYPE_SEM, etc.
//

