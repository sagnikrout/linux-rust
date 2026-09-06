//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/phy.h
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

pub const RTW89_BBMCU_ADDR_OFFSET: c_uint = 0x30000;

pub const PHY_HEADLINE_VALID: c_uint = 0xf;

pub const PHY_COND_BRANCH_IF: c_uint = 0x8;
pub const PHY_COND_BRANCH_ELIF: c_uint = 0x9;
pub const PHY_COND_BRANCH_ELSE: c_uint = 0xa;
pub const PHY_COND_BRANCH_END: c_uint = 0xb;
pub const PHY_COND_CHECK: c_uint = 0x4;
pub const PHY_COND_DONT_CARE: c_uint = 0xff;

pub const RA_MASK_SUBCCK_RATES: c_uint = 0x5ULL;
pub const RA_MASK_SUBOFDM_RATES: c_uint = 0x10ULL;

pub const CFO_PERIOD_CNT: c_int = 15;
pub const CFO_BOUND: c_int = 64;
pub const CFO_TP_UPPER: c_int = 100;
pub const CFO_TP_LOWER: c_int = 50;
pub const CFO_COMP_PERIOD: c_int = 250;
pub const CFO_COMP_WEIGHT: c_int = 8;
pub const MAX_CFO_TOLERANCE: c_int = 30;
pub const CFO_TF_CNT_TH: c_int = 300;
pub const UL_TB_TF_CNT_L2H_TH: c_int = 100;
pub const UL_TB_TF_CNT_H2L_TH: c_int = 70;
pub const ANTDIV_TRAINNING_CNT: c_int = 2;
pub const ANTDIV_TRAINNING_INTVL: c_int = 30;
pub const ANTDIV_DELAY: c_int = 110;
pub const ANTDIV_TP_DIFF_TH_HIGH: c_int = 100;
pub const ANTDIV_TP_DIFF_TH_LOW: c_int = 5;
pub const ANTDIV_EVM_DIFF_TH: c_int = 8;
pub const ANTDIV_RSSI_DIFF_TH: c_int = 3;
pub const CCX_MAX_PERIOD: c_int = 2097;
pub const CCX_MAX_PERIOD_UNIT: c_int = 32;
pub const MS_TO_4US_RATIO: c_int = 250;
pub const ENV_MNTR_FAIL_DWORD: c_uint = 0xffffffff;
pub const ENV_MNTR_IFSCLM_HIS_MAX: c_int = 127;
pub const PERMIL: c_int = 1000;
pub const PERCENT: c_int = 100;
pub const IFS_CLM_TH0_UPPER: c_int = 64;
pub const IFS_CLM_TH_MUL: c_int = 4;
pub const IFS_CLM_TH_START_IDX: c_int = 0;
pub const TIA0_GAIN_A: c_int = 12;
pub const TIA0_GAIN_G: c_int = 16;

pub const U4_MAX_BIT: c_int = 3;
pub const U8_MAX_BIT: c_int = 7;
pub const DIG_GAIN_SHIFT: c_int = 2;
pub const DIG_GAIN: c_int = 8;
pub const LNA_IDX_MAX: c_int = 6;
pub const LNA_IDX_MIN: c_int = 0;
pub const TIA_IDX_MAX: c_int = 1;
pub const TIA_IDX_MIN: c_int = 0;
pub const RXB_IDX_MAX: c_int = 31;
pub const RXB_IDX_MIN: c_int = 0;
pub const IGI_RSSI_MAX: c_int = 110;
pub const PD_TH_MAX_RSSI: c_int = 70;
pub const PD_TH_MIN_RSSI: c_int = 8;

pub const PD_TH_BW160_CMP_VAL: c_int = 9;
pub const PD_TH_BW80_CMP_VAL: c_int = 6;
pub const PD_TH_BW40_CMP_VAL: c_int = 3;
pub const PD_TH_BW20_CMP_VAL: c_int = 0;
pub const PD_TH_CMP_VAL: c_int = 3;
pub const PD_TH_SB_FLTR_CMP_VAL: c_int = 7;

