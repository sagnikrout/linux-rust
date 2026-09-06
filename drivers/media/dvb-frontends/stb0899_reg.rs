//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stb0899_reg.h
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
// S1
pub const STB0899_DEV_ID: c_uint = 0xf000;

pub const STB0899_OFFST_CHIP_ID: c_int = 4;
pub const STB0899_WIDTH_CHIP_ID: c_int = 4;

pub const STB0899_OFFST_CHIP_REL: c_int = 0;
pub const STB0899_WIDTH_CHIP_REL: c_int = 4;
pub const STB0899_DEMOD: c_uint = 0xf40e;

pub const STB0899_OFFST_MODECOEFF: c_int = 0;
pub const STB0899_WIDTH_MODECOEFF: c_int = 1;
pub const STB0899_RCOMPC: c_uint = 0xf410;
pub const STB0899_AGC1CN: c_uint = 0xf412;
pub const STB0899_AGC1REF: c_uint = 0xf413;
pub const STB0899_RTC: c_uint = 0xf417;
pub const STB0899_TMGCFG: c_uint = 0xf418;
pub const STB0899_AGC2REF: c_uint = 0xf419;
pub const STB0899_TLSR: c_uint = 0xf41a;
pub const STB0899_CFD: c_uint = 0xf41b;

pub const STB0899_OFFST_CFD_ON: c_int = 7;
pub const STB0899_WIDTH_CFD_ON: c_int = 1;
pub const STB0899_ACLC: c_uint = 0xf41c;
pub const STB0899_BCLC: c_uint = 0xf41d;
pub const STB0899_OFFST_ALGO: c_int = 6;
pub const STB0899_WIDTH_ALGO_QPSK2: c_int = 2;

pub const STB0899_OFFST_BETA: c_int = 0;
pub const STB0899_WIDTH_BETA: c_int = 6;
pub const STB0899_EQON: c_uint = 0xf41e;
pub const STB0899_LDT: c_uint = 0xf41f;
pub const STB0899_LDT2: c_uint = 0xf420;
pub const STB0899_EQUALREF: c_uint = 0xf425;
pub const STB0899_TMGRAMP: c_uint = 0xf426;
pub const STB0899_TMGTHD: c_uint = 0xf427;
pub const STB0899_IDCCOMP: c_uint = 0xf428;
pub const STB0899_QDCCOMP: c_uint = 0xf429;
pub const STB0899_POWERI: c_uint = 0xf42a;
pub const STB0899_POWERQ: c_uint = 0xf42b;
pub const STB0899_RCOMP: c_uint = 0xf42c;
pub const STB0899_AGCIQIN: c_uint = 0xf42e;

pub const STB0899_OFFST_AGCIQVALUE: c_int = 0;
pub const STB0899_WIDTH_AGCIQVALUE: c_int = 8;
pub const STB0899_AGC2I1: c_uint = 0xf436;
pub const STB0899_AGC2I2: c_uint = 0xf437;
pub const STB0899_TLIR: c_uint = 0xf438;

pub const STB0899_OFFST_TLIR_TMG_LOCK_IND: c_int = 0;
pub const STB0899_WIDTH_TLIR_TMG_LOCK_IND: c_int = 8;
pub const STB0899_RTF: c_uint = 0xf439;

pub const STB0899_OFFST_RTF_TIMING_LOOP_FREQ: c_int = 0;
pub const STB0899_WIDTH_RTF_TIMING_LOOP_FREQ: c_int = 8;
pub const STB0899_DSTATUS: c_uint = 0xf43a;

pub const STB0899_OFFST_CARRIER_FOUND: c_int = 7;
pub const STB0899_WIDTH_CARRIER_FOUND: c_int = 1;

pub const STB0899_OFFST_TMG_LOCK: c_int = 6;
pub const STB0899_WIDTH_TMG_LOCK: c_int = 1;

pub const STB0899_OFFST_DEMOD_LOCK: c_int = 5;
pub const STB0899_WIDTH_DEMOD_LOCK: c_int = 1;

pub const STB0899_OFFST_TMG_AUTO: c_int = 4;
pub const STB0899_WIDTH_TMG_AUTO: c_int = 1;

pub const STB0899_OFFST_END_MAIN: c_int = 3;
pub const STB0899_WIDTH_END_MAIN: c_int = 1;
pub const STB0899_LDI: c_uint = 0xf43b;
pub const STB0899_OFFST_LDI: c_int = 0;
pub const STB0899_WIDTH_LDI: c_int = 8;
pub const STB0899_CFRM: c_uint = 0xf43e;
pub const STB0899_OFFST_CFRM: c_int = 0;
pub const STB0899_WIDTH_CFRM: c_int = 8;
pub const STB0899_CFRL: c_uint = 0xf43f;
pub const STB0899_OFFST_CFRL: c_int = 0;
pub const STB0899_WIDTH_CFRL: c_int = 8;
pub const STB0899_NIRM: c_uint = 0xf440;
pub const STB0899_OFFST_NIRM: c_int = 0;
pub const STB0899_WIDTH_NIRM: c_int = 8;
pub const STB0899_NIRL: c_uint = 0xf441;
pub const STB0899_OFFST_NIRL: c_int = 0;
pub const STB0899_WIDTH_NIRL: c_int = 8;
pub const STB0899_ISYMB: c_uint = 0xf444;
pub const STB0899_QSYMB: c_uint = 0xf445;
pub const STB0899_SFRH: c_uint = 0xf446;
pub const STB0899_OFFST_SFRH: c_int = 0;
pub const STB0899_WIDTH_SFRH: c_int = 8;
pub const STB0899_SFRM: c_uint = 0xf447;
pub const STB0899_OFFST_SFRM: c_int = 0;
pub const STB0899_WIDTH_SFRM: c_int = 8;
pub const STB0899_SFRL: c_uint = 0xf448;
pub const STB0899_OFFST_SFRL: c_int = 4;
pub const STB0899_WIDTH_SFRL: c_int = 4;
pub const STB0899_SFRUPH: c_uint = 0xf44c;
pub const STB0899_SFRUPM: c_uint = 0xf44d;
pub const STB0899_SFRUPL: c_uint = 0xf44e;
pub const STB0899_EQUAI1: c_uint = 0xf4e0;
pub const STB0899_EQUAQ1: c_uint = 0xf4e1;
pub const STB0899_EQUAI2: c_uint = 0xf4e2;
pub const STB0899_EQUAQ2: c_uint = 0xf4e3;
pub const STB0899_EQUAI3: c_uint = 0xf4e4;
pub const STB0899_EQUAQ3: c_uint = 0xf4e5;
pub const STB0899_EQUAI4: c_uint = 0xf4e6;
pub const STB0899_EQUAQ4: c_uint = 0xf4e7;
pub const STB0899_EQUAI5: c_uint = 0xf4e8;
pub const STB0899_EQUAQ5: c_uint = 0xf4e9;
pub const STB0899_DSTATUS2: c_uint = 0xf50c;

pub const STB8999_OFFST_DS2_TMG_AUTOSRCH: c_int = 7;
pub const STB0899_WIDTH_DS2_TMG_AUTOSRCH: c_int = 1;

pub const STB0899_OFFST_DS2_END_MAINLOOP: c_int = 6;
pub const STB0899_WIDTH_DS2_END_MAINLOOP: c_int = 1;

pub const STB0899_OFFST_DS2_CFSYNC: c_int = 5;
pub const STB0899_WIDTH_DS2_CFSYNC: c_int = 1;

pub const STB0899_OFFST_DS2_TMGLOCK: c_int = 4;
pub const STB0899_WIDTH_DS2_TMGLOCK: c_int = 1;

pub const STB0899_OFFST_DS2_DEMODWAIT: c_int = 3;
pub const STB0899_WIDTH_DS2_DEMODWAIT: c_int = 1;

pub const STB0899_OFFST_DS2_FECON: c_int = 1;
pub const STB0899_WIDTH_DS2_FECON: c_int = 1;
// S1 FEC
pub const STB0899_VSTATUS: c_uint = 0xf50d;

pub const STB0899_OFFST_VSTATUS_VITERBI_ON: c_int = 7;
pub const STB0899_WIDTH_VSTATUS_VITERBI_ON: c_int = 1;

pub const STB0899_OFFST_VSTATUS_END_LOOPVIT: c_int = 6;
pub const STB0899_WIDTH_VSTATUS_END_LOOPVIT: c_int = 1;

pub const STB0899_OFFST_VSTATUS_PRFVIT: c_int = 4;
pub const STB0899_WIDTH_VSTATUS_PRFVIT: c_int = 1;

pub const STB0899_OFFST_VSTATUS_LOCKEDVIT: c_int = 3;
pub const STB0899_WIDTH_VSTATUS_LOCKEDVIT: c_int = 1;
pub const STB0899_VERROR: c_uint = 0xf50f;
pub const STB0899_IQSWAP: c_uint = 0xf523;

pub const STB0899_OFFST_SYM: c_int = 3;
pub const STB0899_WIDTH_SYM: c_int = 1;
pub const STB0899_FECAUTO1: c_uint = 0xf530;

pub const STB0899_OFFST_DSSSRCH: c_int = 3;
pub const STB0899_WIDTH_DSSSRCH: c_int = 1;

pub const STB0899_OFFST_SYMSRCH: c_int = 2;
pub const STB0899_WIDTH_SYMSRCH: c_int = 1;

pub const STB0899_OFFST_QPSKSRCH: c_int = 1;
pub const STB0899_WIDTH_QPSKSRCH: c_int = 1;

pub const STB0899_OFFST_BPSKSRCH: c_int = 0;
pub const STB0899_WIDTH_BPSKSRCH: c_int = 1;
pub const STB0899_FECM: c_uint = 0xf533;

pub const STB0899_OFFST_FECM_NOT_DVB: c_int = 7;
pub const STB0899_WIDTH_FECM_NOT_DVB: c_int = 1;

pub const STB0899_OFFST_FECM_RSVD1: c_int = 4;
pub const STB0899_WIDTH_FECM_RSVD1: c_int = 3;

pub const STB0899_OFFST_FECM_VITERBI_ON: c_int = 3;
pub const STB0899_WIDTH_FECM_VITERBI_ON: c_int = 1;

pub const STB0899_OFFST_FECM_RSVD0: c_int = 2;
pub const STB0899_WIDTH_FECM_RSVD0: c_int = 1;

pub const STB0899_OFFST_FECM_SYNCDIS: c_int = 1;
pub const STB0899_WIDTH_FECM_SYNCDIS: c_int = 1;

pub const STB0899_OFFST_FECM_SYMI: c_int = 0;
pub const STB0899_WIDTH_FECM_SYMI: c_int = 1;
pub const STB0899_VTH12: c_uint = 0xf534;
pub const STB0899_VTH23: c_uint = 0xf535;
pub const STB0899_VTH34: c_uint = 0xf536;
pub const STB0899_VTH56: c_uint = 0xf537;
pub const STB0899_VTH67: c_uint = 0xf538;
pub const STB0899_VTH78: c_uint = 0xf539;
pub const STB0899_PRVIT: c_uint = 0xf53c;

pub const STB0899_OFFST_PR_7_8: c_int = 5;
pub const STB0899_WIDTH_PR_7_8: c_int = 1;

pub const STB0899_OFFST_PR_6_7: c_int = 4;
pub const STB0899_WIDTH_PR_6_7: c_int = 1;

pub const STB0899_OFFST_PR_5_6: c_int = 3;
pub const STB0899_WIDTH_PR_5_6: c_int = 1;

pub const STB0899_OFFST_PR_3_4: c_int = 2;
pub const STB0899_WIDTH_PR_3_4: c_int = 1;

pub const STB0899_OFFST_PR_2_3: c_int = 1;
pub const STB0899_WIDTH_PR_2_3: c_int = 1;

pub const STB0899_OFFST_PR_1_2: c_int = 0;
pub const STB0899_WIDTH_PR_1_2: c_int = 1;
pub const STB0899_VITSYNC: c_uint = 0xf53d;

pub const STB0899_OFFST_AM: c_int = 7;
pub const STB0899_WIDTH_AM: c_int = 1;

pub const STB0899_OFFST_FREEZE: c_int = 6;
pub const STB0899_WIDTH_FREEZE: c_int = 1;

pub const STB0899_OFFST_SN_65536: c_int = 4;
pub const STB0899_WIDTH_SN_65536: c_int = 2;

pub const STB0899_OFFST_SN_16384: c_int = 5;
pub const STB0899_WIDTH_SN_16384: c_int = 1;

pub const STB0899_OFFST_SN_4096: c_int = 4;
pub const STB0899_WIDTH_SN_4096: c_int = 1;

pub const STB0899_OFFST_SN_1024: c_int = 4;
pub const STB0899_WIDTH_SN_1024: c_int = 0;

pub const STB0899_OFFST_TO_128: c_int = 2;
pub const STB0899_WIDTH_TO_128: c_int = 2;

pub const STB0899_OFFST_TO_64: c_int = 3;
pub const STB0899_WIDTH_TO_64: c_int = 1;

pub const STB0899_OFFST_TO_32: c_int = 2;
pub const STB0899_WIDTH_TO_32: c_int = 1;

pub const STB0899_OFFST_TO_16: c_int = 2;
pub const STB0899_WIDTH_TO_16: c_int = 0;

pub const STB0899_OFFST_HYST_128: c_int = 1;
pub const STB0899_WIDTH_HYST_128: c_int = 2;

pub const STB0899_OFFST_HYST_64: c_int = 1;
pub const STB0899_WIDTH_HYST_64: c_int = 1;

