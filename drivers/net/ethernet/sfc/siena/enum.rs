//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/enum.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Driver for Solarflare network controllers and boards
// Copyright 2007-2013 Solarflare Communications Inc.
//
// enum efx_loopback_mode - loopback modes
// @LOOPBACK_NONE: no loopback
// @LOOPBACK_DATA: data path loopback
// @LOOPBACK_GMAC: loopback within GMAC
// @LOOPBACK_XGMII: loopback after XMAC
// @LOOPBACK_XGXS: loopback within BPX after XGXS
// @LOOPBACK_XAUI: loopback within BPX before XAUI serdes
// @LOOPBACK_GMII: loopback within BPX after GMAC
// @LOOPBACK_SGMII: loopback within BPX within SGMII
// @LOOPBACK_XGBR: loopback within BPX within XGBR
// @LOOPBACK_XFI: loopback within BPX before XFI serdes
// @LOOPBACK_XAUI_FAR: loopback within BPX after XAUI serdes
// @LOOPBACK_GMII_FAR: loopback within BPX before SGMII
// @LOOPBACK_SGMII_FAR: loopback within BPX after SGMII
// @LOOPBACK_XFI_FAR: loopback after XFI serdes
// @LOOPBACK_GPHY: loopback within 1G PHY at unspecified level
// @LOOPBACK_PHYXS: loopback within 10G PHY at PHYXS level
// @LOOPBACK_PCS: loopback within 10G PHY at PCS level
// @LOOPBACK_PMAPMD: loopback within 10G PHY at PMAPMD level
// @LOOPBACK_XPORT: cross port loopback
// @LOOPBACK_XGMII_WS: wireside loopback excluding XMAC
// @LOOPBACK_XAUI_WS: wireside loopback within BPX within XAUI serdes
// @LOOPBACK_XAUI_WS_FAR: wireside loopback within BPX including XAUI serdes
// @LOOPBACK_XAUI_WS_NEAR: wireside loopback within BPX excluding XAUI serdes
// @LOOPBACK_GMII_WS: wireside loopback excluding GMAC
// @LOOPBACK_XFI_WS: wireside loopback excluding XFI serdes
// @LOOPBACK_XFI_WS_FAR: wireside loopback including XFI serdes
// @LOOPBACK_PHYXS_WS: wireside loopback within 10G PHY at PHYXS level
//
// Please keep up-to-date w.r.t the following two #defines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_loopback_mode {
    LOOPBACK_NONE = 0,
    LOOPBACK_DATA = 1,
    LOOPBACK_GMAC = 2,
    LOOPBACK_XGMII = 3,
    LOOPBACK_XGXS = 4,
    LOOPBACK_XAUI = 5,
    LOOPBACK_GMII = 6,
    LOOPBACK_SGMII = 7,
    LOOPBACK_XGBR = 8,
    LOOPBACK_XFI = 9,
    LOOPBACK_XAUI_FAR = 10,
    LOOPBACK_GMII_FAR = 11,
    LOOPBACK_SGMII_FAR = 12,
    LOOPBACK_XFI_FAR = 13,
    LOOPBACK_GPHY = 14,
    LOOPBACK_PHYXS = 15,
    LOOPBACK_PCS = 16,
    LOOPBACK_PMAPMD = 17,
    LOOPBACK_XPORT = 18,
    LOOPBACK_XGMII_WS = 19,
    LOOPBACK_XAUI_WS = 20,
    LOOPBACK_XAUI_WS_FAR = 21,
    LOOPBACK_XAUI_WS_NEAR = 22,
    LOOPBACK_GMII_WS = 23,
    LOOPBACK_XFI_WS = 24,
    LOOPBACK_XFI_WS_FAR = 25,
    LOOPBACK_PHYXS_WS = 26,
    LOOPBACK_MAX
}

// These loopbacks occur within the controller

//
// enum reset_type - reset types
//
// %RESET_TYPE_INVSIBLE, %RESET_TYPE_ALL, %RESET_TYPE_WORLD and
// %RESET_TYPE_DISABLE specify the method/scope of the reset.  The
// other valuesspecify reasons, which efx_siena_schedule_reset() will choose
// a method for.
//
// Reset methods are numbered in order of increasing scope.
//
// @RESET_TYPE_INVISIBLE: Reset datapath and MAC (Falcon only)
// @RESET_TYPE_RECOVER_OR_ALL: Try to recover. Apply RESET_TYPE_ALL
// if unsuccessful.
// @RESET_TYPE_ALL: Reset datapath, MAC and PHY
// @RESET_TYPE_WORLD: Reset as much as possible
// @RESET_TYPE_RECOVER_OR_DISABLE: Try to recover. Apply RESET_TYPE_DISABLE if
// unsuccessful.
// @RESET_TYPE_DATAPATH: Reset datapath only.
// @RESET_TYPE_MC_BIST: MC entering BIST mode.
// @RESET_TYPE_DISABLE: Reset datapath, MAC and PHY; leave NIC disabled
// @RESET_TYPE_TX_WATCHDOG: reset due to TX watchdog
// @RESET_TYPE_INT_ERROR: reset due to internal error
// @RESET_TYPE_DMA_ERROR: DMA error
// @RESET_TYPE_TX_SKIP: hardware completed empty tx descriptors
// @RESET_TYPE_MC_FAILURE: MC reboot/assertion
// @RESET_TYPE_MCDI_TIMEOUT: MCDI timeout.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reset_type {
    RESET_TYPE_INVISIBLE,
    RESET_TYPE_RECOVER_OR_ALL,
    RESET_TYPE_ALL,
    RESET_TYPE_WORLD,
    RESET_TYPE_RECOVER_OR_DISABLE,
    RESET_TYPE_DATAPATH,
    RESET_TYPE_MC_BIST,
    RESET_TYPE_DISABLE,
    RESET_TYPE_MAX_METHOD,
    RESET_TYPE_TX_WATCHDOG,
    RESET_TYPE_INT_ERROR,
    RESET_TYPE_DMA_ERROR,
    RESET_TYPE_TX_SKIP,
    RESET_TYPE_MC_FAILURE,
// RESET_TYPE_MCDI_TIMEOUT is actually a method, not just a reason, but
// it doesn't fit the scope hierarchy (not well-ordered by inclusion).
// We encode this by having its enum value be greater than
// RESET_TYPE_MAX_METHOD.
//
    RESET_TYPE_MCDI_TIMEOUT,
    RESET_TYPE_MAX,
}
