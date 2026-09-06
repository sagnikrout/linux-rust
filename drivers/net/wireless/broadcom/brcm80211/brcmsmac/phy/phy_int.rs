//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/phy/phy_int.h
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
// Copyright (c) 2010 Broadcom Corporation
//

pub const LCNXN_BASEREV: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_phy_srom_fem {
// TSSI positive slope, 1: positive, 0: negative
    pub tssipos: u8,
// Ext PA gain-type: full-gain: 0, pa-lite: 1, no_pa: 2
    pub extpagain: u8,
// support 32 combinations of different Pdet dynamic ranges
    pub pdetrange: u8,
// TR switch isolation
    pub triso: u8,
// antswctrl lookup table configuration: 32 possible choices
    pub antswctrllut: u8,
}

pub const CH_5G_GROUP: c_int = 3;
pub const A_LOW_CHANS: c_int = 0;
pub const A_MID_CHANS: c_int = 1;
pub const A_HIGH_CHANS: c_int = 2;
pub const CH_2G_GROUP: c_int = 1;
pub const G_ALL_CHANS: c_int = 0;
pub const FIRST_REF5_CHANNUM: c_int = 149;
pub const LAST_REF5_CHANNUM: c_int = 165;
pub const FIRST_5G_CHAN: c_int = 14;
pub const LAST_5G_CHAN: c_int = 50;
pub const FIRST_MID_5G_CHAN: c_int = 14;
pub const LAST_MID_5G_CHAN: c_int = 35;
pub const FIRST_HIGH_5G_CHAN: c_int = 36;
pub const LAST_HIGH_5G_CHAN: c_int = 41;
pub const FIRST_LOW_5G_CHAN: c_int = 42;
pub const LAST_LOW_5G_CHAN: c_int = 50;
pub const BASE_LOW_5G_CHAN: c_int = 4900;
pub const BASE_MID_5G_CHAN: c_int = 5100;
pub const BASE_HIGH_5G_CHAN: c_int = 5500;

pub const TXP_FIRST_CCK: c_int = 0;
pub const TXP_LAST_CCK: c_int = 3;
pub const TXP_FIRST_OFDM: c_int = 4;
pub const TXP_LAST_OFDM: c_int = 11;
pub const TXP_FIRST_OFDM_20_CDD: c_int = 12;
pub const TXP_LAST_OFDM_20_CDD: c_int = 19;
pub const TXP_FIRST_MCS_20_SISO: c_int = 20;
pub const TXP_LAST_MCS_20_SISO: c_int = 27;
pub const TXP_FIRST_MCS_20_CDD: c_int = 28;
pub const TXP_LAST_MCS_20_CDD: c_int = 35;
pub const TXP_FIRST_MCS_20_STBC: c_int = 36;
pub const TXP_LAST_MCS_20_STBC: c_int = 43;
pub const TXP_FIRST_MCS_20_SDM: c_int = 44;
pub const TXP_LAST_MCS_20_SDM: c_int = 51;
pub const TXP_FIRST_OFDM_40_SISO: c_int = 52;
pub const TXP_LAST_OFDM_40_SISO: c_int = 59;
pub const TXP_FIRST_OFDM_40_CDD: c_int = 60;
pub const TXP_LAST_OFDM_40_CDD: c_int = 67;
pub const TXP_FIRST_MCS_40_SISO: c_int = 68;
pub const TXP_LAST_MCS_40_SISO: c_int = 75;
pub const TXP_FIRST_MCS_40_CDD: c_int = 76;
pub const TXP_LAST_MCS_40_CDD: c_int = 83;
pub const TXP_FIRST_MCS_40_STBC: c_int = 84;
pub const TXP_LAST_MCS_40_STBC: c_int = 91;
pub const TXP_FIRST_MCS_40_SDM: c_int = 92;
pub const TXP_LAST_MCS_40_SDM: c_int = 99;
pub const TXP_MCS_32: c_int = 100;
pub const TXP_NUM_RATES: c_int = 101;
pub const ADJ_PWR_TBL_LEN: c_int = 84;
pub const TXP_FIRST_SISO_MCS_20: c_int = 20;
pub const TXP_LAST_SISO_MCS_20: c_int = 27;
pub const PHY_CORE_NUM_1: c_int = 1;
pub const PHY_CORE_NUM_2: c_int = 2;
pub const PHY_CORE_NUM_3: c_int = 3;
pub const PHY_CORE_NUM_4: c_int = 4;

pub const PHY_CORE_0: c_int = 0;
pub const PHY_CORE_1: c_int = 1;
pub const PHY_CORE_2: c_int = 2;
pub const PHY_CORE_3: c_int = 3;
pub const MA_WINDOW_SZ: c_int = 8;
pub const PHY_NOISE_SAMPLE_MON: c_int = 1;
pub const PHY_NOISE_SAMPLE_EXTERNAL: c_int = 2;
pub const PHY_NOISE_WINDOW_SZ: c_int = 16;
pub const PHY_NOISE_GLITCH_INIT_MA: c_int = 10;
pub const PHY_NOISE_GLITCH_INIT_MA_BADPlCP: c_int = 10;
pub const PHY_NOISE_STATE_MON: c_uint = 0x1;
pub const PHY_NOISE_STATE_EXTERNAL: c_uint = 0x2;
pub const PHY_NOISE_SAMPLE_LOG_NUM_NPHY: c_int = 10;
pub const PHY_NOISE_SAMPLE_LOG_NUM_UCODE: c_int = 9;

