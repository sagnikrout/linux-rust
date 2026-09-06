//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/cgx_fw_if.h
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
// Marvell OcteonTx2 CGX driver
//
// Copyright (C) 2018 Marvell.
//

pub const CGX_FIRMWARE_MAJOR_VER: c_int = 1;
pub const CGX_FIRMWARE_MINOR_VER: c_int = 0;

// CGX error types. set for cmd response status as CGX_STAT_FAIL
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgx_error_type {
    CGX_ERR_NONE,
    CGX_ERR_LMAC_NOT_ENABLED,
    CGX_ERR_LMAC_MODE_INVALID,
    CGX_ERR_REQUEST_ID_INVALID,
    CGX_ERR_PREV_ACK_NOT_CLEAR,
    CGX_ERR_PHY_LINK_DOWN,
    CGX_ERR_PCS_RESET_FAIL,
    CGX_ERR_AN_CPT_FAIL,
    CGX_ERR_TX_NOT_IDLE,
    CGX_ERR_RX_NOT_IDLE,
    CGX_ERR_SPUX_BR_BLKLOCK_FAIL,
    CGX_ERR_SPUX_RX_ALIGN_FAIL,
    CGX_ERR_SPUX_TX_FAULT,
    CGX_ERR_SPUX_RX_FAULT,
    CGX_ERR_SPUX_RESET_FAIL,
    CGX_ERR_SPUX_AN_RESET_FAIL,
    CGX_ERR_SPUX_USX_AN_RESET_FAIL,
    CGX_ERR_SMUX_RX_LINK_NOT_OK,
    CGX_ERR_PCS_RECV_LINK_FAIL,
    CGX_ERR_TRAINING_FAIL,
    CGX_ERR_RX_EQU_FAIL,
    CGX_ERR_SPUX_BER_FAIL,
    CGX_ERR_SPUX_RSFEC_ALGN_FAIL,
    CGX_ERR_SPUX_MARKER_LOCK_FAIL,
    CGX_ERR_SET_FEC_INVALID,
    CGX_ERR_SET_FEC_FAIL,
    CGX_ERR_MODULE_INVALID,
    CGX_ERR_MODULE_NOT_PRESENT,
    CGX_ERR_SPEED_CHANGE_INVALID,
}

// LINK speed types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgx_link_speed {
    CGX_LINK_NONE,
    CGX_LINK_10M,
    CGX_LINK_100M,
    CGX_LINK_1G,
    CGX_LINK_2HG,
    CGX_LINK_5G,
    CGX_LINK_10G,
    CGX_LINK_20G,
    CGX_LINK_25G,
    CGX_LINK_40G,
    CGX_LINK_50G,
    CGX_LINK_80G,
    CGX_LINK_100G,
    CGX_LINK_SPEED_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CGX_MODE_ {
    CGX_MODE_SGMII,
    CGX_MODE_1000_BASEX,
    CGX_MODE_QSGMII,
    CGX_MODE_10G_C2C,
    CGX_MODE_10G_C2M,
    CGX_MODE_10G_KR,
    CGX_MODE_20G_C2C,
    CGX_MODE_25G_C2C,
    CGX_MODE_25G_C2M,
    CGX_MODE_25G_2_C2C,
    CGX_MODE_25G_CR,
    CGX_MODE_25G_KR,
    CGX_MODE_40G_C2C,
    CGX_MODE_40G_C2M,
    CGX_MODE_40G_CR4,
    CGX_MODE_40G_KR4,
    CGX_MODE_40GAUI_C2C,
    CGX_MODE_50G_C2C,
    CGX_MODE_50G_C2M,
    CGX_MODE_50G_4_C2C,
    CGX_MODE_50G_CR,
    CGX_MODE_50G_KR,
    CGX_MODE_80GAUI_C2C,
    CGX_MODE_100G_C2C,
    CGX_MODE_100G_C2M,
    CGX_MODE_100G_CR4,
    CGX_MODE_100G_KR4,
    CGX_MODE_LAUI_2_C2C_BIT,
    CGX_MODE_LAUI_2_C2M_BIT,
    CGX_MODE_50GBASE_CR2_C_BIT,
    CGX_MODE_50GBASE_KR2_C_BIT,     /* = 30 */
    CGX_MODE_100GAUI_2_C2C_BIT,
    CGX_MODE_100GAUI_2_C2M_BIT,
    CGX_MODE_100GBASE_CR2_BIT,
    CGX_MODE_100GBASE_KR2_BIT,
    CGX_MODE_SFI_1G_BIT,
    CGX_MODE_25GBASE_CR_C_BIT,
    CGX_MODE_25GBASE_KR_C_BIT,
    CGX_MODE_SGMII_10M_BIT,
    CGX_MODE_SGMII_100M_BIT,        /* = 39 */
    CGX_MODE_2500_BASEX_BIT = 42, /* Mode group 1 */
    CGX_MODE_5000_BASEX_BIT,
    CGX_MODE_O_USGMII_BIT,
    CGX_MODE_Q_USGMII_BIT,
    CGX_MODE_2_5G_USXGMII_BIT,
    CGX_MODE_5G_USXGMII_BIT,
    CGX_MODE_10G_SXGMII_BIT,
    CGX_MODE_10G_DXGMII_BIT,
    CGX_MODE_10G_QXGMII_BIT,
    CGX_MODE_TP_BIT,
    CGX_MODE_FIBER_BIT,
    CGX_MODE_MAX /* = 53 */
}

