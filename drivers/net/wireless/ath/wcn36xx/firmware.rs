//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wcn36xx/firmware.h
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
// Capability bitmap exchange definitions and macros starts
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_firmware_feat_caps {
    MCC = 0,
    P2P = 1,
    DOT11AC = 2,
    SLM_SESSIONIZATION = 3,
    DOT11AC_OPMODE = 4,
    SAP32STA = 5,
    TDLS = 6,
    P2P_GO_NOA_DECOUPLE_INIT_SCAN = 7,
    WLANACTIVE_OFFLOAD = 8,
    BEACON_OFFLOAD = 9,
    SCAN_OFFLOAD = 10,
    ROAM_OFFLOAD = 11,
    BCN_MISS_OFFLOAD = 12,
    STA_POWERSAVE = 13,
    STA_ADVANCED_PWRSAVE = 14,
    AP_UAPSD = 15,
    AP_DFS = 16,
    BLOCKACK = 17,
    PHY_ERR = 18,
    BCN_FILTER = 19,
    RTT = 20,
    RATECTRL = 21,
    WOW = 22,
    WLAN_ROAM_SCAN_OFFLOAD = 23,
    SPECULATIVE_PS_POLL = 24,
    SCAN_SCH = 25,
    IBSS_HEARTBEAT_OFFLOAD = 26,
    WLAN_SCAN_OFFLOAD = 27,
    WLAN_PERIODIC_TX_PTRN = 28,
    ADVANCE_TDLS = 29,
    BATCH_SCAN = 30,
    FW_IN_TX_PATH = 31,
    EXTENDED_NSOFFLOAD_SLOT = 32,
    CH_SWITCH_V1 = 33,
    HT40_OBSS_SCAN = 34,
    UPDATE_CHANNEL_LIST = 35,
    WLAN_MCADDR_FLT = 36,
    WLAN_CH144 = 37,
    NAN = 38,
    TDLS_SCAN_COEXISTENCE = 39,
    LINK_LAYER_STATS_MEAS = 40,
    MU_MIMO = 41,
    EXTENDED_SCAN = 42,
    DYNAMIC_WMM_PS = 43,
    MAC_SPOOFED_SCAN = 44,
    BMU_ERROR_GENERIC_RECOVERY = 45,
    DISA = 46,
    FW_STATS = 47,
    WPS_PRBRSP_TMPL = 48,
    BCN_IE_FLT_DELTA = 49,
    TDLS_OFF_CHANNEL = 51,
    RTT3 = 52,
    MGMT_FRAME_LOGGING = 53,
    ENHANCED_TXBD_COMPLETION = 54,
    LOGGING_ENHANCEMENT = 55,
    EXT_SCAN_ENHANCED = 56,
    MEMORY_DUMP_SUPPORTED = 57,
    PER_PKT_STATS_SUPPORTED = 58,
    EXT_LL_STAT = 60,
    WIFI_CONFIG = 61,
    ANTENNA_DIVERSITY_SELECTION = 62,

    MAX_FEATURE_SUPPORTED = 128,
}
