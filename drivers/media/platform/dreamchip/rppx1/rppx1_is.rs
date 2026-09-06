//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_is.c
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
// Copyright (C) 2026 Renesas Electronics Corp.
// Copyright (C) 2026 Ideas on Board Oy
// Copyright (C) 2026 Ragnatech AB
//

pub const IS_VERSION: c_uint = 0x0000;
pub const IS_H_OFFS: c_uint = 0x0008;
pub const IS_V_OFFS: c_uint = 0x000c;
pub const IS_H_SIZE: c_uint = 0x0010;
pub const IS_V_SIZE: c_uint = 0x0014;
pub const IS_H_OFFS_SHD: c_uint = 0x0024;
pub const IS_V_OFFS_SHD: c_uint = 0x0028;
pub const IS_H_SIZE_SHD: c_uint = 0x002c;
pub const IS_V_SIZE_SHD: c_uint = 0x0030;
#[no_mangle]
unsafe extern "C" fn rppx1_is_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_is_probe(struct rpp_module *mod)
    {
// Version check.
    if (rpp_module_read(mod, IS_VERSION) != 1)
    return -EINVAL;
    return 0;
    }
    static int rppx1_is_start(struct rpp_module *mod,
    const struct v4l2_mbus_framefmt *fmt)
    {
    rpp_module_write(mod, IS_H_OFFS, 0);
    rpp_module_write(mod, IS_V_OFFS, 0);
    rpp_module_write(mod, IS_H_SIZE, fmt.width);
    rpp_module_write(mod, IS_V_SIZE, fmt.height);
    return 0;
    }
    const struct rpp_module_ops rppx1_is_ops = {
    .probe = rppx1_is_probe,
    .start = rppx1_is_start,
    };
