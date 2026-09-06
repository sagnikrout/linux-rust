//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tscs42xx.h
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


// SPDX-License-Identifier: GPL-2.0
// tscs42xx.h -- TSCS42xx ALSA SoC Audio driver
// Copyright 2017 Tempo Semiconductor, Inc.
// Author: Steven Eckhoff <steven.eckhoff.opensource@gmail.com>
pub const R_HPVOLL: c_uint = 0x0;
pub const R_HPVOLR: c_uint = 0x1;
pub const R_SPKVOLL: c_uint = 0x2;
pub const R_SPKVOLR: c_uint = 0x3;
pub const R_DACVOLL: c_uint = 0x4;
pub const R_DACVOLR: c_uint = 0x5;
pub const R_ADCVOLL: c_uint = 0x6;
pub const R_ADCVOLR: c_uint = 0x7;
pub const R_INVOLL: c_uint = 0x8;
pub const R_INVOLR: c_uint = 0x9;
pub const R_INMODE: c_uint = 0x0B;
pub const R_INSELL: c_uint = 0x0C;
pub const R_INSELR: c_uint = 0x0D;
pub const R_AIC1: c_uint = 0x13;
pub const R_AIC2: c_uint = 0x14;
pub const R_CNVRTR0: c_uint = 0x16;
pub const R_ADCSR: c_uint = 0x17;
pub const R_CNVRTR1: c_uint = 0x18;
pub const R_DACSR: c_uint = 0x19;
pub const R_PWRM1: c_uint = 0x1A;
pub const R_PWRM2: c_uint = 0x1B;
pub const R_CTL: c_uint = 0x1C;
pub const R_CONFIG0: c_uint = 0x1F;
pub const R_CONFIG1: c_uint = 0x20;
pub const R_DMICCTL: c_uint = 0x24;
pub const R_CLECTL: c_uint = 0x25;
pub const R_MUGAIN: c_uint = 0x26;
pub const R_COMPTH: c_uint = 0x27;
pub const R_CMPRAT: c_uint = 0x28;
pub const R_CATKTCL: c_uint = 0x29;
pub const R_CATKTCH: c_uint = 0x2A;
pub const R_CRELTCL: c_uint = 0x2B;
pub const R_CRELTCH: c_uint = 0x2C;
pub const R_LIMTH: c_uint = 0x2D;
pub const R_LIMTGT: c_uint = 0x2E;
pub const R_LATKTCL: c_uint = 0x2F;
pub const R_LATKTCH: c_uint = 0x30;
pub const R_LRELTCL: c_uint = 0x31;
pub const R_LRELTCH: c_uint = 0x32;
pub const R_EXPTH: c_uint = 0x33;
pub const R_EXPRAT: c_uint = 0x34;
pub const R_XATKTCL: c_uint = 0x35;
pub const R_XATKTCH: c_uint = 0x36;
pub const R_XRELTCL: c_uint = 0x37;
pub const R_XRELTCH: c_uint = 0x38;
pub const R_FXCTL: c_uint = 0x39;
pub const R_DACCRWRL: c_uint = 0x3A;
pub const R_DACCRWRM: c_uint = 0x3B;
pub const R_DACCRWRH: c_uint = 0x3C;
pub const R_DACCRRDL: c_uint = 0x3D;
pub const R_DACCRRDM: c_uint = 0x3E;
pub const R_DACCRRDH: c_uint = 0x3F;
pub const R_DACCRADDR: c_uint = 0x40;
pub const R_DCOFSEL: c_uint = 0x41;
pub const R_PLLCTL9: c_uint = 0x4E;
pub const R_PLLCTLA: c_uint = 0x4F;
pub const R_PLLCTLB: c_uint = 0x50;
pub const R_PLLCTLC: c_uint = 0x51;
pub const R_PLLCTLD: c_uint = 0x52;
pub const R_PLLCTLE: c_uint = 0x53;
pub const R_PLLCTLF: c_uint = 0x54;
pub const R_PLLCTL10: c_uint = 0x55;
pub const R_PLLCTL11: c_uint = 0x56;
pub const R_PLLCTL12: c_uint = 0x57;
pub const R_PLLCTL1B: c_uint = 0x60;
pub const R_PLLCTL1C: c_uint = 0x61;
pub const R_TIMEBASE: c_uint = 0x77;
pub const R_DEVIDL: c_uint = 0x7D;
pub const R_DEVIDH: c_uint = 0x7E;
pub const R_RESET: c_uint = 0x80;
pub const R_DACCRSTAT: c_uint = 0x8A;
pub const R_PLLCTL0: c_uint = 0x8E;
pub const R_PLLREFSEL: c_uint = 0x8F;
pub const R_DACMBCEN: c_uint = 0xC7;
pub const R_DACMBCCTL: c_uint = 0xC8;
pub const R_DACMBCMUG1: c_uint = 0xC9;
pub const R_DACMBCTHR1: c_uint = 0xCA;
pub const R_DACMBCRAT1: c_uint = 0xCB;
pub const R_DACMBCATK1L: c_uint = 0xCC;
pub const R_DACMBCATK1H: c_uint = 0xCD;
pub const R_DACMBCREL1L: c_uint = 0xCE;
pub const R_DACMBCREL1H: c_uint = 0xCF;
pub const R_DACMBCMUG2: c_uint = 0xD0;
pub const R_DACMBCTHR2: c_uint = 0xD1;
pub const R_DACMBCRAT2: c_uint = 0xD2;
pub const R_DACMBCATK2L: c_uint = 0xD3;
pub const R_DACMBCATK2H: c_uint = 0xD4;
pub const R_DACMBCREL2L: c_uint = 0xD5;
pub const R_DACMBCREL2H: c_uint = 0xD6;
pub const R_DACMBCMUG3: c_uint = 0xD7;
pub const R_DACMBCTHR3: c_uint = 0xD8;
pub const R_DACMBCRAT3: c_uint = 0xD9;
pub const R_DACMBCATK3L: c_uint = 0xDA;
pub const R_DACMBCATK3H: c_uint = 0xDB;
pub const R_DACMBCREL3L: c_uint = 0xDC;
pub const R_DACMBCREL3H: c_uint = 0xDD;
// Helpers

