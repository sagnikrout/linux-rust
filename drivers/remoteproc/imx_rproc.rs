//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/remoteproc/imx_rproc.h
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
// Copyright (c) 2017 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
// Copyright 2021 NXP
//
// address translation table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_rproc_att {
    pub view)*/: *mut *mut u32 da; / device address (From Cortex M4,
    pub /: *mut *mut u32 sa; / system bus address,
    pub /: *mut *mut u32 size; / size of reg range,
    pub flags: c_int,
}

// dcfg flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_rproc_plat_ops {
    pub rproc): *mut *mut int (start)(struct rproc,
    pub rproc): *mut *mut int (stop)(struct rproc,
    pub rproc): *mut *mut int (detach)(struct rproc,
    pub rproc): *mut *mut int (detect_mode)(struct rproc,
    pub rproc): *mut *mut int (prepare)(struct rproc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_rproc_dcfg {
    pub src_reg: u32,
    pub src_mask: u32,
    pub src_start: u32,
    pub src_stop: u32,
    pub gpr_reg: u32,
    pub gpr_wait: u32,
    pub att: *const imx_rproc_att,
    pub att_size: usize,
    pub flags: u32,
    pub ops: *const imx_rproc_plat_ops,
// For System Manager(SM) based SoCs
    pub /: *mut *mut u32 cpuid; / ID of the remote core,
    pub /: *mut *mut u32 lmid; / ID of the Logcial Machine,
// reset_vector = elf_entry_addr & reset_vector_mask
    pub reset_vector_mask: u32,
}
