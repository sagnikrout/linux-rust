//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/mdio/mdio-cavium.h
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
// Copyright (C) 2009-2016 Cavium, Inc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cavium_mdiobus_mode {
    UNINIT = 0,
    C22,
    C45
}

pub const SMI_CMD: c_uint = 0x0;
pub const SMI_WR_DAT: c_uint = 0x8;
pub const SMI_RD_DAT: c_uint = 0x10;
pub const SMI_CLK: c_uint = 0x18;
pub const SMI_EN: c_uint = 0x20;

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_smix_clk {
    pub u64: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_smix_clk_s {
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_smix_cmd {
    pub u64: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_smix_cmd_s {
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_smix_en {
    pub u64: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_smix_en_s {
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_smix_rd_dat {
    pub u64: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_smix_rd_dat_s {
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_smix_wr_dat {
    pub u64: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_smix_wr_dat_s {
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cavium_mdiobus {
    pub mii_bus: *mut mii_bus,
    pub register_base: *mut void __iomem,
    pub mode: cavium_mdiobus_mode,
}

extern "C" {
    pub fn cvmx_read_csr()addr: (u64) -> return;
}

extern "C" {
    pub fn cavium_mdiobus_read_c22(bus: *mut mii_bus, phy_id: c_int, regnum: c_int) -> c_int;
}
