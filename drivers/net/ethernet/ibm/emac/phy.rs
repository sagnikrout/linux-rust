//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ibm/emac/phy.h
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
// drivers/net/ethernet/ibm/emac/phy.h
//
// Driver for PowerPC 4xx on-chip ethernet controller, PHY support
//
// Copyright 2007 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//
// Based on the arch/ppc version of the driver:
//
// Benjamin Herrenschmidt <benh@kernel.crashing.org>
// February 2003
//
// Minor additions by Eugene Surovegin <ebs@ebshome.net>, 2004
//
// This file basically duplicates sungem_phy.{c,h} with different PHYs
// supported. I'm looking into merging that in a single mii layer more
// flexible than mii.c
//
// Operations supported by any kind of PHY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_phy_ops {
    pub phy): *mut *mut *mut int (init) (struct mii_phy,
    pub wol_options): *mut *mut *mut int (suspend) (struct mii_phy  phy, int,
    pub advertise): *mut *mut *mut int (setup_aneg) (struct mii_phy  phy, u32,
    pub fd): *mut *mut *mut int (setup_forced) (struct mii_phy  phy, int speed, int,
    pub phy): *mut *mut *mut int (poll_link) (struct mii_phy,
    pub phy): *mut *mut *mut int (read_link) (struct mii_phy,
}

// Structure used to statically define an mii/gii based PHY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_phy_def {
    pub /: *mut *mut u32 phy_id; / Concatenated ID1 << 16 | ID2,
    pub /: *mut *mut u32 phy_id_mask; / Significant bits,
    pub or: *mut *mut *mut u32 features; / Ethtool SUPPORTED_ defines,
    pub /: *mut *mut int magic_aneg; / Autoneg does all speed test for us,
    pub name: *const c_char,
    pub ops: *const mii_phy_ops,
}

// An instance of a PHY, partially borrowed from mii_if_info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_phy {
    pub def: *mut mii_phy_def,
    pub /: *mut *mut *mut u32 advertising; / Ethtool ADVERTISED_ defines,
    pub mii_phy_def.features: *mut *mut u32 features; / Copied from,
    pub /: *mut *mut int address; / PHY address,
    pub /: *mut *mut int mode; / PHY mode,
    pub /: *mut *mut int gpcs_address; / GPCS PHY address,
// 1: autoneg enabled, 0: disabled
    pub autoneg: c_int,
// forced speed & duplex (no autoneg)
// partner speed & duplex & pause (autoneg)
//
    pub speed: c_int,
    pub duplex: c_int,
    pub pause: c_int,
    pub asym_pause: c_int,
// Provided by host chip
    pub dev: *mut net_device,
    pub reg): *mut *mut *mut int (mdio_read) (struct net_device  dev, int addr, int,
    pub val): c_int,
}

// Pass in a struct mii_phy with dev, mdio_read and mdio_write
// filled, the remaining fields will be filled on return
//
extern "C" {
    pub fn emac_mii_phy_probe(phy: *mut mii_phy, address: c_int) -> c_int;
}
extern "C" {
    pub fn emac_mii_reset_phy(phy: *mut mii_phy) -> c_int;
}
extern "C" {
    pub fn emac_mii_reset_gpcs(phy: *mut mii_phy) -> c_int;
}
