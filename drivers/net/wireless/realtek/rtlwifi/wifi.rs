//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/wifi.h
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

pub const MASKBYTE0: c_uint = 0xff;
pub const MASKBYTE1: c_uint = 0xff00;
pub const MASKBYTE2: c_uint = 0xff0000;
pub const MASKBYTE3: c_uint = 0xff000000;
pub const MASKH3BYTES: c_uint = 0xffffff00;
pub const MASKHWORD: c_uint = 0xffff0000;
pub const MASKLWORD: c_uint = 0x0000ffff;
pub const MASKDWORD: c_uint = 0xffffffff;
pub const MASK12BITS: c_uint = 0xfff;
pub const MASKH4BITS: c_uint = 0xf0000000;
pub const MASKOFDM_D: c_uint = 0xffc00000;
pub const MASKCCK: c_uint = 0x3f3f3f3f;
pub const MASK4BITS: c_uint = 0x0f;
pub const MASK20BITS: c_uint = 0xfffff;
pub const RFREG_OFFSET_MASK: c_uint = 0xfffff;
pub const MASKBYTE0: c_uint = 0xff;
pub const MASKBYTE1: c_uint = 0xff00;
pub const MASKBYTE2: c_uint = 0xff0000;
pub const MASKBYTE3: c_uint = 0xff000000;
pub const MASKHWORD: c_uint = 0xffff0000;
pub const MASKLWORD: c_uint = 0x0000ffff;
pub const MASKDWORD: c_uint = 0xffffffff;
pub const MASK12BITS: c_uint = 0xfff;
pub const MASKH4BITS: c_uint = 0xf0000000;
pub const MASKOFDM_D: c_uint = 0xffc00000;
pub const MASKCCK: c_uint = 0x3f3f3f3f;
pub const MASK4BITS: c_uint = 0x0f;
pub const MASK20BITS: c_uint = 0xfffff;
pub const RFREG_OFFSET_MASK: c_uint = 0xfffff;
// For dual MAC RTL8192DU
pub const MAC0_ACCESS_PHY1: c_uint = 0x4000;
pub const MAC1_ACCESS_PHY0: c_uint = 0x2000;
pub const RF_CHANGE_BY_INIT: c_int = 0;

pub const IQK_ADDA_REG_NUM: c_int = 16;
pub const IQK_MAC_REG_NUM: c_int = 4;
pub const IQK_THRESHOLD: c_int = 8;
pub const MAX_KEY_LEN: c_int = 61;
pub const KEY_BUF_SIZE: c_int = 5;
// QoS related.
// aci: 0x00	Best Effort
// aci: 0x01	Background
// aci: 0x10	Video
// aci: 0x11	Voice
// Max: define total number.
pub const AC0_BE: c_int = 0;
pub const AC1_BK: c_int = 1;
pub const AC2_VI: c_int = 2;
pub const AC3_VO: c_int = 3;
pub const AC_MAX: c_int = 4;
pub const QOS_QUEUE_NUM: c_int = 4;
pub const RTL_MAC80211_NUM_QUEUE: c_int = 5;
pub const REALTEK_USB_VENQT_MAX_BUF_SIZE: c_int = 254;
pub const RTL_USB_MAX_RX_COUNT: c_int = 100;
pub const QBSS_LOAD_SIZE: c_int = 5;
pub const MAX_WMMELE_LENGTH: c_int = 64;
pub const ASPM_L1_LATENCY: c_int = 7;
pub const TOTAL_CAM_ENTRY: c_int = 32;
// slot time for 11g.
pub const RTL_SLOT_TIME_9: c_int = 9;
pub const RTL_SLOT_TIME_20: c_int = 20;
// related to tcp/ip.
pub const SNAP_SIZE: c_int = 6;
pub const PROTOC_TYPE_SIZE: c_int = 2;
// related with 802.11 frame
pub const MAC80211_3ADDR_LEN: c_int = 24;
pub const MAC80211_4ADDR_LEN: c_int = 30;

pub const CHANNEL_MAX_NUMBER_2G: c_int = 14;

// "phy_GetChnlGroup8812A" and
// "Hal_ReadTxPowerInfo8812A"
//
pub const CHANNEL_MAX_NUMBER_5G_80M: c_int = 7;

