//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/bcm3510_priv.h
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
// Support for the Broadcom BCM3510 ATSC demodulator (1st generation Air2PC)
//
// Copyright (C) 2001-5, B2C2 inc.
//
// GPL/Linux driver written by Patrick Boettcher <patrick.boettcher@posteo.de>
//

pub const PANASONIC_FIRST_IF_BASE_IN_KHz: c_int = 1407500;
pub const BCM3510_SYMBOL_RATE: c_int = 5381000;
pub const JDEC_WAIT_AT_RAM: c_uint = 0x7;
pub const JDEC_EEPROM_LOAD_WAIT: c_uint = 0x4;
// HAB commands
// version
pub const CMD_GET_VERSION_INFO: c_uint = 0x3D;
pub const MSGID_GET_VERSION_INFO: c_uint = 0x15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_get_version_info {
    pub microcode_version: u8,
    pub script_version: u8,
    pub config_version: u8,
    pub demod_version: u8,
    pub PACKED: },
pub const BCM3510_DEF_MICROCODE_VERSION: c_uint = 0x0E;
pub const BCM3510_DEF_SCRIPT_VERSION: c_uint = 0x06;
pub const BCM3510_DEF_CONFIG_VERSION: c_uint = 0x01;
pub const BCM3510_DEF_DEMOD_VERSION: c_uint = 0xB1;
// acquire
pub const CMD_ACQUIRE: c_uint = 0x38;
pub const MSGID_EXT_TUNER_ACQUIRE: c_uint = 0x0A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_ext_acquire {
    pub :4: u8 MODE,
    pub :1: u8 BW,
    pub :1: u8 FA,
    pub :1: u8 NTSCSWEEP,
    pub :1: u8 OFFSET,
    pub /: *mut *mut } PACKED ACQUIRE0; / control_byte,
    pub :3: u8 IF_FREQ,
    pub :1: u8 zero0,
    pub :3: u8 SYM_RATE,
    pub :1: u8 zero1,
    pub /: *mut *mut } PACKED ACQUIRE1; / sym_if,
    pub /: *mut *mut u8 IF_OFFSET0; / IF_Offset_10hz,
    pub IF_OFFSET1: u8,
    pub /: *mut *mut u8 SYM_OFFSET0; / SymbolRateOffset,
    pub SYM_OFFSET1: u8,
    pub /: *mut *mut u8 NTSC_OFFSET0; / NTSC_Offset_10hz,
    pub NTSC_OFFSET1: u8,
    pub PACKED: },
pub const MSGID_INT_TUNER_ACQUIRE: c_uint = 0x0B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_int_acquire {
    pub :4: u8 MODE,
    pub :1: u8 BW,
    pub :1: u8 FA,
    pub :1: u8 NTSCSWEEP,
    pub :1: u8 OFFSET,
    pub /: *mut *mut } PACKED ACQUIRE0; / control_byte,
    pub :3: u8 IF_FREQ,
    pub :1: u8 zero0,
    pub :3: u8 SYM_RATE,
    pub :1: u8 zero1,
    pub /: *mut *mut } PACKED ACQUIRE1; / sym_if,
    pub TUNER_FREQ0: u8,
    pub TUNER_FREQ1: u8,
    pub TUNER_FREQ2: u8,
    pub TUNER_FREQ3: u8,
    pub /: *mut *mut u8 IF_OFFSET0; / IF_Offset_10hz,
    pub IF_OFFSET1: u8,
    pub /: *mut *mut u8 SYM_OFFSET0; / SymbolRateOffset,
    pub SYM_OFFSET1: u8,
    pub /: *mut *mut u8 NTSC_OFFSET0; / NTSC_Offset_10hz,
    pub NTSC_OFFSET1: u8,
    pub PACKED: },
// modes