pub const STB0899_OFFST_HYST_32: c_int = 0;
pub const STB0899_WIDTH_HYST_32: c_int = 1;

pub const STB0899_OFFST_HYST_16: c_int = 0;
pub const STB0899_WIDTH_HYST_16: c_int = 0;
pub const STB0899_RSULC: c_uint = 0xf548;

pub const STB0899_OFFST_ULDIL_ON: c_int = 7;
pub const STB0899_WIDTH_ULDIL_ON: c_int = 1;

pub const STB0899_OFFST_ULAUTO_ON: c_int = 6;
pub const STB0899_WIDTH_ULAUTO_ON: c_int = 1;

pub const STB0899_OFFST_ULRS_ON: c_int = 5;
pub const STB0899_WIDTH_ULRS_ON: c_int = 1;

pub const STB0899_OFFST_ULDESCRAM_ON: c_int = 4;
pub const STB0899_WIDTH_ULDESCRAM_ON: c_int = 1;

pub const STB0899_OFFST_UL_DISABLE: c_int = 2;
pub const STB0899_WIDTH_UL_DISABLE: c_int = 1;

pub const STB0899_OFFST_NOFTHRESHOLD: c_int = 0;
pub const STB0899_WIDTH_NOFTHRESHOLD: c_int = 1;
pub const STB0899_RSLLC: c_uint = 0xf54a;
pub const STB0899_DEMAPVIT: c_uint = 0xf583;

pub const STB0899_OFFST_DEMAPVIT_RSVD: c_int = 7;
pub const STB0899_WIDTH_DEMAPVIT_RSVD: c_int = 1;

pub const STB0899_OFFST_DEMAPVIT_KDIVIDER: c_int = 0;
pub const STB0899_WIDTH_DEMAPVIT_KDIVIDER: c_int = 7;
pub const STB0899_PLPARM: c_uint = 0xf58c;

pub const STB0899_OFFST_VITMAPPING: c_int = 5;
pub const STB0899_WIDTH_VITMAPPING: c_int = 3;

pub const STB0899_OFFST_VITMAPPING_BPSK: c_int = 5;
pub const STB0899_WIDTH_VITMAPPING_BPSK: c_int = 1;

pub const STB0899_OFFST_VITMAPPING_QPSK: c_int = 5;
pub const STB0899_WIDTH_VITMAPPING_QPSK: c_int = 0;

pub const STB0899_OFFST_VITCURPUN: c_int = 0;
pub const STB0899_WIDTH_VITCURPUN: c_int = 5;

// S2 DEMOD
pub const STB0899_OFF0_DMD_STATUS: c_uint = 0xf300;
pub const STB0899_BASE_DMD_STATUS: c_uint = 0x00000000;

pub const STB0899_OFFST_IF_AGC_LOCK: c_int = 0;
pub const STB0899_WIDTH_IF_AGC_LOCK: c_int = 1;
pub const STB0899_OFF0_CRL_FREQ: c_uint = 0xf304;
pub const STB0899_BASE_CRL_FREQ: c_uint = 0x00000000;

pub const STB0899_OFFST_CARR_FREQ: c_int = 0;
pub const STB0899_WIDTH_CARR_FREQ: c_int = 30;
pub const STB0899_OFF0_BTR_FREQ: c_uint = 0xf308;
pub const STB0899_BASE_BTR_FREQ: c_uint = 0x00000000;

pub const STB0899_OFFST_BTR_FREQ: c_int = 0;
pub const STB0899_WIDTH_BTR_FREQ: c_int = 28;
pub const STB0899_OFF0_IF_AGC_GAIN: c_uint = 0xf30c;
pub const STB0899_BASE_IF_AGC_GAIN: c_uint = 0x00000000;

pub const STB0899_OFFST_IF_AGC_GAIN: c_int = 0;
pub const STB0899_WIDTH_IF_AGC_GAIN: c_int = 14;
pub const STB0899_OFF0_BB_AGC_GAIN: c_uint = 0xf310;
pub const STB0899_BASE_BB_AGC_GAIN: c_uint = 0x00000000;

pub const STB0899_OFFST_BB_AGC_GAIN: c_int = 0;
pub const STB0899_WIDTH_BB_AGC_GAIN: c_int = 14;
pub const STB0899_OFF0_DC_OFFSET: c_uint = 0xf314;
pub const STB0899_BASE_DC_OFFSET: c_uint = 0x00000000;

pub const STB0899_OFFST_I: c_int = 8;
pub const STB0899_WIDTH_I: c_int = 8;

pub const STB0899_OFFST_Q: c_int = 8;
pub const STB0899_WIDTH_Q: c_int = 8;
pub const STB0899_OFF0_DMD_CNTRL: c_uint = 0xf31c;
pub const STB0899_BASE_DMD_CNTRL: c_uint = 0x00000000;

pub const STB0899_OFFST_ADC0_PINS1IN: c_int = 6;
pub const STB0899_WIDTH_ADC0_PINS1IN: c_int = 1;

pub const STB0899_OFFST_IN2COMP1_OFFBIN0: c_int = 3;
pub const STB0899_WIDTH_IN2COMP1_OFFBIN0: c_int = 1;

pub const STB0899_OFFST_DC_COMP: c_int = 2;
pub const STB0899_WIDTH_DC_COMP: c_int = 1;

pub const STB0899_OFFST_MODMODE: c_int = 0;
pub const STB0899_WIDTH_MODMODE: c_int = 2;
pub const STB0899_OFF0_IF_AGC_CNTRL: c_uint = 0xf320;
pub const STB0899_BASE_IF_AGC_CNTRL: c_uint = 0x00000000;

pub const STB0899_OFFST_IF_GAIN_INIT: c_int = 13;
pub const STB0899_WIDTH_IF_GAIN_INIT: c_int = 14;

pub const STB0899_OFFST_IF_GAIN_SENSE: c_int = 12;
pub const STB0899_WIDTH_IF_GAIN_SENSE: c_int = 1;

pub const STB0899_OFFST_IF_LOOP_GAIN: c_int = 8;
pub const STB0899_WIDTH_IF_LOOP_GAIN: c_int = 4;

pub const STB0899_OFFST_IF_LD_GAIN_INIT: c_int = 7;
pub const STB0899_WIDTH_IF_LD_GAIN_INIT: c_int = 1;

pub const STB0899_OFFST_IF_AGC_REF: c_int = 0;
pub const STB0899_WIDTH_IF_AGC_REF: c_int = 7;
pub const STB0899_OFF0_BB_AGC_CNTRL: c_uint = 0xf324;
pub const STB0899_BASE_BB_AGC_CNTRL: c_uint = 0x00000000;

pub const STB0899_OFFST_BB_GAIN_INIT: c_int = 12;
pub const STB0899_WIDTH_BB_GAIN_INIT: c_int = 14;

pub const STB0899_OFFST_BB_LOOP_GAIN: c_int = 8;
pub const STB0899_WIDTH_BB_LOOP_GAIN: c_int = 4;

pub const STB0899_OFFST_BB_LD_GAIN_INIT: c_int = 7;
pub const STB0899_WIDTH_BB_LD_GAIN_INIT: c_int = 1;

pub const STB0899_OFFST_BB_AGC_REF: c_int = 0;
pub const STB0899_WIDTH_BB_AGC_REF: c_int = 7;
pub const STB0899_OFF0_CRL_CNTRL: c_uint = 0xf328;
pub const STB0899_BASE_CRL_CNTRL: c_uint = 0x00000000;

pub const STB0899_OFFST_CRL_LOCK_CLEAR: c_int = 5;
pub const STB0899_WIDTH_CRL_LOCK_CLEAR: c_int = 1;

pub const STB0899_OFFST_CRL_SWPR_CLEAR: c_int = 4;
pub const STB0899_WIDTH_CRL_SWPR_CLEAR: c_int = 1;

pub const STB0899_OFFST_CRL_SWP_ENA: c_int = 3;
pub const STB0899_WIDTH_CRL_SWP_ENA: c_int = 1;

pub const STB0899_OFFST_CRL_DET_SEL: c_int = 2;
pub const STB0899_WIDTH_CRL_DET_SEL: c_int = 1;

pub const STB0899_OFFST_CRL_SENSE: c_int = 1;
pub const STB0899_WIDTH_CRL_SENSE: c_int = 1;

pub const STB0899_OFFST_CRL_PHSERR_CLEAR: c_int = 0;
pub const STB0899_WIDTH_CRL_PHSERR_CLEAR: c_int = 1;
pub const STB0899_OFF0_CRL_PHS_INIT: c_uint = 0xf32c;
pub const STB0899_BASE_CRL_PHS_INIT: c_uint = 0x00000000;

pub const STB0899_OFFST_CRL_PHS_INIT_31: c_int = 30;
pub const STB0899_WIDTH_CRL_PHS_INIT_31: c_int = 1;

pub const STB0899_OFFST_CRL_LD_INIT_PHASE: c_int = 24;
pub const STB0899_WIDTH_CRL_LD_INIT_PHASE: c_int = 1;

pub const STB0899_OFFST_CRL_INIT_PHASE: c_int = 0;
pub const STB0899_WIDTH_CRL_INIT_PHASE: c_int = 24;
pub const STB0899_OFF0_CRL_FREQ_INIT: c_uint = 0xf330;
pub const STB0899_BASE_CRL_FREQ_INIT: c_uint = 0x00000000;

pub const STB0899_OFFST_CRL_FREQ_INIT_31: c_int = 30;
pub const STB0899_WIDTH_CRL_FREQ_INIT_31: c_int = 1;

pub const STB0899_OFFST_CRL_LD_FREQ_INIT: c_int = 24;
pub const STB0899_WIDTH_CRL_LD_FREQ_INIT: c_int = 1;

pub const STB0899_OFFST_CRL_FREQ_INIT: c_int = 0;
pub const STB0899_WIDTH_CRL_FREQ_INIT: c_int = 24;
pub const STB0899_OFF0_CRL_LOOP_GAIN: c_uint = 0xf334;
pub const STB0899_BASE_CRL_LOOP_GAIN: c_uint = 0x00000000;

pub const STB0899_OFFST_KCRL2_RSHFT: c_int = 16;
pub const STB0899_WIDTH_KCRL2_RSHFT: c_int = 4;

pub const STB0899_OFFST_KCRL1: c_int = 12;
pub const STB0899_WIDTH_KCRL1: c_int = 4;

pub const STB0899_OFFST_KCRL1_RSHFT: c_int = 8;
pub const STB0899_WIDTH_KCRL1_RSHFT: c_int = 4;

pub const STB0899_OFFST_KCRL0: c_int = 4;
pub const STB0899_WIDTH_KCRL0: c_int = 4;

pub const STB0899_OFFST_KCRL0_RSHFT: c_int = 0;
pub const STB0899_WIDTH_KCRL0_RSHFT: c_int = 4;
pub const STB0899_OFF0_CRL_NOM_FREQ: c_uint = 0xf338;
pub const STB0899_BASE_CRL_NOM_FREQ: c_uint = 0x00000000;

pub const STB0899_OFFST_CRL_NOM_FREQ: c_int = 0;
pub const STB0899_WIDTH_CRL_NOM_FREQ: c_int = 30;
pub const STB0899_OFF0_CRL_SWP_RATE: c_uint = 0xf33c;
pub const STB0899_BASE_CRL_SWP_RATE: c_uint = 0x00000000;

pub const STB0899_OFFST_CRL_SWP_RATE: c_int = 0;
pub const STB0899_WIDTH_CRL_SWP_RATE: c_int = 30;
pub const STB0899_OFF0_CRL_MAX_SWP: c_uint = 0xf340;
pub const STB0899_BASE_CRL_MAX_SWP: c_uint = 0x00000000;

pub const STB0899_OFFST_CRL_MAX_SWP: c_int = 0;
pub const STB0899_WIDTH_CRL_MAX_SWP: c_int = 30;
pub const STB0899_OFF0_CRL_LK_CNTRL: c_uint = 0xf344;
pub const STB0899_BASE_CRL_LK_CNTRL: c_uint = 0x00000000;
pub const STB0899_OFF0_DECIM_CNTRL: c_uint = 0xf348;
pub const STB0899_BASE_DECIM_CNTRL: c_uint = 0x00000000;

pub const STB0899_OFFST_BAND_LIMIT_B: c_int = 5;
pub const STB0899_WIDTH_BAND_LIMIT_B: c_int = 1;

pub const STB0899_OFFST_WIN_SEL: c_int = 3;
pub const STB0899_WIDTH_WIN_SEL: c_int = 2;

pub const STB0899_OFFST_DECIM_RATE: c_int = 0;
pub const STB0899_WIDTH_DECIM_RATE: c_int = 3;
pub const STB0899_OFF0_BTR_CNTRL: c_uint = 0xf34c;
pub const STB0899_BASE_BTR_CNTRL: c_uint = 0x00000000;

pub const STB0899_OFFST_BTR_FREQ_CORR: c_int = 4;
pub const STB0899_WIDTH_BTR_FREQ_CORR: c_int = 11;

pub const STB0899_OFFST_BTR_CLR_LOCK: c_int = 3;
pub const STB0899_WIDTH_BTR_CLR_LOCK: c_int = 1;

pub const STB0899_OFFST_BTR_SENSE: c_int = 2;
pub const STB0899_WIDTH_BTR_SENSE: c_int = 1;

pub const STB0899_OFFST_BTR_ERR_ENA: c_int = 1;
pub const STB0899_WIDTH_BTR_ERR_ENA: c_int = 1;

