//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/rpm.h
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
// Marvell CN10K RPM driver
//
// Copyright (C) 2020 Marvell.
//

// PCI device IDs
pub const PCI_DEVID_CN10K_RPM: c_uint = 0xA060;
pub const PCI_SUBSYS_DEVID_CNF10KB_RPM: c_uint = 0xBC00;
pub const PCI_DEVID_CN10KB_RPM: c_uint = 0xA09F;
// Registers
pub const RPMX_CMRX_CFG: c_uint = 0x00;
pub const RPMX_CMR_GLOBAL_CFG: c_uint = 0x08;

pub const RPMX_CMRX_RX_ID_MAP: c_uint = 0x80;
pub const RPMX_CMRX_SW_INT: c_uint = 0x180;
pub const RPMX_CMRX_SW_INT_W1S: c_uint = 0x188;
pub const RPMX_CMRX_SW_INT_ENA_W1S: c_uint = 0x198;
pub const RPMX_CMRX_LINK_CFG: c_uint = 0x1070;
pub const RPMX_MTI_PCS100X_CONTROL1: c_uint = 0x20000;

pub const RPMX_MTI_MAC100X_COMMAND_CONFIG: c_uint = 0x8010;

pub const RPMX_MTI_MAC100X_CL01_PAUSE_QUANTA: c_uint = 0x80A8;
pub const RPMX_MTI_MAC100X_CL23_PAUSE_QUANTA: c_uint = 0x80B0;
pub const RPMX_MTI_MAC100X_CL45_PAUSE_QUANTA: c_uint = 0x80B8;
pub const RPMX_MTI_MAC100X_CL67_PAUSE_QUANTA: c_uint = 0x80C0;
pub const RPMX_MTI_MAC100X_CL01_QUANTA_THRESH: c_uint = 0x80C8;
pub const RPMX_MTI_MAC100X_CL23_QUANTA_THRESH: c_uint = 0x80D0;
pub const RPMX_MTI_MAC100X_CL45_QUANTA_THRESH: c_uint = 0x80D8;
pub const RPMX_MTI_MAC100X_CL67_QUANTA_THRESH: c_uint = 0x80E0;
pub const RPMX_MTI_MAC100X_CL89_PAUSE_QUANTA: c_uint = 0x8108;
pub const RPMX_MTI_MAC100X_CL1011_PAUSE_QUANTA: c_uint = 0x8110;
pub const RPMX_MTI_MAC100X_CL1213_PAUSE_QUANTA: c_uint = 0x8118;
pub const RPMX_MTI_MAC100X_CL1415_PAUSE_QUANTA: c_uint = 0x8120;
pub const RPMX_MTI_MAC100X_CL89_QUANTA_THRESH: c_uint = 0x8128;
pub const RPMX_MTI_MAC100X_CL1011_QUANTA_THRESH: c_uint = 0x8130;
pub const RPMX_MTI_MAC100X_CL1213_QUANTA_THRESH: c_uint = 0x8138;
pub const RPMX_MTI_MAC100X_CL1415_QUANTA_THRESH: c_uint = 0x8140;
pub const RPMX_CMR_RX_OVR_BP: c_uint = 0x4120;

pub const RPMX_CMR_CHAN_MSK_OR: c_uint = 0x4118;
pub const RPMX_MTI_STAT_RX_STAT_PAGES_COUNTERX: c_uint = 0x12000;
pub const RPMX_MTI_STAT_TX_STAT_PAGES_COUNTERX: c_uint = 0x13000;
pub const RPMX_MTI_STAT_DATA_HI_CDC: c_uint = 0x10038;
pub const RPM_LMAC_FWI: c_uint = 0xa;

pub const RPMX_CMRX_PRT_CBFC_CTL: c_uint = 0x5B08;
pub const RPMX_CMRX_PRT_CBFC_CTL_LOGL_EN_RX_SHIFT: c_int = 33;
pub const RPMX_CMRX_PRT_CBFC_CTL_PHYS_BP_SHIFT: c_int = 16;
pub const RPMX_CMRX_PRT_CBFC_CTL_LOGL_EN_TX_SHIFT: c_int = 0;

pub const RPMX_MTI_MAC100X_CL89_QUANTA_THRESH: c_uint = 0x8128;

pub const RPMX_MTI_MAC100X_CL01_PAUSE_QUANTA: c_uint = 0x80A8;
pub const RPMX_MTI_MAC100X_CL89_PAUSE_QUANTA: c_uint = 0x8108;
pub const RPM_DEFAULT_PAUSE_TIME: c_uint = 0x7FF;
pub const RPMX_CMRX_RX_LOGL_XON: c_uint = 0x4100;
pub const RPMX_MTI_MAC100X_XIF_MODE: c_uint = 0x8100;

