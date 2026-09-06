//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/hw.h
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
// Copyright (c) 2018 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_bus {
    ATH10K_BUS_PCI,
    ATH10K_BUS_AHB,
    ATH10K_BUS_SDIO,
    ATH10K_BUS_USB,
    ATH10K_BUS_SNOC,
}

// QCA988X 1.0 definitions (unsupported)
pub const QCA988X_HW_1_0_CHIP_ID_REV: c_uint = 0x0;
// QCA988X 2.0 definitions
pub const QCA988X_HW_2_0_VERSION: c_uint = 0x4100016c;
pub const QCA988X_HW_2_0_CHIP_ID_REV: c_uint = 0x2;

pub const QCA988X_HW_2_0_PATCH_LOAD_ADDR: c_uint = 0x1234;
// QCA9887 1.0 definitions
pub const QCA9887_HW_1_0_VERSION: c_uint = 0x4100016d;
pub const QCA9887_HW_1_0_CHIP_ID_REV: c_int = 0;

pub const QCA9887_HW_1_0_PATCH_LOAD_ADDR: c_uint = 0x1234;
// QCA6174 target BMI version signatures
pub const QCA6174_HW_1_0_VERSION: c_uint = 0x05000000;
pub const QCA6174_HW_1_1_VERSION: c_uint = 0x05000001;
pub const QCA6174_HW_1_3_VERSION: c_uint = 0x05000003;
pub const QCA6174_HW_2_1_VERSION: c_uint = 0x05010000;
pub const QCA6174_HW_3_0_VERSION: c_uint = 0x05020000;
pub const QCA6174_HW_3_2_VERSION: c_uint = 0x05030000;
// QCA9377 target BMI version signatures
pub const QCA9377_HW_1_0_DEV_VERSION: c_uint = 0x05020000;
pub const QCA9377_HW_1_1_DEV_VERSION: c_uint = 0x05020001;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca6174_pci_rev {
    QCA6174_PCI_REV_1_1 = 0x11,
    QCA6174_PCI_REV_1_3 = 0x13,
    QCA6174_PCI_REV_2_0 = 0x20,
    QCA6174_PCI_REV_3_0 = 0x30,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca6174_chip_id_rev {
    QCA6174_HW_1_0_CHIP_ID_REV = 0,
    QCA6174_HW_1_1_CHIP_ID_REV = 1,
    QCA6174_HW_1_3_CHIP_ID_REV = 2,
    QCA6174_HW_2_1_CHIP_ID_REV = 4,
    QCA6174_HW_2_2_CHIP_ID_REV = 5,
    QCA6174_HW_3_0_CHIP_ID_REV = 8,
    QCA6174_HW_3_1_CHIP_ID_REV = 9,
    QCA6174_HW_3_2_CHIP_ID_REV = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca9377_chip_id_rev {
    QCA9377_HW_1_0_CHIP_ID_REV = 0x0,
    QCA9377_HW_1_1_CHIP_ID_REV = 0x1,
}

pub const QCA6174_HW_2_1_PATCH_LOAD_ADDR: c_uint = 0x1234;

pub const QCA6174_HW_3_0_PATCH_LOAD_ADDR: c_uint = 0x1234;
// QCA99X0 1.0 definitions (unsupported)
pub const QCA99X0_HW_1_0_CHIP_ID_REV: c_uint = 0x0;
// QCA99X0 2.0 definitions
pub const QCA99X0_HW_2_0_DEV_VERSION: c_uint = 0x01000000;
pub const QCA99X0_HW_2_0_CHIP_ID_REV: c_uint = 0x1;

pub const QCA99X0_HW_2_0_PATCH_LOAD_ADDR: c_uint = 0x1234;
// QCA9984 1.0 defines
pub const QCA9984_HW_1_0_DEV_VERSION: c_uint = 0x1000000;
pub const QCA9984_HW_DEV_TYPE: c_uint = 0xa;
pub const QCA9984_HW_1_0_CHIP_ID_REV: c_uint = 0x0;

pub const QCA9984_HW_1_0_PATCH_LOAD_ADDR: c_uint = 0x1234;
// QCA9888 2.0 defines
pub const QCA9888_HW_2_0_DEV_VERSION: c_uint = 0x1000000;
pub const QCA9888_HW_DEV_TYPE: c_uint = 0xc;
pub const QCA9888_HW_2_0_CHIP_ID_REV: c_uint = 0x0;

pub const QCA9888_HW_2_0_PATCH_LOAD_ADDR: c_uint = 0x1234;
// QCA9377 1.0 definitions

pub const QCA9377_HW_1_0_PATCH_LOAD_ADDR: c_uint = 0x1234;
// QCA4019 1.0 definitions
pub const QCA4019_HW_1_0_DEV_VERSION: c_uint = 0x01000000;

pub const QCA4019_HW_1_0_PATCH_LOAD_ADDR: c_uint = 0x1234;
// WCN3990 1.0 definitions

pub const ATH10K_FW_API_MAX: c_int = 6;
pub const ATH10K_FW_API_MIN: c_int = 2;

// added support for ATH10K_FW_IE_WMI_OP_VERSION

// HTT id conflict fix for management frames over HTT

// the firmware-6.bin blob

// includes also the null byte

pub const REG_DUMP_COUNT_QCA988X: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_fw_ie {
    pub id: __le32,
    pub len: __le32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_fw_ie_type {
    ATH10K_FW_IE_FW_VERSION = 0,
    ATH10K_FW_IE_TIMESTAMP = 1,
    ATH10K_FW_IE_FEATURES = 2,
    ATH10K_FW_IE_FW_IMAGE = 3,
    ATH10K_FW_IE_OTP_IMAGE = 4,

// WMI "operations" interface version, 32 bit value. Supported from
// FW API 4 and above.
//
    ATH10K_FW_IE_WMI_OP_VERSION = 5,

// HTT "operations" interface version, 32 bit value. Supported from
// FW API 5 and above.
//
    ATH10K_FW_IE_HTT_OP_VERSION = 6,

// Code swap image for firmware binary
    ATH10K_FW_IE_FW_CODE_SWAP_IMAGE = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_fw_wmi_op_version {
    ATH10K_FW_WMI_OP_VERSION_UNSET = 0,

    ATH10K_FW_WMI_OP_VERSION_MAIN = 1,
    ATH10K_FW_WMI_OP_VERSION_10_1 = 2,
    ATH10K_FW_WMI_OP_VERSION_10_2 = 3,
    ATH10K_FW_WMI_OP_VERSION_TLV = 4,
    ATH10K_FW_WMI_OP_VERSION_10_2_4 = 5,
    ATH10K_FW_WMI_OP_VERSION_10_4 = 6,

// keep last
    ATH10K_FW_WMI_OP_VERSION_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_fw_htt_op_version {
    ATH10K_FW_HTT_OP_VERSION_UNSET = 0,

    ATH10K_FW_HTT_OP_VERSION_MAIN = 1,

// also used in 10.2 and 10.2.4 branches
    ATH10K_FW_HTT_OP_VERSION_10_1 = 2,

    ATH10K_FW_HTT_OP_VERSION_TLV = 3,

    ATH10K_FW_HTT_OP_VERSION_10_4 = 4,

// keep last
    ATH10K_FW_HTT_OP_VERSION_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_bd_ie_type {
// contains sub IEs of enum ath10k_bd_ie_board_type
    ATH10K_BD_IE_BOARD = 0,
    ATH10K_BD_IE_BOARD_EXT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_bd_ie_board_type {
    ATH10K_BD_IE_BOARD_NAME = 0,
    ATH10K_BD_IE_BOARD_DATA = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_hw_rev {
    ATH10K_HW_QCA988X,
    ATH10K_HW_QCA6174,
    ATH10K_HW_QCA99X0,
    ATH10K_HW_QCA9888,
    ATH10K_HW_QCA9984,
    ATH10K_HW_QCA9377,
    ATH10K_HW_QCA4019,
    ATH10K_HW_QCA9887,
    ATH10K_HW_WCN3990,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_regs {
    pub rtc_soc_base_address: u32,
    pub rtc_wmac_base_address: u32,
    pub soc_core_base_address: u32,
    pub wlan_mac_base_address: u32,
    pub ce_wrapper_base_address: u32,
    pub ce0_base_address: u32,
    pub ce1_base_address: u32,
    pub ce2_base_address: u32,
    pub ce3_base_address: u32,
    pub ce4_base_address: u32,
    pub ce5_base_address: u32,
    pub ce6_base_address: u32,
    pub ce7_base_address: u32,
    pub ce8_base_address: u32,
    pub ce9_base_address: u32,
    pub ce10_base_address: u32,
    pub ce11_base_address: u32,
    pub soc_reset_control_si0_rst_mask: u32,
    pub soc_reset_control_ce_rst_mask: u32,
    pub soc_chip_id_address: u32,
    pub scratch_3_address: u32,
    pub fw_indicator_address: u32,
    pub pcie_local_base_address: u32,
    pub ce_wrap_intr_sum_host_msi_lsb: u32,
    pub ce_wrap_intr_sum_host_msi_mask: u32,
    pub pcie_intr_fw_mask: u32,
    pub pcie_intr_ce_mask_all: u32,
    pub pcie_intr_clr_address: u32,
    pub cpu_pll_init_address: u32,
    pub cpu_speed_address: u32,
    pub core_clk_div_address: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ce_regs_addr_map {
    pub msb: u32,
    pub lsb: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ce_ctrl1 {
    pub addr: u32,
    pub hw_mask: u32,
    pub sw_mask: u32,
    pub hw_wr_mask: u32,
    pub sw_wr_mask: u32,
    pub reset_mask: u32,
    pub reset: u32,
    pub src_ring: *const ath10k_hw_ce_regs_addr_map,
    pub dst_ring: *const ath10k_hw_ce_regs_addr_map,
    pub dmax: *const ath10k_hw_ce_regs_addr_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ce_cmd_halt {
    pub status_reset: u32,
    pub msb: u32,
    pub mask: u32,
    pub status: *const ath10k_hw_ce_regs_addr_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ce_host_ie {
    pub copy_complete_reset: u32,
    pub copy_complete: *const ath10k_hw_ce_regs_addr_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ce_host_wm_regs {
    pub dstr_lmask: u32,
    pub dstr_hmask: u32,
    pub srcr_lmask: u32,
    pub srcr_hmask: u32,
    pub cc_mask: u32,
    pub wm_mask: u32,
    pub addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ce_misc_regs {
    pub axi_err: u32,
    pub dstr_add_err: u32,
    pub srcr_len_err: u32,
    pub dstr_mlen_vio: u32,
    pub dstr_overflow: u32,
    pub srcr_overflow: u32,
    pub err_mask: u32,
    pub addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ce_dst_src_wm_regs {
    pub addr: u32,
    pub low_rst: u32,
    pub high_rst: u32,
    pub wm_low: *const ath10k_hw_ce_regs_addr_map,
    pub wm_high: *const ath10k_hw_ce_regs_addr_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ce_ctrl1_upd {
    pub shift: u32,
    pub mask: u32,
    pub enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ce_regs {
    pub sr_base_addr_lo: u32,
    pub sr_base_addr_hi: u32,
    pub sr_size_addr: u32,
    pub dr_base_addr_lo: u32,
    pub dr_base_addr_hi: u32,
    pub dr_size_addr: u32,
    pub ce_cmd_addr: u32,
    pub misc_ie_addr: u32,
    pub sr_wr_index_addr: u32,
    pub dst_wr_index_addr: u32,
    pub current_srri_addr: u32,
    pub current_drri_addr: u32,
    pub ddr_addr_for_rri_low: u32,
    pub ddr_addr_for_rri_high: u32,
    pub ce_rri_low: u32,
    pub ce_rri_high: u32,
    pub host_ie_addr: u32,
    pub wm_regs: *const ath10k_hw_ce_host_wm_regs,
    pub misc_regs: *const ath10k_hw_ce_misc_regs,
    pub ctrl1_regs: *const ath10k_hw_ce_ctrl1,
    pub cmd_halt: *const ath10k_hw_ce_cmd_halt,
    pub host_ie: *const ath10k_hw_ce_host_ie,
    pub wm_srcr: *const ath10k_hw_ce_dst_src_wm_regs,
    pub wm_dstr: *const ath10k_hw_ce_dst_src_wm_regs,
    pub upd: *const ath10k_hw_ce_ctrl1_upd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_values {
    pub rtc_state_val_on: u32,
    pub ce_count: u8,
    pub msi_assign_ce_max: u8,
    pub num_target_ce_config_wlan: u8,
    pub ce_desc_meta_data_mask: u16,
    pub ce_desc_meta_data_lsb: u8,
    pub rfkill_pin: u32,
    pub rfkill_cfg: u32,
    pub rfkill_on_level: bool,
}

// Known peculiarities:
// - raw appears in nwifi decap, raw and nwifi appear in ethernet decap
// - raw have FCS, nwifi doesn't
// - ethernet frames have 802.11 header decapped and parts (base hdr, cipher
// param, llc/snap) are aligned to 4byte boundaries each
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_hw_txrx_mode {
    ATH10K_HW_TXRX_RAW = 0,

// Native Wifi decap mode is used to align IP frames to 4-byte
// boundaries and avoid a very expensive re-alignment in mac80211.
//
    ATH10K_HW_TXRX_NATIVE_WIFI = 1,
    ATH10K_HW_TXRX_ETHERNET = 2,

// Valid for HTT >= 3.0. Used for management frames in TX_FRM.
    ATH10K_HW_TXRX_MGMT = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_mcast2ucast_mode {
    ATH10K_MCAST2UCAST_DISABLED = 0,
    ATH10K_MCAST2UCAST_ENABLED = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_hw_rate_ofdm {
    ATH10K_HW_RATE_OFDM_48M = 0,
    ATH10K_HW_RATE_OFDM_24M,
    ATH10K_HW_RATE_OFDM_12M,
    ATH10K_HW_RATE_OFDM_6M,
    ATH10K_HW_RATE_OFDM_54M,
    ATH10K_HW_RATE_OFDM_36M,
    ATH10K_HW_RATE_OFDM_18M,
    ATH10K_HW_RATE_OFDM_9M,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_hw_rate_cck {
    ATH10K_HW_RATE_CCK_LP_11M = 0,
    ATH10K_HW_RATE_CCK_LP_5_5M,
    ATH10K_HW_RATE_CCK_LP_2M,
    ATH10K_HW_RATE_CCK_LP_1M,
    ATH10K_HW_RATE_CCK_SP_11M,
    ATH10K_HW_RATE_CCK_SP_5_5M,
    ATH10K_HW_RATE_CCK_SP_2M,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_hw_rate_rev2_cck {
    ATH10K_HW_RATE_REV2_CCK_LP_1M = 1,
    ATH10K_HW_RATE_REV2_CCK_LP_2M,
    ATH10K_HW_RATE_REV2_CCK_LP_5_5M,
    ATH10K_HW_RATE_REV2_CCK_LP_11M,
    ATH10K_HW_RATE_REV2_CCK_SP_2M,
    ATH10K_HW_RATE_REV2_CCK_SP_5_5M,
    ATH10K_HW_RATE_REV2_CCK_SP_11M,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_hw_cc_wraparound_type {
    ATH10K_HW_CC_WRAP_DISABLED = 0,

// This type is when the HW chip has a quirky Cycle Counter
// wraparound which resets to 0x7fffffff instead of 0. All
// other CC related counters (e.g. Rx Clear Count) are divided
// by 2 so they never wraparound themselves.
//
    ATH10K_HW_CC_WRAP_SHIFTED_ALL = 1,

// Each hw counter wraps around independently. When the
// counter overflows the respective counter is right shifted
// by 1, i.e reset to 0x7fffffff, and other counters will be
// running unaffected. In this type of wraparound, it should
// be possible to report accurate Rx busy time unlike the
// first type.
//
    ATH10K_HW_CC_WRAP_SHIFTED_EACH = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_hw_refclk_speed {
    ATH10K_HW_REFCLK_UNKNOWN = -1,
    ATH10K_HW_REFCLK_48_MHZ = 0,
    ATH10K_HW_REFCLK_19_2_MHZ = 1,
    ATH10K_HW_REFCLK_24_MHZ = 2,
    ATH10K_HW_REFCLK_26_MHZ = 3,
    ATH10K_HW_REFCLK_37_4_MHZ = 4,
    ATH10K_HW_REFCLK_38_4_MHZ = 5,
    ATH10K_HW_REFCLK_40_MHZ = 6,
    ATH10K_HW_REFCLK_52_MHZ = 7,

// must be the last one
    ATH10K_HW_REFCLK_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_clk_params {
    pub refclk: u32,
    pub div: u32,
    pub rnfrac: u32,
    pub settle_time: u32,
    pub refdiv: u32,
    pub outdiv: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_params {
    pub id: u32,
    pub dev_id: u16,
    pub bus: ath10k_bus,
    pub name: *const c_char,
    pub patch_load_addr: u32,
    pub uart_pin: c_int,
    pub led_pin: c_int,
    pub otp_exe_param: u32,
// Type of hw cycle counter wraparound logic, for more info
// refer enum ath10k_hw_cc_wraparound_type.
//
    pub cc_wraparound_type: ath10k_hw_cc_wraparound_type,
// Some of chip expects fragment descriptor to be continuous
// memory for any TX operation. Set continuous_frag_desc flag
// for the hardware which have such requirement.
//
    pub continuous_frag_desc: bool,
// CCK hardware rate table mapping for the newer chipsets
// like QCA99X0, QCA4019 got revised. The CCK h/w rate values
// are in a proper order with respect to the rate/preamble
//
    pub cck_rate_map_rev2: bool,
    pub channel_counters_freq_hz: u32,
// Mgmt tx descriptors threshold for limiting probe response
// frames.
//
    pub max_probe_resp_desc_thres: u32,
    pub tx_chain_mask: u32,
    pub rx_chain_mask: u32,
    pub max_spatial_stream: u32,
    pub cal_data_len: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_params_fw {
    pub dir: *const c_char,
    pub board_size: usize,
    pub ext_board_size: usize,
    pub board_ext_size: usize,
    pub fw: },
// qca99x0 family chips deliver broadcast/multicast management
// frames encrypted and expect software do decryption.
//
    pub sw_decrypt_mcast_mgmt: bool,
// Rx descriptor abstraction
    pub rx_desc_ops: *const ath10k_htt_rx_desc_ops,
    pub hw_ops: *const ath10k_hw_ops,
// Number of bytes used for alignment in rx_hdr_status of rx desc.
    pub decap_align_bytes: c_int,
// hw specific clock control parameters
    pub hw_clk: *const ath10k_hw_clk_params,
    pub target_cpu_freq: c_int,
// Number of bytes to be discarded for each FFT sample
    pub spectral_bin_discard: c_int,
// The board may have a restricted NSS for 160 or 80+80 vs what it
// can do for 80Mhz.
//
    pub vht160_mcs_rx_highest: c_int,
    pub vht160_mcs_tx_highest: c_int,
// Number of ciphers supported (i.e First N) in cipher_suites array
    pub n_cipher_suites: c_int,
    pub num_peers: u32,
    pub ast_skid_limit: u32,
    pub num_wds_entries: u32,
// Targets supporting physical addressing capability above 32-bits
    pub target_64bit: bool,
// Target rx ring fill level
    pub rx_ring_fill_level: u32,
// target supporting shadow register for ce write
    pub shadow_reg_support: bool,
// target supporting retention restore on ddr
    pub rri_on_ddr: bool,
// Number of bytes to be the offset for each FFT sample
    pub spectral_bin_offset: c_int,
// targets which require hw filter reset during boot up,
// to avoid it sending spurious acks.
//
    pub hw_filter_reset_required: bool,
// target supporting fw download via diag ce
    pub fw_diag_ce_download: bool,
// target supporting fw download via large size BMI
    pub bmi_large_size_download: bool,
// need to set uart pin if disable uart print, workaround for a
// firmware bug
//
    pub uart_pin_workaround: bool,
// Workaround for the credit size calculation
    pub credit_size_workaround: bool,
// tx stats support over pktlog
    pub tx_stats_over_pktlog: bool,
// provides bitrates for sta_statistics using WMI_TLV_PEER_STATS_INFO_EVENTID
    pub supports_peer_stats_info: bool,
    pub dynamic_sar_support: bool,
    pub hw_restart_disconnect: bool,
    pub use_fw_tx_credits: bool,
    pub delay_unmap_buffer: bool,
// The hardware support multicast frame registrations
    pub mcast_frame_registration: bool,
}

// Defines needed for Rx descriptor abstraction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_hw_ops {
    pub value): *mut *mut *mut void (set_coverage_class)(struct ath10k ar, int radio_idx, s16,
    pub ar): *mut *mut int (enable_pll_clk)(struct ath10k,
    pub htt): *mut *mut int (tx_data_rssi_pad_bytes)(struct htt_resp,
    pub resp): *mut *mut int (is_rssi_enable)(struct htt_resp,
}

// Target specific defines for MAIN firmware
pub const TARGET_NUM_VDEVS: c_int = 8;
pub const TARGET_NUM_PEER_AST: c_int = 2;
pub const TARGET_NUM_WDS_ENTRIES: c_int = 32;
pub const TARGET_DMA_BURST_SIZE: c_int = 0;
pub const TARGET_MAC_AGGR_DELIM: c_int = 0;
pub const TARGET_AST_SKID_LIMIT: c_int = 16;
pub const TARGET_NUM_STATIONS: c_int = 16;

pub const TARGET_NUM_OFFLOAD_PEERS: c_int = 0;
pub const TARGET_NUM_OFFLOAD_REORDER_BUFS: c_int = 0;
pub const TARGET_NUM_PEER_KEYS: c_int = 2;

pub const TARGET_RX_TIMEOUT_LO_PRI: c_int = 100;
pub const TARGET_RX_TIMEOUT_HI_PRI: c_int = 40;
pub const TARGET_SCAN_MAX_PENDING_REQS: c_int = 4;
pub const TARGET_BMISS_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_ROAM_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_ROAM_OFFLOAD_MAX_AP_PROFILES: c_int = 8;
pub const TARGET_GTK_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_NUM_MCAST_GROUPS: c_int = 0;
pub const TARGET_NUM_MCAST_TABLE_ELEMS: c_int = 0;

pub const TARGET_TX_DBG_LOG_SIZE: c_int = 1024;
pub const TARGET_RX_SKIP_DEFRAG_TIMEOUT_DUP_DETECTION_CHECK: c_int = 0;
pub const TARGET_VOW_CONFIG: c_int = 0;

pub const TARGET_MAX_FRAG_ENTRIES: c_int = 0;
// Target specific defines for 10.X firmware
pub const TARGET_10X_NUM_VDEVS: c_int = 16;
pub const TARGET_10X_NUM_PEER_AST: c_int = 2;
pub const TARGET_10X_NUM_WDS_ENTRIES: c_int = 32;
pub const TARGET_10X_DMA_BURST_SIZE: c_int = 0;
pub const TARGET_10X_MAC_AGGR_DELIM: c_int = 0;
pub const TARGET_10X_AST_SKID_LIMIT: c_int = 128;
pub const TARGET_10X_NUM_STATIONS: c_int = 128;
pub const TARGET_10X_TX_STATS_NUM_STATIONS: c_int = 118;

pub const TARGET_10X_NUM_OFFLOAD_PEERS: c_int = 0;
pub const TARGET_10X_NUM_OFFLOAD_REORDER_BUFS: c_int = 0;
pub const TARGET_10X_NUM_PEER_KEYS: c_int = 2;
pub const TARGET_10X_NUM_TIDS_MAX: c_int = 256;

pub const TARGET_10X_RX_TIMEOUT_LO_PRI: c_int = 100;
pub const TARGET_10X_RX_TIMEOUT_HI_PRI: c_int = 40;
pub const TARGET_10X_SCAN_MAX_PENDING_REQS: c_int = 4;
pub const TARGET_10X_BMISS_OFFLOAD_MAX_VDEV: c_int = 2;
pub const TARGET_10X_ROAM_OFFLOAD_MAX_VDEV: c_int = 2;
pub const TARGET_10X_ROAM_OFFLOAD_MAX_AP_PROFILES: c_int = 8;
pub const TARGET_10X_GTK_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_10X_NUM_MCAST_GROUPS: c_int = 0;
pub const TARGET_10X_NUM_MCAST_TABLE_ELEMS: c_int = 0;

pub const TARGET_10X_TX_DBG_LOG_SIZE: c_int = 1024;
pub const TARGET_10X_RX_SKIP_DEFRAG_TIMEOUT_DUP_DETECTION_CHECK: c_int = 1;
pub const TARGET_10X_VOW_CONFIG: c_int = 0;

pub const TARGET_10X_MAX_FRAG_ENTRIES: c_int = 0;
// 10.2 parameters
pub const TARGET_10_2_DMA_BURST_SIZE: c_int = 0;
// Target specific defines for WMI-TLV firmware
pub const TARGET_TLV_NUM_VDEVS: c_int = 4;
pub const TARGET_TLV_NUM_STATIONS: c_int = 32;
pub const TARGET_TLV_NUM_PEERS: c_int = 33;
pub const TARGET_TLV_NUM_TDLS_VDEVS: c_int = 1;

pub const TARGET_TLV_NUM_MSDU_DESC_HL: c_int = 1024;
pub const TARGET_TLV_NUM_WOW_PATTERNS: c_int = 22;

// Target specific defines for WMI-HL-1.0 firmware
pub const TARGET_HL_TLV_NUM_PEERS: c_int = 33;
pub const TARGET_HL_TLV_AST_SKID_LIMIT: c_int = 16;
pub const TARGET_HL_TLV_NUM_WDS_ENTRIES: c_int = 2;
// Target specific defines for QCA9377 high latency firmware
pub const TARGET_QCA9377_HL_NUM_PEERS: c_int = 15;
// Diagnostic Window
pub const CE_DIAG_PIPE: c_int = 7;

// Target specific defines for 10.4 firmware
pub const TARGET_10_4_NUM_VDEVS: c_int = 16;
pub const TARGET_10_4_NUM_STATIONS: c_int = 32;

pub const TARGET_10_4_ACTIVE_PEERS: c_int = 0;
pub const TARGET_10_4_NUM_QCACHE_PEERS_MAX: c_int = 512;
pub const TARGET_10_4_QCACHE_ACTIVE_PEERS: c_int = 50;
pub const TARGET_10_4_QCACHE_ACTIVE_PEERS_PFC: c_int = 35;
pub const TARGET_10_4_NUM_OFFLOAD_PEERS: c_int = 0;
pub const TARGET_10_4_NUM_OFFLOAD_REORDER_BUFFS: c_int = 0;
pub const TARGET_10_4_NUM_PEER_KEYS: c_int = 2;

pub const TARGET_10_4_NUM_MSDU_DESC_PFC: c_int = 2500;
pub const TARGET_10_4_AST_SKID_LIMIT: c_int = 32;
// 100 ms for video, best-effort, and background
pub const TARGET_10_4_RX_TIMEOUT_LO_PRI: c_int = 100;
// 40 ms for voice
pub const TARGET_10_4_RX_TIMEOUT_HI_PRI: c_int = 40;

pub const TARGET_10_4_SCAN_MAX_REQS: c_int = 4;
pub const TARGET_10_4_BMISS_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_10_4_ROAM_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_10_4_ROAM_OFFLOAD_MAX_PROFILES: c_int = 8;
// Note: mcast to ucast is disabled by default
pub const TARGET_10_4_NUM_MCAST_GROUPS: c_int = 0;
pub const TARGET_10_4_NUM_MCAST_TABLE_ELEMS: c_int = 0;
pub const TARGET_10_4_MCAST2UCAST_MODE: c_int = 0;
pub const TARGET_10_4_TX_DBG_LOG_SIZE: c_int = 1024;
pub const TARGET_10_4_NUM_WDS_ENTRIES: c_int = 32;
pub const TARGET_10_4_DMA_BURST_SIZE: c_int = 1;
pub const TARGET_10_4_MAC_AGGR_DELIM: c_int = 0;
pub const TARGET_10_4_RX_SKIP_DEFRAG_TIMEOUT_DUP_DETECTION_CHECK: c_int = 1;
pub const TARGET_10_4_VOW_CONFIG: c_int = 0;
pub const TARGET_10_4_GTK_OFFLOAD_MAX_VDEV: c_int = 3;
pub const TARGET_10_4_11AC_TX_MAX_FRAGS: c_int = 2;
pub const TARGET_10_4_MAX_PEER_EXT_STATS: c_int = 16;
pub const TARGET_10_4_SMART_ANT_CAP: c_int = 0;
pub const TARGET_10_4_BK_MIN_FREE: c_int = 0;
pub const TARGET_10_4_BE_MIN_FREE: c_int = 0;
pub const TARGET_10_4_VI_MIN_FREE: c_int = 0;
pub const TARGET_10_4_VO_MIN_FREE: c_int = 0;
pub const TARGET_10_4_RX_BATCH_MODE: c_int = 1;
pub const TARGET_10_4_THERMAL_THROTTLING_CONFIG: c_int = 0;
pub const TARGET_10_4_ATF_CONFIG: c_int = 0;
pub const TARGET_10_4_IPHDR_PAD_CONFIG: c_int = 1;
pub const TARGET_10_4_QWRAP_CONFIG: c_int = 0;
// TDLS config
pub const TARGET_10_4_NUM_TDLS_VDEVS: c_int = 1;
pub const TARGET_10_4_NUM_TDLS_BUFFER_STA: c_int = 1;
pub const TARGET_10_4_NUM_TDLS_SLEEP_STA: c_int = 1;
// Maximum number of Copy Engines supported
pub const CE_COUNT_MAX: c_int = 12;
// Number of Copy Engines supported

//
// Granted MSIs are assigned as follows:
// Firmware uses the first
// Remaining MSIs, if any, are used by Copy Engines
// This mapping is known to both Target firmware and Host software.
// It may be changed as long as Host and Target are kept in sync.
//
// MSI for firmware (errors, etc.)
pub const MSI_ASSIGN_FW: c_int = 0;
// MSIs for Copy Engines
pub const MSI_ASSIGN_CE_INITIAL: c_int = 1;

// as of IP3.7.1

pub const RTC_STATE_V_LSB: c_int = 0;
pub const RTC_STATE_V_MASK: c_uint = 0x00000007;
pub const RTC_STATE_ADDRESS: c_uint = 0x0000;
pub const PCIE_SOC_WAKE_V_MASK: c_uint = 0x00000001;
pub const PCIE_SOC_WAKE_ADDRESS: c_uint = 0x0004;
pub const PCIE_SOC_WAKE_RESET: c_uint = 0x00000000;
pub const SOC_GLOBAL_RESET_ADDRESS: c_uint = 0x0008;

pub const MAC_COEX_BASE_ADDRESS: c_uint = 0x00006000;
pub const BT_COEX_BASE_ADDRESS: c_uint = 0x00007000;
pub const SOC_PCIE_BASE_ADDRESS: c_uint = 0x00008000;

pub const WLAN_UART_BASE_ADDRESS: c_uint = 0x0000c000;
pub const WLAN_SI_BASE_ADDRESS: c_uint = 0x00010000;
pub const WLAN_GPIO_BASE_ADDRESS: c_uint = 0x00014000;
pub const WLAN_ANALOG_INTF_BASE_ADDRESS: c_uint = 0x0001c000;

pub const EFUSE_BASE_ADDRESS: c_uint = 0x00030000;
pub const FPGA_REG_BASE_ADDRESS: c_uint = 0x00039000;
pub const WLAN_UART2_BASE_ADDRESS: c_uint = 0x00054c00;

pub const DBI_BASE_ADDRESS: c_uint = 0x00060000;
pub const WLAN_ANALOG_INTF_PCIE_BASE_ADDRESS: c_uint = 0x0006c000;

pub const SOC_RESET_CONTROL_ADDRESS: c_uint = 0x00000000;
pub const SOC_RESET_CONTROL_OFFSET: c_uint = 0x00000000;

pub const SOC_RESET_CONTROL_CPU_WARM_RST_MASK: c_uint = 0x00000040;
pub const SOC_CPU_CLOCK_OFFSET: c_uint = 0x00000020;
pub const SOC_CPU_CLOCK_STANDARD_LSB: c_int = 0;
pub const SOC_CPU_CLOCK_STANDARD_MASK: c_uint = 0x00000003;
pub const SOC_CLOCK_CONTROL_OFFSET: c_uint = 0x00000028;
pub const SOC_CLOCK_CONTROL_SI0_CLK_MASK: c_uint = 0x00000001;
pub const SOC_SYSTEM_SLEEP_OFFSET: c_uint = 0x000000c4;
pub const SOC_LPO_CAL_OFFSET: c_uint = 0x000000e0;
pub const SOC_LPO_CAL_ENABLE_LSB: c_int = 20;
pub const SOC_LPO_CAL_ENABLE_MASK: c_uint = 0x00100000;
pub const SOC_LF_TIMER_CONTROL0_ADDRESS: c_uint = 0x00000050;
pub const SOC_LF_TIMER_CONTROL0_ENABLE_MASK: c_uint = 0x00000004;

pub const SOC_CHIP_ID_REV_LSB: c_int = 8;
pub const SOC_CHIP_ID_REV_MASK: c_uint = 0x00000f00;
pub const WLAN_RESET_CONTROL_COLD_RST_MASK: c_uint = 0x00000008;
pub const WLAN_RESET_CONTROL_WARM_RST_MASK: c_uint = 0x00000004;
pub const WLAN_SYSTEM_SLEEP_DISABLE_LSB: c_int = 0;
pub const WLAN_SYSTEM_SLEEP_DISABLE_MASK: c_uint = 0x00000001;
pub const WLAN_GPIO_PIN0_ADDRESS: c_uint = 0x00000028;
pub const WLAN_GPIO_PIN0_CONFIG_LSB: c_int = 11;
pub const WLAN_GPIO_PIN0_CONFIG_MASK: c_uint = 0x00007800;
pub const WLAN_GPIO_PIN0_PAD_PULL_LSB: c_int = 5;
pub const WLAN_GPIO_PIN0_PAD_PULL_MASK: c_uint = 0x00000060;
pub const WLAN_GPIO_PIN1_ADDRESS: c_uint = 0x0000002c;
pub const WLAN_GPIO_PIN1_CONFIG_MASK: c_uint = 0x00007800;
pub const WLAN_GPIO_PIN10_ADDRESS: c_uint = 0x00000050;
pub const WLAN_GPIO_PIN11_ADDRESS: c_uint = 0x00000054;
pub const WLAN_GPIO_PIN12_ADDRESS: c_uint = 0x00000058;
pub const WLAN_GPIO_PIN13_ADDRESS: c_uint = 0x0000005c;
pub const CLOCK_GPIO_OFFSET: c_uint = 0xffffffff;
pub const CLOCK_GPIO_BT_CLK_OUT_EN_LSB: c_int = 0;
pub const CLOCK_GPIO_BT_CLK_OUT_EN_MASK: c_int = 0;
pub const SI_CONFIG_OFFSET: c_uint = 0x00000000;
pub const SI_CONFIG_ERR_INT_LSB: c_int = 19;
pub const SI_CONFIG_ERR_INT_MASK: c_uint = 0x00080000;
pub const SI_CONFIG_BIDIR_OD_DATA_LSB: c_int = 18;
pub const SI_CONFIG_BIDIR_OD_DATA_MASK: c_uint = 0x00040000;
pub const SI_CONFIG_I2C_LSB: c_int = 16;
pub const SI_CONFIG_I2C_MASK: c_uint = 0x00010000;
pub const SI_CONFIG_POS_SAMPLE_LSB: c_int = 7;
pub const SI_CONFIG_POS_SAMPLE_MASK: c_uint = 0x00000080;
pub const SI_CONFIG_INACTIVE_DATA_LSB: c_int = 5;
pub const SI_CONFIG_INACTIVE_DATA_MASK: c_uint = 0x00000020;
pub const SI_CONFIG_INACTIVE_CLK_LSB: c_int = 4;
pub const SI_CONFIG_INACTIVE_CLK_MASK: c_uint = 0x00000010;
pub const SI_CONFIG_DIVIDER_LSB: c_int = 0;
pub const SI_CONFIG_DIVIDER_MASK: c_uint = 0x0000000f;
pub const SI_CS_OFFSET: c_uint = 0x00000004;
pub const SI_CS_DONE_ERR_LSB: c_int = 10;
pub const SI_CS_DONE_ERR_MASK: c_uint = 0x00000400;
pub const SI_CS_DONE_INT_LSB: c_int = 9;
pub const SI_CS_DONE_INT_MASK: c_uint = 0x00000200;
pub const SI_CS_START_LSB: c_int = 8;
pub const SI_CS_START_MASK: c_uint = 0x00000100;
pub const SI_CS_RX_CNT_LSB: c_int = 4;
pub const SI_CS_RX_CNT_MASK: c_uint = 0x000000f0;
pub const SI_CS_TX_CNT_LSB: c_int = 0;
pub const SI_CS_TX_CNT_MASK: c_uint = 0x0000000f;
pub const SI_TX_DATA0_OFFSET: c_uint = 0x00000008;
pub const SI_TX_DATA1_OFFSET: c_uint = 0x0000000c;
pub const SI_RX_DATA0_OFFSET: c_uint = 0x00000010;
pub const SI_RX_DATA1_OFFSET: c_uint = 0x00000014;
pub const CORE_CTRL_CPU_INTR_MASK: c_uint = 0x00002000;
pub const CORE_CTRL_PCIE_REG_31_MASK: c_uint = 0x00000800;
pub const CORE_CTRL_ADDRESS: c_uint = 0x0000;
pub const PCIE_INTR_ENABLE_ADDRESS: c_uint = 0x0008;
pub const PCIE_INTR_CAUSE_ADDRESS: c_uint = 0x000c;

pub const CPU_INTR_ADDRESS: c_uint = 0x0010;
pub const FW_RAM_CONFIG_ADDRESS: c_uint = 0x0018;

// Firmware indications to the Host via SCRATCH_3 register.

pub const FW_IND_EVENT_PENDING: c_int = 1;
pub const FW_IND_INITIALIZED: c_int = 2;
pub const FW_IND_HOST_READY: c_uint = 0x80000000;
// HOST_REG interrupt from firmware

pub const DRAM_BASE_ADDRESS: c_uint = 0x00400000;
pub const PCIE_BAR_REG_ADDRESS: c_uint = 0x40030;
pub const MISSING: c_int = 0;

pub const LOCAL_SCRATCH_OFFSET: c_uint = 0x18;

pub const QCA9887_1_0_I2C_SDA_GPIO_PIN: c_int = 5;
pub const QCA9887_1_0_I2C_SDA_PIN_CONFIG: c_int = 3;
pub const QCA9887_1_0_SI_CLK_GPIO_PIN: c_int = 17;
pub const QCA9887_1_0_SI_CLK_PIN_CONFIG: c_int = 3;
pub const QCA9887_1_0_GPIO_ENABLE_W1TS_LOW_ADDRESS: c_uint = 0x00000010;
pub const QCA9887_EEPROM_SELECT_READ: c_uint = 0xa10000a0;
pub const QCA9887_EEPROM_ADDR_HI_MASK: c_uint = 0x0000ff00;
pub const QCA9887_EEPROM_ADDR_HI_LSB: c_int = 8;
pub const QCA9887_EEPROM_ADDR_LO_MASK: c_uint = 0x00ff0000;
pub const QCA9887_EEPROM_ADDR_LO_LSB: c_int = 16;
pub const MBOX_RESET_CONTROL_ADDRESS: c_uint = 0x00000000;
pub const MBOX_HOST_INT_STATUS_ADDRESS: c_uint = 0x00000800;
pub const MBOX_HOST_INT_STATUS_ERROR_LSB: c_int = 7;
pub const MBOX_HOST_INT_STATUS_ERROR_MASK: c_uint = 0x00000080;
pub const MBOX_HOST_INT_STATUS_CPU_LSB: c_int = 6;
pub const MBOX_HOST_INT_STATUS_CPU_MASK: c_uint = 0x00000040;
pub const MBOX_HOST_INT_STATUS_COUNTER_LSB: c_int = 4;
pub const MBOX_HOST_INT_STATUS_COUNTER_MASK: c_uint = 0x00000010;
pub const MBOX_CPU_INT_STATUS_ADDRESS: c_uint = 0x00000801;
pub const MBOX_ERROR_INT_STATUS_ADDRESS: c_uint = 0x00000802;
pub const MBOX_ERROR_INT_STATUS_WAKEUP_LSB: c_int = 2;
pub const MBOX_ERROR_INT_STATUS_WAKEUP_MASK: c_uint = 0x00000004;
pub const MBOX_ERROR_INT_STATUS_RX_UNDERFLOW_LSB: c_int = 1;
pub const MBOX_ERROR_INT_STATUS_RX_UNDERFLOW_MASK: c_uint = 0x00000002;
pub const MBOX_ERROR_INT_STATUS_TX_OVERFLOW_LSB: c_int = 0;
pub const MBOX_ERROR_INT_STATUS_TX_OVERFLOW_MASK: c_uint = 0x00000001;
pub const MBOX_COUNTER_INT_STATUS_ADDRESS: c_uint = 0x00000803;
pub const MBOX_COUNTER_INT_STATUS_COUNTER_LSB: c_int = 0;
pub const MBOX_COUNTER_INT_STATUS_COUNTER_MASK: c_uint = 0x000000ff;
pub const MBOX_RX_LOOKAHEAD_VALID_ADDRESS: c_uint = 0x00000805;
pub const MBOX_INT_STATUS_ENABLE_ADDRESS: c_uint = 0x00000828;
pub const MBOX_INT_STATUS_ENABLE_ERROR_LSB: c_int = 7;
pub const MBOX_INT_STATUS_ENABLE_ERROR_MASK: c_uint = 0x00000080;
pub const MBOX_INT_STATUS_ENABLE_CPU_LSB: c_int = 6;
pub const MBOX_INT_STATUS_ENABLE_CPU_MASK: c_uint = 0x00000040;
pub const MBOX_INT_STATUS_ENABLE_INT_LSB: c_int = 5;
pub const MBOX_INT_STATUS_ENABLE_INT_MASK: c_uint = 0x00000020;
pub const MBOX_INT_STATUS_ENABLE_COUNTER_LSB: c_int = 4;
pub const MBOX_INT_STATUS_ENABLE_COUNTER_MASK: c_uint = 0x00000010;
pub const MBOX_INT_STATUS_ENABLE_MBOX_DATA_LSB: c_int = 0;
pub const MBOX_INT_STATUS_ENABLE_MBOX_DATA_MASK: c_uint = 0x0000000f;
pub const MBOX_CPU_INT_STATUS_ENABLE_ADDRESS: c_uint = 0x00000819;
pub const MBOX_CPU_INT_STATUS_ENABLE_BIT_LSB: c_int = 0;
pub const MBOX_CPU_INT_STATUS_ENABLE_BIT_MASK: c_uint = 0x000000ff;
pub const MBOX_CPU_STATUS_ENABLE_ASSERT_MASK: c_uint = 0x00000001;
pub const MBOX_ERROR_STATUS_ENABLE_ADDRESS: c_uint = 0x0000081a;
pub const MBOX_ERROR_STATUS_ENABLE_RX_UNDERFLOW_LSB: c_int = 1;
pub const MBOX_ERROR_STATUS_ENABLE_RX_UNDERFLOW_MASK: c_uint = 0x00000002;
pub const MBOX_ERROR_STATUS_ENABLE_TX_OVERFLOW_LSB: c_int = 0;
pub const MBOX_ERROR_STATUS_ENABLE_TX_OVERFLOW_MASK: c_uint = 0x00000001;
pub const MBOX_COUNTER_INT_STATUS_ENABLE_ADDRESS: c_uint = 0x0000081b;
pub const MBOX_COUNTER_INT_STATUS_ENABLE_BIT_LSB: c_int = 0;
pub const MBOX_COUNTER_INT_STATUS_ENABLE_BIT_MASK: c_uint = 0x000000ff;
pub const MBOX_COUNT_ADDRESS: c_uint = 0x00000820;
pub const MBOX_COUNT_DEC_ADDRESS: c_uint = 0x00000840;
pub const MBOX_WINDOW_DATA_ADDRESS: c_uint = 0x00000874;
pub const MBOX_WINDOW_WRITE_ADDR_ADDRESS: c_uint = 0x00000878;
pub const MBOX_WINDOW_READ_ADDR_ADDRESS: c_uint = 0x0000087c;
pub const MBOX_CPU_DBG_SEL_ADDRESS: c_uint = 0x00000883;
pub const MBOX_CPU_DBG_ADDRESS: c_uint = 0x00000884;
pub const MBOX_RTC_BASE_ADDRESS: c_uint = 0x00000000;
pub const MBOX_GPIO_BASE_ADDRESS: c_uint = 0x00005000;
pub const MBOX_MBOX_BASE_ADDRESS: c_uint = 0x00008000;

// Register definitions for first generation ath10k cards. These cards include
// a mac which has a register allocation similar to ath9k and at least some
// registers including the ones relevant for modifying the coverage class are
// identical to the ath9k definitions.
// These registers are usually managed by the ath10k firmware. However by
// overriding them it is possible to support coverage class modifications.
//
pub const WAVE1_PCU_ACK_CTS_TIMEOUT: c_uint = 0x8014;
pub const WAVE1_PCU_ACK_CTS_TIMEOUT_MAX: c_uint = 0x00003FFF;
pub const WAVE1_PCU_ACK_CTS_TIMEOUT_ACK_MASK: c_uint = 0x00003FFF;
pub const WAVE1_PCU_ACK_CTS_TIMEOUT_ACK_LSB: c_int = 0;
pub const WAVE1_PCU_ACK_CTS_TIMEOUT_CTS_MASK: c_uint = 0x3FFF0000;
pub const WAVE1_PCU_ACK_CTS_TIMEOUT_CTS_LSB: c_int = 16;
pub const WAVE1_PCU_GBL_IFS_SLOT: c_uint = 0x1070;
pub const WAVE1_PCU_GBL_IFS_SLOT_MASK: c_uint = 0x0000FFFF;
pub const WAVE1_PCU_GBL_IFS_SLOT_MAX: c_uint = 0x0000FFFF;
pub const WAVE1_PCU_GBL_IFS_SLOT_LSB: c_int = 0;
pub const WAVE1_PCU_GBL_IFS_SLOT_RESV0: c_uint = 0xFFFF0000;
pub const WAVE1_PHYCLK: c_uint = 0x801C;
pub const WAVE1_PHYCLK_USEC_MASK: c_uint = 0x0000007F;
pub const WAVE1_PHYCLK_USEC_LSB: c_int = 0;
// qca6174 PLL offset/mask
pub const SOC_CORE_CLK_CTRL_OFFSET: c_uint = 0x00000114;
pub const SOC_CORE_CLK_CTRL_DIV_LSB: c_int = 0;
pub const SOC_CORE_CLK_CTRL_DIV_MASK: c_uint = 0x00000007;
pub const EFUSE_OFFSET: c_uint = 0x0000032c;
pub const EFUSE_XTAL_SEL_LSB: c_int = 8;
pub const EFUSE_XTAL_SEL_MASK: c_uint = 0x00000700;
pub const BB_PLL_CONFIG_OFFSET: c_uint = 0x000002f4;
pub const BB_PLL_CONFIG_FRAC_LSB: c_int = 0;
pub const BB_PLL_CONFIG_FRAC_MASK: c_uint = 0x0003ffff;
pub const BB_PLL_CONFIG_OUTDIV_LSB: c_int = 18;
pub const BB_PLL_CONFIG_OUTDIV_MASK: c_uint = 0x001c0000;
pub const WLAN_PLL_SETTLE_OFFSET: c_uint = 0x0018;
pub const WLAN_PLL_SETTLE_TIME_LSB: c_int = 0;
pub const WLAN_PLL_SETTLE_TIME_MASK: c_uint = 0x000007ff;
pub const WLAN_PLL_CONTROL_OFFSET: c_uint = 0x0014;
pub const WLAN_PLL_CONTROL_DIV_LSB: c_int = 0;
pub const WLAN_PLL_CONTROL_DIV_MASK: c_uint = 0x000003ff;
pub const WLAN_PLL_CONTROL_REFDIV_LSB: c_int = 10;
pub const WLAN_PLL_CONTROL_REFDIV_MASK: c_uint = 0x00003c00;
pub const WLAN_PLL_CONTROL_BYPASS_LSB: c_int = 16;
pub const WLAN_PLL_CONTROL_BYPASS_MASK: c_uint = 0x00010000;
pub const WLAN_PLL_CONTROL_NOPWD_LSB: c_int = 18;
pub const WLAN_PLL_CONTROL_NOPWD_MASK: c_uint = 0x00040000;
pub const RTC_SYNC_STATUS_OFFSET: c_uint = 0x0244;
pub const RTC_SYNC_STATUS_PLL_CHANGING_LSB: c_int = 5;
pub const RTC_SYNC_STATUS_PLL_CHANGING_MASK: c_uint = 0x00000020;
// qca6174 PLL offset/mask end
// CPU_ADDR_MSB is a register, bit[3:0] is to specify which memory
// region is accessed. The memory region size is 1M.
// If host wants to access 0xX12345 at target, then CPU_ADDR_MSB[3:0]
// is 0xX.
// The following MACROs are defined to get the 0xX and the size limit.
//

pub const REGION_ACCESS_SIZE_LIMIT: c_uint = 0x100000;