pub const PHY_NOISE_MA_WINDOW_SZ: c_int = 2;
pub const PHY_RSSI_TABLE_SIZE: c_int = 64;
pub const RSSI_ANT_MERGE_MAX: c_int = 0;
pub const RSSI_ANT_MERGE_MIN: c_int = 1;
pub const RSSI_ANT_MERGE_AVG: c_int = 2;
pub const PHY_TSSI_TABLE_SIZE: c_int = 64;
pub const APHY_TSSI_TABLE_SIZE: c_int = 256;
pub const TX_GAIN_TABLE_LENGTH: c_int = 64;
pub const DEFAULT_11A_TXP_IDX: c_int = 24;
pub const NUM_TSSI_FRAMES: c_int = 4;
pub const NULL_TSSI: c_uint = 0x7f;
pub const NULL_TSSI_W: c_uint = 0x7f7f;
pub const PHY_PAPD_EPS_TBL_SIZE_LCNPHY: c_int = 64;
pub const LCNPHY_PERICAL_TEMPBASED_TXPWRCTRL: c_int = 9;
pub const PHY_TXPWR_MIN: c_int = 10;
pub const PHY_TXPWR_MIN_NPHY: c_int = 8;

pub const PWRTBL_NUM_COEFF: c_int = 3;
pub const SPURAVOID_DISABLE: c_int = 0;
pub const SPURAVOID_AUTO: c_int = 1;
pub const SPURAVOID_FORCEON: c_int = 2;
pub const SPURAVOID_FORCEON2: c_int = 3;
pub const PHY_SW_TIMER_FAST: c_int = 15;
pub const PHY_SW_TIMER_SLOW: c_int = 60;
pub const PHY_SW_TIMER_GLACIAL: c_int = 120;
pub const PHY_PERICAL_AUTO: c_int = 0;
pub const PHY_PERICAL_FULL: c_int = 1;
pub const PHY_PERICAL_PARTIAL: c_int = 2;
pub const PHY_PERICAL_NODELAY: c_int = 0;
pub const PHY_PERICAL_INIT_DELAY: c_int = 5;
pub const PHY_PERICAL_ASSOC_DELAY: c_int = 5;
pub const PHY_PERICAL_WDOG_DELAY: c_int = 5;
pub const MPHASE_TXCAL_NUMCMDS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_cal_mode {
    CAL_FULL,
    CAL_RECAL,
    CAL_CURRECAL,
    CAL_DIGCAL,
    CAL_GCTRL,
    CAL_SOFT,
    CAL_DIGLO
}

pub const RDR_NTIERS: c_int = 1;
pub const RDR_TIER_SIZE: c_int = 64;

pub const RDR_EPOCH_SIZE: c_int = 40;
pub const RDR_NANTENNAS: c_int = 2;

