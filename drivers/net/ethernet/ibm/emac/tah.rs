//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ibm/emac/tah.h
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
// drivers/net/ethernet/ibm/emac/tah.h
//
// Driver for PowerPC 4xx on-chip ethernet controller, TAH support.
//
// Copyright 2007 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//
// Based on the arch/ppc version of the driver:
//
// Copyright 2004 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//
// Copyright (c) 2005 Eugene Surovegin <ebs@ebshome.net>
//
// TAH
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tah_regs {
    pub revid: u32,
    pub pad: [u32; 3],
    pub mr: u32,
    pub ssr0: u32,
    pub ssr1: u32,
    pub ssr2: u32,
    pub ssr3: u32,
    pub ssr4: u32,
    pub ssr5: u32,
    pub tsr: u32,
}

// TAH device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tah_instance {
    pub base: *mut tah_regs __iomem,
// Only one EMAC whacks us at a time
    pub lock: mutex,
// number of EMACs using this TAH
    pub users: c_int,
// OF device instance
    pub ofdev: *mut platform_device,
}

// TAH engine
pub const TAH_MR_CVR: c_uint = 0x80000000;
pub const TAH_MR_SR: c_uint = 0x40000000;
pub const TAH_MR_ST_256: c_uint = 0x01000000;
pub const TAH_MR_ST_512: c_uint = 0x02000000;
pub const TAH_MR_ST_768: c_uint = 0x03000000;
pub const TAH_MR_ST_1024: c_uint = 0x04000000;
pub const TAH_MR_ST_1280: c_uint = 0x05000000;
pub const TAH_MR_ST_1536: c_uint = 0x06000000;
pub const TAH_MR_TFS_16KB: c_uint = 0x00000000;
pub const TAH_MR_TFS_2KB: c_uint = 0x00200000;
pub const TAH_MR_TFS_4KB: c_uint = 0x00400000;
pub const TAH_MR_TFS_6KB: c_uint = 0x00600000;
pub const TAH_MR_TFS_8KB: c_uint = 0x00800000;
pub const TAH_MR_TFS_10KB: c_uint = 0x00a00000;
pub const TAH_MR_DTFP: c_uint = 0x00100000;
pub const TAH_MR_DIG: c_uint = 0x00080000;

extern "C" {
    pub fn tah_init() -> c_int;
}
extern "C" {
    pub fn tah_exit();
}
extern "C" {
    pub fn tah_attach(ofdev: *mut platform_device, channel: c_int) -> c_int;
}
extern "C" {
    pub fn tah_detach(ofdev: *mut platform_device, channel: c_int);
}
extern "C" {
    pub fn tah_reset(ofdev: *mut platform_device);
}
extern "C" {
    pub fn tah_get_regs_len(ofdev: *mut platform_device) -> c_int;
}

