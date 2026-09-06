//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/rtw8723x.h
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
// Copyright 2024 Fiona Klute
//
// Based on code originally in rtw8723d.[ch],
// Copyright(c) 2018-2019  Realtek Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw8723x_path {
    PATH_S1,
    PATH_S0,
    PATH_NR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw8723x_iqk_round {
    IQK_ROUND_0,
    IQK_ROUND_1,
    IQK_ROUND_2,
    IQK_ROUND_HYBRID,
    IQK_ROUND_SIZE,
    IQK_ROUND_INVALID = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw8723x_iqk_result {
    IQK_S1_TX_X,
    IQK_S1_TX_Y,
    IQK_S1_RX_X,
    IQK_S1_RX_Y,
    IQK_S0_TX_X,
    IQK_S0_TX_Y,
    IQK_S0_RX_X,
    IQK_S0_RX_Y,
    IQK_NR,
    IQK_SX_NR = IQK_NR / PATH_NR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8723xe_efuse {
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0xd0,
    pub vendor_id: [u8; 2],
    pub device_id: [u8; 2],
    pub sub_vendor_id: [u8; 2],
    pub sub_device_id: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8723xu_efuse {
    pub /: *mut *mut u8 res4[48]; / 0xd0,
    pub /: *mut *mut u8 vendor_id[2]; / 0x100,
    pub /: *mut *mut u8 product_id[2]; / 0x102,
    pub /: *mut *mut u8 usb_option; / 0x104,
    pub /: *mut *mut u8 res5[2]; / 0x105,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x107,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8723xs_efuse {
    pub /: *mut *mut u8 res4[0x4a]; / 0xd0,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x11a,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8723x_efuse {
    pub rtl_id: __le16,
    pub rsvd: [u8; 2],
    pub afe: u8,
    pub rsvd1: [u8; 11],
// power index for four RF paths
    pub txpwr_idx_table: [rtw_txpwr_idx; 4],
    pub /: *mut *mut u8 channel_plan; / 0xb8,
    pub xtal_k: u8,
    pub thermal_meter: u8,
    pub iqk_lck: u8,
    pub /: *mut *mut u8 pa_type; / 0xbc,
    pub /: *mut *mut u8 lna_type_2g[2]; / 0xbd,
    pub lna_type_5g: [u8; 2],
    pub rf_board_option: u8,
    pub rf_feature_option: u8,
    pub rf_bt_setting: u8,
    pub eeprom_version: u8,
    pub eeprom_customer_id: u8,
    pub tx_bb_swing_setting_2g: u8,
    pub res_c7: u8,
    pub tx_pwr_calibrate_rate: u8,
    pub /: *mut *mut u8 rf_antenna_option; / 0xc9,
    pub rfe_option: u8,
    pub country_code: [u8; 2],
    pub res: [u8; 3],
    pub e: rtw8723xe_efuse,
    pub u: rtw8723xu_efuse,
    pub s: rtw8723xs_efuse,
}

pub const RTW8723X_IQK_ADDA_REG_NUM: c_int = 16;
pub const RTW8723X_IQK_MAC8_REG_NUM: c_int = 3;
pub const RTW8723X_IQK_MAC32_REG_NUM: c_int = 1;
pub const RTW8723X_IQK_BB_REG_NUM: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8723x_iqk_backup_regs {
    pub adda: [u32; RTW8723X_IQK_ADDA_REG_NUM],
    pub mac8: [u8; RTW8723X_IQK_MAC8_REG_NUM],
    pub mac32: [u32; RTW8723X_IQK_MAC32_REG_NUM],
    pub bb: [u32; RTW8723X_IQK_BB_REG_NUM],
    pub lte_path: u32,
    pub lte_gnt: u32,
    pub bb_sel_btg: u32,
    pub btg_sel: u8,
    pub igia: u8,
    pub igib: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8723x_common {
// registers that must be backed up before IQK and restored after
    pub iqk_adda_regs: [u32; RTW8723X_IQK_ADDA_REG_NUM],
    pub iqk_mac8_regs: [u32; RTW8723X_IQK_MAC8_REG_NUM],
    pub iqk_mac32_regs: [u32; RTW8723X_IQK_MAC32_REG_NUM],
    pub iqk_bb_regs: [u32; RTW8723X_IQK_BB_REG_NUM],
// chip register definitions
    pub ltecoex_addr: rtw_ltecoex_addr,
    pub rf_sipi_addr: [rtw_rf_sipi_addr; 2],
    pub dig: [rtw_hw_reg; 2],
    pub dig_cck: [rtw_hw_reg; 1],
    pub prioq_addrs: rtw_prioq_addrs,
// common functions
    pub rtwdev): *mut *mut void (lck)(struct rtw_dev,
    pub log_map): *mut *mut *mut int (read_efuse)(struct rtw_dev rtwdev, u8,
    pub rtwdev): *mut *mut int (mac_init)(struct rtw_dev,
    pub rtwdev): *mut *mut int (mac_postinit)(struct rtw_dev,
    pub enable): *mut *mut *mut void (cfg_ldo25)(struct rtw_dev rtwdev, bool,
    pub rtwdev): *mut *mut void (set_tx_power_index)(struct rtw_dev,
    pub on): *mut *mut *mut void (efuse_grant)(struct rtw_dev rtwdev, bool,
    pub rtwdev): *mut *mut void (false_alarm_statistics)(struct rtw_dev,
    pub backup): *mut rtw8723x_iqk_backup_regs,
    pub backup): *const rtw8723x_iqk_backup_regs,
    pub c2): u8 c1, u8,
    pub rtwdev): *mut *mut u8 (pwrtrack_get_limit_ofdm)(struct rtw_dev,
    pub delta): u8,
    pub rtwdev): *mut *mut void (coex_cfg_init)(struct rtw_dev,
    pub txdesc): *mut u8,
    pub tx_path_count): c_int,
}