pub const EDCCA_MAX: c_int = 249;
pub const EDCCA_TH_L2H_LB: c_int = 66;
pub const EDCCA_TH_REF: c_int = 3;
pub const EDCCA_HL_DIFF_NORMAL: c_int = 8;
pub const RSSI_UNIT_CONVER: c_int = 110;
pub const EDCCA_UNIT_CONVER: c_int = 128;
pub const EDCCA_PWROFST_DEFAULT: c_int = 18;
pub const VAR_LEN: c_uint = 0xff;
pub const VAR_LEN_UNIT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_c2h_ra_func {
    RTW89_PHY_C2H_FUNC_STS_RPT,
    RTW89_PHY_C2H_FUNC_MU_GPTBL_RPT,
    RTW89_PHY_C2H_FUNC_TXSTS,
    RTW89_PHY_C2H_FUNC_TX_HISTORY = 0x4,
    RTW89_PHY_C2H_FUNC_ACCELERATE_EN = 0x7,

    RTW89_PHY_C2H_FUNC_RA_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_c2h_rfk_log_func {
    RTW89_PHY_C2H_RFK_LOG_FUNC_IQK = 0,
    RTW89_PHY_C2H_RFK_LOG_FUNC_DPK = 1,
    RTW89_PHY_C2H_RFK_LOG_FUNC_DACK = 2,
    RTW89_PHY_C2H_RFK_LOG_FUNC_RXDCK = 3,
    RTW89_PHY_C2H_RFK_LOG_FUNC_TSSI = 4,
    RTW89_PHY_C2H_RFK_LOG_FUNC_TXGAPK = 5,
    RTW89_PHY_C2H_RFK_LOG_FUNC_TAS_PWR = 9,
    RTW89_PHY_C2H_RFK_LOG_FUNC_TXIQK = 0xc,
    RTW89_PHY_C2H_RFK_LOG_FUNC_CIM3K = 0xe,

    RTW89_PHY_C2H_RFK_LOG_FUNC_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_c2h_rfk_report_func {
    RTW89_PHY_C2H_RFK_REPORT_FUNC_STATE = 0,
    RTW89_PHY_C2H_RFK_REPORT_FUNC_TAS_PWR = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_c2h_dm_func {
    RTW89_PHY_C2H_DM_FUNC_FW_TEST,
    RTW89_PHY_C2H_DM_FUNC_FW_TRIG_TX_RPT,
    RTW89_PHY_C2H_DM_FUNC_SIGB,
    RTW89_PHY_C2H_DM_FUNC_LOWRT_RTY,
    RTW89_PHY_C2H_DM_FUNC_MCC_DIG,
    RTW89_PHY_C2H_DM_FUNC_LPS = 0x9,
    RTW89_PHY_C2H_DM_FUNC_ENV_MNTR = 0xa,
    RTW89_PHY_C2H_DM_FUNC_FW_SCAN = 0xc,
    RTW89_PHY_C2H_DM_FUNC_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_c2h_class {
    RTW89_PHY_C2H_CLASS_RUA,
    RTW89_PHY_C2H_CLASS_RA,
    RTW89_PHY_C2H_CLASS_DM,
    RTW89_PHY_C2H_RFK_LOG = 0x8,
    RTW89_PHY_C2H_RFK_REPORT = 0x9,
    RTW89_PHY_C2H_CLASS_BTC_MIN = 0x10,
    RTW89_PHY_C2H_CLASS_BTC_MAX = 0x17,
    RTW89_PHY_C2H_CLASS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_env_monitor_result_level {
    RTW89_PHY_ENV_MON_CCX_FAIL = 0,
    RTW89_PHY_ENV_MON_NHM = BIT(0),
    RTW89_PHY_ENV_MON_CLM = BIT(1),
    RTW89_PHY_ENV_MON_FAHM = BIT(2),
    RTW89_PHY_ENV_MON_IFS_CLM = BIT(3),
    RTW89_PHY_ENV_MON_EDCCA_CLM = BIT(4),
}

pub const RTW89_NHM_WEIGHT_OFFSET: c_int = 2;

pub const RTW89_NHM_MNTR_TIME: c_int = 40;
pub const RTW89_NHM_TH_FACTOR: c_int = 1;
pub const CCX_US_BASE_RATIO: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_ccx_unit {
    RTW89_CCX_4_US = 0,
    RTW89_CCX_8_US = 1,
    RTW89_CCX_16_US = 2,
    RTW89_CCX_32_US = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_status_ie_type {
    RTW89_PHYSTS_IE00_CMN_CCK			= 0,
    RTW89_PHYSTS_IE01_CMN_OFDM			= 1,
    RTW89_PHYSTS_IE02_CMN_EXT_AX			= 2,
    RTW89_PHYSTS_IE03_CMN_EXT_SEG_1			= 3,
    RTW89_PHYSTS_IE04_CMN_EXT_PATH_A		= 4,
    RTW89_PHYSTS_IE05_CMN_EXT_PATH_B		= 5,
    RTW89_PHYSTS_IE06_CMN_EXT_PATH_C		= 6,
    RTW89_PHYSTS_IE07_CMN_EXT_PATH_D		= 7,
    RTW89_PHYSTS_IE08_FTR_CH			= 8,
    RTW89_PHYSTS_IE09_FTR_0				= 9,
    RTW89_PHYSTS_IE10_FTR_PLCP_EXT			= 10,
    RTW89_PHYSTS_IE11_FTR_PLCP_HISTOGRAM		= 11,
    RTW89_PHYSTS_IE12_MU_EIGEN_INFO			= 12,
    RTW89_PHYSTS_IE13_DL_MU_DEF			= 13,
    RTW89_PHYSTS_IE14_TB_UL_CQI			= 14,
    RTW89_PHYSTS_IE15_TB_UL_DEF			= 15,
    RTW89_PHYSTS_IE16_RSVD16			= 16,
    RTW89_PHYSTS_IE17_TB_UL_CTRL			= 17,
    RTW89_PHYSTS_IE18_DBG_OFDM_FD_CMN		= 18,
    RTW89_PHYSTS_IE19_DBG_OFDM_TD_CMN		= 19,
    RTW89_PHYSTS_IE20_DBG_OFDM_FD_USER_SEG_0	= 20,
    RTW89_PHYSTS_IE21_DBG_OFDM_FD_USER_SEG_1	= 21,
    RTW89_PHYSTS_IE22_DBG_OFDM_FD_USER_AGC		= 22,
    RTW89_PHYSTS_IE23_RSVD23			= 23,
    RTW89_PHYSTS_IE24_OFDM_TD_PATH_A		= 24,
    RTW89_PHYSTS_IE25_OFDM_TD_PATH_B		= 25,
    RTW89_PHYSTS_IE26_OFDM_TD_PATH_C		= 26,
    RTW89_PHYSTS_IE27_OFDM_TD_PATH_D		= 27,
    RTW89_PHYSTS_IE28_DBG_CCK_PATH_A		= 28,
    RTW89_PHYSTS_IE29_DBG_CCK_PATH_B		= 29,
    RTW89_PHYSTS_IE30_DBG_CCK_PATH_C		= 30,
    RTW89_PHYSTS_IE31_DBG_CCK_PATH_D		= 31,

// keep last
    RTW89_PHYSTS_IE_NUM,
    RTW89_PHYSTS_IE_MAX = RTW89_PHYSTS_IE_NUM - 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_phy_status_bitmap {
    RTW89_TD_SEARCH_FAIL  = 0,
    RTW89_BRK_BY_TX_PKT   = 1,
    RTW89_CCA_SPOOF       = 2,
    RTW89_OFDM_BRK        = 3,
    RTW89_CCK_BRK         = 4,
    RTW89_DL_MU_SPOOFING  = 5,
    RTW89_HE_MU           = 6,
    RTW89_VHT_MU          = 7,
    RTW89_UL_TB_SPOOFING  = 8,
    RTW89_RSVD_9          = 9,
    RTW89_TRIG_BASE_PPDU  = 10,
    RTW89_CCK_PKT         = 11,
    RTW89_LEGACY_OFDM_PKT = 12,
    RTW89_HT_PKT          = 13,
    RTW89_VHT_PKT         = 14,
    RTW89_HE_PKT          = 15,
    RTW89_EHT_PKT         = 16,

    RTW89_PHYSTS_BITMAP_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_dig_gain_type {
    RTW89_DIG_GAIN_LNA_G = 0,
    RTW89_DIG_GAIN_TIA_G = 1,
    RTW89_DIG_GAIN_LNA_A = 2,
    RTW89_DIG_GAIN_TIA_A = 3,
    RTW89_DIG_GAIN_MAX = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_dig_gain_lna_idx {
    RTW89_DIG_GAIN_LNA_IDX1 = 1,
    RTW89_DIG_GAIN_LNA_IDX2 = 2,
    RTW89_DIG_GAIN_LNA_IDX3 = 3,
    RTW89_DIG_GAIN_LNA_IDX4 = 4,
    RTW89_DIG_GAIN_LNA_IDX5 = 5,
    RTW89_DIG_GAIN_LNA_IDX6 = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_dig_gain_tia_idx {
    RTW89_DIG_GAIN_TIA_IDX0 = 0,
    RTW89_DIG_GAIN_TIA_IDX1 = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tssi_bandedge_cfg {
    RTW89_TSSI_BANDEDGE_FLAT,
    RTW89_TSSI_BANDEDGE_LOW,
    RTW89_TSSI_BANDEDGE_MID,
    RTW89_TSSI_BANDEDGE_HIGH,

    RTW89_TSSI_CFG_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tssi_sbw_idx {
    RTW89_TSSI_SBW20,
    RTW89_TSSI_SBW40_0,
    RTW89_TSSI_SBW40_1,
    RTW89_TSSI_SBW80_0,
    RTW89_TSSI_SBW80_1,
    RTW89_TSSI_SBW80_2,
    RTW89_TSSI_SBW80_3,
    RTW89_TSSI_SBW160_0,
    RTW89_TSSI_SBW160_1,
    RTW89_TSSI_SBW160_2,
    RTW89_TSSI_SBW160_3,
    RTW89_TSSI_SBW160_4,
    RTW89_TSSI_SBW160_5,
    RTW89_TSSI_SBW160_6,
    RTW89_TSSI_SBW160_7,

    RTW89_TSSI_SBW_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_byrate_cfg {
    pub band: rtw89_band,
    pub nss: rtw89_nss,
    pub rs: rtw89_rate_section,
    pub shf: u8,
    pub len: u8,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_dig_gain_cfg {
    pub table: *const rtw89_reg_def,
    pub size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_dig_gain_table {
    pub cfg_lna_g: *const rtw89_phy_dig_gain_cfg,
    pub cfg_tia_g: *const rtw89_phy_dig_gain_cfg,
    pub cfg_lna_a: *const rtw89_phy_dig_gain_cfg,
    pub cfg_tia_a: *const rtw89_phy_dig_gain_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_tssi_dbw_table {
    pub data: [u32; RTW89_TSSI_CFG_NUM][RTW89_TSSI_SBW_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_reg3_tbl {
    pub reg3: *const rtw89_reg3_def,
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_nbi_reg_def {
    pub notch1_idx: rtw89_reg_def,
    pub notch1_frac_idx: rtw89_reg_def,
    pub notch1_en: rtw89_reg_def,
    pub notch2_idx: rtw89_reg_def,
    pub notch2_frac_idx: rtw89_reg_def,
    pub notch2_en: rtw89_reg_def,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_ccx_regs {
    pub setting_addr: u32,
    pub edcca_opt_mask: u32,
    pub measurement_trig_mask: u32,
    pub trig_opt_mask: u32,
    pub en_mask: u32,
    pub ifs_cnt_addr: u32,
    pub ifs_clm_period_mask: u32,
    pub ifs_clm_cnt_unit_mask: u32,
    pub ifs_clm_cnt_clear_mask: u32,
    pub ifs_collect_en_mask: u32,
    pub ifs_t1_addr: u32,
    pub ifs_t1_th_h_mask: u32,
    pub ifs_t1_en_mask: u32,
    pub ifs_t1_th_l_mask: u32,
    pub ifs_t2_addr: u32,
    pub ifs_t2_th_h_mask: u32,
    pub ifs_t2_en_mask: u32,
    pub ifs_t2_th_l_mask: u32,
    pub ifs_t3_addr: u32,
    pub ifs_t3_th_h_mask: u32,
    pub ifs_t3_en_mask: u32,
    pub ifs_t3_th_l_mask: u32,
    pub ifs_t4_addr: u32,
    pub ifs_t4_th_h_mask: u32,
    pub ifs_t4_en_mask: u32,
    pub ifs_t4_th_l_mask: u32,
    pub ifs_clm_tx_cnt_addr: u32,
    pub ifs_clm_edcca_excl_cca_fa_mask: u32,
    pub ifs_clm_tx_cnt_msk: u32,
    pub ifs_clm_cca_addr: u32,
    pub ifs_clm_ofdmcca_excl_fa_mask: u32,
    pub ifs_clm_cckcca_excl_fa_mask: u32,
    pub ifs_clm_fa_addr: u32,
    pub ifs_clm_ofdm_fa_mask: u32,
    pub ifs_clm_cck_fa_mask: u32,
    pub ifs_his_addr: u32,
    pub ifs_his_addr2: u32,
    pub ifs_t4_his_mask: u32,
    pub ifs_t3_his_mask: u32,
    pub ifs_t2_his_mask: u32,
    pub ifs_t1_his_mask: u32,
    pub ifs_avg_l_addr: u32,
    pub ifs_t2_avg_mask: u32,
    pub ifs_t1_avg_mask: u32,
    pub ifs_avg_h_addr: u32,
    pub ifs_t4_avg_mask: u32,
    pub ifs_t3_avg_mask: u32,
    pub ifs_cca_l_addr: u32,
    pub ifs_t2_cca_mask: u32,
    pub ifs_t1_cca_mask: u32,
    pub ifs_cca_h_addr: u32,
    pub ifs_t4_cca_mask: u32,
    pub ifs_t3_cca_mask: u32,
    pub ifs_total_addr: u32,
    pub ifs_cnt_done_mask: u32,
    pub ifs_total_mask: u32,
    pub nhm: u32,
    pub nhm_ready: u32,
    pub nhm_config: u32,
    pub nhm_period_mask: u32,
    pub nhm_unit_mask: u32,
    pub nhm_include_cca_mask: u32,
    pub nhm_en_mask: u32,
    pub nhm_method: u32,
    pub nhm_pwr_method_msk: u32,
    pub edcca_clm_rdy: u32,
    pub edcca_clm_rdy_mask: u32,
    pub edcca_clm_cnt: u32,
    pub edcca_clm_cnt_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_physts_regs {
    pub setting_addr: u32,
    pub dis_trigger_fail_mask: u32,
    pub dis_trigger_brk_mask: u32,
    pub mac_phy_intf_sel: rtw89_reg_def,
    pub txpwr: *const rtw89_reg_def,
    pub tx_info: rtw89_regs_def,
    pub tx_common_ctrl: rtw89_regs_def,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_cfo_regs {
    pub comp: u32,
    pub weighting_mask: u32,
    pub comp_seg0: u32,
    pub valid_0_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_bb_wrap_regs {
    pub pwr_macid_lmt: u32,
    pub pwr_macid_path: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_nctl_regs {
    pub cfg: u32,
    pub rw: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bandwidth_section_num_ax {
    RTW89_BW20_SEC_NUM_AX = 8,
    RTW89_BW40_SEC_NUM_AX = 4,
    RTW89_BW80_SEC_NUM_AX = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_bandwidth_section_num_be {
    RTW89_BW20_SEC_NUM_BE = 16,
    RTW89_BW40_SEC_NUM_BE = 8,
    RTW89_BW80_SEC_NUM_BE = 4,
    RTW89_BW160_SEC_NUM_BE = 2,
}

pub const RTW89_TXPWR_LMT_PAGE_SIZE_AX: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_limit_ax {
    pub cck_20m: [i8; RTW89_BF_NUM],
    pub cck_40m: [i8; RTW89_BF_NUM],
    pub ofdm: [i8; RTW89_BF_NUM],
    pub mcs_20m: [i8; RTW89_BW20_SEC_NUM_AX][RTW89_BF_NUM],
    pub mcs_40m: [i8; RTW89_BW40_SEC_NUM_AX][RTW89_BF_NUM],
    pub mcs_80m: [i8; RTW89_BW80_SEC_NUM_AX][RTW89_BF_NUM],
    pub mcs_160m: [i8; RTW89_BF_NUM],
    pub mcs_40m_0p5: [i8; RTW89_BF_NUM],
    pub mcs_40m_2p5: [i8; RTW89_BF_NUM],
}

pub const RTW89_TXPWR_LMT_PAGE_SIZE_BE: c_int = 76;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_limit_be {
    pub cck_20m: [i8; RTW89_BF_NUM],
    pub cck_40m: [i8; RTW89_BF_NUM],
    pub ofdm: [i8; RTW89_BF_NUM],
    pub mcs_20m: [i8; RTW89_BW20_SEC_NUM_BE][RTW89_BF_NUM],
    pub mcs_40m: [i8; RTW89_BW40_SEC_NUM_BE][RTW89_BF_NUM],
    pub mcs_80m: [i8; RTW89_BW80_SEC_NUM_BE][RTW89_BF_NUM],
    pub mcs_160m: [i8; RTW89_BW160_SEC_NUM_BE][RTW89_BF_NUM],
    pub mcs_320m: [i8; RTW89_BF_NUM],
    pub mcs_40m_0p5: [i8; RTW89_BF_NUM],
    pub mcs_40m_2p5: [i8; RTW89_BF_NUM],
    pub mcs_40m_4p5: [i8; RTW89_BF_NUM],
    pub mcs_40m_6p5: [i8; RTW89_BF_NUM],
}

pub const RTW89_RU_SEC_NUM_AX: c_int = 8;
pub const RTW89_TXPWR_LMT_RU_PAGE_SIZE_AX: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_limit_ru_ax {
    pub ru26: [i8; RTW89_RU_SEC_NUM_AX],
    pub ru52: [i8; RTW89_RU_SEC_NUM_AX],
    pub ru106: [i8; RTW89_RU_SEC_NUM_AX],
}

pub const RTW89_RU_SEC_NUM_BE: c_int = 16;
pub const RTW89_TXPWR_LMT_RU_PAGE_SIZE_BE: c_int = 80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_limit_ru_be {
    pub ru26: [i8; RTW89_RU_SEC_NUM_BE],
    pub ru52: [i8; RTW89_RU_SEC_NUM_BE],
    pub ru106: [i8; RTW89_RU_SEC_NUM_BE],
    pub ru52_26: [i8; RTW89_RU_SEC_NUM_BE],
    pub ru106_26: [i8; RTW89_RU_SEC_NUM_BE],
}

pub const RTW89_RU484_242_SEC_NUM_BE: c_int = 4;
pub const RTW89_RU996_484_SEC_NUM_BE: c_int = 2;
pub const RTW89_RU996_484_242_SEC_NUM_BE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_txpwr_limit_large_mru_be {
    pub ru484_242: [i8; RTW89_NSS_NUM][RTW89_RU484_242_SEC_NUM_BE],
    pub ru996_484: [i8; RTW89_NSS_NUM][RTW89_RU996_484_SEC_NUM_BE],
    pub ru996_484_242: [i8; RTW89_NSS_NUM][RTW89_RU996_484_242_SEC_NUM_BE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_rfk_log_fmt {
    pub elm: [*const rtw89_fw_element_hdr; RTW89_PHY_C2H_RFK_LOG_FUNC_NUM],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_mdpd_onoff {
    MDPD_ON = 0,
    MDPD_OFF = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_oob_dpd_onoff {
    OOB_DPD_OFF = 0,
    OOB_DPD_ON = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_cim3k_onoff {
    CIM3K_ON = 1,
    CIM3K_OFF = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_cim3k_en_dis {
    CIM3K_ENABLE = 1,
    CIM3K_DISABLE = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rfsi_ctrl_modulation {
    RFSI_BPSK = 0,
    RFSI_QPSK = 1,
    RFSI_16QAM = 2,
    RFSI_64QAM = 3,
    RFSI_256QAM = 4,
    RFSI_1024QAM = 5,
    RFSI_4096QAM = 6,
    RFSI_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_cfir_onoff {
    CFIR_ON = 1,
    CFIR_OFF = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_filter_onoff {
    FILTER_A_ON = 1,
    FILTER_A_OFF = 0,
}

pub const MAX_TX_RFSI_CTRL_OPT: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_bb_wrap_common_data {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_bb_wrap_data_cim3k {
    pub bandedge: u8 th, ow, non_bandedge,,
    pub cim3k: },
    pub rfsi_ct_opt: [u32; 2],
    pub pb_tb: u8,
    pub bands: [}; RFSI_CTRL_BAND_NUM],
    pub qam_th: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_bb_wrap_common_data_gen3 {
    pub qam_th: [u8; 6],
    pub bands: [}; RFSI_CTRL_BAND_NUM],
    pub cck_val: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_bb_wrap_data {
    pub common: *const rtw89_bb_wrap_common_data,
    pub common_gen3: *const rtw89_bb_wrap_common_data_gen3,
    pub qam_comp_th0: [u16; MAX_TX_RFSI_CTRL_OPT],
    pub /: *mut *mut u16 qam_comp_th1[MAX_TX_RFSI_CTRL_OPT]; / encoded,
    pub /: *mut *mut u16 qam_comp_th2[MAX_TX_RFSI_CTRL_OPT]; / encoded,
    pub qam_comp_ow: [u16; MAX_TX_RFSI_CTRL_OPT],
    pub oob_dpd_by_cbw: [u8; 8],
    pub bands: [}; RFSI_CTRL_BAND_NUM],
    pub mdpd_by_dbw: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_gen_def {
    pub cr_base: u32,
    pub physt_bmp_start: u32,
    pub physt_bmp_eht: u32,
    pub physt_ie_len: [u8; 32],
    pub physt_gen: u8,
    pub ccx: *const rtw89_ccx_regs,
    pub physts: *const rtw89_physts_regs,
    pub cfo: *const rtw89_cfo_regs,
    pub bb_wrap: *const rtw89_bb_wrap_regs,
    pub nctl: *const rtw89_nctl_regs,
    pub addr): *mut *mut *mut u32 (phy0_phy1_offset)(struct rtw89_dev rtwdev, u32,
    pub extra_data): *mut c_void,
    pub rtwdev): *mut *mut void (preinit_rf_nctl)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (bb_wrap_init)(struct rtw89_dev,
    pub rtwdev): *mut *mut void (ch_info_init)(struct rtw89_dev,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): rtw89_phy_idx,
    pub phy_idx): rtw89_phy_idx,
}

extern "C" {
    pub fn rtw89_read8(_arg: rtwdev, phy->cr_base: addr +) -> return;
}
extern "C" {
    pub fn rtw89_read16(_arg: rtwdev, phy->cr_base: addr +) -> return;
}
extern "C" {
    pub fn rtw89_read32(_arg: rtwdev, phy->cr_base: addr +) -> return;
}
extern "C" {
    pub fn rtw89_read32_mask(_arg: rtwdev, phy->cr_base: addr +, _arg: mask) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfk_chan_desc {
// desc is valid iff ch is non-zero
    pub ch: u8,
// To avoid us from extending old chip code every time, each new
// field must be defined along with a bool flag in positivte way.
//
    pub has_band: bool,
    pub band: u8,
    pub has_bw: bool,
    pub bw: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rfk_flag {
    RTW89_RFK_F_WRF = 0,
    RTW89_RFK_F_WM = 1,
    RTW89_RFK_F_WS = 2,
    RTW89_RFK_F_WC = 3,
    RTW89_RFK_F_DELAY = 4,
    RTW89_RFK_F_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rfk_tbl {
    pub defs: *const rtw89_reg5_def,
    pub size: u32,
}

extern "C" {
    pub fn rtw89_phy_init_bb_reg(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_init_bb_afe(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_init_rf_reg(rtwdev: *mut rtw89_dev, noio: bool);
}
extern "C" {
    pub fn rtw89_phy_dm_init(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_dm_reinit(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_dm_init_data(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_physts_parsing_init(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_ant_gain_init(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn clamp_t(_arg: i16, chip->txpwr_factor_mac: dbm <<, _arg: -64, _arg: 63) -> return;
}
extern "C" {
    pub fn rtw89_phy_ra_assoc(rtwdev: *mut rtw89_dev, rtwsta_link: *mut rtw89_sta_link);
}
extern "C" {
    pub fn rtw89_phy_ra_update(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_ra_recalc_agg_limit(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_c2h_chk_atomic(rtwdev: *mut rtw89_dev, class: u8, func: u8) -> bool;
}
extern "C" {
    pub fn rtw89_phy_cfo_track(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_cfo_track_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_phy_stat_track(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_env_monitor_track(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_dig_reset(rtwdev: *mut rtw89_dev, bb: *mut rtw89_bb_ctx);
}
extern "C" {
    pub fn rtw89_phy_dig(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_dig_suspend(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_dig_resume(rtwdev: *mut rtw89_dev, restore: bool);
}
extern "C" {
    pub fn rtw89_phy_tx_path_div_track(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_antdiv_track(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_antdiv_work(wiphy: *mut wiphy, work: *mut wiphy_work);
}
extern "C" {
    pub fn rtw89_phy_ul_tb_assoc(rtwdev: *mut rtw89_dev, rtwvif_link: *mut rtw89_vif_link);
}
extern "C" {
    pub fn rtw89_phy_ul_tb_ctrl_track(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_encode_chan_idx(rtwdev: *mut rtw89_dev, central_ch: u8, band: u8) -> u8;
}
extern "C" {
    pub fn rtw89_phy_edcca_track(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_edcca_thre_calc(rtwdev: *mut rtw89_dev, bb: *mut rtw89_bb_ctx);
}
extern "C" {
    pub fn rtw89_phy_nhm_setting_init(rtwdev: *mut rtw89_dev);
}
extern "C" {
    pub fn rtw89_phy_nhm_trigger(rtwdev: *mut rtw89_dev);
}