//
// R_HPVOLL (0x0)
//
// Field Offsets
pub const FB_HPVOLL: c_int = 0;
// Field Masks

// Field Values
pub const FV_HPVOLL_P6DB: c_uint = 0x7F;
pub const FV_HPVOLL_N88PT5DB: c_uint = 0x1;
pub const FV_HPVOLL_MUTE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_HPVOLR (0x1)
//
// Field Offsets
pub const FB_HPVOLR: c_int = 0;
// Field Masks

// Field Values
pub const FV_HPVOLR_P6DB: c_uint = 0x7F;
pub const FV_HPVOLR_N88PT5DB: c_uint = 0x1;
pub const FV_HPVOLR_MUTE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_SPKVOLL (0x2)
//
// Field Offsets
pub const FB_SPKVOLL: c_int = 0;
// Field Masks

// Field Values
pub const FV_SPKVOLL_P12DB: c_uint = 0x7F;
pub const FV_SPKVOLL_N77PT25DB: c_uint = 0x8;
pub const FV_SPKVOLL_MUTE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_SPKVOLR (0x3)
//
// Field Offsets
pub const FB_SPKVOLR: c_int = 0;
// Field Masks

// Field Values
pub const FV_SPKVOLR_P12DB: c_uint = 0x7F;
pub const FV_SPKVOLR_N77PT25DB: c_uint = 0x8;
pub const FV_SPKVOLR_MUTE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_DACVOLL (0x4)
//
// Field Offsets
pub const FB_DACVOLL: c_int = 0;
// Field Masks

// Field Values
pub const FV_DACVOLL_0DB: c_uint = 0xFF;
pub const FV_DACVOLL_N95PT625DB: c_uint = 0x1;
pub const FV_DACVOLL_MUTE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_DACVOLR (0x5)
//
// Field Offsets
pub const FB_DACVOLR: c_int = 0;
// Field Masks

// Field Values
pub const FV_DACVOLR_0DB: c_uint = 0xFF;
pub const FV_DACVOLR_N95PT625DB: c_uint = 0x1;
pub const FV_DACVOLR_MUTE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_ADCVOLL (0x6)
//
// Field Offsets
pub const FB_ADCVOLL: c_int = 0;
// Field Masks

// Field Values
pub const FV_ADCVOLL_P24DB: c_uint = 0xFF;
pub const FV_ADCVOLL_N71PT25DB: c_uint = 0x1;
pub const FV_ADCVOLL_MUTE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_ADCVOLR (0x7)
//
// Field Offsets
pub const FB_ADCVOLR: c_int = 0;
// Field Masks

// Field Values
pub const FV_ADCVOLR_P24DB: c_uint = 0xFF;
pub const FV_ADCVOLR_N71PT25DB: c_uint = 0x1;
pub const FV_ADCVOLR_MUTE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_INVOLL (0x8)
//
// Field Offsets
pub const FB_INVOLL_INMUTEL: c_int = 7;
pub const FB_INVOLL_IZCL: c_int = 6;
pub const FB_INVOLL: c_int = 0;
// Field Masks

// Field Values
pub const FV_INVOLL_INMUTEL_ENABLE: c_uint = 0x1;
pub const FV_INVOLL_INMUTEL_DISABLE: c_uint = 0x0;
pub const FV_INVOLL_IZCL_ENABLE: c_uint = 0x1;
pub const FV_INVOLL_IZCL_DISABLE: c_uint = 0x0;
pub const FV_INVOLL_P30DB: c_uint = 0x3F;
pub const FV_INVOLL_N17PT25DB: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_INVOLR (0x9)
//
// Field Offsets
pub const FB_INVOLR_INMUTER: c_int = 7;
pub const FB_INVOLR_IZCR: c_int = 6;
pub const FB_INVOLR: c_int = 0;
// Field Masks

// Field Values
pub const FV_INVOLR_INMUTER_ENABLE: c_uint = 0x1;
pub const FV_INVOLR_INMUTER_DISABLE: c_uint = 0x0;
pub const FV_INVOLR_IZCR_ENABLE: c_uint = 0x1;
pub const FV_INVOLR_IZCR_DISABLE: c_uint = 0x0;
pub const FV_INVOLR_P30DB: c_uint = 0x3F;
pub const FV_INVOLR_N17PT25DB: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_INMODE (0x0B)
//
// Field Offsets
pub const FB_INMODE_DS: c_int = 0;
// Field Masks

