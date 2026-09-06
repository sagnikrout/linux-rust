//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/rtw8821c.h
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
pub struct rtw8821cu_efuse {
    pub /: *mut *mut u8 res4[4]; / 0xd0,
    pub usb_optional_function: u8,
    pub res5: [u8; 0x1e],
    pub res6: [u8; 2],
    pub /: *mut *mut u8 serial[0x0b]; / 0xf5,
    pub /: *mut *mut u8 vid; / 0x100,
    pub res7: u8,
    pub pid: u8,
    pub res8: [u8; 4],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x107,
    pub res9: [u8; 2],
    pub vendor_name: [u8; 0x07],
    pub res10: [u8; 2],
    pub device_name: [u8; 0x14],
    pub res11: [u8; 0xcf],
    pub /: *mut *mut u8 package_type; / 0x1fb,
    pub res12: [u8; 0x4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8821ce_efuse {
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0xd0,
    pub vender_id: [u8; 2],
    pub device_id: [u8; 2],
    pub sub_vender_id: [u8; 2],
    pub sub_device_id: [u8; 2],
    pub pmc: [u8; 2],
    pub exp_device_cap: [u8; 2],
    pub msi_cap: u8,
    pub /: *mut *mut u8 ltr_cap; / 0xe3,
    pub exp_link_control: [u8; 2],
    pub link_cap: [u8; 4],
    pub link_control: [u8; 2],
    pub serial_number: [u8; 8],
    pub /: *mut *mut u8 res0:2; / 0xf4,
    pub ltr_en:1: u8,
    pub res1:2: u8,
    pub obff:2: u8,
    pub res2_1:1: u8,
    pub res2_2:2: u8,
    pub obff_cap:2: u8,
    pub res3:4: u8,
    pub res4: [u8; 3],
    pub class_code: [u8; 3],
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
pub struct rtw8821cs_efuse {
    pub /: *mut *mut u8 res4[0x4a]; / 0xd0,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x11a,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw8821c_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 0x0e],
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
    pub tx_bb_swing_setting_5g: u8,
    pub tx_pwr_calibrate_rate: u8,
    pub /: *mut *mut u8 rf_antenna_option; / 0xc9,
    pub rfe_option: u8,
    pub country_code: [u8; 2],
    pub res: [u8; 3],
    pub e: rtw8821ce_efuse,
    pub u: rtw8821cu_efuse,
    pub s: rtw8821cs_efuse,
}

// 0xC00-0xCFF and 0xE00-0xEFF have the same layout

pub const WLAN_SLOT_TIME: c_uint = 0x09;
pub const WLAN_PIFS_TIME: c_uint = 0x19;
pub const WLAN_SIFS_CCK_CONT_TX: c_uint = 0xA;
pub const WLAN_SIFS_OFDM_CONT_TX: c_uint = 0xE;
pub const WLAN_SIFS_CCK_TRX: c_uint = 0x10;
pub const WLAN_SIFS_OFDM_TRX: c_uint = 0x10;
pub const WLAN_VO_TXOP_LIMIT: c_uint = 0x186;
pub const WLAN_VI_TXOP_LIMIT: c_uint = 0x3BC;
pub const WLAN_RDG_NAV: c_uint = 0x05;
pub const WLAN_TXOP_NAV: c_uint = 0x1B;
pub const WLAN_CCK_RX_TSF: c_uint = 0x30;
pub const WLAN_OFDM_RX_TSF: c_uint = 0x30;
pub const WLAN_TBTT_PROHIBIT: c_uint = 0x04;
pub const WLAN_TBTT_HOLD_TIME: c_uint = 0x064;
pub const WLAN_DRV_EARLY_INT: c_uint = 0x04;
pub const WLAN_BCN_DMA_TIME: c_uint = 0x02;
pub const WLAN_RX_FILTER0: c_uint = 0xFFFF;
pub const WLAN_RX_FILTER1: c_uint = 0x0FFF;
pub const WLAN_RX_FILTER2: c_uint = 0xFFFF;
pub const WLAN_RCR_CFG: c_uint = 0xE400220E;
pub const WLAN_RXPKT_MAX_SZ: c_int = 12288;

pub const WLAN_AMPDU_MAX_TIME: c_uint = 0x70;
pub const WLAN_RTS_LEN_TH: c_uint = 0xFF;
pub const WLAN_RTS_TX_TIME_TH: c_uint = 0x08;
pub const WLAN_MAX_AGG_PKT_LIMIT: c_uint = 0x20;
pub const WLAN_RTS_MAX_AGG_PKT_LIMIT: c_uint = 0x20;
pub const FAST_EDCA_VO_TH: c_uint = 0x06;
pub const FAST_EDCA_VI_TH: c_uint = 0x06;
pub const FAST_EDCA_BE_TH: c_uint = 0x06;
pub const FAST_EDCA_BK_TH: c_uint = 0x06;
pub const WLAN_BAR_RETRY_LIMIT: c_uint = 0x01;
pub const WLAN_RA_TRY_RATE_AGG_LIMIT: c_uint = 0x08;
pub const WLAN_TX_FUNC_CFG1: c_uint = 0x30;
pub const WLAN_TX_FUNC_CFG2: c_uint = 0x30;
pub const WLAN_MAC_OPT_NORM_FUNC1: c_uint = 0x98;
pub const WLAN_MAC_OPT_LB_FUNC1: c_uint = 0x80;
pub const WLAN_MAC_OPT_FUNC2: c_uint = 0xb0810041;

pub const WLAN_PRE_TXCNT_TIME_TH: c_uint = 0x1E4;
// phy status page0

// phy status page1

pub const REG_SYS_CTRL: c_uint = 0x000;

pub const REG_INIRTS_RATE_SEL: c_uint = 0x0480;
pub const REG_HTSTFWT: c_uint = 0x800;
pub const REG_RXCCAMSK: c_uint = 0x814;
pub const REG_L1WT: c_uint = 0x83c;
pub const REG_L1PKWT: c_uint = 0x840;
pub const REG_MRC: c_uint = 0x850;
pub const REG_ADC40: c_uint = 0x8c8;
pub const REG_CHFIR: c_uint = 0x8f0;
pub const REG_CDDTXP: c_uint = 0x93c;
pub const REG_TXPSEL1: c_uint = 0x940;
pub const REG_ACBB0: c_uint = 0x948;
pub const REG_ACBBRXFIR: c_uint = 0x94c;
pub const REG_ACGG2TBL: c_uint = 0x958;
pub const REG_ADCINI: c_uint = 0xa04;
pub const REG_PWRTH: c_uint = 0xa08;
pub const REG_CCA_FLTR: c_uint = 0xa20;
pub const REG_TXSF2: c_uint = 0xa24;
pub const REG_TXSF6: c_uint = 0xa28;
pub const REG_RXDESC: c_uint = 0xa2c;
pub const REG_ENTXCCK: c_uint = 0xa80;
pub const BTG_LNA: c_uint = 0xfc84;
pub const WLG_LNA: c_uint = 0x7532;
pub const REG_ENRXCCA: c_uint = 0xa84;
pub const BTG_CCA: c_uint = 0x0e;
pub const WLG_CCA: c_uint = 0x12;
pub const REG_PWRTH2: c_uint = 0xaa8;
pub const REG_CSRATIO: c_uint = 0xaaa;
pub const REG_TXFILTER: c_uint = 0xaac;
pub const REG_AGCTR_A: c_uint = 0xc08;
pub const REG_TXDFIR: c_uint = 0xc20;
pub const REG_TRSW: c_uint = 0xca0;
pub const REG_RFESEL0: c_uint = 0xcb0;
pub const REG_RFESEL8: c_uint = 0xcb4;
pub const REG_RFECTL: c_uint = 0xcb8;

pub const REG_RFEINV: c_uint = 0xcbc;
pub const REG_AGCTR_B: c_uint = 0xe08;
pub const REG_DMEM_CTRL: c_uint = 0x1080;

pub const REG_ANTWT: c_uint = 0x1904;
pub const REG_IQKFAILMSK: c_uint = 0x1bf0;

pub const BT_CNT_ENABLE: c_uint = 0x1;

pub const BCN_PRI_EN: c_uint = 0x1;
pub const PTA_CTRL_PIN: c_uint = 0x66;
pub const DPDT_CTRL_PIN: c_uint = 0x77;
pub const ANTDIC_CTRL_PIN: c_uint = 0x88;
pub const REG_CTRL_TYPE: c_uint = 0x67;

