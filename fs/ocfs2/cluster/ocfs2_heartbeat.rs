//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/cluster/ocfs2_heartbeat.h
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
// ocfs2_heartbeat.h
//
// On-disk structures for ocfs2_heartbeat
//
// Copyright (C) 2002, 2004 Oracle.  All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct o2hb_disk_heartbeat_block {
    pub hb_seq: __le64,
    pub hb_node: __u8,
    pub hb_pad1: [__u8; 3],
    pub hb_cksum: __le32,
    pub hb_generation: __le64,
    pub hb_dead_ms: __le32,
}
