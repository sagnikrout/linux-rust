//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/bt8xx/dst_common.h
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

pub const NO_DELAY: c_int = 0;
pub const LONG_DELAY: c_int = 1;
pub const DEVICE_INIT: c_int = 2;
pub const DELAY: c_int = 1;
pub const DST_TYPE_IS_SAT: c_int = 0;
pub const DST_TYPE_IS_TERR: c_int = 1;
pub const DST_TYPE_IS_CABLE: c_int = 2;
pub const DST_TYPE_IS_ATSC: c_int = 3;
pub const DST_TYPE_HAS_TS188: c_int = 1;
pub const DST_TYPE_HAS_TS204: c_int = 2;
pub const DST_TYPE_HAS_SYMDIV: c_int = 4;
pub const DST_TYPE_HAS_FW_1: c_int = 8;
pub const DST_TYPE_HAS_FW_2: c_int = 16;
pub const DST_TYPE_HAS_FW_3: c_int = 32;
pub const DST_TYPE_HAS_FW_BUILD: c_int = 64;
pub const DST_TYPE_HAS_OBS_REGS: c_int = 128;
pub const DST_TYPE_HAS_INC_COUNT: c_int = 256;
pub const DST_TYPE_HAS_MULTI_FE: c_int = 512;
pub const DST_TYPE_HAS_NEWTUNE_2: c_int = 1024;
pub const DST_TYPE_HAS_DBOARD: c_int = 2048;
pub const DST_TYPE_HAS_VLF: c_int = 4096;
// Card capability list
pub const DST_TYPE_HAS_MAC: c_int = 1;
pub const DST_TYPE_HAS_DISEQC3: c_int = 2;
pub const DST_TYPE_HAS_DISEQC4: c_int = 4;
pub const DST_TYPE_HAS_DISEQC5: c_int = 8;
pub const DST_TYPE_HAS_MOTO: c_int = 16;
pub const DST_TYPE_HAS_CA: c_int = 32;

pub const DST_TYPE_HAS_SESSION: c_int = 128;
pub const TUNER_TYPE_MULTI: c_int = 1;
pub const TUNER_TYPE_UNKNOWN: c_int = 2;
// DVB-S
pub const TUNER_TYPE_L64724: c_int = 4;
pub const TUNER_TYPE_STV0299: c_int = 8;
pub const TUNER_TYPE_MB86A15: c_int = 16;
// DVB-T
pub const TUNER_TYPE_TDA10046: c_int = 32;
// ATSC
pub const TUNER_TYPE_NXT200x: c_int = 64;
pub const RDC_8820_PIO_0_DISABLE: c_int = 0;
pub const RDC_8820_PIO_0_ENABLE: c_int = 1;
pub const RDC_8820_INT: c_int = 2;
pub const RDC_8820_RESET: c_int = 4;
// DST Communication
pub const GET_REPLY: c_int = 1;
pub const NO_REPLY: c_int = 0;
pub const GET_ACK: c_int = 1;
pub const FIXED_COMM: c_int = 8;
pub const ACK: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_state {
    pub i2c: *mut *mut i2c_adapter,
    pub bt: *mut *mut bt878,
// configuration settings
    pub config: *const *const dst_config,
    pub frontend: dvb_frontend,
// private ASIC data
    pub tx_tuna: [u8; 10],
    pub rx_tuna: [u8; 10],
    pub rxbuffer: [u8; 10],
    pub diseq_flags: u8,
    pub dst_type: u8,
    pub type_flags: u32,
    pub /: *mut *mut u32 frequency; / intermediate frequency in kHz for QPSK,
    pub inversion: fe_spectral_inversion,
    pub /: *mut *mut u32 symbol_rate; / symbol rate in Symbols per second,
    pub fec: fe_code_rate,
    pub voltage: fe_sec_voltage,
    pub tone: fe_sec_tone_mode,
    pub decode_freq: u32,
    pub decode_lock: u8,
    pub decode_strength: u16,
    pub decode_snr: u16,
    pub cur_jiff: c_ulong,
    pub k22: u8,
    pub bandwidth: u32,
    pub dst_hw_cap: u32,
    pub dst_fw_version: u8,
    pub minicmd: fe_sec_mini_cmd,
    pub modulation: fe_modulation,
    pub messages: [u8; 256],
    pub mac_address: [u8; 8],
    pub fw_version: [u8; 8],
    pub card_info: [u8; 8],
    pub vendor: [u8; 8],
    pub board_info: [u8; 8],
    pub tuner_type: u32,
    pub tuner_name: *mut c_char,
    pub dst_mutex: mutex,
    pub fw_name: [c_char; 8],
    pub dst_ca: *mut dvb_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tuner_types {
    pub tuner_type: u32,
    pub tuner_name: *mut c_char,
    pub board_name: *mut c_char,
    pub fw_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_types {
    pub device_id: *mut c_char,
    pub offset: c_int,
    pub dst_type: u8,
    pub type_flags: u32,
    pub dst_feature: u32,
    pub tuner_type: u32,
}

// the ASIC i2c address
extern "C" {
    pub fn rdc_reset_state(state: *mut dst_state) -> c_int;
}
extern "C" {
    pub fn dst_wait_dst_ready(state: *mut dst_state, delay_mode: u8) -> c_int;
}
extern "C" {
    pub fn dst_pio_disable(state: *mut dst_state) -> c_int;
}
extern "C" {
    pub fn dst_error_recovery(state: *mut *mut dst_state) -> c_int;
}
extern "C" {
    pub fn dst_error_bailout(state: *mut dst_state) -> c_int;
}
extern "C" {
    pub fn dst_comm_init(state: *mut *mut dst_state) -> c_int;
}
extern "C" {
    pub fn write_dst(state: *mut dst_state, data: *mut *mut u8, len: u8) -> c_int;
}
extern "C" {
    pub fn read_dst(state: *mut dst_state, ret: *mut *mut u8, len: u8) -> c_int;
}
extern "C" {
    pub fn dst_check_sum(buf: *mut *mut u8, len: u32) -> u8;
}
extern "C" {
    pub fn dst_attach(state: *mut *mut dst_state, dvb_adapter: *mut dvb_adapter) -> *mut dst_state;
}
