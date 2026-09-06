//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/rtl2832_priv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Realtek RTL2832 DVB-T demodulator driver
//
// Copyright (C) 2012 Thomas Mair <thomas.mair86@gmail.com>
// Copyright (C) 2012-2014 Antti Palosaari <crope@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl2832_dev {
    pub pdata: *mut rtl2832_platform_data,
    pub client: *mut i2c_client,
    pub regmap_config: regmap_config,
    pub regmap: *mut regmap,
    pub muxc: *mut i2c_mux_core,
    pub fe: dvb_frontend,
    pub fe_status: fe_status,
    pub /: *mut *mut u64 post_bit_error_prev; / for old DVBv3 read_ber() calculation,
    pub post_bit_error: u64,
    pub post_bit_count: u64,
    pub sleeping: bool,
    pub i2c_gate_work: delayed_work,
    pub /: *mut *mut unsigned long filters; / PID filter,
    pub slave_ts: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl2832_reg_entry {
    pub start_address: u16,
    pub msb: u8,
    pub lsb: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl2832_reg_value {
    pub reg: c_int,
    pub value: u32,
}

// Demod register bit names
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DVBT_REG_BIT_NAME {
    DVBT_SOFT_RST,
    DVBT_IIC_REPEAT,
    DVBT_TR_WAIT_MIN_8K,
    DVBT_RSD_BER_FAIL_VAL,
    DVBT_EN_BK_TRK,
    DVBT_REG_PI,
    DVBT_REG_PFREQ_1_0,
    DVBT_PD_DA8,
    DVBT_LOCK_TH,
    DVBT_BER_PASS_SCAL,
    DVBT_CE_FFSM_BYPASS,
    DVBT_ALPHAIIR_N,
    DVBT_ALPHAIIR_DIF,
    DVBT_EN_TRK_SPAN,
    DVBT_LOCK_TH_LEN,
    DVBT_CCI_THRE,
    DVBT_CCI_MON_SCAL,
    DVBT_CCI_M0,
    DVBT_CCI_M1,
    DVBT_CCI_M2,
    DVBT_CCI_M3,
    DVBT_SPEC_INIT_0,
    DVBT_SPEC_INIT_1,
    DVBT_SPEC_INIT_2,
    DVBT_AD_EN_REG,
    DVBT_AD_EN_REG1,
    DVBT_EN_BBIN,
    DVBT_MGD_THD0,
    DVBT_MGD_THD1,
    DVBT_MGD_THD2,
    DVBT_MGD_THD3,
    DVBT_MGD_THD4,
    DVBT_MGD_THD5,
    DVBT_MGD_THD6,
    DVBT_MGD_THD7,
    DVBT_EN_CACQ_NOTCH,
    DVBT_AD_AV_REF,
    DVBT_PIP_ON,
    DVBT_SCALE1_B92,
    DVBT_SCALE1_B93,
    DVBT_SCALE1_BA7,
    DVBT_SCALE1_BA9,
    DVBT_SCALE1_BAA,
    DVBT_SCALE1_BAB,
    DVBT_SCALE1_BAC,
    DVBT_SCALE1_BB0,
    DVBT_SCALE1_BB1,
    DVBT_KB_P1,
    DVBT_KB_P2,
    DVBT_KB_P3,
    DVBT_OPT_ADC_IQ,
    DVBT_AD_AVI,
    DVBT_AD_AVQ,
    DVBT_K1_CR_STEP12,
    DVBT_TRK_KS_P2,
    DVBT_TRK_KS_I2,
    DVBT_TR_THD_SET2,
    DVBT_TRK_KC_P2,
    DVBT_TRK_KC_I2,
    DVBT_CR_THD_SET2,
    DVBT_PSET_IFFREQ,
    DVBT_SPEC_INV,
    DVBT_BW_INDEX,
    DVBT_RSAMP_RATIO,
    DVBT_CFREQ_OFF_RATIO,
    DVBT_FSM_STAGE,
    DVBT_RX_CONSTEL,
    DVBT_RX_HIER,
    DVBT_RX_C_RATE_LP,
    DVBT_RX_C_RATE_HP,
    DVBT_GI_IDX,
    DVBT_FFT_MODE_IDX,
    DVBT_RSD_BER_EST,
    DVBT_CE_EST_EVM,
    DVBT_RF_AGC_VAL,
    DVBT_IF_AGC_VAL,
    DVBT_DAGC_VAL,
    DVBT_SFREQ_OFF,
    DVBT_CFREQ_OFF,
    DVBT_POLAR_RF_AGC,
    DVBT_POLAR_IF_AGC,
    DVBT_AAGC_HOLD,
    DVBT_EN_RF_AGC,
    DVBT_EN_IF_AGC,
    DVBT_IF_AGC_MIN,
    DVBT_IF_AGC_MAX,
    DVBT_RF_AGC_MIN,
    DVBT_RF_AGC_MAX,
    DVBT_IF_AGC_MAN,
    DVBT_IF_AGC_MAN_VAL,
    DVBT_RF_AGC_MAN,
    DVBT_RF_AGC_MAN_VAL,
    DVBT_DAGC_TRG_VAL,
    DVBT_AGC_TARG_VAL,
    DVBT_LOOP_GAIN_3_0,
    DVBT_LOOP_GAIN_4,
    DVBT_VTOP,
    DVBT_KRF,
    DVBT_AGC_TARG_VAL_0,
    DVBT_AGC_TARG_VAL_8_1,
    DVBT_AAGC_LOOP_GAIN,
    DVBT_LOOP_GAIN2_3_0,
    DVBT_LOOP_GAIN2_4,
    DVBT_LOOP_GAIN3,
    DVBT_VTOP1,
    DVBT_VTOP2,
    DVBT_VTOP3,
    DVBT_KRF1,
    DVBT_KRF2,
    DVBT_KRF3,
    DVBT_KRF4,
    DVBT_EN_GI_PGA,
    DVBT_THD_LOCK_UP,
    DVBT_THD_LOCK_DW,
    DVBT_THD_UP1,
    DVBT_THD_DW1,
    DVBT_INTER_CNT_LEN,
    DVBT_GI_PGA_STATE,
    DVBT_EN_AGC_PGA,
    DVBT_CKOUTPAR,
    DVBT_CKOUT_PWR,
    DVBT_SYNC_DUR,
    DVBT_ERR_DUR,
    DVBT_SYNC_LVL,
    DVBT_ERR_LVL,
    DVBT_VAL_LVL,
    DVBT_SERIAL,
    DVBT_SER_LSB,
    DVBT_CDIV_PH0,
    DVBT_CDIV_PH1,
    DVBT_MPEG_IO_OPT_2_2,
    DVBT_MPEG_IO_OPT_1_0,
    DVBT_CKOUTPAR_PIP,
    DVBT_CKOUT_PWR_PIP,
    DVBT_SYNC_LVL_PIP,
    DVBT_ERR_LVL_PIP,
    DVBT_VAL_LVL_PIP,
    DVBT_CKOUTPAR_PID,
    DVBT_CKOUT_PWR_PID,
    DVBT_SYNC_LVL_PID,
    DVBT_ERR_LVL_PID,
    DVBT_VAL_LVL_PID,
    DVBT_SM_PASS,
    DVBT_UPDATE_REG_2,
    DVBT_BTHD_P3,
    DVBT_BTHD_D3,
    DVBT_FUNC4_REG0,
    DVBT_FUNC4_REG1,
    DVBT_FUNC4_REG2,
    DVBT_FUNC4_REG3,
    DVBT_FUNC4_REG4,
    DVBT_FUNC4_REG5,
    DVBT_FUNC4_REG6,
    DVBT_FUNC4_REG7,
    DVBT_FUNC4_REG8,
    DVBT_FUNC4_REG9,
    DVBT_FUNC4_REG10,
    DVBT_FUNC5_REG0,
    DVBT_FUNC5_REG1,
    DVBT_FUNC5_REG2,
    DVBT_FUNC5_REG3,
    DVBT_FUNC5_REG4,
    DVBT_FUNC5_REG5,
    DVBT_FUNC5_REG6,
    DVBT_FUNC5_REG7,
    DVBT_FUNC5_REG8,
    DVBT_FUNC5_REG9,
    DVBT_FUNC5_REG10,
    DVBT_FUNC5_REG11,
    DVBT_FUNC5_REG12,
    DVBT_FUNC5_REG13,
    DVBT_FUNC5_REG14,
    DVBT_FUNC5_REG15,
    DVBT_FUNC5_REG16,
    DVBT_FUNC5_REG17,
    DVBT_FUNC5_REG18,
    DVBT_AD7_SETTING,
    DVBT_RSSI_R,
    DVBT_ACI_DET_IND,
    DVBT_REG_MON,
    DVBT_REG_MONSEL,
    DVBT_REG_GPE,
    DVBT_REG_GPO,
    DVBT_REG_4MSEL,
    DVBT_TEST_REG_1,
    DVBT_TEST_REG_2,
    DVBT_TEST_REG_3,
    DVBT_TEST_REG_4,
    DVBT_REG_BIT_NAME_ITEM_TERMINATOR,
}
