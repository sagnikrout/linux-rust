//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/si2165_priv.h
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
// Driver for Silicon Labs SI2165 DVB-C/-T Demodulator
//
// Copyright (C) 2013-2017 Matthias Schwarzott <zzam@gentoo.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct si2165_config {
// i2c addr
// possible values: 0x64,0x65,0x66,0x67
//
    pub i2c_addr: u8,
// external clock or XTAL
    pub chip_mode: u8,
// frequency of external clock or xtal in Hz
// possible values: 4000000, 16000000, 20000000, 240000000, 27000000
//
    pub ref_freq_hz: u32,
// invert the spectrum
    pub inversion: bool,
}

pub const REG_CHIP_MODE: c_uint = 0x0000;
pub const REG_CHIP_REVCODE: c_uint = 0x0023;
pub const REV_CHIP_TYPE: c_uint = 0x0118;
pub const REG_CHIP_INIT: c_uint = 0x0050;
pub const REG_INIT_DONE: c_uint = 0x0054;
pub const REG_START_INIT: c_uint = 0x0096;
pub const REG_PLL_DIVL: c_uint = 0x00a0;
pub const REG_RST_ALL: c_uint = 0x00c0;
pub const REG_LOCK_TIMEOUT: c_uint = 0x00c4;
pub const REG_AUTO_RESET: c_uint = 0x00cb;
pub const REG_OVERSAMP: c_uint = 0x00e4;
pub const REG_IF_FREQ_SHIFT: c_uint = 0x00e8;
pub const REG_DVB_STANDARD: c_uint = 0x00ec;
pub const REG_DSP_CLOCK: c_uint = 0x0104;
pub const REG_ADC_RI8: c_uint = 0x0123;
pub const REG_ADC_RI1: c_uint = 0x012a;
pub const REG_ADC_RI2: c_uint = 0x012b;
pub const REG_ADC_RI3: c_uint = 0x012c;
pub const REG_ADC_RI4: c_uint = 0x012d;
pub const REG_ADC_RI5: c_uint = 0x012e;
pub const REG_ADC_RI6: c_uint = 0x012f;
pub const REG_AGC_CRESTF_DBX8: c_uint = 0x0150;
pub const REG_AGC_UNFREEZE_THR: c_uint = 0x015b;
pub const REG_AGC2_MIN: c_uint = 0x016e;
pub const REG_AGC2_KACQ: c_uint = 0x016c;
pub const REG_AGC2_KLOC: c_uint = 0x016d;
pub const REG_AGC2_OUTPUT: c_uint = 0x0170;
pub const REG_AGC2_CLKDIV: c_uint = 0x0171;
pub const REG_AGC_IF_TRI: c_uint = 0x018b;
pub const REG_AGC_IF_SLR: c_uint = 0x0190;
pub const REG_AAF_CRESTF_DBX8: c_uint = 0x01a0;
pub const REG_ACI_CRESTF_DBX8: c_uint = 0x01c8;
pub const REG_SWEEP_STEP: c_uint = 0x0232;
pub const REG_KP_LOCK: c_uint = 0x023a;
pub const REG_UNKNOWN_24C: c_uint = 0x024c;
pub const REG_CENTRAL_TAP: c_uint = 0x0261;
pub const REG_C_N: c_uint = 0x026c;
pub const REG_EQ_AUTO_CONTROL: c_uint = 0x0278;
pub const REG_UNKNOWN_27C: c_uint = 0x027c;
pub const REG_START_SYNCHRO: c_uint = 0x02e0;
pub const REG_REQ_CONSTELLATION: c_uint = 0x02f4;
pub const REG_T_BANDWIDTH: c_uint = 0x0308;
pub const REG_FREQ_SYNC_RANGE: c_uint = 0x030c;
pub const REG_IMPULSIVE_NOISE_REM: c_uint = 0x031c;
pub const REG_WDOG_AND_BOOT: c_uint = 0x0341;
pub const REG_PATCH_VERSION: c_uint = 0x0344;
pub const REG_ADDR_JUMP: c_uint = 0x0348;
pub const REG_UNKNOWN_350: c_uint = 0x0350;
pub const REG_EN_RST_ERROR: c_uint = 0x035c;
pub const REG_DCOM_CONTROL_BYTE: c_uint = 0x0364;
pub const REG_DCOM_ADDR: c_uint = 0x0368;
pub const REG_DCOM_DATA: c_uint = 0x036c;
pub const REG_RST_CRC: c_uint = 0x0379;
pub const REG_GP_REG0_LSB: c_uint = 0x0384;
pub const REG_GP_REG0_MSB: c_uint = 0x0387;
pub const REG_CRC: c_uint = 0x037a;
pub const REG_CHECK_SIGNAL: c_uint = 0x03a8;
pub const REG_CBER_RST: c_uint = 0x0424;
pub const REG_CBER_BIT: c_uint = 0x0428;
pub const REG_CBER_ERR: c_uint = 0x0430;
pub const REG_CBER_AVAIL: c_uint = 0x0434;
pub const REG_PS_LOCK: c_uint = 0x0440;
pub const REG_UNCOR_CNT: c_uint = 0x0468;
pub const REG_BER_RST: c_uint = 0x046c;
pub const REG_BER_PKT: c_uint = 0x0470;
pub const REG_BER_BIT: c_uint = 0x0478;
pub const REG_BER_AVAIL: c_uint = 0x047c;
pub const REG_FEC_LOCK: c_uint = 0x04e0;
pub const REG_TS_DATA_MODE: c_uint = 0x04e4;
pub const REG_TS_CLK_MODE: c_uint = 0x04e5;
pub const REG_TS_TRI: c_uint = 0x04ef;
pub const REG_TS_SLR: c_uint = 0x04f4;
pub const REG_RSSI_ENABLE: c_uint = 0x0641;
pub const REG_RSSI_PAD_CTRL: c_uint = 0x0646;
pub const REG_TS_PARALLEL_MODE: c_uint = 0x08f8;
