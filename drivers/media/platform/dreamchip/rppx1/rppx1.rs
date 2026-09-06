//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/dreamchip/rppx1/rppx1.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rppx1 {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub acq: rpp_module,
    pub bls: rpp_module,
    pub lin: rpp_module,
    pub lsc: rpp_module,
    pub awbg: rpp_module,
    pub dpcc: rpp_module,
    pub bd: rpp_module,
    pub hist: rpp_module,
    pub hist256: rpp_module,
    pub exm: rpp_module,
    pub pre1: },
    pub acq: rpp_module,
    pub bls: rpp_module,
    pub lin: rpp_module,
    pub lsc: rpp_module,
    pub awbg: rpp_module,
    pub dpcc: rpp_module,
    pub bd: rpp_module,
    pub hist: rpp_module,
    pub exm: rpp_module,
    pub pre2: },
    pub awbg: rpp_module,
    pub ccor: rpp_module,
    pub hist: rpp_module,
    pub db: rpp_module,
    pub cac: rpp_module,
    pub ltm: rpp_module,
    pub ltmmeas: rpp_module,
    pub wbmeas: rpp_module,
    pub bdrgb: rpp_module,
    pub shrp: rpp_module,
    pub post: },
    pub ga: rpp_module,
    pub is: rpp_module,
    pub ccor: rpp_module,
    pub outif: rpp_module,
    pub outregs: rpp_module,
    pub hv: },
    pub ga: rpp_module,
    pub is: rpp_module,
    pub ccor: rpp_module,
    pub outif: rpp_module,
    pub outregs: rpp_module,
    pub xyz2luv: rpp_module,
    pub mv: },
    pub rmap: rpp_module,
    pub rmapmeas: rpp_module,
}

extern "C" {
    pub fn rppx1_write(rpp: *mut rppx1, offset: u32, value: u32);
}
extern "C" {
    pub fn rppx1_read(rpp: *mut rppx1, offset: u32) -> u32;
}