// Field Values
pub const FV_INMODE_DS_LRIN1: c_uint = 0x0;
pub const FV_INMODE_DS_LRIN2: c_uint = 0x1;
// Register Masks

// Register Values

//
// R_INSELL (0x0C)
//
// Field Offsets
pub const FB_INSELL: c_int = 6;
pub const FB_INSELL_MICBSTL: c_int = 4;
// Field Masks

// Field Values
pub const FV_INSELL_IN1: c_uint = 0x0;
pub const FV_INSELL_IN2: c_uint = 0x1;
pub const FV_INSELL_IN3: c_uint = 0x2;
pub const FV_INSELL_D2S: c_uint = 0x3;
pub const FV_INSELL_MICBSTL_OFF: c_uint = 0x0;
pub const FV_INSELL_MICBSTL_10DB: c_uint = 0x1;
pub const FV_INSELL_MICBSTL_20DB: c_uint = 0x2;
pub const FV_INSELL_MICBSTL_30DB: c_uint = 0x3;
// Register Masks

// Register Values

//
// R_INSELR (0x0D)
//
// Field Offsets
pub const FB_INSELR: c_int = 6;
pub const FB_INSELR_MICBSTR: c_int = 4;
// Field Masks

// Field Values
pub const FV_INSELR_IN1: c_uint = 0x0;
pub const FV_INSELR_IN2: c_uint = 0x1;
pub const FV_INSELR_IN3: c_uint = 0x2;
pub const FV_INSELR_D2S: c_uint = 0x3;
pub const FV_INSELR_MICBSTR_OFF: c_uint = 0x0;
pub const FV_INSELR_MICBSTR_10DB: c_uint = 0x1;
pub const FV_INSELR_MICBSTR_20DB: c_uint = 0x2;
pub const FV_INSELR_MICBSTR_30DB: c_uint = 0x3;
// Register Masks

// Register Values

//
// R_AIC1 (0x13)
//
// Field Offsets
pub const FB_AIC1_BCLKINV: c_int = 6;
pub const FB_AIC1_MS: c_int = 5;
pub const FB_AIC1_LRP: c_int = 4;
pub const FB_AIC1_WL: c_int = 2;
pub const FB_AIC1_FORMAT: c_int = 0;
// Field Masks

// Field Values
pub const FV_AIC1_BCLKINV_ENABLE: c_uint = 0x1;
pub const FV_AIC1_BCLKINV_DISABLE: c_uint = 0x0;
pub const FV_AIC1_MS_MASTER: c_uint = 0x1;
pub const FV_AIC1_MS_SLAVE: c_uint = 0x0;
pub const FV_AIC1_LRP_INVERT: c_uint = 0x1;
pub const FV_AIC1_LRP_NORMAL: c_uint = 0x0;
pub const FV_AIC1_WL_16: c_uint = 0x0;
pub const FV_AIC1_WL_20: c_uint = 0x1;
pub const FV_AIC1_WL_24: c_uint = 0x2;
pub const FV_AIC1_WL_32: c_uint = 0x3;
pub const FV_AIC1_FORMAT_RIGHT: c_uint = 0x0;
pub const FV_AIC1_FORMAT_LEFT: c_uint = 0x1;
pub const FV_AIC1_FORMAT_I2S: c_uint = 0x2;
// Register Masks

// Register Values

//
// R_AIC2 (0x14)
//
// Field Offsets
pub const FB_AIC2_DACDSEL: c_int = 6;
pub const FB_AIC2_ADCDSEL: c_int = 4;
pub const FB_AIC2_TRI: c_int = 3;
pub const FB_AIC2_BLRCM: c_int = 0;
// Field Masks

// Field Values
pub const FV_AIC2_BLRCM_DAC_BCLK_LRCLK_SHARED: c_uint = 0x3;
// Register Masks

// Register Values

//
// R_CNVRTR0 (0x16)
//
// Field Offsets
pub const FB_CNVRTR0_ADCPOLR: c_int = 7;
pub const FB_CNVRTR0_ADCPOLL: c_int = 6;
pub const FB_CNVRTR0_AMONOMIX: c_int = 4;
pub const FB_CNVRTR0_ADCMU: c_int = 3;
pub const FB_CNVRTR0_HPOR: c_int = 2;
pub const FB_CNVRTR0_ADCHPDR: c_int = 1;
pub const FB_CNVRTR0_ADCHPDL: c_int = 0;
// Field Masks

// Field Values
pub const FV_CNVRTR0_ADCPOLR_INVERT: c_uint = 0x1;
pub const FV_CNVRTR0_ADCPOLR_NORMAL: c_uint = 0x0;
pub const FV_CNVRTR0_ADCPOLL_INVERT: c_uint = 0x1;
pub const FV_CNVRTR0_ADCPOLL_NORMAL: c_uint = 0x0;
pub const FV_CNVRTR0_ADCMU_ENABLE: c_uint = 0x1;
pub const FV_CNVRTR0_ADCMU_DISABLE: c_uint = 0x0;
pub const FV_CNVRTR0_ADCHPDR_ENABLE: c_uint = 0x1;
pub const FV_CNVRTR0_ADCHPDR_DISABLE: c_uint = 0x0;
pub const FV_CNVRTR0_ADCHPDL_ENABLE: c_uint = 0x1;
pub const FV_CNVRTR0_ADCHPDL_DISABLE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_ADCSR (0x17)
//
// Field Offsets
pub const FB_ADCSR_ABCM: c_int = 6;
pub const FB_ADCSR_ABR: c_int = 3;
pub const FB_ADCSR_ABM: c_int = 0;
// Field Masks

