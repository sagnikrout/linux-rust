//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tda18271-priv.h
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

pub const R_ID: c_uint = 0x00	/* ID byte                */;
pub const R_TM: c_uint = 0x01	/* Thermo byte            */;
pub const R_PL: c_uint = 0x02	/* Power level byte       */;
pub const R_EP1: c_uint = 0x03	/* Easy Prog byte 1       */;
pub const R_EP2: c_uint = 0x04	/* Easy Prog byte 2       */;
pub const R_EP3: c_uint = 0x05	/* Easy Prog byte 3       */;
pub const R_EP4: c_uint = 0x06	/* Easy Prog byte 4       */;
pub const R_EP5: c_uint = 0x07	/* Easy Prog byte 5       */;
pub const R_CPD: c_uint = 0x08	/* Cal Post-Divider byte  */;
pub const R_CD1: c_uint = 0x09	/* Cal Divider byte 1     */;
pub const R_CD2: c_uint = 0x0a	/* Cal Divider byte 2     */;
pub const R_CD3: c_uint = 0x0b	/* Cal Divider byte 3     */;
pub const R_MPD: c_uint = 0x0c	/* Main Post-Divider byte */;
pub const R_MD1: c_uint = 0x0d	/* Main Divider byte 1    */;
pub const R_MD2: c_uint = 0x0e	/* Main Divider byte 2    */;
pub const R_MD3: c_uint = 0x0f	/* Main Divider byte 3    */;
pub const R_EB1: c_uint = 0x10	/* Extended byte 1        */;
pub const R_EB2: c_uint = 0x11	/* Extended byte 2        */;
pub const R_EB3: c_uint = 0x12	/* Extended byte 3        */;
pub const R_EB4: c_uint = 0x13	/* Extended byte 4        */;
pub const R_EB5: c_uint = 0x14	/* Extended byte 5        */;
pub const R_EB6: c_uint = 0x15	/* Extended byte 6        */;
pub const R_EB7: c_uint = 0x16	/* Extended byte 7        */;
pub const R_EB8: c_uint = 0x17	/* Extended byte 8        */;
pub const R_EB9: c_uint = 0x18	/* Extended byte 9        */;
pub const R_EB10: c_uint = 0x19	/* Extended byte 10       */;
pub const R_EB11: c_uint = 0x1a	/* Extended byte 11       */;
pub const R_EB12: c_uint = 0x1b	/* Extended byte 12       */;
pub const R_EB13: c_uint = 0x1c	/* Extended byte 13       */;
pub const R_EB14: c_uint = 0x1d	/* Extended byte 14       */;
pub const R_EB15: c_uint = 0x1e	/* Extended byte 15       */;
pub const R_EB16: c_uint = 0x1f	/* Extended byte 16       */;
pub const R_EB17: c_uint = 0x20	/* Extended byte 17       */;
pub const R_EB18: c_uint = 0x21	/* Extended byte 18       */;
pub const R_EB19: c_uint = 0x22	/* Extended byte 19       */;
pub const R_EB20: c_uint = 0x23	/* Extended byte 20       */;
pub const R_EB21: c_uint = 0x24	/* Extended byte 21       */;
pub const R_EB22: c_uint = 0x25	/* Extended byte 22       */;
pub const R_EB23: c_uint = 0x26	/* Extended byte 23       */;
pub const TDA18271_NUM_REGS: c_int = 39;
// ---------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda18271_rf_tracking_filter_cal {
    pub rfmax: u32,
    pub rfband: u8,
    pub rf1_def: u32,
    pub rf2_def: u32,
    pub rf3_def: u32,
    pub rf1: u32,
    pub rf2: u32,
    pub rf3: u32,
    pub rf_a1: i32,
    pub rf_b1: i32,
    pub rf_a2: i32,
    pub rf_b2: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda18271_pll {
    TDA18271_MAIN_PLL,
    TDA18271_CAL_PLL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda18271_ver {
    TDA18271HDC1,
    TDA18271HDC2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda18271_priv {
    pub tda18271_regs: [c_uchar; TDA18271_NUM_REGS],
    pub hybrid_tuner_instance_list: list_head,
    pub i2c_props: tuner_i2c_props,
    pub mode: tda18271_mode,
    pub role: tda18271_role,
    pub gate: tda18271_i2c_gate,
    pub id: tda18271_ver,
    pub output_opt: tda18271_output_options,
    pub small_i2c: tda18271_small_i2c,
    pub /: *mut *mut unsigned int config; / interface to saa713x / tda829x,
    pub cal_initialized:1: c_uint,
    pub tm_rfcal: u8,
    pub maps: *mut tda18271_map_layout,
    pub std: tda18271_std_map,
    pub rf_cal_state: [tda18271_rf_tracking_filter_cal; 8],
    pub lock: mutex,
    pub if_freq: u16,
    pub frequency: u32,
    pub bandwidth: u32,
}

// ---------------------------------------------------------------------
pub const DBG_INFO: c_int = 1;
pub const DBG_MAP: c_int = 2;
pub const DBG_REG: c_int = 4;
pub const DBG_ADV: c_int = 8;
pub const DBG_CAL: c_int = 16;

// ---------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tda18271_map_type {
// tda18271_pll_map
    MAIN_PLL,
    CAL_PLL,
// tda18271_map
    RF_CAL,
    RF_CAL_KMCO,
    RF_CAL_DC_OVER_DT,
    BP_FILTER,
    RF_BAND,
    GAIN_TAPER,
    IR_MEASURE,
}

extern "C" {
    pub fn tda18271_lookup_thermometer(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn tda18271_assign_map_layout(fe: *mut dvb_frontend) -> c_int;
}
// ---------------------------------------------------------------------
extern "C" {
    pub fn tda18271_read_regs(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn tda18271_read_extended(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn tda18271_write_regs(fe: *mut dvb_frontend, idx: c_int, len: c_int) -> c_int;
}
extern "C" {
    pub fn tda18271_init_regs(fe: *mut dvb_frontend) -> c_int;
}
extern "C" {
    pub fn tda18271_calc_main_pll(fe: *mut dvb_frontend, freq: u32) -> c_int;
}
extern "C" {
    pub fn tda18271_calc_cal_pll(fe: *mut dvb_frontend, freq: u32) -> c_int;
}
extern "C" {
    pub fn tda18271_calc_bp_filter(fe: *mut dvb_frontend, freq: *mut u32) -> c_int;
}
extern "C" {
    pub fn tda18271_calc_km(fe: *mut dvb_frontend, freq: *mut u32) -> c_int;
}
extern "C" {
    pub fn tda18271_calc_rf_band(fe: *mut dvb_frontend, freq: *mut u32) -> c_int;
}
extern "C" {
    pub fn tda18271_calc_gain_taper(fe: *mut dvb_frontend, freq: *mut u32) -> c_int;
}
extern "C" {
    pub fn tda18271_calc_ir_measure(fe: *mut dvb_frontend, freq: *mut u32) -> c_int;
}
extern "C" {
    pub fn tda18271_calc_rf_cal(fe: *mut dvb_frontend, freq: *mut u32) -> c_int;
}
