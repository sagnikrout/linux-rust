//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv0367_regs.h
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
// stv0367_regs.h
//
// Driver for ST STV0367 DVB-T & DVB-C demodulator IC.
//
// Copyright (C) ST Microelectronics.
// Copyright (C) 2010,2011 NetUP Inc.
// Copyright (C) 2010,2011 Igor M. Liplianin <liplianin@netup.ru>
//
// ID
pub const R367TER_ID: c_uint = 0xf000;
pub const F367TER_IDENTIFICATIONREG: c_uint = 0xf00000ff;
// I2CRPT
pub const R367TER_I2CRPT: c_uint = 0xf001;
pub const F367TER_I2CT_ON: c_uint = 0xf0010080;
pub const F367TER_ENARPT_LEVEL: c_uint = 0xf0010070;
pub const F367TER_SCLT_DELAY: c_uint = 0xf0010008;
pub const F367TER_SCLT_NOD: c_uint = 0xf0010004;
pub const F367TER_STOP_ENABLE: c_uint = 0xf0010002;
pub const F367TER_SDAT_NOD: c_uint = 0xf0010001;
// TOPCTRL
pub const R367TER_TOPCTRL: c_uint = 0xf002;
pub const F367TER_STDBY: c_uint = 0xf0020080;
pub const F367TER_STDBY_FEC: c_uint = 0xf0020040;
pub const F367TER_STDBY_CORE: c_uint = 0xf0020020;
pub const F367TER_QAM_COFDM: c_uint = 0xf0020010;
pub const F367TER_TS_DIS: c_uint = 0xf0020008;
pub const F367TER_DIR_CLK_216: c_uint = 0xf0020004;
pub const F367TER_TUNER_BB: c_uint = 0xf0020002;
pub const F367TER_DVBT_H: c_uint = 0xf0020001;
// IOCFG0
pub const R367TER_IOCFG0: c_uint = 0xf003;
pub const F367TER_OP0_SD: c_uint = 0xf0030080;
pub const F367TER_OP0_VAL: c_uint = 0xf0030040;
pub const F367TER_OP0_OD: c_uint = 0xf0030020;
pub const F367TER_OP0_INV: c_uint = 0xf0030010;
pub const F367TER_OP0_DACVALUE_HI: c_uint = 0xf003000f;
// DAc0R
pub const R367TER_DAC0R: c_uint = 0xf004;
pub const F367TER_OP0_DACVALUE_LO: c_uint = 0xf00400ff;
// IOCFG1
pub const R367TER_IOCFG1: c_uint = 0xf005;
pub const F367TER_IP0: c_uint = 0xf0050040;
pub const F367TER_OP1_OD: c_uint = 0xf0050020;
pub const F367TER_OP1_INV: c_uint = 0xf0050010;
pub const F367TER_OP1_DACVALUE_HI: c_uint = 0xf005000f;
// DAC1R
pub const R367TER_DAC1R: c_uint = 0xf006;
pub const F367TER_OP1_DACVALUE_LO: c_uint = 0xf00600ff;
// IOCFG2
pub const R367TER_IOCFG2: c_uint = 0xf007;
pub const F367TER_OP2_LOCK_CONF: c_uint = 0xf00700e0;
pub const F367TER_OP2_OD: c_uint = 0xf0070010;
pub const F367TER_OP2_VAL: c_uint = 0xf0070008;
pub const F367TER_OP1_LOCK_CONF: c_uint = 0xf0070007;
// SDFR
pub const R367TER_SDFR: c_uint = 0xf008;
pub const F367TER_OP0_FREQ: c_uint = 0xf00800f0;
pub const F367TER_OP1_FREQ: c_uint = 0xf008000f;
// STATUS
pub const R367TER_STATUS: c_uint = 0xf009;
pub const F367TER_TPS_LOCK: c_uint = 0xf0090080;
pub const F367TER_SYR_LOCK: c_uint = 0xf0090040;
pub const F367TER_AGC_LOCK: c_uint = 0xf0090020;
pub const F367TER_PRF: c_uint = 0xf0090010;
pub const F367TER_LK: c_uint = 0xf0090008;
pub const F367TER_PR: c_uint = 0xf0090007;
// AUX_CLK
pub const R367TER_AUX_CLK: c_uint = 0xf00a;
pub const F367TER_AUXFEC_CTL: c_uint = 0xf00a00c0;
pub const F367TER_DIS_CKX4: c_uint = 0xf00a0020;
pub const F367TER_CKSEL: c_uint = 0xf00a0018;
pub const F367TER_CKDIV_PROG: c_uint = 0xf00a0006;
pub const F367TER_AUXCLK_ENA: c_uint = 0xf00a0001;
// FREESYS1
pub const R367TER_FREESYS1: c_uint = 0xf00b;
pub const F367TER_FREE_SYS1: c_uint = 0xf00b00ff;
// FREESYS2
pub const R367TER_FREESYS2: c_uint = 0xf00c;
pub const F367TER_FREE_SYS2: c_uint = 0xf00c00ff;
// FREESYS3
pub const R367TER_FREESYS3: c_uint = 0xf00d;
pub const F367TER_FREE_SYS3: c_uint = 0xf00d00ff;
// GPIO_CFG
pub const R367TER_GPIO_CFG: c_uint = 0xf00e;
pub const F367TER_GPIO7_NOD: c_uint = 0xf00e0080;
pub const F367TER_GPIO7_CFG: c_uint = 0xf00e0040;
pub const F367TER_GPIO6_NOD: c_uint = 0xf00e0020;
pub const F367TER_GPIO6_CFG: c_uint = 0xf00e0010;
pub const F367TER_GPIO5_NOD: c_uint = 0xf00e0008;
pub const F367TER_GPIO5_CFG: c_uint = 0xf00e0004;
pub const F367TER_GPIO4_NOD: c_uint = 0xf00e0002;
pub const F367TER_GPIO4_CFG: c_uint = 0xf00e0001;
// GPIO_CMD
pub const R367TER_GPIO_CMD: c_uint = 0xf00f;
pub const F367TER_GPIO7_VAL: c_uint = 0xf00f0008;
pub const F367TER_GPIO6_VAL: c_uint = 0xf00f0004;
pub const F367TER_GPIO5_VAL: c_uint = 0xf00f0002;
pub const F367TER_GPIO4_VAL: c_uint = 0xf00f0001;
// AGC2MAX
pub const R367TER_AGC2MAX: c_uint = 0xf010;
pub const F367TER_AGC2_MAX: c_uint = 0xf01000ff;
// AGC2MIN
pub const R367TER_AGC2MIN: c_uint = 0xf011;
pub const F367TER_AGC2_MIN: c_uint = 0xf01100ff;
// AGC1MAX
pub const R367TER_AGC1MAX: c_uint = 0xf012;
pub const F367TER_AGC1_MAX: c_uint = 0xf01200ff;
// AGC1MIN
pub const R367TER_AGC1MIN: c_uint = 0xf013;
pub const F367TER_AGC1_MIN: c_uint = 0xf01300ff;
// AGCR
pub const R367TER_AGCR: c_uint = 0xf014;
pub const F367TER_RATIO_A: c_uint = 0xf01400e0;
pub const F367TER_RATIO_B: c_uint = 0xf0140018;
pub const F367TER_RATIO_C: c_uint = 0xf0140007;
// AGC2TH
pub const R367TER_AGC2TH: c_uint = 0xf015;
pub const F367TER_AGC2_THRES: c_uint = 0xf01500ff;
// AGC12c
pub const R367TER_AGC12C: c_uint = 0xf016;
pub const F367TER_AGC1_IV: c_uint = 0xf0160080;
pub const F367TER_AGC1_OD: c_uint = 0xf0160040;
pub const F367TER_AGC1_LOAD: c_uint = 0xf0160020;
pub const F367TER_AGC2_IV: c_uint = 0xf0160010;
pub const F367TER_AGC2_OD: c_uint = 0xf0160008;
pub const F367TER_AGC2_LOAD: c_uint = 0xf0160004;
pub const F367TER_AGC12_MODE: c_uint = 0xf0160003;
// AGCCTRL1
pub const R367TER_AGCCTRL1: c_uint = 0xf017;
pub const F367TER_DAGC_ON: c_uint = 0xf0170080;
pub const F367TER_INVERT_AGC12: c_uint = 0xf0170040;
pub const F367TER_AGC1_MODE: c_uint = 0xf0170008;
pub const F367TER_AGC2_MODE: c_uint = 0xf0170007;
// AGCCTRL2
pub const R367TER_AGCCTRL2: c_uint = 0xf018;
pub const F367TER_FRZ2_CTRL: c_uint = 0xf0180060;
pub const F367TER_FRZ1_CTRL: c_uint = 0xf0180018;
pub const F367TER_TIME_CST: c_uint = 0xf0180007;
// AGC1VAL1
pub const R367TER_AGC1VAL1: c_uint = 0xf019;
pub const F367TER_AGC1_VAL_LO: c_uint = 0xf01900ff;
// AGC1VAL2
pub const R367TER_AGC1VAL2: c_uint = 0xf01a;
pub const F367TER_AGC1_VAL_HI: c_uint = 0xf01a000f;
// AGC2VAL1
pub const R367TER_AGC2VAL1: c_uint = 0xf01b;
pub const F367TER_AGC2_VAL_LO: c_uint = 0xf01b00ff;
// AGC2VAL2
pub const R367TER_AGC2VAL2: c_uint = 0xf01c;
pub const F367TER_AGC2_VAL_HI: c_uint = 0xf01c000f;
// AGC2PGA
pub const R367TER_AGC2PGA: c_uint = 0xf01d;
pub const F367TER_AGC2_PGA: c_uint = 0xf01d00ff;
// OVF_RATE1
pub const R367TER_OVF_RATE1: c_uint = 0xf01e;
pub const F367TER_OVF_RATE_HI: c_uint = 0xf01e000f;
// OVF_RATE2
pub const R367TER_OVF_RATE2: c_uint = 0xf01f;
pub const F367TER_OVF_RATE_LO: c_uint = 0xf01f00ff;
// GAIN_SRC1
pub const R367TER_GAIN_SRC1: c_uint = 0xf020;
pub const F367TER_INV_SPECTR: c_uint = 0xf0200080;
pub const F367TER_IQ_INVERT: c_uint = 0xf0200040;
pub const F367TER_INR_BYPASS: c_uint = 0xf0200020;
pub const F367TER_STATUS_INV_SPECRUM: c_uint = 0xf0200010;
pub const F367TER_GAIN_SRC_HI: c_uint = 0xf020000f;
// GAIN_SRC2
pub const R367TER_GAIN_SRC2: c_uint = 0xf021;
pub const F367TER_GAIN_SRC_LO: c_uint = 0xf02100ff;
// INC_DEROT1
pub const R367TER_INC_DEROT1: c_uint = 0xf022;
pub const F367TER_INC_DEROT_HI: c_uint = 0xf02200ff;
// INC_DEROT2
pub const R367TER_INC_DEROT2: c_uint = 0xf023;
pub const F367TER_INC_DEROT_LO: c_uint = 0xf02300ff;
// PPM_CPAMP_DIR
pub const R367TER_PPM_CPAMP_DIR: c_uint = 0xf024;
pub const F367TER_PPM_CPAMP_DIRECT: c_uint = 0xf02400ff;
// PPM_CPAMP_INV
pub const R367TER_PPM_CPAMP_INV: c_uint = 0xf025;
pub const F367TER_PPM_CPAMP_INVER: c_uint = 0xf02500ff;
// FREESTFE_1
pub const R367TER_FREESTFE_1: c_uint = 0xf026;
pub const F367TER_SYMBOL_NUMBER_INC: c_uint = 0xf02600c0;
pub const F367TER_SEL_LSB: c_uint = 0xf0260004;
pub const F367TER_AVERAGE_ON: c_uint = 0xf0260002;
pub const F367TER_DC_ADJ: c_uint = 0xf0260001;
// FREESTFE_2
pub const R367TER_FREESTFE_2: c_uint = 0xf027;
pub const F367TER_SEL_SRCOUT: c_uint = 0xf02700c0;
pub const F367TER_SEL_SYRTHR: c_uint = 0xf027001f;
// DCOFFSET
pub const R367TER_DCOFFSET: c_uint = 0xf028;
pub const F367TER_SELECT_I_Q: c_uint = 0xf0280080;
pub const F367TER_DC_OFFSET: c_uint = 0xf028007f;
// EN_PROCESS
pub const R367TER_EN_PROCESS: c_uint = 0xf029;
pub const F367TER_FREE: c_uint = 0xf02900f0;
pub const F367TER_ENAB_MANUAL: c_uint = 0xf0290001;
// SDI_SMOOTHER
pub const R367TER_SDI_SMOOTHER: c_uint = 0xf02a;
pub const F367TER_DIS_SMOOTH: c_uint = 0xf02a0080;
pub const F367TER_SDI_INC_SMOOTHER: c_uint = 0xf02a007f;
// FE_LOOP_OPEN
pub const R367TER_FE_LOOP_OPEN: c_uint = 0xf02b;
pub const F367TER_TRL_LOOP_OP: c_uint = 0xf02b0002;
pub const F367TER_CRL_LOOP_OP: c_uint = 0xf02b0001;
// FREQOFF1
pub const R367TER_FREQOFF1: c_uint = 0xf02c;
pub const F367TER_FREQ_OFFSET_LOOP_OPEN_VHI: c_uint = 0xf02c00ff;
// FREQOFF2
pub const R367TER_FREQOFF2: c_uint = 0xf02d;
pub const F367TER_FREQ_OFFSET_LOOP_OPEN_HI: c_uint = 0xf02d00ff;
// FREQOFF3
pub const R367TER_FREQOFF3: c_uint = 0xf02e;
pub const F367TER_FREQ_OFFSET_LOOP_OPEN_LO: c_uint = 0xf02e00ff;
// TIMOFF1
pub const R367TER_TIMOFF1: c_uint = 0xf02f;
pub const F367TER_TIM_OFFSET_LOOP_OPEN_HI: c_uint = 0xf02f00ff;
// TIMOFF2
pub const R367TER_TIMOFF2: c_uint = 0xf030;
pub const F367TER_TIM_OFFSET_LOOP_OPEN_LO: c_uint = 0xf03000ff;
// EPQ
pub const R367TER_EPQ: c_uint = 0xf031;
pub const F367TER_EPQ1: c_uint = 0xf03100ff;
// EPQAUTO
pub const R367TER_EPQAUTO: c_uint = 0xf032;
pub const F367TER_EPQ2: c_uint = 0xf03200ff;
// SYR_UPDATE
pub const R367TER_SYR_UPDATE: c_uint = 0xf033;
pub const F367TER_SYR_PROTV: c_uint = 0xf0330080;
pub const F367TER_SYR_PROTV_GAIN: c_uint = 0xf0330060;
pub const F367TER_SYR_FILTER: c_uint = 0xf0330010;
pub const F367TER_SYR_TRACK_THRES: c_uint = 0xf033000c;
// CHPFREE
pub const R367TER_CHPFREE: c_uint = 0xf034;
pub const F367TER_CHP_FREE: c_uint = 0xf03400ff;
// PPM_STATE_MAC
pub const R367TER_PPM_STATE_MAC: c_uint = 0xf035;
pub const F367TER_PPM_STATE_MACHINE_DECODER: c_uint = 0xf035003f;
// INR_THRESHOLD
pub const R367TER_INR_THRESHOLD: c_uint = 0xf036;
pub const F367TER_INR_THRESH: c_uint = 0xf03600ff;
// EPQ_TPS_ID_CELL
pub const R367TER_EPQ_TPS_ID_CELL: c_uint = 0xf037;
pub const F367TER_ENABLE_LGTH_TO_CF: c_uint = 0xf0370080;
pub const F367TER_DIS_TPS_RSVD: c_uint = 0xf0370040;
pub const F367TER_DIS_BCH: c_uint = 0xf0370020;
pub const F367TER_DIS_ID_CEL: c_uint = 0xf0370010;
pub const F367TER_TPS_ADJUST_SYM: c_uint = 0xf037000f;
// EPQ_CFG
pub const R367TER_EPQ_CFG: c_uint = 0xf038;
pub const F367TER_EPQ_RANGE: c_uint = 0xf0380002;
pub const F367TER_EPQ_SOFT: c_uint = 0xf0380001;
// EPQ_STATUS
pub const R367TER_EPQ_STATUS: c_uint = 0xf039;
pub const F367TER_SLOPE_INC: c_uint = 0xf03900fc;
pub const F367TER_TPS_FIELD: c_uint = 0xf0390003;
// AUTORELOCK
pub const R367TER_AUTORELOCK: c_uint = 0xf03a;
pub const F367TER_BYPASS_BER_TEMPO: c_uint = 0xf03a0080;
pub const F367TER_BER_TEMPO: c_uint = 0xf03a0070;
pub const F367TER_BYPASS_COFDM_TEMPO: c_uint = 0xf03a0008;
pub const F367TER_COFDM_TEMPO: c_uint = 0xf03a0007;
// BER_THR_VMSB
pub const R367TER_BER_THR_VMSB: c_uint = 0xf03b;
pub const F367TER_BER_THRESHOLD_HI: c_uint = 0xf03b00ff;
// BER_THR_MSB
pub const R367TER_BER_THR_MSB: c_uint = 0xf03c;
pub const F367TER_BER_THRESHOLD_MID: c_uint = 0xf03c00ff;
// BER_THR_LSB
pub const R367TER_BER_THR_LSB: c_uint = 0xf03d;
pub const F367TER_BER_THRESHOLD_LO: c_uint = 0xf03d00ff;
// CCD
pub const R367TER_CCD: c_uint = 0xf03e;
pub const F367TER_CCD_DETECTED: c_uint = 0xf03e0080;
pub const F367TER_CCD_RESET: c_uint = 0xf03e0040;
pub const F367TER_CCD_THRESHOLD: c_uint = 0xf03e000f;
// SPECTR_CFG
pub const R367TER_SPECTR_CFG: c_uint = 0xf03f;
pub const F367TER_SPECT_CFG: c_uint = 0xf03f0003;
// CONSTMU_MSB
pub const R367TER_CONSTMU_MSB: c_uint = 0xf040;
pub const F367TER_CONSTMU_FREEZE: c_uint = 0xf0400080;
pub const F367TER_CONSTNU_FORCE_EN: c_uint = 0xf0400040;
pub const F367TER_CONST_MU_MSB: c_uint = 0xf040003f;
// CONSTMU_LSB
pub const R367TER_CONSTMU_LSB: c_uint = 0xf041;
pub const F367TER_CONST_MU_LSB: c_uint = 0xf04100ff;
// CONSTMU_MAX_MSB
pub const R367TER_CONSTMU_MAX_MSB: c_uint = 0xf042;
pub const F367TER_CONST_MU_MAX_MSB: c_uint = 0xf042003f;
// CONSTMU_MAX_LSB
pub const R367TER_CONSTMU_MAX_LSB: c_uint = 0xf043;
pub const F367TER_CONST_MU_MAX_LSB: c_uint = 0xf04300ff;
// ALPHANOISE
pub const R367TER_ALPHANOISE: c_uint = 0xf044;
pub const F367TER_USE_ALLFILTER: c_uint = 0xf0440080;
pub const F367TER_INTER_ON: c_uint = 0xf0440040;
pub const F367TER_ALPHA_NOISE: c_uint = 0xf044001f;
// MAXGP_MSB
pub const R367TER_MAXGP_MSB: c_uint = 0xf045;
pub const F367TER_MUFILTER_LENGTH: c_uint = 0xf04500f0;
pub const F367TER_MAX_GP_MSB: c_uint = 0xf045000f;
// MAXGP_LSB
pub const R367TER_MAXGP_LSB: c_uint = 0xf046;
pub const F367TER_MAX_GP_LSB: c_uint = 0xf04600ff;
// ALPHAMSB
pub const R367TER_ALPHAMSB: c_uint = 0xf047;
pub const F367TER_CHC_DATARATE: c_uint = 0xf04700c0;
pub const F367TER_ALPHA_MSB: c_uint = 0xf047003f;
// ALPHALSB
pub const R367TER_ALPHALSB: c_uint = 0xf048;
pub const F367TER_ALPHA_LSB: c_uint = 0xf04800ff;
// PILOT_ACCU
pub const R367TER_PILOT_ACCU: c_uint = 0xf049;
pub const F367TER_USE_SCAT4ADDAPT: c_uint = 0xf0490080;
pub const F367TER_PILOT_ACC: c_uint = 0xf049001f;
// PILOTMU_ACCU
pub const R367TER_PILOTMU_ACCU: c_uint = 0xf04a;
pub const F367TER_DISCARD_BAD_SP: c_uint = 0xf04a0080;
pub const F367TER_DISCARD_BAD_CP: c_uint = 0xf04a0040;
pub const F367TER_PILOT_MU_ACCU: c_uint = 0xf04a001f;
// FILT_CHANNEL_EST
pub const R367TER_FILT_CHANNEL_EST: c_uint = 0xf04b;
pub const F367TER_USE_FILT_PILOT: c_uint = 0xf04b0080;
pub const F367TER_FILT_CHANNEL: c_uint = 0xf04b007f;
// ALPHA_NOPISE_FREQ
pub const R367TER_ALPHA_NOPISE_FREQ: c_uint = 0xf04c;
pub const F367TER_NOISE_FREQ_FILT: c_uint = 0xf04c0040;
pub const F367TER_ALPHA_NOISE_FREQ: c_uint = 0xf04c003f;
// RATIO_PILOT
pub const R367TER_RATIO_PILOT: c_uint = 0xf04d;
pub const F367TER_RATIO_MEAN_SP: c_uint = 0xf04d00f0;
pub const F367TER_RATIO_MEAN_CP: c_uint = 0xf04d000f;
// CHC_CTL
pub const R367TER_CHC_CTL: c_uint = 0xf04e;
pub const F367TER_TRACK_EN: c_uint = 0xf04e0080;
pub const F367TER_NOISE_NORM_EN: c_uint = 0xf04e0040;
pub const F367TER_FORCE_CHC_RESET: c_uint = 0xf04e0020;
pub const F367TER_SHORT_TIME: c_uint = 0xf04e0010;
pub const F367TER_FORCE_STATE_EN: c_uint = 0xf04e0008;
pub const F367TER_FORCE_STATE: c_uint = 0xf04e0007;
// EPQ_ADJUST
pub const R367TER_EPQ_ADJUST: c_uint = 0xf04f;
pub const F367TER_ADJUST_SCAT_IND: c_uint = 0xf04f00c0;
pub const F367TER_ONE_SYMBOL: c_uint = 0xf04f0010;
pub const F367TER_EPQ_DECAY: c_uint = 0xf04f000e;
pub const F367TER_HOLD_SLOPE: c_uint = 0xf04f0001;
// EPQ_THRES
pub const R367TER_EPQ_THRES: c_uint = 0xf050;
pub const F367TER_EPQ_THR: c_uint = 0xf05000ff;
// OMEGA_CTL
pub const R367TER_OMEGA_CTL: c_uint = 0xf051;
pub const F367TER_OMEGA_RST: c_uint = 0xf0510080;
pub const F367TER_FREEZE_OMEGA: c_uint = 0xf0510040;
pub const F367TER_OMEGA_SEL: c_uint = 0xf051003f;
// GP_CTL
pub const R367TER_GP_CTL: c_uint = 0xf052;
pub const F367TER_CHC_STATE: c_uint = 0xf05200e0;
pub const F367TER_FREEZE_GP: c_uint = 0xf0520010;
pub const F367TER_GP_SEL: c_uint = 0xf052000f;
// MUMSB
pub const R367TER_MUMSB: c_uint = 0xf053;
pub const F367TER_MU_MSB: c_uint = 0xf053007f;
// MULSB
pub const R367TER_MULSB: c_uint = 0xf054;
pub const F367TER_MU_LSB: c_uint = 0xf05400ff;
// GPMSB
pub const R367TER_GPMSB: c_uint = 0xf055;
pub const F367TER_CSI_THRESHOLD: c_uint = 0xf05500e0;
pub const F367TER_GP_MSB: c_uint = 0xf055000f;
// GPLSB
pub const R367TER_GPLSB: c_uint = 0xf056;
pub const F367TER_GP_LSB: c_uint = 0xf05600ff;
// OMEGAMSB
pub const R367TER_OMEGAMSB: c_uint = 0xf057;
pub const F367TER_OMEGA_MSB: c_uint = 0xf057007f;
// OMEGALSB
pub const R367TER_OMEGALSB: c_uint = 0xf058;
pub const F367TER_OMEGA_LSB: c_uint = 0xf05800ff;
// SCAT_NB
pub const R367TER_SCAT_NB: c_uint = 0xf059;
pub const F367TER_CHC_TEST: c_uint = 0xf05900f8;
pub const F367TER_SCAT_NUMB: c_uint = 0xf0590003;
// CHC_DUMMY
pub const R367TER_CHC_DUMMY: c_uint = 0xf05a;
pub const F367TER_CHC_DUM: c_uint = 0xf05a00ff;
// INC_CTL
pub const R367TER_INC_CTL: c_uint = 0xf05b;
pub const F367TER_INC_BYPASS: c_uint = 0xf05b0080;
pub const F367TER_INC_NDEPTH: c_uint = 0xf05b000c;
pub const F367TER_INC_MADEPTH: c_uint = 0xf05b0003;
// INCTHRES_COR1
pub const R367TER_INCTHRES_COR1: c_uint = 0xf05c;
pub const F367TER_INC_THRES_COR1: c_uint = 0xf05c00ff;
// INCTHRES_COR2
pub const R367TER_INCTHRES_COR2: c_uint = 0xf05d;
pub const F367TER_INC_THRES_COR2: c_uint = 0xf05d00ff;
// INCTHRES_DET1
pub const R367TER_INCTHRES_DET1: c_uint = 0xf05e;
pub const F367TER_INC_THRES_DET1: c_uint = 0xf05e003f;
// INCTHRES_DET2
pub const R367TER_INCTHRES_DET2: c_uint = 0xf05f;
pub const F367TER_INC_THRES_DET2: c_uint = 0xf05f003f;
// IIR_CELLNB
pub const R367TER_IIR_CELLNB: c_uint = 0xf060;
pub const F367TER_NRST_IIR: c_uint = 0xf0600080;
pub const F367TER_IIR_CELL_NB: c_uint = 0xf0600007;
// IIRCX_COEFF1_MSB
pub const R367TER_IIRCX_COEFF1_MSB: c_uint = 0xf061;
pub const F367TER_IIR_CX_COEFF1_MSB: c_uint = 0xf06100ff;
// IIRCX_COEFF1_LSB
pub const R367TER_IIRCX_COEFF1_LSB: c_uint = 0xf062;
pub const F367TER_IIR_CX_COEFF1_LSB: c_uint = 0xf06200ff;
// IIRCX_COEFF2_MSB
pub const R367TER_IIRCX_COEFF2_MSB: c_uint = 0xf063;
pub const F367TER_IIR_CX_COEFF2_MSB: c_uint = 0xf06300ff;
// IIRCX_COEFF2_LSB
pub const R367TER_IIRCX_COEFF2_LSB: c_uint = 0xf064;
pub const F367TER_IIR_CX_COEFF2_LSB: c_uint = 0xf06400ff;
// IIRCX_COEFF3_MSB
pub const R367TER_IIRCX_COEFF3_MSB: c_uint = 0xf065;
pub const F367TER_IIR_CX_COEFF3_MSB: c_uint = 0xf06500ff;
// IIRCX_COEFF3_LSB
pub const R367TER_IIRCX_COEFF3_LSB: c_uint = 0xf066;
pub const F367TER_IIR_CX_COEFF3_LSB: c_uint = 0xf06600ff;
// IIRCX_COEFF4_MSB
pub const R367TER_IIRCX_COEFF4_MSB: c_uint = 0xf067;
pub const F367TER_IIR_CX_COEFF4_MSB: c_uint = 0xf06700ff;
// IIRCX_COEFF4_LSB
pub const R367TER_IIRCX_COEFF4_LSB: c_uint = 0xf068;
pub const F367TER_IIR_CX_COEFF4_LSB: c_uint = 0xf06800ff;
// IIRCX_COEFF5_MSB
pub const R367TER_IIRCX_COEFF5_MSB: c_uint = 0xf069;
pub const F367TER_IIR_CX_COEFF5_MSB: c_uint = 0xf06900ff;
// IIRCX_COEFF5_LSB
pub const R367TER_IIRCX_COEFF5_LSB: c_uint = 0xf06a;
pub const F367TER_IIR_CX_COEFF5_LSB: c_uint = 0xf06a00ff;
// FEPATH_CFG
pub const R367TER_FEPATH_CFG: c_uint = 0xf06b;
pub const F367TER_DEMUX_SWAP: c_uint = 0xf06b0004;
pub const F367TER_DIGAGC_SWAP: c_uint = 0xf06b0002;
pub const F367TER_LONGPATH_IF: c_uint = 0xf06b0001;
// PMC1_FUNC
pub const R367TER_PMC1_FUNC: c_uint = 0xf06c;
pub const F367TER_SOFT_RSTN: c_uint = 0xf06c0080;
pub const F367TER_PMC1_AVERAGE_TIME: c_uint = 0xf06c0078;
pub const F367TER_PMC1_WAIT_TIME: c_uint = 0xf06c0006;
pub const F367TER_PMC1_2N_SEL: c_uint = 0xf06c0001;
// PMC1_FOR
pub const R367TER_PMC1_FOR: c_uint = 0xf06d;
pub const F367TER_PMC1_FORCE: c_uint = 0xf06d0080;
pub const F367TER_PMC1_FORCE_VALUE: c_uint = 0xf06d007c;
// PMC2_FUNC
pub const R367TER_PMC2_FUNC: c_uint = 0xf06e;
pub const F367TER_PMC2_SOFT_STN: c_uint = 0xf06e0080;
pub const F367TER_PMC2_ACCU_TIME: c_uint = 0xf06e0070;
pub const F367TER_PMC2_CMDP_MN: c_uint = 0xf06e0008;
pub const F367TER_PMC2_SWAP: c_uint = 0xf06e0004;
// STATUS_ERR_DA
pub const R367TER_STATUS_ERR_DA: c_uint = 0xf06f;
pub const F367TER_COM_USEGAINTRK: c_uint = 0xf06f0080;
pub const F367TER_COM_AGCLOCK: c_uint = 0xf06f0040;
pub const F367TER_AUT_AGCLOCK: c_uint = 0xf06f0020;
pub const F367TER_MIN_ERR_X_LSB: c_uint = 0xf06f000f;
// DIG_AGC_R
pub const R367TER_DIG_AGC_R: c_uint = 0xf070;
pub const F367TER_COM_SOFT_RSTN: c_uint = 0xf0700080;
pub const F367TER_COM_AGC_ON: c_uint = 0xf0700040;
pub const F367TER_COM_EARLY: c_uint = 0xf0700020;
pub const F367TER_AUT_SOFT_RESETN: c_uint = 0xf0700010;
pub const F367TER_AUT_AGC_ON: c_uint = 0xf0700008;
pub const F367TER_AUT_EARLY: c_uint = 0xf0700004;
pub const F367TER_AUT_ROT_EN: c_uint = 0xf0700002;
pub const F367TER_LOCK_SOFT_RESETN: c_uint = 0xf0700001;
// COMAGC_TARMSB
pub const R367TER_COMAGC_TARMSB: c_uint = 0xf071;
pub const F367TER_COM_AGC_TARGET_MSB: c_uint = 0xf07100ff;
// COM_AGC_TAR_ENMODE
pub const R367TER_COM_AGC_TAR_ENMODE: c_uint = 0xf072;
pub const F367TER_COM_AGC_TARGET_LSB: c_uint = 0xf07200f0;
pub const F367TER_COM_ENMODE: c_uint = 0xf072000f;
// COM_AGC_CFG
pub const R367TER_COM_AGC_CFG: c_uint = 0xf073;
pub const F367TER_COM_N: c_uint = 0xf07300f8;
pub const F367TER_COM_STABMODE: c_uint = 0xf0730006;
pub const F367TER_ERR_SEL: c_uint = 0xf0730001;
// COM_AGC_GAIN1
pub const R367TER_COM_AGC_GAIN1: c_uint = 0xf074;
pub const F367TER_COM_GAIN1aCK: c_uint = 0xf07400f0;
pub const F367TER_COM_GAIN1TRK: c_uint = 0xf074000f;
// AUT_AGC_TARGETMSB
pub const R367TER_AUT_AGC_TARGETMSB: c_uint = 0xf075;
pub const F367TER_AUT_AGC_TARGET_MSB: c_uint = 0xf07500ff;
// LOCK_DET_MSB
pub const R367TER_LOCK_DET_MSB: c_uint = 0xf076;
pub const F367TER_LOCK_DETECT_MSB: c_uint = 0xf07600ff;
// AGCTAR_LOCK_LSBS
pub const R367TER_AGCTAR_LOCK_LSBS: c_uint = 0xf077;
pub const F367TER_AUT_AGC_TARGET_LSB: c_uint = 0xf07700f0;
pub const F367TER_LOCK_DETECT_LSB: c_uint = 0xf077000f;
// AUT_GAIN_EN
pub const R367TER_AUT_GAIN_EN: c_uint = 0xf078;
pub const F367TER_AUT_ENMODE: c_uint = 0xf07800f0;
pub const F367TER_AUT_GAIN2: c_uint = 0xf078000f;
// AUT_CFG
pub const R367TER_AUT_CFG: c_uint = 0xf079;
pub const F367TER_AUT_N: c_uint = 0xf07900f8;
pub const F367TER_INT_CHOICE: c_uint = 0xf0790006;
pub const F367TER_INT_LOAD: c_uint = 0xf0790001;
// LOCKN
pub const R367TER_LOCKN: c_uint = 0xf07a;
pub const F367TER_LOCK_N: c_uint = 0xf07a00f8;
pub const F367TER_SEL_IQNTAR: c_uint = 0xf07a0004;
pub const F367TER_LOCK_DETECT_CHOICE: c_uint = 0xf07a0003;
// INT_X_3
pub const R367TER_INT_X_3: c_uint = 0xf07b;
pub const F367TER_INT_X3: c_uint = 0xf07b00ff;
// INT_X_2
pub const R367TER_INT_X_2: c_uint = 0xf07c;
pub const F367TER_INT_X2: c_uint = 0xf07c00ff;
// INT_X_1
pub const R367TER_INT_X_1: c_uint = 0xf07d;
pub const F367TER_INT_X1: c_uint = 0xf07d00ff;
// INT_X_0
pub const R367TER_INT_X_0: c_uint = 0xf07e;
pub const F367TER_INT_X0: c_uint = 0xf07e00ff;
// MIN_ERRX_MSB
pub const R367TER_MIN_ERRX_MSB: c_uint = 0xf07f;
pub const F367TER_MIN_ERR_X_MSB: c_uint = 0xf07f00ff;
// COR_CTL
pub const R367TER_COR_CTL: c_uint = 0xf080;
pub const F367TER_CORE_ACTIVE: c_uint = 0xf0800020;
pub const F367TER_HOLD: c_uint = 0xf0800010;
pub const F367TER_CORE_STATE_CTL: c_uint = 0xf080000f;
// COR_STAT
pub const R367TER_COR_STAT: c_uint = 0xf081;
pub const F367TER_SCATT_LOCKED: c_uint = 0xf0810080;
pub const F367TER_TPS_LOCKED: c_uint = 0xf0810040;
pub const F367TER_SYR_LOCKED_COR: c_uint = 0xf0810020;
pub const F367TER_AGC_LOCKED_STAT: c_uint = 0xf0810010;
pub const F367TER_CORE_STATE_STAT: c_uint = 0xf081000f;
// COR_INTEN
pub const R367TER_COR_INTEN: c_uint = 0xf082;
pub const F367TER_INTEN: c_uint = 0xf0820080;
pub const F367TER_INTEN_SYR: c_uint = 0xf0820020;
pub const F367TER_INTEN_FFT: c_uint = 0xf0820010;
pub const F367TER_INTEN_AGC: c_uint = 0xf0820008;
pub const F367TER_INTEN_TPS1: c_uint = 0xf0820004;
pub const F367TER_INTEN_TPS2: c_uint = 0xf0820002;
pub const F367TER_INTEN_TPS3: c_uint = 0xf0820001;
// COR_INTSTAT
pub const R367TER_COR_INTSTAT: c_uint = 0xf083;
pub const F367TER_INTSTAT_SYR: c_uint = 0xf0830020;
pub const F367TER_INTSTAT_FFT: c_uint = 0xf0830010;
pub const F367TER_INTSAT_AGC: c_uint = 0xf0830008;
pub const F367TER_INTSTAT_TPS1: c_uint = 0xf0830004;
pub const F367TER_INTSTAT_TPS2: c_uint = 0xf0830002;
pub const F367TER_INTSTAT_TPS3: c_uint = 0xf0830001;
// COR_MODEGUARD
pub const R367TER_COR_MODEGUARD: c_uint = 0xf084;
pub const F367TER_FORCE: c_uint = 0xf0840010;
pub const F367TER_MODE: c_uint = 0xf084000c;
pub const F367TER_GUARD: c_uint = 0xf0840003;
// AGC_CTL
pub const R367TER_AGC_CTL: c_uint = 0xf085;
pub const F367TER_AGC_TIMING_FACTOR: c_uint = 0xf08500e0;
pub const F367TER_AGC_LAST: c_uint = 0xf0850010;
pub const F367TER_AGC_GAIN: c_uint = 0xf085000c;
pub const F367TER_AGC_NEG: c_uint = 0xf0850002;
pub const F367TER_AGC_SET: c_uint = 0xf0850001;
// AGC_MANUAL1
pub const R367TER_AGC_MANUAL1: c_uint = 0xf086;
pub const F367TER_AGC_VAL_LO: c_uint = 0xf08600ff;
// AGC_MANUAL2
pub const R367TER_AGC_MANUAL2: c_uint = 0xf087;
pub const F367TER_AGC_VAL_HI: c_uint = 0xf087000f;
// AGC_TARG
pub const R367TER_AGC_TARG: c_uint = 0xf088;
pub const F367TER_AGC_TARGET: c_uint = 0xf08800ff;
// AGC_GAIN1
pub const R367TER_AGC_GAIN1: c_uint = 0xf089;
pub const F367TER_AGC_GAIN_LO: c_uint = 0xf08900ff;
// AGC_GAIN2
pub const R367TER_AGC_GAIN2: c_uint = 0xf08a;
pub const F367TER_AGC_LOCKED_GAIN2: c_uint = 0xf08a0010;
pub const F367TER_AGC_GAIN_HI: c_uint = 0xf08a000f;
// RESERVED_1
pub const R367TER_RESERVED_1: c_uint = 0xf08b;
pub const F367TER_RESERVED1: c_uint = 0xf08b00ff;
// RESERVED_2
pub const R367TER_RESERVED_2: c_uint = 0xf08c;
pub const F367TER_RESERVED2: c_uint = 0xf08c00ff;
// RESERVED_3
pub const R367TER_RESERVED_3: c_uint = 0xf08d;
pub const F367TER_RESERVED3: c_uint = 0xf08d00ff;
// CAS_CTL
pub const R367TER_CAS_CTL: c_uint = 0xf08e;
pub const F367TER_CCS_ENABLE: c_uint = 0xf08e0080;
pub const F367TER_ACS_DISABLE: c_uint = 0xf08e0040;
pub const F367TER_DAGC_DIS: c_uint = 0xf08e0020;
pub const F367TER_DAGC_GAIN: c_uint = 0xf08e0018;
pub const F367TER_CCSMU: c_uint = 0xf08e0007;
// CAS_FREQ
pub const R367TER_CAS_FREQ: c_uint = 0xf08f;
pub const F367TER_CCS_FREQ: c_uint = 0xf08f00ff;
// CAS_DAGCGAIN
pub const R367TER_CAS_DAGCGAIN: c_uint = 0xf090;
pub const F367TER_CAS_DAGC_GAIN: c_uint = 0xf09000ff;
// SYR_CTL
pub const R367TER_SYR_CTL: c_uint = 0xf091;
pub const F367TER_SICTH_ENABLE: c_uint = 0xf0910080;
pub const F367TER_LONG_ECHO: c_uint = 0xf0910078;
pub const F367TER_AUTO_LE_EN: c_uint = 0xf0910004;
pub const F367TER_SYR_BYPASS: c_uint = 0xf0910002;
pub const F367TER_SYR_TR_DIS: c_uint = 0xf0910001;
// SYR_STAT
pub const R367TER_SYR_STAT: c_uint = 0xf092;
pub const F367TER_SYR_LOCKED_STAT: c_uint = 0xf0920010;
pub const F367TER_SYR_MODE: c_uint = 0xf092000c;
pub const F367TER_SYR_GUARD: c_uint = 0xf0920003;
// SYR_NCO1
pub const R367TER_SYR_NCO1: c_uint = 0xf093;
pub const F367TER_SYR_NCO_LO: c_uint = 0xf09300ff;
// SYR_NCO2
pub const R367TER_SYR_NCO2: c_uint = 0xf094;
pub const F367TER_SYR_NCO_HI: c_uint = 0xf094003f;
// SYR_OFFSET1
pub const R367TER_SYR_OFFSET1: c_uint = 0xf095;
pub const F367TER_SYR_OFFSET_LO: c_uint = 0xf09500ff;
// SYR_OFFSET2
pub const R367TER_SYR_OFFSET2: c_uint = 0xf096;
pub const F367TER_SYR_OFFSET_HI: c_uint = 0xf096003f;
// FFT_CTL
pub const R367TER_FFT_CTL: c_uint = 0xf097;
pub const F367TER_SHIFT_FFT_TRIG: c_uint = 0xf0970018;
pub const F367TER_FFT_TRIGGER: c_uint = 0xf0970004;
pub const F367TER_FFT_MANUAL: c_uint = 0xf0970002;
pub const F367TER_IFFT_MODE: c_uint = 0xf0970001;
// SCR_CTL
pub const R367TER_SCR_CTL: c_uint = 0xf098;
pub const F367TER_SYRADJDECAY: c_uint = 0xf0980070;
pub const F367TER_SCR_CPEDIS: c_uint = 0xf0980002;
pub const F367TER_SCR_DIS: c_uint = 0xf0980001;
// PPM_CTL1
pub const R367TER_PPM_CTL1: c_uint = 0xf099;
pub const F367TER_PPM_MAXFREQ: c_uint = 0xf0990030;
pub const F367TER_PPM_MAXTIM: c_uint = 0xf0990008;
pub const F367TER_PPM_INVSEL: c_uint = 0xf0990004;
pub const F367TER_PPM_SCATDIS: c_uint = 0xf0990002;
pub const F367TER_PPM_BYP: c_uint = 0xf0990001;
// TRL_CTL
pub const R367TER_TRL_CTL: c_uint = 0xf09a;
pub const F367TER_TRL_NOMRATE_LSB: c_uint = 0xf09a0080;
pub const F367TER_TRL_GAIN_FACTOR: c_uint = 0xf09a0078;
pub const F367TER_TRL_LOOPGAIN: c_uint = 0xf09a0007;
// TRL_NOMRATE1
pub const R367TER_TRL_NOMRATE1: c_uint = 0xf09b;
pub const F367TER_TRL_NOMRATE_LO: c_uint = 0xf09b00ff;
// TRL_NOMRATE2
pub const R367TER_TRL_NOMRATE2: c_uint = 0xf09c;
pub const F367TER_TRL_NOMRATE_HI: c_uint = 0xf09c00ff;
// TRL_TIME1
pub const R367TER_TRL_TIME1: c_uint = 0xf09d;
pub const F367TER_TRL_TOFFSET_LO: c_uint = 0xf09d00ff;
// TRL_TIME2
pub const R367TER_TRL_TIME2: c_uint = 0xf09e;
pub const F367TER_TRL_TOFFSET_HI: c_uint = 0xf09e00ff;
// CRL_CTL
pub const R367TER_CRL_CTL: c_uint = 0xf09f;
pub const F367TER_CRL_DIS: c_uint = 0xf09f0080;
pub const F367TER_CRL_GAIN_FACTOR: c_uint = 0xf09f0078;
pub const F367TER_CRL_LOOPGAIN: c_uint = 0xf09f0007;
// CRL_FREQ1
pub const R367TER_CRL_FREQ1: c_uint = 0xf0a0;
pub const F367TER_CRL_FOFFSET_LO: c_uint = 0xf0a000ff;
// CRL_FREQ2
pub const R367TER_CRL_FREQ2: c_uint = 0xf0a1;
pub const F367TER_CRL_FOFFSET_HI: c_uint = 0xf0a100ff;
// CRL_FREQ3
pub const R367TER_CRL_FREQ3: c_uint = 0xf0a2;
pub const F367TER_CRL_FOFFSET_VHI: c_uint = 0xf0a200ff;
// TPS_SFRAME_CTL
pub const R367TER_TPS_SFRAME_CTL: c_uint = 0xf0a3;
pub const F367TER_TPS_SFRAME_SYNC: c_uint = 0xf0a30001;
// CHC_SNR
pub const R367TER_CHC_SNR: c_uint = 0xf0a4;
pub const F367TER_CHCSNR: c_uint = 0xf0a400ff;
// BDI_CTL
pub const R367TER_BDI_CTL: c_uint = 0xf0a5;
pub const F367TER_BDI_LPSEL: c_uint = 0xf0a50002;
pub const F367TER_BDI_SERIAL: c_uint = 0xf0a50001;
// DMP_CTL
pub const R367TER_DMP_CTL: c_uint = 0xf0a6;
pub const F367TER_DMP_SCALING_FACTOR: c_uint = 0xf0a6001e;
pub const F367TER_DMP_SDDIS: c_uint = 0xf0a60001;
// TPS_RCVD1
pub const R367TER_TPS_RCVD1: c_uint = 0xf0a7;
pub const F367TER_TPS_CHANGE: c_uint = 0xf0a70040;
pub const F367TER_BCH_OK: c_uint = 0xf0a70020;
pub const F367TER_TPS_SYNC: c_uint = 0xf0a70010;
pub const F367TER_TPS_FRAME: c_uint = 0xf0a70003;
// TPS_RCVD2
pub const R367TER_TPS_RCVD2: c_uint = 0xf0a8;
pub const F367TER_TPS_HIERMODE: c_uint = 0xf0a80070;
pub const F367TER_TPS_CONST: c_uint = 0xf0a80003;
// TPS_RCVD3
pub const R367TER_TPS_RCVD3: c_uint = 0xf0a9;
pub const F367TER_TPS_LPCODE: c_uint = 0xf0a90070;
pub const F367TER_TPS_HPCODE: c_uint = 0xf0a90007;
// TPS_RCVD4
pub const R367TER_TPS_RCVD4: c_uint = 0xf0aa;
pub const F367TER_TPS_GUARD: c_uint = 0xf0aa0030;
pub const F367TER_TPS_MODE: c_uint = 0xf0aa0003;
// TPS_ID_CELL1
pub const R367TER_TPS_ID_CELL1: c_uint = 0xf0ab;
pub const F367TER_TPS_ID_CELL_LO: c_uint = 0xf0ab00ff;
// TPS_ID_CELL2
pub const R367TER_TPS_ID_CELL2: c_uint = 0xf0ac;
pub const F367TER_TPS_ID_CELL_HI: c_uint = 0xf0ac00ff;
// TPS_RCVD5_SET1
pub const R367TER_TPS_RCVD5_SET1: c_uint = 0xf0ad;
pub const F367TER_TPS_NA: c_uint = 0xf0ad00fC;
pub const F367TER_TPS_SETFRAME: c_uint = 0xf0ad0003;
// TPS_SET2
pub const R367TER_TPS_SET2: c_uint = 0xf0ae;
pub const F367TER_TPS_SETHIERMODE: c_uint = 0xf0ae0070;
pub const F367TER_TPS_SETCONST: c_uint = 0xf0ae0003;
// TPS_SET3
pub const R367TER_TPS_SET3: c_uint = 0xf0af;
pub const F367TER_TPS_SETLPCODE: c_uint = 0xf0af0070;
pub const F367TER_TPS_SETHPCODE: c_uint = 0xf0af0007;
// TPS_CTL
pub const R367TER_TPS_CTL: c_uint = 0xf0b0;
pub const F367TER_TPS_IMM: c_uint = 0xf0b00004;
pub const F367TER_TPS_BCHDIS: c_uint = 0xf0b00002;
pub const F367TER_TPS_UPDDIS: c_uint = 0xf0b00001;
// CTL_FFTOSNUM
pub const R367TER_CTL_FFTOSNUM: c_uint = 0xf0b1;
pub const F367TER_SYMBOL_NUMBER: c_uint = 0xf0b1007f;
// TESTSELECT
pub const R367TER_TESTSELECT: c_uint = 0xf0b2;
pub const F367TER_TEST_SELECT: c_uint = 0xf0b2001f;
// MSC_REV
pub const R367TER_MSC_REV: c_uint = 0xf0b3;
pub const F367TER_REV_NUMBER: c_uint = 0xf0b300ff;
// PIR_CTL
pub const R367TER_PIR_CTL: c_uint = 0xf0b4;
pub const F367TER_FREEZE: c_uint = 0xf0b40001;
// SNR_CARRIER1
pub const R367TER_SNR_CARRIER1: c_uint = 0xf0b5;
pub const F367TER_SNR_CARRIER_LO: c_uint = 0xf0b500ff;
// SNR_CARRIER2
pub const R367TER_SNR_CARRIER2: c_uint = 0xf0b6;
pub const F367TER_MEAN: c_uint = 0xf0b600c0;
pub const F367TER_SNR_CARRIER_HI: c_uint = 0xf0b6001f;
// PPM_CPAMP
pub const R367TER_PPM_CPAMP: c_uint = 0xf0b7;
pub const F367TER_PPM_CPC: c_uint = 0xf0b700ff;
// TSM_AP0
pub const R367TER_TSM_AP0: c_uint = 0xf0b8;
pub const F367TER_ADDRESS_BYTE_0: c_uint = 0xf0b800ff;
// TSM_AP1
pub const R367TER_TSM_AP1: c_uint = 0xf0b9;
pub const F367TER_ADDRESS_BYTE_1: c_uint = 0xf0b900ff;
// TSM_AP2
pub const R367TER_TSM_AP2: c_uint = 0xf0bA;
pub const F367TER_DATA_BYTE_0: c_uint = 0xf0ba00ff;
// TSM_AP3
pub const R367TER_TSM_AP3: c_uint = 0xf0bB;
pub const F367TER_DATA_BYTE_1: c_uint = 0xf0bb00ff;
// TSM_AP4
pub const R367TER_TSM_AP4: c_uint = 0xf0bC;
pub const F367TER_DATA_BYTE_2: c_uint = 0xf0bc00ff;
// TSM_AP5
pub const R367TER_TSM_AP5: c_uint = 0xf0bD;
pub const F367TER_DATA_BYTE_3: c_uint = 0xf0bd00ff;
// TSM_AP6
pub const R367TER_TSM_AP6: c_uint = 0xf0bE;
pub const F367TER_TSM_AP_6: c_uint = 0xf0be00ff;
// TSM_AP7
pub const R367TER_TSM_AP7: c_uint = 0xf0bF;
pub const F367TER_MEM_SELECT_BYTE: c_uint = 0xf0bf00ff;
// TSTRES
pub const R367TER_TSTRES: c_uint = 0xf0c0;
pub const F367TER_FRES_DISPLAY: c_uint = 0xf0c00080;
pub const F367TER_FRES_FIFO_AD: c_uint = 0xf0c00020;
pub const F367TER_FRESRS: c_uint = 0xf0c00010;
pub const F367TER_FRESACS: c_uint = 0xf0c00008;
pub const F367TER_FRESFEC: c_uint = 0xf0c00004;
pub const F367TER_FRES_PRIF: c_uint = 0xf0c00002;
pub const F367TER_FRESCORE: c_uint = 0xf0c00001;
// ANACTRL
pub const R367TER_ANACTRL: c_uint = 0xf0c1;
pub const F367TER_BYPASS_XTAL: c_uint = 0xf0c10040;
pub const F367TER_BYPASS_PLLXN: c_uint = 0xf0c1000c;
pub const F367TER_DIS_PAD_OSC: c_uint = 0xf0c10002;
pub const F367TER_STDBY_PLLXN: c_uint = 0xf0c10001;
// TSTBUS
pub const R367TER_TSTBUS: c_uint = 0xf0c2;
pub const F367TER_TS_BYTE_CLK_INV: c_uint = 0xf0c20080;
pub const F367TER_CFG_IP: c_uint = 0xf0c20070;
pub const F367TER_CFG_TST: c_uint = 0xf0c2000f;
// TSTRATE
pub const R367TER_TSTRATE: c_uint = 0xf0c6;
pub const F367TER_FORCEPHA: c_uint = 0xf0c60080;
pub const F367TER_FNEWPHA: c_uint = 0xf0c60010;
pub const F367TER_FROT90: c_uint = 0xf0c60008;
pub const F367TER_FR: c_uint = 0xf0c60007;
// CONSTMODE
pub const R367TER_CONSTMODE: c_uint = 0xf0cb;
pub const F367TER_TST_PRIF: c_uint = 0xf0cb00e0;
pub const F367TER_CAR_TYPE: c_uint = 0xf0cb0018;
pub const F367TER_CONST_MODE: c_uint = 0xf0cb0003;
// CONSTCARR1
pub const R367TER_CONSTCARR1: c_uint = 0xf0cc;
pub const F367TER_CONST_CARR_LO: c_uint = 0xf0cc00ff;
// CONSTCARR2
pub const R367TER_CONSTCARR2: c_uint = 0xf0cd;
pub const F367TER_CONST_CARR_HI: c_uint = 0xf0cd001f;
// ICONSTEL
pub const R367TER_ICONSTEL: c_uint = 0xf0ce;
pub const F367TER_PICONSTEL: c_uint = 0xf0ce00ff;
// QCONSTEL
pub const R367TER_QCONSTEL: c_uint = 0xf0cf;
pub const F367TER_PQCONSTEL: c_uint = 0xf0cf00ff;
// TSTBISTRES0
pub const R367TER_TSTBISTRES0: c_uint = 0xf0d0;
pub const F367TER_BEND_PPM: c_uint = 0xf0d00080;
pub const F367TER_BBAD_PPM: c_uint = 0xf0d00040;
pub const F367TER_BEND_FFTW: c_uint = 0xf0d00020;
pub const F367TER_BBAD_FFTW: c_uint = 0xf0d00010;
pub const F367TER_BEND_FFT_BUF: c_uint = 0xf0d00008;
pub const F367TER_BBAD_FFT_BUF: c_uint = 0xf0d00004;
pub const F367TER_BEND_SYR: c_uint = 0xf0d00002;
pub const F367TER_BBAD_SYR: c_uint = 0xf0d00001;
// TSTBISTRES1
pub const R367TER_TSTBISTRES1: c_uint = 0xf0d1;
pub const F367TER_BEND_CHC_CP: c_uint = 0xf0d10080;
pub const F367TER_BBAD_CHC_CP: c_uint = 0xf0d10040;
pub const F367TER_BEND_CHCI: c_uint = 0xf0d10020;
pub const F367TER_BBAD_CHCI: c_uint = 0xf0d10010;
pub const F367TER_BEND_BDI: c_uint = 0xf0d10008;
pub const F367TER_BBAD_BDI: c_uint = 0xf0d10004;
pub const F367TER_BEND_SDI: c_uint = 0xf0d10002;
pub const F367TER_BBAD_SDI: c_uint = 0xf0d10001;
// TSTBISTRES2
pub const R367TER_TSTBISTRES2: c_uint = 0xf0d2;
pub const F367TER_BEND_CHC_INC: c_uint = 0xf0d20080;
pub const F367TER_BBAD_CHC_INC: c_uint = 0xf0d20040;
pub const F367TER_BEND_CHC_SPP: c_uint = 0xf0d20020;
pub const F367TER_BBAD_CHC_SPP: c_uint = 0xf0d20010;
pub const F367TER_BEND_CHC_CPP: c_uint = 0xf0d20008;
pub const F367TER_BBAD_CHC_CPP: c_uint = 0xf0d20004;
pub const F367TER_BEND_CHC_SP: c_uint = 0xf0d20002;
pub const F367TER_BBAD_CHC_SP: c_uint = 0xf0d20001;
// TSTBISTRES3
pub const R367TER_TSTBISTRES3: c_uint = 0xf0d3;
pub const F367TER_BEND_QAM: c_uint = 0xf0d30080;
pub const F367TER_BBAD_QAM: c_uint = 0xf0d30040;
pub const F367TER_BEND_SFEC_VIT: c_uint = 0xf0d30020;
pub const F367TER_BBAD_SFEC_VIT: c_uint = 0xf0d30010;
pub const F367TER_BEND_SFEC_DLINE: c_uint = 0xf0d30008;
pub const F367TER_BBAD_SFEC_DLINE: c_uint = 0xf0d30004;
pub const F367TER_BEND_SFEC_HW: c_uint = 0xf0d30002;
pub const F367TER_BBAD_SFEC_HW: c_uint = 0xf0d30001;
// RF_AGC1
pub const R367TER_RF_AGC1: c_uint = 0xf0d4;
pub const F367TER_RF_AGC1_LEVEL_HI: c_uint = 0xf0d400ff;
// RF_AGC2
pub const R367TER_RF_AGC2: c_uint = 0xf0d5;
pub const F367TER_REF_ADGP: c_uint = 0xf0d50080;
pub const F367TER_STDBY_ADCGP: c_uint = 0xf0d50020;
pub const F367TER_CHANNEL_SEL: c_uint = 0xf0d5001c;
pub const F367TER_RF_AGC1_LEVEL_LO: c_uint = 0xf0d50003;
// ANADIGCTRL
pub const R367TER_ANADIGCTRL: c_uint = 0xf0d7;
pub const F367TER_SEL_CLKDEM: c_uint = 0xf0d70020;
pub const F367TER_EN_BUFFER_Q: c_uint = 0xf0d70010;
pub const F367TER_EN_BUFFER_I: c_uint = 0xf0d70008;
pub const F367TER_ADC_RIS_EGDE: c_uint = 0xf0d70004;
pub const F367TER_SGN_ADC: c_uint = 0xf0d70002;
pub const F367TER_SEL_AD12_SYNC: c_uint = 0xf0d70001;
// PLLMDIV
pub const R367TER_PLLMDIV: c_uint = 0xf0d8;
pub const F367TER_PLL_MDIV: c_uint = 0xf0d800ff;
// PLLNDIV
pub const R367TER_PLLNDIV: c_uint = 0xf0d9;
pub const F367TER_PLL_NDIV: c_uint = 0xf0d900ff;
// PLLSETUP
pub const R367TER_PLLSETUP: c_uint = 0xf0dA;
pub const F367TER_PLL_PDIV: c_uint = 0xf0da0070;
pub const F367TER_PLL_KDIV: c_uint = 0xf0da000f;
// DUAL_AD12
pub const R367TER_DUAL_AD12: c_uint = 0xf0dB;
pub const F367TER_FS20M: c_uint = 0xf0db0020;
pub const F367TER_FS50M: c_uint = 0xf0db0010;
pub const F367TER_INMODe0: c_uint = 0xf0db0008;
pub const F367TER_POFFQ: c_uint = 0xf0db0004;
pub const F367TER_POFFI: c_uint = 0xf0db0002;
pub const F367TER_INMODE1: c_uint = 0xf0db0001;
// TSTBIST
pub const R367TER_TSTBIST: c_uint = 0xf0dC;
pub const F367TER_TST_BYP_CLK: c_uint = 0xf0dc0080;
pub const F367TER_TST_GCLKENA_STD: c_uint = 0xf0dc0040;
pub const F367TER_TST_GCLKENA: c_uint = 0xf0dc0020;
pub const F367TER_TST_MEMBIST: c_uint = 0xf0dc001f;
// PAD_COMP_CTRL
pub const R367TER_PAD_COMP_CTRL: c_uint = 0xf0dD;
pub const F367TER_COMPTQ: c_uint = 0xf0dd0010;
pub const F367TER_COMPEN: c_uint = 0xf0dd0008;
pub const F367TER_FREEZE2: c_uint = 0xf0dd0004;
pub const F367TER_SLEEP_INHBT: c_uint = 0xf0dd0002;
pub const F367TER_CHIP_SLEEP: c_uint = 0xf0dd0001;
// PAD_COMP_WR
pub const R367TER_PAD_COMP_WR: c_uint = 0xf0de;
pub const F367TER_WR_ASRC: c_uint = 0xf0de007f;
// PAD_COMP_RD
pub const R367TER_PAD_COMP_RD: c_uint = 0xf0df;
pub const F367TER_COMPOK: c_uint = 0xf0df0080;
pub const F367TER_RD_ASRC: c_uint = 0xf0df007f;
// SYR_TARGET_FFTADJT_MSB
pub const R367TER_SYR_TARGET_FFTADJT_MSB: c_uint = 0xf100;
pub const F367TER_SYR_START: c_uint = 0xf1000080;
pub const F367TER_SYR_TARGET_FFTADJ_HI: c_uint = 0xf100000f;
// SYR_TARGET_FFTADJT_LSB
pub const R367TER_SYR_TARGET_FFTADJT_LSB: c_uint = 0xf101;
pub const F367TER_SYR_TARGET_FFTADJ_LO: c_uint = 0xf10100ff;
// SYR_TARGET_CHCADJT_MSB
pub const R367TER_SYR_TARGET_CHCADJT_MSB: c_uint = 0xf102;
pub const F367TER_SYR_TARGET_CHCADJ_HI: c_uint = 0xf102000f;
// SYR_TARGET_CHCADJT_LSB
pub const R367TER_SYR_TARGET_CHCADJT_LSB: c_uint = 0xf103;
pub const F367TER_SYR_TARGET_CHCADJ_LO: c_uint = 0xf10300ff;
// SYR_FLAG
pub const R367TER_SYR_FLAG: c_uint = 0xf104;
pub const F367TER_TRIG_FLG1: c_uint = 0xf1040080;
pub const F367TER_TRIG_FLG0: c_uint = 0xf1040040;
pub const F367TER_FFT_FLG1: c_uint = 0xf1040008;
pub const F367TER_FFT_FLG0: c_uint = 0xf1040004;
pub const F367TER_CHC_FLG1: c_uint = 0xf1040002;
pub const F367TER_CHC_FLG0: c_uint = 0xf1040001;
// CRL_TARGET1
pub const R367TER_CRL_TARGET1: c_uint = 0xf105;
pub const F367TER_CRL_START: c_uint = 0xf1050080;
pub const F367TER_CRL_TARGET_VHI: c_uint = 0xf105000f;
// CRL_TARGET2
pub const R367TER_CRL_TARGET2: c_uint = 0xf106;
pub const F367TER_CRL_TARGET_HI: c_uint = 0xf10600ff;
// CRL_TARGET3
pub const R367TER_CRL_TARGET3: c_uint = 0xf107;
pub const F367TER_CRL_TARGET_LO: c_uint = 0xf10700ff;
// CRL_TARGET4
pub const R367TER_CRL_TARGET4: c_uint = 0xf108;
pub const F367TER_CRL_TARGET_VLO: c_uint = 0xf10800ff;
// CRL_FLAG
pub const R367TER_CRL_FLAG: c_uint = 0xf109;
pub const F367TER_CRL_FLAG1: c_uint = 0xf1090002;
pub const F367TER_CRL_FLAG0: c_uint = 0xf1090001;
// TRL_TARGET1
pub const R367TER_TRL_TARGET1: c_uint = 0xf10a;
pub const F367TER_TRL_TARGET_HI: c_uint = 0xf10a00ff;
// TRL_TARGET2
pub const R367TER_TRL_TARGET2: c_uint = 0xf10b;
pub const F367TER_TRL_TARGET_LO: c_uint = 0xf10b00ff;
// TRL_CHC
pub const R367TER_TRL_CHC: c_uint = 0xf10c;
pub const F367TER_TRL_START: c_uint = 0xf10c0080;
pub const F367TER_CHC_START: c_uint = 0xf10c0040;
pub const F367TER_TRL_FLAG1: c_uint = 0xf10c0002;
pub const F367TER_TRL_FLAG0: c_uint = 0xf10c0001;
// CHC_SNR_TARG
pub const R367TER_CHC_SNR_TARG: c_uint = 0xf10d;
pub const F367TER_CHC_SNR_TARGET: c_uint = 0xf10d00ff;
// TOP_TRACK
pub const R367TER_TOP_TRACK: c_uint = 0xf10e;
pub const F367TER_TOP_START: c_uint = 0xf10e0080;
pub const F367TER_FIRST_FLAG: c_uint = 0xf10e0070;
pub const F367TER_TOP_FLAG1: c_uint = 0xf10e0008;
pub const F367TER_TOP_FLAG0: c_uint = 0xf10e0004;
pub const F367TER_CHC_FLAG1: c_uint = 0xf10e0002;
pub const F367TER_CHC_FLAG0: c_uint = 0xf10e0001;
// TRACKER_FREE1
pub const R367TER_TRACKER_FREE1: c_uint = 0xf10f;
pub const F367TER_TRACKER_FREE_1: c_uint = 0xf10f00ff;
// ERROR_CRL1
pub const R367TER_ERROR_CRL1: c_uint = 0xf110;
pub const F367TER_ERROR_CRL_VHI: c_uint = 0xf11000ff;
// ERROR_CRL2
pub const R367TER_ERROR_CRL2: c_uint = 0xf111;
pub const F367TER_ERROR_CRL_HI: c_uint = 0xf11100ff;
// ERROR_CRL3
pub const R367TER_ERROR_CRL3: c_uint = 0xf112;
pub const F367TER_ERROR_CRL_LOI: c_uint = 0xf11200ff;
// ERROR_CRL4
pub const R367TER_ERROR_CRL4: c_uint = 0xf113;
pub const F367TER_ERROR_CRL_VLO: c_uint = 0xf11300ff;
// DEC_NCO1
pub const R367TER_DEC_NCO1: c_uint = 0xf114;
pub const F367TER_DEC_NCO_VHI: c_uint = 0xf11400ff;
// DEC_NCO2
pub const R367TER_DEC_NCO2: c_uint = 0xf115;
pub const F367TER_DEC_NCO_HI: c_uint = 0xf11500ff;
// DEC_NCO3
pub const R367TER_DEC_NCO3: c_uint = 0xf116;
pub const F367TER_DEC_NCO_LO: c_uint = 0xf11600ff;
// SNR
pub const R367TER_SNR: c_uint = 0xf117;
pub const F367TER_SNRATIO: c_uint = 0xf11700ff;
// SYR_FFTADJ1
pub const R367TER_SYR_FFTADJ1: c_uint = 0xf118;
pub const F367TER_SYR_FFTADJ_HI: c_uint = 0xf11800ff;
// SYR_FFTADJ2
pub const R367TER_SYR_FFTADJ2: c_uint = 0xf119;
pub const F367TER_SYR_FFTADJ_LO: c_uint = 0xf11900ff;
// SYR_CHCADJ1
pub const R367TER_SYR_CHCADJ1: c_uint = 0xf11a;
pub const F367TER_SYR_CHCADJ_HI: c_uint = 0xf11a00ff;
// SYR_CHCADJ2
pub const R367TER_SYR_CHCADJ2: c_uint = 0xf11b;
pub const F367TER_SYR_CHCADJ_LO: c_uint = 0xf11b00ff;
// SYR_OFF
pub const R367TER_SYR_OFF: c_uint = 0xf11c;
pub const F367TER_SYR_OFFSET: c_uint = 0xf11c00ff;
// PPM_OFFSET1
pub const R367TER_PPM_OFFSET1: c_uint = 0xf11d;
pub const F367TER_PPM_OFFSET_HI: c_uint = 0xf11d00ff;
// PPM_OFFSET2
pub const R367TER_PPM_OFFSET2: c_uint = 0xf11e;
pub const F367TER_PPM_OFFSET_LO: c_uint = 0xf11e00ff;
// TRACKER_FREE2
pub const R367TER_TRACKER_FREE2: c_uint = 0xf11f;
pub const F367TER_TRACKER_FREE_2: c_uint = 0xf11f00ff;
// DEBG_LT10
pub const R367TER_DEBG_LT10: c_uint = 0xf120;
pub const F367TER_DEBUG_LT10: c_uint = 0xf12000ff;
// DEBG_LT11
pub const R367TER_DEBG_LT11: c_uint = 0xf121;
pub const F367TER_DEBUG_LT11: c_uint = 0xf12100ff;
// DEBG_LT12
pub const R367TER_DEBG_LT12: c_uint = 0xf122;
pub const F367TER_DEBUG_LT12: c_uint = 0xf12200ff;
// DEBG_LT13
pub const R367TER_DEBG_LT13: c_uint = 0xf123;
pub const F367TER_DEBUG_LT13: c_uint = 0xf12300ff;
// DEBG_LT14
pub const R367TER_DEBG_LT14: c_uint = 0xf124;
pub const F367TER_DEBUG_LT14: c_uint = 0xf12400ff;
// DEBG_LT15
pub const R367TER_DEBG_LT15: c_uint = 0xf125;
pub const F367TER_DEBUG_LT15: c_uint = 0xf12500ff;
// DEBG_LT16
pub const R367TER_DEBG_LT16: c_uint = 0xf126;
pub const F367TER_DEBUG_LT16: c_uint = 0xf12600ff;
// DEBG_LT17
pub const R367TER_DEBG_LT17: c_uint = 0xf127;
pub const F367TER_DEBUG_LT17: c_uint = 0xf12700ff;
// DEBG_LT18
pub const R367TER_DEBG_LT18: c_uint = 0xf128;
pub const F367TER_DEBUG_LT18: c_uint = 0xf12800ff;
// DEBG_LT19
pub const R367TER_DEBG_LT19: c_uint = 0xf129;
pub const F367TER_DEBUG_LT19: c_uint = 0xf12900ff;
// DEBG_LT1a
pub const R367TER_DEBG_LT1A: c_uint = 0xf12a;
pub const F367TER_DEBUG_LT1A: c_uint = 0xf12a00ff;
// DEBG_LT1b
pub const R367TER_DEBG_LT1B: c_uint = 0xf12b;
pub const F367TER_DEBUG_LT1B: c_uint = 0xf12b00ff;
// DEBG_LT1c
pub const R367TER_DEBG_LT1C: c_uint = 0xf12c;
pub const F367TER_DEBUG_LT1C: c_uint = 0xf12c00ff;
// DEBG_LT1D
pub const R367TER_DEBG_LT1D: c_uint = 0xf12d;
pub const F367TER_DEBUG_LT1D: c_uint = 0xf12d00ff;
// DEBG_LT1E
pub const R367TER_DEBG_LT1E: c_uint = 0xf12e;
pub const F367TER_DEBUG_LT1E: c_uint = 0xf12e00ff;
// DEBG_LT1F
pub const R367TER_DEBG_LT1F: c_uint = 0xf12f;
pub const F367TER_DEBUG_LT1F: c_uint = 0xf12f00ff;
// RCCFGH
pub const R367TER_RCCFGH: c_uint = 0xf200;
pub const F367TER_TSRCFIFO_DVBCI: c_uint = 0xf2000080;
pub const F367TER_TSRCFIFO_SERIAL: c_uint = 0xf2000040;
pub const F367TER_TSRCFIFO_DISABLE: c_uint = 0xf2000020;
pub const F367TER_TSFIFO_2TORC: c_uint = 0xf2000010;
pub const F367TER_TSRCFIFO_HSGNLOUT: c_uint = 0xf2000008;
pub const F367TER_TSRCFIFO_ERRMODE: c_uint = 0xf2000006;
pub const F367TER_RCCFGH_0: c_uint = 0xf2000001;
// RCCFGM
pub const R367TER_RCCFGM: c_uint = 0xf201;
pub const F367TER_TSRCFIFO_MANSPEED: c_uint = 0xf20100c0;
pub const F367TER_TSRCFIFO_PERMDATA: c_uint = 0xf2010020;
pub const F367TER_TSRCFIFO_NONEWSGNL: c_uint = 0xf2010010;
pub const F367TER_RCBYTE_OVERSAMPLING: c_uint = 0xf201000e;
pub const F367TER_TSRCFIFO_INVDATA: c_uint = 0xf2010001;
// RCCFGL
pub const R367TER_RCCFGL: c_uint = 0xf202;
pub const F367TER_TSRCFIFO_BCLKDEL1cK: c_uint = 0xf20200c0;
pub const F367TER_RCCFGL_5: c_uint = 0xf2020020;
pub const F367TER_TSRCFIFO_DUTY50: c_uint = 0xf2020010;
pub const F367TER_TSRCFIFO_NSGNL2dATA: c_uint = 0xf2020008;
pub const F367TER_TSRCFIFO_DISSERMUX: c_uint = 0xf2020004;
pub const F367TER_RCCFGL_1: c_uint = 0xf2020002;
pub const F367TER_TSRCFIFO_STOPCKDIS: c_uint = 0xf2020001;
// RCINSDELH
pub const R367TER_RCINSDELH: c_uint = 0xf203;
pub const F367TER_TSRCDEL_SYNCBYTE: c_uint = 0xf2030080;
pub const F367TER_TSRCDEL_XXHEADER: c_uint = 0xf2030040;
pub const F367TER_TSRCDEL_BBHEADER: c_uint = 0xf2030020;
pub const F367TER_TSRCDEL_DATAFIELD: c_uint = 0xf2030010;
pub const F367TER_TSRCINSDEL_ISCR: c_uint = 0xf2030008;
pub const F367TER_TSRCINSDEL_NPD: c_uint = 0xf2030004;
pub const F367TER_TSRCINSDEL_RSPARITY: c_uint = 0xf2030002;
pub const F367TER_TSRCINSDEL_CRC8: c_uint = 0xf2030001;
// RCINSDELM
pub const R367TER_RCINSDELM: c_uint = 0xf204;
pub const F367TER_TSRCINS_BBPADDING: c_uint = 0xf2040080;
pub const F367TER_TSRCINS_BCHFEC: c_uint = 0xf2040040;
pub const F367TER_TSRCINS_LDPCFEC: c_uint = 0xf2040020;
pub const F367TER_TSRCINS_EMODCOD: c_uint = 0xf2040010;
pub const F367TER_TSRCINS_TOKEN: c_uint = 0xf2040008;
pub const F367TER_TSRCINS_XXXERR: c_uint = 0xf2040004;
pub const F367TER_TSRCINS_MATYPE: c_uint = 0xf2040002;
pub const F367TER_TSRCINS_UPL: c_uint = 0xf2040001;
// RCINSDELL
pub const R367TER_RCINSDELL: c_uint = 0xf205;
pub const F367TER_TSRCINS_DFL: c_uint = 0xf2050080;
pub const F367TER_TSRCINS_SYNCD: c_uint = 0xf2050040;
pub const F367TER_TSRCINS_BLOCLEN: c_uint = 0xf2050020;
pub const F367TER_TSRCINS_SIGPCOUNT: c_uint = 0xf2050010;
pub const F367TER_TSRCINS_FIFO: c_uint = 0xf2050008;
pub const F367TER_TSRCINS_REALPACK: c_uint = 0xf2050004;
pub const F367TER_TSRCINS_TSCONFIG: c_uint = 0xf2050002;
pub const F367TER_TSRCINS_LATENCY: c_uint = 0xf2050001;
// RCSTATUS
pub const R367TER_RCSTATUS: c_uint = 0xf206;
pub const F367TER_TSRCFIFO_LINEOK: c_uint = 0xf2060080;
pub const F367TER_TSRCFIFO_ERROR: c_uint = 0xf2060040;
pub const F367TER_TSRCFIFO_DATA7: c_uint = 0xf2060020;
pub const F367TER_RCSTATUS_4: c_uint = 0xf2060010;
pub const F367TER_TSRCFIFO_DEMODSEL: c_uint = 0xf2060008;
pub const F367TER_TSRC1FIFOSPEED_STORE: c_uint = 0xf2060004;
pub const F367TER_RCSTATUS_1: c_uint = 0xf2060002;
pub const F367TER_TSRCSERIAL_IMPOSSIBLE: c_uint = 0xf2060001;
// RCSPEED
pub const R367TER_RCSPEED: c_uint = 0xf207;
pub const F367TER_TSRCFIFO_OUTSPEED: c_uint = 0xf20700ff;
// RCDEBUGM
pub const R367TER_RCDEBUGM: c_uint = 0xf208;
pub const F367TER_SD_UNSYNC: c_uint = 0xf2080080;
pub const F367TER_ULFLOCK_DETECTM: c_uint = 0xf2080040;
pub const F367TER_SUL_SELECTOS: c_uint = 0xf2080020;
pub const F367TER_DILUL_NOSCRBLE: c_uint = 0xf2080010;
pub const F367TER_NUL_SCRB: c_uint = 0xf2080008;
pub const F367TER_UL_SCRB: c_uint = 0xf2080004;
pub const F367TER_SCRAULBAD: c_uint = 0xf2080002;
pub const F367TER_SCRAUL_UNSYNC: c_uint = 0xf2080001;
// RCDEBUGL
pub const R367TER_RCDEBUGL: c_uint = 0xf209;
pub const F367TER_RS_ERR: c_uint = 0xf2090080;
pub const F367TER_LLFLOCK_DETECTM: c_uint = 0xf2090040;
pub const F367TER_NOT_SUL_SELECTOS: c_uint = 0xf2090020;
pub const F367TER_DILLL_NOSCRBLE: c_uint = 0xf2090010;
pub const F367TER_NLL_SCRB: c_uint = 0xf2090008;
pub const F367TER_LL_SCRB: c_uint = 0xf2090004;
pub const F367TER_SCRALLBAD: c_uint = 0xf2090002;
pub const F367TER_SCRALL_UNSYNC: c_uint = 0xf2090001;
// RCOBSCFG
pub const R367TER_RCOBSCFG: c_uint = 0xf20a;
pub const F367TER_TSRCFIFO_OBSCFG: c_uint = 0xf20a00ff;
// RCOBSM
pub const R367TER_RCOBSM: c_uint = 0xf20b;
pub const F367TER_TSRCFIFO_OBSDATA_HI: c_uint = 0xf20b00ff;
// RCOBSL
pub const R367TER_RCOBSL: c_uint = 0xf20c;
pub const F367TER_TSRCFIFO_OBSDATA_LO: c_uint = 0xf20c00ff;
// RCFECSPY
pub const R367TER_RCFECSPY: c_uint = 0xf210;
pub const F367TER_SPYRC_ENABLE: c_uint = 0xf2100080;
pub const F367TER_RCNO_SYNCBYTE: c_uint = 0xf2100040;
pub const F367TER_RCSERIAL_MODE: c_uint = 0xf2100020;
pub const F367TER_RCUNUSUAL_PACKET: c_uint = 0xf2100010;
pub const F367TER_BERRCMETER_DATAMODE: c_uint = 0xf210000c;
pub const F367TER_BERRCMETER_LMODE: c_uint = 0xf2100002;
pub const F367TER_BERRCMETER_RESET: c_uint = 0xf2100001;
// RCFSPYCFG
pub const R367TER_RCFSPYCFG: c_uint = 0xf211;
pub const F367TER_FECSPYRC_INPUT: c_uint = 0xf21100c0;
pub const F367TER_RCRST_ON_ERROR: c_uint = 0xf2110020;
pub const F367TER_RCONE_SHOT: c_uint = 0xf2110010;
pub const F367TER_RCI2C_MODE: c_uint = 0xf211000c;
pub const F367TER_SPYRC_HSTERESIS: c_uint = 0xf2110003;
// RCFSPYDATA
pub const R367TER_RCFSPYDATA: c_uint = 0xf212;
pub const F367TER_SPYRC_STUFFING: c_uint = 0xf2120080;
pub const F367TER_RCNOERR_PKTJITTER: c_uint = 0xf2120040;
pub const F367TER_SPYRC_CNULLPKT: c_uint = 0xf2120020;
pub const F367TER_SPYRC_OUTDATA_MODE: c_uint = 0xf212001f;
// RCFSPYOUT
pub const R367TER_RCFSPYOUT: c_uint = 0xf213;
pub const F367TER_FSPYRC_DIRECT: c_uint = 0xf2130080;
pub const F367TER_RCFSPYOUT_6: c_uint = 0xf2130040;
pub const F367TER_SPYRC_OUTDATA_BUS: c_uint = 0xf2130038;
pub const F367TER_RCSTUFF_MODE: c_uint = 0xf2130007;
// RCFSTATUS
pub const R367TER_RCFSTATUS: c_uint = 0xf214;
pub const F367TER_SPYRC_ENDSIM: c_uint = 0xf2140080;
pub const F367TER_RCVALID_SIM: c_uint = 0xf2140040;
pub const F367TER_RCFOUND_SIGNAL: c_uint = 0xf2140020;
pub const F367TER_RCDSS_SYNCBYTE: c_uint = 0xf2140010;
pub const F367TER_RCRESULT_STATE: c_uint = 0xf214000f;
// RCFGOODPACK
pub const R367TER_RCFGOODPACK: c_uint = 0xf215;
pub const F367TER_RCGOOD_PACKET: c_uint = 0xf21500ff;
// RCFPACKCNT
pub const R367TER_RCFPACKCNT: c_uint = 0xf216;
pub const F367TER_RCPACKET_COUNTER: c_uint = 0xf21600ff;
// RCFSPYMISC
pub const R367TER_RCFSPYMISC: c_uint = 0xf217;
pub const F367TER_RCLABEL_COUNTER: c_uint = 0xf21700ff;
// RCFBERCPT4
pub const R367TER_RCFBERCPT4: c_uint = 0xf218;
pub const F367TER_FBERRCMETER_CPT_MMMMSB: c_uint = 0xf21800ff;
// RCFBERCPT3
pub const R367TER_RCFBERCPT3: c_uint = 0xf219;
pub const F367TER_FBERRCMETER_CPT_MMMSB: c_uint = 0xf21900ff;
// RCFBERCPT2
pub const R367TER_RCFBERCPT2: c_uint = 0xf21a;
pub const F367TER_FBERRCMETER_CPT_MMSB: c_uint = 0xf21a00ff;
// RCFBERCPT1
pub const R367TER_RCFBERCPT1: c_uint = 0xf21b;
pub const F367TER_FBERRCMETER_CPT_MSB: c_uint = 0xf21b00ff;
// RCFBERCPT0
pub const R367TER_RCFBERCPT0: c_uint = 0xf21c;
pub const F367TER_FBERRCMETER_CPT_LSB: c_uint = 0xf21c00ff;
// RCFBERERR2
pub const R367TER_RCFBERERR2: c_uint = 0xf21d;
pub const F367TER_FBERRCMETER_ERR_HI: c_uint = 0xf21d00ff;
// RCFBERERR1
pub const R367TER_RCFBERERR1: c_uint = 0xf21e;
pub const F367TER_FBERRCMETER_ERR: c_uint = 0xf21e00ff;
// RCFBERERR0
pub const R367TER_RCFBERERR0: c_uint = 0xf21f;
pub const F367TER_FBERRCMETER_ERR_LO: c_uint = 0xf21f00ff;
// RCFSTATESM
pub const R367TER_RCFSTATESM: c_uint = 0xf220;
pub const F367TER_RCRSTATE_F: c_uint = 0xf2200080;
pub const F367TER_RCRSTATE_E: c_uint = 0xf2200040;
pub const F367TER_RCRSTATE_D: c_uint = 0xf2200020;
pub const F367TER_RCRSTATE_C: c_uint = 0xf2200010;
pub const F367TER_RCRSTATE_B: c_uint = 0xf2200008;
pub const F367TER_RCRSTATE_A: c_uint = 0xf2200004;
pub const F367TER_RCRSTATE_9: c_uint = 0xf2200002;
pub const F367TER_RCRSTATE_8: c_uint = 0xf2200001;
// RCFSTATESL
pub const R367TER_RCFSTATESL: c_uint = 0xf221;
pub const F367TER_RCRSTATE_7: c_uint = 0xf2210080;
pub const F367TER_RCRSTATE_6: c_uint = 0xf2210040;
pub const F367TER_RCRSTATE_5: c_uint = 0xf2210020;
pub const F367TER_RCRSTATE_4: c_uint = 0xf2210010;
pub const F367TER_RCRSTATE_3: c_uint = 0xf2210008;
pub const F367TER_RCRSTATE_2: c_uint = 0xf2210004;
pub const F367TER_RCRSTATE_1: c_uint = 0xf2210002;
pub const F367TER_RCRSTATE_0: c_uint = 0xf2210001;
// RCFSPYBER
pub const R367TER_RCFSPYBER: c_uint = 0xf222;
pub const F367TER_RCFSPYBER_7: c_uint = 0xf2220080;
pub const F367TER_SPYRCOBS_XORREAD: c_uint = 0xf2220040;
pub const F367TER_FSPYRCBER_OBSMODE: c_uint = 0xf2220020;
pub const F367TER_FSPYRCBER_SYNCBYT: c_uint = 0xf2220010;
pub const F367TER_FSPYRCBER_UNSYNC: c_uint = 0xf2220008;
pub const F367TER_FSPYRCBER_CTIME: c_uint = 0xf2220007;
// RCFSPYDISTM
pub const R367TER_RCFSPYDISTM: c_uint = 0xf223;
pub const F367TER_RCPKTTIME_DISTANCE_HI: c_uint = 0xf22300ff;
// RCFSPYDISTL
pub const R367TER_RCFSPYDISTL: c_uint = 0xf224;
pub const F367TER_RCPKTTIME_DISTANCE_LO: c_uint = 0xf22400ff;
// RCFSPYOBS7
pub const R367TER_RCFSPYOBS7: c_uint = 0xf228;
pub const F367TER_RCSPYOBS_SPYFAIL: c_uint = 0xf2280080;
pub const F367TER_RCSPYOBS_SPYFAIL1: c_uint = 0xf2280040;
pub const F367TER_RCSPYOBS_ERROR: c_uint = 0xf2280020;
pub const F367TER_RCSPYOBS_STROUT: c_uint = 0xf2280010;
pub const F367TER_RCSPYOBS_RESULTSTATE1: c_uint = 0xf228000f;
// RCFSPYOBS6
pub const R367TER_RCFSPYOBS6: c_uint = 0xf229;
pub const F367TER_RCSPYOBS_RESULTSTATe0: c_uint = 0xf22900f0;
pub const F367TER_RCSPYOBS_RESULTSTATEM1: c_uint = 0xf229000f;
// RCFSPYOBS5
pub const R367TER_RCFSPYOBS5: c_uint = 0xf22a;
pub const F367TER_RCSPYOBS_BYTEOFPACKET1: c_uint = 0xf22a00ff;
// RCFSPYOBS4
pub const R367TER_RCFSPYOBS4: c_uint = 0xf22b;
pub const F367TER_RCSPYOBS_BYTEVALUE1: c_uint = 0xf22b00ff;
// RCFSPYOBS3
pub const R367TER_RCFSPYOBS3: c_uint = 0xf22c;
pub const F367TER_RCSPYOBS_DATA1: c_uint = 0xf22c00ff;
// RCFSPYOBS2
pub const R367TER_RCFSPYOBS2: c_uint = 0xf22d;
pub const F367TER_RCSPYOBS_DATa0: c_uint = 0xf22d00ff;
// RCFSPYOBS1
pub const R367TER_RCFSPYOBS1: c_uint = 0xf22e;
pub const F367TER_RCSPYOBS_DATAM1: c_uint = 0xf22e00ff;
// RCFSPYOBS0
pub const R367TER_RCFSPYOBS0: c_uint = 0xf22f;
pub const F367TER_RCSPYOBS_DATAM2: c_uint = 0xf22f00ff;
// TSGENERAL
pub const R367TER_TSGENERAL: c_uint = 0xf230;
pub const F367TER_TSGENERAL_7: c_uint = 0xf2300080;
pub const F367TER_TSGENERAL_6: c_uint = 0xf2300040;
pub const F367TER_TSFIFO_BCLK1aLL: c_uint = 0xf2300020;
pub const F367TER_TSGENERAL_4: c_uint = 0xf2300010;
pub const F367TER_MUXSTREAM_OUTMODE: c_uint = 0xf2300008;
pub const F367TER_TSFIFO_PERMPARAL: c_uint = 0xf2300006;
pub const F367TER_RST_REEDSOLO: c_uint = 0xf2300001;
// RC1SPEED
pub const R367TER_RC1SPEED: c_uint = 0xf231;
pub const F367TER_TSRCFIFO1_OUTSPEED: c_uint = 0xf23100ff;
// TSGSTATUS
pub const R367TER_TSGSTATUS: c_uint = 0xf232;
pub const F367TER_TSGSTATUS_7: c_uint = 0xf2320080;
pub const F367TER_TSGSTATUS_6: c_uint = 0xf2320040;
pub const F367TER_RSMEM_FULL: c_uint = 0xf2320020;
pub const F367TER_RS_MULTCALC: c_uint = 0xf2320010;
pub const F367TER_RSIN_OVERTIME: c_uint = 0xf2320008;
pub const F367TER_TSFIFO3_DEMODSEL: c_uint = 0xf2320004;
pub const F367TER_TSFIFO2_DEMODSEL: c_uint = 0xf2320002;
pub const F367TER_TSFIFO1_DEMODSEL: c_uint = 0xf2320001;
// FECM
pub const R367TER_FECM: c_uint = 0xf233;
pub const F367TER_DSS_DVB: c_uint = 0xf2330080;
pub const F367TER_DEMOD_BYPASS: c_uint = 0xf2330040;
pub const F367TER_CMP_SLOWMODE: c_uint = 0xf2330020;
pub const F367TER_DSS_SRCH: c_uint = 0xf2330010;
pub const F367TER_FECM_3: c_uint = 0xf2330008;
pub const F367TER_DIFF_MODEVIT: c_uint = 0xf2330004;
pub const F367TER_SYNCVIT: c_uint = 0xf2330002;
pub const F367TER_I2CSYM: c_uint = 0xf2330001;
// VTH12
pub const R367TER_VTH12: c_uint = 0xf234;
pub const F367TER_VTH_12: c_uint = 0xf23400ff;
// VTH23
pub const R367TER_VTH23: c_uint = 0xf235;
pub const F367TER_VTH_23: c_uint = 0xf23500ff;
// VTH34
pub const R367TER_VTH34: c_uint = 0xf236;
pub const F367TER_VTH_34: c_uint = 0xf23600ff;
// VTH56
pub const R367TER_VTH56: c_uint = 0xf237;
pub const F367TER_VTH_56: c_uint = 0xf23700ff;
// VTH67
pub const R367TER_VTH67: c_uint = 0xf238;
pub const F367TER_VTH_67: c_uint = 0xf23800ff;
// VTH78
pub const R367TER_VTH78: c_uint = 0xf239;
pub const F367TER_VTH_78: c_uint = 0xf23900ff;
// VITCURPUN
pub const R367TER_VITCURPUN: c_uint = 0xf23a;
pub const F367TER_VIT_MAPPING: c_uint = 0xf23a00e0;
pub const F367TER_VIT_CURPUN: c_uint = 0xf23a001f;
// VERROR
pub const R367TER_VERROR: c_uint = 0xf23b;
pub const F367TER_REGERR_VIT: c_uint = 0xf23b00ff;
// PRVIT
pub const R367TER_PRVIT: c_uint = 0xf23c;
pub const F367TER_PRVIT_7: c_uint = 0xf23c0080;
pub const F367TER_DIS_VTHLOCK: c_uint = 0xf23c0040;
pub const F367TER_E7_8VIT: c_uint = 0xf23c0020;
pub const F367TER_E6_7VIT: c_uint = 0xf23c0010;
pub const F367TER_E5_6VIT: c_uint = 0xf23c0008;
pub const F367TER_E3_4VIT: c_uint = 0xf23c0004;
pub const F367TER_E2_3VIT: c_uint = 0xf23c0002;
pub const F367TER_E1_2VIT: c_uint = 0xf23c0001;
// VAVSRVIT
pub const R367TER_VAVSRVIT: c_uint = 0xf23d;
pub const F367TER_AMVIT: c_uint = 0xf23d0080;
pub const F367TER_FROZENVIT: c_uint = 0xf23d0040;
pub const F367TER_SNVIT: c_uint = 0xf23d0030;
pub const F367TER_TOVVIT: c_uint = 0xf23d000c;
pub const F367TER_HYPVIT: c_uint = 0xf23d0003;
// VSTATUSVIT
pub const R367TER_VSTATUSVIT: c_uint = 0xf23e;
pub const F367TER_VITERBI_ON: c_uint = 0xf23e0080;
pub const F367TER_END_LOOPVIT: c_uint = 0xf23e0040;
pub const F367TER_VITERBI_DEPRF: c_uint = 0xf23e0020;
pub const F367TER_PRFVIT: c_uint = 0xf23e0010;
pub const F367TER_LOCKEDVIT: c_uint = 0xf23e0008;
pub const F367TER_VITERBI_DELOCK: c_uint = 0xf23e0004;
pub const F367TER_VIT_DEMODSEL: c_uint = 0xf23e0002;
pub const F367TER_VITERBI_COMPOUT: c_uint = 0xf23e0001;
// VTHINUSE
pub const R367TER_VTHINUSE: c_uint = 0xf23f;
pub const F367TER_VIT_INUSE: c_uint = 0xf23f00ff;
// KDIV12
pub const R367TER_KDIV12: c_uint = 0xf240;
pub const F367TER_KDIV12_MANUAL: c_uint = 0xf2400080;
pub const F367TER_K_DIVIDER_12: c_uint = 0xf240007f;
// KDIV23
pub const R367TER_KDIV23: c_uint = 0xf241;
pub const F367TER_KDIV23_MANUAL: c_uint = 0xf2410080;
pub const F367TER_K_DIVIDER_23: c_uint = 0xf241007f;
// KDIV34
pub const R367TER_KDIV34: c_uint = 0xf242;
pub const F367TER_KDIV34_MANUAL: c_uint = 0xf2420080;
pub const F367TER_K_DIVIDER_34: c_uint = 0xf242007f;
// KDIV56
pub const R367TER_KDIV56: c_uint = 0xf243;
pub const F367TER_KDIV56_MANUAL: c_uint = 0xf2430080;
pub const F367TER_K_DIVIDER_56: c_uint = 0xf243007f;
// KDIV67
pub const R367TER_KDIV67: c_uint = 0xf244;
pub const F367TER_KDIV67_MANUAL: c_uint = 0xf2440080;
pub const F367TER_K_DIVIDER_67: c_uint = 0xf244007f;
// KDIV78
pub const R367TER_KDIV78: c_uint = 0xf245;
pub const F367TER_KDIV78_MANUAL: c_uint = 0xf2450080;
pub const F367TER_K_DIVIDER_78: c_uint = 0xf245007f;
// SIGPOWER
pub const R367TER_SIGPOWER: c_uint = 0xf246;
pub const F367TER_SIGPOWER_MANUAL: c_uint = 0xf2460080;
pub const F367TER_SIG_POWER: c_uint = 0xf246007f;
// DEMAPVIT
pub const R367TER_DEMAPVIT: c_uint = 0xf247;
pub const F367TER_DEMAPVIT_7: c_uint = 0xf2470080;
pub const F367TER_K_DIVIDER_VIT: c_uint = 0xf247007f;
// VITSCALE
pub const R367TER_VITSCALE: c_uint = 0xf248;
pub const F367TER_NVTH_NOSRANGE: c_uint = 0xf2480080;
pub const F367TER_VERROR_MAXMODE: c_uint = 0xf2480040;
pub const F367TER_KDIV_MODE: c_uint = 0xf2480030;
pub const F367TER_NSLOWSN_LOCKED: c_uint = 0xf2480008;
pub const F367TER_DELOCK_PRFLOSS: c_uint = 0xf2480004;
pub const F367TER_DIS_RSFLOCK: c_uint = 0xf2480002;
pub const F367TER_VITSCALE_0: c_uint = 0xf2480001;
// FFEC1PRG
pub const R367TER_FFEC1PRG: c_uint = 0xf249;
pub const F367TER_FDSS_DVB: c_uint = 0xf2490080;
pub const F367TER_FDSS_SRCH: c_uint = 0xf2490040;
pub const F367TER_FFECPROG_5: c_uint = 0xf2490020;
pub const F367TER_FFECPROG_4: c_uint = 0xf2490010;
pub const F367TER_FFECPROG_3: c_uint = 0xf2490008;
pub const F367TER_FFECPROG_2: c_uint = 0xf2490004;
pub const F367TER_FTS1_DISABLE: c_uint = 0xf2490002;
pub const F367TER_FTS2_DISABLE: c_uint = 0xf2490001;
// FVITCURPUN
pub const R367TER_FVITCURPUN: c_uint = 0xf24a;
pub const F367TER_FVIT_MAPPING: c_uint = 0xf24a00e0;
pub const F367TER_FVIT_CURPUN: c_uint = 0xf24a001f;
// FVERROR
pub const R367TER_FVERROR: c_uint = 0xf24b;
pub const F367TER_FREGERR_VIT: c_uint = 0xf24b00ff;
// FVSTATUSVIT
pub const R367TER_FVSTATUSVIT: c_uint = 0xf24c;
pub const F367TER_FVITERBI_ON: c_uint = 0xf24c0080;
pub const F367TER_F1END_LOOPVIT: c_uint = 0xf24c0040;
pub const F367TER_FVITERBI_DEPRF: c_uint = 0xf24c0020;
pub const F367TER_FPRFVIT: c_uint = 0xf24c0010;
pub const F367TER_FLOCKEDVIT: c_uint = 0xf24c0008;
pub const F367TER_FVITERBI_DELOCK: c_uint = 0xf24c0004;
pub const F367TER_FVIT_DEMODSEL: c_uint = 0xf24c0002;
pub const F367TER_FVITERBI_COMPOUT: c_uint = 0xf24c0001;
// DEBUG_LT1
pub const R367TER_DEBUG_LT1: c_uint = 0xf24d;
pub const F367TER_DBG_LT1: c_uint = 0xf24d00ff;
// DEBUG_LT2
pub const R367TER_DEBUG_LT2: c_uint = 0xf24e;
pub const F367TER_DBG_LT2: c_uint = 0xf24e00ff;
// DEBUG_LT3
pub const R367TER_DEBUG_LT3: c_uint = 0xf24f;
pub const F367TER_DBG_LT3: c_uint = 0xf24f00ff;
// TSTSFMET
pub const R367TER_TSTSFMET: c_uint = 0xf250;
pub const F367TER_TSTSFEC_METRIQUES: c_uint = 0xf25000ff;
// SELOUT
pub const R367TER_SELOUT: c_uint = 0xf252;
pub const F367TER_EN_SYNC: c_uint = 0xf2520080;
pub const F367TER_EN_TBUSDEMAP: c_uint = 0xf2520040;
pub const F367TER_SELOUT_5: c_uint = 0xf2520020;
pub const F367TER_SELOUT_4: c_uint = 0xf2520010;
pub const F367TER_TSTSYNCHRO_MODE: c_uint = 0xf2520002;
// TSYNC
pub const R367TER_TSYNC: c_uint = 0xf253;
pub const F367TER_CURPUN_INCMODE: c_uint = 0xf2530080;
pub const F367TER_CERR_TSTMODE: c_uint = 0xf2530040;
pub const F367TER_SHIFTSOF_MODE: c_uint = 0xf2530030;
pub const F367TER_SLOWPHA_MODE: c_uint = 0xf2530008;
pub const F367TER_PXX_BYPALL: c_uint = 0xf2530004;
pub const F367TER_FROTA45_FIRST: c_uint = 0xf2530002;
pub const F367TER_TST_BCHERROR: c_uint = 0xf2530001;
// TSTERR
pub const R367TER_TSTERR: c_uint = 0xf254;
pub const F367TER_TST_LONGPKT: c_uint = 0xf2540080;
pub const F367TER_TST_ISSYION: c_uint = 0xf2540040;
pub const F367TER_TST_NPDON: c_uint = 0xf2540020;
pub const F367TER_TSTERR_4: c_uint = 0xf2540010;
pub const F367TER_TRACEBACK_MODE: c_uint = 0xf2540008;
pub const F367TER_TST_RSPARITY: c_uint = 0xf2540004;
pub const F367TER_METRIQUE_MODE: c_uint = 0xf2540003;
// TSFSYNC
pub const R367TER_TSFSYNC: c_uint = 0xf255;
pub const F367TER_EN_SFECSYNC: c_uint = 0xf2550080;
pub const F367TER_EN_SFECDEMAP: c_uint = 0xf2550040;
pub const F367TER_SFCERR_TSTMODE: c_uint = 0xf2550020;
pub const F367TER_SFECPXX_BYPALL: c_uint = 0xf2550010;
pub const F367TER_SFECTSTSYNCHRO_MODE: c_uint = 0xf255000f;
// TSTSFERR
pub const R367TER_TSTSFERR: c_uint = 0xf256;
pub const F367TER_TSTSTERR_7: c_uint = 0xf2560080;
pub const F367TER_TSTSTERR_6: c_uint = 0xf2560040;
pub const F367TER_TSTSTERR_5: c_uint = 0xf2560020;
pub const F367TER_TSTSTERR_4: c_uint = 0xf2560010;
pub const F367TER_SFECTRACEBACK_MODE: c_uint = 0xf2560008;
pub const F367TER_SFEC_NCONVPROG: c_uint = 0xf2560004;
pub const F367TER_SFECMETRIQUE_MODE: c_uint = 0xf2560003;
// TSTTSSF1
pub const R367TER_TSTTSSF1: c_uint = 0xf258;
pub const F367TER_TSTERSSF: c_uint = 0xf2580080;
pub const F367TER_TSTTSSFEN: c_uint = 0xf2580040;
pub const F367TER_SFEC_OUTMODE: c_uint = 0xf2580030;
pub const F367TER_XLSF_NOFTHRESHOLD: c_uint = 0xf2580008;
pub const F367TER_TSTTSSF_STACKSEL: c_uint = 0xf2580007;
// TSTTSSF2
pub const R367TER_TSTTSSF2: c_uint = 0xf259;
pub const F367TER_DILSF_DBBHEADER: c_uint = 0xf2590080;
pub const F367TER_TSTTSSF_DISBUG: c_uint = 0xf2590040;
pub const F367TER_TSTTSSF_NOBADSTART: c_uint = 0xf2590020;
pub const F367TER_TSTTSSF_SELECT: c_uint = 0xf259001f;
// TSTTSSF3
pub const R367TER_TSTTSSF3: c_uint = 0xf25a;
pub const F367TER_TSTTSSF3_7: c_uint = 0xf25a0080;
pub const F367TER_TSTTSSF3_6: c_uint = 0xf25a0040;
pub const F367TER_TSTTSSF3_5: c_uint = 0xf25a0020;
pub const F367TER_TSTTSSF3_4: c_uint = 0xf25a0010;
pub const F367TER_TSTTSSF3_3: c_uint = 0xf25a0008;
pub const F367TER_TSTTSSF3_2: c_uint = 0xf25a0004;
pub const F367TER_TSTTSSF3_1: c_uint = 0xf25a0002;
pub const F367TER_DISSF_CLKENABLE: c_uint = 0xf25a0001;
// TSTTS1
pub const R367TER_TSTTS1: c_uint = 0xf25c;
pub const F367TER_TSTERS: c_uint = 0xf25c0080;
pub const F367TER_TSFIFO_DSSSYNCB: c_uint = 0xf25c0040;
pub const F367TER_TSTTS_FSPYBEFRS: c_uint = 0xf25c0020;
pub const F367TER_NFORCE_SYNCBYTE: c_uint = 0xf25c0010;
pub const F367TER_XL_NOFTHRESHOLD: c_uint = 0xf25c0008;
pub const F367TER_TSTTS_FRFORCEPKT: c_uint = 0xf25c0004;
pub const F367TER_DESCR_NOTAUTO: c_uint = 0xf25c0002;
pub const F367TER_TSTTSEN: c_uint = 0xf25c0001;
// TSTTS2
pub const R367TER_TSTTS2: c_uint = 0xf25d;
pub const F367TER_DIL_DBBHEADER: c_uint = 0xf25d0080;
pub const F367TER_TSTTS_NOBADXXX: c_uint = 0xf25d0040;
pub const F367TER_TSFIFO_DELSPEEDUP: c_uint = 0xf25d0020;
pub const F367TER_TSTTS_SELECT: c_uint = 0xf25d001f;
// TSTTS3
pub const R367TER_TSTTS3: c_uint = 0xf25e;
pub const F367TER_TSTTS_NOPKTGAIN: c_uint = 0xf25e0080;
pub const F367TER_TSTTS_NOPKTENE: c_uint = 0xf25e0040;
pub const F367TER_TSTTS_ISOLATION: c_uint = 0xf25e0020;
pub const F367TER_TSTTS_DISBUG: c_uint = 0xf25e0010;
pub const F367TER_TSTTS_NOBADSTART: c_uint = 0xf25e0008;
pub const F367TER_TSTTS_STACKSEL: c_uint = 0xf25e0007;
// TSTTS4
pub const R367TER_TSTTS4: c_uint = 0xf25f;
pub const F367TER_TSTTS4_7: c_uint = 0xf25f0080;
pub const F367TER_TSTTS4_6: c_uint = 0xf25f0040;
pub const F367TER_TSTTS4_5: c_uint = 0xf25f0020;
pub const F367TER_TSTTS_DISDSTATE: c_uint = 0xf25f0010;
pub const F367TER_TSTTS_FASTNOSYNC: c_uint = 0xf25f0008;
pub const F367TER_EXT_FECSPYIN: c_uint = 0xf25f0004;
pub const F367TER_TSTTS_NODPZERO: c_uint = 0xf25f0002;
pub const F367TER_TSTTS_NODIV3: c_uint = 0xf25f0001;
// TSTTSRC
pub const R367TER_TSTTSRC: c_uint = 0xf26c;
pub const F367TER_TSTTSRC_7: c_uint = 0xf26c0080;
pub const F367TER_TSRCFIFO_DSSSYNCB: c_uint = 0xf26c0040;
pub const F367TER_TSRCFIFO_DPUNACTIVE: c_uint = 0xf26c0020;
pub const F367TER_TSRCFIFO_DELSPEEDUP: c_uint = 0xf26c0010;
pub const F367TER_TSTTSRC_NODIV3: c_uint = 0xf26c0008;
pub const F367TER_TSTTSRC_FRFORCEPKT: c_uint = 0xf26c0004;
pub const F367TER_SAT25_SDDORIGINE: c_uint = 0xf26c0002;
pub const F367TER_TSTTSRC_INACTIVE: c_uint = 0xf26c0001;
// TSTTSRS
pub const R367TER_TSTTSRS: c_uint = 0xf26d;
pub const F367TER_TSTTSRS_7: c_uint = 0xf26d0080;
pub const F367TER_TSTTSRS_6: c_uint = 0xf26d0040;
pub const F367TER_TSTTSRS_5: c_uint = 0xf26d0020;
pub const F367TER_TSTTSRS_4: c_uint = 0xf26d0010;
pub const F367TER_TSTTSRS_3: c_uint = 0xf26d0008;
pub const F367TER_TSTTSRS_2: c_uint = 0xf26d0004;
pub const F367TER_TSTRS_DISRS2: c_uint = 0xf26d0002;
pub const F367TER_TSTRS_DISRS1: c_uint = 0xf26d0001;
// TSSTATEM
pub const R367TER_TSSTATEM: c_uint = 0xf270;
pub const F367TER_TSDIL_ON: c_uint = 0xf2700080;
pub const F367TER_TSSKIPRS_ON: c_uint = 0xf2700040;
pub const F367TER_TSRS_ON: c_uint = 0xf2700020;
pub const F367TER_TSDESCRAMB_ON: c_uint = 0xf2700010;
pub const F367TER_TSFRAME_MODE: c_uint = 0xf2700008;
pub const F367TER_TS_DISABLE: c_uint = 0xf2700004;
pub const F367TER_TSACM_MODE: c_uint = 0xf2700002;
pub const F367TER_TSOUT_NOSYNC: c_uint = 0xf2700001;
// TSSTATEL
pub const R367TER_TSSTATEL: c_uint = 0xf271;
pub const F367TER_TSNOSYNCBYTE: c_uint = 0xf2710080;
pub const F367TER_TSPARITY_ON: c_uint = 0xf2710040;
pub const F367TER_TSSYNCOUTRS_ON: c_uint = 0xf2710020;
pub const F367TER_TSDVBS2_MODE: c_uint = 0xf2710010;
pub const F367TER_TSISSYI_ON: c_uint = 0xf2710008;
pub const F367TER_TSNPD_ON: c_uint = 0xf2710004;
pub const F367TER_TSCRC8_ON: c_uint = 0xf2710002;
pub const F367TER_TSDSS_PACKET: c_uint = 0xf2710001;
// TSCFGH
pub const R367TER_TSCFGH: c_uint = 0xf272;
pub const F367TER_TSFIFO_DVBCI: c_uint = 0xf2720080;
pub const F367TER_TSFIFO_SERIAL: c_uint = 0xf2720040;
pub const F367TER_TSFIFO_TEIUPDATE: c_uint = 0xf2720020;
pub const F367TER_TSFIFO_DUTY50: c_uint = 0xf2720010;
pub const F367TER_TSFIFO_HSGNLOUT: c_uint = 0xf2720008;
pub const F367TER_TSFIFO_ERRMODE: c_uint = 0xf2720006;
pub const F367TER_RST_HWARE: c_uint = 0xf2720001;
// TSCFGM
pub const R367TER_TSCFGM: c_uint = 0xf273;
pub const F367TER_TSFIFO_MANSPEED: c_uint = 0xf27300c0;
pub const F367TER_TSFIFO_PERMDATA: c_uint = 0xf2730020;
pub const F367TER_TSFIFO_NONEWSGNL: c_uint = 0xf2730010;
pub const F367TER_TSFIFO_BITSPEED: c_uint = 0xf2730008;
pub const F367TER_NPD_SPECDVBS2: c_uint = 0xf2730004;
pub const F367TER_TSFIFO_STOPCKDIS: c_uint = 0xf2730002;
pub const F367TER_TSFIFO_INVDATA: c_uint = 0xf2730001;
// TSCFGL
pub const R367TER_TSCFGL: c_uint = 0xf274;
pub const F367TER_TSFIFO_BCLKDEL1cK: c_uint = 0xf27400c0;
pub const F367TER_BCHERROR_MODE: c_uint = 0xf2740030;
pub const F367TER_TSFIFO_NSGNL2dATA: c_uint = 0xf2740008;
pub const F367TER_TSFIFO_EMBINDVB: c_uint = 0xf2740004;
pub const F367TER_TSFIFO_DPUNACT: c_uint = 0xf2740002;
pub const F367TER_TSFIFO_NPDOFF: c_uint = 0xf2740001;
// TSSYNC
pub const R367TER_TSSYNC: c_uint = 0xf275;
pub const F367TER_TSFIFO_PERMUTE: c_uint = 0xf2750080;
pub const F367TER_TSFIFO_FISCR3B: c_uint = 0xf2750060;
pub const F367TER_TSFIFO_SYNCMODE: c_uint = 0xf2750018;
pub const F367TER_TSFIFO_SYNCSEL: c_uint = 0xf2750007;
// TSINSDELH
pub const R367TER_TSINSDELH: c_uint = 0xf276;
pub const F367TER_TSDEL_SYNCBYTE: c_uint = 0xf2760080;
pub const F367TER_TSDEL_XXHEADER: c_uint = 0xf2760040;
pub const F367TER_TSDEL_BBHEADER: c_uint = 0xf2760020;
pub const F367TER_TSDEL_DATAFIELD: c_uint = 0xf2760010;
pub const F367TER_TSINSDEL_ISCR: c_uint = 0xf2760008;
pub const F367TER_TSINSDEL_NPD: c_uint = 0xf2760004;
pub const F367TER_TSINSDEL_RSPARITY: c_uint = 0xf2760002;
pub const F367TER_TSINSDEL_CRC8: c_uint = 0xf2760001;
// TSINSDELM
pub const R367TER_TSINSDELM: c_uint = 0xf277;
pub const F367TER_TSINS_BBPADDING: c_uint = 0xf2770080;
pub const F367TER_TSINS_BCHFEC: c_uint = 0xf2770040;
pub const F367TER_TSINS_LDPCFEC: c_uint = 0xf2770020;
pub const F367TER_TSINS_EMODCOD: c_uint = 0xf2770010;
pub const F367TER_TSINS_TOKEN: c_uint = 0xf2770008;
pub const F367TER_TSINS_XXXERR: c_uint = 0xf2770004;
pub const F367TER_TSINS_MATYPE: c_uint = 0xf2770002;
pub const F367TER_TSINS_UPL: c_uint = 0xf2770001;
// TSINSDELL
pub const R367TER_TSINSDELL: c_uint = 0xf278;
pub const F367TER_TSINS_DFL: c_uint = 0xf2780080;
pub const F367TER_TSINS_SYNCD: c_uint = 0xf2780040;
pub const F367TER_TSINS_BLOCLEN: c_uint = 0xf2780020;
pub const F367TER_TSINS_SIGPCOUNT: c_uint = 0xf2780010;
pub const F367TER_TSINS_FIFO: c_uint = 0xf2780008;
pub const F367TER_TSINS_REALPACK: c_uint = 0xf2780004;
pub const F367TER_TSINS_TSCONFIG: c_uint = 0xf2780002;
pub const F367TER_TSINS_LATENCY: c_uint = 0xf2780001;
// TSDIVN
pub const R367TER_TSDIVN: c_uint = 0xf279;
pub const F367TER_TSFIFO_LOWSPEED: c_uint = 0xf2790080;
pub const F367TER_BYTE_OVERSAMPLING: c_uint = 0xf2790070;
pub const F367TER_TSMANUAL_PACKETNBR: c_uint = 0xf279000f;
// TSDIVPM
pub const R367TER_TSDIVPM: c_uint = 0xf27a;
pub const F367TER_TSMANUAL_P_HI: c_uint = 0xf27a00ff;
// TSDIVPL
pub const R367TER_TSDIVPL: c_uint = 0xf27b;
pub const F367TER_TSMANUAL_P_LO: c_uint = 0xf27b00ff;
// TSDIVQM
pub const R367TER_TSDIVQM: c_uint = 0xf27c;
pub const F367TER_TSMANUAL_Q_HI: c_uint = 0xf27c00ff;
// TSDIVQL
pub const R367TER_TSDIVQL: c_uint = 0xf27d;
pub const F367TER_TSMANUAL_Q_LO: c_uint = 0xf27d00ff;
// TSDILSTKM
pub const R367TER_TSDILSTKM: c_uint = 0xf27e;
pub const F367TER_TSFIFO_DILSTK_HI: c_uint = 0xf27e00ff;
// TSDILSTKL
pub const R367TER_TSDILSTKL: c_uint = 0xf27f;
pub const F367TER_TSFIFO_DILSTK_LO: c_uint = 0xf27f00ff;
// TSSPEED
pub const R367TER_TSSPEED: c_uint = 0xf280;
pub const F367TER_TSFIFO_OUTSPEED: c_uint = 0xf28000ff;
// TSSTATUS
pub const R367TER_TSSTATUS: c_uint = 0xf281;
pub const F367TER_TSFIFO_LINEOK: c_uint = 0xf2810080;
pub const F367TER_TSFIFO_ERROR: c_uint = 0xf2810040;
pub const F367TER_TSFIFO_DATA7: c_uint = 0xf2810020;
pub const F367TER_TSFIFO_NOSYNC: c_uint = 0xf2810010;
pub const F367TER_ISCR_INITIALIZED: c_uint = 0xf2810008;
pub const F367TER_ISCR_UPDATED: c_uint = 0xf2810004;
pub const F367TER_SOFFIFO_UNREGUL: c_uint = 0xf2810002;
pub const F367TER_DIL_READY: c_uint = 0xf2810001;
// TSSTATUS2
pub const R367TER_TSSTATUS2: c_uint = 0xf282;
pub const F367TER_TSFIFO_DEMODSEL: c_uint = 0xf2820080;
pub const F367TER_TSFIFOSPEED_STORE: c_uint = 0xf2820040;
pub const F367TER_DILXX_RESET: c_uint = 0xf2820020;
pub const F367TER_TSSERIAL_IMPOSSIBLE: c_uint = 0xf2820010;
pub const F367TER_TSFIFO_UNDERSPEED: c_uint = 0xf2820008;
pub const F367TER_BITSPEED_EVENT: c_uint = 0xf2820004;
pub const F367TER_UL_SCRAMBDETECT: c_uint = 0xf2820002;
pub const F367TER_ULDTV67_FALSELOCK: c_uint = 0xf2820001;
// TSBITRATEM
pub const R367TER_TSBITRATEM: c_uint = 0xf283;
pub const F367TER_TSFIFO_BITRATE_HI: c_uint = 0xf28300ff;
// TSBITRATEL
pub const R367TER_TSBITRATEL: c_uint = 0xf284;
pub const F367TER_TSFIFO_BITRATE_LO: c_uint = 0xf28400ff;
// TSPACKLENM
pub const R367TER_TSPACKLENM: c_uint = 0xf285;
pub const F367TER_TSFIFO_PACKCPT: c_uint = 0xf28500e0;
pub const F367TER_DIL_RPLEN_HI: c_uint = 0xf285001f;
// TSPACKLENL
pub const R367TER_TSPACKLENL: c_uint = 0xf286;
pub const F367TER_DIL_RPLEN_LO: c_uint = 0xf28600ff;
// TSBLOCLENM
pub const R367TER_TSBLOCLENM: c_uint = 0xf287;
pub const F367TER_TSFIFO_PFLEN_HI: c_uint = 0xf28700ff;
// TSBLOCLENL
pub const R367TER_TSBLOCLENL: c_uint = 0xf288;
pub const F367TER_TSFIFO_PFLEN_LO: c_uint = 0xf28800ff;
// TSDLYH
pub const R367TER_TSDLYH: c_uint = 0xf289;
pub const F367TER_SOFFIFO_TSTIMEVALID: c_uint = 0xf2890080;
pub const F367TER_SOFFIFO_SPEEDUP: c_uint = 0xf2890040;
pub const F367TER_SOFFIFO_STOP: c_uint = 0xf2890020;
pub const F367TER_SOFFIFO_REGULATED: c_uint = 0xf2890010;
pub const F367TER_SOFFIFO_REALSBOFF_HI: c_uint = 0xf289000f;
// TSDLYM
pub const R367TER_TSDLYM: c_uint = 0xf28a;
pub const F367TER_SOFFIFO_REALSBOFF_MED: c_uint = 0xf28a00ff;
// TSDLYL
pub const R367TER_TSDLYL: c_uint = 0xf28b;
pub const F367TER_SOFFIFO_REALSBOFF_LO: c_uint = 0xf28b00ff;
// TSNPDAV
pub const R367TER_TSNPDAV: c_uint = 0xf28c;
pub const F367TER_TSNPD_AVERAGE: c_uint = 0xf28c00ff;
// TSBUFSTATH
pub const R367TER_TSBUFSTATH: c_uint = 0xf28d;
pub const F367TER_TSISCR_3BYTES: c_uint = 0xf28d0080;
pub const F367TER_TSISCR_NEWDATA: c_uint = 0xf28d0040;
pub const F367TER_TSISCR_BUFSTAT_HI: c_uint = 0xf28d003f;
// TSBUFSTATM
pub const R367TER_TSBUFSTATM: c_uint = 0xf28e;
pub const F367TER_TSISCR_BUFSTAT_MED: c_uint = 0xf28e00ff;
// TSBUFSTATL
pub const R367TER_TSBUFSTATL: c_uint = 0xf28f;
pub const F367TER_TSISCR_BUFSTAT_LO: c_uint = 0xf28f00ff;
// TSDEBUGM
pub const R367TER_TSDEBUGM: c_uint = 0xf290;
pub const F367TER_TSFIFO_ILLPACKET: c_uint = 0xf2900080;
pub const F367TER_DIL_NOSYNC: c_uint = 0xf2900040;
pub const F367TER_DIL_ISCR: c_uint = 0xf2900020;
pub const F367TER_DILOUT_BSYNCB: c_uint = 0xf2900010;
pub const F367TER_TSFIFO_EMPTYPKT: c_uint = 0xf2900008;
pub const F367TER_TSFIFO_EMPTYRD: c_uint = 0xf2900004;
pub const F367TER_SOFFIFO_STOPM: c_uint = 0xf2900002;
pub const F367TER_SOFFIFO_SPEEDUPM: c_uint = 0xf2900001;
// TSDEBUGL
pub const R367TER_TSDEBUGL: c_uint = 0xf291;
pub const F367TER_TSFIFO_PACKLENFAIL: c_uint = 0xf2910080;
pub const F367TER_TSFIFO_SYNCBFAIL: c_uint = 0xf2910040;
pub const F367TER_TSFIFO_VITLIBRE: c_uint = 0xf2910020;
pub const F367TER_TSFIFO_BOOSTSPEEDM: c_uint = 0xf2910010;
pub const F367TER_TSFIFO_UNDERSPEEDM: c_uint = 0xf2910008;
pub const F367TER_TSFIFO_ERROR_EVNT: c_uint = 0xf2910004;
pub const F367TER_TSFIFO_FULL: c_uint = 0xf2910002;
pub const F367TER_TSFIFO_OVERFLOWM: c_uint = 0xf2910001;
// TSDLYSETH
pub const R367TER_TSDLYSETH: c_uint = 0xf292;
pub const F367TER_SOFFIFO_OFFSET: c_uint = 0xf29200e0;
pub const F367TER_SOFFIFO_SYMBOFFSET_HI: c_uint = 0xf292001f;
// TSDLYSETM
pub const R367TER_TSDLYSETM: c_uint = 0xf293;
pub const F367TER_SOFFIFO_SYMBOFFSET_MED: c_uint = 0xf29300ff;
// TSDLYSETL
pub const R367TER_TSDLYSETL: c_uint = 0xf294;
pub const F367TER_SOFFIFO_SYMBOFFSET_LO: c_uint = 0xf29400ff;
// TSOBSCFG
pub const R367TER_TSOBSCFG: c_uint = 0xf295;
pub const F367TER_TSFIFO_OBSCFG: c_uint = 0xf29500ff;
// TSOBSM
pub const R367TER_TSOBSM: c_uint = 0xf296;
pub const F367TER_TSFIFO_OBSDATA_HI: c_uint = 0xf29600ff;
// TSOBSL
pub const R367TER_TSOBSL: c_uint = 0xf297;
pub const F367TER_TSFIFO_OBSDATA_LO: c_uint = 0xf29700ff;
// ERRCTRL1
pub const R367TER_ERRCTRL1: c_uint = 0xf298;
pub const F367TER_ERR_SRC1: c_uint = 0xf29800f0;
pub const F367TER_ERRCTRL1_3: c_uint = 0xf2980008;
pub const F367TER_NUM_EVT1: c_uint = 0xf2980007;
// ERRCNT1H
pub const R367TER_ERRCNT1H: c_uint = 0xf299;
pub const F367TER_ERRCNT1_OLDVALUE: c_uint = 0xf2990080;
pub const F367TER_ERR_CNT1: c_uint = 0xf299007f;
// ERRCNT1M
pub const R367TER_ERRCNT1M: c_uint = 0xf29a;
pub const F367TER_ERR_CNT1_HI: c_uint = 0xf29a00ff;
// ERRCNT1L
pub const R367TER_ERRCNT1L: c_uint = 0xf29b;
pub const F367TER_ERR_CNT1_LO: c_uint = 0xf29b00ff;
// ERRCTRL2
pub const R367TER_ERRCTRL2: c_uint = 0xf29c;
pub const F367TER_ERR_SRC2: c_uint = 0xf29c00f0;
pub const F367TER_ERRCTRL2_3: c_uint = 0xf29c0008;
pub const F367TER_NUM_EVT2: c_uint = 0xf29c0007;
// ERRCNT2H
pub const R367TER_ERRCNT2H: c_uint = 0xf29d;
pub const F367TER_ERRCNT2_OLDVALUE: c_uint = 0xf29d0080;
pub const F367TER_ERR_CNT2_HI: c_uint = 0xf29d007f;
// ERRCNT2M
pub const R367TER_ERRCNT2M: c_uint = 0xf29e;
pub const F367TER_ERR_CNT2_MED: c_uint = 0xf29e00ff;
// ERRCNT2L
pub const R367TER_ERRCNT2L: c_uint = 0xf29f;
pub const F367TER_ERR_CNT2_LO: c_uint = 0xf29f00ff;
// FECSPY
pub const R367TER_FECSPY: c_uint = 0xf2a0;
pub const F367TER_SPY_ENABLE: c_uint = 0xf2a00080;
pub const F367TER_NO_SYNCBYTE: c_uint = 0xf2a00040;
pub const F367TER_SERIAL_MODE: c_uint = 0xf2a00020;
pub const F367TER_UNUSUAL_PACKET: c_uint = 0xf2a00010;
pub const F367TER_BERMETER_DATAMODE: c_uint = 0xf2a0000c;
pub const F367TER_BERMETER_LMODE: c_uint = 0xf2a00002;
pub const F367TER_BERMETER_RESET: c_uint = 0xf2a00001;
// FSPYCFG
pub const R367TER_FSPYCFG: c_uint = 0xf2a1;
pub const F367TER_FECSPY_INPUT: c_uint = 0xf2a100c0;
pub const F367TER_RST_ON_ERROR: c_uint = 0xf2a10020;
pub const F367TER_ONE_SHOT: c_uint = 0xf2a10010;
pub const F367TER_I2C_MOD: c_uint = 0xf2a1000c;
pub const F367TER_SPY_HYSTERESIS: c_uint = 0xf2a10003;
// FSPYDATA
pub const R367TER_FSPYDATA: c_uint = 0xf2a2;
pub const F367TER_SPY_STUFFING: c_uint = 0xf2a20080;
pub const F367TER_NOERROR_PKTJITTER: c_uint = 0xf2a20040;
pub const F367TER_SPY_CNULLPKT: c_uint = 0xf2a20020;
pub const F367TER_SPY_OUTDATA_MODE: c_uint = 0xf2a2001f;
// FSPYOUT
pub const R367TER_FSPYOUT: c_uint = 0xf2a3;
pub const F367TER_FSPY_DIRECT: c_uint = 0xf2a30080;
pub const F367TER_FSPYOUT_6: c_uint = 0xf2a30040;
pub const F367TER_SPY_OUTDATA_BUS: c_uint = 0xf2a30038;
pub const F367TER_STUFF_MODE: c_uint = 0xf2a30007;
// FSTATUS
pub const R367TER_FSTATUS: c_uint = 0xf2a4;
pub const F367TER_SPY_ENDSIM: c_uint = 0xf2a40080;
pub const F367TER_VALID_SIM: c_uint = 0xf2a40040;
pub const F367TER_FOUND_SIGNAL: c_uint = 0xf2a40020;
pub const F367TER_DSS_SYNCBYTE: c_uint = 0xf2a40010;
pub const F367TER_RESULT_STATE: c_uint = 0xf2a4000f;
// FGOODPACK
pub const R367TER_FGOODPACK: c_uint = 0xf2a5;
pub const F367TER_FGOOD_PACKET: c_uint = 0xf2a500ff;
// FPACKCNT
pub const R367TER_FPACKCNT: c_uint = 0xf2a6;
pub const F367TER_FPACKET_COUNTER: c_uint = 0xf2a600ff;
// FSPYMISC
pub const R367TER_FSPYMISC: c_uint = 0xf2a7;
pub const F367TER_FLABEL_COUNTER: c_uint = 0xf2a700ff;
// FBERCPT4
pub const R367TER_FBERCPT4: c_uint = 0xf2a8;
pub const F367TER_FBERMETER_CPT5: c_uint = 0xf2a800ff;
// FBERCPT3
pub const R367TER_FBERCPT3: c_uint = 0xf2a9;
pub const F367TER_FBERMETER_CPT4: c_uint = 0xf2a900ff;
// FBERCPT2
pub const R367TER_FBERCPT2: c_uint = 0xf2aa;
pub const F367TER_FBERMETER_CPT3: c_uint = 0xf2aa00ff;
// FBERCPT1
pub const R367TER_FBERCPT1: c_uint = 0xf2ab;
pub const F367TER_FBERMETER_CPT2: c_uint = 0xf2ab00ff;
// FBERCPT0
pub const R367TER_FBERCPT0: c_uint = 0xf2ac;
pub const F367TER_FBERMETER_CPT1: c_uint = 0xf2ac00ff;
// FBERERR2
pub const R367TER_FBERERR2: c_uint = 0xf2ad;
pub const F367TER_FBERMETER_ERR_HI: c_uint = 0xf2ad00ff;
// FBERERR1
pub const R367TER_FBERERR1: c_uint = 0xf2ae;
pub const F367TER_FBERMETER_ERR_MED: c_uint = 0xf2ae00ff;
// FBERERR0
pub const R367TER_FBERERR0: c_uint = 0xf2af;
pub const F367TER_FBERMETER_ERR_LO: c_uint = 0xf2af00ff;
// FSTATESM
pub const R367TER_FSTATESM: c_uint = 0xf2b0;
pub const F367TER_RSTATE_F: c_uint = 0xf2b00080;
pub const F367TER_RSTATE_E: c_uint = 0xf2b00040;
pub const F367TER_RSTATE_D: c_uint = 0xf2b00020;
pub const F367TER_RSTATE_C: c_uint = 0xf2b00010;
pub const F367TER_RSTATE_B: c_uint = 0xf2b00008;
pub const F367TER_RSTATE_A: c_uint = 0xf2b00004;
pub const F367TER_RSTATE_9: c_uint = 0xf2b00002;
pub const F367TER_RSTATE_8: c_uint = 0xf2b00001;
// FSTATESL
pub const R367TER_FSTATESL: c_uint = 0xf2b1;
pub const F367TER_RSTATE_7: c_uint = 0xf2b10080;
pub const F367TER_RSTATE_6: c_uint = 0xf2b10040;
pub const F367TER_RSTATE_5: c_uint = 0xf2b10020;
pub const F367TER_RSTATE_4: c_uint = 0xf2b10010;
pub const F367TER_RSTATE_3: c_uint = 0xf2b10008;
pub const F367TER_RSTATE_2: c_uint = 0xf2b10004;
pub const F367TER_RSTATE_1: c_uint = 0xf2b10002;
pub const F367TER_RSTATE_0: c_uint = 0xf2b10001;
// FSPYBER
pub const R367TER_FSPYBER: c_uint = 0xf2b2;
pub const F367TER_FSPYBER_7: c_uint = 0xf2b20080;
pub const F367TER_FSPYOBS_XORREAD: c_uint = 0xf2b20040;
pub const F367TER_FSPYBER_OBSMODE: c_uint = 0xf2b20020;
pub const F367TER_FSPYBER_SYNCBYTE: c_uint = 0xf2b20010;
pub const F367TER_FSPYBER_UNSYNC: c_uint = 0xf2b20008;
pub const F367TER_FSPYBER_CTIME: c_uint = 0xf2b20007;
// FSPYDISTM
pub const R367TER_FSPYDISTM: c_uint = 0xf2b3;
pub const F367TER_PKTTIME_DISTANCE_HI: c_uint = 0xf2b300ff;
// FSPYDISTL
pub const R367TER_FSPYDISTL: c_uint = 0xf2b4;
pub const F367TER_PKTTIME_DISTANCE_LO: c_uint = 0xf2b400ff;
// FSPYOBS7
pub const R367TER_FSPYOBS7: c_uint = 0xf2b8;
pub const F367TER_FSPYOBS_SPYFAIL: c_uint = 0xf2b80080;
pub const F367TER_FSPYOBS_SPYFAIL1: c_uint = 0xf2b80040;
pub const F367TER_FSPYOBS_ERROR: c_uint = 0xf2b80020;
pub const F367TER_FSPYOBS_STROUT: c_uint = 0xf2b80010;
pub const F367TER_FSPYOBS_RESULTSTATE1: c_uint = 0xf2b8000f;
// FSPYOBS6
pub const R367TER_FSPYOBS6: c_uint = 0xf2b9;
pub const F367TER_FSPYOBS_RESULTSTATe0: c_uint = 0xf2b900f0;
pub const F367TER_FSPYOBS_RESULTSTATEM1: c_uint = 0xf2b9000f;
// FSPYOBS5
pub const R367TER_FSPYOBS5: c_uint = 0xf2ba;
pub const F367TER_FSPYOBS_BYTEOFPACKET1: c_uint = 0xf2ba00ff;
// FSPYOBS4
pub const R367TER_FSPYOBS4: c_uint = 0xf2bb;
pub const F367TER_FSPYOBS_BYTEVALUE1: c_uint = 0xf2bb00ff;
// FSPYOBS3
pub const R367TER_FSPYOBS3: c_uint = 0xf2bc;
pub const F367TER_FSPYOBS_DATA1: c_uint = 0xf2bc00ff;
// FSPYOBS2
pub const R367TER_FSPYOBS2: c_uint = 0xf2bd;
pub const F367TER_FSPYOBS_DATa0: c_uint = 0xf2bd00ff;
// FSPYOBS1
pub const R367TER_FSPYOBS1: c_uint = 0xf2be;
pub const F367TER_FSPYOBS_DATAM1: c_uint = 0xf2be00ff;
// FSPYOBS0
pub const R367TER_FSPYOBS0: c_uint = 0xf2bf;
pub const F367TER_FSPYOBS_DATAM2: c_uint = 0xf2bf00ff;
// SFDEMAP
pub const R367TER_SFDEMAP: c_uint = 0xf2c0;
pub const F367TER_SFDEMAP_7: c_uint = 0xf2c00080;
pub const F367TER_SFEC_K_DIVIDER_VIT: c_uint = 0xf2c0007f;
// SFERROR
pub const R367TER_SFERROR: c_uint = 0xf2c1;
pub const F367TER_SFEC_REGERR_VIT: c_uint = 0xf2c100ff;
// SFAVSR
pub const R367TER_SFAVSR: c_uint = 0xf2c2;
pub const F367TER_SFEC_SUMERRORS: c_uint = 0xf2c20080;
pub const F367TER_SERROR_MAXMODE: c_uint = 0xf2c20040;
pub const F367TER_SN_SFEC: c_uint = 0xf2c20030;
pub const F367TER_KDIV_MODE_SFEC: c_uint = 0xf2c2000c;
pub const F367TER_SFAVSR_1: c_uint = 0xf2c20002;
pub const F367TER_SFAVSR_0: c_uint = 0xf2c20001;
// SFECSTATUS
pub const R367TER_SFECSTATUS: c_uint = 0xf2c3;
pub const F367TER_SFEC_ON: c_uint = 0xf2c30080;
pub const F367TER_SFSTATUS_6: c_uint = 0xf2c30040;
pub const F367TER_SFSTATUS_5: c_uint = 0xf2c30020;
pub const F367TER_SFSTATUS_4: c_uint = 0xf2c30010;
pub const F367TER_LOCKEDSFEC: c_uint = 0xf2c30008;
pub const F367TER_SFEC_DELOCK: c_uint = 0xf2c30004;
pub const F367TER_SFEC_DEMODSEL1: c_uint = 0xf2c30002;
pub const F367TER_SFEC_OVFON: c_uint = 0xf2c30001;
// SFKDIV12
pub const R367TER_SFKDIV12: c_uint = 0xf2c4;
pub const F367TER_SFECKDIV12_MAN: c_uint = 0xf2c40080;
pub const F367TER_SFEC_K_DIVIDER_12: c_uint = 0xf2c4007f;
// SFKDIV23
pub const R367TER_SFKDIV23: c_uint = 0xf2c5;
pub const F367TER_SFECKDIV23_MAN: c_uint = 0xf2c50080;
pub const F367TER_SFEC_K_DIVIDER_23: c_uint = 0xf2c5007f;
// SFKDIV34
pub const R367TER_SFKDIV34: c_uint = 0xf2c6;
pub const F367TER_SFECKDIV34_MAN: c_uint = 0xf2c60080;
pub const F367TER_SFEC_K_DIVIDER_34: c_uint = 0xf2c6007f;
// SFKDIV56
pub const R367TER_SFKDIV56: c_uint = 0xf2c7;
pub const F367TER_SFECKDIV56_MAN: c_uint = 0xf2c70080;
pub const F367TER_SFEC_K_DIVIDER_56: c_uint = 0xf2c7007f;
// SFKDIV67
pub const R367TER_SFKDIV67: c_uint = 0xf2c8;
pub const F367TER_SFECKDIV67_MAN: c_uint = 0xf2c80080;
pub const F367TER_SFEC_K_DIVIDER_67: c_uint = 0xf2c8007f;
// SFKDIV78
pub const R367TER_SFKDIV78: c_uint = 0xf2c9;
pub const F367TER_SFECKDIV78_MAN: c_uint = 0xf2c90080;
pub const F367TER_SFEC_K_DIVIDER_78: c_uint = 0xf2c9007f;
// SFDILSTKM
pub const R367TER_SFDILSTKM: c_uint = 0xf2ca;
pub const F367TER_SFEC_PACKCPT: c_uint = 0xf2ca00e0;
pub const F367TER_SFEC_DILSTK_HI: c_uint = 0xf2ca001f;
// SFDILSTKL
pub const R367TER_SFDILSTKL: c_uint = 0xf2cb;
pub const F367TER_SFEC_DILSTK_LO: c_uint = 0xf2cb00ff;
// SFSTATUS
pub const R367TER_SFSTATUS: c_uint = 0xf2cc;
pub const F367TER_SFEC_LINEOK: c_uint = 0xf2cc0080;
pub const F367TER_SFEC_ERROR: c_uint = 0xf2cc0040;
pub const F367TER_SFEC_DATA7: c_uint = 0xf2cc0020;
pub const F367TER_SFEC_OVERFLOW: c_uint = 0xf2cc0010;
pub const F367TER_SFEC_DEMODSEL2: c_uint = 0xf2cc0008;
pub const F367TER_SFEC_NOSYNC: c_uint = 0xf2cc0004;
pub const F367TER_SFEC_UNREGULA: c_uint = 0xf2cc0002;
pub const F367TER_SFEC_READY: c_uint = 0xf2cc0001;
// SFDLYH
pub const R367TER_SFDLYH: c_uint = 0xf2cd;
pub const F367TER_SFEC_TSTIMEVALID: c_uint = 0xf2cd0080;
pub const F367TER_SFEC_SPEEDUP: c_uint = 0xf2cd0040;
pub const F367TER_SFEC_STOP: c_uint = 0xf2cd0020;
pub const F367TER_SFEC_REGULATED: c_uint = 0xf2cd0010;
pub const F367TER_SFEC_REALSYMBOFFSET: c_uint = 0xf2cd000f;
// SFDLYM
pub const R367TER_SFDLYM: c_uint = 0xf2ce;
pub const F367TER_SFEC_REALSYMBOFFSET_HI: c_uint = 0xf2ce00ff;
// SFDLYL
pub const R367TER_SFDLYL: c_uint = 0xf2cf;
pub const F367TER_SFEC_REALSYMBOFFSET_LO: c_uint = 0xf2cf00ff;
// SFDLYSETH
pub const R367TER_SFDLYSETH: c_uint = 0xf2d0;
pub const F367TER_SFEC_OFFSET: c_uint = 0xf2d000e0;
pub const F367TER_SFECDLYSETH_4: c_uint = 0xf2d00010;
pub const F367TER_RST_SFEC: c_uint = 0xf2d00008;
pub const F367TER_SFECDLYSETH_2: c_uint = 0xf2d00004;
pub const F367TER_SFEC_DISABLE: c_uint = 0xf2d00002;
pub const F367TER_SFEC_UNREGUL: c_uint = 0xf2d00001;
// SFDLYSETM
pub const R367TER_SFDLYSETM: c_uint = 0xf2d1;
pub const F367TER_SFECDLYSETM_7: c_uint = 0xf2d10080;
pub const F367TER_SFEC_SYMBOFFSET_HI: c_uint = 0xf2d1007f;
// SFDLYSETL
pub const R367TER_SFDLYSETL: c_uint = 0xf2d2;
pub const F367TER_SFEC_SYMBOFFSET_LO: c_uint = 0xf2d200ff;
// SFOBSCFG
pub const R367TER_SFOBSCFG: c_uint = 0xf2d3;
pub const F367TER_SFEC_OBSCFG: c_uint = 0xf2d300ff;
// SFOBSM
pub const R367TER_SFOBSM: c_uint = 0xf2d4;
pub const F367TER_SFEC_OBSDATA_HI: c_uint = 0xf2d400ff;
// SFOBSL
pub const R367TER_SFOBSL: c_uint = 0xf2d5;
pub const F367TER_SFEC_OBSDATA_LO: c_uint = 0xf2d500ff;
// SFECINFO
pub const R367TER_SFECINFO: c_uint = 0xf2d6;
pub const F367TER_SFECINFO_7: c_uint = 0xf2d60080;
pub const F367TER_SFEC_SYNCDLSB: c_uint = 0xf2d60070;
pub const F367TER_SFCE_S1cPHASE: c_uint = 0xf2d6000f;
// SFERRCTRL
pub const R367TER_SFERRCTRL: c_uint = 0xf2d8;
pub const F367TER_SFEC_ERR_SOURCE: c_uint = 0xf2d800f0;
pub const F367TER_SFERRCTRL_3: c_uint = 0xf2d80008;
pub const F367TER_SFEC_NUM_EVENT: c_uint = 0xf2d80007;
// SFERRCNTH
pub const R367TER_SFERRCNTH: c_uint = 0xf2d9;
pub const F367TER_SFERRC_OLDVALUE: c_uint = 0xf2d90080;
pub const F367TER_SFEC_ERR_CNT: c_uint = 0xf2d9007f;
// SFERRCNTM
pub const R367TER_SFERRCNTM: c_uint = 0xf2da;
pub const F367TER_SFEC_ERR_CNT_HI: c_uint = 0xf2da00ff;
// SFERRCNTL
pub const R367TER_SFERRCNTL: c_uint = 0xf2db;
pub const F367TER_SFEC_ERR_CNT_LO: c_uint = 0xf2db00ff;
// SYMBRATEM
pub const R367TER_SYMBRATEM: c_uint = 0xf2e0;
pub const F367TER_DEFGEN_SYMBRATE_HI: c_uint = 0xf2e000ff;
// SYMBRATEL
pub const R367TER_SYMBRATEL: c_uint = 0xf2e1;
pub const F367TER_DEFGEN_SYMBRATE_LO: c_uint = 0xf2e100ff;
// SYMBSTATUS
pub const R367TER_SYMBSTATUS: c_uint = 0xf2e2;
pub const F367TER_SYMBDLINE2_OFF: c_uint = 0xf2e20080;
pub const F367TER_SDDL_REINIT1: c_uint = 0xf2e20040;
pub const F367TER_SDD_REINIT1: c_uint = 0xf2e20020;
pub const F367TER_TOKENID_ERROR: c_uint = 0xf2e20010;
pub const F367TER_SYMBRATE_OVERFLOW: c_uint = 0xf2e20008;
pub const F367TER_SYMBRATE_UNDERFLOW: c_uint = 0xf2e20004;
pub const F367TER_TOKENID_RSTEVENT: c_uint = 0xf2e20002;
pub const F367TER_TOKENID_RESET1: c_uint = 0xf2e20001;
// SYMBCFG
pub const R367TER_SYMBCFG: c_uint = 0xf2e3;
pub const F367TER_SYMBCFG_7: c_uint = 0xf2e30080;
pub const F367TER_SYMBCFG_6: c_uint = 0xf2e30040;
pub const F367TER_SYMBCFG_5: c_uint = 0xf2e30020;
pub const F367TER_SYMBCFG_4: c_uint = 0xf2e30010;
pub const F367TER_SYMRATE_FSPEED: c_uint = 0xf2e3000c;
pub const F367TER_SYMRATE_SSPEED: c_uint = 0xf2e30003;
// SYMBFIFOM
pub const R367TER_SYMBFIFOM: c_uint = 0xf2e4;
pub const F367TER_SYMBFIFOM_7: c_uint = 0xf2e40080;
pub const F367TER_SYMBFIFOM_6: c_uint = 0xf2e40040;
pub const F367TER_DEFGEN_SYMFIFO_HI: c_uint = 0xf2e4003f;
// SYMBFIFOL
pub const R367TER_SYMBFIFOL: c_uint = 0xf2e5;
pub const F367TER_DEFGEN_SYMFIFO_LO: c_uint = 0xf2e500ff;
// SYMBOFFSM
pub const R367TER_SYMBOFFSM: c_uint = 0xf2e6;
pub const F367TER_TOKENID_RESET2: c_uint = 0xf2e60080;
pub const F367TER_SDDL_REINIT2: c_uint = 0xf2e60040;
pub const F367TER_SDD_REINIT2: c_uint = 0xf2e60020;
pub const F367TER_SYMBOFFSM_4: c_uint = 0xf2e60010;
pub const F367TER_SYMBOFFSM_3: c_uint = 0xf2e60008;
pub const F367TER_DEFGEN_SYMBOFFSET_HI: c_uint = 0xf2e60007;
// SYMBOFFSL
pub const R367TER_SYMBOFFSL: c_uint = 0xf2e7;
pub const F367TER_DEFGEN_SYMBOFFSET_LO: c_uint = 0xf2e700ff;
// DEBUG_LT4
pub const R367TER_DEBUG_LT4: c_uint = 0xf400;
pub const F367TER_F_DEBUG_LT4: c_uint = 0xf40000ff;
// DEBUG_LT5
pub const R367TER_DEBUG_LT5: c_uint = 0xf401;
pub const F367TER_F_DEBUG_LT5: c_uint = 0xf40100ff;
// DEBUG_LT6
pub const R367TER_DEBUG_LT6: c_uint = 0xf402;
pub const F367TER_F_DEBUG_LT6: c_uint = 0xf40200ff;
// DEBUG_LT7
pub const R367TER_DEBUG_LT7: c_uint = 0xf403;
pub const F367TER_F_DEBUG_LT7: c_uint = 0xf40300ff;
// DEBUG_LT8
pub const R367TER_DEBUG_LT8: c_uint = 0xf404;
pub const F367TER_F_DEBUG_LT8: c_uint = 0xf40400ff;
// DEBUG_LT9
pub const R367TER_DEBUG_LT9: c_uint = 0xf405;
pub const F367TER_F_DEBUG_LT9: c_uint = 0xf40500ff;
// ID
pub const R367CAB_ID: c_uint = 0xf000;
pub const F367CAB_IDENTIFICATIONREGISTER: c_uint = 0xf00000ff;
// I2CRPT
pub const R367CAB_I2CRPT: c_uint = 0xf001;
pub const F367CAB_I2CT_ON: c_uint = 0xf0010080;
pub const F367CAB_ENARPT_LEVEL: c_uint = 0xf0010070;
pub const F367CAB_SCLT_DELAY: c_uint = 0xf0010008;
pub const F367CAB_SCLT_NOD: c_uint = 0xf0010004;
pub const F367CAB_STOP_ENABLE: c_uint = 0xf0010002;
pub const F367CAB_SDAT_NOD: c_uint = 0xf0010001;
// TOPCTRL
pub const R367CAB_TOPCTRL: c_uint = 0xf002;
pub const F367CAB_STDBY: c_uint = 0xf0020080;
pub const F367CAB_STDBY_CORE: c_uint = 0xf0020020;
pub const F367CAB_QAM_COFDM: c_uint = 0xf0020010;
pub const F367CAB_TS_DIS: c_uint = 0xf0020008;
pub const F367CAB_DIR_CLK_216: c_uint = 0xf0020004;
// IOCFG0
pub const R367CAB_IOCFG0: c_uint = 0xf003;
pub const F367CAB_OP0_SD: c_uint = 0xf0030080;
pub const F367CAB_OP0_VAL: c_uint = 0xf0030040;
pub const F367CAB_OP0_OD: c_uint = 0xf0030020;
pub const F367CAB_OP0_INV: c_uint = 0xf0030010;
pub const F367CAB_OP0_DACVALUE_HI: c_uint = 0xf003000f;
// DAc0R
pub const R367CAB_DAC0R: c_uint = 0xf004;
pub const F367CAB_OP0_DACVALUE_LO: c_uint = 0xf00400ff;
// IOCFG1
pub const R367CAB_IOCFG1: c_uint = 0xf005;
pub const F367CAB_IP0: c_uint = 0xf0050040;
pub const F367CAB_OP1_OD: c_uint = 0xf0050020;
pub const F367CAB_OP1_INV: c_uint = 0xf0050010;
pub const F367CAB_OP1_DACVALUE_HI: c_uint = 0xf005000f;
// DAC1R
pub const R367CAB_DAC1R: c_uint = 0xf006;
pub const F367CAB_OP1_DACVALUE_LO: c_uint = 0xf00600ff;
// IOCFG2
pub const R367CAB_IOCFG2: c_uint = 0xf007;
pub const F367CAB_OP2_LOCK_CONF: c_uint = 0xf00700e0;
pub const F367CAB_OP2_OD: c_uint = 0xf0070010;
pub const F367CAB_OP2_VAL: c_uint = 0xf0070008;
pub const F367CAB_OP1_LOCK_CONF: c_uint = 0xf0070007;
// SDFR
pub const R367CAB_SDFR: c_uint = 0xf008;
pub const F367CAB_OP0_FREQ: c_uint = 0xf00800f0;
pub const F367CAB_OP1_FREQ: c_uint = 0xf008000f;
// AUX_CLK
pub const R367CAB_AUX_CLK: c_uint = 0xf00a;
pub const F367CAB_AUXFEC_CTL: c_uint = 0xf00a00c0;
pub const F367CAB_DIS_CKX4: c_uint = 0xf00a0020;
pub const F367CAB_CKSEL: c_uint = 0xf00a0018;
pub const F367CAB_CKDIV_PROG: c_uint = 0xf00a0006;
pub const F367CAB_AUXCLK_ENA: c_uint = 0xf00a0001;
// FREESYS1
pub const R367CAB_FREESYS1: c_uint = 0xf00b;
pub const F367CAB_FREESYS_1: c_uint = 0xf00b00ff;
// FREESYS2
pub const R367CAB_FREESYS2: c_uint = 0xf00c;
pub const F367CAB_FREESYS_2: c_uint = 0xf00c00ff;
// FREESYS3
pub const R367CAB_FREESYS3: c_uint = 0xf00d;
pub const F367CAB_FREESYS_3: c_uint = 0xf00d00ff;
// GPIO_CFG
pub const R367CAB_GPIO_CFG: c_uint = 0xf00e;
pub const F367CAB_GPIO7_OD: c_uint = 0xf00e0080;
pub const F367CAB_GPIO7_CFG: c_uint = 0xf00e0040;
pub const F367CAB_GPIO6_OD: c_uint = 0xf00e0020;
pub const F367CAB_GPIO6_CFG: c_uint = 0xf00e0010;
pub const F367CAB_GPIO5_OD: c_uint = 0xf00e0008;
pub const F367CAB_GPIO5_CFG: c_uint = 0xf00e0004;
pub const F367CAB_GPIO4_OD: c_uint = 0xf00e0002;
pub const F367CAB_GPIO4_CFG: c_uint = 0xf00e0001;
// GPIO_CMD
pub const R367CAB_GPIO_CMD: c_uint = 0xf00f;
pub const F367CAB_GPIO7_VAL: c_uint = 0xf00f0008;
pub const F367CAB_GPIO6_VAL: c_uint = 0xf00f0004;
pub const F367CAB_GPIO5_VAL: c_uint = 0xf00f0002;
pub const F367CAB_GPIO4_VAL: c_uint = 0xf00f0001;
// TSTRES
pub const R367CAB_TSTRES: c_uint = 0xf0c0;
pub const F367CAB_FRES_DISPLAY: c_uint = 0xf0c00080;
pub const F367CAB_FRES_FIFO_AD: c_uint = 0xf0c00020;
pub const F367CAB_FRESRS: c_uint = 0xf0c00010;
pub const F367CAB_FRESACS: c_uint = 0xf0c00008;
pub const F367CAB_FRESFEC: c_uint = 0xf0c00004;
pub const F367CAB_FRES_PRIF: c_uint = 0xf0c00002;
pub const F367CAB_FRESCORE: c_uint = 0xf0c00001;
// ANACTRL
pub const R367CAB_ANACTRL: c_uint = 0xf0c1;
pub const F367CAB_BYPASS_XTAL: c_uint = 0xf0c10040;
pub const F367CAB_BYPASS_PLLXN: c_uint = 0xf0c1000c;
pub const F367CAB_DIS_PAD_OSC: c_uint = 0xf0c10002;
pub const F367CAB_STDBY_PLLXN: c_uint = 0xf0c10001;
// TSTBUS
pub const R367CAB_TSTBUS: c_uint = 0xf0c2;
pub const F367CAB_TS_BYTE_CLK_INV: c_uint = 0xf0c20080;
pub const F367CAB_CFG_IP: c_uint = 0xf0c20070;
pub const F367CAB_CFG_TST: c_uint = 0xf0c2000f;
// RF_AGC1
pub const R367CAB_RF_AGC1: c_uint = 0xf0d4;
pub const F367CAB_RF_AGC1_LEVEL_HI: c_uint = 0xf0d400ff;
// RF_AGC2
pub const R367CAB_RF_AGC2: c_uint = 0xf0d5;
pub const F367CAB_REF_ADGP: c_uint = 0xf0d50080;
pub const F367CAB_STDBY_ADCGP: c_uint = 0xf0d50020;
pub const F367CAB_RF_AGC1_LEVEL_LO: c_uint = 0xf0d50003;
// ANADIGCTRL
pub const R367CAB_ANADIGCTRL: c_uint = 0xf0d7;
pub const F367CAB_SEL_CLKDEM: c_uint = 0xf0d70020;
pub const F367CAB_EN_BUFFER_Q: c_uint = 0xf0d70010;
pub const F367CAB_EN_BUFFER_I: c_uint = 0xf0d70008;
pub const F367CAB_ADC_RIS_EGDE: c_uint = 0xf0d70004;
pub const F367CAB_SGN_ADC: c_uint = 0xf0d70002;
pub const F367CAB_SEL_AD12_SYNC: c_uint = 0xf0d70001;
// PLLMDIV
pub const R367CAB_PLLMDIV: c_uint = 0xf0d8;
pub const F367CAB_PLL_MDIV: c_uint = 0xf0d800ff;
// PLLNDIV
pub const R367CAB_PLLNDIV: c_uint = 0xf0d9;
pub const F367CAB_PLL_NDIV: c_uint = 0xf0d900ff;
// PLLSETUP
pub const R367CAB_PLLSETUP: c_uint = 0xf0da;
pub const F367CAB_PLL_PDIV: c_uint = 0xf0da0070;
pub const F367CAB_PLL_KDIV: c_uint = 0xf0da000f;
// DUAL_AD12
pub const R367CAB_DUAL_AD12: c_uint = 0xf0db;
pub const F367CAB_FS20M: c_uint = 0xf0db0020;
pub const F367CAB_FS50M: c_uint = 0xf0db0010;
pub const F367CAB_INMODe0: c_uint = 0xf0db0008;
pub const F367CAB_POFFQ: c_uint = 0xf0db0004;
pub const F367CAB_POFFI: c_uint = 0xf0db0002;
pub const F367CAB_INMODE1: c_uint = 0xf0db0001;
// TSTBIST
pub const R367CAB_TSTBIST: c_uint = 0xf0dc;
pub const F367CAB_TST_BYP_CLK: c_uint = 0xf0dc0080;
pub const F367CAB_TST_GCLKENA_STD: c_uint = 0xf0dc0040;
pub const F367CAB_TST_GCLKENA: c_uint = 0xf0dc0020;
pub const F367CAB_TST_MEMBIST: c_uint = 0xf0dc001f;
// CTRL_1
pub const R367CAB_CTRL_1: c_uint = 0xf402;
pub const F367CAB_SOFT_RST: c_uint = 0xf4020080;
pub const F367CAB_EQU_RST: c_uint = 0xf4020008;
pub const F367CAB_CRL_RST: c_uint = 0xf4020004;
pub const F367CAB_TRL_RST: c_uint = 0xf4020002;
pub const F367CAB_AGC_RST: c_uint = 0xf4020001;
// CTRL_2
pub const R367CAB_CTRL_2: c_uint = 0xf403;
pub const F367CAB_DEINT_RST: c_uint = 0xf4030008;
pub const F367CAB_RS_RST: c_uint = 0xf4030004;
// IT_STATUS1
pub const R367CAB_IT_STATUS1: c_uint = 0xf408;
pub const F367CAB_SWEEP_OUT: c_uint = 0xf4080080;
pub const F367CAB_FSM_CRL: c_uint = 0xf4080040;
pub const F367CAB_CRL_LOCK: c_uint = 0xf4080020;
pub const F367CAB_MFSM: c_uint = 0xf4080010;
pub const F367CAB_TRL_LOCK: c_uint = 0xf4080008;
pub const F367CAB_TRL_AGC_LIMIT: c_uint = 0xf4080004;
pub const F367CAB_ADJ_AGC_LOCK: c_uint = 0xf4080002;
pub const F367CAB_AGC_QAM_LOCK: c_uint = 0xf4080001;
// IT_STATUS2
pub const R367CAB_IT_STATUS2: c_uint = 0xf409;
pub const F367CAB_TSMF_CNT: c_uint = 0xf4090080;
pub const F367CAB_TSMF_EOF: c_uint = 0xf4090040;
pub const F367CAB_TSMF_RDY: c_uint = 0xf4090020;
pub const F367CAB_FEC_NOCORR: c_uint = 0xf4090010;
pub const F367CAB_SYNCSTATE: c_uint = 0xf4090008;
pub const F367CAB_DEINT_LOCK: c_uint = 0xf4090004;
pub const F367CAB_FADDING_FRZ: c_uint = 0xf4090002;
pub const F367CAB_TAPMON_ALARM: c_uint = 0xf4090001;
// IT_EN1
pub const R367CAB_IT_EN1: c_uint = 0xf40a;
pub const F367CAB_SWEEP_OUTE: c_uint = 0xf40a0080;
pub const F367CAB_FSM_CRLE: c_uint = 0xf40a0040;
pub const F367CAB_CRL_LOCKE: c_uint = 0xf40a0020;
pub const F367CAB_MFSME: c_uint = 0xf40a0010;
pub const F367CAB_TRL_LOCKE: c_uint = 0xf40a0008;
pub const F367CAB_TRL_AGC_LIMITE: c_uint = 0xf40a0004;
pub const F367CAB_ADJ_AGC_LOCKE: c_uint = 0xf40a0002;
pub const F367CAB_AGC_LOCKE: c_uint = 0xf40a0001;
// IT_EN2
pub const R367CAB_IT_EN2: c_uint = 0xf40b;
pub const F367CAB_TSMF_CNTE: c_uint = 0xf40b0080;
pub const F367CAB_TSMF_EOFE: c_uint = 0xf40b0040;
pub const F367CAB_TSMF_RDYE: c_uint = 0xf40b0020;
pub const F367CAB_FEC_NOCORRE: c_uint = 0xf40b0010;
pub const F367CAB_SYNCSTATEE: c_uint = 0xf40b0008;
pub const F367CAB_DEINT_LOCKE: c_uint = 0xf40b0004;
pub const F367CAB_FADDING_FRZE: c_uint = 0xf40b0002;
pub const F367CAB_TAPMON_ALARME: c_uint = 0xf40b0001;
// CTRL_STATUS
pub const R367CAB_CTRL_STATUS: c_uint = 0xf40c;
pub const F367CAB_QAMFEC_LOCK: c_uint = 0xf40c0004;
pub const F367CAB_TSMF_LOCK: c_uint = 0xf40c0002;
pub const F367CAB_TSMF_ERROR: c_uint = 0xf40c0001;
// TEST_CTL
pub const R367CAB_TEST_CTL: c_uint = 0xf40f;
pub const F367CAB_TST_BLK_SEL: c_uint = 0xf40f0060;
pub const F367CAB_TST_BUS_SEL: c_uint = 0xf40f001f;
// AGC_CTL
pub const R367CAB_AGC_CTL: c_uint = 0xf410;
pub const F367CAB_AGC_LCK_TH: c_uint = 0xf41000f0;
pub const F367CAB_AGC_ACCUMRSTSEL: c_uint = 0xf4100007;
// AGC_IF_CFG
pub const R367CAB_AGC_IF_CFG: c_uint = 0xf411;
pub const F367CAB_AGC_IF_BWSEL: c_uint = 0xf41100f0;
pub const F367CAB_AGC_IF_FREEZE: c_uint = 0xf4110002;
// AGC_RF_CFG
pub const R367CAB_AGC_RF_CFG: c_uint = 0xf412;
pub const F367CAB_AGC_RF_BWSEL: c_uint = 0xf4120070;
pub const F367CAB_AGC_RF_FREEZE: c_uint = 0xf4120002;
// AGC_PWM_CFG
pub const R367CAB_AGC_PWM_CFG: c_uint = 0xf413;
pub const F367CAB_AGC_RF_PWM_TST: c_uint = 0xf4130080;
pub const F367CAB_AGC_RF_PWM_INV: c_uint = 0xf4130040;
pub const F367CAB_AGC_IF_PWM_TST: c_uint = 0xf4130008;
pub const F367CAB_AGC_IF_PWM_INV: c_uint = 0xf4130004;
pub const F367CAB_AGC_PWM_CLKDIV: c_uint = 0xf4130003;
// AGC_PWR_REF_L
pub const R367CAB_AGC_PWR_REF_L: c_uint = 0xf414;
pub const F367CAB_AGC_PWRREF_LO: c_uint = 0xf41400ff;
// AGC_PWR_REF_H
pub const R367CAB_AGC_PWR_REF_H: c_uint = 0xf415;
pub const F367CAB_AGC_PWRREF_HI: c_uint = 0xf4150003;
// AGC_RF_TH_L
pub const R367CAB_AGC_RF_TH_L: c_uint = 0xf416;
pub const F367CAB_AGC_RF_TH_LO: c_uint = 0xf41600ff;
// AGC_RF_TH_H
pub const R367CAB_AGC_RF_TH_H: c_uint = 0xf417;
pub const F367CAB_AGC_RF_TH_HI: c_uint = 0xf417000f;
// AGC_IF_LTH_L
pub const R367CAB_AGC_IF_LTH_L: c_uint = 0xf418;
pub const F367CAB_AGC_IF_THLO_LO: c_uint = 0xf41800ff;
// AGC_IF_LTH_H
pub const R367CAB_AGC_IF_LTH_H: c_uint = 0xf419;
pub const F367CAB_AGC_IF_THLO_HI: c_uint = 0xf419000f;
// AGC_IF_HTH_L
pub const R367CAB_AGC_IF_HTH_L: c_uint = 0xf41a;
pub const F367CAB_AGC_IF_THHI_LO: c_uint = 0xf41a00ff;
// AGC_IF_HTH_H
pub const R367CAB_AGC_IF_HTH_H: c_uint = 0xf41b;
pub const F367CAB_AGC_IF_THHI_HI: c_uint = 0xf41b000f;
// AGC_PWR_RD_L
pub const R367CAB_AGC_PWR_RD_L: c_uint = 0xf41c;
pub const F367CAB_AGC_PWR_WORD_LO: c_uint = 0xf41c00ff;
// AGC_PWR_RD_M
pub const R367CAB_AGC_PWR_RD_M: c_uint = 0xf41d;
pub const F367CAB_AGC_PWR_WORD_ME: c_uint = 0xf41d00ff;
// AGC_PWR_RD_H
pub const R367CAB_AGC_PWR_RD_H: c_uint = 0xf41e;
pub const F367CAB_AGC_PWR_WORD_HI: c_uint = 0xf41e0003;
// AGC_PWM_IFCMD_L
pub const R367CAB_AGC_PWM_IFCMD_L: c_uint = 0xf420;
pub const F367CAB_AGC_IF_PWMCMD_LO: c_uint = 0xf42000ff;
// AGC_PWM_IFCMD_H
pub const R367CAB_AGC_PWM_IFCMD_H: c_uint = 0xf421;
pub const F367CAB_AGC_IF_PWMCMD_HI: c_uint = 0xf421000f;
// AGC_PWM_RFCMD_L
pub const R367CAB_AGC_PWM_RFCMD_L: c_uint = 0xf422;
pub const F367CAB_AGC_RF_PWMCMD_LO: c_uint = 0xf42200ff;
// AGC_PWM_RFCMD_H
pub const R367CAB_AGC_PWM_RFCMD_H: c_uint = 0xf423;
pub const F367CAB_AGC_RF_PWMCMD_HI: c_uint = 0xf423000f;
// IQDEM_CFG
pub const R367CAB_IQDEM_CFG: c_uint = 0xf424;
pub const F367CAB_IQDEM_CLK_SEL: c_uint = 0xf4240004;
pub const F367CAB_IQDEM_INVIQ: c_uint = 0xf4240002;
pub const F367CAB_IQDEM_A2dTYPE: c_uint = 0xf4240001;
// MIX_NCO_LL
pub const R367CAB_MIX_NCO_LL: c_uint = 0xf425;
pub const F367CAB_MIX_NCO_INC_LL: c_uint = 0xf42500ff;
// MIX_NCO_HL
pub const R367CAB_MIX_NCO_HL: c_uint = 0xf426;
pub const F367CAB_MIX_NCO_INC_HL: c_uint = 0xf42600ff;
// MIX_NCO_HH
pub const R367CAB_MIX_NCO_HH: c_uint = 0xf427;
pub const F367CAB_MIX_NCO_INVCNST: c_uint = 0xf4270080;
pub const F367CAB_MIX_NCO_INC_HH: c_uint = 0xf427007f;
// SRC_NCO_LL
pub const R367CAB_SRC_NCO_LL: c_uint = 0xf428;
pub const F367CAB_SRC_NCO_INC_LL: c_uint = 0xf42800ff;
// SRC_NCO_LH
pub const R367CAB_SRC_NCO_LH: c_uint = 0xf429;
pub const F367CAB_SRC_NCO_INC_LH: c_uint = 0xf42900ff;
// SRC_NCO_HL
pub const R367CAB_SRC_NCO_HL: c_uint = 0xf42a;
pub const F367CAB_SRC_NCO_INC_HL: c_uint = 0xf42a00ff;
// SRC_NCO_HH
pub const R367CAB_SRC_NCO_HH: c_uint = 0xf42b;
pub const F367CAB_SRC_NCO_INC_HH: c_uint = 0xf42b007f;
// IQDEM_GAIN_SRC_L
pub const R367CAB_IQDEM_GAIN_SRC_L: c_uint = 0xf42c;
pub const F367CAB_GAIN_SRC_LO: c_uint = 0xf42c00ff;
// IQDEM_GAIN_SRC_H
pub const R367CAB_IQDEM_GAIN_SRC_H: c_uint = 0xf42d;
pub const F367CAB_GAIN_SRC_HI: c_uint = 0xf42d0003;
// IQDEM_DCRM_CFG_LL
pub const R367CAB_IQDEM_DCRM_CFG_LL: c_uint = 0xf430;
pub const F367CAB_DCRM0_DCIN_L: c_uint = 0xf43000ff;
// IQDEM_DCRM_CFG_LH
pub const R367CAB_IQDEM_DCRM_CFG_LH: c_uint = 0xf431;
pub const F367CAB_DCRM1_I_DCIN_L: c_uint = 0xf43100fc;
pub const F367CAB_DCRM0_DCIN_H: c_uint = 0xf4310003;
// IQDEM_DCRM_CFG_HL
pub const R367CAB_IQDEM_DCRM_CFG_HL: c_uint = 0xf432;
pub const F367CAB_DCRM1_Q_DCIN_L: c_uint = 0xf43200f0;
pub const F367CAB_DCRM1_I_DCIN_H: c_uint = 0xf432000f;
// IQDEM_DCRM_CFG_HH
pub const R367CAB_IQDEM_DCRM_CFG_HH: c_uint = 0xf433;
pub const F367CAB_DCRM1_FRZ: c_uint = 0xf4330080;
pub const F367CAB_DCRM0_FRZ: c_uint = 0xf4330040;
pub const F367CAB_DCRM1_Q_DCIN_H: c_uint = 0xf433003f;
// IQDEM_ADJ_COEFf0
pub const R367CAB_IQDEM_ADJ_COEFF0: c_uint = 0xf434;
pub const F367CAB_ADJIIR_COEFF10_L: c_uint = 0xf43400ff;
// IQDEM_ADJ_COEFF1
pub const R367CAB_IQDEM_ADJ_COEFF1: c_uint = 0xf435;
pub const F367CAB_ADJIIR_COEFF11_L: c_uint = 0xf43500fc;
pub const F367CAB_ADJIIR_COEFF10_H: c_uint = 0xf4350003;
// IQDEM_ADJ_COEFF2
pub const R367CAB_IQDEM_ADJ_COEFF2: c_uint = 0xf436;
pub const F367CAB_ADJIIR_COEFF12_L: c_uint = 0xf43600f0;
pub const F367CAB_ADJIIR_COEFF11_H: c_uint = 0xf436000f;
// IQDEM_ADJ_COEFF3
pub const R367CAB_IQDEM_ADJ_COEFF3: c_uint = 0xf437;
pub const F367CAB_ADJIIR_COEFF20_L: c_uint = 0xf43700c0;
pub const F367CAB_ADJIIR_COEFF12_H: c_uint = 0xf437003f;
// IQDEM_ADJ_COEFF4
pub const R367CAB_IQDEM_ADJ_COEFF4: c_uint = 0xf438;
pub const F367CAB_ADJIIR_COEFF20_H: c_uint = 0xf43800ff;
// IQDEM_ADJ_COEFF5
pub const R367CAB_IQDEM_ADJ_COEFF5: c_uint = 0xf439;
pub const F367CAB_ADJIIR_COEFF21_L: c_uint = 0xf43900ff;
// IQDEM_ADJ_COEFF6
pub const R367CAB_IQDEM_ADJ_COEFF6: c_uint = 0xf43a;
pub const F367CAB_ADJIIR_COEFF22_L: c_uint = 0xf43a00fc;
pub const F367CAB_ADJIIR_COEFF21_H: c_uint = 0xf43a0003;
// IQDEM_ADJ_COEFF7
pub const R367CAB_IQDEM_ADJ_COEFF7: c_uint = 0xf43b;
pub const F367CAB_ADJIIR_COEFF22_H: c_uint = 0xf43b000f;
// IQDEM_ADJ_EN
pub const R367CAB_IQDEM_ADJ_EN: c_uint = 0xf43c;
pub const F367CAB_ALLPASSFILT_EN: c_uint = 0xf43c0008;
pub const F367CAB_ADJ_AGC_EN: c_uint = 0xf43c0004;
pub const F367CAB_ADJ_COEFF_FRZ: c_uint = 0xf43c0002;
pub const F367CAB_ADJ_EN: c_uint = 0xf43c0001;
// IQDEM_ADJ_AGC_REF
pub const R367CAB_IQDEM_ADJ_AGC_REF: c_uint = 0xf43d;
pub const F367CAB_ADJ_AGC_REF: c_uint = 0xf43d00ff;
// ALLPASSFILT1
pub const R367CAB_ALLPASSFILT1: c_uint = 0xf440;
pub const F367CAB_ALLPASSFILT_COEFF1_LO: c_uint = 0xf44000ff;
// ALLPASSFILT2
pub const R367CAB_ALLPASSFILT2: c_uint = 0xf441;
pub const F367CAB_ALLPASSFILT_COEFF1_ME: c_uint = 0xf44100ff;
// ALLPASSFILT3
pub const R367CAB_ALLPASSFILT3: c_uint = 0xf442;
pub const F367CAB_ALLPASSFILT_COEFF2_LO: c_uint = 0xf44200c0;
pub const F367CAB_ALLPASSFILT_COEFF1_HI: c_uint = 0xf442003f;
// ALLPASSFILT4
pub const R367CAB_ALLPASSFILT4: c_uint = 0xf443;
pub const F367CAB_ALLPASSFILT_COEFF2_MEL: c_uint = 0xf44300ff;
// ALLPASSFILT5
pub const R367CAB_ALLPASSFILT5: c_uint = 0xf444;
pub const F367CAB_ALLPASSFILT_COEFF2_MEH: c_uint = 0xf44400ff;
// ALLPASSFILT6
pub const R367CAB_ALLPASSFILT6: c_uint = 0xf445;
pub const F367CAB_ALLPASSFILT_COEFF3_LO: c_uint = 0xf44500f0;
pub const F367CAB_ALLPASSFILT_COEFF2_HI: c_uint = 0xf445000f;
// ALLPASSFILT7
pub const R367CAB_ALLPASSFILT7: c_uint = 0xf446;
pub const F367CAB_ALLPASSFILT_COEFF3_MEL: c_uint = 0xf44600ff;
// ALLPASSFILT8
pub const R367CAB_ALLPASSFILT8: c_uint = 0xf447;
pub const F367CAB_ALLPASSFILT_COEFF3_MEH: c_uint = 0xf44700ff;
// ALLPASSFILT9
pub const R367CAB_ALLPASSFILT9: c_uint = 0xf448;
pub const F367CAB_ALLPASSFILT_COEFF4_LO: c_uint = 0xf44800fc;
pub const F367CAB_ALLPASSFILT_COEFF3_HI: c_uint = 0xf4480003;
// ALLPASSFILT10
pub const R367CAB_ALLPASSFILT10: c_uint = 0xf449;
pub const F367CAB_ALLPASSFILT_COEFF4_ME: c_uint = 0xf44900ff;
// ALLPASSFILT11
pub const R367CAB_ALLPASSFILT11: c_uint = 0xf44a;
pub const F367CAB_ALLPASSFILT_COEFF4_HI: c_uint = 0xf44a00ff;
// TRL_AGC_CFG
pub const R367CAB_TRL_AGC_CFG: c_uint = 0xf450;
pub const F367CAB_TRL_AGC_FREEZE: c_uint = 0xf4500080;
pub const F367CAB_TRL_AGC_REF: c_uint = 0xf450007f;
// TRL_LPF_CFG
pub const R367CAB_TRL_LPF_CFG: c_uint = 0xf454;
pub const F367CAB_NYQPOINT_INV: c_uint = 0xf4540040;
pub const F367CAB_TRL_SHIFT: c_uint = 0xf4540030;
pub const F367CAB_NYQ_COEFF_SEL: c_uint = 0xf454000c;
pub const F367CAB_TRL_LPF_FREEZE: c_uint = 0xf4540002;
pub const F367CAB_TRL_LPF_CRT: c_uint = 0xf4540001;
// TRL_LPF_ACQ_GAIN
pub const R367CAB_TRL_LPF_ACQ_GAIN: c_uint = 0xf455;
pub const F367CAB_TRL_GDIR_ACQ: c_uint = 0xf4550070;
pub const F367CAB_TRL_GINT_ACQ: c_uint = 0xf4550007;
// TRL_LPF_TRK_GAIN
pub const R367CAB_TRL_LPF_TRK_GAIN: c_uint = 0xf456;
pub const F367CAB_TRL_GDIR_TRK: c_uint = 0xf4560070;
pub const F367CAB_TRL_GINT_TRK: c_uint = 0xf4560007;
// TRL_LPF_OUT_GAIN
pub const R367CAB_TRL_LPF_OUT_GAIN: c_uint = 0xf457;
pub const F367CAB_TRL_GAIN_OUT: c_uint = 0xf4570007;
// TRL_LOCKDET_LTH
pub const R367CAB_TRL_LOCKDET_LTH: c_uint = 0xf458;
pub const F367CAB_TRL_LCK_THLO: c_uint = 0xf4580007;
// TRL_LOCKDET_HTH
pub const R367CAB_TRL_LOCKDET_HTH: c_uint = 0xf459;
pub const F367CAB_TRL_LCK_THHI: c_uint = 0xf45900ff;
// TRL_LOCKDET_TRGVAL
pub const R367CAB_TRL_LOCKDET_TRGVAL: c_uint = 0xf45a;
pub const F367CAB_TRL_LCK_TRG: c_uint = 0xf45a00ff;
// IQ_QAM
pub const R367CAB_IQ_QAM: c_uint = 0xf45c;
pub const F367CAB_IQ_INPUT: c_uint = 0xf45c0008;
pub const F367CAB_DETECT_MODE: c_uint = 0xf45c0007;
// FSM_STATE
pub const R367CAB_FSM_STATE: c_uint = 0xf460;
pub const F367CAB_CRL_DFE: c_uint = 0xf4600080;
pub const F367CAB_DFE_START: c_uint = 0xf4600040;
pub const F367CAB_CTRLG_START: c_uint = 0xf4600030;
pub const F367CAB_FSM_FORCESTATE: c_uint = 0xf460000f;
// FSM_CTL
pub const R367CAB_FSM_CTL: c_uint = 0xf461;
pub const F367CAB_FEC2_EN: c_uint = 0xf4610040;
pub const F367CAB_SIT_EN: c_uint = 0xf4610020;
pub const F367CAB_TRL_AHEAD: c_uint = 0xf4610010;
pub const F367CAB_TRL2_EN: c_uint = 0xf4610008;
pub const F367CAB_FSM_EQA1_EN: c_uint = 0xf4610004;
pub const F367CAB_FSM_BKP_DIS: c_uint = 0xf4610002;
pub const F367CAB_FSM_FORCE_EN: c_uint = 0xf4610001;
// FSM_STS
pub const R367CAB_FSM_STS: c_uint = 0xf462;
pub const F367CAB_FSM_STATUS: c_uint = 0xf462000f;
// FSM_SNR0_HTH
pub const R367CAB_FSM_SNR0_HTH: c_uint = 0xf463;
pub const F367CAB_SNR0_HTH: c_uint = 0xf46300ff;
// FSM_SNR1_HTH
pub const R367CAB_FSM_SNR1_HTH: c_uint = 0xf464;
pub const F367CAB_SNR1_HTH: c_uint = 0xf46400ff;
// FSM_SNR2_HTH
pub const R367CAB_FSM_SNR2_HTH: c_uint = 0xf465;
pub const F367CAB_SNR2_HTH: c_uint = 0xf46500ff;
// FSM_SNR0_LTH
pub const R367CAB_FSM_SNR0_LTH: c_uint = 0xf466;
pub const F367CAB_SNR0_LTH: c_uint = 0xf46600ff;
// FSM_SNR1_LTH
pub const R367CAB_FSM_SNR1_LTH: c_uint = 0xf467;
pub const F367CAB_SNR1_LTH: c_uint = 0xf46700ff;
// FSM_EQA1_HTH
pub const R367CAB_FSM_EQA1_HTH: c_uint = 0xf468;
pub const F367CAB_SNR3_HTH_LO: c_uint = 0xf46800f0;
pub const F367CAB_EQA1_HTH: c_uint = 0xf468000f;
// FSM_TEMPO
pub const R367CAB_FSM_TEMPO: c_uint = 0xf469;
pub const F367CAB_SIT: c_uint = 0xf46900c0;
pub const F367CAB_WST: c_uint = 0xf4690038;
pub const F367CAB_ELT: c_uint = 0xf4690006;
pub const F367CAB_SNR3_HTH_HI: c_uint = 0xf4690001;
// FSM_CONFIG
pub const R367CAB_FSM_CONFIG: c_uint = 0xf46a;
pub const F367CAB_FEC2_DFEOFF: c_uint = 0xf46a0004;
pub const F367CAB_PRIT_STATE: c_uint = 0xf46a0002;
pub const F367CAB_MODMAP_STATE: c_uint = 0xf46a0001;
// EQU_I_TESTTAP_L
pub const R367CAB_EQU_I_TESTTAP_L: c_uint = 0xf474;
pub const F367CAB_I_TEST_TAP_L: c_uint = 0xf47400ff;
// EQU_I_TESTTAP_M
pub const R367CAB_EQU_I_TESTTAP_M: c_uint = 0xf475;
pub const F367CAB_I_TEST_TAP_M: c_uint = 0xf47500ff;
// EQU_I_TESTTAP_H
pub const R367CAB_EQU_I_TESTTAP_H: c_uint = 0xf476;
pub const F367CAB_I_TEST_TAP_H: c_uint = 0xf476001f;
// EQU_TESTAP_CFG
pub const R367CAB_EQU_TESTAP_CFG: c_uint = 0xf477;
pub const F367CAB_TEST_FFE_DFE_SEL: c_uint = 0xf4770040;
pub const F367CAB_TEST_TAP_SELECT: c_uint = 0xf477003f;
// EQU_Q_TESTTAP_L
pub const R367CAB_EQU_Q_TESTTAP_L: c_uint = 0xf478;
pub const F367CAB_Q_TEST_TAP_L: c_uint = 0xf47800ff;
// EQU_Q_TESTTAP_M
pub const R367CAB_EQU_Q_TESTTAP_M: c_uint = 0xf479;
pub const F367CAB_Q_TEST_TAP_M: c_uint = 0xf47900ff;
// EQU_Q_TESTTAP_H
pub const R367CAB_EQU_Q_TESTTAP_H: c_uint = 0xf47a;
pub const F367CAB_Q_TEST_TAP_H: c_uint = 0xf47a001f;
// EQU_TAP_CTRL
pub const R367CAB_EQU_TAP_CTRL: c_uint = 0xf47b;
pub const F367CAB_MTAP_FRZ: c_uint = 0xf47b0010;
pub const F367CAB_PRE_FREEZE: c_uint = 0xf47b0008;
pub const F367CAB_DFE_TAPMON_EN: c_uint = 0xf47b0004;
pub const F367CAB_FFE_TAPMON_EN: c_uint = 0xf47b0002;
pub const F367CAB_MTAP_ONLY: c_uint = 0xf47b0001;
// EQU_CTR_CRL_CONTROL_L
pub const R367CAB_EQU_CTR_CRL_CONTROL_L: c_uint = 0xf47c;
pub const F367CAB_EQU_CTR_CRL_CONTROL_LO: c_uint = 0xf47c00ff;
// EQU_CTR_CRL_CONTROL_H
pub const R367CAB_EQU_CTR_CRL_CONTROL_H: c_uint = 0xf47d;
pub const F367CAB_EQU_CTR_CRL_CONTROL_HI: c_uint = 0xf47d00ff;
// EQU_CTR_HIPOW_L
pub const R367CAB_EQU_CTR_HIPOW_L: c_uint = 0xf47e;
pub const F367CAB_CTR_HIPOW_L: c_uint = 0xf47e00ff;
// EQU_CTR_HIPOW_H
pub const R367CAB_EQU_CTR_HIPOW_H: c_uint = 0xf47f;
pub const F367CAB_CTR_HIPOW_H: c_uint = 0xf47f00ff;
// EQU_I_EQU_LO
pub const R367CAB_EQU_I_EQU_LO: c_uint = 0xf480;
pub const F367CAB_EQU_I_EQU_L: c_uint = 0xf48000ff;
// EQU_I_EQU_HI
pub const R367CAB_EQU_I_EQU_HI: c_uint = 0xf481;
pub const F367CAB_EQU_I_EQU_H: c_uint = 0xf4810003;
// EQU_Q_EQU_LO
pub const R367CAB_EQU_Q_EQU_LO: c_uint = 0xf482;
pub const F367CAB_EQU_Q_EQU_L: c_uint = 0xf48200ff;
// EQU_Q_EQU_HI
pub const R367CAB_EQU_Q_EQU_HI: c_uint = 0xf483;
pub const F367CAB_EQU_Q_EQU_H: c_uint = 0xf4830003;
// EQU_MAPPER
pub const R367CAB_EQU_MAPPER: c_uint = 0xf484;
pub const F367CAB_QUAD_AUTO: c_uint = 0xf4840080;
pub const F367CAB_QUAD_INV: c_uint = 0xf4840040;
pub const F367CAB_QAM_MODE: c_uint = 0xf4840007;
// EQU_SWEEP_RATE
pub const R367CAB_EQU_SWEEP_RATE: c_uint = 0xf485;
pub const F367CAB_SNR_PER: c_uint = 0xf48500c0;
pub const F367CAB_SWEEP_RATE: c_uint = 0xf485003f;
// EQU_SNR_LO
pub const R367CAB_EQU_SNR_LO: c_uint = 0xf486;
pub const F367CAB_SNR_LO: c_uint = 0xf48600ff;
// EQU_SNR_HI
pub const R367CAB_EQU_SNR_HI: c_uint = 0xf487;
pub const F367CAB_SNR_HI: c_uint = 0xf48700ff;
// EQU_GAMMA_LO
pub const R367CAB_EQU_GAMMA_LO: c_uint = 0xf488;
pub const F367CAB_GAMMA_LO: c_uint = 0xf48800ff;
// EQU_GAMMA_HI
pub const R367CAB_EQU_GAMMA_HI: c_uint = 0xf489;
pub const F367CAB_GAMMA_ME: c_uint = 0xf48900ff;
// EQU_ERR_GAIN
pub const R367CAB_EQU_ERR_GAIN: c_uint = 0xf48a;
pub const F367CAB_EQA1MU: c_uint = 0xf48a0070;
pub const F367CAB_CRL2MU: c_uint = 0xf48a000e;
pub const F367CAB_GAMMA_HI: c_uint = 0xf48a0001;
// EQU_RADIUS
pub const R367CAB_EQU_RADIUS: c_uint = 0xf48b;
pub const F367CAB_RADIUS: c_uint = 0xf48b00ff;
// EQU_FFE_MAINTAP
pub const R367CAB_EQU_FFE_MAINTAP: c_uint = 0xf48c;
pub const F367CAB_FFE_MAINTAP_INIT: c_uint = 0xf48c00ff;
// EQU_FFE_LEAKAGE
pub const R367CAB_EQU_FFE_LEAKAGE: c_uint = 0xf48e;
pub const F367CAB_LEAK_PER: c_uint = 0xf48e00f0;
pub const F367CAB_EQU_OUTSEL: c_uint = 0xf48e0002;
pub const F367CAB_PNT2dFE: c_uint = 0xf48e0001;
// EQU_FFE_MAINTAP_POS
pub const R367CAB_EQU_FFE_MAINTAP_POS: c_uint = 0xf48f;
pub const F367CAB_FFE_LEAK_EN: c_uint = 0xf48f0080;
pub const F367CAB_DFE_LEAK_EN: c_uint = 0xf48f0040;
pub const F367CAB_FFE_MAINTAP_POS: c_uint = 0xf48f003f;
// EQU_GAIN_WIDE
pub const R367CAB_EQU_GAIN_WIDE: c_uint = 0xf490;
pub const F367CAB_DFE_GAIN_WIDE: c_uint = 0xf49000f0;
pub const F367CAB_FFE_GAIN_WIDE: c_uint = 0xf490000f;
// EQU_GAIN_NARROW
pub const R367CAB_EQU_GAIN_NARROW: c_uint = 0xf491;
pub const F367CAB_DFE_GAIN_NARROW: c_uint = 0xf49100f0;
pub const F367CAB_FFE_GAIN_NARROW: c_uint = 0xf491000f;
// EQU_CTR_LPF_GAIN
pub const R367CAB_EQU_CTR_LPF_GAIN: c_uint = 0xf492;
pub const F367CAB_CTR_GTO: c_uint = 0xf4920080;
pub const F367CAB_CTR_GDIR: c_uint = 0xf4920070;
pub const F367CAB_SWEEP_EN: c_uint = 0xf4920008;
pub const F367CAB_CTR_GINT: c_uint = 0xf4920007;
// EQU_CRL_LPF_GAIN
pub const R367CAB_EQU_CRL_LPF_GAIN: c_uint = 0xf493;
pub const F367CAB_CRL_GTO: c_uint = 0xf4930080;
pub const F367CAB_CRL_GDIR: c_uint = 0xf4930070;
pub const F367CAB_SWEEP_DIR: c_uint = 0xf4930008;
pub const F367CAB_CRL_GINT: c_uint = 0xf4930007;
// EQU_GLOBAL_GAIN
pub const R367CAB_EQU_GLOBAL_GAIN: c_uint = 0xf494;
pub const F367CAB_CRL_GAIN: c_uint = 0xf49400f8;
pub const F367CAB_CTR_INC_GAIN: c_uint = 0xf4940004;
pub const F367CAB_CTR_FRAC: c_uint = 0xf4940003;
// EQU_CRL_LD_SEN
pub const R367CAB_EQU_CRL_LD_SEN: c_uint = 0xf495;
pub const F367CAB_CTR_BADPOINT_EN: c_uint = 0xf4950080;
pub const F367CAB_CTR_GAIN: c_uint = 0xf4950070;
pub const F367CAB_LIMANEN: c_uint = 0xf4950008;
pub const F367CAB_CRL_LD_SEN: c_uint = 0xf4950007;
// EQU_CRL_LD_VAL
pub const R367CAB_EQU_CRL_LD_VAL: c_uint = 0xf496;
pub const F367CAB_CRL_BISTH_LIMIT: c_uint = 0xf4960080;
pub const F367CAB_CARE_EN: c_uint = 0xf4960040;
pub const F367CAB_CRL_LD_PER: c_uint = 0xf4960030;
pub const F367CAB_CRL_LD_WST: c_uint = 0xf496000c;
pub const F367CAB_CRL_LD_TFS: c_uint = 0xf4960003;
// EQU_CRL_TFR
pub const R367CAB_EQU_CRL_TFR: c_uint = 0xf497;
pub const F367CAB_CRL_LD_TFR: c_uint = 0xf49700ff;
// EQU_CRL_BISTH_LO
pub const R367CAB_EQU_CRL_BISTH_LO: c_uint = 0xf498;
pub const F367CAB_CRL_BISTH_LO: c_uint = 0xf49800ff;
// EQU_CRL_BISTH_HI
pub const R367CAB_EQU_CRL_BISTH_HI: c_uint = 0xf499;
pub const F367CAB_CRL_BISTH_HI: c_uint = 0xf49900ff;
// EQU_SWEEP_RANGE_LO
pub const R367CAB_EQU_SWEEP_RANGE_LO: c_uint = 0xf49a;
pub const F367CAB_SWEEP_RANGE_LO: c_uint = 0xf49a00ff;
// EQU_SWEEP_RANGE_HI
pub const R367CAB_EQU_SWEEP_RANGE_HI: c_uint = 0xf49b;
pub const F367CAB_SWEEP_RANGE_HI: c_uint = 0xf49b00ff;
// EQU_CRL_LIMITER
pub const R367CAB_EQU_CRL_LIMITER: c_uint = 0xf49c;
pub const F367CAB_BISECTOR_EN: c_uint = 0xf49c0080;
pub const F367CAB_PHEST128_EN: c_uint = 0xf49c0040;
pub const F367CAB_CRL_LIM: c_uint = 0xf49c003f;
// EQU_MODULUS_MAP
pub const R367CAB_EQU_MODULUS_MAP: c_uint = 0xf49d;
pub const F367CAB_PNT_DEPTH: c_uint = 0xf49d00e0;
pub const F367CAB_MODULUS_CMP: c_uint = 0xf49d001f;
// EQU_PNT_GAIN
pub const R367CAB_EQU_PNT_GAIN: c_uint = 0xf49e;
pub const F367CAB_PNT_EN: c_uint = 0xf49e0080;
pub const F367CAB_MODULUSMAP_EN: c_uint = 0xf49e0040;
pub const F367CAB_PNT_GAIN: c_uint = 0xf49e003f;
// FEC_AC_CTR_0
pub const R367CAB_FEC_AC_CTR_0: c_uint = 0xf4a8;
pub const F367CAB_BE_BYPASS: c_uint = 0xf4a80020;
pub const F367CAB_REFRESH47: c_uint = 0xf4a80010;
pub const F367CAB_CT_NBST: c_uint = 0xf4a80008;
pub const F367CAB_TEI_ENA: c_uint = 0xf4a80004;
pub const F367CAB_DS_ENA: c_uint = 0xf4a80002;
pub const F367CAB_TSMF_EN: c_uint = 0xf4a80001;
// FEC_AC_CTR_1
pub const R367CAB_FEC_AC_CTR_1: c_uint = 0xf4a9;
pub const F367CAB_DEINT_DEPTH: c_uint = 0xf4a900ff;
// FEC_AC_CTR_2
pub const R367CAB_FEC_AC_CTR_2: c_uint = 0xf4aa;
pub const F367CAB_DEINT_M: c_uint = 0xf4aa00f8;
pub const F367CAB_DIS_UNLOCK: c_uint = 0xf4aa0004;
pub const F367CAB_DESCR_MODE: c_uint = 0xf4aa0003;
// FEC_AC_CTR_3
pub const R367CAB_FEC_AC_CTR_3: c_uint = 0xf4ab;
pub const F367CAB_DI_UNLOCK: c_uint = 0xf4ab0080;
pub const F367CAB_DI_FREEZE: c_uint = 0xf4ab0040;
pub const F367CAB_MISMATCH: c_uint = 0xf4ab0030;
pub const F367CAB_ACQ_MODE: c_uint = 0xf4ab000c;
pub const F367CAB_TRK_MODE: c_uint = 0xf4ab0003;
// FEC_STATUS
pub const R367CAB_FEC_STATUS: c_uint = 0xf4ac;
pub const F367CAB_DEINT_SMCNTR: c_uint = 0xf4ac00e0;
pub const F367CAB_DEINT_SYNCSTATE: c_uint = 0xf4ac0018;
pub const F367CAB_DEINT_SYNLOST: c_uint = 0xf4ac0004;
pub const F367CAB_DESCR_SYNCSTATE: c_uint = 0xf4ac0002;
// RS_COUNTER_0
pub const R367CAB_RS_COUNTER_0: c_uint = 0xf4ae;
pub const F367CAB_BK_CT_L: c_uint = 0xf4ae00ff;
// RS_COUNTER_1
pub const R367CAB_RS_COUNTER_1: c_uint = 0xf4af;
pub const F367CAB_BK_CT_H: c_uint = 0xf4af00ff;
// RS_COUNTER_2
pub const R367CAB_RS_COUNTER_2: c_uint = 0xf4b0;
pub const F367CAB_CORR_CT_L: c_uint = 0xf4b000ff;
// RS_COUNTER_3
pub const R367CAB_RS_COUNTER_3: c_uint = 0xf4b1;
pub const F367CAB_CORR_CT_H: c_uint = 0xf4b100ff;
// RS_COUNTER_4
pub const R367CAB_RS_COUNTER_4: c_uint = 0xf4b2;
pub const F367CAB_UNCORR_CT_L: c_uint = 0xf4b200ff;
// RS_COUNTER_5
pub const R367CAB_RS_COUNTER_5: c_uint = 0xf4b3;
pub const F367CAB_UNCORR_CT_H: c_uint = 0xf4b300ff;
// BERT_0
pub const R367CAB_BERT_0: c_uint = 0xf4b4;
pub const F367CAB_RS_NOCORR: c_uint = 0xf4b40004;
pub const F367CAB_CT_HOLD: c_uint = 0xf4b40002;
pub const F367CAB_CT_CLEAR: c_uint = 0xf4b40001;
// BERT_1
pub const R367CAB_BERT_1: c_uint = 0xf4b5;
pub const F367CAB_BERT_ON: c_uint = 0xf4b50020;
pub const F367CAB_BERT_ERR_SRC: c_uint = 0xf4b50010;
pub const F367CAB_BERT_ERR_MODE: c_uint = 0xf4b50008;
pub const F367CAB_BERT_NBYTE: c_uint = 0xf4b50007;
// BERT_2
pub const R367CAB_BERT_2: c_uint = 0xf4b6;
pub const F367CAB_BERT_ERRCOUNT_L: c_uint = 0xf4b600ff;
// BERT_3
pub const R367CAB_BERT_3: c_uint = 0xf4b7;
pub const F367CAB_BERT_ERRCOUNT_H: c_uint = 0xf4b700ff;
// OUTFORMAT_0
pub const R367CAB_OUTFORMAT_0: c_uint = 0xf4b8;
pub const F367CAB_CLK_POLARITY: c_uint = 0xf4b80080;
pub const F367CAB_FEC_TYPE: c_uint = 0xf4b80040;
pub const F367CAB_SYNC_STRIP: c_uint = 0xf4b80008;
pub const F367CAB_TS_SWAP: c_uint = 0xf4b80004;
pub const F367CAB_OUTFORMAT: c_uint = 0xf4b80003;
// OUTFORMAT_1
pub const R367CAB_OUTFORMAT_1: c_uint = 0xf4b9;
pub const F367CAB_CI_DIVRANGE: c_uint = 0xf4b900ff;
// SMOOTHER_2
pub const R367CAB_SMOOTHER_2: c_uint = 0xf4be;
pub const F367CAB_FIFO_BYPASS: c_uint = 0xf4be0020;
// TSMF_CTRL_0
pub const R367CAB_TSMF_CTRL_0: c_uint = 0xf4c0;
pub const F367CAB_TS_NUMBER: c_uint = 0xf4c0001e;
pub const F367CAB_SEL_MODE: c_uint = 0xf4c00001;
// TSMF_CTRL_1
pub const R367CAB_TSMF_CTRL_1: c_uint = 0xf4c1;
pub const F367CAB_CHECK_ERROR_BIT: c_uint = 0xf4c10080;
pub const F367CAB_CHCK_F_SYNC: c_uint = 0xf4c10040;
pub const F367CAB_H_MODE: c_uint = 0xf4c10008;
pub const F367CAB_D_V_MODE: c_uint = 0xf4c10004;
pub const F367CAB_MODE: c_uint = 0xf4c10003;
// TSMF_CTRL_3
pub const R367CAB_TSMF_CTRL_3: c_uint = 0xf4c3;
pub const F367CAB_SYNC_IN_COUNT: c_uint = 0xf4c300f0;
pub const F367CAB_SYNC_OUT_COUNT: c_uint = 0xf4c3000f;
// TS_ON_ID_0
pub const R367CAB_TS_ON_ID_0: c_uint = 0xf4c4;
pub const F367CAB_TS_ID_L: c_uint = 0xf4c400ff;
// TS_ON_ID_1
pub const R367CAB_TS_ON_ID_1: c_uint = 0xf4c5;
pub const F367CAB_TS_ID_H: c_uint = 0xf4c500ff;
// TS_ON_ID_2
pub const R367CAB_TS_ON_ID_2: c_uint = 0xf4c6;
pub const F367CAB_ON_ID_L: c_uint = 0xf4c600ff;
// TS_ON_ID_3
pub const R367CAB_TS_ON_ID_3: c_uint = 0xf4c7;
pub const F367CAB_ON_ID_H: c_uint = 0xf4c700ff;
// RE_STATUS_0
pub const R367CAB_RE_STATUS_0: c_uint = 0xf4c8;
pub const F367CAB_RECEIVE_STATUS_L: c_uint = 0xf4c800ff;
// RE_STATUS_1
pub const R367CAB_RE_STATUS_1: c_uint = 0xf4c9;
pub const F367CAB_RECEIVE_STATUS_LH: c_uint = 0xf4c900ff;
// RE_STATUS_2
pub const R367CAB_RE_STATUS_2: c_uint = 0xf4ca;
pub const F367CAB_RECEIVE_STATUS_HL: c_uint = 0xf4ca00ff;
// RE_STATUS_3
pub const R367CAB_RE_STATUS_3: c_uint = 0xf4cb;
pub const F367CAB_RECEIVE_STATUS_HH: c_uint = 0xf4cb003f;
// TS_STATUS_0
pub const R367CAB_TS_STATUS_0: c_uint = 0xf4cc;
pub const F367CAB_TS_STATUS_L: c_uint = 0xf4cc00ff;
// TS_STATUS_1
pub const R367CAB_TS_STATUS_1: c_uint = 0xf4cd;
pub const F367CAB_TS_STATUS_H: c_uint = 0xf4cd007f;
// TS_STATUS_2
pub const R367CAB_TS_STATUS_2: c_uint = 0xf4ce;
pub const F367CAB_ERROR: c_uint = 0xf4ce0080;
pub const F367CAB_EMERGENCY: c_uint = 0xf4ce0040;
pub const F367CAB_CRE_TS: c_uint = 0xf4ce0030;
pub const F367CAB_VER: c_uint = 0xf4ce000e;
pub const F367CAB_M_LOCK: c_uint = 0xf4ce0001;
// TS_STATUS_3
pub const R367CAB_TS_STATUS_3: c_uint = 0xf4cf;
pub const F367CAB_UPDATE_READY: c_uint = 0xf4cf0080;
pub const F367CAB_END_FRAME_HEADER: c_uint = 0xf4cf0040;
pub const F367CAB_CONTCNT: c_uint = 0xf4cf0020;
pub const F367CAB_TS_IDENTIFIER_SEL: c_uint = 0xf4cf000f;
// T_O_ID_0
pub const R367CAB_T_O_ID_0: c_uint = 0xf4d0;
pub const F367CAB_ON_ID_I_L: c_uint = 0xf4d000ff;
// T_O_ID_1
pub const R367CAB_T_O_ID_1: c_uint = 0xf4d1;
pub const F367CAB_ON_ID_I_H: c_uint = 0xf4d100ff;
// T_O_ID_2
pub const R367CAB_T_O_ID_2: c_uint = 0xf4d2;
pub const F367CAB_TS_ID_I_L: c_uint = 0xf4d200ff;
// T_O_ID_3
pub const R367CAB_T_O_ID_3: c_uint = 0xf4d3;
pub const F367CAB_TS_ID_I_H: c_uint = 0xf4d300ff;