pub const STB0899_OFFST_INTRP_PHS_SENSE: c_int = 0;
pub const STB0899_WIDTH_INTRP_PHS_SENSE: c_int = 1;
pub const STB0899_OFF0_BTR_LOOP_GAIN: c_uint = 0xf350;
pub const STB0899_BASE_BTR_LOOP_GAIN: c_uint = 0x00000000;

pub const STB0899_OFFST_KBTR2_RSHFT: c_int = 16;
pub const STB0899_WIDTH_KBTR2_RSHFT: c_int = 4;

pub const STB0899_OFFST_KBTR1: c_int = 12;
pub const STB0899_WIDTH_KBTR1: c_int = 4;

pub const STB0899_OFFST_KBTR1_RSHFT: c_int = 8;
pub const STB0899_WIDTH_KBTR1_RSHFT: c_int = 4;

pub const STB0899_OFFST_KBTR0: c_int = 4;
pub const STB0899_WIDTH_KBTR0: c_int = 4;

pub const STB0899_OFFST_KBTR0_RSHFT: c_int = 0;
pub const STB0899_WIDTH_KBTR0_RSHFT: c_int = 4;
pub const STB0899_OFF0_BTR_PHS_INIT: c_uint = 0xf354;
pub const STB0899_BASE_BTR_PHS_INIT: c_uint = 0x00000000;

pub const STB0899_OFFST_BTR_LD_PHASE_INIT: c_int = 28;
pub const STB0899_WIDTH_BTR_LD_PHASE_INIT: c_int = 1;

pub const STB0899_OFFST_BTR_INIT_PHASE: c_int = 0;
pub const STB0899_WIDTH_BTR_INIT_PHASE: c_int = 28;
pub const STB0899_OFF0_BTR_FREQ_INIT: c_uint = 0xf358;
pub const STB0899_BASE_BTR_FREQ_INIT: c_uint = 0x00000000;

pub const STB0899_OFFST_BTR_LD_FREQ_INIT: c_int = 28;
pub const STB0899_WIDTH_BTR_LD_FREQ_INIT: c_int = 1;

pub const STB0899_OFFST_BTR_FREQ_INIT: c_int = 0;
pub const STB0899_WIDTH_BTR_FREQ_INIT: c_int = 28;
pub const STB0899_OFF0_BTR_NOM_FREQ: c_uint = 0xf35c;
pub const STB0899_BASE_BTR_NOM_FREQ: c_uint = 0x00000000;

pub const STB0899_OFFST_BTR_NOM_FREQ: c_int = 0;
pub const STB0899_WIDTH_BTR_NOM_FREQ: c_int = 28;
pub const STB0899_OFF0_BTR_LK_CNTRL: c_uint = 0xf360;
pub const STB0899_BASE_BTR_LK_CNTRL: c_uint = 0x00000000;

pub const STB0899_OFFST_BTR_MIN_ENERGY: c_int = 24;
pub const STB0899_WIDTH_BTR_MIN_ENERGY: c_int = 4;

pub const STB0899_OFFST_BTR_LOCK_TH_LO: c_int = 16;
pub const STB0899_WIDTH_BTR_LOCK_TH_LO: c_int = 8;

pub const STB0899_OFFST_BTR_LOCK_TH_HI: c_int = 8;
pub const STB0899_WIDTH_BTR_LOCK_TH_HI: c_int = 8;

pub const STB0899_OFFST_BTR_LOCK_GAIN: c_int = 6;
pub const STB0899_WIDTH_BTR_LOCK_GAIN: c_int = 2;

pub const STB0899_OFFST_BTR_LOCK_LEAK: c_int = 0;
pub const STB0899_WIDTH_BTR_LOCK_LEAK: c_int = 6;
pub const STB0899_OFF0_DECN_CNTRL: c_uint = 0xf364;
pub const STB0899_BASE_DECN_CNTRL: c_uint = 0x00000000;
pub const STB0899_OFF0_TP_CNTRL: c_uint = 0xf368;
pub const STB0899_BASE_TP_CNTRL: c_uint = 0x00000000;
pub const STB0899_OFF0_TP_BUF_STATUS: c_uint = 0xf36c;
pub const STB0899_BASE_TP_BUF_STATUS: c_uint = 0x00000000;

pub const STB0899_OFF0_DC_ESTIM: c_uint = 0xf37c;
pub const STB0899_BASE_DC_ESTIM: c_uint = 0x0000;

pub const STB0899_OFFST_I_DC_ESTIMATE: c_int = 8;
pub const STB0899_WIDTH_I_DC_ESTIMATE: c_int = 8;

pub const STB0899_OFFST_Q_DC_ESTIMATE: c_int = 0;
pub const STB0899_WIDTH_Q_DC_ESTIMATE: c_int = 8;
pub const STB0899_OFF0_FLL_CNTRL: c_uint = 0xf310;
pub const STB0899_BASE_FLL_CNTRL: c_uint = 0x00000020;

pub const STB0899_OFFST_CRL_FLL_ACC: c_int = 4;
pub const STB0899_WIDTH_CRL_FLL_ACC: c_int = 1;

pub const STB0899_OFFST_FLL_AVG_PERIOD: c_int = 0;
pub const STB0899_WIDTH_FLL_AVG_PERIOD: c_int = 4;
pub const STB0899_OFF0_FLL_FREQ_WD: c_uint = 0xf314;
pub const STB0899_BASE_FLL_FREQ_WD: c_uint = 0x00000020;

pub const STB0899_OFFST_FLL_FREQ_WD: c_int = 0;
pub const STB0899_WIDTH_FLL_FREQ_WD: c_int = 32;
pub const STB0899_OFF0_ANTI_ALIAS_SEL: c_uint = 0xf358;
pub const STB0899_BASE_ANTI_ALIAS_SEL: c_uint = 0x00000020;

pub const STB0899_OFFST_ANTI_ALIAS_SELB: c_int = 0;
pub const STB0899_WIDTH_ANTI_ALIAS_SELB: c_int = 2;
pub const STB0899_OFF0_RRC_ALPHA: c_uint = 0xf35c;
pub const STB0899_BASE_RRC_ALPHA: c_uint = 0x00000020;

pub const STB0899_OFFST_RRC_ALPHA: c_int = 0;
pub const STB0899_WIDTH_RRC_ALPHA: c_int = 2;
pub const STB0899_OFF0_DC_ADAPT_LSHFT: c_uint = 0xf360;
pub const STB0899_BASE_DC_ADAPT_LSHFT: c_uint = 0x00000020;

pub const STB0899_OFFST_DC_ADAPT_LSHFT: c_int = 0;
pub const STB0899_WIDTH_DC_ADAPT_LSHFT: c_int = 3;
pub const STB0899_OFF0_IMB_OFFSET: c_uint = 0xf364;
pub const STB0899_BASE_IMB_OFFSET: c_uint = 0x00000020;

pub const STB0899_OFFST_PHS_IMB_COMP: c_int = 8;
pub const STB0899_WIDTH_PHS_IMB_COMP: c_int = 8;

pub const STB0899_OFFST_AMPL_IMB_COMP: c_int = 0;
pub const STB0899_WIDTH_AMPL_IMB_COMP: c_int = 8;
pub const STB0899_OFF0_IMB_ESTIMATE: c_uint = 0xf368;
pub const STB0899_BASE_IMB_ESTIMATE: c_uint = 0x00000020;

pub const STB0899_OFFST_PHS_IMB_ESTIMATE: c_int = 8;
pub const STB0899_WIDTH_PHS_IMB_ESTIMATE: c_int = 8;

pub const STB0899_OFFST_AMPL_IMB_ESTIMATE: c_int = 0;
pub const STB0899_WIDTH_AMPL_IMB_ESTIMATE: c_int = 8;
pub const STB0899_OFF0_IMB_CNTRL: c_uint = 0xf36c;
pub const STB0899_BASE_IMB_CNTRL: c_uint = 0x00000020;

pub const STB0899_OFFST_PHS_ADAPT_LSHFT: c_int = 4;
pub const STB0899_WIDTH_PHS_ADAPT_LSHFT: c_int = 3;

pub const STB0899_OFFST_AMPL_ADAPT_LSHFT: c_int = 1;
pub const STB0899_WIDTH_AMPL_ADAPT_LSHFT: c_int = 3;

pub const STB0899_OFFST_IMB_COMP: c_int = 0;
pub const STB0899_WIDTH_IMB_COMP: c_int = 1;
pub const STB0899_OFF0_IF_AGC_CNTRL2: c_uint = 0xf374;
pub const STB0899_BASE_IF_AGC_CNTRL2: c_uint = 0x00000020;

pub const STB0899_OFFST_IF_AGC_LOCK_TH: c_int = 11;
pub const STB0899_WIDTH_IF_AGC_LOCK_TH: c_int = 8;

pub const STB0899_OFFST_IF_AGC_SD_DIV: c_int = 3;
pub const STB0899_WIDTH_IF_AGC_SD_DIV: c_int = 8;

pub const STB0899_OFFST_IF_AGC_DUMP_PER: c_int = 0;
pub const STB0899_WIDTH_IF_AGC_DUMP_PER: c_int = 3;
pub const STB0899_OFF0_DMD_CNTRL2: c_uint = 0xf378;
pub const STB0899_BASE_DMD_CNTRL2: c_uint = 0x00000020;

pub const STB0899_OFFST_SPECTRUM_INVERT: c_int = 2;
pub const STB0899_WIDTH_SPECTRUM_INVERT: c_int = 1;

pub const STB0899_OFFST_AGC_MODE: c_int = 1;
pub const STB0899_WIDTH_AGC_MODE: c_int = 1;

pub const STB0899_OFFST_CRL_FREQ_ADJ: c_int = 0;
pub const STB0899_WIDTH_CRL_FREQ_ADJ: c_int = 1;
pub const STB0899_OFF0_TP_BUFFER: c_uint = 0xf300;
pub const STB0899_BASE_TP_BUFFER: c_uint = 0x00000040;

pub const STB0899_OFFST_TP_BUFFER_IN: c_int = 0;
pub const STB0899_WIDTH_TP_BUFFER_IN: c_int = 16;
pub const STB0899_OFF0_TP_BUFFER1: c_uint = 0xf304;
pub const STB0899_BASE_TP_BUFFER1: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER2: c_uint = 0xf308;
pub const STB0899_BASE_TP_BUFFER2: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER3: c_uint = 0xf30c;
pub const STB0899_BASE_TP_BUFFER3: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER4: c_uint = 0xf310;
pub const STB0899_BASE_TP_BUFFER4: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER5: c_uint = 0xf314;
pub const STB0899_BASE_TP_BUFFER5: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER6: c_uint = 0xf318;
pub const STB0899_BASE_TP_BUFFER6: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER7: c_uint = 0xf31c;
pub const STB0899_BASE_TP_BUFFER7: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER8: c_uint = 0xf320;
pub const STB0899_BASE_TP_BUFFER8: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER9: c_uint = 0xf324;
pub const STB0899_BASE_TP_BUFFER9: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER10: c_uint = 0xf328;
pub const STB0899_BASE_TP_BUFFER10: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER11: c_uint = 0xf32c;
pub const STB0899_BASE_TP_BUFFER11: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER12: c_uint = 0xf330;
pub const STB0899_BASE_TP_BUFFER12: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER13: c_uint = 0xf334;
pub const STB0899_BASE_TP_BUFFER13: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER14: c_uint = 0xf338;
pub const STB0899_BASE_TP_BUFFER14: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER15: c_uint = 0xf33c;
pub const STB0899_BASE_TP_BUFFER15: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER16: c_uint = 0xf340;
pub const STB0899_BASE_TP_BUFFER16: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER17: c_uint = 0xf344;
pub const STB0899_BASE_TP_BUFFER17: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER18: c_uint = 0xf348;
pub const STB0899_BASE_TP_BUFFER18: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER19: c_uint = 0xf34c;
pub const STB0899_BASE_TP_BUFFER19: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER20: c_uint = 0xf350;
pub const STB0899_BASE_TP_BUFFER20: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER21: c_uint = 0xf354;
pub const STB0899_BASE_TP_BUFFER21: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER22: c_uint = 0xf358;
pub const STB0899_BASE_TP_BUFFER22: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER23: c_uint = 0xf35c;
pub const STB0899_BASE_TP_BUFFER23: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER24: c_uint = 0xf360;
pub const STB0899_BASE_TP_BUFFER24: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER25: c_uint = 0xf364;
pub const STB0899_BASE_TP_BUFFER25: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER26: c_uint = 0xf368;
pub const STB0899_BASE_TP_BUFFER26: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER27: c_uint = 0xf36c;
pub const STB0899_BASE_TP_BUFFER27: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER28: c_uint = 0xf370;
pub const STB0899_BASE_TP_BUFFER28: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER29: c_uint = 0xf374;
pub const STB0899_BASE_TP_BUFFER29: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER30: c_uint = 0xf378;
pub const STB0899_BASE_TP_BUFFER30: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER31: c_uint = 0xf37c;
pub const STB0899_BASE_TP_BUFFER31: c_uint = 0x00000040;
pub const STB0899_OFF0_TP_BUFFER32: c_uint = 0xf300;
pub const STB0899_BASE_TP_BUFFER32: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER33: c_uint = 0xf304;
pub const STB0899_BASE_TP_BUFFER33: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER34: c_uint = 0xf308;
pub const STB0899_BASE_TP_BUFFER34: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER35: c_uint = 0xf30c;
pub const STB0899_BASE_TP_BUFFER35: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER36: c_uint = 0xf310;
pub const STB0899_BASE_TP_BUFFER36: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER37: c_uint = 0xf314;
pub const STB0899_BASE_TP_BUFFER37: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER38: c_uint = 0xf318;
pub const STB0899_BASE_TP_BUFFER38: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER39: c_uint = 0xf31c;
pub const STB0899_BASE_TP_BUFFER39: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER40: c_uint = 0xf320;
pub const STB0899_BASE_TP_BUFFER40: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER41: c_uint = 0xf324;
pub const STB0899_BASE_TP_BUFFER41: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER42: c_uint = 0xf328;
pub const STB0899_BASE_TP_BUFFER42: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER43: c_uint = 0xf32c;
pub const STB0899_BASE_TP_BUFFER43: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER44: c_uint = 0xf330;
pub const STB0899_BASE_TP_BUFFER44: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER45: c_uint = 0xf334;
pub const STB0899_BASE_TP_BUFFER45: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER46: c_uint = 0xf338;
pub const STB0899_BASE_TP_BUFFER46: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER47: c_uint = 0xf33c;
pub const STB0899_BASE_TP_BUFFER47: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER48: c_uint = 0xf340;
pub const STB0899_BASE_TP_BUFFER48: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER49: c_uint = 0xf344;
pub const STB0899_BASE_TP_BUFFER49: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER50: c_uint = 0xf348;
pub const STB0899_BASE_TP_BUFFER50: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER51: c_uint = 0xf34c;
pub const STB0899_BASE_TP_BUFFER51: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER52: c_uint = 0xf350;
pub const STB0899_BASE_TP_BUFFER52: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER53: c_uint = 0xf354;
pub const STB0899_BASE_TP_BUFFER53: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER54: c_uint = 0xf358;
pub const STB0899_BASE_TP_BUFFER54: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER55: c_uint = 0xf35c;
pub const STB0899_BASE_TP_BUFFER55: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER56: c_uint = 0xf360;
pub const STB0899_BASE_TP_BUFFER56: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER57: c_uint = 0xf364;
pub const STB0899_BASE_TP_BUFFER57: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER58: c_uint = 0xf368;
pub const STB0899_BASE_TP_BUFFER58: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER59: c_uint = 0xf36c;
pub const STB0899_BASE_TP_BUFFER59: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER60: c_uint = 0xf370;
pub const STB0899_BASE_TP_BUFFER60: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER61: c_uint = 0xf374;
pub const STB0899_BASE_TP_BUFFER61: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER62: c_uint = 0xf378;
pub const STB0899_BASE_TP_BUFFER62: c_uint = 0x00000060;
pub const STB0899_OFF0_TP_BUFFER63: c_uint = 0xf37c;
pub const STB0899_BASE_TP_BUFFER63: c_uint = 0x00000060;
pub const STB0899_OFF0_RESET_CNTRL: c_uint = 0xf300;
pub const STB0899_BASE_RESET_CNTRL: c_uint = 0x00000400;

