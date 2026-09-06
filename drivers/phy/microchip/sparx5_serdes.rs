//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/microchip/sparx5_serdes.h
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 SerDes driver
//
// Copyright (c) 2020 Microchip Technology Inc.
//

pub const SPX5_SERDES_MAX: c_int = 33;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_serdes_type {
    SPX5_SDT_6G  = 6,
    SPX5_SDT_10G = 10,
    SPX5_SDT_25G = 25,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_serdes_mode {
    SPX5_SD_MODE_NONE,
    SPX5_SD_MODE_2G5,
    SPX5_SD_MODE_QSGMII,
    SPX5_SD_MODE_100FX,
    SPX5_SD_MODE_1000BASEX,
    SPX5_SD_MODE_SFI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_10g28cmu_mode {
    SPX5_SD10G28_CMU_MAIN = 0,
    SPX5_SD10G28_CMU_AUX1 = 1,
    SPX5_SD10G28_CMU_AUX2 = 3,
    SPX5_SD10G28_CMU_NONE = 4,
    SPX5_SD10G28_CMU_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_target {
    SPX5_TARGET_SPARX5,
    SPX5_TARGET_LAN969X,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_serdes_macro {
    pub priv: *mut sparx5_serdes_private,
    pub sidx: u32,
    pub stpidx: u32,
    pub serdestype: sparx5_serdes_type,
    pub serdesmode: sparx5_serdes_mode,
    pub portmode: phy_interface_t,
    pub speed: c_int,
    pub media: phy_media,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_serdes_consts {
    pub sd_max: c_int,
    pub cmu_max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_serdes_ops {
    pub sidx): *mut *mut *mut void (serdes_type_set)(struct sparx5_serdes_macro macro, int,
    pub sd_index): *mut *mut int (serdes_cmu_get)(enum sparx5_10g28cmu_mode mode, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_serdes_match_data {
    pub type: sparx5_target,
    pub consts: sparx5_serdes_consts,
    pub ops: sparx5_serdes_ops,
    pub iomap: *const sparx5_serdes_io_resource,
    pub iomap_size: c_int,
    pub tsize: *const c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_serdes_private {
    pub dev: *mut device,
    pub regs: [*mut void __iomem; NUM_TARGETS],
    pub phys: [*mut phy; SPX5_SERDES_MAX],
    pub coreclock: c_ulong,
    pub data: *const sparx5_serdes_match_data,
}

// Read, Write and modify registers content.
// The register definition macros start at the id
//