// Field Values
pub const FV_ADCSR_ABCM_AUTO: c_uint = 0x0;
pub const FV_ADCSR_ABCM_32: c_uint = 0x1;
pub const FV_ADCSR_ABCM_40: c_uint = 0x2;
pub const FV_ADCSR_ABCM_64: c_uint = 0x3;
pub const FV_ADCSR_ABR_32: c_uint = 0x0;
pub const FV_ADCSR_ABR_44_1: c_uint = 0x1;
pub const FV_ADCSR_ABR_48: c_uint = 0x2;
pub const FV_ADCSR_ABM_PT25: c_uint = 0x0;
pub const FV_ADCSR_ABM_PT5: c_uint = 0x1;
pub const FV_ADCSR_ABM_1: c_uint = 0x2;
pub const FV_ADCSR_ABM_2: c_uint = 0x3;
// Register Masks

// Register Values

//
// R_CNVRTR1 (0x18)
//
// Field Offsets
pub const FB_CNVRTR1_DACPOLR: c_int = 7;
pub const FB_CNVRTR1_DACPOLL: c_int = 6;
pub const FB_CNVRTR1_DMONOMIX: c_int = 4;
pub const FB_CNVRTR1_DACMU: c_int = 3;
pub const FB_CNVRTR1_DEEMPH: c_int = 2;
pub const FB_CNVRTR1_DACDITH: c_int = 0;
// Field Masks

// Field Values
pub const FV_CNVRTR1_DACPOLR_INVERT: c_uint = 0x1;
pub const FV_CNVRTR1_DACPOLR_NORMAL: c_uint = 0x0;
pub const FV_CNVRTR1_DACPOLL_INVERT: c_uint = 0x1;
pub const FV_CNVRTR1_DACPOLL_NORMAL: c_uint = 0x0;
pub const FV_CNVRTR1_DMONOMIX_ENABLE: c_uint = 0x1;
pub const FV_CNVRTR1_DMONOMIX_DISABLE: c_uint = 0x0;
pub const FV_CNVRTR1_DACMU_ENABLE: c_uint = 0x1;
pub const FV_CNVRTR1_DACMU_DISABLE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_DACSR (0x19)
//
// Field Offsets
pub const FB_DACSR_DBCM: c_int = 6;
pub const FB_DACSR_DBR: c_int = 3;
pub const FB_DACSR_DBM: c_int = 0;
// Field Masks

// Field Values
pub const FV_DACSR_DBCM_AUTO: c_uint = 0x0;
pub const FV_DACSR_DBCM_32: c_uint = 0x1;
pub const FV_DACSR_DBCM_40: c_uint = 0x2;
pub const FV_DACSR_DBCM_64: c_uint = 0x3;
pub const FV_DACSR_DBR_32: c_uint = 0x0;
pub const FV_DACSR_DBR_44_1: c_uint = 0x1;
pub const FV_DACSR_DBR_48: c_uint = 0x2;
pub const FV_DACSR_DBM_PT25: c_uint = 0x0;
pub const FV_DACSR_DBM_PT5: c_uint = 0x1;
pub const FV_DACSR_DBM_1: c_uint = 0x2;
pub const FV_DACSR_DBM_2: c_uint = 0x3;
// Register Masks

// Register Values

//
// R_PWRM1 (0x1A)
//
// Field Offsets
pub const FB_PWRM1_BSTL: c_int = 7;
pub const FB_PWRM1_BSTR: c_int = 6;
pub const FB_PWRM1_PGAL: c_int = 5;
pub const FB_PWRM1_PGAR: c_int = 4;
pub const FB_PWRM1_ADCL: c_int = 3;
pub const FB_PWRM1_ADCR: c_int = 2;
pub const FB_PWRM1_MICB: c_int = 1;
pub const FB_PWRM1_DIGENB: c_int = 0;
// Field Masks

// Field Values
pub const FV_PWRM1_BSTL_ENABLE: c_uint = 0x1;
pub const FV_PWRM1_BSTL_DISABLE: c_uint = 0x0;
pub const FV_PWRM1_BSTR_ENABLE: c_uint = 0x1;
pub const FV_PWRM1_BSTR_DISABLE: c_uint = 0x0;
pub const FV_PWRM1_PGAL_ENABLE: c_uint = 0x1;
pub const FV_PWRM1_PGAL_DISABLE: c_uint = 0x0;
pub const FV_PWRM1_PGAR_ENABLE: c_uint = 0x1;
pub const FV_PWRM1_PGAR_DISABLE: c_uint = 0x0;
pub const FV_PWRM1_ADCL_ENABLE: c_uint = 0x1;
pub const FV_PWRM1_ADCL_DISABLE: c_uint = 0x0;
pub const FV_PWRM1_ADCR_ENABLE: c_uint = 0x1;
pub const FV_PWRM1_ADCR_DISABLE: c_uint = 0x0;
pub const FV_PWRM1_MICB_ENABLE: c_uint = 0x1;
pub const FV_PWRM1_MICB_DISABLE: c_uint = 0x0;
pub const FV_PWRM1_DIGENB_DISABLE: c_uint = 0x1;
pub const FV_PWRM1_DIGENB_ENABLE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_PWRM2 (0x1B)
//
// Field Offsets
pub const FB_PWRM2_D2S: c_int = 7;
pub const FB_PWRM2_HPL: c_int = 6;
pub const FB_PWRM2_HPR: c_int = 5;
pub const FB_PWRM2_SPKL: c_int = 4;
pub const FB_PWRM2_SPKR: c_int = 3;
pub const FB_PWRM2_INSELL: c_int = 2;
pub const FB_PWRM2_INSELR: c_int = 1;
pub const FB_PWRM2_VREF: c_int = 0;
// Field Masks