pub const STB0899_OFFST_DVBS2_RESET: c_int = 0;
pub const STB0899_WIDTH_DVBS2_RESET: c_int = 1;
pub const STB0899_OFF0_ACM_ENABLE: c_uint = 0xf304;
pub const STB0899_BASE_ACM_ENABLE: c_uint = 0x00000400;
pub const STB0899_ACM_ENABLE: c_int = 1;
pub const STB0899_OFF0_DESCR_CNTRL: c_uint = 0xf30c;
pub const STB0899_BASE_DESCR_CNTRL: c_uint = 0x00000400;
pub const STB0899_OFFST_DESCR_CNTRL: c_int = 0;
pub const STB0899_WIDTH_DESCR_CNTRL: c_int = 16;
pub const STB0899_OFF0_UWP_CNTRL1: c_uint = 0xf320;
pub const STB0899_BASE_UWP_CNTRL1: c_uint = 0x00000400;

pub const STB0899_OFFST_UWP_TH_SOF: c_int = 11;
pub const STB0899_WIDTH_UWP_TH_SOF: c_int = 15;

pub const STB0899_OFFST_UWP_ESN0_QUANT: c_int = 3;
pub const STB0899_WIDTH_UWP_ESN0_QUANT: c_int = 8;

pub const STB0899_OFFST_UWP_ESN0_AVE: c_int = 1;
pub const STB0899_WIDTH_UWP_ESN0_AVE: c_int = 2;

pub const STB0899_OFFST_UWP_START: c_int = 0;
pub const STB0899_WIDTH_UWP_START: c_int = 1;
pub const STB0899_OFF0_UWP_CNTRL2: c_uint = 0xf324;
pub const STB0899_BASE_UWP_CNTRL2: c_uint = 0x00000400;

pub const STB0899_OFFST_UWP_MISS_TH: c_int = 16;
pub const STB0899_WIDTH_UWP_MISS_TH: c_int = 8;

pub const STB0899_OFFST_FE_FINE_TRK: c_int = 8;
pub const STB0899_WIDTH_FE_FINE_TRK: c_int = 8;

pub const STB0899_OFFST_FE_COARSE_TRK: c_int = 0;
pub const STB0899_WIDTH_FE_COARSE_TRK: c_int = 8;
pub const STB0899_OFF0_UWP_STAT1: c_uint = 0xf328;
pub const STB0899_BASE_UWP_STAT1: c_uint = 0x00000400;

pub const STB0899_OFFST_UWP_STATE: c_int = 15;
pub const STB0899_WIDTH_UWP_STATE: c_int = 10;

pub const STB0899_OFFST_UW_MAX_PEAK: c_int = 0;
pub const STB0899_WIDTH_UW_MAX_PEAK: c_int = 15;
pub const STB0899_OFF0_UWP_STAT2: c_uint = 0xf32c;
pub const STB0899_BASE_UWP_STAT2: c_uint = 0x00000400;

pub const STB0899_OFFST_ESN0_EST: c_int = 7;
pub const STB0899_WIDTH_ESN0_EST: c_int = 19;

pub const STB0899_OFFST_UWP_DECODE_MOD: c_int = 0;
pub const STB0899_WIDTH_UWP_DECODE_MOD: c_int = 7;
pub const STB0899_OFF0_DMD_CORE_ID: c_uint = 0xf334;
pub const STB0899_BASE_DMD_CORE_ID: c_uint = 0x00000400;

pub const STB0899_OFFST_CORE_ID: c_int = 0;
pub const STB0899_WIDTH_CORE_ID: c_int = 32;
pub const STB0899_OFF0_DMD_VERSION_ID: c_uint = 0xf33c;
pub const STB0899_BASE_DMD_VERSION_ID: c_uint = 0x00000400;

pub const STB0899_OFFST_VERSION_ID: c_int = 0;
pub const STB0899_WIDTH_VERSION_ID: c_int = 8;
pub const STB0899_OFF0_DMD_STAT2: c_uint = 0xf340;
pub const STB0899_BASE_DMD_STAT2: c_uint = 0x00000400;

pub const STB0899_OFFST_CSM_LOCK: c_int = 1;
pub const STB0899_WIDTH_CSM_LOCK: c_int = 1;

pub const STB0899_OFFST_UWP_LOCK: c_int = 0;
pub const STB0899_WIDTH_UWP_LOCK: c_int = 1;
pub const STB0899_OFF0_FREQ_ADJ_SCALE: c_uint = 0xf344;
pub const STB0899_BASE_FREQ_ADJ_SCALE: c_uint = 0x00000400;

pub const STB0899_OFFST_FREQ_ADJ_SCALE: c_int = 0;
pub const STB0899_WIDTH_FREQ_ADJ_SCALE: c_int = 12;
pub const STB0899_OFF0_UWP_CNTRL3: c_uint = 0xf34c;
pub const STB0899_BASE_UWP_CNTRL3: c_uint = 0x00000400;

pub const STB0899_OFFST_UWP_TH_TRACK: c_int = 15;
pub const STB0899_WIDTH_UWP_TH_TRACK: c_int = 15;

pub const STB0899_OFFST_UWP_TH_ACQ: c_int = 0;
pub const STB0899_WIDTH_UWP_TH_ACQ: c_int = 15;
pub const STB0899_OFF0_SYM_CLK_SEL: c_uint = 0xf350;
pub const STB0899_BASE_SYM_CLK_SEL: c_uint = 0x00000400;

pub const STB0899_OFFST_SYM_CLK_SEL: c_int = 0;
pub const STB0899_WIDTH_SYM_CLK_SEL: c_int = 2;
pub const STB0899_OFF0_SOF_SRCH_TO: c_uint = 0xf354;
pub const STB0899_BASE_SOF_SRCH_TO: c_uint = 0x00000400;

pub const STB0899_OFFST_SOF_SEARCH_TIMEOUT: c_int = 0;
pub const STB0899_WIDTH_SOF_SEARCH_TIMEOUT: c_int = 22;
pub const STB0899_OFF0_ACQ_CNTRL1: c_uint = 0xf358;
pub const STB0899_BASE_ACQ_CNTRL1: c_uint = 0x00000400;

pub const STB0899_OFFST_FE_FINE_ACQ: c_int = 8;
pub const STB0899_WIDTH_FE_FINE_ACQ: c_int = 8;

pub const STB0899_OFFST_FE_COARSE_ACQ: c_int = 0;
pub const STB0899_WIDTH_FE_COARSE_ACQ: c_int = 8;
pub const STB0899_OFF0_ACQ_CNTRL2: c_uint = 0xf35c;
pub const STB0899_BASE_ACQ_CNTRL2: c_uint = 0x00000400;

pub const STB0899_OFFST_ZIGZAG: c_int = 25;
pub const STB0899_WIDTH_ZIGZAG: c_int = 1;

pub const STB0899_OFFST_NUM_STEPS: c_int = 17;
pub const STB0899_WIDTH_NUM_STEPS: c_int = 8;

pub const STB0899_OFFST_FREQ_STEPSIZE: c_int = 0;
pub const STB0899_WIDTH_FREQ_STEPSIZE: c_int = 17;
pub const STB0899_OFF0_ACQ_CNTRL3: c_uint = 0xf360;
pub const STB0899_BASE_ACQ_CNTRL3: c_uint = 0x00000400;

pub const STB0899_OFFST_THRESHOLD_SCL: c_int = 23;
pub const STB0899_WIDTH_THRESHOLD_SCL: c_int = 6;

pub const STB0899_OFFST_UWP_TH_SRCH: c_int = 8;
pub const STB0899_WIDTH_UWP_TH_SRCH: c_int = 15;

pub const STB0899_OFFST_AUTO_REACQUIRE: c_int = 7;
pub const STB0899_WIDTH_AUTO_REACQUIRE: c_int = 1;

pub const STB0899_OFFST_TRACK_LOCK_SEL: c_int = 6;
pub const STB0899_WIDTH_TRACK_LOCK_SEL: c_int = 1;

pub const STB0899_OFFST_ACQ_SEARCH_MODE: c_int = 4;
pub const STB0899_WIDTH_ACQ_SEARCH_MODE: c_int = 2;

pub const STB0899_OFFST_CONFIRM_FRAMES: c_int = 0;
pub const STB0899_WIDTH_CONFIRM_FRAMES: c_int = 4;
pub const STB0899_OFF0_FE_SETTLE: c_uint = 0xf364;
pub const STB0899_BASE_FE_SETTLE: c_uint = 0x00000400;

pub const STB0899_OFFST_SETTLING_TIME: c_int = 0;
pub const STB0899_WIDTH_SETTLING_TIME: c_int = 22;
pub const STB0899_OFF0_AC_DWELL: c_uint = 0xf368;
pub const STB0899_BASE_AC_DWELL: c_uint = 0x00000400;

pub const STB0899_OFFST_DWELL_TIME: c_int = 0;
pub const STB0899_WIDTH_DWELL_TIME: c_int = 22;
pub const STB0899_OFF0_ACQUIRE_TRIG: c_uint = 0xf36c;
pub const STB0899_BASE_ACQUIRE_TRIG: c_uint = 0x00000400;

pub const STB0899_OFFST_ACQUIRE: c_int = 0;
pub const STB0899_WIDTH_ACQUIRE: c_int = 1;
pub const STB0899_OFF0_LOCK_LOST: c_uint = 0xf370;
pub const STB0899_BASE_LOCK_LOST: c_uint = 0x00000400;

pub const STB0899_OFFST_LOCK_LOST: c_int = 0;
pub const STB0899_WIDTH_LOCK_LOST: c_int = 1;
pub const STB0899_OFF0_ACQ_STAT1: c_uint = 0xf374;
pub const STB0899_BASE_ACQ_STAT1: c_uint = 0x00000400;

pub const STB0899_OFFST_STEP_FREQ: c_int = 11;
pub const STB0899_WIDTH_STEP_FREQ: c_int = 21;

pub const STB0899_OFFST_ACQ_STATE: c_int = 8;
pub const STB0899_WIDTH_ACQ_STATE: c_int = 3;

pub const STB0899_OFFST_UW_DETECT_COUNT: c_int = 0;
pub const STB0899_WIDTH_UW_DETECT_COUNT: c_int = 8;
pub const STB0899_OFF0_ACQ_TIMEOUT: c_uint = 0xf378;
pub const STB0899_BASE_ACQ_TIMEOUT: c_uint = 0x00000400;

