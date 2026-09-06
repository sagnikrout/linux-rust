//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/tc_act/tc_tunnel_key.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright (c) 2016, Amir Vadai <amir@vadai.me>
// Copyright (c) 2016, Mellanox Technologies. All rights reserved.
//

pub const TCA_TUNNEL_KEY_ACT_SET: c_int = 1;
pub const TCA_TUNNEL_KEY_ACT_RELEASE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_tunnel_key {
    pub t_action: c_int,
}

// attributes
//

// TCA_TUNNEL_KEY_ENC_OPTS_
// attributes
//
// TCA_TUNNEL_KEY_ENC_OPTS_
// attributes
//
// TCA_TUNNEL_KEY_ENC_OPTS_
// attributes
//

