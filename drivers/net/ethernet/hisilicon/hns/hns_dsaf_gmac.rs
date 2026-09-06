//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns/hns_dsaf_gmac.h
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
// Copyright (c) 2014-2015 Hisilicon Limited.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_port_mode {
    GMAC_10M_MII = 0,
    GMAC_100M_MII,
    GMAC_1000M_GMII,
    GMAC_10M_RGMII,
    GMAC_100M_RGMII,
    GMAC_1000M_RGMII,
    GMAC_10M_SGMII,
    GMAC_100M_SGMII,
    GMAC_1000M_SGMII,
    GMAC_10000M_SGMII	/* 10GE */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_gmac_duplex_mdoe {
    GMAC_HALF_DUPLEX_MODE = 0,
    GMAC_FULL_DUPLEX_MODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_gmac_port_mode_cfg {
    pub port_mode: hns_port_mode,
    pub max_frm_size: u32,
    pub short_runts_thr: u32,
    pub pad_enable: u32,
    pub crc_add: u32,
    pub /: *mut *mut u32 an_enable; /auto-nego enable,
    pub runt_pkt_en: u32,
    pub strip_pad_en: u32,
}

pub const ETH_GMAC_DUMP_NUM: c_int = 96;