pub const RPMX_CONST1: c_uint = 0x2008;
// FEC stats
pub const RPMX_MTI_STAT_STATN_CONTROL: c_uint = 0x10018;
pub const RPMX_MTI_STAT_DATA_HI_CDC: c_uint = 0x10038;

pub const RPMX_MTI_RSFEC_STAT_STATN_CONTROL: c_uint = 0x40018;
pub const RPMX_MTI_RSFEC_STAT_FAST_DATA_HI_CDC: c_uint = 0x40000;
pub const RPMX_MTI_RSFEC_STAT_COUNTER_CAPTURE_2: c_uint = 0x40050;
pub const RPMX_MTI_RSFEC_STAT_COUNTER_CAPTURE_3: c_uint = 0x40058;

// CN10KB CSR Declaration
pub const RPM2_CMRX_SW_INT: c_uint = 0x1b0;
pub const RPM2_CMRX_SW_INT_ENA_W1S: c_uint = 0x1c8;
pub const RPM2_LMAC_FWI: c_uint = 0x12;
pub const RPM2_CMR_CHAN_MSK_OR: c_uint = 0x3120;

pub const RPM2_CMR_RX_OVR_BP: c_uint = 0x3130;
pub const RPM2_CSR_OFFSET: c_uint = 0x3e00;
pub const RPM2_CMRX_PRT_CBFC_CTL: c_uint = 0x6510;
pub const RPM2_CMRX_RX_LMACS: c_uint = 0x100;
pub const RPM2_CMRX_RX_LOGL_XON: c_uint = 0x3100;
pub const RPM2_CMRX_RX_STAT2: c_uint = 0x3010;
pub const RPM2_USX_PCSX_CONTROL1: c_uint = 0x80000;

// Function Declarations
extern "C" {
    pub fn rpm_get_nr_lmacs(rpmd: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rpm_get_lmac_type(rpmd: *mut c_void, lmac_id: c_int) -> u8;
}
extern "C" {
    pub fn rpm_get_lmac_fifo_len(rpmd: *mut c_void, lmac_id: c_int) -> u32;
}
extern "C" {
    pub fn rpm2_get_lmac_fifo_len(rpmd: *mut c_void, lmac_id: c_int) -> u32;
}
extern "C" {
    pub fn rpm_lmac_internal_loopback(rpmd: *mut c_void, lmac_id: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn rpm_lmac_enadis_rx_pause_fwding(rpmd: *mut c_void, lmac_id: c_int, enable: bool);
}
extern "C" {
    pub fn rpm_lmac_pause_frm_config(rpmd: *mut c_void, lmac_id: c_int, enable: bool);
}
extern "C" {
    pub fn rpm_get_tx_stats(rpmd: *mut c_void, lmac_id: c_int, idx: c_int, tx_stat: *mut u64) -> c_int;
}
extern "C" {
    pub fn rpm_get_rx_stats(rpmd: *mut c_void, lmac_id: c_int, idx: c_int, rx_stat: *mut u64) -> c_int;
}
extern "C" {
    pub fn rpm_lmac_ptp_config(rpmd: *mut c_void, lmac_id: c_int, enable: bool);
}
extern "C" {
    pub fn rpm_lmac_rx_tx_enable(rpmd: *mut c_void, lmac_id: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn rpm_lmac_tx_enable(rpmd: *mut c_void, lmac_id: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn rpm2_get_nr_lmacs(rpmd: *mut c_void) -> c_int;
}
extern "C" {
    pub fn is_dev_rpm2(rpmd: *mut c_void) -> bool;
}
extern "C" {
    pub fn rpm_get_fec_stats(cgxd: *mut c_void, lmac_id: c_int, rsp: *mut cgx_fec_stats_rsp) -> c_int;
}
extern "C" {
    pub fn rpm_lmac_reset(rpmd: *mut c_void, lmac_id: c_int, pf_req_flr: u8) -> c_int;
}
extern "C" {
    pub fn rpm_stats_reset(rpmd: *mut c_void, lmac_id: c_int) -> c_int;
}
extern "C" {
    pub fn rpm_x2p_reset(rpmd: *mut c_void, enable: bool);
}
extern "C" {
    pub fn rpm_enadis_rx(rpmd: *mut c_void, lmac_id: c_int, enable: bool) -> c_int;
}