pub const RDR_LP_BUFFER_SIZE: c_int = 64;
pub const LP_LEN_HIS_SIZE: c_int = 10;
pub const STATIC_NUM_RF: c_int = 32;
pub const STATIC_NUM_BB: c_int = 9;
pub const BB_MULT_MASK: c_uint = 0x0000ffff;
pub const BB_MULT_VALID_MASK: c_uint = 0x80000000;
pub const PHY_CHAIN_TX_DISABLE_TEMP: c_int = 115;
pub const PHY_HYSTERESIS_DELTATEMP: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_table_info {
    pub table: c_uint,
    pub q: c_int,
    pub max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phytbl_info {
    pub tbl_ptr: *const c_void,
    pub tbl_len: u32,
    pub tbl_id: u32,
    pub tbl_offset: u32,
    pub tbl_width: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct interference_info {
    pub curr_home_channel: u8,
    pub crsminpwrthld_40_stored: u16,
    pub crsminpwrthld_20L_stored: u16,
    pub crsminpwrthld_20U_stored: u16,
    pub init_gain_code_core1_stored: u16,
    pub init_gain_code_core2_stored: u16,
    pub init_gain_codeb_core1_stored: u16,
    pub init_gain_codeb_core2_stored: u16,
    pub init_gain_table_stored: [u16; 4],
    pub clip1_hi_gain_code_core1_stored: u16,
    pub clip1_hi_gain_code_core2_stored: u16,
    pub clip1_hi_gain_codeb_core1_stored: u16,
    pub clip1_hi_gain_codeb_core2_stored: u16,
    pub nb_clip_thresh_core1_stored: u16,
    pub nb_clip_thresh_core2_stored: u16,
    pub init_ofdmlna2gainchange_stored: [u16; 4],
    pub init_ccklna2gainchange_stored: [u16; 4],
    pub clip1_lo_gain_code_core1_stored: u16,
    pub clip1_lo_gain_code_core2_stored: u16,
    pub clip1_lo_gain_codeb_core1_stored: u16,
    pub clip1_lo_gain_codeb_core2_stored: u16,
    pub w1_clip_thresh_core1_stored: u16,
    pub w1_clip_thresh_core2_stored: u16,
    pub radio_2056_core1_rssi_gain_stored: u16,
    pub radio_2056_core2_rssi_gain_stored: u16,
    pub energy_drop_timeout_len_stored: u16,
    pub ed_crs40_assertthld0_stored: u16,
    pub ed_crs40_assertthld1_stored: u16,
    pub ed_crs40_deassertthld0_stored: u16,
    pub ed_crs40_deassertthld1_stored: u16,
    pub ed_crs20L_assertthld0_stored: u16,
    pub ed_crs20L_assertthld1_stored: u16,
    pub ed_crs20L_deassertthld0_stored: u16,
    pub ed_crs20L_deassertthld1_stored: u16,
    pub ed_crs20U_assertthld0_stored: u16,
    pub ed_crs20U_assertthld1_stored: u16,
    pub ed_crs20U_deassertthld0_stored: u16,
    pub ed_crs20U_deassertthld1_stored: u16,
    pub badplcp_ma: u16,
    pub badplcp_ma_previous: u16,
    pub badplcp_ma_total: u16,
    pub badplcp_ma_list: [u16; MA_WINDOW_SZ],
    pub badplcp_ma_index: c_int,
    pub pre_badplcp_cnt: i16,
    pub bphy_pre_badplcp_cnt: i16,
    pub init_gain_core1: u16,
    pub init_gain_core2: u16,
    pub init_gainb_core1: u16,
    pub init_gainb_core2: u16,
    pub init_gain_rfseq: [u16; 4],
    pub crsminpwr0: u16,
    pub crsminpwrl0: u16,
    pub crsminpwru0: u16,
    pub crsminpwr_index: i16,
    pub radio_2057_core1_rssi_wb1a_gc_stored: u16,
    pub radio_2057_core2_rssi_wb1a_gc_stored: u16,
    pub radio_2057_core1_rssi_wb1g_gc_stored: u16,
    pub radio_2057_core2_rssi_wb1g_gc_stored: u16,
    pub radio_2057_core1_rssi_wb2_gc_stored: u16,
    pub radio_2057_core2_rssi_wb2_gc_stored: u16,
    pub radio_2057_core1_rssi_nb_gc_stored: u16,
    pub radio_2057_core2_rssi_nb_gc_stored: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aci_save_gphy {
    pub rc_cal_ovr: u16,
    pub phycrsth1: u16,
    pub phycrsth2: u16,
    pub init_n1p1_gain: u16,
    pub p1_p2_gain: u16,
    pub n1_n2_gain: u16,
    pub n1_p1_gain: u16,
    pub div_search_gain: u16,
    pub div_p1_p2_gain: u16,
    pub div_search_gn_change: u16,
    pub table_7_2: u16,
    pub table_7_3: u16,
    pub cckshbits_gnref: u16,
    pub clip_thresh: u16,
    pub clip2_thresh: u16,
    pub clip3_thresh: u16,
    pub clip_p2_thresh: u16,
    pub clip_pwdn_thresh: u16,
    pub clip_n1p1_thresh: u16,
    pub clip_n1_pwdn_thresh: u16,
    pub bbconfig: u16,
    pub cthr_sthr_shdin: u16,
    pub energy: u16,
    pub clip_p1_p2_thresh: u16,
    pub threshold: u16,
    pub reg15: u16,
    pub reg16: u16,
    pub reg17: u16,
    pub div_srch_idx: u16,
    pub div_srch_p1_p2: u16,
    pub div_srch_gn_back: u16,
    pub ant_dwell: u16,
    pub ant_wr_settle: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lo_complex_abgphy_info {
    pub i: i8,
    pub q: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_iq_comp {
    pub a0: i16,
    pub b0: i16,
    pub a1: i16,
    pub b1: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_txpwrindex {
    pub index: i8,
    pub index_internal: i8,
    pub index_internal_save: i8,
    pub AfectrlOverride: u16,
    pub AfeCtrlDacGain: u16,
    pub rad_gain: u16,
    pub bbmult: u8,
    pub iqcomp_a: u16,
    pub iqcomp_b: u16,
    pub locomp: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txiqcal_cache {
    pub txcal_coeffs_2G: [u16; 8],
    pub txcal_radio_regs_2G: [u16; 8],
    pub rxcal_coeffs_2G: nphy_iq_comp,
    pub txcal_coeffs_5G: [u16; 8],
    pub txcal_radio_regs_5G: [u16; 8],
    pub rxcal_coeffs_5G: nphy_iq_comp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_pwrctrl {
    pub max_pwr_2g: i8,
    pub idle_targ_2g: i8,
    pub pwrdet_2g_a1: i16,
    pub pwrdet_2g_b0: i16,
    pub pwrdet_2g_b1: i16,
    pub max_pwr_5gm: i8,
    pub idle_targ_5gm: i8,
    pub max_pwr_5gh: i8,
    pub max_pwr_5gl: i8,
    pub pwrdet_5gm_a1: i16,
    pub pwrdet_5gm_b0: i16,
    pub pwrdet_5gm_b1: i16,
    pub pwrdet_5gl_a1: i16,
    pub pwrdet_5gl_b0: i16,
    pub pwrdet_5gl_b1: i16,
    pub pwrdet_5gh_a1: i16,
    pub pwrdet_5gh_b0: i16,
    pub pwrdet_5gh_b1: i16,
    pub idle_targ_5gl: i8,
    pub idle_targ_5gh: i8,
    pub idle_tssi_2g: i8,
    pub idle_tssi_5g: i8,
    pub idle_tssi: i8,
    pub a1: i16,
    pub b0: i16,
    pub b1: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_txgains {
    pub txlpf: [u16; 2],
    pub txgm: [u16; 2],
    pub pga: [u16; 2],
    pub pad: [u16; 2],
    pub ipa: [u16; 2],
}

pub const PHY_NOISEVAR_BUFSIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nphy_noisevar_buf {
    pub bufcount: c_int,
    pub tone_id: [c_int; PHY_NOISEVAR_BUFSIZE],
    pub noise_vars: [u32; PHY_NOISEVAR_BUFSIZE],
    pub min_noise_vars: [u32; PHY_NOISEVAR_BUFSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rssical_cache {
    pub rssical_radio_regs_2G: [u16; 2],
    pub rssical_phyregs_2G: [u16; 12],
    pub rssical_radio_regs_5G: [u16; 2],
    pub rssical_phyregs_5G: [u16; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcnphy_cal_results {
    pub txiqlocal_a: u16,
    pub txiqlocal_b: u16,
    pub txiqlocal_didq: u16,
    pub txiqlocal_ei0: u8,
    pub txiqlocal_eq0: u8,
    pub txiqlocal_fi0: u8,
    pub txiqlocal_fq0: u8,
    pub txiqlocal_bestcoeffs: [u16; 11],
    pub txiqlocal_bestcoeffs_valid: u16,
    pub papd_eps_tbl: [u32; PHY_PAPD_EPS_TBL_SIZE_LCNPHY],
    pub analog_gain_ref: u16,
    pub lut_begin: u16,
    pub lut_end: u16,
    pub lut_step: u16,
    pub rxcompdbm: u16,
    pub papdctrl: u16,
    pub sslpnCalibClkEnCtrl: u16,
    pub rxiqcal_coeff_a0: u16,
    pub rxiqcal_coeff_b0: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared_phy {
    pub phy_head: *mut brcms_phy,
    pub unit: c_uint,
    pub physhim: *mut phy_shim_info,
    pub corerev: c_uint,
    pub machwcap: u32,
    pub up: bool,
    pub clk: bool,
    pub now: c_uint,
    pub vid: u16,
    pub did: u16,
    pub chip: c_uint,
    pub chiprev: c_uint,
    pub chippkg: c_uint,
    pub sromrev: c_uint,
    pub boardtype: c_uint,
    pub boardrev: c_uint,
    pub boardflags: u32,
    pub boardflags2: u32,
    pub fast_timer: c_uint,
    pub slow_timer: c_uint,
    pub glacial_timer: c_uint,
    pub rx_antdiv: u8,
    pub phy_noise_window: [i8; MA_WINDOW_SZ],
    pub phy_noise_index: c_uint,
    pub hw_phytxchain: u8,
    pub hw_phyrxchain: u8,
    pub phytxchain: u8,
    pub phyrxchain: u8,
    pub rssi_mode: u8,
    pub _rifs_phy: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_phy_pub {
    pub phy_type: c_uint,
    pub phy_rev: c_uint,
    pub phy_corenum: u8,
    pub radioid: u16,
    pub radiorev: u8,
    pub radiover: u8,
    pub coreflags: c_uint,
    pub ana_rev: c_uint,
    pub abgphy_encore: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_func_ptr {
    pub ): *mut *mut void (init)(struct brcms_phy,
    pub ): *mut *mut void (calinit)(struct brcms_phy,
    pub chanspec): *mut *mut *mut void (chanset)(struct brcms_phy , u16,
    pub ): *mut *mut void (txpwrrecalc)(struct brcms_phy,
    pub int): *mut *mut *mut int (longtrn)(struct brcms_phy ,,
    pub ): *mut *mut *mut *mut void (txiqccget)(struct brcms_phy , u16 , u16,
    pub u16): *mut *mut *mut void (txiqccset)(struct brcms_phy , u16,,
    pub ): *mut *mut u16 (txloccget)(struct brcms_phy,
    pub ): *mut *mut *mut *mut *mut *mut void (radioloftget)(struct brcms_phy , u8 , u8 , u8 , u8,
    pub ): *mut *mut void (carrsuppr)(struct brcms_phy,
    pub s32): *mut *mut *mut s32 (rxsigpwr)(struct brcms_phy ,,
    pub ): *mut *mut void (detach)(struct brcms_phy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcms_phy {
    pub pubpi_ro: brcms_phy_pub,
    pub sh: *mut shared_phy,
    pub pi_fptr: phy_func_ptr,
    pub pi_lcnphy: *mut brcms_phy_lcnphy,
    pub u: },
    pub user_txpwr_at_rfport: bool,
    pub d11core: *mut bcma_device,
    pub next: *mut brcms_phy,
    pub pubpi: brcms_phy_pub,
    pub do_initcal: bool,
    pub phytest_on: bool,
    pub ofdm_rateset_war: bool,
    pub bf_preempt_4306: bool,
    pub radio_chanspec: u16,
    pub antsel_type: u8,
    pub bw: u16,
    pub txpwr_percent: u8,
    pub phy_init_por: bool,
    pub init_in_progress: bool,
    pub initialized: bool,
    pub sbtml_gm: bool,
    pub refcnt: c_uint,
    pub watchdog_override: bool,
    pub phynoise_state: u8,
    pub phynoise_now: c_uint,
    pub phynoise_chan_watchdog: c_int,
    pub phynoise_polling: bool,
    pub disable_percal: bool,
    pub measure_hold: u32,
    pub txpa_2g: [i16; PWRTBL_NUM_COEFF],
    pub txpa_2g_low_temp: [i16; PWRTBL_NUM_COEFF],
    pub txpa_2g_high_temp: [i16; PWRTBL_NUM_COEFF],
    pub txpa_5g_low: [i16; PWRTBL_NUM_COEFF],
    pub txpa_5g_mid: [i16; PWRTBL_NUM_COEFF],
    pub txpa_5g_hi: [i16; PWRTBL_NUM_COEFF],
    pub tx_srom_max_2g: u8,
    pub tx_srom_max_5g_low: u8,
    pub tx_srom_max_5g_mid: u8,
    pub tx_srom_max_5g_hi: u8,
    pub tx_srom_max_rate_2g: [u8; TXP_NUM_RATES],
    pub tx_srom_max_rate_5g_low: [u8; TXP_NUM_RATES],
    pub tx_srom_max_rate_5g_mid: [u8; TXP_NUM_RATES],
    pub tx_srom_max_rate_5g_hi: [u8; TXP_NUM_RATES],
    pub tx_user_target: [u8; TXP_NUM_RATES],
    pub tx_power_offset: [i8; TXP_NUM_RATES],
    pub tx_power_target: [u8; TXP_NUM_RATES],
    pub srom_fem2g: brcms_phy_srom_fem,
    pub srom_fem5g: brcms_phy_srom_fem,
    pub tx_power_max: u8,
    pub tx_power_max_rate_ind: u8,
    pub hwpwrctrl: bool,
    pub nphy_txpwrctrl: u8,
    pub nphy_txrx_chain: i8,
    pub phy_5g_pwrgain: bool,
    pub phy_wreg: u16,
    pub phy_wreg_limit: u16,
    pub n_preamble_override: i8,
    pub antswitch: u8,
    pub aa5g: u8 aa2g,,
    pub idle_tssi: [i8; CH_5G_GROUP],
    pub target_idle_tssi: i8,
    pub txpwr_est_Pout: i8,
    pub tx_power_min: u8,
    pub txpwr_limit: [u8; TXP_NUM_RATES],
    pub txpwr_env_limit: [u8; TXP_NUM_RATES],
    pub adj_pwr_tbl_nphy: [u8; ADJ_PWR_TBL_LEN],
    pub channel_14_wide_filter: bool,
    pub txpwroverride: bool,
    pub txpwridx_override_aphy: bool,
    pub radiopwr_override: i16,
    pub hwpwr_txcur: u16,
    pub saved_txpwr_idx: u8,
    pub edcrs_threshold_lock: bool,
    pub tr_R_gain_val: u32,
    pub tr_T_gain_val: u32,
    pub ofdm_analog_filt_bw_override: i16,
    pub cck_analog_filt_bw_override: i16,
    pub ofdm_rccal_override: i16,
    pub cck_rccal_override: i16,
    pub extlna_type: u16,
    pub interference_mode_crs_time: c_uint,
    pub crsglitch_prev: u16,
    pub interference_mode_crs: bool,
    pub phy_tx_tone_freq: u32,
    pub phy_lastcal: c_uint,
    pub phy_forcecal: bool,
    pub phy_fixed_noise: bool,
    pub xtalfreq: u32,
    pub pdiv: u8,
    pub carrier_suppr_disable: i8,
    pub phy_bphy_evm: bool,
    pub phy_bphy_rfcs: bool,
    pub phy_scraminit: i8,
    pub phy_gpiosel: u8,
    pub phy_txcore_disable_temp: i16,
    pub phy_txcore_enable_temp: i16,
    pub phy_tempsense_offset: i8,
    pub phy_txcore_heatedup: bool,
    pub radiopwr: u16,
    pub bb_atten: u16,
    pub txctl1: u16,
    pub mintxbias: u16,
    pub mintxmag: u16,
    pub stats_11b_txpower: [i8; STATIC_NUM_RF][STATIC_NUM_BB],
    pub gain_table: [u16; TX_GAIN_TABLE_LENGTH],
    pub loopback_gain: bool,
    pub max_lpback_gain_hdB: i16,
    pub trsw_rx_gain_hdB: i16,
    pub power_vec: [u8; 8],
    pub rc_cal: u16,
    pub nrssi_table_delta: c_int,
    pub nrssi_slope_scale: c_int,
    pub nrssi_slope_offset: c_int,
    pub min_rssi: c_int,
    pub max_rssi: c_int,
    pub txpwridx: i8,
    pub min_txpower: u8,
    pub a_band_high_disable: u8,
    pub tx_vos: u16,
    pub global_tx_bb_dc_bias_loft: u16,
    pub rf_max: c_int,
    pub bb_max: c_int,
    pub rf_list_size: c_int,
    pub bb_list_size: c_int,
    pub rf_attn_list: *mut u16,
    pub bb_attn_list: *mut u16,
    pub padmix_mask: u16,
    pub padmix_reg: u16,
    pub txmag_list: *mut u16,
    pub txmag_len: c_uint,
    pub txmag_enable: bool,
    pub a_tssi_to_dbm: *mut i8,
    pub m_tssi_to_dbm: *mut i8,
    pub l_tssi_to_dbm: *mut i8,
    pub h_tssi_to_dbm: *mut i8,
    pub hwtxpwr: *mut u8,
    pub freqtrack_saved_regs: [u16; 2],
    pub cur_interference_mode: c_int,
    pub hwpwrctrl_capable: bool,
    pub temppwrctrl_capable: bool,
    pub phycal_nslope: c_uint,
    pub phycal_noffset: c_uint,
    pub phycal_mlo: c_uint,
    pub phycal_txpower: c_uint,
    pub phy_aa2g: u8,
    pub nphy_tableloaded: bool,
    pub nphy_rssisel: i8,
    pub nphy_bb_mult_save: u32,
    pub nphy_txiqlocal_bestc: [u16; 11],
    pub nphy_txiqlocal_coeffsvalid: bool,
    pub nphy_txpwrindex: [nphy_txpwrindex; PHY_CORE_NUM_2],
    pub nphy_pwrctrl_info: [nphy_pwrctrl; PHY_CORE_NUM_2],
    pub cck2gpo: u16,
    pub ofdm2gpo: u32,
    pub ofdm5gpo: u32,
    pub ofdm5glpo: u32,
    pub ofdm5ghpo: u32,
    pub bw402gpo: u8,
    pub bw405gpo: u8,
    pub bw405glpo: u8,
    pub bw405ghpo: u8,
    pub cdd2gpo: u8,
    pub cdd5gpo: u8,
    pub cdd5glpo: u8,
    pub cdd5ghpo: u8,
    pub stbc2gpo: u8,
    pub stbc5gpo: u8,
    pub stbc5glpo: u8,
    pub stbc5ghpo: u8,
    pub bwdup2gpo: u8,
    pub bwdup5gpo: u8,
    pub bwdup5glpo: u8,
    pub bwdup5ghpo: u8,
    pub mcs2gpo: [u16; 8],
    pub mcs5gpo: [u16; 8],
    pub mcs5glpo: [u16; 8],
    pub mcs5ghpo: [u16; 8],
    pub nphy_rxcalparams: u32,
    pub phy_spuravoid: u8,
    pub phy_isspuravoid: bool,
    pub phy_pabias: u8,
    pub nphy_papd_skip: u8,
    pub nphy_tssi_slope: u8,
    pub nphy_noise_win: [i16; PHY_CORE_MAX][PHY_NOISE_WINDOW_SZ],
    pub nphy_noise_index: u8,
    pub nphy_gain_boost: bool,
    pub nphy_elna_gain_config: bool,
    pub old_bphy_test: u16,
    pub old_bphy_testcontrol: u16,
    pub phyhang_avoid: bool,
    pub rssical_nphy: bool,
    pub nphy_perical: u8,
    pub nphy_perical_last: c_uint,
    pub cal_type_override: u8,
    pub mphase_cal_phase_id: u8,
    pub mphase_txcal_cmdidx: u8,
    pub mphase_txcal_numcmds: u8,
    pub mphase_txcal_bestcoeffs: [u16; 11],
    pub nphy_txiqlocal_chanspec: u16,
    pub nphy_iqcal_chanspec_2G: u16,
    pub nphy_iqcal_chanspec_5G: u16,
    pub nphy_rssical_chanspec_2G: u16,
    pub nphy_rssical_chanspec_5G: u16,
    pub phycal_timer: *mut wlapi_timer,
    pub use_int_tx_iqlo_cal_nphy: bool,
    pub internal_tx_iqlo_cal_tapoff_intpa_nphy: bool,
    pub nphy_lastcal_temp: i16,
    pub calibration_cache: txiqcal_cache,
    pub rssical_cache: rssical_cache,
    pub nphy_txpwr_idx: [u8; 2],
    pub nphy_papd_cal_type: u8,
    pub nphy_papd_last_cal: c_uint,
    pub nphy_papd_tx_gain_at_last_cal: [u16; 2],
    pub nphy_papd_cal_gain_index: [u8; 2],
    pub nphy_papd_epsilon_offset: [i16; 2],
    pub nphy_papd_recal_enable: bool,
    pub nphy_papd_recal_counter: u32,
    pub nphy_force_papd_cal: bool,
    pub nphy_papdcomp: bool,
    pub ipa2g_on: bool,
    pub ipa5g_on: bool,
    pub classifier_state: u16,
    pub clip_state: [u16; 2],
    pub nphy_deaf_count: c_uint,
    pub rxiq_samps: u8,
    pub rxiq_antsel: u8,
    pub rfctrlIntc1_save: u16,
    pub rfctrlIntc2_save: u16,
    pub first_cal_after_assoc: bool,
    pub tx_rx_cal_radio_saveregs: [u16; 22],
    pub tx_rx_cal_phy_saveregs: [u16; 15],
    pub nphy_cal_orig_pwr_idx: [u8; 2],
    pub nphy_txcal_pwr_idx: [u8; 2],
    pub nphy_rxcal_pwr_idx: [u8; 2],
    pub nphy_cal_orig_tx_gain: [u16; 2],
    pub nphy_cal_target_gain: nphy_txgains,
    pub nphy_txcal_bbmult: u16,
    pub nphy_gmval: u16,
    pub nphy_saved_bbconf: u16,
    pub nphy_gband_spurwar_en: bool,
    pub nphy_gband_spurwar2_en: bool,
    pub nphy_aband_spurwar_en: bool,
    pub nphy_rccal_value: u16,
    pub nphy_crsminpwr: [u16; 3],
    pub nphy_saved_noisevars: nphy_noisevar_buf,
    pub nphy_anarxlpf_adjusted: bool,
    pub nphy_crsminpwr_adjusted: bool,
    pub nphy_noisevars_adjusted: bool,
    pub nphy_rxcal_active: bool,
    pub radar_percal_mask: u16,
    pub dfs_lp_buffer_nphy: bool,
    pub nphy_fineclockgatecontrol: u16,
    pub rx2tx_biasentry: i8,
    pub crsminpwr0: u16,
    pub crsminpwrl0: u16,
    pub crsminpwru0: u16,
    pub noise_crsminpwr_index: i16,
    pub init_gain_core1: u16,
    pub init_gain_core2: u16,
    pub init_gainb_core1: u16,
    pub init_gainb_core2: u16,
    pub aci_noise_curr_channel: u8,
    pub init_gain_rfseq: [u16; 4],
    pub radio_is_on: bool,
    pub nphy_sample_play_lpf_bw_ctl_ovr: bool,
    pub tbl_data_hi: u16,
    pub tbl_data_lo: u16,
    pub tbl_addr: u16,
    pub tbl_save_id: c_uint,
    pub tbl_save_offset: c_uint,
    pub txpwrctrl: u8,
    pub txpwrindex: [i8; PHY_CORE_MAX],
    pub phycal_tempdelta: u8,
    pub mcs20_po: u32,
    pub mcs40_po: u32,
    pub wiphy: *mut wiphy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs32 {
    pub q: i32,
    pub i: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radio_regs {
    pub address: u16,
    pub init_a: u32,
    pub init_g: u32,
    pub do_init_a: u8,
    pub do_init_g: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radio_20xx_regs {
    pub address: u16,
    pub init: u8,
    pub do_init: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcnphy_radio_regs {
    pub address: u16,
    pub init_a: u8,
    pub init_g: u8,
    pub do_init_a: u8,
    pub do_init_g: u8,
}

extern "C" {
    pub fn read_phy_reg(pi: *mut brcms_phy, addr: u16) -> u16;
}
extern "C" {
    pub fn write_phy_reg(pi: *mut brcms_phy, addr: u16, val: u16);
}
extern "C" {
    pub fn and_phy_reg(pi: *mut brcms_phy, addr: u16, val: u16);
}
extern "C" {
    pub fn or_phy_reg(pi: *mut brcms_phy, addr: u16, val: u16);
}
extern "C" {
    pub fn mod_phy_reg(pi: *mut brcms_phy, addr: u16, mask: u16, val: u16);
}
extern "C" {
    pub fn read_radio_reg(pi: *mut brcms_phy, addr: u16) -> u16;
}
extern "C" {
    pub fn or_radio_reg(pi: *mut brcms_phy, addr: u16, val: u16);
}
extern "C" {
    pub fn and_radio_reg(pi: *mut brcms_phy, addr: u16, val: u16);
}
extern "C" {
    pub fn mod_radio_reg(pi: *mut brcms_phy, addr: u16, mask: u16, val: u16);
}
extern "C" {
    pub fn xor_radio_reg(pi: *mut brcms_phy, addr: u16, mask: u16);
}
extern "C" {
    pub fn write_radio_reg(pi: *mut brcms_phy, addr: u16, val: u16);
}
extern "C" {
    pub fn wlc_phyreg_enter(pih: *mut brcms_phy_pub);
}
extern "C" {
    pub fn wlc_phyreg_exit(pih: *mut brcms_phy_pub);
}
extern "C" {
    pub fn wlc_phy_table_data_write(pi: *mut brcms_phy, width: c_uint, val: u32);
}
extern "C" {
    pub fn wlc_phy_txpower_update_shm(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_nbits(value: i32) -> u8;
}
extern "C" {
    pub fn wlc_phy_compute_dB(cmplx_pwr: *mut u32, p_dB: *mut i8, core: u8);
}
extern "C" {
    pub fn wlc_phy_txpower_ipa_upd(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_do_dummy_tx(pi: *mut brcms_phy, ofdm: bool, pa_on: bool);
}
extern "C" {
    pub fn wlc_phy_papd_decode_epsilon(epsilon: u32, eps_real: *mut i32, eps_imag: *mut i32);
}
extern "C" {
    pub fn wlc_phy_cal_perical_mphase_reset(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_cal_perical_mphase_restart(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_attach_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_attach_lcnphy(pi: *mut brcms_phy) -> bool;
}
extern "C" {
    pub fn wlc_phy_detach_lcnphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_init_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_init_lcnphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_cal_init_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_cal_init_lcnphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_chanspec_set_nphy(pi: *mut brcms_phy, chanspec: u16);
}
extern "C" {
    pub fn wlc_phy_chanspec_set_lcnphy(pi: *mut brcms_phy, chanspec: u16);
}
extern "C" {
    pub fn wlc_phy_chanspec_set_fixup_lcnphy(pi: *mut brcms_phy, chanspec: u16);
}
extern "C" {
    pub fn wlc_phy_channel2freq(channel: c_uint) -> c_int;
}
extern "C" {
    pub fn wlc_phy_chanspec_freq2bandrange_lpssn(_arg: c_uint) -> c_int;
}
extern "C" {
    pub fn wlc_phy_chanspec_bandrange_get(: *mut brcms_phy, chanspec: u16) -> c_int;
}
extern "C" {
    pub fn wlc_lcnphy_set_tx_pwr_ctrl(pi: *mut brcms_phy, mode: u16);
}
extern "C" {
    pub fn wlc_lcnphy_get_current_tx_pwr_idx(pi: *mut brcms_phy) -> i8;
}
extern "C" {
    pub fn wlc_phy_txpower_recalc_target_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_lcnphy_txpower_recalc_target(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_txpower_recalc_target_lcnphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_lcnphy_set_tx_pwr_by_index(pi: *mut brcms_phy, index: c_int);
}
extern "C" {
    pub fn wlc_lcnphy_tx_pu(pi: *mut brcms_phy, bEnable: bool);
}
extern "C" {
    pub fn wlc_lcnphy_stop_tx_tone(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_lcnphy_tempsense(pi: *mut brcms_phy, mode: bool) -> u16;
}
extern "C" {
    pub fn wlc_lcnphy_tempsense_new(pi: *mut brcms_phy, mode: bool) -> i16;
}
extern "C" {
    pub fn wlc_lcnphy_tempsense_degree(pi: *mut brcms_phy, mode: bool) -> i8;
}
extern "C" {
    pub fn wlc_lcnphy_vbatsense(pi: *mut brcms_phy, mode: bool) -> i8;
}
extern "C" {
    pub fn wlc_phy_carrier_suppress_lcnphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_lcnphy_crsuprs(pi: *mut brcms_phy, channel: c_int);
}
extern "C" {
    pub fn wlc_2064_vco_cal(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_txpower_recalc_target(pi: *mut brcms_phy);
}
pub const LCNPHY_TBL_ID_PAPDCOMPDELTATBL: c_uint = 0x18;
pub const LCNPHY_TX_POWER_TABLE_SIZE: c_int = 128;

pub const LCNPHY_TBL_ID_TXPWRCTL: c_uint = 0x07;
pub const LCNPHY_TX_PWR_CTRL_OFF: c_int = 0;

pub const LCNPHY_TX_PWR_CTRL_TEMPBASED: c_uint = 0xE001;
extern "C" {
    pub fn wlc_lcnphy_read_table(pi: *mut brcms_phy, pti: *mut phytbl_info);
}
extern "C" {
    pub fn wlc_lcnphy_set_tx_iqcc(pi: *mut brcms_phy, a: u16, b: u16);
}
extern "C" {
    pub fn wlc_lcnphy_set_tx_locc(pi: *mut brcms_phy, didq: u16);
}
extern "C" {
    pub fn wlc_lcnphy_get_tx_iqcc(pi: *mut brcms_phy, a: *mut u16, b: *mut u16);
}
extern "C" {
    pub fn wlc_lcnphy_get_tx_locc(pi: *mut brcms_phy) -> u16;
}
extern "C" {
    pub fn wlc_lcnphy_calib_modes(pi: *mut brcms_phy, mode: c_uint);
}
extern "C" {
    pub fn wlc_lcnphy_deaf_mode(pi: *mut brcms_phy, mode: bool);
}
extern "C" {
    pub fn wlc_phy_tpc_isenabled_lcnphy(pi: *mut brcms_phy) -> bool;
}
extern "C" {
    pub fn wlc_lcnphy_tx_pwr_update_npt(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_lcnphy_tssi2dbm(tssi: i32, a1: i32, b0: i32, b1: i32) -> i32;
}
extern "C" {
    pub fn wlc_lcnphy_get_tssi(pi: *mut brcms_phy, ofdm_pwr: *mut i8, cck_pwr: *mut i8);
}
extern "C" {
    pub fn wlc_lcnphy_tx_power_adjustment(ppi: *mut brcms_phy_pub);
}
extern "C" {
    pub fn wlc_lcnphy_rx_signal_power(pi: *mut brcms_phy, gain_index: i32) -> i32;
}
pub const NPHY_MAX_HPVGA1_INDEX: c_int = 10;
pub const NPHY_DEF_HPVGA1_INDEXLIMIT: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_iq_est {
    pub iq_prod: i32,
    pub i_pwr: u32,
    pub q_pwr: u32,
}

extern "C" {
    pub fn wlc_phy_stay_in_carriersearch_nphy(pi: *mut brcms_phy, enable: bool);
}

extern "C" {
    pub fn wlc_phy_cal_perical_nphy_run(pi: *mut brcms_phy, caltype: u8);
}
extern "C" {
    pub fn wlc_phy_aci_reset_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_pa_override_nphy(pi: *mut brcms_phy, en: bool);
}
extern "C" {
    pub fn wlc_phy_get_chan_freq_range_nphy(pi: *mut brcms_phy, chan: c_uint) -> u8;
}
extern "C" {
    pub fn wlc_phy_switch_radio_nphy(pi: *mut brcms_phy, on: bool);
}
extern "C" {
    pub fn wlc_phy_stf_chain_upd_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_force_rfseq_nphy(pi: *mut brcms_phy, cmd: u8);
}
extern "C" {
    pub fn wlc_phy_tempsense_nphy(pi: *mut brcms_phy) -> i16;
}
extern "C" {
    pub fn wlc_phy_classifier_nphy(pi: *mut brcms_phy, mask: u16, val: u16) -> u16;
}
extern "C" {
    pub fn wlc_phy_aci_and_noise_reduction_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_rxcore_setstate_nphy(pih: *mut brcms_phy_pub, rxcore_bitmask: u8);
}
extern "C" {
    pub fn wlc_phy_rxcore_getstate_nphy(pih: *mut brcms_phy_pub) -> u8;
}
extern "C" {
    pub fn wlc_phy_txpwrctrl_enable_nphy(pi: *mut brcms_phy, ctrl_type: u8);
}
extern "C" {
    pub fn wlc_phy_txpwr_fixpower_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_txpwr_apply_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_txpwr_papd_cal_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_txpwr_idx_get_nphy(pi: *mut brcms_phy) -> u16;
}
extern "C" {
    pub fn wlc_phy_get_tx_gain_nphy(pi: *mut brcms_phy) -> nphy_txgains;
}
extern "C" {
    pub fn wlc_phy_rssisel_nphy(pi: *mut brcms_phy, core: u8, rssi_type: u8);
}
extern "C" {
    pub fn wlc_phy_rssi_cal_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_aci_scan_nphy(pi: *mut brcms_phy) -> c_int;
}
extern "C" {
    pub fn wlc_phy_stopplayback_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_radio205x_vcocal_nphy(pi: *mut brcms_phy);
}
extern "C" {
    pub fn wlc_phy_rssi_compute_nphy(pi: *mut brcms_phy, rxh: *mut d11rxhdr) -> c_int;
}
pub const NPHY_TESTPATTERN_BPHY_EVM: c_int = 0;
pub const NPHY_TESTPATTERN_BPHY_RFCS: c_int = 1;
extern "C" {
    pub fn wlc_phy_nphy_tkip_rifs_war(pi: *mut brcms_phy, rifs: u8);
}
