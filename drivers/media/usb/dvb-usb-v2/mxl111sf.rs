//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/mxl111sf.h
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
// Copyright (C) 2010-2014 Michael Krufky (mkrufky@linuxtv.org)
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

// Max transfer size done by I2C transfer functions
pub const MXL_MAX_XFER_SIZE: c_int = 64;
pub const MXL_EP1_REG_READ: c_int = 1;
pub const MXL_EP2_REG_WRITE: c_int = 2;
pub const MXL_EP3_INTERRUPT: c_int = 3;
pub const MXL_EP4_MPEG2: c_int = 4;
pub const MXL_EP5_I2S: c_int = 5;
pub const MXL_EP6_656: c_int = 6;
pub const MXL_EP6_MPEG2: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl111sf_current_mode {
    mxl_mode_dvbt = MXL_EP4_MPEG2,
    mxl_mode_mh   = MXL_EP5_I2S,
    mxl_mode_atsc = MXL_EP6_MPEG2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl111sf_gpio_port_expander {
    mxl111sf_gpio_hw,
    mxl111sf_PCA9534,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl111sf_adap_state {
    pub alt_mode: c_int,
    pub gpio_mode: c_int,
    pub device_mode: c_int,
    pub ep6_clockphase: c_int,
    pub ): *mut *mut int (fe_init)(struct dvb_frontend,
    pub ): *mut *mut int (fe_sleep)(struct dvb_frontend,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl111sf_pads {
    MXL111SF_PAD_RF_INPUT,
    MXL111SF_PAD_OUTPUT,
    MXL111SF_NUM_PADS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl111sf_state {
    pub d: *mut dvb_usb_device,
    pub gpio_port_expander: mxl111sf_gpio_port_expander,
    pub port_expander_addr: u8,
    pub chip_id: u8,
    pub chip_ver: u8,
pub const MXL111SF_V6: c_int = 1;
pub const MXL111SF_V8_100: c_int = 2;
pub const MXL111SF_V8_200: c_int = 3;
    pub chip_rev: u8,

    pub current_mode: mxl111sf_current_mode,

pub const MXL_TUNER_MODE: c_int = 0;
pub const MXL_SOC_MODE: c_int = 1;
pub const MXL_DEV_MODE_MASK: c_uint = 0x01;

    pub device_mode: c_int,

// use usb alt setting 1 for EP4 ISOC transfer (dvb-t),
//
    pub alt_mode: c_int,
    pub gpio_mode: c_int,
    pub tv: tveeprom,
    pub fe_lock: mutex,
    pub num_frontends: u8,
    pub adap_state: [mxl111sf_adap_state; 3],
    pub sndbuf: [u8; MXL_MAX_XFER_SIZE],
    pub rcvbuf: [u8; MXL_MAX_XFER_SIZE],
    pub msg_lock: mutex,

    pub tuner: media_entity,
    pub tuner_pads: [media_pad; MXL111SF_NUM_PADS],
}

extern "C" {
    pub fn mxl111sf_read_reg(state: *mut mxl111sf_state, addr: u8, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn mxl111sf_write_reg(state: *mut mxl111sf_state, addr: u8, data: u8) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl111sf_reg_ctrl_info {
    pub addr: u8,
    pub mask: u8,
    pub data: u8,
}

// needed for hardware i2c functions in mxl111sf-i2c.c:
// mxl111sf_i2c_send_data / mxl111sf_i2c_get_data

pub const MXL_I2C_DBG: c_uint = 0x04;
pub const MXL_ADV_DBG: c_uint = 0x10;

// The following allows the mxl_fail() macro defined below to work
// in externel modules, such as mxl111sf-tuner.ko, even though
// dvb_usb_mxl111sf_debug is not defined within those modules

