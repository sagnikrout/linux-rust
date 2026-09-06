//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/wmi.h
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
// Copyright (c) 2018-2019, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

//
// This file specifies the WMI interface for the Unified Software
// Architecture.
//
// It includes definitions of all the commands and events. Commands are
// messages from the host to the target. Events and Replies are messages
// from the target to the host.
//
// Ownership of correctness in regards to WMI commands belongs to the host
// driver and the target is not required to validate parameters for value,
// proper range, or any other checking.
//
// Guidelines for extending this interface are below.
//
// 1. Add new WMI commands ONLY within the specified range - 0x9000 - 0x9fff
//
// 2. Use ONLY u32 type for defining member variables within WMI
// command/event structures. Do not use u8, u16, bool or
// enum types within these structures.
//
// 3. DO NOT define bit fields within structures. Implement bit fields
// using masks if necessary. Do not use the programming language's bit
// field definition.
//
// 4. Define macros for encode/decode of u8, u16 fields within
// the u32 variables. Use these macros for set/get of these fields.
// Try to use this to optimize the structure without bloating it with
// u32 variables for every lower sized field.
//
// 5. Do not use PACK/UNPACK attributes for the structures as each member
// variable is already 4-byte aligned by virtue of being a u32
// type.
//
// 6. Comment each parameter part of the WMI command/event structure by
// using the 2 stars at the beginning of C comment instead of one star to
// enable HTML document generation using Doxygen.
//
// Control Path
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cmd_hdr {
    pub cmd_id: __le32,
    pub __packed: },
pub const WMI_CMD_HDR_CMD_ID_MASK: c_uint = 0x00FFFFFF;
pub const WMI_CMD_HDR_CMD_ID_LSB: c_int = 0;
pub const WMI_CMD_HDR_PLT_PRIV_MASK: c_uint = 0xFF000000;
pub const WMI_CMD_HDR_PLT_PRIV_LSB: c_int = 24;
pub const HTC_PROTOCOL_VERSION: c_uint = 0x0002;
pub const WMI_PROTOCOL_VERSION: c_uint = 0x0002;
//
// There is no signed version of __le32, so for a temporary solution come
// up with our own version. The idea is from fs/ntfs/endian.h.
//
// Use a_ prefix so that it doesn't conflict if we get proper support to
// linux/types.h.
//
pub type a_sle32 = __s32 ;
    pub a_sle32)cpu_to_le32(val): return (,
    pub __le32)val): return le32_to_cpu((,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_service {
    WMI_SERVICE_BEACON_OFFLOAD = 0,
    WMI_SERVICE_SCAN_OFFLOAD,
    WMI_SERVICE_ROAM_OFFLOAD,
    WMI_SERVICE_BCN_MISS_OFFLOAD,
    WMI_SERVICE_STA_PWRSAVE,
    WMI_SERVICE_STA_ADVANCED_PWRSAVE,
    WMI_SERVICE_AP_UAPSD,
    WMI_SERVICE_AP_DFS,
    WMI_SERVICE_11AC,
    WMI_SERVICE_BLOCKACK,
    WMI_SERVICE_PHYERR,
    WMI_SERVICE_BCN_FILTER,
    WMI_SERVICE_RTT,
    WMI_SERVICE_RATECTRL,
    WMI_SERVICE_WOW,
    WMI_SERVICE_RATECTRL_CACHE,
    WMI_SERVICE_IRAM_TIDS,
    WMI_SERVICE_ARPNS_OFFLOAD,
    WMI_SERVICE_NLO,
    WMI_SERVICE_GTK_OFFLOAD,
    WMI_SERVICE_SCAN_SCH,
    WMI_SERVICE_CSA_OFFLOAD,
    WMI_SERVICE_CHATTER,
    WMI_SERVICE_COEX_FREQAVOID,
    WMI_SERVICE_PACKET_POWER_SAVE,
    WMI_SERVICE_FORCE_FW_HANG,
    WMI_SERVICE_GPIO,
    WMI_SERVICE_STA_DTIM_PS_MODULATED_DTIM,
    WMI_SERVICE_STA_UAPSD_BASIC_AUTO_TRIG,
    WMI_SERVICE_STA_UAPSD_VAR_AUTO_TRIG,
    WMI_SERVICE_STA_KEEP_ALIVE,
    WMI_SERVICE_TX_ENCAP,
    WMI_SERVICE_BURST,
    WMI_SERVICE_SMART_ANTENNA_SW_SUPPORT,
    WMI_SERVICE_SMART_ANTENNA_HW_SUPPORT,
    WMI_SERVICE_ROAM_SCAN_OFFLOAD,
    WMI_SERVICE_AP_PS_DETECT_OUT_OF_SYNC,
    WMI_SERVICE_EARLY_RX,
    WMI_SERVICE_STA_SMPS,
    WMI_SERVICE_FWTEST,
    WMI_SERVICE_STA_WMMAC,
    WMI_SERVICE_TDLS,
    WMI_SERVICE_MCC_BCN_INTERVAL_CHANGE,
    WMI_SERVICE_ADAPTIVE_OCS,
    WMI_SERVICE_BA_SSN_SUPPORT,
    WMI_SERVICE_FILTER_IPSEC_NATKEEPALIVE,
    WMI_SERVICE_WLAN_HB,
    WMI_SERVICE_LTE_ANT_SHARE_SUPPORT,
    WMI_SERVICE_BATCH_SCAN,
    WMI_SERVICE_QPOWER,
    WMI_SERVICE_PLMREQ,
    WMI_SERVICE_THERMAL_MGMT,
    WMI_SERVICE_RMC,
    WMI_SERVICE_MHF_OFFLOAD,
    WMI_SERVICE_COEX_SAR,
    WMI_SERVICE_BCN_TXRATE_OVERRIDE,
    WMI_SERVICE_NAN,
    WMI_SERVICE_L1SS_STAT,
    WMI_SERVICE_ESTIMATE_LINKSPEED,
    WMI_SERVICE_OBSS_SCAN,
    WMI_SERVICE_TDLS_OFFCHAN,
    WMI_SERVICE_TDLS_UAPSD_BUFFER_STA,
    WMI_SERVICE_TDLS_UAPSD_SLEEP_STA,
    WMI_SERVICE_IBSS_PWRSAVE,
    WMI_SERVICE_LPASS,
    WMI_SERVICE_EXTSCAN,
    WMI_SERVICE_D0WOW,
    WMI_SERVICE_HSOFFLOAD,
    WMI_SERVICE_ROAM_HO_OFFLOAD,
    WMI_SERVICE_RX_FULL_REORDER,
    WMI_SERVICE_DHCP_OFFLOAD,
    WMI_SERVICE_STA_RX_IPA_OFFLOAD_SUPPORT,
    WMI_SERVICE_MDNS_OFFLOAD,
    WMI_SERVICE_SAP_AUTH_OFFLOAD,
    WMI_SERVICE_ATF,
    WMI_SERVICE_COEX_GPIO,
    WMI_SERVICE_ENHANCED_PROXY_STA,
    WMI_SERVICE_TT,
    WMI_SERVICE_PEER_CACHING,
    WMI_SERVICE_AUX_SPECTRAL_INTF,
    WMI_SERVICE_AUX_CHAN_LOAD_INTF,
    WMI_SERVICE_BSS_CHANNEL_INFO_64,
    WMI_SERVICE_EXT_RES_CFG_SUPPORT,
    WMI_SERVICE_MESH_11S,
    WMI_SERVICE_MESH_NON_11S,
    WMI_SERVICE_PEER_STATS,
    WMI_SERVICE_RESTRT_CHNL_SUPPORT,
    WMI_SERVICE_PERIODIC_CHAN_STAT_SUPPORT,
    WMI_SERVICE_TX_MODE_PUSH_ONLY,
    WMI_SERVICE_TX_MODE_PUSH_PULL,
    WMI_SERVICE_TX_MODE_DYNAMIC,
    WMI_SERVICE_VDEV_RX_FILTER,
    WMI_SERVICE_BTCOEX,
    WMI_SERVICE_CHECK_CAL_VERSION,
    WMI_SERVICE_DBGLOG_WARN2,
    WMI_SERVICE_BTCOEX_DUTY_CYCLE,
    WMI_SERVICE_4_WIRE_COEX_SUPPORT,
    WMI_SERVICE_EXTENDED_NSS_SUPPORT,
    WMI_SERVICE_PROG_GPIO_BAND_SELECT,
    WMI_SERVICE_SMART_LOGGING_SUPPORT,
    WMI_SERVICE_TDLS_CONN_TRACKER_IN_HOST_MODE,
    WMI_SERVICE_TDLS_EXPLICIT_MODE_ONLY,
    WMI_SERVICE_MGMT_TX_WMI,
    WMI_SERVICE_TDLS_WIDER_BANDWIDTH,
    WMI_SERVICE_HTT_MGMT_TX_COMP_VALID_FLAGS,
    WMI_SERVICE_HOST_DFS_CHECK_SUPPORT,
    WMI_SERVICE_TPC_STATS_FINAL,
    WMI_SERVICE_RESET_CHIP,
    WMI_SERVICE_SPOOF_MAC_SUPPORT,
    WMI_SERVICE_TX_DATA_ACK_RSSI,
    WMI_SERVICE_VDEV_DIFFERENT_BEACON_INTERVAL_SUPPORT,
    WMI_SERVICE_VDEV_DISABLE_4_ADDR_SRC_LRN_SUPPORT,
    WMI_SERVICE_BB_TIMING_CONFIG_SUPPORT,
    WMI_SERVICE_THERM_THROT,
    WMI_SERVICE_RTT_RESPONDER_ROLE,
    WMI_SERVICE_PER_PACKET_SW_ENCRYPT,
    WMI_SERVICE_REPORT_AIRTIME,
    WMI_SERVICE_SYNC_DELETE_CMDS,
    WMI_SERVICE_TX_PWR_PER_PEER,
    WMI_SERVICE_SUPPORT_EXTEND_ADDRESS,
    WMI_SERVICE_PEER_TID_CONFIGS_SUPPORT,
    WMI_SERVICE_EXT_PEER_TID_CONFIGS_SUPPORT,

// Remember to add the new value to wmi_service_name()!

// keep last
    WMI_SERVICE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10x_service {
    WMI_10X_SERVICE_BEACON_OFFLOAD = 0,
    WMI_10X_SERVICE_SCAN_OFFLOAD,
    WMI_10X_SERVICE_ROAM_OFFLOAD,
    WMI_10X_SERVICE_BCN_MISS_OFFLOAD,
    WMI_10X_SERVICE_STA_PWRSAVE,
    WMI_10X_SERVICE_STA_ADVANCED_PWRSAVE,
    WMI_10X_SERVICE_AP_UAPSD,
    WMI_10X_SERVICE_AP_DFS,
    WMI_10X_SERVICE_11AC,
    WMI_10X_SERVICE_BLOCKACK,
    WMI_10X_SERVICE_PHYERR,
    WMI_10X_SERVICE_BCN_FILTER,
    WMI_10X_SERVICE_RTT,
    WMI_10X_SERVICE_RATECTRL,
    WMI_10X_SERVICE_WOW,
    WMI_10X_SERVICE_RATECTRL_CACHE,
    WMI_10X_SERVICE_IRAM_TIDS,
    WMI_10X_SERVICE_BURST,

// introduced in 10.2
    WMI_10X_SERVICE_SMART_ANTENNA_SW_SUPPORT,
    WMI_10X_SERVICE_FORCE_FW_HANG,
    WMI_10X_SERVICE_SMART_ANTENNA_HW_SUPPORT,
    WMI_10X_SERVICE_ATF,
    WMI_10X_SERVICE_COEX_GPIO,
    WMI_10X_SERVICE_AUX_SPECTRAL_INTF,
    WMI_10X_SERVICE_AUX_CHAN_LOAD_INTF,
    WMI_10X_SERVICE_BSS_CHANNEL_INFO_64,
    WMI_10X_SERVICE_MESH,
    WMI_10X_SERVICE_EXT_RES_CFG_SUPPORT,
    WMI_10X_SERVICE_PEER_STATS,
    WMI_10X_SERVICE_RESET_CHIP,
    WMI_10X_SERVICE_HTT_MGMT_TX_COMP_VALID_FLAGS,
    WMI_10X_SERVICE_VDEV_BCN_RATE_CONTROL,
    WMI_10X_SERVICE_PER_PACKET_SW_ENCRYPT,
    WMI_10X_SERVICE_BB_TIMING_CONFIG_SUPPORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_main_service {
    WMI_MAIN_SERVICE_BEACON_OFFLOAD = 0,
    WMI_MAIN_SERVICE_SCAN_OFFLOAD,
    WMI_MAIN_SERVICE_ROAM_OFFLOAD,
    WMI_MAIN_SERVICE_BCN_MISS_OFFLOAD,
    WMI_MAIN_SERVICE_STA_PWRSAVE,
    WMI_MAIN_SERVICE_STA_ADVANCED_PWRSAVE,
    WMI_MAIN_SERVICE_AP_UAPSD,
    WMI_MAIN_SERVICE_AP_DFS,
    WMI_MAIN_SERVICE_11AC,
    WMI_MAIN_SERVICE_BLOCKACK,
    WMI_MAIN_SERVICE_PHYERR,
    WMI_MAIN_SERVICE_BCN_FILTER,
    WMI_MAIN_SERVICE_RTT,
    WMI_MAIN_SERVICE_RATECTRL,
    WMI_MAIN_SERVICE_WOW,
    WMI_MAIN_SERVICE_RATECTRL_CACHE,
    WMI_MAIN_SERVICE_IRAM_TIDS,
    WMI_MAIN_SERVICE_ARPNS_OFFLOAD,
    WMI_MAIN_SERVICE_NLO,
    WMI_MAIN_SERVICE_GTK_OFFLOAD,
    WMI_MAIN_SERVICE_SCAN_SCH,
    WMI_MAIN_SERVICE_CSA_OFFLOAD,
    WMI_MAIN_SERVICE_CHATTER,
    WMI_MAIN_SERVICE_COEX_FREQAVOID,
    WMI_MAIN_SERVICE_PACKET_POWER_SAVE,
    WMI_MAIN_SERVICE_FORCE_FW_HANG,
    WMI_MAIN_SERVICE_GPIO,
    WMI_MAIN_SERVICE_STA_DTIM_PS_MODULATED_DTIM,
    WMI_MAIN_SERVICE_STA_UAPSD_BASIC_AUTO_TRIG,
    WMI_MAIN_SERVICE_STA_UAPSD_VAR_AUTO_TRIG,
    WMI_MAIN_SERVICE_STA_KEEP_ALIVE,
    WMI_MAIN_SERVICE_TX_ENCAP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_4_service {
    WMI_10_4_SERVICE_BEACON_OFFLOAD = 0,
    WMI_10_4_SERVICE_SCAN_OFFLOAD,
    WMI_10_4_SERVICE_ROAM_OFFLOAD,
    WMI_10_4_SERVICE_BCN_MISS_OFFLOAD,
    WMI_10_4_SERVICE_STA_PWRSAVE,
    WMI_10_4_SERVICE_STA_ADVANCED_PWRSAVE,
    WMI_10_4_SERVICE_AP_UAPSD,
    WMI_10_4_SERVICE_AP_DFS,
    WMI_10_4_SERVICE_11AC,
    WMI_10_4_SERVICE_BLOCKACK,
    WMI_10_4_SERVICE_PHYERR,
    WMI_10_4_SERVICE_BCN_FILTER,
    WMI_10_4_SERVICE_RTT,
    WMI_10_4_SERVICE_RATECTRL,
    WMI_10_4_SERVICE_WOW,
    WMI_10_4_SERVICE_RATECTRL_CACHE,
    WMI_10_4_SERVICE_IRAM_TIDS,
    WMI_10_4_SERVICE_BURST,
    WMI_10_4_SERVICE_SMART_ANTENNA_SW_SUPPORT,
    WMI_10_4_SERVICE_GTK_OFFLOAD,
    WMI_10_4_SERVICE_SCAN_SCH,
    WMI_10_4_SERVICE_CSA_OFFLOAD,
    WMI_10_4_SERVICE_CHATTER,
    WMI_10_4_SERVICE_COEX_FREQAVOID,
    WMI_10_4_SERVICE_PACKET_POWER_SAVE,
    WMI_10_4_SERVICE_FORCE_FW_HANG,
    WMI_10_4_SERVICE_SMART_ANTENNA_HW_SUPPORT,
    WMI_10_4_SERVICE_GPIO,
    WMI_10_4_SERVICE_STA_UAPSD_BASIC_AUTO_TRIG,
    WMI_10_4_SERVICE_STA_UAPSD_VAR_AUTO_TRIG,
    WMI_10_4_SERVICE_STA_KEEP_ALIVE,
    WMI_10_4_SERVICE_TX_ENCAP,
    WMI_10_4_SERVICE_AP_PS_DETECT_OUT_OF_SYNC,
    WMI_10_4_SERVICE_EARLY_RX,
    WMI_10_4_SERVICE_ENHANCED_PROXY_STA,
    WMI_10_4_SERVICE_TT,
    WMI_10_4_SERVICE_ATF,
    WMI_10_4_SERVICE_PEER_CACHING,
    WMI_10_4_SERVICE_COEX_GPIO,
    WMI_10_4_SERVICE_AUX_SPECTRAL_INTF,
    WMI_10_4_SERVICE_AUX_CHAN_LOAD_INTF,
    WMI_10_4_SERVICE_BSS_CHANNEL_INFO_64,
    WMI_10_4_SERVICE_EXT_RES_CFG_SUPPORT,
    WMI_10_4_SERVICE_MESH_NON_11S,
    WMI_10_4_SERVICE_RESTRT_CHNL_SUPPORT,
    WMI_10_4_SERVICE_PEER_STATS,
    WMI_10_4_SERVICE_MESH_11S,
    WMI_10_4_SERVICE_PERIODIC_CHAN_STAT_SUPPORT,
    WMI_10_4_SERVICE_TX_MODE_PUSH_ONLY,
    WMI_10_4_SERVICE_TX_MODE_PUSH_PULL,
    WMI_10_4_SERVICE_TX_MODE_DYNAMIC,
    WMI_10_4_SERVICE_VDEV_RX_FILTER,
    WMI_10_4_SERVICE_BTCOEX,
    WMI_10_4_SERVICE_CHECK_CAL_VERSION,
    WMI_10_4_SERVICE_DBGLOG_WARN2,
    WMI_10_4_SERVICE_BTCOEX_DUTY_CYCLE,
    WMI_10_4_SERVICE_4_WIRE_COEX_SUPPORT,
    WMI_10_4_SERVICE_EXTENDED_NSS_SUPPORT,
    WMI_10_4_SERVICE_PROG_GPIO_BAND_SELECT,
    WMI_10_4_SERVICE_SMART_LOGGING_SUPPORT,
    WMI_10_4_SERVICE_TDLS,
    WMI_10_4_SERVICE_TDLS_OFFCHAN,
    WMI_10_4_SERVICE_TDLS_UAPSD_BUFFER_STA,
    WMI_10_4_SERVICE_TDLS_UAPSD_SLEEP_STA,
    WMI_10_4_SERVICE_TDLS_CONN_TRACKER_IN_HOST_MODE,
    WMI_10_4_SERVICE_TDLS_EXPLICIT_MODE_ONLY,
    WMI_10_4_SERVICE_TDLS_WIDER_BANDWIDTH,
    WMI_10_4_SERVICE_HTT_MGMT_TX_COMP_VALID_FLAGS,
    WMI_10_4_SERVICE_HOST_DFS_CHECK_SUPPORT,
    WMI_10_4_SERVICE_TPC_STATS_FINAL,
    WMI_10_4_SERVICE_CFR_CAPTURE_SUPPORT,
    WMI_10_4_SERVICE_TX_DATA_ACK_RSSI,
    WMI_10_4_SERVICE_CFR_CAPTURE_IND_MSG_TYPE_LEGACY,
    WMI_10_4_SERVICE_PER_PACKET_SW_ENCRYPT,
    WMI_10_4_SERVICE_PEER_TID_CONFIGS_SUPPORT,
    WMI_10_4_SERVICE_VDEV_BCN_RATE_CONTROL,
    WMI_10_4_SERVICE_VDEV_DIFFERENT_BEACON_INTERVAL_SUPPORT,
    WMI_10_4_SERVICE_HTT_ASSERT_TRIGGER_SUPPORT,
    WMI_10_4_SERVICE_VDEV_FILTER_NEIGHBOR_RX_PACKETS,
    WMI_10_4_SERVICE_VDEV_DISABLE_4_ADDR_SRC_LRN_SUPPORT,
    WMI_10_4_SERVICE_PEER_CHWIDTH_CHANGE,
    WMI_10_4_SERVICE_RX_FILTER_OUT_COUNT,
    WMI_10_4_SERVICE_RTT_RESPONDER_ROLE,
    WMI_10_4_SERVICE_EXT_PEER_TID_CONFIGS_SUPPORT,
    WMI_10_4_SERVICE_REPORT_AIRTIME,
    WMI_10_4_SERVICE_TX_PWR_PER_PEER,
    WMI_10_4_SERVICE_FETCH_PEER_TX_PN,
    WMI_10_4_SERVICE_MULTIPLE_VDEV_RESTART,
    WMI_10_4_SERVICE_ENHANCED_RADIO_COUNTERS,
    WMI_10_4_SERVICE_QINQ_SUPPORT,
    WMI_10_4_SERVICE_RESET_CHIP,
}

    pub NULL: return,

    pub NULL: return,

// This extension is required to accommodate new services, current limit
// for wmi_services is 64 as target is using only 4-bits of each 32-bit
// wmi_service word. Extending this to make use of remaining unused bits
// for new services.
//

    pub \: __set_bit(y, out);,
    pub len): WMI_SERVICE_BEACON_OFFLOAD,,
    pub len): WMI_SERVICE_SCAN_OFFLOAD,,
    pub len): WMI_SERVICE_ROAM_OFFLOAD,,
    pub len): WMI_SERVICE_BCN_MISS_OFFLOAD,,
    pub len): WMI_SERVICE_STA_PWRSAVE,,
    pub len): WMI_SERVICE_STA_ADVANCED_PWRSAVE,,
    pub len): WMI_SERVICE_AP_UAPSD,,
    pub len): WMI_SERVICE_AP_DFS,,
    pub len): WMI_SERVICE_11AC,,
    pub len): WMI_SERVICE_BLOCKACK,,
    pub len): WMI_SERVICE_PHYERR,,
    pub len): WMI_SERVICE_BCN_FILTER,,
    pub len): WMI_SERVICE_RTT,,
    pub len): WMI_SERVICE_RATECTRL,,
    pub len): WMI_SERVICE_WOW,,
    pub len): WMI_SERVICE_RATECTRL_CACHE,,
    pub len): WMI_SERVICE_IRAM_TIDS,,
    pub len): WMI_SERVICE_BURST,,
    pub len): WMI_SERVICE_SMART_ANTENNA_SW_SUPPORT,,
    pub len): WMI_SERVICE_FORCE_FW_HANG,,
    pub len): WMI_SERVICE_SMART_ANTENNA_HW_SUPPORT,,
    pub len): WMI_SERVICE_ATF,,
    pub len): WMI_SERVICE_COEX_GPIO,,
    pub len): WMI_SERVICE_AUX_SPECTRAL_INTF,,
    pub len): WMI_SERVICE_AUX_CHAN_LOAD_INTF,,
    pub len): WMI_SERVICE_BSS_CHANNEL_INFO_64,,
    pub len): WMI_SERVICE_MESH_11S,,
    pub len): WMI_SERVICE_EXT_RES_CFG_SUPPORT,,
    pub len): WMI_SERVICE_PEER_STATS,,
    pub len): WMI_SERVICE_RESET_CHIP,,
    pub len): WMI_SERVICE_HTT_MGMT_TX_COMP_VALID_FLAGS,,
    pub len): WMI_SERVICE_BB_TIMING_CONFIG_SUPPORT,,
    pub len): WMI_SERVICE_PER_PACKET_SW_ENCRYPT,,
    pub len): WMI_SERVICE_BEACON_OFFLOAD,,
    pub len): WMI_SERVICE_SCAN_OFFLOAD,,
    pub len): WMI_SERVICE_ROAM_OFFLOAD,,
    pub len): WMI_SERVICE_BCN_MISS_OFFLOAD,,
    pub len): WMI_SERVICE_STA_PWRSAVE,,
    pub len): WMI_SERVICE_STA_ADVANCED_PWRSAVE,,
    pub len): WMI_SERVICE_AP_UAPSD,,
    pub len): WMI_SERVICE_AP_DFS,,
    pub len): WMI_SERVICE_11AC,,
    pub len): WMI_SERVICE_BLOCKACK,,
    pub len): WMI_SERVICE_PHYERR,,
    pub len): WMI_SERVICE_BCN_FILTER,,
    pub len): WMI_SERVICE_RTT,,
    pub len): WMI_SERVICE_RATECTRL,,
    pub len): WMI_SERVICE_WOW,,
    pub len): WMI_SERVICE_RATECTRL_CACHE,,
    pub len): WMI_SERVICE_IRAM_TIDS,,
    pub len): WMI_SERVICE_ARPNS_OFFLOAD,,
    pub len): WMI_SERVICE_NLO,,
    pub len): WMI_SERVICE_GTK_OFFLOAD,,
    pub len): WMI_SERVICE_SCAN_SCH,,
    pub len): WMI_SERVICE_CSA_OFFLOAD,,
    pub len): WMI_SERVICE_CHATTER,,
    pub len): WMI_SERVICE_COEX_FREQAVOID,,
    pub len): WMI_SERVICE_PACKET_POWER_SAVE,,
    pub len): WMI_SERVICE_FORCE_FW_HANG,,
    pub len): WMI_SERVICE_GPIO,,
    pub len): WMI_SERVICE_STA_DTIM_PS_MODULATED_DTIM,,
    pub len): WMI_SERVICE_STA_UAPSD_BASIC_AUTO_TRIG,,
    pub len): WMI_SERVICE_STA_UAPSD_VAR_AUTO_TRIG,,
    pub len): WMI_SERVICE_STA_KEEP_ALIVE,,
    pub len): WMI_SERVICE_TX_ENCAP,,
    pub len): WMI_SERVICE_BEACON_OFFLOAD,,
    pub len): WMI_SERVICE_SCAN_OFFLOAD,,
    pub len): WMI_SERVICE_ROAM_OFFLOAD,,
    pub len): WMI_SERVICE_BCN_MISS_OFFLOAD,,
    pub len): WMI_SERVICE_STA_PWRSAVE,,
    pub len): WMI_SERVICE_STA_ADVANCED_PWRSAVE,,
    pub len): WMI_SERVICE_AP_UAPSD,,
    pub len): WMI_SERVICE_AP_DFS,,
    pub len): WMI_SERVICE_11AC,,
    pub len): WMI_SERVICE_BLOCKACK,,
    pub len): WMI_SERVICE_PHYERR,,
    pub len): WMI_SERVICE_BCN_FILTER,,
    pub len): WMI_SERVICE_RTT,,
    pub len): WMI_SERVICE_RATECTRL,,
    pub len): WMI_SERVICE_WOW,,
    pub len): WMI_SERVICE_RATECTRL_CACHE,,
    pub len): WMI_SERVICE_IRAM_TIDS,,
    pub len): WMI_SERVICE_BURST,,
    pub len): WMI_SERVICE_SMART_ANTENNA_SW_SUPPORT,,
    pub len): WMI_SERVICE_GTK_OFFLOAD,,
    pub len): WMI_SERVICE_SCAN_SCH,,
    pub len): WMI_SERVICE_CSA_OFFLOAD,,
    pub len): WMI_SERVICE_CHATTER,,
    pub len): WMI_SERVICE_COEX_FREQAVOID,,
    pub len): WMI_SERVICE_PACKET_POWER_SAVE,,
    pub len): WMI_SERVICE_FORCE_FW_HANG,,
    pub len): WMI_SERVICE_SMART_ANTENNA_HW_SUPPORT,,
    pub len): WMI_SERVICE_GPIO,,
    pub len): WMI_SERVICE_STA_UAPSD_BASIC_AUTO_TRIG,,
    pub len): WMI_SERVICE_STA_UAPSD_VAR_AUTO_TRIG,,
    pub len): WMI_SERVICE_STA_KEEP_ALIVE,,
    pub len): WMI_SERVICE_TX_ENCAP,,
    pub len): WMI_SERVICE_AP_PS_DETECT_OUT_OF_SYNC,,
    pub len): WMI_SERVICE_EARLY_RX,,
    pub len): WMI_SERVICE_ENHANCED_PROXY_STA,,
    pub len): WMI_SERVICE_TT,,
    pub len): WMI_SERVICE_ATF,,
    pub len): WMI_SERVICE_PEER_CACHING,,
    pub len): WMI_SERVICE_COEX_GPIO,,
    pub len): WMI_SERVICE_AUX_SPECTRAL_INTF,,
    pub len): WMI_SERVICE_AUX_CHAN_LOAD_INTF,,
    pub len): WMI_SERVICE_BSS_CHANNEL_INFO_64,,
    pub len): WMI_SERVICE_EXT_RES_CFG_SUPPORT,,
    pub len): WMI_SERVICE_MESH_NON_11S,,
    pub len): WMI_SERVICE_RESTRT_CHNL_SUPPORT,,
    pub len): WMI_SERVICE_PEER_STATS,,
    pub len): WMI_SERVICE_MESH_11S,,
    pub len): WMI_SERVICE_PERIODIC_CHAN_STAT_SUPPORT,,
    pub len): WMI_SERVICE_TX_MODE_PUSH_ONLY,,
    pub len): WMI_SERVICE_TX_MODE_PUSH_PULL,,
    pub len): WMI_SERVICE_TX_MODE_DYNAMIC,,
    pub len): WMI_SERVICE_VDEV_RX_FILTER,,
    pub len): WMI_SERVICE_BTCOEX,,
    pub len): WMI_SERVICE_CHECK_CAL_VERSION,,
    pub len): WMI_SERVICE_DBGLOG_WARN2,,
    pub len): WMI_SERVICE_BTCOEX_DUTY_CYCLE,,
    pub len): WMI_SERVICE_4_WIRE_COEX_SUPPORT,,
    pub len): WMI_SERVICE_EXTENDED_NSS_SUPPORT,,
    pub len): WMI_SERVICE_PROG_GPIO_BAND_SELECT,,
    pub len): WMI_SERVICE_SMART_LOGGING_SUPPORT,,
    pub len): WMI_SERVICE_TDLS,,
    pub len): WMI_SERVICE_TDLS_OFFCHAN,,
    pub len): WMI_SERVICE_TDLS_UAPSD_BUFFER_STA,,
    pub len): WMI_SERVICE_TDLS_UAPSD_SLEEP_STA,,
    pub len): WMI_SERVICE_TDLS_CONN_TRACKER_IN_HOST_MODE,,
    pub len): WMI_SERVICE_TDLS_EXPLICIT_MODE_ONLY,,
    pub len): WMI_SERVICE_TDLS_WIDER_BANDWIDTH,,
    pub len): WMI_SERVICE_HTT_MGMT_TX_COMP_VALID_FLAGS,,
    pub len): WMI_SERVICE_HOST_DFS_CHECK_SUPPORT,,
    pub len): WMI_SERVICE_TPC_STATS_FINAL,,
    pub len): WMI_SERVICE_TX_DATA_ACK_RSSI,,
    pub len): WMI_SERVICE_VDEV_DIFFERENT_BEACON_INTERVAL_SUPPORT,,
    pub len): WMI_SERVICE_VDEV_DISABLE_4_ADDR_SRC_LRN_SUPPORT,,
    pub len): WMI_SERVICE_RTT_RESPONDER_ROLE,,
    pub len): WMI_SERVICE_PER_PACKET_SW_ENCRYPT,,
    pub len): WMI_SERVICE_REPORT_AIRTIME,,
    pub len): WMI_SERVICE_TX_PWR_PER_PEER,,
    pub len): WMI_SERVICE_RESET_CHIP,,
    pub len): WMI_SERVICE_PEER_TID_CONFIGS_SUPPORT,,
    pub len): WMI_SERVICE_PEER_TID_CONFIGS_SUPPORT,,

// 2 word representation of MAC addr
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mac_addr {
    pub addr: [u8; 6],
    pub word0: u32,
    pub word1: u32,
    pub __packed: },
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cmd_map {
    pub init_cmdid: u32,
    pub start_scan_cmdid: u32,
    pub stop_scan_cmdid: u32,
    pub scan_chan_list_cmdid: u32,
    pub scan_sch_prio_tbl_cmdid: u32,
    pub scan_prob_req_oui_cmdid: u32,
    pub pdev_set_regdomain_cmdid: u32,
    pub pdev_set_channel_cmdid: u32,
    pub pdev_set_param_cmdid: u32,
    pub pdev_pktlog_enable_cmdid: u32,
    pub pdev_pktlog_disable_cmdid: u32,
    pub pdev_set_wmm_params_cmdid: u32,
    pub pdev_set_ht_cap_ie_cmdid: u32,
    pub pdev_set_vht_cap_ie_cmdid: u32,
    pub pdev_set_dscp_tid_map_cmdid: u32,
    pub pdev_set_quiet_mode_cmdid: u32,
    pub pdev_green_ap_ps_enable_cmdid: u32,
    pub pdev_get_tpc_config_cmdid: u32,
    pub pdev_set_base_macaddr_cmdid: u32,
    pub vdev_create_cmdid: u32,
    pub vdev_delete_cmdid: u32,
    pub vdev_start_request_cmdid: u32,
    pub vdev_restart_request_cmdid: u32,
    pub vdev_up_cmdid: u32,
    pub vdev_stop_cmdid: u32,
    pub vdev_down_cmdid: u32,
    pub vdev_set_param_cmdid: u32,
    pub vdev_install_key_cmdid: u32,
    pub peer_create_cmdid: u32,
    pub peer_delete_cmdid: u32,
    pub peer_flush_tids_cmdid: u32,
    pub peer_set_param_cmdid: u32,
    pub peer_assoc_cmdid: u32,
    pub peer_add_wds_entry_cmdid: u32,
    pub peer_remove_wds_entry_cmdid: u32,
    pub peer_mcast_group_cmdid: u32,
    pub bcn_tx_cmdid: u32,
    pub pdev_send_bcn_cmdid: u32,
    pub bcn_tmpl_cmdid: u32,
    pub bcn_filter_rx_cmdid: u32,
    pub prb_req_filter_rx_cmdid: u32,
    pub mgmt_tx_cmdid: u32,
    pub mgmt_tx_send_cmdid: u32,
    pub prb_tmpl_cmdid: u32,
    pub addba_clear_resp_cmdid: u32,
    pub addba_send_cmdid: u32,
    pub addba_status_cmdid: u32,
    pub delba_send_cmdid: u32,
    pub addba_set_resp_cmdid: u32,
    pub send_singleamsdu_cmdid: u32,
    pub sta_powersave_mode_cmdid: u32,
    pub sta_powersave_param_cmdid: u32,
    pub sta_mimo_ps_mode_cmdid: u32,
    pub pdev_dfs_enable_cmdid: u32,
    pub pdev_dfs_disable_cmdid: u32,
    pub roam_scan_mode: u32,
    pub roam_scan_rssi_threshold: u32,
    pub roam_scan_period: u32,
    pub roam_scan_rssi_change_threshold: u32,
    pub roam_ap_profile: u32,
    pub ofl_scan_add_ap_profile: u32,
    pub ofl_scan_remove_ap_profile: u32,
    pub ofl_scan_period: u32,
    pub p2p_dev_set_device_info: u32,
    pub p2p_dev_set_discoverability: u32,
    pub p2p_go_set_beacon_ie: u32,
    pub p2p_go_set_probe_resp_ie: u32,
    pub p2p_set_vendor_ie_data_cmdid: u32,
    pub ap_ps_peer_param_cmdid: u32,
    pub ap_ps_peer_uapsd_coex_cmdid: u32,
    pub peer_rate_retry_sched_cmdid: u32,
    pub wlan_profile_trigger_cmdid: u32,
    pub wlan_profile_set_hist_intvl_cmdid: u32,
    pub wlan_profile_get_profile_data_cmdid: u32,
    pub wlan_profile_enable_profile_id_cmdid: u32,
    pub wlan_profile_list_profile_id_cmdid: u32,
    pub pdev_suspend_cmdid: u32,
    pub pdev_resume_cmdid: u32,
    pub add_bcn_filter_cmdid: u32,
    pub rmv_bcn_filter_cmdid: u32,
    pub wow_add_wake_pattern_cmdid: u32,
    pub wow_del_wake_pattern_cmdid: u32,
    pub wow_enable_disable_wake_event_cmdid: u32,
    pub wow_enable_cmdid: u32,
    pub wow_hostwakeup_from_sleep_cmdid: u32,
    pub rtt_measreq_cmdid: u32,
    pub rtt_tsf_cmdid: u32,
    pub vdev_spectral_scan_configure_cmdid: u32,
    pub vdev_spectral_scan_enable_cmdid: u32,
    pub request_stats_cmdid: u32,
    pub request_peer_stats_info_cmdid: u32,
    pub set_arp_ns_offload_cmdid: u32,
    pub network_list_offload_config_cmdid: u32,
    pub gtk_offload_cmdid: u32,
    pub csa_offload_enable_cmdid: u32,
    pub csa_offload_chanswitch_cmdid: u32,
    pub chatter_set_mode_cmdid: u32,
    pub peer_tid_addba_cmdid: u32,
    pub peer_tid_delba_cmdid: u32,
    pub sta_dtim_ps_method_cmdid: u32,
    pub sta_uapsd_auto_trig_cmdid: u32,
    pub sta_keepalive_cmd: u32,
    pub echo_cmdid: u32,
    pub pdev_utf_cmdid: u32,
    pub dbglog_cfg_cmdid: u32,
    pub pdev_qvit_cmdid: u32,
    pub pdev_ftm_intg_cmdid: u32,
    pub vdev_set_keepalive_cmdid: u32,
    pub vdev_get_keepalive_cmdid: u32,
    pub force_fw_hang_cmdid: u32,
    pub gpio_config_cmdid: u32,
    pub gpio_output_cmdid: u32,
    pub pdev_get_temperature_cmdid: u32,
    pub vdev_set_wmm_params_cmdid: u32,
    pub tdls_set_state_cmdid: u32,
    pub tdls_peer_update_cmdid: u32,
    pub adaptive_qcs_cmdid: u32,
    pub scan_update_request_cmdid: u32,
    pub vdev_standby_response_cmdid: u32,
    pub vdev_resume_response_cmdid: u32,
    pub wlan_peer_caching_add_peer_cmdid: u32,
    pub wlan_peer_caching_evict_peer_cmdid: u32,
    pub wlan_peer_caching_restore_peer_cmdid: u32,
    pub wlan_peer_caching_print_all_peers_info_cmdid: u32,
    pub peer_update_wds_entry_cmdid: u32,
    pub peer_add_proxy_sta_entry_cmdid: u32,
    pub rtt_keepalive_cmdid: u32,
    pub oem_req_cmdid: u32,
    pub nan_cmdid: u32,
    pub vdev_ratemask_cmdid: u32,
    pub qboost_cfg_cmdid: u32,
    pub pdev_smart_ant_enable_cmdid: u32,
    pub pdev_smart_ant_set_rx_antenna_cmdid: u32,
    pub peer_smart_ant_set_tx_antenna_cmdid: u32,
    pub peer_smart_ant_set_train_info_cmdid: u32,
    pub peer_smart_ant_set_node_config_ops_cmdid: u32,
    pub pdev_set_antenna_switch_table_cmdid: u32,
    pub pdev_set_ctl_table_cmdid: u32,
    pub pdev_set_mimogain_table_cmdid: u32,
    pub pdev_ratepwr_table_cmdid: u32,
    pub pdev_ratepwr_chainmsk_table_cmdid: u32,
    pub pdev_fips_cmdid: u32,
    pub tt_set_conf_cmdid: u32,
    pub fwtest_cmdid: u32,
    pub vdev_atf_request_cmdid: u32,
    pub peer_atf_request_cmdid: u32,
    pub pdev_get_ani_cck_config_cmdid: u32,
    pub pdev_get_ani_ofdm_config_cmdid: u32,
    pub pdev_reserve_ast_entry_cmdid: u32,
    pub pdev_get_nfcal_power_cmdid: u32,
    pub pdev_get_tpc_cmdid: u32,
    pub pdev_get_ast_info_cmdid: u32,
    pub vdev_set_dscp_tid_map_cmdid: u32,
    pub pdev_get_info_cmdid: u32,
    pub vdev_get_info_cmdid: u32,
    pub vdev_filter_neighbor_rx_packets_cmdid: u32,
    pub mu_cal_start_cmdid: u32,
    pub set_cca_params_cmdid: u32,
    pub pdev_bss_chan_info_request_cmdid: u32,
    pub pdev_enable_adaptive_cca_cmdid: u32,
    pub ext_resource_cfg_cmdid: u32,
    pub vdev_set_ie_cmdid: u32,
    pub set_lteu_config_cmdid: u32,
    pub atf_ssid_grouping_request_cmdid: u32,
    pub peer_atf_ext_request_cmdid: u32,
    pub set_periodic_channel_stats_cfg_cmdid: u32,
    pub peer_bwf_request_cmdid: u32,
    pub btcoex_cfg_cmdid: u32,
    pub peer_tx_mu_txmit_count_cmdid: u32,
    pub peer_tx_mu_txmit_rstcnt_cmdid: u32,
    pub peer_gid_userpos_list_cmdid: u32,
    pub pdev_check_cal_version_cmdid: u32,
    pub coex_version_cfg_cmid: u32,
    pub pdev_get_rx_filter_cmdid: u32,
    pub pdev_extended_nss_cfg_cmdid: u32,
    pub vdev_set_scan_nac_rssi_cmdid: u32,
    pub prog_gpio_band_select_cmdid: u32,
    pub config_smart_logging_cmdid: u32,
    pub debug_fatal_condition_cmdid: u32,
    pub get_tsf_timer_cmdid: u32,
    pub pdev_get_tpc_table_cmdid: u32,
    pub vdev_sifs_trigger_time_cmdid: u32,
    pub pdev_wds_entry_list_cmdid: u32,
    pub tdls_set_offchan_mode_cmdid: u32,
    pub radar_found_cmdid: u32,
    pub set_bb_timing_cmdid: u32,
    pub per_peer_per_tid_config_cmdid: u32,
}

