//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1_xyz2luv.c
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

pub const XYZ2LUV_VERSION_REG: c_uint = 0x0000;
pub const XYZ2LUV_U_REF_REG: c_uint = 0x0004;
pub const XYZ2LUV_V_REF_REG: c_uint = 0x0008;
pub const XYZ2LUV_LUMA_OUT_FAC_REG: c_uint = 0x000c;
pub const XYZ2LUV_CHROMA_OUT_FAC_REG: c_uint = 0x0010;
#[no_mangle]
unsafe extern "C" fn rppx1_xyz2luv_probe(mod: *mut rpp_module) -> c_int {
    static int rppx1_xyz2luv_probe(struct rpp_module *mod)
    {
// Version check.
    if (rpp_module_read(mod, XYZ2LUV_VERSION_REG) != 4)
    return -EINVAL;
    return 0;
    }
    const struct rpp_module_ops rppx1_xyz2luv_ops = {
    .probe = rppx1_xyz2luv_probe,
    };
