//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dib7000m.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib7000m_config {
    pub dvbt_mode: u8,
    pub output_mpeg2_in_188_bytes: u8,
    pub hostbus_diversity: u8,
    pub tuner_is_baseband: u8,
    pub mobile_mode: u8,
    pub agc_global): *mut *mut *mut int (update_lna) (struct dvb_frontend , u16,
    pub agc_config_count: u8,
    pub agc: *mut dibx000_agc_config,
    pub bw: *mut dibx000_bandwidth_config,
pub const DIB7000M_GPIO_DEFAULT_DIRECTIONS: c_uint = 0xffff;
    pub gpio_dir: u16,
pub const DIB7000M_GPIO_DEFAULT_VALUES: c_uint = 0x0000;
    pub gpio_val: u16,

pub const DIB7000M_GPIO_DEFAULT_PWM_POS: c_uint = 0xffff;
    pub gpio_pwm_pos: u16,
    pub pwm_freq_div: u16,
    pub quartz_direct: u8,
    pub input_clk_is_div_2: u8,
    pub before): *mut *mut *mut int (agc_control) (struct dvb_frontend , u8,
}

pub const DEFAULT_DIB7000M_I2C_ADDRESS: c_int = 18;

extern "C" {
    pub fn dib7000m_pid_filter(: *mut dvb_frontend, id: u8, pid: u16, onoff: u8) -> c_int;
}
extern "C" {
    pub fn dib7000m_pid_filter_ctrl(fe: *mut dvb_frontend, onoff: u8) -> c_int;
}

// TODO
extern "C" {
    pub fn dib7000m_set_gpio(demod: *mut dibDemod, num: UCHAR, dir: UCHAR, val: UCHAR) -> INT;
}
extern "C" {
    pub fn dib7000m_enable_vbg_voltage(demod: *mut dibDemod) -> INT;
}
extern "C" {
    pub fn dib7000m_set_hostbus_diversity(demod: *mut dibDemod, onoff: UCHAR);
}
extern "C" {
    pub fn dib7000m_get_current_agc_global(demod: *mut dibDemod) -> USHORT;
}
//