// Field Values
pub const FV_PWRM2_D2S_ENABLE: c_uint = 0x1;
pub const FV_PWRM2_D2S_DISABLE: c_uint = 0x0;
pub const FV_PWRM2_HPL_ENABLE: c_uint = 0x1;
pub const FV_PWRM2_HPL_DISABLE: c_uint = 0x0;
pub const FV_PWRM2_HPR_ENABLE: c_uint = 0x1;
pub const FV_PWRM2_HPR_DISABLE: c_uint = 0x0;
pub const FV_PWRM2_SPKL_ENABLE: c_uint = 0x1;
pub const FV_PWRM2_SPKL_DISABLE: c_uint = 0x0;
pub const FV_PWRM2_SPKR_ENABLE: c_uint = 0x1;
pub const FV_PWRM2_SPKR_DISABLE: c_uint = 0x0;
pub const FV_PWRM2_INSELL_ENABLE: c_uint = 0x1;
pub const FV_PWRM2_INSELL_DISABLE: c_uint = 0x0;
pub const FV_PWRM2_INSELR_ENABLE: c_uint = 0x1;
pub const FV_PWRM2_INSELR_DISABLE: c_uint = 0x0;
pub const FV_PWRM2_VREF_ENABLE: c_uint = 0x1;
pub const FV_PWRM2_VREF_DISABLE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_CTL (0x1C)
//
// Fiel Offsets
pub const FB_CTL_HPSWEN: c_int = 7;
pub const FB_CTL_HPSWPOL: c_int = 6;
//
// R_CONFIG0 (0x1F)
//
// Field Offsets
pub const FB_CONFIG0_ASDM: c_int = 6;
pub const FB_CONFIG0_DSDM: c_int = 4;
pub const FB_CONFIG0_DC_BYPASS: c_int = 1;
pub const FB_CONFIG0_SD_FORCE_ON: c_int = 0;
// Field Masks

// Field Values
pub const FV_CONFIG0_ASDM_HALF: c_uint = 0x1;
pub const FV_CONFIG0_ASDM_FULL: c_uint = 0x2;
pub const FV_CONFIG0_ASDM_AUTO: c_uint = 0x3;
pub const FV_CONFIG0_DSDM_HALF: c_uint = 0x1;
pub const FV_CONFIG0_DSDM_FULL: c_uint = 0x2;
pub const FV_CONFIG0_DSDM_AUTO: c_uint = 0x3;
pub const FV_CONFIG0_DC_BYPASS_ENABLE: c_uint = 0x1;
pub const FV_CONFIG0_DC_BYPASS_DISABLE: c_uint = 0x0;
pub const FV_CONFIG0_SD_FORCE_ON_ENABLE: c_uint = 0x1;
pub const FV_CONFIG0_SD_FORCE_ON_DISABLE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_CONFIG1 (0x20)
//
// Field Offsets
pub const FB_CONFIG1_EQ2_EN: c_int = 7;
pub const FB_CONFIG1_EQ2_BE: c_int = 4;
pub const FB_CONFIG1_EQ1_EN: c_int = 3;
pub const FB_CONFIG1_EQ1_BE: c_int = 0;
// Field Masks

// Field Values
pub const FV_CONFIG1_EQ2_EN_ENABLE: c_uint = 0x1;
pub const FV_CONFIG1_EQ2_EN_DISABLE: c_uint = 0x0;
pub const FV_CONFIG1_EQ2_BE_PRE: c_uint = 0x0;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ_0: c_uint = 0x1;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_1: c_uint = 0x2;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_2: c_uint = 0x3;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_3: c_uint = 0x4;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_4: c_uint = 0x5;
pub const FV_CONFIG1_EQ2_BE_PRE_EQ0_5: c_uint = 0x6;
pub const FV_CONFIG1_EQ1_EN_ENABLE: c_uint = 0x1;
pub const FV_CONFIG1_EQ1_EN_DISABLE: c_uint = 0x0;
pub const FV_CONFIG1_EQ1_BE_PRE: c_uint = 0x0;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ_0: c_uint = 0x1;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_1: c_uint = 0x2;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_2: c_uint = 0x3;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_3: c_uint = 0x4;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_4: c_uint = 0x5;
pub const FV_CONFIG1_EQ1_BE_PRE_EQ0_5: c_uint = 0x6;
// Register Masks

// Register Values

//
// R_DMICCTL (0x24)
//
// Field Offsets
pub const FB_DMICCTL_DMICEN: c_int = 7;
pub const FB_DMICCTL_DMONO: c_int = 4;
pub const FB_DMICCTL_DMPHADJ: c_int = 2;
pub const FB_DMICCTL_DMRATE: c_int = 0;
// Field Masks

