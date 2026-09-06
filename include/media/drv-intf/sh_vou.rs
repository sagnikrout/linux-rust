//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/sh_vou.h
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
// SuperH Video Output Unit (VOU) driver header
//
// Copyright (C) 2010, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//

// Bus flags

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sh_vou_bus_fmt {
    SH_VOU_BUS_8BIT,
    SH_VOU_BUS_16BIT,
    SH_VOU_BUS_BT656,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_vou_pdata {
    pub bus_fmt: sh_vou_bus_fmt,
    pub i2c_adap: c_int,
    pub board_info: *mut i2c_board_info,
    pub flags: c_ulong,
}