pub const STB0899_OFFST_ACQ_TIMEOUT: c_int = 0;
pub const STB0899_WIDTH_ACQ_TIMEOUT: c_int = 22;
pub const STB0899_OFF0_ACQ_TIME: c_uint = 0xf37c;
pub const STB0899_BASE_ACQ_TIME: c_uint = 0x00000400;

pub const STB0899_OFFST_ACQ_TIME_SYM: c_int = 0;
pub const STB0899_WIDTH_ACQ_TIME_SYM: c_int = 24;
pub const STB0899_OFF0_FINAL_AGC_CNTRL: c_uint = 0xf308;
pub const STB0899_BASE_FINAL_AGC_CNTRL: c_uint = 0x00000440;

pub const STB0899_OFFST_FINAL_GAIN_INIT: c_int = 12;
pub const STB0899_WIDTH_FINAL_GAIN_INIT: c_int = 14;

pub const STB0899_OFFST_FINAL_LOOP_GAIN: c_int = 8;
pub const STB0899_WIDTH_FINAL_LOOP_GAIN: c_int = 4;

pub const STB0899_OFFST_FINAL_LD_GAIN_INIT: c_int = 7;
pub const STB0899_WIDTH_FINAL_LD_GAIN_INIT: c_int = 1;

pub const STB0899_OFFST_FINAL_AGC_REF: c_int = 0;
pub const STB0899_WIDTH_FINAL_AGC_REF: c_int = 7;
pub const STB0899_OFF0_FINAL_AGC_GAIN: c_uint = 0xf30c;
pub const STB0899_BASE_FINAL_AGC_GAIN: c_uint = 0x00000440;

pub const STB0899_OFFST_FINAL_AGC_GAIN: c_int = 0;
pub const STB0899_WIDTH_FINAL_AGC_GAIN: c_int = 14;
pub const STB0899_OFF0_EQUALIZER_INIT: c_uint = 0xf310;
pub const STB0899_BASE_EQUALIZER_INIT: c_uint = 0x00000440;

pub const STB0899_OFFST_EQ_SRST: c_int = 1;
pub const STB0899_WIDTH_EQ_SRST: c_int = 1;

pub const STB0899_OFFST_EQ_INIT: c_int = 0;
pub const STB0899_WIDTH_EQ_INIT: c_int = 1;
pub const STB0899_OFF0_EQ_CNTRL: c_uint = 0xf314;
pub const STB0899_BASE_EQ_CNTRL: c_uint = 0x00000440;

pub const STB0899_OFFST_EQ_ADAPT_MODE: c_int = 18;
pub const STB0899_WIDTH_EQ_ADAPT_MODE: c_int = 1;

pub const STB0899_OFFST_EQ_DELAY: c_int = 14;
pub const STB0899_WIDTH_EQ_DELAY: c_int = 4;

pub const STB0899_OFFST_EQ_QUANT_LEVEL: c_int = 6;
pub const STB0899_WIDTH_EQ_QUANT_LEVEL: c_int = 8;

pub const STB0899_OFFST_EQ_DISABLE_UPDATE: c_int = 5;
pub const STB0899_WIDTH_EQ_DISABLE_UPDATE: c_int = 1;

pub const STB0899_OFFST_EQ_BYPASS: c_int = 4;
pub const STB0899_WIDTH_EQ_BYPASS: c_int = 1;

pub const STB0899_OFFST_EQ_SHIFT: c_int = 0;
pub const STB0899_WIDTH_EQ_SHIFT: c_int = 4;
pub const STB0899_OFF0_EQ_I_INIT_COEFF_0: c_uint = 0xf320;
pub const STB0899_OFF1_EQ_I_INIT_COEFF_1: c_uint = 0xf324;
pub const STB0899_OFF2_EQ_I_INIT_COEFF_2: c_uint = 0xf328;
pub const STB0899_OFF3_EQ_I_INIT_COEFF_3: c_uint = 0xf32c;
pub const STB0899_OFF4_EQ_I_INIT_COEFF_4: c_uint = 0xf330;
pub const STB0899_OFF5_EQ_I_INIT_COEFF_5: c_uint = 0xf334;
pub const STB0899_OFF6_EQ_I_INIT_COEFF_6: c_uint = 0xf338;
pub const STB0899_OFF7_EQ_I_INIT_COEFF_7: c_uint = 0xf33c;
pub const STB0899_OFF8_EQ_I_INIT_COEFF_8: c_uint = 0xf340;
pub const STB0899_OFF9_EQ_I_INIT_COEFF_9: c_uint = 0xf344;
pub const STB0899_OFFa_EQ_I_INIT_COEFF_10: c_uint = 0xf348;
pub const STB0899_BASE_EQ_I_INIT_COEFF_N: c_uint = 0x00000440;

pub const STB0899_OFFST_EQ_I_INIT_COEFF_N: c_int = 0;
pub const STB0899_WIDTH_EQ_I_INIT_COEFF_N: c_int = 12;
pub const STB0899_OFF0_EQ_Q_INIT_COEFF_0: c_uint = 0xf350;
pub const STB0899_OFF1_EQ_Q_INIT_COEFF_1: c_uint = 0xf354;
pub const STB0899_OFF2_EQ_Q_INIT_COEFF_2: c_uint = 0xf358;
pub const STB0899_OFF3_EQ_Q_INIT_COEFF_3: c_uint = 0xf35c;
pub const STB0899_OFF4_EQ_Q_INIT_COEFF_4: c_uint = 0xf360;
pub const STB0899_OFF5_EQ_Q_INIT_COEFF_5: c_uint = 0xf364;
pub const STB0899_OFF6_EQ_Q_INIT_COEFF_6: c_uint = 0xf368;
pub const STB0899_OFF7_EQ_Q_INIT_COEFF_7: c_uint = 0xf36c;
pub const STB0899_OFF8_EQ_Q_INIT_COEFF_8: c_uint = 0xf370;
pub const STB0899_OFF9_EQ_Q_INIT_COEFF_9: c_uint = 0xf374;
pub const STB0899_OFFa_EQ_Q_INIT_COEFF_10: c_uint = 0xf378;
pub const STB0899_BASE_EQ_Q_INIT_COEFF_N: c_uint = 0x00000440;

pub const STB0899_OFFST_EQ_Q_INIT_COEFF_N: c_int = 0;
pub const STB0899_WIDTH_EQ_Q_INIT_COEFF_N: c_int = 12;
pub const STB0899_OFF0_EQ_I_OUT_COEFF_0: c_uint = 0xf300;
pub const STB0899_OFF1_EQ_I_OUT_COEFF_1: c_uint = 0xf304;
pub const STB0899_OFF2_EQ_I_OUT_COEFF_2: c_uint = 0xf308;
pub const STB0899_OFF3_EQ_I_OUT_COEFF_3: c_uint = 0xf30c;
pub const STB0899_OFF4_EQ_I_OUT_COEFF_4: c_uint = 0xf310;
pub const STB0899_OFF5_EQ_I_OUT_COEFF_5: c_uint = 0xf314;
pub const STB0899_OFF6_EQ_I_OUT_COEFF_6: c_uint = 0xf318;
pub const STB0899_OFF7_EQ_I_OUT_COEFF_7: c_uint = 0xf31c;
pub const STB0899_OFF8_EQ_I_OUT_COEFF_8: c_uint = 0xf320;
pub const STB0899_OFF9_EQ_I_OUT_COEFF_9: c_uint = 0xf324;
pub const STB0899_OFFa_EQ_I_OUT_COEFF_10: c_uint = 0xf328;
pub const STB0899_BASE_EQ_I_OUT_COEFF_N: c_uint = 0x00000460;

pub const STB0899_OFFST_EQ_I_OUT_COEFF_N: c_int = 0;
pub const STB0899_WIDTH_EQ_I_OUT_COEFF_N: c_int = 12;
pub const STB0899_OFF0_EQ_Q_OUT_COEFF_0: c_uint = 0xf330;
pub const STB0899_OFF1_EQ_Q_OUT_COEFF_1: c_uint = 0xf334;
pub const STB0899_OFF2_EQ_Q_OUT_COEFF_2: c_uint = 0xf338;
pub const STB0899_OFF3_EQ_Q_OUT_COEFF_3: c_uint = 0xf33c;
pub const STB0899_OFF4_EQ_Q_OUT_COEFF_4: c_uint = 0xf340;
pub const STB0899_OFF5_EQ_Q_OUT_COEFF_5: c_uint = 0xf344;
pub const STB0899_OFF6_EQ_Q_OUT_COEFF_6: c_uint = 0xf348;
pub const STB0899_OFF7_EQ_Q_OUT_COEFF_7: c_uint = 0xf34c;
pub const STB0899_OFF8_EQ_Q_OUT_COEFF_8: c_uint = 0xf350;
pub const STB0899_OFF9_EQ_Q_OUT_COEFF_9: c_uint = 0xf354;
pub const STB0899_OFFa_EQ_Q_OUT_COEFF_10: c_uint = 0xf358;
pub const STB0899_BASE_EQ_Q_OUT_COEFF_N: c_uint = 0x00000460;

pub const STB0899_OFFST_EQ_Q_OUT_COEFF_N: c_int = 0;
pub const STB0899_WIDTH_EQ_Q_OUT_COEFF_N: c_int = 12;
// S2 FEC
pub const STB0899_OFF0_BLOCK_LNGTH: c_uint = 0xfa04;
pub const STB0899_BASE_BLOCK_LNGTH: c_uint = 0x00000000;

pub const STB0899_OFFST_BLOCK_LENGTH: c_int = 0;
pub const STB0899_WIDTH_BLOCK_LENGTH: c_int = 8;
pub const STB0899_OFF0_ROW_STR: c_uint = 0xfa08;
pub const STB0899_BASE_ROW_STR: c_uint = 0x00000000;

pub const STB0899_OFFST_ROW_STRIDE: c_int = 0;
pub const STB0899_WIDTH_ROW_STRIDE: c_int = 8;
pub const STB0899_OFF0_MAX_ITER: c_uint = 0xfa0c;
pub const STB0899_BASE_MAX_ITER: c_uint = 0x00000000;

pub const STB0899_OFFST_MAX_ITERATIONS: c_int = 0;
pub const STB0899_WIDTH_MAX_ITERATIONS: c_int = 8;
pub const STB0899_OFF0_BN_END_ADDR: c_uint = 0xfa10;
pub const STB0899_BASE_BN_END_ADDR: c_uint = 0x00000000;

pub const STB0899_OFFST_BN_END_ADDR: c_int = 0;
pub const STB0899_WIDTH_BN_END_ADDR: c_int = 12;
pub const STB0899_OFF0_CN_END_ADDR: c_uint = 0xfa14;
pub const STB0899_BASE_CN_END_ADDR: c_uint = 0x00000000;

pub const STB0899_OFFST_CN_END_ADDR: c_int = 0;
pub const STB0899_WIDTH_CN_END_ADDR: c_int = 12;
pub const STB0899_OFF0_INFO_LENGTH: c_uint = 0xfa1c;
pub const STB0899_BASE_INFO_LENGTH: c_uint = 0x00000000;

pub const STB0899_OFFST_INFO_LENGTH: c_int = 0;
pub const STB0899_WIDTH_INFO_LENGTH: c_int = 8;
pub const STB0899_OFF0_BOT_ADDR: c_uint = 0xfa20;
pub const STB0899_BASE_BOT_ADDR: c_uint = 0x00000000;

pub const STB0899_OFFST_BOTTOM_BASE_ADDR: c_int = 0;
pub const STB0899_WIDTH_BOTTOM_BASE_ADDR: c_int = 10;
pub const STB0899_OFF0_BCH_BLK_LN: c_uint = 0xfa24;
pub const STB0899_BASE_BCH_BLK_LN: c_uint = 0x00000000;

pub const STB0899_OFFST_BCH_BLOCK_LENGTH: c_int = 0;
pub const STB0899_WIDTH_BCH_BLOCK_LENGTH: c_int = 16;
pub const STB0899_OFF0_BCH_T: c_uint = 0xfa28;
pub const STB0899_BASE_BCH_T: c_uint = 0x00000000;

pub const STB0899_OFFST_BCH_T: c_int = 0;
pub const STB0899_WIDTH_BCH_T: c_int = 4;
pub const STB0899_OFF0_CNFG_MODE: c_uint = 0xfa00;
pub const STB0899_BASE_CNFG_MODE: c_uint = 0x00000800;

pub const STB0899_OFFST_MODCOD: c_int = 2;
pub const STB0899_WIDTH_MODCOD: c_int = 5;

pub const STB0899_OFFST_MODCOD_SEL: c_int = 1;
pub const STB0899_WIDTH_MODCOD_SEL: c_int = 1;

pub const STB0899_OFFST_CONFIG_MODE: c_int = 0;
pub const STB0899_WIDTH_CONFIG_MODE: c_int = 1;
pub const STB0899_OFF0_LDPC_STAT: c_uint = 0xfa04;
pub const STB0899_BASE_LDPC_STAT: c_uint = 0x00000800;

pub const STB0899_OFFST_ITERATION: c_int = 3;
pub const STB0899_WIDTH_ITERATION: c_int = 8;

pub const STB0899_OFFST_LDPC_DEC_STATE: c_int = 0;
pub const STB0899_WIDTH_LDPC_DEC_STATE: c_int = 3;
pub const STB0899_OFF0_ITER_SCALE: c_uint = 0xfa08;
pub const STB0899_BASE_ITER_SCALE: c_uint = 0x00000800;

pub const STB0899_OFFST_ITERATION_SCALE: c_int = 0;
pub const STB0899_WIDTH_ITERATION_SCALE: c_int = 8;
pub const STB0899_OFF0_INPUT_MODE: c_uint = 0xfa0c;
pub const STB0899_BASE_INPUT_MODE: c_uint = 0x00000800;

