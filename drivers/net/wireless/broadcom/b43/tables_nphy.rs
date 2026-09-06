//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/tables_nphy.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_n_sfo_cfg {
    pub phy_bw1a: u16,
    pub phy_bw2: u16,
    pub phy_bw3: u16,
    pub phy_bw4: u16,
    pub phy_bw5: u16,
    pub phy_bw6: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_txiqcal_ladder {
    pub percent: u8,
    pub g_env: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_rf_control_override_rev2 {
    pub addr0: u8,
    pub addr1: u8,
    pub bmask: u16,
    pub shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_rf_control_override_rev3 {
    pub val_mask: u16,
    pub val_shift: u8,
    pub en_addr0: u8,
    pub val_addr0: u8,
    pub en_addr1: u8,
    pub val_addr1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_rf_control_override_rev7 {
    pub field: u16,
    pub val_addr_core0: u16,
    pub val_addr_core1: u16,
    pub val_mask: u16,
    pub val_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_gain_ctl_workaround_entry {
    pub lna1_gain: [i8; 4],
    pub lna2_gain: [i8; 4],
    pub gain_db: [u8; 10],
    pub gain_bits: [u8; 10],
    pub init_gain: u16,
    pub rfseq_init: [u16; 4],
    pub cliphi_gain: u16,
    pub clipmd_gain: u16,
    pub cliplo_gain: u16,
    pub crsmin: u16,
    pub crsminl: u16,
    pub crsminu: u16,
    pub nbclip: u16,
    pub wlclip: u16,
}

// Get entry with workaround values for gain ctl. Does not return NULL.
// The N-PHY tables.
pub const B43_NTAB_TYPEMASK: c_uint = 0xF0000000;
pub const B43_NTAB_8BIT: c_uint = 0x10000000;
pub const B43_NTAB_16BIT: c_uint = 0x20000000;
pub const B43_NTAB_32BIT: c_uint = 0x30000000;

// Static N-PHY tables

pub const B43_NTAB_FRAMESTRUCT_SIZE: c_int = 832;

pub const B43_NTAB_FRAMELT_SIZE: c_int = 32;

pub const B43_NTAB_TMAP_SIZE: c_int = 448;

pub const B43_NTAB_TDTRN_SIZE: c_int = 704;

pub const B43_NTAB_INTLEVEL_SIZE: c_int = 7;

pub const B43_NTAB_PILOT_SIZE: c_int = 88;

pub const B43_NTAB_PILOTLT_SIZE: c_int = 6;

pub const B43_NTAB_TDI20A0_SIZE: c_int = 55;

pub const B43_NTAB_TDI20A1_SIZE: c_int = 55;

pub const B43_NTAB_TDI40A0_SIZE: c_int = 110;

pub const B43_NTAB_TDI40A1_SIZE: c_int = 110;

pub const B43_NTAB_BDI_SIZE: c_int = 6;

pub const B43_NTAB_CHANEST_SIZE: c_int = 96;

pub const B43_NTAB_MCS_SIZE: c_int = 128;
// Volatile N-PHY tables

pub const B43_NTAB_NOISEVAR10_SIZE: c_int = 256;

pub const B43_NTAB_NOISEVAR11_SIZE: c_int = 256;

pub const B43_NTAB_C0_ESTPLT_SIZE: c_int = 64;

pub const B43_NTAB_C0_ADJPLT_SIZE: c_int = 128;

pub const B43_NTAB_C0_GAINCTL_SIZE: c_int = 128;

pub const B43_NTAB_C0_IQLT_SIZE: c_int = 128;

pub const B43_NTAB_C0_LOFEEDTH_SIZE: c_int = 128;

pub const B43_NTAB_C1_ESTPLT_SIZE: c_int = 64;

pub const B43_NTAB_C1_ADJPLT_SIZE: c_int = 128;

pub const B43_NTAB_C1_GAINCTL_SIZE: c_int = 128;

pub const B43_NTAB_C1_IQLT_SIZE: c_int = 128;

pub const B43_NTAB_C1_LOFEEDTH_SIZE: c_int = 128;
// Volatile N-PHY tables, PHY revision >= 3

// Static N-PHY tables, PHY revision >= 3

// Static N-PHY tables, PHY revision >= 7

pub const B43_NTAB_TX_IQLO_CAL_LOFT_LADDER_40_SIZE: c_int = 18;
pub const B43_NTAB_TX_IQLO_CAL_LOFT_LADDER_20_SIZE: c_int = 18;
pub const B43_NTAB_TX_IQLO_CAL_IQIMB_LADDER_40_SIZE: c_int = 18;
pub const B43_NTAB_TX_IQLO_CAL_IQIMB_LADDER_20_SIZE: c_int = 18;
pub const B43_NTAB_TX_IQLO_CAL_STARTCOEFS_REV3: c_int = 11;
pub const B43_NTAB_TX_IQLO_CAL_STARTCOEFS: c_int = 9;
pub const B43_NTAB_TX_IQLO_CAL_CMDS_RECAL_REV3: c_int = 12;
pub const B43_NTAB_TX_IQLO_CAL_CMDS_RECAL: c_int = 10;
pub const B43_NTAB_TX_IQLO_CAL_CMDS_FULLCAL: c_int = 10;
pub const B43_NTAB_TX_IQLO_CAL_CMDS_FULLCAL_REV3: c_int = 12;
extern "C" {
    pub fn b43_ntab_read(dev: *mut b43_wldev, offset: u32) -> u32;
}
extern "C" {
    pub fn b43_ntab_write(dev: *mut b43_wldev, offset: u32, value: u32);
}
extern "C" {
    pub fn b43_nphy_tables_init(dev: *mut b43_wldev);
}