pub const PATH_IQK_RETRY: c_int = 2;
pub const MAX_TOLERANCE: c_int = 5;
pub const IQK_TX_X_ERR: c_uint = 0x142;
pub const IQK_TX_Y_ERR: c_uint = 0x42;
pub const IQK_RX_X_ERR: c_uint = 0x132;
pub const IQK_RX_Y_ERR: c_uint = 0x36;
pub const IQK_RX_X_UPPER: c_uint = 0x11a;
pub const IQK_RX_X_LOWER: c_uint = 0xe6;
pub const IQK_RX_Y_LMT: c_uint = 0x1a;

pub const WLAN_TXQ_RPT_EN: c_uint = 0x1F;
pub const SPUR_THRES: c_uint = 0x16;
pub const DIS_3WIRE: c_uint = 0xccf000c0;
pub const EN_3WIRE: c_uint = 0xccc000c0;
pub const START_PSD: c_uint = 0x400000;
pub const FREQ_CH5: c_uint = 0xfccd;
pub const FREQ_CH6: c_uint = 0xfc4d;
pub const FREQ_CH7: c_uint = 0xffcd;
pub const FREQ_CH8: c_uint = 0xff4d;
pub const FREQ_CH13: c_uint = 0xfccd;
pub const FREQ_CH14: c_uint = 0xff9a;

pub const REG_GPIO_INTM: c_uint = 0x0048;
pub const REG_BTG_SEL: c_uint = 0x0067;

pub const REG_LTECOEX_PATH_CONTROL: c_uint = 0x0070;
pub const REG_LTECOEX_CTRL: c_uint = 0x07c0;
pub const REG_LTECOEX_WRITE_DATA: c_uint = 0x07c4;
pub const REG_LTECOEX_READ_DATA: c_uint = 0x07c8;
pub const REG_PSDFN: c_uint = 0x0808;
pub const REG_BB_PWR_SAV1_11N: c_uint = 0x0874;
pub const REG_ANA_PARAM1: c_uint = 0x0880;
pub const REG_ANALOG_P4: c_uint = 0x088c;
pub const REG_PSDRPT: c_uint = 0x08b4;
pub const REG_FPGA1_RFMOD: c_uint = 0x0900;
pub const REG_BB_SEL_BTG: c_uint = 0x0948;
pub const REG_BBRX_DFIR: c_uint = 0x0954;

pub const REG_CCK0_SYS: c_uint = 0x0a00;

pub const REG_CCK_ANT_SEL_11N: c_uint = 0x0a04;
pub const REG_PWRTH: c_uint = 0x0a08;
pub const REG_CCK_FA_RST_11N: c_uint = 0x0a2c;

pub const REG_CCK_FA_LSB_11N: c_uint = 0x0a5c;
pub const REG_CCK_FA_MSB_11N: c_uint = 0x0a58;
pub const REG_CCK_CCA_CNT_11N: c_uint = 0x0a60;

pub const REG_PWRTH2: c_uint = 0x0aa8;
pub const REG_CSRATIO: c_uint = 0x0aaa;
pub const REG_OFDM_FA_HOLDC_11N: c_uint = 0x0c00;

pub const REG_BB_RX_PATH_11N: c_uint = 0x0c04;
pub const REG_TRMUX_11N: c_uint = 0x0c08;
pub const REG_OFDM_FA_RSTC_11N: c_uint = 0x0c0c;

pub const REG_A_RXIQI: c_uint = 0x0c14;
pub const BIT_MASK_RXIQ_S1_X: c_uint = 0x000003FF;
pub const BIT_MASK_RXIQ_S1_Y1: c_uint = 0x0000FC00;

pub const REG_OFDM0_RXDSP: c_uint = 0x0c40;

pub const REG_OFDM_0_ECCA_THRESHOLD: c_uint = 0x0c4c;