// Field Values
pub const FV_DMICCTL_DMICEN_ENABLE: c_uint = 0x1;
pub const FV_DMICCTL_DMICEN_DISABLE: c_uint = 0x0;
pub const FV_DMICCTL_DMONO_STEREO: c_uint = 0x0;
pub const FV_DMICCTL_DMONO_MONO: c_uint = 0x1;
// Register Masks

// Register Values

//
// R_CLECTL (0x25)
//
// Field Offsets
pub const FB_CLECTL_LVL_MODE: c_int = 4;
pub const FB_CLECTL_WINDOWSEL: c_int = 3;
pub const FB_CLECTL_EXP_EN: c_int = 2;
pub const FB_CLECTL_LIMIT_EN: c_int = 1;
pub const FB_CLECTL_COMP_EN: c_int = 0;
// Field Masks

// Field Values
pub const FV_CLECTL_LVL_MODE_AVG: c_uint = 0x0;
pub const FV_CLECTL_LVL_MODE_PEAK: c_uint = 0x1;
pub const FV_CLECTL_WINDOWSEL_512: c_uint = 0x0;
pub const FV_CLECTL_WINDOWSEL_64: c_uint = 0x1;
pub const FV_CLECTL_EXP_EN_ENABLE: c_uint = 0x1;
pub const FV_CLECTL_EXP_EN_DISABLE: c_uint = 0x0;
pub const FV_CLECTL_LIMIT_EN_ENABLE: c_uint = 0x1;
pub const FV_CLECTL_LIMIT_EN_DISABLE: c_uint = 0x0;
pub const FV_CLECTL_COMP_EN_ENABLE: c_uint = 0x1;
pub const FV_CLECTL_COMP_EN_DISABLE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_MUGAIN (0x26)
//
// Field Offsets
pub const FB_MUGAIN_CLEMUG: c_int = 0;
// Field Masks

// Field Values
pub const FV_MUGAIN_CLEMUG_46PT5DB: c_uint = 0x1F;
pub const FV_MUGAIN_CLEMUG_0DB: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_COMPTH (0x27)
//
// Field Offsets
pub const FB_COMPTH: c_int = 0;
// Field Masks

// Field Values
pub const FV_COMPTH_0DB: c_uint = 0xFF;
pub const FV_COMPTH_N95PT625DB: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_CMPRAT (0x28)
//
// Field Offsets
pub const FB_CMPRAT: c_int = 0;
// Field Masks

// Register Masks

//
// R_CATKTCL (0x29)
//
// Field Offsets
pub const FB_CATKTCL: c_int = 0;
// Field Masks

// Register Masks

//
// R_CATKTCH (0x2A)
//
// Field Offsets
pub const FB_CATKTCH: c_int = 0;
// Field Masks

// Register Masks

//
// R_CRELTCL (0x2B)
//
// Field Offsets
pub const FB_CRELTCL: c_int = 0;
// Field Masks

// Register Masks

//
// R_CRELTCH (0x2C)
//
// Field Offsets
pub const FB_CRELTCH: c_int = 0;
// Field Masks

// Register Masks

//
// R_LIMTH (0x2D)
//
// Field Offsets
pub const FB_LIMTH: c_int = 0;
// Field Masks

// Field Values
pub const FV_LIMTH_0DB: c_uint = 0xFF;
pub const FV_LIMTH_N95PT625DB: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_LIMTGT (0x2E)
//
// Field Offsets
pub const FB_LIMTGT: c_int = 0;
// Field Masks

// Field Values
pub const FV_LIMTGT_0DB: c_uint = 0xFF;
pub const FV_LIMTGT_N95PT625DB: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_LATKTCL (0x2F)
//
// Field Offsets
pub const FB_LATKTCL: c_int = 0;
// Field Masks

// Register Masks

//
// R_LATKTCH (0x30)
//
// Field Offsets
pub const FB_LATKTCH: c_int = 0;
// Field Masks

// Register Masks

//
// R_LRELTCL (0x31)
//
// Field Offsets
pub const FB_LRELTCL: c_int = 0;
// Field Masks

// Register Masks

//
// R_LRELTCH (0x32)
//
// Field Offsets
pub const FB_LRELTCH: c_int = 0;
// Field Masks

// Register Masks

//
// R_EXPTH (0x33)
//
// Field Offsets
pub const FB_EXPTH: c_int = 0;
// Field Masks

// Field Values
pub const FV_EXPTH_0DB: c_uint = 0xFF;
pub const FV_EXPTH_N95PT625DB: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_EXPRAT (0x34)
//
// Field Offsets
pub const FB_EXPRAT: c_int = 0;
// Field Masks

// Register Masks

//
// R_XATKTCL (0x35)
//
// Field Offsets
pub const FB_XATKTCL: c_int = 0;
// Field Masks

// Register Masks

//
// R_XATKTCH (0x36)
//
// Field Offsets
pub const FB_XATKTCH: c_int = 0;
// Field Masks

// Register Masks

//
// R_XRELTCL (0x37)
//
// Field Offsets
pub const FB_XRELTCL: c_int = 0;
// Field Masks

// Register Masks