pub const STB0899_OFFST_SD_BLOCK1_STREAM0: c_int = 0;
pub const STB0899_WIDTH_SD_BLOCK1_STREAM0: c_int = 1;
pub const STB0899_OFF0_LDPCDECRST: c_uint = 0xfa10;
pub const STB0899_BASE_LDPCDECRST: c_uint = 0x00000800;

pub const STB0899_OFFST_LDPC_DEC_RST: c_int = 0;
pub const STB0899_WIDTH_LDPC_DEC_RST: c_int = 1;
pub const STB0899_OFF0_CLK_PER_BYTE_RW: c_uint = 0xfa14;
pub const STB0899_BASE_CLK_PER_BYTE_RW: c_uint = 0x00000800;

pub const STB0899_OFFST_CLKS_PER_BYTE: c_int = 0;
pub const STB0899_WIDTH_CLKS_PER_BYTE: c_int = 5;
pub const STB0899_OFF0_BCH_ERRORS: c_uint = 0xfa18;
pub const STB0899_BASE_BCH_ERRORS: c_uint = 0x00000800;

pub const STB0899_OFFST_BCH_ERRORS: c_int = 0;
pub const STB0899_WIDTH_BCH_ERRORS: c_int = 4;
pub const STB0899_OFF0_LDPC_ERRORS: c_uint = 0xfa1c;
pub const STB0899_BASE_LDPC_ERRORS: c_uint = 0x00000800;

pub const STB0899_OFFST_LDPC_ERRORS: c_int = 0;
pub const STB0899_WIDTH_LDPC_ERRORS: c_int = 16;
pub const STB0899_OFF0_BCH_MODE: c_uint = 0xfa20;
pub const STB0899_BASE_BCH_MODE: c_uint = 0x00000800;

pub const STB0899_OFFST_BCH_CORRECT_N: c_int = 1;
pub const STB0899_WIDTH_BCH_CORRECT_N: c_int = 1;

pub const STB0899_OFFST_FULL_BYPASS: c_int = 0;
pub const STB0899_WIDTH_FULL_BYPASS: c_int = 1;
pub const STB0899_OFF0_ERR_ACC_PER: c_uint = 0xfa24;
pub const STB0899_BASE_ERR_ACC_PER: c_uint = 0x00000800;

pub const STB0899_OFFST_BCH_ERR_ACC_PERIOD: c_int = 0;
pub const STB0899_WIDTH_BCH_ERR_ACC_PERIOD: c_int = 4;
pub const STB0899_OFF0_BCH_ERR_ACC: c_uint = 0xfa28;
pub const STB0899_BASE_BCH_ERR_ACC: c_uint = 0x00000800;

pub const STB0899_OFFST_BCH_ERR_ACCUM: c_int = 0;
pub const STB0899_WIDTH_BCH_ERR_ACCUM: c_int = 8;
pub const STB0899_OFF0_FEC_CORE_ID_REG: c_uint = 0xfa2c;
pub const STB0899_BASE_FEC_CORE_ID_REG: c_uint = 0x00000800;

pub const STB0899_OFFST_FEC_CORE_ID: c_int = 0;
pub const STB0899_WIDTH_FEC_CORE_ID: c_int = 32;
pub const STB0899_OFF0_FEC_VER_ID_REG: c_uint = 0xfa34;
pub const STB0899_BASE_FEC_VER_ID_REG: c_uint = 0x00000800;

pub const STB0899_OFFST_FEC_VER_ID: c_int = 0;
pub const STB0899_WIDTH_FEC_VER_ID: c_int = 8;
pub const STB0899_OFF0_FEC_TP_SEL: c_uint = 0xfa38;
pub const STB0899_BASE_FEC_TP_SEL: c_uint = 0x00000800;
pub const STB0899_OFF0_CSM_CNTRL1: c_uint = 0xf310;
pub const STB0899_BASE_CSM_CNTRL1: c_uint = 0x00000400;

pub const STB0899_OFFST_CSM_FORCE_FREQLOCK: c_int = 19;
pub const STB0899_WIDTH_CSM_FORCE_FREQLOCK: c_int = 1;

pub const STB0899_OFFST_CSM_FREQ_LOCKSTATE: c_int = 18;
pub const STB0899_WIDTH_CSM_FREQ_LOCKSTATE: c_int = 1;

pub const STB0899_OFFST_CSM_AUTO_PARAM: c_int = 17;
pub const STB0899_WIDTH_CSM_AUTO_PARAM: c_int = 1;

pub const STB0899_OFFST_FE_LOOP_SHIFT: c_int = 14;
pub const STB0899_WIDTH_FE_LOOP_SHIFT: c_int = 3;

pub const STB0899_OFFST_CSM_AGC_SHIFT: c_int = 11;
pub const STB0899_WIDTH_CSM_AGC_SHIFT: c_int = 3;

pub const STB0899_OFFST_CSM_AGC_GAIN: c_int = 2;
pub const STB0899_WIDTH_CSM_AGC_GAIN: c_int = 9;

pub const STB0899_OFFST_CSM_TWO_PASS: c_int = 1;
pub const STB0899_WIDTH_CSM_TWO_PASS: c_int = 1;

pub const STB0899_OFFST_CSM_DVT_TABLE: c_int = 0;
pub const STB0899_WIDTH_CSM_DVT_TABLE: c_int = 1;
pub const STB0899_OFF0_CSM_CNTRL2: c_uint = 0xf314;
pub const STB0899_BASE_CSM_CNTRL2: c_uint = 0x00000400;

pub const STB0899_OFFST_CSM_GAMMA_RHOACQ: c_int = 9;
pub const STB0899_WIDTH_CSM_GAMMA_RHOACQ: c_int = 9;

pub const STB0899_OFFST_CSM_GAMMA_ACQ: c_int = 0;
pub const STB0899_WIDTH_CSM_GAMMA_ACQ: c_int = 9;
pub const STB0899_OFF0_CSM_CNTRL3: c_uint = 0xf318;
pub const STB0899_BASE_CSM_CNTRL3: c_uint = 0x00000400;

pub const STB0899_OFFST_CSM_GAMMA_RHOTRACK: c_int = 9;
pub const STB0899_WIDTH_CSM_GAMMA_RHOTRACK: c_int = 9;

pub const STB0899_OFFST_CSM_GAMMA_TRACK: c_int = 0;
pub const STB0899_WIDTH_CSM_GAMMA_TRACK: c_int = 9;
pub const STB0899_OFF0_CSM_CNTRL4: c_uint = 0xf31c;
pub const STB0899_BASE_CSM_CNTRL4: c_uint = 0x00000400;

pub const STB0899_OFFST_CSM_PHASEDIFF_THRESH: c_int = 8;
pub const STB0899_WIDTH_CSM_PHASEDIFF_THRESH: c_int = 4;

pub const STB0899_OFFST_CSM_LOCKCOUNT_THRESH: c_int = 0;
pub const STB0899_WIDTH_CSM_LOCKCOUNT_THRESH: c_int = 8;
// Check on chapter 8 page 42
pub const STB0899_ERRCTRL1: c_uint = 0xf574;
pub const STB0899_ERRCTRL2: c_uint = 0xf575;
pub const STB0899_ERRCTRL3: c_uint = 0xf576;

pub const STB0899_OFFST_ERR_SRC_S1: c_int = 3;
pub const STB0899_WIDTH_ERR_SRC_S1: c_int = 5;

pub const STB0899_OFFST_ERR_SRC_S2: c_int = 0;
pub const STB0899_WIDTH_ERR_SRC_S2: c_int = 4;

pub const STB0899_OFFST_NOE: c_int = 0;
pub const STB0899_WIDTH_NOE: c_int = 3;
pub const STB0899_ECNT1M: c_uint = 0xf524;
pub const STB0899_ECNT1L: c_uint = 0xf525;
pub const STB0899_ECNT2M: c_uint = 0xf526;
pub const STB0899_ECNT2L: c_uint = 0xf527;
pub const STB0899_ECNT3M: c_uint = 0xf528;
pub const STB0899_ECNT3L: c_uint = 0xf529;
pub const STB0899_DMONMSK1: c_uint = 0xf57b;

pub const STB0899_DMONMSK0: c_uint = 0xf57c;

pub const STB0899_TSULC: c_uint = 0xf549;

pub const STB0899_OFFST_ULNOSYNCBYTES: c_int = 7;
pub const STB0899_WIDTH_ULNOSYNCBYTES: c_int = 1;

pub const STB0899_OFFST_ULPARITY_ON: c_int = 6;
pub const STB0899_WIDTH_ULPARITY_ON: c_int = 1;

pub const STB0899_OFFST_ULSYNCOUTRS: c_int = 5;
pub const STB0899_WIDTH_ULSYNCOUTRS: c_int = 1;

pub const STB0899_OFFST_ULDSS_PACKETS: c_int = 0;
pub const STB0899_WIDTH_ULDSS_PACKETS: c_int = 1;
pub const STB0899_TSLPL: c_uint = 0xf54b;

pub const STB0899_OFFST_LLDVBS2_MODE: c_int = 4;
pub const STB0899_WIDTH_LLDVBS2_MODE: c_int = 1;

pub const STB0899_OFFST_LLISSYI_ON: c_int = 3;
pub const STB0899_WIDTH_LLISSYI_ON: c_int = 1;

pub const STB0899_OFFST_LLNPD_ON: c_int = 2;
pub const STB0899_WIDTH_LLNPD_ON: c_int = 1;

pub const STB0899_OFFST_LLCRC8_ON: c_int = 1;
pub const STB0899_WIDTH_LLCRC8_ON: c_int = 1;
pub const STB0899_TSCFGH: c_uint = 0xf54c;

pub const STB0899_OFFST_OUTRS_PS: c_int = 6;
pub const STB0899_WIDTH_OUTRS_PS: c_int = 1;

pub const STB0899_OFFST_SYNCBYTE: c_int = 5;
pub const STB0899_WIDTH_SYNCBYTE: c_int = 1;

pub const STB0899_OFFST_PFBIT: c_int = 4;
pub const STB0899_WIDTH_PFBIT: c_int = 1;

pub const STB0899_OFFST_ERR_BIT: c_int = 3;
pub const STB0899_WIDTH_ERR_BIT: c_int = 1;

pub const STB0899_OFFST_MPEG: c_int = 2;
pub const STB0899_WIDTH_MPEG: c_int = 1;

pub const STB0899_OFFST_CLK_POL: c_int = 1;
pub const STB0899_WIDTH_CLK_POL: c_int = 1;

pub const STB0899_OFFST_FORCE0: c_int = 0;
pub const STB0899_WIDTH_FORCE0: c_int = 1;
pub const STB0899_TSCFGM: c_uint = 0xf54d;

pub const STB0899_OFFST_LLPRIORIY: c_int = 3;
pub const STB0899_WIDTH_LLPRIORITY: c_int = 1;

pub const STB0899_OFFST_EN188: c_int = 2;
pub const STB0899_WIDTH_EN188: c_int = 1;
pub const STB0899_TSCFGL: c_uint = 0xf54e;

pub const STB0899_OFFST_DEL_ERRPCK: c_int = 7;
pub const STB0899_WIDTH_DEL_ERRPCK: c_int = 1;

pub const STB0899_OFFST_ERRFLAGSTD: c_int = 5;
pub const STB0899_WIDTH_ERRFLAGSTD: c_int = 1;

pub const STB0899_OFFST_MPEGERR: c_int = 4;
pub const STB0899_WIDTH_MPEGERR: c_int = 1;

pub const STB0899_OFFST_BCH_CHK: c_int = 5;
pub const STB0899_WIDTH_BCH_CHK: c_int = 1;

pub const STB0899_OFFST_CRC8CHK: c_int = 2;
pub const STB0899_WIDTH_CRC8CHK: c_int = 1;

pub const STB0899_OFFST_SPEC_INFO: c_int = 1;
pub const STB0899_WIDTH_SPEC_INFO: c_int = 1;

pub const STB0899_OFFST_LOW_PRIO_CLK: c_int = 0;
pub const STB0899_WIDTH_LOW_PRIO_CLK: c_int = 1;

pub const STB0899_OFFST_ERROR_NORM: c_int = 0;
pub const STB0899_WIDTH_ERROR_NORM: c_int = 0;
pub const STB0899_TSOUT: c_uint = 0xf54f;
pub const STB0899_RSSYNCDEL: c_uint = 0xf550;
pub const STB0899_TSINHDELH: c_uint = 0xf551;
pub const STB0899_TSINHDELM: c_uint = 0xf552;
pub const STB0899_TSINHDELL: c_uint = 0xf553;
pub const STB0899_TSLLSTKM: c_uint = 0xf55a;
pub const STB0899_TSLLSTKL: c_uint = 0xf55b;
pub const STB0899_TSULSTKM: c_uint = 0xf55c;
pub const STB0899_TSULSTKL: c_uint = 0xf55d;
pub const STB0899_TSSTATUS: c_uint = 0xf561;
pub const STB0899_PDELCTRL: c_uint = 0xf600;

pub const STB0899_OFFST_INVERT_RES: c_int = 7;
pub const STB0899_WIDTH_INVERT_RES: c_int = 1;

pub const STB0899_OFFST_FORCE_ACCEPTED: c_int = 6;
pub const STB0899_WIDTH_FORCE_ACCEPTED: c_int = 1;

pub const STB0899_OFFST_FILTER_EN: c_int = 5;
pub const STB0899_WIDTH_FILTER_EN: c_int = 1;

