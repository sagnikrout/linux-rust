//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/s5p_mfc_cmd.h
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
// linux/drivers/media/platform/samsung/s5p-mfc/s5p_mfc_cmd.h
//
// Copyright (C) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

pub const MAX_H2R_ARG: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_cmd_args {
    pub arg: [c_uint; MAX_H2R_ARG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5p_mfc_hw_cmds {
    pub args): *const s5p_mfc_cmd_args,
    pub dev): *mut *mut int (sys_init_cmd)(struct s5p_mfc_dev,
    pub dev): *mut *mut int (sleep_cmd)(struct s5p_mfc_dev,
    pub dev): *mut *mut int (wakeup_cmd)(struct s5p_mfc_dev,
    pub ctx): *mut *mut int (open_inst_cmd)(struct s5p_mfc_ctx,
    pub ctx): *mut *mut int (close_inst_cmd)(struct s5p_mfc_ctx,
}

extern "C" {
    pub fn s5p_mfc_init_hw_cmds(dev: *mut s5p_mfc_dev);
}
