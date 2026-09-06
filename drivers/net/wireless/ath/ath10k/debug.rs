//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/debug.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2017 Qualcomm Atheros, Inc.
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_debug_mask {
    ATH10K_DBG_PCI		= 0x00000001,
    ATH10K_DBG_WMI		= 0x00000002,
    ATH10K_DBG_HTC		= 0x00000004,
    ATH10K_DBG_HTT		= 0x00000008,
    ATH10K_DBG_MAC		= 0x00000010,
    ATH10K_DBG_BOOT		= 0x00000020,
    ATH10K_DBG_PCI_DUMP	= 0x00000040,
    ATH10K_DBG_HTT_DUMP	= 0x00000080,
    ATH10K_DBG_MGMT		= 0x00000100,
    ATH10K_DBG_DATA		= 0x00000200,
    ATH10K_DBG_BMI		= 0x00000400,
    ATH10K_DBG_REGULATORY	= 0x00000800,
    ATH10K_DBG_TESTMODE	= 0x00001000,
    ATH10K_DBG_WMI_PRINT	= 0x00002000,
    ATH10K_DBG_PCI_PS	= 0x00004000,
    ATH10K_DBG_AHB		= 0x00008000,
    ATH10K_DBG_SDIO		= 0x00010000,
    ATH10K_DBG_SDIO_DUMP	= 0x00020000,
    ATH10K_DBG_USB		= 0x00040000,
    ATH10K_DBG_USB_BULK	= 0x00080000,
    ATH10K_DBG_SNOC		= 0x00100000,
    ATH10K_DBG_QMI		= 0x00200000,
    ATH10K_DBG_STA		= 0x00400000,
    ATH10K_DBG_ANY		= 0xffffffff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_pktlog_filter {
    ATH10K_PKTLOG_RX         = 0x000000001,
    ATH10K_PKTLOG_TX         = 0x000000002,
    ATH10K_PKTLOG_RCFIND     = 0x000000004,
    ATH10K_PKTLOG_RCUPDATE   = 0x000000008,
    ATH10K_PKTLOG_DBG_PRINT  = 0x000000010,
    ATH10K_PKTLOG_PEER_STATS = 0x000000040,
    ATH10K_PKTLOG_ANY        = 0x00000005f,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_dbg_aggr_mode {
    ATH10K_DBG_AGGR_MODE_AUTO,
    ATH10K_DBG_AGGR_MODE_MANUAL,
    ATH10K_DBG_AGGR_MODE_MAX,
}

// Types of packet log events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_pktlog_type {
    ATH_PKTLOG_TYPE_TX_CTRL = 1,
    ATH_PKTLOG_TYPE_TX_STAT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_pktlog_hdr {
    pub flags: __le16,
    pub missed_cnt: __le16,
    pub /: *mut *mut __le16 log_type; / Type of log information foll this header,
    pub /: *mut *mut __le16 size; / Size of variable length log information in bytes,
    pub timestamp: __le32,
    pub payload: [u8; ],
    pub __packed: },
// FIXME: How to calculate the buffer size sanely?

pub const ATH10K_TX_POWER_MAX_VAL: c_int = 70;
pub const ATH10K_TX_POWER_MIN_VAL: c_int = 0;
    pub ath10k_debug_mask: extern unsigned int,
    pub ...): *const *const *const __printf(2, 3) void ath10k_info(struct ath10k ar, char fmt,,
    pub ...): *const *const *const __printf(2, 3) void ath10k_err(struct ath10k ar, char fmt,,
    pub ...): *const *const *const __printf(2, 3) void ath10k_warn(struct ath10k ar, char fmt,,
    pub ar): *mut void ath10k_debug_print_hwfw_info(struct ath10k,
    pub ar): *mut void ath10k_debug_print_board_info(struct ath10k,
    pub ar): *mut void ath10k_debug_print_boot_info(struct ath10k,
    pub ar): *mut void ath10k_print_driver_info(struct ath10k,

    pub ar): *mut int ath10k_debug_start(struct ath10k,
    pub ar): *mut void ath10k_debug_stop(struct ath10k,
    pub ar): *mut int ath10k_debug_create(struct ath10k,
    pub ar): *mut void ath10k_debug_destroy(struct ath10k,
    pub ar): *mut int ath10k_debug_register(struct ath10k,
    pub ar): *mut void ath10k_debug_unregister(struct ath10k,
    pub skb): *mut *mut void ath10k_debug_fw_stats_process(struct ath10k ar, struct sk_buff,
    pub tpc_stats): *mut ath10k_tpc_stats,
    pub tpc_stats): *mut ath10k_tpc_stats_final,
    pub len): *mut *mut *mut void ath10k_debug_dbglog_add(struct ath10k ar, u8 buffer, int,

    pub data): *mut u32 sset, u8,
    pub sset): *mut *mut ieee80211_vif vif, int,
    pub data): *mut *mut ethtool_stats stats, u64,
    pub ar->debug.fw_dbglog_mask: return,
    pub ar->debug.fw_dbglog_level: return,
    pub ar->debug.enable_extd_tx_stats: return,
    pub ar): *mut int ath10k_debug_fw_stats_request(struct ath10k,

    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,

    pub dir): *mut *mut ieee80211_sta sta, dentry,
    pub stats): *mut ath10k_fw_stats,
    pub queued_msdus): c_ulong,
    pub num_ranges): c_int,

    pub ...): *const *const char fmt,,
    pub len): *const *const void buf, size_t,

    pub 0: return,

// Avoid calling __ath10k_dbg() if debug_mask is not set and tracing
// disabled.
//

    pub \: __ath10k_dbg(ar, dbg_mask, fmt, ##__VA_ARGS__);,