pub const STB0899_OFFST_LOCKFALL_THRESH: c_int = 4;
pub const STB0899_WIDTH_LOCKFALL_THRESH: c_int = 1;

pub const STB0899_OFFST_HYST_EN: c_int = 3;
pub const STB0899_WIDTH_HYST_EN: c_int = 1;

pub const STB0899_OFFST_HYST_SWRST: c_int = 2;
pub const STB0899_WIDTH_HYST_SWRST: c_int = 1;

pub const STB0899_OFFST_ALGO_EN: c_int = 1;
pub const STB0899_WIDTH_ALGO_EN: c_int = 1;

pub const STB0899_OFFST_ALGO_SWRST: c_int = 0;
pub const STB0899_WIDTH_ALGO_SWRST: c_int = 1;
pub const STB0899_PDELCTRL2: c_uint = 0xf601;
pub const STB0899_BBHCTRL1: c_uint = 0xf602;
pub const STB0899_BBHCTRL2: c_uint = 0xf603;
pub const STB0899_HYSTTHRESH: c_uint = 0xf604;
pub const STB0899_MATCSTM: c_uint = 0xf605;
pub const STB0899_MATCSTL: c_uint = 0xf606;
pub const STB0899_UPLCSTM: c_uint = 0xf607;
pub const STB0899_UPLCSTL: c_uint = 0xf608;
pub const STB0899_DFLCSTM: c_uint = 0xf609;
pub const STB0899_DFLCSTL: c_uint = 0xf60a;
pub const STB0899_SYNCCST: c_uint = 0xf60b;
pub const STB0899_SYNCDCSTM: c_uint = 0xf60c;
pub const STB0899_SYNCDCSTL: c_uint = 0xf60d;
pub const STB0899_ISI_ENTRY: c_uint = 0xf60e;
pub const STB0899_ISI_BIT_EN: c_uint = 0xf60f;
pub const STB0899_MATSTRM: c_uint = 0xf610;
pub const STB0899_MATSTRL: c_uint = 0xf611;
pub const STB0899_UPLSTRM: c_uint = 0xf612;
pub const STB0899_UPLSTRL: c_uint = 0xf613;
pub const STB0899_DFLSTRM: c_uint = 0xf614;
pub const STB0899_DFLSTRL: c_uint = 0xf615;
pub const STB0899_SYNCSTR: c_uint = 0xf616;
pub const STB0899_SYNCDSTRM: c_uint = 0xf617;
pub const STB0899_SYNCDSTRL: c_uint = 0xf618;
pub const STB0899_CFGPDELSTATUS1: c_uint = 0xf619;

pub const STB0899_OFFST_BADDFL: c_int = 6;
pub const STB0899_WIDTH_BADDFL: c_int = 1;

pub const STB0899_OFFST_CONTINUOUS_STREAM: c_int = 5;
pub const STB0899_WIDTH_CONTINUOUS_STREAM: c_int = 1;

pub const STB0899_OFFST_ACCEPTED_STREAM: c_int = 4;
pub const STB0899_WIDTH_ACCEPTED_STREAM: c_int = 1;

pub const STB0899_OFFST_BCH_ERRFLAG: c_int = 3;
pub const STB0899_WIDTH_BCH_ERRFLAG: c_int = 1;

pub const STB0899_OFFST_CRCRES: c_int = 2;
pub const STB0899_WIDTH_CRCRES: c_int = 1;

pub const STB0899_OFFST_CFGPDELSTATUS_LOCK: c_int = 1;
pub const STB0899_WIDTH_CFGPDELSTATUS_LOCK: c_int = 1;

pub const STB0899_OFFST_1STLOCK: c_int = 0;
pub const STB0899_WIDTH_1STLOCK: c_int = 1;
pub const STB0899_CFGPDELSTATUS2: c_uint = 0xf61a;
pub const STB0899_BBFERRORM: c_uint = 0xf61b;
pub const STB0899_BBFERRORL: c_uint = 0xf61c;
pub const STB0899_UPKTERRORM: c_uint = 0xf61d;
pub const STB0899_UPKTERRORL: c_uint = 0xf61e;
pub const STB0899_TSTCK: c_uint = 0xff10;
pub const STB0899_TSTRES: c_uint = 0xff11;

pub const STB0899_OFFST_FRESLDPC: c_int = 7;
pub const STB0899_WIDTH_FRESLDPC: c_int = 1;

pub const STB0899_OFFST_FRESRS: c_int = 6;
pub const STB0899_WIDTH_FRESRS: c_int = 1;

pub const STB0899_OFFST_FRESVIT: c_int = 5;
pub const STB0899_WIDTH_FRESVIT: c_int = 1;

pub const STB0899_OFFST_FRESMAS1_2: c_int = 4;
pub const STB0899_WIDTH_FRESMAS1_2: c_int = 1;

pub const STB0899_OFFST_FRESACS: c_int = 3;
pub const STB0899_WIDTH_FRESACS: c_int = 1;

pub const STB0899_OFFST_FRESSYM: c_int = 2;
pub const STB0899_WIDTH_FRESSYM: c_int = 1;

pub const STB0899_OFFST_FRESMAS: c_int = 1;
pub const STB0899_WIDTH_FRESMAS: c_int = 1;

pub const STB0899_OFFST_FRESINIT: c_int = 0;
pub const STB0899_WIDTH_FRESINIT: c_int = 1;
pub const STB0899_TSTOUT: c_uint = 0xff12;

pub const STB0899_OFFST_EN_SIGNATURE: c_int = 7;
pub const STB0899_WIDTH_EN_SIGNATURE: c_int = 1;

pub const STB0899_OFFST_BCLK_CLK: c_int = 6;
pub const STB0899_WIDTH_BCLK_CLK: c_int = 1;

pub const STB0899_OFFST_SGNL_OUT: c_int = 5;
pub const STB0899_WIDTH_SGNL_OUT: c_int = 1;

pub const STB0899_OFFST_TS: c_int = 4;
pub const STB0899_WIDTH_TS: c_int = 1;

pub const STB0899_OFFST_CTEST: c_int = 0;
pub const STB0899_WIDTH_CTEST: c_int = 1;
pub const STB0899_TSTIN: c_uint = 0xff13;

pub const STB0899_OFFST_TEST_IN: c_int = 7;
pub const STB0899_WIDTH_TEST_IN: c_int = 1;

pub const STB0899_OFFST_EN_ADC: c_int = 6;
pub const STB0899_WIDTH_ENADC: c_int = 1;

pub const STB0899_OFFST_SGN_ADC: c_int = 5;
pub const STB0899_WIDTH_SGN_ADC: c_int = 1;

pub const STB0899_OFFST_BCLK_IN: c_int = 4;
pub const STB0899_WIDTH_BCLK_IN: c_int = 1;

pub const STB0899_OFFST_JETONIN_MODE: c_int = 3;
pub const STB0899_WIDTH_JETONIN_MODE: c_int = 1;

pub const STB0899_OFFST_BCLK_VALUE: c_int = 2;
pub const STB0899_WIDTH_BCLK_VALUE: c_int = 1;

pub const STB0899_OFFST_SGNRST_T12: c_int = 1;
pub const STB0899_WIDTH_SGNRST_T12: c_int = 1;

pub const STB0899_OFFST_LOWSP_ENAX: c_int = 0;
pub const STB0899_WIDTH_LOWSP_ENAX: c_int = 1;
pub const STB0899_TSTSYS: c_uint = 0xff14;
pub const STB0899_TSTCHIP: c_uint = 0xff15;
pub const STB0899_TSTFREE: c_uint = 0xff16;
pub const STB0899_TSTI2C: c_uint = 0xff17;
pub const STB0899_BITSPEEDM: c_uint = 0xff1c;
pub const STB0899_BITSPEEDL: c_uint = 0xff1d;
pub const STB0899_TBUSBIT: c_uint = 0xff1e;
pub const STB0899_TSTDIS: c_uint = 0xff24;
pub const STB0899_TSTDISRX: c_uint = 0xff25;
pub const STB0899_TSTJETON: c_uint = 0xff28;
pub const STB0899_TSTDCADJ: c_uint = 0xff40;
pub const STB0899_TSTAGC1: c_uint = 0xff41;
pub const STB0899_TSTAGC1N: c_uint = 0xff42;
pub const STB0899_TSTPOLYPH: c_uint = 0xff48;
pub const STB0899_TSTR: c_uint = 0xff49;
pub const STB0899_TSTAGC2: c_uint = 0xff4a;
pub const STB0899_TSTCTL1: c_uint = 0xff4b;
pub const STB0899_TSTCTL2: c_uint = 0xff4c;
pub const STB0899_TSTCTL3: c_uint = 0xff4d;
pub const STB0899_TSTDEMAP: c_uint = 0xff50;
pub const STB0899_TSTDEMAP2: c_uint = 0xff51;
pub const STB0899_TSTDEMMON: c_uint = 0xff52;
pub const STB0899_TSTRATE: c_uint = 0xff53;
pub const STB0899_TSTSELOUT: c_uint = 0xff54;
pub const STB0899_TSYNC: c_uint = 0xff55;
pub const STB0899_TSTERR: c_uint = 0xff56;
pub const STB0899_TSTRAM1: c_uint = 0xff58;
pub const STB0899_TSTVSELOUT: c_uint = 0xff59;
pub const STB0899_TSTFORCEIN: c_uint = 0xff5a;
pub const STB0899_TSTRS1: c_uint = 0xff5c;
pub const STB0899_TSTRS2: c_uint = 0xff5d;
pub const STB0899_TSTRS3: c_uint = 0xff53;
pub const STB0899_INTBUFSTATUS: c_uint = 0xf200;
pub const STB0899_INTBUFCTRL: c_uint = 0xf201;
pub const STB0899_PCKLENUL: c_uint = 0xf55e;
pub const STB0899_PCKLENLL: c_uint = 0xf55f;
pub const STB0899_RSPCKLEN: c_uint = 0xf560;
// 2 registers
pub const STB0899_SYNCDCST: c_uint = 0xf60c;
// DiSEqC
pub const STB0899_DISCNTRL1: c_uint = 0xf0a0;

pub const STB0899_OFFST_TIMOFF: c_int = 7;
pub const STB0899_WIDTH_TIMOFF: c_int = 1;

pub const STB0899_OFFST_DISEQCRESET: c_int = 6;
pub const STB0899_WIDTH_DISEQCRESET: c_int = 1;

pub const STB0899_OFFST_TIMCMD: c_int = 4;
pub const STB0899_WIDTH_TIMCMD: c_int = 2;

pub const STB0899_OFFST_DISPRECHARGE: c_int = 2;
pub const STB0899_WIDTH_DISPRECHARGE: c_int = 1;

pub const STB0899_OFFST_DISEQCMODE: c_int = 0;
pub const STB0899_WIDTH_DISEQCMODE: c_int = 2;
pub const STB0899_DISCNTRL2: c_uint = 0xf0a1;

pub const STB0899_OFFST_RECEIVER_ON: c_int = 7;
pub const STB0899_WIDTH_RECEIVER_ON: c_int = 1;

pub const STB0899_OFFST_IGNO_SHORT_22K: c_int = 6;
pub const STB0899_WIDTH_IGNO_SHORT_22K: c_int = 1;

pub const STB0899_OFFST_ONECHIP_TRX: c_int = 5;
pub const STB0899_WIDTH_ONECHIP_TRX: c_int = 1;

pub const STB0899_OFFST_EXT_ENVELOP: c_int = 4;
pub const STB0899_WIDTH_EXT_ENVELOP: c_int = 1;

pub const STB0899_OFFST_PIN_SELCT: c_int = 2;
pub const STB0899_WIDTH_PIN_SELCT: c_int = 2;

pub const STB0899_OFFST_IRQ_RXEND: c_int = 1;
pub const STB0899_WIDTH_IRQ_RXEND: c_int = 1;

pub const STB0899_OFFST_IRQ_4NBYTES: c_int = 0;
pub const STB0899_WIDTH_IRQ_4NBYTES: c_int = 1;
pub const STB0899_DISRX_ST0: c_uint = 0xf0a4;

pub const STB0899_OFFST_RXEND: c_int = 7;
pub const STB0899_WIDTH_RXEND: c_int = 1;

pub const STB0899_OFFST_RXACTIVE: c_int = 6;
pub const STB0899_WIDTH_RXACTIVE: c_int = 1;

pub const STB0899_OFFST_SHORT22K: c_int = 5;
pub const STB0899_WIDTH_SHORT22K: c_int = 1;

pub const STB0899_OFFST_CONTTONE: c_int = 4;
pub const STB0899_WIDTH_CONTONE: c_int = 1;

pub const STB0899_OFFST_4BFIFOREDY: c_int = 3;
pub const STB0899_WIDTH_4BFIFOREDY: c_int = 1;

pub const STB0899_OFFST_FIFOEMPTY: c_int = 2;
pub const STB0899_WIDTH_FIFOEMPTY: c_int = 1;

pub const STB0899_OFFST_ABORTTRX: c_int = 0;
pub const STB0899_WIDTH_ABORTTRX: c_int = 1;
pub const STB0899_DISRX_ST1: c_uint = 0xf0a5;

pub const STB0899_OFFST_RXFAIL: c_int = 7;
pub const STB0899_WIDTH_RXFAIL: c_int = 1;

pub const STB0899_OFFST_FIFOPFAIL: c_int = 6;
pub const STB0899_WIDTH_FIFOPFAIL: c_int = 1;