//
// wmi command groups.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cmd_group {
// 0 to 2 are reserved
    WMI_GRP_START = 0x3,
    WMI_GRP_SCAN = WMI_GRP_START,
    WMI_GRP_PDEV,
    WMI_GRP_VDEV,
    WMI_GRP_PEER,
    WMI_GRP_MGMT,
    WMI_GRP_BA_NEG,
    WMI_GRP_STA_PS,
    WMI_GRP_DFS,
    WMI_GRP_ROAM,
    WMI_GRP_OFL_SCAN,
    WMI_GRP_P2P,
    WMI_GRP_AP_PS,
    WMI_GRP_RATE_CTRL,
    WMI_GRP_PROFILE,
    WMI_GRP_SUSPEND,
    WMI_GRP_BCN_FILTER,
    WMI_GRP_WOW,
    WMI_GRP_RTT,
    WMI_GRP_SPECTRAL,
    WMI_GRP_STATS,
    WMI_GRP_ARP_NS_OFL,
    WMI_GRP_NLO_OFL,
    WMI_GRP_GTK_OFL,
    WMI_GRP_CSA_OFL,
    WMI_GRP_CHATTER,
    WMI_GRP_TID_ADDBA,
    WMI_GRP_MISC,
    WMI_GRP_GPIO,
}

pub const WMI_CMD_UNSUPPORTED: c_int = 0;
// Command IDs and command events for MAIN FW.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cmd_id {
    WMI_INIT_CMDID = 0x1,

// Scan specific commands
    WMI_START_SCAN_CMDID = WMI_CMD_GRP(WMI_GRP_SCAN),
    WMI_STOP_SCAN_CMDID,
    WMI_SCAN_CHAN_LIST_CMDID,
    WMI_SCAN_SCH_PRIO_TBL_CMDID,

// PDEV (physical device) specific commands
    WMI_PDEV_SET_REGDOMAIN_CMDID = WMI_CMD_GRP(WMI_GRP_PDEV),
    WMI_PDEV_SET_CHANNEL_CMDID,
    WMI_PDEV_SET_PARAM_CMDID,
    WMI_PDEV_PKTLOG_ENABLE_CMDID,
    WMI_PDEV_PKTLOG_DISABLE_CMDID,
    WMI_PDEV_SET_WMM_PARAMS_CMDID,
    WMI_PDEV_SET_HT_CAP_IE_CMDID,
    WMI_PDEV_SET_VHT_CAP_IE_CMDID,
    WMI_PDEV_SET_DSCP_TID_MAP_CMDID,
    WMI_PDEV_SET_QUIET_MODE_CMDID,
    WMI_PDEV_GREEN_AP_PS_ENABLE_CMDID,
    WMI_PDEV_GET_TPC_CONFIG_CMDID,
    WMI_PDEV_SET_BASE_MACADDR_CMDID,

// VDEV (virtual device) specific commands
    WMI_VDEV_CREATE_CMDID = WMI_CMD_GRP(WMI_GRP_VDEV),
    WMI_VDEV_DELETE_CMDID,
    WMI_VDEV_START_REQUEST_CMDID,
    WMI_VDEV_RESTART_REQUEST_CMDID,
    WMI_VDEV_UP_CMDID,
    WMI_VDEV_STOP_CMDID,
    WMI_VDEV_DOWN_CMDID,
    WMI_VDEV_SET_PARAM_CMDID,
    WMI_VDEV_INSTALL_KEY_CMDID,

// peer specific commands
    WMI_PEER_CREATE_CMDID = WMI_CMD_GRP(WMI_GRP_PEER),
    WMI_PEER_DELETE_CMDID,
    WMI_PEER_FLUSH_TIDS_CMDID,
    WMI_PEER_SET_PARAM_CMDID,
    WMI_PEER_ASSOC_CMDID,
    WMI_PEER_ADD_WDS_ENTRY_CMDID,
    WMI_PEER_REMOVE_WDS_ENTRY_CMDID,
    WMI_PEER_MCAST_GROUP_CMDID,

// beacon/management specific commands
    WMI_BCN_TX_CMDID = WMI_CMD_GRP(WMI_GRP_MGMT),
    WMI_PDEV_SEND_BCN_CMDID,
    WMI_BCN_TMPL_CMDID,
    WMI_BCN_FILTER_RX_CMDID,
    WMI_PRB_REQ_FILTER_RX_CMDID,
    WMI_MGMT_TX_CMDID,
    WMI_PRB_TMPL_CMDID,

// commands to directly control BA negotiation directly from host.
    WMI_ADDBA_CLEAR_RESP_CMDID = WMI_CMD_GRP(WMI_GRP_BA_NEG),
    WMI_ADDBA_SEND_CMDID,
    WMI_ADDBA_STATUS_CMDID,
    WMI_DELBA_SEND_CMDID,
    WMI_ADDBA_SET_RESP_CMDID,
    WMI_SEND_SINGLEAMSDU_CMDID,

// Station power save specific config
    WMI_STA_POWERSAVE_MODE_CMDID = WMI_CMD_GRP(WMI_GRP_STA_PS),
    WMI_STA_POWERSAVE_PARAM_CMDID,
    WMI_STA_MIMO_PS_MODE_CMDID,

// DFS-specific commands
    WMI_PDEV_DFS_ENABLE_CMDID = WMI_CMD_GRP(WMI_GRP_DFS),
    WMI_PDEV_DFS_DISABLE_CMDID,

// Roaming specific  commands
    WMI_ROAM_SCAN_MODE = WMI_CMD_GRP(WMI_GRP_ROAM),
    WMI_ROAM_SCAN_RSSI_THRESHOLD,
    WMI_ROAM_SCAN_PERIOD,
    WMI_ROAM_SCAN_RSSI_CHANGE_THRESHOLD,
    WMI_ROAM_AP_PROFILE,

// offload scan specific commands
    WMI_OFL_SCAN_ADD_AP_PROFILE = WMI_CMD_GRP(WMI_GRP_OFL_SCAN),
    WMI_OFL_SCAN_REMOVE_AP_PROFILE,
    WMI_OFL_SCAN_PERIOD,

// P2P specific commands
    WMI_P2P_DEV_SET_DEVICE_INFO = WMI_CMD_GRP(WMI_GRP_P2P),
    WMI_P2P_DEV_SET_DISCOVERABILITY,
    WMI_P2P_GO_SET_BEACON_IE,
    WMI_P2P_GO_SET_PROBE_RESP_IE,
    WMI_P2P_SET_VENDOR_IE_DATA_CMDID,

// AP power save specific config
    WMI_AP_PS_PEER_PARAM_CMDID = WMI_CMD_GRP(WMI_GRP_AP_PS),
    WMI_AP_PS_PEER_UAPSD_COEX_CMDID,

// Rate-control specific commands
    WMI_PEER_RATE_RETRY_SCHED_CMDID =
    WMI_CMD_GRP(WMI_GRP_RATE_CTRL),

// WLAN Profiling commands.
    WMI_WLAN_PROFILE_TRIGGER_CMDID = WMI_CMD_GRP(WMI_GRP_PROFILE),
    WMI_WLAN_PROFILE_SET_HIST_INTVL_CMDID,
    WMI_WLAN_PROFILE_GET_PROFILE_DATA_CMDID,
    WMI_WLAN_PROFILE_ENABLE_PROFILE_ID_CMDID,
    WMI_WLAN_PROFILE_LIST_PROFILE_ID_CMDID,

// Suspend resume command Ids
    WMI_PDEV_SUSPEND_CMDID = WMI_CMD_GRP(WMI_GRP_SUSPEND),
    WMI_PDEV_RESUME_CMDID,

// Beacon filter commands
    WMI_ADD_BCN_FILTER_CMDID = WMI_CMD_GRP(WMI_GRP_BCN_FILTER),
    WMI_RMV_BCN_FILTER_CMDID,

// WOW Specific WMI commands
    WMI_WOW_ADD_WAKE_PATTERN_CMDID = WMI_CMD_GRP(WMI_GRP_WOW),
    WMI_WOW_DEL_WAKE_PATTERN_CMDID,
    WMI_WOW_ENABLE_DISABLE_WAKE_EVENT_CMDID,
    WMI_WOW_ENABLE_CMDID,
    WMI_WOW_HOSTWAKEUP_FROM_SLEEP_CMDID,

// RTT measurement related cmd
    WMI_RTT_MEASREQ_CMDID = WMI_CMD_GRP(WMI_GRP_RTT),
    WMI_RTT_TSF_CMDID,

// spectral scan commands
    WMI_VDEV_SPECTRAL_SCAN_CONFIGURE_CMDID = WMI_CMD_GRP(WMI_GRP_SPECTRAL),
    WMI_VDEV_SPECTRAL_SCAN_ENABLE_CMDID,

// F/W stats
    WMI_REQUEST_STATS_CMDID = WMI_CMD_GRP(WMI_GRP_STATS),

// ARP OFFLOAD REQUEST
    WMI_SET_ARP_NS_OFFLOAD_CMDID = WMI_CMD_GRP(WMI_GRP_ARP_NS_OFL),

// NS offload confid
    WMI_NETWORK_LIST_OFFLOAD_CONFIG_CMDID = WMI_CMD_GRP(WMI_GRP_NLO_OFL),

// GTK offload Specific WMI commands
    WMI_GTK_OFFLOAD_CMDID = WMI_CMD_GRP(WMI_GRP_GTK_OFL),

// CSA offload Specific WMI commands
    WMI_CSA_OFFLOAD_ENABLE_CMDID = WMI_CMD_GRP(WMI_GRP_CSA_OFL),
    WMI_CSA_OFFLOAD_CHANSWITCH_CMDID,

// Chatter commands
    WMI_CHATTER_SET_MODE_CMDID = WMI_CMD_GRP(WMI_GRP_CHATTER),

// addba specific commands
    WMI_PEER_TID_ADDBA_CMDID = WMI_CMD_GRP(WMI_GRP_TID_ADDBA),
    WMI_PEER_TID_DELBA_CMDID,

// set station mimo powersave method
    WMI_STA_DTIM_PS_METHOD_CMDID,
// Configure the Station UAPSD AC Auto Trigger Parameters
    WMI_STA_UAPSD_AUTO_TRIG_CMDID,

// STA Keep alive parameter configuration,
// Requires WMI_SERVICE_STA_KEEP_ALIVE
//
    WMI_STA_KEEPALIVE_CMD,

// misc command group
    WMI_ECHO_CMDID = WMI_CMD_GRP(WMI_GRP_MISC),
    WMI_PDEV_UTF_CMDID,
    WMI_DBGLOG_CFG_CMDID,
    WMI_PDEV_QVIT_CMDID,
    WMI_PDEV_FTM_INTG_CMDID,
    WMI_VDEV_SET_KEEPALIVE_CMDID,
    WMI_VDEV_GET_KEEPALIVE_CMDID,
    WMI_FORCE_FW_HANG_CMDID,

// GPIO Configuration
    WMI_GPIO_CONFIG_CMDID = WMI_CMD_GRP(WMI_GRP_GPIO),
    WMI_GPIO_OUTPUT_CMDID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_event_id {
    WMI_SERVICE_READY_EVENTID = 0x1,
    WMI_READY_EVENTID,
    WMI_SERVICE_AVAILABLE_EVENTID,

// Scan specific events
    WMI_SCAN_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_SCAN),

// PDEV specific events
    WMI_PDEV_TPC_CONFIG_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_PDEV),
    WMI_CHAN_INFO_EVENTID,
    WMI_PHYERR_EVENTID,

// VDEV specific events
    WMI_VDEV_START_RESP_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_VDEV),
    WMI_VDEV_STOPPED_EVENTID,
    WMI_VDEV_INSTALL_KEY_COMPLETE_EVENTID,

// peer specific events
    WMI_PEER_STA_KICKOUT_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_PEER),

// beacon/mgmt specific events
    WMI_MGMT_RX_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_MGMT),
    WMI_HOST_SWBA_EVENTID,
    WMI_TBTTOFFSET_UPDATE_EVENTID,

// ADDBA Related WMI Events
    WMI_TX_DELBA_COMPLETE_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_BA_NEG),
    WMI_TX_ADDBA_COMPLETE_EVENTID,

// Roam event to trigger roaming on host
    WMI_ROAM_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_ROAM),
    WMI_PROFILE_MATCH,

// WoW
    WMI_WOW_WAKEUP_HOST_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_WOW),

// RTT
    WMI_RTT_MEASUREMENT_REPORT_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_RTT),
    WMI_TSF_MEASUREMENT_REPORT_EVENTID,
    WMI_RTT_ERROR_REPORT_EVENTID,

// GTK offload
    WMI_GTK_OFFLOAD_STATUS_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_GTK_OFL),
    WMI_GTK_REKEY_FAIL_EVENTID,

// CSA IE received event
    WMI_CSA_HANDLING_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_CSA_OFL),

// Misc events
    WMI_ECHO_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_MISC),
    WMI_PDEV_UTF_EVENTID,
    WMI_DEBUG_MESG_EVENTID,
    WMI_UPDATE_STATS_EVENTID,
    WMI_DEBUG_PRINT_EVENTID,
    WMI_DCS_INTERFERENCE_EVENTID,
    WMI_PDEV_QVIT_EVENTID,
    WMI_WLAN_PROFILE_DATA_EVENTID,
    WMI_PDEV_FTM_INTG_EVENTID,
    WMI_WLAN_FREQ_AVOID_EVENTID,
    WMI_VDEV_GET_KEEPALIVE_EVENTID,

// GPIO Event
    WMI_GPIO_INPUT_EVENTID = WMI_EVT_GRP_START_ID(WMI_GRP_GPIO),
}

// Command IDs and command events for 10.X firmware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10x_cmd_id {
    WMI_10X_START_CMDID = 0x9000,
    WMI_10X_END_CMDID = 0x9FFF,

// initialize the wlan sub system
    WMI_10X_INIT_CMDID,

// Scan specific commands

    WMI_10X_START_SCAN_CMDID = WMI_10X_START_CMDID,
    WMI_10X_STOP_SCAN_CMDID,
    WMI_10X_SCAN_CHAN_LIST_CMDID,
    WMI_10X_ECHO_CMDID,

// PDEV(physical device) specific commands
    WMI_10X_PDEV_SET_REGDOMAIN_CMDID,
    WMI_10X_PDEV_SET_CHANNEL_CMDID,
    WMI_10X_PDEV_SET_PARAM_CMDID,
    WMI_10X_PDEV_PKTLOG_ENABLE_CMDID,
    WMI_10X_PDEV_PKTLOG_DISABLE_CMDID,
    WMI_10X_PDEV_SET_WMM_PARAMS_CMDID,
    WMI_10X_PDEV_SET_HT_CAP_IE_CMDID,
    WMI_10X_PDEV_SET_VHT_CAP_IE_CMDID,
    WMI_10X_PDEV_SET_BASE_MACADDR_CMDID,
    WMI_10X_PDEV_SET_DSCP_TID_MAP_CMDID,
    WMI_10X_PDEV_SET_QUIET_MODE_CMDID,
    WMI_10X_PDEV_GREEN_AP_PS_ENABLE_CMDID,
    WMI_10X_PDEV_GET_TPC_CONFIG_CMDID,

// VDEV(virtual device) specific commands
    WMI_10X_VDEV_CREATE_CMDID,
    WMI_10X_VDEV_DELETE_CMDID,
    WMI_10X_VDEV_START_REQUEST_CMDID,
    WMI_10X_VDEV_RESTART_REQUEST_CMDID,
    WMI_10X_VDEV_UP_CMDID,
    WMI_10X_VDEV_STOP_CMDID,
    WMI_10X_VDEV_DOWN_CMDID,
    WMI_10X_VDEV_STANDBY_RESPONSE_CMDID,
    WMI_10X_VDEV_RESUME_RESPONSE_CMDID,
    WMI_10X_VDEV_SET_PARAM_CMDID,
    WMI_10X_VDEV_INSTALL_KEY_CMDID,

// peer specific commands
    WMI_10X_PEER_CREATE_CMDID,
    WMI_10X_PEER_DELETE_CMDID,
    WMI_10X_PEER_FLUSH_TIDS_CMDID,
    WMI_10X_PEER_SET_PARAM_CMDID,
    WMI_10X_PEER_ASSOC_CMDID,
    WMI_10X_PEER_ADD_WDS_ENTRY_CMDID,
    WMI_10X_PEER_REMOVE_WDS_ENTRY_CMDID,
    WMI_10X_PEER_MCAST_GROUP_CMDID,

// beacon/management specific commands

    WMI_10X_BCN_TX_CMDID,
    WMI_10X_BCN_PRB_TMPL_CMDID,
    WMI_10X_BCN_FILTER_RX_CMDID,
    WMI_10X_PRB_REQ_FILTER_RX_CMDID,
    WMI_10X_MGMT_TX_CMDID,

// commands to directly control ba negotiation directly from host.
    WMI_10X_ADDBA_CLEAR_RESP_CMDID,
    WMI_10X_ADDBA_SEND_CMDID,
    WMI_10X_ADDBA_STATUS_CMDID,
    WMI_10X_DELBA_SEND_CMDID,
    WMI_10X_ADDBA_SET_RESP_CMDID,
    WMI_10X_SEND_SINGLEAMSDU_CMDID,

// Station power save specific config
    WMI_10X_STA_POWERSAVE_MODE_CMDID,
    WMI_10X_STA_POWERSAVE_PARAM_CMDID,
    WMI_10X_STA_MIMO_PS_MODE_CMDID,

// set debug log config
    WMI_10X_DBGLOG_CFG_CMDID,

// DFS-specific commands
    WMI_10X_PDEV_DFS_ENABLE_CMDID,
    WMI_10X_PDEV_DFS_DISABLE_CMDID,

// QVIT specific command id
    WMI_10X_PDEV_QVIT_CMDID,

// Offload Scan and Roaming related  commands
    WMI_10X_ROAM_SCAN_MODE,
    WMI_10X_ROAM_SCAN_RSSI_THRESHOLD,
    WMI_10X_ROAM_SCAN_PERIOD,
    WMI_10X_ROAM_SCAN_RSSI_CHANGE_THRESHOLD,
    WMI_10X_ROAM_AP_PROFILE,
    WMI_10X_OFL_SCAN_ADD_AP_PROFILE,
    WMI_10X_OFL_SCAN_REMOVE_AP_PROFILE,
    WMI_10X_OFL_SCAN_PERIOD,

// P2P specific commands
    WMI_10X_P2P_DEV_SET_DEVICE_INFO,
    WMI_10X_P2P_DEV_SET_DISCOVERABILITY,
    WMI_10X_P2P_GO_SET_BEACON_IE,
    WMI_10X_P2P_GO_SET_PROBE_RESP_IE,

// AP power save specific config
    WMI_10X_AP_PS_PEER_PARAM_CMDID,
    WMI_10X_AP_PS_PEER_UAPSD_COEX_CMDID,

// Rate-control specific commands
    WMI_10X_PEER_RATE_RETRY_SCHED_CMDID,

// WLAN Profiling commands.
    WMI_10X_WLAN_PROFILE_TRIGGER_CMDID,
    WMI_10X_WLAN_PROFILE_SET_HIST_INTVL_CMDID,
    WMI_10X_WLAN_PROFILE_GET_PROFILE_DATA_CMDID,
    WMI_10X_WLAN_PROFILE_ENABLE_PROFILE_ID_CMDID,
    WMI_10X_WLAN_PROFILE_LIST_PROFILE_ID_CMDID,

// Suspend resume command Ids
    WMI_10X_PDEV_SUSPEND_CMDID,
    WMI_10X_PDEV_RESUME_CMDID,

// Beacon filter commands
    WMI_10X_ADD_BCN_FILTER_CMDID,
    WMI_10X_RMV_BCN_FILTER_CMDID,

// WOW Specific WMI commands
    WMI_10X_WOW_ADD_WAKE_PATTERN_CMDID,
    WMI_10X_WOW_DEL_WAKE_PATTERN_CMDID,
    WMI_10X_WOW_ENABLE_DISABLE_WAKE_EVENT_CMDID,
    WMI_10X_WOW_ENABLE_CMDID,
    WMI_10X_WOW_HOSTWAKEUP_FROM_SLEEP_CMDID,

// RTT measurement related cmd
    WMI_10X_RTT_MEASREQ_CMDID,
    WMI_10X_RTT_TSF_CMDID,

// transmit beacon by value
    WMI_10X_PDEV_SEND_BCN_CMDID,

// F/W stats
    WMI_10X_VDEV_SPECTRAL_SCAN_CONFIGURE_CMDID,
    WMI_10X_VDEV_SPECTRAL_SCAN_ENABLE_CMDID,
    WMI_10X_REQUEST_STATS_CMDID,

