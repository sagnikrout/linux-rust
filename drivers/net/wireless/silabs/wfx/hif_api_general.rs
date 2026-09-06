//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/hif_api_general.h
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


// SPDX-License-Identifier: GPL-2.0-only or Apache-2.0
//
// WF200 hardware interface definitions
//
// Copyright (c) 2018-2020, Silicon Laboratories Inc.
//

pub const HIF_ID_IS_INDICATION: c_uint = 0x80;
pub const HIF_COUNTER_MAX: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_msg {
    pub len: __le16,
    pub id: u8,
    pub reserved:1: u8,
    pub interface:2: u8,
    pub seqnum:3: u8,
    pub encrypted:2: u8,
    pub body: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_general_requests_ids {
    HIF_REQ_ID_CONFIGURATION        = 0x09,
    HIF_REQ_ID_CONTROL_GPIO         = 0x26,
    HIF_REQ_ID_SET_SL_MAC_KEY       = 0x27,
    HIF_REQ_ID_SL_EXCHANGE_PUB_KEYS = 0x28,
    HIF_REQ_ID_SL_CONFIGURE         = 0x29,
    HIF_REQ_ID_PREVENT_ROLLBACK     = 0x2a,
    HIF_REQ_ID_PTA_SETTINGS         = 0x2b,
    HIF_REQ_ID_PTA_PRIORITY         = 0x2c,
    HIF_REQ_ID_PTA_STATE            = 0x2d,
    HIF_REQ_ID_SHUT_DOWN            = 0x32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_general_confirmations_ids {
    HIF_CNF_ID_CONFIGURATION        = 0x09,
    HIF_CNF_ID_CONTROL_GPIO         = 0x26,
    HIF_CNF_ID_SET_SL_MAC_KEY       = 0x27,
    HIF_CNF_ID_SL_EXCHANGE_PUB_KEYS = 0x28,
    HIF_CNF_ID_SL_CONFIGURE         = 0x29,
    HIF_CNF_ID_PREVENT_ROLLBACK     = 0x2a,
    HIF_CNF_ID_PTA_SETTINGS         = 0x2b,
    HIF_CNF_ID_PTA_PRIORITY         = 0x2c,
    HIF_CNF_ID_PTA_STATE            = 0x2d,
    HIF_CNF_ID_SHUT_DOWN            = 0x32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_general_indications_ids {
    HIF_IND_ID_EXCEPTION            = 0xe0,
    HIF_IND_ID_STARTUP              = 0xe1,
    HIF_IND_ID_WAKEUP               = 0xe2,
    HIF_IND_ID_GENERIC              = 0xe3,
    HIF_IND_ID_ERROR                = 0xe4,
    HIF_IND_ID_SL_EXCHANGE_PUB_KEYS = 0xe5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_api_rate_index {
    API_RATE_INDEX_B_1MBPS     = 0,
    API_RATE_INDEX_B_2MBPS     = 1,
    API_RATE_INDEX_B_5P5MBPS   = 2,
    API_RATE_INDEX_B_11MBPS    = 3,
    API_RATE_INDEX_PBCC_22MBPS = 4,
    API_RATE_INDEX_PBCC_33MBPS = 5,
    API_RATE_INDEX_G_6MBPS     = 6,
    API_RATE_INDEX_G_9MBPS     = 7,
    API_RATE_INDEX_G_12MBPS    = 8,
    API_RATE_INDEX_G_18MBPS    = 9,
    API_RATE_INDEX_G_24MBPS    = 10,
    API_RATE_INDEX_G_36MBPS    = 11,
    API_RATE_INDEX_G_48MBPS    = 12,
    API_RATE_INDEX_G_54MBPS    = 13,
    API_RATE_INDEX_N_6P5MBPS   = 14,
    API_RATE_INDEX_N_13MBPS    = 15,
    API_RATE_INDEX_N_19P5MBPS  = 16,
    API_RATE_INDEX_N_26MBPS    = 17,
    API_RATE_INDEX_N_39MBPS    = 18,
    API_RATE_INDEX_N_52MBPS    = 19,
    API_RATE_INDEX_N_58P5MBPS  = 20,
    API_RATE_INDEX_N_65MBPS    = 21,
    API_RATE_NUM_ENTRIES       = 22
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_startup {
    pub status: __le32,
    pub hardware_id: __le16,
    pub opn: [u8; 14],
    pub uid: [u8; 8],
    pub num_inp_ch_bufs: __le16,
    pub size_inp_ch_buf: __le16,
    pub num_links_ap: u8,
    pub num_interfaces: u8,
    pub mac_addr: [u8; 2][ETH_ALEN],
    pub api_version_minor: u8,
    pub api_version_major: u8,
    pub link_mode:2: u8,
    pub reserved1:6: u8,
    pub reserved2: u8,
    pub reserved3: u8,
    pub reserved4: u8,
    pub firmware_build: u8,
    pub firmware_minor: u8,
    pub firmware_major: u8,
    pub firmware_type: u8,
    pub disabled_channel_list: [u8; 2],
    pub region_sel_mode:4: u8,
    pub reserved5:4: u8,
    pub phy1_region:3: u8,
    pub phy0_region:3: u8,
    pub otp_phy_ver:2: u8,
    pub supported_rate_mask: __le32,
    pub firmware_label: [u8; 128],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_wakeup {
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_configuration {
    pub length: __le16,
    pub pds_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_configuration {
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_gpio_mode {
    HIF_GPIO_MODE_D0       = 0x0,
    HIF_GPIO_MODE_D1       = 0x1,
    HIF_GPIO_MODE_OD0      = 0x2,
    HIF_GPIO_MODE_OD1      = 0x3,
    HIF_GPIO_MODE_TRISTATE = 0x4,
    HIF_GPIO_MODE_TOGGLE   = 0x5,
    HIF_GPIO_MODE_READ     = 0x6
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_req_control_gpio {
    pub gpio_label: u8,
    pub gpio_mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_cnf_control_gpio {
    pub status: __le32,
    pub value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_generic_indication_type {
    HIF_GENERIC_INDICATION_TYPE_RAW                = 0x0,
    HIF_GENERIC_INDICATION_TYPE_STRING             = 0x1,
    HIF_GENERIC_INDICATION_TYPE_RX_STATS           = 0x2,
    HIF_GENERIC_INDICATION_TYPE_TX_POWER_LOOP_INFO = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_rx_stats {
    pub nb_rx_frame: __le32,
    pub nb_crc_frame: __le32,
    pub per_total: __le32,
    pub throughput: __le32,
    pub nb_rx_by_rate: [__le32; API_RATE_NUM_ENTRIES],
    pub per: [__le16; API_RATE_NUM_ENTRIES],
    pub /: *mut *mut __le16 snr[API_RATE_NUM_ENTRIES]; / signed value,
    pub /: *mut *mut __le16 rssi[API_RATE_NUM_ENTRIES]; / signed value,
    pub /: *mut *mut __le16 cfo[API_RATE_NUM_ENTRIES]; / signed value,
    pub date: __le32,
    pub pwr_clk_freq: __le32,
    pub is_ext_pwr_clk: u8,
    pub current_temp: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_tx_power_loop_info {
    pub tx_gain_dig: __le16,
    pub tx_gain_pa: __le16,
    pub /: *mut *mut __le16 target_pout; / signed value,
    pub /: *mut *mut __le16 p_estimation; / signed value,
    pub vpdet: __le16,
    pub measurement_index: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_generic {
    pub type: __le32,
    pub rx_stats: wfx_hif_rx_stats,
    pub tx_power_loop_info: wfx_hif_tx_power_loop_info,
    pub data: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_error {
    HIF_ERROR_FIRMWARE_ROLLBACK           = 0x00,
    HIF_ERROR_FIRMWARE_DEBUG_ENABLED      = 0x01,
    HIF_ERROR_SLK_OUTDATED_SESSION_KEY    = 0x02,
    HIF_ERROR_SLK_SESSION_KEY             = 0x03,
    HIF_ERROR_OOR_VOLTAGE                 = 0x04,
    HIF_ERROR_PDS_PAYLOAD                 = 0x05,
    HIF_ERROR_OOR_TEMPERATURE             = 0x06,
    HIF_ERROR_SLK_REQ_DURING_KEY_EXCHANGE = 0x07,
    HIF_ERROR_SLK_MULTI_TX_UNSUPPORTED    = 0x08,
    HIF_ERROR_SLK_OVERFLOW                = 0x09,
    HIF_ERROR_SLK_DECRYPTION              = 0x0a,
    HIF_ERROR_SLK_WRONG_ENCRYPTION_STATE  = 0x0b,
    HIF_ERROR_HIF_BUS_FREQUENCY_TOO_LOW   = 0x0c,
    HIF_ERROR_HIF_RX_DATA_TOO_LARGE       = 0x0e,
    HIF_ERROR_HIF_TX_QUEUE_FULL           = 0x0d,
    HIF_ERROR_HIF_BUS                     = 0x0f,
    HIF_ERROR_PDS_TESTFEATURE             = 0x10,
    HIF_ERROR_SLK_UNCONFIGURED            = 0x11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_error {
    pub type: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif_ind_exception {
    pub type: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wfx_hif_secure_link_state {
    SEC_LINK_UNAVAILABLE = 0x0,
    SEC_LINK_RESERVED    = 0x1,
    SEC_LINK_EVAL        = 0x2,
    SEC_LINK_ENFORCED    = 0x3
}
