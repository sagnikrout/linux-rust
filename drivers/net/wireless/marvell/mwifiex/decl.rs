//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/decl.h
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
// NXP Wireless LAN device driver: generic data structures and APIs
//
// Copyright 2011-2020 NXP
//

pub const MWIFIEX_BSS_COEX_COUNT: c_int = 2;

pub const MWIFIEX_DMA_ALIGN_SZ: c_int = 64;
pub const MWIFIEX_RX_HEADROOM: c_int = 64;
pub const MAX_TXPD_SZ: c_int = 32;
pub const INTF_HDR_ALIGN: c_int = 4;

// + sizeof(tx_control)
//
pub const FRMCTL_LEN: c_int = 2;
pub const DURATION_LEN: c_int = 2;
pub const SEQCTL_LEN: c_int = 2;
// special FW 4 address management header

pub const AUTH_ALG_LEN: c_int = 2;
pub const AUTH_TRANSACTION_LEN: c_int = 2;
pub const AUTH_STATUS_LEN: c_int = 2;

pub const AUTH_TX_DEFAULT_WAIT_TIME: c_int = 2400;
pub const WLAN_AUTH_NONE: c_uint = 0xFFFF;
pub const MWIFIEX_MAX_TX_BASTREAM_SUPPORTED: c_int = 2;
pub const MWIFIEX_MAX_RX_BASTREAM_SUPPORTED: c_int = 16;
pub const MWIFIEX_MAX_TDLS_PEER_SUPPORTED: c_int = 8;
pub const MWIFIEX_STA_AMPDU_DEF_TXWINSIZE: c_int = 64;
pub const MWIFIEX_STA_AMPDU_DEF_RXWINSIZE: c_int = 64;
pub const MWIFIEX_STA_COEX_AMPDU_DEF_RXWINSIZE: c_int = 16;
pub const MWIFIEX_UAP_AMPDU_DEF_TXWINSIZE: c_int = 32;
pub const MWIFIEX_UAP_COEX_AMPDU_DEF_RXWINSIZE: c_int = 16;
pub const MWIFIEX_UAP_AMPDU_DEF_RXWINSIZE: c_int = 16;
pub const MWIFIEX_11AC_STA_AMPDU_DEF_TXWINSIZE: c_int = 64;
pub const MWIFIEX_11AC_STA_AMPDU_DEF_RXWINSIZE: c_int = 64;
pub const MWIFIEX_11AC_UAP_AMPDU_DEF_TXWINSIZE: c_int = 64;
pub const MWIFIEX_11AC_UAP_AMPDU_DEF_RXWINSIZE: c_int = 64;
pub const MWIFIEX_DEFAULT_BLOCK_ACK_TIMEOUT: c_uint = 0xffff;
pub const MWIFIEX_RATE_BITMAP_MCS0: c_int = 32;

pub const MWIFIEX_WMM_VERSION: c_uint = 0x01;
pub const MWIFIEX_WMM_SUBTYPE: c_uint = 0x01;
pub const MWIFIEX_RETRY_LIMIT: c_int = 14;
pub const MWIFIEX_SDIO_BLOCK_SIZE: c_int = 256;

pub const MWIFIEX_BRIDGED_PKTS_THR_HIGH: c_int = 1024;
pub const MWIFIEX_BRIDGED_PKTS_THR_LOW: c_int = 128;
pub const MWIFIEX_TDLS_DISABLE_LINK: c_uint = 0x00;
pub const MWIFIEX_TDLS_ENABLE_LINK: c_uint = 0x01;
pub const MWIFIEX_TDLS_CREATE_LINK: c_uint = 0x02;
pub const MWIFIEX_TDLS_CONFIG_LINK: c_uint = 0x03;
pub const MWIFIEX_TDLS_RSSI_HIGH: c_int = 50;
pub const MWIFIEX_TDLS_RSSI_LOW: c_int = 55;
pub const MWIFIEX_TDLS_MAX_FAIL_COUNT: c_int = 4;
pub const MWIFIEX_AUTO_TDLS_IDLE_TIME: c_int = 10;
// 54M rates, index from 0 to 11
pub const MWIFIEX_RATE_INDEX_MCS0: c_int = 12;
// 12-27=MCS0-15(BW20)
pub const MWIFIEX_BW20_MCS_NUM: c_int = 15;
// Rate index for OFDM 0
pub const MWIFIEX_RATE_INDEX_OFDM0: c_int = 4;
pub const MWIFIEX_MAX_STA_NUM: c_int = 3;
pub const MWIFIEX_MAX_UAP_NUM: c_int = 3;
pub const MWIFIEX_MAX_P2P_NUM: c_int = 3;
pub const MWIFIEX_A_BAND_START_FREQ: c_int = 5000;
// SDIO Aggr data packet special info

