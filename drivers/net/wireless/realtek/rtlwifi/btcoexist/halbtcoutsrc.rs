//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/btcoexist/halbtcoutsrc.h
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

pub const BTC_RF_OFF: c_uint = 0x0;
pub const BTC_RF_ON: c_uint = 0x1;

pub const BTC_MP_UNKNOWN: c_uint = 0xff;
// Macro flag: #define		IN
// Macro flag: #define		OUT
pub const BT_TMP_BUF_SIZE: c_int = 100;
pub const BT_COEX_ANT_TYPE_PG: c_int = 0;
pub const BT_COEX_ANT_TYPE_ANTDIV: c_int = 1;
pub const BT_COEX_ANT_TYPE_DETECTED: c_int = 2;
pub const BTC_MIMO_PS_STATIC: c_int = 0;
pub const BTC_MIMO_PS_DYNAMIC: c_int = 1;
pub const BTC_RATE_DISABLE: c_int = 0;
pub const BTC_RATE_ENABLE: c_int = 1;
// single Antenna definition
pub const BTC_ANT_PATH_WIFI: c_int = 0;
pub const BTC_ANT_PATH_BT: c_int = 1;
pub const BTC_ANT_PATH_PTA: c_int = 2;
pub const BTC_ANT_PATH_WIFI5G: c_int = 3;
pub const BTC_ANT_PATH_AUTO: c_int = 4;
// dual Antenna definition
pub const BTC_ANT_WIFI_AT_MAIN: c_int = 0;
pub const BTC_ANT_WIFI_AT_AUX: c_int = 1;
// coupler Antenna definition
pub const BTC_ANT_WIFI_AT_CPL_MAIN: c_int = 0;
pub const BTC_ANT_WIFI_AT_CPL_AUX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_bt_reg_type {
    BTC_BT_REG_RF		= 0,
    BTC_BT_REG_MODEM	= 1,
    BTC_BT_REG_BLUEWIZE	= 2,
    BTC_BT_REG_VENDOR	= 3,
    BTC_BT_REG_LE		= 4,
    BTC_BT_REG_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_chip_interface {
    BTC_INTF_UNKNOWN	= 0,
    BTC_INTF_PCI		= 1,
    BTC_INTF_USB		= 2,
    BTC_INTF_SDIO		= 3,
    BTC_INTF_GSPI		= 4,
    BTC_INTF_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_chip_type {
    BTC_CHIP_UNDEF		= 0,
    BTC_CHIP_CSR_BC4	= 1,
    BTC_CHIP_CSR_BC8	= 2,
    BTC_CHIP_RTL8723A	= 3,
    BTC_CHIP_RTL8821	= 4,
    BTC_CHIP_RTL8723B	= 5,
    BTC_CHIP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_msg_type {
    BTC_MSG_INTERFACE	= 0x0,
    BTC_MSG_ALGORITHM	= 0x1,
    BTC_MSG_MAX
}

// following is for BTC_MSG_INTERFACE

// following is for BTC_ALGORITHM

// following is for wifi link status

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_power_save_type {
    BTC_PS_WIFI_NATIVE = 0,
    BTC_PS_LPS_ON = 1,
    BTC_PS_LPS_OFF = 2,
    BTC_PS_LPS_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btc_board_info {
// The following is some board information
    pub bt_chip_type: u8,
    pub /: *mut *mut u8 pg_ant_num; / pg ant number,
    pub /: *mut *mut u8 btdm_ant_num; / ant number for btdm,
    pub btdm_ant_pos: u8,
    pub /: *mut *mut u8 single_ant_path; / current used for 8723b only, 1=>s0, 0=>s1,
    pub tfbga_package: bool,
    pub rfe_type: u8,
    pub ant_div_cfg: u8,
    pub customer_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_dbg_opcode {
    BTC_DBG_SET_COEX_NORMAL = 0x0,
    BTC_DBG_SET_COEX_WIFI_ONLY = 0x1,
    BTC_DBG_SET_COEX_BT_ONLY = 0x2,
    BTC_DBG_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_rssi_state {
    BTC_RSSI_STATE_HIGH = 0x0,
    BTC_RSSI_STATE_MEDIUM = 0x1,
    BTC_RSSI_STATE_LOW = 0x2,
    BTC_RSSI_STATE_STAY_HIGH = 0x3,
    BTC_RSSI_STATE_STAY_MEDIUM = 0x4,
    BTC_RSSI_STATE_STAY_LOW = 0x5,
    BTC_RSSI_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wifi_role {
    BTC_ROLE_STATION = 0x0,
    BTC_ROLE_AP = 0x1,
    BTC_ROLE_IBSS = 0x2,
    BTC_ROLE_HS_MODE = 0x3,
    BTC_ROLE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wireless_freq {
    BTC_FREQ_2_4G = 0x0,
    BTC_FREQ_5G = 0x1,
    BTC_FREQ_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wifi_bw_mode {
    BTC_WIFI_BW_LEGACY = 0x0,
    BTC_WIFI_BW_HT20 = 0x1,
    BTC_WIFI_BW_HT40 = 0x2,
    BTC_WIFI_BW_HT80 = 0x3,
    BTC_WIFI_BW_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wifi_traffic_dir {
    BTC_WIFI_TRAFFIC_TX = 0x0,
    BTC_WIFI_TRAFFIC_RX = 0x1,
    BTC_WIFI_TRAFFIC_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_wifi_pnp {
    BTC_WIFI_PNP_WAKE_UP = 0x0,
    BTC_WIFI_PNP_SLEEP = 0x1,
    BTC_WIFI_PNP_SLEEP_KEEP_ANT = 0x2,
    BTC_WIFI_PNP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_iot_peer {
    BTC_IOT_PEER_UNKNOWN = 0,
    BTC_IOT_PEER_REALTEK = 1,
    BTC_IOT_PEER_REALTEK_92SE = 2,
    BTC_IOT_PEER_BROADCOM = 3,
    BTC_IOT_PEER_RALINK = 4,
    BTC_IOT_PEER_ATHEROS = 5,
    BTC_IOT_PEER_CISCO = 6,
    BTC_IOT_PEER_MERU = 7,
    BTC_IOT_PEER_MARVELL = 8,
    BTC_IOT_PEER_REALTEK_SOFTAP = 9,
    BTC_IOT_PEER_SELF_SOFTAP = 10, /* Self is SoftAP */
    BTC_IOT_PEER_AIRGO = 11,
    BTC_IOT_PEER_REALTEK_JAGUAR_BCUTAP = 12,
    BTC_IOT_PEER_REALTEK_JAGUAR_CCUTAP = 13,
    BTC_IOT_PEER_MAX,
}

// for 8723b-d cut large current issue
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_wifi_coex_state {
    BTC_WIFI_STAT_INIT,
    BTC_WIFI_STAT_IQK,
    BTC_WIFI_STAT_NORMAL_OFF,
    BTC_WIFI_STAT_MP_OFF,
    BTC_WIFI_STAT_NORMAL,
    BTC_WIFI_STAT_ANT_DIV,
    BTC_WIFI_STAT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_ant_type {
    BTC_ANT_TYPE_0,
    BTC_ANT_TYPE_1,
    BTC_ANT_TYPE_2,
    BTC_ANT_TYPE_3,
    BTC_ANT_TYPE_4,
    BTC_ANT_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_get_type {
// type bool
    BTC_GET_BL_HS_OPERATION,
    BTC_GET_BL_HS_CONNECTING,
    BTC_GET_BL_WIFI_CONNECTED,
    BTC_GET_BL_WIFI_DUAL_BAND_CONNECTED,
    BTC_GET_BL_WIFI_BUSY,
    BTC_GET_BL_WIFI_SCAN,
    BTC_GET_BL_WIFI_LINK,
    BTC_GET_BL_WIFI_DHCP,
    BTC_GET_BL_WIFI_SOFTAP_IDLE,
    BTC_GET_BL_WIFI_SOFTAP_LINKING,
    BTC_GET_BL_WIFI_IN_EARLY_SUSPEND,
    BTC_GET_BL_WIFI_ROAM,
    BTC_GET_BL_WIFI_4_WAY_PROGRESS,
    BTC_GET_BL_WIFI_UNDER_5G,
    BTC_GET_BL_WIFI_AP_MODE_ENABLE,
    BTC_GET_BL_WIFI_ENABLE_ENCRYPTION,
    BTC_GET_BL_WIFI_UNDER_B_MODE,
    BTC_GET_BL_EXT_SWITCH,
    BTC_GET_BL_WIFI_IS_IN_MP_MODE,
    BTC_GET_BL_IS_ASUS_8723B,
    BTC_GET_BL_FW_READY,
    BTC_GET_BL_RF4CE_CONNECTED,

// type s4Byte
    BTC_GET_S4_WIFI_RSSI,
    BTC_GET_S4_HS_RSSI,

// type u32
    BTC_GET_U4_WIFI_BW,
    BTC_GET_U4_WIFI_TRAFFIC_DIRECTION,
    BTC_GET_U4_WIFI_FW_VER,
    BTC_GET_U4_WIFI_LINK_STATUS,
    BTC_GET_U4_BT_PATCH_VER,
    BTC_GET_U4_VENDOR,
    BTC_GET_U4_SUPPORTED_VERSION,
    BTC_GET_U4_SUPPORTED_FEATURE,
    BTC_GET_U4_BT_DEVICE_INFO,
    BTC_GET_U4_BT_FORBIDDEN_SLOT_VAL,
    BTC_GET_U4_WIFI_IQK_TOTAL,
    BTC_GET_U4_WIFI_IQK_OK,
    BTC_GET_U4_WIFI_IQK_FAIL,

// type u1Byte
    BTC_GET_U1_WIFI_DOT11_CHNL,
    BTC_GET_U1_WIFI_CENTRAL_CHNL,
    BTC_GET_U1_WIFI_HS_CHNL,
    BTC_GET_U1_MAC_PHY_MODE,
    BTC_GET_U1_AP_NUM,
    BTC_GET_U1_ANT_TYPE,
    BTC_GET_U1_IOT_PEER,

// for 1Ant
    BTC_GET_U1_LPS_MODE,
    BTC_GET_BL_BT_SCO_BUSY,

// for test mode
    BTC_GET_DRIVER_TEST_CFG,
    BTC_GET_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_vendor {
    BTC_VENDOR_LENOVO,
    BTC_VENDOR_ASUS,
    BTC_VENDOR_OTHER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_set_type {
// type bool
    BTC_SET_BL_BT_DISABLE,
    BTC_SET_BL_BT_TRAFFIC_BUSY,
    BTC_SET_BL_BT_LIMITED_DIG,
    BTC_SET_BL_FORCE_TO_ROAM,
    BTC_SET_BL_TO_REJ_AP_AGG_PKT,
    BTC_SET_BL_BT_CTRL_AGG_SIZE,
    BTC_SET_BL_INC_SCAN_DEV_NUM,
    BTC_SET_BL_BT_TX_RX_MASK,
    BTC_SET_BL_MIRACAST_PLUS_BT,

// type u1Byte
    BTC_SET_U1_RSSI_ADJ_VAL_FOR_AGC_TABLE_ON,
    BTC_SET_UI_SCAN_SIG_COMPENSATION,
    BTC_SET_U1_AGG_BUF_SIZE,

// type trigger some action
    BTC_SET_ACT_GET_BT_RSSI,
    BTC_SET_ACT_AGGREGATE_CTRL,
    BTC_SET_ACT_ANTPOSREGRISTRY_CTRL,
    BTC_SET_MIMO_PS_MODE,

// for 1Ant
// type bool
    BTC_SET_BL_BT_SCO_BUSY,
// type u1Byte
    BTC_SET_U1_RSSI_ADJ_VAL_FOR_1ANT_COEX_TYPE,
    BTC_SET_U1_LPS_VAL,
    BTC_SET_U1_RPWM_VAL,
    BTC_SET_U1_1ANT_LPS,
    BTC_SET_U1_1ANT_RPWM,
// type trigger some action
    BTC_SET_ACT_LEAVE_LPS,
    BTC_SET_ACT_ENTER_LPS,
    BTC_SET_ACT_NORMAL_LPS,
    BTC_SET_ACT_PRE_NORMAL_LPS,
    BTC_SET_ACT_POST_NORMAL_LPS,
    BTC_SET_ACT_INC_FORCE_EXEC_PWR_CMD_CNT,
    BTC_SET_ACT_DISABLE_LOW_POWER,
    BTC_SET_BL_BT_LNA_CONSTRAIN_LEVEL,
    BTC_SET_ACT_UPDATE_RAMASK,
    BTC_SET_ACT_SEND_MIMO_PS,
// BT Coex related
    BTC_SET_ACT_CTRL_BT_INFO,
    BTC_SET_ACT_CTRL_BT_COEX,
    BTC_SET_ACT_CTRL_8723B_ANT,
//
    BTC_SET_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_dbg_disp_type {
    BTC_DBG_DISP_COEX_STATISTICS = 0x0,
    BTC_DBG_DISP_BT_LINK_INFO = 0x1,
    BTC_DBG_DISP_BT_FW_VER = 0x2,
    BTC_DBG_DISP_FW_PWR_MODE_CMD = 0x3,
    BTC_DBG_DISP_WIFI_STATUS = 0x04,
    BTC_DBG_DISP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_notify_type_ips {
    BTC_IPS_LEAVE = 0x0,
    BTC_IPS_ENTER = 0x1,
    BTC_IPS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_notify_type_lps {
    BTC_LPS_DISABLE = 0x0,
    BTC_LPS_ENABLE = 0x1,
    BTC_LPS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_notify_type_scan {
    BTC_SCAN_FINISH = 0x0,
    BTC_SCAN_START = 0x1,
    BTC_SCAN_START_2G = 0x2,
    BTC_SCAN_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_notify_type_switchband {
    BTC_NOT_SWITCH = 0x0,
    BTC_SWITCH_TO_24G = 0x1,
    BTC_SWITCH_TO_5G = 0x2,
    BTC_SWITCH_TO_24G_NOFORSCAN = 0x3,
    BTC_SWITCH_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_notify_type_associate {
    BTC_ASSOCIATE_FINISH = 0x0,
    BTC_ASSOCIATE_START = 0x1,
    BTC_ASSOCIATE_5G_FINISH = 0x2,
    BTC_ASSOCIATE_5G_START = 0x3,
    BTC_ASSOCIATE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_notify_type_media_status {
    BTC_MEDIA_DISCONNECT = 0x0,
    BTC_MEDIA_CONNECT = 0x1,
    BTC_MEDIA_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_notify_type_special_packet {
    BTC_PACKET_UNKNOWN = 0x0,
    BTC_PACKET_DHCP = 0x1,
    BTC_PACKET_ARP = 0x2,
    BTC_PACKET_EAPOL = 0x3,
    BTC_PACKET_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hci_ext_bt_operation {
    HCI_BT_OP_NONE = 0x0,
    HCI_BT_OP_INQUIRY_START = 0x1,
    HCI_BT_OP_INQUIRY_FINISH = 0x2,
    HCI_BT_OP_PAGING_START = 0x3,
    HCI_BT_OP_PAGING_SUCCESS = 0x4,
    HCI_BT_OP_PAGING_UNSUCCESS = 0x5,
    HCI_BT_OP_PAIRING_START = 0x6,
    HCI_BT_OP_PAIRING_FINISH = 0x7,
    HCI_BT_OP_BT_DEV_ENABLE = 0x8,
    HCI_BT_OP_BT_DEV_DISABLE = 0x9,
    HCI_BT_OP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_notify_type_stack_operation {
    BTC_STACK_OP_NONE = 0x0,
    BTC_STACK_OP_INQ_PAGE_PAIR_START = 0x1,
    BTC_STACK_OP_INQ_PAGE_PAIR_FINISH = 0x2,
    BTC_STACK_OP_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btc_bt_info {
    pub bt_disabled: bool,
    pub rssi_adjust_for_agc_table_on: u8,
    pub rssi_adjust_for_1ant_coex_type: u8,
    pub pre_bt_ctrl_agg_buf_size: bool,
    pub bt_busy: bool,
    pub pre_agg_buf_size: u8,
    pub agg_buf_size: u8,
    pub limited_dig: bool,
    pub pre_reject_agg_pkt: bool,
    pub reject_agg_pkt: bool,
    pub bt_ctrl_buf_size: bool,
    pub increase_scan_dev_num: bool,
    pub miracast_plus_bt: bool,
    pub bt_ctrl_agg_buf_size: bool,
    pub bt_tx_rx_mask: bool,
    pub bt_hci_ver: u16,
    pub bt_real_fw_ver: u16,
    pub bt_fw_ver: u8,
    pub bt_get_fw_ver: u32,
    pub bt_disable_low_pwr: bool,
// the following is for 1Ant solution
    pub bt_ctrl_lps: bool,
    pub bt_pwr_save_mode: bool,
    pub bt_lps_on: bool,
    pub force_to_roam: bool,
    pub force_exec_pwr_cmd_cnt: u8,
    pub lps_val: u8,
    pub rpwm_val: u8,
    pub ra_mask: u32,
    pub afh_map_l: u32,
    pub afh_map_m: u32,
    pub afh_map_h: u16,
    pub bt_supported_feature: u32,
    pub bt_supported_version: u32,
    pub bt_device_info: u32,
    pub bt_forb_slot_val: u32,
    pub bt_ant_det_val: u8,
    pub bt_ble_scan_type: u8,
    pub bt_ble_scan_para: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btc_stack_info {
    pub profile_notified: bool,
    pub /: *mut *mut u16 hci_version; / stack hci version,
    pub num_of_link: u8,
    pub bt_link_exist: bool,
    pub sco_exist: bool,
    pub acl_exist: bool,
    pub a2dp_exist: bool,
    pub hid_exist: bool,
    pub num_of_hid: u8,
    pub pan_exist: bool,
    pub unknown_acl_exist: bool,
    pub min_bt_rssi: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btc_statistics {
    pub cnt_bind: u32,
    pub cnt_init_hw_config: u32,
    pub cnt_init_coex_dm: u32,
    pub cnt_ips_notify: u32,
    pub cnt_lps_notify: u32,
    pub cnt_scan_notify: u32,
    pub cnt_connect_notify: u32,
    pub cnt_media_status_notify: u32,
    pub cnt_special_packet_notify: u32,
    pub cnt_bt_info_notify: u32,
    pub cnt_periodical: u32,
    pub cnt_coex_dm_switch: u32,
    pub cnt_stack_operation_notify: u32,
    pub cnt_dbg_ctrl: u32,
    pub cnt_pre_load_firmware: u32,
    pub cnt_power_on: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btc_bt_link_info {
    pub bt_link_exist: bool,
    pub bt_hi_pri_link_exist: bool,
    pub sco_exist: bool,
    pub sco_only: bool,
    pub a2dp_exist: bool,
    pub a2dp_only: bool,
    pub hid_exist: bool,
    pub hid_only: bool,
    pub pan_exist: bool,
    pub pan_only: bool,
    pub slave_role: bool,
    pub acl_busy: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_antenna_pos {
    BTC_ANTENNA_AT_MAIN_PORT = 0x1,
    BTC_ANTENNA_AT_AUX_PORT = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_mp_h2c_op_code {
    BT_OP_GET_BT_VERSION			= 0,
    BT_OP_WRITE_REG_ADDR			= 12,
    BT_OP_WRITE_REG_VALUE			= 13,
    BT_OP_READ_REG				= 17,
    BT_OP_GET_AFH_MAP_L			= 30,
    BT_OP_GET_AFH_MAP_M			= 31,
    BT_OP_GET_AFH_MAP_H			= 32,
    BT_OP_GET_BT_COEX_SUPPORTED_FEATURE	= 42,
    BT_OP_GET_BT_COEX_SUPPORTED_VERSION	= 43,
    BT_OP_GET_BT_ANT_DET_VAL		= 44,
    BT_OP_GET_BT_BLE_SCAN_PARA		= 45,
    BT_OP_GET_BT_BLE_SCAN_TYPE		= 46,
    BT_OP_GET_BT_DEVICE_INFO		= 48,
    BT_OP_GET_BT_FORBIDDEN_SLOT_VAL		= 49,
    BT_OP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btc_mp_h2c_req_num {
// 4 bits only
    BT_SEQ_DONT_CARE			= 0,
    BT_SEQ_GET_BT_VERSION			= 0xE,
    BT_SEQ_GET_AFH_MAP_L			= 0x5,
    BT_SEQ_GET_AFH_MAP_M			= 0x6,
    BT_SEQ_GET_AFH_MAP_H			= 0x9,
    BT_SEQ_GET_BT_COEX_SUPPORTED_FEATURE	= 0x7,
    BT_SEQ_GET_BT_COEX_SUPPORTED_VERSION	= 0x8,
    BT_SEQ_GET_BT_ANT_DET_VAL		= 0x2,
    BT_SEQ_GET_BT_BLE_SCAN_PARA		= 0x3,
    BT_SEQ_GET_BT_BLE_SCAN_TYPE		= 0x4,
    BT_SEQ_GET_BT_DEVICE_INFO		= 0xA,
    BT_SEQ_GET_BT_FORB_SLOT_VAL		= 0xB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btc_coexist {
// make sure only one adapter can bind the data context
    pub binded: bool,
// default adapter
    pub adapter: *mut c_void,
    pub board_info: btc_board_info,
// some bt info referenced by non-bt module
    pub bt_info: btc_bt_info,
    pub stack_info: btc_stack_info,
    pub chip_interface: btc_chip_interface,
    pub bt_link_info: btc_bt_link_info,
// boolean variables to replace BT_AUTO_REPORT_ONLY_XXXXY_ZANT
// configuration parameters
//
    pub auto_report_1ant: bool,
    pub auto_report_2ant: bool,
    pub dbg_mode_1ant: bool,
    pub dbg_mode_2ant: bool,
    pub initialized: bool,
    pub stop_coex_dm: bool,
    pub manual_control: bool,
    pub statistics: btc_statistics,
    pub pwr_mode_val: [u8; 10],
    pub bt_mp_comp: completion,
// function pointers - io related
    pub reg_addr): *mut *mut *mut u8 (btc_read_1byte)(void btc_context, u32,
    pub data): *mut *mut *mut void (btc_write_1byte)(void btc_context, u32 reg_addr, u32,
    pub data1b): u32 bit_mask, u8,
    pub reg_addr): *mut *mut *mut u16 (btc_read_2byte)(void btc_context, u32,
    pub data): *mut *mut *mut void (btc_write_2byte)(void btc_context, u32 reg_addr, u16,
    pub reg_addr): *mut *mut *mut u32 (btc_read_4byte)(void btc_context, u32,
    pub data): *mut *mut *mut void (btc_write_4byte)(void btc_context, u32 reg_addr, u32,
    pub data): u8,
    pub data): u32 bit_mask, u32,
    pub bit_mask): u32,
    pub data): u32 bit_mask, u32,
    pub bit_mask): u32 reg_addr, u32,
    pub cmd_buffer): *mut u32 cmd_len, u8,
    pub m): *mut seq_file,
    pub out_buf): *mut *mut *mut bool (btc_get)(void btcoexist, u8 get_type, void,
    pub in_buf): *mut *mut *mut bool (btc_set)(void btcoexist, u8 set_type, void,
    pub value): u32,
    pub offset): *mut *mut *mut u32 (btc_get_bt_reg)(void btc_context, u8 reg_type, u32,
    pub btcoexist): *mut *mut u32 (btc_get_bt_coex_supported_feature)(void,
    pub btcoexist): *mut *mut u32 (btc_get_bt_coex_supported_version)(void,
    pub btcoexist): *mut *mut u32 (btc_get_bt_phydm_version)(void,
    pub ra_threshold_offset): u8,
    pub dm_id): dm_info_query,
    pub btcoexist): *mut *mut u8 (btc_get_ant_det_val_from_bt)(void,
    pub btcoexist): *mut *mut u8 (btc_get_ble_scan_type_from_bt)(void,
    pub scan_type): *mut *mut *mut u32 (btc_get_ble_scan_para_from_bt)(void btcoexist, u8,
    pub afh_map): *mut u8,
}

extern "C" {
    pub fn halbtc_is_wifi_uplink(adapter: *mut rtl_priv) -> bool;
}

extern "C" {
    pub fn exhalbtc_initlize_variables(rtlpriv: *mut rtl_priv) -> bool;
}
extern "C" {
    pub fn exhalbtc_initlize_variables_wifi_only(rtlpriv: *mut rtl_priv) -> bool;
}
extern "C" {
    pub fn exhalbtc_bind_bt_coex_withadapter(adapter: *mut c_void) -> bool;
}
extern "C" {
    pub fn exhalbtc_power_on_setting(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn exhalbtc_pre_load_firmware(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn exhalbtc_init_hw_config(btcoexist: *mut btc_coexist, wifi_only: bool);
}
extern "C" {
    pub fn exhalbtc_init_hw_config_wifi_only(wifionly_cfg: *mut wifi_only_cfg);
}
extern "C" {
    pub fn exhalbtc_init_coex_dm(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn exhalbtc_ips_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn exhalbtc_lps_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn exhalbtc_scan_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn exhalbtc_connect_notify(btcoexist: *mut btc_coexist, action: u8);
}
extern "C" {
    pub fn exhalbtc_special_packet_notify(btcoexist: *mut btc_coexist, pkt_type: u8);
}
extern "C" {
    pub fn exhalbtc_halt_notify(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn exhalbtc_pnp_notify(btcoexist: *mut btc_coexist, pnp_state: u8);
}
extern "C" {
    pub fn exhalbtc_periodical(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn exhalbtc_update_min_bt_rssi(btcoexist: *mut btc_coexist, bt_rssi: i8);
}
extern "C" {
    pub fn exhalbtc_set_bt_exist(btcoexist: *mut btc_coexist, bt_exist: bool);
}
extern "C" {
    pub fn exhalbtc_set_chip_type(btcoexist: *mut btc_coexist, chip_type: u8);
}
extern "C" {
    pub fn exhalbtc_set_ant_num(rtlpriv: *mut rtl_priv, type: u8, ant_num: u8);
}
extern "C" {
    pub fn exhalbtc_switch_band_notify(btcoexist: *mut btc_coexist, type: u8);
}
extern "C" {
    pub fn exhalbtc_lps_leave(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn exhalbtc_low_wifi_traffic_notify(btcoexist: *mut btc_coexist);
}
extern "C" {
    pub fn halbtc_send_wifi_port_id_cmd(bt_context: *mut c_void);
}
extern "C" {
    pub fn halbtc_set_default_port_id_cmd(bt_context: *mut c_void);
}
// The following are used by wifi_only case
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wifionly_chip_interface {
    WIFIONLY_INTF_UNKNOWN	= 0,
    WIFIONLY_INTF_PCI		= 1,
    WIFIONLY_INTF_USB		= 2,
    WIFIONLY_INTF_SDIO		= 3,
    WIFIONLY_INTF_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wifionly_customer_id {
    CUSTOMER_NORMAL			= 0,
    CUSTOMER_HP_1			= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wifi_only_haldata {
    pub customer_id: u16,
    pub efuse_pg_antnum: u8,
    pub efuse_pg_antpath: u8,
    pub rfe_type: u8,
    pub ant_div_cfg: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wifi_only_cfg {
    pub adapter: *mut c_void,
    pub haldata_info: wifi_only_haldata,
    pub chip_interface: wifionly_chip_interface,
}
