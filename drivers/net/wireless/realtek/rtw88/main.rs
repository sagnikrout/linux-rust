//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/main.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2018-2019  Realtek Corporation
//

pub const RTW_MAX_MAC_ID_NUM: c_int = 32;
pub const RTW_MAX_SEC_CAM_NUM: c_int = 32;
pub const MAX_PG_CAM_BACKUP_NUM: c_int = 8;
pub const RTW_SCAN_MAX_SSIDS: c_int = 4;
pub const RTW_MAX_PATTERN_NUM: c_int = 12;
pub const RTW_MAX_PATTERN_MASK_SIZE: c_int = 16;
pub const RTW_MAX_PATTERN_SIZE: c_int = 128;

pub const RFREG_MASK: c_uint = 0xfffff;
pub const INV_RF_DATA: c_uint = 0xffffffff;
pub const TX_PAGE_SIZE_SHIFT: c_int = 7;

pub const RTW_CHANNEL_WIDTH_MAX: c_int = 3;
pub const RTW_RF_PATH_MAX: c_int = 4;
pub const HW_FEATURE_LEN: c_int = 13;

pub const RTW_MAX_CHANNEL_NUM_2G: c_int = 14;
pub const RTW_MAX_CHANNEL_NUM_5G: c_int = 49;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_hci_type {
    RTW_HCI_TYPE_PCIE,
    RTW_HCI_TYPE_USB,
    RTW_HCI_TYPE_SDIO,

    RTW_HCI_TYPE_UNDEFINE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_hci {
    pub ops: *const rtw_hci_ops,
    pub type: rtw_hci_type,
    pub rpwm_addr: u32,
    pub cpwm_addr: u32,
    pub bulkout_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_supported_band {
    RTW_BAND_2G = BIT(NL80211_BAND_2GHZ),
    RTW_BAND_5G = BIT(NL80211_BAND_5GHZ),
    RTW_BAND_60G = BIT(NL80211_BAND_60GHZ),
}

// now, support up to 80M bw

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_bandwidth {
    RTW_CHANNEL_WIDTH_20	= 0,
    RTW_CHANNEL_WIDTH_40	= 1,
    RTW_CHANNEL_WIDTH_80	= 2,
    RTW_CHANNEL_WIDTH_160	= 3,
    RTW_CHANNEL_WIDTH_80_80	= 4,
    RTW_CHANNEL_WIDTH_5	= 5,
    RTW_CHANNEL_WIDTH_10	= 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_sc_offset {
    RTW_SC_DONT_CARE	= 0,
    RTW_SC_20_UPPER		= 1,
    RTW_SC_20_LOWER		= 2,
    RTW_SC_20_UPMOST	= 3,
    RTW_SC_20_LOWEST	= 4,
    RTW_SC_40_UPPER		= 9,
    RTW_SC_40_LOWER		= 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_net_type {
    RTW_NET_NO_LINK		= 0,
    RTW_NET_AD_HOC		= 1,
    RTW_NET_MGD_LINKED	= 2,
    RTW_NET_AP_MODE		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_rf_type {
    RF_1T1R			= 0,
    RF_1T2R			= 1,
    RF_2T2R			= 2,
    RF_2T3R			= 3,
    RF_2T4R			= 4,
    RF_3T3R			= 5,
    RF_3T4R			= 6,
    RF_4T4R			= 7,
    RF_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_rf_path {
    RF_PATH_A = 0,
    RF_PATH_B = 1,
    RF_PATH_C = 2,
    RF_PATH_D = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_bb_path {
    BB_PATH_A = BIT(0),
    BB_PATH_B = BIT(1),
    BB_PATH_C = BIT(2),
    BB_PATH_D = BIT(3),

    BB_PATH_AB = (BB_PATH_A | BB_PATH_B),
    BB_PATH_AC = (BB_PATH_A | BB_PATH_C),
    BB_PATH_AD = (BB_PATH_A | BB_PATH_D),
    BB_PATH_BC = (BB_PATH_B | BB_PATH_C),
    BB_PATH_BD = (BB_PATH_B | BB_PATH_D),
    BB_PATH_CD = (BB_PATH_C | BB_PATH_D),

    BB_PATH_ABC = (BB_PATH_A | BB_PATH_B | BB_PATH_C),
    BB_PATH_ABD = (BB_PATH_A | BB_PATH_B | BB_PATH_D),
    BB_PATH_ACD = (BB_PATH_A | BB_PATH_C | BB_PATH_D),
    BB_PATH_BCD = (BB_PATH_B | BB_PATH_C | BB_PATH_D),

    BB_PATH_ABCD = (BB_PATH_A | BB_PATH_B | BB_PATH_C | BB_PATH_D),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_rate_section {
    RTW_RATE_SECTION_CCK = 0,
    RTW_RATE_SECTION_OFDM,
    RTW_RATE_SECTION_HT_1S,
    RTW_RATE_SECTION_HT_2S,
    RTW_RATE_SECTION_VHT_1S,
    RTW_RATE_SECTION_VHT_2S,
    __RTW_RATE_SECTION_2SS_MAX = RTW_RATE_SECTION_VHT_2S,
    RTW_RATE_SECTION_HT_3S,
    RTW_RATE_SECTION_HT_4S,
    RTW_RATE_SECTION_VHT_3S,
    RTW_RATE_SECTION_VHT_4S,

// keep last
    RTW_RATE_SECTION_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_wireless_set {
    WIRELESS_CCK	= 0x00000001,
    WIRELESS_OFDM	= 0x00000002,
    WIRELESS_HT	= 0x00000004,
    WIRELESS_VHT	= 0x00000008,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_chip_type {
    RTW_CHIP_TYPE_8822B,
    RTW_CHIP_TYPE_8822C,
    RTW_CHIP_TYPE_8723D,
    RTW_CHIP_TYPE_8821C,
    RTW_CHIP_TYPE_8703B,
    RTW_CHIP_TYPE_8821A,
    RTW_CHIP_TYPE_8812A,
    RTW_CHIP_TYPE_8814A,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_tx_queue_type {
// the order of AC queues matters
    RTW_TX_QUEUE_BK = 0x0,
    RTW_TX_QUEUE_BE = 0x1,
    RTW_TX_QUEUE_VI = 0x2,
    RTW_TX_QUEUE_VO = 0x3,

    RTW_TX_QUEUE_BCN = 0x4,
    RTW_TX_QUEUE_MGMT = 0x5,
    RTW_TX_QUEUE_HI0 = 0x6,
    RTW_TX_QUEUE_H2C = 0x7,
// keep it last
    RTK_MAX_TX_QUEUE_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_rx_queue_type {
    RTW_RX_QUEUE_MPDU = 0x0,
    RTW_RX_QUEUE_C2H = 0x1,
// keep it last
    RTK_MAX_RX_QUEUE_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_fw_type {
    RTW_NORMAL_FW = 0x0,
    RTW_WOWLAN_FW = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_rate_index {
    RTW_RATEID_BGN_40M_2SS	= 0,
    RTW_RATEID_BGN_40M_1SS	= 1,
    RTW_RATEID_BGN_20M_2SS	= 2,
    RTW_RATEID_BGN_20M_1SS	= 3,
    RTW_RATEID_GN_N2SS	= 4,
    RTW_RATEID_GN_N1SS	= 5,
    RTW_RATEID_BG		= 6,
    RTW_RATEID_G		= 7,
    RTW_RATEID_B_20M	= 8,
    RTW_RATEID_ARFR0_AC_2SS	= 9,
    RTW_RATEID_ARFR1_AC_1SS	= 10,
    RTW_RATEID_ARFR2_AC_2G_1SS = 11,
    RTW_RATEID_ARFR3_AC_2G_2SS = 12,
    RTW_RATEID_ARFR4_AC_3SS	= 13,
    RTW_RATEID_ARFR5_N_3SS	= 14,
    RTW_RATEID_ARFR7_N_4SS	= 15,
    RTW_RATEID_ARFR6_AC_4SS	= 16
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_trx_desc_rate {
    DESC_RATE1M	= 0x00,
    DESC_RATE2M	= 0x01,
    DESC_RATE5_5M	= 0x02,
    DESC_RATE11M	= 0x03,

    DESC_RATE6M	= 0x04,
    DESC_RATE9M	= 0x05,
    DESC_RATE12M	= 0x06,
    DESC_RATE18M	= 0x07,
    DESC_RATE24M	= 0x08,
    DESC_RATE36M	= 0x09,
    DESC_RATE48M	= 0x0a,
    DESC_RATE54M	= 0x0b,

    DESC_RATEMCS0	= 0x0c,
    DESC_RATEMCS1	= 0x0d,
    DESC_RATEMCS2	= 0x0e,
    DESC_RATEMCS3	= 0x0f,
    DESC_RATEMCS4	= 0x10,
    DESC_RATEMCS5	= 0x11,
    DESC_RATEMCS6	= 0x12,
    DESC_RATEMCS7	= 0x13,
    DESC_RATEMCS8	= 0x14,
    DESC_RATEMCS9	= 0x15,
    DESC_RATEMCS10	= 0x16,
    DESC_RATEMCS11	= 0x17,
    DESC_RATEMCS12	= 0x18,
    DESC_RATEMCS13	= 0x19,
    DESC_RATEMCS14	= 0x1a,
    DESC_RATEMCS15	= 0x1b,
    DESC_RATEMCS16	= 0x1c,
    DESC_RATEMCS17	= 0x1d,
    DESC_RATEMCS18	= 0x1e,
    DESC_RATEMCS19	= 0x1f,
    DESC_RATEMCS20	= 0x20,
    DESC_RATEMCS21	= 0x21,
    DESC_RATEMCS22	= 0x22,
    DESC_RATEMCS23	= 0x23,
    DESC_RATEMCS24	= 0x24,
    DESC_RATEMCS25	= 0x25,
    DESC_RATEMCS26	= 0x26,
    DESC_RATEMCS27	= 0x27,
    DESC_RATEMCS28	= 0x28,
    DESC_RATEMCS29	= 0x29,
    DESC_RATEMCS30	= 0x2a,
    DESC_RATEMCS31	= 0x2b,

    DESC_RATEVHT1SS_MCS0	= 0x2c,
    DESC_RATEVHT1SS_MCS1	= 0x2d,
    DESC_RATEVHT1SS_MCS2	= 0x2e,
    DESC_RATEVHT1SS_MCS3	= 0x2f,
    DESC_RATEVHT1SS_MCS4	= 0x30,
    DESC_RATEVHT1SS_MCS5	= 0x31,
    DESC_RATEVHT1SS_MCS6	= 0x32,
    DESC_RATEVHT1SS_MCS7	= 0x33,
    DESC_RATEVHT1SS_MCS8	= 0x34,
    DESC_RATEVHT1SS_MCS9	= 0x35,

    DESC_RATEVHT2SS_MCS0	= 0x36,
    DESC_RATEVHT2SS_MCS1	= 0x37,
    DESC_RATEVHT2SS_MCS2	= 0x38,
    DESC_RATEVHT2SS_MCS3	= 0x39,
    DESC_RATEVHT2SS_MCS4	= 0x3a,
    DESC_RATEVHT2SS_MCS5	= 0x3b,
    DESC_RATEVHT2SS_MCS6	= 0x3c,
    DESC_RATEVHT2SS_MCS7	= 0x3d,
    DESC_RATEVHT2SS_MCS8	= 0x3e,
    DESC_RATEVHT2SS_MCS9	= 0x3f,

    DESC_RATEVHT3SS_MCS0	= 0x40,
    DESC_RATEVHT3SS_MCS1	= 0x41,
    DESC_RATEVHT3SS_MCS2	= 0x42,
    DESC_RATEVHT3SS_MCS3	= 0x43,
    DESC_RATEVHT3SS_MCS4	= 0x44,
    DESC_RATEVHT3SS_MCS5	= 0x45,
    DESC_RATEVHT3SS_MCS6	= 0x46,
    DESC_RATEVHT3SS_MCS7	= 0x47,
    DESC_RATEVHT3SS_MCS8	= 0x48,
    DESC_RATEVHT3SS_MCS9	= 0x49,

    DESC_RATEVHT4SS_MCS0	= 0x4a,
    DESC_RATEVHT4SS_MCS1	= 0x4b,
    DESC_RATEVHT4SS_MCS2	= 0x4c,
    DESC_RATEVHT4SS_MCS3	= 0x4d,
    DESC_RATEVHT4SS_MCS4	= 0x4e,
    DESC_RATEVHT4SS_MCS5	= 0x4f,
    DESC_RATEVHT4SS_MCS6	= 0x50,
    DESC_RATEVHT4SS_MCS7	= 0x51,
    DESC_RATEVHT4SS_MCS8	= 0x52,
    DESC_RATEVHT4SS_MCS9	= 0x53,

    DESC_RATE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_regulatory_domains {
    RTW_REGD_FCC		= 0,
    RTW_REGD_MKK		= 1,
    RTW_REGD_ETSI		= 2,
    RTW_REGD_IC		= 3,
    RTW_REGD_KCC		= 4,
    RTW_REGD_ACMA		= 5,
    RTW_REGD_CHILE		= 6,
    RTW_REGD_UKRAINE	= 7,
    RTW_REGD_MEXICO		= 8,
    RTW_REGD_CN		= 9,
    RTW_REGD_QATAR		= 10,
    RTW_REGD_UK		= 11,

    RTW_REGD_WW,
    RTW_REGD_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_txq_flags {
    RTW_TXQ_AMPDU,
    RTW_TXQ_BLOCK_BA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_flags {
    RTW_FLAG_RUNNING,
    RTW_FLAG_FW_RUNNING,
    RTW_FLAG_SCANNING,
    RTW_FLAG_POWERON,
    RTW_FLAG_LEISURE_PS,
    RTW_FLAG_LEISURE_PS_DEEP,
    RTW_FLAG_DIG_DISABLE,
    RTW_FLAG_BUSY_TRAFFIC,
    RTW_FLAG_WOWLAN,
    RTW_FLAG_RESTARTING,
    RTW_FLAG_RESTART_TRIGGERING,
    RTW_FLAG_FORCE_LOWEST_RATE,

    NUM_OF_RTW_FLAGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_evm {
    RTW_EVM_OFDM = 0,
    RTW_EVM_1SS,
    RTW_EVM_2SS_A,
    RTW_EVM_2SS_B,
    RTW_EVM_3SS_A,
    RTW_EVM_3SS_B,
    RTW_EVM_3SS_C,
// keep it last
    RTW_EVM_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_snr {
    RTW_SNR_OFDM_A = 0,
    RTW_SNR_OFDM_B,
    RTW_SNR_OFDM_C,
    RTW_SNR_OFDM_D,
    RTW_SNR_1SS_A,
    RTW_SNR_1SS_B,
    RTW_SNR_1SS_C,
    RTW_SNR_1SS_D,
    RTW_SNR_2SS_A,
    RTW_SNR_2SS_B,
    RTW_SNR_2SS_C,
    RTW_SNR_2SS_D,
    RTW_SNR_3SS_A,
    RTW_SNR_3SS_B,
    RTW_SNR_3SS_C,
    RTW_SNR_3SS_D,
// keep it last
    RTW_SNR_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_port {
    RTW_PORT_0 = 0,
    RTW_PORT_1 = 1,
    RTW_PORT_2 = 2,
    RTW_PORT_3 = 3,
    RTW_PORT_4 = 4,
    RTW_PORT_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_wow_flags {
    RTW_WOW_FLAG_EN_MAGIC_PKT,
    RTW_WOW_FLAG_EN_REKEY_PKT,
    RTW_WOW_FLAG_EN_DISCONNECT,

// keep it last
    RTW_WOW_FLAG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_quirk_dis_caps {
    QUIRK_DIS_CAP_PCI_ASPM,
    QUIRK_DIS_CAP_LPS_DEEP,
}

// the power index is represented by differences, which cck-1s & ht40-1s are
// the base values, so for 1s's differences, there are only ht20 & ofdm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_2g_1s_pwr_idx_diff {

    pub ofdm:4: i8,
    pub bw20:4: i8,

    pub bw20:4: i8,
    pub ofdm:4: i8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_2g_ns_pwr_idx_diff {

    pub bw20:4: i8,
    pub bw40:4: i8,
    pub cck:4: i8,
    pub ofdm:4: i8,

    pub ofdm:4: i8,
    pub cck:4: i8,
    pub bw40:4: i8,
    pub bw20:4: i8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_2g_txpwr_idx {
    pub cck_base: [u8; 6],
    pub bw40_base: [u8; 5],
    pub ht_1s_diff: rtw_2g_1s_pwr_idx_diff,
    pub ht_2s_diff: rtw_2g_ns_pwr_idx_diff,
    pub ht_3s_diff: rtw_2g_ns_pwr_idx_diff,
    pub ht_4s_diff: rtw_2g_ns_pwr_idx_diff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_5g_ht_1s_pwr_idx_diff {

    pub ofdm:4: i8,
    pub bw20:4: i8,

    pub bw20:4: i8,
    pub ofdm:4: i8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_5g_ht_ns_pwr_idx_diff {

    pub bw20:4: i8,
    pub bw40:4: i8,

    pub bw40:4: i8,
    pub bw20:4: i8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_5g_ofdm_ns_pwr_idx_diff {

    pub ofdm_3s:4: i8,
    pub ofdm_2s:4: i8,
    pub ofdm_4s:4: i8,
    pub res:4: i8,

    pub res:4: i8,
    pub ofdm_4s:4: i8,
    pub ofdm_2s:4: i8,
    pub ofdm_3s:4: i8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_5g_vht_ns_pwr_idx_diff {

    pub bw160:4: i8,
    pub bw80:4: i8,

    pub bw80:4: i8,
    pub bw160:4: i8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_5g_txpwr_idx {
    pub bw40_base: [u8; 14],
    pub ht_1s_diff: rtw_5g_ht_1s_pwr_idx_diff,
    pub ht_2s_diff: rtw_5g_ht_ns_pwr_idx_diff,
    pub ht_3s_diff: rtw_5g_ht_ns_pwr_idx_diff,
    pub ht_4s_diff: rtw_5g_ht_ns_pwr_idx_diff,
    pub ofdm_diff: rtw_5g_ofdm_ns_pwr_idx_diff,
    pub vht_1s_diff: rtw_5g_vht_ns_pwr_idx_diff,
    pub vht_2s_diff: rtw_5g_vht_ns_pwr_idx_diff,
    pub vht_3s_diff: rtw_5g_vht_ns_pwr_idx_diff,
    pub vht_4s_diff: rtw_5g_vht_ns_pwr_idx_diff,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_txpwr_idx {
    pub pwr_idx_2g: rtw_2g_txpwr_idx,
    pub pwr_idx_5g: rtw_5g_txpwr_idx,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_channel_params {
    pub center_chan: u8,
    pub primary_chan: u8,
    pub bandwidth: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_hw_reg {
    pub addr: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_hw_reg_desc {
    pub addr: u32,
    pub mask: u32,
    pub desc: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_ltecoex_addr {
    pub ctrl: u32,
    pub wdata: u32,
    pub rdata: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_reg_domain {
    pub addr: u32,
    pub mask: u32,
pub const RTW_REG_DOMAIN_MAC32: c_int = 0;
pub const RTW_REG_DOMAIN_MAC16: c_int = 1;
pub const RTW_REG_DOMAIN_MAC8: c_int = 2;
pub const RTW_REG_DOMAIN_RF_A: c_int = 3;
pub const RTW_REG_DOMAIN_RF_B: c_int = 4;
pub const RTW_REG_DOMAIN_NL: c_uint = 0xFF;
    pub domain: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_rf_sipi_addr {
    pub hssi_1: u32,
    pub hssi_2: u32,
    pub lssi_read: u32,
    pub lssi_read_pi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_hw_reg_offset {
    pub hw_reg: rtw_hw_reg,
    pub offset: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_backup_info {
    pub len: u8,
    pub reg: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_vif_port_set {
    PORT_SET_MAC_ADDR	= BIT(0),
    PORT_SET_BSSID		= BIT(1),
    PORT_SET_NET_TYPE	= BIT(2),
    PORT_SET_AID		= BIT(3),
    PORT_SET_BCN_CTRL	= BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_vif_port {
    pub mac_addr: rtw_hw_reg,
    pub bssid: rtw_hw_reg,
    pub net_type: rtw_hw_reg,
    pub aid: rtw_hw_reg,
    pub bcn_ctrl: rtw_hw_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_tx_pkt_info {
    pub tx_pkt_size: u32,
    pub offset: u8,
    pub pkt_offset: u8,
    pub tim_offset: u8,
    pub mac_id: u8,
    pub rate_id: u8,
    pub rate: u8,
    pub qsel: u8,
    pub bw: u8,
    pub sec_type: u8,
    pub sn: u8,
    pub ampdu_en: bool,
    pub ampdu_factor: u8,
    pub ampdu_density: u8,
    pub seq: u16,
    pub stbc: bool,
    pub ldpc: bool,
    pub dis_rate_fallback: bool,
    pub bmc: bool,
    pub use_rate: bool,
    pub ls: bool,
    pub fs: bool,
    pub short_gi: bool,
    pub report: bool,
    pub rts: bool,
    pub dis_qselseq: bool,
    pub en_hwseq: bool,
    pub hw_ssn_sel: u8,
    pub nav_use_hdr: bool,
    pub bt_null: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_rx_pkt_stat {
    pub phy_status: bool,
    pub icv_err: bool,
    pub crc_err: bool,
    pub decrypted: bool,
    pub is_c2h: bool,
    pub channel_invalid: bool,
    pub signal_power: i32,
    pub pkt_len: u16,
    pub bw: u8,
    pub drv_info_sz: u8,
    pub shift: u8,
    pub rate: u8,
    pub mac_id: u8,
    pub cam_id: u8,
    pub ppdu_cnt: u8,
    pub tsf_low: u32,
    pub rx_power: [i8; RTW_RF_PATH_MAX],
    pub rssi: u8,
    pub rxsc: u8,
    pub rx_snr: [i8; RTW_RF_PATH_MAX],
    pub rx_evm: [u8; RTW_RF_PATH_MAX],
    pub cfo_tail: [i8; RTW_RF_PATH_MAX],
    pub freq: u16,
    pub band: u8,
    pub si: *mut rtw_sta_info,
    pub vif: *mut ieee80211_vif,
    pub hdr: *mut ieee80211_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_traffic_stats {
// units in bytes
    pub tx_unicast: u64,
    pub rx_unicast: u64,
// count for packets
    pub tx_cnt: u64,
    pub rx_cnt: u64,
// units in Mbps
    pub tx_throughput: u32,
    pub rx_throughput: u32,
    pub tx_ewma_tp: ewma_tp,
    pub rx_ewma_tp: ewma_tp,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_lps_mode {
    RTW_MODE_ACTIVE	= 0,
    RTW_MODE_LPS	= 1,
    RTW_MODE_WMM_PS	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_lps_deep_mode {
    LPS_DEEP_MODE_NONE	= 0,
    LPS_DEEP_MODE_LCLK	= 1,
    LPS_DEEP_MODE_PG	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_pwr_state {
    RTW_RF_OFF	= 0x0,
    RTW_RF_ON	= 0x4,
    RTW_ALL_ON	= 0xc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_lps_conf {
    pub mode: rtw_lps_mode,
    pub deep_mode: rtw_lps_deep_mode,
    pub wow_deep_mode: rtw_lps_deep_mode,
    pub state: rtw_pwr_state,
    pub awake_interval: u8,
    pub rlbm: u8,
    pub smart_ps: u8,
    pub port_id: u8,
    pub sec_cam_backup: bool,
    pub pattern_cam_backup: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_hw_key_type {
    RTW_CAM_NONE	= 0,
    RTW_CAM_WEP40	= 1,
    RTW_CAM_TKIP	= 2,
    RTW_CAM_AES	= 4,
    RTW_CAM_WEP104	= 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_cam_entry {
    pub valid: bool,
    pub group: bool,
    pub addr: [u8; ETH_ALEN],
    pub hw_key_type: u8,
    pub key: *mut ieee80211_key_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_sec_desc {
// search strategy
    pub default_key_search: bool,
    pub total_cam_num: u32,
    pub cam_table: [rtw_cam_entry; RTW_MAX_SEC_CAM_NUM],
    pub RTW_MAX_SEC_CAM_NUM): DECLARE_BITMAP(cam_map,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_tx_report {
// protect the tx report queue
    pub q_lock: spinlock_t,
    pub queue: sk_buff_head,
    pub sn: core::sync::atomic::AtomicI32,
    pub purge_timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_ra_report {
    pub txrate: rate_info,
    pub bit_rate: u32,
    pub desc_rate: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_txq {
    pub list: list_head,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_sta_info {
    pub rtwdev: *mut rtw_dev,
    pub sta: *mut ieee80211_sta,
    pub vif: *mut ieee80211_vif,
    pub avg_rssi: ewma_rssi,
    pub rssi_level: u8,
    pub mac_id: u8,
    pub rate_id: u8,
    pub bw_mode: rtw_bandwidth,
    pub stbc_en:2: u8,
    pub ldpc_en:2: u8,
    pub sgi_enable: bool,
    pub vht_enable: bool,
    pub init_ra_lv: u8,
    pub ra_mask: u64,
    pub IEEE80211_NUM_TIDS): DECLARE_BITMAP(tid_ba,,
    pub ra_report: rtw_ra_report,
    pub use_cfg_mask: bool,
    pub mask: *mut cfg80211_bitrate_mask,
    pub rc_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_bfee_role {
    RTW_BFEE_NONE,
    RTW_BFEE_SU,
    RTW_BFEE_MU
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_bfee {
    pub role: rtw_bfee_role,
    pub p_aid: u16,
    pub g_id: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub sound_dim: u8,
// SU-MIMO
    pub su_reg_index: u8,
// MU-MIMO
    pub aid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_bf_info {
    pub bfer_mu_cnt: u8,
    pub bfer_su_cnt: u8,
    pub 2): DECLARE_BITMAP(bfer_su_reg_maping,,
    pub cur_csi_rpt_rate: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_vif {
    pub net_type: rtw_net_type,
    pub aid: u16,
    pub mac_id: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub port: u8,
    pub bcn_ctrl: u8,
    pub rsvd_page_list: list_head,
    pub tx_params: [ieee80211_tx_queue_params; IEEE80211_NUM_ACS],
    pub conf: *const rtw_vif_port,
    pub scan_req: *mut cfg80211_scan_request,
    pub scan_ies: *mut ieee80211_scan_ies,
    pub stats: rtw_traffic_stats,
    pub bfee: rtw_bfee,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_regulatory {
    pub __nonstring: char alpha2[2],
    pub txpwr_regd_2g: u8,
    pub txpwr_regd_5g: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_regd_state {
    RTW_REGD_STATE_WORLDWIDE,
    RTW_REGD_STATE_PROGRAMMED,
    RTW_REGD_STATE_SETTING,

    RTW_REGD_STATE_NR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_regd {
    pub state: rtw_regd_state,
    pub regulatory: *const rtw_regulatory,
    pub dfs_region: nl80211_dfs_regions,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_chip_ops {
    pub rtwdev): *mut *mut int (power_on)(struct rtw_dev,
    pub rtwdev): *mut *mut void (power_off)(struct rtw_dev,
    pub rtwdev): *mut *mut int (mac_init)(struct rtw_dev,
    pub rtwdev): *mut *mut int (mac_postinit)(struct rtw_dev,
    pub rtwdev): *mut *mut int (dump_fw_crash)(struct rtw_dev,
    pub rtwdev): *mut *mut void (shutdown)(struct rtw_dev,
    pub map): *mut *mut *mut int (read_efuse)(struct rtw_dev rtwdev, u8,
    pub rtwdev): *mut *mut void (phy_set_param)(struct rtw_dev,
    pub primary_chan_idx): u8 bandwidth, u8,
    pub pkt_stat): *mut rtw_rx_pkt_stat,
    pub mask): u32 addr, u32,
    pub data): u32 addr, u32 mask, u32,
    pub rtwdev): *mut *mut void (set_tx_power_index)(struct rtw_dev,
    pub size): u32,
    pub antenna_rx): u32,
    pub enable): *mut *mut *mut void (cfg_ldo25)(struct rtw_dev rtwdev, bool,
    pub enable): *mut *mut *mut void (efuse_grant)(struct rtw_dev rtwdev, bool,
    pub factor): *mut *mut *mut void (set_ampdu_factor)(struct rtw_dev rtwdev, u8,
    pub rtwdev): *mut *mut void (false_alarm_statistics)(struct rtw_dev,
    pub rtwdev): *mut *mut void (phy_calibration)(struct rtw_dev,
    pub rtwdev): *mut *mut void (dpk_track)(struct rtw_dev,
    pub level): *mut *mut *mut void (cck_pd_set)(struct rtw_dev rtwdev, u8,
    pub rtwdev): *mut *mut void (pwr_track)(struct rtw_dev,
    pub enable): *mut *mut rtw_bfee bfee, bool,
    pub conf): *mut ieee80211_bss_conf,
    pub new_rate): *mut u8 fixrate_en, u8,
    pub rtwdev): *mut *mut void (adaptivity_init)(struct rtw_dev,
    pub rtwdev): *mut *mut void (adaptivity)(struct rtw_dev,
    pub rtwdev): *mut *mut void (cfo_init)(struct rtw_dev,
    pub rtwdev): *mut *mut void (cfo_track)(struct rtw_dev,
    pub is_tx2_path): bool,
    pub is_tx2_path): u8 rx_path, bool,
    pub brightness): *mut *mut *mut void (led_set)(struct led_classdev led, enum led_brightness,
// for USB/SDIO only
    pub txdesc): *mut u8,
// for coex
    pub rtwdev): *mut *mut void (coex_set_init)(struct rtw_dev,
    pub pos_type): u8 ctrl_type, u8,
    pub rtwdev): *mut *mut void (coex_set_gnt_fix)(struct rtw_dev,
    pub rtwdev): *mut *mut void (coex_set_gnt_debug)(struct rtw_dev,
    pub rtwdev): *mut *mut void (coex_set_rfe_type)(struct rtw_dev,
    pub wl_pwr): *mut *mut *mut void (coex_set_wl_tx_power)(struct rtw_dev rtwdev, u8,
    pub low_gain): *mut *mut *mut void (coex_set_wl_rx_gain)(struct rtw_dev rtwdev, bool,
}

pub const RTW_PWR_POLLING_CNT: c_int = 20000;
pub const RTW_PWR_CMD_READ: c_uint = 0x00;
pub const RTW_PWR_CMD_WRITE: c_uint = 0x01;
pub const RTW_PWR_CMD_POLLING: c_uint = 0x02;
pub const RTW_PWR_CMD_DELAY: c_uint = 0x03;
pub const RTW_PWR_CMD_END: c_uint = 0x04;
// define the base address of each block
pub const RTW_PWR_ADDR_MAC: c_uint = 0x00;
pub const RTW_PWR_ADDR_USB: c_uint = 0x01;
pub const RTW_PWR_ADDR_PCIE: c_uint = 0x02;
pub const RTW_PWR_ADDR_SDIO: c_uint = 0x03;

pub const RTW_PWR_CUT_ALL_MSK: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_pwr_seq_cmd_delay_unit {
    RTW_PWR_DELAY_US,
    RTW_PWR_DELAY_MS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pwr_seq_cmd {
    pub offset: u16,
    pub cut_mask: u8,
    pub intf_mask: u8,
    pub base:4: u8,
    pub cmd:4: u8,
    pub mask: u8,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_chip_ver {
    RTW_CHIP_VER_CUT_A = 0x00,
    RTW_CHIP_VER_CUT_B = 0x01,
    RTW_CHIP_VER_CUT_C = 0x02,
    RTW_CHIP_VER_CUT_D = 0x03,
    RTW_CHIP_VER_CUT_E = 0x04,
    RTW_CHIP_VER_CUT_F = 0x05,
    RTW_CHIP_VER_CUT_G = 0x06,
}

pub const RTW_INTF_PHY_PLATFORM_ALL: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_intf_phy_cut {
    RTW_INTF_PHY_CUT_A = BIT(0),
    RTW_INTF_PHY_CUT_B = BIT(1),
    RTW_INTF_PHY_CUT_C = BIT(2),
    RTW_INTF_PHY_CUT_D = BIT(3),
    RTW_INTF_PHY_CUT_E = BIT(4),
    RTW_INTF_PHY_CUT_F = BIT(5),
    RTW_INTF_PHY_CUT_G = BIT(6),
    RTW_INTF_PHY_CUT_ALL = 0xFFFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_ip_sel {
    RTW_IP_SEL_PHY = 0,
    RTW_IP_SEL_MAC = 1,
    RTW_IP_SEL_DBI = 2,

    RTW_IP_SEL_UNDEF = 0xFFFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_pq_map_id {
    RTW_PQ_MAP_VO = 0x0,
    RTW_PQ_MAP_VI = 0x1,
    RTW_PQ_MAP_BE = 0x2,
    RTW_PQ_MAP_BK = 0x3,
    RTW_PQ_MAP_MG = 0x4,
    RTW_PQ_MAP_HI = 0x5,
    RTW_PQ_MAP_NUM = 0x6,

    RTW_PQ_MAP_UNDEF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_dma_mapping {
    RTW_DMA_MAPPING_EXTRA	= 0,
    RTW_DMA_MAPPING_LOW	= 1,
    RTW_DMA_MAPPING_NORMAL	= 2,
    RTW_DMA_MAPPING_HIGH	= 3,

    RTW_DMA_MAPPING_MAX,
    RTW_DMA_MAPPING_UNDEF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_rqpn {
    pub dma_map_vo: rtw_dma_mapping,
    pub dma_map_vi: rtw_dma_mapping,
    pub dma_map_be: rtw_dma_mapping,
    pub dma_map_bk: rtw_dma_mapping,
    pub dma_map_mg: rtw_dma_mapping,
    pub dma_map_hi: rtw_dma_mapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_prioq_addr {
    pub rsvd: u32,
    pub avail: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_prioq_addrs {
    pub prio: [rtw_prioq_addr; RTW_DMA_MAPPING_MAX],
    pub wsize: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_page_table {
    pub hq_num: u16,
    pub nq_num: u16,
    pub lq_num: u16,
    pub exq_num: u16,
    pub gapq_num: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_intf_phy_para {
    pub offset: u16,
    pub value: u16,
    pub ip_sel: u16,
    pub cut_mask: u16,
    pub platform: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_wow_pattern {
    pub crc: u16,
    pub type: u8,
    pub valid: u8,
    pub mask: [u8; RTW_MAX_PATTERN_MASK_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pno_request {
    pub inited: bool,
    pub match_set_cnt: u32,
    pub match_sets: *mut cfg80211_match_set,
    pub channel_cnt: u8,
    pub channels: *mut ieee80211_channel,
    pub scan_plan: cfg80211_sched_scan_plan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_wow_param {
    pub wow_vif: *mut ieee80211_vif,
    pub RTW_WOW_FLAG_MAX): DECLARE_BITMAP(flags,,
    pub txpause: u8,
    pub pattern_cnt: u8,
    pub patterns: [rtw_wow_pattern; RTW_MAX_PATTERN_NUM],
    pub ips_enabled: bool,
    pub pno_req: rtw_pno_request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_intf_phy_para_table {
    pub usb2_para: *const rtw_intf_phy_para,
    pub usb3_para: *const rtw_intf_phy_para,
    pub gen1_para: *const rtw_intf_phy_para,
    pub gen2_para: *const rtw_intf_phy_para,
    pub n_usb2_para: u8,
    pub n_usb3_para: u8,
    pub n_gen1_para: u8,
    pub n_gen2_para: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_table {
    pub data: *const c_void,
    pub size: u32,
    pub tbl): *const *const *const void (parse)(struct rtw_dev rtwdev, struct rtw_table,
    pub data): u32 addr, u32,
    pub rf_path: rtw_rf_path,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_rfe_fem {
    RTW_RFE_IFEM,
    RTW_RFE_EFEM,
    RTW_RFE_IFEM2G_EFEM5G,
    RTW_RFE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_rfe_def {
    pub phy_pg_tbl: *const rtw_table,
    pub txpwr_lmt_tbl: *const rtw_table,
    pub pwr_track_tbl: *const rtw_pwr_track_tbl,
    pub agc_btg_tbl: *const rtw_table,
}

pub const RTW_PWR_TRK_5G_1: c_int = 0;
pub const RTW_PWR_TRK_5G_2: c_int = 1;
pub const RTW_PWR_TRK_5G_3: c_int = 2;
pub const RTW_PWR_TRK_5G_NUM: c_int = 3;
pub const RTW_PWR_TRK_TBL_SZ: c_int = 30;
// This table stores the values of TX power that will be adjusted by power
// tracking.
//
// For 5G bands, there are 3 different settings.
// For 2G there are cck rate and ofdm rate with different settings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pwr_track_tbl {
    pub pwrtrk_5gd_n: [*const u8; RTW_PWR_TRK_5G_NUM],
    pub pwrtrk_5gd_p: [*const u8; RTW_PWR_TRK_5G_NUM],
    pub pwrtrk_5gc_n: [*const u8; RTW_PWR_TRK_5G_NUM],
    pub pwrtrk_5gc_p: [*const u8; RTW_PWR_TRK_5G_NUM],
    pub pwrtrk_5gb_n: [*const u8; RTW_PWR_TRK_5G_NUM],
    pub pwrtrk_5gb_p: [*const u8; RTW_PWR_TRK_5G_NUM],
    pub pwrtrk_5ga_n: [*const u8; RTW_PWR_TRK_5G_NUM],
    pub pwrtrk_5ga_p: [*const u8; RTW_PWR_TRK_5G_NUM],
    pub pwrtrk_2gd_n: *const u8,
    pub pwrtrk_2gd_p: *const u8,
    pub pwrtrk_2gc_n: *const u8,
    pub pwrtrk_2gc_p: *const u8,
    pub pwrtrk_2gb_n: *const u8,
    pub pwrtrk_2gb_p: *const u8,
    pub pwrtrk_2ga_n: *const u8,
    pub pwrtrk_2ga_p: *const u8,
    pub pwrtrk_2g_cckd_n: *const u8,
    pub pwrtrk_2g_cckd_p: *const u8,
    pub pwrtrk_2g_cckc_n: *const u8,
    pub pwrtrk_2g_cckc_p: *const u8,
    pub pwrtrk_2g_cckb_n: *const u8,
    pub pwrtrk_2g_cckb_p: *const u8,
    pub pwrtrk_2g_ccka_n: *const u8,
    pub pwrtrk_2g_ccka_p: *const u8,
    pub pwrtrk_xtal_n: *const i8,
    pub pwrtrk_xtal_p: *const i8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_wlan_cpu {
    RTW_WCPU_3081,
    RTW_WCPU_8051,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_fw_fifo_sel {
    RTW_FW_FIFO_SEL_TX,
    RTW_FW_FIFO_SEL_RX,
    RTW_FW_FIFO_SEL_RSVD_PAGE,
    RTW_FW_FIFO_SEL_REPORT,
    RTW_FW_FIFO_SEL_LLT,
    RTW_FW_FIFO_SEL_RXBUF_FW,

    RTW_FW_FIFO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_fwcd_item {
    RTW_FWCD_TLV,
    RTW_FWCD_REG,
    RTW_FWCD_ROM,
    RTW_FWCD_IMEM,
    RTW_FWCD_DMEM,
    RTW_FWCD_EMEM,
}

// hardware configuration for each IC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_chip_info {
    pub ops: *const rtw_chip_ops,
    pub id: u8,
    pub fw_name: *const c_char,
    pub wlan_cpu: rtw_wlan_cpu,
    pub tx_pkt_desc_sz: u8,
    pub tx_buf_desc_sz: u8,
    pub rx_pkt_desc_sz: u8,
    pub rx_buf_desc_sz: u8,
    pub phy_efuse_size: u32,
    pub log_efuse_size: u32,
    pub ptct_efuse_size: u32,
    pub txff_size: u32,
    pub rxff_size: u32,
    pub fw_rxff_size: u32,
    pub rsvd_drv_pg_num: u16,
    pub band: u8,
    pub page_size: u16,
    pub csi_buf_pg_num: u8,
    pub dig_max: u8,
    pub dig_min: u8,
    pub txgi_factor: u8,
    pub is_pwr_by_rate_dec: bool,
    pub rx_ldpc: bool,
    pub tx_stbc: bool,
    pub max_power_index: u8,
    pub ampdu_density: u8,
    pub fw_fifo_addr: [u16; RTW_FW_FIFO_MAX],
    pub fwcd_segs: *const rtw_fwcd_segs,
    pub amsdu_in_ampdu: bool,
    pub usb_tx_agg_desc_num: u8,
    pub hw_feature_report: bool,
    pub c2h_ra_report_size: u8,
    pub old_datarate_fb_limit: bool,
    pub default_1ss_tx_path: u8,
    pub path_div_supported: bool,
    pub ht_supported: bool,
    pub vht_supported: bool,
    pub lps_deep_mode_supported: u8,
// init values
    pub sys_func_en: u8,
    pub pwr_on_seq: *const *const rtw_pwr_seq_cmd,
    pub pwr_off_seq: *const *const rtw_pwr_seq_cmd,
    pub rqpn_table: *const rtw_rqpn,
    pub prioq_addrs: *const rtw_prioq_addrs,
    pub page_table: *const rtw_page_table,
    pub intf_table: *const rtw_intf_phy_para_table,
    pub dig: *const rtw_hw_reg,
    pub dig_cck: *const rtw_hw_reg,
    pub rf_base_addr: [u32; RTW_RF_PATH_MAX],
    pub rf_sipi_addr: [u32; RTW_RF_PATH_MAX],
    pub rf_sipi_read_addr: *const rtw_rf_sipi_addr,
    pub fix_rf_phy_num: u8,
    pub ltecoex_addr: *const rtw_ltecoex_addr,
    pub mac_tbl: *const rtw_table,
    pub agc_tbl: *const rtw_table,
    pub bb_tbl: *const rtw_table,
    pub rf_tbl: [*const rtw_table; RTW_RF_PATH_MAX],
    pub rfk_init_tbl: *const rtw_table,
    pub rfe_defs: *const rtw_rfe_def,
    pub rfe_defs_size: u32,
    pub en_dis_dpd: bool,
    pub dpd_ratemask: u16,
    pub iqk_threshold: u8,
    pub lck_threshold: u8,
    pub bfer_su_max_num: u8,
    pub bfer_mu_max_num: u8,
    pub edcca_th: *const rtw_hw_reg_offset,
    pub l2h_th_ini_cs: i8,
    pub l2h_th_ini_ad: i8,
    pub wow_fw_name: *const c_char,
    pub wowlan_stub: *const wiphy_wowlan_support,
    pub max_sched_scan_ssids: u8,
    pub max_scan_ie_len: u16,
// coex paras
    pub coex_para_ver: u32,
    pub bt_desired_ver: u8,
    pub scbd_support: bool,
    pub /: *mut *mut bool new_scbd10_def; / true: fix 2M(8822c),
    pub ble_hid_profile_support: bool,
    pub wl_mimo_ps_support: bool,
    pub /: *mut *mut u8 pstdma_type; / 0: LPSoff, 1:LPSon,
    pub bt_rssi_type: u8,
    pub ant_isolation: u8,
    pub rssi_tolerance: u8,
    pub table_sant_num: u8,
    pub table_nsant_num: u8,
    pub tdma_sant_num: u8,
    pub tdma_nsant_num: u8,
    pub bt_afh_span_bw20: u8,
    pub bt_afh_span_bw40: u8,
    pub afh_5g_num: u8,
    pub wl_rf_para_num: u8,
    pub coex_info_hw_regs_num: u8,
    pub bt_rssi_step: *const u8,
    pub wl_rssi_step: *const u8,
    pub table_nsant: *const coex_table_para,
    pub table_sant: *const coex_table_para,
    pub tdma_sant: *const coex_tdma_para,
    pub tdma_nsant: *const coex_tdma_para,
    pub wl_rf_para_tx: *const coex_rf_para,
    pub wl_rf_para_rx: *const coex_rf_para,
    pub afh_5g: *const coex_5g_afh_map,
    pub btg_reg: *const rtw_hw_reg,
    pub coex_info_hw_regs: *const rtw_reg_domain,
    pub wl_fw_desired_ver: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_coex_bt_state_cnt {
    COEX_CNT_BT_RETRY,
    COEX_CNT_BT_REINIT,
    COEX_CNT_BT_REENABLE,
    COEX_CNT_BT_POPEVENT,
    COEX_CNT_BT_SETUPLINK,
    COEX_CNT_BT_IGNWLANACT,
    COEX_CNT_BT_INQ,
    COEX_CNT_BT_PAGE,
    COEX_CNT_BT_ROLESWITCH,
    COEX_CNT_BT_AFHUPDATE,
    COEX_CNT_BT_INFOUPDATE,
    COEX_CNT_BT_IQK,
    COEX_CNT_BT_IQKFAIL,

    COEX_CNT_BT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_coex_wl_state_cnt {
    COEX_CNT_WL_SCANAP,
    COEX_CNT_WL_CONNPKT,
    COEX_CNT_WL_COEXRUN,
    COEX_CNT_WL_NOISY0,
    COEX_CNT_WL_NOISY1,
    COEX_CNT_WL_NOISY2,
    COEX_CNT_WL_5MS_NOEXTEND,
    COEX_CNT_WL_FW_NOTIFY,

    COEX_CNT_WL_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_coex_rfe {
    pub ant_switch_exist: bool,
    pub ant_switch_diversity: bool,
    pub ant_switch_with_bt: bool,
    pub rfe_module_type: u8,
    pub ant_switch_polarity: u8,
// true if WLG at BTG, else at WLAG
    pub wlg_at_btg: bool,
}

pub const COEX_WL_TDMA_PARA_LENGTH: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_coex_dm {
    pub cur_ps_tdma_on: bool,
    pub cur_wl_rx_low_gain_en: bool,
    pub ignore_wl_act: bool,
    pub reason: u8,
    pub bt_rssi_state: [u8; 4],
    pub wl_rssi_state: [u8; 4],
    pub wl_ch_info: [u8; 3],
    pub cur_ps_tdma: u8,
    pub cur_table: u8,
    pub ps_tdma_para: [u8; 5],
    pub cur_bt_pwr_lvl: u8,
    pub cur_bt_lna_lvl: u8,
    pub cur_wl_pwr_lvl: u8,
    pub bt_status: u8,
    pub cur_ant_pos_type: u32,
    pub cur_switch_status: u32,
    pub setting_tdma: u32,
    pub fw_tdma_para: [u8; COEX_WL_TDMA_PARA_LENGTH],
}

pub const COEX_BTINFO_SRC_WL_FW: c_uint = 0x0;
pub const COEX_BTINFO_SRC_BT_RSP: c_uint = 0x1;
pub const COEX_BTINFO_SRC_BT_ACT: c_uint = 0x2;
pub const COEX_BTINFO_SRC_BT_IQK: c_uint = 0x3;
pub const COEX_BTINFO_SRC_BT_SCBD: c_uint = 0x4;
pub const COEX_BTINFO_SRC_H2C60: c_uint = 0x5;
pub const COEX_BTINFO_SRC_MAX: c_uint = 0x6;

pub const COEX_BTINFO_LENGTH_MAX: c_int = 10;
pub const COEX_BTINFO_LENGTH: c_int = 7;
pub const COEX_BT_HIDINFO_LIST: c_uint = 0x0;
pub const COEX_BT_HIDINFO_A: c_uint = 0x1;
pub const COEX_BT_HIDINFO_NAME: c_int = 3;
pub const COEX_BT_HIDINFO_LENGTH: c_int = 6;
pub const COEX_BT_HIDINFO_HANDLE_NUM: c_int = 4;
pub const COEX_BT_HIDINFO_C2H_HANDLE: c_int = 0;
pub const COEX_BT_HIDINFO_C2H_VENDOR: c_int = 1;
pub const COEX_BT_BLE_HANDLE_THRS: c_uint = 0x10;
pub const COEX_BT_HIDINFO_NOTCON: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_coex_hid {
    pub hid_handle: u8,
    pub hid_vendor: u8,
    pub hid_name: [u8; COEX_BT_HIDINFO_NAME],
    pub hid_info_completed: bool,
    pub is_game_hid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_coex_hid_handle_list {
    pub cmd_id: u8,
    pub len: u8,
    pub subid: u8,
    pub handle_cnt: u8,
    pub handle: [u8; COEX_BT_HIDINFO_HANDLE_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_coex_hid_info_a {
    pub cmd_id: u8,
    pub len: u8,
    pub subid: u8,
    pub handle: u8,
    pub vendor: u8,
    pub name: [u8; COEX_BT_HIDINFO_NAME],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_coex_stat {
    pub bt_disabled: bool,
    pub bt_disabled_pre: bool,
    pub bt_link_exist: bool,
    pub bt_whck_test: bool,
    pub bt_inq_page: bool,
    pub bt_inq_remain: bool,
    pub bt_inq: bool,
    pub bt_page: bool,
    pub bt_ble_voice: bool,
    pub bt_ble_exist: bool,
    pub bt_hfp_exist: bool,
    pub bt_a2dp_exist: bool,
    pub bt_hid_exist: bool,
    pub /: *mut *mut bool bt_pan_exist; / PAN or OPP,
    pub /: *mut *mut bool bt_opp_exist; / OPP only,
    pub bt_acl_busy: bool,
    pub bt_fix_2M: bool,
    pub bt_setup_link: bool,
    pub bt_multi_link: bool,
    pub bt_multi_link_pre: bool,
    pub bt_multi_link_remain: bool,
    pub bt_a2dp_sink: bool,
    pub bt_a2dp_active: bool,
    pub bt_reenable: bool,
    pub bt_ble_scan_en: bool,
    pub bt_init_scan: bool,
    pub bt_slave: bool,
    pub bt_418_hid_exist: bool,
    pub bt_ble_hid_exist: bool,
    pub bt_game_hid_exist: bool,
    pub bt_hid_handle_cnt: bool,
    pub bt_mailbox_reply: bool,
    pub bt_ctr_ok: bool,
    pub wl_under_lps: bool,
    pub wl_under_ips: bool,
    pub wl_hi_pri_task1: bool,
    pub wl_hi_pri_task2: bool,
    pub wl_force_lps_ctrl: bool,
    pub wl_gl_busy: bool,
    pub wl_linkscan_proc: bool,
    pub wl_ps_state_fail: bool,
    pub wl_tx_limit_en: bool,
    pub wl_ampdu_limit_en: bool,
    pub wl_connected: bool,
    pub wl_slot_extend: bool,
    pub wl_cck_lock: bool,
    pub wl_cck_lock_pre: bool,
    pub wl_cck_lock_ever: bool,
    pub wl_connecting: bool,
    pub wl_slot_toggle: bool,
    pub /: *mut *mut bool wl_slot_toggle_change; / if toggle to no-toggle,
    pub wl_mimo_ps: bool,
    pub bt_supported_version: u32,
    pub bt_supported_feature: u32,
    pub hi_pri_tx: u32,
    pub hi_pri_rx: u32,
    pub lo_pri_tx: u32,
    pub lo_pri_rx: u32,
    pub patch_ver: u32,
    pub bt_reg_vendor_ae: u16,
    pub bt_reg_vendor_ac: u16,
    pub bt_rssi: i8,
    pub kt_ver: u8,
    pub gnt_workaround_state: u8,
    pub tdma_timer_base: u8,
    pub bt_profile_num: u8,
    pub bt_info_c2h: [u8; COEX_BTINFO_SRC_MAX][COEX_BTINFO_LENGTH_MAX],
    pub bt_info_lb2: u8,
    pub bt_info_lb3: u8,
    pub bt_info_hb0: u8,
    pub bt_info_hb1: u8,
    pub bt_info_hb2: u8,
    pub bt_info_hb3: u8,
    pub bt_ble_scan_type: u8,
    pub bt_hid_pair_num: u8,
    pub bt_hid_slot: u8,
    pub bt_a2dp_bitpool: u8,
    pub bt_iqk_state: u8,
    pub bt_disable_cnt: u8,
    pub wl_beacon_interval: u16,
    pub wl_noisy_level: u8,
    pub wl_fw_dbg_info: [u8; 10],
    pub wl_fw_dbg_info_pre: [u8; 10],
    pub wl_rx_rate: u8,
    pub wl_tx_rate: u8,
    pub wl_rts_rx_rate: u8,
    pub wl_coex_mode: u8,
    pub wl_iot_peer: u8,
    pub ampdu_max_time: u8,
    pub wl_tput_dir: u8,
    pub wl_toggle_para: [u8; 6],
    pub wl_toggle_interval: u8,
    pub score_board: u16,
    pub retry_limit: u16,
// counters to record bt states
    pub cnt_bt: [u32; COEX_CNT_BT_MAX],
// counters to record wifi states
    pub cnt_wl: [u32; COEX_CNT_WL_MAX],
// counters to record bt c2h data
    pub cnt_bt_info_c2h: [u32; COEX_BTINFO_SRC_MAX],
    pub darfrc: u32,
    pub darfrch: u32,
    pub hid_info: [rtw_coex_hid; COEX_BT_HIDINFO_HANDLE_NUM],
    pub hid_handle_list: rtw_coex_hid_handle_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_coex {
    pub queue: sk_buff_head,
    pub wait: wait_queue_head_t,
    pub under_5g: bool,
    pub stop_dm: bool,
    pub freeze: bool,
    pub freerun: bool,
    pub wl_rf_off: bool,
    pub manual_control: bool,
    pub stat: rtw_coex_stat,
    pub dm: rtw_coex_dm,
    pub rfe: rtw_coex_rfe,
    pub bt_relink_work: delayed_work,
    pub bt_reenable_work: delayed_work,
    pub defreeze_work: delayed_work,
    pub wl_remain_work: delayed_work,
    pub bt_remain_work: delayed_work,
    pub wl_connecting_work: delayed_work,
    pub bt_multi_link_remain_work: delayed_work,
    pub wl_ccklock_work: delayed_work,
}

pub const DPK_RF_REG_NUM: c_int = 7;
pub const DPK_RF_PATH_NUM: c_int = 2;
pub const DPK_BB_REG_NUM: c_int = 18;
pub const DPK_CHANNEL_WIDTH_80: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_dpk_info {
    pub is_dpk_pwr_on: bool,
    pub is_reload: bool,
    pub DPK_RF_PATH_NUM): DECLARE_BITMAP(dpk_path_ok,,
    pub thermal_dpk: [u8; DPK_RF_PATH_NUM],
    pub avg_thermal: [ewma_thermal; DPK_RF_PATH_NUM],
    pub gnt_control: u32,
    pub gnt_value: u32,
    pub result: [u8; RTW_RF_PATH_MAX],
    pub dpk_txagc: [u8; RTW_RF_PATH_MAX],
    pub coef: [u32; RTW_RF_PATH_MAX][20],
    pub dpk_gs: [u16; RTW_RF_PATH_MAX],
    pub thermal_dpk_delta: [u8; RTW_RF_PATH_MAX],
    pub pre_pwsf: [u8; RTW_RF_PATH_MAX],
    pub dpk_band: u8,
    pub dpk_ch: u8,
    pub dpk_bw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_phy_cck_pd_reg {
    pub reg_pd: u32,
    pub mask_pd: u32,
    pub reg_cs: u32,
    pub mask_cs: u32,
}

pub const DACK_MSBK_BACKUP_NUM: c_uint = 0xf;
pub const DACK_DCK_BACKUP_NUM: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_swing_table {
    pub p: [*const u8; RTW_RF_PATH_MAX],
    pub n: [*const u8; RTW_RF_PATH_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pkt_count {
    pub num_bcn_pkt: u16,
    pub num_qry_pkt: [u16; DESC_RATE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_iqk_info {
    pub done: bool,
    pub s1_x: u32,
    pub s1_y: u32,
    pub s0_x: u32,
    pub s0_y: u32,
    pub result: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_rf_band {
    RF_BAND_2G_CCK,
    RF_BAND_2G_OFDM,
    RF_BAND_5G_L,
    RF_BAND_5G_M,
    RF_BAND_5G_H,
    RF_BAND_MAX
}

pub const RF_GAIN_NUM: c_int = 11;
pub const RF_HW_OFFSET_NUM: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_gapk_info {
    pub rf3f_bp: [u32; RF_BAND_MAX][RF_GAIN_NUM][RTW_RF_PATH_MAX],
    pub rf3f_fs: [u32; RTW_RF_PATH_MAX][RF_GAIN_NUM],
    pub txgapk_bp_done: bool,
    pub offset: [i8; RF_GAIN_NUM][RTW_RF_PATH_MAX],
    pub fianl_offset: [i8; RF_GAIN_NUM][RTW_RF_PATH_MAX],
    pub read_txgain: u8,
    pub channel: u8,
}

pub const EDCCA_TH_L2H_IDX: c_int = 0;
pub const EDCCA_TH_H2L_IDX: c_int = 1;
pub const EDCCA_TH_L2H_LB: c_int = 48;
pub const EDCCA_ADC_BACKOFF: c_int = 12;
pub const EDCCA_IGI_BASE: c_int = 50;
pub const EDCCA_IGI_L2H_DIFF: c_int = 8;
pub const EDCCA_L2H_H2L_DIFF: c_int = 7;
pub const EDCCA_L2H_H2L_DIFF_NORMAL: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_edcca_mode {
    RTW_EDCCA_NORMAL	= 0,
    RTW_EDCCA_ADAPTIVITY	= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_cfo_track {
    pub is_adjust: bool,
    pub crystal_cap: u8,
    pub cfo_tail: [i32; RTW_RF_PATH_MAX],
    pub cfo_cnt: [i32; RTW_RF_PATH_MAX],
    pub packet_count: u32,
    pub packet_count_pre: u32,
}

pub const RRSR_INIT_2G: c_uint = 0x15f;
pub const RRSR_INIT_5G: c_uint = 0x150;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_dm_cap {
    RTW_DM_CAP_NA,
    RTW_DM_CAP_TXGAPK,
    RTW_DM_CAP_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_dm_info {
    pub cck_fa_cnt: u32,
    pub ofdm_fa_cnt: u32,
    pub total_fa_cnt: u32,
    pub cck_cca_cnt: u32,
    pub ofdm_cca_cnt: u32,
    pub total_cca_cnt: u32,
    pub cck_ok_cnt: u32,
    pub cck_err_cnt: u32,
    pub ofdm_ok_cnt: u32,
    pub ofdm_err_cnt: u32,
    pub ht_ok_cnt: u32,
    pub ht_err_cnt: u32,
    pub vht_ok_cnt: u32,
    pub vht_err_cnt: u32,
    pub min_rssi: u8,
    pub pre_min_rssi: u8,
    pub fa_history: [u16; 4],
    pub igi_history: [u8; 4],
    pub igi_bitmap: u8,
    pub damping: bool,
    pub damping_cnt: u8,
    pub damping_rssi: u8,
    pub cck_gi_u_bnd: u8,
    pub cck_gi_l_bnd: u8,
    pub fix_rate: u8,
    pub tx_rate: u8,
    pub rrsr_val_init: u32,
    pub rrsr_mask_min: u32,
    pub thermal_avg: [u8; RTW_RF_PATH_MAX],
    pub thermal_meter_k: u8,
    pub thermal_meter_lck: u8,
    pub delta_power_index: [i8; RTW_RF_PATH_MAX],
    pub delta_power_index_last: [i8; RTW_RF_PATH_MAX],
    pub default_ofdm_index: u8,
    pub default_cck_index: u8,
    pub pwr_trk_triggered: bool,
    pub pwr_trk_init_trigger: bool,
    pub avg_thermal: [ewma_thermal; RTW_RF_PATH_MAX],
    pub txagc_remnant_cck: i8,
    pub txagc_remnant_ofdm: [i8; RTW_RF_PATH_MAX],
    pub rx_cck_agc_report_type: u8,
// backup dack results for each path and I/Q
    pub dack_adck: [u32; RTW_RF_PATH_MAX],
    pub dack_msbk: [u16; RTW_RF_PATH_MAX][2][DACK_MSBK_BACKUP_NUM],
    pub dack_dck: [u8; RTW_RF_PATH_MAX][2][DACK_DCK_BACKUP_NUM],
    pub dpk_info: rtw_dpk_info,
    pub cfo_track: rtw_cfo_track,
// [bandwidth 0:20M/1:40M][number of path]
    pub cck_pd_lv: [u8; 2][RTW_RF_PATH_MAX],
    pub cck_fa_avg: u32,
    pub cck_pd_default: u8,
// save the last rx phy status for debug
    pub rx_snr: [i8; RTW_RF_PATH_MAX],
    pub rx_evm_dbm: [u8; RTW_RF_PATH_MAX],
    pub cfo_tail: [i16; RTW_RF_PATH_MAX],
    pub rssi: [u8; RTW_RF_PATH_MAX],
    pub curr_rx_rate: u8,
    pub cur_pkt_count: rtw_pkt_count,
    pub last_pkt_count: rtw_pkt_count,
    pub ewma_evm: [ewma_evm; RTW_EVM_NUM],
    pub ewma_snr: [ewma_snr; RTW_SNR_NUM],
    pub /: *mut *mut u32 dm_flags; / enum rtw_dm_cap,
    pub iqk: rtw_iqk_info,
    pub gapk: rtw_gapk_info,
    pub is_bt_iqk_timeout: bool,
    pub l2h_th_ini: i8,
    pub edcca_mode: rtw_edcca_mode,
    pub scan_density: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_efuse {
    pub size: u32,
    pub physical_size: u32,
    pub logical_size: u32,
    pub protect_size: u32,
    pub addr: [u8; ETH_ALEN],
    pub channel_plan: u8,
    pub country_code: [u8; 2],
    pub rf_board_option: u8,
    pub rfe_option: u8,
    pub power_track_type: u8,
    pub thermal_meter: [u8; RTW_RF_PATH_MAX],
    pub thermal_meter_k: u8,
    pub crystal_cap: u8,
    pub ant_div_cfg: u8,
    pub ant_div_type: u8,
    pub regd: u8,
    pub afe: u8,
    pub lna_type_2g: u8,
    pub lna_type_5g: u8,
    pub glna_type: u8,
    pub alna_type: u8,
    pub ext_lna_2g: bool,
    pub ext_lna_5g: bool,
    pub pa_type_2g: u8,
    pub pa_type_5g: u8,
    pub gpa_type: u8,
    pub apa_type: u8,
    pub ext_pa_2g: bool,
    pub ext_pa_5g: bool,
    pub tx_bb_swing_setting_2g: u8,
    pub tx_bb_swing_setting_5g: u8,
    pub btcoex: bool,
// bt share antenna with wifi
    pub share_ant: bool,
    pub bt_setting: u8,
    pub usb_mode_switch: u8,
    pub hci: u8,
    pub bw: u8,
    pub ptcl: u8,
    pub nss: u8,
    pub ant_num: u8,
    pub hw_cap: },
    pub txpwr_idx_table: [rtw_txpwr_idx; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_phy_cond {

    pub rfe:8: u32,
    pub intf:4: u32,
    pub pkg:4: u32,
    pub plat:4: u32,
    pub intf_rsvd:4: u32,
    pub cut:4: u32,
    pub branch:2: u32,
    pub neg:1: u32,
    pub pos:1: u32,

    pub pos:1: u32,
    pub neg:1: u32,
    pub branch:2: u32,
    pub cut:4: u32,
    pub intf_rsvd:4: u32,
    pub plat:4: u32,
    pub pkg:4: u32,
    pub intf:4: u32,
    pub rfe:8: u32,

// for intf:4

// for branch:2
pub const BRANCH_IF: c_int = 0;
pub const BRANCH_ELIF: c_int = 1;
pub const BRANCH_ELSE: c_int = 2;
pub const BRANCH_ENDIF: c_int = 3;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_phy_cond2 {

    pub type_glna: u8,
    pub type_gpa: u8,
    pub type_alna: u8,
    pub type_apa: u8,

    pub type_apa: u8,
    pub type_alna: u8,
    pub type_gpa: u8,
    pub type_glna: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fifo_conf {
// tx fifo information
    pub rsvd_boundary: u16,
    pub rsvd_pg_num: u16,
    pub rsvd_drv_pg_num: u16,
    pub txff_pg_num: u16,
    pub acq_pg_num: u16,
    pub rsvd_drv_addr: u16,
    pub rsvd_h2c_info_addr: u16,
    pub rsvd_h2c_sta_info_addr: u16,
    pub rsvd_h2cq_addr: u16,
    pub rsvd_cpu_instr_addr: u16,
    pub rsvd_fw_txbuf_addr: u16,
    pub rsvd_csibuf_addr: u16,
    pub rqpn: *const rtw_rqpn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fwcd_desc {
    pub size: u32,
    pub next: *mut u8,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fwcd_segs {
    pub segs: *const u32,
    pub num: u8,
}

pub const FW_CD_TYPE: c_uint = 0xffff;
pub const FW_CD_LEN: c_int = 4;
pub const FW_CD_VAL: c_uint = 0xaabbccdd;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_fw_state {
    pub firmware: *const firmware,
    pub rtwdev: *mut rtw_dev,
    pub completion: completion,
    pub fwcd_desc: rtw_fwcd_desc,
    pub version: u16,
    pub sub_version: u8,
    pub sub_index: u8,
    pub h2c_version: u16,
    pub feature: u32,
    pub feature_ext: u32,
    pub type: rtw_fw_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_sar_sources {
    RTW_SAR_SOURCE_NONE,
    RTW_SAR_SOURCE_COMMON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_sar_bands {
    RTW_SAR_BAND_0,
    RTW_SAR_BAND_1,
// RTW_SAR_BAND_2, not used now
    RTW_SAR_BAND_3,
    RTW_SAR_BAND_4,

    RTW_SAR_BAND_NR,
}

// the union is reserved for other kinds of SAR sources
// which might not re-use same format with array common.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw_sar_cfg {
    pub common: [i8; RTW_SAR_BAND_NR],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_sar {
    pub src: rtw_sar_sources,
    pub cfg: [rtw_sar_cfg; RTW_RF_PATH_MAX][RTW_RATE_SECTION_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_hal {
    pub rcr: u32,
    pub rxfltmap1: u16,
    pub chip_version: u32,
    pub cut_version: u8,
    pub mp_chip: u8,
    pub oem_id: u8,
    pub pkg_type: u8,
    pub phy_cond: rtw_phy_cond,
    pub phy_cond2: rtw_phy_cond2,
    pub rfe_btg: bool,
    pub ps_mode: u8,
    pub current_channel: u8,
    pub current_primary_channel_index: u8,
    pub current_band_width: u8,
    pub current_band_type: u8,
    pub primary_channel: u8,
// center channel for different available bandwidth,
// val of (bw > current_band_width) is invalid
//
    pub 1]: u8 cch_by_bw[RTW_MAX_CHANNEL_WIDTH +,
    pub sec_ch_offset: u8,
    pub rf_type: u8,
    pub rf_path_num: u8,
    pub rf_phy_num: u8,
    pub antenna_tx: u32,
    pub antenna_rx: u32,
    pub bfee_sts_cap: u8,
    pub txrx_1ss: bool,
    pub cck_high_power: bool,
// protect tx power section
    pub tx_power_mutex: mutex,
    pub sar_band: rtw_sar_bands,
    pub sar: rtw_sar,
// for 8821c set channel
    pub ch_param: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_path_div {
    pub current_tx_path: rtw_bb_path,
    pub path_a_sum: u32,
    pub path_b_sum: u32,
    pub path_a_cnt: u16,
    pub path_b_cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_chan_info {
    pub pri_ch_idx: c_int,
    pub action_id: c_int,
    pub bw: c_int,
    pub extra_info: u8,
    pub channel: u8,
    pub timeout: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_chan_list {
    pub buf_size: u32,
    pub ch_num: u32,
    pub size: u32,
    pub addr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_hw_scan_info {
    pub scanning_vif: *mut ieee80211_vif,
    pub probe_pg_size: u8,
    pub op_pri_ch_idx: u8,
    pub op_pri_ch: u8,
    pub op_chan: u8,
    pub op_bw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_dev {
    pub hw: *mut ieee80211_hw,
    pub dev: *mut device,
    pub hci: rtw_hci,
    pub scan_info: rtw_hw_scan_info,
    pub chip: *const rtw_chip_info,
    pub hal: rtw_hal,
    pub fifo: rtw_fifo_conf,
    pub fw: rtw_fw_state,
    pub efuse: rtw_efuse,
    pub sec: rtw_sec_desc,
    pub stats: rtw_traffic_stats,
    pub regd: rtw_regd,
    pub bf_info: rtw_bf_info,
    pub dm_info: rtw_dm_info,
    pub coex: rtw_coex,
// ensures exclusive access from mac80211 callbacks
    pub mutex: mutex,
// watch dog every 2 sec
    pub watch_dog_work: delayed_work,
    pub watch_dog_cnt: u32,
    pub rsvd_page_list: list_head,
// c2h cmd queue & handler work
    pub c2h_queue: sk_buff_head,
    pub c2h_work: work_struct,
    pub ips_work: work_struct,
    pub fw_recovery_work: work_struct,
    pub update_beacon_work: work_struct,
// used to protect txqs list
    pub txq_lock: spinlock_t,
    pub txqs: list_head,
    pub tx_wq: *mut workqueue_struct,
    pub tx_work: work_struct,
    pub ba_work: work_struct,
    pub tx_report: rtw_tx_report,
// indicate the mail box to use with fw
    pub last_box_num: u8,
    pub seq: u32,
    pub h2c: },
// lps power state & handler work
    pub lps_conf: rtw_lps_conf,
    pub ps_enabled: bool,
    pub beacon_loss: bool,
    pub lps_leave_check: completion,
    pub debugfs: *mut rtw_debugfs,
    pub sta_cnt: u8,
    pub rts_threshold: u32,
    pub RTW_PORT_NUM): DECLARE_BITMAP(hw_port,,
    pub RTW_MAX_MAC_ID_NUM): DECLARE_BITMAP(mac_id_map,,
    pub NUM_OF_RTW_FLAGS): DECLARE_BITMAP(flags,,
    pub mp_mode: u8,
    pub dm_path_div: rtw_path_div,
    pub wow_fw: rtw_fw_state,
    pub wow: rtw_wow_param,
    pub need_rfk: bool,
    pub fw_scan_density: completion,
    pub ap_active: bool,
    pub led_registered: bool,
    pub led_name: [c_char; 32],
    pub led_cdev: led_classdev,
// hci related data, must be last
    pub )): *mut u8 priv[] __aligned(sizeof(void,
}

extern "C" {
    pub fn container_of(_arg: p, ieee80211_txq: struct, _arg: drv_priv) -> return;
}
extern "C" {
    pub fn container_of(_arg: p, ieee80211_vif: struct, _arg: drv_priv) -> return;
}
extern "C" {
    pub fn rtw_set_rx_freq_band(pkt_stat: *mut rtw_rx_pkt_stat, channel: u8);
}
extern "C" {
    pub fn rtw_set_dtim_period(rtwdev: *mut rtw_dev, dtim_period: u8);
}
extern "C" {
    pub fn check_hw_ready(rtwdev: *mut rtw_dev, addr: u32, mask: u32, target: u32) -> bool;
}
extern "C" {
    pub fn ltecoex_read_reg(rtwdev: *mut rtw_dev, offset: u16, val: *mut u32) -> bool;
}
extern "C" {
    pub fn ltecoex_reg_write(rtwdev: *mut rtw_dev, offset: u16, value: u32) -> bool;
}
extern "C" {
    pub fn rtw_desc_to_mcsrate(rate: u16, mcs: *mut u8, nss: *mut u8);
}
extern "C" {
    pub fn rtw_set_channel(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_chip_prepare_tx(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_tx_report_purge_timer(t: *mut timer_list);
}
extern "C" {
    pub fn rtw_core_start(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_power_off(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_core_stop(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_chip_info_setup(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_core_init(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_core_deinit(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_register_hw(rtwdev: *mut rtw_dev, hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rtw_unregister_hw(rtwdev: *mut rtw_dev, hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtw_desc_to_bitrate(desc_rate: u8) -> u16;
}
extern "C" {
    pub fn rtw_fw_recovery(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_wait_firmware_completion(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_power_on(rtwdev: *mut rtw_dev) -> c_int;
}
extern "C" {
    pub fn rtw_core_fw_scan_notify(rtwdev: *mut rtw_dev, start: bool);
}
extern "C" {
    pub fn rtw_dump_reg(rtwdev: *mut rtw_dev, addr: u32, size: u32) -> c_int;
}
extern "C" {
    pub fn rtw_set_txrx_1ss(rtwdev: *mut rtw_dev, config_1ss: bool);
}
extern "C" {
    pub fn rtw_core_port_switch(rtwdev: *mut rtw_dev, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn rtw_core_check_sta_active(rtwdev: *mut rtw_dev) -> bool;
}
extern "C" {
    pub fn rtw_core_enable_beacon(rtwdev: *mut rtw_dev, enable: bool);
}