pub const REG_OFDM0_XAAGC1: c_uint = 0x0c50;
pub const REG_OFDM0_XBAGC1: c_uint = 0x0c58;
pub const REG_AGCRSSI: c_uint = 0x0c78;
pub const REG_OFDM_0_XA_TX_IQ_IMBALANCE: c_uint = 0x0c80;
pub const REG_OFDM_0_XB_TX_IQ_IMBALANCE: c_uint = 0x0c88;
pub const BIT_MASK_TXIQ_ELM_A: c_uint = 0x03ff;

pub const REG_TXIQK_MATRIXA_LSB2_11N: c_uint = 0x0c94;

pub const REG_RXIQK_MATRIX_LSB_11N: c_uint = 0x0ca0;
pub const BIT_MASK_RXIQ_S1_Y2: c_uint = 0xF0000000;

pub const REG_TXIQ_AB_S0: c_uint = 0x0cd0;
pub const BIT_MASK_TXIQ_A_S0: c_uint = 0x000007FE;

pub const BIT_MASK_TXIQ_B_S0: c_uint = 0x0007E000;
pub const REG_TXIQ_CD_S0: c_uint = 0x0cd4;
pub const BIT_MASK_TXIQ_C_S0: c_uint = 0x000007FE;

pub const REG_RXIQ_AB_S0: c_uint = 0x0cd8;
pub const BIT_MASK_RXIQ_X_S0: c_uint = 0x000003FF;
pub const BIT_MASK_RXIQ_Y_S0: c_uint = 0x003FF000;
pub const REG_OFDM_FA_TYPE1_11N: c_uint = 0x0cf0;

pub const REG_OFDM_FA_RSTD_11N: c_uint = 0x0d00;

pub const REG_CTX: c_uint = 0x0d03;

pub const REG_OFDM1_CFOTRK: c_uint = 0x0d2c;

pub const REG_OFDM1_CSI1: c_uint = 0x0d40;
pub const REG_OFDM1_CSI2: c_uint = 0x0d44;
pub const REG_OFDM1_CSI3: c_uint = 0x0d48;
pub const REG_OFDM1_CSI4: c_uint = 0x0d4c;
pub const REG_OFDM_FA_TYPE2_11N: c_uint = 0x0da0;

pub const REG_OFDM_FA_TYPE3_11N: c_uint = 0x0da4;

pub const REG_OFDM_FA_TYPE4_11N: c_uint = 0x0da8;

pub const REG_FPGA0_IQK_11N: c_uint = 0x0e28;
pub const BIT_MASK_IQK_MOD: c_uint = 0xffffff00;
pub const EN_IQK: c_uint = 0x808000;
pub const RST_IQK: c_uint = 0x000000;
pub const REG_TXIQK_TONE_A_11N: c_uint = 0x0e30;
pub const REG_RXIQK_TONE_A_11N: c_uint = 0x0e34;
pub const REG_TXIQK_PI_A_11N: c_uint = 0x0e38;
pub const REG_RXIQK_PI_A_11N: c_uint = 0x0e3c;
pub const REG_TXIQK_11N: c_uint = 0x0e40;

pub const REG_RXIQK_11N: c_uint = 0x0e44;
pub const REG_IQK_AGC_PTS_11N: c_uint = 0x0e48;
pub const REG_IQK_AGC_RSP_11N: c_uint = 0x0e4c;
pub const REG_TX_IQK_TONE_B: c_uint = 0x0e50;
pub const REG_RX_IQK_TONE_B: c_uint = 0x0e54;
pub const REG_TXIQK_PI_B: c_uint = 0x0e58;
pub const REG_RXIQK_PI_B: c_uint = 0x0e5c;
pub const REG_IQK_RES_TX: c_uint = 0x0e94;

pub const REG_IQK_RES_TY: c_uint = 0x0e9c;

pub const REG_IQK_RES_RX: c_uint = 0x0ea4;

pub const REG_IQK_RES_RY: c_uint = 0x0eac;

pub const REG_PAGE_F_RST_11N: c_uint = 0x0f14;

pub const REG_IGI_C_11N: c_uint = 0x0f84;
pub const REG_IGI_D_11N: c_uint = 0x0f88;
pub const REG_HT_CRC32_CNT_11N: c_uint = 0x0f90;

pub const REG_OFDM_CRC32_CNT_11N: c_uint = 0x0f94;

pub const REG_HT_CRC32_CNT_11N_AGG: c_uint = 0x0fb8;

// val is Q10.8
extern "C" {
    pub fn sign_extend32(_arg: val, _arg: 9) -> return;
}
// x, y and return value are Q10.8
// ext = (t >> 7) & 0x1;	/* Q.16 --> Q.9; get LSB of Q.9
// IQK helper functions, defined as inline so they can be shared
// without needing an EXPORT_SYMBOL each.
//
// set all ADDA registers to the given value
