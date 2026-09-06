//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vivid/vivid-rds-gen.h
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
// vivid-rds-gen.h - rds (radio data system) generator support functions.
//
// Copyright 2014 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
// It takes almost exactly 5 seconds to transmit 57 RDS groups.
// Each group has 4 blocks and each block has a payload of 16 bits + a
// block identification. The driver will generate the contents of these
// 57 groups only when necessary and it will just be played continuously.
//
pub const VIVID_RDS_GEN_GROUPS: c_int = 57;
pub const VIVID_RDS_GEN_BLKS_PER_GRP: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vivid_rds_gen {
    pub data: [v4l2_rds_data; VIVID_RDS_GEN_BLOCKS],
    pub use_rbds: bool,
    pub picode: u16,
    pub pty: u8,
    pub mono_stereo: bool,
    pub art_head: bool,
    pub compressed: bool,
    pub dyn_pty: bool,
    pub ta: bool,
    pub tp: bool,
    pub ms: bool,
    pub 1]: char psname[8 +,
    pub 1]: char radiotext[64 +,
}

extern "C" {
    pub fn vivid_rds_generate(rds: *mut vivid_rds_gen);
}
