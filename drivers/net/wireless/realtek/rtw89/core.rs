//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/core.h
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
// Copyright(c) 2019-2020  Realtek Corporation
//

pub const MASKBYTE0: c_uint = 0xff;
pub const MASKBYTE1: c_uint = 0xff00;
pub const MASKBYTE2: c_uint = 0xff0000;
pub const MASKBYTE3: c_uint = 0xff000000;
pub const MASKBYTE4: c_uint = 0xff00000000ULL;
pub const MASKHWORD: c_uint = 0xffff0000;
pub const MASKLWORD: c_uint = 0x0000ffff;
pub const MASKDWORD: c_uint = 0xffffffff;
pub const RFREG_MASK: c_uint = 0xfffff;
pub const INV_RF_DATA: c_uint = 0xffffffff;
pub const BYPASS_CR_DATA: c_uint = 0xbabecafe;
pub const RTW89_R32_EA: c_uint = 0xEAEAEAEA;
pub const RTW89_R32_DEAD: c_uint = 0xDEADBEEF;

pub const RTW89_PS_HANG_MAX_CNT: c_int = 3;
pub const CFO_TRACK_MAX_USER: c_int = 64;
pub const MAX_RSSI: c_int = 110;
pub const RSSI_FACTOR: c_int = 1;

pub const DELTA_SWINGIDX_SIZE: c_int = 30;

pub const RTW89_HTC_VARIANT_HE: c_int = 3;

pub const RTW89_HTC_VARIANT_HE_CID_OM: c_int = 1;
pub const RTW89_HTC_VARIANT_HE_CID_CAS: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_om_channel_width {
    HTC_OM_CHANNEL_WIDTH_20 = 0,
    HTC_OM_CHANNEL_WIDTH_40 = 1,
    HTC_OM_CHANNEL_WIDTH_80 = 2,
    HTC_OM_CHANNEL_WIDTH_160_OR_80_80 = 3,
}

