//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/rtw8822c.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8822cu_efuse {
    pub /: *mut *mut u8 res0[0x30]; / 0x120,
    pub /: *mut *mut u8 vid[2]; / 0x150,
    pub pid: [u8; 2],
    pub res1: [u8; 3],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x157,
    pub res2: [u8; 0x3d],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8822cs_efuse {
    pub /: *mut *mut u8 res0[0x4a]; / 0x120,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x16a,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8822ce_efuse {
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x120,
    pub vender_id: [u8; 2],
    pub device_id: [u8; 2],
    pub sub_vender_id: [u8; 2],
    pub sub_device_id: [u8; 2],
    pub pmc: [u8; 2],
    pub exp_device_cap: [u8; 2],
    pub msi_cap: u8,
    pub /: *mut *mut u8 ltr_cap; / 0x133,
    pub exp_link_control: [u8; 2],
    pub link_cap: [u8; 4],
    pub link_control: [u8; 2],
    pub serial_number: [u8; 8],
    pub /: *mut *mut u8 res0:2; / 0x144,
    pub ltr_en:1: u8,
    pub res1:2: u8,
    pub obff:2: u8,
    pub res2_1:1: u8,
    pub res2_2:2: u8,
    pub obff_cap:2: u8,
    pub res3:4: u8,
    pub class_code: [u8; 3],
    pub res4: u8,
    pub pci_pm_L1_2_supp:1: u8,
    pub pci_pm_L1_1_supp:1: u8,
    pub aspm_pm_L1_2_supp:1: u8,
    pub aspm_pm_L1_1_supp:1: u8,
    pub L1_pm_substates_supp:1: u8,
    pub res5:3: u8,
    pub port_common_mode_restore_time: u8,
    pub port_t_power_on_scale:2: u8,
    pub res6:1: u8,
    pub port_t_power_on_value:5: u8,
    pub res7: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8822c_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 4],
    pub usb_mode: u8,
    pub res1: [u8; 0x09],
// power index for four RF paths
    pub txpwr_idx_table: [rtw_txpwr_idx; 4],
    pub /: *mut *mut u8 channel_plan; / 0xb8,
    pub xtal_k: u8,
    pub res2: u8,
    pub iqk_lck: u8,
    pub /: *mut *mut u8 res3[5]; / 0xbc,
    pub rf_board_option: u8,
    pub rf_feature_option: u8,
    pub rf_bt_setting: u8,
    pub eeprom_version: u8,
    pub eeprom_customer_id: u8,
    pub tx_bb_swing_setting_2g: u8,
    pub tx_bb_swing_setting_5g: u8,
    pub tx_pwr_calibrate_rate: u8,
    pub /: *mut *mut u8 rf_antenna_option; / 0xc9,
    pub rfe_option: u8,
    pub country_code: [u8; 2],
    pub res4: [u8; 3],
    pub /: *mut *mut u8 path_a_thermal; / 0xd0,
    pub path_b_thermal: u8,
    pub res5: [u8; 2],
    pub rx_gain_gap_2g_ofdm: u8,
    pub res6: u8,
    pub rx_gain_gap_2g_cck: u8,
    pub res7: u8,
    pub rx_gain_gap_5gl: u8,
    pub res8: u8,
    pub rx_gain_gap_5gm: u8,
    pub res9: u8,
    pub rx_gain_gap_5gh: u8,
    pub res10: u8,
    pub res11: [u8; 0x42],
    pub e: rtw8822ce_efuse,
    pub u: rtw8822cu_efuse,
    pub s: rtw8822cs_efuse,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw8822c_dpk_agc_phase {
    RTW_DPK_GAIN_CHECK,
    RTW_DPK_GAIN_LARGE,
    RTW_DPK_GAIN_LESS,
    RTW_DPK_GL_LARGE,
    RTW_DPK_GL_LESS,
    RTW_DPK_LOSS_CHECK,
    RTW_DPK_AGC_OUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw8822c_dpk_one_shot_action {
    RTW_DPK_CAL_PWR,
    RTW_DPK_GAIN_LOSS,
    RTW_DPK_DO_DPK,
    RTW_DPK_DPK_ON,
    RTW_DPK_DAGC,
    RTW_DPK_ACTION_MAX
}

pub const DACK_PATH_8822C: c_int = 2;
pub const DACK_REG_8822C: c_int = 16;
pub const DACK_RF_8822C: c_int = 1;
pub const DACK_SN_8822C: c_int = 100;
// phy status page0

// phy status page1

pub const RTW8822C_EDCCA_MAX: c_uint = 0x7f;
pub const REG_ANAPARLDO_POW_MAC: c_uint = 0x0029;

pub const CFO_TRK_ENABLE_TH: c_int = 20;
pub const CFO_TRK_STOP_TH: c_int = 10;
pub const CFO_TRK_ADJ_TH: c_int = 10;
pub const REG_TXDFIR0: c_uint = 0x808;
pub const REG_DFIRBW: c_uint = 0x810;
pub const REG_ANTMAP0: c_uint = 0x820;

pub const REG_ANTMAP: c_uint = 0x824;
pub const REG_EDCCA_DECISION: c_uint = 0x844;

pub const REG_DYMPRITH: c_uint = 0x86c;
pub const REG_DYMENTH0: c_uint = 0x870;
pub const REG_DYMENTH: c_uint = 0x874;
pub const REG_SBD: c_uint = 0x88c;

pub const REG_DYMTHMIN: c_uint = 0x8a4;
pub const REG_TXBWCTL: c_uint = 0x9b0;
pub const REG_TXCLK: c_uint = 0x9b4;
pub const REG_SCOTRK: c_uint = 0xc30;
pub const REG_MRCM: c_uint = 0xc38;
pub const REG_AGCSWSH: c_uint = 0xc44;
pub const REG_ANTWTPD: c_uint = 0xc54;
pub const REG_PT_CHSMO: c_uint = 0xcbc;

pub const REG_ORITXCODE: c_uint = 0x1800;

pub const REG_3WIRE: c_uint = 0x180c;

pub const REG_ANAPAR_A: c_uint = 0x1830;

pub const REG_RFTXEN_GCK_A: c_uint = 0x1864;

pub const REG_DIS_SHARE_RX_A: c_uint = 0x186c;

pub const REG_RXAGCCTL0: c_uint = 0x18ac;

pub const REG_DCKA_I_0: c_uint = 0x18bc;
pub const REG_DCKA_I_1: c_uint = 0x18c0;
pub const REG_DCKA_Q_0: c_uint = 0x18d8;
pub const REG_DCKA_Q_1: c_uint = 0x18dc;
pub const REG_CCKSB: c_uint = 0x1a00;

pub const REG_RXCCKSEL: c_uint = 0x1a04;
pub const REG_BGCTRL: c_uint = 0x1a14;

pub const REG_TXF0: c_uint = 0x1a20;
pub const REG_TXF1: c_uint = 0x1a24;
pub const REG_TXF2: c_uint = 0x1a28;
pub const REG_CCANRX: c_uint = 0x1a2c;

pub const REG_CCK_FACNT: c_uint = 0x1a5c;
pub const REG_CCKTXONLY: c_uint = 0x1a80;

pub const REG_TXF3: c_uint = 0x1a98;
pub const REG_TXF4: c_uint = 0x1a9c;
pub const REG_TXF5: c_uint = 0x1aa0;
pub const REG_TXF6: c_uint = 0x1aac;
pub const REG_TXF7: c_uint = 0x1ab0;
pub const REG_CCK_SOURCE: c_uint = 0x1abc;

pub const REG_NCTL0: c_uint = 0x1b00;

pub const REG_DPD_CTL0_S0: c_uint = 0x1b04;

pub const REG_DPD_CTL1_S0: c_uint = 0x1b08;

pub const REG_IQKSTAT: c_uint = 0x1b10;
pub const REG_IQK_CTL1: c_uint = 0x1b20;

pub const REG_TX_TONE_IDX: c_uint = 0x1b2c;
pub const REG_DPD_LUT0: c_uint = 0x1b44;

pub const REG_DPD_CTL0_S1: c_uint = 0x1b5c;
pub const REG_DPD_CTL1_S1: c_uint = 0x1b60;
pub const REG_DPD_AGC: c_uint = 0x1b67;
pub const REG_TABLE_SEL: c_uint = 0x1b98;

pub const REG_TX_GAIN_SET: c_uint = 0x1b9c;

pub const REG_DPD_CTL0: c_uint = 0x1bb4;
pub const REG_SINGLE_TONE_SW: c_uint = 0x1bb8;

pub const REG_R_CONFIG: c_uint = 0x1bcc;

pub const BIT_2G_SWING: c_uint = 0x2d;
pub const BIT_5G_SWING: c_uint = 0x36;
pub const REG_RXSRAM_CTL: c_uint = 0x1bd4;

pub const REG_DPD_CTL11: c_uint = 0x1be4;
pub const REG_DPD_CTL12: c_uint = 0x1be8;
pub const REG_DPD_CTL15: c_uint = 0x1bf4;
pub const REG_DPD_CTL16: c_uint = 0x1bf8;
pub const REG_STAT_RPT: c_uint = 0x1bfc;

pub const REG_TXANT: c_uint = 0x1c28;
pub const REG_IQK_CTRL: c_uint = 0x1c38;
pub const REG_ENCCK: c_uint = 0x1c3c;

pub const REG_CCAMSK: c_uint = 0x1c80;
pub const REG_RSTB: c_uint = 0x1c90;

pub const REG_CH_DELAY_EXTR2: c_uint = 0x1cd0;

pub const REG_RX_BREAK: c_uint = 0x1d2c;

pub const REG_RXFNCTL: c_uint = 0x1d30;
pub const REG_CCA_OFF: c_uint = 0x1d58;

pub const REG_RXIGI: c_uint = 0x1d70;
pub const REG_ENFN: c_uint = 0x1e24;

pub const REG_TXANTSEG: c_uint = 0x1e28;

pub const REG_TXLGMAP: c_uint = 0x1e2c;
pub const REG_CCKPATH: c_uint = 0x1e5c;
pub const REG_TX_FIFO: c_uint = 0x1e70;

pub const REG_CNT_CTRL: c_uint = 0x1eb4;

pub const REG_OFDM_FACNT: c_uint = 0x2d00;
pub const REG_OFDM_FACNT1: c_uint = 0x2d04;
pub const REG_OFDM_FACNT2: c_uint = 0x2d08;
pub const REG_OFDM_FACNT3: c_uint = 0x2d0c;
pub const REG_OFDM_FACNT4: c_uint = 0x2d10;
pub const REG_OFDM_FACNT5: c_uint = 0x2d20;
pub const REG_RPT_CIP: c_uint = 0x2d9c;

pub const REG_OFDM_TXCNT: c_uint = 0x2de0;
pub const REG_ORITXCODE2: c_uint = 0x4100;
pub const REG_3WIRE2: c_uint = 0x410c;
pub const REG_ANAPAR_B: c_uint = 0x4130;
pub const REG_RFTXEN_GCK_B: c_uint = 0x4164;
pub const REG_DIS_SHARE_RX_B: c_uint = 0x416c;

pub const REG_RXAGCCTL: c_uint = 0x41ac;
pub const REG_DCKB_I_0: c_uint = 0x41bc;
pub const REG_DCKB_I_1: c_uint = 0x41c0;
pub const REG_DCKB_Q_0: c_uint = 0x41d8;
pub const REG_DCKB_Q_1: c_uint = 0x41dc;
pub const RF_MODE_TRXAGC: c_uint = 0x00;

pub const RF_RXAGC_OFFSET: c_uint = 0x19;
pub const RF_BW_TRXBB: c_uint = 0x1a;

pub const RF_TX_GAIN_OFFSET: c_uint = 0x55;

pub const RF_TX_GAIN: c_uint = 0x56;

pub const RF_IDAC: c_uint = 0x58;

pub const RF_TX_RESULT: c_uint = 0x5f;

pub const RF_PA: c_uint = 0x60;

pub const RF_TXA_LB_SW: c_uint = 0x63;

pub const RF_RXG_GAIN: c_uint = 0x87;

pub const RF_RXA_MIX_GAIN: c_uint = 0x8a;

pub const RF_EXT_TIA_BW: c_uint = 0x8f;

pub const RF_DIS_BYPASS_TXBB: c_uint = 0x9e;

pub const RF_DEBUG: c_uint = 0xde;

pub const PPG_THERMAL_B: c_uint = 0x1b0;

pub const PPG_2GH_TXAB: c_uint = 0x1d2;

pub const PPG_2GL_TXAB: c_uint = 0x1d4;
pub const PPG_PABIAS_2GB: c_uint = 0x1d5;
pub const PPG_PABIAS_2GA: c_uint = 0x1d6;

pub const PPG_PABIAS_5GB: c_uint = 0x1d7;
pub const PPG_PABIAS_5GA: c_uint = 0x1d8;

pub const PPG_5GH1_TXB: c_uint = 0x1db;
pub const PPG_5GH1_TXA: c_uint = 0x1dc;
pub const PPG_5GM2_TXB: c_uint = 0x1df;
pub const PPG_5GM2_TXA: c_uint = 0x1e0;
pub const PPG_5GM1_TXB: c_uint = 0x1e3;
pub const PPG_5GM1_TXA: c_uint = 0x1e4;
pub const PPG_5GL2_TXB: c_uint = 0x1e7;
pub const PPG_5GL2_TXA: c_uint = 0x1e8;
pub const PPG_5GL1_TXB: c_uint = 0x1eb;
pub const PPG_5GL1_TXA: c_uint = 0x1ec;
pub const PPG_2GM_TXAB: c_uint = 0x1ee;
pub const PPG_THERMAL_A: c_uint = 0x1ef;