// GPIO Configuration
    WMI_10X_GPIO_CONFIG_CMDID,
    WMI_10X_GPIO_OUTPUT_CMDID,

    WMI_10X_PDEV_UTF_CMDID = WMI_10X_END_CMDID - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10x_event_id {
    WMI_10X_SERVICE_READY_EVENTID = 0x8000,
    WMI_10X_READY_EVENTID,
    WMI_10X_START_EVENTID = 0x9000,
    WMI_10X_END_EVENTID = 0x9FFF,

// Scan specific events
    WMI_10X_SCAN_EVENTID = WMI_10X_START_EVENTID,
    WMI_10X_ECHO_EVENTID,
    WMI_10X_DEBUG_MESG_EVENTID,
    WMI_10X_UPDATE_STATS_EVENTID,

// Instantaneous RSSI event
    WMI_10X_INST_RSSI_STATS_EVENTID,

// VDEV specific events
    WMI_10X_VDEV_START_RESP_EVENTID,
    WMI_10X_VDEV_STANDBY_REQ_EVENTID,
    WMI_10X_VDEV_RESUME_REQ_EVENTID,
    WMI_10X_VDEV_STOPPED_EVENTID,

// peer  specific events
    WMI_10X_PEER_STA_KICKOUT_EVENTID,

// beacon/mgmt specific events
    WMI_10X_HOST_SWBA_EVENTID,
    WMI_10X_TBTTOFFSET_UPDATE_EVENTID,
    WMI_10X_MGMT_RX_EVENTID,

// Channel stats event
    WMI_10X_CHAN_INFO_EVENTID,

// PHY Error specific WMI event
    WMI_10X_PHYERR_EVENTID,

// Roam event to trigger roaming on host
    WMI_10X_ROAM_EVENTID,

// matching AP found from list of profiles
    WMI_10X_PROFILE_MATCH,

// debug print message used for tracing FW code while debugging
    WMI_10X_DEBUG_PRINT_EVENTID,
// VI spoecific event
    WMI_10X_PDEV_QVIT_EVENTID,
// FW code profile data in response to profile request
    WMI_10X_WLAN_PROFILE_DATA_EVENTID,

// RTT related event ID
    WMI_10X_RTT_MEASUREMENT_REPORT_EVENTID,
    WMI_10X_TSF_MEASUREMENT_REPORT_EVENTID,
    WMI_10X_RTT_ERROR_REPORT_EVENTID,

    WMI_10X_WOW_WAKEUP_HOST_EVENTID,
    WMI_10X_DCS_INTERFERENCE_EVENTID,

// TPC config for the current operating channel
    WMI_10X_PDEV_TPC_CONFIG_EVENTID,

    WMI_10X_GPIO_INPUT_EVENTID,
    WMI_10X_PDEV_UTF_EVENTID = WMI_10X_END_EVENTID - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_2_cmd_id {
    WMI_10_2_START_CMDID = 0x9000,
    WMI_10_2_END_CMDID = 0x9FFF,
    WMI_10_2_INIT_CMDID,
    WMI_10_2_START_SCAN_CMDID = WMI_10_2_START_CMDID,
    WMI_10_2_STOP_SCAN_CMDID,
    WMI_10_2_SCAN_CHAN_LIST_CMDID,
    WMI_10_2_ECHO_CMDID,
    WMI_10_2_PDEV_SET_REGDOMAIN_CMDID,
    WMI_10_2_PDEV_SET_CHANNEL_CMDID,
    WMI_10_2_PDEV_SET_PARAM_CMDID,
    WMI_10_2_PDEV_PKTLOG_ENABLE_CMDID,
    WMI_10_2_PDEV_PKTLOG_DISABLE_CMDID,
    WMI_10_2_PDEV_SET_WMM_PARAMS_CMDID,
    WMI_10_2_PDEV_SET_HT_CAP_IE_CMDID,
    WMI_10_2_PDEV_SET_VHT_CAP_IE_CMDID,
    WMI_10_2_PDEV_SET_BASE_MACADDR_CMDID,
    WMI_10_2_PDEV_SET_QUIET_MODE_CMDID,
    WMI_10_2_PDEV_GREEN_AP_PS_ENABLE_CMDID,
    WMI_10_2_PDEV_GET_TPC_CONFIG_CMDID,
    WMI_10_2_VDEV_CREATE_CMDID,
    WMI_10_2_VDEV_DELETE_CMDID,
    WMI_10_2_VDEV_START_REQUEST_CMDID,
    WMI_10_2_VDEV_RESTART_REQUEST_CMDID,
    WMI_10_2_VDEV_UP_CMDID,
    WMI_10_2_VDEV_STOP_CMDID,
    WMI_10_2_VDEV_DOWN_CMDID,
    WMI_10_2_VDEV_STANDBY_RESPONSE_CMDID,
    WMI_10_2_VDEV_RESUME_RESPONSE_CMDID,
    WMI_10_2_VDEV_SET_PARAM_CMDID,
    WMI_10_2_VDEV_INSTALL_KEY_CMDID,
    WMI_10_2_VDEV_SET_DSCP_TID_MAP_CMDID,
    WMI_10_2_PEER_CREATE_CMDID,
    WMI_10_2_PEER_DELETE_CMDID,
    WMI_10_2_PEER_FLUSH_TIDS_CMDID,
    WMI_10_2_PEER_SET_PARAM_CMDID,
    WMI_10_2_PEER_ASSOC_CMDID,
    WMI_10_2_PEER_ADD_WDS_ENTRY_CMDID,
    WMI_10_2_PEER_UPDATE_WDS_ENTRY_CMDID,
    WMI_10_2_PEER_REMOVE_WDS_ENTRY_CMDID,
    WMI_10_2_PEER_MCAST_GROUP_CMDID,
    WMI_10_2_BCN_TX_CMDID,
    WMI_10_2_BCN_PRB_TMPL_CMDID,
    WMI_10_2_BCN_FILTER_RX_CMDID,
    WMI_10_2_PRB_REQ_FILTER_RX_CMDID,
    WMI_10_2_MGMT_TX_CMDID,
    WMI_10_2_ADDBA_CLEAR_RESP_CMDID,
    WMI_10_2_ADDBA_SEND_CMDID,
    WMI_10_2_ADDBA_STATUS_CMDID,
    WMI_10_2_DELBA_SEND_CMDID,
    WMI_10_2_ADDBA_SET_RESP_CMDID,
    WMI_10_2_SEND_SINGLEAMSDU_CMDID,
    WMI_10_2_STA_POWERSAVE_MODE_CMDID,
    WMI_10_2_STA_POWERSAVE_PARAM_CMDID,
    WMI_10_2_STA_MIMO_PS_MODE_CMDID,
    WMI_10_2_DBGLOG_CFG_CMDID,
    WMI_10_2_PDEV_DFS_ENABLE_CMDID,
    WMI_10_2_PDEV_DFS_DISABLE_CMDID,
    WMI_10_2_PDEV_QVIT_CMDID,
    WMI_10_2_ROAM_SCAN_MODE,
    WMI_10_2_ROAM_SCAN_RSSI_THRESHOLD,
    WMI_10_2_ROAM_SCAN_PERIOD,
    WMI_10_2_ROAM_SCAN_RSSI_CHANGE_THRESHOLD,
    WMI_10_2_ROAM_AP_PROFILE,
    WMI_10_2_OFL_SCAN_ADD_AP_PROFILE,
    WMI_10_2_OFL_SCAN_REMOVE_AP_PROFILE,
    WMI_10_2_OFL_SCAN_PERIOD,
    WMI_10_2_P2P_DEV_SET_DEVICE_INFO,
    WMI_10_2_P2P_DEV_SET_DISCOVERABILITY,
    WMI_10_2_P2P_GO_SET_BEACON_IE,
    WMI_10_2_P2P_GO_SET_PROBE_RESP_IE,
    WMI_10_2_AP_PS_PEER_PARAM_CMDID,
    WMI_10_2_AP_PS_PEER_UAPSD_COEX_CMDID,
    WMI_10_2_PEER_RATE_RETRY_SCHED_CMDID,
    WMI_10_2_WLAN_PROFILE_TRIGGER_CMDID,
    WMI_10_2_WLAN_PROFILE_SET_HIST_INTVL_CMDID,
    WMI_10_2_WLAN_PROFILE_GET_PROFILE_DATA_CMDID,
    WMI_10_2_WLAN_PROFILE_ENABLE_PROFILE_ID_CMDID,
    WMI_10_2_WLAN_PROFILE_LIST_PROFILE_ID_CMDID,
    WMI_10_2_PDEV_SUSPEND_CMDID,
    WMI_10_2_PDEV_RESUME_CMDID,
    WMI_10_2_ADD_BCN_FILTER_CMDID,
    WMI_10_2_RMV_BCN_FILTER_CMDID,
    WMI_10_2_WOW_ADD_WAKE_PATTERN_CMDID,
    WMI_10_2_WOW_DEL_WAKE_PATTERN_CMDID,
    WMI_10_2_WOW_ENABLE_DISABLE_WAKE_EVENT_CMDID,
    WMI_10_2_WOW_ENABLE_CMDID,
    WMI_10_2_WOW_HOSTWAKEUP_FROM_SLEEP_CMDID,
    WMI_10_2_RTT_MEASREQ_CMDID,
    WMI_10_2_RTT_TSF_CMDID,
    WMI_10_2_RTT_KEEPALIVE_CMDID,
    WMI_10_2_PDEV_SEND_BCN_CMDID,
    WMI_10_2_VDEV_SPECTRAL_SCAN_CONFIGURE_CMDID,
    WMI_10_2_VDEV_SPECTRAL_SCAN_ENABLE_CMDID,
    WMI_10_2_REQUEST_STATS_CMDID,
    WMI_10_2_GPIO_CONFIG_CMDID,
    WMI_10_2_GPIO_OUTPUT_CMDID,
    WMI_10_2_VDEV_RATEMASK_CMDID,
    WMI_10_2_PDEV_SMART_ANT_ENABLE_CMDID,
    WMI_10_2_PDEV_SMART_ANT_SET_RX_ANTENNA_CMDID,
    WMI_10_2_PEER_SMART_ANT_SET_TX_ANTENNA_CMDID,
    WMI_10_2_PEER_SMART_ANT_SET_TRAIN_INFO_CMDID,
    WMI_10_2_PEER_SMART_ANT_SET_NODE_CONFIG_OPS_CMDID,
    WMI_10_2_FORCE_FW_HANG_CMDID,
    WMI_10_2_PDEV_SET_ANTENNA_SWITCH_TABLE_CMDID,
    WMI_10_2_PDEV_SET_CTL_TABLE_CMDID,
    WMI_10_2_PDEV_SET_MIMOGAIN_TABLE_CMDID,
    WMI_10_2_PDEV_RATEPWR_TABLE_CMDID,
    WMI_10_2_PDEV_RATEPWR_CHAINMSK_TABLE_CMDID,
    WMI_10_2_PDEV_GET_INFO,
    WMI_10_2_VDEV_GET_INFO,
    WMI_10_2_VDEV_ATF_REQUEST_CMDID,
    WMI_10_2_PEER_ATF_REQUEST_CMDID,
    WMI_10_2_PDEV_GET_TEMPERATURE_CMDID,
    WMI_10_2_MU_CAL_START_CMDID,
    WMI_10_2_SET_LTEU_CONFIG_CMDID,
    WMI_10_2_SET_CCA_PARAMS,
    WMI_10_2_PDEV_BSS_CHAN_INFO_REQUEST_CMDID,
    WMI_10_2_FWTEST_CMDID,
    WMI_10_2_PDEV_SET_BB_TIMING_CONFIG_CMDID,
    WMI_10_2_PDEV_UTF_CMDID = WMI_10_2_END_CMDID - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_2_event_id {
    WMI_10_2_SERVICE_READY_EVENTID = 0x8000,
    WMI_10_2_READY_EVENTID,
    WMI_10_2_DEBUG_MESG_EVENTID,
    WMI_10_2_START_EVENTID = 0x9000,
    WMI_10_2_END_EVENTID = 0x9FFF,
    WMI_10_2_SCAN_EVENTID = WMI_10_2_START_EVENTID,
    WMI_10_2_ECHO_EVENTID,
    WMI_10_2_UPDATE_STATS_EVENTID,
    WMI_10_2_INST_RSSI_STATS_EVENTID,
    WMI_10_2_VDEV_START_RESP_EVENTID,
    WMI_10_2_VDEV_STANDBY_REQ_EVENTID,
    WMI_10_2_VDEV_RESUME_REQ_EVENTID,
    WMI_10_2_VDEV_STOPPED_EVENTID,
    WMI_10_2_PEER_STA_KICKOUT_EVENTID,
    WMI_10_2_HOST_SWBA_EVENTID,
    WMI_10_2_TBTTOFFSET_UPDATE_EVENTID,
    WMI_10_2_MGMT_RX_EVENTID,
    WMI_10_2_CHAN_INFO_EVENTID,
    WMI_10_2_PHYERR_EVENTID,
    WMI_10_2_ROAM_EVENTID,
    WMI_10_2_PROFILE_MATCH,
    WMI_10_2_DEBUG_PRINT_EVENTID,
    WMI_10_2_PDEV_QVIT_EVENTID,
    WMI_10_2_WLAN_PROFILE_DATA_EVENTID,
    WMI_10_2_RTT_MEASUREMENT_REPORT_EVENTID,
    WMI_10_2_TSF_MEASUREMENT_REPORT_EVENTID,
    WMI_10_2_RTT_ERROR_REPORT_EVENTID,
    WMI_10_2_RTT_KEEPALIVE_EVENTID,
    WMI_10_2_WOW_WAKEUP_HOST_EVENTID,
    WMI_10_2_DCS_INTERFERENCE_EVENTID,
    WMI_10_2_PDEV_TPC_CONFIG_EVENTID,
    WMI_10_2_GPIO_INPUT_EVENTID,
    WMI_10_2_PEER_RATECODE_LIST_EVENTID,
    WMI_10_2_GENERIC_BUFFER_EVENTID,
    WMI_10_2_MCAST_BUF_RELEASE_EVENTID,
    WMI_10_2_MCAST_LIST_AGEOUT_EVENTID,
    WMI_10_2_WDS_PEER_EVENTID,
    WMI_10_2_PEER_STA_PS_STATECHG_EVENTID,
    WMI_10_2_PDEV_TEMPERATURE_EVENTID,
    WMI_10_2_MU_REPORT_EVENTID,
    WMI_10_2_PDEV_BSS_CHAN_INFO_EVENTID,
    WMI_10_2_PDEV_UTF_EVENTID = WMI_10_2_END_EVENTID - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_4_cmd_id {
    WMI_10_4_START_CMDID = 0x9000,
    WMI_10_4_END_CMDID = 0x9FFF,
    WMI_10_4_INIT_CMDID,
    WMI_10_4_START_SCAN_CMDID = WMI_10_4_START_CMDID,
    WMI_10_4_STOP_SCAN_CMDID,
    WMI_10_4_SCAN_CHAN_LIST_CMDID,
    WMI_10_4_SCAN_SCH_PRIO_TBL_CMDID,
    WMI_10_4_SCAN_UPDATE_REQUEST_CMDID,
    WMI_10_4_ECHO_CMDID,
    WMI_10_4_PDEV_SET_REGDOMAIN_CMDID,
    WMI_10_4_PDEV_SET_CHANNEL_CMDID,
    WMI_10_4_PDEV_SET_PARAM_CMDID,
    WMI_10_4_PDEV_PKTLOG_ENABLE_CMDID,
    WMI_10_4_PDEV_PKTLOG_DISABLE_CMDID,
    WMI_10_4_PDEV_SET_WMM_PARAMS_CMDID,
    WMI_10_4_PDEV_SET_HT_CAP_IE_CMDID,
    WMI_10_4_PDEV_SET_VHT_CAP_IE_CMDID,
    WMI_10_4_PDEV_SET_BASE_MACADDR_CMDID,
    WMI_10_4_PDEV_SET_DSCP_TID_MAP_CMDID,
    WMI_10_4_PDEV_SET_QUIET_MODE_CMDID,
    WMI_10_4_PDEV_GREEN_AP_PS_ENABLE_CMDID,
    WMI_10_4_PDEV_GET_TPC_CONFIG_CMDID,
    WMI_10_4_VDEV_CREATE_CMDID,
    WMI_10_4_VDEV_DELETE_CMDID,
    WMI_10_4_VDEV_START_REQUEST_CMDID,
    WMI_10_4_VDEV_RESTART_REQUEST_CMDID,
    WMI_10_4_VDEV_UP_CMDID,
    WMI_10_4_VDEV_STOP_CMDID,
    WMI_10_4_VDEV_DOWN_CMDID,
    WMI_10_4_VDEV_STANDBY_RESPONSE_CMDID,
    WMI_10_4_VDEV_RESUME_RESPONSE_CMDID,
    WMI_10_4_VDEV_SET_PARAM_CMDID,
    WMI_10_4_VDEV_INSTALL_KEY_CMDID,
    WMI_10_4_WLAN_PEER_CACHING_ADD_PEER_CMDID,
    WMI_10_4_WLAN_PEER_CACHING_EVICT_PEER_CMDID,
    WMI_10_4_WLAN_PEER_CACHING_RESTORE_PEER_CMDID,
    WMI_10_4_WLAN_PEER_CACHING_PRINT_ALL_PEERS_INFO_CMDID,
    WMI_10_4_PEER_CREATE_CMDID,
    WMI_10_4_PEER_DELETE_CMDID,
    WMI_10_4_PEER_FLUSH_TIDS_CMDID,
    WMI_10_4_PEER_SET_PARAM_CMDID,
    WMI_10_4_PEER_ASSOC_CMDID,
    WMI_10_4_PEER_ADD_WDS_ENTRY_CMDID,
    WMI_10_4_PEER_UPDATE_WDS_ENTRY_CMDID,
    WMI_10_4_PEER_REMOVE_WDS_ENTRY_CMDID,
    WMI_10_4_PEER_ADD_PROXY_STA_ENTRY_CMDID,
    WMI_10_4_PEER_MCAST_GROUP_CMDID,
    WMI_10_4_BCN_TX_CMDID,
    WMI_10_4_PDEV_SEND_BCN_CMDID,
    WMI_10_4_BCN_PRB_TMPL_CMDID,
    WMI_10_4_BCN_FILTER_RX_CMDID,
    WMI_10_4_PRB_REQ_FILTER_RX_CMDID,
    WMI_10_4_MGMT_TX_CMDID,
    WMI_10_4_PRB_TMPL_CMDID,
    WMI_10_4_ADDBA_CLEAR_RESP_CMDID,
    WMI_10_4_ADDBA_SEND_CMDID,
    WMI_10_4_ADDBA_STATUS_CMDID,
    WMI_10_4_DELBA_SEND_CMDID,
    WMI_10_4_ADDBA_SET_RESP_CMDID,
    WMI_10_4_SEND_SINGLEAMSDU_CMDID,
    WMI_10_4_STA_POWERSAVE_MODE_CMDID,
    WMI_10_4_STA_POWERSAVE_PARAM_CMDID,
    WMI_10_4_STA_MIMO_PS_MODE_CMDID,
    WMI_10_4_DBGLOG_CFG_CMDID,
    WMI_10_4_PDEV_DFS_ENABLE_CMDID,
    WMI_10_4_PDEV_DFS_DISABLE_CMDID,
    WMI_10_4_PDEV_QVIT_CMDID,
    WMI_10_4_ROAM_SCAN_MODE,
    WMI_10_4_ROAM_SCAN_RSSI_THRESHOLD,
    WMI_10_4_ROAM_SCAN_PERIOD,
    WMI_10_4_ROAM_SCAN_RSSI_CHANGE_THRESHOLD,
    WMI_10_4_ROAM_AP_PROFILE,
    WMI_10_4_OFL_SCAN_ADD_AP_PROFILE,
    WMI_10_4_OFL_SCAN_REMOVE_AP_PROFILE,
    WMI_10_4_OFL_SCAN_PERIOD,
    WMI_10_4_P2P_DEV_SET_DEVICE_INFO,
    WMI_10_4_P2P_DEV_SET_DISCOVERABILITY,
    WMI_10_4_P2P_GO_SET_BEACON_IE,
    WMI_10_4_P2P_GO_SET_PROBE_RESP_IE,
    WMI_10_4_P2P_SET_VENDOR_IE_DATA_CMDID,
    WMI_10_4_AP_PS_PEER_PARAM_CMDID,
    WMI_10_4_AP_PS_PEER_UAPSD_COEX_CMDID,
    WMI_10_4_PEER_RATE_RETRY_SCHED_CMDID,
    WMI_10_4_WLAN_PROFILE_TRIGGER_CMDID,
    WMI_10_4_WLAN_PROFILE_SET_HIST_INTVL_CMDID,
    WMI_10_4_WLAN_PROFILE_GET_PROFILE_DATA_CMDID,
    WMI_10_4_WLAN_PROFILE_ENABLE_PROFILE_ID_CMDID,
    WMI_10_4_WLAN_PROFILE_LIST_PROFILE_ID_CMDID,
    WMI_10_4_PDEV_SUSPEND_CMDID,
    WMI_10_4_PDEV_RESUME_CMDID,
    WMI_10_4_ADD_BCN_FILTER_CMDID,
    WMI_10_4_RMV_BCN_FILTER_CMDID,
    WMI_10_4_WOW_ADD_WAKE_PATTERN_CMDID,
    WMI_10_4_WOW_DEL_WAKE_PATTERN_CMDID,
    WMI_10_4_WOW_ENABLE_DISABLE_WAKE_EVENT_CMDID,
    WMI_10_4_WOW_ENABLE_CMDID,
    WMI_10_4_WOW_HOSTWAKEUP_FROM_SLEEP_CMDID,
    WMI_10_4_RTT_MEASREQ_CMDID,
    WMI_10_4_RTT_TSF_CMDID,
    WMI_10_4_RTT_KEEPALIVE_CMDID,
    WMI_10_4_OEM_REQ_CMDID,
    WMI_10_4_NAN_CMDID,
    WMI_10_4_VDEV_SPECTRAL_SCAN_CONFIGURE_CMDID,
    WMI_10_4_VDEV_SPECTRAL_SCAN_ENABLE_CMDID,
    WMI_10_4_REQUEST_STATS_CMDID,
    WMI_10_4_GPIO_CONFIG_CMDID,
    WMI_10_4_GPIO_OUTPUT_CMDID,
    WMI_10_4_VDEV_RATEMASK_CMDID,
    WMI_10_4_CSA_OFFLOAD_ENABLE_CMDID,
    WMI_10_4_GTK_OFFLOAD_CMDID,
    WMI_10_4_QBOOST_CFG_CMDID,
    WMI_10_4_CSA_OFFLOAD_CHANSWITCH_CMDID,
    WMI_10_4_PDEV_SMART_ANT_ENABLE_CMDID,
    WMI_10_4_PDEV_SMART_ANT_SET_RX_ANTENNA_CMDID,
    WMI_10_4_PEER_SMART_ANT_SET_TX_ANTENNA_CMDID,
    WMI_10_4_PEER_SMART_ANT_SET_TRAIN_INFO_CMDID,
    WMI_10_4_PEER_SMART_ANT_SET_NODE_CONFIG_OPS_CMDID,
    WMI_10_4_VDEV_SET_KEEPALIVE_CMDID,
    WMI_10_4_VDEV_GET_KEEPALIVE_CMDID,
    WMI_10_4_FORCE_FW_HANG_CMDID,
    WMI_10_4_PDEV_SET_ANTENNA_SWITCH_TABLE_CMDID,
    WMI_10_4_PDEV_SET_CTL_TABLE_CMDID,
    WMI_10_4_PDEV_SET_MIMOGAIN_TABLE_CMDID,
    WMI_10_4_PDEV_RATEPWR_TABLE_CMDID,
    WMI_10_4_PDEV_RATEPWR_CHAINMSK_TABLE_CMDID,
    WMI_10_4_PDEV_FIPS_CMDID,
    WMI_10_4_TT_SET_CONF_CMDID,
    WMI_10_4_FWTEST_CMDID,
    WMI_10_4_VDEV_ATF_REQUEST_CMDID,
    WMI_10_4_PEER_ATF_REQUEST_CMDID,
    WMI_10_4_PDEV_GET_ANI_CCK_CONFIG_CMDID,
    WMI_10_4_PDEV_GET_ANI_OFDM_CONFIG_CMDID,
    WMI_10_4_PDEV_RESERVE_AST_ENTRY_CMDID,
    WMI_10_4_PDEV_GET_NFCAL_POWER_CMDID,
    WMI_10_4_PDEV_GET_TPC_CMDID,
    WMI_10_4_PDEV_GET_AST_INFO_CMDID,
    WMI_10_4_VDEV_SET_DSCP_TID_MAP_CMDID,
    WMI_10_4_PDEV_GET_TEMPERATURE_CMDID,
    WMI_10_4_PDEV_GET_INFO_CMDID,
    WMI_10_4_VDEV_GET_INFO_CMDID,
    WMI_10_4_VDEV_FILTER_NEIGHBOR_RX_PACKETS_CMDID,
    WMI_10_4_MU_CAL_START_CMDID,
    WMI_10_4_SET_CCA_PARAMS_CMDID,
    WMI_10_4_PDEV_BSS_CHAN_INFO_REQUEST_CMDID,
    WMI_10_4_EXT_RESOURCE_CFG_CMDID,
    WMI_10_4_VDEV_SET_IE_CMDID,
    WMI_10_4_SET_LTEU_CONFIG_CMDID,
    WMI_10_4_ATF_SSID_GROUPING_REQUEST_CMDID,
    WMI_10_4_PEER_ATF_EXT_REQUEST_CMDID,
    WMI_10_4_SET_PERIODIC_CHANNEL_STATS_CONFIG,
    WMI_10_4_PEER_BWF_REQUEST_CMDID,
    WMI_10_4_BTCOEX_CFG_CMDID,
    WMI_10_4_PEER_TX_MU_TXMIT_COUNT_CMDID,
    WMI_10_4_PEER_TX_MU_TXMIT_RSTCNT_CMDID,
    WMI_10_4_PEER_GID_USERPOS_LIST_CMDID,
    WMI_10_4_PDEV_CHECK_CAL_VERSION_CMDID,
    WMI_10_4_COEX_VERSION_CFG_CMID,
    WMI_10_4_PDEV_GET_RX_FILTER_CMDID,
    WMI_10_4_PDEV_EXTENDED_NSS_CFG_CMDID,
    WMI_10_4_VDEV_SET_SCAN_NAC_RSSI_CMDID,
    WMI_10_4_PROG_GPIO_BAND_SELECT_CMDID,
    WMI_10_4_CONFIG_SMART_LOGGING_CMDID,
    WMI_10_4_DEBUG_FATAL_CONDITION_CMDID,
    WMI_10_4_GET_TSF_TIMER_CMDID,
    WMI_10_4_PDEV_GET_TPC_TABLE_CMDID,
    WMI_10_4_VDEV_SIFS_TRIGGER_TIME_CMDID,
    WMI_10_4_PDEV_WDS_ENTRY_LIST_CMDID,
    WMI_10_4_TDLS_SET_STATE_CMDID,
    WMI_10_4_TDLS_PEER_UPDATE_CMDID,
    WMI_10_4_TDLS_SET_OFFCHAN_MODE_CMDID,
    WMI_10_4_PDEV_SEND_FD_CMDID,
    WMI_10_4_ENABLE_FILS_CMDID,
    WMI_10_4_PDEV_SET_BRIDGE_MACADDR_CMDID,
    WMI_10_4_ATF_GROUP_WMM_AC_CONFIG_REQUEST_CMDID,
    WMI_10_4_RADAR_FOUND_CMDID,
    WMI_10_4_PEER_CFR_CAPTURE_CMDID,
    WMI_10_4_PER_PEER_PER_TID_CONFIG_CMDID,
    WMI_10_4_PDEV_UTF_CMDID = WMI_10_4_END_CMDID - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_4_event_id {
    WMI_10_4_SERVICE_READY_EVENTID = 0x8000,
    WMI_10_4_READY_EVENTID,
    WMI_10_4_DEBUG_MESG_EVENTID,
    WMI_10_4_START_EVENTID = 0x9000,
    WMI_10_4_END_EVENTID = 0x9FFF,
    WMI_10_4_SCAN_EVENTID = WMI_10_4_START_EVENTID,
    WMI_10_4_ECHO_EVENTID,
    WMI_10_4_UPDATE_STATS_EVENTID,
    WMI_10_4_INST_RSSI_STATS_EVENTID,
    WMI_10_4_VDEV_START_RESP_EVENTID,
    WMI_10_4_VDEV_STANDBY_REQ_EVENTID,
    WMI_10_4_VDEV_RESUME_REQ_EVENTID,
    WMI_10_4_VDEV_STOPPED_EVENTID,
    WMI_10_4_PEER_STA_KICKOUT_EVENTID,
    WMI_10_4_HOST_SWBA_EVENTID,
    WMI_10_4_TBTTOFFSET_UPDATE_EVENTID,
    WMI_10_4_MGMT_RX_EVENTID,
    WMI_10_4_CHAN_INFO_EVENTID,
    WMI_10_4_PHYERR_EVENTID,
    WMI_10_4_ROAM_EVENTID,
    WMI_10_4_PROFILE_MATCH,
    WMI_10_4_DEBUG_PRINT_EVENTID,
    WMI_10_4_PDEV_QVIT_EVENTID,
    WMI_10_4_WLAN_PROFILE_DATA_EVENTID,
    WMI_10_4_RTT_MEASUREMENT_REPORT_EVENTID,
    WMI_10_4_TSF_MEASUREMENT_REPORT_EVENTID,
    WMI_10_4_RTT_ERROR_REPORT_EVENTID,
    WMI_10_4_RTT_KEEPALIVE_EVENTID,
    WMI_10_4_OEM_CAPABILITY_EVENTID,
    WMI_10_4_OEM_MEASUREMENT_REPORT_EVENTID,
    WMI_10_4_OEM_ERROR_REPORT_EVENTID,
    WMI_10_4_NAN_EVENTID,
    WMI_10_4_WOW_WAKEUP_HOST_EVENTID,
    WMI_10_4_GTK_OFFLOAD_STATUS_EVENTID,
    WMI_10_4_GTK_REKEY_FAIL_EVENTID,
    WMI_10_4_DCS_INTERFERENCE_EVENTID,
    WMI_10_4_PDEV_TPC_CONFIG_EVENTID,
    WMI_10_4_CSA_HANDLING_EVENTID,
    WMI_10_4_GPIO_INPUT_EVENTID,
    WMI_10_4_PEER_RATECODE_LIST_EVENTID,
    WMI_10_4_GENERIC_BUFFER_EVENTID,
    WMI_10_4_MCAST_BUF_RELEASE_EVENTID,
    WMI_10_4_MCAST_LIST_AGEOUT_EVENTID,
    WMI_10_4_VDEV_GET_KEEPALIVE_EVENTID,
    WMI_10_4_WDS_PEER_EVENTID,
    WMI_10_4_PEER_STA_PS_STATECHG_EVENTID,
    WMI_10_4_PDEV_FIPS_EVENTID,
    WMI_10_4_TT_STATS_EVENTID,
    WMI_10_4_PDEV_CHANNEL_HOPPING_EVENTID,
    WMI_10_4_PDEV_ANI_CCK_LEVEL_EVENTID,
    WMI_10_4_PDEV_ANI_OFDM_LEVEL_EVENTID,
    WMI_10_4_PDEV_RESERVE_AST_ENTRY_EVENTID,
    WMI_10_4_PDEV_NFCAL_POWER_EVENTID,
    WMI_10_4_PDEV_TPC_EVENTID,
    WMI_10_4_PDEV_GET_AST_INFO_EVENTID,
    WMI_10_4_PDEV_TEMPERATURE_EVENTID,
    WMI_10_4_PDEV_NFCAL_POWER_ALL_CHANNELS_EVENTID,
    WMI_10_4_PDEV_BSS_CHAN_INFO_EVENTID,
    WMI_10_4_MU_REPORT_EVENTID,
    WMI_10_4_TX_DATA_TRAFFIC_CTRL_EVENTID,
    WMI_10_4_PEER_TX_MU_TXMIT_COUNT_EVENTID,
    WMI_10_4_PEER_GID_USERPOS_LIST_EVENTID,
    WMI_10_4_PDEV_CHECK_CAL_VERSION_EVENTID,
    WMI_10_4_ATF_PEER_STATS_EVENTID,
    WMI_10_4_PDEV_GET_RX_FILTER_EVENTID,
    WMI_10_4_NAC_RSSI_EVENTID,
    WMI_10_4_DEBUG_FATAL_CONDITION_EVENTID,
    WMI_10_4_GET_TSF_TIMER_RESP_EVENTID,
    WMI_10_4_PDEV_TPC_TABLE_EVENTID,
    WMI_10_4_PDEV_WDS_ENTRY_LIST_EVENTID,
    WMI_10_4_TDLS_PEER_EVENTID,
    WMI_10_4_HOST_SWFDA_EVENTID,
    WMI_10_4_ESP_ESTIMATE_EVENTID,
    WMI_10_4_DFS_STATUS_CHECK_EVENTID,
    WMI_10_4_PDEV_UTF_EVENTID = WMI_10_4_END_EVENTID - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_phy_mode {
    MODE_11A        = 0,   /* 11a Mode */
    MODE_11G        = 1,   /* 11b/g Mode */
    MODE_11B        = 2,   /* 11b Mode */
    MODE_11GONLY    = 3,   /* 11g only Mode */
    MODE_11NA_HT20   = 4,  /* 11a HT20 mode */
    MODE_11NG_HT20   = 5,  /* 11g HT20 mode */
    MODE_11NA_HT40   = 6,  /* 11a HT40 mode */
    MODE_11NG_HT40   = 7,  /* 11g HT40 mode */
    MODE_11AC_VHT20 = 8,
    MODE_11AC_VHT40 = 9,
    MODE_11AC_VHT80 = 10,
// MODE_11AC_VHT160 = 11,
    MODE_11AC_VHT20_2G = 11,
    MODE_11AC_VHT40_2G = 12,
    MODE_11AC_VHT80_2G = 13,
    MODE_11AC_VHT80_80 = 14,
    MODE_11AC_VHT160 = 15,
    MODE_UNKNOWN    = 16,
    MODE_MAX        = 16
}

// skip
// no default handler to allow compiler to check that the
// enum is fully handled
//
pub const WMI_CHAN_LIST_TAG: c_uint = 0x1;
pub const WMI_SSID_LIST_TAG: c_uint = 0x2;
pub const WMI_BSSID_LIST_TAG: c_uint = 0x3;
pub const WMI_IE_TAG: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_channel {
    pub mhz: __le32,
    pub band_center_freq1: __le32,
    pub /: *mut *mut __le32 band_center_freq2; / valid for 11ac, 80plus80,
    pub /: *mut *mut __le32 flags; / WMI_CHAN_FLAG_,
    pub /: *mut *mut u8 mode; / only 6 LSBs,
    pub __packed: },
    pub __packed: },
    pub reginfo0: __le32,
// note: power unit is 0.5 dBm
    pub min_power: u8,
    pub max_power: u8,
    pub reg_power: u8,
    pub reg_classid: u8,
    pub __packed: },
    pub __packed: },
    pub reginfo1: __le32,
// note: power unit is 1 dBm
    pub antenna_max: u8,
// note: power unit is 0.5 dBm
    pub max_tx_power: u8,
    pub __packed: },
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_channel_arg {
    pub freq: u32,
    pub band_center_freq1: u32,
    pub band_center_freq2: u32,
    pub passive: bool,
    pub allow_ibss: bool,
    pub allow_ht: bool,
    pub allow_vht: bool,
    pub ht40plus: bool,
    pub chan_radar: bool,
// note: power unit is 0.5 dBm
    pub min_power: u32,
    pub max_power: u32,
    pub max_reg_power: u32,
// note: power unit is 1 dBm
    pub max_antenna_gain: u32,
    pub reg_class_id: u32,
    pub mode: wmi_phy_mode,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_channel_change_cause {
    WMI_CHANNEL_CHANGE_CAUSE_NONE = 0,
    WMI_CHANNEL_CHANGE_CAUSE_CSA,
}

// Indicate reason for channel switch

// DFS required on channel for 2nd segment of VHT160 and VHT80+80

// HT Capabilities
pub const WMI_HT_CAP_ENABLED: c_uint = 0x0001   /* HT Enabled/ disabled */;
pub const WMI_HT_CAP_HT20_SGI: c_uint = 0x0002   /* Short Guard Interval with HT20 */;
pub const WMI_HT_CAP_DYNAMIC_SMPS: c_uint = 0x0004   /* Dynamic MIMO powersave */;
pub const WMI_HT_CAP_TX_STBC: c_uint = 0x0008   /* B3 TX STBC */;
pub const WMI_HT_CAP_TX_STBC_MASK_SHIFT: c_int = 3;
pub const WMI_HT_CAP_RX_STBC: c_uint = 0x0030   /* B4-B5 RX STBC */;
pub const WMI_HT_CAP_RX_STBC_MASK_SHIFT: c_int = 4;
pub const WMI_HT_CAP_LDPC: c_uint = 0x0040   /* LDPC supported */;
pub const WMI_HT_CAP_L_SIG_TXOP_PROT: c_uint = 0x0080   /* L-SIG TXOP Protection */;
pub const WMI_HT_CAP_MPDU_DENSITY: c_uint = 0x0700   /* MPDU Density */;
pub const WMI_HT_CAP_MPDU_DENSITY_MASK_SHIFT: c_int = 8;
pub const WMI_HT_CAP_HT40_SGI: c_uint = 0x0800;
pub const WMI_HT_CAP_RX_LDPC: c_uint = 0x1000   /* LDPC RX support */;
pub const WMI_HT_CAP_TX_LDPC: c_uint = 0x2000   /* LDPC TX support */;

//
// WMI_VHT_CAP_* these maps to ieee 802.11ac vht capability information
// field. The fields not defined here are not supported, or reserved.
// Do not change these masks and if you have to add new one follow the
// bitmask as specified by 802.11ac draft.
//
pub const WMI_VHT_CAP_MAX_MPDU_LEN_MASK: c_uint = 0x00000003;
pub const WMI_VHT_CAP_RX_LDPC: c_uint = 0x00000010;
pub const WMI_VHT_CAP_SGI_80MHZ: c_uint = 0x00000020;
pub const WMI_VHT_CAP_SGI_160MHZ: c_uint = 0x00000040;
pub const WMI_VHT_CAP_TX_STBC: c_uint = 0x00000080;
pub const WMI_VHT_CAP_RX_STBC_MASK: c_uint = 0x00000300;
pub const WMI_VHT_CAP_RX_STBC_MASK_SHIFT: c_int = 8;
pub const WMI_VHT_CAP_SU_BFER: c_uint = 0x00000800;
pub const WMI_VHT_CAP_SU_BFEE: c_uint = 0x00001000;
pub const WMI_VHT_CAP_MAX_CS_ANT_MASK: c_uint = 0x0000E000;
pub const WMI_VHT_CAP_MAX_CS_ANT_MASK_SHIFT: c_int = 13;
pub const WMI_VHT_CAP_MAX_SND_DIM_MASK: c_uint = 0x00070000;
pub const WMI_VHT_CAP_MAX_SND_DIM_MASK_SHIFT: c_int = 16;
pub const WMI_VHT_CAP_MU_BFER: c_uint = 0x00080000;
pub const WMI_VHT_CAP_MU_BFEE: c_uint = 0x00100000;
pub const WMI_VHT_CAP_MAX_AMPDU_LEN_EXP: c_uint = 0x03800000;
pub const WMI_VHT_CAP_MAX_AMPDU_LEN_EXP_SHIFT: c_int = 23;
pub const WMI_VHT_CAP_RX_FIXED_ANT: c_uint = 0x10000000;
pub const WMI_VHT_CAP_TX_FIXED_ANT: c_uint = 0x20000000;
// The following also refer for max HT AMSDU
pub const WMI_VHT_CAP_MAX_MPDU_LEN_3839: c_uint = 0x00000000;
pub const WMI_VHT_CAP_MAX_MPDU_LEN_7935: c_uint = 0x00000001;
pub const WMI_VHT_CAP_MAX_MPDU_LEN_11454: c_uint = 0x00000002;

//
// Interested readers refer to Rx/Tx MCS Map definition as defined in
// 802.11ac
//

pub const WMI_VHT_MAX_SUPP_RATE_MASK: c_uint = 0x1fff0000;
pub const WMI_VHT_MAX_SUPP_RATE_MASK_SHIFT: c_int = 16;
pub const REGDMN_CAP1_CHAN_HALF_RATE: c_uint = 0x00000001;
pub const REGDMN_CAP1_CHAN_QUARTER_RATE: c_uint = 0x00000002;
pub const REGDMN_CAP1_CHAN_HAL49GHZ: c_uint = 0x00000004;
// regulatory capabilities
pub const REGDMN_EEPROM_EEREGCAP_EN_FCC_MIDBAND: c_uint = 0x0040;
pub const REGDMN_EEPROM_EEREGCAP_EN_KK_U1_EVEN: c_uint = 0x0080;
pub const REGDMN_EEPROM_EEREGCAP_EN_KK_U2: c_uint = 0x0100;
pub const REGDMN_EEPROM_EEREGCAP_EN_KK_MIDBAND: c_uint = 0x0200;
pub const REGDMN_EEPROM_EEREGCAP_EN_KK_U1_ODD: c_uint = 0x0400;
pub const REGDMN_EEPROM_EEREGCAP_EN_KK_NEW_11A: c_uint = 0x0800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal_reg_capabilities {
// regdomain value specified in EEPROM
    pub eeprom_rd: __le32,
// regdomain
    pub eeprom_rd_ext: __le32,
// CAP1 capabilities bit map.
    pub regcap1: __le32,
// REGDMN EEPROM CAP.
    pub regcap2: __le32,
// REGDMN MODE
    pub wireless_modes: __le32,
    pub low_2ghz_chan: __le32,
    pub high_2ghz_chan: __le32,
    pub low_5ghz_chan: __le32,
    pub high_5ghz_chan: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlan_mode_capability {
    WHAL_WLAN_11A_CAPABILITY   = 0x1,
    WHAL_WLAN_11G_CAPABILITY   = 0x2,
    WHAL_WLAN_11AG_CAPABILITY  = 0x3,
}

// structure used by FW for requesting host memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlan_host_mem_req {
// ID of the request
    pub req_id: __le32,
// size of the  of each unit
    pub unit_size: __le32,
// flags to  indicate that
// the number units is dependent
// on number of resources(num vdevs num peers .. etc)
//
    pub num_unit_info: __le32,
//
// actual number of units to allocate . if flags in the num_unit_info
// indicate that number of units is tied to number of a particular
// resource to allocate then  num_units filed is set to 0 and host
// will derive the number units from number of the resources it is
// requesting.
//
    pub num_units: __le32,
    pub __packed: },
//
// The following struct holds optional payload for
// wmi_service_ready_event,e.g., 11ac pass some of the
// device capability to the host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_service_ready_event {
    pub sw_version: __le32,
    pub sw_version_1: __le32,
    pub abi_version: __le32,
// WMI_PHY_CAPABILITY
    pub phy_capability: __le32,
// Maximum number of frag table entries that SW will populate less 1
    pub max_frag_entry: __le32,
    pub wmi_service_bitmap: [__le32; 16],
    pub num_rf_chains: __le32,
//
// The following field is only valid for service type
// WMI_SERVICE_11AC
//
    pub /: *mut *mut __le32 ht_cap_info; / WMI HT Capability,
    pub /: *mut *mut __le32 vht_cap_info; / VHT capability info field of 802.11ac,
    pub /: *mut *mut __le32 vht_supp_mcs; / VHT Supported MCS Set field Rx/Tx same,
    pub hw_min_tx_power: __le32,
    pub hw_max_tx_power: __le32,
    pub hal_reg_capabilities: hal_reg_capabilities,
    pub sys_cap_info: __le32,
    pub /: *mut *mut __le32 min_pkt_size_enable; / Enterprise mode short pkt enable,
//
// Max beacon and Probe Response IE offload size
// (includes optional P2P IEs)
//
    pub max_bcn_ie_size: __le32,
//
// request to host to allocate a chuck of memory and pss it down to FW
// via WM_INIT. FW uses this as FW extesnsion memory for saving its
// data structures. Only valid for low latency interfaces like PCIE
// where FW can access this memory directly (or) by DMA.
//
    pub num_mem_reqs: __le32,
    pub mem_reqs: [wlan_host_mem_req; ],
    pub __packed: },
// This is the definition from 10.X firmware branch
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10x_service_ready_event {
    pub sw_version: __le32,
    pub abi_version: __le32,
// WMI_PHY_CAPABILITY
    pub phy_capability: __le32,
// Maximum number of frag table entries that SW will populate less 1
    pub max_frag_entry: __le32,
    pub wmi_service_bitmap: [__le32; 16],
    pub num_rf_chains: __le32,
//
// The following field is only valid for service type
// WMI_SERVICE_11AC
//
    pub /: *mut *mut __le32 ht_cap_info; / WMI HT Capability,
    pub /: *mut *mut __le32 vht_cap_info; / VHT capability info field of 802.11ac,
    pub /: *mut *mut __le32 vht_supp_mcs; / VHT Supported MCS Set field Rx/Tx same,
    pub hw_min_tx_power: __le32,
    pub hw_max_tx_power: __le32,
    pub hal_reg_capabilities: hal_reg_capabilities,
    pub sys_cap_info: __le32,
    pub /: *mut *mut __le32 min_pkt_size_enable; / Enterprise mode short pkt enable,
//
// request to host to allocate a chuck of memory and pss it down to FW
// via WM_INIT. FW uses this as FW extesnsion memory for saving its
// data structures. Only valid for low latency interfaces like PCIE
// where FW can access this memory directly (or) by DMA.
//
    pub num_mem_reqs: __le32,
    pub mem_reqs: [wlan_host_mem_req; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ready_event {
    pub sw_version: __le32,
    pub abi_version: __le32,
    pub mac_addr: wmi_mac_addr,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_resource_config {
// number of virtual devices (VAPs) to support
    pub num_vdevs: __le32,
// number of peer nodes to support
    pub num_peers: __le32,
//
// In offload mode target supports features like WOW, chatter and
// other protocol offloads. In order to support them some
// functionalities like reorder buffering, PN checking need to be
// done in target. This determines maximum number of peers supported
// by target in offload mode
//
    pub num_offload_peers: __le32,
// For target-based RX reordering
    pub num_offload_reorder_bufs: __le32,
// number of keys per peer
    pub num_peer_keys: __le32,
// total number of TX/RX data TIDs
    pub num_tids: __le32,
//
// max skid for resolving hash collisions
//
// The address search table is sparse, so that if two MAC addresses
// result in the same hash value, the second of these conflicting
// entries can slide to the next index in the address search table,
// and use it, if it is unoccupied.  This ast_skid_limit parameter
// specifies the upper bound on how many subsequent indices to search
// over to find an unoccupied space.
//
    pub ast_skid_limit: __le32,
//
// the nominal chain mask for transmit
//
// The chain mask may be modified dynamically, e.g. to operate AP
// tx with a reduced number of chains if no clients are associated.
// This configuration parameter specifies the nominal chain-mask that
// should be used when not operating with a reduced set of tx chains.
//
    pub tx_chain_mask: __le32,
//
// the nominal chain mask for receive
//
// The chain mask may be modified dynamically, e.g. for a client
// to use a reduced number of chains for receive if the traffic to
// the client is low enough that it doesn't require downlink MIMO
// or antenna diversity.
// This configuration parameter specifies the nominal chain-mask that
// should be used when not operating with a reduced set of rx chains.
//
    pub rx_chain_mask: __le32,
//
// what rx reorder timeout (ms) to use for the AC
//
// Each WMM access class (voice, video, best-effort, background) will
// have its own timeout value to dictate how long to wait for missing
// rx MPDUs to arrive before flushing subsequent MPDUs that have
// already been received.
// This parameter specifies the timeout in milliseconds for each
// class.
//
    pub rx_timeout_pri_vi: __le32,
    pub rx_timeout_pri_vo: __le32,
    pub rx_timeout_pri_be: __le32,
    pub rx_timeout_pri_bk: __le32,
//
// what mode the rx should decap packets to
//
// MAC can decap to RAW (no decap), native wifi or Ethernet types
// THis setting also determines the default TX behavior, however TX
// behavior can be modified on a per VAP basis during VAP init
//
    pub rx_decap_mode: __le32,
// what is the maximum number of scan requests that can be queued
    pub scan_max_pending_reqs: __le32,
// maximum VDEV that could use BMISS offload
    pub bmiss_offload_max_vdev: __le32,
// maximum VDEV that could use offload roaming
    pub roam_offload_max_vdev: __le32,
// maximum AP profiles that would push to offload roaming
    pub roam_offload_max_ap_profiles: __le32,
//
// how many groups to use for mcast->ucast conversion
//
// The target's WAL maintains a table to hold information regarding
// which peers belong to a given multicast group, so that if
// multicast->unicast conversion is enabled, the target can convert
// multicast tx frames to a series of unicast tx frames, to each
// peer within the multicast group.
// many multicast groups to provide storage for within its multicast
// group membership table.
//
    pub num_mcast_groups: __le32,
//
// size to alloc for the mcast membership table
//
// This num_mcast_table_elems configuration parameter tells the
// target how many peer elements it needs to provide storage for in
// its multicast group membership table.
// These multicast group membership table elements are shared by the
// multicast groups stored within the table.
//
    pub num_mcast_table_elems: __le32,
//
// whether/how to do multicast->unicast conversion
//
// This configuration parameter specifies whether the target should
// perform multicast --> unicast conversion on transmit, and if so,
// what to do if it finds no entries in its multicast group
// membership table for the multicast IP address in the tx frame.
// Configuration value:
// 0 -> Do not perform multicast to unicast conversion.
// 1 -> Convert multicast frames to unicast, if the IP multicast
// address from the tx frame is found in the multicast group
// membership table.  If the IP multicast address is not found,
// drop the frame.
// 2 -> Convert multicast frames to unicast, if the IP multicast
// address from the tx frame is found in the multicast group
// membership table.  If the IP multicast address is not found,
// transmit the frame as multicast.
//
    pub mcast2ucast_mode: __le32,
//
// how much memory to allocate for a tx PPDU dbg log
//
// This parameter controls how much memory the target will allocate
// to store a log of tx PPDU meta-information (how large the PPDU
// was, when it was sent, whether it was successful, etc.)
//
    pub tx_dbg_log_size: __le32,
// how many AST entries to be allocated for WDS
    pub num_wds_entries: __le32,
//
// MAC DMA burst size, e.g., For target PCI limit can be
// 0 -default, 1 256B
//
    pub dma_burst_size: __le32,
//
// Fixed delimiters to be inserted after every MPDU to
// account for interface latency to avoid underrun.
//
    pub mac_aggr_delim: __le32,
//
// determine whether target is responsible for detecting duplicate
// non-aggregate MPDU and timing out stale fragments.
//
// A-MPDU reordering is always performed on the target.
//
// 0: target responsible for frag timeout and dup checking
// 1: host responsible for frag timeout and dup checking
//
    pub rx_skip_defrag_timeout_dup_detection_check: __le32,
//
// Configuration for VoW :
// No of Video Nodes to be supported
// and Max no of descriptors for each Video link (node).
//
    pub vow_config: __le32,
// maximum VDEV that could use GTK offload
    pub gtk_offload_max_vdev: __le32,
// Number of msdu descriptors target should use
    pub num_msdu_desc: __le32,
//
// Max. number of Tx fragments per MSDU
// This parameter controls the max number of Tx fragments per MSDU.
// This is sent by the target as part of the WMI_SERVICE_READY event
// and is overridden by the OS shim as required.
//
    pub max_frag_entries: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_resource_config_10x {
// number of virtual devices (VAPs) to support
    pub num_vdevs: __le32,
// number of peer nodes to support
    pub num_peers: __le32,
// number of keys per peer
    pub num_peer_keys: __le32,
// total number of TX/RX data TIDs
    pub num_tids: __le32,
//
// max skid for resolving hash collisions
//
// The address search table is sparse, so that if two MAC addresses
// result in the same hash value, the second of these conflicting
// entries can slide to the next index in the address search table,
// and use it, if it is unoccupied.  This ast_skid_limit parameter
// specifies the upper bound on how many subsequent indices to search
// over to find an unoccupied space.
//
    pub ast_skid_limit: __le32,
//
// the nominal chain mask for transmit
//
// The chain mask may be modified dynamically, e.g. to operate AP
// tx with a reduced number of chains if no clients are associated.
// This configuration parameter specifies the nominal chain-mask that
// should be used when not operating with a reduced set of tx chains.
//
    pub tx_chain_mask: __le32,
//
// the nominal chain mask for receive
//
// The chain mask may be modified dynamically, e.g. for a client
// to use a reduced number of chains for receive if the traffic to
// the client is low enough that it doesn't require downlink MIMO
// or antenna diversity.
// This configuration parameter specifies the nominal chain-mask that
// should be used when not operating with a reduced set of rx chains.
//
    pub rx_chain_mask: __le32,
//
// what rx reorder timeout (ms) to use for the AC
//
// Each WMM access class (voice, video, best-effort, background) will
// have its own timeout value to dictate how long to wait for missing
// rx MPDUs to arrive before flushing subsequent MPDUs that have
// already been received.
// This parameter specifies the timeout in milliseconds for each
// class.
//
    pub rx_timeout_pri_vi: __le32,
    pub rx_timeout_pri_vo: __le32,
    pub rx_timeout_pri_be: __le32,
    pub rx_timeout_pri_bk: __le32,
//
// what mode the rx should decap packets to
//
// MAC can decap to RAW (no decap), native wifi or Ethernet types
// THis setting also determines the default TX behavior, however TX
// behavior can be modified on a per VAP basis during VAP init
//
    pub rx_decap_mode: __le32,
// what is the maximum number of scan requests that can be queued
    pub scan_max_pending_reqs: __le32,
// maximum VDEV that could use BMISS offload
    pub bmiss_offload_max_vdev: __le32,
// maximum VDEV that could use offload roaming
    pub roam_offload_max_vdev: __le32,
// maximum AP profiles that would push to offload roaming
    pub roam_offload_max_ap_profiles: __le32,
//
// how many groups to use for mcast->ucast conversion
//
// The target's WAL maintains a table to hold information regarding
// which peers belong to a given multicast group, so that if
// multicast->unicast conversion is enabled, the target can convert
// multicast tx frames to a series of unicast tx frames, to each
// peer within the multicast group.
// many multicast groups to provide storage for within its multicast
// group membership table.
//
    pub num_mcast_groups: __le32,
//
// size to alloc for the mcast membership table
//
// This num_mcast_table_elems configuration parameter tells the
// target how many peer elements it needs to provide storage for in
// its multicast group membership table.
// These multicast group membership table elements are shared by the
// multicast groups stored within the table.
//
    pub num_mcast_table_elems: __le32,
//
// whether/how to do multicast->unicast conversion
//
// This configuration parameter specifies whether the target should
// perform multicast --> unicast conversion on transmit, and if so,
// what to do if it finds no entries in its multicast group
// membership table for the multicast IP address in the tx frame.
// Configuration value:
// 0 -> Do not perform multicast to unicast conversion.
// 1 -> Convert multicast frames to unicast, if the IP multicast
// address from the tx frame is found in the multicast group
// membership table.  If the IP multicast address is not found,
// drop the frame.
// 2 -> Convert multicast frames to unicast, if the IP multicast
// address from the tx frame is found in the multicast group
// membership table.  If the IP multicast address is not found,
// transmit the frame as multicast.
//
    pub mcast2ucast_mode: __le32,
//
// how much memory to allocate for a tx PPDU dbg log
//
// This parameter controls how much memory the target will allocate
// to store a log of tx PPDU meta-information (how large the PPDU
// was, when it was sent, whether it was successful, etc.)
//
    pub tx_dbg_log_size: __le32,
// how many AST entries to be allocated for WDS
    pub num_wds_entries: __le32,
//
// MAC DMA burst size, e.g., For target PCI limit can be
// 0 -default, 1 256B
//
    pub dma_burst_size: __le32,
//
// Fixed delimiters to be inserted after every MPDU to
// account for interface latency to avoid underrun.
//
    pub mac_aggr_delim: __le32,
//
// determine whether target is responsible for detecting duplicate
// non-aggregate MPDU and timing out stale fragments.
//
// A-MPDU reordering is always performed on the target.
//
// 0: target responsible for frag timeout and dup checking
// 1: host responsible for frag timeout and dup checking
//
    pub rx_skip_defrag_timeout_dup_detection_check: __le32,
//
// Configuration for VoW :
// No of Video Nodes to be supported
// and Max no of descriptors for each Video link (node).
//
    pub vow_config: __le32,
// Number of msdu descriptors target should use
    pub num_msdu_desc: __le32,
//
// Max. number of Tx fragments per MSDU
// This parameter controls the max number of Tx fragments per MSDU.
// This is sent by the target as part of the WMI_SERVICE_READY event
// and is overridden by the OS shim as required.
//
    pub max_frag_entries: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_2_feature_mask {
    WMI_10_2_RX_BATCH_MODE = BIT(0),
    WMI_10_2_ATF_CONFIG    = BIT(1),
    WMI_10_2_COEX_GPIO     = BIT(3),
    WMI_10_2_BSS_CHAN_INFO = BIT(6),
    WMI_10_2_PEER_STATS    = BIT(7),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_resource_config_10_2 {
    pub common: wmi_resource_config_10x,
    pub max_peer_ext_stats: __le32,
    pub /: *mut *mut __le32 smart_ant_cap; / 0-disable, 1-enable,
    pub bk_min_free: __le32,
    pub be_min_free: __le32,
    pub vi_min_free: __le32,
    pub vo_min_free: __le32,
    pub feature_mask: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_resource_config_10_4 {
// Number of virtual devices (VAPs) to support
    pub num_vdevs: __le32,
// Number of peer nodes to support
    pub num_peers: __le32,
// Number of active peer nodes to support
    pub num_active_peers: __le32,
// In offload mode, target supports features like WOW, chatter and other
// protocol offloads. In order to support them some functionalities like
// reorder buffering, PN checking need to be done in target.
// This determines maximum number of peers supported by target in
// offload mode.
//
    pub num_offload_peers: __le32,
// Number of reorder buffers available for doing target based reorder
// Rx reorder buffering
//
    pub num_offload_reorder_buffs: __le32,
// Number of keys per peer
    pub num_peer_keys: __le32,
// Total number of TX/RX data TIDs
    pub num_tids: __le32,
// Max skid for resolving hash collisions.
// The address search table is sparse, so that if two MAC addresses
// result in the same hash value, the second of these conflicting
// entries can slide to the next index in the address search table,
// and use it, if it is unoccupied.  This ast_skid_limit parameter
// specifies the upper bound on how many subsequent indices to search
// over to find an unoccupied space.
//
    pub ast_skid_limit: __le32,
// The nominal chain mask for transmit.
// The chain mask may be modified dynamically, e.g. to operate AP tx
// with a reduced number of chains if no clients are associated.
// This configuration parameter specifies the nominal chain-mask that
// should be used when not operating with a reduced set of tx chains.
//
    pub tx_chain_mask: __le32,
// The nominal chain mask for receive.
// The chain mask may be modified dynamically, e.g. for a client to use
// a reduced number of chains for receive if the traffic to the client
// is low enough that it doesn't require downlink MIMO or antenna
// diversity. This configuration parameter specifies the nominal
// chain-mask that should be used when not operating with a reduced
// set of rx chains.
//
    pub rx_chain_mask: __le32,
// What rx reorder timeout (ms) to use for the AC.
// Each WMM access class (voice, video, best-effort, background) will
// have its own timeout value to dictate how long to wait for missing
// rx MPDUs to arrive before flushing subsequent MPDUs that have already
// been received. This parameter specifies the timeout in milliseconds
// for each class.
//
    pub rx_timeout_pri: [__le32; 4],
// What mode the rx should decap packets to.
// MAC can decap to RAW (no decap), native wifi or Ethernet types.
// This setting also determines the default TX behavior, however TX
// behavior can be modified on a per VAP basis during VAP init
//
    pub rx_decap_mode: __le32,
    pub scan_max_pending_req: __le32,
    pub bmiss_offload_max_vdev: __le32,
    pub roam_offload_max_vdev: __le32,
    pub roam_offload_max_ap_profiles: __le32,
// How many groups to use for mcast->ucast conversion.
// The target's WAL maintains a table to hold information regarding
// which peers belong to a given multicast group, so that if
// multicast->unicast conversion is enabled, the target can convert
// multicast tx frames to a series of unicast tx frames, to each peer
// within the multicast group. This num_mcast_groups configuration
// parameter tells the target how many multicast groups to provide
// storage for within its multicast group membership table.
//
    pub num_mcast_groups: __le32,
// Size to alloc for the mcast membership table.
// This num_mcast_table_elems configuration parameter tells the target
// how many peer elements it needs to provide storage for in its
// multicast group membership table. These multicast group membership
// table elements are shared by the multicast groups stored within
// the table.
//
    pub num_mcast_table_elems: __le32,
// Whether/how to do multicast->unicast conversion.
// This configuration parameter specifies whether the target should
// perform multicast --> unicast conversion on transmit, and if so,
// what to do if it finds no entries in its multicast group membership
// table for the multicast IP address in the tx frame.
// Configuration value:
// 0 -> Do not perform multicast to unicast conversion.
// 1 -> Convert multicast frames to unicast, if the IP multicast address
// from the tx frame is found in the multicast group membership
// table.  If the IP multicast address is not found, drop the frame
// 2 -> Convert multicast frames to unicast, if the IP multicast address
// from the tx frame is found in the multicast group membership
// table.  If the IP multicast address is not found, transmit the
// frame as multicast.
//
    pub mcast2ucast_mode: __le32,
// How much memory to allocate for a tx PPDU dbg log.
// This parameter controls how much memory the target will allocate to
// store a log of tx PPDU meta-information (how large the PPDU was,
// when it was sent, whether it was successful, etc.)
//
    pub tx_dbg_log_size: __le32,
// How many AST entries to be allocated for WDS
    pub num_wds_entries: __le32,
// MAC DMA burst size. 0 -default, 1 -256B
    pub dma_burst_size: __le32,
// Fixed delimiters to be inserted after every MPDU to account for
// interface latency to avoid underrun.
//
    pub mac_aggr_delim: __le32,
// Determine whether target is responsible for detecting duplicate
// non-aggregate MPDU and timing out stale fragments. A-MPDU reordering
// is always performed on the target.
//
// 0: target responsible for frag timeout and dup checking
// 1: host responsible for frag timeout and dup checking
//
    pub rx_skip_defrag_timeout_dup_detection_check: __le32,
// Configuration for VoW : No of Video nodes to be supported and max
// no of descriptors for each video link (node).
//
    pub vow_config: __le32,
// Maximum vdev that could use gtk offload
    pub gtk_offload_max_vdev: __le32,
// Number of msdu descriptors target should use
    pub num_msdu_desc: __le32,
// Max number of tx fragments per MSDU.
// This parameter controls the max number of tx fragments per MSDU.
// This will passed by target as part of the WMI_SERVICE_READY event
// and is overridden by the OS shim as required.
//
    pub max_frag_entries: __le32,
// Max number of extended peer stats.
// This parameter controls the max number of peers for which extended
// statistics are supported by target
//
    pub max_peer_ext_stats: __le32,
// Smart antenna capabilities information.
// 1 - Smart antenna is enabled
// 0 - Smart antenna is disabled
// In future this can contain smart antenna specific capabilities.
//
    pub smart_ant_cap: __le32,
// User can configure the buffers allocated for each AC (BE, BK, VI, VO)
// during init.
//
    pub bk_minfree: __le32,
    pub be_minfree: __le32,
    pub vi_minfree: __le32,
    pub vo_minfree: __le32,
// Rx batch mode capability.
// 1 - Rx batch mode enabled
// 0 - Rx batch mode disabled
//
    pub rx_batchmode: __le32,
// Thermal throttling capability.
// 1 - Capable of thermal throttling
// 0 - Not capable of thermal throttling
//
    pub tt_support: __le32,
// ATF configuration.
// 1  - Enable ATF
// 0  - Disable ATF
//
    pub atf_config: __le32,
// Configure padding to manage IP header un-alignment
// 1  - Enable padding
// 0  - Disable padding
//
    pub iphdr_pad_config: __le32,
// qwrap configuration (bits 15-0)
// 1  - This is qwrap configuration
// 0  - This is not qwrap
//
// Bits 31-16 is alloc_frag_desc_for_data_pkt (1 enables, 0 disables)
// In order to get ack-RSSI reporting and to specify the tx-rate for
// individual frames, this option must be enabled.  This uses an extra
// 4 bytes per tx-msdu descriptor, so don't enable it unless you need it.
//
    pub qwrap_config: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_coex_version {
    WMI_NO_COEX_VERSION_SUPPORT	= 0,
// 3 wire coex support
    WMI_COEX_VERSION_1		= 1,
// 2.5 wire coex support
    WMI_COEX_VERSION_2		= 2,
// 2.5 wire coex with duty cycle support
    WMI_COEX_VERSION_3		= 3,
// 4 wire coex support
    WMI_COEX_VERSION_4		= 4,
}

//
// enum wmi_10_4_feature_mask - WMI 10.4 feature enable/disable flags
// @WMI_10_4_LTEU_SUPPORT: LTEU config
// @WMI_10_4_COEX_GPIO_SUPPORT: COEX GPIO config
// @WMI_10_4_AUX_RADIO_SPECTRAL_INTF: AUX Radio Enhancement for spectral scan
// @WMI_10_4_AUX_RADIO_CHAN_LOAD_INTF: AUX Radio Enhancement for chan load scan
// @WMI_10_4_BSS_CHANNEL_INFO_64: BSS channel info stats
// @WMI_10_4_PEER_STATS: Per station stats
// @WMI_10_4_VDEV_STATS: Per vdev stats
// @WMI_10_4_TDLS: Implicit TDLS support in firmware enable/disable
// @WMI_10_4_TDLS_OFFCHAN: TDLS offchannel support enable/disable
// @WMI_10_4_TDLS_UAPSD_BUFFER_STA: TDLS buffer sta support enable/disable
// @WMI_10_4_TDLS_UAPSD_SLEEP_STA: TDLS sleep sta support enable/disable
// @WMI_10_4_TDLS_CONN_TRACKER_IN_HOST_MODE: TDLS connection tracker in host
// enable/disable
// @WMI_10_4_TDLS_EXPLICIT_MODE_ONLY: Explicit TDLS mode enable/disable
// @WMI_10_4_TX_DATA_ACK_RSSI: Enable DATA ACK RSSI if firmware is capable
// @WMI_10_4_EXT_PEER_TID_CONFIGS_SUPPORT:  Firmware supports Extended Peer
// TID configuration for QoS related settings
// @WMI_10_4_REPORT_AIRTIME: Firmware supports transmit airtime reporting
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_4_feature_mask {
    WMI_10_4_LTEU_SUPPORT			= BIT(0),
    WMI_10_4_COEX_GPIO_SUPPORT		= BIT(1),
    WMI_10_4_AUX_RADIO_SPECTRAL_INTF	= BIT(2),
    WMI_10_4_AUX_RADIO_CHAN_LOAD_INTF	= BIT(3),
    WMI_10_4_BSS_CHANNEL_INFO_64		= BIT(4),
    WMI_10_4_PEER_STATS			= BIT(5),
    WMI_10_4_VDEV_STATS			= BIT(6),
    WMI_10_4_TDLS				= BIT(7),
    WMI_10_4_TDLS_OFFCHAN			= BIT(8),
    WMI_10_4_TDLS_UAPSD_BUFFER_STA		= BIT(9),
    WMI_10_4_TDLS_UAPSD_SLEEP_STA		= BIT(10),
    WMI_10_4_TDLS_CONN_TRACKER_IN_HOST_MODE = BIT(11),
    WMI_10_4_TDLS_EXPLICIT_MODE_ONLY	= BIT(12),
    WMI_10_4_TX_DATA_ACK_RSSI               = BIT(16),
    WMI_10_4_EXT_PEER_TID_CONFIGS_SUPPORT	= BIT(17),
    WMI_10_4_REPORT_AIRTIME			= BIT(18),

}

// WMI_GPIO_CONFIG_CMDID
}

// WMI_GPIO_CONFIG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_gpio_config_cmd {
    pub /: *mut *mut __le32 gpio_num; / GPIO number to be setup,
    pub /: *mut *mut __le32 input; / 0 - Output/ 1 - Input,
    pub /: *mut *mut __le32 pull_type; / Pull type defined above,
    pub /: *mut *mut __le32 intr_mode; / Interrupt mode defined above (Input),
    pub __packed: },
// WMI_GPIO_OUTPUT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_gpio_output_cmd {
    pub /: *mut *mut __le32 gpio_num; / GPIO number to be setup,
    pub pin*/: *mut *mut __le32 set; / Set the GPIO,
    pub __packed: },
// WMI_GPIO_INPUT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_gpio_input_event {
    pub /: *mut *mut __le32 gpio_num; / GPIO number which changed state,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ext_resource_config_10_4_cmd {
// contains enum wmi_host_platform_type
    pub host_platform_config: __le32,
// see enum wmi_10_4_feature_mask
    pub fw_feature_bitmap: __le32,
// WLAN priority GPIO number
    pub wlan_gpio_priority: __le32,
// see enum wmi_coex_version
    pub coex_version: __le32,
// COEX GPIO config
    pub coex_gpio_pin1: __le32,
    pub coex_gpio_pin2: __le32,
    pub coex_gpio_pin3: __le32,
// number of vdevs allowed to perform tdls
    pub num_tdls_vdevs: __le32,
// number of peers to track per TDLS vdev
    pub num_tdls_conn_table_entries: __le32,
// number of tdls sleep sta supported
    pub max_tdls_concurrent_sleep_sta: __le32,
// number of tdls buffer sta supported
    pub max_tdls_concurrent_buffer_sta: __le32,
}

// structure describing host memory chunk.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_memory_chunk {
// id of the request that is passed up in service ready
    pub req_id: __le32,
// the physical address the memory chunk
    pub ptr: __le32,
// size of the chunk
    pub size: __le32,
    pub __packed: },
pub const WMI_IRAM_RECOVERY_HOST_MEM_REQ_ID: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_host_mem_chunks {
    pub count: __le32,
// some fw revisions require at least 1 chunk regardless of count
    pub item: host_memory_chunk,
    pub items): DECLARE_FLEX_ARRAY(struct host_memory_chunk,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_init_cmd {
    pub resource_config: wmi_resource_config,
    pub mem_chunks: wmi_host_mem_chunks,
    pub __packed: },
// _10x structure is from 10.X FW API
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_init_cmd_10x {
    pub resource_config: wmi_resource_config_10x,
    pub mem_chunks: wmi_host_mem_chunks,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_init_cmd_10_2 {
    pub resource_config: wmi_resource_config_10_2,
    pub mem_chunks: wmi_host_mem_chunks,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_init_cmd_10_4 {
    pub resource_config: wmi_resource_config_10_4,
    pub mem_chunks: wmi_host_mem_chunks,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_chan_list_entry {
    pub freq: __le16,
    pub /: *mut *mut u8 phy_mode; / valid for 10.2 only,
    pub reserved: u8,
    pub __packed: },
// TLV for channel list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_chan_list {
    pub /: *mut *mut __le32 tag; / WMI_CHAN_LIST_TAG,
    pub num_chan: __le32,
    pub channel_list: [wmi_chan_list_entry; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bssid_list {
    pub /: *mut *mut __le32 tag; / WMI_BSSID_LIST_TAG,
    pub num_bssid: __le32,
    pub bssid_list: [wmi_mac_addr; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ie_data {
    pub /: *mut *mut __le32 tag; / WMI_IE_TAG,
    pub ie_len: __le32,
    pub ie_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ssid {
    pub ssid_len: __le32,
    pub ssid: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ssid_list {
    pub /: *mut *mut __le32 tag; / WMI_SSID_LIST_TAG,
    pub num_ssids: __le32,
    pub ssids: [wmi_ssid; ],
    pub __packed: },
// prefix used by scan requestor ids on the host
pub const WMI_HOST_SCAN_REQUESTOR_ID_PREFIX: c_uint = 0xA000;
// prefix used by scan request ids generated on the host
// host cycles through the lower 12 bits to generate ids
pub const WMI_HOST_SCAN_REQ_ID_PREFIX: c_uint = 0xA000;
pub const WLAN_SCAN_PARAMS_MAX_SSID: c_int = 16;
pub const WLAN_SCAN_PARAMS_MAX_BSSID: c_int = 4;
pub const WLAN_SCAN_PARAMS_MAX_IE_LEN: c_int = 256;
// Values lower than this may be refused by some firmware revisions with a scan
// completion with a timedout reason.
//
pub const WMI_SCAN_CHAN_MIN_TIME_MSEC: c_int = 40;
// Scan priority numbers must be sequential, starting with 0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_scan_priority {
    WMI_SCAN_PRIORITY_VERY_LOW = 0,
    WMI_SCAN_PRIORITY_LOW,
    WMI_SCAN_PRIORITY_MEDIUM,
    WMI_SCAN_PRIORITY_HIGH,
    WMI_SCAN_PRIORITY_VERY_HIGH,
    WMI_SCAN_PRIORITY_COUNT   /* number of priorities supported */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_start_scan_common {
// Scan ID
    pub scan_id: __le32,
// Scan requestor ID
    pub scan_req_id: __le32,
// VDEV id(interface) that is requesting scan
    pub vdev_id: __le32,
// Scan Priority, input to scan scheduler
    pub scan_priority: __le32,
// Scan events subscription
    pub notify_scan_events: __le32,
// dwell time in msec on active channels
    pub dwell_time_active: __le32,
// dwell time in msec on passive channels
    pub dwell_time_passive: __le32,
//
// min time in msec on the BSS channel,only valid if at least one
// VDEV is active
//
    pub min_rest_time: __le32,
//
// max rest time in msec on the BSS channel,only valid if at least
// one VDEV is active
//
// the scanner will rest on the bss channel at least min_rest_time
// after min_rest_time the scanner will start checking for tx/rx
// activity on all VDEVs. if there is no activity the scanner will
// switch to off channel. if there is activity the scanner will let
// the radio on the bss channel until max_rest_time expires.at
// max_rest_time scanner will switch to off channel irrespective of
// activity. activity is determined by the idle_time parameter.
//
    pub max_rest_time: __le32,
//
// time before sending next set of probe requests.
// The scanner keeps repeating probe requests transmission with
// period specified by repeat_probe_time.
// The number of probe requests specified depends on the ssid_list
// and bssid_list
//
    pub repeat_probe_time: __le32,
// time in msec between 2 consecutive probe requests with in a set.
    pub probe_spacing_time: __le32,
//
// data inactivity time in msec on bss channel that will be used by
// scanner for measuring the inactivity.
//
    pub idle_time: __le32,
// maximum time in msec allowed for scan
    pub max_scan_time: __le32,
//
// delay in msec before sending first probe request after switching
// to a channel
//
    pub probe_delay: __le32,
// Scan control flags
    pub scan_ctrl_flags: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_start_scan_cmd {
    pub common: wmi_start_scan_common,
    pub burst_duration_ms: __le32,
    pub tlvs: [u8; ],
    pub __packed: },
// This is the definition from 10.X firmware branch
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10x_start_scan_cmd {
    pub common: wmi_start_scan_common,
    pub tlvs: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ssid_arg {
    pub len: c_int,
    pub ssid: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bssid_arg {
    pub bssid: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_start_scan_arg {
    pub scan_id: u32,
    pub scan_req_id: u32,
    pub vdev_id: u32,
    pub scan_priority: u32,
    pub notify_scan_events: u32,
    pub dwell_time_active: u32,
    pub dwell_time_passive: u32,
    pub min_rest_time: u32,
    pub max_rest_time: u32,
    pub repeat_probe_time: u32,
    pub probe_spacing_time: u32,
    pub idle_time: u32,
    pub max_scan_time: u32,
    pub probe_delay: u32,
    pub scan_ctrl_flags: u32,
    pub burst_duration_ms: u32,
    pub ie_len: u32,
    pub n_channels: u32,
    pub n_ssids: u32,
    pub n_bssids: u32,
    pub ie: [u8; WLAN_SCAN_PARAMS_MAX_IE_LEN],
    pub channels: [u16; 64],
    pub ssids: [wmi_ssid_arg; WLAN_SCAN_PARAMS_MAX_SSID],
    pub bssids: [wmi_bssid_arg; WLAN_SCAN_PARAMS_MAX_BSSID],
    pub mac_addr: wmi_mac_addr,
    pub mac_mask: wmi_mac_addr,
}

// scan control flags
// passively scan all channels including active channels
pub const WMI_SCAN_FLAG_PASSIVE: c_uint = 0x1;
// add wild card ssid probe request even though ssid_list is specified.
pub const WMI_SCAN_ADD_BCAST_PROBE_REQ: c_uint = 0x2;
// add cck rates to rates/xrate ie for the generated probe request
pub const WMI_SCAN_ADD_CCK_RATES: c_uint = 0x4;
// add ofdm rates to rates/xrate ie for the generated probe request
pub const WMI_SCAN_ADD_OFDM_RATES: c_uint = 0x8;
// To enable indication of Chan load and Noise floor to host
pub const WMI_SCAN_CHAN_STAT_EVENT: c_uint = 0x10;
// Filter Probe request frames
pub const WMI_SCAN_FILTER_PROBE_REQ: c_uint = 0x20;
// When set, DFS channels will not be scanned
pub const WMI_SCAN_BYPASS_DFS_CHN: c_uint = 0x40;
// Different FW scan engine may choose to bail out on errors.
// Allow the driver to have influence over that.
//
pub const WMI_SCAN_CONTINUE_ON_ERROR: c_uint = 0x80;
// Use random MAC address for TA for Probe Request frame and add
// OUI specified by WMI_SCAN_PROB_REQ_OUI_CMDID to the Probe Request frame.
// if OUI is not set by WMI_SCAN_PROB_REQ_OUI_CMDID then the flag is ignored.
//
pub const WMI_SCAN_ADD_SPOOFED_MAC_IN_PROBE_REQ: c_uint = 0x1000;
// WMI_SCAN_CLASS_MASK must be the same value as IEEE80211_SCAN_CLASS_MASK
pub const WMI_SCAN_CLASS_MASK: c_uint = 0xFF000000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_stop_scan_type {
    WMI_SCAN_STOP_ONE	= 0x00000000, /* stop by scan_id */
    WMI_SCAN_STOP_VDEV_ALL	= 0x01000000, /* stop by vdev_id */
    WMI_SCAN_STOP_ALL	= 0x04000000, /* stop all scans */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_stop_scan_cmd {
    pub scan_req_id: __le32,
    pub scan_id: __le32,
    pub req_type: __le32,
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_stop_scan_arg {
    pub req_id: u32,
    pub req_type: wmi_stop_scan_type,
    pub scan_id: u32,
    pub vdev_id: u32,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_scan_chan_list_cmd {
    pub num_scan_chans: __le32,
    pub chan_info: [wmi_channel; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_scan_chan_list_arg {
    pub n_channels: u32,
    pub channels: *mut wmi_channel_arg,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_bss_filter {
    WMI_BSS_FILTER_NONE = 0,        /* no beacons forwarded */
    WMI_BSS_FILTER_ALL,             /* all beacons forwarded */
    WMI_BSS_FILTER_PROFILE,         /* only beacons matching profile */
    WMI_BSS_FILTER_ALL_BUT_PROFILE, /* all but beacons matching profile */
    WMI_BSS_FILTER_CURRENT_BSS,     /* only beacons matching current BSS */
    WMI_BSS_FILTER_ALL_BUT_BSS,     /* all but beacons matching BSS */
    WMI_BSS_FILTER_PROBED_SSID,     /* beacons matching probed ssid */
    WMI_BSS_FILTER_LAST_BSS,        /* marker only */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_scan_event_type {
    WMI_SCAN_EVENT_STARTED              = BIT(0),
    WMI_SCAN_EVENT_COMPLETED            = BIT(1),
    WMI_SCAN_EVENT_BSS_CHANNEL          = BIT(2),
    WMI_SCAN_EVENT_FOREIGN_CHANNEL      = BIT(3),
    WMI_SCAN_EVENT_DEQUEUED             = BIT(4),
// possibly by high-prio scan
    WMI_SCAN_EVENT_PREEMPTED            = BIT(5),
    WMI_SCAN_EVENT_START_FAILED         = BIT(6),
    WMI_SCAN_EVENT_RESTARTED            = BIT(7),
    WMI_SCAN_EVENT_FOREIGN_CHANNEL_EXIT = BIT(8),
    WMI_SCAN_EVENT_MAX                  = BIT(15),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_scan_completion_reason {
    WMI_SCAN_REASON_COMPLETED,
    WMI_SCAN_REASON_CANCELLED,
    WMI_SCAN_REASON_PREEMPTED,
    WMI_SCAN_REASON_TIMEDOUT,
    WMI_SCAN_REASON_INTERNAL_FAILURE,
    WMI_SCAN_REASON_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_scan_event {
    pub /: *mut *mut __le32 event_type; / %WMI_SCAN_EVENT_,
    pub /: *mut *mut __le32 reason; / %WMI_SCAN_REASON_,
    pub /: *mut *mut __le32 channel_freq; / only valid for WMI_SCAN_EVENT_FOREIGN_CHANNEL,
    pub scan_req_id: __le32,
    pub scan_id: __le32,
    pub vdev_id: __le32,
    pub __packed: },
//
// This defines how much headroom is kept in the
// receive frame between the descriptor and the
// payload, in order for the WMI PHY error and
// management handler to insert header contents.
//
// This is in bytes.
//
pub const WMI_MGMT_RX_HDR_HEADROOM: c_int = 52;
//
// This event will be used for sending scan results
// as well as rx mgmt frames to the host. The rx buffer
// will be sent as part of this WMI event. It would be a
// good idea to pass all the fields in the RX status
// descriptor up to the host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mgmt_rx_hdr_v1 {
    pub channel: __le32,
    pub snr: __le32,
    pub rate: __le32,
    pub phy_mode: __le32,
    pub buf_len: __le32,
    pub /: *mut *mut __le32 status; / %WMI_RX_STATUS_,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mgmt_rx_hdr_v2 {
    pub v1: wmi_mgmt_rx_hdr_v1,
    pub rssi_ctl: [__le32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mgmt_rx_event_v1 {
    pub hdr: wmi_mgmt_rx_hdr_v1,
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mgmt_rx_event_v2 {
    pub hdr: wmi_mgmt_rx_hdr_v2,
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_mgmt_rx_hdr {
    pub channel: __le32,
    pub snr: __le32,
    pub rssi_ctl: [u8; 4],
    pub rate: __le32,
    pub phy_mode: __le32,
    pub buf_len: __le32,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_mgmt_rx_event {
    pub hdr: wmi_10_4_mgmt_rx_hdr,
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mgmt_rx_ext_info {
    pub rx_mac_timestamp: __le64,
    pub __aligned(4): } __packed,
pub const WMI_RX_STATUS_OK: c_uint = 0x00;
pub const WMI_RX_STATUS_ERR_CRC: c_uint = 0x01;
pub const WMI_RX_STATUS_ERR_DECRYPT: c_uint = 0x08;
pub const WMI_RX_STATUS_ERR_MIC: c_uint = 0x10;
pub const WMI_RX_STATUS_ERR_KEY_CACHE_MISS: c_uint = 0x20;
// Extension data at the end of mgmt frame
pub const WMI_RX_STATUS_EXT_INFO: c_uint = 0x40;
pub const PHY_ERROR_GEN_SPECTRAL_SCAN: c_uint = 0x26;
pub const PHY_ERROR_GEN_FALSE_RADAR_EXT: c_uint = 0x24;
pub const PHY_ERROR_GEN_RADAR: c_uint = 0x05;
pub const PHY_ERROR_10_4_RADAR_MASK: c_uint = 0x4;
pub const PHY_ERROR_10_4_SPECTRAL_SCAN_MASK: c_uint = 0x4000000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_err_type {
    PHY_ERROR_UNKNOWN,
    PHY_ERROR_SPECTRAL_SCAN,
    PHY_ERROR_FALSE_RADAR_EXT,
    PHY_ERROR_RADAR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_phyerr {
    pub tsf_timestamp: __le32,
    pub freq1: __le16,
    pub freq2: __le16,
    pub rssi_combined: u8,
    pub chan_width_mhz: u8,
    pub phy_err_code: u8,
    pub rsvd0: u8,
    pub rssi_chains: [__le32; 4],
    pub nf_chains: [__le16; 4],
    pub buf_len: __le32,
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_phyerr_event {
    pub num_phyerrs: __le32,
    pub tsf_l32: __le32,
    pub tsf_u32: __le32,
// array of struct wmi_phyerr
    pub phyerrs: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_phyerr_event {
    pub tsf_l32: __le32,
    pub tsf_u32: __le32,
    pub freq1: __le16,
    pub freq2: __le16,
    pub rssi_combined: u8,
    pub chan_width_mhz: u8,
    pub phy_err_code: u8,
    pub rsvd0: u8,
    pub rssi_chains: [__le32; 4],
    pub nf_chains: [__le16; 4],
    pub phy_err_mask: [__le32; 2],
    pub tsf_timestamp: __le32,
    pub buf_len: __le32,
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_found_info {
    pub pri_min: __le32,
    pub pri_max: __le32,
    pub width_min: __le32,
    pub width_max: __le32,
    pub sidx_min: __le32,
    pub sidx_max: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_radar_confirmation_status {
// Detected radar was due to SW pulses
    WMI_SW_RADAR_DETECTED    = 0,

    WMI_RADAR_DETECTION_FAIL = 1,

// Real radar detected
    WMI_HW_RADAR_DETECTED    = 2,
}

pub const PHYERR_TLV_SIG: c_uint = 0xBB;
pub const PHYERR_TLV_TAG_SEARCH_FFT_REPORT: c_uint = 0xFB;
pub const PHYERR_TLV_TAG_RADAR_PULSE_SUMMARY: c_uint = 0xF8;
pub const PHYERR_TLV_TAG_SPECTRAL_SUMMARY_REPORT: c_uint = 0xF9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phyerr_radar_report {
    pub /: *mut *mut *mut __le32 reg0; / RADAR_REPORT_REG0_,
    pub /: *mut *mut *mut __le32 reg1; / RADAR_REPORT_REG1_,
    pub __packed: },
pub const RADAR_REPORT_REG0_PULSE_IS_CHIRP_MASK: c_uint = 0x80000000;
pub const RADAR_REPORT_REG0_PULSE_IS_CHIRP_LSB: c_int = 31;
pub const RADAR_REPORT_REG0_PULSE_IS_MAX_WIDTH_MASK: c_uint = 0x40000000;
pub const RADAR_REPORT_REG0_PULSE_IS_MAX_WIDTH_LSB: c_int = 30;
pub const RADAR_REPORT_REG0_AGC_TOTAL_GAIN_MASK: c_uint = 0x3FF00000;
pub const RADAR_REPORT_REG0_AGC_TOTAL_GAIN_LSB: c_int = 20;
pub const RADAR_REPORT_REG0_PULSE_DELTA_DIFF_MASK: c_uint = 0x000F0000;
pub const RADAR_REPORT_REG0_PULSE_DELTA_DIFF_LSB: c_int = 16;
pub const RADAR_REPORT_REG0_PULSE_DELTA_PEAK_MASK: c_uint = 0x0000FC00;
pub const RADAR_REPORT_REG0_PULSE_DELTA_PEAK_LSB: c_int = 10;
pub const RADAR_REPORT_REG0_PULSE_SIDX_MASK: c_uint = 0x000003FF;
pub const RADAR_REPORT_REG0_PULSE_SIDX_LSB: c_int = 0;
pub const RADAR_REPORT_REG1_PULSE_SRCH_FFT_VALID_MASK: c_uint = 0x80000000;
pub const RADAR_REPORT_REG1_PULSE_SRCH_FFT_VALID_LSB: c_int = 31;
pub const RADAR_REPORT_REG1_PULSE_AGC_MB_GAIN_MASK: c_uint = 0x7F000000;
pub const RADAR_REPORT_REG1_PULSE_AGC_MB_GAIN_LSB: c_int = 24;
pub const RADAR_REPORT_REG1_PULSE_SUBCHAN_MASK_MASK: c_uint = 0x00FF0000;
pub const RADAR_REPORT_REG1_PULSE_SUBCHAN_MASK_LSB: c_int = 16;
pub const RADAR_REPORT_REG1_PULSE_TSF_OFFSET_MASK: c_uint = 0x0000FF00;
pub const RADAR_REPORT_REG1_PULSE_TSF_OFFSET_LSB: c_int = 8;
pub const RADAR_REPORT_REG1_PULSE_DUR_MASK: c_uint = 0x000000FF;
pub const RADAR_REPORT_REG1_PULSE_DUR_LSB: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phyerr_fft_report {
    pub /: *mut *mut *mut __le32 reg0; / SEARCH_FFT_REPORT_REG0_,
    pub /: *mut *mut *mut __le32 reg1; / SEARCH_FFT_REPORT_REG1_,
    pub __packed: },
pub const SEARCH_FFT_REPORT_REG0_TOTAL_GAIN_DB_MASK: c_uint = 0xFF800000;
pub const SEARCH_FFT_REPORT_REG0_TOTAL_GAIN_DB_LSB: c_int = 23;
pub const SEARCH_FFT_REPORT_REG0_BASE_PWR_DB_MASK: c_uint = 0x007FC000;
pub const SEARCH_FFT_REPORT_REG0_BASE_PWR_DB_LSB: c_int = 14;
pub const SEARCH_FFT_REPORT_REG0_FFT_CHN_IDX_MASK: c_uint = 0x00003000;
pub const SEARCH_FFT_REPORT_REG0_FFT_CHN_IDX_LSB: c_int = 12;
pub const SEARCH_FFT_REPORT_REG0_PEAK_SIDX_MASK: c_uint = 0x00000FFF;
pub const SEARCH_FFT_REPORT_REG0_PEAK_SIDX_LSB: c_int = 0;
pub const SEARCH_FFT_REPORT_REG1_RELPWR_DB_MASK: c_uint = 0xFC000000;
pub const SEARCH_FFT_REPORT_REG1_RELPWR_DB_LSB: c_int = 26;
pub const SEARCH_FFT_REPORT_REG1_AVGPWR_DB_MASK: c_uint = 0x03FC0000;
pub const SEARCH_FFT_REPORT_REG1_AVGPWR_DB_LSB: c_int = 18;
pub const SEARCH_FFT_REPORT_REG1_PEAK_MAG_MASK: c_uint = 0x0003FF00;
pub const SEARCH_FFT_REPORT_REG1_PEAK_MAG_LSB: c_int = 8;
pub const SEARCH_FFT_REPORT_REG1_NUM_STR_BINS_IB_MASK: c_uint = 0x000000FF;
pub const SEARCH_FFT_REPORT_REG1_NUM_STR_BINS_IB_LSB: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phyerr_tlv {
    pub len: __le16,
    pub tag: u8,
    pub sig: u8,
    pub __packed: },
pub const DFS_RSSI_POSSIBLY_FALSE: c_int = 50;
pub const DFS_PEAK_MAG_THOLD_POSSIBLY_FALSE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mgmt_tx_hdr {
    pub vdev_id: __le32,
    pub peer_macaddr: wmi_mac_addr,
    pub tx_rate: __le32,
    pub tx_power: __le32,
    pub buf_len: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mgmt_tx_cmd {
    pub hdr: wmi_mgmt_tx_hdr,
    pub buf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_echo_event {
    pub value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_echo_cmd {
    pub value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_set_regdomain_cmd {
    pub reg_domain: __le32,
    pub reg_domain_2G: __le32,
    pub reg_domain_5G: __le32,
    pub conformance_test_limit_2G: __le32,
    pub conformance_test_limit_5G: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_dfs_region {
// Uninitialized dfs domain
    WMI_UNINIT_DFS_DOMAIN = 0,

// FCC3 dfs domain
    WMI_FCC_DFS_DOMAIN = 1,

// ETSI dfs domain
    WMI_ETSI_DFS_DOMAIN = 2,

// Japan dfs domain
    WMI_MKK4_DFS_DOMAIN = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_set_regdomain_cmd_10x {
    pub reg_domain: __le32,
    pub reg_domain_2G: __le32,
    pub reg_domain_5G: __le32,
    pub conformance_test_limit_2G: __le32,
    pub conformance_test_limit_5G: __le32,
// dfs domain from wmi_dfs_region
    pub dfs_domain: __le32,
    pub __packed: },
// Command to set/unset chip in quiet mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_set_quiet_cmd {
// period in TUs
    pub period: __le32,
// duration in TUs
    pub duration: __le32,
// offset in TUs
    pub next_start: __le32,
// enable/disable
    pub enabled: __le32,
    pub __packed: },
//
// 802.11g protection mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_protmode {
    ATH10K_PROT_NONE     = 0,    /* no protection */
    ATH10K_PROT_CTSONLY  = 1,    /* CTS to self */
    ATH10K_PROT_RTSCTS   = 2,    /* RTS-CTS */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_rtscts_profile {
    WMI_RTSCTS_FOR_NO_RATESERIES = 0,
    WMI_RTSCTS_FOR_SECOND_RATESERIES,
    WMI_RTSCTS_ACROSS_SW_RETRIES
}

pub const WMI_RTSCTS_ENABLED: c_int = 1;
pub const WMI_RTSCTS_SET_MASK: c_uint = 0x0f;
pub const WMI_RTSCTS_SET_LSB: c_int = 0;
pub const WMI_RTSCTS_PROFILE_MASK: c_uint = 0xf0;
pub const WMI_RTSCTS_PROFILE_LSB: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_beacon_gen_mode {
    WMI_BEACON_STAGGERED_MODE = 0,
    WMI_BEACON_BURST_MODE = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_csa_event_ies_present_flag {
    WMI_CSA_IE_PRESENT = 0x00000001,
    WMI_XCSA_IE_PRESENT = 0x00000002,
    WMI_WBW_IE_PRESENT = 0x00000004,
    WMI_CSWARP_IE_PRESENT = 0x00000008,
}

// wmi CSA receive event from beacon frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_csa_event {
    pub i_fc_dur: __le32,
// Bit 0-15: FC
// Bit 16-31: DUR
    pub i_addr1: wmi_mac_addr,
    pub i_addr2: wmi_mac_addr,
    pub csa_ie: [__le32; 2],
    pub xcsa_ie: [__le32; 2],
    pub wb_ie: [__le32; 2],
    pub cswarp_ie: __le32,
    pub /: *mut *mut __le32 ies_present_flag; / wmi_csa_event_ies_present_flag,
    pub __packed: },
// the definition of different PDEV parameters
pub const PDEV_DEFAULT_STATS_UPDATE_PERIOD: c_int = 500;
pub const VDEV_DEFAULT_STATS_UPDATE_PERIOD: c_int = 500;
pub const PEER_DEFAULT_STATS_UPDATE_PERIOD: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_param_map {
    pub tx_chain_mask: u32,
    pub rx_chain_mask: u32,
    pub txpower_limit2g: u32,
    pub txpower_limit5g: u32,
    pub txpower_scale: u32,
    pub beacon_gen_mode: u32,
    pub beacon_tx_mode: u32,
    pub resmgr_offchan_mode: u32,
    pub protection_mode: u32,
    pub dynamic_bw: u32,
    pub non_agg_sw_retry_th: u32,
    pub agg_sw_retry_th: u32,
    pub sta_kickout_th: u32,
    pub ac_aggrsize_scaling: u32,
    pub ltr_enable: u32,
    pub ltr_ac_latency_be: u32,
    pub ltr_ac_latency_bk: u32,
    pub ltr_ac_latency_vi: u32,
    pub ltr_ac_latency_vo: u32,
    pub ltr_ac_latency_timeout: u32,
    pub ltr_sleep_override: u32,
    pub ltr_rx_override: u32,
    pub ltr_tx_activity_timeout: u32,
    pub l1ss_enable: u32,
    pub dsleep_enable: u32,
    pub pcielp_txbuf_flush: u32,
    pub pcielp_txbuf_watermark: u32,
    pub pcielp_txbuf_tmo_en: u32,
    pub pcielp_txbuf_tmo_value: u32,
    pub pdev_stats_update_period: u32,
    pub vdev_stats_update_period: u32,
    pub peer_stats_update_period: u32,
    pub bcnflt_stats_update_period: u32,
    pub pmf_qos: u32,
    pub arp_ac_override: u32,
    pub dcs: u32,
    pub ani_enable: u32,
    pub ani_poll_period: u32,
    pub ani_listen_period: u32,
    pub ani_ofdm_level: u32,
    pub ani_cck_level: u32,
    pub dyntxchain: u32,
    pub proxy_sta: u32,
    pub idle_ps_config: u32,
    pub power_gating_sleep: u32,
    pub fast_channel_reset: u32,
    pub burst_dur: u32,
    pub burst_enable: u32,
    pub cal_period: u32,
    pub aggr_burst: u32,
    pub rx_decap_mode: u32,
    pub smart_antenna_default_antenna: u32,
    pub igmpmld_override: u32,
    pub igmpmld_tid: u32,
    pub antenna_gain: u32,
    pub rx_filter: u32,
    pub set_mcast_to_ucast_tid: u32,
    pub proxy_sta_mode: u32,
    pub set_mcast2ucast_mode: u32,
    pub set_mcast2ucast_buffer: u32,
    pub remove_mcast2ucast_buffer: u32,
    pub peer_sta_ps_statechg_enable: u32,
    pub igmpmld_ac_override: u32,
    pub block_interbss: u32,
    pub set_disable_reset_cmdid: u32,
    pub set_msdu_ttl_cmdid: u32,
    pub set_ppdu_duration_cmdid: u32,
    pub txbf_sound_period_cmdid: u32,
    pub set_promisc_mode_cmdid: u32,
    pub set_burst_mode_cmdid: u32,
    pub en_stats: u32,
    pub mu_group_policy: u32,
    pub noise_detection: u32,
    pub noise_threshold: u32,
    pub dpd_enable: u32,
    pub set_mcast_bcast_echo: u32,
    pub atf_strict_sch: u32,
    pub atf_sched_duration: u32,
    pub ant_plzn: u32,
    pub mgmt_retry_limit: u32,
    pub sensitivity_level: u32,
    pub signed_txpower_2g: u32,
    pub signed_txpower_5g: u32,
    pub enable_per_tid_amsdu: u32,
    pub enable_per_tid_ampdu: u32,
    pub cca_threshold: u32,
    pub rts_fixed_rate: u32,
    pub pdev_reset: u32,
    pub wapi_mbssid_offset: u32,
    pub arp_srcaddr: u32,
    pub arp_dstaddr: u32,
    pub enable_btcoex: u32,
    pub rfkill_config: u32,
    pub rfkill_enable: u32,
    pub peer_stats_info_enable: u32,
}

pub const WMI_PDEV_PARAM_UNSUPPORTED: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_pdev_param {
// TX chain mask
    WMI_PDEV_PARAM_TX_CHAIN_MASK = 0x1,
// RX chain mask
    WMI_PDEV_PARAM_RX_CHAIN_MASK,
// TX power limit for 2G Radio
    WMI_PDEV_PARAM_TXPOWER_LIMIT2G,
// TX power limit for 5G Radio
    WMI_PDEV_PARAM_TXPOWER_LIMIT5G,
// TX power scale
    WMI_PDEV_PARAM_TXPOWER_SCALE,
// Beacon generation mode . 0: host, 1: target
    WMI_PDEV_PARAM_BEACON_GEN_MODE,
// Beacon generation mode . 0: staggered 1: bursted
    WMI_PDEV_PARAM_BEACON_TX_MODE,
//
// Resource manager off chan mode .
// 0: turn off offchan mode. 1: turn on offchan mode
//
    WMI_PDEV_PARAM_RESMGR_OFFCHAN_MODE,
//
// Protection mode:
// 0: no protection 1:use CTS-to-self 2: use RTS/CTS
//
    WMI_PDEV_PARAM_PROTECTION_MODE,
//
// Dynamic bandwidth - 0: disable, 1: enable
//
// When enabled HW rate control tries different bandwidths when
// retransmitting frames.
//
    WMI_PDEV_PARAM_DYNAMIC_BW,
// Non aggregate/ 11g sw retry threshold.0-disable
    WMI_PDEV_PARAM_NON_AGG_SW_RETRY_TH,
// aggregate sw retry threshold. 0-disable
    WMI_PDEV_PARAM_AGG_SW_RETRY_TH,
// Station kickout threshold (non of consecutive failures).0-disable
    WMI_PDEV_PARAM_STA_KICKOUT_TH,
// Aggerate size scaling configuration per AC
    WMI_PDEV_PARAM_AC_AGGRSIZE_SCALING,
// LTR enable
    WMI_PDEV_PARAM_LTR_ENABLE,
// LTR latency for BE, in us
    WMI_PDEV_PARAM_LTR_AC_LATENCY_BE,
// LTR latency for BK, in us
    WMI_PDEV_PARAM_LTR_AC_LATENCY_BK,
// LTR latency for VI, in us
    WMI_PDEV_PARAM_LTR_AC_LATENCY_VI,
// LTR latency for VO, in us
    WMI_PDEV_PARAM_LTR_AC_LATENCY_VO,
// LTR AC latency timeout, in ms
    WMI_PDEV_PARAM_LTR_AC_LATENCY_TIMEOUT,
// LTR platform latency override, in us
    WMI_PDEV_PARAM_LTR_SLEEP_OVERRIDE,
// LTR-RX override, in us
    WMI_PDEV_PARAM_LTR_RX_OVERRIDE,
// Tx activity timeout for LTR, in us
    WMI_PDEV_PARAM_LTR_TX_ACTIVITY_TIMEOUT,
// L1SS state machine enable
    WMI_PDEV_PARAM_L1SS_ENABLE,
// Deep sleep state machine enable
    WMI_PDEV_PARAM_DSLEEP_ENABLE,
// RX buffering flush enable
    WMI_PDEV_PARAM_PCIELP_TXBUF_FLUSH,
// RX buffering matermark
    WMI_PDEV_PARAM_PCIELP_TXBUF_WATERMARK,
// RX buffering timeout enable
    WMI_PDEV_PARAM_PCIELP_TXBUF_TMO_EN,
// RX buffering timeout value
    WMI_PDEV_PARAM_PCIELP_TXBUF_TMO_VALUE,
// pdev level stats update period in ms
    WMI_PDEV_PARAM_PDEV_STATS_UPDATE_PERIOD,
// vdev level stats update period in ms
    WMI_PDEV_PARAM_VDEV_STATS_UPDATE_PERIOD,
// peer level stats update period in ms
    WMI_PDEV_PARAM_PEER_STATS_UPDATE_PERIOD,
// beacon filter status update period
    WMI_PDEV_PARAM_BCNFLT_STATS_UPDATE_PERIOD,
// QOS Mgmt frame protection MFP/PMF 0: disable, 1: enable
    WMI_PDEV_PARAM_PMF_QOS,
// Access category on which ARP frames are sent
    WMI_PDEV_PARAM_ARP_AC_OVERRIDE,
// DCS configuration
    WMI_PDEV_PARAM_DCS,
// Enable/Disable ANI on target
    WMI_PDEV_PARAM_ANI_ENABLE,
// configure the ANI polling period
    WMI_PDEV_PARAM_ANI_POLL_PERIOD,
// configure the ANI listening period
    WMI_PDEV_PARAM_ANI_LISTEN_PERIOD,
// configure OFDM immunity level
    WMI_PDEV_PARAM_ANI_OFDM_LEVEL,
// configure CCK immunity level
    WMI_PDEV_PARAM_ANI_CCK_LEVEL,
// Enable/Disable CDD for 1x1 STAs in rate control module
    WMI_PDEV_PARAM_DYNTXCHAIN,
// Enable/Disable proxy STA
    WMI_PDEV_PARAM_PROXY_STA,
// Enable/Disable low power state when all VDEVs are inactive/idle.
    WMI_PDEV_PARAM_IDLE_PS_CONFIG,
// Enable/Disable power gating sleep
    WMI_PDEV_PARAM_POWER_GATING_SLEEP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10x_pdev_param {
// TX chian mask
    WMI_10X_PDEV_PARAM_TX_CHAIN_MASK = 0x1,
// RX chian mask
    WMI_10X_PDEV_PARAM_RX_CHAIN_MASK,
// TX power limit for 2G Radio
    WMI_10X_PDEV_PARAM_TXPOWER_LIMIT2G,
// TX power limit for 5G Radio
    WMI_10X_PDEV_PARAM_TXPOWER_LIMIT5G,
// TX power scale
    WMI_10X_PDEV_PARAM_TXPOWER_SCALE,
// Beacon generation mode . 0: host, 1: target
    WMI_10X_PDEV_PARAM_BEACON_GEN_MODE,
// Beacon generation mode . 0: staggered 1: bursted
    WMI_10X_PDEV_PARAM_BEACON_TX_MODE,
//
// Resource manager off chan mode .
// 0: turn off offchan mode. 1: turn on offchan mode
//
    WMI_10X_PDEV_PARAM_RESMGR_OFFCHAN_MODE,
//
// Protection mode:
// 0: no protection 1:use CTS-to-self 2: use RTS/CTS
//
    WMI_10X_PDEV_PARAM_PROTECTION_MODE,
// Dynamic bandwidth 0: disable 1: enable
    WMI_10X_PDEV_PARAM_DYNAMIC_BW,
// Non aggregate/ 11g sw retry threshold.0-disable
    WMI_10X_PDEV_PARAM_NON_AGG_SW_RETRY_TH,
// aggregate sw retry threshold. 0-disable
    WMI_10X_PDEV_PARAM_AGG_SW_RETRY_TH,
// Station kickout threshold (non of consecutive failures).0-disable
    WMI_10X_PDEV_PARAM_STA_KICKOUT_TH,
// Aggerate size scaling configuration per AC
    WMI_10X_PDEV_PARAM_AC_AGGRSIZE_SCALING,
// LTR enable
    WMI_10X_PDEV_PARAM_LTR_ENABLE,
// LTR latency for BE, in us
    WMI_10X_PDEV_PARAM_LTR_AC_LATENCY_BE,
// LTR latency for BK, in us
    WMI_10X_PDEV_PARAM_LTR_AC_LATENCY_BK,
// LTR latency for VI, in us
    WMI_10X_PDEV_PARAM_LTR_AC_LATENCY_VI,
// LTR latency for VO, in us
    WMI_10X_PDEV_PARAM_LTR_AC_LATENCY_VO,
// LTR AC latency timeout, in ms
    WMI_10X_PDEV_PARAM_LTR_AC_LATENCY_TIMEOUT,
// LTR platform latency override, in us
    WMI_10X_PDEV_PARAM_LTR_SLEEP_OVERRIDE,
// LTR-RX override, in us
    WMI_10X_PDEV_PARAM_LTR_RX_OVERRIDE,
// Tx activity timeout for LTR, in us
    WMI_10X_PDEV_PARAM_LTR_TX_ACTIVITY_TIMEOUT,
// L1SS state machine enable
    WMI_10X_PDEV_PARAM_L1SS_ENABLE,
// Deep sleep state machine enable
    WMI_10X_PDEV_PARAM_DSLEEP_ENABLE,
// pdev level stats update period in ms
    WMI_10X_PDEV_PARAM_PDEV_STATS_UPDATE_PERIOD,
// vdev level stats update period in ms
    WMI_10X_PDEV_PARAM_VDEV_STATS_UPDATE_PERIOD,
// peer level stats update period in ms
    WMI_10X_PDEV_PARAM_PEER_STATS_UPDATE_PERIOD,
// beacon filter status update period
    WMI_10X_PDEV_PARAM_BCNFLT_STATS_UPDATE_PERIOD,
// QOS Mgmt frame protection MFP/PMF 0: disable, 1: enable
    WMI_10X_PDEV_PARAM_PMF_QOS,
// Access category on which ARP and DHCP frames are sent
    WMI_10X_PDEV_PARAM_ARPDHCP_AC_OVERRIDE,
// DCS configuration
    WMI_10X_PDEV_PARAM_DCS,
// Enable/Disable ANI on target
    WMI_10X_PDEV_PARAM_ANI_ENABLE,
// configure the ANI polling period
    WMI_10X_PDEV_PARAM_ANI_POLL_PERIOD,
// configure the ANI listening period
    WMI_10X_PDEV_PARAM_ANI_LISTEN_PERIOD,
// configure OFDM immunity level
    WMI_10X_PDEV_PARAM_ANI_OFDM_LEVEL,
// configure CCK immunity level
    WMI_10X_PDEV_PARAM_ANI_CCK_LEVEL,
// Enable/Disable CDD for 1x1 STAs in rate control module
    WMI_10X_PDEV_PARAM_DYNTXCHAIN,
// Enable/Disable Fast channel reset
    WMI_10X_PDEV_PARAM_FAST_CHANNEL_RESET,
// Set Bursting DUR
    WMI_10X_PDEV_PARAM_BURST_DUR,
// Set Bursting Enable
    WMI_10X_PDEV_PARAM_BURST_ENABLE,

// following are available as of firmware 10.2
    WMI_10X_PDEV_PARAM_SMART_ANTENNA_DEFAULT_ANTENNA,
    WMI_10X_PDEV_PARAM_IGMPMLD_OVERRIDE,
    WMI_10X_PDEV_PARAM_IGMPMLD_TID,
    WMI_10X_PDEV_PARAM_ANTENNA_GAIN,
    WMI_10X_PDEV_PARAM_RX_DECAP_MODE,
    WMI_10X_PDEV_PARAM_RX_FILTER,
    WMI_10X_PDEV_PARAM_SET_MCAST_TO_UCAST_TID,
    WMI_10X_PDEV_PARAM_PROXY_STA_MODE,
    WMI_10X_PDEV_PARAM_SET_MCAST2UCAST_MODE,
    WMI_10X_PDEV_PARAM_SET_MCAST2UCAST_BUFFER,
    WMI_10X_PDEV_PARAM_REMOVE_MCAST2UCAST_BUFFER,
    WMI_10X_PDEV_PARAM_PEER_STA_PS_STATECHG_ENABLE,
    WMI_10X_PDEV_PARAM_RTS_FIXED_RATE,
    WMI_10X_PDEV_PARAM_CAL_PERIOD,
    WMI_10X_PDEV_PARAM_ATF_STRICT_SCH,
    WMI_10X_PDEV_PARAM_ATF_SCHED_DURATION,
    WMI_10X_PDEV_PARAM_SET_PROMISC_MODE_CMDID,
    WMI_10X_PDEV_PARAM_PDEV_RESET
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_4_pdev_param {
    WMI_10_4_PDEV_PARAM_TX_CHAIN_MASK = 0x1,
    WMI_10_4_PDEV_PARAM_RX_CHAIN_MASK,
    WMI_10_4_PDEV_PARAM_TXPOWER_LIMIT2G,
    WMI_10_4_PDEV_PARAM_TXPOWER_LIMIT5G,
    WMI_10_4_PDEV_PARAM_TXPOWER_SCALE,
    WMI_10_4_PDEV_PARAM_BEACON_GEN_MODE,
    WMI_10_4_PDEV_PARAM_BEACON_TX_MODE,
    WMI_10_4_PDEV_PARAM_RESMGR_OFFCHAN_MODE,
    WMI_10_4_PDEV_PARAM_PROTECTION_MODE,
    WMI_10_4_PDEV_PARAM_DYNAMIC_BW,
    WMI_10_4_PDEV_PARAM_NON_AGG_SW_RETRY_TH,
    WMI_10_4_PDEV_PARAM_AGG_SW_RETRY_TH,
    WMI_10_4_PDEV_PARAM_STA_KICKOUT_TH,
    WMI_10_4_PDEV_PARAM_AC_AGGRSIZE_SCALING,
    WMI_10_4_PDEV_PARAM_LTR_ENABLE,
    WMI_10_4_PDEV_PARAM_LTR_AC_LATENCY_BE,
    WMI_10_4_PDEV_PARAM_LTR_AC_LATENCY_BK,
    WMI_10_4_PDEV_PARAM_LTR_AC_LATENCY_VI,
    WMI_10_4_PDEV_PARAM_LTR_AC_LATENCY_VO,
    WMI_10_4_PDEV_PARAM_LTR_AC_LATENCY_TIMEOUT,
    WMI_10_4_PDEV_PARAM_LTR_SLEEP_OVERRIDE,
    WMI_10_4_PDEV_PARAM_LTR_RX_OVERRIDE,
    WMI_10_4_PDEV_PARAM_LTR_TX_ACTIVITY_TIMEOUT,
    WMI_10_4_PDEV_PARAM_L1SS_ENABLE,
    WMI_10_4_PDEV_PARAM_DSLEEP_ENABLE,
    WMI_10_4_PDEV_PARAM_PCIELP_TXBUF_FLUSH,
    WMI_10_4_PDEV_PARAM_PCIELP_TXBUF_WATERMARK,
    WMI_10_4_PDEV_PARAM_PCIELP_TXBUF_TMO_EN,
    WMI_10_4_PDEV_PARAM_PCIELP_TXBUF_TMO_VALUE,
    WMI_10_4_PDEV_PARAM_PDEV_STATS_UPDATE_PERIOD,
    WMI_10_4_PDEV_PARAM_VDEV_STATS_UPDATE_PERIOD,
    WMI_10_4_PDEV_PARAM_PEER_STATS_UPDATE_PERIOD,
    WMI_10_4_PDEV_PARAM_BCNFLT_STATS_UPDATE_PERIOD,
    WMI_10_4_PDEV_PARAM_PMF_QOS,
    WMI_10_4_PDEV_PARAM_ARP_AC_OVERRIDE,
    WMI_10_4_PDEV_PARAM_DCS,
    WMI_10_4_PDEV_PARAM_ANI_ENABLE,
    WMI_10_4_PDEV_PARAM_ANI_POLL_PERIOD,
    WMI_10_4_PDEV_PARAM_ANI_LISTEN_PERIOD,
    WMI_10_4_PDEV_PARAM_ANI_OFDM_LEVEL,
    WMI_10_4_PDEV_PARAM_ANI_CCK_LEVEL,
    WMI_10_4_PDEV_PARAM_DYNTXCHAIN,
    WMI_10_4_PDEV_PARAM_PROXY_STA,
    WMI_10_4_PDEV_PARAM_IDLE_PS_CONFIG,
    WMI_10_4_PDEV_PARAM_POWER_GATING_SLEEP,
    WMI_10_4_PDEV_PARAM_AGGR_BURST,
    WMI_10_4_PDEV_PARAM_RX_DECAP_MODE,
    WMI_10_4_PDEV_PARAM_FAST_CHANNEL_RESET,
    WMI_10_4_PDEV_PARAM_BURST_DUR,
    WMI_10_4_PDEV_PARAM_BURST_ENABLE,
    WMI_10_4_PDEV_PARAM_SMART_ANTENNA_DEFAULT_ANTENNA,
    WMI_10_4_PDEV_PARAM_IGMPMLD_OVERRIDE,
    WMI_10_4_PDEV_PARAM_IGMPMLD_TID,
    WMI_10_4_PDEV_PARAM_ANTENNA_GAIN,
    WMI_10_4_PDEV_PARAM_RX_FILTER,
    WMI_10_4_PDEV_SET_MCAST_TO_UCAST_TID,
    WMI_10_4_PDEV_PARAM_PROXY_STA_MODE,
    WMI_10_4_PDEV_PARAM_SET_MCAST2UCAST_MODE,
    WMI_10_4_PDEV_PARAM_SET_MCAST2UCAST_BUFFER,
    WMI_10_4_PDEV_PARAM_REMOVE_MCAST2UCAST_BUFFER,
    WMI_10_4_PDEV_PEER_STA_PS_STATECHG_ENABLE,
    WMI_10_4_PDEV_PARAM_IGMPMLD_AC_OVERRIDE,
    WMI_10_4_PDEV_PARAM_BLOCK_INTERBSS,
    WMI_10_4_PDEV_PARAM_SET_DISABLE_RESET_CMDID,
    WMI_10_4_PDEV_PARAM_SET_MSDU_TTL_CMDID,
    WMI_10_4_PDEV_PARAM_SET_PPDU_DURATION_CMDID,
    WMI_10_4_PDEV_PARAM_TXBF_SOUND_PERIOD_CMDID,
    WMI_10_4_PDEV_PARAM_SET_PROMISC_MODE_CMDID,
    WMI_10_4_PDEV_PARAM_SET_BURST_MODE_CMDID,
    WMI_10_4_PDEV_PARAM_EN_STATS,
    WMI_10_4_PDEV_PARAM_MU_GROUP_POLICY,
    WMI_10_4_PDEV_PARAM_NOISE_DETECTION,
    WMI_10_4_PDEV_PARAM_NOISE_THRESHOLD,
    WMI_10_4_PDEV_PARAM_DPD_ENABLE,
    WMI_10_4_PDEV_PARAM_SET_MCAST_BCAST_ECHO,
    WMI_10_4_PDEV_PARAM_ATF_STRICT_SCH,
    WMI_10_4_PDEV_PARAM_ATF_SCHED_DURATION,
    WMI_10_4_PDEV_PARAM_ANT_PLZN,
    WMI_10_4_PDEV_PARAM_MGMT_RETRY_LIMIT,
    WMI_10_4_PDEV_PARAM_SENSITIVITY_LEVEL,
    WMI_10_4_PDEV_PARAM_SIGNED_TXPOWER_2G,
    WMI_10_4_PDEV_PARAM_SIGNED_TXPOWER_5G,
    WMI_10_4_PDEV_PARAM_ENABLE_PER_TID_AMSDU,
    WMI_10_4_PDEV_PARAM_ENABLE_PER_TID_AMPDU,
    WMI_10_4_PDEV_PARAM_CCA_THRESHOLD,
    WMI_10_4_PDEV_PARAM_RTS_FIXED_RATE,
    WMI_10_4_PDEV_PARAM_CAL_PERIOD,
    WMI_10_4_PDEV_PARAM_PDEV_RESET,
    WMI_10_4_PDEV_PARAM_WAPI_MBSSID_OFFSET,
    WMI_10_4_PDEV_PARAM_ARP_SRCADDR,
    WMI_10_4_PDEV_PARAM_ARP_DSTADDR,
    WMI_10_4_PDEV_PARAM_TXPOWER_DECR_DB,
    WMI_10_4_PDEV_PARAM_RX_BATCHMODE,
    WMI_10_4_PDEV_PARAM_PACKET_AGGR_DELAY,
    WMI_10_4_PDEV_PARAM_ATF_OBSS_NOISE_SCH,
    WMI_10_4_PDEV_PARAM_ATF_OBSS_NOISE_SCALING_FACTOR,
    WMI_10_4_PDEV_PARAM_CUST_TXPOWER_SCALE,
    WMI_10_4_PDEV_PARAM_ATF_DYNAMIC_ENABLE,
    WMI_10_4_PDEV_PARAM_ATF_SSID_GROUP_POLICY,
    WMI_10_4_PDEV_PARAM_ENABLE_BTCOEX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_set_param_cmd {
    pub param_id: __le32,
    pub param_value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_set_base_macaddr_cmd {
    pub mac_addr: wmi_mac_addr,
    pub __packed: },
// valid period is 1 ~ 60000ms, unit in millisecond
pub const WMI_PDEV_PARAM_CAL_PERIOD_MAX: c_int = 60000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_get_tpc_config_cmd {
// parameter
    pub param: __le32,
    pub __packed: },
pub const WMI_TPC_CONFIG_PARAM: c_int = 1;
pub const WMI_TPC_FINAL_RATE_MAX: c_int = 240;
pub const WMI_TPC_TX_N_CHAIN: c_int = 4;

pub const WMI_TPC_PREAM_TABLE_MAX: c_int = 10;
pub const WMI_TPC_FLAG: c_int = 3;
pub const WMI_TPC_BUF_SIZE: c_int = 10;
pub const WMI_TPC_BEAMFORMING: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tpc_table_type {
    WMI_TPC_TABLE_TYPE_CDD = 0,
    WMI_TPC_TABLE_TYPE_STBC = 1,
    WMI_TPC_TABLE_TYPE_TXBF = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tpc_config_event_flag {
    WMI_TPC_CONFIG_EVENT_FLAG_TABLE_CDD	= 0x1,
    WMI_TPC_CONFIG_EVENT_FLAG_TABLE_STBC	= 0x2,
    WMI_TPC_CONFIG_EVENT_FLAG_TABLE_TXBF	= 0x4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_tpc_config_event {
    pub reg_domain: __le32,
    pub chan_freq: __le32,
    pub phy_mode: __le32,
    pub twice_antenna_reduction: __le32,
    pub twice_max_rd_power: __le32,
    pub twice_antenna_gain: a_sle32,
    pub power_limit: __le32,
    pub rate_max: __le32,
    pub num_tx_chain: __le32,
    pub ctl: __le32,
    pub flags: __le32,
    pub max_reg_allow_pow: [i8; WMI_TPC_TX_N_CHAIN],
    pub max_reg_allow_pow_agcdd: [i8; WMI_TPC_TX_N_CHAIN][WMI_TPC_TX_N_CHAIN],
    pub max_reg_allow_pow_agstbc: [i8; WMI_TPC_TX_N_CHAIN][WMI_TPC_TX_N_CHAIN],
    pub max_reg_allow_pow_agtxbf: [i8; WMI_TPC_TX_N_CHAIN][WMI_TPC_TX_N_CHAIN],
    pub rates_array: [u8; WMI_TPC_RATE_MAX],
    pub __packed: },
// Transmit power scale factor.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tp_scale {
    WMI_TP_SCALE_MAX    = 0,	/* no scaling (default) */
    WMI_TP_SCALE_50     = 1,	/* 50% of max (-3 dBm) */
    WMI_TP_SCALE_25     = 2,	/* 25% of max (-6 dBm) */
    WMI_TP_SCALE_12     = 3,	/* 12% of max (-9 dBm) */
    WMI_TP_SCALE_MIN    = 4,	/* min, but still on   */
    WMI_TP_SCALE_SIZE   = 5,	/* max num of enum     */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_tpc_final_table_event {
    pub reg_domain: __le32,
    pub chan_freq: __le32,
    pub phy_mode: __le32,
    pub twice_antenna_reduction: __le32,
    pub twice_max_rd_power: __le32,
    pub twice_antenna_gain: a_sle32,
    pub power_limit: __le32,
    pub rate_max: __le32,
    pub num_tx_chain: __le32,
    pub ctl: __le32,
    pub flags: __le32,
    pub max_reg_allow_pow: [i8; WMI_TPC_TX_N_CHAIN],
    pub max_reg_allow_pow_agcdd: [i8; WMI_TPC_TX_N_CHAIN][WMI_TPC_TX_N_CHAIN],
    pub max_reg_allow_pow_agstbc: [i8; WMI_TPC_TX_N_CHAIN][WMI_TPC_TX_N_CHAIN],
    pub max_reg_allow_pow_agtxbf: [i8; WMI_TPC_TX_N_CHAIN][WMI_TPC_TX_N_CHAIN],
    pub rates_array: [u8; WMI_TPC_FINAL_RATE_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_get_tpc_table_cmd {
    pub param: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tpc_pream_2ghz {
    WMI_TPC_PREAM_2GHZ_CCK = 0,
    WMI_TPC_PREAM_2GHZ_OFDM,
    WMI_TPC_PREAM_2GHZ_HT20,
    WMI_TPC_PREAM_2GHZ_HT40,
    WMI_TPC_PREAM_2GHZ_VHT20,
    WMI_TPC_PREAM_2GHZ_VHT40,
    WMI_TPC_PREAM_2GHZ_VHT80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tpc_pream_5ghz {
    WMI_TPC_PREAM_5GHZ_OFDM = 1,
    WMI_TPC_PREAM_5GHZ_HT20,
    WMI_TPC_PREAM_5GHZ_HT40,
    WMI_TPC_PREAM_5GHZ_VHT20,
    WMI_TPC_PREAM_5GHZ_VHT40,
    WMI_TPC_PREAM_5GHZ_VHT80,
    WMI_TPC_PREAM_5GHZ_HTCUP,
}

pub const WMI_PEER_PS_STATE_DISABLED: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_sta_ps_state_chg_event {
    pub peer_macaddr: wmi_mac_addr,
    pub peer_ps_state: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_debug_mesg_event {
// message buffer, NULL terminated
    pub bufp: [c_char; WMI_MAX_DEBUG_MESG],
    pub __packed: },
// P2P device
// P2P client
// P2P GO
// BT3.0 HS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_set_channel_cmd {
// idnore power , only use flags , mode and freq
    pub chan: wmi_channel,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_pktlog_enable_cmd {
    pub ev_bitmap: __le32,
    pub __packed: },
// Customize the DSCP (bit) to TID (0-7) mapping for QOS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_set_dscp_tid_map_cmd {
// map indicating DSCP to TID conversion
    pub dscp_to_tid_map: [__le32; WMI_DSCP_MAP_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcast_bcast_rate_id {
    WMI_SET_MCAST_RATE,
    WMI_SET_BCAST_RATE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcast_bcast_rate {
    pub rate_id: mcast_bcast_rate_id,
    pub rate: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_wmm_params {
    pub cwmin: __le32,
    pub cwmax: __le32,
    pub aifs: __le32,
    pub txop: __le32,
    pub acm: __le32,
    pub no_ack: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_set_wmm_params {
    pub ac_be: wmi_wmm_params,
    pub ac_bk: wmi_wmm_params,
    pub ac_vi: wmi_wmm_params,
    pub ac_vo: wmi_wmm_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_wmm_params_arg {
    pub cwmin: u32,
    pub cwmax: u32,
    pub aifs: u32,
    pub txop: u32,
    pub acm: u32,
    pub no_ack: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_wmm_params_all_arg {
    pub ac_be: wmi_wmm_params_arg,
    pub ac_bk: wmi_wmm_params_arg,
    pub ac_vi: wmi_wmm_params_arg,
    pub ac_vo: wmi_wmm_params_arg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_stats_tx {
// Num HTT cookies queued to dispatch list
    pub comp_queued: __le32,
// Num HTT cookies dispatched
    pub comp_delivered: __le32,
// Num MSDU queued to WAL
    pub msdu_enqued: __le32,
// Num MPDU queue to WAL
    pub mpdu_enqued: __le32,
// Num MSDUs dropped by WMM limit
    pub wmm_drop: __le32,
// Num Local frames queued
    pub local_enqued: __le32,
// Num Local frames done
    pub local_freed: __le32,
// Num queued to HW
    pub hw_queued: __le32,
// Num PPDU reaped from HW
    pub hw_reaped: __le32,
// Num underruns
    pub underrun: __le32,
// Num PPDUs cleaned up in TX abort
    pub tx_abort: __le32,
// Num MPDUs requeued by SW
    pub mpdus_requeued: __le32,
// excessive retries
    pub tx_ko: __le32,
// data hw rate code
    pub data_rc: __le32,
// Scheduler self triggers
    pub self_triggers: __le32,
// frames dropped due to excessive sw retries
    pub sw_retry_failure: __le32,
// illegal rate phy errors
    pub illgl_rate_phy_err: __le32,
// wal pdev continuous xretry
    pub pdev_cont_xretry: __le32,
// wal pdev continuous xretry
    pub pdev_tx_timeout: __le32,
// wal pdev resets
    pub pdev_resets: __le32,
// frames dropped due to non-availability of stateless TIDs
    pub stateless_tid_alloc_failure: __le32,
    pub phy_underrun: __le32,
// MPDU is more than txop limit
    pub txop_ovf: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_pdev_stats_tx {
// Num HTT cookies queued to dispatch list
    pub comp_queued: __le32,
// Num HTT cookies dispatched
    pub comp_delivered: __le32,
// Num MSDU queued to WAL
    pub msdu_enqued: __le32,
// Num MPDU queue to WAL
    pub mpdu_enqued: __le32,
// Num MSDUs dropped by WMM limit
    pub wmm_drop: __le32,
// Num Local frames queued
    pub local_enqued: __le32,
// Num Local frames done
    pub local_freed: __le32,
// Num queued to HW
    pub hw_queued: __le32,
// Num PPDU reaped from HW
    pub hw_reaped: __le32,
// Num underruns
    pub underrun: __le32,
// HW Paused.
    pub hw_paused: __le32,
// Num PPDUs cleaned up in TX abort
    pub tx_abort: __le32,
// Num MPDUs requeued by SW
    pub mpdus_requeued: __le32,
// excessive retries
    pub tx_ko: __le32,
// data hw rate code
    pub data_rc: __le32,
// Scheduler self triggers
    pub self_triggers: __le32,
// frames dropped due to excessive sw retries
    pub sw_retry_failure: __le32,
// illegal rate phy errors
    pub illgl_rate_phy_err: __le32,
// wal pdev continuous xretry
    pub pdev_cont_xretry: __le32,
// wal pdev tx timeouts
    pub pdev_tx_timeout: __le32,
// wal pdev resets
    pub pdev_resets: __le32,
// frames dropped due to non-availability of stateless TIDs
    pub stateless_tid_alloc_failure: __le32,
    pub phy_underrun: __le32,
// MPDU is more than txop limit
    pub txop_ovf: __le32,
// Number of Sequences posted
    pub seq_posted: __le32,
// Number of Sequences failed queueing
    pub seq_failed_queueing: __le32,
// Number of Sequences completed
    pub seq_completed: __le32,
// Number of Sequences restarted
    pub seq_restarted: __le32,
// Number of MU Sequences posted
    pub mu_seq_posted: __le32,
// Num MPDUs flushed by SW, HWPAUSED,SW TXABORT(Reset,channel change)
    pub mpdus_sw_flush: __le32,
// Num MPDUs filtered by HW, all filter condition (TTL expired)
    pub mpdus_hw_filter: __le32,
// Num MPDUs truncated by PDG
// (TXOP, TBTT, PPDU_duration based on rate, dyn_bw)
//
    pub mpdus_truncated: __le32,
// Num MPDUs that was tried but didn't receive ACK or BA
    pub mpdus_ack_failed: __le32,
// Num MPDUs that was dropped due to expiry.
    pub mpdus_expired: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_stats_rx {
// Cnts any change in ring routing mid-ppdu
    pub mid_ppdu_route_change: __le32,
// Total number of statuses processed
    pub status_rcvd: __le32,
// Extra frags on rings 0-3
    pub r0_frags: __le32,
    pub r1_frags: __le32,
    pub r2_frags: __le32,
    pub r3_frags: __le32,
// MSDUs / MPDUs delivered to HTT
    pub htt_msdus: __le32,
    pub htt_mpdus: __le32,
// MSDUs / MPDUs delivered to local stack
    pub loc_msdus: __le32,
    pub loc_mpdus: __le32,
// AMSDUs that have more MSDUs than the status ring size
    pub oversize_amsdu: __le32,
// Number of PHY errors
    pub phy_errs: __le32,
// Number of PHY errors drops
    pub phy_err_drop: __le32,
// Number of mpdu errors - FCS, MIC, ENC etc.
    pub mpdu_errs: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_stats_peer {
// REMOVE THIS ONCE REAL PEER STAT COUNTERS ARE ADDED
    pub dummy: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_stats_id {
    WMI_STAT_PEER = BIT(0),
    WMI_STAT_AP = BIT(1),
    WMI_STAT_PDEV = BIT(2),
    WMI_STAT_VDEV = BIT(3),
    WMI_STAT_BCNFLT = BIT(4),
    WMI_STAT_VDEV_RATE = BIT(5),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_4_stats_id {
    WMI_10_4_STAT_PEER		= BIT(0),
    WMI_10_4_STAT_AP		= BIT(1),
    WMI_10_4_STAT_INST		= BIT(2),
    WMI_10_4_STAT_PEER_EXTD		= BIT(3),
    WMI_10_4_STAT_VDEV_EXTD		= BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tlv_stats_id {
    WMI_TLV_STAT_PEER	= BIT(0),
    WMI_TLV_STAT_AP		= BIT(1),
    WMI_TLV_STAT_PDEV	= BIT(2),
    WMI_TLV_STAT_VDEV	= BIT(3),
    WMI_TLV_STAT_PEER_EXTD  = BIT(10),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlan_inst_rssi_args {
    pub cfg_retry_count: __le16,
    pub retry_count: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_request_stats_cmd {
    pub stats_id: __le32,
    pub vdev_id: __le32,
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
// Instantaneous RSSI arguments
    pub inst_rssi_args: wlan_inst_rssi_args,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_peer_stats_info_request_type {
// request stats of one specified peer
    WMI_REQUEST_ONE_PEER_STATS_INFO = 0x01,
// request stats of all peers belong to specified VDEV
    WMI_REQUEST_VDEV_ALL_PEER_STATS_INFO = 0x02,
}

// Suspend option
// suspend
// suspend and disable all interrupts
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_suspend_cmd {
// suspend option sent to target
    pub suspend_opt: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_stats_event {
    pub /: *mut *mut __le32 stats_id; / WMI_STAT_,
//
// number of pdev stats event structures
// (wmi_pdev_stats) 0 or 1
//
    pub num_pdev_stats: __le32,
//
// number of vdev stats event structures
// (wmi_vdev_stats) 0 or max vdevs
//
    pub num_vdev_stats: __le32,
//
// number of peer stats event structures
// (wmi_peer_stats) 0 or max peers
//
    pub num_peer_stats: __le32,
    pub num_bcnflt_stats: __le32,
//
// followed by
// num_pdev_stats * size of(struct wmi_pdev_stats)
// num_vdev_stats * size of(struct wmi_vdev_stats)
// num_peer_stats * size of(struct wmi_peer_stats)
//
// By having a zero sized array, the pointer to data area
// becomes available without increasing the struct size
//
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_2_stats_event {
    pub /: *mut *mut __le32 stats_id; / %WMI_REQUEST_,
    pub num_pdev_stats: __le32,
    pub num_pdev_ext_stats: __le32,
    pub num_vdev_stats: __le32,
    pub num_peer_stats: __le32,
    pub num_bcnflt_stats: __le32,
    pub data: [u8; ],
    pub __packed: },
//
// PDEV statistics
// TODO: add all PDEV stats here
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_stats_base {
    pub chan_nf: __le32,
    pub /: *mut *mut __le32 tx_frame_count; / Cycles spent transmitting frames,
    pub /: *mut *mut __le32 rx_frame_count; / Cycles spent receiving frames,
    pub /: *mut *mut __le32 rx_clear_count; / Total channel busy time, evidently,
    pub /: *mut *mut __le32 cycle_count; / Total on-channel time,
    pub phy_err_count: __le32,
    pub chan_tx_pwr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_stats {
    pub base: wmi_pdev_stats_base,
    pub tx: wmi_pdev_stats_tx,
    pub rx: wmi_pdev_stats_rx,
    pub peer: wmi_pdev_stats_peer,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_stats_extra {
    pub ack_rx_bad: __le32,
    pub rts_bad: __le32,
    pub rts_good: __le32,
    pub fcs_bad: __le32,
    pub no_beacons: __le32,
    pub mib_int_count: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10x_pdev_stats {
    pub base: wmi_pdev_stats_base,
    pub tx: wmi_pdev_stats_tx,
    pub rx: wmi_pdev_stats_rx,
    pub peer: wmi_pdev_stats_peer,
    pub extra: wmi_pdev_stats_extra,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_stats_mem {
    pub dram_free: __le32,
    pub iram_free: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_2_pdev_stats {
    pub base: wmi_pdev_stats_base,
    pub tx: wmi_pdev_stats_tx,
    pub mc_drop: __le32,
    pub rx: wmi_pdev_stats_rx,
    pub pdev_rx_timeout: __le32,
    pub mem: wmi_pdev_stats_mem,
    pub peer: wmi_pdev_stats_peer,
    pub extra: wmi_pdev_stats_extra,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_pdev_stats {
    pub base: wmi_pdev_stats_base,
    pub tx: wmi_10_4_pdev_stats_tx,
    pub rx: wmi_pdev_stats_rx,
    pub rx_ovfl_errs: __le32,
    pub mem: wmi_pdev_stats_mem,
    pub sram_free_size: __le32,
    pub extra: wmi_pdev_stats_extra,
    pub __packed: },
//
// VDEV statistics
//

pub const WMI_VDEV_STATS_FTM_COUNT_LSB: c_int = 0;
pub const WMI_VDEV_STATS_FTM_COUNT_MASK: c_uint = 0x7fffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_stats {
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_stats_extd {
    pub vdev_id: __le32,
    pub ppdu_aggr_cnt: __le32,
    pub ppdu_noack: __le32,
    pub mpdu_queued: __le32,
    pub ppdu_nonaggr_cnt: __le32,
    pub mpdu_sw_requeued: __le32,
    pub mpdu_suc_retry: __le32,
    pub mpdu_suc_multitry: __le32,
    pub mpdu_fail_retry: __le32,
    pub tx_ftm_suc: __le32,
    pub tx_ftm_suc_retry: __le32,
    pub tx_ftm_fail: __le32,
    pub rx_ftmr_cnt: __le32,
    pub rx_ftmr_dup_cnt: __le32,
    pub rx_iftmr_cnt: __le32,
    pub rx_iftmr_dup_cnt: __le32,
    pub reserved: [__le32; 6],
    pub __packed: },
//
// peer statistics.
// TODO: add more stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_stats {
    pub peer_macaddr: wmi_mac_addr,
    pub peer_rssi: __le32,
    pub peer_tx_rate: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10x_peer_stats {
    pub old: wmi_peer_stats,
    pub peer_rx_rate: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_2_peer_stats {
    pub old: wmi_peer_stats,
    pub peer_rx_rate: __le32,
    pub current_per: __le32,
    pub retries: __le32,
    pub tx_rate_count: __le32,
    pub max_4ms_frame_len: __le32,
    pub total_sub_frames: __le32,
    pub tx_bytes: __le32,
    pub num_pkt_loss_overflow: [__le32; 4],
    pub num_pkt_loss_excess_retry: [__le32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_2_4_peer_stats {
    pub common: wmi_10_2_peer_stats,
    pub peer_rssi_changed: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_2_4_ext_peer_stats {
    pub common: wmi_10_2_peer_stats,
    pub peer_rssi_changed: __le32,
    pub rx_duration: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_peer_stats {
    pub peer_macaddr: wmi_mac_addr,
    pub peer_rssi: __le32,
    pub peer_rssi_seq_num: __le32,
    pub peer_tx_rate: __le32,
    pub peer_rx_rate: __le32,
    pub current_per: __le32,
    pub retries: __le32,
    pub tx_rate_count: __le32,
    pub max_4ms_frame_len: __le32,
    pub total_sub_frames: __le32,
    pub tx_bytes: __le32,
    pub num_pkt_loss_overflow: [__le32; 4],
    pub num_pkt_loss_excess_retry: [__le32; 4],
    pub peer_rssi_changed: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_peer_extd_stats {
    pub peer_macaddr: wmi_mac_addr,
    pub inactive_time: __le32,
    pub peer_chain_rssi: __le32,
    pub rx_duration: __le32,
    pub reserved: [__le32; 10],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_bss_bcn_stats {
    pub vdev_id: __le32,
    pub bss_bcns_dropped: __le32,
    pub bss_bcn_delivered: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_bss_bcn_filter_stats {
    pub bcns_dropped: __le32,
    pub bcns_delivered: __le32,
    pub active_filters: __le32,
    pub bss_stats: wmi_10_4_bss_bcn_stats,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_2_pdev_ext_stats {
    pub rx_rssi_comb: __le32,
    pub rx_rssi: [__le32; 4],
    pub rx_mcs: [__le32; 10],
    pub tx_mcs: [__le32; 10],
    pub ack_rssi: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_create_cmd {
    pub vdev_id: __le32,
    pub vdev_type: __le32,
    pub vdev_subtype: __le32,
    pub vdev_macaddr: wmi_mac_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vdev_type {
    WMI_VDEV_TYPE_AP      = 1,
    WMI_VDEV_TYPE_STA     = 2,
    WMI_VDEV_TYPE_IBSS    = 3,
    WMI_VDEV_TYPE_MONITOR = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vdev_subtype {
    WMI_VDEV_SUBTYPE_NONE,
    WMI_VDEV_SUBTYPE_P2P_DEVICE,
    WMI_VDEV_SUBTYPE_P2P_CLIENT,
    WMI_VDEV_SUBTYPE_P2P_GO,
    WMI_VDEV_SUBTYPE_PROXY_STA,
    WMI_VDEV_SUBTYPE_MESH_11S,
    WMI_VDEV_SUBTYPE_MESH_NON_11S,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vdev_subtype_legacy {
    WMI_VDEV_SUBTYPE_LEGACY_NONE      = 0,
    WMI_VDEV_SUBTYPE_LEGACY_P2P_DEV   = 1,
    WMI_VDEV_SUBTYPE_LEGACY_P2P_CLI   = 2,
    WMI_VDEV_SUBTYPE_LEGACY_P2P_GO    = 3,
    WMI_VDEV_SUBTYPE_LEGACY_PROXY_STA = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vdev_subtype_10_2_4 {
    WMI_VDEV_SUBTYPE_10_2_4_NONE      = 0,
    WMI_VDEV_SUBTYPE_10_2_4_P2P_DEV   = 1,
    WMI_VDEV_SUBTYPE_10_2_4_P2P_CLI   = 2,
    WMI_VDEV_SUBTYPE_10_2_4_P2P_GO    = 3,
    WMI_VDEV_SUBTYPE_10_2_4_PROXY_STA = 4,
    WMI_VDEV_SUBTYPE_10_2_4_MESH_11S  = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vdev_subtype_10_4 {
    WMI_VDEV_SUBTYPE_10_4_NONE         = 0,
    WMI_VDEV_SUBTYPE_10_4_P2P_DEV      = 1,
    WMI_VDEV_SUBTYPE_10_4_P2P_CLI      = 2,
    WMI_VDEV_SUBTYPE_10_4_P2P_GO       = 3,
    WMI_VDEV_SUBTYPE_10_4_PROXY_STA    = 4,
    WMI_VDEV_SUBTYPE_10_4_MESH_NON_11S = 5,
    WMI_VDEV_SUBTYPE_10_4_MESH_11S     = 6,
}

// values for vdev_subtype
// values for vdev_start_request flags
//
// Indicates that AP VDEV uses hidden ssid. only valid for
// AP/GO
//

//
// Indicates if robust management frame/management frame
// protection is enabled. For GO/AP vdevs, it indicates that
// it may support station/client associations with RMF enabled.
// For STA/client vdevs, it indicates that sta will
// associate with AP with RMF enabled.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_noa_descriptor {
    pub /: *mut *mut __le32 type_count; / 255: continuous schedule, 0: reserved,
    pub /: *mut *mut __le32 duration; / Absent period duration in micro seconds,
    pub /: *mut *mut __le32 interval; / Absent period interval in micro seconds,
    pub /: *mut *mut __le32 start_time; / 32 bit tsf time when in starts,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_start_request_cmd {
// WMI channel
    pub chan: wmi_channel,
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
// requestor id identifying the caller module
    pub requestor_id: __le32,
// beacon interval from received beacon
    pub beacon_interval: __le32,
// DTIM Period from the received beacon
    pub dtim_period: __le32,
// Flags
    pub flags: __le32,
// ssid field. Only valid for AP/GO/IBSS/BTAmp VDEV type.
    pub ssid: wmi_ssid,
// beacon/probe response xmit rate. Applicable for SoftAP.
    pub bcn_tx_rate: __le32,
// beacon/probe response xmit power. Applicable for SoftAP.
    pub bcn_tx_power: __le32,
// number of p2p NOA descriptor(s) from scan entry
    pub num_noa_descriptors: __le32,
//
// Disable H/W ack. This used by WMI_VDEV_RESTART_REQUEST_CMDID.
// During CAC, Our HW shouldn't ack ditected frames
//
    pub disable_hw_ack: __le32,
// actual p2p NOA descriptor from scan entry
    pub noa_descriptors: [wmi_p2p_noa_descriptor; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_restart_request_cmd {
    pub vdev_start_request_cmd: wmi_vdev_start_request_cmd,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_start_request_arg {
    pub vdev_id: u32,
    pub channel: wmi_channel_arg,
    pub bcn_intval: u32,
    pub dtim_period: u32,
    pub ssid: *mut u8,
    pub ssid_len: u32,
    pub bcn_tx_rate: u32,
    pub bcn_tx_power: u32,
    pub disable_hw_ack: bool,
    pub hidden_ssid: bool,
    pub pmf_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_delete_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_up_cmd {
    pub vdev_id: __le32,
    pub vdev_assoc_id: __le32,
    pub vdev_bssid: wmi_mac_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_stop_cmd {
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_down_cmd {
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_standby_response_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_resume_response_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_set_param_cmd {
    pub vdev_id: __le32,
    pub param_id: __le32,
    pub param_value: __le32,
    pub __packed: },
pub const WMI_MAX_KEY_INDEX: c_int = 3;
pub const WMI_MAX_KEY_LEN: c_int = 32;
pub const WMI_KEY_PAIRWISE: c_uint = 0x00;
pub const WMI_KEY_GROUP: c_uint = 0x01;
pub const WMI_KEY_TX_USAGE: c_uint = 0x02 /* default tx key - static wep */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_key_seq_counter {
    pub key_seq_counter_l: __le32,
    pub key_seq_counter_h: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cipher_suites {
    WMI_CIPHER_NONE,
    WMI_CIPHER_WEP,
    WMI_CIPHER_TKIP,
    WMI_CIPHER_AES_OCB,
    WMI_CIPHER_AES_CCM,
    WMI_CIPHER_WAPI,
    WMI_CIPHER_CKIP,
    WMI_CIPHER_AES_CMAC,
    WMI_CIPHER_AES_GCM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tlv_cipher_suites {
    WMI_TLV_CIPHER_NONE,
    WMI_TLV_CIPHER_WEP,
    WMI_TLV_CIPHER_TKIP,
    WMI_TLV_CIPHER_AES_OCB,
    WMI_TLV_CIPHER_AES_CCM,
    WMI_TLV_CIPHER_WAPI,
    WMI_TLV_CIPHER_CKIP,
    WMI_TLV_CIPHER_AES_CMAC,
    WMI_TLV_CIPHER_ANY,
    WMI_TLV_CIPHER_AES_GCM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_install_key_cmd {
    pub vdev_id: __le32,
    pub peer_macaddr: wmi_mac_addr,
    pub key_idx: __le32,
    pub key_flags: __le32,
    pub /: *mut *mut __le32 key_cipher; / %WMI_CIPHER_,
    pub key_rsc_counter: wmi_key_seq_counter,
    pub key_global_rsc_counter: wmi_key_seq_counter,
    pub key_tsc_counter: wmi_key_seq_counter,
    pub wpi_key_rsc_counter: [u8; 16],
    pub wpi_key_tsc_counter: [u8; 16],
    pub key_len: __le32,
    pub key_txmic_len: __le32,
    pub key_rxmic_len: __le32,
// contains key followed by tx mic followed by rx mic
    pub key_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_install_key_arg {
    pub vdev_id: u32,
    pub macaddr: *const u8,
    pub key_idx: u32,
    pub key_flags: u32,
    pub key_cipher: u32,
    pub key_len: u32,
    pub key_txmic_len: u32,
    pub key_rxmic_len: u32,
    pub key_data: *const c_void,
}

//
// vdev fixed rate format:
// - preamble - b7:b6 - see WMI_RATE_PREMABLE_
// - nss      - b5:b4 - ss number (0 mean 1ss)
// - rate_mcs - b3:b0 - as below
// CCK:  0 - 11Mbps, 1 - 5,5Mbps, 2 - 2Mbps, 3 - 1Mbps,
// 4 - 11Mbps (s), 5 - 5,5Mbps (s), 6 - 2Mbps (s)
// OFDM: 0 - 48Mbps, 1 - 24Mbps, 2 - 12Mbps, 3 - 6Mbps,
// 4 - 54Mbps, 5 - 36Mbps, 6 - 18Mbps, 7 - 9Mbps
// HT/VHT: MCS index
//
// Preamble types to be used with VDEV fixed rate configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_rate_preamble {
    WMI_RATE_PREAMBLE_OFDM,
    WMI_RATE_PREAMBLE_CCK,
    WMI_RATE_PREAMBLE_HT,
    WMI_RATE_PREAMBLE_VHT,
}

pub const ATH10K_VHT_MCS_NUM: c_int = 10;
pub const ATH10K_BW_NUM: c_int = 6;
pub const ATH10K_NSS_NUM: c_int = 4;
pub const ATH10K_LEGACY_NUM: c_int = 12;
pub const ATH10K_GI_NUM: c_int = 2;
pub const ATH10K_HT_MCS_NUM: c_int = 32;
pub const ATH10K_RATE_TABLE_NUM: c_int = 320;
pub const ATH10K_RATE_INFO_FLAGS_SGI_BIT: c_int = 2;
// Value to disable fixed rate setting

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_param_map {
    pub smps_state: u32,
    pub ampdu: u32,
    pub authorize: u32,
    pub chan_width: u32,
    pub nss: u32,
    pub use_4addr: u32,
    pub membership: u32,
    pub use_fixed_power: u32,
    pub user_pos: u32,
    pub crit_proto_hint_enabled: u32,
    pub tx_fail_cnt_thr: u32,
    pub set_hw_retry_cts2s: u32,
    pub ibss_atim_win_len: u32,
    pub debug: u32,
    pub phymode: u32,
    pub dummy_var: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_param_map {
    pub rts_threshold: u32,
    pub fragmentation_threshold: u32,
    pub beacon_interval: u32,
    pub listen_interval: u32,
    pub multicast_rate: u32,
    pub mgmt_tx_rate: u32,
    pub slot_time: u32,
    pub preamble: u32,
    pub swba_time: u32,
    pub wmi_vdev_stats_update_period: u32,
    pub wmi_vdev_pwrsave_ageout_time: u32,
    pub wmi_vdev_host_swba_interval: u32,
    pub dtim_period: u32,
    pub wmi_vdev_oc_scheduler_air_time_limit: u32,
    pub wds: u32,
    pub atim_window: u32,
    pub bmiss_count_max: u32,
    pub bmiss_first_bcnt: u32,
    pub bmiss_final_bcnt: u32,
    pub feature_wmm: u32,
    pub chwidth: u32,
    pub chextoffset: u32,
    pub disable_htprotection: u32,
    pub sta_quickkickout: u32,
    pub mgmt_rate: u32,
    pub protection_mode: u32,
    pub fixed_rate: u32,
    pub sgi: u32,
    pub ldpc: u32,
    pub tx_stbc: u32,
    pub rx_stbc: u32,
    pub intra_bss_fwd: u32,
    pub def_keyid: u32,
    pub nss: u32,
    pub bcast_data_rate: u32,
    pub mcast_data_rate: u32,
    pub mcast_indicate: u32,
    pub dhcp_indicate: u32,
    pub unknown_dest_indicate: u32,
    pub ap_keepalive_min_idle_inactive_time_secs: u32,
    pub ap_keepalive_max_idle_inactive_time_secs: u32,
    pub ap_keepalive_max_unresponsive_time_secs: u32,
    pub ap_enable_nawds: u32,
    pub mcast2ucast_set: u32,
    pub enable_rtscts: u32,
    pub txbf: u32,
    pub packet_powersave: u32,
    pub drop_unencry: u32,
    pub tx_encap_type: u32,
    pub ap_detect_out_of_sync_sleeping_sta_time_secs: u32,
    pub rc_num_retries: u32,
    pub cabq_maxdur: u32,
    pub mfptest_set: u32,
    pub rts_fixed_rate: u32,
    pub vht_sgimask: u32,
    pub vht80_ratemask: u32,
    pub early_rx_adjust_enable: u32,
    pub early_rx_tgt_bmiss_num: u32,
    pub early_rx_bmiss_sample_cycle: u32,
    pub early_rx_slop_step: u32,
    pub early_rx_init_slop: u32,
    pub early_rx_adjust_pause: u32,
    pub proxy_sta: u32,
    pub meru_vc: u32,
    pub rx_decap_type: u32,
    pub bw_nss_ratemask: u32,
    pub inc_tsf: u32,
    pub dec_tsf: u32,
    pub disable_4addr_src_lrn: u32,
    pub rtt_responder_role: u32,
}

pub const WMI_VDEV_PARAM_UNSUPPORTED: c_int = 0;
// the definition of different VDEV parameters
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vdev_param {
// RTS Threshold
    WMI_VDEV_PARAM_RTS_THRESHOLD = 0x1,
// Fragmentation threshold
    WMI_VDEV_PARAM_FRAGMENTATION_THRESHOLD,
// beacon interval in TUs
    WMI_VDEV_PARAM_BEACON_INTERVAL,
// Listen interval in TUs
    WMI_VDEV_PARAM_LISTEN_INTERVAL,
// multicast rate in Mbps
    WMI_VDEV_PARAM_MULTICAST_RATE,
// management frame rate in Mbps
    WMI_VDEV_PARAM_MGMT_TX_RATE,
// slot time (long vs short)
    WMI_VDEV_PARAM_SLOT_TIME,
// preamble (long vs short)
    WMI_VDEV_PARAM_PREAMBLE,
// SWBA time (time before tbtt in msec)
    WMI_VDEV_PARAM_SWBA_TIME,
// time period for updating VDEV stats
    WMI_VDEV_STATS_UPDATE_PERIOD,
// age out time in msec for frames queued for station in power save
    WMI_VDEV_PWRSAVE_AGEOUT_TIME,
//
// Host SWBA interval (time in msec before tbtt for SWBA event
// generation).
//
    WMI_VDEV_HOST_SWBA_INTERVAL,
// DTIM period (specified in units of num beacon intervals)
    WMI_VDEV_PARAM_DTIM_PERIOD,
//
// scheduler air time limit for this VDEV. used by off chan
// scheduler.
//
    WMI_VDEV_OC_SCHEDULER_AIR_TIME_LIMIT,
// enable/disable WDS for this VDEV
    WMI_VDEV_PARAM_WDS,
// ATIM Window
    WMI_VDEV_PARAM_ATIM_WINDOW,
// BMISS max
    WMI_VDEV_PARAM_BMISS_COUNT_MAX,
// BMISS first time
    WMI_VDEV_PARAM_BMISS_FIRST_BCNT,
// BMISS final time
    WMI_VDEV_PARAM_BMISS_FINAL_BCNT,
// WMM enables/disabled
    WMI_VDEV_PARAM_FEATURE_WMM,
// Channel width
    WMI_VDEV_PARAM_CHWIDTH,
// Channel Offset
    WMI_VDEV_PARAM_CHEXTOFFSET,
// Disable HT Protection
    WMI_VDEV_PARAM_DISABLE_HTPROTECTION,
// Quick STA Kickout
    WMI_VDEV_PARAM_STA_QUICKKICKOUT,
// Rate to be used with Management frames
    WMI_VDEV_PARAM_MGMT_RATE,
// Protection Mode
    WMI_VDEV_PARAM_PROTECTION_MODE,
// Fixed rate setting
    WMI_VDEV_PARAM_FIXED_RATE,
// Short GI Enable/Disable
    WMI_VDEV_PARAM_SGI,
// Enable LDPC
    WMI_VDEV_PARAM_LDPC,
// Enable Tx STBC
    WMI_VDEV_PARAM_TX_STBC,
// Enable Rx STBC
    WMI_VDEV_PARAM_RX_STBC,
// Intra BSS forwarding
    WMI_VDEV_PARAM_INTRA_BSS_FWD,
// Setting Default xmit key for Vdev
    WMI_VDEV_PARAM_DEF_KEYID,
// NSS width
    WMI_VDEV_PARAM_NSS,
// Set the custom rate for the broadcast data frames
    WMI_VDEV_PARAM_BCAST_DATA_RATE,
// Set the custom rate (rate-code) for multicast data frames
    WMI_VDEV_PARAM_MCAST_DATA_RATE,
// Tx multicast packet indicate Enable/Disable
    WMI_VDEV_PARAM_MCAST_INDICATE,
// Tx DHCP packet indicate Enable/Disable
    WMI_VDEV_PARAM_DHCP_INDICATE,
// Enable host inspection of Tx unicast packet to unknown destination
    WMI_VDEV_PARAM_UNKNOWN_DEST_INDICATE,

// The minimum amount of time AP begins to consider STA inactive
    WMI_VDEV_PARAM_AP_KEEPALIVE_MIN_IDLE_INACTIVE_TIME_SECS,

//
// An associated STA is considered inactive when there is no recent
// TX/RX activity and no downlink frames are buffered for it. Once a
// STA exceeds the maximum idle inactive time, the AP will send an
// 802.11 data-null as a keep alive to verify the STA is still
// associated. If the STA does ACK the data-null, or if the data-null
// is buffered and the STA does not retrieve it, the STA will be
// considered unresponsive
// (see WMI_VDEV_AP_KEEPALIVE_MAX_UNRESPONSIVE_TIME_SECS).
//
    WMI_VDEV_PARAM_AP_KEEPALIVE_MAX_IDLE_INACTIVE_TIME_SECS,

//
// An associated STA is considered unresponsive if there is no recent
// TX/RX activity and downlink frames are buffered for it. Once a STA
// exceeds the maximum unresponsive time, the AP will send a
// WMI_STA_KICKOUT event to the host so the STA can be deleted.
//
    WMI_VDEV_PARAM_AP_KEEPALIVE_MAX_UNRESPONSIVE_TIME_SECS,

// Enable NAWDS : MCAST INSPECT Enable, NAWDS Flag set
    WMI_VDEV_PARAM_AP_ENABLE_NAWDS,
// Enable/Disable RTS-CTS
    WMI_VDEV_PARAM_ENABLE_RTSCTS,
// Enable TXBFee/er
    WMI_VDEV_PARAM_TXBF,

// Set packet power save
    WMI_VDEV_PARAM_PACKET_POWERSAVE,

//
// Drops un-encrypted packets if eceived in an encrypted connection
// otherwise forwards to host.
//
    WMI_VDEV_PARAM_DROP_UNENCRY,

//
// Set the encapsulation type for frames.
//
    WMI_VDEV_PARAM_TX_ENCAP_TYPE,
}

// the definition of different VDEV parameters
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10x_vdev_param {
// RTS Threshold
    WMI_10X_VDEV_PARAM_RTS_THRESHOLD = 0x1,
// Fragmentation threshold
    WMI_10X_VDEV_PARAM_FRAGMENTATION_THRESHOLD,
// beacon interval in TUs
    WMI_10X_VDEV_PARAM_BEACON_INTERVAL,
// Listen interval in TUs
    WMI_10X_VDEV_PARAM_LISTEN_INTERVAL,
// multicast rate in Mbps
    WMI_10X_VDEV_PARAM_MULTICAST_RATE,
// management frame rate in Mbps
    WMI_10X_VDEV_PARAM_MGMT_TX_RATE,
// slot time (long vs short)
    WMI_10X_VDEV_PARAM_SLOT_TIME,
// preamble (long vs short)
    WMI_10X_VDEV_PARAM_PREAMBLE,
// SWBA time (time before tbtt in msec)
    WMI_10X_VDEV_PARAM_SWBA_TIME,
// time period for updating VDEV stats
    WMI_10X_VDEV_STATS_UPDATE_PERIOD,
// age out time in msec for frames queued for station in power save
    WMI_10X_VDEV_PWRSAVE_AGEOUT_TIME,
//
// Host SWBA interval (time in msec before tbtt for SWBA event
// generation).
//
    WMI_10X_VDEV_HOST_SWBA_INTERVAL,
// DTIM period (specified in units of num beacon intervals)
    WMI_10X_VDEV_PARAM_DTIM_PERIOD,
//
// scheduler air time limit for this VDEV. used by off chan
// scheduler.
//
    WMI_10X_VDEV_OC_SCHEDULER_AIR_TIME_LIMIT,
// enable/disable WDS for this VDEV
    WMI_10X_VDEV_PARAM_WDS,
// ATIM Window
    WMI_10X_VDEV_PARAM_ATIM_WINDOW,
// BMISS max
    WMI_10X_VDEV_PARAM_BMISS_COUNT_MAX,
// WMM enables/disabled
    WMI_10X_VDEV_PARAM_FEATURE_WMM,
// Channel width
    WMI_10X_VDEV_PARAM_CHWIDTH,
// Channel Offset
    WMI_10X_VDEV_PARAM_CHEXTOFFSET,
// Disable HT Protection
    WMI_10X_VDEV_PARAM_DISABLE_HTPROTECTION,
// Quick STA Kickout
    WMI_10X_VDEV_PARAM_STA_QUICKKICKOUT,
// Rate to be used with Management frames
    WMI_10X_VDEV_PARAM_MGMT_RATE,
// Protection Mode
    WMI_10X_VDEV_PARAM_PROTECTION_MODE,
// Fixed rate setting
    WMI_10X_VDEV_PARAM_FIXED_RATE,
// Short GI Enable/Disable
    WMI_10X_VDEV_PARAM_SGI,
// Enable LDPC
    WMI_10X_VDEV_PARAM_LDPC,
// Enable Tx STBC
    WMI_10X_VDEV_PARAM_TX_STBC,
// Enable Rx STBC
    WMI_10X_VDEV_PARAM_RX_STBC,
// Intra BSS forwarding
    WMI_10X_VDEV_PARAM_INTRA_BSS_FWD,
// Setting Default xmit key for Vdev
    WMI_10X_VDEV_PARAM_DEF_KEYID,
// NSS width
    WMI_10X_VDEV_PARAM_NSS,
// Set the custom rate for the broadcast data frames
    WMI_10X_VDEV_PARAM_BCAST_DATA_RATE,
// Set the custom rate (rate-code) for multicast data frames
    WMI_10X_VDEV_PARAM_MCAST_DATA_RATE,
// Tx multicast packet indicate Enable/Disable
    WMI_10X_VDEV_PARAM_MCAST_INDICATE,
// Tx DHCP packet indicate Enable/Disable
    WMI_10X_VDEV_PARAM_DHCP_INDICATE,
// Enable host inspection of Tx unicast packet to unknown destination
    WMI_10X_VDEV_PARAM_UNKNOWN_DEST_INDICATE,

// The minimum amount of time AP begins to consider STA inactive
    WMI_10X_VDEV_PARAM_AP_KEEPALIVE_MIN_IDLE_INACTIVE_TIME_SECS,

//
// An associated STA is considered inactive when there is no recent
// TX/RX activity and no downlink frames are buffered for it. Once a
// STA exceeds the maximum idle inactive time, the AP will send an
// 802.11 data-null as a keep alive to verify the STA is still
// associated. If the STA does ACK the data-null, or if the data-null
// is buffered and the STA does not retrieve it, the STA will be
// considered unresponsive
// (see WMI_10X_VDEV_AP_KEEPALIVE_MAX_UNRESPONSIVE_TIME_SECS).
//
    WMI_10X_VDEV_PARAM_AP_KEEPALIVE_MAX_IDLE_INACTIVE_TIME_SECS,

//
// An associated STA is considered unresponsive if there is no recent
// TX/RX activity and downlink frames are buffered for it. Once a STA
// exceeds the maximum unresponsive time, the AP will send a
// WMI_10X_STA_KICKOUT event to the host so the STA can be deleted.
//
    WMI_10X_VDEV_PARAM_AP_KEEPALIVE_MAX_UNRESPONSIVE_TIME_SECS,

// Enable NAWDS : MCAST INSPECT Enable, NAWDS Flag set
    WMI_10X_VDEV_PARAM_AP_ENABLE_NAWDS,

    WMI_10X_VDEV_PARAM_MCAST2UCAST_SET,
// Enable/Disable RTS-CTS
    WMI_10X_VDEV_PARAM_ENABLE_RTSCTS,

    WMI_10X_VDEV_PARAM_AP_DETECT_OUT_OF_SYNC_SLEEPING_STA_TIME_SECS,

// following are available as of firmware 10.2
    WMI_10X_VDEV_PARAM_TX_ENCAP_TYPE,
    WMI_10X_VDEV_PARAM_CABQ_MAXDUR,
    WMI_10X_VDEV_PARAM_MFPTEST_SET,
    WMI_10X_VDEV_PARAM_RTS_FIXED_RATE,
    WMI_10X_VDEV_PARAM_VHT_SGIMASK,
    WMI_10X_VDEV_PARAM_VHT80_RATEMASK,
    WMI_10X_VDEV_PARAM_TSF_INCREMENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_4_vdev_param {
    WMI_10_4_VDEV_PARAM_RTS_THRESHOLD = 0x1,
    WMI_10_4_VDEV_PARAM_FRAGMENTATION_THRESHOLD,
    WMI_10_4_VDEV_PARAM_BEACON_INTERVAL,
    WMI_10_4_VDEV_PARAM_LISTEN_INTERVAL,
    WMI_10_4_VDEV_PARAM_MULTICAST_RATE,
    WMI_10_4_VDEV_PARAM_MGMT_TX_RATE,
    WMI_10_4_VDEV_PARAM_SLOT_TIME,
    WMI_10_4_VDEV_PARAM_PREAMBLE,
    WMI_10_4_VDEV_PARAM_SWBA_TIME,
    WMI_10_4_VDEV_STATS_UPDATE_PERIOD,
    WMI_10_4_VDEV_PWRSAVE_AGEOUT_TIME,
    WMI_10_4_VDEV_HOST_SWBA_INTERVAL,
    WMI_10_4_VDEV_PARAM_DTIM_PERIOD,
    WMI_10_4_VDEV_OC_SCHEDULER_AIR_TIME_LIMIT,
    WMI_10_4_VDEV_PARAM_WDS,
    WMI_10_4_VDEV_PARAM_ATIM_WINDOW,
    WMI_10_4_VDEV_PARAM_BMISS_COUNT_MAX,
    WMI_10_4_VDEV_PARAM_BMISS_FIRST_BCNT,
    WMI_10_4_VDEV_PARAM_BMISS_FINAL_BCNT,
    WMI_10_4_VDEV_PARAM_FEATURE_WMM,
    WMI_10_4_VDEV_PARAM_CHWIDTH,
    WMI_10_4_VDEV_PARAM_CHEXTOFFSET,
    WMI_10_4_VDEV_PARAM_DISABLE_HTPROTECTION,
    WMI_10_4_VDEV_PARAM_STA_QUICKKICKOUT,
    WMI_10_4_VDEV_PARAM_MGMT_RATE,
    WMI_10_4_VDEV_PARAM_PROTECTION_MODE,
    WMI_10_4_VDEV_PARAM_FIXED_RATE,
    WMI_10_4_VDEV_PARAM_SGI,
    WMI_10_4_VDEV_PARAM_LDPC,
    WMI_10_4_VDEV_PARAM_TX_STBC,
    WMI_10_4_VDEV_PARAM_RX_STBC,
    WMI_10_4_VDEV_PARAM_INTRA_BSS_FWD,
    WMI_10_4_VDEV_PARAM_DEF_KEYID,
    WMI_10_4_VDEV_PARAM_NSS,
    WMI_10_4_VDEV_PARAM_BCAST_DATA_RATE,
    WMI_10_4_VDEV_PARAM_MCAST_DATA_RATE,
    WMI_10_4_VDEV_PARAM_MCAST_INDICATE,
    WMI_10_4_VDEV_PARAM_DHCP_INDICATE,
    WMI_10_4_VDEV_PARAM_UNKNOWN_DEST_INDICATE,
    WMI_10_4_VDEV_PARAM_AP_KEEPALIVE_MIN_IDLE_INACTIVE_TIME_SECS,
    WMI_10_4_VDEV_PARAM_AP_KEEPALIVE_MAX_IDLE_INACTIVE_TIME_SECS,
    WMI_10_4_VDEV_PARAM_AP_KEEPALIVE_MAX_UNRESPONSIVE_TIME_SECS,
    WMI_10_4_VDEV_PARAM_AP_ENABLE_NAWDS,
    WMI_10_4_VDEV_PARAM_MCAST2UCAST_SET,
    WMI_10_4_VDEV_PARAM_ENABLE_RTSCTS,
    WMI_10_4_VDEV_PARAM_RC_NUM_RETRIES,
    WMI_10_4_VDEV_PARAM_TXBF,
    WMI_10_4_VDEV_PARAM_PACKET_POWERSAVE,
    WMI_10_4_VDEV_PARAM_DROP_UNENCRY,
    WMI_10_4_VDEV_PARAM_TX_ENCAP_TYPE,
    WMI_10_4_VDEV_PARAM_AP_DETECT_OUT_OF_SYNC_SLEEPING_STA_TIME_SECS,
    WMI_10_4_VDEV_PARAM_CABQ_MAXDUR,
    WMI_10_4_VDEV_PARAM_MFPTEST_SET,
    WMI_10_4_VDEV_PARAM_RTS_FIXED_RATE,
    WMI_10_4_VDEV_PARAM_VHT_SGIMASK,
    WMI_10_4_VDEV_PARAM_VHT80_RATEMASK,
    WMI_10_4_VDEV_PARAM_EARLY_RX_ADJUST_ENABLE,
    WMI_10_4_VDEV_PARAM_EARLY_RX_TGT_BMISS_NUM,
    WMI_10_4_VDEV_PARAM_EARLY_RX_BMISS_SAMPLE_CYCLE,
    WMI_10_4_VDEV_PARAM_EARLY_RX_SLOP_STEP,
    WMI_10_4_VDEV_PARAM_EARLY_RX_INIT_SLOP,
    WMI_10_4_VDEV_PARAM_EARLY_RX_ADJUST_PAUSE,
    WMI_10_4_VDEV_PARAM_PROXY_STA,
    WMI_10_4_VDEV_PARAM_MERU_VC,
    WMI_10_4_VDEV_PARAM_RX_DECAP_TYPE,
    WMI_10_4_VDEV_PARAM_BW_NSS_RATEMASK,
    WMI_10_4_VDEV_PARAM_SENSOR_AP,
    WMI_10_4_VDEV_PARAM_BEACON_RATE,
    WMI_10_4_VDEV_PARAM_DTIM_ENABLE_CTS,
    WMI_10_4_VDEV_PARAM_STA_KICKOUT,
    WMI_10_4_VDEV_PARAM_CAPABILITIES,
    WMI_10_4_VDEV_PARAM_TSF_INCREMENT,
    WMI_10_4_VDEV_PARAM_RX_FILTER,
    WMI_10_4_VDEV_PARAM_MGMT_TX_POWER,
    WMI_10_4_VDEV_PARAM_ATF_SSID_SCHED_POLICY,
    WMI_10_4_VDEV_PARAM_DISABLE_DYN_BW_RTS,
    WMI_10_4_VDEV_PARAM_TSF_DECREMENT,
    WMI_10_4_VDEV_PARAM_SELFGEN_FIXED_RATE,
    WMI_10_4_VDEV_PARAM_AMPDU_SUBFRAME_SIZE_PER_AC,
    WMI_10_4_VDEV_PARAM_NSS_VHT160,
    WMI_10_4_VDEV_PARAM_NSS_VHT80_80,
    WMI_10_4_VDEV_PARAM_AMSDU_SUBFRAME_SIZE_PER_AC,
    WMI_10_4_VDEV_PARAM_DISABLE_CABQ,
    WMI_10_4_VDEV_PARAM_SIFS_TRIGGER_RATE,
    WMI_10_4_VDEV_PARAM_TX_POWER,
    WMI_10_4_VDEV_PARAM_ENABLE_DISABLE_RTT_RESPONDER_ROLE,
    WMI_10_4_VDEV_PARAM_DISABLE_4_ADDR_SRC_LRN,
}

pub const WMI_VDEV_DISABLE_4_ADDR_SRC_LRN: c_int = 1;

pub const WMI_TXBF_STS_CAP_OFFSET_LSB: c_int = 4;
pub const WMI_TXBF_STS_CAP_OFFSET_MASK: c_uint = 0x70;

pub const WMI_BF_SOUND_DIM_OFFSET_LSB: c_int = 8;
pub const WMI_BF_SOUND_DIM_OFFSET_MASK: c_uint = 0xf00;
// slot time long
pub const WMI_VDEV_SLOT_TIME_LONG: c_uint = 0x1;
// slot time short
pub const WMI_VDEV_SLOT_TIME_SHORT: c_uint = 0x2;
// preablbe long
pub const WMI_VDEV_PREAMBLE_LONG: c_uint = 0x1;
// preablbe short
pub const WMI_VDEV_PREAMBLE_SHORT: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_start_event_param {
    WMI_VDEV_RESP_START_EVENT = 0,
    WMI_VDEV_RESP_RESTART_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_start_response_event {
    pub vdev_id: __le32,
    pub req_id: __le32,
    pub /: *mut *mut __le32 resp_type; / %WMI_VDEV_RESP_,
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_standby_req_event {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_resume_req_event {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_stopped_event {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
    pub __packed: },
//
// common structure used for simple events
// (stopped, resume_req, standby response)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_simple_event {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
    pub __packed: },
// VDEV start response status codes
// VDEV successfully started
pub const WMI_INIFIED_VDEV_START_RESPONSE_STATUS_SUCCESS: c_uint = 0x0;
// requested VDEV not found
pub const WMI_INIFIED_VDEV_START_RESPONSE_INVALID_VDEVID: c_uint = 0x1;
// unsupported VDEV combination
pub const WMI_INIFIED_VDEV_START_RESPONSE_NOT_SUPPORTED: c_uint = 0x2;
// TODO: please add more comments if you have in-depth information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_spectral_conf_cmd {
    pub vdev_id: __le32,
// number of fft samples to send (0 for infinite)
    pub scan_count: __le32,
    pub scan_period: __le32,
    pub scan_priority: __le32,
// number of bins in the FFT: 2^(fft_size - bin_scale)
    pub scan_fft_size: __le32,
    pub scan_gc_ena: __le32,
    pub scan_restart_ena: __le32,
    pub scan_noise_floor_ref: __le32,
    pub scan_init_delay: __le32,
    pub scan_nb_tone_thr: __le32,
    pub scan_str_bin_thr: __le32,
    pub scan_wb_rpt_mode: __le32,
    pub scan_rssi_rpt_mode: __le32,
    pub scan_rssi_thr: __le32,
    pub scan_pwr_format: __le32,
// rpt_mode: Format of FFT report to software for spectral scan
// triggered FFTs:
// 0: No FFT report (only spectral scan summary report)
// 1: 2-dword summary of metrics for each completed FFT + spectral
// scan	summary report
// 2: 2-dword summary of metrics for each completed FFT +
// 1x- oversampled bins(in-band) per FFT + spectral scan summary
// report
// 3: 2-dword summary of metrics for each completed FFT +
// 2x- oversampled bins	(all) per FFT + spectral scan summary
//
    pub scan_rpt_mode: __le32,
    pub scan_bin_scale: __le32,
    pub scan_dbm_adj: __le32,
    pub scan_chn_mask: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_spectral_conf_arg {
    pub vdev_id: u32,
    pub scan_count: u32,
    pub scan_period: u32,
    pub scan_priority: u32,
    pub scan_fft_size: u32,
    pub scan_gc_ena: u32,
    pub scan_restart_ena: u32,
    pub scan_noise_floor_ref: u32,
    pub scan_init_delay: u32,
    pub scan_nb_tone_thr: u32,
    pub scan_str_bin_thr: u32,
    pub scan_wb_rpt_mode: u32,
    pub scan_rssi_rpt_mode: u32,
    pub scan_rssi_thr: u32,
    pub scan_pwr_format: u32,
    pub scan_rpt_mode: u32,
    pub scan_bin_scale: u32,
    pub scan_dbm_adj: u32,
    pub scan_chn_mask: u32,
}

pub const WMI_SPECTRAL_ENABLE_DEFAULT: c_int = 0;
pub const WMI_SPECTRAL_COUNT_DEFAULT: c_int = 0;
pub const WMI_SPECTRAL_PERIOD_DEFAULT: c_int = 35;
pub const WMI_SPECTRAL_PRIORITY_DEFAULT: c_int = 1;
pub const WMI_SPECTRAL_FFT_SIZE_DEFAULT: c_int = 7;
pub const WMI_SPECTRAL_GC_ENA_DEFAULT: c_int = 1;
pub const WMI_SPECTRAL_RESTART_ENA_DEFAULT: c_int = 0;

pub const WMI_SPECTRAL_INIT_DELAY_DEFAULT: c_int = 80;
pub const WMI_SPECTRAL_NB_TONE_THR_DEFAULT: c_int = 12;
pub const WMI_SPECTRAL_STR_BIN_THR_DEFAULT: c_int = 8;
pub const WMI_SPECTRAL_WB_RPT_MODE_DEFAULT: c_int = 0;
pub const WMI_SPECTRAL_RSSI_RPT_MODE_DEFAULT: c_int = 0;
pub const WMI_SPECTRAL_RSSI_THR_DEFAULT: c_uint = 0xf0;
pub const WMI_SPECTRAL_PWR_FORMAT_DEFAULT: c_int = 0;
pub const WMI_SPECTRAL_RPT_MODE_DEFAULT: c_int = 2;
pub const WMI_SPECTRAL_BIN_SCALE_DEFAULT: c_int = 1;
pub const WMI_SPECTRAL_DBM_ADJ_DEFAULT: c_int = 1;
pub const WMI_SPECTRAL_CHN_MASK_DEFAULT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_spectral_enable_cmd {
    pub vdev_id: __le32,
    pub trigger_cmd: __le32,
    pub enable_cmd: __le32,
    pub __packed: },
pub const WMI_SPECTRAL_TRIGGER_CMD_TRIGGER: c_int = 1;
pub const WMI_SPECTRAL_TRIGGER_CMD_CLEAR: c_int = 2;
pub const WMI_SPECTRAL_ENABLE_CMD_ENABLE: c_int = 1;
pub const WMI_SPECTRAL_ENABLE_CMD_DISABLE: c_int = 2;
// Beacon processing related command and event structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcn_tx_hdr {
    pub vdev_id: __le32,
    pub tx_rate: __le32,
    pub tx_power: __le32,
    pub bcn_len: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcn_tx_cmd {
    pub hdr: wmi_bcn_tx_hdr,
    pub bcn: [*mut u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcn_tx_arg {
    pub vdev_id: u32,
    pub tx_rate: u32,
    pub tx_power: u32,
    pub bcn_len: u32,
    pub bcn: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_bcn_tx_ref_flags {
    WMI_BCN_TX_REF_FLAG_DTIM_ZERO = 0x1,
    WMI_BCN_TX_REF_FLAG_DELIVER_CAB = 0x2,
}

// TODO: It is unclear why "no antenna" works while any other seemingly valid
// chainmask yields no beacons on the air at all.
//
pub const WMI_BCN_TX_REF_DEF_ANTENNA: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcn_tx_ref_cmd {
    pub vdev_id: __le32,
    pub data_len: __le32,
// physical address of the frame - dma pointer
    pub data_ptr: __le32,
// id for host to track
    pub msdu_id: __le32,
// frame ctrl to setup PPDU desc
    pub frame_control: __le32,
// to control CABQ traffic: WMI_BCN_TX_REF_FLAG_
    pub flags: __le32,
// introduced in 10.2
    pub antenna_mask: __le32,
    pub __packed: },
// Beacon filter

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcn_filter_rx_cmd {
// Filter ID
    pub bcn_filter_id: __le32,
// Filter type - wmi_bcn_filter
    pub bcn_filter: __le32,
// Buffer len
    pub bcn_filter_len: __le32,
// Filter info (threshold, BSSID, RSSI)
    pub bcn_filter_buf: *mut u8,
    pub __packed: },
// Capabilities and IEs to be passed to firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcn_prb_info {
// Capabilities
    pub caps: __le32,
// ERP info
    pub erp: __le32,
// Advanced capabilities
// HT capabilities
// HT Info
// ibss_dfs
// wpa Info
// rsn Info
// rrm info
// ath_ext
// app IE
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sta_ps_mode {
// enable power save for the given STA VDEV
    WMI_STA_PS_MODE_DISABLED = 0,
// disable power save  for a given STA VDEV
    WMI_STA_PS_MODE_ENABLED = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_powersave_mode_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
//
// Power save mode
// (see enum wmi_sta_ps_mode)
//
    pub sta_ps_mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_csa_offload_en {
    WMI_CSA_OFFLOAD_DISABLE = 0,
    WMI_CSA_OFFLOAD_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_csa_offload_enable_cmd {
    pub vdev_id: __le32,
    pub csa_offload_enable: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_csa_offload_chanswitch_cmd {
    pub vdev_id: __le32,
    pub chan: wmi_channel,
    pub __packed: },
//
// This parameter controls the policy for retrieving frames from AP while the
// STA is in sleep state.
//
// Only takes affect if the sta_ps_mode is enabled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sta_ps_param_rx_wake_policy {
//
// Wake up when ever there is an  RX activity on the VDEV. In this mode
// the Power save SM(state machine) will come out of sleep by either
// sending null frame (or) a data frame (with PS==0) in response to TIM
// bit set in the received beacon frame from AP.
//
    WMI_STA_PS_RX_WAKE_POLICY_WAKE = 0,

//
// Here the power save state machine will not wakeup in response to TIM
// bit, instead it will send a PSPOLL (or) UASPD trigger based on UAPSD
// configuration setup by WMISET_PS_SET_UAPSD  WMI command.  When all
// access categories are delivery-enabled, the station will send a
// UAPSD trigger frame, otherwise it will send a PS-Poll.
//
    WMI_STA_PS_RX_WAKE_POLICY_POLL_UAPSD = 1,
}

//
// Number of tx frames/beacon  that cause the power save SM to wake up.
//
// Value 1 causes the SM to wake up for every TX. Value 0 has a special
// meaning, It will cause the SM to never wake up. This is useful if you want
// to keep the system to sleep all the time for some kind of test mode . host
// can change this parameter any time.  It will affect at the next tx frame.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sta_ps_param_tx_wake_threshold {
    WMI_STA_PS_TX_WAKE_THRESHOLD_NEVER = 0,
    WMI_STA_PS_TX_WAKE_THRESHOLD_ALWAYS = 1,

//
// Values greater than one indicate that many TX attempts per beacon
// interval before the STA will wake up
//
}

//
// The maximum number of PS-Poll frames the FW will send in response to
// traffic advertised in TIM before waking up (by sending a null frame with PS
// = 0). Value 0 has a special meaning: there is no maximum count and the FW
// will send as many PS-Poll as are necessary to retrieve buffered BU. This
// parameter is used when the RX wake policy is
// WMI_STA_PS_RX_WAKE_POLICY_POLL_UAPSD and ignored when the RX wake
// policy is WMI_STA_PS_RX_WAKE_POLICY_WAKE.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sta_ps_param_pspoll_count {
    WMI_STA_PS_PSPOLL_COUNT_NO_MAX = 0,
//
// Values greater than 0 indicate the maximum number of PS-Poll frames
// FW will send before waking up.
//

// When u-APSD is enabled the firmware will be very reluctant to exit
// STA PS. This could result in very poor Rx performance with STA doing
// PS-Poll for each and every buffered frame. This value is a bit
// arbitrary.
//
    WMI_STA_PS_PSPOLL_COUNT_UAPSD = 3,
}

//
// This will include the delivery and trigger enabled state for every AC.
// This is the negotiated state with AP. The host MLME needs to set this based
// on AP capability and the state Set in the association request by the
// station MLME.Lower 8 bits of the value specify the UAPSD configuration.
//
pub const WMI_UAPSD_AC_TYPE_DELI: c_int = 0;
pub const WMI_UAPSD_AC_TYPE_TRIG: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sta_ps_param_uapsd {
    WMI_STA_PS_UAPSD_AC0_DELIVERY_EN = (1 << 0),
    WMI_STA_PS_UAPSD_AC0_TRIGGER_EN  = (1 << 1),
    WMI_STA_PS_UAPSD_AC1_DELIVERY_EN = (1 << 2),
    WMI_STA_PS_UAPSD_AC1_TRIGGER_EN  = (1 << 3),
    WMI_STA_PS_UAPSD_AC2_DELIVERY_EN = (1 << 4),
    WMI_STA_PS_UAPSD_AC2_TRIGGER_EN  = (1 << 5),
    WMI_STA_PS_UAPSD_AC3_DELIVERY_EN = (1 << 6),
    WMI_STA_PS_UAPSD_AC3_TRIGGER_EN  = (1 << 7),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_uapsd_auto_trig_param {
    pub wmm_ac: __le32,
    pub user_priority: __le32,
    pub service_interval: __le32,
    pub suspend_interval: __le32,
    pub delay_interval: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_uapsd_auto_trig_cmd_fixed_param {
    pub vdev_id: __le32,
    pub peer_macaddr: wmi_mac_addr,
    pub num_ac: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_uapsd_auto_trig_arg {
    pub wmm_ac: u32,
    pub user_priority: u32,
    pub service_interval: u32,
    pub suspend_interval: u32,
    pub delay_interval: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sta_powersave_param {
//
// Controls how frames are retrievd from AP while STA is sleeping
//
// (see enum wmi_sta_ps_param_rx_wake_policy)
//
    WMI_STA_PS_PARAM_RX_WAKE_POLICY = 0,

//
// The STA will go active after this many TX
//
// (see enum wmi_sta_ps_param_tx_wake_threshold)
//
    WMI_STA_PS_PARAM_TX_WAKE_THRESHOLD = 1,

//
// Number of PS-Poll to send before STA wakes up
//
// (see enum wmi_sta_ps_param_pspoll_count)
//
    WMI_STA_PS_PARAM_PSPOLL_COUNT = 2,

//
// TX/RX inactivity time in msec before going to sleep.
//
// The power save SM will monitor tx/rx activity on the VDEV, if no
// activity for the specified msec of the parameter the Power save
// SM will go to sleep.
//
    WMI_STA_PS_PARAM_INACTIVITY_TIME = 3,

//
// Set uapsd configuration.
//
// (see enum wmi_sta_ps_param_uapsd)
//
    WMI_STA_PS_PARAM_UAPSD = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_powersave_param_cmd {
    pub vdev_id: __le32,
    pub /: *mut *mut __le32 param_id; / %WMI_STA_PS_PARAM_,
    pub param_value: __le32,
    pub __packed: },
// No MIMO power save
// Macro flag: #define WMI_STA_MIMO_PS_MODE_DISABLE
// mimo powersave mode static
// Macro flag: #define WMI_STA_MIMO_PS_MODE_STATIC
// mimo powersave mode dynamic
// Macro flag: #define WMI_STA_MIMO_PS_MODE_DYNAMIC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_mimo_ps_mode_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
// mimo powersave mode as defined above
    pub mimo_pwrsave_mode: __le32,
    pub __packed: },
// U-APSD configuration of peer station from (re)assoc request and TSPECs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ap_ps_param_uapsd {
    WMI_AP_PS_UAPSD_AC0_DELIVERY_EN = (1 << 0),
    WMI_AP_PS_UAPSD_AC0_TRIGGER_EN  = (1 << 1),
    WMI_AP_PS_UAPSD_AC1_DELIVERY_EN = (1 << 2),
    WMI_AP_PS_UAPSD_AC1_TRIGGER_EN  = (1 << 3),
    WMI_AP_PS_UAPSD_AC2_DELIVERY_EN = (1 << 4),
    WMI_AP_PS_UAPSD_AC2_TRIGGER_EN  = (1 << 5),
    WMI_AP_PS_UAPSD_AC3_DELIVERY_EN = (1 << 6),
    WMI_AP_PS_UAPSD_AC3_TRIGGER_EN  = (1 << 7),
}

// U-APSD maximum service period of peer station
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ap_ps_peer_param_max_sp {
    WMI_AP_PS_PEER_PARAM_MAX_SP_UNLIMITED = 0,
    WMI_AP_PS_PEER_PARAM_MAX_SP_2 = 1,
    WMI_AP_PS_PEER_PARAM_MAX_SP_4 = 2,
    WMI_AP_PS_PEER_PARAM_MAX_SP_6 = 3,
    MAX_WMI_AP_PS_PEER_PARAM_MAX_SP,
}

//
// AP power save parameter
// Set a power save specific parameter for a peer station
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ap_ps_peer_param {
// Set uapsd configuration for a given peer.
//
// Include the delivery and trigger enabled state for every AC.
// The host  MLME needs to set this based on AP capability and stations
// request Set in the association request  received from the station.
//
// Lower 8 bits of the value specify the UAPSD configuration.
//
// (see enum wmi_ap_ps_param_uapsd)
// The default value is 0.
//
    WMI_AP_PS_PEER_PARAM_UAPSD = 0,

//
// Set the service period for a UAPSD capable station
//
// The service period from wme ie in the (re)assoc request frame.
//
// (see enum wmi_ap_ps_peer_param_max_sp)
//
    WMI_AP_PS_PEER_PARAM_MAX_SP = 1,

// Time in seconds for aging out buffered frames for STA in PS
    WMI_AP_PS_PEER_PARAM_AGEOUT_TIME = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ap_ps_peer_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
// AP powersave param (see enum wmi_ap_ps_peer_param)
    pub param_id: __le32,
// AP powersave param value
    pub param_value: __le32,
    pub __packed: },
// 128 clients = 4 words
pub const WMI_TIM_BITMAP_ARRAY_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tim_info {
    pub tim_len: __le32,
    pub tim_mcast: __le32,
    pub tim_bitmap: [__le32; WMI_TIM_BITMAP_ARRAY_SIZE],
    pub tim_changed: __le32,
    pub tim_num_ps_pending: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tim_info_arg {
    pub tim_len: __le32,
    pub tim_mcast: __le32,
    pub tim_bitmap: *const __le32,
    pub tim_changed: __le32,
    pub tim_num_ps_pending: __le32,
    pub __packed: },
// Maximum number of NOA Descriptors supported
pub const WMI_P2P_MAX_NOA_DESCRIPTORS: c_int = 4;

pub const WMI_P2P_OPPPS_CTWINDOW_OFFSET: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_noa_info {
// Bit 0 - Flag to indicate an update in NOA schedule
// Bits 7-1 - Reserved
//
    pub changed: u8,
// NOA index
    pub index: u8,
// Bit 0 - Opp PS state of the AP
// Bits 1-7 - Ctwindow in TUs
//
    pub ctwindow_oppps: u8,
// Number of NOA descriptors
    pub num_descriptors: u8,
    pub descriptors: [wmi_p2p_noa_descriptor; WMI_P2P_MAX_NOA_DESCRIPTORS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcn_info {
    pub tim_info: wmi_tim_info,
    pub p2p_noa_info: wmi_p2p_noa_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_host_swba_event {
    pub vdev_map: __le32,
    pub bcn_info: [wmi_bcn_info; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_2_4_bcn_info {
    pub tim_info: wmi_tim_info,
// The 10.2.4 FW doesn't have p2p NOA info
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_2_4_host_swba_event {
    pub vdev_map: __le32,
    pub bcn_info: [wmi_10_2_4_bcn_info; ],
    pub __packed: },
// 16 words = 512 client + 1 word = for guard
pub const WMI_10_4_TIM_BITMAP_ARRAY_SIZE: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_tim_info {
    pub tim_len: __le32,
    pub tim_mcast: __le32,
    pub tim_bitmap: [__le32; WMI_10_4_TIM_BITMAP_ARRAY_SIZE],
    pub tim_changed: __le32,
    pub tim_num_ps_pending: __le32,
    pub __packed: },
pub const WMI_10_4_P2P_MAX_NOA_DESCRIPTORS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_p2p_noa_info {
// Bit 0 - Flag to indicate an update in NOA schedule
// Bits 7-1 - Reserved
//
    pub changed: u8,
// NOA index
    pub index: u8,
// Bit 0 - Opp PS state of the AP
// Bits 1-7 - Ctwindow in TUs
//
    pub ctwindow_oppps: u8,
// Number of NOA descriptors
    pub num_descriptors: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_bcn_info {
    pub tim_info: wmi_10_4_tim_info,
    pub p2p_noa_info: wmi_10_4_p2p_noa_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_host_swba_event {
    pub vdev_map: __le32,
    pub bcn_info: [wmi_10_4_bcn_info; ],
    pub __packed: },
pub const WMI_MAX_AP_VDEV: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tbtt_offset_event {
    pub vdev_map: __le32,
    pub tbttoffset_list: [__le32; WMI_MAX_AP_VDEV],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_create_cmd {
    pub vdev_id: __le32,
    pub peer_macaddr: wmi_mac_addr,
    pub peer_type: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_peer_type {
    WMI_PEER_TYPE_DEFAULT = 0,
    WMI_PEER_TYPE_BSS = 1,
    WMI_PEER_TYPE_TDLS = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_delete_cmd {
    pub vdev_id: __le32,
    pub peer_macaddr: wmi_mac_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_flush_tids_cmd {
    pub vdev_id: __le32,
    pub peer_macaddr: wmi_mac_addr,
    pub peer_tid_bitmap: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_fixed_rate {
//
// rate mode . 0: disable fixed rate (auto rate)
// 1: legacy (non 11n) rate  specified as ieee rate 2*Mbps
// 2: ht20 11n rate  specified as mcs index
// 3: ht40 11n rate  specified as mcs index
//
    pub rate_mode: __le32,
//
// 4 rate values for 4 rate series. series 0 is stored in byte 0 (LSB)
// and series 3 is stored at byte 3 (MSB)
//
    pub rate_series: __le32,
//
// 4 retry counts for 4 rate series. retry count for rate 0 is stored
// in byte 0 (LSB) and retry count for rate 3 is stored at byte 3
// (MSB)
//
    pub rate_retries: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_fixed_rate_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
// fixed rate
    pub peer_fixed_rate: wmi_fixed_rate,
    pub __packed: },
pub const WMI_MGMT_TID: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_addba_clear_resp_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_addba_send_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
// Tid number
    pub tid: __le32,
// Buffer/Window size
    pub buffersize: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_delba_send_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
// Tid number
    pub tid: __le32,
// Is Initiator
    pub initiator: __le32,
// Reason code
    pub reasoncode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_addba_setresponse_cmd {
// unique id identifying the vdev, generated by the caller
    pub vdev_id: __le32,
// peer mac address
    pub peer_macaddr: wmi_mac_addr,
// Tid number
    pub tid: __le32,
// status code
    pub statuscode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_send_singleamsdu_cmd {
// unique id identifying the vdev, generated by the caller
    pub vdev_id: __le32,
// peer mac address
    pub peer_macaddr: wmi_mac_addr,
// Tid number
    pub tid: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_peer_smps_state {
    WMI_PEER_SMPS_PS_NONE = 0x0,
    WMI_PEER_SMPS_STATIC  = 0x1,
    WMI_PEER_SMPS_DYNAMIC = 0x2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_peer_chwidth {
    WMI_PEER_CHWIDTH_20MHZ = 0,
    WMI_PEER_CHWIDTH_40MHZ = 1,
    WMI_PEER_CHWIDTH_80MHZ = 2,
    WMI_PEER_CHWIDTH_160MHZ = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_peer_param {
    WMI_PEER_SMPS_STATE = 0x1, /* see %wmi_peer_smps_state */
    WMI_PEER_AMPDU      = 0x2,
    WMI_PEER_AUTHORIZE  = 0x3,
    WMI_PEER_CHAN_WIDTH = 0x4,
    WMI_PEER_NSS        = 0x5,
    WMI_PEER_USE_4ADDR  = 0x6,
    WMI_PEER_USE_FIXED_PWR = 0x8,
    WMI_PEER_PARAM_FIXED_RATE = 0x9,
    WMI_PEER_DEBUG      = 0xa,
    WMI_PEER_PHYMODE    = 0xd,
    WMI_PEER_DUMMY_VAR  = 0xff, /* dummy parameter for STA PS workaround */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_set_param_cmd {
    pub vdev_id: __le32,
    pub peer_macaddr: wmi_mac_addr,
    pub param_id: __le32,
    pub param_value: __le32,
    pub __packed: },
pub const MAX_SUPPORTED_RATES: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rate_set {
// total number of rates
    pub num_rates: __le32,
//
// rates (each 8bit value) packed into a 32 bit word.
// the rates are filled from least significant byte to most
// significant byte.
//
    pub 1]: __le32 rates[(MAX_SUPPORTED_RATES / 4) +,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rate_set_arg {
    pub num_rates: c_uint,
    pub rates: [u8; MAX_SUPPORTED_RATES],
}

//
// NOTE: It would bea good idea to represent the Tx MCS
// info in one word and Rx in another word. This is split
// into multiple words for convenience
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vht_rate_set {
    pub /: *mut *mut __le32 rx_max_rate; / Max Rx data rate,
    pub /: *mut *mut __le32 rx_mcs_set; / Negotiated RX VHT rates,
    pub /: *mut *mut __le32 tx_max_rate; / Max Tx data rate,
    pub /: *mut *mut __le32 tx_mcs_set; / Negotiated TX VHT rates,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vht_rate_set_arg {
    pub rx_max_rate: u32,
    pub rx_mcs_set: u32,
    pub tx_max_rate: u32,
    pub tx_mcs_set: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_set_rates_cmd {
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
// legacy rate set
    pub peer_legacy_rates: wmi_rate_set,
// ht rate set
    pub peer_ht_rates: wmi_rate_set,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_set_q_empty_callback_cmd {
// unique id identifying the VDEV, generated by the caller
    pub vdev_id: __le32,
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
    pub callback_enable: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_flags_map {
    pub auth: u32,
    pub qos: u32,
    pub need_ptk_4_way: u32,
    pub need_gtk_2_way: u32,
    pub apsd: u32,
    pub ht: u32,
    pub bw40: u32,
    pub stbc: u32,
    pub ldbc: u32,
    pub dyn_mimops: u32,
    pub static_mimops: u32,
    pub spatial_mux: u32,
    pub vht: u32,
    pub bw80: u32,
    pub vht_2g: u32,
    pub pmf: u32,
    pub bw160: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_peer_flags {
    WMI_PEER_AUTH = 0x00000001,
    WMI_PEER_QOS = 0x00000002,
    WMI_PEER_NEED_PTK_4_WAY = 0x00000004,
    WMI_PEER_NEED_GTK_2_WAY = 0x00000010,
    WMI_PEER_APSD = 0x00000800,
    WMI_PEER_HT = 0x00001000,
    WMI_PEER_40MHZ = 0x00002000,
    WMI_PEER_STBC = 0x00008000,
    WMI_PEER_LDPC = 0x00010000,
    WMI_PEER_DYN_MIMOPS = 0x00020000,
    WMI_PEER_STATIC_MIMOPS = 0x00040000,
    WMI_PEER_SPATIAL_MUX = 0x00200000,
    WMI_PEER_VHT = 0x02000000,
    WMI_PEER_80MHZ = 0x04000000,
    WMI_PEER_VHT_2G = 0x08000000,
    WMI_PEER_PMF = 0x10000000,
    WMI_PEER_160MHZ = 0x20000000
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10x_peer_flags {
    WMI_10X_PEER_AUTH = 0x00000001,
    WMI_10X_PEER_QOS = 0x00000002,
    WMI_10X_PEER_NEED_PTK_4_WAY = 0x00000004,
    WMI_10X_PEER_NEED_GTK_2_WAY = 0x00000010,
    WMI_10X_PEER_APSD = 0x00000800,
    WMI_10X_PEER_HT = 0x00001000,
    WMI_10X_PEER_40MHZ = 0x00002000,
    WMI_10X_PEER_STBC = 0x00008000,
    WMI_10X_PEER_LDPC = 0x00010000,
    WMI_10X_PEER_DYN_MIMOPS = 0x00020000,
    WMI_10X_PEER_STATIC_MIMOPS = 0x00040000,
    WMI_10X_PEER_SPATIAL_MUX = 0x00200000,
    WMI_10X_PEER_VHT = 0x02000000,
    WMI_10X_PEER_80MHZ = 0x04000000,
    WMI_10X_PEER_160MHZ = 0x20000000
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_10_2_peer_flags {
    WMI_10_2_PEER_AUTH = 0x00000001,
    WMI_10_2_PEER_QOS = 0x00000002,
    WMI_10_2_PEER_NEED_PTK_4_WAY = 0x00000004,
    WMI_10_2_PEER_NEED_GTK_2_WAY = 0x00000010,
    WMI_10_2_PEER_APSD = 0x00000800,
    WMI_10_2_PEER_HT = 0x00001000,
    WMI_10_2_PEER_40MHZ = 0x00002000,
    WMI_10_2_PEER_STBC = 0x00008000,
    WMI_10_2_PEER_LDPC = 0x00010000,
    WMI_10_2_PEER_DYN_MIMOPS = 0x00020000,
    WMI_10_2_PEER_STATIC_MIMOPS = 0x00040000,
    WMI_10_2_PEER_SPATIAL_MUX = 0x00200000,
    WMI_10_2_PEER_VHT = 0x02000000,
    WMI_10_2_PEER_80MHZ = 0x04000000,
    WMI_10_2_PEER_VHT_2G = 0x08000000,
    WMI_10_2_PEER_PMF = 0x10000000,
    WMI_10_2_PEER_160MHZ = 0x20000000
}

//
// Peer rate capabilities.
//
// This is of interest to the ratecontrol
// module which resides in the firmware. The bit definitions are
// consistent with that defined in if_athrate.c.
//
pub const WMI_RC_DS_FLAG: c_uint = 0x01;
pub const WMI_RC_CW40_FLAG: c_uint = 0x02;
pub const WMI_RC_SGI_FLAG: c_uint = 0x04;
pub const WMI_RC_HT_FLAG: c_uint = 0x08;
pub const WMI_RC_RTSCTS_FLAG: c_uint = 0x10;
pub const WMI_RC_TX_STBC_FLAG: c_uint = 0x20;
pub const WMI_RC_RX_STBC_FLAG: c_uint = 0xC0;
pub const WMI_RC_RX_STBC_FLAG_S: c_int = 6;
pub const WMI_RC_WEP_TKIP_FLAG: c_uint = 0x100;
pub const WMI_RC_TS_FLAG: c_uint = 0x200;
pub const WMI_RC_UAPSD_FLAG: c_uint = 0x400;
// Maximum listen interval supported by hw in units of beacon interval
pub const ATH10K_MAX_HW_LISTEN_INTERVAL: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_common_peer_assoc_complete_cmd {
    pub peer_macaddr: wmi_mac_addr,
    pub vdev_id: __le32,
    pub /: *mut *mut __le32 peer_new_assoc; / 1=assoc, 0=reassoc,
    pub /: *mut *mut __le32 peer_associd; / 16 LSBs,
    pub peer_flags: __le32,
    pub /: *mut *mut __le32 peer_caps; / 16 LSBs,
    pub peer_listen_intval: __le32,
    pub peer_ht_caps: __le32,
    pub peer_max_mpdu: __le32,
    pub /: *mut *mut __le32 peer_mpdu_density; / 0..16,
    pub peer_rate_caps: __le32,
    pub peer_legacy_rates: wmi_rate_set,
    pub peer_ht_rates: wmi_rate_set,
    pub /: *mut *mut __le32 peer_nss; / num of spatial streams,
    pub peer_vht_caps: __le32,
    pub peer_phymode: __le32,
    pub peer_vht_rates: wmi_vht_rate_set,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_main_peer_assoc_complete_cmd {
    pub cmd: wmi_common_peer_assoc_complete_cmd,
// HT Operation Element of the peer. Five bytes packed in 2
// INT32 array and filled from lsb to msb.
//
    pub peer_ht_info: [__le32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_1_peer_assoc_complete_cmd {
    pub cmd: wmi_common_peer_assoc_complete_cmd,
    pub __packed: },
pub const WMI_PEER_ASSOC_INFO0_MAX_MCS_IDX_LSB: c_int = 0;
pub const WMI_PEER_ASSOC_INFO0_MAX_MCS_IDX_MASK: c_uint = 0x0f;
pub const WMI_PEER_ASSOC_INFO0_MAX_NSS_LSB: c_int = 4;
pub const WMI_PEER_ASSOC_INFO0_MAX_NSS_MASK: c_uint = 0xf0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_2_peer_assoc_complete_cmd {
    pub cmd: wmi_common_peer_assoc_complete_cmd,
    pub /: *mut *mut __le32 info0; / WMI_PEER_ASSOC_INFO0_,
    pub __packed: },
// NSS Mapping to FW

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_peer_assoc_complete_cmd {
    pub cmd: wmi_10_2_peer_assoc_complete_cmd,
    pub peer_bw_rxnss_override: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_assoc_complete_arg {
    pub addr: [u8; ETH_ALEN],
    pub vdev_id: u32,
    pub peer_reassoc: bool,
    pub peer_aid: u16,
    pub /: *mut *mut u32 peer_flags; / see %WMI_PEER_,
    pub peer_caps: u16,
    pub peer_listen_intval: u32,
    pub peer_ht_caps: u32,
    pub peer_max_mpdu: u32,
    pub /: *mut *mut u32 peer_mpdu_density; / 0..16,
    pub /: *mut *mut u32 peer_rate_caps; / see %WMI_RC_,
    pub peer_legacy_rates: wmi_rate_set_arg,
    pub peer_ht_rates: wmi_rate_set_arg,
    pub peer_num_spatial_streams: u32,
    pub peer_vht_caps: u32,
    pub peer_phymode: wmi_phy_mode,
    pub peer_vht_rates: wmi_vht_rate_set_arg,
    pub peer_bw_rxnss_override: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_add_wds_entry_cmd {
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
// wds MAC addr
    pub wds_macaddr: wmi_mac_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_remove_wds_entry_cmd {
// wds MAC addr
    pub wds_macaddr: wmi_mac_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_q_empty_callback_event {
// peer MAC address
    pub peer_macaddr: wmi_mac_addr,
    pub __packed: },
//
// Channel info WMI event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_chan_info_event {
    pub err_code: __le32,
    pub freq: __le32,
    pub cmd_flags: __le32,
    pub noise_floor: __le32,
    pub rx_clear_count: __le32,
    pub cycle_count: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_chan_info_event {
    pub err_code: __le32,
    pub freq: __le32,
    pub cmd_flags: __le32,
    pub noise_floor: __le32,
    pub rx_clear_count: __le32,
    pub cycle_count: __le32,
    pub chan_tx_pwr_range: __le32,
    pub chan_tx_pwr_tp: __le32,
    pub rx_frame_count: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_sta_kickout_event {
    pub peer_macaddr: wmi_mac_addr,
    pub __packed: },

// Beacon filter wmi command info
pub const BCN_FLT_MAX_SUPPORTED_IES: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_bcn_stats {
    pub vdev_id: __le32,
    pub bss_bcnsdropped: __le32,
    pub bss_bcnsdelivered: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcn_filter_stats {
    pub bcns_dropped: __le32,
    pub bcns_delivered: __le32,
    pub activefilters: __le32,
    pub bss_stats: bss_bcn_stats,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_add_bcn_filter_cmd {
    pub vdev_id: u32,
    pub ie_map: [u32; BCN_FLT_MAX_ELEMS_IE_LIST],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sta_keepalive_method {
    WMI_STA_KEEPALIVE_METHOD_NULL_FRAME = 1,
    WMI_STA_KEEPALIVE_METHOD_UNSOLICITATED_ARP_RESPONSE = 2,
}

pub const WMI_STA_KEEPALIVE_INTERVAL_DISABLE: c_int = 0;
// Firmware crashes if keepalive interval exceeds this limit
pub const WMI_STA_KEEPALIVE_INTERVAL_MAX_SECONDS: c_uint = 0xffff;
// note: ip4 addresses are in network byte order, i.e. big endian
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_keepalive_arp_resp {
    pub src_ip4_addr: __be32,
    pub dest_ip4_addr: __be32,
    pub dest_mac_addr: wmi_mac_addr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_keepalive_cmd {
    pub vdev_id: __le32,
    pub enabled: __le32,
    pub /: *mut *mut __le32 method; / WMI_STA_KEEPALIVE_METHOD_,
    pub /: *mut *mut __le32 interval; / in seconds,
    pub arp_resp: wmi_sta_keepalive_arp_resp,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_keepalive_arg {
    pub vdev_id: u32,
    pub enabled: u32,
    pub method: u32,
    pub interval: u32,
    pub src_ip4_addr: __be32,
    pub dest_ip4_addr: __be32,
    pub dest_mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_force_fw_hang_type {
    WMI_FORCE_FW_HANG_ASSERT = 1,
    WMI_FORCE_FW_HANG_NO_DETECT,
    WMI_FORCE_FW_HANG_CTRL_EP_FULL,
    WMI_FORCE_FW_HANG_EMPTY_POINT,
    WMI_FORCE_FW_HANG_STACK_OVERFLOW,
    WMI_FORCE_FW_HANG_INFINITE_LOOP,
}

pub const WMI_FORCE_FW_HANG_RANDOM_TIME: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_force_fw_hang_cmd {
    pub type: __le32,
    pub delay_ms: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_pdev_reset_mode_type {
    WMI_RST_MODE_TX_FLUSH = 1,
    WMI_RST_MODE_WARM_RESET,
    WMI_RST_MODE_COLD_RESET,
    WMI_RST_MODE_WARM_RESET_RESTORE_CAL,
    WMI_RST_MODE_COLD_RESET_RESTORE_CAL,
    WMI_RST_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_dbglog_level {
    ATH10K_DBGLOG_LEVEL_VERBOSE = 0,
    ATH10K_DBGLOG_LEVEL_INFO = 1,
    ATH10K_DBGLOG_LEVEL_WARN = 2,
    ATH10K_DBGLOG_LEVEL_ERR = 3,
}

// VAP ids to enable dbglog
pub const ATH10K_DBGLOG_CFG_VAP_LOG_LSB: c_int = 0;
pub const ATH10K_DBGLOG_CFG_VAP_LOG_MASK: c_uint = 0x0000ffff;
// to enable dbglog in the firmware
pub const ATH10K_DBGLOG_CFG_REPORTING_ENABLE_LSB: c_int = 16;
pub const ATH10K_DBGLOG_CFG_REPORTING_ENABLE_MASK: c_uint = 0x00010000;
// timestamp resolution
pub const ATH10K_DBGLOG_CFG_RESOLUTION_LSB: c_int = 17;
pub const ATH10K_DBGLOG_CFG_RESOLUTION_MASK: c_uint = 0x000E0000;
// number of queued messages before sending them to the host
pub const ATH10K_DBGLOG_CFG_REPORT_SIZE_LSB: c_int = 20;
pub const ATH10K_DBGLOG_CFG_REPORT_SIZE_MASK: c_uint = 0x0ff00000;
//
// Log levels to enable. This defines the minimum level to enable, this is
// not a bitmask. See enum ath10k_dbglog_level for the values.
//
pub const ATH10K_DBGLOG_CFG_LOG_LVL_LSB: c_int = 28;
pub const ATH10K_DBGLOG_CFG_LOG_LVL_MASK: c_uint = 0x70000000;
//
// Note: this is a cleaned up version of a struct firmware uses. For
// example, config_valid was hidden inside an array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_dbglog_cfg_cmd {
// bitmask to hold mod id config
    pub module_enable: __le32,
// see ATH10K_DBGLOG_CFG_
    pub config_enable: __le32,
// mask of module id bits to be changed
    pub module_valid: __le32,
// mask of config bits to be changed, see ATH10K_DBGLOG_CFG_
    pub config_valid: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_dbglog_cfg_cmd {
// bitmask to hold mod id config
    pub module_enable: __le64,
// see ATH10K_DBGLOG_CFG_
    pub config_enable: __le32,
// mask of module id bits to be changed
    pub module_valid: __le64,
// mask of config bits to be changed, see ATH10K_DBGLOG_CFG_
    pub config_valid: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_roam_reason {
    WMI_ROAM_REASON_BETTER_AP = 1,
    WMI_ROAM_REASON_BEACON_MISS = 2,
    WMI_ROAM_REASON_LOW_RSSI = 3,
    WMI_ROAM_REASON_SUITABLE_AP_FOUND = 4,
    WMI_ROAM_REASON_HO_FAILED = 5,

// keep last
    WMI_ROAM_REASON_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_roam_ev {
    pub vdev_id: __le32,
    pub reason: __le32,
    pub __packed: },
pub const ATH10K_FRAGMT_THRESHOLD_MIN: c_int = 540;
pub const ATH10K_FRAGMT_THRESHOLD_MAX: c_int = 2346;
pub const WMI_MAX_EVENT: c_uint = 0x1000;
// Maximum number of pending TXed WMI packets

// By default disable power save for IBSS
pub const ATH10K_DEFAULT_ATIM: c_int = 0;
pub const WMI_MAX_MEM_REQS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_scan_ev_arg {
    pub /: *mut *mut __le32 event_type; / %WMI_SCAN_EVENT_,
    pub /: *mut *mut __le32 reason; / %WMI_SCAN_REASON_,
    pub /: *mut *mut __le32 channel_freq; / only valid for WMI_SCAN_EVENT_FOREIGN_CHANNEL,
    pub scan_req_id: __le32,
    pub scan_id: __le32,
    pub vdev_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_tx_compl_params {
    pub desc_id: u32,
    pub status: u32,
    pub ppdu_id: u32,
    pub ack_rssi: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tlv_mgmt_tx_compl_ev_arg {
    pub desc_id: __le32,
    pub status: __le32,
    pub pdev_id: __le32,
    pub ppdu_id: __le32,
    pub ack_rssi: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tlv_mgmt_tx_bundle_compl_ev_arg {
    pub num_reports: __le32,
    pub desc_ids: *const __le32,
    pub status: *const __le32,
    pub ppdu_ids: *const __le32,
    pub ack_rssi: *const __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_delete_resp_ev_arg {
    pub vdev_id: __le32,
    pub peer_addr: wmi_mac_addr,
}

pub const WMI_MGMT_RX_NUM_RSSI: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mgmt_rx_ev_arg {
    pub channel: __le32,
    pub snr: __le32,
    pub rate: __le32,
    pub phy_mode: __le32,
    pub buf_len: __le32,
    pub /: *mut *mut __le32 status; / %WMI_RX_STATUS_,
    pub ext_info: wmi_mgmt_rx_ext_info,
    pub rssi: [__le32; WMI_MGMT_RX_NUM_RSSI],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ch_info_ev_arg {
    pub err_code: __le32,
    pub freq: __le32,
    pub cmd_flags: __le32,
    pub noise_floor: __le32,
    pub rx_clear_count: __le32,
    pub cycle_count: __le32,
    pub chan_tx_pwr_range: __le32,
    pub chan_tx_pwr_tp: __le32,
    pub rx_frame_count: __le32,
    pub my_bss_rx_cycle_count: __le32,
    pub rx_11b_mode_data_duration: __le32,
    pub tx_frame_cnt: __le32,
    pub mac_clk_mhz: __le32,
}

// From 10.4 firmware, not sure all have the same values.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vdev_start_status {
    WMI_VDEV_START_OK = 0,
    WMI_VDEV_START_CHAN_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vdev_start_ev_arg {
    pub vdev_id: __le32,
    pub req_id: __le32,
    pub /: *mut *mut __le32 resp_type; / %WMI_VDEV_RESP_,
    pub /: *mut *mut __le32 status; / See wmi_vdev_start_status enum above,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_kick_ev_arg {
    pub mac_addr: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_swba_ev_arg {
    pub vdev_map: __le32,
    pub tim_info: [wmi_tim_info_arg; WMI_MAX_AP_VDEV],
    pub noa_info: [*const wmi_p2p_noa_info; WMI_MAX_AP_VDEV],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_phyerr_ev_arg {
    pub tsf_timestamp: u32,
    pub freq1: u16,
    pub freq2: u16,
    pub rssi_combined: u8,
    pub chan_width_mhz: u8,
    pub phy_err_code: u8,
    pub nf_chains: [u16; 4],
    pub buf_len: u32,
    pub buf: *const u8,
    pub hdr_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_phyerr_hdr_arg {
    pub num_phyerrs: u32,
    pub tsf_l32: u32,
    pub tsf_u32: u32,
    pub buf_len: u32,
    pub phyerrs: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_dfs_status_ev_arg {
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_svc_rdy_ev_arg {
    pub min_tx_power: __le32,
    pub max_tx_power: __le32,
    pub ht_cap: __le32,
    pub vht_cap: __le32,
    pub vht_supp_mcs: __le32,
    pub sw_ver0: __le32,
    pub sw_ver1: __le32,
    pub fw_build: __le32,
    pub phy_capab: __le32,
    pub num_rf_chains: __le32,
    pub eeprom_rd: __le32,
    pub num_mem_reqs: __le32,
    pub low_2ghz_chan: __le32,
    pub high_2ghz_chan: __le32,
    pub low_5ghz_chan: __le32,
    pub high_5ghz_chan: __le32,
    pub sys_cap_info: __le32,
    pub service_map: *const __le32,
    pub service_map_len: usize,
    pub mem_reqs: [*const wlan_host_mem_req; WMI_MAX_MEM_REQS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_svc_avail_ev_arg {
    pub service_map_ext_valid: bool,
    pub service_map_ext_len: __le32,
    pub service_map_ext: *const __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rdy_ev_arg {
    pub sw_version: __le32,
    pub abi_version: __le32,
    pub status: __le32,
    pub mac_addr: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_roam_ev_arg {
    pub vdev_id: __le32,
    pub reason: __le32,
    pub rssi: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_echo_ev_arg {
    pub value: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_temperature_event {
// temperature value in Celsius degree
    pub temperature: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_bss_chan_info_event {
    pub freq: __le32,
    pub noise_floor: __le32,
    pub cycle_busy: __le64,
    pub cycle_total: __le64,
    pub cycle_tx: __le64,
    pub cycle_rx: __le64,
    pub cycle_rx_bss: __le64,
    pub reserved: __le32,
    pub __packed: },
// WOW structures
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_wow_wakeup_event {
    WOW_BMISS_EVENT = 0,
    WOW_BETTER_AP_EVENT,
    WOW_DEAUTH_RECVD_EVENT,
    WOW_MAGIC_PKT_RECVD_EVENT,
    WOW_GTK_ERR_EVENT,
    WOW_FOURWAY_HSHAKE_EVENT,
    WOW_EAPOL_RECVD_EVENT,
    WOW_NLO_DETECTED_EVENT,
    WOW_DISASSOC_RECVD_EVENT,
    WOW_PATTERN_MATCH_EVENT,
    WOW_CSA_IE_EVENT,
    WOW_PROBE_REQ_WPS_IE_EVENT,
    WOW_AUTH_REQ_EVENT,
    WOW_ASSOC_REQ_EVENT,
    WOW_HTT_EVENT,
    WOW_RA_MATCH_EVENT,
    WOW_HOST_AUTO_SHUTDOWN_EVENT,
    WOW_IOAC_MAGIC_EVENT,
    WOW_IOAC_SHORT_EVENT,
    WOW_IOAC_EXTEND_EVENT,
    WOW_IOAC_TIMER_EVENT,
    WOW_DFS_PHYERR_RADAR_EVENT,
    WOW_BEACON_EVENT,
    WOW_CLIENT_KICKOUT_EVENT,
    WOW_EVENT_MAX,
}

    pub NULL: return,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_wow_wake_reason {
    WOW_REASON_UNSPECIFIED = -1,
    WOW_REASON_NLOD = 0,
    WOW_REASON_AP_ASSOC_LOST,
    WOW_REASON_LOW_RSSI,
    WOW_REASON_DEAUTH_RECVD,
    WOW_REASON_DISASSOC_RECVD,
    WOW_REASON_GTK_HS_ERR,
    WOW_REASON_EAP_REQ,
    WOW_REASON_FOURWAY_HS_RECV,
    WOW_REASON_TIMER_INTR_RECV,
    WOW_REASON_PATTERN_MATCH_FOUND,
    WOW_REASON_RECV_MAGIC_PATTERN,
    WOW_REASON_P2P_DISC,
    WOW_REASON_WLAN_HB,
    WOW_REASON_CSA_EVENT,
    WOW_REASON_PROBE_REQ_WPS_IE_RECV,
    WOW_REASON_AUTH_REQ_RECV,
    WOW_REASON_ASSOC_REQ_RECV,
    WOW_REASON_HTT_EVENT,
    WOW_REASON_RA_MATCH,
    WOW_REASON_HOST_AUTO_SHUTDOWN,
    WOW_REASON_IOAC_MAGIC_EVENT,
    WOW_REASON_IOAC_SHORT_EVENT,
    WOW_REASON_IOAC_EXTEND_EVENT,
    WOW_REASON_IOAC_TIMER_EVENT,
    WOW_REASON_ROAM_HO,
    WOW_REASON_DFS_PHYERR_RADADR_EVENT,
    WOW_REASON_BEACON_RECV,
    WOW_REASON_CLIENT_KICKOUT_EVENT,
    WOW_REASON_DEBUG_TEST = 0xFF,
}

    pub NULL: return,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_wow_ev_arg {
    pub vdev_id: u32,
    pub flag: u32,
    pub wake_reason: wmi_wow_wake_reason,
    pub data_len: u32,
}

pub const WOW_MIN_PATTERN_SIZE: c_int = 1;
pub const WOW_MAX_PATTERN_SIZE: c_int = 148;
pub const WOW_MAX_PKT_OFFSET: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tdls_state {
    WMI_TDLS_DISABLE,
    WMI_TDLS_ENABLE_PASSIVE,
    WMI_TDLS_ENABLE_ACTIVE,
    WMI_TDLS_ENABLE_ACTIVE_EXTERNAL_CONTROL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tdls_peer_state {
    WMI_TDLS_PEER_STATE_PEERING,
    WMI_TDLS_PEER_STATE_CONNECTED,
    WMI_TDLS_PEER_STATE_TEARDOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tdls_peer_update_cmd_arg {
    pub vdev_id: u32,
    pub peer_state: wmi_tdls_peer_state,
    pub addr: [u8; ETH_ALEN],
}

pub const WMI_TDLS_MAX_SUPP_OPER_CLASSES: c_int = 32;
pub const WMI_TDLS_PEER_SP_MASK: c_uint = 0x60;
pub const WMI_TDLS_PEER_SP_LSB: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tdls_options {
    WMI_TDLS_OFFCHAN_EN = BIT(0),
    WMI_TDLS_BUFFER_STA_EN = BIT(1),
    WMI_TDLS_SLEEP_STA_EN = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tdls_peer_capab_arg {
    pub peer_uapsd_queues: u8,
    pub peer_max_sp: u8,
    pub buff_sta_support: u32,
    pub off_chan_support: u32,
    pub peer_curr_operclass: u32,
    pub self_curr_operclass: u32,
    pub peer_chan_len: u32,
    pub peer_operclass_len: u32,
    pub peer_operclass: [u8; WMI_TDLS_MAX_SUPP_OPER_CLASSES],
    pub is_peer_responder: u32,
    pub pref_offchan_num: u32,
    pub pref_offchan_bw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_tdls_set_state_cmd {
    pub vdev_id: __le32,
    pub state: __le32,
    pub notification_interval_ms: __le32,
    pub tx_discovery_threshold: __le32,
    pub tx_teardown_threshold: __le32,
    pub rssi_teardown_threshold: __le32,
    pub rssi_delta: __le32,
    pub tdls_options: __le32,
    pub tdls_peer_traffic_ind_window: __le32,
    pub tdls_peer_traffic_response_timeout_ms: __le32,
    pub tdls_puapsd_mask: __le32,
    pub tdls_puapsd_inactivity_time_ms: __le32,
    pub tdls_puapsd_rx_frame_threshold: __le32,
    pub teardown_notification_ms: __le32,
    pub tdls_peer_kickout_threshold: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tdls_peer_capabilities {
    pub peer_qos: __le32,
    pub buff_sta_support: __le32,
    pub off_chan_support: __le32,
    pub peer_curr_operclass: __le32,
    pub self_curr_operclass: __le32,
    pub peer_chan_len: __le32,
    pub peer_operclass_len: __le32,
    pub peer_operclass: [u8; WMI_TDLS_MAX_SUPP_OPER_CLASSES],
    pub is_peer_responder: __le32,
    pub pref_offchan_num: __le32,
    pub pref_offchan_bw: __le32,
// to match legacy implementation allocate room for
// at least one record even if peer_chan_len is 0
//
    pub peer_chan_min_allocation: wmi_channel,
    pub peer_chan_list): DECLARE_FLEX_ARRAY(struct wmi_channel,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_10_4_tdls_peer_update_cmd {
    pub vdev_id: __le32,
    pub peer_macaddr: wmi_mac_addr,
    pub peer_state: __le32,
    pub reserved: [__le32; 4],
    pub peer_capab: wmi_tdls_peer_capabilities,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tdls_peer_reason {
    WMI_TDLS_TEARDOWN_REASON_TX,
    WMI_TDLS_TEARDOWN_REASON_RSSI,
    WMI_TDLS_TEARDOWN_REASON_SCAN,
    WMI_TDLS_DISCONNECTED_REASON_PEER_DELETE,
    WMI_TDLS_TEARDOWN_REASON_PTR_TIMEOUT,
    WMI_TDLS_TEARDOWN_REASON_BAD_PTR,
    WMI_TDLS_TEARDOWN_REASON_NO_RESPONSE,
    WMI_TDLS_ENTER_BUF_STA,
    WMI_TDLS_EXIT_BUF_STA,
    WMI_TDLS_ENTER_BT_BUSY_MODE,
    WMI_TDLS_EXIT_BT_BUSY_MODE,
    WMI_TDLS_SCAN_STARTED_EVENT,
    WMI_TDLS_SCAN_COMPLETED_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tdls_peer_notification {
    WMI_TDLS_SHOULD_DISCOVER,
    WMI_TDLS_SHOULD_TEARDOWN,
    WMI_TDLS_PEER_DISCONNECTED,
    WMI_TDLS_CONNECTION_TRACKER_NOTIFICATION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tdls_peer_event {
    pub peer_macaddr: wmi_mac_addr,
// see enum wmi_tdls_peer_notification
    pub peer_status: __le32,
// see enum wmi_tdls_peer_reason
    pub peer_reason: __le32,
    pub vdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tid_aggr_control_conf {
    WMI_TID_CONFIG_AGGR_CONTROL_IGNORE,
    WMI_TID_CONFIG_AGGR_CONTROL_ENABLE,
    WMI_TID_CONFIG_AGGR_CONTROL_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_noack_tid_conf {
    WMI_NOACK_TID_CONFIG_IGNORE_ACK_POLICY,
    WMI_PEER_TID_CONFIG_ACK,
    WMI_PEER_TID_CONFIG_NOACK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tid_rate_ctrl_conf {
    WMI_TID_CONFIG_RATE_CONTROL_IGNORE,
    WMI_TID_CONFIG_RATE_CONTROL_AUTO,
    WMI_TID_CONFIG_RATE_CONTROL_FIXED_RATE,
    WMI_TID_CONFIG_RATE_CONTROL_DEFAULT_LOWEST_RATE,
    WMI_PEER_TID_CONFIG_RATE_UPPER_CAP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tid_rtscts_control_conf {
    WMI_TID_CONFIG_RTSCTS_CONTROL_ENABLE,
    WMI_TID_CONFIG_RTSCTS_CONTROL_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ext_tid_config_map {
    WMI_EXT_TID_RTS_CTS_CONFIG = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_per_peer_per_tid_cfg_arg {
    pub vdev_id: u32,
    pub peer_macaddr: wmi_mac_addr,
    pub tid: u32,
    pub ack_policy: wmi_noack_tid_conf,
    pub aggr_control: wmi_tid_aggr_control_conf,
    pub rate_ctrl: u8,
    pub retry_count: u32,
    pub rcode_flags: u32,
    pub ext_tid_cfg_bitmap: u32,
    pub rtscts_ctrl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_per_tid_cfg_cmd {
    pub vdev_id: __le32,
    pub peer_macaddr: wmi_mac_addr,
    pub tid: __le32,
// see enum wmi_noack_tid_conf
    pub ack_policy: __le32,
// see enum wmi_tid_aggr_control_conf
    pub aggr_control: __le32,
// see enum wmi_tid_rate_ctrl_conf
    pub rate_control: __le32,
    pub rcode_flags: __le32,
    pub retry_count: __le32,
// See enum wmi_ext_tid_config_map
    pub ext_tid_cfg_bitmap: __le32,
// see enum wmi_tid_rtscts_control_conf
    pub rtscts_ctrl: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_txbf_conf {
    WMI_TXBF_CONF_UNSUPPORTED,
    WMI_TXBF_CONF_BEFORE_ASSOC,
    WMI_TXBF_CONF_AFTER_ASSOC,
}

pub const WMI_CCA_DETECT_LEVEL_AUTO: c_int = 0;
pub const WMI_CCA_DETECT_MARGIN_AUTO: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_set_adaptive_cca_params {
    pub enable: __le32,
    pub cca_detect_level: __le32,
    pub cca_detect_margin: __le32,
    pub __packed: },
pub const WMI_PNO_MAX_SCHED_SCAN_PLANS: c_int = 2;
pub const WMI_PNO_MAX_SCHED_SCAN_PLAN_INT: c_int = 7200;
pub const WMI_PNO_MAX_SCHED_SCAN_PLAN_ITRNS: c_int = 100;
pub const WMI_PNO_MAX_NETW_CHANNELS: c_int = 26;
pub const WMI_PNO_MAX_NETW_CHANNELS_EX: c_int = 60;

// size based of dot11 declaration without extra IEs as we will not carry those for PNO
pub const WMI_PNO_MAX_PB_REQ_SIZE: c_int = 450;
pub const WMI_PNO_24G_DEFAULT_CH: c_int = 1;
pub const WMI_PNO_5G_DEFAULT_CH: c_int = 36;
pub const WMI_ACTIVE_MAX_CHANNEL_TIME: c_int = 40;
pub const WMI_PASSIVE_MAX_CHANNEL_TIME: c_int = 110;
// SSID broadcast type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_SSID_bcast_type {
    BCAST_UNKNOWN      = 0,
    BCAST_NORMAL       = 1,
    BCAST_HIDDEN       = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_network_type {
    pub ssid: wmi_ssid,
    pub authentication: u32,
    pub encryption: u32,
    pub bcast_nw_type: u32,
    pub channel_count: u8,
    pub channels: [u16; WMI_PNO_MAX_NETW_CHANNELS_EX],
    pub rssi_threshold: i32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pno_scan_req {
    pub enable: u8,
    pub vdev_id: u8,
    pub uc_networks_count: u8,
    pub a_networks: [wmi_network_type; WMI_PNO_MAX_SUPP_NETWORKS],
    pub fast_scan_period: u32,
    pub slow_scan_period: u32,
    pub fast_scan_max_cycles: u8,
    pub do_passive_scan: bool,
    pub delay_start_time: u32,
    pub active_min_time: u32,
    pub active_max_time: u32,
    pub passive_min_time: u32,
    pub passive_max_time: u32,
// mac address randomization attributes
    pub enable_pno_scan_randomization: u32,
    pub mac_addr: [u8; ETH_ALEN],
    pub mac_addr_mask: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_host_platform_type {
    WMI_HOST_PLATFORM_HIGH_PERF,
    WMI_HOST_PLATFORM_LOW_PERF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_bss_survey_req_type {
    WMI_BSS_SURVEY_REQ_TYPE_READ = 1,
    WMI_BSS_SURVEY_REQ_TYPE_READ_CLEAR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_chan_info_req_cmd {
    pub type: __le32,
    pub reserved: __le32,
    pub __packed: },
// bb timing register configurations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bb_timing_cfg_arg {
// Tx_end to pa off timing
    pub bb_tx_timing: u32,
// Tx_end to external pa off timing
    pub bb_xpa_timing: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pdev_bb_timing_cfg_cmd {
// Tx_end to pa off timing
    pub bb_tx_timing: __le32,
// Tx_end to external pa off timing
    pub bb_xpa_timing: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ftm_seg_hdr {
    pub len: __le32,
    pub msgref: __le32,
    pub segmentinfo: __le32,
    pub pdev_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ftm_cmd {
    pub tlv_header: __le32,
    pub seg_hdr: wmi_ftm_seg_hdr,
    pub data: [u8; ],
    pub __packed: },

pub const MAX_WMI_UTF_LEN: c_int = 252;
    pub ath10k: struct,
    pub ath10k_vif: struct,
    pub ath10k_fw_stats_pdev: struct,
    pub ath10k_fw_stats_peer: struct,
    pub ath10k_fw_stats: struct,
    pub ar): *mut int ath10k_wmi_attach(struct ath10k,
    pub ar): *mut void ath10k_wmi_detach(struct ath10k,
    pub ar): *mut void ath10k_wmi_free_host_mem(struct ath10k,
    pub ar): *mut int ath10k_wmi_wait_for_service_ready(struct ath10k,
    pub ar): *mut int ath10k_wmi_wait_for_unified_ready(struct ath10k,
    pub len): *mut *mut *mut sk_buff ath10k_wmi_alloc_skb(ath10k ar, u32,
    pub ar): *mut int ath10k_wmi_connect(struct ath10k,
    pub cmd_id): *mut *mut *mut int ath10k_wmi_cmd_send(struct ath10k ar, struct sk_buff skb, u32,
    pub cmd_id): u32,
    pub arg): *mut *mut void ath10k_wmi_start_scan_init(struct ath10k ar, struct wmi_start_scan_arg,
    pub dst): *mut ath10k_fw_stats_pdev,
    pub dst): *mut ath10k_fw_stats_pdev,
    pub dst): *mut ath10k_fw_stats_pdev,
    pub dst): *mut ath10k_fw_stats_pdev,
    pub dst): *mut ath10k_fw_stats_peer,
    pub chunks): *mut wmi_host_mem_chunks,
    pub arg): *const wmi_start_scan_arg,
    pub arg): *const wmi_wmm_params_arg,
    pub arg): *const wmi_channel_arg,
    pub arg): *const int ath10k_wmi_start_scan_verify(struct wmi_start_scan_arg,
    pub skb): *mut *mut int ath10k_wmi_event_scan(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut int ath10k_wmi_event_mgmt_rx(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut int ath10k_wmi_event_mgmt_tx_compl(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut int ath10k_wmi_event_mgmt_tx_bundle_compl(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_chan_info(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_echo(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut int ath10k_wmi_event_debug_mesg(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_update_stats(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_vdev_start_resp(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_vdev_stopped(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_peer_sta_kickout(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_host_swba(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_tbttoffset_update(struct ath10k ar, struct sk_buff,
    pub tsf): *mut *mut wmi_phyerr_ev_arg phyerr, u64,
    pub tsf): u64,
    pub skb): *mut *mut void ath10k_wmi_event_phyerr(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_roam(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_profile_match(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_debug_print(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_pdev_qvit(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_wlan_profile_data(struct ath10k ar, struct sk_buff,
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_rtt_error_report(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_wow_wakeup_host(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_dcs_interference(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_pdev_tpc_config(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_pdev_ftm_intg(struct ath10k ar, struct sk_buff,
    pub skb): *mut sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_gtk_rekey_fail(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_delba_complete(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_addba_complete(struct ath10k ar, struct sk_buff,
    pub skb): *mut sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_inst_rssi_stats(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_vdev_standby_req(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_vdev_resume_req(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_service_ready(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut int ath10k_wmi_event_ready(struct ath10k ar, struct sk_buff,
    pub skb): *mut *mut void ath10k_wmi_event_service_available(struct ath10k ar, struct sk_buff,
    pub arg): *mut int left_len, struct wmi_phyerr_ev_arg,
    pub buf): *mut c_char,
    pub buf): *mut c_char,
    pub buf): *mut c_char,
    pub subtype): wmi_vdev_subtype,
    pub ar): *mut int ath10k_wmi_barrier(struct ath10k,
    pub num_tx_chain): u32,
    pub skb): *mut *mut void ath10k_wmi_event_tpc_final_table(struct ath10k ar, struct sk_buff,
