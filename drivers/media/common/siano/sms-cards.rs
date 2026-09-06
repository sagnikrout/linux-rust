//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/common/siano/sms-cards.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Card-specific functions for the Siano SMS1xxx USB dongle
//
// Copyright (c) 2008 Michael Krufky <mkrufky@linuxtv.org>
//

pub const SMS_BOARD_UNKNOWN: c_int = 0;
pub const SMS1XXX_BOARD_SIANO_STELLAR: c_int = 1;
pub const SMS1XXX_BOARD_SIANO_NOVA_A: c_int = 2;
pub const SMS1XXX_BOARD_SIANO_NOVA_B: c_int = 3;
pub const SMS1XXX_BOARD_SIANO_VEGA: c_int = 4;
pub const SMS1XXX_BOARD_HAUPPAUGE_CATAMOUNT: c_int = 5;
pub const SMS1XXX_BOARD_HAUPPAUGE_OKEMO_A: c_int = 6;
pub const SMS1XXX_BOARD_HAUPPAUGE_OKEMO_B: c_int = 7;
pub const SMS1XXX_BOARD_HAUPPAUGE_WINDHAM: c_int = 8;
pub const SMS1XXX_BOARD_HAUPPAUGE_TIGER_MINICARD: c_int = 9;
pub const SMS1XXX_BOARD_HAUPPAUGE_TIGER_MINICARD_R2: c_int = 10;
pub const SMS1XXX_BOARD_SIANO_NICE: c_int = 11;
pub const SMS1XXX_BOARD_SIANO_VENICE: c_int = 12;
pub const SMS1XXX_BOARD_SIANO_STELLAR_ROM: c_int = 13;
pub const SMS1XXX_BOARD_ZTE_DVB_DATA_CARD: c_int = 14;
pub const SMS1XXX_BOARD_ONDA_MDTV_DATA_CARD: c_int = 15;
pub const SMS1XXX_BOARD_SIANO_MING: c_int = 16;
pub const SMS1XXX_BOARD_SIANO_PELE: c_int = 17;
pub const SMS1XXX_BOARD_SIANO_RIO: c_int = 18;
pub const SMS1XXX_BOARD_SIANO_DENVER_1530: c_int = 19;
pub const SMS1XXX_BOARD_SIANO_DENVER_2160: c_int = 20;
pub const SMS1XXX_BOARD_PCTV_77E: c_int = 21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sms_board_gpio_cfg {
    pub lna_vhf_exist: c_int,
    pub lna_vhf_ctrl: c_int,
    pub lna_uhf_exist: c_int,
    pub lna_uhf_ctrl: c_int,
    pub lna_uhf_d_ctrl: c_int,
    pub lna_sband_exist: c_int,
    pub lna_sband_ctrl: c_int,
    pub lna_sband_d_ctrl: c_int,
    pub foreign_lna0_ctrl: c_int,
    pub foreign_lna1_ctrl: c_int,
    pub foreign_lna2_ctrl: c_int,
    pub rf_switch_vhf: c_int,
    pub rf_switch_uhf: c_int,
    pub rf_switch_sband: c_int,
    pub leds_power: c_int,
    pub led0: c_int,
    pub led1: c_int,
    pub led2: c_int,
    pub led3: c_int,
    pub led4: c_int,
    pub ir: c_int,
    pub eeprom_wp: c_int,
    pub mrc_sense: c_int,
    pub mrc_pdn_resetn: c_int,
    pub /: *mut *mut int mrc_gp0; / mrcs spi int,
    pub mrc_gp1: c_int,
    pub mrc_gp2: c_int,
    pub mrc_gp3: c_int,
    pub mrc_gp4: c_int,
    pub host_spi_gsp_ts_int: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sms_board {
    pub type: sms_device_type_st,
    pub fw: [*mut *mut char name,; DEVICE_MODE_MAX],
    pub board_cfg: sms_board_gpio_cfg,
    pub /: *mut *mut *mut char rc_codes; / Name of IR codes table,
// gpios
    pub rf_switch: int led_power, led_hi, led_lo, lna_ctrl,,
    pub intf_num: c_char,
    pub default_mode: c_int,
    pub mtu: c_uint,
    pub crystal: c_uint,
    pub antenna_config: *mut sms_antenna_config_ST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMS_BOARD_EVENTS {
    BOARD_EVENT_POWER_INIT,
    BOARD_EVENT_POWER_SUSPEND,
    BOARD_EVENT_POWER_RESUME,
    BOARD_EVENT_BIND,
    BOARD_EVENT_SCAN_PROG,
    BOARD_EVENT_SCAN_COMP,
    BOARD_EVENT_EMERGENCY_WARNING_SIGNAL,
    BOARD_EVENT_FE_LOCK,
    BOARD_EVENT_FE_UNLOCK,
    BOARD_EVENT_DEMOD_LOCK,
    BOARD_EVENT_DEMOD_UNLOCK,
    BOARD_EVENT_RECEPTION_MAX_4,
    BOARD_EVENT_RECEPTION_3,
    BOARD_EVENT_RECEPTION_2,
    BOARD_EVENT_RECEPTION_1,
    BOARD_EVENT_RECEPTION_LOST_0,
    BOARD_EVENT_MULTIPLEX_OK,
    BOARD_EVENT_MULTIPLEX_ERRORS
}

extern "C" {
    pub fn sms_board_setup(coredev: *mut smscore_device_t) -> c_int;
}
pub const SMS_LED_OFF: c_int = 0;
pub const SMS_LED_LO: c_int = 1;
pub const SMS_LED_HI: c_int = 2;
extern "C" {
    pub fn sms_board_led_feedback(coredev: *mut smscore_device_t, led: c_int) -> c_int;
}
extern "C" {
    pub fn sms_board_power(coredev: *mut smscore_device_t, onoff: c_int) -> c_int;
}
extern "C" {
    pub fn sms_board_lna_control(coredev: *mut smscore_device_t, onoff: c_int) -> c_int;
}
extern "C" {
    pub fn sms_board_load_modules(id: c_int) -> c_int;
}
