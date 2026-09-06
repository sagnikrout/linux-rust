//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dib8000.h
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
pub struct dib8000_config {
    pub output_mpeg2_in_188_bytes: u8,
    pub hostbus_diversity: u8,
    pub tuner_is_baseband: u8,
    pub agc_global): *mut *mut *mut int (update_lna) (struct dvb_frontend , u16,
    pub agc_config_count: u8,
    pub agc: *mut dibx000_agc_config,
    pub pll: *mut dibx000_bandwidth_config,
pub const DIB8000_GPIO_DEFAULT_DIRECTIONS: c_uint = 0xffff;
    pub gpio_dir: u16,
pub const DIB8000_GPIO_DEFAULT_VALUES: c_uint = 0x0000;
    pub gpio_val: u16,

pub const DIB8000_GPIO_DEFAULT_PWM_POS: c_uint = 0xffff;
    pub gpio_pwm_pos: u16,
    pub pwm_freq_div: u16,
    pub before): *mut *mut *mut void (agc_control) (struct dvb_frontend , u8,
    pub drives: u16,
    pub diversity_delay: u16,
    pub div_cfg: u8,
    pub output_mode: u8,
    pub refclksel: u8,
    pub enMpegOutput:1: u8,
    pub plltable: *mut dibx000_bandwidth_config,
}

pub const DEFAULT_DIB8000_I2C_ADDRESS: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib8000_ops {
    pub value): *mut *mut *mut int (set_wbd_ref)(struct dvb_frontend fe, u16,
    pub ratio): *mut *mut dibx000_bandwidth_config pll, u32 bw, u8,
    pub val): *mut *mut *mut int (set_gpio)(struct dvb_frontend fe, u8 num, u8 dir, u8,
    pub fe): *mut *mut void (pwm_agc_reset)(struct dvb_frontend,
    pub fe): *mut *mut *mut i2c_adapter (get_i2c_tuner)(dvb_frontend,
    pub onoff): *mut *mut *mut int (tuner_sleep)(struct dvb_frontend fe, int,
    pub mode): *mut *mut *mut s32 (get_adc_power)(struct dvb_frontend fe, u8,
    pub IQ): *mut *mut *mut int (get_dc_power)(struct dvb_frontend fe, u8,
    pub timf): *mut *mut *mut u32 (ctrl_timf)(struct dvb_frontend fe, uint8_t op, uint32_t,
    pub fe): *mut *mut frontend_tune_state (get_tune_state)(struct dvb_frontend,
    pub tune_state): *mut *mut *mut int (set_tune_state)(struct dvb_frontend fe, enum frontend_tune_state,
    pub fe_slave): *mut *mut *mut int (set_slave_frontend)(struct dvb_frontend fe, struct dvb_frontend,
    pub slave_index): *mut *mut *mut *mut dvb_frontend (get_slave_frontend)(dvb_frontend fe, int,
    pub is_dib8096p): u8 default_addr, u8 first_addr, u8,
    pub gating): *mut *mut *mut *mut i2c_adapter (get_i2c_master)(dvb_frontend fe, enum dibx000_i2c_interface intf, int,
    pub onoff): *mut *mut *mut int (pid_filter_ctrl)(struct dvb_frontend fe, u8,
    pub onoff): *mut *mut *mut int (pid_filter)(struct dvb_frontend fe, u8 id, u16 pid, u8,
    pub cfg): *mut *mut *mut *mut dvb_frontend (init)(i2c_adapter i2c_adap, u8 i2c_addr, dib8000_config,
}