//
// R_XRELTCH (0x38)
//
// Field Offsets
pub const FB_XRELTCH: c_int = 0;
// Field Masks

// Register Masks

//
// R_FXCTL (0x39)
//
// Field Offsets
pub const FB_FXCTL_3DEN: c_int = 4;
pub const FB_FXCTL_TEEN: c_int = 3;
pub const FB_FXCTL_TNLFBYPASS: c_int = 2;
pub const FB_FXCTL_BEEN: c_int = 1;
pub const FB_FXCTL_BNLFBYPASS: c_int = 0;
// Field Masks

// Field Values
pub const FV_FXCTL_3DEN_ENABLE: c_uint = 0x1;
pub const FV_FXCTL_3DEN_DISABLE: c_uint = 0x0;
pub const FV_FXCTL_TEEN_ENABLE: c_uint = 0x1;
pub const FV_FXCTL_TEEN_DISABLE: c_uint = 0x0;
pub const FV_FXCTL_TNLFBYPASS_ENABLE: c_uint = 0x1;
pub const FV_FXCTL_TNLFBYPASS_DISABLE: c_uint = 0x0;
pub const FV_FXCTL_BEEN_ENABLE: c_uint = 0x1;
pub const FV_FXCTL_BEEN_DISABLE: c_uint = 0x0;
pub const FV_FXCTL_BNLFBYPASS_ENABLE: c_uint = 0x1;
pub const FV_FXCTL_BNLFBYPASS_DISABLE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_DACCRWRL (0x3A)
//
// Field Offsets
pub const FB_DACCRWRL_DACCRWDL: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACCRWRM (0x3B)
//
// Field Offsets
pub const FB_DACCRWRM_DACCRWDM: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACCRWRH (0x3C)
//
// Field Offsets
pub const FB_DACCRWRH_DACCRWDH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACCRRDL (0x3D)
//
// Field Offsets
pub const FB_DACCRRDL: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACCRRDM (0x3E)
//
// Field Offsets
pub const FB_DACCRRDM: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACCRRDH (0x3F)
//
// Field Offsets
pub const FB_DACCRRDH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACCRADDR (0x40)
//
// Field Offsets
pub const FB_DACCRADDR_DACCRADD: c_int = 0;
// Field Masks

// Register Masks

//
// R_DCOFSEL (0x41)
//
// Field Offsets
pub const FB_DCOFSEL_DC_COEF_SEL: c_int = 0;
// Field Masks

// Field Values
pub const FV_DCOFSEL_DC_COEF_SEL_2_N8: c_uint = 0x0;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N9: c_uint = 0x1;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N10: c_uint = 0x2;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N11: c_uint = 0x3;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N12: c_uint = 0x4;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N13: c_uint = 0x5;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N14: c_uint = 0x6;
pub const FV_DCOFSEL_DC_COEF_SEL_2_N15: c_uint = 0x7;
// Register Masks

// Register Values

//
// R_PLLCTL9 (0x4E)
//
// Field Offsets
pub const FB_PLLCTL9_REFDIV_PLL1: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTLA (0x4F)
//
// Field Offsets
pub const FB_PLLCTLA_OUTDIV_PLL1: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTLB (0x50)
//
// Field Offsets
pub const FB_PLLCTLB_FBDIV_PLL1L: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTLC (0x51)
//
// Field Offsets
pub const FB_PLLCTLC_FBDIV_PLL1H: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTLD (0x52)
//
// Field Offsets
pub const FB_PLLCTLD_RZ_PLL1: c_int = 3;
pub const FB_PLLCTLD_CP_PLL1: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTLE (0x53)
//
// Field Offsets
pub const FB_PLLCTLE_REFDIV_PLL2: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTLF (0x54)
//
// Field Offsets
pub const FB_PLLCTLF_OUTDIV_PLL2: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTL10 (0x55)
//
// Field Offsets
pub const FB_PLLCTL10_FBDIV_PLL2L: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTL11 (0x56)
//
// Field Offsets
pub const FB_PLLCTL11_FBDIV_PLL2H: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTL12 (0x57)
//
// Field Offsets
pub const FB_PLLCTL12_RZ_PLL2: c_int = 3;
pub const FB_PLLCTL12_CP_PLL2: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLCTL1B (0x60)
//
// Field Offsets
pub const FB_PLLCTL1B_VCOI_PLL2: c_int = 4;
pub const FB_PLLCTL1B_VCOI_PLL1: c_int = 2;
// Field Masks

// Register Masks

//
// R_PLLCTL1C (0x61)
//
// Field Offsets
pub const FB_PLLCTL1C_PDB_PLL2: c_int = 2;
pub const FB_PLLCTL1C_PDB_PLL1: c_int = 1;
// Field Masks

// Field Values
pub const FV_PLLCTL1C_PDB_PLL2_ENABLE: c_uint = 0x1;
pub const FV_PLLCTL1C_PDB_PLL2_DISABLE: c_uint = 0x0;
pub const FV_PLLCTL1C_PDB_PLL1_ENABLE: c_uint = 0x1;
pub const FV_PLLCTL1C_PDB_PLL1_DISABLE: c_uint = 0x0;
// Register Masks

// Register Values

//
// R_TIMEBASE (0x77)
//
// Field Offsets
pub const FB_TIMEBASE_DIVIDER: c_int = 0;
// Field Masks

// Register Masks

