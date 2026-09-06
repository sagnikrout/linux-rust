//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ibm/emac/rgmii.h
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
// drivers/net/ethernet/ibm/emac/rgmii.h
//
// Driver for PowerPC 4xx on-chip ethernet controller, RGMII bridge support.
//
// Copyright 2007 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//
// Based on the arch/ppc version of the driver:
//
// Based on ocp_zmii.h/ibm_emac_zmii.h
// Armin Kuster akuster@mvista.com
//
// Copyright 2004 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//
// Copyright (c) 2004, 2005 Zultys Technologies.
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
//
// RGMII bridge type
pub const RGMII_STANDARD: c_int = 0;
pub const RGMII_AXON: c_int = 1;
// RGMII bridge
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rgmii_regs {
    pub /: *mut *mut u32 fer; / Function enable register,
    pub /: *mut *mut u32 ssr; / Speed select register,
}

// RGMII device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rgmii_instance {
    pub base: *mut rgmii_regs __iomem,
// RGMII bridge flags
    pub flags: c_int,
pub const EMAC_RGMII_FLAG_HAS_MDIO: c_uint = 0x00000001;
// Only one EMAC whacks us at a time
    pub lock: mutex,
// number of EMACs using this RGMII bridge
    pub users: c_int,
// OF device instance
    pub ofdev: *mut platform_device,
}

extern "C" {
    pub fn rgmii_init() -> c_int;
}
extern "C" {
    pub fn rgmii_exit();
}
extern "C" {
    pub fn rgmii_attach(ofdev: *mut platform_device, input: c_int, mode: c_int) -> c_int;
}
extern "C" {
    pub fn rgmii_detach(ofdev: *mut platform_device, input: c_int);
}
extern "C" {
    pub fn rgmii_get_mdio(ofdev: *mut platform_device, input: c_int);
}
extern "C" {
    pub fn rgmii_put_mdio(ofdev: *mut platform_device, input: c_int);
}
extern "C" {
    pub fn rgmii_set_speed(ofdev: *mut platform_device, input: c_int, speed: c_int);
}
extern "C" {
    pub fn rgmii_get_regs_len(ofdev: *mut platform_device) -> c_int;
}