// REQUEST ID types. Input to firmware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgx_cmd_id {
    CGX_CMD_NONE,
    CGX_CMD_GET_FW_VER,
    CGX_CMD_GET_MAC_ADDR,
    CGX_CMD_SET_MTU,
    CGX_CMD_GET_LINK_STS,		/* optional to user */
    CGX_CMD_LINK_BRING_UP,
    CGX_CMD_LINK_BRING_DOWN,
    CGX_CMD_INTERNAL_LBK,
    CGX_CMD_EXTERNAL_LBK,
    CGX_CMD_HIGIG,
    CGX_CMD_LINK_STAT_CHANGE,
    CGX_CMD_MODE_CHANGE,		/* hot plug support */
    CGX_CMD_INTF_SHUTDOWN,
    CGX_CMD_GET_MKEX_PRFL_SIZE,
    CGX_CMD_GET_MKEX_PRFL_ADDR,
    CGX_CMD_GET_FWD_BASE,		/* get base address of shared FW data */
    CGX_CMD_GET_LINK_MODES,		/* Supported Link Modes */
    CGX_CMD_SET_LINK_MODE,
    CGX_CMD_GET_SUPPORTED_FEC,
    CGX_CMD_SET_FEC,
    CGX_CMD_GET_AN,
    CGX_CMD_SET_AN,
    CGX_CMD_GET_ADV_LINK_MODES,
    CGX_CMD_GET_ADV_FEC,
    CGX_CMD_GET_PHY_MOD_TYPE, /* line-side modulation type: NRZ or PAM4 */
    CGX_CMD_SET_PHY_MOD_TYPE,
    CGX_CMD_PRBS,
    CGX_CMD_DISPLAY_EYE,
    CGX_CMD_GET_PHY_FEC_STATS,
}

// async event ids
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgx_evt_id {
    CGX_EVT_NONE,
    CGX_EVT_LINK_CHANGE,
}

// event types - cause of interrupt
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgx_evt_type {
    CGX_EVT_ASYNC,
    CGX_EVT_CMD_RESP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgx_stat {
    CGX_STAT_SUCCESS,
    CGX_STAT_FAIL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgx_cmd_own {
    CGX_CMD_OWN_NS,
    CGX_CMD_OWN_FIRMWARE,
}

// m - bit mask
// y - value to be written in the bitrange
// x - input value whose bitrange to be modified
//

// scratchx(0) CSR used for ATF->non-secure SW communication.
// This acts as the status register
// Provides details on command ack/status, command response, error details
//

// Response to command IDs with command status as CGX_STAT_FAIL
//
// Not applicable for commands :
// CGX_CMD_LINK_BRING_UP/DOWN/CGX_EVT_LINK_CHANGE
//

// Response to cmd ID as CGX_CMD_GET_FW_VER with cmd status as
// CGX_STAT_SUCCESS
//

// Response to cmd ID as CGX_CMD_GET_MAC_ADDR with cmd status as
// CGX_STAT_SUCCESS
//

// Response to cmd ID as CGX_CMD_GET_MKEX_PRFL_SIZE with cmd status as
// CGX_STAT_SUCCESS
//

// Response to cmd ID as CGX_CMD_GET_MKEX_PRFL_ADDR with cmd status as
// CGX_STAT_SUCCESS
//

// Response to cmd ID as CGX_CMD_GET_FWD_BASE with cmd status as
// CGX_STAT_SUCCESS
//

// Response to cmd ID - CGX_CMD_LINK_BRING_UP/DOWN, event ID CGX_EVT_LINK_CHANGE
// status can be either CGX_STAT_FAIL or CGX_STAT_SUCCESS
//
// In case of CGX_STAT_FAIL, it indicates CGX configuration failed
// when processing link up/down/change command.
// Both err_type and current link status will be updated
//
// In case of CGX_STAT_SUCCESS, err_type will be CGX_ERR_NONE and current
// link status will be updated
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_lnk_sts {
    pub reserved1:9: u64,
    pub link_up:1: u64,
    pub full_duplex:1: u64,
    pub /: *mut *mut uint64_t speed:4; / cgx_link_speed,
    pub err_type:10: u64,
    pub /: *mut *mut uint64_t an:1; / AN supported or not,
    pub /: *mut *mut uint64_t fec:2; / FEC type if enabled, if not 0,
    pub port:8: u64,
    pub reserved2:28: u64,
}

// scratchx(1) CSR used for non-secure SW->ATF communication
// This CSR acts as a command register
//

// Any command using enable/disable as an argument need
// to set this bitfield.
// Ex: Loopback, HiGig...
//

// command argument to be passed for cmd ID - CGX_CMD_SET_MTU

// command argument to be passed for cmd ID - CGX_CMD_LINK_CHANGE

// command argument to be passed for cmd ID - CGX_CMD_MODE_CHANGE

// this field categorize the mode ID(FLAGS) range to accommodate
// more modes.
// To specify mode ID range of 0 - 41, this field will be 0.
// To specify mode ID range of 42 - 83, this field will be 1.
//

// LINK_BRING_UP command timeout