//
// R_DEVIDL (0x7D)
//
// Field Offsets
pub const FB_DEVIDL_DIDL: c_int = 0;
// Field Masks

// Register Masks

//
// R_DEVIDH (0x7E)
//
// Field Offsets
pub const FB_DEVIDH_DIDH: c_int = 0;
// Field Masks

// Register Masks

//
// R_RESET (0x80)
//
// Field Offsets
pub const FB_RESET: c_int = 0;
// Field Masks

// Field Values
pub const FV_RESET_ENABLE: c_uint = 0x85;
// Register Masks

// Register Values

//
// R_DACCRSTAT (0x8A)
//
// Field Offsets
pub const FB_DACCRSTAT_DACCR_BUSY: c_int = 7;
// Field Masks

// Register Masks

//
// R_PLLCTL0 (0x8E)
//
// Field Offsets
pub const FB_PLLCTL0_PLL2_LOCK: c_int = 1;
pub const FB_PLLCTL0_PLL1_LOCK: c_int = 0;
// Field Masks

// Register Masks

//
// R_PLLREFSEL (0x8F)
//
// Field Offsets
pub const FB_PLLREFSEL_PLL2_REF_SEL: c_int = 4;
pub const FB_PLLREFSEL_PLL1_REF_SEL: c_int = 0;
// Field Masks

// Field Values
pub const FV_PLLREFSEL_PLL2_REF_SEL_XTAL_MCLK1: c_uint = 0x0;
pub const FV_PLLREFSEL_PLL2_REF_SEL_MCLK2: c_uint = 0x1;
pub const FV_PLLREFSEL_PLL1_REF_SEL_XTAL_MCLK1: c_uint = 0x0;
pub const FV_PLLREFSEL_PLL1_REF_SEL_MCLK2: c_uint = 0x1;
// Register Masks

// Register Values

//
// R_DACMBCEN (0xC7)
//
// Field Offsets
pub const FB_DACMBCEN_MBCEN3: c_int = 2;
pub const FB_DACMBCEN_MBCEN2: c_int = 1;
pub const FB_DACMBCEN_MBCEN1: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCCTL (0xC8)
//
// Field Offsets
pub const FB_DACMBCCTL_LVLMODE3: c_int = 5;
pub const FB_DACMBCCTL_WINSEL3: c_int = 4;
pub const FB_DACMBCCTL_LVLMODE2: c_int = 3;
pub const FB_DACMBCCTL_WINSEL2: c_int = 2;
pub const FB_DACMBCCTL_LVLMODE1: c_int = 1;
pub const FB_DACMBCCTL_WINSEL1: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCMUG1 (0xC9)
//
// Field Offsets
pub const FB_DACMBCMUG1_PHASE: c_int = 5;
pub const FB_DACMBCMUG1_MUGAIN: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCTHR1 (0xCA)
//
// Field Offsets
pub const FB_DACMBCTHR1_THRESH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCRAT1 (0xCB)
//
// Field Offsets
pub const FB_DACMBCRAT1_RATIO: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCATK1L (0xCC)
//
// Field Offsets
pub const FB_DACMBCATK1L_TCATKL: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCATK1H (0xCD)
//
// Field Offsets
pub const FB_DACMBCATK1H_TCATKH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCREL1L (0xCE)
//
// Field Offsets
pub const FB_DACMBCREL1L_TCRELL: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCREL1H (0xCF)
//
// Field Offsets
pub const FB_DACMBCREL1H_TCRELH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCMUG2 (0xD0)
//
// Field Offsets
pub const FB_DACMBCMUG2_PHASE: c_int = 5;
pub const FB_DACMBCMUG2_MUGAIN: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCTHR2 (0xD1)
//
// Field Offsets
pub const FB_DACMBCTHR2_THRESH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCRAT2 (0xD2)
//
// Field Offsets
pub const FB_DACMBCRAT2_RATIO: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCATK2L (0xD3)
//
// Field Offsets
pub const FB_DACMBCATK2L_TCATKL: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCATK2H (0xD4)
//
// Field Offsets
pub const FB_DACMBCATK2H_TCATKH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCREL2L (0xD5)
//
// Field Offsets
pub const FB_DACMBCREL2L_TCRELL: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCREL2H (0xD6)
//
// Field Offsets
pub const FB_DACMBCREL2H_TCRELH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCMUG3 (0xD7)
//
// Field Offsets
pub const FB_DACMBCMUG3_PHASE: c_int = 5;
pub const FB_DACMBCMUG3_MUGAIN: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCTHR3 (0xD8)
//
// Field Offsets
pub const FB_DACMBCTHR3_THRESH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCRAT3 (0xD9)
//
// Field Offsets
pub const FB_DACMBCRAT3_RATIO: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCATK3L (0xDA)
//
// Field Offsets
pub const FB_DACMBCATK3L_TCATKL: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCATK3H (0xDB)
//
// Field Offsets
pub const FB_DACMBCATK3H_TCATKH: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCREL3L (0xDC)
//
// Field Offsets
pub const FB_DACMBCREL3L_TCRELL: c_int = 0;
// Field Masks

// Register Masks

//
// R_DACMBCREL3H (0xDD)
//
// Field Offsets
pub const FB_DACMBCREL3H_TCRELH: c_int = 0;
// Field Masks

// Register Masks