pub const MAX_PG_GROUP: c_int = 13;
pub const CHANNEL_GROUP_MAX_2G: c_int = 3;
pub const CHANNEL_GROUP_IDX_5GL: c_int = 3;
pub const CHANNEL_GROUP_IDX_5GM: c_int = 6;
pub const CHANNEL_GROUP_IDX_5GH: c_int = 9;
pub const CHANNEL_GROUP_MAX_5G: c_int = 9;
pub const AVG_THERMAL_NUM: c_int = 8;
pub const AVG_THERMAL_NUM_88E: c_int = 4;
pub const AVG_THERMAL_NUM_8723BE: c_int = 4;
pub const MAX_TID_COUNT: c_int = 9;
// for early mode
pub const FCS_LEN: c_int = 4;
pub const EM_HDR_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8192c_h2c_cmd {
    H2C_AP_OFFLOAD = 0,
    H2C_SETPWRMODE = 1,
    H2C_JOINBSSRPT = 2,
    H2C_RSVDPAGE = 3,
    H2C_RSSI_REPORT = 5,
    H2C_RA_MASK = 6,
    H2C_MACID_PS_MODE = 7,
    H2C_P2P_PS_OFFLOAD = 8,
    H2C_MAC_MODE_SEL = 9,
    H2C_PWRM = 15,
    H2C_P2P_PS_CTW_CMD = 24,
    MAX_H2CCMD
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_c2h_evt_v1 {
    C2H_DBG = 0,
    C2H_LB = 1,
    C2H_TXBF = 2,
    C2H_TX_REPORT = 3,
    C2H_BT_INFO = 9,
    C2H_BT_MP = 11,
    C2H_RA_RPT = 12,

    C2H_FW_SWCHNL = 0x10,
    C2H_IQK_FINISH = 0x11,

    C2H_EXT_V2 = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_c2h_evt_v2 {
    C2H_V2_CCX_RPT = 0x0F,
}

pub const C2H_DATA_OFFSET: c_int = 2;

pub const MAX_TX_COUNT: c_int = 4;
pub const MAX_REGULATION_NUM: c_int = 4;
pub const MAX_RF_PATH_NUM: c_int = 4;

pub const MAX_2_4G_BANDWIDTH_NUM: c_int = 4;
pub const MAX_5G_BANDWIDTH_NUM: c_int = 4;
pub const MAX_RF_PATH: c_int = 4;
pub const MAX_CHNL_GROUP_24G: c_int = 6;
pub const MAX_CHNL_GROUP_5G: c_int = 14;
pub const TX_PWR_BY_RATE_NUM_BAND: c_int = 2;
pub const TX_PWR_BY_RATE_NUM_RF: c_int = 4;
pub const TX_PWR_BY_RATE_NUM_SECTION: c_int = 12;

pub const DEL_SW_IDX_SZ: c_int = 30;
// For now, it's just for 8192ee
// but not OK yet, keep it 0
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rf_tx_num {
    RF_1TX = 0,
    RF_2TX,
    RF_MAX_TX_NUM,
    RF_TX_NUM_NONIMPLEMENT,
}

pub const PACKET_NORMAL: c_int = 0;
pub const PACKET_DHCP: c_int = 1;
pub const PACKET_ARP: c_int = 2;
pub const PACKET_EAPOL: c_int = 3;
pub const MAX_SUPPORT_WOL_PATTERN_NUM: c_int = 16;
pub const RSVD_WOL_PATTERN_NUM: c_int = 1;
pub const WKFMCAM_ADDR_NUM: c_int = 6;
pub const WKFMCAM_SIZE: c_int = 24;
pub const MAX_WOL_BIT_MASK_SIZE: c_int = 16;
// MIN LEN keeps 13 here
pub const MIN_WOL_PATTERN_SIZE: c_int = 13;
pub const MAX_WOL_PATTERN_SIZE: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtlwifi_firmware_header {
    pub signature: __le16,
    pub category: u8,
    pub function: u8,
    pub version: __le16,
    pub subversion: u8,
    pub rsvd1: u8,
    pub month: u8,
    pub date: u8,
    pub hour: u8,
    pub minute: u8,
    pub ramcodesize: __le16,
    pub rsvd2: __le16,
    pub svnindex: __le32,
    pub rsvd3: __le32,
    pub rsvd4: __le32,
    pub rsvd5: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txpower_info_2g {
    pub index_cck_base: [u8; MAX_RF_PATH][MAX_CHNL_GROUP_24G],
    pub index_bw40_base: [u8; MAX_RF_PATH][MAX_CHNL_GROUP_24G],
// If only one tx, only BW20 and OFDM are used.
    pub cck_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
    pub ofdm_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
    pub bw20_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
    pub bw40_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
    pub bw80_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
    pub bw160_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txpower_info_5g {
    pub index_bw40_base: [u8; MAX_RF_PATH][MAX_CHNL_GROUP_5G],
// If only one tx, only BW20, OFDM, BW80 and BW160 are used.
    pub ofdm_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
    pub bw20_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
    pub bw40_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
    pub bw80_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
    pub bw160_diff: [u8; MAX_RF_PATH][MAX_TX_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rate_section {
    CCK = 0,
    OFDM,
    HT_MCS0_MCS7,
    HT_MCS8_MCS15,
    VHT_1SSMCS0_1SSMCS9,
    VHT_2SSMCS0_2SSMCS9,
    MAX_RATE_SECTION,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intf_type {
    INTF_PCI = 0,
    INTF_USB = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radio_path {
    RF90_PATH_A = 0,
    RF90_PATH_B = 1,
    RF90_PATH_C = 2,
    RF90_PATH_D = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radio_mask {
    RF_MASK_A = BIT(0),
    RF_MASK_B = BIT(1),
    RF_MASK_C = BIT(2),
    RF_MASK_D = BIT(3),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regulation_txpwr_lmt {
    TXPWR_LMT_FCC = 0,
    TXPWR_LMT_MKK = 1,
    TXPWR_LMT_ETSI = 2,
    TXPWR_LMT_WW = 3,

    TXPWR_LMT_MAX_REGULATION_NUM = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_eeprom_type {
    EEPROM_93C46,
    EEPROM_93C56,
    EEPROM_BOOT_EFUSE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ttl_status {
    RTL_STATUS_INTERFACE_START = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hardware_type {
    HARDWARE_TYPE_RTL8192E,
    HARDWARE_TYPE_RTL8192U,
    HARDWARE_TYPE_RTL8192SE,
    HARDWARE_TYPE_RTL8192SU,
    HARDWARE_TYPE_RTL8192CE,
    HARDWARE_TYPE_RTL8192CU,
    HARDWARE_TYPE_RTL8192DE,
    HARDWARE_TYPE_RTL8192DU,
    HARDWARE_TYPE_RTL8723AE,
    HARDWARE_TYPE_RTL8723U,
    HARDWARE_TYPE_RTL8188EE,
    HARDWARE_TYPE_RTL8723BE,
    HARDWARE_TYPE_RTL8192EE,
    HARDWARE_TYPE_RTL8821AE,
    HARDWARE_TYPE_RTL8812AE,
    HARDWARE_TYPE_RTL8822BE,

// keep it last
    HARDWARE_TYPE_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scan_operation_backup_opt {
    SCAN_OPT_BACKUP = 0,
    SCAN_OPT_BACKUP_BAND0 = 0,
    SCAN_OPT_BACKUP_BAND1,
    SCAN_OPT_RESTORE,
    SCAN_OPT_MAX
}

// RF state.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rf_pwrstate {
    ERFON,
    ERFSLEEP,
    ERFOFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bb_reg_def {
    pub rfintfs: u32,
    pub rfintfi: u32,
    pub rfintfo: u32,
    pub rfintfe: u32,
    pub rf3wire_offset: u32,
    pub rflssi_select: u32,
    pub rftxgain_stage: u32,
    pub rfhssi_para1: u32,
    pub rfhssi_para2: u32,
    pub rfsw_ctrl: u32,
    pub rfagc_control1: u32,
    pub rfagc_control2: u32,
    pub rfrxiq_imbal: u32,
    pub rfrx_afe: u32,
    pub rftxiq_imbal: u32,
    pub rftx_afe: u32,
    pub /: *mut *mut u32 rf_rb; / rflssi_readback,
    pub /: *mut *mut u32 rf_rbpi; / rflssi_readbackpi,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum io_type {
    IO_CMD_PAUSE_DM_BY_SCAN = 0,
    IO_CMD_PAUSE_BAND0_DM_BY_SCAN = 0,
    IO_CMD_PAUSE_BAND1_DM_BY_SCAN = 1,
    IO_CMD_RESUME_DM_BY_SCAN = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_variables {
    HW_VAR_ETHER_ADDR = 0x0,
    HW_VAR_MULTICAST_REG = 0x1,
    HW_VAR_BASIC_RATE = 0x2,
    HW_VAR_BSSID = 0x3,
    HW_VAR_MEDIA_STATUS = 0x4,
    HW_VAR_SECURITY_CONF = 0x5,
    HW_VAR_BEACON_INTERVAL = 0x6,
    HW_VAR_ATIM_WINDOW = 0x7,
    HW_VAR_LISTEN_INTERVAL = 0x8,
    HW_VAR_CS_COUNTER = 0x9,
    HW_VAR_DEFAULTKEY0 = 0xa,
    HW_VAR_DEFAULTKEY1 = 0xb,
    HW_VAR_DEFAULTKEY2 = 0xc,
    HW_VAR_DEFAULTKEY3 = 0xd,
    HW_VAR_SIFS = 0xe,
    HW_VAR_R2T_SIFS = 0xf,
    HW_VAR_DIFS = 0x10,
    HW_VAR_EIFS = 0x11,
    HW_VAR_SLOT_TIME = 0x12,
    HW_VAR_ACK_PREAMBLE = 0x13,
    HW_VAR_CW_CONFIG = 0x14,
    HW_VAR_CW_VALUES = 0x15,
    HW_VAR_RATE_FALLBACK_CONTROL = 0x16,
    HW_VAR_CONTENTION_WINDOW = 0x17,
    HW_VAR_RETRY_COUNT = 0x18,
    HW_VAR_TR_SWITCH = 0x19,
    HW_VAR_COMMAND = 0x1a,
    HW_VAR_WPA_CONFIG = 0x1b,
    HW_VAR_AMPDU_MIN_SPACE = 0x1c,
    HW_VAR_SHORTGI_DENSITY = 0x1d,
    HW_VAR_AMPDU_FACTOR = 0x1e,
    HW_VAR_MCS_RATE_AVAILABLE = 0x1f,
    HW_VAR_AC_PARAM = 0x20,
    HW_VAR_ACM_CTRL = 0x21,
    HW_VAR_DIS_REQ_QSIZE = 0x22,
    HW_VAR_CCX_CHNL_LOAD = 0x23,
    HW_VAR_CCX_NOISE_HISTOGRAM = 0x24,
    HW_VAR_CCX_CLM_NHM = 0x25,
    HW_VAR_TXOPLIMIT = 0x26,
    HW_VAR_TURBO_MODE = 0x27,
    HW_VAR_RF_STATE = 0x28,
    HW_VAR_RF_OFF_BY_HW = 0x29,
    HW_VAR_BUS_SPEED = 0x2a,
    HW_VAR_SET_DEV_POWER = 0x2b,

    HW_VAR_RCR = 0x2c,
    HW_VAR_RATR_0 = 0x2d,
    HW_VAR_RRSR = 0x2e,
    HW_VAR_CPU_RST = 0x2f,
    HW_VAR_CHECK_BSSID = 0x30,
    HW_VAR_LBK_MODE = 0x31,
    HW_VAR_AES_11N_FIX = 0x32,
    HW_VAR_USB_RX_AGGR = 0x33,
    HW_VAR_USER_CONTROL_TURBO_MODE = 0x34,
    HW_VAR_RETRY_LIMIT = 0x35,
    HW_VAR_INIT_TX_RATE = 0x36,
    HW_VAR_TX_RATE_REG = 0x37,
    HW_VAR_EFUSE_USAGE = 0x38,
    HW_VAR_EFUSE_BYTES = 0x39,
    HW_VAR_AUTOLOAD_STATUS = 0x3a,
    HW_VAR_RF_2R_DISABLE = 0x3b,
    HW_VAR_SET_RPWM = 0x3c,
    HW_VAR_H2C_FW_PWRMODE = 0x3d,
    HW_VAR_H2C_FW_JOINBSSRPT = 0x3e,
    HW_VAR_H2C_FW_MEDIASTATUSRPT = 0x3f,
    HW_VAR_H2C_FW_P2P_PS_OFFLOAD = 0x40,
    HW_VAR_FW_PSMODE_STATUS = 0x41,
    HW_VAR_INIT_RTS_RATE = 0x42,
    HW_VAR_RESUME_CLK_ON = 0x43,
    HW_VAR_FW_LPS_ACTION = 0x44,
    HW_VAR_1X1_RECV_COMBINE = 0x45,
    HW_VAR_STOP_SEND_BEACON = 0x46,
    HW_VAR_TSF_TIMER = 0x47,
    HW_VAR_IO_CMD = 0x48,

    HW_VAR_RF_RECOVERY = 0x49,
    HW_VAR_H2C_FW_UPDATE_GTK = 0x4a,
    HW_VAR_WF_MASK = 0x4b,
    HW_VAR_WF_CRC = 0x4c,
    HW_VAR_WF_IS_MAC_ADDR = 0x4d,
    HW_VAR_H2C_FW_OFFLOAD = 0x4e,
    HW_VAR_RESET_WFCRC = 0x4f,

    HW_VAR_HANDLE_FW_C2H = 0x50,
    HW_VAR_DL_FW_RSVD_PAGE = 0x51,
    HW_VAR_AID = 0x52,
    HW_VAR_HW_SEQ_ENABLE = 0x53,
    HW_VAR_CORRECT_TSF = 0x54,
    HW_VAR_BCN_VALID = 0x55,
    HW_VAR_FWLPS_RF_ON = 0x56,
    HW_VAR_DUAL_TSF_RST = 0x57,
    HW_VAR_SWITCH_EPHY_WOWLAN = 0x58,
    HW_VAR_INT_MIGRATION = 0x59,
    HW_VAR_INT_AC = 0x5a,
    HW_VAR_RF_TIMING = 0x5b,

    HAL_DEF_WOWLAN = 0x5c,
    HW_VAR_MRC = 0x5d,
    HW_VAR_KEEP_ALIVE = 0x5e,
    HW_VAR_NAV_UPPER = 0x5f,

    HW_VAR_MGT_FILTER = 0x60,
    HW_VAR_CTRL_FILTER = 0x61,
    HW_VAR_DATA_FILTER = 0x62,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_media_status {
    RT_MEDIA_DISCONNECT = 0,
    RT_MEDIA_CONNECT = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_oem_id {
    RT_CID_DEFAULT = 0,
    RT_CID_8187_ALPHA0 = 1,
    RT_CID_8187_SERCOMM_PS = 2,
    RT_CID_8187_HW_LED = 3,
    RT_CID_8187_NETGEAR = 4,
    RT_CID_WHQL = 5,
    RT_CID_819X_CAMEO = 6,
    RT_CID_819X_RUNTOP = 7,
    RT_CID_819X_SENAO = 8,
    RT_CID_TOSHIBA = 9,
    RT_CID_819X_NETCORE = 10,
    RT_CID_NETTRONIX = 11,
    RT_CID_DLINK = 12,
    RT_CID_PRONET = 13,
    RT_CID_COREGA = 14,
    RT_CID_819X_ALPHA = 15,
    RT_CID_819X_SITECOM = 16,
    RT_CID_CCX = 17,
    RT_CID_819X_LENOVO = 18,
    RT_CID_819X_QMI = 19,
    RT_CID_819X_EDIMAX_BELKIN = 20,
    RT_CID_819X_SERCOMM_BELKIN = 21,
    RT_CID_819X_CAMEO1 = 22,
    RT_CID_819X_MSI = 23,
    RT_CID_819X_ACER = 24,
    RT_CID_819X_HP = 27,
    RT_CID_819X_CLEVO = 28,
    RT_CID_819X_ARCADYAN_BELKIN = 29,
    RT_CID_819X_SAMSUNG = 30,
    RT_CID_819X_WNC_COREGA = 31,
    RT_CID_819X_FOXCOON = 32,
    RT_CID_819X_DELL = 33,
    RT_CID_819X_PRONETS = 34,
    RT_CID_819X_EDIMAX_ASUS = 35,
    RT_CID_NETGEAR = 36,
    RT_CID_PLANEX = 37,
    RT_CID_CC_C = 38,
    RT_CID_LENOVO_CHINA = 40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_descs {
    HW_DESC_OWN,
    HW_DESC_RXOWN,
    HW_DESC_TX_NEXTDESC_ADDR,
    HW_DESC_TXBUFF_ADDR,
    HW_DESC_RXBUFF_ADDR,
    HW_DESC_RXPKT_LEN,
    HW_DESC_RXERO,
    HW_DESC_RX_PREPARE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prime_sc {
    PRIME_CHNL_OFFSET_DONT_CARE = 0,
    PRIME_CHNL_OFFSET_LOWER = 1,
    PRIME_CHNL_OFFSET_UPPER = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rf_type {
    RF_1T1R = 0,
    RF_1T2R = 1,
    RF_2T2R = 2,
    RF_2T2R_GREEN = 3,
    RF_2T3R = 4,
    RF_2T4R = 5,
    RF_3T3R = 6,
    RF_3T4R = 7,
    RF_4T4R = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ht_channel_width {
    HT_CHANNEL_WIDTH_20 = 0,
    HT_CHANNEL_WIDTH_20_40 = 1,
    HT_CHANNEL_WIDTH_80 = 2,
    HT_CHANNEL_WIDTH_MAX,
}

// Ref: 802.11i spec D10.0 7.3.2.25.1
// Cipher Suites Encryption Algorithms
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_enc_alg {
    NO_ENCRYPTION = 0,
    WEP40_ENCRYPTION = 1,
    TKIP_ENCRYPTION = 2,
    RSERVED_ENCRYPTION = 3,
    AESCCMP_ENCRYPTION = 4,
    WEP104_ENCRYPTION = 5,
    AESCMAC_ENCRYPTION = 6,	/*IEEE802.11w */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_hal_state {
    _HAL_STATE_STOP = 0,
    _HAL_STATE_START = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_desc_rate {
    DESC_RATE1M = 0x00,
    DESC_RATE2M = 0x01,
    DESC_RATE5_5M = 0x02,
    DESC_RATE11M = 0x03,

    DESC_RATE6M = 0x04,
    DESC_RATE9M = 0x05,
    DESC_RATE12M = 0x06,
    DESC_RATE18M = 0x07,
    DESC_RATE24M = 0x08,
    DESC_RATE36M = 0x09,
    DESC_RATE48M = 0x0a,
    DESC_RATE54M = 0x0b,

    DESC_RATEMCS0 = 0x0c,
    DESC_RATEMCS1 = 0x0d,
    DESC_RATEMCS2 = 0x0e,
    DESC_RATEMCS3 = 0x0f,
    DESC_RATEMCS4 = 0x10,
    DESC_RATEMCS5 = 0x11,
    DESC_RATEMCS6 = 0x12,
    DESC_RATEMCS7 = 0x13,
    DESC_RATEMCS8 = 0x14,
    DESC_RATEMCS9 = 0x15,
    DESC_RATEMCS10 = 0x16,
    DESC_RATEMCS11 = 0x17,
    DESC_RATEMCS12 = 0x18,
    DESC_RATEMCS13 = 0x19,
    DESC_RATEMCS14 = 0x1a,
    DESC_RATEMCS15 = 0x1b,
    DESC_RATEMCS15_SG = 0x1c,
    DESC_RATEMCS32 = 0x20,

    DESC_RATEVHT1SS_MCS0 = 0x2c,
    DESC_RATEVHT1SS_MCS1 = 0x2d,
    DESC_RATEVHT1SS_MCS2 = 0x2e,
    DESC_RATEVHT1SS_MCS3 = 0x2f,
    DESC_RATEVHT1SS_MCS4 = 0x30,
    DESC_RATEVHT1SS_MCS5 = 0x31,
    DESC_RATEVHT1SS_MCS6 = 0x32,
    DESC_RATEVHT1SS_MCS7 = 0x33,
    DESC_RATEVHT1SS_MCS8 = 0x34,
    DESC_RATEVHT1SS_MCS9 = 0x35,
    DESC_RATEVHT2SS_MCS0 = 0x36,
    DESC_RATEVHT2SS_MCS1 = 0x37,
    DESC_RATEVHT2SS_MCS2 = 0x38,
    DESC_RATEVHT2SS_MCS3 = 0x39,
    DESC_RATEVHT2SS_MCS4 = 0x3a,
    DESC_RATEVHT2SS_MCS5 = 0x3b,
    DESC_RATEVHT2SS_MCS6 = 0x3c,
    DESC_RATEVHT2SS_MCS7 = 0x3d,
    DESC_RATEVHT2SS_MCS8 = 0x3e,
    DESC_RATEVHT2SS_MCS9 = 0x3f,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_var_map {
// reg map
    SYS_ISO_CTRL = 0,
    SYS_FUNC_EN,
    SYS_CLK,
    MAC_RCR_AM,
    MAC_RCR_AB,
    MAC_RCR_ACRC32,
    MAC_RCR_ACF,
    MAC_RCR_AAP,
    MAC_HIMR,
    MAC_HIMRE,
    MAC_HSISR,

// efuse map
    EFUSE_TEST,
    EFUSE_CTRL,
    EFUSE_CLK,
    EFUSE_CLK_CTRL,
    EFUSE_PWC_EV12V,
    EFUSE_FEN_ELDR,
    EFUSE_LOADER_CLK_EN,
    EFUSE_ANA8M,
    EFUSE_HWSET_MAX_SIZE,
    EFUSE_MAX_SECTION_MAP,
    EFUSE_REAL_CONTENT_SIZE,
    EFUSE_OOB_PROTECT_BYTES_LEN,
    EFUSE_ACCESS,

// CAM map
    RWCAM,
    WCAMI,
    RCAMO,
    CAMDBG,
    SECR,
    SEC_CAM_NONE,
    SEC_CAM_WEP40,
    SEC_CAM_TKIP,
    SEC_CAM_AES,
    SEC_CAM_WEP104,

// IMR map
    RTL_IMR_BCNDMAINT6,	/*Beacon DMA Interrupt 6 */
    RTL_IMR_BCNDMAINT5,	/*Beacon DMA Interrupt 5 */
    RTL_IMR_BCNDMAINT4,	/*Beacon DMA Interrupt 4 */
    RTL_IMR_BCNDMAINT3,	/*Beacon DMA Interrupt 3 */
    RTL_IMR_BCNDMAINT2,	/*Beacon DMA Interrupt 2 */
    RTL_IMR_BCNDMAINT1,	/*Beacon DMA Interrupt 1 */
    RTL_IMR_BCNDOK8,	/*Beacon Queue DMA OK Interrup 8 */
    RTL_IMR_BCNDOK7,	/*Beacon Queue DMA OK Interrup 7 */
    RTL_IMR_BCNDOK6,	/*Beacon Queue DMA OK Interrup 6 */
    RTL_IMR_BCNDOK5,	/*Beacon Queue DMA OK Interrup 5 */
    RTL_IMR_BCNDOK4,	/*Beacon Queue DMA OK Interrup 4 */
    RTL_IMR_BCNDOK3,	/*Beacon Queue DMA OK Interrup 3 */
    RTL_IMR_BCNDOK2,	/*Beacon Queue DMA OK Interrup 2 */
    RTL_IMR_BCNDOK1,	/*Beacon Queue DMA OK Interrup 1 */
    RTL_IMR_TIMEOUT2,	/*Timeout interrupt 2 */
    RTL_IMR_TIMEOUT1,	/*Timeout interrupt 1 */
    RTL_IMR_TXFOVW,		/*Transmit FIFO Overflow */
    RTL_IMR_PSTIMEOUT,	/*Power save time out interrupt */
    RTL_IMR_BCNINT,		/*Beacon DMA Interrupt 0 */
    RTL_IMR_RXFOVW,		/*Receive FIFO Overflow */
    RTL_IMR_RDU,		/*Receive Descriptor Unavailable */
    RTL_IMR_ATIMEND,	/*For 92C,ATIM Window End Interrupt */
    RTL_IMR_H2CDOK,		/*H2C Queue DMA OK Interrupt */
    RTL_IMR_BDOK,		/*Beacon Queue DMA OK Interrup */
    RTL_IMR_HIGHDOK,	/*High Queue DMA OK Interrupt */
    RTL_IMR_COMDOK,		/*Command Queue DMA OK Interrupt*/
    RTL_IMR_TBDOK,		/*Transmit Beacon OK interrup */
    RTL_IMR_MGNTDOK,	/*Management Queue DMA OK Interrupt */
    RTL_IMR_TBDER,		/*For 92C,Transmit Beacon Error Interrupt */
    RTL_IMR_BKDOK,		/*AC_BK DMA OK Interrupt */
    RTL_IMR_BEDOK,		/*AC_BE DMA OK Interrupt */
    RTL_IMR_VIDOK,		/*AC_VI DMA OK Interrupt */
    RTL_IMR_VODOK,		/*AC_VO DMA Interrupt */
    RTL_IMR_ROK,		/*Receive DMA OK Interrupt */
    RTL_IMR_HSISR_IND,	/*HSISR Interrupt*/
    RTL_IBSS_INT_MASKS,	/*(RTL_IMR_BCNINT | RTL_IMR_TBDOK |
// RTL_IMR_TBDER)
//
    RTL_IMR_C2HCMD,		/*fw interrupt*/

// CCK Rates, TxHT = 0
    RTL_RC_CCK_RATE1M,
    RTL_RC_CCK_RATE2M,
    RTL_RC_CCK_RATE5_5M,
    RTL_RC_CCK_RATE11M,

// OFDM Rates, TxHT = 0
    RTL_RC_OFDM_RATE6M,
    RTL_RC_OFDM_RATE9M,
    RTL_RC_OFDM_RATE12M,
    RTL_RC_OFDM_RATE18M,
    RTL_RC_OFDM_RATE24M,
    RTL_RC_OFDM_RATE36M,
    RTL_RC_OFDM_RATE48M,
    RTL_RC_OFDM_RATE54M,

    RTL_RC_HT_RATEMCS7,
    RTL_RC_HT_RATEMCS15,

    RTL_RC_VHT_RATE_1SS_MCS7,
    RTL_RC_VHT_RATE_1SS_MCS8,
    RTL_RC_VHT_RATE_1SS_MCS9,
    RTL_RC_VHT_RATE_2SS_MCS7,
    RTL_RC_VHT_RATE_2SS_MCS8,
    RTL_RC_VHT_RATE_2SS_MCS9,

// keep it last
    RTL_VAR_MAP_MAX,
}

// Firmware PS mode for control LPS.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _fw_ps_mode {
    FW_PS_ACTIVE_MODE = 0,
    FW_PS_MIN_MODE = 1,
    FW_PS_MAX_MODE = 2,
    FW_PS_DTIM_MODE = 3,
    FW_PS_VOIP_MODE = 4,
    FW_PS_UAPSD_WMM_MODE = 5,
    FW_PS_UAPSD_MODE = 6,
    FW_PS_IBSS_MODE = 7,
    FW_PS_WWLAN_MODE = 8,
    FW_PS_PM_RADIO_OFF = 9,
    FW_PS_PM_CARD_DISABLE = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_psmode {
    EACTIVE,		/*Active/Continuous access. */
    EMAXPS,			/*Max power save mode. */
    EFASTPS,		/*Fast power save mode. */
    EAUTOPS,		/*Auto power save mode. */
}

// LED related.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_ctl_mode {
    LED_CTL_POWER_ON = 1,
    LED_CTL_LINK = 2,
    LED_CTL_NO_LINK = 3,
    LED_CTL_TX = 4,
    LED_CTL_RX = 5,
    LED_CTL_SITE_SURVEY = 6,
    LED_CTL_POWER_OFF = 7,
    LED_CTL_START_TO_LINK = 8,
    LED_CTL_START_WPS = 9,
    LED_CTL_STOP_WPS = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_led_pin {
    LED_PIN_GPIO0,
    LED_PIN_LED0,
    LED_PIN_LED1,
    LED_PIN_LED2
}

// QoS related.
// acm implementation method.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acm_method {
    EACMWAY0_SWANDHW = 0,
    EACMWAY1_HW = 1,
    EACMWAY2_SW = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macphy_mode {
    SINGLEMAC_SINGLEPHY = 0,
    DUALMAC_DUALPHY,
    DUALMAC_SINGLEPHY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum band_type {
    BAND_ON_2_4G = 0,
    BAND_ON_5G,
    BAND_ON_BOTH,
    BANDMAX
}

// aci/aifsn Field.
// Ref: WMM spec 2.2.2: WME Parameter Element, p.12.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aci_aifsn {
    pub char_data: u8,
    pub aifsn:4: u8,
    pub acm:1: u8,
    pub aci:2: u8,
    pub reserved:1: u8,
    pub /: *mut *mut } f; / Field,
}

// mlme related.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wireless_mode {
    WIRELESS_MODE_UNKNOWN = 0x00,
    WIRELESS_MODE_A = 0x01,
    WIRELESS_MODE_B = 0x02,
    WIRELESS_MODE_G = 0x04,
    WIRELESS_MODE_AUTO = 0x08,
    WIRELESS_MODE_N_24G = 0x10,
    WIRELESS_MODE_N_5G = 0x20,
    WIRELESS_MODE_AC_5G = 0x40,
    WIRELESS_MODE_AC_24G  = 0x80,
    WIRELESS_MODE_AC_ONLY = 0x100,
    WIRELESS_MODE_MAX = 0x800
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ratr_table_mode {
    RATR_INX_WIRELESS_NGB = 0,
    RATR_INX_WIRELESS_NG = 1,
    RATR_INX_WIRELESS_NB = 2,
    RATR_INX_WIRELESS_N = 3,
    RATR_INX_WIRELESS_GB = 4,
    RATR_INX_WIRELESS_G = 5,
    RATR_INX_WIRELESS_B = 6,
    RATR_INX_WIRELESS_MC = 7,
    RATR_INX_WIRELESS_A = 8,
    RATR_INX_WIRELESS_AC_5N = 8,
    RATR_INX_WIRELESS_AC_24N = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ratr_table_mode_new {
    RATEID_IDX_BGN_40M_2SS = 0,
    RATEID_IDX_BGN_40M_1SS = 1,
    RATEID_IDX_BGN_20M_2SS_BN = 2,
    RATEID_IDX_BGN_20M_1SS_BN = 3,
    RATEID_IDX_GN_N2SS = 4,
    RATEID_IDX_GN_N1SS = 5,
    RATEID_IDX_BG = 6,
    RATEID_IDX_G = 7,
    RATEID_IDX_B = 8,
    RATEID_IDX_VHT_2SS = 9,
    RATEID_IDX_VHT_1SS = 10,
    RATEID_IDX_MIX1 = 11,
    RATEID_IDX_MIX2 = 12,
    RATEID_IDX_VHT_3SS = 13,
    RATEID_IDX_BGN_3SS = 14,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_link_state {
    MAC80211_NOLINK = 0,
    MAC80211_LINKING = 1,
    MAC80211_LINKED = 2,
    MAC80211_LINKED_SCANNING = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum act_category {
    ACT_CAT_QOS = 1,
    ACT_CAT_DLS = 2,
    ACT_CAT_BA = 3,
    ACT_CAT_HT = 7,
    ACT_CAT_WMM = 17,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ba_action {
    ACT_ADDBAREQ = 0,
    ACT_ADDBARSP = 1,
    ACT_DELBA = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt_polarity_ctl {
    RT_POLARITY_LOW_ACT = 0,
    RT_POLARITY_HIGH_ACT = 1,
}

// After 8188E, we use V2 reason define. 88C/8723A use V1 reason.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_wow_reason_v2 {
    FW_WOW_V2_PTK_UPDATE_EVENT = 0x01,
    FW_WOW_V2_GTK_UPDATE_EVENT = 0x02,
    FW_WOW_V2_DISASSOC_EVENT = 0x04,
    FW_WOW_V2_DEAUTH_EVENT = 0x08,
    FW_WOW_V2_FW_DISCONNECT_EVENT = 0x10,
    FW_WOW_V2_MAGIC_PKT_EVENT = 0x21,
    FW_WOW_V2_UNICAST_PKT_EVENT = 0x22,
    FW_WOW_V2_PATTERN_PKT_EVENT = 0x23,
    FW_WOW_V2_RTD3_SSID_MATCH_EVENT = 0x24,
    FW_WOW_V2_REALWOW_V2_WAKEUPPKT = 0x30,
    FW_WOW_V2_REALWOW_V2_ACKLOST = 0x31,
    FW_WOW_V2_REASON_MAX = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wolpattern_type {
    UNICAST_PATTERN = 0,
    MULTICAST_PATTERN = 1,
    BROADCAST_PATTERN = 2,
    DONT_CARE_DA = 3,
    UNKNOWN_TYPE = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum package_type {
    PACKAGE_DEFAULT,
    PACKAGE_QFN68,
    PACKAGE_TFBGA90,
    PACKAGE_TFBGA80,
    PACKAGE_TFBGA79
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_spec_ver {
    RTL_SPEC_NEW_RATEID = BIT(0),	/* use ratr_table_mode_new */
    RTL_SPEC_SUPPORT_VHT = BIT(1),	/* support VHT */
    RTL_SPEC_EXT_C2H = BIT(2),	/* extend FW C2H (e.g. TX REPORT) */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dm_info_query {
    DM_INFO_FA_OFDM,
    DM_INFO_FA_CCK,
    DM_INFO_FA_TOTAL,
    DM_INFO_CCA_OFDM,
    DM_INFO_CCA_CCK,
    DM_INFO_CCA_ALL,
    DM_INFO_CRC32_OK_VHT,
    DM_INFO_CRC32_OK_HT,
    DM_INFO_CRC32_OK_LEGACY,
    DM_INFO_CRC32_OK_CCK,
    DM_INFO_CRC32_ERROR_VHT,
    DM_INFO_CRC32_ERROR_HT,
    DM_INFO_CRC32_ERROR_LEGACY,
    DM_INFO_CRC32_ERROR_CCK,
    DM_INFO_EDCCA_FLAG,
    DM_INFO_OFDM_ENABLE,
    DM_INFO_CCK_ENABLE,
    DM_INFO_CRC32_OK_HT_AGG,
    DM_INFO_CRC32_ERROR_HT_AGG,
    DM_INFO_DBG_PORT_0,
    DM_INFO_CURR_IGI,
    DM_INFO_RSSI_MIN,
    DM_INFO_RSSI_MAX,
    DM_INFO_CLM_RATIO,
    DM_INFO_NHM_RATIO,
    DM_INFO_IQK_ALL,
    DM_INFO_IQK_OK,
    DM_INFO_IQK_NG,
    DM_INFO_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_packet_type {
    NORMAL_RX,
    TX_REPORT1,
    TX_REPORT2,
    HIS_REPORT,
    C2H_PACKET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtlwifi_tx_info {
    pub sn: c_int,
    pub send_time: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octet_string {
    pub octet: *mut u8,
    pub length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_led_ctl {
    pub led_opendrain: bool,
    pub sw_led0: rtl_led_pin,
    pub sw_led1: rtl_led_pin,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_qos_parameters {
    pub cw_min: __le16,
    pub cw_max: __le16,
    pub aifs: u8,
    pub flag: u8,
    pub tx_op: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_smooth_data {
    pub /: *mut *mut u32 elements[100]; /array to store values,
    pub /: *mut *mut u32 index; /index to current array to store,
    pub /: *mut *mut u32 total_num; /num of valid elements,
    pub /: *mut *mut u32 total_val; /sum of valid elements,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct false_alarm_statistics {
    pub cnt_parity_fail: u32,
    pub cnt_rate_illegal: u32,
    pub cnt_crc8_fail: u32,
    pub cnt_mcs_fail: u32,
    pub cnt_fast_fsync_fail: u32,
    pub cnt_sb_search_fail: u32,
    pub cnt_ofdm_fail: u32,
    pub cnt_cck_fail: u32,
    pub cnt_all: u32,
    pub cnt_ofdm_cca: u32,
    pub cnt_cck_cca: u32,
    pub cnt_cca_all: u32,
    pub cnt_bw_usc: u32,
    pub cnt_bw_lsc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_gain {
    pub xaagccore1: u8,
    pub xbagccore1: u8,
    pub xcagccore1: u8,
    pub xdagccore1: u8,
    pub cca: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wireless_stats {
    pub txbytesunicast: u64,
    pub txbytesmulticast: u64,
    pub txbytesbroadcast: u64,
    pub rxbytesunicast: u64,
    pub txbytesunicast_inperiod: u64,
    pub rxbytesunicast_inperiod: u64,
    pub txbytesunicast_inperiod_tp: u32,
    pub rxbytesunicast_inperiod_tp: u32,
    pub txbytesunicast_last: u64,
    pub rxbytesunicast_last: u64,
    pub rx_snr_db: [c_long; 4],
// Correct smoothed ss in Dbm, only used
// in driver to report real power now.
//
    pub recv_signal_power: c_long,
    pub signal_quality: c_long,
    pub last_sigstrength_inpercent: c_long,
    pub rssi_calculate_cnt: u32,
    pub pwdb_all_cnt: u32,
// Transformed, in dbm. Beautified signal
// strength for UI, not correct.
//
    pub signal_strength: c_long,
    pub rx_rssi_percentage: [u8; 4],
    pub rx_evm_dbm: [u8; 4],
    pub rx_evm_percentage: [u8; 2],
    pub rx_cfo_short: [u16; 4],
    pub rx_cfo_tail: [u16; 4],
    pub ui_rssi: rt_smooth_data,
    pub ui_link_quality: rt_smooth_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rate_adaptive {
    pub rate_adaptive_disabled: u8,
    pub ratr_state: u8,
    pub reserve: u16,
    pub high_rssi_thresh_for_ra: u32,
    pub high2low_rssi_thresh_for_ra: u32,
    pub low2high_rssi_thresh_for_ra40m: u8,
    pub low_rssi_thresh_for_ra40m: u32,
    pub low2high_rssi_thresh_for_ra20m: u8,
    pub low_rssi_thresh_for_ra20m: u32,
    pub upper_rssi_threshold_ratr: u32,
    pub middleupper_rssi_threshold_ratr: u32,
    pub middle_rssi_threshold_ratr: u32,
    pub middlelow_rssi_threshold_ratr: u32,
    pub low_rssi_threshold_ratr: u32,
    pub ultralow_rssi_threshold_ratr: u32,
    pub low_rssi_threshold_ratr_40m: u32,
    pub low_rssi_threshold_ratr_20m: u32,
    pub ping_rssi_enable: u8,
    pub ping_rssi_ratr: u32,
    pub ping_rssi_thresh_for_ra: u32,
    pub last_ratr: u32,
    pub pre_ratr_state: u8,
    pub ldpc_thres: u8,
    pub use_ldpc: bool,
    pub lower_rts_rate: bool,
    pub is_special_data: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regd_pair_mapping {
    pub reg_dmnenum: u16,
    pub reg_5ghz_ctl: u16,
    pub reg_2ghz_ctl: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_primary_cca {
    pub pricca_flag: u8,
    pub intf_flag: u8,
    pub intf_type: u8,
    pub dup_rts_flag: u8,
    pub monitor_flag: u8,
    pub ch_offset: u8,
    pub mf_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_regulatory {
    pub alpha2: [i8; 2],
    pub country_code: u16,
    pub max_power_level: u16,
    pub tp_scale: u32,
    pub current_rd: u16,
    pub current_rd_ext: u16,
    pub power_limit: i16,
    pub regpair: *mut regd_pair_mapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_rfkill {
    pub /: *mut *mut bool rfkill_state; /0 is off, 1 is on,
}

// for P2P PS
pub const P2P_MAX_NOA_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p2p_role {
    P2P_ROLE_DISABLE = 0,
    P2P_ROLE_DEVICE = 1,
    P2P_ROLE_CLIENT = 2,
    P2P_ROLE_GO = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p2p_ps_state {
    P2P_PS_DISABLE = 0,
    P2P_PS_ENABLE = 1,
    P2P_PS_SCAN = 2,
    P2P_PS_SCAN_DONE = 3,
    P2P_PS_ALLSTASLEEP = 4, /* for P2P GO */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p2p_ps_mode {
    P2P_PS_NONE = 0,
    P2P_PS_CTWINDOW = 1,
    P2P_PS_NOA	 = 2,
    P2P_PS_MIX = 3, /* CTWindow and NoA */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_p2p_ps_info {
    pub /: *mut *mut p2p_ps_mode p2p_ps_mode; / indicate p2p ps mode,
    pub /: *mut *mut p2p_ps_state p2p_ps_state; / indicate p2p ps state,
    pub /: *mut *mut u8 noa_index; / Identifies instance of Notice of Absence timing.,
// Client traffic window. A period of time in TU after TBTT.
    pub ctwindow: u8,
    pub /: *mut *mut u8 opp_ps; / opportunistic power save.,
    pub /: *mut *mut u8 noa_num; / number of NoA descriptor in P2P IE.,
// Count for owner, Type of client.
    pub noa_count_type: [u8; P2P_MAX_NOA_NUM],
// Max duration for owner, preferred or min acceptable duration
// for client.
//
    pub noa_duration: [u32; P2P_MAX_NOA_NUM],
// Length of interval for owner, preferred or max acceptable intervali
// of client.
//
    pub noa_interval: [u32; P2P_MAX_NOA_NUM],
// schedule in terms of the lower 4 bytes of the TSF timer.
    pub noa_start_time: [u32; P2P_MAX_NOA_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p2p_ps_offload_t {
    pub offload_en:1: u8,
    pub /: *mut *mut u8 role:1; / 1: Owner, 0: Client,
    pub ctwindow_en:1: u8,
    pub noa0_en:1: u8,
    pub noa1_en:1: u8,
    pub allstasleep:1: u8,
    pub discovery:1: u8,
    pub reserved:1: u8,
}

pub const IQK_MATRIX_REG_NUM: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqk_matrix_regs {
    pub iqk_done: bool,
    pub value: [c_long; 1][IQK_MATRIX_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_parameters {
    pub length: u16,
    pub pdata: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_param_tab_index {
    PHY_REG_2T,
    PHY_REG_1T,
    PHY_REG_PG,
    RADIOA_2T,
    RADIOB_2T,
    RADIOA_1T,
    RADIOB_1T,
    MAC_REG,
    AGCTAB_2T,
    AGCTAB_1T,
    MAX_TAB
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_phy {
    pub /: *mut *mut bb_reg_def phyreg_def[4]; /Radio A/B/C/D,
    pub initgain_backup: init_gain,
    pub current_io_type: io_type,
    pub rf_mode: u8,
    pub rf_type: u8,
    pub current_chan_bw: u8,
    pub set_bwmode_inprogress: u8,
    pub sw_chnl_inprogress: u8,
    pub sw_chnl_stage: u8,
    pub sw_chnl_step: u8,
    pub current_channel: u8,
    pub set_io_inprogress: u8,
    pub lck_inprogress: u8,
// record for power tracking
    pub reg_e94: i32,
    pub reg_e9c: i32,
    pub reg_ea4: i32,
    pub reg_eac: i32,
    pub reg_eb4: i32,
    pub reg_ebc: i32,
    pub reg_ec4: i32,
    pub reg_ecc: i32,
    pub reg_874: u32 reg_c04, reg_c08,,
    pub adda_backup: [u32; 16],
    pub iqk_mac_backup: [u32; IQK_MAC_REG_NUM],
    pub iqk_bb_backup: [u32; 10],
    pub iqk_initialized: bool,
    pub rfpath_rx_enable: [bool; MAX_RF_PATH],
    pub reg_837: u8,
// Dual mac
    pub need_iqk: bool,
    pub iqk_matrix: [iqk_matrix_regs; IQK_MATRIX_SETTINGS_NUM],
    pub rfpi_enable: bool,
    pub pwrgroup_cnt: u8,
    pub cck_high_power: u8,
// this is for 88E & 8723A
    pub mcs_txpwrlevel_origoffset: [u32; MAX_PG_GROUP][16],
// MAX_PG_GROUP groups of pwr diff by rates
    pub mcs_offset: [u32; MAX_PG_GROUP][16],
    pub default_initialgain: [u8; 4],
// the current Tx power level
    pub cur_cck_txpwridx: u8,
    pub cur_ofdm24g_txpwridx: u8,
    pub cur_bw20_txpwridx: u8,
    pub cur_bw40_txpwridx: u8,
    pub rfreg_chnlval: [u32; 2],
    pub /: *mut *mut u32 reg_rf3c[2]; / pathA / pathB,
    pub backup_rf_0x1a;/*92ee*/: *mut u32,
// bfsync
    pub framesync: u8,
    pub framesync_c34: u32,
    pub num_total_rfpath: u8,
    pub hwparam_tables: [phy_parameters; MAX_TAB],
    pub rf_pathmap: u16,
    pub polarity_ctl: rt_polarity_ctl,
}

pub const MAX_TID_COUNT: c_int = 9;
pub const RTL_AGG_STOP: c_int = 0;
pub const RTL_AGG_PROGRESS: c_int = 1;
pub const RTL_AGG_START: c_int = 2;
pub const RTL_AGG_OPERATIONAL: c_int = 3;
pub const RTL_RX_AGG_START: c_int = 1;
pub const RTL_RX_AGG_STOP: c_int = 0;
pub const RTL_AGG_EMPTYING_HW_QUEUE_ADDBA: c_int = 2;
pub const RTL_AGG_EMPTYING_HW_QUEUE_DELBA: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_ht_agg {
    pub txq_id: u16,
    pub wait_for_ba: u16,
    pub start_idx: u16,
    pub bitmap: u64,
    pub rate_n_flags: u32,
    pub agg_state: u8,
    pub rx_agg_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rssi_sta {
    pub undec_sm_pwdb: c_long,
    pub undec_sm_cck: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_tid_data {
    pub agg: rtl_ht_agg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_sta_info {
    pub list: list_head,
    pub tids: [rtl_tid_data; MAX_TID_COUNT],
// just used for ap adhoc or mesh
    pub rssi_stat: rssi_sta,
    pub rssi_level: u8,
    pub wireless_mode: u16,
    pub ratr_index: u8,
    pub mimo_ps: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub __packed: },
    pub rtl_priv: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_io {
    pub dev: *mut device,
    pub bb_mutex: mutex,
// PCI MEM map
    pub /: *mut *mut unsigned long pci_mem_end; /shared mem end,
    pub /: *mut *mut unsigned long pci_mem_start; /shared mem start,
// PCI IO map
    pub /: *mut *mut unsigned long pci_base_addr; /device I/O address,
    pub val): *mut *mut *mut void (write8)(struct rtl_priv rtlpriv, u32 addr, u8,
    pub val): *mut *mut *mut void (write16)(struct rtl_priv rtlpriv, u32 addr, u16,
    pub val): *mut *mut *mut void (write32)(struct rtl_priv rtlpriv, u32 addr, u32,
    pub data): *mut u8,
    pub addr): *mut *mut *mut u8 (read8)(struct rtl_priv rtlpriv, u32,
    pub addr): *mut *mut *mut u16 (read16)(struct rtl_priv rtlpriv, u32,
    pub addr): *mut *mut *mut u32 (read32)(struct rtl_priv rtlpriv, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_mac {
    pub mac_addr: [u8; ETH_ALEN],
    pub mac80211_registered: u8,
    pub beacon_enabled: u8,
    pub tx_ss_num: u32,
    pub rx_ss_num: u32,
    pub bands: [ieee80211_supported_band; NUM_NL80211_BANDS],
    pub hw: *mut ieee80211_hw,
    pub vif: *mut ieee80211_vif,
    pub opmode: nl80211_iftype,
// Probe Beacon management
    pub link_state: rtl_link_state,
    pub n_channels: c_int,
    pub n_bitrates: c_int,
    pub offchan_delay: bool,
    pub role*/: *mut *mut u8 p2p; /using p2p,
    pub p2p_in_use: bool,
// filters
    pub rx_conf: u32,
    pub rx_mgt_filter: u16,
    pub rx_ctrl_filter: u16,
    pub rx_data_filter: u16,
    pub act_scanning: bool,
    pub cnt_after_linked: u8,
    pub skip_scan: bool,
// early mode
// skb wait queue
    pub skb_waitq: [sk_buff_head; MAX_TID_COUNT],
    pub ht_stbc_cap: u8,
    pub ht_cur_stbc: u8,
// vht support
    pub vht_enable: u8,
    pub bw_80: u8,
    pub vht_cur_ldpc: u8,
    pub vht_cur_stbc: u8,
    pub vht_stbc_cap: u8,
    pub vht_ldpc_cap: u8,
// RDG
    pub rdg_en: bool,
// AP
    pub __aligned(2): u8 bssid[ETH_ALEN],
    pub vendor: u32,
    pub /: *mut *mut u8 mcs[16]; / 16 bytes mcs for HT rates.,
    pub /: *mut *mut u32 basic_rates; / b/g rates,
    pub ht_enable: u8,
    pub sgi_40: u8,
    pub sgi_20: u8,
    pub bw_40: u8,
    pub /: *mut *mut u16 mode; / wireless mode,
    pub slot_time: u8,
    pub short_preamble: u8,
    pub use_cts_protect: u8,
    pub cur_40_prime_sc: u8,
    pub cur_40_prime_sc_bk: u8,
    pub cur_80_prime_sc: u8,
    pub tsf: u64,
    pub retry_short: u8,
    pub retry_long: u8,
    pub assoc_id: u16,
    pub hiddenssid: bool,
// IBSS
    pub beacon_interval: c_int,
// AMPDU
    pub /: *mut *mut u8 min_space_cfg; /For Min spacing configurations,
    pub max_mss_density: u8,
    pub current_ampdu_factor: u8,
    pub current_ampdu_density: u8,
// QOS & EDCA
    pub edca_param: [ieee80211_tx_queue_params; RTL_MAC80211_NUM_QUEUE],
    pub ac: [rtl_qos_parameters; AC_MAX],
// counters
    pub last_txok_cnt: u64,
    pub last_rxok_cnt: u64,
    pub last_bt_edca_ul: u32,
    pub last_bt_edca_dl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btdm_8723 {
    pub all_off: bool,
    pub agc_table_en: bool,
    pub adc_back_off_on: bool,
    pub b2_ant_hid_en: bool,
    pub low_penalty_rate_adaptive: bool,
    pub rf_rx_lpf_shrink: bool,
    pub reject_aggre_pkt: bool,
    pub tra_tdma_on: bool,
    pub tra_tdma_nav: u8,
    pub tra_tdma_ant: u8,
    pub tdma_on: bool,
    pub tdma_ant: u8,
    pub tdma_nav: u8,
    pub tdma_dac_swing: u8,
    pub fw_dac_swing_lvl: u8,
    pub ps_tdma_on: bool,
    pub ps_tdma_byte: [u8; 5],
    pub pta_on: bool,
    pub val_0x6c0: u32,
    pub val_0x6c8: u32,
    pub val_0x6cc: u32,
    pub sw_dac_swing_on: bool,
    pub sw_dac_swing_lvl: u32,
    pub wlan_act_hi: u32,
    pub wlan_act_lo: u32,
    pub bt_retry_index: u32,
    pub dec_bt_pwr: bool,
    pub ignore_wlan_act: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_coexist_8723 {
    pub high_priority_tx: u32,
    pub high_priority_rx: u32,
    pub low_priority_tx: u32,
    pub low_priority_rx: u32,
    pub c2h_bt_info: u8,
    pub c2h_bt_info_req_sent: bool,
    pub c2h_bt_inquiry_page: bool,
    pub bt_inq_page_start_time: c_ulong,
    pub bt_retry_cnt: u8,
    pub c2h_bt_info_original: u8,
    pub bt_inquiry_page_cnt: u8,
    pub btdm: btdm_8723,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_hal {
    pub hw: *mut ieee80211_hw,
    pub driver_is_goingto_unload: bool,
    pub up_first_time: bool,
    pub first_init: bool,
    pub being_init_adapter: bool,
    pub mac_func_enable: bool,
    pub pre_edcca_enable: bool,
    pub hal_coex_8723: bt_coexist_8723,
    pub interface: intf_type,
    pub /: *mut *mut u16 hw_type; /92c or 92d or 92s and so on,
    pub ic_class: u8,
    pub oem_id: u8,
    pub /: *mut *mut u32 version; /version of chip,
    pub /: *mut *mut u8 state; /stop 0, start 1,
    pub board_type: u8,
    pub package_type: u8,
    pub pa_type_2g: u8,
    pub pa_type_5g: u8,
    pub lna_type_2g: u8,
    pub lna_type_5g: u8,
    pub external_pa_2g: u8,
    pub external_lna_2g: u8,
    pub external_pa_5g: u8,
    pub external_lna_5g: u8,
    pub type_glna: u8,
    pub type_gpa: u8,
    pub type_alna: u8,
    pub type_apa: u8,
    pub rfe_type: u8,
// firmware
    pub fwsize: u32,
    pub pfirmware: *mut u8,
    pub fw_version: u16,
    pub fw_subversion: u16,
    pub h2c_setinprogress: bool,
    pub last_hmeboxnum: u8,
    pub fw_ready: bool,
// Reserve page start offset except beacon in TxQ.
    pub fw_rsvdpage_startoffset: u8,
    pub h2c_txcmd_seq: u8,
    pub current_ra_rate: u8,
// FW Cmd IO related
    pub fwcmd_iomap: u16,
    pub fwcmd_ioparam: u32,
    pub set_fwcmd_inprogress: bool,
    pub current_fwcmd_io: u8,
    pub p2p_ps_offload: p2p_ps_offload_t,
    pub fw_clk_change_in_progress: bool,
    pub allow_sw_to_change_hwclc: bool,
    pub fw_ps_state: u8,
// AMPDU init min space
    pub /: *mut *mut u8 minspace_cfg; /For Min spacing configurations,
// Dual mac
    pub macphymode: macphy_mode,
    pub /: *mut *mut band_type current_bandtype; / 0:2.4G, 1:5G,
    pub current_bandtypebackup: band_type,
    pub bandset: band_type,
// dual MAC 0--Mac0 1--Mac1
    pub interfaceindex: u32,
// just for DualMac S3S4
    pub macphyctl_reg: u8,
    pub earlymode_enable: bool,
    pub max_earlymode_num: u8,
// Dual mac
    pub during_mac0init_radiob: bool,
    pub during_mac1init_radioa: bool,
    pub reloadtxpowerindex: bool,
// True if IMR or IQK  have done
// for 2.4G in scan progress
//
    pub load_imrandiqk_setting_for2g: bool,
    pub disable_amsdu_8k: bool,
    pub master_of_dmsp: bool,
    pub slave_of_dmsp: bool,
// for wowlan
    pub enter_pnp_sleep: bool,
    pub wake_from_pnp_sleep: bool,
    pub last_suspend_sec: time64_t,
    pub wowlan_fwsize: u32,
    pub wowlan_firmware: *mut u8,
    pub source*/: *mut *mut u8 hw_rof_enable; /Enable GPIO[9] as WL RF HW PDn,
    pub real_wow_v2_enable: bool,
    pub re_init_llt_table: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_security {
// default 0
    pub use_sw_sec: bool,
    pub being_setkey: bool,
    pub use_defaultkey: bool,
// Encryption Algorithm for Unicast Packet
    pub pairwise_enc_algorithm: rt_enc_alg,
// Encryption Algorithm for Brocast/Multicast
    pub group_enc_algorithm: rt_enc_alg,
// Cam Entry Bitmap
    pub hwsec_cam_bitmap: u32,
    pub hwsec_cam_sta_addr: [u8; TOTAL_CAM_ENTRY][ETH_ALEN],
// local Key buffer, indx 0 is for
// pairwise key 1-4 is for agoup key.
//
    pub key_buf: [u8; KEY_BUF_SIZE][MAX_KEY_LEN],
    pub key_len: [u8; KEY_BUF_SIZE],
// The pointer of Pairwise Key,
// it always points to KeyBuf[4]
//
    pub pairwise_key: *mut u8,
}

pub const ASSOCIATE_ENTRY_NUM: c_int = 33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fast_ant_training {
    pub bssid: [u8; 6],
    pub antsel_rx_keep_0: u8,
    pub antsel_rx_keep_1: u8,
    pub antsel_rx_keep_2: u8,
    pub ant_sum: [u32; 7],
    pub ant_cnt: [u32; 7],
    pub ant_ave: [u32; 7],
    pub fat_state: u8,
    pub train_idx: u32,
    pub antsel_a: [u8; ASSOCIATE_ENTRY_NUM],
    pub antsel_b: [u8; ASSOCIATE_ENTRY_NUM],
    pub antsel_c: [u8; ASSOCIATE_ENTRY_NUM],
    pub main_ant_sum: [u32; ASSOCIATE_ENTRY_NUM],
    pub aux_ant_sum: [u32; ASSOCIATE_ENTRY_NUM],
    pub main_ant_cnt: [u32; ASSOCIATE_ENTRY_NUM],
    pub aux_ant_cnt: [u32; ASSOCIATE_ENTRY_NUM],
    pub rx_idle_ant: u8,
    pub becomelinked: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_phy_dbg_info {
    pub rx_snrdb: [i8; 4],
    pub num_qry_phy_status: u64,
    pub num_qry_phy_status_cck: u64,
    pub num_qry_phy_status_ofdm: u64,
    pub num_qry_beacon_pkt: u16,
    pub num_non_be_pkt: u16,
    pub rx_evm: [i32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_dm {
// PHY status for Dynamic Management
    pub entry_min_undec_sm_pwdb: c_long,
    pub undec_sm_cck: c_long,
    pub /: *mut *mut long undec_sm_pwdb; /out dm,
    pub entry_max_undec_sm_pwdb: c_long,
    pub ofdm_pkt_cnt: i32,
    pub dm_initialgain_enable: bool,
    pub dynamic_txpower_enable: bool,
    pub current_turbo_edca: bool,
    pub /: *mut *mut bool is_any_nonbepkts; /out dm,
    pub is_cur_rdlstate: bool,
    pub txpower_trackinginit: bool,
    pub disable_framebursting: bool,
    pub cck_inch14: bool,
    pub txpower_tracking: bool,
    pub useramask: bool,
    pub rfpath_rxenable: [bool; 4],
    pub inform_fw_driverctrldm: bool,
    pub current_mrc_switch: bool,
    pub txpowercount: u8,
    pub powerindex_backup: [u8; 6],
    pub thermalvalue_rxgain: u8,
    pub thermalvalue_iqk: u8,
    pub thermalvalue_lck: u8,
    pub thermalvalue: u8,
    pub last_dtp_lvl: u8,
    pub thermalvalue_avg: [u8; AVG_THERMAL_NUM],
    pub thermalvalue_avg_index: u8,
    pub tm_trigger: u8,
    pub done_txpower: bool,
    pub /: *mut *mut u8 dynamic_txhighpower_lvl; /Tx high power level,
    pub /: *mut *mut u8 dm_flag; /Indicate each dynamic mechanism's status.,
    pub dm_flag_tmp: u8,
    pub dm_type: u8,
    pub dm_rssi_sel: u8,
    pub txpower_track_control: u8,
    pub interrupt_migration: bool,
    pub disable_tx_int: bool,
    pub ofdm_index: [i8; MAX_RF_PATH],
    pub default_ofdm_index: u8,
    pub default_cck_index: u8,
    pub cck_index: i8,
    pub delta_power_index: [i8; MAX_RF_PATH],
    pub delta_power_index_last: [i8; MAX_RF_PATH],
    pub power_index_offset: [i8; MAX_RF_PATH],
    pub absolute_ofdm_swing_idx: [i8; MAX_RF_PATH],
    pub remnant_ofdm_swing_idx: [i8; MAX_RF_PATH],
    pub remnant_cck_idx: i8,
    pub modify_txagc_flag_path_a: bool,
    pub modify_txagc_flag_path_b: bool,
    pub one_entry_only: bool,
    pub dbginfo: dm_phy_dbg_info,
// Dynamic ATC switch
    pub atc_status: bool,
    pub large_cfo_hit: bool,
    pub is_freeze: bool,
    pub cfo_tail: [c_int; 2],
    pub cfo_ave_pre: c_int,
    pub crystal_cap: c_int,
    pub cfo_threshold: u8,
    pub packet_count: u32,
    pub packet_count_pre: u32,
    pub tx_rate: u8,
// 88e tx power tracking
    pub swing_idx_ofdm: [u8; MAX_RF_PATH],
    pub swing_idx_ofdm_cur: u8,
    pub swing_idx_ofdm_base: [u8; MAX_RF_PATH],
    pub swing_flag_ofdm: bool,
    pub swing_idx_cck: u8,
    pub swing_idx_cck_cur: u8,
    pub swing_idx_cck_base: u8,
    pub swing_flag_cck: bool,
    pub swing_diff_2g: i8,
    pub swing_diff_5g: i8,
// DMSP
    pub supp_phymode_switch: bool,
// DulMac
    pub fat_table: fast_ant_training,
    pub resp_tx_path: u8,
    pub path_sel: u8,
    pub patha_sum: u32,
    pub pathb_sum: u32,
    pub patha_cnt: u32,
    pub pathb_cnt: u32,
    pub pre_channel: u8,
    pub p_channel: *mut u8,
    pub linked_interval: u8,
    pub last_tx_ok_cnt: u64,
    pub last_rx_ok_cnt: u64,
}

pub const EFUSE_MAX_LOGICAL_SIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_efuse {
    pub efuse_ops: *const rtl_efuse_ops,
    pub autoload_ok: bool,
    pub bootfromefuse: bool,
    pub max_physical_size: u16,
    pub efuse_map: [u8; 2][EFUSE_MAX_LOGICAL_SIZE],
    pub efuse_usedbytes: u16,
    pub efuse_usedpercentage: u8,
    pub autoload_failflag: u8,
    pub autoload_status: u8,
    pub epromtype: c_short,
    pub eeprom_vid: u16,
    pub eeprom_did: u16,
    pub eeprom_svid: u16,
    pub eeprom_smid: u16,
    pub eeprom_oemid: u8,
    pub eeprom_channelplan: u16,
    pub eeprom_version: u8,
    pub board_type: u8,
    pub external_pa: u8,
    pub dev_addr: [u8; 6],
    pub wowlan_enable: u8,
    pub antenna_div_cfg: u8,
    pub antenna_div_type: u8,
    pub txpwr_fromeprom: bool,
    pub eeprom_crystalcap: u8,
    pub eeprom_tssi: [u8; 2],
    pub /: *mut *mut u8 eeprom_tssi_5g[3][2]; / for 5GL/5GM/5GH band.,
    pub eeprom_pwrlimit_ht20: [u8; CHANNEL_GROUP_MAX],
    pub eeprom_pwrlimit_ht40: [u8; CHANNEL_GROUP_MAX],
    pub eeprom_chnlarea_txpwr_cck: [u8; MAX_RF_PATH][CHANNEL_GROUP_MAX_2G],
    pub eeprom_chnlarea_txpwr_ht40_1s: [u8; MAX_RF_PATH][CHANNEL_GROUP_MAX],
    pub eprom_chnl_txpwr_ht40_2sdf: [u8; MAX_RF_PATH][CHANNEL_GROUP_MAX],
    pub /: *mut *mut u8 internal_pa_5g[2]; / pathA / pathB,
    pub eeprom_c9: u8,
    pub eeprom_cc: u8,
// For power group
    pub eeprom_pwrgroup: [u8; 2][3],
    pub pwrgroup_ht20: [u8; 2][CHANNEL_MAX_NUMBER],
    pub pwrgroup_ht40: [u8; 2][CHANNEL_MAX_NUMBER],
    pub txpwrlevel_cck: [u8; MAX_RF_PATH][CHANNEL_MAX_NUMBER_2G],
// For HT 40MHZ pwr
    pub txpwrlevel_ht40_1s: [u8; MAX_RF_PATH][CHANNEL_MAX_NUMBER],
// For HT 40MHZ pwr
    pub txpwrlevel_ht40_2s: [u8; MAX_RF_PATH][CHANNEL_MAX_NUMBER],
// --------------------------------------------------------
// 8192CE\8192SE\8192DE\8723AE use the following 4 arrays,
// other ICs (8188EE\8723BE\8192EE\8812AE...)
// define new arrays in Windows code.
// BUT, in linux code, we use the same array for all ICs.
//
// The Correspondance relation between two arrays is:
// txpwr_cckdiff[][] == CCK_24G_Diff[][]
// txpwr_ht20diff[][] == BW20_24G_Diff[][]
// txpwr_ht40diff[][] == BW40_24G_Diff[][]
// txpwr_legacyhtdiff[][] == OFDM_24G_Diff[][]
//
// Sizes of these arrays are decided by the larger ones.
//
    pub txpwr_cckdiff: [i8; MAX_RF_PATH][CHANNEL_MAX_NUMBER],
    pub txpwr_ht20diff: [i8; MAX_RF_PATH][CHANNEL_MAX_NUMBER],
    pub txpwr_ht40diff: [i8; MAX_RF_PATH][CHANNEL_MAX_NUMBER],
    pub txpwr_legacyhtdiff: [i8; MAX_RF_PATH][CHANNEL_MAX_NUMBER],
    pub txpwr_5g_bw40base: [u8; MAX_RF_PATH][CHANNEL_MAX_NUMBER],
    pub txpwr_5g_bw80base: [u8; MAX_RF_PATH][CHANNEL_MAX_NUMBER_5G_80M],
    pub txpwr_5g_ofdmdiff: [i8; MAX_RF_PATH][MAX_TX_COUNT],
    pub txpwr_5g_bw20diff: [i8; MAX_RF_PATH][MAX_TX_COUNT],
    pub txpwr_5g_bw40diff: [i8; MAX_RF_PATH][MAX_TX_COUNT],
    pub txpwr_5g_bw80diff: [i8; MAX_RF_PATH][MAX_TX_COUNT],
    pub /: *mut *mut u8 txpwr_safetyflag; / Band edge enable flag,
    pub eeprom_txpowerdiff: u16,
    pub antenna_txpwdiff: [u8; 3],
    pub eeprom_regulatory: u8,
    pub eeprom_thermalmeter: u8,
    pub /: *mut *mut u8 thermalmeter[2]; /ThermalMeter, index 0 for RFIC0, 1 for RFIC1,
    pub tssi_13dbm: u16,
    pub /: *mut *mut u8 crystalcap; / CrystalCap.,
    pub delta_iqk: u8,
    pub delta_lck: u8,
    pub /: *mut *mut u8 legacy_ht_txpowerdiff; /Legacy to HT rate power diff,
    pub apk_thermalmeterignore: bool,
    pub b1x1_recvcombine: bool,
    pub b1ss_support: bool,
// channel plan
    pub channel_plan: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_efuse_ops {
    pub data): *mut *mut *mut int (efuse_onebyte_read)(struct ieee80211_hw hw, u16 addr, u8,
    pub value): *mut u16 offset, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_tx_report {
    pub sn: core::sync::atomic::AtomicI32,
    pub last_sent_sn: u16,
    pub last_sent_time: c_ulong,
    pub last_recv_sn: u16,
    pub queue: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_ps_ctl {
    pub pwrdomain_protect: bool,
    pub in_powersavemode: bool,
    pub rfchange_inprogress: bool,
    pub swrf_processing: bool,
    pub hwradiooff: bool,
// just for PCIE ASPM
// If it supports ASPM, Offset[560h] = 0x40,
// otherwise Offset[560h] = 0x00.
//
    pub support_aspm: bool,
    pub support_backdoor: bool,
// for LPS
    pub /: *mut *mut rt_psmode dot11_psmode; /Power save mode configured.,
    pub swctrl_lps: bool,
    pub leisure_ps: bool,
    pub fwctrl_lps: bool,
    pub fwctrl_psmode: u8,
// For Fw control LPS mode
    pub reg_fwctrl_lps: u8,
// Record Fw PS mode status.
    pub fw_current_inpsmode: bool,
    pub reg_max_lps_awakeintvl: u8,
    pub report_linked: bool,
    pub 32k*/: *mut *mut bool low_power_enable;/for,
// for IPS
    pub inactiveps: bool,
    pub rfoff_reason: u32,
// RF OFF Level
    pub cur_ps_level: u32,
    pub reg_rfps_level: u32,
    pub pwrdown_mode: bool,
    pub inactive_pwrstate: rf_pwrstate,
    pub /: *mut *mut rf_pwrstate rfpwr_state; /cur power state,
// for SW LPS
    pub sw_ps_enabled: bool,
    pub state_inap: bool,
    pub multi_buffered: bool,
    pub nullfunc_seq: u16,
    pub dtim_counter: c_uint,
    pub last_sleep_jiffies: c_ulong,
    pub last_awake_jiffies: c_ulong,
    pub last_delaylps_stamp_jiffies: c_ulong,
    pub last_dtim: c_ulong,
    pub last_beacon: c_ulong,
// For P2P PS
    pub p2p_ps_info: rtl_p2p_ps_info,
    pub pwr_mode: u8,
    pub smart_ps: u8,
// wake up on line
    pub wo_wlan_mode: u8,
    pub arp_offload_enable: u8,
    pub gtk_offload_enable: u8,
// Used for WOL, indicates the reason for waking event.
    pub wakeup_reason: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_stats {
    pub psaddr: [u8; ETH_ALEN],
    pub mac_time: [u32; 2],
    pub rssi: i8,
    pub signal: u8,
    pub noise: u8,
    pub /: *mut *mut u8 rate; / hw desc rate,
    pub received_channel: u8,
    pub control: u8,
    pub mask: u8,
    pub freq: u8,
    pub len: u16,
    pub tsf: u64,
    pub beacon_time: u32,
    pub nic_type: u8,
    pub length: u16,
    pub /: *mut *mut u8 signalquality; /in 0-100 index.,
// Real power in dBm for this packet,
// no beautification and aggregation.
//
    pub recvsignalpower: i32,
    pub /: *mut *mut s8 rxpower; /in dBm Translate from PWdB,
    pub /: *mut *mut u8 signalstrength; /in 0-100 index.,
    pub hwerror:1: u16,
    pub crc:1: u16,
    pub icv:1: u16,
    pub shortpreamble:1: u16,
    pub antenna:1: u16,
    pub decrypted:1: u16,
    pub wakeup:1: u16,
    pub timestamp_low: u32,
    pub timestamp_high: u32,
    pub shift: bool,
    pub rx_drvinfo_size: u8,
    pub rx_bufshift: u8,
    pub isampdu: bool,
    pub isfirst_ampdu: bool,
    pub rx_is40mhzpacket: bool,
    pub rx_packet_bw: u8,
    pub rx_pwdb_all: u32,
    pub /: *mut *mut u8 rx_mimo_signalstrength[4]; /in 0~100 index,
    pub rx_mimo_signalquality: [i8; 4],
    pub rx_mimo_evm_dbm: [u8; 4],
    pub /: *mut *mut u16 cfo_short[4]; / per-path's Cfo_short,
    pub cfo_tail: [u16; 4],
    pub rx_mimo_sig_qual: [i8; 4],
    pub /: *mut *mut u8 rx_pwr[4]; / per-path's pwdb,
    pub /: *mut *mut u8 rx_snr[4]; / per-path's SNR,
    pub bandwidth: u8,
    pub bt_coex_pwr_adjust: u8,
    pub packet_matchbssid: bool,
    pub is_cck: bool,
    pub is_ht: bool,
    pub packet_toself: bool,
    pub /: *mut *mut bool packet_beacon; /for rssi,
    pub /: *mut *mut s8 cck_adc_pwdb[4]; /for rx path selection,
    pub is_vht: bool,
    pub is_short_gi: bool,
    pub vht_nss: u8,
    pub packet_report_type: u8,
    pub macid: u32,
    pub bt_rx_rssi_percentage: u32,
    pub macid_valid_entry: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_link_detect {
// count for roaming
    pub bcn_rx_inperiod: u32,
    pub roam_times: u32,
    pub num_tx_in4period: [u32; 4],
    pub num_rx_in4period: [u32; 4],
    pub num_tx_inperiod: u32,
    pub num_rx_inperiod: u32,
    pub busytraffic: bool,
    pub tx_busy_traffic: bool,
    pub rx_busy_traffic: bool,
    pub higher_busytraffic: bool,
    pub higher_busyrxtraffic: bool,
    pub tidtx_in4period: [u32; MAX_TID_COUNT][4],
    pub tidtx_inperiod: [u32; MAX_TID_COUNT],
    pub higher_busytxtraffic: [bool; MAX_TID_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_tcb_desc {
    pub packet_bw:2: u8,
    pub multicast:1: u8,
    pub broadcast:1: u8,
    pub rts_stbc:1: u8,
    pub rts_enable:1: u8,
    pub cts_enable:1: u8,
    pub rts_use_shortpreamble:1: u8,
    pub rts_use_shortgi:1: u8,
    pub rts_sc:1: u8,
    pub rts_bw:1: u8,
    pub rts_rate: u8,
    pub use_shortgi:1: u8,
    pub use_shortpreamble:1: u8,
    pub use_driver_rate:1: u8,
    pub disable_ratefallback:1: u8,
    pub use_spe_rpt:1: u8,
    pub ratr_index: u8,
    pub mac_id: u8,
    pub hw_rate: u8,
    pub last_inipkt:1: u8,
    pub cmd_or_init:1: u8,
    pub queue_index: u8,
// early mode
    pub empkt_num: u8,
// The max value by HW
    pub empkt_len: [u32; 10],
    pub tx_enable_sw_calc_duration: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_wow_pattern {
    pub type: u8,
    pub crc: u16,
    pub mask: [u32; 4],
}

// struct to store contents of interrupt vectors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_int {
    pub inta: u32,
    pub intb: u32,
    pub intc: u32,
    pub intd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_hal_ops {
    pub hw): *mut *mut int (init_sw_vars)(struct ieee80211_hw,
    pub hw): *mut *mut void (deinit_sw_vars)(struct ieee80211_hw,
    pub hw): *mut *mut void (read_chip_version)(struct ieee80211_hw,
    pub hw): *mut *mut void (read_eeprom_info)(struct ieee80211_hw,
    pub intvec): *mut rtl_int,
    pub hw): *mut *mut int (hw_init)(struct ieee80211_hw,
    pub hw): *mut *mut void (hw_disable)(struct ieee80211_hw,
    pub hw): *mut *mut void (hw_suspend)(struct ieee80211_hw,
    pub hw): *mut *mut void (hw_resume)(struct ieee80211_hw,
    pub hw): *mut *mut void (enable_interrupt)(struct ieee80211_hw,
    pub hw): *mut *mut void (disable_interrupt)(struct ieee80211_hw,
    pub type): nl80211_iftype,
    pub check_bssid): bool,
    pub ch_type): nl80211_channel_type,
    pub hw): *mut *mut u8 (switch_channel)(struct ieee80211_hw,
    pub aci): *mut *mut *mut void (set_qos)(struct ieee80211_hw hw, int,
    pub hw): *mut *mut void (set_bcn_reg)(struct ieee80211_hw,
    pub hw): *mut *mut void (set_bcn_intv)(struct ieee80211_hw,
    pub rm_msr): u32 add_msr, u32,
    pub val): *mut *mut *mut void (get_hw_reg)(struct ieee80211_hw hw, u8 variable, u8,
    pub val): *mut *mut *mut void (set_hw_reg)(struct ieee80211_hw hw, u8 variable, u8,
    pub update_bw): bool,
    pub rssi_level): *mut *mut *mut void (update_rate_mask)(struct ieee80211_hw hw, u8,
    pub queue_index): u8,
    pub queue_index): u8,
    pub ptcb_desc): *mut rtl_tcb_desc,
    pub skb): *mut sk_buff,
    pub hw_queue): *mut *mut sk_buff skb, u8,
    pub skb): *mut *mut u8 pdesc, struct sk_buff,
    pub hw): *mut *mut void (set_channel_access)(struct ieee80211_hw,
    pub valid): *mut *mut *mut bool (radio_onoff_checking)(struct ieee80211_hw hw, u8,
    pub hw): *mut *mut void (dm_watchdog)(struct ieee80211_hw,
    pub operation): *mut *mut *mut void (scan_operation_backup)(struct ieee80211_hw hw, u8,
    pub rfpwr_state): rf_pwrstate,
    pub ledaction): led_ctl_mode,
    pub val): *mut u8 desc_name, u8,
    pub desc_name): u8,
    pub index): u8 hw_queue, u16,
    pub hw_queue): *mut *mut *mut void (tx_polling)(struct ieee80211_hw hw, u8,
    pub hw): *mut *mut void (enable_hw_sec)(struct ieee80211_hw,
    pub clear_all): bool is_wepkey, bool,
    pub bitmask): *mut *mut *mut u32 (get_bbreg)(struct ieee80211_hw hw, u32 regaddr, u32,
    pub data): u32,
    pub bitmask): u32 regaddr, u32,
    pub data): u32 regaddr, u32 bitmask, u32,
    pub hw): *mut *mut void (linked_set_reg)(struct ieee80211_hw,
    pub hw): *mut *mut void (dualmac_switch_to_dmdp)(struct ieee80211_hw,
    pub hw): *mut *mut bool (phy_rf6052_config)(struct ieee80211_hw,
    pub powerlevel): *mut u8,
    pub channel): *mut *mut u8 ppowerlevel, u8,
    pub configtype): u8,
    pub configtype): u8,
    pub is2t): *mut *mut *mut void (phy_lc_calibrate)(struct ieee80211_hw hw, bool,
    pub hw): *mut *mut void (phy_iq_calibrate)(struct ieee80211_hw,
    pub hw): *mut *mut void (phy_set_bw_mode_callback)(struct ieee80211_hw,
    pub hw): *mut *mut void (dm_dynamic_txpower)(struct ieee80211_hw,
    pub hw): *mut *mut void (c2h_command_handle)(struct ieee80211_hw,
    pub mstate): bool,
    pub hw): *mut *mut void (bt_coex_off_before_lps)(struct ieee80211_hw,
    pub p_cmdbuffer): *mut u32 cmd_len, u8,
    pub hw): *mut *mut void (set_default_port_id_cmd)(struct ieee80211_hw,
    pub (*get_btc_status)(void): *mut bool,
    pub hdr): *mut *mut bool (is_fw_header)(struct rtlwifi_firmware_header,
    pub index): u8,
    pub q_idx): *mut *mut *mut u16 (get_available_desc)(struct ieee80211_hw hw, u8,
    pub cmd_len): *mut *mut u8 cmd_buf, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_intf_ops {
// com
    pub hw): *mut *mut int (adapter_start)(struct ieee80211_hw,
    pub hw): *mut *mut void (adapter_stop)(struct ieee80211_hw,
    pub ptcb_desc): *mut rtl_tcb_desc,
    pub drop): *mut *mut *mut void (flush)(struct ieee80211_hw hw, u32 queues, bool,
    pub hw): *mut *mut int (reset_trx_ring)(struct ieee80211_hw,
    pub skb): *mut sk_buff,
// pci
    pub hw): *mut *mut void (disable_aspm)(struct ieee80211_hw,
    pub hw): *mut *mut void (enable_aspm)(struct ieee80211_hw,
// usb
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_mod_params {
// default: 0,0
    pub debug_mask: u64,
// default: 0 = using hardware encryption
    pub sw_crypto: bool,
// default: 0 = DBG_EMERG (0)
    pub debug_level: c_int,
// default: 1 = using no linked power save
    pub inactiveps: bool,
// default: 1 = using linked sw power save
    pub swctrl_lps: bool,
// default: 1 = using linked fw power save
    pub fwctrl_lps: bool,
// default: 0 = not using MSI interrupts mode
// submodules should set their own default value
//
    pub msi_support: bool,
// default: 0 = dma 32
    pub dma64: bool,
// default: 1 = enable aspm
    pub aspm_support: c_int,
// default 0: 1 means disable
    pub disable_watchdog: bool,
// default 0: 1 means do not disable interrupts
    pub int_clear: bool,
// select antenna
    pub ant_sel: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_hal_usbint_cfg {
// data - rx
    pub rx_urb_num: u32,
    pub rx_max_size: u32,
// op - rx
    pub ): *mut *mut *mut void (usb_rx_hdl)(struct ieee80211_hw , struct sk_buff,
    pub ): *mut sk_buff_head,
// tx
    pub ): *mut *mut *mut void (usb_tx_cleanup)(struct ieee80211_hw , struct sk_buff,
    pub ): *mut sk_buff,
    pub ): *mut sk_buff_head,
// endpoint mapping
    pub hw): *mut *mut int (usb_endpoint_mapping)(struct ieee80211_hw,
    pub mac80211_queue_index): *mut *mut u16 (usb_mq_to_hwq)(__le16 fc, u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_hal_cfg {
    pub bar_id: u8,
    pub write_readback: bool,
    pub name: *mut c_char,
    pub alt_fw_name: *mut c_char,
    pub ops: *const rtl_hal_ops,
    pub mod_params: *mut rtl_mod_params,
    pub usb_interface_cfg: *const rtl_hal_usbint_cfg,
    pub spec_ver: rtl_spec_ver,
// this map used for some registers or vars
// defined int HAL but used in MAIN
//
    pub maps: [u32; RTL_VAR_MAP_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_locks {
// mutex
    pub conf_mutex: mutex,
    pub /: *mut *mut mutex ips_mutex; / mutex for enter/leave IPS,
    pub /: *mut *mut mutex lps_mutex; / mutex for enter/leave LPS,
// spin lock
    pub irq_th_lock: spinlock_t,
    pub h2c_lock: spinlock_t,
    pub rf_ps_lock: spinlock_t,
    pub rf_lock: spinlock_t,
    pub waitq_lock: spinlock_t,
    pub entry_list_lock: spinlock_t,
    pub usb_lock: spinlock_t,
    pub /: *mut *mut spinlock_t scan_list_lock; / lock for the scan list,
// FW clock change
    pub fw_ps_lock: spinlock_t,
// Dual mac
    pub cck_and_rw_pagea_lock: spinlock_t,
    pub iqk_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_works {
    pub hw: *mut ieee80211_hw,
// timer
    pub watchdog_timer: timer_list,
    pub fw_clockoff_timer: timer_list,
    pub fast_antenna_training_timer: timer_list,
// task
    pub irq_tasklet: tasklet_struct,
    pub irq_prepare_bcn_tasklet: tasklet_struct,
// work queue
    pub rtl_wq: *mut workqueue_struct,
    pub watchdog_wq: delayed_work,
    pub ips_nic_off_wq: delayed_work,
    pub c2hcmd_wq: delayed_work,
// For SW LPS
    pub ps_work: delayed_work,
    pub ps_rfon_wq: delayed_work,
    pub fwevt_wq: delayed_work,
    pub lps_change_work: work_struct,
    pub fill_h2c_cmd: work_struct,
    pub update_beacon_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_debug {
// add for debug
    pub debugfs_dir: *mut dentry,
    pub debugfs_name: [c_char; 20],
}

pub const MIMO_PS_STATIC: c_int = 0;
pub const MIMO_PS_DYNAMIC: c_int = 1;
pub const MIMO_PS_NOLIMIT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_dmsp_ctl {
    pub activescan_for_slaveofdmsp: bool,
    pub scan_for_anothermac_fordmsp: bool,
    pub scan_for_itself_fordmsp: bool,
    pub writedig_for_anothermacofdmsp: bool,
    pub curdigvalue_for_anothermacofdmsp: u32,
    pub changecckpdstate_for_anothermacofdmsp: bool,
    pub curcckpdstate_for_anothermacofdmsp: u8,
    pub changetxhighpowerlvl_for_anothermacofdmsp: bool,
    pub curtxhighlvl_for_anothermacofdmsp: u8,
    pub rssivalmin_for_anothermacofdmsp: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps_t {
    pub pre_ccastate: u8,
    pub cur_ccasate: u8,
    pub pre_rfstate: u8,
    pub cur_rfstate: u8,
    pub initialize: u8,
    pub rssi_val_min: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dig_t {
    pub rssi_lowthresh: u32,
    pub rssi_highthresh: u32,
    pub fa_lowthresh: u32,
    pub fa_highthresh: u32,
    pub last_min_undec_pwdb_for_dm: c_long,
    pub rssi_highpower_lowthresh: c_long,
    pub rssi_highpower_highthresh: c_long,
    pub recover_cnt: u32,
    pub pre_igvalue: u32,
    pub cur_igvalue: u32,
    pub rssi_val: c_long,
    pub dig_enable_flag: u8,
    pub dig_ext_port_stage: u8,
    pub dig_algorithm: u8,
    pub dig_twoport_algorithm: u8,
    pub dig_dbgmode: u8,
    pub dig_slgorithm_switch: u8,
    pub cursta_cstate: u8,
    pub presta_cstate: u8,
    pub curmultista_cstate: u8,
    pub stop_dig: u8,
    pub back_val: i8,
    pub back_range_max: i8,
    pub back_range_min: i8,
    pub rx_gain_max: u8,
    pub rx_gain_min: u8,
    pub min_undec_pwdb_for_dm: u8,
    pub rssi_val_min: u8,
    pub pre_cck_cca_thres: u8,
    pub cur_cck_cca_thres: u8,
    pub pre_cck_pd_state: u8,
    pub cur_cck_pd_state: u8,
    pub pre_cck_fa_state: u8,
    pub cur_cck_fa_state: u8,
    pub pre_ccastate: u8,
    pub cur_ccasate: u8,
    pub large_fa_hit: u8,
    pub forbidden_igi: u8,
    pub dig_state: u8,
    pub dig_highpwrstate: u8,
    pub cur_sta_cstate: u8,
    pub pre_sta_cstate: u8,
    pub cur_ap_cstate: u8,
    pub pre_ap_cstate: u8,
    pub cur_pd_thstate: u8,
    pub pre_pd_thstate: u8,
    pub cur_cs_ratiostate: u8,
    pub pre_cs_ratiostate: u8,
    pub backoff_enable_flag: u8,
    pub backoffval_range_max: i8,
    pub backoffval_range_min: i8,
    pub dig_min_0: u8,
    pub dig_min_1: u8,
    pub bt30_cur_igi: u8,
    pub media_connect_0: bool,
    pub media_connect_1: bool,
    pub antdiv_rssi_max: u32,
    pub rssi_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_btc_info {
    pub bt_type: u8,
    pub btcoexist: u8,
    pub ant_num: u8,
    pub single_ant_path: u8,
    pub ap_num: u8,
    pub in_4way: bool,
    pub in_4way_ts: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_coexist_info {
    pub btc_ops: *mut rtl_btc_ops,
    pub btc_info: rtl_btc_info,
// btc context
    pub btc_context: *mut c_void,
    pub wifi_only_context: *mut c_void,
// EEPROM BT info.
    pub eeprom_bt_coexist: u8,
    pub eeprom_bt_type: u8,
    pub eeprom_bt_ant_num: u8,
    pub eeprom_bt_ant_isol: u8,
    pub eeprom_bt_radio_shared: u8,
    pub bt_coexistence: u8,
    pub bt_ant_num: u8,
    pub bt_coexist_type: u8,
    pub bt_state: u8,
    pub /: *mut *mut u8 bt_cur_state; / 0:on, 1:off,
    pub /: *mut *mut u8 bt_ant_isolation; / 0:good, 1:bad,
    pub /: *mut *mut u8 bt_pape_ctrl; / 0:SW, 1:SW/HW dynamic,
    pub bt_service: u8,
    pub bt_radio_shared_type: u8,
    pub bt_rfreg_origin_1e: u8,
    pub bt_rfreg_origin_1f: u8,
    pub bt_rssi_state: u8,
    pub ratio_tx: u32,
    pub ratio_pri: u32,
    pub bt_edca_ul: u32,
    pub bt_edca_dl: u32,
    pub init_set: bool,
    pub bt_busy_traffic: bool,
    pub bt_traffic_mode_set: bool,
    pub bt_non_traffic_mode_set: bool,
    pub fw_coexist_all_off: bool,
    pub sw_coexist_all_off: bool,
    pub hw_coexist_all_off: bool,
    pub cstate: u32,
    pub previous_state: u32,
    pub cstate_h: u32,
    pub previous_state_h: u32,
    pub bt_pre_rssi_state: u8,
    pub bt_pre_rssi_state1: u8,
    pub reg_bt_iso: u8,
    pub reg_bt_sco: u8,
    pub balance_on: bool,
    pub bt_active_zero_cnt: u8,
    pub cur_bt_disabled: bool,
    pub pre_bt_disabled: bool,
    pub bt_profile_case: u8,
    pub bt_profile_action: u8,
    pub bt_busy: bool,
    pub hold_for_bt_operation: bool,
    pub lps_counter: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_btc_ops {
    pub rtlpriv): *mut *mut void (btc_init_variables)(struct rtl_priv,
    pub rtlpriv): *mut *mut void (btc_init_variables_wifi_only)(struct rtl_priv,
    pub rtlpriv): *mut *mut void (btc_deinit_variables)(struct rtl_priv,
    pub rtlpriv): *mut *mut void (btc_init_hal_vars)(struct rtl_priv,
    pub rtlpriv): *mut *mut void (btc_power_on_setting)(struct rtl_priv,
    pub rtlpriv): *mut *mut void (btc_init_hw_config)(struct rtl_priv,
    pub rtlpriv): *mut *mut void (btc_init_hw_config_wifi_only)(struct rtl_priv,
    pub type): *mut *mut *mut void (btc_ips_notify)(struct rtl_priv rtlpriv, u8,
    pub type): *mut *mut *mut void (btc_lps_notify)(struct rtl_priv rtlpriv, u8,
    pub scantype): *mut *mut *mut void (btc_scan_notify)(struct rtl_priv rtlpriv, u8,
    pub scantype): u8,
    pub action): *mut *mut *mut void (btc_connect_notify)(struct rtl_priv rtlpriv, u8,
    pub mstatus): rt_media_status,
    pub rtlpriv): *mut *mut void (btc_periodical)(struct rtl_priv,
    pub rtlpriv): *mut *mut void (btc_halt_notify)(struct rtl_priv,
    pub length): *mut *mut u8 tmp_buf, u8,
    pub length): *mut *mut u8 tmp_buf, u8,
    pub rtlpriv): *mut *mut bool (btc_is_limited_dig)(struct rtl_priv,
    pub rtlpriv): *mut *mut bool (btc_is_disable_edca_turbo)(struct rtl_priv,
    pub rtlpriv): *mut *mut bool (btc_is_bt_disabled)(struct rtl_priv,
    pub pkt_type): u8,
    pub scanning): bool,
    pub scanning): u8 type, bool,
    pub m): *mut seq_file,
    pub len): *mut *mut *mut *mut void (btc_record_pwr_mode)(struct rtl_priv rtlpriv, u8 buf, u8,
    pub rtlpriv): *mut *mut u8 (btc_get_lps_val)(struct rtl_priv,
    pub rtlpriv): *mut *mut u8 (btc_get_rpwm_val)(struct rtl_priv,
    pub rtlpriv): *mut *mut bool (btc_is_bt_ctrl_lps)(struct rtl_priv,
    pub agg_size): *mut *mut u8 ctrl_agg_size, u8,
    pub rtlpriv): *mut *mut bool (btc_is_bt_lps_on)(struct rtl_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proxim {
    pub proxim_on: bool,
    pub proximity_priv: *mut c_void,
    pub skb): *mut sk_buff,
    pub type): *mut *mut *mut u8 (proxim_get_var)(struct ieee80211_hw hw, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_c2hcmd {
    pub list: list_head,
    pub tag: u8,
    pub len: u8,
    pub val: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_bssid_entry {
    pub list: list_head,
    pub bssid: [u8; ETH_ALEN],
    pub age: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_scan_list {
    pub num: c_int,
    pub /: *mut *mut list_head list; / sort by age,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_priv {
    pub hw: *mut ieee80211_hw,
    pub firmware_loading_complete: completion,
    pub buddy_priv: *mut rtl_priv,
    pub dmsp_ctl: rtl_dmsp_ctl,
    pub locks: rtl_locks,
    pub works: rtl_works,
    pub mac80211: rtl_mac,
    pub rtlhal: rtl_hal,
    pub regd: rtl_regulatory,
    pub rfkill: rtl_rfkill,
    pub io: rtl_io,
    pub phy: rtl_phy,
    pub dm: rtl_dm,
    pub sec: rtl_security,
    pub efuse: rtl_efuse,
    pub ledctl: rtl_led_ctl,
    pub tx_report: rtl_tx_report,
    pub scan_list: rtl_scan_list,
    pub psc: rtl_ps_ctl,
    pub ra: rate_adaptive,
    pub primarycca: dynamic_primary_cca,
    pub stats: wireless_stats,
    pub link_info: rt_link_detect,
    pub falsealm_cnt: false_alarm_statistics,
    pub rate_priv: *mut rtl_rate_priv,
// sta entry list for ap adhoc or mesh
    pub entry_list: list_head,
// c2hcmd list for kthread level access
    pub c2hcmd_queue: sk_buff_head,
    pub dbg: rtl_debug,
    pub max_fw_size: c_int,
// hal_cfg : for diff cards
// intf_ops : for diff interrface usb/pcie
//
    pub cfg: *const rtl_hal_cfg,
    pub intf_ops: *const rtl_intf_ops,
// this var will be set by set_bit,
// and was used to indicate status of
// interface or hardware
//
    pub status: c_ulong,
// tables for dm
    pub dm_digtable: dig_t,
    pub dm_pstable: ps_t,
    pub reg_874: u32,
    pub reg_c70: u32,
    pub reg_85c: u32,
    pub reg_a74: u32,
    pub /: *mut *mut bool reg_init; / true if regs saved,
    pub bt_operation_on: bool,
    pub usb_data: *mut __le32,
    pub usb_data_index: c_int,
    pub initialized: bool,
    pub /: *mut *mut bool enter_ps; / true when entering PS,
    pub rate_mask: [u8; 5],
// intel Proximity, should be alloc mem
// in intel Proximity module and can only
// be used in intel Proximity mode
//
    pub proximity: proxim,
// for bt coexist use
    pub btcoexist: bt_coexist_info,
// separate 92ee from other ICs,
// 92ee use new trx flow.
//
    pub use_new_trx_flow: bool,
// For dual MAC RTL8192DU, things shared by the 2 USB interfaces
    pub curveindex_2g: *mut u32,
    pub curveindex_5g: *mut u32,
    pub /: *mut *mut *mut mutex mutex_for_power_on_off; / for power on/off,
    pub /: *mut *mut *mut mutex mutex_for_hw_init; / for hardware init,

    pub wowlan: wiphy_wowlan_support,

// This must be the last item so
// that it points to the data allocated
// beyond  this structure like:
// rtl_pci_priv or rtl_usb_priv
//
    pub )): *mut u8 priv[] __aligned(sizeof(void,
}

// Bluetooth Co-existence Related
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_ant_num {
    ANT_X2 = 0,
    ANT_X1 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_ant_path {
    ANT_MAIN = 0,
    ANT_AUX = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_co_type {
    BT_2WIRE = 0,
    BT_ISSC_3WIRE = 1,
    BT_ACCEL = 2,
    BT_CSR_BC4 = 3,
    BT_CSR_BC8 = 4,
    BT_RTL8756 = 5,
    BT_RTL8723A = 6,
    BT_RTL8821A = 7,
    BT_RTL8723B = 8,
    BT_RTL8192E = 9,
    BT_RTL8812A = 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_cur_state {
    BT_OFF = 0,
    BT_ON = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_service_type {
    BT_SCO = 0,
    BT_A2DP = 1,
    BT_HID = 2,
    BT_HID_IDLE = 3,
    BT_SCAN = 4,
    BT_IDLE = 5,
    BT_OTHER_ACTION = 6,
    BT_BUSY = 7,
    BT_OTHERBUSY = 8,
    BT_PAN = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_radio_shared {
    BT_RADIO_SHARED = 0,
    BT_RADIO_INDIVIDUAL = 1,
}

//
// mem access macro define start
// Call endian free function when
// 1. Read/write packet content.
// 2. Before write integer to IO.
// 3. After read integer from IO.
//

// mem access macro define end

pub const RTL_WATCH_DOG_TIME: c_int = 2000;

// NIC halt, re-initialize hw parameters

// Always enable ASPM and Clock Req in initialization.

// no matter RFOFF or SLEEP we set PS_ASPM_LEVL

// When LPS is on, disable 2R if no packet is received or transmittd.

extern "C" {
    pub fn ieee80211_get_tid(_arg: rtl_get_hdr(skb)) -> return;
}
extern "C" {
    pub fn ieee80211_find_sta(_arg: vif, _arg: bssid) -> return;
}
extern "C" {
    pub fn ieee80211_find_sta(_arg: mac->vif, _arg: mac_addr) -> return;
}
extern "C" {
    pub fn __ffs(_arg: bitmask) -> return;
}