// IF_FREQS
pub const BCM3510_IF_TERRESTRIAL: c_uint = 0x0;
pub const BCM3510_IF_CABLE: c_uint = 0x1;
pub const BCM3510_IF_USE_CMD: c_uint = 0x7;
// SYM_RATE
pub const BCM3510_SR_8VSB: c_uint = 0x0 /* 5381119 s/sec */;
pub const BCM3510_SR_256QAM: c_uint = 0x1 /* 5360537 s/sec */;
pub const BCM3510_SR_16QAM: c_uint = 0x2 /* 5056971 s/sec */;
pub const BCM3510_SR_MISC: c_uint = 0x3 /* 5000000 s/sec */;
pub const BCM3510_SR_USE_CMD: c_uint = 0x7;
// special symbol rate
pub const CMD_SET_VALUE_NOT_LISTED: c_uint = 0x2d;
pub const MSGID_SET_SYMBOL_RATE_NOT_LISTED: c_uint = 0x0c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_set_sr_not_listed {
    pub HOST_SYM_RATE0: u8,
    pub HOST_SYM_RATE1: u8,
    pub HOST_SYM_RATE2: u8,
    pub HOST_SYM_RATE3: u8,
    pub PACKED: },
// special IF
pub const MSGID_SET_IF_FREQ_NOT_LISTED: c_uint = 0x0d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_set_if_freq_not_listed {
    pub HOST_IF_FREQ0: u8,
    pub HOST_IF_FREQ1: u8,
    pub HOST_IF_FREQ2: u8,
    pub HOST_IF_FREQ3: u8,
    pub PACKED: },
// auto reacquire
pub const CMD_AUTO_PARAM: c_uint = 0x2a;
pub const MSGID_AUTO_REACQUIRE: c_uint = 0x0e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_auto_reacquire {
    pub on/off*/: *mut *mut u8 ACQ :1; /,
    pub :7: u8 unused,
    pub PACKED: },
pub const MSGID_SET_RF_AGC_SEL: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_set_agc {
    pub :1: u8 LVL,
    pub :6: u8 unused,
    pub :1: u8 SEL,
    pub PACKED: },
pub const MSGID_SET_AUTO_INVERSION: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_auto_inversion {
    pub :1: u8 AI,
    pub :7: u8 unused,
    pub PACKED: },
// bert control
pub const CMD_STATE_CONTROL: c_uint = 0x12;
pub const MSGID_BERT_CONTROL: c_uint = 0x0e;
pub const MSGID_BERT_SET: c_uint = 0xfa;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_bert_control {
    pub :1: u8 BE,
    pub :7: u8 unused,
    pub PACKED: },
pub const MSGID_TRI_STATE: c_uint = 0x2e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_tri_state {
    pub /: *mut *mut u8 RE :1; / a/d ram port pins,
    pub /: *mut *mut u8 PE :1; / baud clock pin,
    pub /: *mut *mut u8 AC :1; / a/d clock pin,
    pub /: *mut *mut u8 BE :1; / baud clock pin,
    pub :4: u8 unused,
    pub PACKED: },
// tune
pub const CMD_TUNE: c_uint = 0x38;
pub const MSGID_TUNE: c_uint = 0x16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_tune_ctrl_data_pair {
pub const BITS_8: c_uint = 0x07;
pub const BITS_7: c_uint = 0x06;
pub const BITS_6: c_uint = 0x05;
pub const BITS_5: c_uint = 0x04;
pub const BITS_4: c_uint = 0x03;
pub const BITS_3: c_uint = 0x02;
pub const BITS_2: c_uint = 0x01;
pub const BITS_1: c_uint = 0x00;
    pub :3: u8 size,
    pub :2: u8 unk,
    pub :1: u8 clk_off,
    pub :1: u8 cs0,
    pub :1: u8 cs1,
    pub ctrl: } PACKED,
    pub data: u8,
    pub PACKED: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_tune {
    pub length: u8,
    pub clock_width: u8,
    pub misc: u8,
    pub TUNCTL_state: u8,
    pub ctl_dat: [bcm3510_hab_cmd_tune_ctrl_data_pair; 16],
    pub PACKED: },
