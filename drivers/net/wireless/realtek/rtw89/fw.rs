//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/fw.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_dl_status {
    RTW89_FWDL_INITIAL_STATE = 0,
    RTW89_FWDL_FWDL_ONGOING = 1,
    RTW89_FWDL_CHECKSUM_FAIL = 2,
    RTW89_FWDL_SECURITY_FAIL = 3,
    RTW89_FWDL_CV_NOT_MATCH = 4,
    RTW89_FWDL_RSVD0 = 5,
    RTW89_FWDL_WCPU_FWDL_RDY = 6,
    RTW89_FWDL_WCPU_FW_INIT_RDY = 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2hreg_hdr {
    pub w0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2hreg_phycap {
    pub w0: u32,
    pub w1: u32,
    pub w2: u32,
    pub w3: u32,
    pub __packed: },

pub const RTW89_C2HREG_PHYCAP_W1_PROT_11N: c_int = 1;
pub const RTW89_C2HREG_PHYCAP_W1_PROT_11AC: c_int = 2;
pub const RTW89_C2HREG_PHYCAP_W1_PROT_11AX: c_int = 3;
pub const RTW89_C2HREG_PHYCAP_W1_PROT_11BE: c_int = 4;

pub const RTW89_C2HREG_PHYCAP_P1_W2_QAM_256: c_uint = 0x1;
pub const RTW89_C2HREG_PHYCAP_P1_W2_QAM_1024: c_uint = 0x2;
pub const RTW89_C2HREG_PHYCAP_P1_W2_QAM_4096: c_uint = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2creg_hdr {
    pub w0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2creg_sch_tx_en {
    pub w0: u32,
    pub w1: u32,
    pub __packed: },

pub const RTW89_H2CREG_MAX: c_int = 4;
pub const RTW89_C2HREG_MAX: c_int = 4;
pub const RTW89_C2HREG_HDR_LEN: c_int = 2;
pub const RTW89_H2CREG_HDR_LEN: c_int = 2;
pub const RTW89_C2H_TIMEOUT: c_int = 1000000;
pub const RTW89_C2H_TIMEOUT_USB: c_int = 4000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_c2h_info {
    pub id: u8,
    pub content_len: u8,
    pub timeout: u32,
    pub c2hreg: [u32; RTW89_C2HREG_MAX],
    pub hdr: rtw89_c2hreg_hdr,
    pub phycap: rtw89_c2hreg_phycap,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_h2c_info {
    pub id: u8,
    pub content_len: u8,
    pub h2creg: [u32; RTW89_H2CREG_MAX],
    pub hdr: rtw89_h2creg_hdr,
    pub sch_tx_en: rtw89_h2creg_sch_tx_en,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_h2c_type {
    RTW89_FWCMD_H2CREG_FUNC_H2CREG_LB = 0,
    RTW89_FWCMD_H2CREG_FUNC_CNSL_CMD,
    RTW89_FWCMD_H2CREG_FUNC_FWERR,
    RTW89_FWCMD_H2CREG_FUNC_GET_FEATURE,
    RTW89_FWCMD_H2CREG_FUNC_GETPKT_INFORM,
    RTW89_FWCMD_H2CREG_FUNC_SCH_TX_EN,
    RTW89_FWCMD_H2CREG_FUNC_WOW_TRX_STOP,
    RTW89_FWCMD_H2CREG_FUNC_AOAC_RPT_1,
    RTW89_FWCMD_H2CREG_FUNC_AOAC_RPT_2,
    RTW89_FWCMD_H2CREG_FUNC_AOAC_RPT_3_REQ,
    RTW89_FWCMD_H2CREG_FUNC_WOW_CPUIO_RX_CTRL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mac_c2h_type {
    RTW89_FWCMD_C2HREG_FUNC_C2HREG_LB = 0,
    RTW89_FWCMD_C2HREG_FUNC_ERR_RPT,
    RTW89_FWCMD_C2HREG_FUNC_ERR_MSG,
    RTW89_FWCMD_C2HREG_FUNC_PHY_CAP,
    RTW89_FWCMD_C2HREG_FUNC_TX_PAUSE_RPT,
    RTW89_FWCMD_C2HREG_FUNC_WOW_CPUIO_RX_ACK = 0xA,
    RTW89_FWCMD_C2HREG_FUNC_PHY_CAP_PART1 = 0xC,
    RTW89_FWCMD_C2HREG_FUNC_PS_LEAVE_ACK = 0xD,
    RTW89_FWCMD_C2HREG_FUNC_NULL = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_c2h_category {
    RTW89_C2H_CAT_TEST,
    RTW89_C2H_CAT_MAC,
    RTW89_C2H_CAT_OUTSRC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_log_level {
    RTW89_FW_LOG_LEVEL_OFF,
    RTW89_FW_LOG_LEVEL_CRT,
    RTW89_FW_LOG_LEVEL_SER,
    RTW89_FW_LOG_LEVEL_WARN,
    RTW89_FW_LOG_LEVEL_LOUD,
    RTW89_FW_LOG_LEVEL_TR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_log_path {
    RTW89_FW_LOG_LEVEL_UART,
    RTW89_FW_LOG_LEVEL_C2H,
    RTW89_FW_LOG_LEVEL_SNI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_log_comp {
    RTW89_FW_LOG_COMP_VER,
    RTW89_FW_LOG_COMP_INIT,
    RTW89_FW_LOG_COMP_TASK,
    RTW89_FW_LOG_COMP_CNS,
    RTW89_FW_LOG_COMP_H2C,
    RTW89_FW_LOG_COMP_C2H,
    RTW89_FW_LOG_COMP_TX,
    RTW89_FW_LOG_COMP_RX,
    RTW89_FW_LOG_COMP_IPSEC,
    RTW89_FW_LOG_COMP_TIMER,
    RTW89_FW_LOG_COMP_DBGPKT,
    RTW89_FW_LOG_COMP_PS,
    RTW89_FW_LOG_COMP_ERROR,
    RTW89_FW_LOG_COMP_WOWLAN,
    RTW89_FW_LOG_COMP_SECURE_BOOT,
    RTW89_FW_LOG_COMP_BTC,
    RTW89_FW_LOG_COMP_BB,
    RTW89_FW_LOG_COMP_TWT,
    RTW89_FW_LOG_COMP_RF,
    RTW89_FW_LOG_COMP_MCC = 20,
    RTW89_FW_LOG_COMP_MLO = 26,
    RTW89_FW_LOG_COMP_SCAN = 28,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_pkt_offload_op {
    RTW89_PKT_OFLD_OP_ADD,
    RTW89_PKT_OFLD_OP_DEL,
    RTW89_PKT_OFLD_OP_READ,

    NUM_OF_RTW89_PKT_OFFLOAD_OP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_scanofld_notify_reason {
    RTW89_SCAN_DWELL_NOTIFY,
    RTW89_SCAN_PRE_TX_NOTIFY,
    RTW89_SCAN_POST_TX_NOTIFY,
    RTW89_SCAN_ENTER_CH_NOTIFY,
    RTW89_SCAN_LEAVE_CH_NOTIFY,
    RTW89_SCAN_END_SCAN_NOTIFY,
    RTW89_SCAN_REPORT_NOTIFY,
    RTW89_SCAN_CHKPT_NOTIFY,
    RTW89_SCAN_ENTER_OP_NOTIFY,
    RTW89_SCAN_LEAVE_OP_NOTIFY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_scanofld_status {
    RTW89_SCAN_STATUS_NOTIFY,
    RTW89_SCAN_STATUS_SUCCESS,
    RTW89_SCAN_STATUS_FAIL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_chan_type {
    RTW89_CHAN_OPERATE = 0,
    RTW89_CHAN_ACTIVE,
    RTW89_CHAN_DFS,
    RTW89_CHAN_EXTRA_OP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_p2pps_action {
    RTW89_P2P_ACT_INIT = 0,
    RTW89_P2P_ACT_UPDATE = 1,
    RTW89_P2P_ACT_REMOVE = 2,
    RTW89_P2P_ACT_TERMINATE = 3,
}

pub const RTW89_DEFAULT_CQM_HYST: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bcn_fltr_offload_mode {
    RTW89_BCN_FLTR_OFFLOAD_MODE_0 = 0,
    RTW89_BCN_FLTR_OFFLOAD_MODE_1,
    RTW89_BCN_FLTR_OFFLOAD_MODE_2,
    RTW89_BCN_FLTR_OFFLOAD_MODE_3,

    RTW89_BCN_FLTR_OFFLOAD_MODE_DEFAULT = RTW89_BCN_FLTR_OFFLOAD_MODE_0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bcn_fltr_type {
    RTW89_BCN_FLTR_BEACON_LOSS,
    RTW89_BCN_FLTR_RSSI,
    RTW89_BCN_FLTR_NOTIFY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bcn_fltr_rssi_event {
    RTW89_BCN_FLTR_RSSI_NOT_CHANGED,
    RTW89_BCN_FLTR_RSSI_HIGH,
    RTW89_BCN_FLTR_RSSI_LOW,
}

pub const FWDL_SECTION_MAX_NUM: c_int = 10;
pub const FWDL_SECTION_CHKSUM_LEN: c_int = 8;
pub const FWDL_SECTION_PER_PKT_LEN: c_int = 2020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_hdr_section_info {
    pub redl: u8,
    pub addr: *const u8,
    pub len: u32,
    pub len_override: u32,
    pub dladdr: u32,
    pub mssc: u32,
    pub type: u8,
    pub ignore: bool,
    pub key_addr: *const u8,
    pub key_len: u32,
    pub key_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_bin_info {
    pub section_num: u8,
    pub part_size: u32,
    pub hdr_len: u32,
    pub dynamic_hdr_en: bool,
    pub dynamic_hdr_len: u32,
    pub idmem_share_mode: u8,
    pub dsp_checksum: bool,
    pub secure_section_exist: bool,
    pub section_info: [rtw89_fw_hdr_section_info; FWDL_SECTION_MAX_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_macid_pause_grp {
    pub pause_grp: [__le32; 4],
    pub mask_grp: [__le32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_macid_pause_sleep_grp {
    pub pause_grp: [__le32; 4],
    pub pause_mask_grp: [__le32; 4],
    pub sleep_grp: [__le32; 4],
    pub sleep_mask_grp: [__le32; 4],
    pub n: [} __packed; 4],
    pub __packed: },
pub const RTW89_H2C_MAX_SIZE: c_int = 2048;
pub const RTW89_CHANNEL_TIME: c_int = 45;
pub const RTW89_CHANNEL_TIME_6G: c_int = 20;
pub const RTW89_CHANNEL_TIME_EXTRA_OP: c_int = 30;
pub const RTW89_DFS_CHAN_TIME: c_int = 105;
pub const RTW89_OFF_CHAN_TIME: c_int = 100;
pub const RTW89_P2P_CHAN_TIME: c_int = 105;
pub const RTW89_DWELL_TIME: c_int = 20;
pub const RTW89_DWELL_TIME_6G: c_int = 10;
pub const RTW89_SCAN_WIDTH: c_int = 0;
pub const RTW89_SCANOFLD_MAX_SSID: c_int = 8;
pub const RTW89_SCANOFLD_MAX_IE_LEN: c_int = 512;
pub const RTW89_SCANOFLD_PKT_NONE: c_uint = 0xFF;
pub const RTW89_SCANOFLD_DEBUG_MASK: c_uint = 0x1F;
pub const RTW89_CHAN_INVALID: c_uint = 0xFF;
pub const RTW89_MAC_CHINFO_SIZE: c_int = 28;
pub const RTW89_MAC_CHINFO_SIZE_BE: c_int = 32;
pub const RTW89_SCAN_LIST_GUARD: c_int = 4;

pub const RTW89_BCN_LOSS_CNT: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_chinfo_ax {
    pub period: u8,
    pub dwell_time: u8,
    pub central_ch: u8,
    pub pri_ch: u8,
    pub bw:3: u8,
    pub notify_action:5: u8,
    pub num_pkt:4: u8,
    pub tx_pkt:1: u8,
    pub pause_data:1: u8,
    pub ch_band:2: u8,
    pub probe_id: u8,
    pub dfs_ch:1: u8,
    pub tx_null:1: u8,
    pub rand_seq_num:1: u8,
    pub cfg_tx_pwr:1: u8,
    pub 1: u8 macid_tx:,
    pub 3: u8 rsvd0:,
    pub pkt_id: [u8; RTW89_SCANOFLD_MAX_SSID],
    pub tx_pwr_idx: u16,
    pub rsvd1: u8,
    pub list: list_head,
    pub is_psc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_chinfo_be {
    pub period: u8,
    pub dwell_time: u8,
    pub central_ch: u8,
    pub pri_ch: u8,
    pub bw:3: u8,
    pub ch_band:2: u8,
    pub dfs_ch:1: u8,
    pub pause_data:1: u8,
    pub tx_null:1: u8,
    pub rand_seq_num:1: u8,
    pub notify_action:5: u8,
    pub probe_id: u8,
    pub leave_crit: u8,
    pub chkpt_timer: u8,
    pub leave_time: u8,
    pub leave_th: u8,
    pub tx_pkt_ctrl: u16,
    pub pkt_id: [u8; RTW89_SCANOFLD_MAX_SSID],
    pub sw_def: u8,
    pub fw_probe0_ssids: u16,
    pub fw_probe0_shortssids: u16,
    pub fw_probe0_bssids: u16,
    pub list: list_head,
    pub is_psc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_pktofld_info {
    pub list: list_head,
    pub id: u8,
    pub wildcard_6ghz: bool,
// Below fields are for WiFi 6 chips 6 GHz RNR use only
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: u8,
    pub bssid: [u8; ETH_ALEN],
    pub channel_6ghz: u16,
    pub cancel: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ra {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ra_v1 {
    pub v0: rtw89_h2c_ra,
    pub w4: __le32,
    pub w5: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ra_tx_history {
    pub w0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ra_phy_ch_rpt {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ra_drv_ctrl_fw {
    pub w0: __le32,
    pub __packed: },

    pub 0)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x00, val, GENMASK(7,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x00, val, GENMASK(15,,
    pub 16)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x00, val, GENMASK(23,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x01, val, GENMASK(3,,
    pub BIT(4)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x01, val,,
    pub BIT(5)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x01, val,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x02, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x03, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x04, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x05, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x00, val, GENMASK(1,,
    pub BIT(3)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x00, val,,
    pub BIT(4)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x00, val,,
    pub 5)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x00, val, GENMASK(6,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(cmd) + 0x01, val, GENMASK(31,,

pub const FWDL_SECURITY_SECTION_TYPE: c_int = 9;
pub const FWDL_SECURITY_SIGLEN: c_int = 512;
pub const FWDL_SECURITY_CHKSUM_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_dynhdr_sec {
    pub w0: __le32,
    pub content: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_dynhdr_hdr {
    pub hdr_len: __le32,
    pub setcion_count: __le32,
// struct rtw89_fw_dynhdr_sec (nested flexible structures)
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_hdr_section {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_hdr {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub sections: [rtw89_fw_hdr_section; ],
// struct rtw89_fw_dynhdr_hdr (optional)
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_hdr_section_v1 {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub __packed: },

pub const FORMATTED_MSSC: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_hdr_v1 {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub w10: __le32,
    pub w11: __le32,
    pub sections: [rtw89_fw_hdr_section_v1; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_mss_pool_rmp_tbl_type {
    MSS_POOL_RMP_TBL_BITMASK = 0x0,
    MSS_POOL_RMP_TBL_RECORD = 0x1,
}

pub const FWDL_MSS_POOL_DEFKEYSETS_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mss_pool_hdr {
    pub /: *mut *mut u8 signature[8]; / equal to mss_signature[],
    pub rmp_tbl_offset: __le32,
    pub key_raw_offset: __le32,
    pub defen: u8,
    pub rsvd: [u8; 3],
    pub /: *mut *mut u8 rmpfmt; / enum rtw89_fw_mss_pool_rmp_tbl_type,
    pub mssdev_max: u8,
    pub keypair_num: __le16,
    pub msscust_max: __le16,
    pub msskey_num_max: __le16,
    pub rsvd3: __le32,
    pub rmp_tbl: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_fw_section_mssc_content {
    pub pad: [u8; 0x20],
    pub bit_in_chip_list: u8,
    pub ver: u8,
    pub blacklist: } __packed,
    pub pad: [u8; 58],
    pub v: __le32,
    pub sb_sel_ver: } __packed,
    pub pad: [u8; 60],
    pub v: __le16,
    pub key_sign_len: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_blacklist {
    pub ver: u8,
    pub list: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cctlinfo_ud_g7 {
    pub c0: __le32,
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub w10: __le32,
    pub w11: __le32,
    pub w12: __le32,
    pub w13: __le32,
    pub w14: __le32,
    pub w15: __le32,
    pub m0: __le32,
    pub m1: __le32,
    pub m2: __le32,
    pub m3: __le32,
    pub m4: __le32,
    pub m5: __le32,
    pub m6: __le32,
    pub m7: __le32,
    pub m8: __le32,
    pub m9: __le32,
    pub m10: __le32,
    pub m11: __le32,
    pub m12: __le32,
    pub m13: __le32,
    pub m14: __le32,
    pub m15: __le32,
    pub __packed: },

// W9~13 are reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cctlinfo_ud_be {
    pub c0: __le32,
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub w10: __le32,
    pub w11: __le32,
    pub w12: __le32,
    pub w13: __le32,
    pub w14: __le32,
    pub w15: __le32,
    pub m0: __le32,
    pub m1: __le32,
    pub m2: __le32,
    pub m3: __le32,
    pub m4: __le32,
    pub m5: __le32,
    pub m6: __le32,
    pub m7: __le32,
    pub m8: __le32,
    pub m9: __le32,
    pub m10: __le32,
    pub m11: __le32,
    pub m12: __le32,
    pub m13: __le32,
    pub m14: __le32,
    pub m15: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_bcn_upd {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_bcn_upd_be {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub w10: __le32,
    pub w11: __le32,
    pub w12: __le32,
    pub w13: __le32,
    pub w14: __le32,
    pub w15: __le32,
    pub w16: __le32,
    pub w17: __le32,
    pub w18: __le32,
    pub w19: __le32,
    pub w20: __le32,
    pub w21: __le32,
    pub w22: __le32,
    pub w23: __le32,
    pub w24: __le32,
    pub w25: __le32,
    pub w26: __le32,
    pub w27: __le32,
    pub w28: __le32,
    pub w29: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_tbtt_tuning {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_pwr_lvl {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_role_maintain {
    pub w0: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_sta_type {
    RTW89_FW_N_AC_STA = 0,
    RTW89_FW_AX_STA = 1,
    RTW89_FW_BE_STA = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_join {
    pub w0: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_join_v1 {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub __packed: },

pub const RTW89_H2C_JOININFO_MLO_MODE_MLMR: c_int = 0;
pub const RTW89_H2C_JOININFO_MLO_MODE_MLSR: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_notify_dbcc {
    pub w0: __le32,
    pub __packed: },

    pub 0)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(7,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(15,,
    pub 16)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(23,,
    pub 24)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 1, val, GENMASK(7,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 1, val, GENMASK(15,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(7,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(15,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 1, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 2, val, GENMASK(31,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ba_cam {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ba_cam_v1 {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ba_cam_init {
    pub w0: __le32,
    pub __packed: },

    pub 0)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(7,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(15,,
    pub 16)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(19,,
    pub 20)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(23,,
    pub 24)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(31,,
    pub BIT(0)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 1, val,,
    pub BIT(1)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 1, val,,
    pub BIT(2)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 1, val,,
    pub BIT(3)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 1, val,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 1, val, GENMASK(15,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_lps_ch_info {
    pub pri_ch: u8,
    pub central_ch: u8,
    pub bw: u8,
    pub band: u8,
    pub info: [} __packed; 2],
    pub mlo_dbcc_mode_lps: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_lps_ml_cmn_info {
    pub fmt_id: u8,
    pub rfe_type: u8,
    pub rsvd0: [u8; 2],
    pub mlo_dbcc_mode: __le32,
    pub central_ch: [u8; RTW89_PHY_NUM],
    pub pri_ch: [u8; RTW89_PHY_NUM],
    pub bw: [u8; RTW89_PHY_NUM],
    pub band: [u8; RTW89_PHY_NUM],
    pub bcn_rate_type: [u8; RTW89_PHY_NUM],
    pub rsvd1: [u8; 2],
    pub tia_gain: [__le16; RTW89_PHY_NUM][TIA_GAIN_NUM],
    pub lna_gain: [u8; RTW89_PHY_NUM][LNA_GAIN_NUM],
    pub rsvd2: [u8; 2],
    pub 1]: u8 tia_lna_op1db[RTW89_PHY_NUM][LNA_GAIN_NUM +,
    pub lna_op1db: [u8; RTW89_PHY_NUM][LNA_GAIN_NUM],
    pub dup_bcn_ofst: [u8; RTW89_PHY_NUM],
    pub __packed: },
pub const BB_RX_GAIN_TB_RSSI_COMP_NUM: c_int = 3;
pub const BB_RX_GAIN_CCK_RPL_BIAS_COMP_NUM: c_int = 2;
pub const BB_GT2_GS_IDX_NUM: c_int = 11;
pub const BB_GT2_WB_GIDX_ELNA_NUM: c_int = 16;
pub const BB_GT2_G_ELNA_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bb_link_rx_gain_table_type {
    RTW89_BB_PS_LINK_RX_GAIN_TAB_BCN_PATH_A = 0x00,
    RTW89_BB_PS_LINK_RX_GAIN_TAB_BCN_PATH_B = 0x01,
    RTW89_BB_PS_LINK_RX_GAIN_TAB_NOR_PATH_A = 0x02,
    RTW89_BB_PS_LINK_RX_GAIN_TAB_NOR_PATH_B = 0x03,
    RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX,
}

pub const RTW89_BB_PS_LINK_ID_SKIP: c_uint = 0xfe;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bb_ps_link_buf_id {
    RTW89_BB_PS_LINK_BUF_0 = 0x00,
    RTW89_BB_PS_LINK_BUF_1 = 0x01,
    RTW89_BB_PS_LINK_BUF_2 = 0x02,
    RTW89_BB_PS_LINK_BUF_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_bb_link_info_rx_gain {
    pub gain_ofst: [u8; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX],
    pub rpl_bias_comp: [__le16; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX],
    pub cck_gain_ofst: [u8; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX],
    pub gain_err_lna: [u8; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX][LNA_GAIN_NUM],
    pub gain_err_tia: [__le16; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX][TIA_GAIN_NUM],
    pub op1db_lna: [u8; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX][LNA_GAIN_NUM],
    pub op1db_tia: [u8; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX][TIA_LNA_OP1DB_NUM],
    pub _20M: [u8; RTW89_BW20_SC_20M],
    pub _40M: [u8; RTW89_BW20_SC_40M],
    pub _80M: [u8; RTW89_BW20_SC_80M],
    pub _160M: [u8; RTW89_BW20_SC_160M],
    pub rpl_bias_comp_bw: [}; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX],
    pub wb_gs: [u8; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX][BB_GT2_GS_IDX_NUM],
    pub bypass_lna: [u8; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX][LNA_GAIN_NUM],
    pub wb_lna_tia: [u8; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX][BB_GT2_WB_GIDX_ELNA_NUM],
    pub wb_g_elna: [u8; RTW89_BB_PS_LINK_RX_GAIN_TAB_MAX][BB_GT2_G_ELNA_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_lps_ml_cmn_info_v1 {
    pub fmt_id: u8,
    pub rfe_type: u8,
    pub rssi_main: u8,
    pub rsvd0: u8,
    pub mlo_dbcc_mode: __le32,
    pub link_id: [u8; RTW89_BB_PS_LINK_BUF_MAX],
    pub central_ch: [u8; RTW89_BB_PS_LINK_BUF_MAX],
    pub pri_ch: [u8; RTW89_BB_PS_LINK_BUF_MAX],
    pub bw: [u8; RTW89_BB_PS_LINK_BUF_MAX],
    pub band: [u8; RTW89_BB_PS_LINK_BUF_MAX],
    pub dup_bcn_ofst: [u8; RTW89_BB_PS_LINK_BUF_MAX],
    pub rx_gain: [rtw89_bb_link_info_rx_gain; RTW89_BB_PS_LINK_BUF_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_trig_cpu_except {
    pub w0: __le32,
    pub __packed: },

    pub 0)): *mut *mut le32p_replace_bits((__le32 )cmd, val, GENMASK(7,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )cmd, val, GENMASK(15,,
    pub 16)): *mut *mut le32p_replace_bits((__le32 )cmd, val, GENMASK(23,,
    pub 24)): *mut *mut le32p_replace_bits((__le32 )cmd, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )cmd + 1, val, GENMASK(7,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )cmd + 1, val, GENMASK(15,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )cmd + 2, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )cmd + 3, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )cmd + 4, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )cmd + 5, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(1,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(15,,
    pub 16)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(24,,
    pub 24)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(31,,
    pub BIT(0)): *mut *mut le32p_replace_bits((__le32 )h2c, val,,
    pub BIT(1)): *mut *mut le32p_replace_bits((__le32 )h2c, val,,
    pub BIT(2)): *mut *mut le32p_replace_bits((__le32 )h2c, val,,
    pub 8)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(15,,
    pub 16)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(23,,
    pub 24)): *mut *mut le32p_replace_bits((__le32 )h2c, val, GENMASK(31,,
    pub 0)): *mut *mut le32p_replace_bits((__le32 )(h2c) + 1, val, GENMASK(7,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_wow_global {
    pub w0: __le32,
    pub key_info: rtw89_wow_key_info,
    pub __packed: },

pub const RTW89_MAX_SUPPORT_NL_NUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cfg_nlo {
    pub w0: __le32,
    pub nlo_cnt: u8,
    pub rsvd: [u8; 3],
    pub patterncheck: __le32,
    pub rsvd1: __le32,
    pub rsvd2: __le32,
    pub ssid_len: [u8; RTW89_MAX_SUPPORT_NL_NUM],
    pub chiper: [u8; RTW89_MAX_SUPPORT_NL_NUM],
    pub rsvd3: [u8; 24],
    pub ssid: [u8; RTW89_MAX_SUPPORT_NL_NUM][IEEE80211_MAX_SSID_LEN],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_wow_wakeup_ctrl {
    pub w0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_wow_cam_update {
    pub w0: __le32,
    pub wkfm0: __le32,
    pub wkfm1: __le32,
    pub wkfm2: __le32,
    pub wkfm3: __le32,
    pub w5: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_wow_payload_cam_update {
    pub w0: __le32,
    pub wkfm0: __le32,
    pub wkfm1: __le32,
    pub wkfm2: __le32,
    pub wkfm3: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_wow_gtk_ofld {
    pub w0: __le32,
    pub w1: __le32,
    pub gtk_info: rtw89_wow_gtk_info,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_arp_offload {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

pub const BT_H2C_FUNC_BT2ND: c_uint = 0x80;
pub const BT_C2H_FUNC_BT2ND: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_btf_h2c_class {
    BTFC_SET = 0x10,
    BTFC_GET = 0x11,
    BTFC_FW_EVENT = 0x12,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_btf_set {
    SET_REPORT_EN = 0x0,
    SET_SLOT_TABLE,
    SET_MREG_TABLE,
    SET_CX_POLICY,
    SET_GPIO_DBG,
    SET_DRV_INFO,
    SET_DRV_EVENT,
    SET_BT_WREG_ADDR,
    SET_BT_WREG_VAL,
    SET_BT_RREG_ADDR,
    SET_BT_WL_CH_INFO,
    SET_BT_INFO_REPORT,
    SET_BT_IGNORE_WLAN_ACT,
    SET_BT_TX_PWR,
    SET_BT_LNA_CONSTRAIN,
    SET_BT_QUERY_DEV_LIST,
    SET_BT_QUERY_DEV_INFO,
    SET_BT_PSD_REPORT,
    SET_H2C_TEST,
    SET_IOFLD_RF,
    SET_IOFLD_BB,
    SET_IOFLD_MAC,
    SET_IOFLD_SCBD,
    SET_H2C_MACRO,
    SET_MAX1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_btc_cxdrvinfo {
    CXDRVINFO_INIT = 0,
    CXDRVINFO_ROLE,
    CXDRVINFO_DBCC,
    CXDRVINFO_SMAP,
    CXDRVINFO_RFK,
    CXDRVINFO_RUN,
    CXDRVINFO_CTRL,
    CXDRVINFO_SCAN,
    CXDRVINFO_TRX,  /* WL traffic to WL fw */
    CXDRVINFO_TXPWR,
    CXDRVINFO_FDDT,
    CXDRVINFO_MLO,
    CXDRVINFO_OSI,
    CXDRVINFO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fbtc_gpio_type {
    CXDGPIO_EN_MAP = 0x0,
    CXDGPIO_MUX_MAP = 0x1,
    CXDGPIO_EXT_HPTA = 0x2,
    CXDGPIO_EXT_HMBX = 0x3,
    CXDGPIO_EXT_SWOUT = 0x4,
    CXDGPIO_EXT_SWIN = 0x5,
    CXDGPIO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_scan_mode {
    RTW89_SCAN_IMMEDIATE,
    RTW89_SCAN_DELAY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_scan_type {
    RTW89_SCAN_ONCE,
    RTW89_SCAN_NORMAL,
    RTW89_SCAN_NORMAL_SLOW,
    RTW89_SCAN_SEAMLESS,
    RTW89_SCAN_MAX,
}

    pub 0)): *mut *mut u8p_replace_bits((u8 )(cmd) + 0, val, GENMASK(7,,
    pub 0)): *mut *mut u8p_replace_bits((u8 )(cmd) + 1, val, GENMASK(7,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxhdr {
    pub type: u8,
    pub len: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxhdr_v7 {
    pub type: u8,
    pub ver: u8,
    pub len: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxctrl_v7 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub ctrl: rtw89_btc_ctrl_v7,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxctrl_v9 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub ctrl: rtw89_btc_ctrl_v9,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxrole_v7 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub r: rtw89_btc_wl_role_info_v7,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxrole_v8 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub r: rtw89_btc_wl_role_info_v8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxrole_v10 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub r: rtw89_btc_wl_role_info_v10,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxmlo_v2 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub mlo: rtw89_btc_wl_mlo_info_v2,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxosi {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub osi: rtw89_btc_fbtc_outsrc_set_info_v1,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxosi_v6 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub osi: rtw89_btc_fbtc_outsrc_set_info_v6,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxinit {
    pub hdr: rtw89_h2c_cxhdr,
    pub ant_type: u8,
    pub ant_num: u8,
    pub ant_iso: u8,
    pub ant_info: u8,
    pub mod_rfe: u8,
    pub mod_cv: u8,
    pub mod_info: u8,
    pub mod_adie_kt: u8,
    pub wl_gch: u8,
    pub info: u8,
    pub rsvd: u8,
    pub rsvd1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_trx_info_u8 {
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
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_trx_info_v7_u8 {
    pub tx_lvl: u8,
    pub rx_lvl: u8,
    pub wl_rssi: u8,
    pub bt_rssi: u8,
    pub wl_tx_power: i8,
    pub wl_rx_gain: i8,
    pub bt_tx_power: i8,
    pub bt_rx_gain: i8,
    pub zb_tx_power: i8,
    pub zb_rx_gain: i8,
    pub cn: u8,
    pub nhm: i8,
    pub bt_profile: u8,
    pub rsvd2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_trx_info_v107_u8 {
    pub tx_lvl: u8,
    pub rx_lvl: u8,
    pub wl_rssi: u8,
    pub bt_rssi: u8,
    pub wl_tx_power: i8,
    pub wl_rx_gain: i8,
    pub bt_tx_power: i8,
    pub bt_rx_gain: i8,
    pub cn: u8,
    pub nhm: i8,
    pub bt_profile: u8,
    pub rsvd2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_trx_info_le {
    pub tx_rate: __le16,
    pub rx_rate: __le16,
    pub tx_tp: __le32,
    pub rx_tp: __le32,
    pub rx_err_ratio: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxtrx_v9 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub v9_u8: rtw89_btc_trx_info_u8,
    pub v9_le: rtw89_btc_trx_info_le,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxtrx_v7 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub v7_u8: rtw89_btc_trx_info_v7_u8,
    pub v7_le: rtw89_btc_trx_info_le,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxtrx_v107 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub v107_u8: rtw89_btc_trx_info_v107_u8,
    pub v7_le: rtw89_btc_trx_info_le,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxtxpwr_v7 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub pwr: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxtxpwr_v9 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub pwr: u8,
    pub band: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxinit_v7 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub init: rtw89_btc_init_info_v7,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxinit_v107 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub init: rtw89_btc_init_info_v107,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxinit_v10 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub init: rtw89_btc_init_info_v10,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxinit_v11 {
    pub hdr: rtw89_h2c_cxhdr_v7,
    pub init: rtw89_btc_init_info_v11,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_btc_wl_active_role_v101 {
    pub map_role_status: u8,
    pub map_clips_bw: u8,
    pub role: u8,
    pub ch: u8,
    pub noa_duration: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cxrole_v101 {
    pub hdr: rtw89_h2c_cxhdr,
    pub connect_cnt: u8,
    pub link_mode: u8,
    pub role_map: __le16,
    pub act_role: [rtw89_btc_wl_active_role_v101; RTW89_PORT_NUM],
    pub mrole_type: __le32,
    pub mrole_noa_duration: __le32,
    pub map_dbcc_linkmode_chg: __le32,
    pub __packed: },
    pub 0)): *mut *mut u8p_replace_bits((u8 )(cmd) + 2, val, GENMASK(7,,
    pub 0)): *mut *mut u8p_replace_bits((u8 )(cmd) + 3, val, GENMASK(7,,
    pub BIT(0)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(1)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(2)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(3)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(4)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(5)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(6)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(7)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(8)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(9)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(10)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(11)): *mut *mut *mut le16p_replace_bits((__le16 )((u8 )(cmd) + 4), val,,
    pub BIT(0)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val,,
    pub 1)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val, GENMASK(3,,
    pub BIT(4)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val,,
    pub BIT(5)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val,,
    pub 6)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val, GENMASK(7,,
    pub BIT(0)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (7 + (12 + offset)  n), val,,
    pub 1)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (7 + (12 + offset)  n), val, GENMASK(7,,
    pub 0)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (8 + (12 + offset)  n), val, GENMASK(7,,
    pub 0)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (9 + (12 + offset)  n), val, GENMASK(7,,
    pub 0)): *mut *mut *mut *mut le16p_replace_bits((__le16 )((u8 )cmd + (10 + (12 + offset)  n)), val, GENMASK(15,,
    pub 0)): *mut *mut *mut *mut le16p_replace_bits((__le16 )((u8 )cmd + (12 + (12 + offset)  n)), val, GENMASK(15,,
    pub 0)): *mut *mut *mut *mut le16p_replace_bits((__le16 )((u8 )cmd + (14 + (12 + offset)  n)), val, GENMASK(15,,
    pub 0)): *mut *mut *mut *mut le16p_replace_bits((__le16 )((u8 )cmd + (16 + (12 + offset)  n)), val, GENMASK(15,,
    pub 0)): *mut *mut *mut *mut le32p_replace_bits((__le32 )((u8 )cmd + (20 + (12 + offset)  n)), val, GENMASK(31,,
    pub BIT(0)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val,,
    pub 1)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val, GENMASK(3,,
    pub BIT(4)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val,,
    pub BIT(5)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val,,
    pub 6)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (6 + (12 + offset)  n), val, GENMASK(7,,
    pub BIT(0)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (7 + (12 + offset)  n), val,,
    pub 1)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (7 + (12 + offset)  n), val, GENMASK(7,,
    pub 0)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (8 + (12 + offset)  n), val, GENMASK(7,,
    pub 0)): *mut *mut *mut u8p_replace_bits((u8 )cmd + (9 + (12 + offset)  n), val, GENMASK(7,,
    pub 0)): *mut *mut *mut *mut le32p_replace_bits((__le32 )((u8 )cmd + (10 + (12 + offset)  n)), val, GENMASK(31,,
    pub 0)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )cmd + offset), val, GENMASK(31,,
    pub 0)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )cmd + offset + 4), val, GENMASK(31,,
    pub BIT(0)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )cmd + offset + 8), val,,
    pub BIT(1)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )cmd + offset + 8), val,,
    pub 2)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )cmd + offset + 8), val, GENMASK(3,,
    pub BIT(4)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )cmd + offset + 8), val,,
    pub BIT(0)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd) + 2), val,,
    pub BIT(1)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd) + 2), val,,
    pub BIT(2)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd) + 2), val,,
    pub 3)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd) + 2), val, GENMASK(18,,
    pub 0)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd) + 2), val, GENMASK(1,,
    pub 2)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd) + 2), val, GENMASK(5,,
    pub 6)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd) + 2), val, GENMASK(7,,
    pub 8)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd) + 2), val, GENMASK(9,,
    pub 10)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd) + 2), val, GENMASK(17,,
    pub 0)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd)), val, GENMASK(7,,
    pub 8)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd)), val, GENMASK(10,,
    pub 16)): *mut *mut *mut le32p_replace_bits((__le32 )((u8 )(cmd)), val, GENMASK(31,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_chinfo_elem {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_chinfo_elem_be {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_chinfo {
    pub ch_num: u8,
    pub elem_size: u8,
    pub arg: u8,
    pub rsvd0: u8,
    pub __counted_by(ch_num): rtw89_h2c_chinfo_elem elem[],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_chinfo_be {
    pub ch_num: u8,
    pub elem_size: u8,
    pub arg: u8,
    pub rsvd0: u8,
    pub __counted_by(ch_num): rtw89_h2c_chinfo_elem_be elem[],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_scanofld {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub tsf_high: __le32,
    pub tsf_low: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_scanofld_be_macc_role {
    pub w0: __le32,
    pub __packed: },
pub const RTW89_MAX_OP_NUM_BE: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_scanofld_be_opch {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_scanofld_be {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub /: *mut *mut __le32 w9; / Added after SCAN_OFFLOAD_BE_V1,
    pub /: *mut *mut __le32 w10; / Added after SCAN_OFFLOAD_BE_V2,
    pub /: *mut *mut __le32 w11; / Added after SCAN_OFFLOAD_BE_V2,
// struct rtw89_h2c_scanofld_be_macc_role (flexible number)
// struct rtw89_h2c_scanofld_be_opch (flexible number)
// probe SSID list (flexible number); Added after SCAN_OFFLOAD_BE_V3
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_trx_protect {
    pub c0: __le32,
    pub c1: __le32,
    pub w0: __le32,
    pub m0: __le32,
    pub w1: __le32,
    pub m1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_cmd_ofld_arg_src {
    RTW89_FW_CMD_OFLD_SRC_BB,
    RTW89_FW_CMD_OFLD_SRC_RF,
    RTW89_FW_CMD_OFLD_SRC_MAC,
    RTW89_FW_CMD_OFLD_SRC_RF_DDIE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_cmd_ofld_arg_type {
    RTW89_FW_CMD_OFLD_WRITE,
    RTW89_FW_CMD_OFLD_COMPARE,
    RTW89_FW_CMD_OFLD_DELAY,
    RTW89_FW_CMD_OFLD_MOVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_cmd_ofld_arg {
    pub src: rtw89_fw_cmd_ofld_arg_src,
    pub type: rtw89_fw_cmd_ofld_arg_type,
    pub rf_path: rtw89_rf_path,
    pub value: u32,
    pub mask: u32,
    pub offset: u32,
    pub id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_cmd_ofld {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub __packed: },

pub const RTW89_FW_CMD_OFLD_NR: c_int = 125;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_cmd_ofld_info {
    pub pack_level: c_uint,
    pub cnt: u32,
    pub accu_delay: u32,
    pub cmds: [rtw89_h2c_cmd_ofld; RTW89_FW_CMD_OFLD_NR],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_fwips {
    pub w0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mlo_link_cfg {
    pub w0: __le32,
}

// ((__le32 *)cmd + 1) = val;
// ((__le32 *)cmd + 2) = val;
// ((__le32 *)cmd + 3) = val;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_mcc_c2h_rpt_cfg {
    RTW89_FW_MCC_C2H_RPT_OFF	= 0,
    RTW89_FW_MCC_C2H_RPT_FAIL_ONLY	= 1,
    RTW89_FW_MCC_C2H_RPT_ALL	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mcc_add_req {
    pub macid: u8,
    pub central_ch_seg0: u8,
    pub central_ch_seg1: u8,
    pub primary_ch: u8,
    pub 4: rtw89_bandwidth bandwidth:,
    pub 2: u32 group:,
    pub 2: u32 c2h_rpt:,
    pub 1: u32 dis_tx_null:,
    pub 1: u32 dis_sw_retry:,
    pub 1: u32 in_curr_ch:,
    pub 3: u32 sw_retry_count:,
    pub 4: u32 tx_null_early:,
    pub 1: u32 btc_in_2g:,
    pub 1: u32 pta_en:,
    pub 1: u32 rfk_by_pass:,
    pub 2: u32 ch_band_type:,
    pub 9: u32 rsvd0:,
    pub duration: u32,
    pub courtesy_en: u8,
    pub courtesy_num: u8,
    pub courtesy_target: u8,
    pub rsvd1: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_mcc_old_group_actions {
    RTW89_FW_MCC_OLD_GROUP_ACT_NONE = 0,
    RTW89_FW_MCC_OLD_GROUP_ACT_REPLACE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mcc_start_req {
    pub 2: u32 group:,
    pub 1: u32 btc_in_group:,
    pub 2: u32 old_group_action:,
    pub 2: u32 old_group:,
    pub 9: u32 rsvd0:,
    pub 3: u32 notify_cnt:,
    pub 2: u32 rsvd1:,
    pub 1: u32 notify_rxdbg_en:,
    pub 2: u32 rsvd2:,
    pub 8: u32 macid:,
    pub tsf_low: u32,
    pub tsf_high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mcc_tsf_req {
    pub 2: u8 group:,
    pub 6: u8 rsvd0:,
    pub macid_x: u8,
    pub macid_y: u8,
    pub rsvd1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mcc_duration {
    pub 2: u32 group:,
    pub 1: u32 btc_in_group:,
    pub 5: u32 rsvd0:,
    pub 8: u32 start_macid:,
    pub 8: u32 macid_x:,
    pub 8: u32 macid_y:,
    pub start_tsf_low: u32,
    pub start_tsf_high: u32,
    pub duration_x: u32,
    pub duration_y: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_h2c_mrc_sch_types {
    RTW89_H2C_MRC_SCH_BAND0_ONLY = 0,
    RTW89_H2C_MRC_SCH_BAND1_ONLY = 1,
    RTW89_H2C_MRC_SCH_DUAL_BAND = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_h2c_mrc_role_types {
    RTW89_H2C_MRC_ROLE_WIFI = 0,
    RTW89_H2C_MRC_ROLE_BT = 1,
    RTW89_H2C_MRC_ROLE_EMPTY = 2,
}

pub const RTW89_MAC_MRC_MAX_ADD_SLOT_NUM: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mrc_add_slot_arg {
    pub /: *mut *mut u16 duration; / unit: TU,
    pub courtesy_en: bool,
    pub courtesy_period: u8,
    pub /: *mut *mut u8 courtesy_target; / slot idx,
    pub role_num: c_uint,
    pub role_type: rtw89_h2c_mrc_role_types,
    pub is_master: bool,
    pub en_tx_null: bool,
    pub band: rtw89_band,
    pub bw: rtw89_bandwidth,
    pub macid: u8,
    pub central_ch: u8,
    pub primary_ch: u8,
    pub /: *mut *mut u8 null_early; / unit: TU,
// if MLD, for macid: [0, chip::support_mld_num)
// otherwise, for macid: [0, 32)
//
    pub macid_main_bitmap: u32,
// for MLD, bit X maps to macid: X + chip::support_mld_num
    pub macid_paired_bitmap: u32,
    pub roles: [}; RTW89_MAC_MRC_MAX_ADD_ROLE_NUM_PER_SLOT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mrc_add_arg {
    pub sch_idx: u8,
    pub sch_type: rtw89_h2c_mrc_sch_types,
    pub btc_in_sch: bool,
    pub slot_num: c_uint,
    pub slots: [rtw89_fw_mrc_add_slot_arg; RTW89_MAC_MRC_MAX_ADD_SLOT_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mrc_add_role {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub macid_main_bitmap: __le32,
    pub macid_paired_bitmap: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mrc_add_slot {
    pub w0: __le32,
    pub w1: __le32,
    pub roles: [rtw89_h2c_mrc_add_role; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mrc_add {
    pub w0: __le32,
// Logically append flexible struct rtw89_h2c_mrc_add_slot, but there
// are other flexible array inside it. We cannot access them correctly
// through this struct. So, in case misusing, we don't really declare
// it here.
//
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_h2c_mrc_start_actions {
    RTW89_H2C_MRC_START_ACTION_START_NEW = 0,
    RTW89_H2C_MRC_START_ACTION_REPLACE_OLD = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mrc_start_arg {
    pub sch_idx: u8,
    pub old_sch_idx: u8,
    pub start_tsf: u64,
    pub action: rtw89_h2c_mrc_start_actions,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mrc_start {
    pub w0: __le32,
    pub start_tsf_low: __le32,
    pub start_tsf_high: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mrc_del {
    pub w0: __le32,
    pub __packed: },

pub const RTW89_MAC_MRC_MAX_REQ_TSF_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mrc_req_tsf_arg {
    pub num: c_uint,
    pub band: u8,
    pub port: u8,
    pub infos: [}; RTW89_MAC_MRC_MAX_REQ_TSF_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mrc_req_tsf {
    pub req_tsf_num: u8,
    pub __counted_by(req_tsf_num): u8 infos[],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_h2c_mrc_upd_bitmap_actions {
    RTW89_H2C_MRC_UPD_BITMAP_ACTION_DEL = 0,
    RTW89_H2C_MRC_UPD_BITMAP_ACTION_ADD = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mrc_upd_bitmap_arg {
    pub sch_idx: u8,
    pub macid: u8,
    pub client_macid: u8,
    pub action: rtw89_h2c_mrc_upd_bitmap_actions,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mrc_upd_bitmap {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mrc_sync_arg {
    pub /: *mut *mut u8 offset; / unit: TU,
    pub band: u8,
    pub port: u8,
    pub dest: } src,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mrc_sync {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_mrc_upd_duration_arg {
    pub sch_idx: u8,
    pub start_tsf: u64,
    pub slot_num: c_uint,
    pub slot_idx: u8,
    pub /: *mut *mut u16 duration; / unit: TU,
    pub slots: [}; RTW89_MAC_MRC_MAX_ADD_SLOT_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mrc_upd_duration {
    pub w0: __le32,
    pub start_tsf_low: __le32,
    pub start_tsf_high: __le32,
    pub slots: [__le32; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_wow_aoac {
    pub w0: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ap_info {
    pub w0: __le32,
    pub __packed: },

pub const RTW89_C2H_HEADER_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_hdr {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_c2h_attr {
    pub category: u8,
    pub class: u8,
    pub func: u8,
    pub len: u16,
    pub 1: u8 is_scan_event:,
    pub 2: u8 scan_seq:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_done_ack {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rev_ack {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_c2h_log_fmt {
    pub signature: __le16,
    pub feature: u8,
    pub syntax: u8,
    pub fmt_id: __le32,
    pub file_num: u8,
    pub line_num: __le16,
    pub argc: u8,
    pub raw): DECLARE_FLEX_ARRAY(u8,,
    pub argv): DECLARE_FLEX_ARRAY(__le32,,
    pub u: } __packed,
    pub __packed: },
pub const RTW89_C2H_FW_FORMATTED_LOG_MIN_LEN: c_int = 11;

pub const RTW89_C2H_FW_LOG_MAX_PARA_NUM: c_int = 16;
pub const RTW89_C2H_FW_LOG_SIGNATURE: c_uint = 0xA5A5;
pub const RTW89_C2H_FW_LOG_STR_BUF_SIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_bcn_upd_done {
    pub hdr: rtw89_c2h_hdr,
    pub w2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_mac_bcnfltr_rpt {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_ra_rpt {
    pub hdr: rtw89_c2h_hdr,
    pub w2: __le32,
    pub w3: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_lps_rpt {
    pub hdr: rtw89_c2h_hdr,
    pub type: u8,
    pub cnt_bbcr: u8,
    pub cnt_bbmcucr: u8,
    pub cnt_rfcr: u8,
    pub data: [u8; ],
//
// The layout of data:
// u8 info[][4], size = total_len - size of below fields
// __le16 bbcr_addr[], size = cnt_bbcr
// __le32 bbcr_data[], size = cnt_bbcr
// __le16 bbmcucr_addr[], size = cnt_bbmcucr
// __le32 bbmcucr_data[], size = cnt_bbmcucr
// __le16 rfcr_addr[],   size = cnt_rfcr
// __le32 rfcr_data_a[], size = cnt_rfcr
// __le32 rfcr_data_b[], size = cnt_rfcr
//
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_ra_tx_history {
    pub hdr: rtw89_c2h_hdr,
    pub ra_tbtt_cnt: __le32,
    pub tx_rate_tot_cnt_hist: [__le32; RTW89_TX_RATE_NR],
    pub tx_cat_cnt: [__le32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_fw_scan_rpt {
    pub hdr: rtw89_c2h_hdr,
    pub phy_idx: u8,
    pub band: u8,
    pub center_ch: u8,
    pub /: *mut *mut u8 ofdm_pd_idx; / in unit of 2 dBm,
pub const PD_LOWER_BOUND_BASE: c_int = 102;
    pub cck_pd_idx: i8,
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub __packed: },
// For WiFi 6 chips:
// VHT, HE, HT-old: [6:4]: NSS, [3:0]: MCS
// HT-new: [6:5]: NA, [4:0]: MCS
// For WiFi 7 chips (V1):
// HT, VHT, HE, EHT: [7:5]: NSS, [4:0]: MCS
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_scanofld {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_mac_tx_rpt {
    pub hdr: rtw89_c2h_hdr,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_mac_tx_rpt_v2 {
    pub hdr: rtw89_c2h_hdr,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub w10: __le32,
    pub w11: __le32,
    pub w12: __le32,
    pub w13: __le32,
    pub w14: __le32,
    pub w15: __le32,
    pub w16: __le32,
    pub w17: __le32,
    pub w18: __le32,
    pub w19: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_mcc_tsf_rpt {
    pub macid_x: u32,
    pub macid_y: u32,
    pub tsf_x_low: u32,
    pub tsf_x_high: u32,
    pub tsf_y_low: u32,
    pub tsf_y_high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_mlo_link_cfg_rpt {
    pub hdr: rtw89_c2h_hdr,
    pub w2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_c2h_mlo_link_status {
    RTW89_C2H_MLO_LINK_CFG_IDLE = 0,
    RTW89_C2H_MLO_LINK_CFG_DONE = 1,
    RTW89_C2H_MLO_LINK_CFG_ISSUE_NULL_FAIL = 2,
    RTW89_C2H_MLO_LINK_CFG_TX_NULL_FAIL = 3,
    RTW89_C2H_MLO_LINK_CFG_ROLE_NOT_EXIST = 4,
    RTW89_C2H_MLO_LINK_CFG_NULL_1_TIMEOUT = 5,
    RTW89_C2H_MLO_LINK_CFG_NULL_0_TIMEOUT = 6,
    RTW89_C2H_MLO_LINK_CFG_RUNNING = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mac_mrc_tsf_rpt {
    pub num: c_uint,
    pub tsfs: [u64; RTW89_MAC_MRC_MAX_REQ_TSF_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_mrc_tsf_rpt_info {
    pub tsf_low: __le32,
    pub tsf_high: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_mrc_tsf_rpt {
    pub hdr: rtw89_c2h_hdr,
    pub w2: __le32,
    pub infos: [rtw89_c2h_mrc_tsf_rpt_info; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_mrc_status_rpt {
    pub hdr: rtw89_c2h_hdr,
    pub w2: __le32,
    pub tsf_low: __le32,
    pub tsf_high: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_pkt_ofld_rsp {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_tx_duty_rpt {
    pub c2h_hdr: rtw89_c2h_hdr,
    pub w2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_wow_aoac_report {
    pub c2h_hdr: rtw89_c2h_hdr,
    pub rpt_ver: u8,
    pub sec_type: u8,
    pub key_idx: u8,
    pub pattern_idx: u8,
    pub rekey_ok: u8,
    pub rsvd1: [u8; 3],
    pub ptk_tx_iv: [u8; 8],
    pub eapol_key_replay_count: [u8; 8],
    pub gtk: [u8; 32],
    pub ptk_rx_iv: [u8; 8],
    pub gtk_rx_iv: [u8; 4][8],
    pub igtk_key_id: __le64,
    pub igtk_ipn: __le64,
    pub igtk: [u8; 32],
    pub csa_pri_ch: u8,
    pub csa_bw_ch_offset: u8,
    pub csa_ch_band_chsw_failed: u8,
    pub csa_rsvd1: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_pwr_int_notify {
    pub hdr: rtw89_c2h_hdr,
    pub w2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_tx_duty {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_bcnfltr {
    pub w0: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ofld_rssi {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_ofld {
    pub w0: __le32,
    pub __packed: },

pub const RTW89_MFW_SIG: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mfw_info {
    pub cv: u8,
    pub /: *mut *mut u8 type; / enum rtw89_fw_type,
    pub mp: u8,
    pub rsvd: u8,
    pub shift: __le32,
    pub size: __le32,
    pub rsvd2: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_mfw_hdr {
    pub /: *mut *mut u8 sig; / RTW89_MFW_SIG,
    pub fw_nr: u8,
    pub rsvd0: [u8; 2],
    pub major: u8,
    pub minor: u8,
    pub sub: u8,
    pub idx: u8,
    pub ver: },
    pub rsvd1: [u8; 8],
    pub info: [rtw89_mfw_info; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_logsuit_hdr {
    pub rsvd: __le32,
    pub count: __le32,
    pub ids: [__le32; ],
    pub __packed: },
pub const RTW89_FW_ELEMENT_ALIGN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_element_id {
    RTW89_FW_ELEMENT_ID_BBMCU0 = 0,
    RTW89_FW_ELEMENT_ID_BBMCU1 = 1,
    RTW89_FW_ELEMENT_ID_BB_REG = 2,
    RTW89_FW_ELEMENT_ID_BB_GAIN = 3,
    RTW89_FW_ELEMENT_ID_RADIO_A = 4,
    RTW89_FW_ELEMENT_ID_RADIO_B = 5,
    RTW89_FW_ELEMENT_ID_RADIO_C = 6,
    RTW89_FW_ELEMENT_ID_RADIO_D = 7,
    RTW89_FW_ELEMENT_ID_RF_NCTL = 8,
    RTW89_FW_ELEMENT_ID_TXPWR_BYRATE = 9,
    RTW89_FW_ELEMENT_ID_TXPWR_LMT_2GHZ = 10,
    RTW89_FW_ELEMENT_ID_TXPWR_LMT_5GHZ = 11,
    RTW89_FW_ELEMENT_ID_TXPWR_LMT_6GHZ = 12,
    RTW89_FW_ELEMENT_ID_TXPWR_LMT_RU_2GHZ = 13,
    RTW89_FW_ELEMENT_ID_TXPWR_LMT_RU_5GHZ = 14,
    RTW89_FW_ELEMENT_ID_TXPWR_LMT_RU_6GHZ = 15,
    RTW89_FW_ELEMENT_ID_TX_SHAPE_LMT = 16,
    RTW89_FW_ELEMENT_ID_TX_SHAPE_LMT_RU = 17,
    RTW89_FW_ELEMENT_ID_TXPWR_TRK = 18,
    RTW89_FW_ELEMENT_ID_RFKLOG_FMT = 19,
    RTW89_FW_ELEMENT_ID_REGD = 20,
    RTW89_FW_ELEMENT_ID_TXPWR_DA_LMT_2GHZ = 21,
    RTW89_FW_ELEMENT_ID_TXPWR_DA_LMT_5GHZ = 22,
    RTW89_FW_ELEMENT_ID_TXPWR_DA_LMT_6GHZ = 23,
    RTW89_FW_ELEMENT_ID_TXPWR_DA_LMT_RU_2GHZ = 24,
    RTW89_FW_ELEMENT_ID_TXPWR_DA_LMT_RU_5GHZ = 25,
    RTW89_FW_ELEMENT_ID_TXPWR_DA_LMT_RU_6GHZ = 26,
    RTW89_FW_ELEMENT_ID_AFE_PWR_SEQ = 27,
    RTW89_FW_ELEMENT_ID_DIAG_MAC = 28,
    RTW89_FW_ELEMENT_ID_TX_COMP = 29,

    __RTW89_FW_ELEMENT_ID_INTL_TRANSITION,
    RTW89_FW_ELEMENT_ID_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __rtw89_fw_txpwr_element {
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rfe_type: u8,
    pub ent_sz: u8,
    pub num_ents: __le32,
    pub content: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __rtw89_fw_regd_element {
    pub rsvd0: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub ent_sz: u8,
    pub num_ents: __le32,
    pub content: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_txpwr_trk_type {
    __RTW89_FW_TXPWR_TRK_TYPE_6GHZ_START = 0,
    RTW89_FW_TXPWR_TRK_TYPE_6GB_N = 0,
    RTW89_FW_TXPWR_TRK_TYPE_6GB_P = 1,
    RTW89_FW_TXPWR_TRK_TYPE_6GA_N = 2,
    RTW89_FW_TXPWR_TRK_TYPE_6GA_P = 3,
    __RTW89_FW_TXPWR_TRK_TYPE_6GHZ_MAX = 3,

    __RTW89_FW_TXPWR_TRK_TYPE_5GHZ_START = 4,
    RTW89_FW_TXPWR_TRK_TYPE_5GB_N = 4,
    RTW89_FW_TXPWR_TRK_TYPE_5GB_P = 5,
    RTW89_FW_TXPWR_TRK_TYPE_5GA_N = 6,
    RTW89_FW_TXPWR_TRK_TYPE_5GA_P = 7,
    __RTW89_FW_TXPWR_TRK_TYPE_5GHZ_MAX = 7,

    __RTW89_FW_TXPWR_TRK_TYPE_2GHZ_START = 8,
    RTW89_FW_TXPWR_TRK_TYPE_2GB_N = 8,
    RTW89_FW_TXPWR_TRK_TYPE_2GB_P = 9,
    RTW89_FW_TXPWR_TRK_TYPE_2GA_N = 10,
    RTW89_FW_TXPWR_TRK_TYPE_2GA_P = 11,
    RTW89_FW_TXPWR_TRK_TYPE_2G_CCK_B_N = 12,
    RTW89_FW_TXPWR_TRK_TYPE_2G_CCK_B_P = 13,
    RTW89_FW_TXPWR_TRK_TYPE_2G_CCK_A_N = 14,
    RTW89_FW_TXPWR_TRK_TYPE_2G_CCK_A_P = 15,
    __RTW89_FW_TXPWR_TRK_TYPE_2GHZ_MAX = 15,

    RTW89_FW_TXPWR_TRK_TYPE_NR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_txpwr_track_cfg {
    pub (*delta[RTW89_FW_TXPWR_TRK_TYPE_NR])[DELTA_SWINGIDX_SIZE]: *const i8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_afe_action {
    RTW89_FW_AFE_ACTION_WRITE = 0,
    RTW89_FW_AFE_ACTION_DELAY = 1,
    RTW89_FW_AFE_ACTION_POLL = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_afe_cat {
    RTW89_FW_AFE_CAT_BB = 0,
    RTW89_FW_AFE_CAT_BB1 = 1,
    RTW89_FW_AFE_CAT_MAC = 2,
    RTW89_FW_AFE_CAT_MAC1 = 3,
    RTW89_FW_AFE_CAT_AFEDIG = 4,
    RTW89_FW_AFE_CAT_AFEDIG1 = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_afe_class {
    RTW89_FW_AFE_CLASS_P0 = 0,
    RTW89_FW_AFE_CLASS_P1 = 1,
    RTW89_FW_AFE_CLASS_P2 = 2,
    RTW89_FW_AFE_CLASS_P3 = 3,
    RTW89_FW_AFE_CLASS_P4 = 4,
    RTW89_FW_AFE_CLASS_CMN = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_element_hdr {
    pub /: *mut *mut __le32 id; / enum rtw89_fw_element_id,
    pub /: *mut *mut __le32 size; / exclude header size,
    pub ver: [u8; 4],
    pub /: *mut *mut __le16 aid; / should match rtw89_hal::aid,
    pub rsvd0: __le16,
    pub rsvd1: __le32,
    pub rsvd2: __le32,
    pub priv: [u8; 8],
    pub contents: [u8; ],
    pub common: } __packed,
    pub idx: u8,
    pub rsvd: [u8; 7],
    pub addr: __le32,
    pub data: __le32,
    pub regs: [} __packed; ],
    pub reg2: } __packed,
    pub cv: u8,
    pub priv: [u8; 7],
    pub contents: [u8; ],
    pub bbmcu: } __packed,
    pub /: *mut *mut __le32 bitmap; / bitmap of enum rtw89_fw_txpwr_trk_type,
    pub rsvd: __le32,
    pub contents: [i8; ][DELTA_SWINGIDX_SIZE],
    pub txpwr_trk: } __packed,
    pub nr: u8,
    pub rsvd: [u8; 3],
    pub /: *mut *mut u8 rfk_id; / enum rtw89_phy_c2h_rfk_log_func,
    pub rsvd1: [u8; 3],
    pub offset: [__le16; ],
    pub rfk_log_fmt: } __packed,
    pub rsvd: [u8; 8],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_afe_info {
    pub /: *mut *mut __le32 action; / enum rtw89_fw_afe_action,
    pub /: *mut *mut __le32 cat; / enum rtw89_fw_afe_cat,
    pub /: *mut *mut __le32 class; / enum rtw89_fw_afe_class,
    pub addr: __le32,
    pub mask: __le32,
    pub val: __le32,
    pub infos: [} __packed; ],
    pub afe: } __packed,
    pub rule_size: __le32,
    pub rsvd: [u8; 4],
    pub rules_and_msgs: [u8; ],
    pub diag_mac: } __packed,
    pub rfe_type: u8,
    pub priv: [u8; 7],
    pub contents: [u8; ],
    pub tx_comp: } __packed,
    pub txpwr: __rtw89_fw_txpwr_element,
    pub regd: __rtw89_fw_regd_element,
    pub u: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwcmd_hdr {
    pub hdr0: __le32,
    pub hdr1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_compat_fw_hdr {
    pub mfw_hdr: rtw89_mfw_hdr,
    pub fw_hdr: rtw89_fw_hdr,
}

extern "C" {
    pub fn RTW89_MFW_HDR_VER_CODE(_arg: &compat->mfw_hdr) -> return;
}
extern "C" {
    pub fn RTW89_FW_HDR_VER_CODE(_arg: &compat->fw_hdr) -> return;
}
pub const RTW89_H2C_RF_PAGE_SIZE: c_int = 500;
pub const RTW89_H2C_RF_PAGE_NUM: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rf_reg_info {
    pub rf_path: rtw89_rf_path,
    pub rtw89_phy_config_rf_h2c: [__le32; RTW89_H2C_RF_PAGE_NUM][RTW89_H2C_RF_PAGE_SIZE],
    pub curr_idx: u16,
}

pub const H2C_SEC_CAM_LEN: c_int = 24;
pub const H2C_HEADER_LEN: c_int = 8;

pub const FWCMD_TYPE_H2C: c_int = 0;
pub const H2C_CAT_TEST: c_uint = 0x0;
// CLASS 5 - FW STATUS TEST
pub const H2C_CL_FW_STATUS_TEST: c_uint = 0x5;
pub const H2C_FUNC_CPU_EXCEPTION: c_uint = 0x1;
pub const H2C_CAT_MAC: c_uint = 0x1;
// CLASS 0 - FW INFO
pub const H2C_CL_FW_INFO: c_uint = 0x0;
pub const H2C_FUNC_LOG_CFG: c_uint = 0x0;
pub const H2C_FUNC_MAC_GENERAL_PKT: c_uint = 0x1;
// CLASS 1 - WOW
pub const H2C_CL_MAC_WOW: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_wow_h2c_func {
    H2C_FUNC_KEEP_ALIVE		= 0x0,
    H2C_FUNC_DISCONNECT_DETECT	= 0x1,
    H2C_FUNC_WOW_GLOBAL		= 0x2,
    H2C_FUNC_GTK_OFLD		= 0x3,
    H2C_FUNC_ARP_OFLD		= 0x4,
    H2C_FUNC_NLO			= 0x7,
    H2C_FUNC_WAKEUP_CTRL		= 0x8,
    H2C_FUNC_WOW_CAM_UPD		= 0xC,
    H2C_FUNC_AOAC_REPORT_REQ	= 0xD,
    H2C_FUNC_WOW_PLD_CAM_UPD	= 0x12,

    NUM_OF_RTW89_WOW_H2C_FUNC,
}

// CLASS 2 - PS
pub const H2C_CL_MAC_PS: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ps_h2c_func {
    H2C_FUNC_MAC_LPS_PARM		= 0x0,
    H2C_FUNC_P2P_ACT		= 0x1,
    H2C_FUNC_IPS_CFG		= 0x3,
    H2C_FUNC_PS_POWER_LEVEL		= 0x7,
    H2C_FUNC_TBTT_TUNING		= 0xA,

    NUM_OF_RTW89_PS_H2C_FUNC,
}

// CLASS 3 - FW download
pub const H2C_CL_MAC_FWDL: c_uint = 0x3;
pub const H2C_FUNC_MAC_FWHDR_DL: c_uint = 0x0;
// CLASS 5 - Frame Exchange
pub const H2C_CL_MAC_FR_EXCHG: c_uint = 0x5;
pub const H2C_FUNC_MAC_CCTLINFO_UD: c_uint = 0x2;
pub const H2C_FUNC_MAC_BCN_UPD: c_uint = 0x5;
pub const H2C_FUNC_MAC_DCTLINFO_UD_V1: c_uint = 0x9;
pub const H2C_FUNC_MAC_CCTLINFO_UD_V1: c_uint = 0xa;
pub const H2C_FUNC_MAC_DCTLINFO_UD_V2: c_uint = 0xc;
pub const H2C_FUNC_MAC_BCN_UPD_BE: c_uint = 0xd;
pub const H2C_FUNC_MAC_DCTLINFO_UD_V3: c_uint = 0x10;
pub const H2C_FUNC_MAC_CCTLINFO_UD_G7: c_uint = 0x11;
// CLASS 6 - Address CAM
pub const H2C_CL_MAC_ADDR_CAM_UPDATE: c_uint = 0x6;
pub const H2C_FUNC_MAC_ADDR_CAM_UPD: c_uint = 0x0;
// CLASS 8 - Media Status Report
pub const H2C_CL_MAC_MEDIA_RPT: c_uint = 0x8;
pub const H2C_FUNC_MAC_JOININFO: c_uint = 0x0;
pub const H2C_FUNC_MAC_FWROLE_MAINTAIN: c_uint = 0x4;
pub const H2C_FUNC_NOTIFY_DBCC: c_uint = 0x5;
// CLASS 9 - FW offload
pub const H2C_CL_MAC_FW_OFLD: c_uint = 0x9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_fw_ofld_h2c_func {
    H2C_FUNC_PACKET_OFLD		= 0x1,
    H2C_FUNC_MAC_MACID_PAUSE	= 0x8,
    H2C_FUNC_USR_EDCA		= 0xF,
    H2C_FUNC_TSF32_TOGL		= 0x10,
    H2C_FUNC_CMD_OFLD_PKT		= 0x13,
    H2C_FUNC_OFLD_CFG		= 0x14,
    H2C_FUNC_ADD_SCANOFLD_CH	= 0x16,
    H2C_FUNC_SCANOFLD		= 0x17,
    H2C_FUNC_TX_DUTY		= 0x18,
    H2C_FUNC_PKT_DROP		= 0x1b,
    H2C_FUNC_CFG_BCNFLTR		= 0x1e,
    H2C_FUNC_OFLD_RSSI		= 0x1f,
    H2C_FUNC_OFLD_TP		= 0x20,
    H2C_FUNC_MAC_MACID_PAUSE_SLEEP	= 0x28,
    H2C_FUNC_SCANOFLD_BE		= 0x2c,
    H2C_FUNC_TRX_PROTECT		= 0x34,

    NUM_OF_RTW89_FW_OFLD_H2C_FUNC,
}

// CLASS 10 - Security CAM
pub const H2C_CL_MAC_SEC_CAM: c_uint = 0xa;
pub const H2C_FUNC_MAC_SEC_UPD: c_uint = 0x1;
// CLASS 12 - BA CAM
pub const H2C_CL_BA_CAM: c_uint = 0xc;
pub const H2C_FUNC_MAC_BA_CAM: c_uint = 0x0;
pub const H2C_FUNC_MAC_BA_CAM_V1: c_uint = 0x1;
pub const H2C_FUNC_MAC_BA_CAM_INIT: c_uint = 0x2;
// CLASS 14 - MCC
pub const H2C_CL_MCC: c_uint = 0xe;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mcc_h2c_func {
    H2C_FUNC_ADD_MCC		= 0x0,
    H2C_FUNC_START_MCC		= 0x1,
    H2C_FUNC_STOP_MCC		= 0x2,
    H2C_FUNC_DEL_MCC_GROUP		= 0x3,
    H2C_FUNC_RESET_MCC_GROUP	= 0x4,
    H2C_FUNC_MCC_REQ_TSF		= 0x5,
    H2C_FUNC_MCC_MACID_BITMAP	= 0x6,
    H2C_FUNC_MCC_SYNC		= 0x7,
    H2C_FUNC_MCC_SET_DURATION	= 0x8,

    NUM_OF_RTW89_MCC_H2C_FUNC,
}

// CLASS 20 - MLO
pub const H2C_CL_MLO: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mlo_h2c_func {
    H2C_FUNC_MLO_TBL_CFG		= 0x0,
    H2C_FUNC_MLO_STA_CFG		= 0x1,
    H2C_FUNC_MLO_TTLM		= 0x2,
    H2C_FUNC_MLO_DM_CFG		= 0x3,
    H2C_FUNC_MLO_EMLSR_STA_CFG	= 0x4,
    H2C_FUNC_MLO_MCMLO_RELINK_DROP	= 0x5,
    H2C_FUNC_MLO_MCMLO_SN_SYNC	= 0x6,
    H2C_FUNC_MLO_RELINK		= 0x7,
    H2C_FUNC_MLO_LINK_CFG		= 0x8,
    H2C_FUNC_MLO_DM_DBG		= 0x9,

    NUM_OF_RTW89_MLO_H2C_FUNC,
}

// CLASS 24 - MRC
pub const H2C_CL_MRC: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mrc_h2c_func {
    H2C_FUNC_MRC_REQ_TSF		= 0x0,
    H2C_FUNC_ADD_MRC		= 0x1,
    H2C_FUNC_START_MRC		= 0x2,
    H2C_FUNC_DEL_MRC		= 0x3,
    H2C_FUNC_MRC_SYNC		= 0x4,
    H2C_FUNC_MRC_UPD_DURATION	= 0x5,
    H2C_FUNC_MRC_UPD_BITMAP		= 0x6,

    NUM_OF_RTW89_MRC_H2C_FUNC,
}

// can consider MRC's sch_idx as MCC's group

// CLASS 36 - AP
pub const H2C_CL_AP: c_uint = 0x24;
pub const H2C_FUNC_AP_INFO: c_uint = 0x0;
pub const H2C_CAT_OUTSRC: c_uint = 0x2;
pub const H2C_CL_OUTSRC_RA: c_uint = 0x1;
pub const H2C_FUNC_OUTSRC_RA_MACIDCFG: c_uint = 0x0;
pub const H2C_FUNC_OUTSRC_RA_TX_HISTORY: c_uint = 0x9;
pub const H2C_FUNC_OUTSRC_RA_PHY_CH_RPT: c_uint = 0xe;
pub const H2C_FUNC_OUTSRC_RA_DRV_CTRL_FW: c_uint = 0xf;
pub const H2C_CL_OUTSRC_DM: c_uint = 0x2;
pub const H2C_FUNC_FW_MCC_DIG: c_uint = 0x6;
pub const H2C_FUNC_FW_LPS_CH_INFO: c_uint = 0xb;
pub const H2C_FUNC_FW_LPS_ML_CMN_INFO: c_uint = 0xe;
pub const H2C_CL_OUTSRC_RF_REG_A: c_uint = 0x8;
pub const H2C_CL_OUTSRC_RF_REG_B: c_uint = 0x9;
pub const H2C_CL_OUTSRC_RF_FW_NOTIFY: c_uint = 0xa;
pub const H2C_FUNC_OUTSRC_RF_GET_MCCCH: c_uint = 0x2;
pub const H2C_FUNC_OUTSRC_RF_MCC_INFO: c_uint = 0xf;
pub const H2C_FUNC_OUTSRC_RF_PS_INFO: c_uint = 0x10;
pub const H2C_CL_OUTSRC_RF_FW_RFK: c_uint = 0xb;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rfk_offload_h2c_func {
    H2C_FUNC_RFK_TSSI_OFFLOAD = 0x0,
    H2C_FUNC_RFK_IQK_OFFLOAD = 0x1,
    H2C_FUNC_RFK_DPK_OFFLOAD = 0x3,
    H2C_FUNC_RFK_TXGAPK_OFFLOAD = 0x4,
    H2C_FUNC_RFK_DACK_OFFLOAD = 0x5,
    H2C_FUNC_RFK_RXDCK_OFFLOAD = 0x6,
    H2C_FUNC_RFK_PRE_NOTIFY = 0x8,
    H2C_FUNC_RFK_TAS_OFFLOAD = 0x9,
    H2C_FUNC_RFK_TXIQK_OFFOAD = 0xc,
    H2C_FUNC_RFK_CIM3K_OFFOAD = 0xe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rf_get_mccch {
    pub ch_0_0: __le32,
    pub ch_0_1: __le32,
    pub ch_1_0: __le32,
    pub ch_1_1: __le32,
    pub current_channel: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rf_get_mccch_v0 {
    pub ch_0: __le32,
    pub ch_1: __le32,
    pub band_0: __le32,
    pub band_1: __le32,
    pub current_channel: __le32,
    pub current_band_type: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_mcc_dig {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub __packed: },

pub const NUM_OF_RTW89_FW_RFK_PATH: c_int = 2;
pub const NUM_OF_RTW89_FW_RFK_TBL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_ps_info {
    pub rf18: [__le32; NUM_OF_RTW89_FW_RFK_PATH],
    pub mlo_mode: __le32,
    pub pri_ch: [u8; NUM_OF_RTW89_FW_RFK_PATH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rfk_pre_info_common {
    pub ch: [__le32; NUM_OF_RTW89_FW_RFK_PATH][NUM_OF_RTW89_FW_RFK_TBL],
    pub band: [__le32; NUM_OF_RTW89_FW_RFK_PATH][NUM_OF_RTW89_FW_RFK_TBL],
    pub dbcc: } __packed,
    pub mlo_mode: __le32,
    pub cur_ch: [__le32; NUM_OF_RTW89_FW_RFK_PATH],
    pub cur_band: [__le32; NUM_OF_RTW89_FW_RFK_PATH],
    pub tbl: } __packed,
    pub phy_idx: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rfk_pre_info_v0 {
    pub common: rtw89_fw_h2c_rfk_pre_info_common,
    pub cur_band: __le32,
    pub cur_bw: __le32,
    pub cur_center_ch: __le32,
    pub ktbl_sel0: __le32,
    pub ktbl_sel1: __le32,
    pub rfmod0: __le32,
    pub rfmod1: __le32,
    pub mlo_1_1: __le32,
    pub rfe_type: __le32,
    pub drv_mode: __le32,
    pub ch: [__le32; NUM_OF_RTW89_FW_RFK_PATH],
    pub band: [__le32; NUM_OF_RTW89_FW_RFK_PATH],
    pub mlo: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rfk_pre_info_v1 {
    pub common: rtw89_fw_h2c_rfk_pre_info_common,
    pub mlo_1_1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rfk_pre_info_v2 {
    pub base_v1: rtw89_fw_h2c_rfk_pre_info_v1,
    pub cur_bandwidth: [__le32; NUM_OF_RTW89_FW_RFK_PATH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rfk_pre_info {
    pub mlo_mode: __le32,
    pub phy_idx: __le32,
    pub mlo_1_1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rfk_pre_info_mcc_v0 {
    pub tbl_18: [__le32; NUM_OF_RTW89_FW_RFK_TBL][NUM_OF_RTW89_FW_RFK_PATH],
    pub cur_18: [__le32; NUM_OF_RTW89_FW_RFK_PATH],
    pub mlo_mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rfk_pre_info_mcc_v1 {
    pub tbl_18: [__le32; NUM_OF_RTW89_FW_RFK_TBL],
    pub cur_18: [__le32; NUM_OF_RTW89_FW_RFK_PATH],
    pub mlo_mode: __le32,
    pub mlo_1_1: __le32,
    pub phy_idx: u8,
    pub tbl_idx: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_h2c_rfk_pre_info_mcc {
    pub base: rtw89_fw_h2c_rfk_pre_info_mcc_v1,
    pub rsvd: [u8; 2],
    pub aid: __le32,
    pub acv: u8,
    pub rsvd2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_tssi {
    pub len: __le16,
    pub phy: u8,
    pub ch: u8,
    pub bw: u8,
    pub band: u8,
    pub hwtx_en: u8,
    pub cv: u8,
    pub curr_tssi_cck_de: [i8; 2],
    pub curr_tssi_cck_de_20m: [i8; 2],
    pub curr_tssi_cck_de_40m: [i8; 2],
    pub curr_tssi_efuse_cck_de: [i8; 2],
    pub curr_tssi_ofdm_de: [i8; 2],
    pub curr_tssi_ofdm_de_20m: [i8; 2],
    pub curr_tssi_ofdm_de_40m: [i8; 2],
    pub curr_tssi_ofdm_de_80m: [i8; 2],
    pub curr_tssi_ofdm_de_160m: [i8; 2],
    pub curr_tssi_ofdm_de_320m: [i8; 2],
    pub curr_tssi_efuse_ofdm_de: [i8; 2],
    pub curr_tssi_ofdm_de_diff_20m: [i8; 2],
    pub curr_tssi_ofdm_de_diff_80m: [i8; 2],
    pub curr_tssi_ofdm_de_diff_160m: [i8; 2],
    pub curr_tssi_ofdm_de_diff_320m: [i8; 2],
    pub curr_tssi_trim_de: [i8; 2],
    pub pg_thermal: [u8; 2],
    pub ftable: [u8; 2][128],
    pub tssi_mode: u8,
    pub rfe_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_iqk_v0 {
    pub phy_idx: __le32,
    pub dbcc: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_iqk {
    pub len: u8,
    pub ktype: u8,
    pub phy: u8,
    pub kpath: u8,
    pub band: u8,
    pub bw: u8,
    pub ch: u8,
    pub cv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_dpk {
    pub len: u8,
    pub phy: u8,
    pub dpk_enable: u8,
    pub kpath: u8,
    pub cur_band: u8,
    pub cur_bw: u8,
    pub cur_ch: u8,
    pub dpk_dbg_en: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_txgapk {
    pub len: u8,
    pub ktype: u8,
    pub phy: u8,
    pub kpath: u8,
    pub band: u8,
    pub bw: u8,
    pub ch: u8,
    pub cv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_dack {
    pub len: u8,
    pub phy: u8,
    pub type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_rxdck_v0 {
    pub len: u8,
    pub phy: u8,
    pub is_afe: u8,
    pub kpath: u8,
    pub cur_band: u8,
    pub cur_bw: u8,
    pub cur_ch: u8,
    pub rxdck_dbg_en: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_tas {
    pub enable: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_rxdck {
    pub v0: rtw89_h2c_rf_rxdck_v0,
    pub is_chl_k: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_txiqk_v0 {
    pub len: u8,
    pub phy: u8,
    pub txiqk_enable: u8,
    pub is_wb_txiqk: u8,
    pub kpath: u8,
    pub cur_band: u8,
    pub cur_bw: u8,
    pub cur_ch: u8,
    pub txiqk_dbg_en: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_txiqk {
    pub v0: rtw89_h2c_rf_txiqk_v0,
    pub is_ther_rek: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_rf_cim3k {
    pub len: u8,
    pub phy: u8,
    pub su_cim3k_enable: [u8; 2],
    pub ru_cim3k_enable: [u8; 2],
    pub kpath: u8,
    pub cur_band: u8,
    pub cur_bw: u8,
    pub cur_ch: u8,
    pub cim3k_dbg_en: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rf_log_type {
    RTW89_RF_RUN_LOG = 0,
    RTW89_RF_RPT_LOG = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_log_hdr {
    pub /: *mut *mut u8 type; / enum rtw89_rf_log_type,
    pub len: __le16,
    pub content: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_run_log {
    pub fmt_idx: __le32,
    pub arg: [__le32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_iqk_rpt_log {
    pub iqk_tx_fail: [bool; 2],
    pub iqk_rx_fail: [bool; 2],
    pub is_iqk_init: bool,
    pub is_reload: bool,
    pub is_wb_txiqk: [bool; 2],
    pub is_wb_rxiqk: [bool; 2],
    pub is_nbiqk: bool,
    pub txiqk_en: bool,
    pub rxiqk_en: bool,
    pub lok_en: bool,
    pub iqk_xym_en: bool,
    pub iqk_sram_en: bool,
    pub iqk_fft_en: bool,
    pub is_fw_iqk: bool,
    pub is_iqk_enable: bool,
    pub iqk_cfir_en: bool,
    pub thermal_rek_en: bool,
    pub iqk_band: [u8; 2],
    pub iqk_ch: [u8; 2],
    pub iqk_bw: [u8; 2],
    pub iqk_times: u8,
    pub version: u8,
    pub phy: u8,
    pub fwk_status: u8,
    pub rsvd: u8,
    pub reload_cnt: __le32,
    pub iqk_fail_cnt: __le32,
    pub rf_0x18: [__le32; 2],
    pub lok_idac: [__le32; 2],
    pub lok_vbuf: [__le32; 2],
    pub rftxgain: [__le32; 2][6],
    pub rfrxgain: [__le32; 2][6],
    pub tx_xym: [__le32; 2][6],
    pub rx_xym: [__le32; 2][6],
    pub rx_wb_xym: [__le32; 2][32],
    pub is_radar: bool,
    pub rsvd1: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_dpk_rpt_log {
    pub ver: u8,
    pub idx: [u8; 2],
    pub band: [u8; 2],
    pub bw: [u8; 2],
    pub ch: [u8; 2],
    pub path_ok: [u8; 2],
    pub txagc: [u8; 2],
    pub ther: [u8; 2],
    pub gs: [u8; 2],
    pub dc_i: [u8; 4],
    pub dc_q: [u8; 4],
    pub corr_val: [u8; 2],
    pub corr_idx: [u8; 2],
    pub is_timeout: [u8; 2],
    pub rxbb_ov: [u8; 2],
    pub rsvd: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_dack_rpt_log {
    pub fwdack_ver: u8,
    pub fwdack_info_ver: u8,
    pub msbk_d: [u8; 2][2][16],
    pub dadck_d: [u8; 2][2],
    pub cdack_d: [u8; 2][2][2],
    pub addck2_hd: [u8; 2][2][2],
    pub addck2_ld: [u8; 2][2][2],
    pub adgaink_d: [u8; 2][2],
    pub biask_hd: [u8; 2][2],
    pub biask_ld: [u8; 2][2],
    pub addck_timeout: u8,
    pub cdack_timeout: u8,
    pub dadck_timeout: u8,
    pub msbk_timeout: u8,
    pub adgaink_timeout: u8,
    pub wbadcdck_timeout: u8,
    pub drck_timeout: u8,
    pub dack_fail: u8,
    pub wbdck_d: [u8; 2],
    pub rck_d: u8,
    pub adgaink_ex_d: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_rxdck_rpt_log {
    pub ver: u8,
    pub band: [u8; 2],
    pub bw: [u8; 2],
    pub ch: [u8; 2],
    pub timeout: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_tssi_rpt_log {
    pub alignment_power: [i8; 2][2][4],
    pub alignment_power_cw_h: [u8; 2][2][4],
    pub alignment_power_cw_l: [u8; 2][2][4],
    pub tssi_alimk_state: [u8; 2][2],
    pub default_txagc_offset: [u8; 2][2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_txgapk_rpt_log {
    pub r0x8010: [__le32; 2],
    pub chk_cnt: __le32,
    pub track_d: [u8; 2][17],
    pub power_d: [u8; 2][17],
    pub is_txgapk_ok: u8,
    pub chk_id: u8,
    pub ver: u8,
    pub d_bnd_ok: u8,
    pub stage: [__le32; 2],
    pub failcode: [__le16; 2],
    pub rsvd: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_txiqk_rpt_log {
    pub fw_txiqk_ver: u8,
    pub iqk_band: [u8; 2],
    pub iqk_ch: [u8; 2],
    pub iqk_bw: [u8; 2],
    pub tx_iqk_fail: [bool; 2],
    pub is_iqk_init: bool,
    pub txiqk_en: bool,
    pub lok_en: bool,
    pub lok_fail: [bool; 2],
    pub rsvd: [u8; 2],
    pub iqk_times: __le32,
    pub txiqk_nctldone: [bool; 2],
    pub rsvd2: [u8; 2],
    pub txgain: [__le32; 2][6],
    pub tx_iqc: [__le32; 2][6],
    pub tx_xym: [__le32; 2][6][14],
    pub kidx: [__le32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_cim3k_rpt_log {
    pub cim3k_band: [u8; 2],
    pub cim3k_ch: [u8; 2],
    pub cim3k_bw: [u8; 2],
    pub su_path_ok: [u8; 2],
    pub ru_path_ok: [u8; 2],
    pub txagc_cim3k: [u8; 2],
    pub ther_cim3k: [u8; 2],
    pub cim3k_gs: [u8; 2],
    pub cim3k_pwsf: [__le16; 2],
    pub cim3k_nctldone: [bool; 2],
    pub rsvd: [u8; 2],
    pub cim3k_rxiqc: [__le32; 2],
    pub cim3k_su_coef: [__le32; 2][3],
    pub dc_i: [__le16; 2],
    pub dc_q: [__le16; 2],
    pub corr_val: [u8; 2],
    pub corr_idx: [u8; 2],
    pub rxbb_ov: [u8; 2],
    pub cim3k_txiqc: [u8; 2],
    pub kidx: [u8; 2],
    pub fw_cim3k_ver: u8,
    pub su_cim3k_en: [bool; 2],
    pub ru_cim3k_en: [bool; 2],
    pub rsvd1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rfk_report {
    pub hdr: rtw89_c2h_hdr,
    pub /: *mut *mut u8 state; / enum rtw89_rfk_report_state,
    pub version: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_tas_rpt_log {
    pub cur_idx: __le32,
    pub txpwr_history: [__le16; 20],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_c2h_rf_tas_info {
    pub hdr: rtw89_c2h_hdr,
    pub content: rtw89_c2h_rf_tas_rpt_log,
    pub __packed: },
pub const RTW89_FW_RSVD_PLE_SIZE: c_uint = 0x800;
pub const RTW89_FW_BACKTRACE_INFO_SIZE: c_int = 8;

pub const RTW89_FW_BACKTRACE_KEY: c_uint = 0xBACEBACE;
pub const FWDL_WAIT_CNT: c_int = 400000;
pub const FWDL_WAIT_CNT_USB: c_int = 3200;
    pub type): *mut *mut int rtw89_fw_check_rdy(struct rtw89_dev rtwdev, enum rtw89_fwdl_check_type,
    pub rtwdev): *mut int rtw89_fw_recognize(struct rtw89_dev,
    pub rtwdev): *mut int rtw89_fw_recognize_elements(struct rtw89_dev,
    pub used_fw_format): *mut c_int,
    pub include_bb): bool,
    pub work): *mut void rtw89_load_firmware_work(struct work_struct,
    pub rtwdev): *mut void rtw89_unload_firmware(struct rtw89_dev,
    pub rtwdev): *mut int rtw89_wait_firmware_completion(struct rtw89_dev,
    pub rtwdev): *mut int rtw89_fw_log_prepare(struct rtw89_dev,
    pub len): *mut *mut *mut void rtw89_fw_log_dump(struct rtw89_dev rtwdev, u8 buf, u32,
    pub len): bool rack, bool dack, u32,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub punctured): u16,
    pub punctured): u16,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub offset): *mut *mut rtw89_vif_link rtwvif_link, u32,
    pub rtwvif_link): *mut *mut int rtw89_fw_h2c_pwr_lvl(struct rtw89_dev rtwdev, struct rtw89_vif_link,
    pub upd_mode): rtw89_upd_mode,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub rtwsta_link): *mut rtw89_sta_link,
    pub c2h): *mut *mut void rtw89_fw_c2h_irqsafe(struct rtw89_dev rtwdev, struct sk_buff,
    pub work): *mut *mut void rtw89_fw_c2h_work(struct wiphy wiphy, struct wiphy_work,
    pub rtwdev): *mut void rtw89_fw_c2h_purge_obsoleted_scan_events(struct rtw89_dev,
    pub len): *mut *mut *mut void rtw89_fw_c2h_dummy_handler(struct rtw89_dev rtwdev, struct sk_buff c2h, u32,
    pub upd_mode): rtw89_upd_mode,
    pub dis_conn): *mut *mut rtw89_sta_link rtwsta_link, bool,
    pub en): *mut *mut int rtw89_fw_h2c_notify_dbcc(struct rtw89_dev rtwdev, bool,
    pub pause): bool,
    pub val): u8 ac, u32,
    pub rtwdev): *mut int rtw89_fw_h2c_set_ofld_cfg(struct rtw89_dev,
    pub lv): *mut *mut int rtw89_fw_h2c_tx_duty(struct rtw89_dev rtwdev, u8,
    pub connect): bool,
    pub phy_ppdu): *mut rtw89_rx_phy_ppdu,
    pub rtwvif_link): *mut *mut int rtw89_fw_h2c_tp_offload(struct rtw89_dev rtwdev, struct rtw89_vif_link,
    pub csi): *mut *mut *mut int rtw89_fw_h2c_ra(struct rtw89_dev rtwdev, struct rtw89_ra_info ra, bool,
    pub rtwdev): *mut int rtw89_fw_h2c_phy_ch_rpt(struct rtw89_dev,
    pub mac_id): *mut *mut int rtw89_fw_h2c_tx_history(struct rtw89_dev rtwdev, u16,
    pub rtwdev): *mut int rtw89_fw_h2c_drv_ctrl_fw(struct rtw89_dev,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_init(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_init_v7(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_init_v10(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_role(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_role_v1(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_role_v101(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_role_v2(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_role_v7(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_role_v8(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_role_v10(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_init_v11(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_mlo_v2(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_osi_info(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_osi_info_v6(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_ctrl(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_ctrl_v7(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_ctrl_v9(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_trx_v7(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_trx_v9(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_trx_v107(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxdrv_rfk(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxtxpwr_v7(struct rtw89_dev rtwdev, u8,
    pub type): *mut *mut int rtw89_fw_h2c_cxtxpwr_v9(struct rtw89_dev rtwdev, u8,
    pub id): *mut *mut int rtw89_fw_h2c_del_pkt_offload(struct rtw89_dev rtwdev, u8,
    pub skb_ofld): *mut sk_buff,
    pub wowlan): bool,
    pub wowlan): bool,
    pub enable): rtw89_phy_idx phy_idx, bool,
    pub page): u16 len, u8,
    pub rtwdev): *mut int rtw89_fw_h2c_rf_ntfy_mcc(struct rtw89_dev,
    pub rtwvif): *mut *mut int rtw89_fw_h2c_rf_ps_info(struct rtw89_dev rtwdev, struct rtw89_vif,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): *mut *mut int rtw89_fw_h2c_rf_pre_ntfy_mcc(struct rtw89_dev rtwdev, enum rtw89_phy_idx,
    pub en): u8 mcc_role_idx, u8 pd_val, bool,
    pub tssi_mode): *const *const rtw89_chan chan, enum rtw89_tssi_mode,
    pub chan): *const rtw89_chan,
    pub chan): *const rtw89_chan,
    pub chan): *const rtw89_chan,
    pub chan): *const rtw89_chan,
    pub is_chl_k): *const *const rtw89_chan chan, bool,
    pub enable): *mut *mut int rtw89_fw_h2c_rf_tas_trigger(struct rtw89_dev rtwdev, bool,
    pub chan): *const rtw89_chan,
    pub chan): *const rtw89_chan,
    pub dack): bool rack, bool,
    pub len): *const *const *const int rtw89_fw_h2c_raw(struct rtw89_dev rtwdev, u8 buf, u16,
    pub rtwdev): *mut void rtw89_fw_send_all_early_h2c(struct rtw89_dev,
    pub rtwdev): *mut void __rtw89_fw_free_all_early_h2c(struct rtw89_dev,
    pub rtwdev): *mut void rtw89_fw_free_all_early_h2c(struct rtw89_dev,
    pub macid): u8,
    pub notify_fw): bool,
    pub notify_fw): *mut *mut void rtw89_fw_release_general_pkt_list(struct rtw89_dev rtwdev, bool,
    pub params): *mut bool valid, struct ieee80211_ampdu_params,
    pub params): *mut bool valid, struct ieee80211_ampdu_params,
    pub rtwdev): *mut void rtw89_fw_h2c_init_dynamic_ba_cam_v0_ext(struct rtw89_dev,
    pub mac_idx): u8 offset, u8,
    pub lps_param): *mut rtw89_lps_parm,
    pub rtwvif): *mut *mut int rtw89_fw_h2c_lps_ch_info(struct rtw89_dev rtwdev, struct rtw89_vif,
    pub rtwvif): *mut rtw89_vif,
    pub phy_idx): *const *const rtw89_chan chan, u8,
    pub rtwvif): *mut rtw89_vif,
    pub enable): bool,
    pub len): *mut *mut *mut sk_buff rtw89_fw_h2c_alloc_skb_with_hdr(rtw89_dev rtwdev, u32,
    pub len): *mut *mut *mut sk_buff rtw89_fw_h2c_alloc_skb_no_hdr(rtw89_dev rtwdev, u32,
    pub c2h_info): *mut rtw89_mac_c2h_info,
    pub enable): *mut *mut int rtw89_fw_h2c_fw_log(struct rtw89_dev rtwdev, bool,
    pub rtwdev): *mut void rtw89_fw_st_dbg_dump(struct rtw89_dev,
    pub scan_req): *mut ieee80211_scan_request,
    pub aborted): bool,
    pub enable): bool,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwdev): *mut void rtw89_hw_scan_free_chan_list_ax(struct rtw89_dev,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwdev): *mut void rtw89_hw_scan_free_chan_list_be(struct rtw89_dev,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub wowlan): *const *const u8 mac_addr, bool,
    pub wowlan): *mut *mut rtw89_vif_link rtwvif_link, bool,
    pub rtwdev): *mut int rtw89_fw_h2c_trigger_cpu_exception(struct rtw89_dev,
    pub params): *const rtw89_pkt_drop_params,
    pub ctwindow_oppps): u8 act, u8 noa_id, u8,
    pub en): bool,
    pub enable): bool,
    pub enable): *mut *mut rtw89_vif_link rtwvif_link, bool,
    pub enable): bool,
    pub enable): bool,
    pub enable): *mut *mut rtw89_vif_link rtwvif_link, bool,
    pub enable): *mut *mut rtw89_vif_link rtwvif_link, bool,
    pub enable): bool,
    pub enable): *mut *mut rtw89_vif_link rtwvif_link, bool,
    pub cam_info): *mut rtw89_wow_cam_info,
    pub cam_info): *mut rtw89_wow_cam_info,
    pub enable): bool,
    pub rtwdev): *mut int rtw89_fw_h2c_wow_request_aoac(struct rtw89_dev,
    pub p): *const rtw89_fw_mcc_add_req,
    pub p): *const rtw89_fw_mcc_start_req,
    pub prev_groups): bool,
    pub prev_groups): bool,
    pub group): *mut *mut int rtw89_fw_h2c_reset_mcc_group(struct rtw89_dev rtwdev, u8,
    pub rpt): *mut rtw89_mac_mcc_tsf_rpt,
    pub bitmap): *mut u8,
    pub offset): u8 target, u8,
    pub p): *const rtw89_fw_mcc_duration,
    pub arg): *const rtw89_fw_mrc_add_arg,
    pub arg): *const rtw89_fw_mrc_start_arg,
    pub slot_idx): *mut *mut int rtw89_fw_h2c_mrc_del(struct rtw89_dev rtwdev, u8 sch_idx, u8,
    pub rpt): *mut rtw89_mac_mrc_tsf_rpt,
    pub arg): *const rtw89_fw_mrc_upd_bitmap_arg,
    pub arg): *const rtw89_fw_mrc_sync_arg,
    pub arg): *const rtw89_fw_mrc_upd_duration_arg,
    pub en): *mut *mut int rtw89_fw_h2c_ap_info_refcount(struct rtw89_dev rtwdev, bool,
    pub enable): bool,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub rtw89_get_active_phy_bitmap(rtwdev): u8 active_bands =,
    pub i: c_int,
    pub i++): for (i = 0; i < RTW89_PHY_NUM;,
    pub BIT(i)): rtw89_fw_h2c_trx_protect(rtwdev, i, active_bands &,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub rtwsta_link): return chip->ops->h2c_default_cmac_tbl(rtwdev, rtwvif_link,,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub rtwsta_link): return chip->ops->h2c_default_dmac_tbl(rtwdev, rtwvif_link,,
    pub 0: return,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub rtwvif_link): return chip->ops->h2c_update_beacon(rtwdev,,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub rtwsta_link): return chip->ops->h2c_assoc_cmac_tbl(rtwdev, rtwvif_link,,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub 0: return,
    pub rtwvif_link: *mut rtw89_vif_link,
    pub rtwsta_link: *mut rtw89_sta_link,
    pub link_id: c_uint,
    pub ret: c_int,
    pub rtwsta_link->rtwvif_link: rtwvif_link =,
    pub ret: return,
    pub 0: return,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub rtwsta_link): return chip->ops->h2c_txtime_cmac_tbl(rtwdev,,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub 0: return,
    pub punctured): return chip->ops->h2c_punctured_cmac_tbl(rtwdev, rtwvif_link,,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub rtwvif_link: *mut rtw89_vif_link,
    pub rtwsta_link: *mut rtw89_sta_link,
    pub link_id: c_uint,
    pub ret: c_int,
    pub rtwsta_link->rtwvif_link: rtwvif_link =,
    pub params): valid,,
    pub ret: return,
    pub 0: return,
    pub rtwdev->chip: *const *const rtw89_chip_info chip =,
    pub cam_info): return chip->ops->h2c_wow_cam_update(rtwdev,,
// Must consider compatibility; don't insert new in the mid.
// Fill each field's default value in rtw89_regd_entcpy().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_regd_entry {
    pub alpha2_0: u8,
    pub alpha2_1: u8,
    pub rule_2ghz: u8,
    pub rule_5ghz: u8,
    pub rule_6ghz: u8,
    pub fmap: __le32,
    pub __packed: },
// must consider compatibility; don't insert new in the mid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_txpwr_byrate_entry {
    pub band: u8,
    pub nss: u8,
    pub rs: u8,
    pub shf: u8,
    pub len: u8,
    pub data: __le32,
    pub bw: u8,
    pub ofdma: u8,
    pub __packed: },
// must consider compatibility; don't insert new in the mid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_txpwr_lmt_2ghz_entry {
    pub bw: u8,
    pub nt: u8,
    pub rs: u8,
    pub bf: u8,
    pub regd: u8,
    pub ch_idx: u8,
    pub v: i8,
    pub __packed: },
// must consider compatibility; don't insert new in the mid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_txpwr_lmt_5ghz_entry {
    pub bw: u8,
    pub nt: u8,
    pub rs: u8,
    pub bf: u8,
    pub regd: u8,
    pub ch_idx: u8,
    pub v: i8,
    pub __packed: },
// must consider compatibility; don't insert new in the mid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_txpwr_lmt_6ghz_entry {
    pub bw: u8,
    pub nt: u8,
    pub rs: u8,
    pub bf: u8,
    pub regd: u8,
    pub reg_6ghz_power: u8,
    pub ch_idx: u8,
    pub v: i8,
    pub __packed: },
// must consider compatibility; don't insert new in the mid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_txpwr_lmt_ru_2ghz_entry {
    pub ru: u8,
    pub nt: u8,
    pub regd: u8,
    pub ch_idx: u8,
    pub v: i8,
    pub __packed: },
// must consider compatibility; don't insert new in the mid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_txpwr_lmt_ru_5ghz_entry {
    pub ru: u8,
    pub nt: u8,
    pub regd: u8,
    pub ch_idx: u8,
    pub v: i8,
    pub __packed: },
// must consider compatibility; don't insert new in the mid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_txpwr_lmt_ru_6ghz_entry {
    pub ru: u8,
    pub nt: u8,
    pub regd: u8,
    pub reg_6ghz_power: u8,
    pub ch_idx: u8,
    pub v: i8,
    pub __packed: },
// must consider compatibility; don't insert new in the mid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_tx_shape_lmt_entry {
    pub band: u8,
    pub tx_shape_rs: u8,
    pub regd: u8,
    pub v: u8,
    pub reg6_pwr: u8,
    pub __packed: },
// must consider compatibility; don't insert new in the mid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_fw_tx_shape_lmt_ru_entry {
    pub band: u8,
    pub regd: u8,
    pub v: u8,
    pub reg6_pwr: u8,
    pub __packed: },
    pub init): *const rtw89_rfe_parms,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_wow_wakeup_ver {
    RTW89_WOW_REASON_V0,
    RTW89_WOW_REASON_V1,
    RTW89_WOW_REASON_NUM,
}

    pub rtwdev): *mut rtw89_fw_cmd_ofld_alloc_and_get_io_ops(struct rtw89_dev,
