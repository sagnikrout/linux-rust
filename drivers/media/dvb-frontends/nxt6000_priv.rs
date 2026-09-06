//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/nxt6000_priv.h
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
//
// Public Include File for DRV6000 users
// (ie. NxtWave Communications - NXT6000 demodulator driver)
//
// Copyright (C) 2001 NxtWave Communications, Inc.
//
// Nxt6000 Register Addresses and Bit Masks
// Maximum Register Number

// 0x1B A_VIT_BER_0  aka 0x3A

// 0x1D A_VIT_BER_TIMER_0 aka 0x38

// 0x21 RS_COR_STAT

// 0x22 RS_COR_INTEN

// 0x23 RS_COR_INSTAT

// 0x24 RS_COR_SYNC_PARAM

// 0x25 BER_CTRL

// 0x26 BER_PAY

// 0x27 BER_PKT_L

// 0x30 VIT_COR_CTL

// 0x32 VIT_SYNC_STATUS

// 0x33 VIT_COR_INTEN

// 0x34 VIT_COR_INTSTAT

// 0x38 VIT_BERTIME_2

// 0x39 VIT_BERTIME_1

// 0x3A VIT_BERTIME_0

// 0x38 OFDM_BERTimer *//* Use the alias registers

// 0x3A VIT_BER_TIMER_0 *//* Use the alias registers

// 0x3B VIT_BER_1

// 0x3C VIT_BER_0

// 0x40 OFDM_COR_CTL

// 0x41 OFDM_COR_STAT

// 0x42 OFDM_COR_INTEN

// 0x43 OFDM_COR_INSTAT

// 0x44 OFDM_COR_MODEGUARD

// 0x45 OFDM_AGC_CTL

// 0x48 OFDM_AGC_TARGET

// 0x49 OFDM_AGC_GAIN_1

// 0x4B OFDM_ITB_CTL

// 0x49 AGC_GAIN_1

// 0x4A AGC_GAIN_2

// 0x4C OFDM_ITB_FREQ_1

// 0x4D OFDM_ITB_FREQ_2

// 0x4E  OFDM_CAS_CTL

// 0x4F CAS_FREQ

// 0x51 OFDM_SYR_CTL

// 0x52 OFDM_SYR_STAT

// 0x55 OFDM_SYR_OFFSET_1

// 0x56 OFDM_SYR_OFFSET_2

// 0x58 OFDM_SCR_CTL

// 0x59 OFDM_PPM_CTL_1

// 0x5B OFDM_TRL_NOMINALRATE_1

// 0x5C OFDM_TRL_NOMINALRATE_2

// 0x5D OFDM_TRL_TIME_1

// 0x60 OFDM_CRL_FREQ_1

// 0x63 OFDM_CHC_CTL_1

// 0x64 OFDM_CHC_SNR

// 0x65 OFDM_BDI_CTL

// 0x67 OFDM_TPS_RCVD_1

// 0x68 OFDM_TPS_RCVD_2

// 0x69 OFDM_TPS_RCVD_3

// 0x6A OFDM_TPS_RCVD_4

// 0x6B OFDM_TPS_RESERVED_1

// 0x6C OFDM_TPS_RESERVED_2

// 0x73 OFDM_MSC_REV

// 0x76 OFDM_SNR_CARRIER_2

// 0x80 ANALOG_CONTROL_0

// 0x81 ENABLE_TUNER_IIC

// 0x82 EN_DMD_RACQ

// 0x84 SNR_COMMAND

// 0x85 SNRCARRIERNUMBER_LSB

// 0x87 SNRMINTHRESHOLD_LSB

// 0x89 SNR_PER_CARRIER_LSB

// 0x8B SNRBELOWTHRESHOLD_LSB

// 0x91 RF_AGC_VAL_1

// 0x92 RF_AGC_STATUS

// 0x98 DIAG_CONFIG

// 0x99 SUB_DIAG_MODE_SEL

// 0x9A TS_FORMAT