pub const RTW89_TF_BASIC_USER_INFO_SZ: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_subband {
    RTW89_CH_2G = 0,
    RTW89_CH_5G_BAND_1 = 1,
// RTW89_CH_5G_BAND_2 = 2, unused
    RTW89_CH_5G_BAND_3 = 3,
    RTW89_CH_5G_BAND_4 = 4,

    RTW89_CH_6G_BAND_IDX0, /* Low */
    RTW89_CH_6G_BAND_IDX1, /* Low */
    RTW89_CH_6G_BAND_IDX2, /* Mid */
    RTW89_CH_6G_BAND_IDX3, /* Mid */
    RTW89_CH_6G_BAND_IDX4, /* High */
    RTW89_CH_6G_BAND_IDX5, /* High */
    RTW89_CH_6G_BAND_IDX6, /* Ultra-high */
    RTW89_CH_6G_BAND_IDX7, /* Ultra-high */

    RTW89_SUBBAND_NR,
    RTW89_SUBBAND_2GHZ_5GHZ_NR = RTW89_CH_5G_BAND_4 + 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tx_comp_band {
    RTW89_TX_COMP_BAND_2GHZ,
    RTW89_TX_COMP_BAND_5GHZ_L,
    RTW89_TX_COMP_BAND_5GHZ_H,
    RTW89_TX_COMP_BAND_6GHZ_M,
    RTW89_TX_COMP_BAND_6GHZ_UH,

    RTW89_TX_COMP_BAND_NR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_gain_offset {
    RTW89_GAIN_OFFSET_2G_CCK,
    RTW89_GAIN_OFFSET_2G_OFDM,
    RTW89_GAIN_OFFSET_5G_LOW,
    RTW89_GAIN_OFFSET_5G_MID,
    RTW89_GAIN_OFFSET_5G_HIGH,
    RTW89_GAIN_OFFSET_6G_L0,
    RTW89_GAIN_OFFSET_6G_L1,
    RTW89_GAIN_OFFSET_6G_M0,
    RTW89_GAIN_OFFSET_6G_M1,
    RTW89_GAIN_OFFSET_6G_H0,
    RTW89_GAIN_OFFSET_6G_H1,
    RTW89_GAIN_OFFSET_6G_UH0,
    RTW89_GAIN_OFFSET_6G_UH1,

    RTW89_GAIN_OFFSET_NR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_hci_type {
    RTW89_HCI_TYPE_PCIE,
    RTW89_HCI_TYPE_USB,
    RTW89_HCI_TYPE_SDIO,

    RTW89_HCI_TYPE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_hci_dle_type {
    RTW89_HCI_DLE_TYPE_PCIE,
    RTW89_HCI_DLE_TYPE_USB2,
    RTW89_HCI_DLE_TYPE_USB3,
    RTW89_HCI_DLE_TYPE_SDIO,

    RTW89_HCI_DLE_TYPE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_core_chip_id {
    RTL8852A,
    RTL8852B,
    RTL8852BT,
    RTL8852C,
    RTL8851B,
    RTL8922A,
    RTL8922D,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_core_chip_cid {
    RTL8922D_CID7025 = 0x74,
    RTL8922D_CID7090 = 0x79,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_core_chip_aid {
    RTL8922D_AID1348 = 0x1348,
    RTL8922D_AID7060 = 0x7060,
    RTL8922D_AID7102 = 0x7102,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_chip_gen {
    RTW89_CHIP_AX,
    RTW89_CHIP_BE,

    RTW89_CHIP_GEN_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_cv {
    CHIP_CAV,
    CHIP_CBV,
    CHIP_CCV,
    CHIP_CDV,
    CHIP_CEV,
    CHIP_CFV,
    CHIP_CV_MAX,
    CHIP_CV_INVALID = CHIP_CV_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bacam_ver {
    RTW89_BACAM_V0,
    RTW89_BACAM_V1,

    RTW89_BACAM_V0_EXT = 99,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_core_tx_type {
    RTW89_CORE_TX_TYPE_DATA,
    RTW89_CORE_TX_TYPE_MGMT,
    RTW89_CORE_TX_TYPE_FWCMD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_core_rx_type {
    RTW89_CORE_RX_TYPE_WIFI		= 0,
    RTW89_CORE_RX_TYPE_PPDU_STAT	= 1,
    RTW89_CORE_RX_TYPE_CHAN_INFO	= 2,
    RTW89_CORE_RX_TYPE_BB_SCOPE	= 3,
    RTW89_CORE_RX_TYPE_F2P_TXCMD	= 4,
    RTW89_CORE_RX_TYPE_SS2FW	= 5,
    RTW89_CORE_RX_TYPE_TX_REPORT	= 6,
    RTW89_CORE_RX_TYPE_TX_REL_HOST	= 7,
    RTW89_CORE_RX_TYPE_DFS_REPORT	= 8,
    RTW89_CORE_RX_TYPE_TX_REL_CPU	= 9,
    RTW89_CORE_RX_TYPE_C2H		= 10,
    RTW89_CORE_RX_TYPE_CSI		= 11,
    RTW89_CORE_RX_TYPE_CQI		= 12,
    RTW89_CORE_RX_TYPE_H2C		= 13,
    RTW89_CORE_RX_TYPE_FWDL		= 14,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_txq_flags {
    RTW89_TXQ_F_AMPDU		= 0,
    RTW89_TXQ_F_BLOCK_BA		= 1,
    RTW89_TXQ_F_FORBID_BA		= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_net_type {
    RTW89_NET_TYPE_NO_LINK		= 0,
    RTW89_NET_TYPE_AD_HOC		= 1,
    RTW89_NET_TYPE_INFRA		= 2,
    RTW89_NET_TYPE_AP_MODE		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_wifi_role {
    RTW89_WIFI_ROLE_NONE,
    RTW89_WIFI_ROLE_STATION,
    RTW89_WIFI_ROLE_AP,
    RTW89_WIFI_ROLE_AP_VLAN,
    RTW89_WIFI_ROLE_ADHOC,
    RTW89_WIFI_ROLE_ADHOC_MASTER,
    RTW89_WIFI_ROLE_MESH_POINT,
    RTW89_WIFI_ROLE_MONITOR,
    RTW89_WIFI_ROLE_P2P_DEVICE,
    RTW89_WIFI_ROLE_P2P_CLIENT,
    RTW89_WIFI_ROLE_P2P_GO,
    RTW89_WIFI_ROLE_NAN,
    RTW89_WIFI_ROLE_MLME_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_upd_mode {
    RTW89_ROLE_CREATE,
    RTW89_ROLE_REMOVE,
    RTW89_ROLE_TYPE_CHANGE,
    RTW89_ROLE_INFO_CHANGE,
    RTW89_ROLE_CON_DISCONN,
    RTW89_ROLE_BAND_SW,
    RTW89_ROLE_FW_RESTORE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_self_role {
    RTW89_SELF_ROLE_CLIENT,
    RTW89_SELF_ROLE_AP,
    RTW89_SELF_ROLE_AP_CLIENT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_msk_sO_el {
    RTW89_NO_MSK,
    RTW89_SMA,
    RTW89_TMA,
    RTW89_BSSID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_sch_tx_sel {
    RTW89_SCH_TX_SEL_ALL,
    RTW89_SCH_TX_SEL_HIQ,
    RTW89_SCH_TX_SEL_MG0,
    RTW89_SCH_TX_SEL_MACID,
}

// RTW89_ADDR_CAM_SEC_NONE	: not enabled
// RTW89_ADDR_CAM_SEC_ALL_UNI	: 0 - 6 unicast
// RTW89_ADDR_CAM_SEC_NORMAL	: 0 - 1 unicast, 2 - 4 group, 5 - 6 BIP
// RTW89_ADDR_CAM_SEC_4GROUP	: 0 - 1 unicast, 2 - 5 group, 6 BIP
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_add_cam_sec_mode {
    RTW89_ADDR_CAM_SEC_NONE		= 0,
    RTW89_ADDR_CAM_SEC_ALL_UNI	= 1,
    RTW89_ADDR_CAM_SEC_NORMAL	= 2,
    RTW89_ADDR_CAM_SEC_4GROUP	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_sec_key_type {
    RTW89_SEC_KEY_TYPE_NONE		= 0,
    RTW89_SEC_KEY_TYPE_WEP40	= 1,
    RTW89_SEC_KEY_TYPE_WEP104	= 2,
    RTW89_SEC_KEY_TYPE_TKIP		= 3,
    RTW89_SEC_KEY_TYPE_WAPI		= 4,
    RTW89_SEC_KEY_TYPE_GCMSMS4	= 5,
    RTW89_SEC_KEY_TYPE_CCMP128	= 6,
    RTW89_SEC_KEY_TYPE_CCMP256	= 7,
    RTW89_SEC_KEY_TYPE_GCMP128	= 8,
    RTW89_SEC_KEY_TYPE_GCMP256	= 9,
    RTW89_SEC_KEY_TYPE_BIP_CCMP128	= 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_port {
    RTW89_PORT_0 = 0,
    RTW89_PORT_1 = 1,
    RTW89_PORT_2 = 2,
    RTW89_PORT_3 = 3,
    RTW89_PORT_4 = 4,
    RTW89_PORT_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_band {
    RTW89_BAND_2G = 0,
    RTW89_BAND_5G = 1,
    RTW89_BAND_6G = 2,
    RTW89_BAND_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_hw_rate {
    RTW89_HW_RATE_CCK1	= 0x0,
    RTW89_HW_RATE_CCK2	= 0x1,
    RTW89_HW_RATE_CCK5_5	= 0x2,
    RTW89_HW_RATE_CCK11	= 0x3,
    RTW89_HW_RATE_OFDM6	= 0x4,
    RTW89_HW_RATE_OFDM9	= 0x5,
    RTW89_HW_RATE_OFDM12	= 0x6,
    RTW89_HW_RATE_OFDM18	= 0x7,
    RTW89_HW_RATE_OFDM24	= 0x8,
    RTW89_HW_RATE_OFDM36	= 0x9,
    RTW89_HW_RATE_OFDM48	= 0xA,
    RTW89_HW_RATE_OFDM54	= 0xB,
    RTW89_HW_RATE_MCS0	= 0x80,
    RTW89_HW_RATE_MCS1	= 0x81,
    RTW89_HW_RATE_MCS2	= 0x82,
    RTW89_HW_RATE_MCS3	= 0x83,
    RTW89_HW_RATE_MCS4	= 0x84,
    RTW89_HW_RATE_MCS5	= 0x85,
    RTW89_HW_RATE_MCS6	= 0x86,
    RTW89_HW_RATE_MCS7	= 0x87,
    RTW89_HW_RATE_MCS8	= 0x88,
    RTW89_HW_RATE_MCS9	= 0x89,
    RTW89_HW_RATE_MCS10	= 0x8A,
    RTW89_HW_RATE_MCS11	= 0x8B,
    RTW89_HW_RATE_MCS12	= 0x8C,
    RTW89_HW_RATE_MCS13	= 0x8D,
    RTW89_HW_RATE_MCS14	= 0x8E,
    RTW89_HW_RATE_MCS15	= 0x8F,
    RTW89_HW_RATE_MCS16	= 0x90,
    RTW89_HW_RATE_MCS17	= 0x91,
    RTW89_HW_RATE_MCS18	= 0x92,
    RTW89_HW_RATE_MCS19	= 0x93,
    RTW89_HW_RATE_MCS20	= 0x94,
    RTW89_HW_RATE_MCS21	= 0x95,
    RTW89_HW_RATE_MCS22	= 0x96,
    RTW89_HW_RATE_MCS23	= 0x97,
    RTW89_HW_RATE_MCS24	= 0x98,
    RTW89_HW_RATE_MCS25	= 0x99,
    RTW89_HW_RATE_MCS26	= 0x9A,
    RTW89_HW_RATE_MCS27	= 0x9B,
    RTW89_HW_RATE_MCS28	= 0x9C,
    RTW89_HW_RATE_MCS29	= 0x9D,
    RTW89_HW_RATE_MCS30	= 0x9E,
    RTW89_HW_RATE_MCS31	= 0x9F,
    RTW89_HW_RATE_VHT_NSS1_MCS0	= 0x100,
    RTW89_HW_RATE_VHT_NSS1_MCS1	= 0x101,
    RTW89_HW_RATE_VHT_NSS1_MCS2	= 0x102,
    RTW89_HW_RATE_VHT_NSS1_MCS3	= 0x103,
    RTW89_HW_RATE_VHT_NSS1_MCS4	= 0x104,
    RTW89_HW_RATE_VHT_NSS1_MCS5	= 0x105,
    RTW89_HW_RATE_VHT_NSS1_MCS6	= 0x106,
    RTW89_HW_RATE_VHT_NSS1_MCS7	= 0x107,
    RTW89_HW_RATE_VHT_NSS1_MCS8	= 0x108,
    RTW89_HW_RATE_VHT_NSS1_MCS9	= 0x109,
    RTW89_HW_RATE_VHT_NSS2_MCS0	= 0x110,
    RTW89_HW_RATE_VHT_NSS2_MCS1	= 0x111,
    RTW89_HW_RATE_VHT_NSS2_MCS2	= 0x112,
    RTW89_HW_RATE_VHT_NSS2_MCS3	= 0x113,
    RTW89_HW_RATE_VHT_NSS2_MCS4	= 0x114,
    RTW89_HW_RATE_VHT_NSS2_MCS5	= 0x115,
    RTW89_HW_RATE_VHT_NSS2_MCS6	= 0x116,
    RTW89_HW_RATE_VHT_NSS2_MCS7	= 0x117,
    RTW89_HW_RATE_VHT_NSS2_MCS8	= 0x118,
    RTW89_HW_RATE_VHT_NSS2_MCS9	= 0x119,
    RTW89_HW_RATE_VHT_NSS3_MCS0	= 0x120,
    RTW89_HW_RATE_VHT_NSS3_MCS1	= 0x121,
    RTW89_HW_RATE_VHT_NSS3_MCS2	= 0x122,
    RTW89_HW_RATE_VHT_NSS3_MCS3	= 0x123,
    RTW89_HW_RATE_VHT_NSS3_MCS4	= 0x124,
    RTW89_HW_RATE_VHT_NSS3_MCS5	= 0x125,
    RTW89_HW_RATE_VHT_NSS3_MCS6	= 0x126,
    RTW89_HW_RATE_VHT_NSS3_MCS7	= 0x127,
    RTW89_HW_RATE_VHT_NSS3_MCS8	= 0x128,
    RTW89_HW_RATE_VHT_NSS3_MCS9	= 0x129,
    RTW89_HW_RATE_VHT_NSS4_MCS0	= 0x130,
    RTW89_HW_RATE_VHT_NSS4_MCS1	= 0x131,
    RTW89_HW_RATE_VHT_NSS4_MCS2	= 0x132,
    RTW89_HW_RATE_VHT_NSS4_MCS3	= 0x133,
    RTW89_HW_RATE_VHT_NSS4_MCS4	= 0x134,
    RTW89_HW_RATE_VHT_NSS4_MCS5	= 0x135,
    RTW89_HW_RATE_VHT_NSS4_MCS6	= 0x136,
    RTW89_HW_RATE_VHT_NSS4_MCS7	= 0x137,
    RTW89_HW_RATE_VHT_NSS4_MCS8	= 0x138,
    RTW89_HW_RATE_VHT_NSS4_MCS9	= 0x139,
    RTW89_HW_RATE_HE_NSS1_MCS0	= 0x180,
    RTW89_HW_RATE_HE_NSS1_MCS1	= 0x181,
    RTW89_HW_RATE_HE_NSS1_MCS2	= 0x182,
    RTW89_HW_RATE_HE_NSS1_MCS3	= 0x183,
    RTW89_HW_RATE_HE_NSS1_MCS4	= 0x184,
    RTW89_HW_RATE_HE_NSS1_MCS5	= 0x185,
    RTW89_HW_RATE_HE_NSS1_MCS6	= 0x186,
    RTW89_HW_RATE_HE_NSS1_MCS7	= 0x187,
    RTW89_HW_RATE_HE_NSS1_MCS8	= 0x188,
    RTW89_HW_RATE_HE_NSS1_MCS9	= 0x189,
    RTW89_HW_RATE_HE_NSS1_MCS10	= 0x18A,
    RTW89_HW_RATE_HE_NSS1_MCS11	= 0x18B,
    RTW89_HW_RATE_HE_NSS2_MCS0	= 0x190,
    RTW89_HW_RATE_HE_NSS2_MCS1	= 0x191,
    RTW89_HW_RATE_HE_NSS2_MCS2	= 0x192,
    RTW89_HW_RATE_HE_NSS2_MCS3	= 0x193,
    RTW89_HW_RATE_HE_NSS2_MCS4	= 0x194,
    RTW89_HW_RATE_HE_NSS2_MCS5	= 0x195,
    RTW89_HW_RATE_HE_NSS2_MCS6	= 0x196,
    RTW89_HW_RATE_HE_NSS2_MCS7	= 0x197,
    RTW89_HW_RATE_HE_NSS2_MCS8	= 0x198,
    RTW89_HW_RATE_HE_NSS2_MCS9	= 0x199,
    RTW89_HW_RATE_HE_NSS2_MCS10	= 0x19A,
    RTW89_HW_RATE_HE_NSS2_MCS11	= 0x19B,
    RTW89_HW_RATE_HE_NSS3_MCS0	= 0x1A0,
    RTW89_HW_RATE_HE_NSS3_MCS1	= 0x1A1,
    RTW89_HW_RATE_HE_NSS3_MCS2	= 0x1A2,
    RTW89_HW_RATE_HE_NSS3_MCS3	= 0x1A3,
    RTW89_HW_RATE_HE_NSS3_MCS4	= 0x1A4,
    RTW89_HW_RATE_HE_NSS3_MCS5	= 0x1A5,
    RTW89_HW_RATE_HE_NSS3_MCS6	= 0x1A6,
    RTW89_HW_RATE_HE_NSS3_MCS7	= 0x1A7,
    RTW89_HW_RATE_HE_NSS3_MCS8	= 0x1A8,
    RTW89_HW_RATE_HE_NSS3_MCS9	= 0x1A9,
    RTW89_HW_RATE_HE_NSS3_MCS10	= 0x1AA,
    RTW89_HW_RATE_HE_NSS3_MCS11	= 0x1AB,
    RTW89_HW_RATE_HE_NSS4_MCS0	= 0x1B0,
    RTW89_HW_RATE_HE_NSS4_MCS1	= 0x1B1,
    RTW89_HW_RATE_HE_NSS4_MCS2	= 0x1B2,
    RTW89_HW_RATE_HE_NSS4_MCS3	= 0x1B3,
    RTW89_HW_RATE_HE_NSS4_MCS4	= 0x1B4,
    RTW89_HW_RATE_HE_NSS4_MCS5	= 0x1B5,
    RTW89_HW_RATE_HE_NSS4_MCS6	= 0x1B6,
    RTW89_HW_RATE_HE_NSS4_MCS7	= 0x1B7,
    RTW89_HW_RATE_HE_NSS4_MCS8	= 0x1B8,
    RTW89_HW_RATE_HE_NSS4_MCS9	= 0x1B9,
    RTW89_HW_RATE_HE_NSS4_MCS10	= 0x1BA,
    RTW89_HW_RATE_HE_NSS4_MCS11	= 0x1BB,

    RTW89_HW_RATE_V1_MCS0		= 0x100,
    RTW89_HW_RATE_V1_MCS1		= 0x101,
    RTW89_HW_RATE_V1_MCS2		= 0x102,
    RTW89_HW_RATE_V1_MCS3		= 0x103,
    RTW89_HW_RATE_V1_MCS4		= 0x104,
    RTW89_HW_RATE_V1_MCS5		= 0x105,
    RTW89_HW_RATE_V1_MCS6		= 0x106,
    RTW89_HW_RATE_V1_MCS7		= 0x107,
    RTW89_HW_RATE_V1_MCS8		= 0x108,
    RTW89_HW_RATE_V1_MCS9		= 0x109,
    RTW89_HW_RATE_V1_MCS10		= 0x10A,
    RTW89_HW_RATE_V1_MCS11		= 0x10B,
    RTW89_HW_RATE_V1_MCS12		= 0x10C,
    RTW89_HW_RATE_V1_MCS13		= 0x10D,
    RTW89_HW_RATE_V1_MCS14		= 0x10E,
    RTW89_HW_RATE_V1_MCS15		= 0x10F,
    RTW89_HW_RATE_V1_MCS16		= 0x110,
    RTW89_HW_RATE_V1_MCS17		= 0x111,
    RTW89_HW_RATE_V1_MCS18		= 0x112,
    RTW89_HW_RATE_V1_MCS19		= 0x113,
    RTW89_HW_RATE_V1_MCS20		= 0x114,
    RTW89_HW_RATE_V1_MCS21		= 0x115,
    RTW89_HW_RATE_V1_MCS22		= 0x116,
    RTW89_HW_RATE_V1_MCS23		= 0x117,
    RTW89_HW_RATE_V1_MCS24		= 0x118,
    RTW89_HW_RATE_V1_MCS25		= 0x119,
    RTW89_HW_RATE_V1_MCS26		= 0x11A,
    RTW89_HW_RATE_V1_MCS27		= 0x11B,
    RTW89_HW_RATE_V1_MCS28		= 0x11C,
    RTW89_HW_RATE_V1_MCS29		= 0x11D,
    RTW89_HW_RATE_V1_MCS30		= 0x11E,
    RTW89_HW_RATE_V1_MCS31		= 0x11F,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS0	= 0x200,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS1	= 0x201,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS2	= 0x202,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS3	= 0x203,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS4	= 0x204,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS5	= 0x205,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS6	= 0x206,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS7	= 0x207,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS8	= 0x208,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS9	= 0x209,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS10	= 0x20A,
    RTW89_HW_RATE_V1_VHT_NSS1_MCS11	= 0x20B,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS0	= 0x220,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS1	= 0x221,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS2	= 0x222,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS3	= 0x223,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS4	= 0x224,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS5	= 0x225,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS6	= 0x226,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS7	= 0x227,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS8	= 0x228,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS9	= 0x229,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS10	= 0x22A,
    RTW89_HW_RATE_V1_VHT_NSS2_MCS11	= 0x22B,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS0	= 0x240,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS1	= 0x241,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS2	= 0x242,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS3	= 0x243,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS4	= 0x244,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS5	= 0x245,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS6	= 0x246,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS7	= 0x247,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS8	= 0x248,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS9	= 0x249,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS10	= 0x24A,
    RTW89_HW_RATE_V1_VHT_NSS3_MCS11	= 0x24B,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS0	= 0x260,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS1	= 0x261,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS2	= 0x262,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS3	= 0x263,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS4	= 0x264,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS5	= 0x265,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS6	= 0x266,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS7	= 0x267,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS8	= 0x268,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS9	= 0x269,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS10	= 0x26A,
    RTW89_HW_RATE_V1_VHT_NSS4_MCS11	= 0x26B,
    RTW89_HW_RATE_V1_HE_NSS1_MCS0	= 0x300,
    RTW89_HW_RATE_V1_HE_NSS1_MCS1	= 0x301,
    RTW89_HW_RATE_V1_HE_NSS1_MCS2	= 0x302,
    RTW89_HW_RATE_V1_HE_NSS1_MCS3	= 0x303,
    RTW89_HW_RATE_V1_HE_NSS1_MCS4	= 0x304,
    RTW89_HW_RATE_V1_HE_NSS1_MCS5	= 0x305,
    RTW89_HW_RATE_V1_HE_NSS1_MCS6	= 0x306,
    RTW89_HW_RATE_V1_HE_NSS1_MCS7	= 0x307,
    RTW89_HW_RATE_V1_HE_NSS1_MCS8	= 0x308,
    RTW89_HW_RATE_V1_HE_NSS1_MCS9	= 0x309,
    RTW89_HW_RATE_V1_HE_NSS1_MCS10	= 0x30A,
    RTW89_HW_RATE_V1_HE_NSS1_MCS11	= 0x30B,
    RTW89_HW_RATE_V1_HE_NSS2_MCS0	= 0x320,
    RTW89_HW_RATE_V1_HE_NSS2_MCS1	= 0x321,
    RTW89_HW_RATE_V1_HE_NSS2_MCS2	= 0x322,
    RTW89_HW_RATE_V1_HE_NSS2_MCS3	= 0x323,
    RTW89_HW_RATE_V1_HE_NSS2_MCS4	= 0x324,
    RTW89_HW_RATE_V1_HE_NSS2_MCS5	= 0x325,
    RTW89_HW_RATE_V1_HE_NSS2_MCS6	= 0x326,
    RTW89_HW_RATE_V1_HE_NSS2_MCS7	= 0x327,
    RTW89_HW_RATE_V1_HE_NSS2_MCS8	= 0x328,
    RTW89_HW_RATE_V1_HE_NSS2_MCS9	= 0x329,
    RTW89_HW_RATE_V1_HE_NSS2_MCS10	= 0x32A,
    RTW89_HW_RATE_V1_HE_NSS2_MCS11	= 0x32B,
    RTW89_HW_RATE_V1_HE_NSS3_MCS0	= 0x340,
    RTW89_HW_RATE_V1_HE_NSS3_MCS1	= 0x341,
    RTW89_HW_RATE_V1_HE_NSS3_MCS2	= 0x342,
    RTW89_HW_RATE_V1_HE_NSS3_MCS3	= 0x343,
    RTW89_HW_RATE_V1_HE_NSS3_MCS4	= 0x344,
    RTW89_HW_RATE_V1_HE_NSS3_MCS5	= 0x345,
    RTW89_HW_RATE_V1_HE_NSS3_MCS6	= 0x346,
    RTW89_HW_RATE_V1_HE_NSS3_MCS7	= 0x347,
    RTW89_HW_RATE_V1_HE_NSS3_MCS8	= 0x348,
    RTW89_HW_RATE_V1_HE_NSS3_MCS9	= 0x349,
    RTW89_HW_RATE_V1_HE_NSS3_MCS10	= 0x34A,
    RTW89_HW_RATE_V1_HE_NSS3_MCS11	= 0x34B,
    RTW89_HW_RATE_V1_HE_NSS4_MCS0	= 0x360,
    RTW89_HW_RATE_V1_HE_NSS4_MCS1	= 0x361,
    RTW89_HW_RATE_V1_HE_NSS4_MCS2	= 0x362,
    RTW89_HW_RATE_V1_HE_NSS4_MCS3	= 0x363,
    RTW89_HW_RATE_V1_HE_NSS4_MCS4	= 0x364,
    RTW89_HW_RATE_V1_HE_NSS4_MCS5	= 0x365,
    RTW89_HW_RATE_V1_HE_NSS4_MCS6	= 0x366,
    RTW89_HW_RATE_V1_HE_NSS4_MCS7	= 0x367,
    RTW89_HW_RATE_V1_HE_NSS4_MCS8	= 0x368,
    RTW89_HW_RATE_V1_HE_NSS4_MCS9	= 0x369,
    RTW89_HW_RATE_V1_HE_NSS4_MCS10	= 0x36A,
    RTW89_HW_RATE_V1_HE_NSS4_MCS11	= 0x36B,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS0	= 0x400,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS1	= 0x401,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS2	= 0x402,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS3	= 0x403,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS4	= 0x404,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS5	= 0x405,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS6	= 0x406,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS7	= 0x407,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS8	= 0x408,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS9	= 0x409,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS10	= 0x40A,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS11	= 0x40B,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS12	= 0x40C,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS13	= 0x40D,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS14	= 0x40E,
    RTW89_HW_RATE_V1_EHT_NSS1_MCS15	= 0x40F,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS0	= 0x420,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS1	= 0x421,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS2	= 0x422,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS3	= 0x423,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS4	= 0x424,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS5	= 0x425,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS6	= 0x426,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS7	= 0x427,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS8	= 0x428,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS9	= 0x429,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS10	= 0x42A,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS11	= 0x42B,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS12	= 0x42C,
    RTW89_HW_RATE_V1_EHT_NSS2_MCS13	= 0x42D,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS0	= 0x440,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS1	= 0x441,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS2	= 0x442,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS3	= 0x443,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS4	= 0x444,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS5	= 0x445,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS6	= 0x446,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS7	= 0x447,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS8	= 0x448,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS9	= 0x449,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS10	= 0x44A,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS11	= 0x44B,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS12	= 0x44C,
    RTW89_HW_RATE_V1_EHT_NSS3_MCS13	= 0x44D,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS0	= 0x460,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS1	= 0x461,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS2	= 0x462,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS3	= 0x463,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS4	= 0x464,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS5	= 0x465,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS6	= 0x466,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS7	= 0x467,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS8	= 0x468,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS9	= 0x469,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS10	= 0x46A,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS11	= 0x46B,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS12	= 0x46C,
    RTW89_HW_RATE_V1_EHT_NSS4_MCS13	= 0x46D,

    RTW89_HW_RATE_NR,
    RTW89_HW_RATE_INVAL,

    RTW89_HW_RATE_MASK_MOD = GENMASK(8, 7),
    RTW89_HW_RATE_MASK_VAL = GENMASK(6, 0),
    RTW89_HW_RATE_V1_MASK_MOD = GENMASK(10, 8),
    RTW89_HW_RATE_V1_MASK_VAL = GENMASK(7, 0),
}

// 2G channels,
// 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
//
pub const RTW89_2G_CH_NUM: c_int = 14;
// 5G channels,
// 36, 38, 40, 42, 44, 46, 48, 50,
// 52, 54, 56, 58, 60, 62, 64,
// 100, 102, 104, 106, 108, 110, 112, 114,
// 116, 118, 120, 122, 124, 126, 128, 130,
// 132, 134, 136, 138, 140, 142, 144,
// 149, 151, 153, 155, 157, 159, 161, 163,
// 165, 167, 169, 171, 173, 175, 177
//
pub const RTW89_5G_CH_NUM: c_int = 53;
// 6G channels,
// 1, 3, 5, 7, 9, 11, 13, 15,
// 17, 19, 21, 23, 25, 27, 29, 33,
// 35, 37, 39, 41, 43, 45, 47, 49,
// 51, 53, 55, 57, 59, 61, 65, 67,
// 69, 71, 73, 75, 77, 79, 81, 83,
// 85, 87, 89, 91, 93, 97, 99, 101,
// 103, 105, 107, 109, 111, 113, 115, 117,
// 119, 121, 123, 125, 129, 131, 133, 135,
// 137, 139, 141, 143, 145, 147, 149, 151,
// 153, 155, 157, 161, 163, 165, 167, 169,
// 171, 173, 175, 177, 179, 181, 183, 185,
// 187, 189, 193, 195, 197, 199, 201, 203,
// 205, 207, 209, 211, 213, 215, 217, 219,
// 221, 225, 227, 229, 231, 233, 235, 237,
// 239, 241, 243, 245, 247, 249, 251, 253,
//
pub const RTW89_6G_CH_NUM: c_int = 120;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rate_section {
    RTW89_RS_CCK,
    RTW89_RS_OFDM,
    RTW89_RS_MCS, /* for HT/VHT/HE */
    RTW89_RS_HEDCM,
    RTW89_RS_OFFSET,
    RTW89_RS_NUM,
    RTW89_RS_LMT_NUM = RTW89_RS_MCS + 1,
    RTW89_RS_TX_SHAPE_NUM = RTW89_RS_OFDM + 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rate_offset_indexes {
    RTW89_RATE_OFFSET_HE,
    RTW89_RATE_OFFSET_VHT,
    RTW89_RATE_OFFSET_HT,
    RTW89_RATE_OFFSET_OFDM,
    RTW89_RATE_OFFSET_CCK,
    RTW89_RATE_OFFSET_DLRU_EHT,
    RTW89_RATE_OFFSET_DLRU_HE,
    RTW89_RATE_OFFSET_EHT,
    __RTW89_RATE_OFFSET_NUM,

    RTW89_RATE_OFFSET_NUM_AX = RTW89_RATE_OFFSET_CCK + 1,
    RTW89_RATE_OFFSET_NUM_BE = RTW89_RATE_OFFSET_EHT + 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rate_num {
    RTW89_RATE_CCK_NUM	= 4,
    RTW89_RATE_OFDM_NUM	= 8,
    RTW89_RATE_HEDCM_NUM	= 4, /* for HEDCM MCS0/1/3/4 */

    RTW89_RATE_MCS_NUM_AX	= 12,
    RTW89_RATE_MCS_NUM_BE	= 16,
    __RTW89_RATE_MCS_NUM	= 16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_nss {
    RTW89_NSS_1		= 0,
    RTW89_NSS_2		= 1,
// HE DCM only support 1ss and 2ss
    RTW89_NSS_HEDCM_NUM	= RTW89_NSS_2 + 1,
    RTW89_NSS_3		= 2,
    RTW89_NSS_4		= 3,
    RTW89_NSS_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ntx {
    RTW89_1TX	= 0,
    RTW89_2TX	= 1,
    RTW89_NTX_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_beamforming_type {
    RTW89_NONBF	= 0,
    RTW89_BF	= 1,
    RTW89_BF_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ofdma_type {
    RTW89_NON_OFDMA	= 0,
    RTW89_OFDMA	= 1,
    RTW89_OFDMA_NUM,
}

// neither insert new in the middle, nor change any given definition
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_regulation_type {
    RTW89_WW	= 0,
    RTW89_ETSI	= 1,
    RTW89_FCC	= 2,
    RTW89_MKK	= 3,
    RTW89_NA	= 4,
    RTW89_IC	= 5,
    RTW89_KCC	= 6,
    RTW89_ACMA	= 7,
    RTW89_NCC	= 8,
    RTW89_MEXICO	= 9,
    RTW89_CHILE	= 10,
    RTW89_UKRAINE	= 11,
    RTW89_CN	= 12,
    RTW89_QATAR	= 13,
    RTW89_UK	= 14,
    RTW89_THAILAND	= 15,
    RTW89_REGD_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_reg_6ghz_power {
    RTW89_REG_6GHZ_POWER_VLP = 0,
    RTW89_REG_6GHZ_POWER_LPI = 1,
    RTW89_REG_6GHZ_POWER_STD = 2,

    NUM_OF_RTW89_REG_6GHZ_POWER,
    RTW89_REG_6GHZ_POWER_DFLT = RTW89_REG_6GHZ_POWER_VLP,
}

// calculate based on ieee80211 Transmit Power Envelope
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_reg_6ghz_tpe {
    pub valid: bool,
    pub /: *mut *mut s8 constraint; / unit: dBm,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_pkt_ofld_type {
    RTW89_PKT_OFLD_TYPE_PROBE_RSP = 0,
    RTW89_PKT_OFLD_TYPE_PS_POLL = 1,
    RTW89_PKT_OFLD_TYPE_NULL_DATA = 2,
    RTW89_PKT_OFLD_TYPE_QOS_NULL = 3,
    RTW89_PKT_OFLD_TYPE_CTS2SELF = 4,
    RTW89_PKT_OFLD_TYPE_ARP_RSP = 5,
    RTW89_PKT_OFLD_TYPE_NDP = 6,
    RTW89_PKT_OFLD_TYPE_EAPOL_KEY = 7,
    RTW89_PKT_OFLD_TYPE_SA_QUERY = 8,
    RTW89_PKT_OFLD_TYPE_PROBE_REQ = 12,
    RTW89_PKT_OFLD_TYPE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_byrate {
    pub cck: [i8; RTW89_RATE_CCK_NUM],
    pub ofdm: [i8; RTW89_RATE_OFDM_NUM],
    pub mcs: [i8; RTW89_OFDMA_NUM][RTW89_NSS_NUM][__RTW89_RATE_MCS_NUM],
    pub hedcm: [i8; RTW89_OFDMA_NUM][RTW89_NSS_HEDCM_NUM][RTW89_RATE_HEDCM_NUM],
    pub offset: [i8; __RTW89_RATE_OFFSET_NUM],
    pub trap: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rate_desc {
    pub nss: rtw89_nss,
    pub rs: rtw89_rate_section,
    pub ofdma: rtw89_ofdma_type,
    pub idx: u8,
}

pub const PHY_STS_HDR_LEN: c_int = 8;
pub const RF_PATH_MAX: c_int = 4;
pub const RTW89_MAX_PPDU_CNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rx_phy_ppdu {
    pub buf: *mut c_void,
    pub len: u32,
    pub rssi_avg: u8,
    pub rssi: [u8; RF_PATH_MAX],
    pub mac_id: u8,
    pub chan_idx: u8,
    pub phy_idx: u8,
    pub /: *mut *mut u8 ie; / enum rtw89_phy_status_bitmap,
    pub rate: u16,
    pub rpl_avg: u8,
    pub rpl_path: [u8; RF_PATH_MAX],
    pub rpl_fd: [u8; RF_PATH_MAX],
    pub bw_idx: u8,
    pub rx_path_en: u8,
    pub has: bool,
    pub avg_snr: u8,
    pub evm_max: u8,
    pub evm_min: u8,
    pub ofdm: },
    pub has_data: bool,
    pub has_bcn: bool,
    pub su: bool,
    pub ldpc: bool,
    pub stbc: bool,
    pub bf: bool,
    pub to_self: bool,
    pub valid: bool,
    pub hdr_2_en: bool,
    pub /: *const *const *const rtw89_phy_sts_ie09 ie09; / SIG-A,
    pub /: *const *const *const rtw89_phy_sts_ie10 ie10; / SIG-B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_idx {
    RTW89_MAC_0 = 0,
    RTW89_MAC_1 = 1,
    RTW89_MAC_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_idx {
    RTW89_PHY_0 = 0,
    RTW89_PHY_1 = 1,
    RTW89_PHY_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fbtc_bt_index {
    BTC_BT_1ST = 0x0,
    BTC_BT_2ND = 0x1,
    BTC_BT_EXT = 0x2,
    BTC_ALL_BT = 0x2,
    BTC_ALL_BT_EZL = 0x3 /* BT0+BT1+Ext-ZB(or Thread, or LTE) */
}

pub const __RTW89_MLD_MAX_LINK_NUM: c_int = 2;
pub const RTW89_MLD_NON_STA_LINK_NUM: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_chanctx_idx {
    RTW89_CHANCTX_0 = 0,
    RTW89_CHANCTX_1 = 1,

    NUM_OF_RTW89_CHANCTX,
    RTW89_CHANCTX_IDLE = NUM_OF_RTW89_CHANCTX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rf_path {
    RF_PATH_A = 0,
    RF_PATH_B = 1,
    RF_PATH_C = 2,
    RF_PATH_D = 3,
    RF_PATH_AB,
    RF_PATH_AC,
    RF_PATH_AD,
    RF_PATH_BC,
    RF_PATH_BD,
    RF_PATH_CD,
    RF_PATH_ABC,
    RF_PATH_ABD,
    RF_PATH_ACD,
    RF_PATH_BCD,
    RF_PATH_ABCD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rf_path_bit {
    RF_A	= BIT(0),
    RF_B	= BIT(1),
    RF_C	= BIT(2),
    RF_D	= BIT(3),

    RF_AB	= (RF_A | RF_B),
    RF_AC	= (RF_A | RF_C),
    RF_AD	= (RF_A | RF_D),
    RF_BC	= (RF_B | RF_C),
    RF_BD	= (RF_B | RF_D),
    RF_CD	= (RF_C | RF_D),

    RF_ABC	= (RF_A | RF_B | RF_C),
    RF_ABD	= (RF_A | RF_B | RF_D),
    RF_ACD	= (RF_A | RF_C | RF_D),
    RF_BCD	= (RF_B | RF_C | RF_D),

    RF_ABCD	= (RF_A | RF_B | RF_C | RF_D),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bandwidth {
    RTW89_CHANNEL_WIDTH_20	= 0,
    RTW89_CHANNEL_WIDTH_40	= 1,
    RTW89_CHANNEL_WIDTH_80	= 2,
    RTW89_CHANNEL_WIDTH_160	= 3,
    RTW89_CHANNEL_WIDTH_320	= 4,

// keep index order above
    RTW89_CHANNEL_WIDTH_ORDINARY_NUM = 5,

    RTW89_CHANNEL_WIDTH_80_80 = 5,
    RTW89_CHANNEL_WIDTH_5 = 6,
    RTW89_CHANNEL_WIDTH_10 = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rx_ppdu_type {
    RTW89_RX_PPDU_T_LCCK = 0,
    RTW89_RX_PPDU_T_SCCK = 1,
    RTW89_RX_PPDU_T_OFDM = 2,
    RTW89_RX_PPDU_T_HT = 3,
    RTW89_RX_PPDU_T_HTGF = 4,
    RTW89_RX_PPDU_T_VHT_SU = 5,
    RTW89_RX_PPDU_T_VHT_MU = 6,
    RTW89_RX_PPDU_T_HE_SU = 7,
    RTW89_RX_PPDU_T_HE_ERSU = 8,
    RTW89_RX_PPDU_T_HE_MU = 9,
    RTW89_RX_PPDU_T_HE_TB = 10,
    RTW89_RX_PPDU_T_UNKNOWN = 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ps_mode {
    RTW89_PS_MODE_NONE	= 0,
    RTW89_PS_MODE_RFOFF	= 1,
    RTW89_PS_MODE_CLK_GATED	= 2,
    RTW89_PS_MODE_PWR_GATED	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_pe_duration {
    RTW89_PE_DURATION_0 = 0,
    RTW89_PE_DURATION_8 = 1,
    RTW89_PE_DURATION_16 = 2,
    RTW89_PE_DURATION_16_20 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ru_bandwidth {
    RTW89_RU26 = 0,
    RTW89_RU52 = 1,
    RTW89_RU106 = 2,
    RTW89_RU52_26 = 3,
    RTW89_RU106_26 = 4,
    RTW89_RU484_242 = 5,
    RTW89_RU996_484 = 6,
    RTW89_RU996_484_242 = 7,
    RTW89_RU_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_sc_offset {
    RTW89_SC_DONT_CARE	= 0,
    RTW89_SC_20_UPPER	= 1,
    RTW89_SC_20_LOWER	= 2,
    RTW89_SC_20_UPMOST	= 3,
    RTW89_SC_20_LOWEST	= 4,
    RTW89_SC_20_UP2X	= 5,
    RTW89_SC_20_LOW2X	= 6,
    RTW89_SC_20_UP3X	= 7,
    RTW89_SC_20_LOW3X	= 8,
    RTW89_SC_40_UPPER	= 9,
    RTW89_SC_40_LOWER	= 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rfsi_ctrl_band {
    RFSI_CTRL_BAND_5_6GHZ,
    RFSI_CTRL_BAND_2GHZ,

    RFSI_CTRL_BAND_NUM,
}

// only mgd features can be added to the enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_wow_flags {
    RTW89_WOW_FLAG_EN_MAGIC_PKT,
    RTW89_WOW_FLAG_EN_REKEY_PKT,
    RTW89_WOW_FLAG_EN_DISCONNECT,
    RTW89_WOW_FLAG_EN_PATTERN,
    RTW89_WOW_FLAG_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chan {
    pub channel: u8,
    pub primary_channel: u8,
    pub band_type: rtw89_band,
    pub band_width: rtw89_bandwidth,
// The follow-up are derived from the above. We must ensure that it
// is assigned correctly in rtw89_chan_create() if new one is added.
//
    pub freq: u32,
    pub subband_type: rtw89_subband,
    pub tx_comp_band: rtw89_tx_comp_band,
    pub pri_ch_idx: rtw89_sc_offset,
    pub pri_sb_idx: u8,
    pub rfsi_band: rtw89_rfsi_ctrl_band,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chan_rcd {
    pub prev_primary_channel: u8,
    pub prev_band_type: rtw89_band,
    pub band_changed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_channel_help_params {
    pub tx_en: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_port_reg {
    pub port_cfg: u32,
    pub tbtt_prohib: u32,
    pub bcn_area: u32,
    pub bcn_early: u32,
    pub tbtt_early: u32,
    pub tbtt_agg: u32,
    pub bcn_space: u32,
    pub bcn_forcetx: u32,
    pub bcn_err_cnt: u32,
    pub bcn_err_flag: u32,
    pub dtim_ctrl: u32,
    pub tbtt_shift: u32,
    pub bcn_cnt_tmr: u32,
    pub tsftr_l: u32,
    pub tsftr_h: u32,
    pub md_tsft: u32,
    pub bss_color: u32,
    pub mbssid: u32,
    pub mbssid_drop: u32,
    pub tsf_sync: u32,
    pub ptcl_dbg: u32,
    pub ptcl_dbg_info: u32,
    pub bcn_drop_all: u32,
    pub bcn_psr_rpt: u32,
    pub hiq_win: [u32; RTW89_PORT_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txwd_body {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txwd_body_v1 {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub dword6: __le32,
    pub dword7: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txwd_body_v2 {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub dword6: __le32,
    pub dword7: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txwd_info {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txwd_info_v2 {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub dword6: __le32,
    pub dword7: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rx_desc_info {
    pub pkt_size: u16,
    pub pkt_type: u8,
    pub drv_info_size: u8,
    pub phy_rpt_size: u8,
    pub hdr_cnv_size: u8,
    pub shift: u8,
    pub wl_hd_iv_len: u8,
    pub long_rxdesc: bool,
    pub bb_sel: bool,
    pub mac_info_valid: bool,
    pub data_rate: u16,
    pub gi_ltf: u8,
    pub bw: u8,
    pub free_run_cnt: u32,
    pub user_id: u8,
    pub sr_en: bool,
    pub ppdu_cnt: u8,
    pub ppdu_type: u8,
    pub icv_err: bool,
    pub crc32_err: bool,
    pub hw_dec: bool,
    pub sw_dec: bool,
    pub addr1_match: bool,
    pub ampdu: bool,
    pub frag: u8,
    pub seq: u16,
    pub frame_type: u8,
    pub rx_pl_id: u8,
    pub addr_cam_valid: bool,
    pub addr_cam_id: u8,
    pub sec_cam_id: u8,
    pub sec_type: u8,
    pub mac_id: u8,
    pub offset: u16,
    pub rxd_len: u16,
    pub ready: bool,
    pub rssi: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rxdesc_short {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rxdesc_short_v2 {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rxdesc_short_v3 {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rxdesc_long {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub dword6: __le32,
    pub dword7: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rxdesc_long_v2 {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub dword6: __le32,
    pub dword7: __le32,
    pub dword8: __le32,
    pub dword9: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rxdesc_long_v3 {
    pub dword0: __le32,
    pub dword1: __le32,
    pub dword2: __le32,
    pub dword3: __le32,
    pub dword4: __le32,
    pub dword5: __le32,
    pub dword6: __le32,
    pub dword7: __le32,
    pub dword8: __le32,
    pub dword9: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rxdesc_phy_rpt_v2 {
    pub dword0: __le32,
    pub dword1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tx_desc_info {
    pub pkt_size: u16,
    pub wp_offset: u8,
    pub mac_id: u8,
    pub qsel: u8,
    pub ch_dma: u8,
    pub hdr_llc_len: u8,
    pub is_bmc: bool,
    pub en_wd_info: bool,
    pub wd_page: bool,
    pub use_rate: bool,
    pub dis_data_fb: bool,
    pub tid_indicate: bool,
    pub agg_en: bool,
    pub bk: bool,
    pub ampdu_density: u8,
    pub ampdu_num: u8,
    pub sec_en: bool,
    pub report: bool,
    pub tx_cnt_lmt_en: bool,
    pub 4: u8 sn:,
    pub 6: u8 tx_cnt_lmt:,
    pub addr_info_nr: u8,
    pub sec_keyid: u8,
    pub sec_type: u8,
    pub sec_cam_idx: u8,
    pub sec_seq: [u8; 6],
    pub data_rate: u16,
    pub data_retry_lowest_rate: u16,
    pub data_bw: u8,
    pub gi_ltf: u8,
    pub fw_dl: bool,
    pub seq: u16,
    pub a_ctrl_bsr: bool,
    pub hw_ssn_sel: u8,
pub const RTW89_MGMT_HW_SSN_SEL: c_int = 1;
    pub hw_seq_mode: u8,
pub const RTW89_MGMT_HW_SEQ_MODE: c_int = 1;
    pub hiq: bool,
    pub port: u8,
    pub er_cap: bool,
    pub stbc: bool,
    pub ldpc: bool,
    pub upd_wlan_hdr: bool,
    pub mlo: bool,
    pub sw_mld: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_core_tx_request {
    pub tx_type: rtw89_core_tx_type,
    pub skb: *mut sk_buff,
    pub vif: *mut ieee80211_vif,
    pub sta: *mut ieee80211_sta,
    pub rtwvif_link: *mut rtw89_vif_link,
    pub rtwsta_link: *mut rtw89_sta_link,
    pub desc_info: rtw89_tx_desc_info,
    pub with_wait: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txq {
    pub list: list_head,
    pub flags: c_ulong,
    pub wait_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_ax_gnt {
    pub gnt_bt_sw_en: u8,
    pub gnt_bt: u8,
    pub gnt_wl_sw_en: u8,
    pub gnt_wl: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_gnt_ctrl {
    pub gnt_zb_sw_en: u8,
    pub gnt_zb: u8,
    pub gnt_bt1_sw_en: u8,
    pub gnt_bt1: u8,
    pub gnt_bt0_sw_en: u8,
    pub gnt_bt0: u8,
    pub gnt_wl_sw_en: u8,
    pub gnt_wl: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_ax_wl_act {
    pub wlan_act_en: u8,
    pub wlan_act: u8,
    pub __packed: },
pub const RTW89_MAC_AX_COEX_GNT_NR: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_ax_coex_gnt {
    pub band: [rtw89_mac_ax_gnt; RTW89_MAC_AX_COEX_GNT_NR],
    pub bt: [rtw89_mac_ax_wl_act; RTW89_MAC_AX_COEX_GNT_NR],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_ncnt {
    BTC_NCNT_POWER_ON = 0x0,
    BTC_NCNT_POWER_OFF,
    BTC_NCNT_INIT_COEX,
    BTC_NCNT_SCAN_START,
    BTC_NCNT_SCAN_FINISH,
    BTC_NCNT_SPECIAL_PACKET,
    BTC_NCNT_SWITCH_BAND,
    BTC_NCNT_RFK_TIMEOUT,
    BTC_NCNT_SHOW_COEX_INFO,
    BTC_NCNT_ROLE_INFO,
    BTC_NCNT_CONTROL,
    BTC_NCNT_RADIO_STATE,
    BTC_NCNT_CUSTOMERIZE,
    BTC_NCNT_WL_RFK,
    BTC_NCNT_WL_STA,
    BTC_NCNT_WL_STA_LAST,
    BTC_NCNT_FWINFO,
    BTC_NCNT_TIMER,
    BTC_NCNT_SWITCH_CHBW,
    BTC_NCNT_RESUME_DL_FW,
    BTC_NCNT_COUNTRYCODE,
    BTC_NCNT_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_btinfo {
    BTC_BTINFO_L0 = 0,
    BTC_BTINFO_L1,
    BTC_BTINFO_L2,
    BTC_BTINFO_L3,
    BTC_BTINFO_H0,
    BTC_BTINFO_H1,
    BTC_BTINFO_H2,
    BTC_BTINFO_H3,
    BTC_BTINFO_MAX
}

pub const BTC_BTINFO_BISTDMA: c_uint = 0x48 /* cmd value that identifies BISTDMA data */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_dcnt {
    BTC_DCNT_RUN = 0x0,
    BTC_DCNT_CX_RUNINFO,
    BTC_DCNT_RPT,
    BTC_DCNT_RPT_HANG,
    BTC_DCNT_CYCLE,
    BTC_DCNT_CYCLE_HANG,
    BTC_DCNT_W1,
    BTC_DCNT_W1_HANG,
    BTC_DCNT_B1,
    BTC_DCNT_B1_HANG,
    BTC_DCNT_TDMA_NONSYNC,
    BTC_DCNT_SLOT_NONSYNC,
    BTC_DCNT_BTCNT_HANG,
    BTC_DCNT_BTTX_HANG,
    BTC_DCNT_WL_SLOT_DRIFT,
    BTC_DCNT_WL_STA_LAST,
    BTC_DCNT_BT_SLOT_DRIFT,
    BTC_DCNT_BT_SLOT_FLOOD,
    BTC_DCNT_FDDT_TRIG,
    BTC_DCNT_E2G,
    BTC_DCNT_E2G_HANG,
    BTC_DCNT_WL_FW_VER_MATCH,
    BTC_DCNT_NULL_TX_FAIL,
    BTC_DCNT_WL_STA_NTFY,
    BTC_DCNT_W2B_SCBD_NOSYNC,
    BTC_DCNT_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_wl_state_cnt {
    BTC_WCNT_SCANAP = 0x0,
    BTC_WCNT_DHCP,
    BTC_WCNT_EAPOL,
    BTC_WCNT_ARP,
    BTC_WCNT_SCBDUPDATE,
    BTC_WCNT_RFK_REQ,
    BTC_WCNT_RFK_GO,
    BTC_WCNT_RFK_REJECT,
    BTC_WCNT_RFK_TIMEOUT,
    BTC_WCNT_CH_UPDATE,
    BTC_WCNT_DBCC_ALL_2G,
    BTC_WCNT_DBCC_CHG,
    BTC_WCNT_RX_OK_LAST,
    BTC_WCNT_RX_OK_LAST2S,
    BTC_WCNT_RX_ERR_LAST,
    BTC_WCNT_RX_ERR_LAST2S,
    BTC_WCNT_RX_LAST,
    BTC_WCNT_SCBDUPDATE2,
    BTC_WCNT_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_bt_state_cnt {
    BTC_BCNT_RETRY = 0x0,
    BTC_BCNT_REINIT,
    BTC_BCNT_REENABLE,
    BTC_BCNT_SCBDREAD,
    BTC_BCNT_RELINK,
    BTC_BCNT_IGNOWL,
    BTC_BCNT_INQPAG,
    BTC_BCNT_INQ,
    BTC_BCNT_PAGE,
    BTC_BCNT_ROLESW,
    BTC_BCNT_AFH,
    BTC_BCNT_INFOUPDATE,
    BTC_BCNT_LEAUDIO_INFOUPDATE,
    BTC_BCNT_INFOSAME,
    BTC_BCNT_LEAUDIO_INFOSAME,
    BTC_BCNT_SCBDUPDATE,
    BTC_BCNT_HIPRI_TX,
    BTC_BCNT_HIPRI_RX,
    BTC_BCNT_LOPRI_TX,
    BTC_BCNT_LOPRI_RX,
    BTC_BCNT_POLUT_NOW,
    BTC_BCNT_POLUT_DIFF,
    BTC_BCNT_RATECHG,
    BTC_BCNT_AFH_CONFLICT,
    BTC_BCNT_AFH_LE_CONFLICT,
    BTC_BCNT_AFH_UPDATE,
    BTC_BCNT_AFH_LE_UPDATE,
    BTC_BCNT_AFH_CHN,
    BTC_BCNT_AFH_LE_CHN,
    BTC_BCNT_TXPWR_UPDATE,
    BTC_BCNT_PROTECT,
    BTC_BCNT_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_bt_mech_type {
    BTC_MECH_TDD = 0,
    BTC_MECH_FDD = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_bt_rf_band {
    BTC_BT_B2G = 0x0, /* 2.4GHz */
    BTC_BT_B5G = 0x1, /* 5GHz or 6GHz */
    BTC_BT_BMAX = 0x2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_io_offload_type {
    BTC_IO_OFLD_NO_SUPPORT = 0,
    BTC_IO_OFLD_MAC_API = 1,
    BTC_IO_OFLD_BTC_H2C = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_bt_profile {
    BTC_BT_NOPROFILE = 0,
    BTC_BT_HFP = BIT(0),
    BTC_BT_HID = BIT(1),
    BTC_BT_A2DP = BIT(2),
    BTC_BT_PAN = BIT(3),
    BTC_BT_BIS = BIT(4),
    BTC_BT_CIS = BIT(5),
    BTC_BT_THREAD = BIT(6),
    BTC_BT_ULL = BIT(7),
    BTC_BT_LEGACY = 0xf,
    BTC_BT_FULL = 0x3f,
    BTC_PROFILE_MAX = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ant_info_v0 {
    pub /: *mut *mut u8 type; / shared, dedicated,
    pub num: u8,
    pub isolation: u8,
    pub /: *mut *mut u8 single_pos: 1;/ Single antenna at S0 or S1,
    pub 1: u8 diversity:,
    pub 2: u8 btg_pos:,
    pub 4: u8 stream_cnt:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ant_info_v7 {
    pub /: *mut *mut u8 type; / shared, dedicated(non-shared),
    pub /: *mut *mut u8 num; / antenna count,
    pub isolation: u8,
    pub /: *mut *mut u8 single_pos;/ wifi 1ss-1ant at 0:S0 or 1:S1,
    pub /: *mut *mut u8 diversity; / only for wifi use 1-antenna,
    pub /: *mut *mut u8 btg_pos; / btg-circuit at 0:S0/1:S1/others:all,
    pub /: *mut *mut u8 stream_cnt; / spatial_stream count,
    pub rsvd: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ant_info_v10 {
    pub /: *mut *mut u8 type; / shared, dedicated(non-shared),
    pub /: *mut *mut u8 num; / antenna count,
    pub /: *mut *mut u8 isolation; / Ant-Iso between WL/BT,
    pub /: *mut *mut u8 single_pos; / wifi 1ss-1ant at 0:S0 or 1:S1,
    pub /: *mut *mut u8 stream_cnt; / spatial_stream count: Tx[7:4], Rx[3:0],
    pub /: *mut *mut u8 btg_pos; / BT0 btg-circuit at 0:WL-S0/1:WL-S1,
    pub /: *mut *mut u8 btg1_pos; / BT1 btg-circuit at 0:WL-S0/1:WL-S1,
    pub /: *mut *mut u8 func[5]; / function at 1~5 Ant refer to enum btc_bt_func_type,
    pub ant_xmap: [u8; 2][4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ant_info_v11 {
    pub type: u8,
    pub num: u8,
    pub isolation: u8,
    pub single_pos: u8,
    pub stream_cnt: u8,
    pub /: *mut *mut u8 path_pos; / WL path position: Tx[7:4], Rx[3:0],
    pub btg_pos: u8,
    pub btg1_pos: u8,
    pub func: [u8; 5],
    pub ant_xmap: [u8; 2][4],
    pub rsvd0: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ant_info {
    pub /: *mut *mut u8 type; / shared, dedicated(non-shared),
    pub /: *mut *mut u8 num; / antenna count,
    pub /: *mut *mut u8 isolation; / Ant-Iso between WL/BT,
    pub /: *mut *mut u8 single_pos; / wifi 1ss-1ant at 0:S0 or 1:S1,
    pub /: *mut *mut u8 stream_cnt; / spatial_stream count: Tx[7:4], Rx[3:0],
    pub /: *mut *mut u8 btg_pos; / BT0 btg-circuit at 0:WL-S0/1:WL-S1,
    pub /: *mut *mut u8 btg1_pos; / BT1 btg-circuit at 0:WL-S0/1:WL-S1,
    pub /: *mut *mut u8 path_pos; / WL path position: Tx[7:4], Rx[3:0],
    pub /: *mut *mut u8 func[5]; / function at 1~5 Ant refer to enum btc_bt_func_type,
    pub ant_xmap: [u8; 2][4],
    pub /: *mut *mut u8 diversity; / only for wifi use 1-antenna,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tfc_dir {
    RTW89_TFC_UL,
    RTW89_TFC_DL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_smap {
    pub 1: u32 busy:,
    pub 1: u32 scan:,
    pub 1: u32 dhcp:,
    pub 1: u32 roaming:,
    pub 1: u32 transacting:,
    pub 1: u32 _4way:,
    pub 1: u32 handshake:,
    pub 1: u32 rf_off:,
    pub 1: u32 rf_off_pre:,
    pub 1: u32 ips:,
    pub 2: u32 lps:,
    pub 2: u32 lps_pre:,
    pub 1: u32 lps_exiting:,
    pub 1: u32 emlsr:,
    pub 1: u32 init_ok:,
    pub 2: u32 traffic_dir :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tfc_interval {
    RTW89_TFC_INTERVAL_100MS,
    RTW89_TFC_INTERVAL_2SEC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tfc_lv {
    RTW89_TFC_IDLE,
    RTW89_TFC_ULTRA_LOW,
    RTW89_TFC_LOW,
    RTW89_TFC_MID,
    RTW89_TFC_HIGH,
}

pub const RTW89_TCP_TH: c_int = 40;
pub const RTW89_UDP_RATIO_TH: c_int = 70;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_traffic_stats {
// units in bytes
    pub tx_unicast: u64,
    pub rx_unicast: u64,
    pub tx_avg_len: u32,
    pub rx_avg_len: u32,
// count for packets
    pub tx_cnt: u64,
    pub rx_cnt: u64,
// units in Mbps
    pub tx_throughput: u32,
    pub rx_throughput: u32,
    pub tx_throughput_raw: u32,
    pub rx_throughput_raw: u32,
    pub rx_tf_acc: u32,
    pub rx_tf_periodic: u32,
    pub tx_tfc_lv: rtw89_tfc_lv,
    pub rx_tfc_lv: rtw89_tfc_lv,
    pub tx_ewma_tp: ewma_tp,
    pub rx_ewma_tp: ewma_tp,
    pub tx_rate: u16,
    pub rx_rate: u16,
// used by rtwvif only
    pub udp_ratio: u64 tcp, udp,,
    pub active_histogram: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_chdef {
    pub center_ch: u8,
    pub band: u8,
    pub chan: u8,
    pub offset: rtw89_sc_offset,
    pub bw: rtw89_bandwidth,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_statistic {
    pub /: *mut *mut u8 rssi; / 0%~110% (dBm = rssi -110),
    pub traffic: rtw89_traffic_stats,
}

pub const BTC_WL_RSSI_THMAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_link_info {
    pub chdef: rtw89_btc_chdef,
    pub stat: rtw89_btc_statistic,
    pub dir: rtw89_tfc_dir,
    pub rssi_state: [u8; BTC_WL_RSSI_THMAX],
    pub mac_addr: [u8; ETH_ALEN],
    pub busy: u8,
    pub ch: u8,
    pub bw: u8,
    pub band: u8,
    pub role: u8,
    pub pid: u8,
    pub phy: u8,
    pub dtim_period: u8,
    pub mode: u8,
    pub tx_1ss_limit: u8,
    pub mac_id: u8,
    pub tx_retry: u8,
    pub bcn_period: u32,
    pub busy_t: u32,
    pub tx_time: u32,
    pub client_cnt: u32,
    pub rx_rate_drop_cnt: u32,
    pub noa_duration: u32,
    pub 1: u32 active:,
    pub 1: u32 noa:,
    pub 1: u32 client_ps:,
    pub 2: u32 connected:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_wl_state_map {
    pub val: u32,
    pub map: rtw89_btc_wl_smap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_hfp_desc {
    pub 1: u32 exist:,
    pub 2: u32 type:,
    pub 29: u32 rsvd:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_hid_desc {
    pub 1: u32 exist:,
    pub 2: u32 slot_info:,
    pub 2: u32 pair_cnt:,
    pub 8: u32 type:,
    pub 19: u32 rsvd:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_a2dp_desc {
    pub 1: u8 exist:,
    pub 1: u8 exist_last:,
    pub 1: u8 play_latency:,
    pub 3: u8 type:,
    pub 1: u8 active:,
    pub 1: u8 sink:,
    pub 1: u32 handle_update:,
    pub 1: u32 devinfo_query:,
    pub 8: u32 no_empty_streak_2s:,
    pub 8: u32 no_empty_streak_max:,
    pub 6: u32 rsvd:,
    pub bitpool: u8,
    pub vendor_id: u16,
    pub device_name: u32,
    pub flush_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_pan_desc {
    pub 1: u32 exist:,
    pub 1: u32 type:,
    pub 1: u32 active:,
    pub 29: u32 rsvd:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_rfk_info {
    pub 1: u32 run:,
    pub 1: u32 req:,
    pub 1: u32 timeout:,
    pub 29: u32 rsvd:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_bt_rfk_info_map {
    pub val: u32,
    pub map: rtw89_btc_bt_rfk_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_ver_info {
    pub /: *mut *mut u32 fw_coex; / match with which coex_ver,
    pub fw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bool_sta_chg {
    pub 1: u8 now:,
    pub 1: u8 last:,
    pub 1: u8 remain:,
    pub 5: u8 srvd:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_u8_sta_chg {
    pub now: u8,
    pub last: u8,
    pub chg: u8,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_scan_info {
    pub band: [u8; RTW89_PHY_NUM],
    pub phy_map: u8,
    pub hw_band_map: u8,
    pub type: u8,
    pub fw_scan: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_dbcc_info {
    pub /: *mut *mut u8 op_band[RTW89_PHY_NUM]; / op band in each phy,
    pub /: *mut *mut u8 scan_band[RTW89_PHY_NUM]; / scan band in each phy,
    pub real_band: [u8; RTW89_PHY_NUM],
    pub /: *mut *mut u8 role[RTW89_PHY_NUM]; / role in each phy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_mlo_info_v2 {
    pub wmode: [u8; RTW89_PHY_NUM],
    pub ch_type: [u8; RTW89_PHY_NUM],
    pub hwb_rf_band: [u8; RTW89_PHY_NUM],
    pub path_rf_band: [u8; RTW89_PHY_NUM],
    pub wtype: u8,
    pub mrcx_mode: u8,
    pub mrcx_act_hwb_map: u8,
    pub mrcx_bt_slot_rsp: u8,
    pub rf_combination: u8,
    pub mlo_en: u8,
    pub mlo_adie: u8,
    pub dual_hw_band_en: u8,
    pub link_status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_mlo_info {
    pub /: *mut *mut u8 wmode[RTW89_PHY_NUM]; / enum phl_mr_wmode,
    pub /: *mut *mut u8 ch_type[RTW89_PHY_NUM]; / enum phl_mr_ch_type,
    pub /: *mut *mut u8 hwb_rf_band[RTW89_PHY_NUM]; / enum band_type, RF-band for HW-band,
    pub /: *mut *mut u8 path_rf_band[RTW89_PHY_NUM]; / enum band_type, RF-band for PHY0/1,
    pub /: *mut *mut u8 wtype; / enum phl_mr_wtype,
    pub mrcx_mode: u8,
    pub mrcx_act_hwb_map: u8,
    pub mrcx_bt_slot_rsp: u8,
    pub /: *mut *mut u8 rf_combination; / enum btc_mlo_rf_combin 0:2+0, 1:0+2, 2:1+1,3:2+2,
    pub /: *mut *mut u8 mlo_en; / MLO enable,
    pub /: *mut *mut u8 mlo_adie; / a-die count,
    pub /: *mut *mut u8 dual_hw_band_en; / both 2 HW-band link exist,
    pub /: *mut *mut u32 link_status; / enum mlo_dbcc_mode_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_active_role {
    pub 1: u8 connected:,
    pub 3: u8 pid:,
    pub 1: u8 phy:,
    pub 1: u8 noa:,
    pub 2: u8 band:,
    pub 1: u8 client_ps:,
    pub 7: u8 bw:,
    pub role: u8,
    pub ch: u8,
    pub tx_lvl: u16,
    pub rx_lvl: u16,
    pub tx_rate: u16,
    pub rx_rate: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_active_role_v1 {
    pub 1: u8 connected:,
    pub 3: u8 pid:,
    pub 1: u8 phy:,
    pub 1: u8 noa:,
    pub 2: u8 band:,
    pub 1: u8 client_ps:,
    pub 7: u8 bw:,
    pub role: u8,
    pub ch: u8,
    pub tx_lvl: u16,
    pub rx_lvl: u16,
    pub tx_rate: u16,
    pub rx_rate: u16,
    pub /: *mut *mut u32 noa_duration; / ms,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_active_role_v2 {
    pub 1: u8 connected:,
    pub 3: u8 pid:,
    pub 1: u8 phy:,
    pub 1: u8 noa:,
    pub 2: u8 band:,
    pub 1: u8 client_ps:,
    pub 7: u8 bw:,
    pub role: u8,
    pub ch: u8,
    pub /: *mut *mut u32 noa_duration; / ms,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_active_role_v7 {
    pub connected: u8,
    pub pid: u8,
    pub phy: u8,
    pub noa: u8,
    pub band: u8,
    pub client_ps: u8,
    pub bw: u8,
    pub role: u8,
    pub ch: u8,
    pub noa_dur: u8,
    pub client_cnt: u8,
    pub rsvd2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_role_info_bpos {
    pub 1: u16 none:,
    pub 1: u16 station:,
    pub 1: u16 ap:,
    pub 1: u16 vap:,
    pub 1: u16 adhoc:,
    pub 1: u16 adhoc_master:,
    pub 1: u16 mesh:,
    pub 1: u16 moniter:,
    pub 1: u16 p2p_device:,
    pub 1: u16 p2p_gc:,
    pub 1: u16 p2p_go:,
    pub 1: u16 nan:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_eslot_ctrl {
    pub /: *mut *mut u8 en; / 1: toggle tx-flow-ctrl (null 0/1), tx-pause by Ext-slot,
    pub nulltx_role1: u8,
    pub nulltx_role2: u8,
    pub /: *mut *mut u8 nulltx_pre_time; / null-tx time prior to EBT-start (from E2G-end),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_wl_role_info_map {
    pub val: u16,
    pub role: rtw89_btc_wl_role_info_bpos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_role_info_v0 {
    pub connect_cnt: u8,
    pub link_mode: u8,
    pub role_map: rtw89_btc_wl_role_info_map,
    pub active_role: [rtw89_btc_wl_active_role; RTW89_PORT_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_role_info_v1 {
    pub connect_cnt: u8,
    pub link_mode: u8,
    pub role_map: rtw89_btc_wl_role_info_map,
    pub active_role_v1: [rtw89_btc_wl_active_role_v1; RTW89_PORT_NUM],
    pub /: *mut *mut u32 mrole_type; / btc_wl_mrole_type,
    pub /: *mut *mut u32 mrole_noa_duration; / ms,
    pub 1: u32 dbcc_en:,
    pub 1: u32 dbcc_chg:,
    pub /: *mut *mut u32 dbcc_2g_phy: 2; / which phy operate in 2G, HW_PHY_0 or HW_PHY_1,
    pub 1: u32 link_mode_chg:,
    pub 27: u32 rsvd:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_role_info_v2 {
    pub connect_cnt: u8,
    pub link_mode: u8,
    pub role_map: rtw89_btc_wl_role_info_map,
    pub active_role_v2: [rtw89_btc_wl_active_role_v2; RTW89_PORT_NUM],
    pub /: *mut *mut u32 mrole_type; / btc_wl_mrole_type,
    pub /: *mut *mut u32 mrole_noa_duration; / ms,
    pub 1: u32 dbcc_en:,
    pub 1: u32 dbcc_chg:,
    pub /: *mut *mut u32 dbcc_2g_phy: 2; / which phy operate in 2G, HW_PHY_0 or HW_PHY_1,
    pub 1: u32 link_mode_chg:,
    pub 27: u32 rsvd:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_rlink_v0 {
    pub connected: u8,
    pub pid: u8,
    pub phy: u8,
    pub noa: u8,
    pub /: *mut *mut u8 rf_band; / enum band_type RF band: 2.4G/5G/6G,
    pub /: *mut *mut u8 active; / 0:rlink is under doze,
    pub /: *mut *mut u8 bw; / enum channel_width,
    pub /: *mut *mut u8 role; /enum role_type,
    pub ch: u8,
    pub /: *mut *mut u8 noa_dur; / ms,
    pub /: *mut *mut u8 client_cnt; / for Role = P2P-Go/AP,
    pub /: *mut *mut u8 mode; / wifi protocol,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_rlink_v10 {
    pub connected: u8,
    pub pid: u8,
    pub phy: u8,
    pub noa: u8,
    pub /: *mut *mut u8 rf_band; / enum band_type RF band: 2.4G/5G/6G,
    pub /: *mut *mut u8 active; / 0:rlink is under doze,
    pub /: *mut *mut u8 bw; / enum channel_width,
    pub /: *mut *mut u8 role; /enum role_type,
    pub ch: u8,
    pub /: *mut *mut u8 noa_dur; / ms,
    pub /: *mut *mut u8 client_cnt; / for Role = P2P-Go/AP,
    pub /: *mut *mut u8 mode; / wifi protocol,
    pub mac_id: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub __packed: },
pub const RTW89_BE_BTC_WL_MAX_ROLE_NUMBER: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_role_info_v7 {
    pub connect_cnt: u8,
    pub link_mode: u8,
    pub link_mode_chg: u8,
    pub p2p_2g: u8,
    pub active_role: [rtw89_btc_wl_active_role_v7; RTW89_BE_BTC_WL_MAX_ROLE_NUMBER],
    pub role_map: __le32,
    pub /: *mut *mut __le32 mrole_type; / btc_wl_mrole_type,
    pub /: *mut *mut __le32 mrole_noa_duration; / ms,
    pub dbcc_en: __le32,
    pub dbcc_chg: __le32,
    pub /: *mut *mut __le32 dbcc_2g_phy; / which phy operate in 2G, HW_PHY_0 or HW_PHY_1,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_role_info_v8 {
    pub connect_cnt: u8,
    pub link_mode: u8,
    pub link_mode_chg: u8,
    pub p2p_2g: u8,
    pub pta_req_band: u8,
    pub /: *mut *mut u8 dbcc_en; / 1+1 and 2.4G-included,
    pub dbcc_chg: u8,
    pub /: *mut *mut u8 dbcc_2g_phy; / which phy operate in 2G, HW_PHY_0 or HW_PHY_1,
    pub rlink: [rtw89_btc_wl_rlink_v0; RTW89_BE_BTC_WL_MAX_ROLE_NUMBER][RTW89_MAC_NUM],
    pub role_map: __le32,
    pub /: *mut *mut __le32 mrole_type; / btc_wl_mrole_type,
    pub /: *mut *mut __le32 mrole_noa_duration; / ms,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_role_info_v10 {
    pub rlink: [rtw89_btc_wl_rlink_v10; RTW89_BE_BTC_WL_MAX_ROLE_NUMBER][RTW89_MAC_NUM],
    pub link_mode: u8,
    pub link_mode_hb1: u8,
    pub p2p_exist: u8,
    pub p2p_exist_hb1: u8,
    pub pta_req_band: u8,
    pub /: *mut *mut u8 dbcc_en; / 1+1 and 2.4G-included,
    pub /: *mut *mut u8 dbcc_2g_phy; / which phy operate in 2G, HW_PHY_0 or HW_PHY_1,
    pub rsvd: u8,
    pub role_map: __le32,
    pub role_map_hb1: __le32,
    pub /: *mut *mut __le32 mrole_type; / btc_wl_mrole_type: [31:16]:band1, [15:0]:band0,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_rlink {
    pub connected: u8,
    pub pid: u8,
    pub phy: u8,
    pub noa: u8,
    pub /: *mut *mut u8 rf_band; / enum band_type RF band: 2.4G/5G/6G,
    pub /: *mut *mut u8 active; / 0:rlink is under doze,
    pub /: *mut *mut u8 bw; / enum channel_width,
    pub /: *mut *mut u8 role; /enum role_type,
    pub ch: u8,
    pub /: *mut *mut u8 noa_dur; / ms,
    pub /: *mut *mut u8 client_cnt; / for Role = P2P-Go/AP,
    pub /: *mut *mut u8 mode; / wifi protocol,
// v0 v1
    pub tx_lvl: u16,
    pub rx_lvl: u16,
    pub tx_rate: u16,
    pub rx_rate: u16,
// v7
    pub v0*/: *mut *mut u8 client_ps; /v7 v2 v1,
// v10
    pub mac_id: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_role_info {
    pub rlink: [rtw89_btc_wl_rlink; RTW89_BE_BTC_WL_MAX_ROLE_NUMBER][RTW89_MAC_NUM],
    pub link_mode: u8,
    pub link_mode_hb1: u8,
    pub p2p_exist: u8,
    pub p2p_exist_hb1: u8,
    pub pta_req_band: u8,
    pub /: *mut *mut u8 dbcc_en; / 1+1 and 2.4G-included,
    pub /: *mut *mut u8 dbcc_2g_phy; / which phy operate in 2G, HW_PHY_0 or HW_PHY_1,
    pub rsvd: u8,
    pub role_map: u32,
    pub role_map_hb1: u32,
    pub /: *mut *mut u32 mrole_type; / btc_wl_mrole_type: [31:16]:band1, [15:0]:band0,
// Before v10 use this linkmode
    pub link_mode_v0: u8,
// v7
    pub connect_cnt: u8,
    pub v2*/: *mut *mut u32 dbcc_chg; / v7,
// v8
    pub v2*/: *mut *mut u8 link_mode_chg; / v8, v7,
    pub /: *mut *mut u8 p2p_2g; / v8, v7,
    pub v2*/: *mut *mut u32 mrole_noa_duration; / v8, v7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_ver_info {
    pub build_time: [c_char; 12],
    pub build_date: [c_char; 12],
    pub /: *mut *mut u32 fw_coex; / match with which coex_ver,
    pub fw: u32,
    pub mac: u32,
    pub bb: u32,
    pub rf: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_afh_info {
    pub en: u8,
    pub ch: u8,
    pub bw: u8,
    pub band: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_rfk_info {
    pub 2: u32 state:,
    pub 4: u32 path_map:,
    pub 2: u32 phy_map:,
    pub 2: u32 band:,
    pub 8: u32 type:,
    pub 1: u32 con_rfk:,
    pub 13: u32 rsvd:,
    pub start_time: u32,
    pub proc_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_smap {
    pub 1: u32 connect:,
    pub 1: u32 ble_connect:,
    pub 1: u32 acl_busy:,
    pub 1: u32 sco_busy:,
    pub 1: u32 mesh_busy:,
    pub 1: u32 inq_pag:,
    pub 8: u32 profile_map:,
    pub 18: u32 rsvd:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_bt_state_map {
    pub val: u32,
    pub map: rtw89_btc_bt_smap,
}

pub const BTC_BT_RSSI_THMAX: c_int = 4;
pub const BTC_BT_AFH_GROUP: c_int = 12;
pub const BTC_BT_AFH_LE_GROUP: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_txpwr_desc {
    pub br_dbm: i8,
    pub le_dbm: i8,
    pub br_gain_index: u8,
    pub le_gain_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_leaudio_info {
    pub cmd: u8,
    pub len: u8,
    pub bis_cis: u8,

    pub rssi: u8,
    pub hbrsvd: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_bistdma_info_le {
    pub cmd: u8,
    pub len: u8,
    pub /: *mut *mut u8 bis; / BIT(2) ~ BIT(7) is rsvd,

    pub diff_t_lb: u8,
    pub /: *mut *mut *mut *mut u8 diff_t_hb; / diff_t = (diff_t_hb  256 + diff_t_lb)  0.625 ms,
    pub hb1rsvd: u8,
    pub hb2rsvd: u8,
    pub hb3rsvd: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_leaudio_desc {
    pub 1: u32 bis_exist:,
    pub 1: u32 bis_exist_last:,
    pub 1: u32 cis_exist:,
    pub 1: u32 cis_exist_last:,
    pub 3: u32 bis_cnt:,
    pub 3: u32 cis_cnt:,
    pub 8: u32 rssi:,
    pub 3: u32 bis_cnt_last:,
    pub 3: u32 cis_cnt_last:,
    pub 1: u32 bis_trx:,
    pub 1: u32 bis_start_end:,
    pub 6: u32 rsvd:,
    pub diff_t: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_link_info {
    pub link_cnt: rtw89_btc_u8_sta_chg,
    pub multi_link: rtw89_btc_bool_sta_chg,
    pub relink: rtw89_btc_bool_sta_chg,
    pub hfp_desc: rtw89_btc_bt_hfp_desc,
    pub hid_desc: rtw89_btc_bt_hid_desc,
    pub a2dp_desc: rtw89_btc_bt_a2dp_desc,
    pub pan_desc: rtw89_btc_bt_pan_desc,
    pub leaudio_desc: rtw89_btc_bt_leaudio_desc,
    pub status: rtw89_btc_bt_state_map,
    pub bt_txpwr_desc: rtw89_btc_bt_txpwr_desc,
    pub sut_pwr_level: [u8; BTC_PROFILE_MAX],
    pub golden_rx_shift: [u8; BTC_PROFILE_MAX],
    pub rssi_state: [u8; BTC_BT_RSSI_THMAX],
    pub afh_map: [u8; BTC_BT_AFH_GROUP],
    pub afh_map_le: [u8; BTC_BT_AFH_LE_GROUP],
    pub rssi: u8,
    pub 1: u8 role_sw:,
    pub 1: u8 slave_role:,
    pub 1: u8 afh_update:,
    pub 1: u8 cqddr:,
    pub 1: u8 tx_3m:,
    pub 1: u8 inq:,
    pub 1: u8 pag:,
    pub 1: u8 igno_wl:,
    pub 1: u8 ble_scan_en:,
    pub 1: u8 reinit:,
    pub 6: u8 rsvd:,
    pub /: *mut *mut u8 leaudio_raw_info[BTC_BTINFO_MAX]; / raw LE audio info from BT mailbox,
    pub /: *mut *mut u8 bistdma_raw_info[BTC_BTINFO_MAX]; / raw BIS-TDMA info from BT mailbox,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bind_bt_status {
    pub 1: u8 a2dp_active:,
    pub 1: u8 a2dp_sink:,
    pub 1: u8 pan_active:,
    pub 1: u8 connect:,
    pub 1: u8 inq_page:,
    pub 1: u8 multi_link:,
    pub 1: u8 slave_role:,
    pub 1: u8 page:,
    pub 1: u8 hfp_exist:,
    pub 1: u8 hid_exist:,
    pub 1: u8 a2dp_exist:,
    pub 1: u8 pan_exist:,
    pub 1: u8 bis_exist:,
    pub 1: u8 cis_exist:,
    pub 1: u8 thread_exist:,
    pub 1: u8 ull_exist:,
    pub hid_cnt: u8,
    pub hid_type: u8,
    pub cis_cnt: u8,
    pub link_cnt: u8,
    pub a2dp_vendor_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bind_info {
    pub /: *mut *mut u8 wl_hwb_sel; / map,
    pub wl_link_mode: u8,
    pub wl_bg_mode: u8,
    pub /: *mut *mut u8 rf_band; / map, 0: no any rf-band bind,
    pub /: *mut *mut u8 bt_sel; / map,
    pub /: *mut *mut u8 bt_link_weight; / select the highest weight between bt/rf-band,
    pub /: *mut *mut u32 bt_profile; / map,
    pub bt_smap: rtw89_btc_bind_bt_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_extsoc_info {
    pub chip_id: u8,
    pub max_tx_pwr: u8,
    pub rf_band_map: u8,
    pub ant_iso_to_wl: u8,
    pub link_weight: [u8; BTC_BT_BMAX],
    pub /: *mut *mut u8 func_type; / 0: none, 1:zigbee, 2:LTE,
    pub /: *mut *mut u8 hw_coex; / Hard-Wire coex interface support,
    pub /: *mut *mut u8 pta_type; / 0: RTK 4-wire mode, 1: 3-wire mode,
    pub pta_req_exist: u8,
    pub hpta_cfg: u32,
    pub hmbx_cfg: u32,
    pub swout_cfg: u32,
    pub swin_cfg: u32,
    pub profile_map: [u32; BTC_BT_BMAX],
    pub bcnt: [u32; BTC_BCNT_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_dm_emap {
    pub 1: u32 init:,
    pub 1: u32 pta_owner:,
    pub 1: u32 wl_rfk_timeout:,
    pub 1: u32 bt_rfk_timeout:,
    pub 1: u32 wl_fw_hang:,
    pub 1: u32 cycle_hang:,
    pub 1: u32 w1_hang:,
    pub 1: u32 b1_hang:,
    pub 1: u32 tdma_no_sync:,
    pub 1: u32 slot_no_sync:,
    pub 1: u32 wl_slot_drift:,
    pub 1: u32 bt_slot_drift:,
    pub 1: u32 role_num_mismatch:,
    pub 1: u32 null1_tx_late:,
    pub 1: u32 bt_afh_conflict:,
    pub 1: u32 bt_leafh_conflict:,
    pub 1: u32 bt_slot_flood:,
    pub 1: u32 wl_e2g_hang:,
    pub 1: u32 wl_ver_mismatch:,
    pub 1: u32 bt_ver_mismatch:,
    pub 1: u32 rfe_type0:,
    pub 1: u32 h2c_buffer_over:,
    pub req*/: *mut *mut u32 bt_tx_hang: 1; / for SNR too low bug, BT has no Tx,
    pub 1: u32 wl_no_sta_ntfy:,
    pub 1: u32 w2b_scbd_no_sync:,
    pub 1: u32 h2c_bmap_mismatch:,
    pub 1: u32 c2h_bmap_mismatch:,
    pub 1: u32 h2c_struct_invalid:,
    pub 1: u32 c2h_struct_invalid:,
    pub 1: u32 h2c_c2h_buffer_mismatch:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_dm_error_map {
    pub val: u32,
    pub map: rtw89_btc_dm_emap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rf_para {
    pub tx_pwr_freerun: u32,
    pub rx_gain_freerun: u32,
    pub tx_pwr_perpkt: u32,
    pub rx_gain_perpkt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_nhm {
    pub instant_wl_nhm_dbm: u8,
    pub instant_wl_nhm_per_mhz: u8,
    pub valid_record_times: u16,
    pub record_pwr: [i8; 16],
    pub record_ratio: [u8; 16],
    pub /: *mut *mut s8 pwr; / dbm_per_MHz,
    pub ratio: u8,
    pub current_status: u8,
    pub refresh: u8,
    pub start_flag: bool,
    pub pwr_max: i8,
    pub pwr_min: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_info {
    pub rlink_info: [rtw89_btc_wl_link_info; RTW89_BE_BTC_WL_MAX_ROLE_NUMBER][RTW89_MAC_NUM],
    pub rf_ch_info: [rtw89_btc_chdef; RTW89_PHY_NUM],
    pub rfk_info: rtw89_btc_wl_rfk_info,
    pub ver_info: rtw89_btc_wl_ver_info,
    pub afh_info: [rtw89_btc_wl_afh_info; RTW89_MAC_NUM][RTW89_BAND_NUM],
    pub afh_info_last: [rtw89_btc_wl_afh_info; RTW89_MAC_NUM][RTW89_BAND_NUM],
    pub role_info: rtw89_btc_wl_role_info,
    pub scan_info: rtw89_btc_wl_scan_info,
    pub dbcc_info: rtw89_btc_wl_dbcc_info,
    pub mlo_info: rtw89_btc_wl_mlo_info,
    pub rf_para: rtw89_btc_rf_para,
    pub nhm: rtw89_btc_wl_nhm,
    pub status: rtw89_btc_wl_state_map,
    pub port_id: [u8; RTW89_WIFI_ROLE_MLME_MAX],
    pub rssi_level: u8,
    pub cn_report: u8,
    pub coex_mode: u8,
    pub pta_req_mac: u8,
    pub /: *mut *mut u8 bt_polut_type[RTW89_PHY_NUM]; / BT polluted WL-Tx type for phy0/1,
    pub /: *mut *mut u8 rf_band_map[RTW89_PHY_NUM]; / rf_band bit-map,
    pub ch_map: [u8; 12],
    pub ch_map_le: [u8; 5],
    pub is_5g_hi_ch: bool,
    pub is_5g_hi_ch_hb1: bool,
    pub go_client_exist: bool,
    pub go_client_exist_hb1: bool,
    pub noa_exist: bool,
    pub noa_exist_hb1: bool,
    pub pta_reg_mac_chg: bool,
    pub bg_mode: bool,
    pub bg_mode_hb1: bool,
    pub he_mode: bool,
    pub scbd_chg: [bool; BTC_ALL_BT],
    pub fw_ver_mismatch: bool,
    pub client_cnt_inc_2g: bool,
    pub link_mode_chg: bool,
    pub dbcc_chg: bool,
    pub scbd: [u32; BTC_ALL_BT],
    pub scbd_rb: [u32; BTC_ALL_BT],
    pub wcnt: [u32; BTC_WCNT_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_module_v0 {
    pub ant: rtw89_btc_ant_info_v0,
    pub rfe_type: u8,
    pub cv: u8,
    pub 1: u8 bt_solo:,
    pub 1: u8 bt_pos:,
    pub 1: u8 switch_type:,
    pub 3: u8 wa_type:,
    pub kt_ver_adie: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_module_v7 {
    pub rfe_type: u8,
    pub kt_ver: u8,
    pub bt_solo: u8,
    pub bt.btg_type*/: *mut *mut u8 bt_pos; / wl-end view: get from efuse, must compare,
    pub /: *mut *mut u8 switch_type; / WL/BT switch type: 0: internal, 1: external,
    pub /: *mut *mut u8 wa_type; / WA type: 0:none, 1: 51B 5G_Hi-Ch_Rx,
    pub kt_ver_adie: u8,
    pub rsvd: u8,
    pub ant: rtw89_btc_ant_info_v7,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_module_v10 {
    pub rfe_type: u8,
    pub /: *mut *mut u8 wa_type; / Refer to enum btc_wa_type,
    pub kt_ver: u8,
    pub kt_ver_adie: u8,
    pub bt.btg_type*/: *mut *mut u8 bt0_pos; / wl-end view: get from efuse, must compare,
    pub Ext(SPDT)*/: *mut *mut u8 bt0_sw_type; / BT Ant-switch: None(non-share), Int(BTG),,
    pub /: *mut *mut u8 bt1_pos; / BTC_BT_ALONE or BTC_BT_BTG,
    pub bt1_sw_type: u8,
    pub ant: rtw89_btc_ant_info_v10,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_module_v11 {
    pub rfe_type: u8,
    pub wa_type: u8,
    pub kt_ver: u8,
    pub kt_ver_adie: u8,
    pub bt0_pos: u8,
    pub bt0_sw_type: u8,
    pub bt1_pos: u8,
    pub bt1_sw_type: u8,
    pub ant: rtw89_btc_ant_info_v11,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_module_info {
    pub md_v0: rtw89_btc_module_v0,
    pub md_v7: rtw89_btc_module_v7,
    pub md_v10: rtw89_btc_module_v10,
    pub md_v11: rtw89_btc_module_v11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_module {
    pub rfe_type: u8,
    pub /: *mut *mut u8 wa_type; / Refer to enum btc_wa_type,
    pub kt_ver: u8,
    pub kt_ver_adie: u8,
    pub bt.btg_type*/: *mut *mut u8 bt0_pos; / wl-end view: get from efuse, must compare,
    pub Ext(SPDT)*/: *mut *mut u8 bt0_sw_type; / BT Ant-switch: None(non-share), Int(BTG),,
    pub /: *mut *mut u8 bt1_pos; / BTC_BT_ALONE or BTC_BT_BTG,
    pub bt1_sw_type: u8,
    pub bt_solo: u8,
    pub ant: rtw89_btc_ant_info,
}

pub const RTW89_BTC_DM_MAXSTEP: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_dm_step {
    pub step: [u16; RTW89_BTC_DM_MAXSTEP],
    pub step_pos: u8,
    pub step_ov: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_init_info_v0 {
    pub module: rtw89_btc_module_v0,
    pub wl_guard_ch: u8,
    pub 1: u8 wl_only:,
    pub 1: u8 wl_init_ok:,
    pub 1: u8 dbcc_en:,
    pub 1: u8 cx_other:,
    pub 1: u8 bt_only:,
    pub rsvd: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_init_info_v7 {
    pub wl_guard_ch: u8,
    pub wl_only: u8,
    pub wl_init_ok: u8,
    pub rsvd3: u8,
    pub cx_other: u8,
    pub bt_only: u8,
    pub pta_mode: u8,
    pub pta_direction: u8,
    pub module: rtw89_btc_module_v7,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_init_info_v107 {
    pub wl_guard_ch: u8,
    pub wl_only: u8,
    pub wl_init_ok: u8,
    pub dbcc_en: u8,
    pub cx_other: u8,
    pub bt_only: u8,
    pub rsvd: u8,
    pub rsvd1: u8,
    pub module: rtw89_btc_module_v7,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_init_info_v10 {
    pub /: *mut *mut u8 endian_type; / 0: little-endian, 1:big-endian,
    pub /: *mut *mut u8 init_mode; / refer to enum BTC_MODE_xxx,
    pub wl_init_ok: u8,
    pub bt0_function: u8,
    pub bt1_function: u8,
    pub bt2_function: u8,
    pub pta_mode: u8,
    pub pta_direction: u8,
    pub module: rtw89_btc_module_v10,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_init_info_v11 {
    pub /: *mut *mut u8 endian_type; / 0: little-endian, 1:big-endian,
    pub /: *mut *mut u8 init_mode; / refer to enum BTC_MODE_xxx,
    pub wl_init_ok: u8,
    pub bt0_function: u8,
    pub bt1_function: u8,
    pub bt2_function: u8,
    pub pta_mode: u8,
    pub pta_direction: u8,
    pub module: rtw89_btc_module_v11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_init_info_u {
    pub init_v0: rtw89_btc_init_info_v0,
    pub init_v7: rtw89_btc_init_info_v7,
    pub init_v10: rtw89_btc_init_info_v10,
    pub init_v107: rtw89_btc_init_info_v107,
    pub init_v11: rtw89_btc_init_info_v11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_init_info {
    pub /: *mut *mut u8 endian_type; / 0: little-endian, 1:big-endian,
    pub /: *mut *mut u8 init_mode; / refer to enum BTC_MODE_xxx,
    pub wl_init_ok: u8,
    pub bt0_function: u8,
    pub bt1_function: u8,
    pub bt2_function: u8,
    pub pta_mode: u8,
    pub pta_direction: u8,
    pub dbcc_en: u8,
    pub cx_other: u8,
    pub bt_only: u8,
    pub wl_only: u8,
    pub wl_guard_ch: u8,
    pub module: rtw89_btc_module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_tx_limit_para {
    pub enable: u16,
    pub /: *mut *mut u32 tx_time; / unit: us,
    pub tx_retry: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_trx_nss_para {
    pub tx_limit: u8,
    pub rx_limit: u8,
    pub tx_ss: u8,
    pub rx_ss: u8,
    pub tx_path: u8,
    pub rx_path: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_bt_scan_type {
    BTC_SCAN_INQ	= 0,
    BTC_SCAN_PAGE,
    BTC_SCAN_BLE,
    BTC_SCAN_INIT,
    BTC_SCAN_TV,
    BTC_SCAN_ADV,
    BTC_SCAN_MAX1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_ble_scan_type {
    CXSCAN_BG = 0,
    CXSCAN_INIT,
    CXSCAN_LE,
    CXSCAN_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_bt_func_type {
    BTC_BTF_NONE = 0,
    BTC_BTF_BT = BIT(0),
    BTC_BTF_ZB = BIT(1),
    BTC_BTF_THREAD = BIT(2),
    BTC_BTF_24GPRO = BIT(3), /* 2.4GHz Proprietary */
    BTC_BTF_ULL = BIT(4),
}

pub const RTW89_BTC_BT_DEF_BR_TX_PWR: c_int = 4;
pub const RTW89_BTC_BT_DEF_LE_TX_PWR: c_int = 4;
pub const RTW89_BTC_DEFAULT_ANISO: c_int = 10;
pub const RTW89_BTC_BT_DEF_LE_TX_PWR_1: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_scan_info_v1 {
    pub win: __le16,
    pub intvl: __le16,
    pub flags: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_scan_info_v2 {
    pub win: __le16,
    pub intvl: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btscan_v1 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxbtscan,
    pub rsvd: u8,
    pub rsvd2: __le16,
    pub scan: [rtw89_btc_bt_scan_info_v1; BTC_SCAN_MAX1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btscan_v2 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxbtscan,
    pub type: u8,
    pub rsvd2: __le16,
    pub para: [rtw89_btc_bt_scan_info_v2; CXSCAN_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btscan_v7 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxbtscan,
    pub type: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub para: [rtw89_btc_bt_scan_info_v2; CXSCAN_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btscan_v8 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxbtscan,
    pub type: u8,
    pub /: *mut *mut u8 bt_id; / 0:BT0, 1:BT1,
    pub rsvd1: u8,
    pub para: [rtw89_btc_bt_scan_info_v2; CXSCAN_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_btscan {
    pub v1: rtw89_btc_fbtc_btscan_v1,
    pub v2: rtw89_btc_fbtc_btscan_v2,
    pub v7: rtw89_btc_fbtc_btscan_v7,
    pub v8: rtw89_btc_fbtc_btscan_v8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_bt_info {
    pub link_info: rtw89_btc_bt_link_info,
    pub link_info_56g: rtw89_btc_bt_link_info,
    pub scan_info_v1: [rtw89_btc_bt_scan_info_v1; BTC_SCAN_MAX1],
    pub scan_info_v2: [rtw89_btc_bt_scan_info_v2; CXSCAN_MAX],
    pub ver_info: rtw89_btc_bt_ver_info,
    pub enable: rtw89_btc_bool_sta_chg,
    pub inq_pag: rtw89_btc_bool_sta_chg,
    pub rf_para: rtw89_btc_rf_para,
    pub rfk_info: rtw89_btc_bt_rfk_info_map,
    pub /: *mut *mut u8 raw_info[BTC_BTINFO_MAX]; / raw bt info from mailbox (2.4G),
    pub /: *mut *mut u8 raw_info_56g[BTC_BTINFO_MAX]; / raw bt info from mailbox (5/6G),
    pub txpwr_info: [u8; BTC_BTINFO_MAX],
    pub /: *mut *mut u8 link_weight[BTC_BT_BMAX]; / Link Weight for RF-band/HWB selection,
    pub rssi_level: u8,
    pub rf_band_map: u8,
    pub func_type: u8,
    pub tx_power_now: u8,
    pub tx_power_now_6g: u8,
    pub /: *mut *mut u8 ant_iso_to_wl; / ant isolation between BTx and WL,
    pub 1: u8 fw_ver_mismatch:,
    pub 1: u8 band_56G_support:,
    pub 1: u8 hi_lna_rx:,
    pub 3: u8 lna_constrain:,
    pub 1: u8 hi_lna_rx_6g:,
    pub 3: u8 lna_constrain_6g:,
    pub 6: u8 rsvd:,
    pub scbd: u32,
    pub scbd_rb: u32,
    pub scbd_c2h: u32,
    pub feature: u32,
    pub 1: u32 mbx_avl:,
    pub 1: u32 whql_test:,
    pub 1: u32 igno_wl:,
    pub 1: u32 reinit:,
    pub 1: u32 ble_scan_en:,
    pub 1: u32 btg_type:,
    pub 1: u32 inq:,
    pub 1: u32 pag:,
    pub 1: u32 run_patch_code:,
    pub 1: u32 scan_rx_low_pri:,
    pub 1: u32 scan_info_update:,
    pub 22: u32 rsvd1:,
    pub bcnt: [u32; BTC_BCNT_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rf_trx_para_v0 {
    pub /: *mut *mut u32 wl_tx_power; / absolute Tx power (dBm), 0xff-> no BTC control,
    pub /: *mut *mut u32 wl_rx_gain; / rx gain table index (TBD.),
    pub /: *mut *mut u8 bt_tx_power; / decrease Tx power (dB),
    pub /: *mut *mut u8 bt_rx_gain; / LNA constrain level,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rf_trx_para_v9 {
    pub /: *mut *mut u32 wl_tx_power[RTW89_PHY_NUM]; / absolute Tx power (dBm), 1's complement -5->0x85,
    pub /: *mut *mut u32 wl_rx_gain[RTW89_PHY_NUM]; / rx gain table index (TBD.),
    pub /: *mut *mut u32 bt_tx_power[BTC_ALL_BT]; / decrease Tx power (dB),
    pub /: *mut *mut u32 bt_rx_gain[BTC_ALL_BT]; / LNA constrain level,
    pub /: *mut *mut u32 zb_tx_power[BTC_ALL_BT]; / 15.4 devrease Tx power (dB),
    pub /: *mut *mut u32 zb_rx_gain[BTC_ALL_BT]; / 15.4 constrain level,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_cx {
    pub wl: rtw89_btc_wl_info,
    pub bt0: rtw89_btc_bt_info,
    pub bt1: rtw89_btc_bt_info,
    pub bt_ext: rtw89_btc_extsoc_info,
    pub rf_para: rtw89_btc_rf_trx_para_v9,
    pub state_map: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_tdma {
    pub /: *mut *mut u8 type; / btc_ver::fcxtdma,
    pub rxflctrl: u8,
    pub txflctrl: u8,
    pub bind: u8,
    pub leak_n: u8,
    pub ext_ctrl: u8,
    pub rxflctrl_role: u8,
    pub option_ctrl: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_tdma_v3 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxtdma,
    pub rsvd: u8,
    pub rsvd1: __le16,
    pub tdma: rtw89_btc_fbtc_tdma,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_tdma_le32 {
    pub v1: rtw89_btc_fbtc_tdma,
    pub v3: rtw89_btc_fbtc_tdma_v3,
}

pub const CXMREG_MAX: c_int = 30;
pub const CXMREG_MAX_V2: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_bt_sta_counter {
    BTC_BCNT_RFK_REQ = 0,
    BTC_BCNT_RFK_GO = 1,
    BTC_BCNT_RFK_REJECT = 2,
    BTC_BCNT_RFK_FAIL = 3,
    BTC_BCNT_RFK_TIMEOUT = 4,
    BTC_BCNT_HI_TX = 5,
    BTC_BCNT_HI_RX = 6,
    BTC_BCNT_LO_TX = 7,
    BTC_BCNT_LO_RX = 8,
    BTC_BCNT_POLLUTED = 9,
    BTC_BCNT_STA_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_bt_sta_counter_v105 {
    BTC_BCNT_RFK_REQ_V105 = 0,
    BTC_BCNT_HI_TX_V105 = 1,
    BTC_BCNT_HI_RX_V105 = 2,
    BTC_BCNT_LO_TX_V105 = 3,
    BTC_BCNT_LO_RX_V105 = 4,
    BTC_BCNT_POLLUTED_V105 = 5,
    BTC_BCNT_STA_MAX_V105
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_v1 {
    pub /: *mut *mut u16 fver; / btc_ver::fcxbtcrpt,
    pub /: *mut *mut u16 rpt_cnt; / tmr counters,
    pub /: *mut *mut u32 wl_fw_coex_ver; / match which driver's coex version,
    pub wl_fw_cx_offload: u32,
    pub wl_fw_ver: u32,
    pub rpt_enable: u32,
    pub /: *mut *mut u32 rpt_para; / ms,
    pub /: *mut *mut u32 mb_send_fail_cnt; / fw send mailbox fail counter,
    pub /: *mut *mut u32 mb_send_ok_cnt; / fw send mailbox ok counter,
    pub /: *mut *mut u32 mb_recv_cnt; / fw recv mailbox counter,
    pub /: *mut *mut u32 mb_a2dp_empty_cnt; / a2dp empty count,
    pub /: *mut *mut u32 mb_a2dp_flct_cnt; / a2dp empty flow control counter,
    pub /: *mut *mut u32 mb_a2dp_full_cnt; / a2dp empty full counter,
    pub bt_rfk_cnt: [u32; BTC_BCNT_HI_TX],
    pub /: *mut *mut u32 c2h_cnt; / fw send c2h counter,
    pub /: *mut *mut u32 h2c_cnt; / fw recv h2c counter,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_info {
    pub /: *mut *mut __le32 cnt; / fw report counter,
    pub /: *mut *mut __le32 en; / report map,
    pub /: *mut *mut __le32 para; / not used,
    pub /: *mut *mut __le32 cnt_c2h; / fw send c2h counter,
    pub /: *mut *mut __le32 cnt_h2c; / fw recv h2c counter,
    pub /: *mut *mut __le32 len_c2h; / The total length of the last C2H,
    pub /: *mut *mut __le32 cnt_aoac_rf_on; / rf-on counter for aoac switch notify,
    pub /: *mut *mut __le32 cnt_aoac_rf_off; / rf-off counter for aoac switch notify,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_info_v5 {
    pub /: *mut *mut __le32 cx_ver; / match which driver's coex version,
    pub fw_ver: __le32,
    pub /: *mut *mut __le32 en; / report map,
    pub /: *mut *mut __le16 cnt; / fw report counter,
    pub /: *mut *mut __le16 cnt_c2h; / fw send c2h counter,
    pub /: *mut *mut __le16 cnt_h2c; / fw recv h2c counter,
    pub /: *mut *mut __le16 len_c2h; / The total length of the last C2H,
    pub /: *mut *mut __le16 cnt_aoac_rf_on; / rf-on counter for aoac switch notify,
    pub /: *mut *mut __le16 cnt_aoac_rf_off; / rf-off counter for aoac switch notify,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_info_v8 {
    pub /: *mut *mut __le16 cnt; / fw report counter,
    pub /: *mut *mut __le16 cnt_c2h; / fw send c2h counter,
    pub /: *mut *mut __le16 cnt_h2c; / fw recv h2c counter,
    pub /: *mut *mut __le16 len_c2h; / The total length of the last C2H,
    pub /: *mut *mut __le16 cnt_aoac_rf_on; / rf-on counter for aoac switch notify,
    pub /: *mut *mut __le16 cnt_aoac_rf_off; / rf-off counter for aoac switch notify,
    pub /: *mut *mut __le32 cx_ver; / match which driver's coex version,
    pub fw_ver: __le32,
    pub /: *mut *mut __le32 en; / report map,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_wl_fw_info {
    pub /: *mut *mut __le32 cx_ver; / match which driver's coex version,
    pub cx_offload: __le32,
    pub fw_ver: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_a2dp_empty {
    pub /: *mut *mut __le32 cnt_empty; / a2dp empty count,
    pub /: *mut *mut __le32 cnt_flowctrl; / a2dp empty flow control counter,
    pub cnt_tx: __le32,
    pub cnt_ack: __le32,
    pub cnt_nack: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_bt_mailbox {
    pub /: *mut *mut __le32 cnt_send_ok; / fw send mailbox ok counter,
    pub /: *mut *mut __le32 cnt_send_fail; / fw send mailbox fail counter,
    pub /: *mut *mut __le32 cnt_recv; / fw recv mailbox counter,
    pub a2dp: rtw89_btc_fbtc_rpt_ctrl_a2dp_empty,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_v4 {
    pub fver: u8,
    pub rsvd: u8,
    pub rsvd1: __le16,
    pub rpt_info: rtw89_btc_fbtc_rpt_ctrl_info,
    pub wl_fw_info: rtw89_btc_fbtc_rpt_ctrl_wl_fw_info,
    pub bt_mbx_info: rtw89_btc_fbtc_rpt_ctrl_bt_mailbox,
    pub bt_cnt: [__le32; BTC_BCNT_STA_MAX],
    pub gnt_val: [rtw89_mac_ax_gnt; RTW89_PHY_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_v5 {
    pub fver: u8,
    pub rsvd: u8,
    pub rsvd1: __le16,
    pub gnt_val: [u8; RTW89_PHY_NUM][4],
    pub bt_cnt: [__le16; BTC_BCNT_STA_MAX],
    pub rpt_info: rtw89_btc_fbtc_rpt_ctrl_info_v5,
    pub bt_mbx_info: rtw89_btc_fbtc_rpt_ctrl_bt_mailbox,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_v105 {
    pub fver: u8,
    pub rsvd: u8,
    pub rsvd1: __le16,
    pub gnt_val: [u8; RTW89_PHY_NUM][4],
    pub bt_cnt: [__le16; BTC_BCNT_STA_MAX_V105],
    pub rpt_info: rtw89_btc_fbtc_rpt_ctrl_info_v5,
    pub bt_mbx_info: rtw89_btc_fbtc_rpt_ctrl_bt_mailbox,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_v7 {
    pub fver: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub gnt_val: [u8; RTW89_PHY_NUM][4],
    pub bt_cnt: [__le16; BTC_BCNT_STA_MAX_V105],
    pub rpt_info: rtw89_btc_fbtc_rpt_ctrl_info_v8,
    pub bt_mbx_info: rtw89_btc_fbtc_rpt_ctrl_bt_mailbox,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_v8 {
    pub fver: u8,
    pub rsvd0: u8,
    pub /: *mut *mut u8 rpt_len_max_l; / BTC_RPT_MAX bit0~7,
    pub /: *mut *mut u8 rpt_len_max_h; / BTC_RPT_MAX bit8~15,
    pub gnt_val: [u8; RTW89_PHY_NUM][4],
    pub bt_cnt: [__le16; BTC_BCNT_STA_MAX_V105],
    pub rpt_info: rtw89_btc_fbtc_rpt_ctrl_info_v8,
    pub bt_mbx_info: rtw89_btc_fbtc_rpt_ctrl_bt_mailbox,
    pub __packed: },
pub const RTW89_BTC_TIME_DATE_FMT: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_v9 {
    pub fver: u8,
    pub ext_req_exist: u8,
    pub pta_owner: u8,
    pub rsvd: u8,
    pub build_time: [u8; RTW89_BTC_TIME_DATE_FMT],
    pub build_date: [u8; RTW89_BTC_TIME_DATE_FMT],
    pub /: *mut *mut u8 gnt_val[RTW89_PHY_NUM][4]; / gwl/gbt012 refer to struct btc_gnt_ctrl,
    pub bt_cnt: [__le16; BTC_BCNT_STA_MAX_V105],
    pub rpt_info: rtw89_btc_fbtc_rpt_ctrl_info_v8,
    pub bt_mbx_info: rtw89_btc_fbtc_rpt_ctrl_bt_mailbox,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_rpt_ctrl_v11 {
    pub fver: u8,
    pub rsvd0: u8,
    pub /: *mut *mut u8 rpt_len_max_l; / BTC_RPT_MAX bit0~7,
    pub /: *mut *mut u8 rpt_len_max_h; / BTC_RPT_MAX bit8~15,
    pub build_time: [u8; 12],
    pub build_date: [u8; 12],
    pub /: *mut *mut u8 gnt_val[RTW89_PHY_NUM][8]; / gwl/gbt012 refer to struct btc_gnt_ctrl,
    pub bt_cnt: [__le16; BTC_ALL_BT_EZL][BTC_BCNT_STA_MAX_V105],
    pub rpt_info: rtw89_btc_fbtc_rpt_ctrl_info_v8,
    pub bt_mbx_info: rtw89_btc_fbtc_rpt_ctrl_bt_mailbox,
    pub error_code: __le32,
    pub scbd_w2b: [__le32; 2],
    pub scbd_b2w: [__le32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_rpt_ctrl_ver_info {
    pub v1: rtw89_btc_fbtc_rpt_ctrl_v1,
    pub v4: rtw89_btc_fbtc_rpt_ctrl_v4,
    pub v5: rtw89_btc_fbtc_rpt_ctrl_v5,
    pub v105: rtw89_btc_fbtc_rpt_ctrl_v105,
    pub v7: rtw89_btc_fbtc_rpt_ctrl_v7,
    pub v8: rtw89_btc_fbtc_rpt_ctrl_v8,
    pub v9: rtw89_btc_fbtc_rpt_ctrl_v9,
    pub v11: rtw89_btc_fbtc_rpt_ctrl_v11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fbtc_ext_ctrl_type {
    CXECTL_OFF = 0x0, /* tdma off */
    CXECTL_B2 = 0x1, /* allow B2 (beacon-early) */
    CXECTL_EXT = 0x2,
    CXECTL_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_rxflct {
    pub val: u8,
    pub 3: u8 type:,
    pub 5: u8 tgln_n:,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_cxst_state {
    CXST_OFF = 0x0,
    CXST_B2W = 0x1,
    CXST_W1 = 0x2,
    CXST_W2 = 0x3,
    CXST_W2B = 0x4,
    CXST_B1 = 0x5,
    CXST_B2 = 0x6,
    CXST_B3 = 0x7,
    CXST_B4 = 0x8,
    CXST_LK = 0x9,
    CXST_BLK = 0xa,
    CXST_E2G = 0xb,
    CXST_E5G = 0xc,
    CXST_EBT = 0xd,
    CXST_ENULL = 0xe,
    CXST_WLK = 0xf,
    CXST_W1FDD = 0x10,
    CXST_B1FDD = 0x11,
    CXST_MAX = 0x12,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_cxevnt {
    CXEVNT_TDMA_ENTRY = 0x0,
    CXEVNT_WL_TMR,
    CXEVNT_B1_TMR,
    CXEVNT_B2_TMR,
    CXEVNT_B3_TMR,
    CXEVNT_B4_TMR,
    CXEVNT_W2B_TMR,
    CXEVNT_B2W_TMR,
    CXEVNT_BCN_EARLY,
    CXEVNT_A2DP_EMPTY,
    CXEVNT_LK_END,
    CXEVNT_RX_ISR,
    CXEVNT_RX_FC0,
    CXEVNT_RX_FC1,
    CXEVNT_BT_RELINK,
    CXEVNT_BT_RETRY,
    CXEVNT_E2G,
    CXEVNT_E5G,
    CXEVNT_EBT,
    CXEVNT_ENULL,
    CXEVNT_DRV_WLK,
    CXEVNT_BCN_OK,
    CXEVNT_BT_CHANGE,
    CXEVNT_EBT_EXTEND,
    CXEVNT_E2G_NULL1,
    CXEVNT_B1FDD_TMR,
    CXEVNT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_slot_type {
    SLOT_MIX = 0x0, /* accept BT Lower-Pri Tx/Rx request 0x778 = 1 */
    SLOT_ISO = 0x1, /* no accept BT Lower-Pri Tx/Rx request 0x778 = d*/
    CXSTYPE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_afh_map_type {
    RPT_BT_AFH_SEQ_LEGACY = 0x10,
    RPT_BT_AFH_SEQ_LE = 0x20
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wl_gpio_debug {
    BTC_DBG_GNT_BT = 0,
    BTC_DBG_GNT_WL = 1,
    BTC_DBG_GNT_BT1 = 2,
    BTC_DBG_GNT_WL1 = 3,
// The following signals should 0-1 tiggle by each function-call
    BTC_DBG_BCN_EARLY = 4,
    BTC_DBG_WL_NULL0 = 5,
    BTC_DBG_WL_NULL1 = 6,
    BTC_DBG_WL_RXISR = 7,
    BTC_DBG_TDMA_ENTRY = 8,
    BTC_DBG_A2DP_EMPTY = 9,
    BTC_DBG_BT_RETRY = 10,
// The following signals should 0-1 tiggle by state L/H
    BTC_DBG_BT_RELINK = 11,
    BTC_DBG_SLOT_WL = 12,
    BTC_DBG_SLOT_BT = 13,
// The following signals should 0-1 tiggle by external
    BTC_DBG_WL_ERR = 14,
    BTC_DBG_WL_OK = 15,
// The following signals appear only 1-active at same time
    BTC_DBG_SLOT_B2W = 16,
    BTC_DBG_SLOT_W1 = 17,
    BTC_DBG_SLOT_W2 = 18,
    BTC_DBG_SLOT_W2B = 19,
    BTC_DBG_SLOT_B1 = 20,
    BTC_DBG_SLOT_B2 = 21,
    BTC_DBG_SLOT_B3 = 22,
    BTC_DBG_SLOT_B4 = 23,
    BTC_DBG_SLOT_LK = 24,
    BTC_DBG_SLOT_E2G = 25,
    BTC_DBG_SLOT_E5G = 26,
    BTC_DBG_SLOT_EBT = 27,
    BTC_DBG_SLOT_WLK = 28,
    BTC_DBG_SLOT_B1FDD = 29,
    BTC_DBG_BT_CHANGE = 30,
// The following signals should 0-1 tiggle by external
    BTC_DBG_WL_CCA = 31,

    BTC_DBG_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_gpio_dbg_v1 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxgpiodbg,
    pub rsvd: u8,
    pub rsvd2: __le16,
    pub /: *mut *mut __le32 en_map; / which debug signal (see btc_wl_gpio_debug) is enable,
    pub /: *mut *mut __le32 pre_state; / the debug signal is 1 or 0,
    pub /: *mut *mut u8 gpio_map[BTC_DBG_NUM]; /the debug signals to GPIO-Position,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_gpio_dbg_v7 {
    pub fver: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub gpio_map: [u8; BTC_DBG_NUM],
    pub en_map: __le32,
    pub pre_state: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_gpio_dbg {
    pub v1: rtw89_btc_fbtc_gpio_dbg_v1,
    pub v7: rtw89_btc_fbtc_gpio_dbg_v7,
}

//
// SET_GPIO_CTRL payload (max len = 7 bytes)
//
// type = CXDGPIO_EN_MAP
// data.val[31:0] = debug signal enable map
//
// type = CXDGPIO_MUX_MAP
// data.mux.sig  = debug signal id
// data.mux.gpio = GPIO id
//
// type = CXDGPIO_EXT_HPTA / CXDGPIO_EXT_HMBX / CXDGPIO_EXT_SWOUT
// data.map.map_low  = GPIO 7~0 map
// data.map.map_high = GPIO 15~8 map
//
// type = CXDGPIO_EXT_SWIN
// data.swin.in_map_low  = GPIO 7~0 input-en-map
// data.swin.in_map_high = GPIO 15~8 input-en-map
// data.swin.int_map_low = GPIO 7~0 interrupt source map
// data.swin.int_map_high = GPIO 15~8 interrupt source map
//
pub const CXDGPIO_SET_L4: c_int = 4;
pub const CXDGPIO_SET_L2: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_h2c_set_gpio_en_map {
    pub /: *mut *mut u8 type; / gpio_type,
    pub /: *mut *mut u8 fver; / FCX_VER_GPIODBG,
    pub dlen: u8,
    pub en_map: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_h2c_set_gpio_mux {
    pub /: *mut *mut u8 type; / gpio_type,
    pub /: *mut *mut u8 fver; / FCX_VER_GPIODBG,
    pub dlen: u8,
    pub sig: u8,
    pub gpio: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_h2c_set_gpio_ext_pta {
    pub /: *mut *mut u8 type; / gpio_type,
    pub /: *mut *mut u8 fver; / FCX_VER_GPIODBG,
    pub dlen: u8,
    pub map_low: u8,
    pub map_high: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_h2c_set_gpio_ext_mb {
    pub /: *mut *mut u8 type; / gpio_type,
    pub /: *mut *mut u8 fver; / FCX_VER_GPIODBG,
    pub dlen: u8,
    pub map_low: u8,
    pub map_high: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_h2c_set_gpio_ext_swout {
    pub /: *mut *mut u8 type; / gpio_type,
    pub /: *mut *mut u8 fver; / FCX_VER_GPIODBG,
    pub dlen: u8,
    pub map_low: u8,
    pub map_high: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_h2c_set_gpio_ext_swin {
    pub /: *mut *mut u8 type; / gpio_type,
    pub /: *mut *mut u8 fver; / FCX_VER_GPIODBG,
    pub dlen: u8,
    pub in_map_low: u8,
    pub in_map_high: u8,
    pub int_map_low: u8,
    pub int_map_high: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_h2c_set_gpio_2b {
    pub /: *mut *mut u8 type; / gpio_type,
    pub /: *mut *mut u8 fver; / FCX_VER_GPIODBG,
    pub dlen: u8,
    pub data: [u8; CXDGPIO_SET_L2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_h2c_set_gpio_4b {
    pub /: *mut *mut u8 type; / gpio_type,
    pub /: *mut *mut u8 fver; / FCX_VER_GPIODBG,
    pub dlen: u8,
    pub data: [u8; CXDGPIO_SET_L4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_fbtc_h2c_set_gpio_en_map_u {
    pub fmt: rtw89_fbtc_h2c_set_gpio_4b,
    pub data: rtw89_fbtc_h2c_set_gpio_en_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_fbtc_h2c_set_gpio_mux_u {
    pub fmt: rtw89_fbtc_h2c_set_gpio_2b,
    pub data: rtw89_fbtc_h2c_set_gpio_mux,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_fbtc_h2c_set_gpio_ext_pta_u {
    pub fmt: rtw89_fbtc_h2c_set_gpio_2b,
    pub data: rtw89_fbtc_h2c_set_gpio_ext_pta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_fbtc_h2c_set_gpio_ext_swin_u {
    pub fmt: rtw89_fbtc_h2c_set_gpio_4b,
    pub data: rtw89_fbtc_h2c_set_gpio_ext_swin,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_h2c_set_gpio {
    pub en_map: rtw89_fbtc_h2c_set_gpio_en_map_u,
    pub mux: rtw89_fbtc_h2c_set_gpio_mux_u,
    pub ext_pta: rtw89_fbtc_h2c_set_gpio_ext_pta_u,
    pub ext_mb: rtw89_fbtc_h2c_set_gpio_ext_pta_u,
    pub ext_swout: rtw89_fbtc_h2c_set_gpio_ext_pta_u,
    pub ext_swin: rtw89_fbtc_h2c_set_gpio_ext_swin_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_mreg_val_v1 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxmreg,
    pub reg_num: u8,
    pub rsvd: __le16,
    pub mreg_val: [__le32; CXMREG_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_mreg_val_v2 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxmreg,
    pub reg_num: u8,
    pub rsvd: __le16,
    pub mreg_val: [__le32; CXMREG_MAX_V2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_mreg_val_v7 {
    pub fver: u8,
    pub reg_num: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub mreg_val: [__le32; CXMREG_MAX_V2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_mreg_val {
    pub v1: rtw89_btc_fbtc_mreg_val_v1,
    pub v2: rtw89_btc_fbtc_mreg_val_v2,
    pub v7: rtw89_btc_fbtc_mreg_val_v7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_mreg {
    pub type: __le16,
    pub bytes: __le16,
    pub offset: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_slot {
    pub dur: __le16,
    pub cxtbl: __le32,
    pub cxtype: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_slots {
    pub /: *mut *mut u8 fver; / btc_ver::fcxslots,
    pub tbl_num: u8,
    pub rsvd: __le16,
    pub update_map: __le32,
    pub slot: [rtw89_btc_fbtc_slot; CXST_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_slot_v7 {
    pub /: *mut *mut __le16 dur; / slot duration,
    pub cxtype: __le16,
    pub cxtbl: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_slots_v2 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxslots,
    pub tbl_num: u8,
    pub rsvd: __le16,
    pub update_map: __le32,
    pub slot: [rtw89_btc_fbtc_slot_v7; CXST_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_slot_u16 {
    pub /: *mut *mut __le16 dur; / slot duration,
    pub cxtype: __le16,
    pub /: *mut *mut __le16 cxtbl_l16; / coex table [15:0],
    pub /: *mut *mut __le16 cxtbl_h16; / coex table [31:16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_1slot_v7 {
    pub fver: u8,
    pub /: *mut *mut u8 sid; / slot id,
    pub rsvd: __le16,
    pub slot: rtw89_btc_fbtc_slot_v7,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_slots_v7 {
    pub fver: u8,
    pub slot_cnt: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub slot: [rtw89_btc_fbtc_slot_u16; CXST_MAX],
    pub update_map: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_slots_info {
    pub v1: rtw89_btc_fbtc_slots,
    pub v2: rtw89_btc_fbtc_slots_v2,
    pub v7: rtw89_btc_fbtc_slots_v7,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_step {
    pub type: u8,
    pub val: u8,
    pub difft: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_steps_v2 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxstep,
    pub rsvd: u8,
    pub cnt: __le16,
    pub pos_old: __le16,
    pub pos_new: __le16,
    pub step: [rtw89_btc_fbtc_step; FCXMAX_STEP],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_steps_v3 {
    pub fver: u8,
    pub en: u8,
    pub rsvd: __le16,
    pub cnt: __le32,
    pub step: [rtw89_btc_fbtc_step; FCXMAX_STEP],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_steps_info {
    pub v2: rtw89_btc_fbtc_steps_v2,
    pub v3: rtw89_btc_fbtc_steps_v3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cysta_v2 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxcysta,
    pub rsvd: u8,
    pub /: *mut *mut __le16 cycles; / total cycle number,
    pub cycles_a2dp: [__le16; CXT_FLCTRL_MAX],
    pub /: *mut *mut __le16 a2dpept; / a2dp empty cnt,
    pub cnt*/: *mut *mut __le16 a2dpeptto; / a2dp empty timeout,
    pub /: *mut *mut __le16 tavg_cycle[CXT_MAX]; / avg wl/bt cycle time,
    pub /: *mut *mut __le16 tmax_cycle[CXT_MAX]; / max wl/bt cycle time,
    pub /: *mut *mut __le16 tmaxdiff_cycle[CXT_MAX]; / max wl-wl bt-bt cycle diff time,
    pub /: *mut *mut __le16 tavg_a2dp[CXT_FLCTRL_MAX]; / avg a2dp PSTDMA/TDMA time,
    pub /: *mut *mut __le16 tmax_a2dp[CXT_FLCTRL_MAX]; / max a2dp PSTDMA/TDMA time,
    pub /: *mut *mut __le16 tavg_a2dpept; / avg a2dp empty time,
    pub /: *mut *mut __le16 tmax_a2dpept; / max a2dp empty time,
    pub /: *mut *mut __le16 tavg_lk; / avg leak-slot time,
    pub /: *mut *mut __le16 tmax_lk; / max leak-slot time,
    pub /: *mut *mut __le32 slot_cnt[CXST_MAX]; / slot count,
    pub bcn_cnt: [__le32; CXBCN_MAX],
    pub /: *mut *mut __le32 leakrx_cnt; / the rximr occur at leak slot,
    pub /: *mut *mut __le32 collision_cnt; / counter for event/timer occur at same time,
    pub skip_cnt: __le32,
    pub exception: __le32,
    pub except_cnt: __le32,
    pub tslot_cycle: [__le16; BTC_CYCLE_SLOT_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_fdd_try_info {
    pub cycles: [__le16; CXT_FLCTRL_MAX],
    pub /: *mut *mut __le16 tavg[CXT_FLCTRL_MAX]; / avg try BT-Slot-TDD/BT-slot-FDD time,
    pub /: *mut *mut __le16 tmax[CXT_FLCTRL_MAX]; / max try BT-Slot-TDD/BT-slot-FDD time,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cycle_time_info {
    pub /: *mut *mut __le16 tavg[CXT_MAX]; / avg wl/bt cycle time,
    pub /: *mut *mut __le16 tmax[CXT_MAX]; / max wl/bt cycle time,
    pub /: *mut *mut __le16 tmaxdiff[CXT_MAX]; / max wl-wl bt-bt cycle diff time,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cycle_time_info_v5 {
    pub /: *mut *mut __le16 tavg[CXT_MAX]; / avg wl/bt cycle time,
    pub /: *mut *mut __le16 tmax[CXT_MAX]; / max wl/bt cycle time,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_a2dp_trx_stat {
    pub empty_cnt: u8,
    pub retry_cnt: u8,
    pub tx_rate: u8,
    pub tx_cnt: u8,
    pub ack_cnt: u8,
    pub nack_cnt: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_a2dp_trx_stat_v4 {
    pub empty_cnt: u8,
    pub retry_cnt: u8,
    pub tx_rate: u8,
    pub tx_cnt: u8,
    pub ack_cnt: u8,
    pub nack_cnt: u8,
    pub no_empty_cnt: u8,
    pub rsvd: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cycle_a2dp_empty_info {
    pub /: *mut *mut __le16 cnt; / a2dp empty cnt,
    pub cnt*/: *mut *mut __le16 cnt_timeout; / a2dp empty timeout,
    pub /: *mut *mut __le16 tavg; / avg a2dp empty time,
    pub /: *mut *mut __le16 tmax; / max a2dp empty time,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cycle_leak_info {
    pub /: *mut *mut __le32 cnt_rximr; / the rximr occur at leak slot,
    pub /: *mut *mut __le16 tavg; / avg leak-slot time,
    pub /: *mut *mut __le16 tmax; / max leak-slot time,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cycle_leak_info_v7 {
    pub tavg: __le16,
    pub tamx: __le16,
    pub cnt_rximr: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cycle_fddt_info {
    pub train_cycle: __le16,
    pub tp: __le16,
    pub /: *mut *mut s8 tx_power; / absolute Tx power (dBm), 0xff-> no BTC control,
    pub /: *mut *mut s8 bt_tx_power; / decrease Tx power (dB),
    pub /: *mut *mut s8 bt_rx_gain; / LNA constrain level,
    pub no_empty_cnt: u8,
    pub /: *mut *mut u8 rssi; / [7:4] -> bt_rssi_level, [3:0]-> wl_rssi_level,
    pub /: *mut *mut u8 cn; / condition_num,
    pub /: *mut *mut u8 train_status; / [7:4]-> train-state, [3:0]-> train-phase,
    pub /: *mut *mut u8 train_result; / refer to enum btc_fddt_check_map,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cycle_fddt_info_v5 {
    pub train_cycle: __le16,
    pub tp: __le16,
    pub /: *mut *mut s8 tx_power; / absolute Tx power (dBm), 0xff-> no BTC control,
    pub /: *mut *mut s8 bt_tx_power; / decrease Tx power (dB),
    pub /: *mut *mut s8 bt_rx_gain; / LNA constrain level,
    pub no_empty_cnt: u8,
    pub /: *mut *mut u8 rssi; / [7:4] -> bt_rssi_level, [3:0]-> wl_rssi_level,
    pub /: *mut *mut u8 cn; / condition_num,
    pub /: *mut *mut u8 train_status; / [7:4]-> train-state, [3:0]-> train-phase,
    pub /: *mut *mut u8 train_result; / refer to enum btc_fddt_check_map,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_fddt_cell_status {
    pub wl_tx_pwr: i8,
    pub bt_tx_pwr: i8,
    pub bt_rx_gain: i8,
    pub /: *mut *mut u8 state_phase; / [0:3] train state, [4:7] train phase,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cysta_v3 {
    pub fver: u8,
    pub rsvd: u8,
    pub /: *mut *mut __le16 cycles; / total cycle number,
    pub slot_step_time: [__le16; BTC_CYCLE_SLOT_MAX],
    pub cycle_time: rtw89_btc_fbtc_cycle_time_info,
    pub fdd_try: rtw89_btc_fbtc_fdd_try_info,
    pub a2dp_ept: rtw89_btc_fbtc_cycle_a2dp_empty_info,
    pub a2dp_trx: [rtw89_btc_fbtc_a2dp_trx_stat; BTC_CYCLE_SLOT_MAX],
    pub leak_slot: rtw89_btc_fbtc_cycle_leak_info,
    pub /: *mut *mut __le32 slot_cnt[CXST_MAX]; / slot count,
    pub bcn_cnt: [__le32; CXBCN_MAX],
    pub /: *mut *mut __le32 collision_cnt; / counter for event/timer occur at the same time,
    pub skip_cnt: __le32,
    pub except_cnt: __le32,
    pub except_map: __le32,
    pub __packed: },
pub const FDD_TRAIN_WL_DIRECTION: c_int = 2;
pub const FDD_TRAIN_WL_RSSI_LEVEL: c_int = 5;
pub const FDD_TRAIN_BT_RSSI_LEVEL: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cysta_v4 {
    pub fver: u8,
    pub rsvd: u8,
    pub /: *mut *mut u8 collision_cnt; / counter for event/timer occur at the same time,
    pub except_cnt: u8,
    pub skip_cnt: __le16,
    pub /: *mut *mut __le16 cycles; / total cycle number,
    pub /: *mut *mut __le16 slot_step_time[BTC_CYCLE_SLOT_MAX]; / record the wl/bt slot time,
    pub /: *mut *mut __le16 slot_cnt[CXST_MAX]; / slot count,
    pub bcn_cnt: [__le16; CXBCN_MAX],
    pub cycle_time: rtw89_btc_fbtc_cycle_time_info,
    pub leak_slot: rtw89_btc_fbtc_cycle_leak_info,
    pub a2dp_ept: rtw89_btc_fbtc_cycle_a2dp_empty_info,
    pub a2dp_trx: [rtw89_btc_fbtc_a2dp_trx_stat_v4; BTC_CYCLE_SLOT_MAX],
    pub fddt_trx: [rtw89_btc_fbtc_cycle_fddt_info; BTC_CYCLE_SLOT_MAX],
    pub except_map: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cysta_v5 {
    pub fver: u8,
    pub rsvd: u8,
    pub /: *mut *mut u8 collision_cnt; / counter for event/timer occur at the same time,
    pub except_cnt: u8,
    pub wl_rx_err_ratio: [u8; BTC_CYCLE_SLOT_MAX],
    pub skip_cnt: __le16,
    pub /: *mut *mut __le16 cycles; / total cycle number,
    pub /: *mut *mut __le16 slot_step_time[BTC_CYCLE_SLOT_MAX]; / record the wl/bt slot time,
    pub /: *mut *mut __le16 slot_cnt[CXST_MAX]; / slot count,
    pub bcn_cnt: [__le16; CXBCN_MAX],
    pub cycle_time: rtw89_btc_fbtc_cycle_time_info_v5,
    pub leak_slot: rtw89_btc_fbtc_cycle_leak_info,
    pub a2dp_ept: rtw89_btc_fbtc_cycle_a2dp_empty_info,
    pub a2dp_trx: [rtw89_btc_fbtc_a2dp_trx_stat_v4; BTC_CYCLE_SLOT_MAX],
    pub fddt_trx: [rtw89_btc_fbtc_cycle_fddt_info_v5; BTC_CYCLE_SLOT_MAX],
    pub except_map: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cysta_v105 {
    pub fver: u8,
    pub rsvd: u8,
    pub collision_cnt: u8,
    pub except_cnt: u8,
    pub wl_rx_err_ratio: [u8; BTC_CYCLE_SLOT_MAX],
    pub skip_cnt: __le16,
    pub cycles: __le16,
    pub slot_step_time: [__le16; BTC_CYCLE_SLOT_MAX],
    pub slot_cnt: [__le16; CXST_MAX],
    pub bcn_cnt: [__le16; CXBCN_MAX],
    pub cycle_time: rtw89_btc_fbtc_cycle_time_info_v5,
    pub leak_slot: rtw89_btc_fbtc_cycle_leak_info,
    pub a2dp_ept: rtw89_btc_fbtc_cycle_a2dp_empty_info,
    pub a2dp_trx: [rtw89_btc_fbtc_a2dp_trx_stat_v4; BTC_CYCLE_SLOT_MAX],
    pub except_map: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cysta_v7 {
    pub fver: u8,
    pub rsvd: u8,
    pub /: *mut *mut u8 collision_cnt; / counter for event/timer occur at the same time,
    pub except_cnt: u8,
    pub wl_rx_err_ratio: [u8; BTC_CYCLE_SLOT_MAX],
    pub a2dp_trx: [rtw89_btc_fbtc_a2dp_trx_stat_v4; BTC_CYCLE_SLOT_MAX],
    pub skip_cnt: __le16,
    pub /: *mut *mut __le16 cycles; / total cycle number,
    pub /: *mut *mut __le16 slot_step_time[BTC_CYCLE_SLOT_MAX]; / record the wl/bt slot time,
    pub /: *mut *mut __le16 slot_cnt[CXST_MAX]; / slot count,
    pub bcn_cnt: [__le16; CXBCN_MAX],
    pub cycle_time: rtw89_btc_fbtc_cycle_time_info_v5,
    pub a2dp_ept: rtw89_btc_fbtc_cycle_a2dp_empty_info,
    pub leak_slot: rtw89_btc_fbtc_cycle_leak_info_v7,
    pub except_map: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_cysta_info {
    pub v2: rtw89_btc_fbtc_cysta_v2,
    pub v3: rtw89_btc_fbtc_cysta_v3,
    pub v4: rtw89_btc_fbtc_cysta_v4,
    pub v5: rtw89_btc_fbtc_cysta_v5,
    pub v105: rtw89_btc_fbtc_cysta_v105,
    pub v7: rtw89_btc_fbtc_cysta_v7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cynullsta_v1 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxnullsta,
    pub rsvd: u8,
    pub rsvd2: __le16,
    pub /: *mut *mut __le32 max_t[2]; / max_t for 0:null0/1:null1,
    pub /: *mut *mut __le32 avg_t[2]; / avg_t for 0:null0/1:null1,
    pub /: *mut *mut __le32 result[2][4]; / 0:fail, 1:ok, 2:on_time, 3:retry,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cynullsta_v2 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxnullsta,
    pub rsvd: u8,
    pub rsvd2: __le16,
    pub /: *mut *mut __le32 max_t[2]; / max_t for 0:null0/1:null1,
    pub /: *mut *mut __le32 avg_t[2]; / avg_t for 0:null0/1:null1,
    pub /: *mut *mut __le32 result[2][5]; / 0:fail, 1:ok, 2:on_time, 3:retry, 4:tx,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_cynullsta_v7 {
    pub fver: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub tmax: [__le32; 2],
    pub tavg: [__le32; 2],
    pub result: [__le32; 2][5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_cynullsta_info {
    pub /: *mut *mut rtw89_btc_fbtc_cynullsta_v1 v1; / info from fw,
    pub v2: rtw89_btc_fbtc_cynullsta_v2,
    pub v7: rtw89_btc_fbtc_cynullsta_v7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btver_v1 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxbtver,
    pub rsvd: u8,
    pub rsvd2: __le16,
    pub /: *mut *mut __le32 coex_ver; /bit[15:8]->shared, bit[7:0]->non-shared,
    pub fw_ver: __le32,
    pub feature: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btver_v7 {
    pub fver: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub /: *mut *mut __le32 coex_ver; /bit[15:8]->shared, bit[7:0]->non-shared,
    pub fw_ver: __le32,
    pub feature: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btver_v8 {
    pub fver: u8,
    pub /: *mut *mut u8 bt_id; / 0:BT0, 1:BT1,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub /: *mut *mut __le32 coex_ver; /bit[15:8]->shared, bit[7:0]->non-shared,
    pub fw_ver: __le32,
    pub feature: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_btver {
    pub v1: rtw89_btc_fbtc_btver_v1,
    pub v7: rtw89_btc_fbtc_btver_v7,
    pub v8: rtw89_btc_fbtc_btver_v8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btafh {
    pub /: *mut *mut u8 fver; / btc_ver::fcxbtafh,
    pub rsvd: u8,
    pub rsvd2: __le16,
    pub /: *mut *mut u8 afh_l[4]; /bit0:2402, bit1: 2403.... bit31:2433,
    pub /: *mut *mut u8 afh_m[4]; /bit0:2434, bit1: 2435.... bit31:2465,
    pub /: *mut *mut u8 afh_h[4]; /bit0:2466, bit1:2467......bit14:2480,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btafh_v2 {
    pub /: *mut *mut u8 fver; / btc_ver::fcxbtafh,
    pub rsvd: u8,
    pub rsvd2: u8,
    pub map_type: u8,
    pub afh_l: [u8; 4],
    pub afh_m: [u8; 4],
    pub afh_h: [u8; 4],
    pub afh_le_a: [u8; 4],
    pub afh_le_b: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btafh_v7 {
    pub fver: u8,
    pub map_type: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub /: *mut *mut u8 afh_l[4]; /bit0:2402, bit1:2403.... bit31:2433,
    pub /: *mut *mut u8 afh_m[4]; /bit0:2434, bit1:2435.... bit31:2465,
    pub /: *mut *mut u8 afh_h[4]; /bit0:2466, bit1:2467.....bit14:2480,
    pub afh_le_a: [u8; 4],
    pub afh_le_b: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btafh_v8 {
    pub fver: u8,
    pub map_type: u8,
    pub /: *mut *mut u8 bt_id; / 0:BT0, 1:BT1,
    pub rsvd1: u8,
    pub /: *mut *mut u8 afh_l[4]; /bit0:2402, bit1:2403.... bit31:2433,
    pub /: *mut *mut u8 afh_m[4]; /bit0:2434, bit1:2435.... bit31:2465,
    pub /: *mut *mut u8 afh_h[4]; /bit0:2466, bit1:2467.....bit14:2480,
    pub afh_le_a: [u8; 4],
    pub afh_le_b: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_btdevinfo {
    pub /: *mut *mut u8 fver; / btc_ver::fcxbtdevinfo,
    pub rsvd: u8,
    pub vendor_id: __le16,
    pub /: *mut *mut __le32 dev_name; / only 24 bits valid,
    pub flush_time: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_trx_info {
    pub tx_lvl: u8,
    pub rx_lvl: u8,
    pub wl_rssi: u8,
    pub bt_rssi: u8,
    pub /: *mut *mut s8 wl_tx_power[RTW89_PHY_NUM]; / absolute Tx power (dBm), 0xff-> no BTC control,
    pub /: *mut *mut s8 wl_rx_gain[RTW89_PHY_NUM]; / rx gain table index (TBD.),
    pub /: *mut *mut s8 bt_tx_power[BTC_ALL_BT]; / decrease Tx power (dB),
    pub /: *mut *mut s8 bt_rx_gain[BTC_ALL_BT]; / LNA constrain level,
    pub zb_tx_power: [i8; BTC_ALL_BT],
    pub zb_rx_gain: [i8; BTC_ALL_BT],
    pub /: *mut *mut u8 cn; / condition_num,
    pub nhm: i8,
    pub bt_profile: u8,
    pub rsvd2: u8,
    pub tx_rate: u16,
    pub rx_rate: u16,
    pub tx_tp: u32,
    pub rx_tp: u32,
    pub rx_err_ratio: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_rf_path {
    BTC_RF_S0 = 0,
    BTC_RF_S1 = 1,
    BTC_RF_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_outsrc_set_info_v1 {
    pub rf_band: [u8; BTC_RF_NUM],
    pub btg_rx: [u8; BTC_RF_NUM],
    pub nbtg_tx: [u8; BTC_RF_NUM],
    pub gnt_set: [rtw89_mac_ax_gnt; BTC_RF_NUM],
    pub wlact_set: [rtw89_mac_ax_wl_act; BTC_ALL_BT],
    pub pta_req_hw_band: u8,
    pub rf_gbt_source: u8,
    pub bt_enable_state: u8,
    pub wl_btg_standby_chg: u8,
    pub fbd_group_en: [u8; RTW89_MAC_NUM][2],
    pub rf_center_freq: [__le16; RTW89_MAC_NUM],
    pub fbd_group_bound: [__le16; RTW89_MAC_NUM][2],
    pub freq_diff_thres: [__le16; RTW89_MAC_NUM][BTC_ALL_BT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_outsrc_set_info_v6 {
    pub rf_band: [u8; BTC_RF_NUM],
    pub btg_rx: [u8; BTC_RF_NUM],
    pub nbtg_tx: [u8; BTC_RF_NUM],
    pub gnt_set: [rtw89_btc_gnt_ctrl; RTW89_MAC_AX_COEX_GNT_NR],
    pub wlact_set: [rtw89_mac_ax_wl_act; BTC_ALL_BT_EZL],
    pub pta_req_hw_band: u8,
    pub rf_gbt_source: u8,
    pub bt_enable_state: u8,
    pub bt_plut_type: u8,
    pub wl_tx_limit_en: u8,
    pub fc_exec: u8,
    pub wl_btg_standby_chg: u8,
    pub rsvd: u8,
    pub bb_path_sel_bt: [u8; BTC_RF_NUM],
    pub bb_phy_sel_bt: [u8; RTW89_PHY_NUM],
    pub fbd_group_en: [u8; RTW89_MAC_NUM][2],
    pub rf_center_freq: [__le16; RTW89_MAC_NUM],
    pub freq_diff_thres: [__le16; RTW89_MAC_NUM][BTC_ALL_BT_EZL],
    pub fbd_group_bound: [__le16; RTW89_MAC_NUM][2],
    pub wl_tx_limit_time: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fbtc_outsrc_set_info {
    pub /: *mut *mut u8 rf_band[BTC_RF_NUM]; / 0:2GHz/1:5GHz for MPI_bb_hwsi_ignore_gnt_wl(),
    pub /: *mut *mut u8 btg_rx[BTC_RF_NUM]; / for MPI_bb_btg_bt_rx(),
    pub /: *mut *mut u8 nbtg_tx[BTC_RF_NUM]; / for MPI_bb_nbtg_bt_tx pre=AGC control,
    pub /: *mut *mut rtw89_mac_ax_gnt gnt_set[BTC_RF_NUM]; / refer to btc_gnt_ctrl,
    pub gnt_set_be: [rtw89_btc_gnt_ctrl; RTW89_MAC_AX_COEX_GNT_NR],
    pub wlact_set: [rtw89_mac_ax_wl_act; BTC_ALL_BT_EZL],
    pub /: *mut *mut u8 pta_req_hw_band; / Bind PTA to HWB0 or HWB1, only for 8922a 1-PTA,
    pub /: *mut *mut u8 rf_gbt_source; / gbt from S0 or S1 for RF 0x2[9], only for 8922a,
// The followngs are for 8922c/d  new Multi-PTA design
// 0:BT0/1:BT1/2:ZB on/off for MAC(0xe580[0]/0xe680[0])/ RF 0x4[3:2]
    pub bt_enable_state: u8,
    pub /: *mut *mut u8 bt_plut_type; / BT polluted type, refer to enum btc_plt_map,
    pub wl_tx_limit_en: u8,
    pub fc_exec: u8,
    pub /: *mut *mut u8 wl_btg_standby_chg; / keep RX-IQGen on in standby mode,
    pub rsvd: u8,
    pub /: *mut *mut u8 bb_path_sel_bt[BTC_RF_NUM]; / bb s0(1) select GNT_BT0 or BT1,
    pub /: *mut *mut u8 bb_phy_sel_bt[RTW89_PHY_NUM]; / bb phy0(1) select GNT_BT0 or BT1,
// forbidden group-> bit[1]:fbd rf-band, bit[0]: fbd enable
    pub /: *mut *mut u8 fbd_group_en[RTW89_MAC_NUM][2]; / HWB0/1 +.Group0/1,
// bit[15]-> 0:2G/1:5G,6G, bit[14:0]-> WL HWBx ch freq in MHz
    pub /: *mut *mut u16 rf_center_freq[RTW89_MAC_NUM]; / HWB0/1,
// 11-bit in MHz, freq diff threshold
    pub /: *mut *mut u16 freq_diff_thres[RTW89_MAC_NUM][BTC_ALL_BT_EZL]; / HWB0/1 vs.BT0/1/2,
// forbidden group boundary: [15:8]->UP, [7:0]->LO
    pub /: *mut *mut u16 fbd_group_bound[RTW89_MAC_NUM][2]; / HWB0/1 +.Group0/1,
    pub wl_tx_limit_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_slot_u {
    pub v1: [rtw89_btc_fbtc_slot; CXST_MAX],
    pub v7: [rtw89_btc_fbtc_slot_v7; CXST_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddr_cell {
    pub en: u8,
    pub wl_rx_max: u8,
    pub wl_rx_min: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddr_result {
    pub wl_rx_limit: u8,
    pub /: *mut *mut u8 wl_rx_limit_step[6]; / record search process,
    pub /: *mut *mut u8 search_cnt; / the rx-limit serach count,
    pub wl_tp: u32,
    pub /: *mut *mut u32 wl_tp_step[6]; / record search process,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddr_train_info {
    pub rx_limit_pre: u8,
    pub rx_limit_now: u8,
    pub tp_rec_cnt: u32,
    pub tp_avg_cnt: u32,
    pub wl_tp_pre: u32,
    pub wl_tp_now: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddr_info {
    pub /: *mut *mut u8 state; / refer to enum btc_fddr_state,
    pub /: *mut *mut u8 cell_now; / wl rssi_level after filter LNA != 6,
    pub cell_change: u8,
    pub wl_low_rate: u8,
    pub /: *mut *mut u8 tp_setup_time; / calculate TP after this value (in second),
    pub /: *mut *mut u8 tp_hold_time; / TP calculation period (in second),
    pub /: *mut *mut u8 wl_rssi_thres[BTC_WL_RSSI_THMAX]; / index 0 -> Max RSSI,
    pub /: *mut *mut u8 search_mode; / 0: search, 1:look-up,
    pub /: *mut *mut u8 search_dir; / 0: Max->Min, 1:Min->Max,
    pub /: *mut *mut rtw89_btc_fddr_train_info tctrl; / train flag,
    pub 1]: rtw89_btc_fddr_result cell_result[BTC_WL_RSSI_THMAX +,
    pub /: *mut *mut rtw89_btc_fddr_cell cell[BTC_WL_RSSI_THMAX + 1]; / parameters,
    pub /: *mut *mut u16 wl_rx_rate_thres; / switch to TDD if rx_rate < this threshold,
    pub /: *mut *mut u32 nrsn_map; / the reason map for no-run fdd-traing,
    pub wl_rx_rate_now: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_ctrl_a2dp_empty {
    pub /: *mut *mut u32 cnt_empty; / a2dp empty count,
    pub /: *mut *mut u32 cnt_flowctrl; / a2dp empty flow control counter,
    pub cnt_tx: u32,
    pub cnt_ack: u32,
    pub cnt_nack: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddt_bt_stat {
    pub a2dp_last: rtw89_btc_rpt_ctrl_a2dp_empty,
    pub retry_last: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddt_cell {
    pub wl_pwr_min: i8,
    pub wl_pwr_max: i8,
    pub bt_pwr_dec_max: i8,
    pub bt_rx_gain: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddt_fail_check {
    pub /: *mut *mut u8 check_map; / check pass condition if bit-map = 1,
    pub /: *mut *mut u8 bt_no_empty_cnt; / 0-fail if no bt-empty >= th in train_cycle,
    pub /: *mut *mut u8 wl_tp_ratio; / 1-fail if wl tp rise ratio < th,
    pub /: *mut *mut *mut u8 wl_kpibtr_ratio; / 2-fail if phase_now_tp < phase_last_tp  kpibtr_ratio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddt_break_check {
    pub /: *mut *mut u8 check_map; / check break condition if bit-map = 1,
    pub /: *mut *mut u8 bt_no_empty_cnt; / 0-break if no empty count >= th,
    pub /: *mut *mut u8 wl_tp_ratio; / 1-break if wl tp ratio < th (%),
    pub /: *mut *mut u8 wl_tp_low_bound; / 2-break if wl tp (in Mbps) < th,
    pub /: *mut *mut u8 cn; / 3-break if (cn >= cn_limit) >= th cycle,
    pub /: *mut *mut u8 cell_chg; / 4-break if non-matched-RSSI >= th cycle,
    pub /: *mut *mut s8 nhm_limit; / 5-break if nhm >= th --> ill-condition,
    pub /: *mut *mut u8 cn_limit; / if condition number >= th --> ill-condition,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddt_time_ctrl {
// 1 TDD cycle = w1 + b1, FDD 1cycle = w1fdd-slot + b1fdd-slot
    pub /: *mut *mut u8 m_cycle; / KPI Moving-Average-Cycle: 1~32 cycles,
    pub /: *mut *mut u8 w_cycle; / Start to calcul WKPI after this if train-phase change,
    pub /: *mut *mut u8 k_cycle; / Total kpi-estimate cycles for each training-step,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddt_train_info {
    pub t_ctrl: rtw89_btc_fddt_time_ctrl,
    pub b_chk: rtw89_btc_fddt_break_check,
    pub f_chk: rtw89_btc_fddt_fail_check,
    pub cell_ul: [rtw89_btc_fddt_cell; 5][5],
    pub cell_dl: [rtw89_btc_fddt_cell; 5][5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_fddt_info {
    pub /: *mut *mut u8 type; / refer to enum btc_fddt_type,
    pub /: *mut *mut u8 result; / fw send fdd-training status by c2h,
    pub /: *mut *mut u8 state; / refer to enum btc_fddt_state,
    pub /: *mut *mut u8 wl_iot[6]; / wl bssid,
    pub /: *mut *mut u16 bt_iot; / bt vendor-id,
    pub /: *mut *mut u32 nrsn_map; / the reason map for no-run fdd-traing,
    pub /: *mut *mut rtw89_btc_fddt_bt_stat bt_stat; / bt statistics,
    pub train: rtw89_btc_fddt_train_info,
    pub train_now: rtw89_btc_fddt_train_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_dm {
    pub /: *mut *mut rtw89_btc_fbtc_outsrc_set_info ost_info_last; / outsrc API setup info,
    pub /: *mut *mut rtw89_btc_fbtc_outsrc_set_info ost_info; / outsrc API setup info,
    pub slot: rtw89_btc_fbtc_slot_u,
    pub slot_now: rtw89_btc_fbtc_slot_u,
    pub tdma: rtw89_btc_fbtc_tdma,
    pub tdma_now: rtw89_btc_fbtc_tdma,
    pub gnt_set: [rtw89_btc_gnt_ctrl; RTW89_MAC_AX_COEX_GNT_NR],
    pub gnt_val: [rtw89_btc_gnt_ctrl; RTW89_MAC_AX_COEX_GNT_NR],
    pub wlact_set: [rtw89_mac_ax_wl_act; BTC_ALL_BT_EZL],
    pub /: *mut *mut rtw89_btc_init_info init_info; / pass to wl_fw if offload,
    pub rf_trx_para: rtw89_btc_rf_trx_para_v9,
    pub wl_tx_limit: rtw89_btc_wl_tx_limit_para,
    pub dm_step: rtw89_btc_dm_step,
    pub eslot_ctrl: rtw89_btc_eslot_ctrl,
    pub trx_info: rtw89_btc_trx_info,
    pub error: rtw89_btc_dm_error_map,
    pub tdd_bind: rtw89_btc_bind_info,
    pub fdd_bind: rtw89_btc_bind_info,
    pub fddt_info: rtw89_btc_fddt_info,
    pub fddr_info: rtw89_btc_fddr_info,
    pub wl_trx_nss: rtw89_btc_wl_trx_nss_para,
    pub cnt_dm: [u32; BTC_DCNT_NUM],
    pub cnt_notify: [u32; BTC_NCNT_NUM],
    pub /: *mut *mut u8 ant_xmap[BTC_RF_NUM][BTC_ALL_BT_EZL]; / WL-BT ANT interact-map,
    pub /: *mut *mut u8 xtk_xmap[BTC_RF_NUM][BTC_ALL_BT_EZL]; / 1: If RSSI<(BT-Pin -SIR),
    pub /: *mut *mut u8 sit_xmap[BTC_RF_NUM][BTC_ALL_BT_EZL]; / WL-BT space interact-map,
    pub /: *mut *mut u8 fit_xmap[RTW89_PHY_NUM][BTC_ALL_BT_EZL]; / HWB-BT freq interact-map,
    pub /: *mut *mut u8 tdd_map[BTC_RF_NUM][BTC_ALL_BT_EZL]; / WL-BT tdd-map,
    pub /: *mut *mut u8 fdd_map[BTC_RF_NUM][BTC_ALL_BT_EZL]; / WL-BT fdd-map,
    pub /: *mut *mut u8 corx_map[BTC_RF_NUM][BTC_ALL_BT_EZL]; / WL-BT Co-Rx,
    pub sit_xmap_last: [u8; BTC_RF_NUM][BTC_ALL_BT_EZL],
    pub fit_xmap_last: [u8; RTW89_PHY_NUM][BTC_ALL_BT_EZL],
    pub /: *mut *mut u8 tdd_rssi_thres; / The FDD/TDD switch RSSI (in %),
    pub /: *mut *mut u8 sir_thres; / WL(Signal) to BT(interference Pin) ratio,
    pub /: *mut *mut u8 sir_state[BTC_ALL_BT_EZL]; / 1: WL RSSI > BTx-interference,
    pub update_slot_map: u32,
    pub set_ant_path: u32,
    pub e2g_slot_limit: u32,
    pub e2g_slot_nulltx_time: u32,
    pub 1: u32 wl_only:,
    pub 1: u32 wl_fw_cx_offload:,
    pub 1: u32 freerun:,
    pub 1: u32 fddt_train:,
    pub 2: u32 wl_ps_ctrl:,
    pub 1: u32 wl_mimo_ps:,
    pub 1: u32 leak_ap:,
    pub 3: u32 noisy_level:,
    pub 8: u32 coex_info_map:,
    pub 1: u32 bt_only:,
    pub 2: u32 wl_btg_rx:,
    pub 8: u32 trx_para_level:,
    pub 1: u32 wl_stb_chg:,
    pub 1: u32 pta_owner:,
    pub 1: u32 tdma_instant_excute:,
    pub 2: u32 wl_btg_rx_rb:,
    pub slot_dur: [u16; CXST_MAX],
    pub bt_slot_flood: u16,
    pub run_reason: u8,
    pub run_action: u8,
    pub wl_tx_pwr_phy_map: u8,
    pub vid: u8,
    pub client_ps_tdma_on: u8,
    pub wl_trx_nss_en: u8,
    pub 2: u8 wl_pre_agc:,
    pub 1: u8 wl_lna2:,
    pub 1: u8 freerun_chk:,
    pub 2: u8 wl_pre_agc_rb:,
    pub /: *mut *mut u8 bt_select: 2; / 0:s0, 1:s1, 2:s0 & s1, refer to enum btc_bt_index,
    pub 1: u8 slot_req_more:,
    pub 1: u8 out_of_band:,
    pub 1: u8 fdd_en:,
    pub 1: u8 tdd_en:,
    pub 1: u8 lps_ctrl_scbd:,
    pub 1: u8 lps_ctrl_change:,
    pub /: *mut *mut u8 bis_tdma: 1; / BIS TDMA mode active,
    pub scbd_write_instant: u8,
    pub scbd_b2w_update: bool,
    pub scbd_w2b_update: bool,
    pub pre_agc_chg: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_wl_ctrl_info {
    pub rf_band_map: [u8; RTW89_MAC_NUM],
    pub rf_ch: [u8; RTW89_MAC_NUM],
    pub client_pstdma_on: u8,
    pub fw_scan: u8,
    pub rfk_state: u8,
    pub rfk_type: u8,
    pub smap_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ctrl {
    pub manual: u32,
    pub 1: u32 igno_bt:,
    pub 1: u32 always_freerun:,
    pub 16: u32 trace_step:,
    pub wl_only: u8,
    pub bt_only: u8,
    pub ntfy_type: u8,
    pub wl_ctrl_info: rtw89_fbtc_wl_ctrl_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ctrl_v0 {
    pub 1: u32 manual:,
    pub 1: u32 igno_bt:,
    pub 1: u32 always_freerun:,
    pub 16: u32 trace_step:,
    pub 12: u32 rsvd:,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ctrl_v7 {
    pub manual: u8,
    pub igno_bt: u8,
    pub always_freerun: u8,
    pub rsvd: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fbtc_wl_ctrl_info_v9 {
    pub rf_band_map: [u8; RTW89_MAC_NUM],
    pub rf_ch: [u8; RTW89_MAC_NUM],
    pub client_pstdma_on: u8,
    pub fw_scan: u8,
    pub rfk_state: u8,
    pub rfk_type: u8,
    pub smap_val: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ctrl_v9 {
    pub manual: u8,
    pub always_freerun: u8,
    pub wl_only: u8,
    pub bt_only: u8,
    pub ntfy_type: u8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub wl_ctrl_info: rtw89_fbtc_wl_ctrl_info_v9,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_dbg {
// cmd "rb"
    pub rb_done: bool,
    pub rb_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_btf_fw_event {
    BTF_EVNT_RPT = 0,
    BTF_EVNT_BT_INFO = 1,
    BTF_EVNT_BT_SCBD = 2,
    BTF_EVNT_BT_REG = 3,
    BTF_EVNT_CX_RUNINFO = 4,
    BTF_EVNT_BT_PSD = 5,
    BTF_EVNT_BT_DEV_INFO = 6, /* fwc2hfunc > 0 */
    BTF_EVNT_BT_LEAUDIO_INFO = 7, /* fwc2hfunc > 1 */
    BTF_EVNT_BUF_OVERFLOW,
    BTF_EVNT_C2H_LOOPBACK,
    BTF_EVNT_BT_QUERY_TXPWR, /* fwc2hfunc > 3 */
    BTF_EVNT_ZB_INFO = 11,
    BTF_EVNT_ZB_CH = 12,
    BTF_EVNT_ZB_QUERY_TXPWR = 13,
    BTF_EVNT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_fw_event_report {
    BTC_RPT_TYPE_CTRL = 0x0,
    BTC_RPT_TYPE_TDMA,
    BTC_RPT_TYPE_SLOT,
    BTC_RPT_TYPE_CYSTA,
    BTC_RPT_TYPE_STEP,
    BTC_RPT_TYPE_NULLSTA,
    BTC_RPT_TYPE_FDDT, /* added by ver->fwevntrptl == 1 */
    BTC_RPT_TYPE_MREG,
    BTC_RPT_TYPE_GPIO_DBG,
    BTC_RPT_TYPE_BT_VER,
    BTC_RPT_TYPE_BT_SCAN,
    BTC_RPT_TYPE_BT_AFH,
    BTC_RPT_TYPE_BT_DEVICE,
    BTC_RPT_TYPE_TEST,
    BTC_RPT_TYPE_MAX = 31,

    __BTC_RPT_TYPE_V0_SAME = BTC_RPT_TYPE_NULLSTA,
    __BTC_RPT_TYPE_V0_MAX = 12,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_btc_btf_reg_type {
    REG_MAC = 0x0,
    REG_BB = 0x1,
    REG_RF = 0x2,
    REG_BT_RF = 0x3,
    REG_BT_MODEM = 0x4,
    REG_BT_BLUEWIZE = 0x5,
    REG_BT_VENDOR = 0x6,
    REG_BT_LE = 0x7,
    REG_MAX_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_cmn_info {
    pub rx_cnt: u32,
    pub rx_len: u32,
    pub /: *mut *mut u32 req_len; / expected rsp len,
    pub /: *mut *mut u8 req_fver; / expected rsp fver,
    pub /: *mut *mut u8 rsp_fver; / fver from fw,
    pub valid: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_btc_fbtc_btafh_info {
    pub v1: rtw89_btc_fbtc_btafh,
    pub v2: rtw89_btc_fbtc_btafh_v2,
    pub v7: rtw89_btc_fbtc_btafh_v7,
    pub v8: rtw89_btc_fbtc_btafh_v8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_report_ctrl_state {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub finfo: rtw89_btc_fbtc_rpt_ctrl_ver_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_tdma {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub finfo: rtw89_btc_fbtc_tdma_le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_slots {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub /: *mut *mut rtw89_btc_fbtc_slots_info finfo; / info from fw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_cysta {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub finfo: rtw89_btc_fbtc_cysta_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_step {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub /: *mut *mut rtw89_btc_fbtc_steps_info finfo; / info from fw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_nullsta {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub finfo: rtw89_btc_fbtc_cynullsta_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_mreg {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub /: *mut *mut rtw89_btc_fbtc_mreg_val finfo; / info from fw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_gpio_dbg {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub /: *mut *mut rtw89_btc_fbtc_gpio_dbg finfo; / info from fw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_btver {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub /: *mut *mut rtw89_btc_fbtc_btver finfo; / info from fw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_btscan {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub /: *mut *mut rtw89_btc_fbtc_btscan finfo; / info from fw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_btafh {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub finfo: rtw89_btc_fbtc_btafh_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_rpt_fbtc_btdev {
    pub /: *mut *mut rtw89_btc_rpt_cmn_info cinfo; / common info, by driver,
    pub /: *mut *mut rtw89_btc_fbtc_btdevinfo finfo; / info from fw,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_btfre_type {
    BTFRE_INVALID_INPUT = 0x0, /* invalid input parameters */
    BTFRE_UNDEF_TYPE,
    BTFRE_EXCEPTION,
    BTFRE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_ver {
    pub chip_id: rtw89_core_chip_id,
    pub fw_ver_code: u32,
    pub fcxbtcrpt: u8,
    pub fcxtdma: u8,
    pub fcxslots: u8,
    pub fcxcysta: u8,
    pub fcxstep: u8,
    pub fcxnullsta: u8,
    pub fcxmreg: u8,
    pub fcxgpiodbg: u8,
    pub fcxbtver: u8,
    pub fcxbtscan: u8,
    pub fcxbtafh: u8,
    pub fcxbtdevinfo: u8,
    pub fwlrole: u8,
    pub frptmap: u8,
    pub fcxctrl: u8,
    pub fcxinit: u8,
    pub fwevntrptl: u8,
    pub fwc2hfunc: u8,
    pub drvinfo_ver: u8,
    pub info_buf: u16,
    pub max_role_num: u8,
    pub fcxosi: u8,
    pub fcxmlo: u8,
    pub bt_desired: u8,
    pub fcxtrx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_btf_fwinfo {
    pub cnt_c2h: u32,
    pub cnt_h2c: u32,
    pub cnt_h2c_fail: u32,
    pub event: [u32; BTF_EVNT_MAX],
    pub err: [u32; BTFRE_MAX],
    pub len_mismch: u32,
    pub fver_mismch: u32,
    pub rpt_en_map: u32,
    pub fw_subver: rtw89_btc_ver,
    pub rpt_ctrl: rtw89_btc_report_ctrl_state,
    pub rpt_fbtc_tdma: rtw89_btc_rpt_fbtc_tdma,
    pub rpt_fbtc_slots: rtw89_btc_rpt_fbtc_slots,
    pub rpt_fbtc_cysta: rtw89_btc_rpt_fbtc_cysta,
    pub rpt_fbtc_step: rtw89_btc_rpt_fbtc_step,
    pub rpt_fbtc_nullsta: rtw89_btc_rpt_fbtc_nullsta,
    pub rpt_fbtc_mregval: rtw89_btc_rpt_fbtc_mreg,
    pub rpt_fbtc_gpio_dbg: rtw89_btc_rpt_fbtc_gpio_dbg,
    pub rpt_fbtc_btver: rtw89_btc_rpt_fbtc_btver,
    pub rpt_fbtc_btscan: rtw89_btc_rpt_fbtc_btscan,
    pub rpt_fbtc_btafh: rtw89_btc_rpt_fbtc_btafh,
    pub rpt_fbtc_btdev: rtw89_btc_rpt_fbtc_btdev,
}

pub const RTW89_BTC_POLICY_MAXLEN: c_int = 512;
pub const BTC_H2C_MAXLENC: c_int = 2020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc {
    pub ver: *const rtw89_btc_ver,
    pub cx: rtw89_btc_cx,
    pub dm: rtw89_btc_dm,
    pub ctrl: rtw89_btc_ctrl,
    pub mdinfo: rtw89_btc_module,
    pub fwinfo: rtw89_btc_btf_fwinfo,
    pub dbg: rtw89_btc_dbg,
    pub gpio: rtw89_fbtc_h2c_set_gpio,
    pub eapol_notify_work: wiphy_work,
    pub arp_notify_work: wiphy_work,
    pub dhcp_notify_work: wiphy_work,
    pub icmp_notify_work: wiphy_work,
    pub bt_req_len: [u32; RTW89_PHY_NUM],
    pub policy: [u8; RTW89_BTC_POLICY_MAXLEN],
    pub /: *mut *mut u8 hbuf[BTC_H2C_MAXLENC]; / H2C Macro buffer,
    pub /: *mut *mut u8 hbuf_cnt; / H2C cmd count in buffer,
    pub ant_type: u8,
    pub btg_pos: u8,
    pub io_oflld_type: u8,
    pub policy_len: u16,
    pub policy_type: u16,
    pub /: *mut *mut u16 hbuf_len; / H2C used length, it sshould be <= BTC_H2C_MAXLEN,
    pub hubmsg_cnt: u32,
    pub bt_req_en: bool,
    pub update_policy_force: bool,
    pub btc_ctrl_lps: bool,
    pub manual_ctrl: bool,
    pub cli_h2c_cmd: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_hmsg {
    RTW89_BTC_HMSG_TMR_EN = 0x0,
    RTW89_BTC_HMSG_BT_REG_READBACK = 0x1,
    RTW89_BTC_HMSG_SET_BT_REQ_SLOT = 0x2,
    RTW89_BTC_HMSG_FW_EV = 0x3,
    RTW89_BTC_HMSG_BT_LINK_CHG = 0x4,
    RTW89_BTC_HMSG_SET_BT_REQ_STBC = 0x5,

    NUM_OF_RTW89_BTC_HMSG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ra_mode {
    RTW89_RA_MODE_CCK = BIT(0),
    RTW89_RA_MODE_OFDM = BIT(1),
    RTW89_RA_MODE_HT = BIT(2),
    RTW89_RA_MODE_VHT = BIT(3),
    RTW89_RA_MODE_HE = BIT(4),
    RTW89_RA_MODE_EHT = BIT(5),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ra_report_mode {
    RTW89_RA_RPT_MODE_LEGACY,
    RTW89_RA_RPT_MODE_HT,
    RTW89_RA_RPT_MODE_VHT,
    RTW89_RA_RPT_MODE_HE,
    RTW89_RA_RPT_MODE_EHT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_dig_noisy_level {
    RTW89_DIG_NOISY_LEVEL0 = -1,
    RTW89_DIG_NOISY_LEVEL1 = 0,
    RTW89_DIG_NOISY_LEVEL2 = 1,
    RTW89_DIG_NOISY_LEVEL3 = 2,
    RTW89_DIG_NOISY_LEVEL_MAX = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_gi_ltf {
    RTW89_GILTF_LGI_4XHE32 = 0,
    RTW89_GILTF_SGI_4XHE08 = 1,
    RTW89_GILTF_2XHE16 = 2,
    RTW89_GILTF_2XHE08 = 3,
    RTW89_GILTF_1XHE16 = 4,
    RTW89_GILTF_1XHE08 = 5,
    RTW89_GILTF_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rx_frame_type {
    RTW89_RX_TYPE_MGNT = 0,
    RTW89_RX_TYPE_CTRL = 1,
    RTW89_RX_TYPE_DATA = 2,
    RTW89_RX_TYPE_RSVD = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_efuse_block {
    RTW89_EFUSE_BLOCK_SYS = 0,
    RTW89_EFUSE_BLOCK_RF = 1,
    RTW89_EFUSE_BLOCK_HCI_DIG_PCIE_SDIO = 2,
    RTW89_EFUSE_BLOCK_HCI_DIG_USB = 3,
    RTW89_EFUSE_BLOCK_HCI_PHY_PCIE = 4,
    RTW89_EFUSE_BLOCK_HCI_PHY_USB3 = 5,
    RTW89_EFUSE_BLOCK_HCI_PHY_USB2 = 6,
    RTW89_EFUSE_BLOCK_ADIE = 7,

    RTW89_EFUSE_BLOCK_NUM,
    RTW89_EFUSE_BLOCK_IGNORE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ra_info {
    pub is_dis_ra:1: u8,
// Bit0 : CCK
// Bit1 : OFDM
// Bit2 : HT
// Bit3 : VHT
// Bit4 : HE
// Bit5 : EHT
//
    pub mode_ctrl:6: u8,
    pub /: *mut *mut u8 bw_cap:3; / enum rtw89_bandwidth,
    pub macid: u8,
    pub dcm_cap:1: u8,
    pub er_cap:1: u8,
    pub init_rate_lv:2: u8,
    pub upd_all:1: u8,
    pub en_sgi:1: u8,
    pub ldpc_cap:1: u8,
    pub stbc_cap:1: u8,
    pub ss_num:3: u8,
    pub giltf:3: u8,
    pub upd_bw_nss_mask:1: u8,
    pub upd_mask:1: u8,
    pub /: *mut *mut u64 ra_mask; / 63 bits ra_mask + 1 bit CSI ctrl,
// BFee CSI
    pub band_num: u8,
    pub ra_csi_rate_en:1: u8,
    pub fixed_csi_rate_en:1: u8,
    pub cr_tbl_sel:1: u8,
    pub fix_giltf_en:1: u8,
    pub fix_giltf:3: u8,
    pub partial_bw_er:1: u8,
    pub csi_mcs_ss_idx: u8,
    pub csi_mode:2: u8,
    pub csi_gi_ltf:3: u8,
    pub csi_bw:3: u8,
// after v1
    pub is_noisy:1: u8,
    pub psra_en:1: u8,
    pub rsvd0:1: u8,
    pub macid_msb:2: u8,
    pub /: *mut *mut u8 band:2; / enum rtw89_band,
    pub is_new_dbgreg:1: u8,
}

pub const RTW89_PPDU_MAC_INFO_USR_SIZE: c_int = 4;
pub const RTW89_PPDU_MAC_INFO_SIZE: c_int = 8;
pub const RTW89_PPDU_MAC_RX_CNT_SIZE: c_int = 96;
pub const RTW89_PPDU_MAC_RX_CNT_SIZE_V1: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ampdu_params {
    pub agg_num: u16,
    pub amsdu: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ra_report {
    pub txrate: rate_info,
    pub bit_rate: u32,
    pub hw_rate: u16,
    pub retry_ratio: u8,
    pub might_fallback_legacy: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ba_cam_entry {
    pub list: list_head,
    pub tid: u8,
}

pub const RTW89_MAX_ADDR_CAM_NUM: c_int = 128;
pub const RTW89_MAX_BSSID_CAM_NUM: c_int = 20;
pub const RTW89_MAX_SEC_CAM_NUM: c_int = 128;
pub const RTW89_MAX_BA_CAM_NUM: c_int = 24;
pub const RTW89_SEC_CAM_IN_ADDR_CAM: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_addr_cam_entry {
    pub addr_cam_idx: u8,
    pub offset: u8,
    pub len: u8,
    pub 1: u8 valid :,
    pub 6: u8 addr_mask :,
    pub 1: u8 wapi :,
    pub 2: u8 mask_sel :,
    pub 6: u8 bssid_cam_idx:,
    pub sec_ent_mode: u8,
    pub RTW89_SEC_CAM_IN_ADDR_CAM): DECLARE_BITMAP(sec_cam_map,,
    pub sec_ent_keyid: [u8; RTW89_SEC_CAM_IN_ADDR_CAM],
    pub sec_ent: [u8; RTW89_SEC_CAM_IN_ADDR_CAM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_bssid_cam_entry {
    pub bssid: [u8; ETH_ALEN],
    pub phy_idx: u8,
    pub bssid_cam_idx: u8,
    pub offset: u8,
    pub len: u8,
    pub 1: u8 valid :,
    pub num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sec_cam_entry {
    pub sec_cam_idx: u8,
    pub offset: u8,
    pub len: u8,
    pub 4: u8 type :,
    pub 1: u8 ext_key :,
    pub 1: u8 spp_mode :,
// 256 bits
    pub key: [u8; 32],
    pub key_conf: *mut ieee80211_key_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sta_link {
    pub rtwsta: *mut rtw89_sta,
    pub dlink_schd: list_head,
    pub link_id: c_uint,
    pub mac_id: u8,
    pub tx_retry: u8,
    pub er_cap: bool,
    pub rtwvif_link: *mut rtw89_vif_link,
    pub ra: rtw89_ra_info,
    pub ra_report: rtw89_ra_report,
    pub max_agg_wait: c_int,
    pub prev_rssi: u8,
    pub avg_rssi: ewma_rssi,
    pub rssi: [ewma_rssi; RF_PATH_MAX],
    pub avg_snr: ewma_snr,
    pub evm_1ss: ewma_evm,
    pub evm_min: [ewma_evm; RF_PATH_MAX],
    pub evm_max: [ewma_evm; RF_PATH_MAX],
    pub rx_status: ieee80211_rx_status,
    pub rx_hw_rate: u16,
    pub htc_template: __le32,
    pub /: *mut *mut rtw89_addr_cam_entry addr_cam; / AP mode or TDLS peer only,
    pub /: *mut *mut rtw89_bssid_cam_entry bssid_cam; / TDLS peer only,
    pub ba_cam_list: list_head,
    pub use_cfg_mask: bool,
    pub mask: cfg80211_bitrate_mask,
    pub cctl_tx_time: bool,
    pub ampdu_max_time:4: u32,
    pub cctl_tx_retry_limit: bool,
    pub data_tx_cnt_lmt:6: u32,
}

pub const RTW89_EFUSE_SN_LEN: c_int = 5;
pub const RTW89_EFUSE_UUID_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_efuse {
    pub valid: bool,
    pub power_k_valid: bool,
    pub vcore_valid: bool,
    pub dswr_valid: bool,
    pub xtal_cap: u8,
    pub addr: [u8; ETH_ALEN],
    pub rfe_type: u8,
    pub country_code: [c_char; 2],
    pub adc_td: u8,
    pub bt_setting_2: u8,
    pub bt_setting_3: u8,
    pub sn: [u8; RTW89_EFUSE_SN_LEN],
    pub uuid: [u8; RTW89_EFUSE_UUID_LEN],
    pub vcore_vmax_reduce: u8,
    pub dswr_vmin: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_rate_pattern {
    pub ra_mask: u64,
    pub rate: u16,
    pub ra_mode: u8,
    pub enable: bool,
}

pub const RTW89_TX_DONE: c_uint = 0x0;
pub const RTW89_TX_RETRY_LIMIT: c_uint = 0x1;
pub const RTW89_TX_LIFE_TIME: c_uint = 0x2;
pub const RTW89_TX_MACID_DROP: c_uint = 0x3;
pub const RTW89_MAX_TX_RPTS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tx_rpt {
    pub skbs: [*mut sk_buff; RTW89_MAX_TX_RPTS],
// protect skbs array access/modification
    pub skb_lock: spinlock_t,
    pub sn: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tx_wait_info {
    pub rcu_head: rcu_head,
    pub list: list_head,
    pub completion: completion,
    pub skb: *mut sk_buff,
    pub tx_done: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tx_skb_data {
    pub wait: *mut rtw89_tx_wait_info __rcu,
    pub tx_rpt_sn: u8,
    pub tx_pkt_cnt_lmt: u8,
    pub hci_priv: [u8; ],
}

pub const RTW89_SCAN_NULL_TIMEOUT: c_int = 30;
pub const RTW89_ROC_IDLE_TIMEOUT: c_int = 500;
pub const RTW89_ROC_TX_TIMEOUT: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_roc_state {
    RTW89_ROC_IDLE,
    RTW89_ROC_NORMAL,
    RTW89_ROC_MGMT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_roc {
    pub chan: ieee80211_channel,
    pub roc_work: wiphy_delayed_work,
    pub type: ieee80211_roc_type,
    pub state: rtw89_roc_state,
    pub duration: c_int,
    pub link_id: c_uint,
}

pub const RTW89_P2P_MAX_NOA_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_p2p_ie_head {
    pub eid: u8,
    pub ie_len: u8,
    pub oui: [u8; 3],
    pub oui_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_noa_attr_head {
    pub attr_type: u8,
    pub attr_len: __le16,
    pub index: u8,
    pub oppps_ctwindow: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_p2p_noa_ie {
    pub p2p_head: rtw89_p2p_ie_head,
    pub noa_head: rtw89_noa_attr_head,
    pub noa_desc: [ieee80211_p2p_noa_desc; RTW89_P2P_MAX_NOA_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_p2p_noa_setter {
    pub ie: rtw89_p2p_noa_ie,
    pub noa_count: u8,
    pub noa_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ps_noa_once_handler {
    pub in_duration: bool,
    pub tsf_begin: u64,
    pub tsf_end: u64,
    pub set_work: wiphy_delayed_work,
    pub clr_work: wiphy_delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_vif_link {
    pub rtwvif: *mut rtw89_vif,
    pub dlink_schd: list_head,
    pub link_id: c_uint,
    pub /: *mut *mut bool chanctx_assigned; / only valid when running with chanctx_ops,
    pub chanctx_idx: rtw89_chanctx_idx,
    pub reg_6ghz_power: rtw89_reg_6ghz_power,
    pub reg_6ghz_tpe: rtw89_reg_6ghz_tpe,
    pub mac_id: u8,
    pub port: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub phy_idx: u8,
    pub mac_idx: u8,
    pub net_type: u8,
    pub wifi_role: u8,
    pub self_role: u8,
    pub wmm: u8,
    pub bcn_hit_cond: u8,
    pub bcn_bw_idx: u8,
    pub hit_rule: u8,
    pub last_noa_nr: u8,
    pub sync_bcn_tsf: u64,
    pub last_sync_bcn_tsf: u64,
    pub rand_tsf_done: bool,
    pub trigger: bool,
    pub lsig_txop: bool,
    pub tgt_ind: u8,
    pub frm_tgt_ind: u8,
    pub wowlan_pattern: bool,
    pub wowlan_uc: bool,
    pub wowlan_magic: bool,
    pub is_hesta: bool,
    pub last_a_ctrl: bool,
    pub dyn_tb_bedge_en: bool,
    pub pre_pwr_diff_en: bool,
    pub pwr_diff_en: bool,
    pub def_tri_idx: u8,
    pub update_beacon_work: wiphy_work,
    pub csa_beacon_work: wiphy_delayed_work,
    pub addr_cam: rtw89_addr_cam_entry,
    pub bssid_cam: rtw89_bssid_cam_entry,
    pub tx_params: [ieee80211_tx_queue_params; IEEE80211_NUM_ACS],
    pub rate_pattern: rtw89_phy_rate_pattern,
    pub general_pkt_list: list_head,
    pub p2p_noa: rtw89_p2p_noa_setter,
    pub noa_once: rtw89_ps_noa_once_handler,
    pub mcc_gc_detect_beacon_work: wiphy_delayed_work,
    pub detect_bcn_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_lv1_rcvy_step {
    RTW89_LV1_RCVY_STEP_1,
    RTW89_LV1_RCVY_STEP_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hci_ops {
    pub tx_req): *mut *mut *mut int (tx_write)(struct rtw89_dev rtwdev, struct rtw89_core_tx_request,
    pub txch): *mut *mut *mut void (tx_kick_off)(struct rtw89_dev rtwdev, u8,
    pub drop): *mut *mut *mut void (flush_queues)(struct rtw89_dev rtwdev, u32 queues, bool,
    pub rtwdev): *mut *mut void (reset)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (start)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (stop)(struct rtw89_dev,
    pub pause): *mut *mut *mut void (pause)(struct rtw89_dev rtwdev, bool,
    pub low_power): *mut *mut *mut void (switch_mode)(struct rtw89_dev rtwdev, bool,
    pub rtwdev): *mut *mut void (recalc_int_mit)(struct rtw89_dev,
    pub addr): *mut *mut *mut u8 (read8)(struct rtw89_dev rtwdev, u32,
    pub addr): *mut *mut *mut u16 (read16)(struct rtw89_dev rtwdev, u32,
    pub addr): *mut *mut *mut u32 (read32)(struct rtw89_dev rtwdev, u32,
    pub data): *mut *mut *mut void (write8)(struct rtw89_dev rtwdev, u32 addr, u8,
    pub data): *mut *mut *mut void (write16)(struct rtw89_dev rtwdev, u32 addr, u16,
    pub data): *mut *mut *mut void (write32)(struct rtw89_dev rtwdev, u32 addr, u32,
    pub addr): *mut *mut *mut u32 (read32_pci_cfg)(struct rtw89_dev rtwdev, u32,
    pub rtwdev): *mut *mut int (mac_pre_init)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (mac_pre_deinit)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (mac_post_init)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (deinit)(struct rtw89_dev,
    pub txch): *mut *mut *mut u32 (check_and_reclaim_tx_resource)(struct rtw89_dev rtwdev, u8,
    pub step): *mut *mut *mut int (mac_lv1_rcvy)(struct rtw89_dev rtwdev, enum rtw89_lv1_rcvy_step,
    pub rtwdev): *mut *mut void (dump_err_status)(struct rtw89_dev,
    pub budget): *mut *mut *mut int (napi_poll)(struct napi_struct napi, int,
// Deal with locks inside recovery_start and recovery_complete callbacks
// by hci instance, and handle things which need to consider under SER.
// e.g. turn on/off interrupts except for the one for halt notification.
//
    pub rtwdev): *mut *mut void (recovery_start)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (recovery_complete)(struct rtw89_dev,
    pub enable): *mut *mut *mut void (ctrl_txdma_ch)(struct rtw89_dev rtwdev, bool,
    pub enable): *mut *mut *mut void (ctrl_txdma_fw_ch)(struct rtw89_dev rtwdev, bool,
    pub enable): *mut *mut *mut void (ctrl_trxhci)(struct rtw89_dev rtwdev, bool,
    pub rtwdev): *mut *mut int (poll_txdma_ch_idle)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (clr_idx_all)(struct rtw89_dev,
    pub pdev): *mut *mut *mut void (clear)(struct rtw89_dev rtwdev, struct pci_dev,
    pub rtwdev): *mut *mut void (disable_intr)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (enable_intr)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (rst_bdram)(struct rtw89_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hci_info {
    pub ops: *const rtw89_hci_ops,
    pub type: rtw89_hci_type,
    pub dle_type: rtw89_hci_dle_type,
    pub rpwm_addr: u32,
    pub cpwm_addr: u32,
    pub paused: bool,
    pub tx_rpt_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chip_ops {
    pub rtwdev): *mut *mut int (enable_bb_rf)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (disable_bb_rf)(struct rtw89_dev,
    pub phy_idx): *mut *mut *mut void (bb_preinit)(struct rtw89_dev rtwdev, enum rtw89_phy_idx,
    pub phy_idx): *mut *mut *mut void (bb_postinit)(struct rtw89_dev rtwdev, enum rtw89_phy_idx,
    pub phy_idx): rtw89_phy_idx,
    pub rtwdev): *mut *mut void (bb_sethw)(struct rtw89_dev,
    pub mask): u32 addr, u32,
    pub data): u32 addr, u32 mask, u32,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): rtw89_phy_idx,
    pub block): rtw89_efuse_block,
    pub phycap_map): *mut *mut *mut int (read_phycap)(struct rtw89_dev rtwdev, u8,
    pub rtwdev): *mut *mut void (fem_setup)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (data_setup)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (rfe_gpio)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (rfk_hw_init)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (rfk_init)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (rfk_init_late)(struct rtw89_dev,
    pub rtwvif_link): *mut *mut *mut void (rfk_channel)(struct rtw89_dev rtwdev, struct rtw89_vif_link,
    pub chan): *const rtw89_chan,
    pub start): bool,
    pub rtwdev): *mut *mut void (rfk_track)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (power_trim)(struct rtw89_dev,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): *mut *mut *mut int (init_txpwr_unit)(struct rtw89_dev rtwdev, enum rtw89_phy_idx,
    pub rf_path): *mut *mut *mut u8 (get_thermal)(struct rtw89_dev rtwdev, enum rtw89_rf_path,
    pub chan): *const rtw89_chan,
    pub phy_idx): rtw89_phy_idx,
    pub status): *mut ieee80211_rx_status,
    pub phy_ppdu): *mut rtw89_rx_phy_ppdu,
    pub rx_status): *mut ieee80211_rx_status,
    pub phy_idx): rtw89_phy_idx,
    pub rtwdev): *mut *mut void (cfg_txrx_path)(struct rtw89_dev,
    pub mac_idx): s8 pw_ofst, enum rtw89_mac_idx,
    pub phy_idx): rtw89_phy_idx,
    pub calc): *mut rtw89_phy_calc_efuse_gain,
    pub bb): *mut rtw89_bb_ctx,
    pub rtwdev): *mut *mut int (pwr_on_func)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (pwr_off_func)(struct rtw89_dev,
    pub data_offset): *mut *mut u8 data, u32,
    pub txdesc): *mut c_void,
    pub txdesc): *mut c_void,
    pub qsel): *mut *mut *mut u8 (get_ch_dma[RTW89_HCI_TYPE_NUM])(struct rtw89_dev rtwdev, u8,
    pub wl): *mut *mut *mut int (cfg_ctrl_path)(struct rtw89_dev rtwdev, bool,
    pub gnt_cfg): *const rtw89_mac_ax_coex_gnt,
    pub sel): *mut *mut u32 tx_en, enum rtw89_sch_tx_sel,
    pub tx_en): *mut *mut *mut int (resume_sch_tx)(struct rtw89_dev rtwdev, u8 mac_idx, u32,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub punctured): u16,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub params): *mut bool valid, struct ieee80211_ampdu_params,
    pub cam_info): *mut rtw89_wow_cam_info,
    pub rtwdev): *mut *mut void (btc_set_rfe)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (btc_init_cfg)(struct rtw89_dev,
    pub state): *mut *mut *mut void (btc_set_wl_pri)(struct rtw89_dev rtwdev, u8 map, bool,
    pub txpwr_val): *mut *mut *mut void (btc_set_wl_txpwr_ctrl)(struct rtw89_dev rtwdev, u32,
    pub val): *mut *mut *mut s8 (btc_get_bt_rssi)(struct rtw89_dev rtwdev, s8,
    pub rtwdev): *mut *mut void (btc_update_bt_cnt)(struct rtw89_dev,
    pub state): *mut *mut *mut void (btc_wl_s1_standby)(struct rtw89_dev rtwdev, bool,
    pub policy_type): *mut *mut *mut void (btc_set_policy)(struct rtw89_dev rtwdev, u16,
    pub level): *mut *mut *mut void (btc_set_wl_rx_gain)(struct rtw89_dev rtwdev, u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_dma_ch {
    RTW89_DMA_ACH0 = 0,
    RTW89_DMA_ACH1 = 1,
    RTW89_DMA_ACH2 = 2,
    RTW89_DMA_ACH3 = 3,
    RTW89_DMA_ACH4 = 4,
    RTW89_DMA_ACH5 = 5,
    RTW89_DMA_ACH6 = 6,
    RTW89_DMA_ACH7 = 7,
    RTW89_DMA_B0MG = 8,
    RTW89_DMA_B0HI = 9,
    RTW89_DMA_B1MG = 10,
    RTW89_DMA_B1HI = 11,
    RTW89_DMA_H2C = 12,
    RTW89_DMA_CH_NUM = 13
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mlo_dbcc_mode {
    MLO_DBCC_NOT_SUPPORT = 1,
    MLO_0_PLUS_2_1RF = MLO_MODE_FOR_BB0_BB1_RF(0, 2, 1),
    MLO_0_PLUS_2_2RF = MLO_MODE_FOR_BB0_BB1_RF(0, 2, 2),
    MLO_1_PLUS_1_1RF = MLO_MODE_FOR_BB0_BB1_RF(1, 1, 1),
    MLO_1_PLUS_1_2RF = MLO_MODE_FOR_BB0_BB1_RF(1, 1, 2),
    MLO_2_PLUS_0_1RF = MLO_MODE_FOR_BB0_BB1_RF(2, 0, 1),
    MLO_2_PLUS_0_2RF = MLO_MODE_FOR_BB0_BB1_RF(2, 0, 2),
    MLO_2_PLUS_2_2RF = MLO_MODE_FOR_BB0_BB1_RF(2, 2, 2),
    DBCC_LEGACY = 0xffffffff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_scan_be_operation {
    RTW89_SCAN_OP_STOP,
    RTW89_SCAN_OP_START,
    RTW89_SCAN_OP_SETPARM,
    RTW89_SCAN_OP_GETRPT,
    RTW89_SCAN_OP_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_scan_be_mode {
    RTW89_SCAN_MODE_SA,
    RTW89_SCAN_MODE_MACC,
    RTW89_SCAN_MODE_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_scan_be_opmode {
    RTW89_SCAN_OPMODE_NONE,
    RTW89_SCAN_OPMODE_TBTT,
    RTW89_SCAN_OPMODE_INTV,
    RTW89_SCAN_OPMODE_CNT,
    RTW89_SCAN_OPMODE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_scan_option {
    pub enable: bool,
    pub target_ch_mode: bool,
    pub num_macc_role: u8,
    pub num_opch: u8,
    pub repeat: u8,
    pub norm_pd: u16,
    pub slow_pd: u16,
    pub norm_cy: u16,
    pub opch_end: u8,
    pub /: *mut *mut u16 delay; / in unit of ms,
    pub prohib_chan: u64,
    pub band: rtw89_phy_idx,
    pub operation: rtw89_scan_be_operation,
    pub scan_mode: rtw89_scan_be_mode,
    pub mlo_mode: rtw89_mlo_dbcc_mode,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_qta_mode {
    RTW89_QTA_SCC,
    RTW89_QTA_DBCC,
    RTW89_QTA_DLFW,
    RTW89_QTA_WOW,

// keep last
    RTW89_QTA_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hfc_ch_cfg {
    pub min: u16,
    pub max: u16,
pub const grp_0: c_int = 0;
pub const grp_1: c_int = 1;
pub const grp_num: c_int = 2;
    pub grp: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hfc_ch_info {
    pub aval: u16,
    pub used: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hfc_pub_cfg {
    pub grp0: u16,
    pub grp1: u16,
    pub pub_max: u16,
    pub wp_thrd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hfc_pub_info {
    pub g0_used: u16,
    pub g1_used: u16,
    pub g0_aval: u16,
    pub g1_aval: u16,
    pub pub_aval: u16,
    pub wp_aval: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hfc_prec_cfg {
    pub ch011_prec: u16,
    pub h2c_prec: u16,
    pub wp_ch07_prec: u16,
    pub wp_ch811_prec: u16,
    pub ch011_full_cond: u8,
    pub h2c_full_cond: u8,
    pub wp_ch07_full_cond: u8,
    pub wp_ch811_full_cond: u8,
// for WiFi 7 chips after 8922D
    pub ch011_full_page: u16,
    pub h2c_full_page: u16,
    pub wp_ch07_full_page: u16,
    pub wp_ch811_full_page: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hfc_param {
    pub en: bool,
    pub h2c_en: bool,
    pub mode: u8,
    pub ch_cfg: *const rtw89_hfc_ch_cfg,
    pub ch_info: [rtw89_hfc_ch_info; RTW89_DMA_CH_NUM],
    pub pub_cfg: rtw89_hfc_pub_cfg,
    pub pub_info: rtw89_hfc_pub_info,
    pub prec_cfg: rtw89_hfc_prec_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hfc_param_ini {
    pub ch_cfg: *const rtw89_hfc_ch_cfg,
    pub pub_cfg: *const rtw89_hfc_pub_cfg,
    pub prec_cfg: *const rtw89_hfc_prec_cfg,
    pub mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dle_size {
    pub pge_size: u16,
    pub lnk_pge_num: u16,
    pub unlnk_pge_num: u16,
// for WiFi 7 chips below (suffix v1)
    pub srt_ofst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_wde_quota {
    pub hif: u16,
    pub wcpu: u16,
// unused dcpu isn't listed
    pub pkt_in: u16,
    pub cpu_io: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ple_quota {
    pub cma0_tx: u16,
    pub cma1_tx: u16,
    pub c2h: u16,
    pub h2c: u16,
    pub wcpu: u16,
    pub mpdu_proc: u16,
    pub cma0_dma: u16,
    pub cma1_dma: u16,
    pub bb_rpt: u16,
    pub wd_rel: u16,
    pub cpu_io: u16,
    pub tx_rpt: u16,
// for WiFi 7 chips below (suffix v1)
    pub h2d: u16,
// for WiFi 7 chips after 8922D (suffix v2)
    pub snrpt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rsvd_quota {
    pub mpdu_info_tbl: u16,
    pub b0_csi: u16,
    pub b1_csi: u16,
    pub b0_lmr: u16,
    pub b1_lmr: u16,
    pub b0_ftm: u16,
    pub b1_ftm: u16,
    pub b0_smr: u16,
    pub b1_smr: u16,
    pub others: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dle_rsvd_size {
    pub srt_ofst: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dle_input {
    pub tx_ampdu_num_b0: u32,
    pub tx_ampdu_num_b1: u32,
    pub /: *mut *mut u32 tx_amsdu_size; / unit: KB,
    pub h2c_max_size: u32,
    pub /: *mut *mut u32 rx_amsdu_size; / unit: KB,
    pub c2h_max_size: u32,
    pub rls_rpt_max_size: u32,
    pub mpdu_info_tbl_b0: u32,
    pub mpdu_info_tbl_b1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dle_mem {
    pub mode: rtw89_qta_mode,
    pub wde_size: *const rtw89_dle_size,
    pub ple_size: *const rtw89_dle_size,
    pub wde_min_qt: *const rtw89_wde_quota,
    pub wde_max_qt: *const rtw89_wde_quota,
    pub ple_min_qt: *const rtw89_ple_quota,
    pub ple_max_qt: *const rtw89_ple_quota,
// for WiFi 7 chips below
    pub rsvd_qt: *const rtw89_rsvd_quota,
    pub rsvd0_size: *const rtw89_dle_rsvd_size,
    pub rsvd1_size: *const rtw89_dle_rsvd_size,
// for WiFi 7 chips after 8922D
    pub dle_input: *const rtw89_dle_input,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_reg_def {
    pub addr: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_reg2_def {
    pub addr: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_reg3_def {
    pub addr: u32,
    pub mask: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_reg5_def {
    pub /: *mut *mut u8 flag; / recognized by parsers,
    pub path: u8,
    pub addr: u32,
    pub mask: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_regs_def {
    pub regs: *const u32,
    pub reg_nr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_reg_imr {
    pub addr: u32,
    pub clr: u32,
    pub set: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_def {
    pub fw_basename: *const c_char,
    pub fw_format_max: u8,
    pub fw_b_aid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_qta_def {
    pub hfc_param_ini: [*const rtw89_hfc_param_ini; RTW89_HCI_DLE_TYPE_NUM],
    pub dle_mem: [*const rtw89_dle_mem; RTW89_HCI_DLE_TYPE_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_table {
    pub regs: *const rtw89_reg2_def,
    pub n_regs: u32,
    pub rf_path: rtw89_rf_path,
    pub data): *mut rtw89_rf_path rf_path, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_table {
    pub data: *const c_void,
    pub size: u32,
    pub tbl): *const rtw89_txpwr_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_rule_2ghz {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_rule_5ghz {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_rule_6ghz {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tx_shape {
    pub (*lmt_ru)[RTW89_BAND_NUM][RTW89_REGD_NUM][NUM_OF_RTW89_REG_6GHZ_POWER]: *const u8,
    pub (*lmt_v0)[RTW89_BAND_NUM][RTW89_RS_TX_SHAPE_NUM][RTW89_REGD_NUM]: *const u8,
    pub (*lmt_ru_v0)[RTW89_BAND_NUM][RTW89_REGD_NUM]: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfe_parms {
    pub byr_tbl: *const rtw89_txpwr_table,
    pub rule_2ghz: rtw89_txpwr_rule_2ghz,
    pub rule_5ghz: rtw89_txpwr_rule_5ghz,
    pub rule_6ghz: rtw89_txpwr_rule_6ghz,
    pub rule_da_2ghz: rtw89_txpwr_rule_2ghz,
    pub rule_da_5ghz: rtw89_txpwr_rule_5ghz,
    pub rule_da_6ghz: rtw89_txpwr_rule_6ghz,
    pub tx_shape: rtw89_tx_shape,
    pub has_da: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfe_parms_conf {
    pub rfe_parms: *const rtw89_rfe_parms,
    pub rfe_type: u8,
}

pub const RTW89_TXPWR_CONF_DFLT_RFE_TYPE: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_conf {
    pub rfe_type: u8,
    pub ent_sz: u8,
    pub num_ents: u32,
    pub data: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_byrate_data {
    pub conf: rtw89_txpwr_conf,
    pub tbl: rtw89_txpwr_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_lmt_2ghz_data {
    pub conf: rtw89_txpwr_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_lmt_5ghz_data {
    pub conf: rtw89_txpwr_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_lmt_6ghz_data {
    pub conf: rtw89_txpwr_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_lmt_ru_2ghz_data {
    pub conf: rtw89_txpwr_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_lmt_ru_5ghz_data {
    pub conf: rtw89_txpwr_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_lmt_ru_6ghz_data {
    pub conf: rtw89_txpwr_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tx_shape_lmt_data {
    pub conf: rtw89_txpwr_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tx_shape_lmt_ru_data {
    pub conf: rtw89_txpwr_conf,
    pub v: [u8; RTW89_BAND_NUM][RTW89_REGD_NUM][NUM_OF_RTW89_REG_6GHZ_POWER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfe_data {
    pub byrate: rtw89_txpwr_byrate_data,
    pub lmt_2ghz: rtw89_txpwr_lmt_2ghz_data,
    pub lmt_5ghz: rtw89_txpwr_lmt_5ghz_data,
    pub lmt_6ghz: rtw89_txpwr_lmt_6ghz_data,
    pub da_lmt_2ghz: rtw89_txpwr_lmt_2ghz_data,
    pub da_lmt_5ghz: rtw89_txpwr_lmt_5ghz_data,
    pub da_lmt_6ghz: rtw89_txpwr_lmt_6ghz_data,
    pub lmt_ru_2ghz: rtw89_txpwr_lmt_ru_2ghz_data,
    pub lmt_ru_5ghz: rtw89_txpwr_lmt_ru_5ghz_data,
    pub lmt_ru_6ghz: rtw89_txpwr_lmt_ru_6ghz_data,
    pub da_lmt_ru_2ghz: rtw89_txpwr_lmt_ru_2ghz_data,
    pub da_lmt_ru_5ghz: rtw89_txpwr_lmt_ru_5ghz_data,
    pub da_lmt_ru_6ghz: rtw89_txpwr_lmt_ru_6ghz_data,
    pub tx_shape_lmt: rtw89_tx_shape_lmt_data,
    pub tx_shape_lmt_ru: rtw89_tx_shape_lmt_ru_data,
    pub rfe_parms: rtw89_rfe_parms,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_page_regs {
    pub hci_fc_ctrl: u32,
    pub ch_page_ctrl: u32,
    pub ach_page_ctrl: u32,
    pub ach_page_info: u32,
    pub pub_page_info3: u32,
    pub pub_page_ctrl1: u32,
    pub pub_page_ctrl2: u32,
    pub pub_page_info1: u32,
    pub pub_page_info2: u32,
    pub wp_page_ctrl1: u32,
    pub wp_page_ctrl2: u32,
    pub wp_page_info1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_imr_info {
    pub wdrls_imr_set: u32,
    pub wsec_imr_reg: u32,
    pub wsec_imr_set: u32,
    pub mpdu_tx_imr_set: u32,
    pub mpdu_rx_imr_set: u32,
    pub sta_sch_imr_set: u32,
    pub txpktctl_imr_b0_reg: u32,
    pub txpktctl_imr_b0_clr: u32,
    pub txpktctl_imr_b0_set: u32,
    pub txpktctl_imr_b1_reg: u32,
    pub txpktctl_imr_b1_clr: u32,
    pub txpktctl_imr_b1_set: u32,
    pub wde_imr_clr: u32,
    pub wde_imr_set: u32,
    pub ple_imr_clr: u32,
    pub ple_imr_set: u32,
    pub host_disp_imr_clr: u32,
    pub host_disp_imr_set: u32,
    pub cpu_disp_imr_clr: u32,
    pub cpu_disp_imr_set: u32,
    pub other_disp_imr_clr: u32,
    pub other_disp_imr_set: u32,
    pub bbrpt_com_err_imr_reg: u32,
    pub bbrpt_chinfo_err_imr_reg: u32,
    pub bbrpt_err_imr_set: u32,
    pub bbrpt_dfs_err_imr_reg: u32,
    pub ptcl_imr_clr: u32,
    pub ptcl_imr_set: u32,
    pub cdma_imr_0_reg: u32,
    pub cdma_imr_0_clr: u32,
    pub cdma_imr_0_set: u32,
    pub cdma_imr_1_reg: u32,
    pub cdma_imr_1_clr: u32,
    pub cdma_imr_1_set: u32,
    pub phy_intf_imr_reg: u32,
    pub phy_intf_imr_clr: u32,
    pub phy_intf_imr_set: u32,
    pub rmac_imr_reg: u32,
    pub rmac_imr_clr: u32,
    pub rmac_imr_set: u32,
    pub tmac_imr_reg: u32,
    pub tmac_imr_clr: u32,
    pub tmac_imr_set: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_imr_table {
    pub regs: *const rtw89_reg_imr,
    pub n_regs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_xtal_info {
    pub xcap_reg: u32,
    pub sc_xo_mask: u32,
    pub sc_xi_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rrsr_cfgs {
    pub ref_rate: rtw89_reg3_def,
    pub rsc: rtw89_reg3_def,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfkill_regs {
    pub pinmux: rtw89_reg3_def,
    pub mode: rtw89_reg3_def,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sb_regs {
    pub cfg: u32,
    pub get: u32,
    pub n: [}; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dig_regs {
    pub seg0_pd_reg: u32,
    pub pd_lower_bound_mask: u32,
    pub pd_spatial_reuse_en: u32,
    pub bmode_pd_reg: u32,
    pub bmode_cca_rssi_limit_en: u32,
    pub bmode_pd_lower_bound_reg: u32,
    pub bmode_rssi_nocca_low_th_mask: u32,
    pub p0_lna_init: rtw89_reg_def,
    pub p1_lna_init: rtw89_reg_def,
    pub p0_tia_init: rtw89_reg_def,
    pub p1_tia_init: rtw89_reg_def,
    pub p0_rxb_init: rtw89_reg_def,
    pub p1_rxb_init: rtw89_reg_def,
    pub p0_p20_pagcugc_en: rtw89_reg_def,
    pub p0_s20_pagcugc_en: rtw89_reg_def,
    pub p1_p20_pagcugc_en: rtw89_reg_def,
    pub p1_s20_pagcugc_en: rtw89_reg_def,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_edcca_regs {
    pub edcca_level: u32,
    pub edcca_mask: u32,
    pub edcca_p_mask: u32,
    pub ppdu_level: u32,
    pub ppdu_mask: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_edcca_p_regs {
    pub rpt_a: u32,
    pub rpt_b: u32,
    pub rpt_sel: u32,
    pub rpt_sel_mask: u32,
    pub p: [}; RTW89_PHY_NUM],
    pub rpt_sel_be: u32,
    pub rpt_sel_be_mask: u32,
    pub tx_collision_t2r_st: u32,
    pub tx_collision_t2r_st_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pmac_regs {
    pub cck_txon: rtw89_reg_def,
    pub cck_txen: rtw89_reg_def,
    pub cck_cca: rtw89_reg_def,
    pub cck_sfd_gg: rtw89_reg_def,
    pub cck_sig_gg: rtw89_reg_def,
    pub cck_spoofing: rtw89_reg_def,
    pub cck_brk: rtw89_reg_def,
    pub brk: rtw89_reg_def,
    pub brk_option: rtw89_reg_def,
    pub search_fail: rtw89_reg_def,
    pub lsig_brk_s_th: rtw89_reg_def,
    pub lsig_brk_l_th: rtw89_reg_def,
    pub rxl_err_parity: rtw89_reg_def,
    pub rxl_err_rate: rtw89_reg_def,
    pub ofdm_cca: rtw89_reg_def,
    pub cca_spoofing: rtw89_reg_def,
    pub ampdu_miss: rtw89_reg_def,
    pub r1b_rx_rpt_rst: rtw89_reg_def,
    pub r1b_rr_sel: rtw89_reg_def,
    pub enable_all_cnt: rtw89_reg_def,
    pub rst_all_cnt: rtw89_reg_def,
    pub cck_crc32: u32,
    pub cck_crc32_ok_mask: u32,
    pub cck_crc32_fail_mask: u32,
    pub ofdm_txon: u32,
    pub ofdm_txon_mask: u32,
    pub ofdm_txen_mask: u32,
    pub l_crc: u32,
    pub l_crc_ok_mask: u32,
    pub l_crc_err_mask: u32,
    pub ht_crc: u32,
    pub ht_crc_ok_mask: u32,
    pub ht_crc_err_mask: u32,
    pub vht_crc: u32,
    pub vht_crc_ok_mask: u32,
    pub vht_crc_err_mask: u32,
    pub he_crc: u32,
    pub he_crc_ok_mask: u32,
    pub he_crc_err_mask: u32,
    pub eht_crc: u32,
    pub eht_crc_ok_mask: u32,
    pub eht_crc_err_mask: u32,
    pub ampdu_crc: u32,
    pub ampdu_crc_ok_mask: u32,
    pub ampdu_crc_err_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_ul_tb_info {
    pub dyn_tb_tri_en: bool,
    pub def_if_bandedge: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_antdiv_stats {
    pub cck_rssi_avg: ewma_rssi,
    pub ofdm_rssi_avg: ewma_rssi,
    pub non_legacy_rssi_avg: ewma_rssi,
    pub pkt_cnt_cck: u16,
    pub pkt_cnt_ofdm: u16,
    pub pkt_cnt_non_legacy: u16,
    pub evm: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_antdiv_info {
    pub target_stats: rtw89_antdiv_stats,
    pub main_stats: rtw89_antdiv_stats,
    pub aux_stats: rtw89_antdiv_stats,
    pub training_count: u8,
    pub rssi_pre: u8,
    pub get_stats: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_bb_stat_cfg {
    pub enable: bool,
    pub mac_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_info {
    pub bb_wrap_data: *const rtw89_bb_wrap_data,
    pub bb_stat_cfg: rtw89_bb_stat_cfg,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_chanctx_state {
    RTW89_CHANCTX_STATE_MCC_START,
    RTW89_CHANCTX_STATE_MCC_STOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_chanctx_callbacks {
    RTW89_CHANCTX_CALLBACK_PLACEHOLDER,
    RTW89_CHANCTX_CALLBACK_RFK,
    RTW89_CHANCTX_CALLBACK_TAS,

    NUM_OF_RTW89_CHANCTX_CALLBACKS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chanctx_listener {
    pub state): *mut *mut (struct rtw89_dev rtwdev, enum rtw89_chanctx_state,
}

pub const RTW89_NHM_TH_NUM: c_int = 11;
pub const RTW89_NHM_RPT_NUM: c_int = 12;
pub const RTW89_LED_MAX_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_led_gpio_entry {
    pub pin: u8,
    pub color: c_uint,
    pub intensity: u8,
    pub pinmux: rtw89_reg3_def,
    pub mode: rtw89_reg2_def,
    pub out: rtw89_reg2_def,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_led_desc {
    pub gpios: *const rtw89_led_gpio_entry,
    pub n_gpio: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_led {
    pub registered: bool,
    pub desc: *const rtw89_led_desc,
    pub led: led_classdev,
    pub led_mc: led_classdev_mc,
    pub subled: [mc_subled; RTW89_LED_MAX_NUM],
    pub brightness_cache: [led_brightness; RTW89_LED_MAX_NUM],
    pub name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chip_info {
    pub chip_id: rtw89_core_chip_id,
    pub chip_gen: rtw89_chip_gen,
    pub ops: *const rtw89_chip_ops,
    pub mac_def: *const rtw89_mac_gen_def,
    pub phy_def: *const rtw89_phy_gen_def,
    pub fw_def: rtw89_fw_def,
    pub try_ce_fw: bool,
    pub bbmcu_nr: u8,
    pub needed_fw_elms: u32,
    pub fw_blacklist: *const rtw89_fw_blacklist,
    pub fifo_size: u32,
    pub small_fifo_size: bool,
    pub dle_scc_rsvd_size: u32,
    pub max_amsdu_limit: u16,
    pub max_vht_mpdu_cap: u16,
    pub max_eht_mpdu_cap: u16,
    pub max_tx_agg_num: u16,
    pub max_rx_agg_num: u16,
    pub dis_2g_40m_ul_ofdma: bool,
    pub rsvd_ple_ofst: u32,
    pub qta_def: rtw89_qta_def,
    pub wde_qempty_acq_grpnum: u8,
    pub wde_qempty_mgq_grpsel: u8,
    pub rf_base_addr: [u32; 2],
    pub thermal_th: [u8; 2],
    pub support_macid_num: u8,
    pub support_link_num: u8,
    pub support_chanctx_num: u8,
    pub support_bands: u8,
    pub support_bandwidths: u16,
    pub support_unii4: bool,
    pub support_rnr: bool,
    pub support_ant_gain: bool,
    pub support_tas: bool,
    pub support_sar_by_ant: bool,
    pub support_noise: bool,
    pub support_fw_cmd_ofld: bool,
    pub ul_tb_waveform_ctrl: bool,
    pub ul_tb_pwr_diff: bool,
    pub rx_freq_from_ie: bool,
    pub hw_sec_hdr: bool,
    pub hw_mgmt_tx_encrypt: bool,
    pub hw_tkip_crypto: bool,
    pub hw_mlo_bmc_crypto: bool,
    pub rf_path_num: u8,
    pub tx_nss: u8,
    pub rx_nss: u8,
    pub acam_num: u8,
    pub bcam_num: u8,
    pub scam_num: u8,
    pub bacam_num: u8,
    pub bacam_dynamic_num: u8,
    pub bacam_ver: rtw89_bacam_ver,
    pub addrcam_ver: u8,
    pub ppdu_max_usr: u8,
    pub sec_ctrl_efuse_size: u8,
    pub physical_efuse_size: u32,
    pub logical_efuse_size: u32,
    pub limit_efuse_size: u32,
    pub dav_phy_efuse_size: u32,
    pub dav_log_efuse_size: u32,
    pub phycap_addr: u32,
    pub phycap_size: u32,
    pub efuse_blocks: *const rtw89_efuse_block_cfg,
    pub pwr_on_seq: *const *const rtw89_pwr_cfg,
    pub pwr_off_seq: *const *const rtw89_pwr_cfg,
    pub bb_table: *const rtw89_phy_table,
    pub bb_gain_table: *const rtw89_phy_table,
    pub rf_table: [*const rtw89_phy_table; RF_PATH_MAX],
    pub nctl_table: *const rtw89_phy_table,
    pub nctl_post_table: *const rtw89_rfk_tbl,
    pub dig_table: *const rtw89_phy_dig_gain_table,
    pub dig_regs: *const rtw89_dig_regs,
    pub tssi_dbw_table: *const rtw89_phy_tssi_dbw_table,
// NULL if no rfe-specific, or a null-terminated array by rfe_parms
    pub rfe_parms_conf: *const rtw89_rfe_parms_conf,
    pub dflt_parms: *const rtw89_rfe_parms,
    pub chanctx_listener: *const rtw89_chanctx_listener,
    pub txpwr_factor_bb: u8,
    pub txpwr_factor_rf: u8,
    pub txpwr_factor_mac: u8,
    pub para_ver: u32,
    pub wlcx_desired: u32,
    pub scbd: u8,
    pub mailbox: u8,
    pub afh_guard_ch: u8,
    pub fdd_iso_freq: u16,
    pub wl_rssi_thres: *const u8,
    pub bt_rssi_thres: *const u8,
    pub rssi_tol: u8,
    pub mon_reg_num: u8,
    pub mon_reg: *const rtw89_btc_fbtc_mreg,
    pub rf_para_ulink_v0: *const rtw89_btc_rf_trx_para_v0,
    pub rf_para_dlink_v0: *const rtw89_btc_rf_trx_para_v0,
    pub rf_para_ulink_num_v0: u8,
    pub rf_para_dlink_num_v0: u8,
    pub rf_para_ulink_v9: *const rtw89_btc_rf_trx_para_v9,
    pub rf_para_dlink_v9: *const rtw89_btc_rf_trx_para_v9,
    pub rf_para_ulink_num_v9: u8,
    pub rf_para_dlink_num_v9: u8,
    pub ps_mode_supported: u8,
    pub low_power_hci_modes: u8,
    pub h2c_cctl_func_id: u32,
    pub hci_func_en_addr: u32,
    pub h2c_desc_size: u32,
    pub txwd_body_size: u32,
    pub txwd_info_size: u32,
    pub h2c_ctrl_reg: u32,
    pub h2c_regs: *const u32,
    pub h2c_counter_reg: rtw89_reg_def,
    pub c2h_ctrl_reg: u32,
    pub c2h_regs: *const u32,
    pub c2h_counter_reg: rtw89_reg_def,
    pub page_regs: *const rtw89_page_regs,
    pub wow_reason_reg: *const u32,
    pub cfo_src_fd: bool,
    pub cfo_hw_comp: bool,
    pub dcfo_comp: *const rtw89_reg_def,
    pub dcfo_comp_sft: u8,
    pub (*nhm_report)[RTW89_NHM_RPT_NUM]: *const rtw89_reg_def,
    pub (*nhm_th)[RTW89_NHM_TH_NUM]: *const rtw89_reg_def,
    pub imr_info: *const rtw89_imr_info,
    pub imr_dmac_table: *const rtw89_imr_table,
    pub imr_cmac_table: *const rtw89_imr_table,
    pub rrsr_cfgs: *const rtw89_rrsr_cfgs,
    pub bss_clr_vld: rtw89_reg_def,
    pub bss_clr_map_reg: u32,
    pub rfkill_init: *const rtw89_rfkill_regs,
    pub rfkill_get: rtw89_reg_def,
    pub btc_sb: rtw89_sb_regs,
    pub dma_ch_mask: u32,
    pub edcca_regs: *const rtw89_edcca_regs,
    pub pmac_regs: *const rtw89_pmac_regs,
    pub wowlan_stub: *const wiphy_wowlan_support,
    pub xtal_info: *const rtw89_xtal_info,
    pub /: *mut *mut unsigned long default_quirks; / bitmap of rtw89_quirks,
    pub txtime_limit_2ghz: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chip_variant {
    pub 1: bool no_mcs_12_13:,
    pub fw_min_ver_code: u32,
    pub fw_def_override: *const rtw89_fw_def,
    pub qta_def_override: *const rtw89_qta_def,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_board_variant {
    pub led_desc: *const rtw89_led_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_bus_info {
    pub pci: *const rtw89_pci_info,
    pub usb: *const rtw89_usb_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_driver_info {
    pub chip: *const rtw89_chip_info,
    pub variant: *const rtw89_chip_variant,
    pub board: *const rtw89_board_variant,
    pub quirks: *const dmi_system_id,
    pub /: *mut *mut unsigned long dev_id_quirks; / bitmap of rtw89_quirks,
    pub bus: rtw89_bus_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_hcifc_mode {
    RTW89_HCIFC_POH = 0,
    RTW89_HCIFC_STF = 1,
    RTW89_HCIFC_SDIO = 2,

// keep last
    RTW89_HCIFC_MODE_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dle_info {
    pub rsvd_qt: *const rtw89_rsvd_quota,
    pub dle_input: *const rtw89_dle_input,
    pub qta_mode: rtw89_qta_mode,
    pub ple_pg_size: u16,
    pub ple_free_pg: u16,
    pub c0_rx_qta: u16,
    pub c1_rx_qta: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_host_rpr_mode {
    RTW89_RPR_MODE_POH = 0,
    RTW89_RPR_MODE_STF
}

pub const RTW89_COMPLETION_BUF_SIZE: c_int = 40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_completion_data {
    pub err: bool,
    pub buf: [u8; RTW89_COMPLETION_BUF_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_wait_response {
    pub rcu_head: rcu_head,
    pub completion: completion,
    pub data: rtw89_completion_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_wait_info {
    pub cond: core::sync::atomic::AtomicI32,
    pub data: rtw89_completion_data,
    pub resp: *mut rtw89_wait_response __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_info {
    pub dle_info: rtw89_dle_info,
    pub hfc_param: rtw89_hfc_param,
    pub qta_mode: rtw89_qta_mode,
    pub rpwm_seq_num: u8,
    pub cpwm_seq_num: u8,
// see RTW89_FW_OFLD_WAIT_COND series for wait condition
    pub fw_ofld_wait: rtw89_wait_info,
// see RTW89_PS_WAIT_COND series for wait condition
    pub ps_wait: rtw89_wait_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fwdl_check_type {
    RTW89_FWDL_CHECK_FREERTOS_DONE,
    RTW89_FWDL_CHECK_WCPU_FWDL_DONE,
    RTW89_FWDL_CHECK_DCPU_FWDL_DONE,
    RTW89_FWDL_CHECK_BB0_FWDL_DONE,
    RTW89_FWDL_CHECK_BB1_FWDL_DONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_type {
    RTW89_FW_NORMAL = 1,
    RTW89_FW_WOWLAN = 3,
    RTW89_FW_NORMAL_CE = 5,
    RTW89_FW_NORMAL_B = 14,
    RTW89_FW_WOWLAN_B = 15,
    RTW89_FW_BBMCU0 = 64,
    RTW89_FW_BBMCU1 = 65,
    RTW89_FW_LOGFMT = 255,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_feature {
    RTW89_FW_FEATURE_OLD_HT_RA_FORMAT,
    RTW89_FW_FEATURE_SCAN_OFFLOAD,
    RTW89_FW_FEATURE_TX_WAKE,
    RTW89_FW_FEATURE_GROUP(CRASH_TRIGGER,
    RTW89_FW_FEATURE_CRASH_TRIGGER_TYPE_0,
    RTW89_FW_FEATURE_CRASH_TRIGGER_TYPE_1,
    ),
    RTW89_FW_FEATURE_NO_PACKET_DROP,
    RTW89_FW_FEATURE_NO_DEEP_PS,
    RTW89_FW_FEATURE_NO_LPS_PG,
    RTW89_FW_FEATURE_BEACON_FILTER,
    RTW89_FW_FEATURE_MACID_PAUSE_SLEEP,
    RTW89_FW_FEATURE_SCAN_OFFLOAD_BE_V0,
    RTW89_FW_FEATURE_SCAN_OFFLOAD_BE_V1,
    RTW89_FW_FEATURE_SCAN_OFFLOAD_BE_V2,
    RTW89_FW_FEATURE_WOW_REASON_V1,
    RTW89_FW_FEATURE_GROUP(WITH_RFK_PRE_NOTIFY,
    RTW89_FW_FEATURE_RFK_PRE_NOTIFY_V0,
    RTW89_FW_FEATURE_RFK_PRE_NOTIFY_V1,
    RTW89_FW_FEATURE_RFK_PRE_NOTIFY_V2,
    RTW89_FW_FEATURE_RFK_PRE_NOTIFY_V3,
    ),
    RTW89_FW_FEATURE_GROUP(WITH_RFK_PRE_NOTIFY_MCC,
    RTW89_FW_FEATURE_RFK_PRE_NOTIFY_MCC_V0,
    RTW89_FW_FEATURE_RFK_PRE_NOTIFY_MCC_V1,
    RTW89_FW_FEATURE_RFK_PRE_NOTIFY_MCC_V2,
    ),
    RTW89_FW_FEATURE_RFK_RXDCK_V0,
    RTW89_FW_FEATURE_RFK_IQK_V0,
    RTW89_FW_FEATURE_RFK_TXIQK_V0,
    RTW89_FW_FEATURE_NO_WOW_CPU_IO_RX,
    RTW89_FW_FEATURE_NOTIFY_AP_INFO,
    RTW89_FW_FEATURE_CH_INFO_BE_V0,
    RTW89_FW_FEATURE_LPS_CH_INFO,
    RTW89_FW_FEATURE_NO_PHYCAP_P1,
    RTW89_FW_FEATURE_NO_POWER_DIFFERENCE,
    RTW89_FW_FEATURE_BEACON_LOSS_COUNT_V1,
    RTW89_FW_FEATURE_SCAN_OFFLOAD_EXTRA_OP,
    RTW89_FW_FEATURE_RFK_NTFY_MCC_V0,
    RTW89_FW_FEATURE_LPS_DACK_BY_C2H_REG,
    RTW89_FW_FEATURE_BEACON_TRACKING,
    RTW89_FW_FEATURE_ADDR_CAM_V0,
    RTW89_FW_FEATURE_SER_L1_BY_EVENT,
    RTW89_FW_FEATURE_SIM_SER_L0L1_BY_HALT_H2C,
    RTW89_FW_FEATURE_LPS_ML_INFO_V1,
    RTW89_FW_FEATURE_SER_POST_RECOVER_DMAC,
    RTW89_FW_FEATURE_TX_HISTORY_V1,

    NUM_OF_RTW89_FW_FEATURES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_suit {
    pub type: rtw89_fw_type,
    pub data: *const u8,
    pub size: u32,
    pub major_ver: u8,
    pub minor_ver: u8,
    pub sub_ver: u8,
    pub sub_idex: u8,
    pub build_year: u16,
    pub build_mon: u16,
    pub build_date: u16,
    pub build_hour: u16,
    pub build_min: u16,
    pub cmd_ver: u8,
    pub hdr_ver: u8,
    pub commitid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_req_info {
    pub firmware: *const firmware,
    pub completion: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_log {
    pub suit: rtw89_fw_suit,
    pub enable: bool,
    pub last_fmt_id: u32,
    pub fmt_count: u32,
    pub fmt_ids: *const __le32,
    pub (*fmts)[]: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_elm_info {
    pub bb_tbl: *mut rtw89_phy_table,
    pub bb_gain: *mut rtw89_phy_table,
    pub rf_radio: [*mut rtw89_phy_table; RF_PATH_MAX],
    pub rf_nctl: *mut rtw89_phy_table,
    pub txpwr_trk: *mut rtw89_fw_txpwr_track_cfg,
    pub rfk_log_fmt: *mut rtw89_phy_rfk_log_fmt,
    pub regd: *const rtw89_regd_data,
    pub afe: *const rtw89_fw_element_hdr,
    pub diag_mac: *const rtw89_fw_element_hdr,
    pub tx_comp: *const rtw89_fw_element_hdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_mss_dev_type {
    RTW89_FW_MSS_DEV_TYPE_FWSEC_DEF = 0xF,
    RTW89_FW_MSS_DEV_TYPE_FWSEC_INV = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_secure {
    pub 1: bool secure_boot:,
    pub 1: bool can_mss_v1:,
    pub 1: bool can_mss_v0:,
    pub sb_sel_mgn: u32,
    pub mss_dev_type: u8,
    pub mss_cust_idx: u8,
    pub mss_key_num: u8,
    pub /: *mut *mut u8 mss_idx; / v0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_info {
    pub req: rtw89_fw_req_info,
    pub fw_format: c_int,
    pub h2c_seq: u8,
    pub rec_seq: u8,
    pub h2c_counter: u8,
    pub c2h_counter: u8,
    pub normal: rtw89_fw_suit,
    pub wowlan: rtw89_fw_suit,
    pub bbmcu0: rtw89_fw_suit,
    pub bbmcu1: rtw89_fw_suit,
    pub log: rtw89_fw_log,
    pub elm_info: rtw89_fw_elm_info,
    pub sec: rtw89_fw_secure,
    pub NUM_OF_RTW89_FW_FEATURES): DECLARE_BITMAP(feature_map,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_cam_info {
    pub RTW89_MAX_ADDR_CAM_NUM): DECLARE_BITMAP(addr_cam_map,,
    pub RTW89_MAX_BSSID_CAM_NUM): DECLARE_BITMAP(bssid_cam_map,,
    pub RTW89_MAX_SEC_CAM_NUM): DECLARE_BITMAP(sec_cam_map,,
    pub RTW89_MAX_BA_CAM_NUM): DECLARE_BITMAP(ba_cam_map,,
    pub ba_cam_entry: [rtw89_ba_cam_entry; RTW89_MAX_BA_CAM_NUM],
    pub sec_entries: [*const rtw89_sec_cam_entry; RTW89_MAX_SEC_CAM_NUM],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_sar_sources {
    RTW89_SAR_SOURCE_NONE,
    RTW89_SAR_SOURCE_COMMON,
    RTW89_SAR_SOURCE_ACPI,

    RTW89_SAR_SOURCE_NR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_sar_subband {
    RTW89_SAR_2GHZ_SUBBAND,
    RTW89_SAR_5GHZ_SUBBAND_1_2, /* U-NII-1 and U-NII-2 */
    RTW89_SAR_5GHZ_SUBBAND_2_E, /* U-NII-2-Extended */
    RTW89_SAR_5GHZ_SUBBAND_3_4, /* U-NII-3 and U-NII-4 */
    RTW89_SAR_6GHZ_SUBBAND_5_L, /* U-NII-5 lower part */
    RTW89_SAR_6GHZ_SUBBAND_5_H, /* U-NII-5 higher part */
    RTW89_SAR_6GHZ_SUBBAND_6,   /* U-NII-6 */
    RTW89_SAR_6GHZ_SUBBAND_7_L, /* U-NII-7 lower part */
    RTW89_SAR_6GHZ_SUBBAND_7_H, /* U-NII-7 higher part */
    RTW89_SAR_6GHZ_SUBBAND_8,   /* U-NII-8 */

    RTW89_SAR_SUBBAND_NR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sar_cfg_common {
    pub set: [bool; RTW89_SAR_SUBBAND_NR],
    pub cfg: [i32; RTW89_SAR_SUBBAND_NR],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_acpi_sar_subband {
    RTW89_ACPI_SAR_2GHZ_SUBBAND,
    RTW89_ACPI_SAR_5GHZ_SUBBAND_1,   /* U-NII-1 */
    RTW89_ACPI_SAR_5GHZ_SUBBAND_2,   /* U-NII-2 */
    RTW89_ACPI_SAR_5GHZ_SUBBAND_2E,  /* U-NII-2-Extended */
    RTW89_ACPI_SAR_5GHZ_SUBBAND_3_4, /* U-NII-3 and U-NII-4 */
    RTW89_ACPI_SAR_6GHZ_SUBBAND_5_L, /* U-NII-5 lower part */
    RTW89_ACPI_SAR_6GHZ_SUBBAND_5_H, /* U-NII-5 higher part */
    RTW89_ACPI_SAR_6GHZ_SUBBAND_6,   /* U-NII-6 */
    RTW89_ACPI_SAR_6GHZ_SUBBAND_7_L, /* U-NII-7 lower part */
    RTW89_ACPI_SAR_6GHZ_SUBBAND_7_H, /* U-NII-7 higher part */
    RTW89_ACPI_SAR_6GHZ_SUBBAND_8,   /* U-NII-8 */

    NUM_OF_RTW89_ACPI_SAR_SUBBAND,
    RTW89_ACPI_SAR_SUBBAND_NR_LEGACY = RTW89_ACPI_SAR_5GHZ_SUBBAND_3_4 + 1,
    RTW89_ACPI_SAR_SUBBAND_NR_HAS_6GHZ = RTW89_ACPI_SAR_6GHZ_SUBBAND_8 + 1,
}

pub const MAX_NUM_OF_RTW89_ACPI_SAR_TBL: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sar_entry_from_acpi {
    pub v: [i16; NUM_OF_RTW89_ACPI_SAR_SUBBAND][NUM_OF_RTW89_ACPI_SAR_RF_PATH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sar_table_from_acpi {
// If this table is active, must fill all fields according to either
// configuration in BIOS or some default values for SAR to work well.
//
    pub entries: [rtw89_sar_entry_from_acpi; RTW89_REGD_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sar_indicator_from_acpi {
    pub enable_sync: bool,
    pub fields: c_uint,
    pub rfpath): *mut *mut u8 (rfpath_to_antidx)(enum rtw89_rf_path,
// Select among @tables of container, rtw89_sar_cfg_acpi, by path.
// Not design with pointers since addresses will be invalid after
// sync content with local container instance.
//
    pub tblsel: [u8; NUM_OF_RTW89_ACPI_SAR_RF_PATH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sar_cfg_acpi {
    pub downgrade_2tx: u8,
    pub valid_num: c_uint,
    pub tables: [rtw89_sar_table_from_acpi; MAX_NUM_OF_RTW89_ACPI_SAR_TBL],
    pub indicator: rtw89_sar_indicator_from_acpi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sar_info {
// used to decide how to access SAR cfg union
    pub src: rtw89_sar_sources,
// reserved for different knids of SAR cfg struct.
// supposed that a single cfg struct cannot handle various SAR sources.
//
    pub cfg_common: rtw89_sar_cfg_common,
    pub cfg_acpi: rtw89_sar_cfg_acpi,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ant_gain_subband {
    RTW89_ANT_GAIN_2GHZ_SUBBAND,
    RTW89_ANT_GAIN_5GHZ_SUBBAND_1,   /* U-NII-1 */
    RTW89_ANT_GAIN_5GHZ_SUBBAND_2,   /* U-NII-2 */
    RTW89_ANT_GAIN_5GHZ_SUBBAND_2E,  /* U-NII-2-Extended */
    RTW89_ANT_GAIN_5GHZ_SUBBAND_3_4, /* U-NII-3 and U-NII-4 */
    RTW89_ANT_GAIN_6GHZ_SUBBAND_5_L, /* U-NII-5 lower part */
    RTW89_ANT_GAIN_6GHZ_SUBBAND_5_H, /* U-NII-5 higher part */
    RTW89_ANT_GAIN_6GHZ_SUBBAND_6,   /* U-NII-6 */
    RTW89_ANT_GAIN_6GHZ_SUBBAND_7_L, /* U-NII-7 lower part */
    RTW89_ANT_GAIN_6GHZ_SUBBAND_7_H, /* U-NII-7 higher part */
    RTW89_ANT_GAIN_6GHZ_SUBBAND_8,   /* U-NII-8 */

    RTW89_ANT_GAIN_SUBBAND_NR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ant_gain_domain_type {
    RTW89_ANT_GAIN_ETSI = 0,

    RTW89_ANT_GAIN_DOMAIN_NUM,
}

pub const RTW89_ANT_GAIN_CHAIN_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ant_gain_info {
    pub offset: [i8; RTW89_ANT_GAIN_CHAIN_NUM][RTW89_ANT_GAIN_SUBBAND_NR],
    pub regd_enabled: u32,
    pub block_country: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_6ghz_span {
    pub sar_subband_low: rtw89_sar_subband,
    pub sar_subband_high: rtw89_sar_subband,
    pub acpi_sar_subband_low: rtw89_acpi_sar_subband,
    pub acpi_sar_subband_high: rtw89_acpi_sar_subband,
    pub ant_gain_subband_low: rtw89_ant_gain_subband,
    pub ant_gain_subband_high: rtw89_ant_gain_subband,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tas_state {
    RTW89_TAS_STATE_DPR_OFF,
    RTW89_TAS_STATE_DPR_ON,
    RTW89_TAS_STATE_STATIC_SAR,
}

pub const RTW89_TAS_TX_RATIO_WINDOW: c_int = 6;
pub const RTW89_TAS_TXPWR_WINDOW: c_int = 180;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tas_info {
    pub tx_ratio_history: [u16; RTW89_TAS_TX_RATIO_WINDOW],
    pub txpwr_history: [u64; RTW89_TAS_TXPWR_WINDOW],
    pub enabled_countries: u8,
    pub txpwr_head_idx: u8,
    pub txpwr_tail_idx: u8,
    pub tx_ratio_idx: u8,
    pub total_tx_ratio: u16,
    pub total_txpwr: u64,
    pub instant_txpwr: u64,
    pub window_size: u32,
    pub dpr_on_threshold: i8,
    pub dpr_off_threshold: i8,
    pub backup_state: rtw89_tas_state,
    pub state: rtw89_tas_state,
    pub keep_history: bool,
    pub block_regd: bool,
    pub enable: bool,
    pub pause: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chanctx_cfg {
    pub idx: rtw89_chanctx_idx,
    pub ref_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_chanctx_changes {
    RTW89_CHANCTX_REMOTE_STA_CHANGE,
    RTW89_CHANCTX_BCN_OFFSET_CHANGE,
    RTW89_CHANCTX_P2P_PS_CHANGE,
    RTW89_CHANCTX_BT_SLOT_CHANGE,
    RTW89_CHANCTX_TSF32_TOGGLE_CHANGE,

    NUM_OF_RTW89_CHANCTX_CHANGES,
    RTW89_CHANCTX_CHANGE_DFLT = NUM_OF_RTW89_CHANCTX_CHANGES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_entity_mode {
    RTW89_ENTITY_MODE_SCC_OR_SMLD,
    RTW89_ENTITY_MODE_MCC_PREPARE,
    RTW89_ENTITY_MODE_MCC,

    NUM_OF_RTW89_ENTITY_MODE,
    RTW89_ENTITY_MODE_INVALID = -EINVAL,
    RTW89_ENTITY_MODE_UNHANDLED = -ESRCH,
}

pub const RTW89_MAX_INTERFACE_NUM: c_int = 2;
// only valid when running with chanctx_ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_entity_mgnt {
    pub active_list: list_head,
    pub active_roles: [*mut rtw89_vif; RTW89_MAX_INTERFACE_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_chanctx {
    pub chandef: cfg80211_chan_def,
    pub chan: rtw89_chan,
    pub rcd: rtw89_chan_rcd,
// only assigned when running with chanctx_ops
    pub cfg: *mut rtw89_chanctx_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_edcca_bak {
    pub a: u8,
    pub p: u8,
    pub ppdu: u8,
    pub th_old: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_dm_type {
    RTW89_DM_DYNAMIC_EDCCA,
    RTW89_DM_THERMAL_PROTECT,
    RTW89_DM_TAS,
    RTW89_DM_MLO,
    RTW89_DM_HW_SCAN,
    RTW89_DM_INACTIVE_PS,
    RTW89_DM_DIG_PD,
    RTW89_DM_VCORE,
}

pub const RTW89_THERMAL_PROT_LV_MAX: c_int = 5;

pub const RTW89_THERMAL_PROT_VLV_MAX: c_int = 6;
pub const RTW89_THERMAL_PROT_VLV_TH_OFFSET: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hal {
    pub rx_fltr: u32,
    pub cv: u8,
    pub /: *mut *mut u8 cid; / enum rtw89_core_chip_cid,
    pub acv: u8,
    pub /: *mut *mut u16 aid; / enum rtw89_core_chip_aid,
    pub antenna_tx: u32,
    pub antenna_rx: u32,
    pub tx_nss: u8,
    pub rx_nss: u8,
    pub tx_path_diversity: bool,
    pub ant_diversity: bool,
    pub ant_diversity_fixed: bool,
    pub support_cckpd: bool,
    pub support_igi: bool,
    pub no_mcs_12_13: bool,
    pub no_eht: bool,
    pub roc_chanctx_idx: core::sync::atomic::AtomicI32,
    pub NUM_OF_RTW89_CHANCTX_CHANGES): DECLARE_BITMAP(changes,,
    pub NUM_OF_RTW89_CHANCTX): DECLARE_BITMAP(entity_map,,
    pub chanctx: [rtw89_chanctx; NUM_OF_RTW89_CHANCTX],
    pub roc_chandef: cfg80211_chan_def,
    pub entity_active: [bool; RTW89_PHY_NUM],
    pub entity_pause: bool,
    pub entity_mode: rtw89_entity_mode,
    pub entity_mgnt: rtw89_entity_mgnt,
    pub entity_force_hw: rtw89_phy_idx,
    pub /: *mut *mut u32 disabled_dm_bitmap; / bitmap of enum rtw89_dm_type,
    pub thermal_prot_th: u8,
    pub /: *mut *mut u8 thermal_prot_lv; / 0 ~ RTW89_THERMAL_PROT_LV_MAX,
    pub thermal_prot_vmax: u8,
    pub thermal_prot_vmin: u8,
    pub /: *mut *mut u8 thermal_prot_vlv; / 0 ~ RTW89_THERMAL_PROT_VLV_MAX (6),
    pub /: *mut *mut u8 fixed_dig_pd_th; / v = (X(dBm) + 102)/2,
    pub /: *mut *mut s8 fixed_dig_cck_pd_th; / dBm,
}

pub const RTW89_MAX_MAC_ID_NUM: c_int = 128;
pub const RTW89_MAX_PKT_OFLD_NUM: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_flags {
    RTW89_FLAG_POWERON,
    RTW89_FLAG_DMAC_FUNC,
    RTW89_FLAG_CMAC0_FUNC,
    RTW89_FLAG_CMAC1_FUNC,
    RTW89_FLAG_CMAC0_PWR,
    RTW89_FLAG_CMAC1_PWR,
    RTW89_FLAG_FW_RDY,
    RTW89_FLAG_RUNNING,
    RTW89_FLAG_PROBE_DONE,
    RTW89_FLAG_BFEE_MON,
    RTW89_FLAG_BFEE_EN,
    RTW89_FLAG_BFEE_TIMER_KEEP,
    RTW89_FLAG_NAPI_RUNNING,
    RTW89_FLAG_LEISURE_PS,
    RTW89_FLAG_LOW_POWER_MODE,
    RTW89_FLAG_INACTIVE_PS,
    RTW89_FLAG_CRASH_SIMULATING,
    RTW89_FLAG_SER_HANDLING,
    RTW89_FLAG_WOWLAN,
    RTW89_FLAG_FORBIDDEN_TRACK_WORK,
    RTW89_FLAG_CHANGING_INTERFACE,
    RTW89_FLAG_HW_RFKILL_STATE,
    RTW89_FLAG_UNPLUGGED,
    RTW89_FLAG_SHUTDOWN,

    NUM_OF_RTW89_FLAGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_quirks {
    RTW89_QUIRK_PCI_BER,
    RTW89_QUIRK_THERMAL_PROT_120C,
    RTW89_QUIRK_THERMAL_PROT_110C,
    RTW89_QUIRK_HW_INFO_SYSFS,
    RTW89_QUIRK_DISABLE_2GHZ,

    NUM_OF_RTW89_QUIRKS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_custid {
    RTW89_CUSTID_NONE = 0,
    RTW89_CUSTID_HP = 1,
    RTW89_CUSTID_ASUS = 2,
    RTW89_CUSTID_ACER = 3,
    RTW89_CUSTID_LENOVO = 4,
    RTW89_CUSTID_NEC = 5,
    RTW89_CUSTID_AMD = 6,
    RTW89_CUSTID_FUJITSU = 7,
    RTW89_CUSTID_DELL = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_pkt_drop_sel {
    RTW89_PKT_DROP_SEL_MACID_BE_ONCE,
    RTW89_PKT_DROP_SEL_MACID_BK_ONCE,
    RTW89_PKT_DROP_SEL_MACID_VI_ONCE,
    RTW89_PKT_DROP_SEL_MACID_VO_ONCE,
    RTW89_PKT_DROP_SEL_MACID_ALL,
    RTW89_PKT_DROP_SEL_MG0_ONCE,
    RTW89_PKT_DROP_SEL_HIQ_ONCE,
    RTW89_PKT_DROP_SEL_HIQ_PORT,
    RTW89_PKT_DROP_SEL_HIQ_MBSSID,
    RTW89_PKT_DROP_SEL_BAND,
    RTW89_PKT_DROP_SEL_BAND_ONCE,
    RTW89_PKT_DROP_SEL_REL_MACID,
    RTW89_PKT_DROP_SEL_REL_HIQ_PORT,
    RTW89_PKT_DROP_SEL_REL_HIQ_MBSSID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pkt_drop_params {
    pub sel: rtw89_pkt_drop_sel,
    pub mac_band: rtw89_mac_idx,
    pub macid: u8,
    pub port: u8,
    pub mbssid: u8,
    pub tf_trs: bool,
    pub macid_band_sel: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pkt_stat {
    pub beacon_nr: u16,
    pub beacon_rate: u8,
    pub beacon_len: u32,
    pub rx_rate_cnt: [u32; RTW89_HW_RATE_NR],
    pub ldpc: u32,
    pub bcc: u32,
    pub stbc: u32,
    pub su_bf: u32,
    pub su_non_bf: u32,
    pub mu: u32,
    pub rx: },
}

pub const RTW89_BCN_TRACK_STAT_NR: c_int = 32;
pub const RTW89_BCN_TRACK_SCALE_FACTOR: c_int = 10;
pub const RTW89_BCN_TRACK_MAX_BIN_NUM: c_int = 6;
pub const RTW89_BCN_TRACK_BIN_WIDTH: c_int = 5;
pub const RTW89_BCN_TRACK_TARGET_BCN: c_int = 80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_beacon_dist {
    pub min: u16,
    pub max: u16,
    pub outlier_count: u16,
    pub lower_bound: u16,
    pub upper_bound: u16,
    pub bins: [u16; RTW89_BCN_TRACK_MAX_BIN_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_beacon_stat {
    pub num: u8,
    pub wp: u8,
    pub tbtt_tu_min: u16,
    pub tbtt_tu_max: u16,
    pub drift: [u16; RTW89_BCN_TRACK_STAT_NR],
    pub tbtt_us: [u32; RTW89_BCN_TRACK_STAT_NR],
    pub tbtt_tu: [u16; RTW89_BCN_TRACK_STAT_NR],
    pub bcn_dist: rtw89_beacon_dist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_path_diff {
    pub avg: ewma_path_diff,
    pub raw: u8,
    pub bf_smo_en: bool,
    pub link_mode: u8,
}

pub const RTW89_TX_RATE_NR: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_stat {
    pub avg_thermal: [ewma_thermal; RF_PATH_MAX],
    pub last_thermal_max: u8,
    pub tx_rate_cnt: [u32; RTW89_TX_RATE_NR],
    pub bcn_stat: rtw89_beacon_stat,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rfk_report_state {
    RTW89_RFK_STATE_START = 0x0,
    RTW89_RFK_STATE_OK = 0x1,
    RTW89_RFK_STATE_FAIL = 0x2,
    RTW89_RFK_STATE_TIMEOUT = 0x3,
    RTW89_RFK_STATE_H2C_CMD_ERR = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rfk_report_types {
    RTW89_RFK_REPORT_PRE_NTFY,
    RTW89_RFK_REPORT_TSSI,
    RTW89_RFK_REPORT_IQK,
    RTW89_RFK_REPORT_DPK,
    RTW89_RFK_REPORT_TXGAPK,
    RTW89_RFK_REPORT_DACK,
    RTW89_RFK_REPORT_RX_DCK,
    RTW89_RFK_REPORT_TX_IQK,
    RTW89_RFK_REPORT_CIM3k,

    NUM_OF_RTW89_RFK_REPORT_TYPES,
}

pub const RTW89_RFK_RECORD_PATH_NR: c_int = 2;
pub const RTW89_RFK_RECORD_HISTORY_NR: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfk_record {
    pub ch: [u32; RTW89_RFK_RECORD_PATH_NR],
    pub cv: [u32; RTW89_RFK_RECORD_PATH_NR],
    pub c5: [u32; RTW89_RFK_RECORD_PATH_NR],
    pub phy_idx: rtw89_phy_idx,
    pub states: [rtw89_rfk_report_state; NUM_OF_RTW89_RFK_REPORT_TYPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfk_wait_info {
    pub completion: completion,
    pub start_time: ktime_t,
    pub state: rtw89_rfk_report_state,
    pub version: u8,
    pub record_idx: c_int,
    pub record_ptr: *mut rtw89_rfk_record,
    pub records: [rtw89_rfk_record; RTW89_RFK_RECORD_HISTORY_NR],
    pub record_tssi_idx: c_int,
    pub tssi_code: [u32; RTW89_RFK_RECORD_HISTORY_NR][RTW89_RFK_RECORD_PATH_NR],
}

pub const RTW89_DACK_PATH_NR: c_int = 2;
pub const RTW89_DACK_IDX_NR: c_int = 2;
pub const RTW89_DACK_MSBK_NR: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dack_info {
    pub dack_done: bool,
    pub msbk_d: [u8; RTW89_DACK_PATH_NR][RTW89_DACK_IDX_NR][RTW89_DACK_MSBK_NR],
    pub dadck_d: [u8; RTW89_DACK_PATH_NR][RTW89_DACK_IDX_NR],
    pub addck_d: [u16; RTW89_DACK_PATH_NR][RTW89_DACK_IDX_NR],
    pub biask_d: [u16; RTW89_DACK_PATH_NR][RTW89_DACK_IDX_NR],
    pub dack_cnt: u32,
    pub addck_timeout: [bool; RTW89_DACK_PATH_NR],
    pub dadck_timeout: [bool; RTW89_DACK_PATH_NR],
    pub msbk_timeout: [bool; RTW89_DACK_PATH_NR],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rfk_chs_nrs {
    __RTW89_RFK_CHS_NR_V0 = 2,
    __RTW89_RFK_CHS_NR_V1 = 3,

    RTW89_RFK_CHS_NR = __RTW89_RFK_CHS_NR_V1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfk_mcc_info_data {
    pub ch: [u8; RTW89_RFK_CHS_NR],
    pub band: [u8; RTW89_RFK_CHS_NR],
    pub bw: [u8; RTW89_RFK_CHS_NR],
    pub rf18: [u32; RTW89_RFK_CHS_NR],
    pub table_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfk_mcc_info {
    pub data: [rtw89_rfk_mcc_info_data; 2],
}

pub const RTW89_IQK_CHS_NR: c_int = 2;
pub const RTW89_IQK_PATH_NR: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_lck_info {
    pub thermal: [u8; RF_PATH_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rx_dck_info {
    pub thermal: [u8; RF_PATH_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_iqk_info {
    pub lok_cor_fail: [bool; RTW89_IQK_CHS_NR][RTW89_IQK_PATH_NR],
    pub lok_fin_fail: [bool; RTW89_IQK_CHS_NR][RTW89_IQK_PATH_NR],
    pub lok_fail: [bool; RTW89_IQK_PATH_NR],
    pub iqk_tx_fail: [bool; RTW89_IQK_CHS_NR][RTW89_IQK_PATH_NR],
    pub iqk_rx_fail: [bool; RTW89_IQK_CHS_NR][RTW89_IQK_PATH_NR],
    pub iqk_fail_cnt: u32,
    pub is_iqk_init: bool,
    pub iqk_channel: [u32; RTW89_IQK_CHS_NR],
    pub iqk_band: [u8; RTW89_IQK_PATH_NR],
    pub iqk_ch: [u8; RTW89_IQK_PATH_NR],
    pub iqk_bw: [u8; RTW89_IQK_PATH_NR],
    pub iqk_times: u8,
    pub version: u8,
    pub nb_txcfir: [u32; RTW89_IQK_PATH_NR],
    pub nb_rxcfir: [u32; RTW89_IQK_PATH_NR],
    pub bp_txkresult: [u32; RTW89_IQK_PATH_NR],
    pub bp_rxkresult: [u32; RTW89_IQK_PATH_NR],
    pub bp_iqkenable: [u32; RTW89_IQK_PATH_NR],
    pub is_wb_txiqk: [bool; RTW89_IQK_PATH_NR],
    pub is_wb_rxiqk: [bool; RTW89_IQK_PATH_NR],
    pub is_nbiqk: bool,
    pub iqk_fft_en: bool,
    pub iqk_xym_en: bool,
    pub iqk_sram_en: bool,
    pub iqk_cfir_en: bool,
    pub syn1to2: u32,
    pub iqk_mcc_ch: [u8; RTW89_IQK_CHS_NR][RTW89_IQK_PATH_NR],
    pub iqk_table_idx: [u8; RTW89_IQK_PATH_NR],
    pub lok_idac: [u32; RTW89_IQK_CHS_NR][RTW89_IQK_PATH_NR],
    pub lok_vbuf: [u32; RTW89_IQK_CHS_NR][RTW89_IQK_PATH_NR],
    pub iqc_bak: [u32; 2],
}

pub const RTW89_DPK_RF_PATH: c_int = 2;
pub const RTW89_DPK_AVG_THERMAL_NUM: c_int = 8;
pub const RTW89_DPK_BKUP_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dpk_bkup_para {
    pub band: rtw89_band,
    pub bw: rtw89_bandwidth,
    pub ch: u8,
    pub path_ok: u8,
    pub mdpd_en: u8,
    pub txagc_dpk: u8,
    pub ther_dpk: u8,
    pub gs: u8,
    pub pwsf: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dpk_info {
    pub is_dpk_enable: bool,
    pub is_dpk_reload_en: bool,
    pub dpk_gs: [u8; RTW89_PHY_NUM],
    pub dc_i: [u16; RTW89_DPK_RF_PATH][RTW89_DPK_BKUP_NUM],
    pub dc_q: [u16; RTW89_DPK_RF_PATH][RTW89_DPK_BKUP_NUM],
    pub corr_val: [u8; RTW89_DPK_RF_PATH][RTW89_DPK_BKUP_NUM],
    pub corr_idx: [u8; RTW89_DPK_RF_PATH][RTW89_DPK_BKUP_NUM],
    pub cur_idx: [u8; RTW89_DPK_RF_PATH],
    pub cur_k_set: u8,
    pub bp: [rtw89_dpk_bkup_para; RTW89_DPK_RF_PATH][RTW89_DPK_BKUP_NUM],
    pub max_dpk_txagc: [u8; RTW89_DPK_RF_PATH],
    pub dpk_order: [u32; RTW89_DPK_RF_PATH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fem_info {
    pub elna_2g: bool,
    pub elna_5g: bool,
    pub epa_2g: bool,
    pub epa_5g: bool,
    pub epa_6g: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_ch_info {
    pub rssi_min: u8,
    pub rssi_min_macid: u16,
    pub pre_rssi_min: u8,
    pub rssi_max: u8,
    pub rssi_max_macid: u16,
    pub rxsc_160: u8,
    pub rxsc_80: u8,
    pub rxsc_40: u8,
    pub rxsc_20: u8,
    pub rxsc_l: u8,
    pub is_noisy: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pmac_stat_info {
    pub cck_phy_txon: u32,
    pub cck_mac_txen: u32,
    pub ofdm_mac_txen: u32,
    pub ofdm_phy_txon: u32,
    pub cnt_ofdm_cca: u32,
    pub cnt_cck_cca: u32,
    pub cnt_cca_all: u32,
    pub cnt_cck_spoofing: u32,
    pub cnt_ofdm_spoofing: u32,
    pub cnt_ampdu_miss: u32,
    pub cnt_ampdu_crc_error: u32,
    pub cnt_ampdu_crc_ok: u32,
    pub cnt_cck_crc32_error: u32,
    pub cnt_cck_crc32_ok: u32,
    pub cnt_ofdm_crc32_error: u32,
    pub cnt_ofdm_crc32_ok: u32,
    pub cnt_ht_crc32_error: u32,
    pub cnt_ht_crc32_ok: u32,
    pub cnt_vht_crc32_error: u32,
    pub cnt_vht_crc32_ok: u32,
    pub cnt_he_crc32_ok: u32,
    pub cnt_he_crc32_error: u32,
    pub cnt_eht_crc32_ok: u32,
    pub cnt_eht_crc32_error: u32,
    pub cnt_crc32_error_all: u32,
    pub cnt_crc32_ok_all: u32,
    pub cnt_sfd_gg: u32,
    pub cnt_sig_gg: u32,
    pub cnt_cck_fail: u32,
    pub cnt_ofdm_fail: u32,
    pub cnt_fail_all: u32,
    pub cnt_lsig_brk_s_th: u32,
    pub cnt_lsig_brk_l_th: u32,
    pub cnt_parity_fail: u32,
    pub cnt_rate_illegal: u32,
    pub cnt_sb_search_fail: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tx_stat_info {
    pub info: [u32; 6],
    pub common_ctrl: [u32; 2],
    pub txpwr: [u32; 2],
    pub type: u8,
    pub subtype: u8,
    pub txcmd: u8,
    pub txsc: u8,
    pub bw: u8,
    pub tmac_txpwr: u16,
    pub tx_path_en: u8,
    pub path_map: u8,
    pub max_mcs: u8,
    pub stbc: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_diag_bb_type {
    RTW89_DIAG_BB_HANG,
    RTW89_DIAG_BB_PD,
    RTW89_DIAG_BB_NO_RX,
    RTW89_DIAG_BB_FA,
    RTW89_DIAG_BB_EDCCA,

    RTW89_DIAG_BB_NR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_diag_bb {
    pub /: *mut *mut u32 diag_bb_bitmap; / bitmap of enum rtw89_diag_bb_type,
    pub diag_bb_cnt: [u32; RTW89_DIAG_BB_NR],
    pub consecutive_no_tx_cnt: u16,
    pub consecutive_no_rx_cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_agc_gaincode_set {
    pub lna_idx: u8,
    pub tia_idx: u8,
    pub rxb_idx: u8,
}

pub const IGI_RSSI_TH_NUM: c_int = 5;
pub const FA_TH_NUM: c_int = 4;
pub const TIA_LNA_OP1DB_NUM: c_int = 8;
pub const LNA_GAIN_NUM: c_int = 7;
pub const TIA_GAIN_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dig_info {
    pub cur_gaincode: rtw89_agc_gaincode_set,
    pub force_gaincode_idx_en: bool,
    pub force_gaincode: rtw89_agc_gaincode_set,
    pub noisy_lv: rtw89_dig_noisy_level,
    pub igi_rssi_th: [u8; IGI_RSSI_TH_NUM],
    pub fa_th: [u16; FA_TH_NUM],
    pub igi_rssi: u8,
    pub igi_fa_rssi: u8,
    pub fa_rssi_ofst: u8,
    pub dyn_igi_max: u8,
    pub dyn_igi_min: u8,
    pub dyn_pd_th_en: bool,
    pub dyn_pd_th_max: u8,
    pub dyn_pd_max_cnt: u8,
    pub pd_low_th_ofst: u8,
    pub ib_pbk: u8,
    pub ib_pkpwr: i8,
    pub lna_gain_a: [i8; LNA_GAIN_NUM],
    pub lna_gain_g: [i8; LNA_GAIN_NUM],
    pub lna_gain: *mut i8,
    pub tia_gain_a: [i8; TIA_GAIN_NUM],
    pub tia_gain_g: [i8; TIA_GAIN_NUM],
    pub tia_gain: *mut i8,
    pub bak_dig: u32,
    pub is_linked_pre: bool,
    pub bypass_dig: bool,
    pub pause_dig: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_multi_cfo_mode {
    RTW89_PKT_BASED_AVG_MODE = 0,
    RTW89_ENTRY_BASED_AVG_MODE = 1,
    RTW89_TP_BASED_AVG_MODE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_cfo_status {
    RTW89_PHY_DCFO_STATE_NORMAL = 0,
    RTW89_PHY_DCFO_STATE_ENHANCE = 1,
    RTW89_PHY_DCFO_STATE_HOLD = 2,
    RTW89_PHY_DCFO_STATE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_cfo_ul_ofdma_acc_mode {
    RTW89_CFO_UL_OFDMA_ACC_DISABLE = 0,
    RTW89_CFO_UL_OFDMA_ACC_ENABLE = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_cfo_tracking_info {
    pub cfo_timer_ms: u16,
    pub cfo_trig_by_timer_en: bool,
    pub phy_cfo_status: rtw89_phy_cfo_status,
    pub cfo_ul_ofdma_acc_mode: rtw89_phy_cfo_ul_ofdma_acc_mode,
    pub phy_cfo_trk_cnt: u8,
    pub is_adjust: bool,
    pub rtw89_multi_cfo_mode: rtw89_multi_cfo_mode,
    pub apply_compensation: bool,
    pub crystal_cap: u8,
    pub crystal_cap_default: u8,
    pub def_x_cap: u8,
    pub x_cap_ofst: i8,
    pub sta_cfo_tolerance: u32,
    pub cfo_tail: [i32; CFO_TRACK_MAX_USER],
    pub cfo_cnt: [u16; CFO_TRACK_MAX_USER],
    pub cfo_avg_pre: i32,
    pub cfo_avg: [i32; CFO_TRACK_MAX_USER],
    pub pre_cfo_avg: [i32; CFO_TRACK_MAX_USER],
    pub dcfo_avg: i32,
    pub dcfo_avg_pre: i32,
    pub packet_count: u32,
    pub packet_count_pre: u32,
    pub residual_cfo_acc: i32,
    pub phy_cfotrk_state: u8,
    pub phy_cfotrk_cnt: u8,
    pub divergence_lock_en: bool,
    pub x_cap_lb: u8,
    pub x_cap_ub: u8,
    pub lock_cnt: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tssi_mode {
    RTW89_TSSI_NORMAL = 0,
    RTW89_TSSI_SCAN = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tssi_alimk_band {
    TSSI_ALIMK_2G = 0,
    TSSI_ALIMK_5GL,
    TSSI_ALIMK_5GM,
    TSSI_ALIMK_5GH,
    TSSI_ALIMK_MAX
}

// 2GL, 2GH, 5GL1, 5GH1, 5GM1, 5GM2, 5GH1, 5GH2
pub const TSSI_TRIM_CH_GROUP_NUM: c_int = 8;
pub const TSSI_TRIM_CH_GROUP_NUM_6G: c_int = 16;
pub const TSSI_CCK_CH_GROUP_NUM: c_int = 6;
pub const TSSI_MCS_2G_CH_GROUP_NUM: c_int = 5;
pub const TSSI_MCS_5G_CH_GROUP_NUM: c_int = 14;
pub const TSSI_MCS_6G_CH_GROUP_NUM: c_int = 32;

pub const TSSI_MAX_CH_NUM: c_int = 67;
pub const TSSI_ALIMK_VALUE_NUM: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tssi_info {
    pub thermal: [u8; RF_PATH_MAX],
    pub tssi_trim: [i8; RF_PATH_MAX][TSSI_TRIM_CH_GROUP_NUM],
    pub tssi_trim_6g: [i8; RF_PATH_MAX][TSSI_TRIM_CH_GROUP_NUM_6G],
    pub tssi_cck: [i8; RF_PATH_MAX][TSSI_CCK_CH_GROUP_NUM],
    pub tssi_mcs: [i8; RF_PATH_MAX][TSSI_MCS_CH_GROUP_NUM],
    pub tssi_6g_mcs: [i8; RF_PATH_MAX][TSSI_MCS_6G_CH_GROUP_NUM],
    pub extra_ofst: [i8; RF_PATH_MAX],
    pub tssi_tracking_check: [bool; RF_PATH_MAX],
    pub default_txagc_offset: [u8; RF_PATH_MAX],
    pub base_thermal: [u32; RF_PATH_MAX],
    pub check_backup_aligmk: [bool; RF_PATH_MAX][TSSI_MAX_CH_NUM],
    pub alignment_backup_by_ch: [u32; RF_PATH_MAX][TSSI_MAX_CH_NUM][TSSI_ALIMK_VALUE_NUM],
    pub alignment_value: [u32; RF_PATH_MAX][TSSI_ALIMK_MAX][TSSI_ALIMK_VALUE_NUM],
    pub alignment_done: [bool; RF_PATH_MAX][TSSI_ALIMK_MAX],
    pub tssi_alimk_time: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_power_trim_info {
    pub pg_thermal_trim: bool,
    pub pg_pa_bias_trim: bool,
    pub pg_vco_trim: bool,
    pub thermal_trim: [u8; RF_PATH_MAX],
    pub pa_bias_trim: [u8; RF_PATH_MAX],
    pub pad_bias_trim: [u8; RF_PATH_MAX],
    pub vco_trim: [u8; RF_PATH_MAX],
    pub thermal_k: i16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_regd_func {
    RTW89_REGD_FUNC_TAS = 0, /* TAS (Time Average SAR) */
    RTW89_REGD_FUNC_DAG = 1, /* DAG (Dynamic Antenna Gain) */

    NUM_OF_RTW89_REGD_FUNC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_regd {
    pub alpha2: [c_char; 3],
    pub txpwr_regd: [u8; RTW89_BAND_NUM],
    pub NUM_OF_RTW89_REGD_FUNC): DECLARE_BITMAP(func_bitmap,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_regd_data {
    pub nr: c_uint,
    pub __counted_by(nr): rtw89_regd map[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_regd_ctrl {
    pub nr: c_uint,
    pub map: *const rtw89_regd,
}

pub const RTW89_5GHZ_UNII4_CHANNEL_NUM: c_int = 3;
pub const RTW89_5GHZ_UNII4_START_INDEX: c_int = 25;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_regulatory_info {
    pub ctrl: rtw89_regd_ctrl,
    pub regd: *const rtw89_regd,
    pub programmed: bool,
    pub reg_6ghz_power: rtw89_reg_6ghz_power,
    pub reg_6ghz_tpe: rtw89_reg_6ghz_tpe,
    pub txpwr_uk_follow_etsi: bool,
    pub RTW89_REGD_MAX_COUNTRY_NUM): DECLARE_BITMAP(block_unii4,,
    pub RTW89_REGD_MAX_COUNTRY_NUM): DECLARE_BITMAP(block_6ghz,,
    pub RTW89_REGD_MAX_COUNTRY_NUM): DECLARE_BITMAP(block_6ghz_sp,,
    pub RTW89_REGD_MAX_COUNTRY_NUM): DECLARE_BITMAP(block_6ghz_vlp,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ifs_clm_application {
    RTW89_IFS_CLM_INIT = 0,
    RTW89_IFS_CLM_BACKGROUND = 1,
    RTW89_IFS_CLM_ACS = 2,
    RTW89_IFS_CLM_DIG = 3,
    RTW89_IFS_CLM_TDMA_DIG = 4,
    RTW89_IFS_CLM_DBG = 5,
    RTW89_IFS_CLM_DBG_MANUAL = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_env_racing_lv {
    RTW89_RAC_RELEASE = 0,
    RTW89_RAC_LV_1 = 1,
    RTW89_RAC_LV_2 = 2,
    RTW89_RAC_LV_3 = 3,
    RTW89_RAC_LV_4 = 4,
    RTW89_RAC_MAX_NUM = 5
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ccx_para_info {
    pub rac_lv: rtw89_env_racing_lv,
    pub mntr_time: u16,
    pub nhm_incld_cca: bool,
    pub nhm_manual_th_ofst: u8,
    pub nhm_manual_th0: u8,
    pub ifs_clm_app: rtw89_ifs_clm_application,
    pub ifs_clm_manual_th_times: u32,
    pub ifs_clm_manual_th0: u32,
    pub fahm_manual_th_ofst: u8,
    pub fahm_manual_th0: u8,
    pub fahm_numer_opt: u8,
    pub fahm_denom_opt: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ccx_edcca_opt_sc_idx {
    RTW89_CCX_EDCCA_SEG0_P0 = 0,
    RTW89_CCX_EDCCA_SEG0_S1 = 1,
    RTW89_CCX_EDCCA_SEG0_S2 = 2,
    RTW89_CCX_EDCCA_SEG0_S3 = 3,
    RTW89_CCX_EDCCA_SEG1_P0 = 4,
    RTW89_CCX_EDCCA_SEG1_S1 = 5,
    RTW89_CCX_EDCCA_SEG1_S2 = 6,
    RTW89_CCX_EDCCA_SEG1_S3 = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ccx_edcca_opt_bw_idx {
    RTW89_CCX_EDCCA_BW20_0 = 0,
    RTW89_CCX_EDCCA_BW20_1 = 1,
    RTW89_CCX_EDCCA_BW20_2 = 2,
    RTW89_CCX_EDCCA_BW20_3 = 3,
    RTW89_CCX_EDCCA_BW20_4 = 4,
    RTW89_CCX_EDCCA_BW20_5 = 5,
    RTW89_CCX_EDCCA_BW20_6 = 6,
    RTW89_CCX_EDCCA_BW20_7 = 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_nhm_report {
    pub list: list_head,
    pub channel: *mut ieee80211_channel,
    pub noise: u8,
}

pub const RTW89_FAHM_TH_NUM: c_int = 11;
pub const RTW89_FAHM_RPT_NUM: c_int = 12;
pub const RTW89_IFS_CLM_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_env_monitor_info {
    pub ccx_watchdog_result: u8,
    pub ccx_ongoing: bool,
    pub ccx_rac_lv: u8,
    pub ccx_manual_ctrl: bool,
    pub ifs_clm_mntr_time: u16,
    pub ifs_clm_app: rtw89_ifs_clm_application,
    pub ccx_period: u16,
    pub ccx_unit_idx: u8,
    pub ifs_clm_th_l: [u16; RTW89_IFS_CLM_NUM],
    pub ifs_clm_th_h: [u16; RTW89_IFS_CLM_NUM],
    pub ifs_clm_tx: u16,
    pub ifs_clm_edcca_excl_cca: u16,
    pub ifs_clm_ofdmfa: u16,
    pub ifs_clm_ofdmcca_excl_fa: u16,
    pub ifs_clm_cckfa: u16,
    pub ifs_clm_cckcca_excl_fa: u16,
    pub ifs_clm_total_ifs: u16,
    pub ifs_clm_his: [u16; RTW89_IFS_CLM_NUM],
    pub ifs_clm_avg: [u16; RTW89_IFS_CLM_NUM],
    pub ifs_clm_cca: [u16; RTW89_IFS_CLM_NUM],
    pub ifs_clm_tx_ratio: u8,
    pub ifs_clm_edcca_excl_cca_ratio: u8,
    pub ifs_clm_cck_fa_ratio: u8,
    pub ifs_clm_ofdm_fa_ratio: u8,
    pub ifs_clm_cck_cca_excl_fa_ratio: u8,
    pub ifs_clm_ofdm_cca_excl_fa_ratio: u8,
    pub ifs_clm_cck_fa_permil: u16,
    pub ifs_clm_ofdm_fa_permil: u16,
    pub ifs_clm_ifs_avg: [u32; RTW89_IFS_CLM_NUM],
    pub ifs_clm_cca_avg: [u32; RTW89_IFS_CLM_NUM],
    pub nhm_include_cca: bool,
    pub nhm_sum: u32,
    pub nhm_mntr_time: u32,
    pub nhm_result: [u16; RTW89_NHM_RPT_NUM],
    pub nhm_th: [u8; RTW89_NHM_RPT_NUM],
    pub nhm_his: [*mut rtw89_nhm_report; RTW89_BAND_NUM],
    pub nhm_rpt_list: list_head,
    pub edcca_clm_ratio: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ser_rcvy_step {
    RTW89_SER_DRV_STOP_TX,
    RTW89_SER_DRV_STOP_RX,
    RTW89_SER_DRV_STOP_RUN,
    RTW89_SER_HAL_STOP_DMA,
    RTW89_SER_SUPPRESS_LOG,
    RTW89_NUM_OF_SER_FLAGS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ser_count {
    pub l1: c_uint,
    pub l2: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ser {
    pub state: u8,
    pub alarm_event: u8,
    pub prehandle_l1: bool,
    pub sw_cnt: rtw89_ser_count,
    pub ser_hdl_work: work_struct,
    pub ser_alarm_work: delayed_work,
    pub st_tbl: *const state_ent,
    pub ev_tbl: *const event_ent,
    pub msg_q: list_head,
    pub /: *mut *mut spinlock_t msg_q_lock; / lock when read/write ser msg,
    pub RTW89_NUM_OF_SER_FLAGS): DECLARE_BITMAP(flags,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_ax_ps_mode {
    RTW89_MAC_AX_PS_MODE_ACTIVE = 0,
    RTW89_MAC_AX_PS_MODE_LEGACY = 1,
    RTW89_MAC_AX_PS_MODE_WMMPS  = 2,
    RTW89_MAC_AX_PS_MODE_MAX    = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_last_rpwm_mode {
    RTW89_LAST_RPWM_PS        = 0x0,
    RTW89_LAST_RPWM_ACTIVE    = 0x6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_lps_parm {
    pub macid: u8,
    pub /: *mut *mut u8 psmode; / enum rtw89_mac_ax_ps_mode,
    pub /: *mut *mut u8 lastrpwm; / enum rtw89_last_rpwm_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ppdu_sts_info {
    pub rx_queue: [sk_buff_head; RTW89_PHY_NUM],
    pub curr_rx_ppdu_cnt: [u8; RTW89_PHY_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_early_h2c {
    pub list: list_head,
    pub h2c: *mut u8,
    pub h2c_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hw_scan_extra_op {
    pub set: bool,
    pub macid: u8,
    pub port: u8,
    pub chan: rtw89_chan,
    pub rtwvif_link: *mut rtw89_vif_link,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_hw_scan_info {
    pub scanning_vif: *mut rtw89_vif_link,
    pub pkt_list: [list_head; NUM_NL80211_BANDS],
    pub chan_list: list_head,
    pub op_chan: rtw89_chan,
    pub extra_op: rtw89_hw_scan_extra_op,
    pub wildcard_pkt_id: [u8; NUM_NL80211_BANDS],
    pub ssid_total_len: u16,
    pub n_ssids: c_int,
    pub connected: bool,
    pub abort: bool,
    pub /: *mut *mut u16 delay; / in unit of ms,
    pub 2: u8 seq:,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_bb_gain_band {
    RTW89_BB_GAIN_BAND_2G = 0,
    RTW89_BB_GAIN_BAND_5G_L = 1,
    RTW89_BB_GAIN_BAND_5G_M = 2,
    RTW89_BB_GAIN_BAND_5G_H = 3,
    RTW89_BB_GAIN_BAND_6G_L = 4,
    RTW89_BB_GAIN_BAND_6G_M = 5,
    RTW89_BB_GAIN_BAND_6G_H = 6,
    RTW89_BB_GAIN_BAND_6G_UH = 7,

    RTW89_BB_GAIN_BAND_NR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_gain_band_be {
    RTW89_BB_GAIN_BAND_2G_BE = 0,
    RTW89_BB_GAIN_BAND_5G_L_BE = 1,
    RTW89_BB_GAIN_BAND_5G_M_BE = 2,
    RTW89_BB_GAIN_BAND_5G_H_BE = 3,
    RTW89_BB_GAIN_BAND_6G_L0_BE = 4,
    RTW89_BB_GAIN_BAND_6G_L1_BE = 5,
    RTW89_BB_GAIN_BAND_6G_M0_BE = 6,
    RTW89_BB_GAIN_BAND_6G_M1_BE = 7,
    RTW89_BB_GAIN_BAND_6G_H0_BE = 8,
    RTW89_BB_GAIN_BAND_6G_H1_BE = 9,
    RTW89_BB_GAIN_BAND_6G_UH0_BE = 10,
    RTW89_BB_GAIN_BAND_6G_UH1_BE = 11,

    RTW89_BB_GAIN_BAND_NR_BE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_bb_bw_be {
    RTW89_BB_BW_20_40 = 0,
    RTW89_BB_BW_80_160_320 = 1,

    RTW89_BB_BW_NR_BE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bw20_sc {
    RTW89_BW20_SC_20M = 1,
    RTW89_BW20_SC_40M = 2,
    RTW89_BW20_SC_80M = 4,
    RTW89_BW20_SC_160M = 8,
    RTW89_BW20_SC_320M = 16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_cmac_table_bw {
    RTW89_CMAC_BW_20M = 0,
    RTW89_CMAC_BW_40M = 1,
    RTW89_CMAC_BW_80M = 2,
    RTW89_CMAC_BW_160M = 3,
    RTW89_CMAC_BW_320M = 4,

    RTW89_CMAC_BW_NR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_bb_rxsc_num {
    RTW89_BB_RXSC_NUM_40 = 9, /* SC: 0, 1~8 */
    RTW89_BB_RXSC_NUM_80 = 13, /* SC: 0, 1~8, 9~12 */
    RTW89_BB_RXSC_NUM_160 = 15, /* SC: 0, 1~8, 9~12, 13~14 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_bb_gain_info {
    pub lna_gain: [i8; RTW89_BB_GAIN_BAND_NR][RF_PATH_MAX][LNA_GAIN_NUM],
    pub tia_gain: [i8; RTW89_BB_GAIN_BAND_NR][RF_PATH_MAX][TIA_GAIN_NUM],
    pub lna_gain_bypass: [i8; RTW89_BB_GAIN_BAND_NR][RF_PATH_MAX][LNA_GAIN_NUM],
    pub lna_op1db: [i8; RTW89_BB_GAIN_BAND_NR][RF_PATH_MAX][LNA_GAIN_NUM],
    pub /: *mut *mut [LNA_GAIN_NUM + 1]; / TIA0_LNA0~6 + TIA1_LNA6,
    pub rpl_ofst_20: [i8; RTW89_BB_GAIN_BAND_NR][RF_PATH_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_bb_gain_info_be {
    pub 1]: [RF_PATH_MAX][LNA_GAIN_NUM +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_efuse_gain {
    pub offset_valid: bool,
    pub comp_valid: bool,
    pub /: *mut *mut s8 offset[RF_PATH_MAX][RTW89_GAIN_OFFSET_NR]; / S(8, 0),
    pub /: *mut *mut s8 offset2[RF_PATH_MAX][RTW89_GAIN_OFFSET_NR]; / S(8, 0),
    pub /: *mut *mut s8 offset_base[RTW89_PHY_NUM]; / S(8, 4),
    pub /: *mut *mut s8 rssi_base[RTW89_PHY_NUM]; / S(8, 4),
    pub /: *mut *mut s8 ref_gain_base[RTW89_PHY_NUM]; / S(8, 2),
    pub /: *mut *mut s8 cck_rpl_base[RTW89_PHY_NUM]; / S(8, 0),
    pub /: *mut *mut s8 comp[RF_PATH_MAX][RTW89_SUBBAND_NR]; / S(8, 0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_calc_efuse_gain {
    pub cck_mean_gain_bias: i8,
    pub cck_rpl_ofst: i8,
    pub rssi_ofst: i8,
}

pub const RTW89_MAX_PATTERN_NUM: c_int = 18;
pub const RTW89_MAX_PATTERN_MASK_SIZE: c_int = 4;
pub const RTW89_MAX_PATTERN_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_wow_cam_info {
    pub r_w: bool,
    pub idx: u8,
    pub mask: [__le32; RTW89_MAX_PATTERN_MASK_SIZE],
    pub crc: u16,
    pub negative_pattern_match: bool,
    pub skip_mac_hdr: bool,
    pub uc: bool,
    pub mc: bool,
    pub bc: bool,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_wow_key_info {
    pub ptk_tx_iv: [u8; 8],
    pub valid_check: u8,
    pub symbol_check_en: u8,
    pub gtk_keyidx: u8,
    pub rsvd: [u8; 5],
    pub ptk_rx_iv: [u8; 8],
    pub gtk_rx_iv: [u8; 4][8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_wow_gtk_info {
    pub kck: [u8; 32],
    pub kek: [u8; 32],
    pub tk1: [u8; 16],
    pub rxmickey: [u8; 8],
    pub txmickey: [u8; 8],
    pub igtk_keyid: __le32,
    pub ipn: __le64,
    pub igtk: [u8; 2][32],
    pub psk: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_wow_aoac_report {
    pub rpt_ver: u8,
    pub sec_type: u8,
    pub key_idx: u8,
    pub pattern_idx: u8,
    pub rekey_ok: u8,
    pub ptk_tx_iv: [u8; 8],
    pub eapol_key_replay_count: [u8; 8],
    pub gtk: [u8; 32],
    pub ptk_rx_iv: [u8; 8],
    pub gtk_rx_iv: [u8; 4][8],
    pub igtk_key_id: u64,
    pub igtk_ipn: u64,
    pub igtk: [u8; 32],
    pub csa_pri_ch: u8,
    pub csa_bw: u8,
    pub csa_ch_offset: u8,
    pub csa_chsw_failed: u8,
    pub csa_ch_band: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_wow_param {
    pub rtwvif_link: *mut rtw89_vif_link,
    pub RTW89_WOW_FLAG_NUM): DECLARE_BITMAP(flags,,
    pub patterns: [rtw89_wow_cam_info; RTW89_MAX_PATTERN_NUM],
    pub key_info: rtw89_wow_key_info,
    pub gtk_info: rtw89_wow_gtk_info,
    pub aoac_rpt: rtw89_wow_aoac_report,
    pub pattern_cnt: u8,
    pub ptk_alg: u8,
    pub gtk_alg: u8,
    pub ptk_keyidx: u8,
    pub akm: u8,
// see RTW89_WOW_WAIT_COND series for wait condition
    pub wait: rtw89_wait_info,
    pub pno_inited: bool,
    pub pno_pkt_list: list_head,
    pub nd_config: *mut cfg80211_sched_scan_request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_limit {
    pub enable: bool,
    pub /: *mut *mut u16 max_tob; / TU; max time offset behind,
    pub /: *mut *mut u16 max_toa; / TU; max time offset ahead,
    pub /: *mut *mut u16 max_dur; / TU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_policy {
    pub c2h_rpt: u8,
    pub tx_null_early: u8,
    pub dis_tx_null: u8,
    pub in_curr_ch: u8,
    pub dis_sw_retry: u8,
    pub sw_retry_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_role {
    pub rtwvif_link: *mut rtw89_vif_link,
    pub policy: rtw89_mcc_policy,
    pub limit: rtw89_mcc_limit,
    pub crtz: *const rtw89_mcc_courtesy_cfg,
// only valid when running with FW MRC mechanism
    pub slot_idx: u8,
// byte-array in LE order for FW
    pub macid_bitmap: [u8; BITS_TO_BYTES(RTW89_MAX_MAC_ID_NUM)],
    pub probe_count: u8,
    pub /: *mut *mut u16 duration; / TU,
    pub /: *mut *mut u16 beacon_interval; / TU,
    pub is_2ghz: bool,
    pub is_go: bool,
    pub is_gc: bool,
    pub ignore_bcn: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_bt_role {
    pub /: *mut *mut u16 duration; / TU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_courtesy_cfg {
    pub slot_num: u8,
    pub macid_tgt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_courtesy {
    pub ref: rtw89_mcc_courtesy_cfg,
    pub aux: rtw89_mcc_courtesy_cfg,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mcc_plan {
    RTW89_MCC_PLAN_TAIL_BT,
    RTW89_MCC_PLAN_MID_BT,
    RTW89_MCC_PLAN_NO_BT,

    NUM_OF_RTW89_MCC_PLAN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_pattern {
    pub /: *mut *mut s16 tob_ref; / TU; time offset behind of reference role,
    pub /: *mut *mut s16 toa_ref; / TU; time offset ahead of reference role,
    pub /: *mut *mut s16 tob_aux; / TU; time offset behind of auxiliary role,
    pub /: *mut *mut s16 toa_aux; / TU; time offset ahead of auxiliary role,
    pub plan: rtw89_mcc_plan,
    pub courtesy: rtw89_mcc_courtesy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_sync {
    pub enable: bool,
    pub /: *mut *mut u16 offset; / TU,
    pub macid_src: u8,
    pub band_src: u8,
    pub port_src: u8,
    pub macid_tgt: u8,
    pub band_tgt: u8,
    pub port_tgt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_config {
    pub pattern: rtw89_mcc_pattern,
    pub sync: rtw89_mcc_sync,
    pub start_tsf: u64,
    pub start_tsf_in_aux_domain: u64,
    pub prepare_delay: u64,
    pub /: *mut *mut u16 mcc_interval; / TU,
    pub /: *mut *mut u16 beacon_offset; / TU,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mcc_mode {
    RTW89_MCC_MODE_GO_STA,
    RTW89_MCC_MODE_GC_STA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mcc_info {
    pub wait: rtw89_wait_info,
    pub group: u8,
    pub mode: rtw89_mcc_mode,
    pub /: *mut *mut rtw89_mcc_role role_ref; / reference role,
    pub /: *mut *mut rtw89_mcc_role role_aux; / auxiliary role,
    pub bt_role: rtw89_mcc_bt_role,
    pub config: rtw89_mcc_config,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mlo_mode {
    RTW89_MLO_MODE_MLSR = 0,
    RTW89_MLO_MODE_EMLSR = 1,

    NUM_OF_RTW89_MLO_MODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mlo_info {
    pub wait: rtw89_wait_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_beacon_track_info {
    pub is_data_ready: bool,
    pub /: *mut *mut u32 tbtt_offset; / in unit of microsecond,
    pub /: *mut *mut u16 bcn_timeout; / in unit of millisecond,
// The following are constant and set at association.
    pub dtim: u8,
    pub beacon_int: u16,
    pub low_bcn_th: u16,
    pub med_bcn_th: u16,
    pub high_bcn_th: u16,
    pub target_bcn_th: u16,
    pub outlier_low_bcn_th: u16,
    pub outlier_high_bcn_th: u16,
    pub close_bcn_intvl_th: u32,
    pub tbtt_diff_th: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_tid_stats {
    pub last_pn: i64,
    pub last_sn: u16,
    pub started: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_io_ops {
    pub rtwdev): *mut *mut int (pack)(struct rtw89_dev,
    pub rtwdev): *mut *mut int (unpack)(struct rtw89_dev,
    pub us): *mut *mut *mut void (do_udelay)(struct rtw89_dev rtwdev, u32,
    pub ms): *mut *mut *mut void (do_mdelay)(struct rtw89_dev rtwdev, u32,
    pub data): *mut *mut *mut void (write8)(struct rtw89_dev rtwdev, u32 addr, u8,
    pub data): *mut *mut *mut void (write16)(struct rtw89_dev rtwdev, u32 addr, u16,
    pub data): *mut *mut *mut void (write32)(struct rtw89_dev rtwdev, u32 addr, u32,
    pub data): *mut *mut *mut void (phy_write8)(struct rtw89_dev rtwdev, u32 addr, u8,
    pub data): *mut *mut *mut void (phy_write16)(struct rtw89_dev rtwdev, u32 addr, u16,
    pub data): *mut *mut *mut void (phy_write32)(struct rtw89_dev rtwdev, u32 addr, u32,
    pub data): u32 addr, u32 mask, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_dev {
    pub hw: *mut ieee80211_hw,
    pub dev: *mut device,
    pub ops: *const ieee80211_ops,
    pub io: *const rtw89_io_ops,
    pub dbcc_en: bool,
    pub support_mlo: bool,
    pub mlo_dbcc_mode: rtw89_mlo_dbcc_mode,
    pub scan_info: rtw89_hw_scan_info,
    pub chip: *const rtw89_chip_info,
    pub variant: *const rtw89_chip_variant,
    pub board: *const rtw89_board_variant,
    pub pci_info: *const rtw89_pci_info,
    pub rfe_parms: *const rtw89_rfe_parms,
    pub hal: rtw89_hal,
    pub bcn_track: rtw89_beacon_track_info,
    pub mcc: rtw89_mcc_info,
    pub mlo: rtw89_mlo_info,
    pub mac: rtw89_mac_info,
    pub fw: rtw89_fw_info,
    pub hci: rtw89_hci_info,
    pub efuse: rtw89_efuse,
    pub stats: rtw89_traffic_stats,
    pub rfe_data: *mut rtw89_rfe_data,
    pub custid: rtw89_custid,
    pub fw_cmd_ofld_info: *mut rtw89_fw_cmd_ofld_info,
    pub assoc_link_on_macid: [*mut rtw89_sta_link __rcu; RTW89_MAX_MAC_ID_NUM],
    pub refcount_ap_info: refcount_t,
    pub rtwvifs_list: list_head,
    pub txq_wq: *mut workqueue_struct,
    pub txq_work: work_struct,
    pub txq_reinvoke_work: delayed_work,
// used to protect ba_list and forbid_ba_list
    pub ba_lock: spinlock_t,
// txqs to setup ba session
    pub ba_list: list_head,
// txqs to forbid ba session
    pub forbid_ba_list: list_head,
    pub ba_work: work_struct,
// used to protect rpwm
    pub rpwm_lock: spinlock_t,
    pub tx_waits: list_head,
    pub tx_wait_work: wiphy_delayed_work,
    pub tx_rpt: rtw89_tx_rpt,
    pub cam_info: rtw89_cam_info,
    pub c2h_queue: sk_buff_head,
    pub c2h_work: wiphy_work,
    pub ips_work: wiphy_work,
    pub cancel_6ghz_probe_work: wiphy_work,
    pub load_firmware_work: work_struct,
    pub early_h2c_list: list_head,
    pub ser: rtw89_ser,
    pub RTW89_PORT_NUM): DECLARE_BITMAP(hw_port,,
    pub RTW89_MAX_MAC_ID_NUM): DECLARE_BITMAP(mac_id_map,,
    pub NUM_OF_RTW89_FLAGS): DECLARE_BITMAP(flags,,
    pub RTW89_MAX_PKT_OFLD_NUM): DECLARE_BITMAP(pkt_offload,,
    pub NUM_OF_RTW89_QUIRKS): DECLARE_BITMAP(quirks,,
    pub phystat: rtw89_phy_stat,
    pub rfk_wait: rtw89_rfk_wait_info,
    pub dack: rtw89_dack_info,
    pub iqk: rtw89_iqk_info,
    pub dpk: rtw89_dpk_info,
    pub rfk_mcc: rtw89_rfk_mcc_info,
    pub lck: rtw89_lck_info,
    pub rx_dck: rtw89_rx_dck_info,
    pub is_tssi_mode: [bool; RF_PATH_MAX],
    pub is_bt_iqk_timeout: bool,
    pub fem: rtw89_fem_info,
    pub byr: [rtw89_txpwr_byrate; RTW89_BAND_NUM][RTW89_BYR_BW_NUM],
    pub tssi: rtw89_tssi_info,
    pub pwr_trim: rtw89_power_trim_info,
    pub cfo_tracking: rtw89_cfo_tracking_info,
    pub ax: rtw89_phy_bb_gain_info,
    pub be: rtw89_phy_bb_gain_info_be,
    pub bb_gain: },
    pub efuse_gain: rtw89_phy_efuse_gain,
    pub ul_tb_info: rtw89_phy_ul_tb_info,
    pub antdiv: rtw89_antdiv_info,
    pub phy_info: rtw89_phy_info,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_bb_ctx {
    pub phy_idx: rtw89_phy_idx,
    pub env_monitor: rtw89_env_monitor_info,
    pub dig: rtw89_dig_info,
    pub ch_info: rtw89_phy_ch_info,
    pub edcca_bak: rtw89_edcca_bak,
    pub bcn_rssi: ewma_rssi,
    pub cur_pkt_stat: rtw89_pkt_stat,
    pub last_pkt_stat: rtw89_pkt_stat,
    pub pmac_stat: rtw89_pmac_stat_info,
    pub tx_stat: rtw89_tx_stat_info,
    pub diag: rtw89_diag_bb,
    pub path_diff: rtw89_phy_path_diff,
    pub bbs: [}; RTW89_PHY_NUM],
    pub track_work: wiphy_delayed_work,
    pub track_ps_work: wiphy_delayed_work,
    pub chanctx_work: wiphy_delayed_work,
    pub coex_act1_work: wiphy_delayed_work,
    pub coex_bt_devinfo_work: wiphy_delayed_work,
    pub coex_rfk_chk_work: wiphy_delayed_work,
    pub cfo_track_work: wiphy_delayed_work,
    pub mcc_prepare_done_work: wiphy_delayed_work,
    pub forbid_ba_work: delayed_work,
    pub antdiv_work: wiphy_delayed_work,
    pub ppdu_sts: rtw89_ppdu_sts_info,
    pub total_sta_assoc: u8,
    pub scanning: bool,
    pub regulatory: rtw89_regulatory_info,
    pub sar: rtw89_sar_info,
    pub tas: rtw89_tas_info,
    pub ant_gain: rtw89_ant_gain_info,
    pub btc: rtw89_btc,
    pub ps_mode: rtw89_ps_mode,
    pub lps_enabled: bool,
    pub ps_hang_cnt: u8,
    pub wow: rtw89_wow_param,
// napi structure
    pub netdev: *mut net_device,
    pub napi: napi_struct,
    pub napi_budget_countdown: c_int,
    pub debugfs: *mut rtw89_debugfs,
    pub pure_monitor_mode_vif: *mut rtw89_vif,
    pub led: rtw89_led,
// HCI related data, keep last
    pub )): *mut u8 priv[] __aligned(sizeof(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_link_conf_container {
    pub link_conf: [*mut ieee80211_bss_conf; IEEE80211_MLD_MAX_NUM_LINKS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_vif_ml_trans {
    pub mediate_links: u16,
    pub links_to_del: u16,
    pub links_to_add: u16,
}

pub const RTW89_VIF_IDLE_LINK_ID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_vif {
    pub rtwdev: *mut rtw89_dev,
    pub list: list_head,
    pub mgnt_entry: list_head,
    pub snap_link_confs: *mut rtw89_link_conf_container __rcu,
    pub mac_addr: [u8; ETH_ALEN],
    pub ip_addr: __be32,
    pub stats: rtw89_traffic_stats,
    pub stats_ps: rtw89_traffic_stats,
    pub tdls_peer: u32,
    pub scan_ies: *mut ieee80211_scan_ies,
    pub scan_req: *mut cfg80211_scan_request,
    pub roc: rtw89_roc,
    pub offchan: bool,
    pub burst_active: bool,
    pub mlo_mode: rtw89_mlo_mode,
    pub ml_trans: rtw89_vif_ml_trans,
    pub dlink_pool: list_head,
    pub links_inst_valid_num: u8,
    pub __RTW89_MLD_MAX_LINK_NUM): DECLARE_BITMAP(links_inst_map,,
    pub links: [*mut rtw89_vif_link; IEEE80211_MLD_MAX_NUM_LINKS],
    pub __counted_by(links_inst_valid_num): rtw89_vif_link links_inst[],
}

// rtwvif_link = rtwvif->links[link_id];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_sta_flags {
    RTW89_REMOTE_STA_IN_PS,

    NUM_OF_RTW89_STA_FLAGS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_sta {
    pub rtwdev: *mut rtw89_dev,
    pub rtwvif: *mut rtw89_vif,
    pub NUM_OF_RTW89_STA_FLAGS): DECLARE_BITMAP(flags,,
    pub disassoc: bool,
    pub roc_queue: sk_buff_head,
    pub ampdu_params: [rtw89_ampdu_params; IEEE80211_NUM_TIDS],
    pub tid_rx_stats: [rtw89_tid_stats; IEEE80211_NUM_TIDS],
    pub IEEE80211_NUM_TIDS): DECLARE_BITMAP(ampdu_map,,
    pub RTW89_MAX_SEC_CAM_NUM): DECLARE_BITMAP(pairwise_sec_cam_map,,
    pub dlink_pool: list_head,
    pub links_inst_valid_num: u8,
    pub __RTW89_MLD_MAX_LINK_NUM): DECLARE_BITMAP(links_inst_map,,
    pub links: [*mut rtw89_sta_link; IEEE80211_MLD_MAX_NUM_LINKS],
    pub __counted_by(links_inst_valid_num): rtw89_sta_link links_inst[],
}

// rtwsta_link = rtwsta->links[link_id];

// const after init, so no need to check if active first
extern "C" {
    pub fn rcu_dereference(_arg: rtwdev->assoc_link_on_macid[macid]) -> return;
}

// hci.ops->reset must complete all pending TX wait SKBs
//
// This should be used by/after rtw89_hci_tx_write() and before doing
// ieee80211_tx_info_clear_status().
//
extern "C" {
    pub fn container_of(_arg: p, ieee80211_txq: struct, _arg: drv_priv) -> return;
}
extern "C" {
    pub fn container_of(_arg: p, ieee80211_vif: struct, _arg: drv_priv) -> return;
}
extern "C" {
    pub fn rtwvif_to_vif(_arg: rtwvif_link->rtwvif) -> return;
}
extern "C" {
    pub fn container_of(_arg: p, ieee80211_sta: struct, _arg: drv_priv) -> return;
}
extern "C" {
    pub fn rtwsta_to_sta(_arg: rtwsta_link->rtwsta) -> return;
}
// nolink = true;
// nolink = false;

// nolink = true;
// nolink = false;

extern "C" {
    pub fn rtw89_chan_get(_arg: rtwdev, _arg: rtwvif_link->chanctx_idx) -> return;
}
extern "C" {
    pub fn rtw89_chan_get(_arg: rtwdev, _arg: RTW89_CHANCTX_0) -> return;
}
extern "C" {
    pub fn __rtw89_chip_get_fw_def(_arg: rtwdev->chip, _arg: rtwdev->variant) -> return;
}
extern "C" {
    pub fn dev_alloc_skb(_arg: length) -> return;
}
extern "C" {
    pub fn rcu_access_pointer(_arg: skb_data->wait) -> return;
}
// Don't access skb anymore after completion
extern "C" {
    pub fn BIT(_arg: RTW89_PHY_0) -> return;
}
extern "C" {
    pub fn BIT(_arg: RTW89_PHY_1) -> return;
}
extern "C" {
    pub fn BIT(BIT(RTW89_PHY_1: RTW89_PHY_0) |) -> return;
}
extern "C" {
    pub fn BIT(_arg: RTW89_PHY_0) -> return;
}

extern "C" {
    pub fn rtw89_core_tx_kick_off(rtwdev: *mut rtw89_dev, qsel: u8);
}
extern "C" {
    pub fn rtw89_core_get_ch_dma(rtwdev: *mut rtw89_dev, qsel: u8) -> u8;
}
extern "C" {
    pub fn rtw89_core_get_ch_dma_v1(rtwdev: *mut rtw89_dev, qsel: u8) -> u8;
}
extern "C" {
    pub fn rtw89_core_get_ch_dma_v2(rtwdev: *mut rtw89_dev, qsel: u8) -> u8;
}
extern "C" {
    pub fn rtw89_core_napi_start(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_core_napi_stop(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_core_napi_init(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_core_napi_deinit(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_core_tid_rx_stats_reset(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_core_rfkill_poll(rtwdev: *mut rtw89_dev, force: bool);
}
extern "C" {
    pub fn rtw89_check_quirks(rtwdev: *mut rtw89_dev, quirks: *const dmi_system_id);
}
extern "C" {
    pub fn rtw89_core_init(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_core_deinit(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_core_register(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_core_unregister(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_free_ieee80211_hw(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_acquire_mac_id(rtwdev: *mut rtw89_dev) -> u8;
}
extern "C" {
    pub fn rtw89_release_mac_id(rtwdev: *mut rtw89_dev, mac_id: u8);
}
extern "C" {
    pub fn rtw89_vif_unset_link(rtwvif: *mut rtw89_vif, link_id: c_uint);
}
extern "C" {
    pub fn rtw89_sta_unset_link(rtwsta: *mut rtw89_sta, link_id: c_uint);
}
extern "C" {
    pub fn rtw89_core_set_chip_txpwr(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_get_default_chandef(chandef: *mut cfg80211_chan_def);
}
extern "C" {
    pub fn rtw89_set_channel(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_core_acquire_bit_map(addr: *mut c_ulong, size: c_ulong) -> u8;
}
extern "C" {
    pub fn rtw89_core_release_bit_map(addr: *mut c_ulong, bit: u8);
}
extern "C" {
    pub fn rtw89_core_release_all_bits_map(addr: *mut c_ulong, nbits: c_uint);
}
extern "C" {
    pub fn rtw89_vif_type_mapping(rtwvif_link: *mut rtw89_vif_link, assoc: bool);
}
extern "C" {
    pub fn rtw89_chip_info_setup(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_legacy_rate_to_bitrate(rtwdev: *mut rtw89_dev, legacy_rate: u8, bitrate: *mut u16) -> bool;
}
extern "C" {
    pub fn rtw89_regd_setup(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_regd_init_hint(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_core_start(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_core_stop(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_core_update_beacon_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_core_csa_beacon_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_roc_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_roc_start(rtwdev: *mut rtw89_dev, rtwvif: *mut rtw89_vif);
}
extern "C" {
    pub fn rtw89_roc_end(rtwdev: *mut rtw89_dev, rtwvif: *mut rtw89_vif);
}
extern "C" {
    pub fn rtw89_core_dm_disable_cfg(rtwdev: *mut rtw89_dev, new: u32);
}
extern "C" {
    pub fn rtw89_core_dm_disable_set(rtwdev: *mut rtw89_dev, type: rtw89_dm_type);
}
extern "C" {
    pub fn rtw89_core_dm_disable_clr(rtwdev: *mut rtw89_dev, type: rtw89_dm_type);
}