pub const BLOCK_NUMBER_OFFSET: c_int = 15;
pub const SDIO_HEADER_OFFSET: c_int = 28;
pub const MWIFIEX_SIZE_4K: c_uint = 0x4000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_bss_type {
    MWIFIEX_BSS_TYPE_STA = 0,
    MWIFIEX_BSS_TYPE_UAP = 1,
    MWIFIEX_BSS_TYPE_P2P = 2,
    MWIFIEX_BSS_TYPE_ANY = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_bss_role {
    MWIFIEX_BSS_ROLE_STA = 0,
    MWIFIEX_BSS_ROLE_UAP = 1,
    MWIFIEX_BSS_ROLE_ANY = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_tdls_status {
    TDLS_NOT_SETUP = 0,
    TDLS_SETUP_INPROGRESS,
    TDLS_SETUP_COMPLETE,
    TDLS_SETUP_FAILURE,
    TDLS_LINK_TEARDOWN,
    TDLS_CHAN_SWITCHING,
    TDLS_IN_BASE_CHAN,
    TDLS_IN_OFF_CHAN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_tdls_error_code {
    TDLS_ERR_NO_ERROR = 0,
    TDLS_ERR_INTERNAL_ERROR,
    TDLS_ERR_MAX_LINKS_EST,
    TDLS_ERR_LINK_EXISTS,
    TDLS_ERR_LINK_NONEXISTENT,
    TDLS_ERR_PEER_STA_UNREACHABLE = 25,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_data_frame_type {
    MWIFIEX_DATA_FRAME_TYPE_ETH_II = 0,
    MWIFIEX_DATA_FRAME_TYPE_802_11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_fw_image {
    pub helper_buf: *mut u8,
    pub helper_len: u32,
    pub fw_buf: *mut u8,
    pub fw_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_802_11_ssid {
    pub ssid_len: u32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_wait_queue {
    pub wait: wait_queue_head_t,
    pub status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_rxinfo {
    pub parent: *mut sk_buff,
    pub bss_num: u8,
    pub bss_type: u8,
    pub use_count: u8,
    pub buf_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_txinfo {
    pub flags: u8,
    pub bss_num: u8,
    pub bss_type: u8,
    pub aggr_num: u8,
    pub pkt_len: u32,
    pub ack_frame_id: u8,
    pub cookie: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_wmm_ac_e {
    WMM_AC_BK,
    WMM_AC_BE,
    WMM_AC_VI,
    WMM_AC_VO
    } __packed;

    struct ieee_types_wmm_ac_parameters {
    u8 aci_aifsn_bitmap;
    u8 ecw_bitmap;
    __le16 tx_op_limit;
    } __packed;

    struct mwifiex_types_wmm_info {
    u8 oui[4];
    u8 subtype;
    u8 version;
    u8 qos_info;
    u8 reserved;
    struct ieee_types_wmm_ac_parameters ac_params[IEEE80211_NUM_ACS];
    } __packed;

    struct mwifiex_arp_eth_header {
    struct arphdr hdr;
    u8 ar_sha[ETH_ALEN];
    u8 ar_sip[4];
    u8 ar_tha[ETH_ALEN];
    u8 ar_tip[4];
    } __packed;

    struct mwifiex_chan_stats {
    u8 chan_num;
    u8 bandcfg;
    u8 flags;
    s8 noise;
    u16 total_bss;
    u16 cca_scan_dur;
    u16 cca_busy_dur;
    } __packed;

pub const MWIFIEX_HIST_MAX_SAMPLES: c_int = 1048576;
pub const MWIFIEX_MAX_RX_RATES: c_int = 44;
pub const MWIFIEX_MAX_AC_RX_RATES: c_int = 74;
pub const MWIFIEX_MAX_SNR: c_int = 256;
pub const MWIFIEX_MAX_NOISE_FLR: c_int = 256;
pub const MWIFIEX_MAX_SIG_STRENGTH: c_int = 256;

    struct mwifiex_histogram_data {
    atomic_t rx_rate[MWIFIEX_MAX_AC_RX_RATES];
    atomic_t snr[MWIFIEX_MAX_SNR];
    atomic_t noise_flr[MWIFIEX_MAX_NOISE_FLR];
    atomic_t sig_str[MWIFIEX_MAX_SIG_STRENGTH];
    atomic_t num_samples;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_iface_comb {
    pub sta_intf: u8,
    pub uap_intf: u8,
    pub p2p_intf: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_radar_params {
    pub chandef: *mut cfg80211_chan_def,
    pub cac_time_ms: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_11h_intf_state {
    pub is_11h_enabled: bool,
    pub is_11h_active: bool,
    pub __packed: },
pub const MWIFIEX_FW_DUMP_IDX: c_uint = 0xff;
pub const MWIFIEX_FW_DUMP_MAX_MEMSIZE: c_uint = 0x160000;
pub const MWIFIEX_DRV_INFO_IDX: c_int = 20;
pub const FW_DUMP_MAX_NAME_LEN: c_int = 8;
pub const FW_DUMP_HOST_READY: c_uint = 0xEE;
pub const FW_DUMP_DONE: c_uint = 0xFF;
pub const FW_DUMP_READ_DONE: c_uint = 0xFE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_type_mapping {
    pub mem_name: [u8; FW_DUMP_MAX_NAME_LEN],
    pub mem_ptr: *mut u8,
    pub mem_size: u32,
    pub done_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdwr_status {
    RDWR_STATUS_SUCCESS = 0,
    RDWR_STATUS_FAILURE = 1,
    RDWR_STATUS_DONE = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_chan_width {
    CHAN_BW_20MHZ = 0,
    CHAN_BW_10MHZ,
    CHAN_BW_40MHZ,
    CHAN_BW_80MHZ,
    CHAN_BW_8080MHZ,
    CHAN_BW_160MHZ,
    CHAN_BW_5MHZ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_chan_offset {
    SEC_CHAN_NONE = 0,
    SEC_CHAN_ABOVE = 1,
    SEC_CHAN_5MHZ = 2,
    SEC_CHAN_BELOW = 3
}