pub const STB0899_OFFST_RXNONBYTES: c_int = 5;
pub const STB0899_WIDTH_RXNONBYTES: c_int = 1;

pub const STB0899_OFFST_FIFOOVF: c_int = 4;
pub const STB0899_WIDTH_FIFOOVF: c_int = 1;

pub const STB0899_OFFST_FIFOBYTENBR: c_int = 0;
pub const STB0899_WIDTH_FIFOBYTENBR: c_int = 4;
pub const STB0899_DISPARITY: c_uint = 0xf0a6;
pub const STB0899_DISFIFO: c_uint = 0xf0a7;
pub const STB0899_DISSTATUS: c_uint = 0xf0a8;

pub const STB0899_OFFST_FIFOFULL: c_int = 6;
pub const STB0899_WIDTH_FIFOFULL: c_int = 1;

pub const STB0899_OFFST_TXIDLE: c_int = 5;
pub const STB0899_WIDTH_TXIDLE: c_int = 1;

pub const STB0899_OFFST_GAPBURST: c_int = 4;
pub const STB0899_WIDTH_GAPBURST: c_int = 1;

pub const STB0899_OFFST_TXFIFOBYTES: c_int = 0;
pub const STB0899_WIDTH_TXFIFOBYTES: c_int = 4;
pub const STB0899_DISF22: c_uint = 0xf0a9;
pub const STB0899_DISF22RX: c_uint = 0xf0aa;
// General Purpose
pub const STB0899_SYSREG: c_uint = 0xf101;
pub const STB0899_ACRPRESC: c_uint = 0xf110;
pub const STB0899_OFFST_RSVD2: c_int = 7;
pub const STB0899_WIDTH_RSVD2: c_int = 1;
pub const STB0899_OFFST_ACRPRESC: c_int = 4;
pub const STB0899_WIDTH_ACRPRESC: c_int = 3;
pub const STB0899_OFFST_RSVD1: c_int = 3;
pub const STB0899_WIDTH_RSVD1: c_int = 1;
pub const STB0899_OFFST_ACRPRESC2: c_int = 0;
pub const STB0899_WIDTH_ACRPRESC2: c_int = 3;
pub const STB0899_ACRDIV1: c_uint = 0xf111;
pub const STB0899_ACRDIV2: c_uint = 0xf112;
pub const STB0899_DACR1: c_uint = 0xf113;
pub const STB0899_DACR2: c_uint = 0xf114;
pub const STB0899_OUTCFG: c_uint = 0xf11c;
pub const STB0899_MODECFG: c_uint = 0xf11d;
pub const STB0899_NCOARSE: c_uint = 0xf1b3;
pub const STB0899_SYNTCTRL: c_uint = 0xf1b6;

pub const STB0899_OFFST_STANDBY: c_int = 7;
pub const STB0899_WIDTH_STANDBY: c_int = 1;

pub const STB0899_OFFST_BYPASSPLL: c_int = 6;
pub const STB0899_WIDTH_BYPASSPLL: c_int = 1;

pub const STB0899_OFFST_SEL1XRATIO: c_int = 5;
pub const STB0899_WIDTH_SEL1XRATIO: c_int = 1;

pub const STB0899_OFFST_SELOSCI: c_int = 1;
pub const STB0899_WIDTH_SELOSCI: c_int = 1;
pub const STB0899_FILTCTRL: c_uint = 0xf1b7;
pub const STB0899_SYSCTRL: c_uint = 0xf1b8;
pub const STB0899_STOPCLK1: c_uint = 0xf1c2;

pub const STB0899_OFFST_STOP_CKINTBUF108: c_int = 7;
pub const STB0899_WIDTH_STOP_CKINTBUF108: c_int = 1;

pub const STB0899_OFFST_STOP_CKINTBUF216: c_int = 6;
pub const STB0899_WIDTH_STOP_CKINTBUF216: c_int = 1;

pub const STB0899_OFFST_STOP_CHK8PSK: c_int = 5;
pub const STB0899_WIDTH_STOP_CHK8PSK: c_int = 1;

pub const STB0899_OFFST_STOP_CKFEC108: c_int = 4;
pub const STB0899_WIDTH_STOP_CKFEC108: c_int = 1;

pub const STB0899_OFFST_STOP_CKFEC216: c_int = 3;
pub const STB0899_WIDTH_STOP_CKFEC216: c_int = 1;

pub const STB0899_OFFST_STOP_CKCORE216: c_int = 2;
pub const STB0899_WIDTH_STOP_CKCORE216: c_int = 1;

pub const STB0899_OFFST_STOP_CKADCI108: c_int = 1;
pub const STB0899_WIDTH_STOP_CKADCI108: c_int = 1;

pub const STB0899_OFFST_STOP_INVCKADCI108: c_int = 0;
pub const STB0899_WIDTH_STOP_INVCKADCI108: c_int = 1;
pub const STB0899_STOPCLK2: c_uint = 0xf1c3;

pub const STB0899_OFFST_STOP_CKS2DMD108: c_int = 2;
pub const STB0899_WIDTH_STOP_CKS2DMD108: c_int = 1;

pub const STB0899_OFFST_STOP_CKPKDLIN108: c_int = 1;
pub const STB0899_WIDTH_STOP_CKPKDLIN108: c_int = 1;

pub const STB0899_OFFST_STOP_CKPKDLIN216: c_int = 0;
pub const STB0899_WIDTH_STOP_CKPKDLIN216: c_int = 1;
pub const STB0899_TSTTNR1: c_uint = 0xf1e0;

pub const STB0899_OFFST_BYPASS_ADC: c_int = 7;
pub const STB0899_WIDTH_BYPASS_ADC: c_int = 1;

pub const STB0899_OFFST_INVADCICKOUT: c_int = 6;
pub const STB0899_WIDTH_INVADCICKOUT: c_int = 1;

pub const STB0899_OFFST_ADCTEST_VOLTAGE: c_int = 4;
pub const STB0899_WIDTH_ADCTEST_VOLTAGE: c_int = 1;

pub const STB0899_OFFST_ADC_RESET: c_int = 3;
pub const STB0899_WIDTH_ADC_RESET: c_int = 1;

pub const STB0899_OFFST_TSTTNR1_2: c_int = 2;
pub const STB0899_WIDTH_TSTTNR1_2: c_int = 1;

pub const STB0899_OFFST_ADCPON: c_int = 1;
pub const STB0899_WIDTH_ADCPON: c_int = 1;

pub const STB0899_OFFST_ADCIN_MODE: c_int = 0;
pub const STB0899_WIDTH_ADCIN_MODE: c_int = 1;
pub const STB0899_TSTTNR2: c_uint = 0xf1e1;

pub const STB0899_OFFST_TSTTNR2_7: c_int = 7;
pub const STB0899_WIDTH_TSTTNR2_7: c_int = 1;

pub const STB0899_OFFST_NOT_DISRX_WIRED: c_int = 6;
pub const STB0899_WIDTH_NOT_DISRX_WIRED: c_int = 1;

pub const STB0899_OFFST_DISEQC_DCURRENT: c_int = 5;
pub const STB0899_WIDTH_DISEQC_DCURRENT: c_int = 1;

pub const STB0899_OFFST_DISEQC_ZCURRENT: c_int = 4;
pub const STB0899_WIDTH_DISEQC_ZCURRENT: c_int = 1;

pub const STB0899_OFFST_DISEQC_SINC_SOURCE: c_int = 2;
pub const STB0899_WIDTH_DISEQC_SINC_SOURCE: c_int = 2;

pub const STB0899_OFFST_SELIQSRC: c_int = 0;
pub const STB0899_WIDTH_SELIQSRC: c_int = 2;
pub const STB0899_TSTTNR3: c_uint = 0xf1e2;
pub const STB0899_I2CCFG: c_uint = 0xf129;

pub const STB0899_OFFST_I2CCFGRSVD: c_int = 4;
pub const STB0899_WIDTH_I2CCFGRSVD: c_int = 4;

pub const STB0899_OFFST_I2CFASTMODE: c_int = 3;
pub const STB0899_WIDTH_I2CFASTMODE: c_int = 1;

pub const STB0899_OFFST_STATUSWR: c_int = 2;
pub const STB0899_WIDTH_STATUSWR: c_int = 1;

pub const STB0899_OFFST_I2CADDRINC: c_int = 0;
pub const STB0899_WIDTH_I2CADDRINC: c_int = 2;
pub const STB0899_I2CRPT: c_uint = 0xf12a;

pub const STB0899_OFFST_I2CTON: c_int = 7;
pub const STB0899_WIDTH_I2CTON: c_int = 1;

pub const STB0899_OFFST_ENARPTLEVEL: c_int = 6;
pub const STB0899_WIDTH_ENARPTLEVEL: c_int = 2;

pub const STB0899_OFFST_SCLTDELAY: c_int = 3;
pub const STB0899_WIDTH_SCLTDELAY: c_int = 1;

pub const STB0899_OFFST_STOPENA: c_int = 2;
pub const STB0899_WIDTH_STOPENA: c_int = 1;

pub const STB0899_OFFST_STOPSDAT2SDA: c_int = 1;
pub const STB0899_WIDTH_STOPSDAT2SDA: c_int = 1;
pub const STB0899_IOPVALUE8: c_uint = 0xf136;
pub const STB0899_IOPVALUE7: c_uint = 0xf137;
pub const STB0899_IOPVALUE6: c_uint = 0xf138;
pub const STB0899_IOPVALUE5: c_uint = 0xf139;
pub const STB0899_IOPVALUE4: c_uint = 0xf13a;
pub const STB0899_IOPVALUE3: c_uint = 0xf13b;
pub const STB0899_IOPVALUE2: c_uint = 0xf13c;
pub const STB0899_IOPVALUE1: c_uint = 0xf13d;
pub const STB0899_IOPVALUE0: c_uint = 0xf13e;
pub const STB0899_GPIO00CFG: c_uint = 0xf140;
pub const STB0899_GPIO01CFG: c_uint = 0xf141;
pub const STB0899_GPIO02CFG: c_uint = 0xf142;
pub const STB0899_GPIO03CFG: c_uint = 0xf143;
pub const STB0899_GPIO04CFG: c_uint = 0xf144;
pub const STB0899_GPIO05CFG: c_uint = 0xf145;
pub const STB0899_GPIO06CFG: c_uint = 0xf146;
pub const STB0899_GPIO07CFG: c_uint = 0xf147;
pub const STB0899_GPIO08CFG: c_uint = 0xf148;
pub const STB0899_GPIO09CFG: c_uint = 0xf149;
pub const STB0899_GPIO10CFG: c_uint = 0xf14a;
pub const STB0899_GPIO11CFG: c_uint = 0xf14b;
pub const STB0899_GPIO12CFG: c_uint = 0xf14c;
pub const STB0899_GPIO13CFG: c_uint = 0xf14d;
pub const STB0899_GPIO14CFG: c_uint = 0xf14e;
pub const STB0899_GPIO15CFG: c_uint = 0xf14f;
pub const STB0899_GPIO16CFG: c_uint = 0xf150;
pub const STB0899_GPIO17CFG: c_uint = 0xf151;
pub const STB0899_GPIO18CFG: c_uint = 0xf152;
pub const STB0899_GPIO19CFG: c_uint = 0xf153;
pub const STB0899_GPIO20CFG: c_uint = 0xf154;
pub const STB0899_SDATCFG: c_uint = 0xf155;
pub const STB0899_SCLTCFG: c_uint = 0xf156;
pub const STB0899_AGCRFCFG: c_uint = 0xf157;
pub const STB0899_GPIO22: c_uint = 0xf158	/* AGCBB2CFG	*/;
pub const STB0899_GPIO21: c_uint = 0xf159  /* AGCBB1CFG	*/;
pub const STB0899_DIRCLKCFG: c_uint = 0xf15a;
pub const STB0899_CLKOUT27CFG: c_uint = 0xf15b;
pub const STB0899_STDBYCFG: c_uint = 0xf15c;
pub const STB0899_CS0CFG: c_uint = 0xf15d;
pub const STB0899_CS1CFG: c_uint = 0xf15e;
pub const STB0899_DISEQCOCFG: c_uint = 0xf15f;
pub const STB0899_GPIO32CFG: c_uint = 0xf160;
pub const STB0899_GPIO33CFG: c_uint = 0xf161;
pub const STB0899_GPIO34CFG: c_uint = 0xf162;
pub const STB0899_GPIO35CFG: c_uint = 0xf163;
pub const STB0899_GPIO36CFG: c_uint = 0xf164;
pub const STB0899_GPIO37CFG: c_uint = 0xf165;
pub const STB0899_GPIO38CFG: c_uint = 0xf166;
pub const STB0899_GPIO39CFG: c_uint = 0xf167;
pub const STB0899_IRQSTATUS_3: c_uint = 0xf120;
pub const STB0899_IRQSTATUS_2: c_uint = 0xf121;
pub const STB0899_IRQSTATUS_1: c_uint = 0xf122;
pub const STB0899_IRQSTATUS_0: c_uint = 0xf123;
pub const STB0899_IRQMSK_3: c_uint = 0xf124;
pub const STB0899_IRQMSK_2: c_uint = 0xf125;
pub const STB0899_IRQMSK_1: c_uint = 0xf126;
pub const STB0899_IRQMSK_0: c_uint = 0xf127;
pub const STB0899_IRQCFG: c_uint = 0xf128;
pub const STB0899_GHOSTREG: c_uint = 0xf000;
pub const STB0899_S2DEMOD: c_uint = 0xf3fc;
pub const STB0899_S2FEC: c_uint = 0xfafc;
