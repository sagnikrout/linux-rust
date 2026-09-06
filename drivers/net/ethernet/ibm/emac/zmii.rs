//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ibm/emac/zmii.h
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
// drivers/net/ethernet/ibm/emac/zmii.h
//
// Driver for PowerPC 4xx on-chip ethernet controller, ZMII bridge support.
//
// Copyright 2007 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//
// Based on the arch/ppc version of the driver:
//
// Copyright (c) 2004, 2005 Zultys Technologies.
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
//
// Based on original work by
// Armin Kuster <akuster@mvista.com>
// Copyright 2001 MontaVista Softare Inc.
//
// ZMII bridge registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zmii_regs {
    pub /: *mut *mut u32 fer; / Function enable reg,
    pub /: *mut *mut u32 ssr; / Speed select reg,
    pub /: *mut *mut u32 smiirs; / SMII status reg,
}

// ZMII device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zmii_instance {
    pub base: *mut zmii_regs __iomem,
// Only one EMAC whacks us at a time
    pub lock: mutex,
// subset of PHY_MODE_XXXX
    pub mode: c_int,
// number of EMACs using this ZMII bridge
    pub users: c_int,
// FER value left by firmware
    pub fer_save: u32,
// OF device instance
    pub ofdev: *mut platform_device,
}

extern "C" {
    pub fn zmii_init() -> c_int;
}
extern "C" {
    pub fn zmii_exit();
}
extern "C" {
    pub fn zmii_detach(ofdev: *mut platform_device, input: c_int);
}
extern "C" {
    pub fn zmii_get_mdio(ofdev: *mut platform_device, input: c_int);
}
extern "C" {
    pub fn zmii_put_mdio(ofdev: *mut platform_device, input: c_int);
}
extern "C" {
    pub fn zmii_set_speed(ofdev: *mut platform_device, input: c_int, speed: c_int);
}
extern "C" {
    pub fn zmii_get_regs_len(ocpdev: *mut platform_device) -> c_int;
}