pub const CMD_STATUS: c_uint = 0x38;
pub const MSGID_STATUS1: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_status1 {
    pub :4: u8 EQ_MODE,
    pub :2: u8 reserved,
    pub /: *mut *mut u8 QRE :1; / if QSE and the spectrum is inversed,
    pub /: *mut *mut u8 QSE :1; / automatic spectral inversion,
    pub STATUS0: } PACKED,
    pub :1: u8 RECEIVER_LOCK,
    pub :1: u8 FEC_LOCK,
    pub :1: u8 OUT_PLL_LOCK,
    pub :5: u8 reserved,
    pub STATUS1: } PACKED,
    pub :2: u8 reserved,
    pub :1: u8 BW,
    pub /: *mut *mut u8 NTE :1; / NTSC filter sweep enabled,
    pub /: *mut *mut u8 AQI :1; / currently acquiring,
    pub /: *mut *mut u8 FA :1; / fast acquisition,
    pub /: *mut *mut u8 ARI :1; / auto reacquire,
    pub /: *mut *mut u8 TI :1; / programming the tuner,
    pub STATUS2: } PACKED,
    pub STATUS3: u8,
    pub SNR_EST0: u8,
    pub SNR_EST1: u8,
    pub TUNER_FREQ0: u8,
    pub TUNER_FREQ1: u8,
    pub TUNER_FREQ2: u8,
    pub TUNER_FREQ3: u8,
    pub SYM_RATE0: u8,
    pub SYM_RATE1: u8,
    pub SYM_RATE2: u8,
    pub SYM_RATE3: u8,
    pub SYM_OFFSET0: u8,
    pub SYM_OFFSET1: u8,
    pub SYM_ERROR0: u8,
    pub SYM_ERROR1: u8,
    pub IF_FREQ0: u8,
    pub IF_FREQ1: u8,
    pub IF_FREQ2: u8,
    pub IF_FREQ3: u8,
    pub IF_OFFSET0: u8,
    pub IF_OFFSET1: u8,
    pub IF_ERROR0: u8,
    pub IF_ERROR1: u8,
    pub NTSC_FILTER0: u8,
    pub NTSC_FILTER1: u8,
    pub NTSC_FILTER2: u8,
    pub NTSC_FILTER3: u8,
    pub NTSC_OFFSET0: u8,
    pub NTSC_OFFSET1: u8,
    pub NTSC_ERROR0: u8,
    pub NTSC_ERROR1: u8,
    pub INT_AGC_LEVEL0: u8,
    pub INT_AGC_LEVEL1: u8,
    pub EXT_AGC_LEVEL0: u8,
    pub EXT_AGC_LEVEL1: u8,
    pub PACKED: },
pub const MSGID_STATUS2: c_uint = 0x14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm3510_hab_cmd_status2 {
    pub :4: u8 EQ_MODE,
    pub :2: u8 reserved,
    pub :1: u8 QRE,
    pub :1: u8 QSR,
    pub STATUS0: } PACKED,
    pub :1: u8 RL,
    pub :1: u8 FL,
    pub :1: u8 OL,
    pub :5: u8 reserved,
    pub STATUS1: } PACKED,
    pub SYMBOL_RATE0: u8,
    pub SYMBOL_RATE1: u8,
    pub SYMBOL_RATE2: u8,
    pub SYMBOL_RATE3: u8,
    pub LDCERC0: u8,
    pub LDCERC1: u8,
    pub LDCERC2: u8,
    pub LDCERC3: u8,
    pub LDUERC0: u8,
    pub LDUERC1: u8,
    pub LDUERC2: u8,
    pub LDUERC3: u8,
    pub LDBER0: u8,
    pub LDBER1: u8,
    pub LDBER2: u8,
    pub LDBER3: u8,
    pub /: *mut *mut u8 MODE_TYPE :4; / acquire mode 0,
    pub :4: u8 reservd,
    pub MODE_TYPE: },
    pub SNR_EST0: u8,
    pub SNR_EST1: u8,
    pub SIGNAL: u8,
    pub PACKED: },
pub const CMD_SET_RF_BW_NOT_LISTED: c_uint = 0x3f;
pub const MSGID_SET_RF_BW_NOT_LISTED: c_uint = 0x11;
// TODO
