//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192se/def.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
pub const RX_MPDU_QUEUE: c_int = 0;
pub const RX_CMD_QUEUE: c_int = 1;
pub const SHORT_SLOT_TIME: c_int = 9;
pub const NON_SHORT_SLOT_TIME: c_int = 20;
// Queue Select Value in TxDesc
pub const QSLT_BK: c_uint = 0x2;
pub const QSLT_BE: c_uint = 0x0;
pub const QSLT_VI: c_uint = 0x5;
pub const QSLT_VO: c_uint = 0x6;
pub const QSLT_BEACON: c_uint = 0x10;
pub const QSLT_HIGH: c_uint = 0x11;
pub const QSLT_MGNT: c_uint = 0x12;
pub const QSLT_CMD: c_uint = 0x13;
// Tx Desc

// macros to read/write various fields in RX or TX descriptors
// Dword 0
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(31)) -> return;
}
// Dword 1
// Dword 2
// Dword 3
// Dword 4
// Dword 5
// Dword 7
// Dword 8
// (__pdesc + 8) = cpu_to_le32(__val);
extern "C" {
    pub fn le32_to_cpu(8)): *mut *mut ((__pdesc +) -> return;
}
// Dword 9
// (__pdesc + 9) = cpu_to_le32(__val);
// Because the PCI Tx descriptors are chaied at the
// initialization and all the NextDescAddresses in
// these descriptors cannot not be cleared (,or
// driver/HW cannot find the next descriptor), the
// offset 36 (NextDescAddresses) is reserved when
// the desc is cleared.
pub const TX_DESC_NEXT_DESC_OFFSET: c_int = 36;

// Rx Desc
pub const RX_STATUS_DESC_SIZE: c_int = 24;
pub const RX_DRV_INFO_SIZE_UNIT: c_int = 8;
// DWORD 0
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: GENMASK(13, _arg: 0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(14)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(15)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: GENMASK(19, _arg: 16)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: GENMASK(25, _arg: 24)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(26)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(27)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut (__pdesc), _arg: BIT(31)) -> return;
}
// DWORD 1
extern "C" {
    pub fn le32_get_bits(1): *mut *mut (__pdesc +, _arg: BIT(14)) -> return;
}
extern "C" {
    pub fn le32_get_bits(1): *mut *mut (__pdesc +, _arg: BIT(15)) -> return;
}
// DWORD 3
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: GENMASK(5, _arg: 0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(6)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(8)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(9)) -> return;
}
// DWORD 5
extern "C" {
    pub fn le32_to_cpu(5)): *mut *mut ((__pdesc +) -> return;
}
// DWORD 6
// (__pdesc + 6) = cpu_to_le32(__val);
extern "C" {
    pub fn le32_to_cpu(6): *mut *mut (__pdesc +) -> return;
}
// Macro flag: #define SE_RX_HAL_IS_CCK_RATE(_pdesc)\
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rf_optype {
    RF_OP_BY_SW_3WIRE = 0,
    RF_OP_BY_FW,
    RF_OP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ic_inferiority {
    IC_INFERIORITY_A = 0,
    IC_INFERIORITY_B = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fwcmd_iotype {
// For DIG DM
    FW_CMD_DIG_ENABLE = 0,
    FW_CMD_DIG_DISABLE = 1,
    FW_CMD_DIG_HALT = 2,
    FW_CMD_DIG_RESUME = 3,
// For High Power DM
    FW_CMD_HIGH_PWR_ENABLE = 4,
    FW_CMD_HIGH_PWR_DISABLE = 5,
// For Rate adaptive DM
    FW_CMD_RA_RESET = 6,
    FW_CMD_RA_ACTIVE = 7,
    FW_CMD_RA_REFRESH_N = 8,
    FW_CMD_RA_REFRESH_BG = 9,
    FW_CMD_RA_INIT = 10,
// For FW supported IQK
    FW_CMD_IQK_INIT = 11,
// Tx power tracking switch,
// MP driver only
    FW_CMD_TXPWR_TRACK_ENABLE = 12,
// Tx power tracking switch,
// MP driver only
    FW_CMD_TXPWR_TRACK_DISABLE = 13,
// Tx power tracking with thermal
// indication, for Normal driver
    FW_CMD_TXPWR_TRACK_THERMAL = 14,
    FW_CMD_PAUSE_DM_BY_SCAN = 15,
    FW_CMD_RESUME_DM_BY_SCAN = 16,
    FW_CMD_RA_REFRESH_N_COMB = 17,
    FW_CMD_RA_REFRESH_BG_COMB = 18,
    FW_CMD_ANTENNA_SW_ENABLE = 19,
    FW_CMD_ANTENNA_SW_DISABLE = 20,
// Tx Status report for CCX from FW
    FW_CMD_TX_FEEDBACK_CCX_ENABLE = 21,
// Indifate firmware that driver
// enters LPS, For PS-Poll issue
    FW_CMD_LPS_ENTER = 22,
// Indicate firmware that driver
// leave LPS
    FW_CMD_LPS_LEAVE = 23,
// Set DIG mode to signal strength
    FW_CMD_DIG_MODE_SS = 24,
// Set DIG mode to false alarm.
    FW_CMD_DIG_MODE_FA = 25,
    FW_CMD_ADD_A2_ENTRY = 26,
    FW_CMD_CTRL_DM_BY_DRIVER = 27,
    FW_CMD_CTRL_DM_BY_DRIVER_NEW = 28,
    FW_CMD_PAPE_CONTROL = 29,
    FW_CMD_IQK_ENABLE = 30,
}

// Driver info contain PHY status
// and other variabel size info
// PHY Status content as below
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_fwinfo {
// DWORD 0
    pub gain_trsw: [u8; 4],
// DWORD 1
    pub pwdb_all: u8,
    pub cfosho: [u8; 4],
// DWORD 2
    pub cfotail: [u8; 4],
// DWORD 3
    pub rxevm: [i8; 2],
    pub rxsnr: [i8; 4],
// DWORD 4
    pub pdsnr: [u8; 2],
// DWORD 5
    pub csi_current: [u8; 2],
    pub csi_target: [u8; 2],
// DWORD 6
    pub sigevm: u8,
    pub max_ex_pwr: u8,
    pub ex_intf_flag:1: u8,
    pub sgi_en:1: u8,
    pub rxsc:2: u8,
    pub reserve:4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_sts_cck_8192s_t {
    pub adc_pwdb_x: [u8; 4],
    pub sq_rpt: u8,
    pub cck_agc_rpt: u8,
}
