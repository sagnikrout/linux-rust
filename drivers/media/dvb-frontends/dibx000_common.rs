//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/dibx000_common.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dibx000_i2c_interface {
    DIBX000_I2C_INTERFACE_TUNER = 0,
    DIBX000_I2C_INTERFACE_GPIO_1_2 = 1,
    DIBX000_I2C_INTERFACE_GPIO_3_4 = 2,
    DIBX000_I2C_INTERFACE_GPIO_6_7 = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibx000_i2c_master {
pub const DIB3000MC: c_int = 1;
pub const DIB7000: c_int = 2;
pub const DIB7000P: c_int = 11;
pub const DIB7000MC: c_int = 12;
pub const DIB8000: c_int = 13;
    pub device_rev: u16,
    pub selected_interface: dibx000_i2c_interface,
// struct i2c_adapter  tuner_i2c_adap;
    pub gated_tuner_i2c_adap: i2c_adapter,
    pub master_i2c_adap_gpio12: i2c_adapter,
    pub master_i2c_adap_gpio34: i2c_adapter,
    pub master_i2c_adap_gpio67: i2c_adapter,
    pub i2c_adap: *mut i2c_adapter,
    pub i2c_addr: u8,
    pub base_reg: u16,
// for the I2C transfer
    pub msg: [i2c_msg; 34],
    pub i2c_write_buffer: [u8; 8],
    pub i2c_read_buffer: [u8; 2],
    pub i2c_buffer_lock: mutex,
}

// mst,
extern "C" {
    pub fn dibx000_exit_i2c_master(mst: *mut dibx000_i2c_master);
}
extern "C" {
    pub fn dibx000_reset_i2c_master(mst: *mut dibx000_i2c_master);
}
extern "C" {
    pub fn dibx000_i2c_set_speed(i2c_adap: *mut i2c_adapter, speed: u16) -> c_int;
}
pub const BAND_LBAND: c_uint = 0x01;
pub const BAND_UHF: c_uint = 0x02;
pub const BAND_VHF: c_uint = 0x04;
pub const BAND_SBAND: c_uint = 0x08;
pub const BAND_FM: c_uint = 0x10;
pub const BAND_CBAND: c_uint = 0x20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibx000_agc_config {
// defines the capabilities of this AGC-setting - using the BAND_-defines
    pub band_caps: u8,
    pub setup: u16,
    pub inv_gain: u16,
    pub time_stabiliz: u16,
    pub alpha_level: u8,
    pub thlock: u16,
    pub wbd_inv: u8,
    pub wbd_ref: u16,
    pub wbd_sel: u8,
    pub wbd_alpha: u8,
    pub agc1_max: u16,
    pub agc1_min: u16,
    pub agc2_max: u16,
    pub agc2_min: u16,
    pub agc1_pt1: u8,
    pub agc1_pt2: u8,
    pub agc1_pt3: u8,
    pub agc1_slope1: u8,
    pub agc1_slope2: u8,
    pub agc2_pt1: u8,
    pub agc2_pt2: u8,
    pub agc2_slope1: u8,
    pub agc2_slope2: u8,
    pub alpha_mant: u8,
    pub alpha_exp: u8,
    pub beta_mant: u8,
    pub beta_exp: u8,
    pub perform_agc_softsplit: u8,
    pub min: u16,
    pub max: u16,
    pub min_thres: u16,
    pub max_thres: u16,
    pub split: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibx000_bandwidth_config {
    pub internal: u32,
    pub sampling: u32,
    pub pll_prediv: u8,
    pub pll_ratio: u8,
    pub pll_range: u8,
    pub pll_reset: u8,
    pub pll_bypass: u8,
    pub enable_refdiv: u8,
    pub bypclk_div: u8,
    pub IO_CLK_en_core: u8,
    pub ADClkSrc: u8,
    pub modulo: u8,
    pub sad_cfg: u16,
    pub ifreq: u32,
    pub timf: u32,
    pub xtal_hz: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dibx000_adc_states {
    DIBX000_SLOW_ADC_ON = 0,
    DIBX000_SLOW_ADC_OFF,
    DIBX000_ADC_ON,
    DIBX000_ADC_OFF,
    DIBX000_VBG_ENABLE,
    DIBX000_VBG_DISABLE,
}

// Chip output mode.
pub const OUTMODE_HIGH_Z: c_int = 0;
pub const OUTMODE_MPEG2_PAR_GATED_CLK: c_int = 1;
pub const OUTMODE_MPEG2_PAR_CONT_CLK: c_int = 2;
pub const OUTMODE_MPEG2_SERIAL: c_int = 7;
pub const OUTMODE_DIVERSITY: c_int = 4;
pub const OUTMODE_MPEG2_FIFO: c_int = 5;
pub const OUTMODE_ANALOG_ADC: c_int = 6;
pub const INPUT_MODE_OFF: c_uint = 0x11;
pub const INPUT_MODE_DIVERSITY: c_uint = 0x12;
pub const INPUT_MODE_MPEG: c_uint = 0x13;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum frontend_tune_state {
    CT_TUNER_START = 10,
    CT_TUNER_STEP_0,
    CT_TUNER_STEP_1,
    CT_TUNER_STEP_2,
    CT_TUNER_STEP_3,
    CT_TUNER_STEP_4,
    CT_TUNER_STEP_5,
    CT_TUNER_STEP_6,
    CT_TUNER_STEP_7,
    CT_TUNER_STOP,

    CT_AGC_START = 20,
    CT_AGC_STEP_0,
    CT_AGC_STEP_1,
    CT_AGC_STEP_2,
    CT_AGC_STEP_3,
    CT_AGC_STEP_4,
    CT_AGC_STOP,

    CT_DEMOD_START = 30,
    CT_DEMOD_STEP_1,
    CT_DEMOD_STEP_2,
    CT_DEMOD_STEP_3,
    CT_DEMOD_STEP_4,
    CT_DEMOD_STEP_5,
    CT_DEMOD_STEP_6,
    CT_DEMOD_STEP_7,
    CT_DEMOD_STEP_8,
    CT_DEMOD_STEP_9,
    CT_DEMOD_STEP_10,
    CT_DEMOD_STEP_11,
    CT_DEMOD_SEARCH_NEXT = 51,
    CT_DEMOD_STEP_LOCKED,
    CT_DEMOD_STOP,

    CT_DONE = 100,
    CT_SHUTDOWN,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_frontend_parametersContext {
pub const CHANNEL_STATUS_PARAMETERS_UNKNOWN: c_uint = 0x01;
pub const CHANNEL_STATUS_PARAMETERS_SET: c_uint = 0x02;
    pub status: u8,
    pub tune_time_estimation: [u32; 2],
    pub tps_available: i32,
    pub tps: [u16; 9],
}

pub const FE_STATUS_TUNE_FAILED: c_int = 0;

pub const FE_CALLBACK_TIME_NEVER: c_uint = 0xffffffff;
pub const DATA_BUS_ACCESS_MODE_8BIT: c_uint = 0x01;
pub const DATA_BUS_ACCESS_MODE_16BIT: c_uint = 0x02;
pub const DATA_BUS_ACCESS_MODE_NO_ADDRESS_INCREMENT: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibGPIOFunction {
pub const BOARD_GPIO_COMPONENT_BUS_ADAPTER: c_int = 1;
pub const BOARD_GPIO_COMPONENT_DEMOD: c_int = 2;
    pub component: u8,
pub const BOARD_GPIO_FUNCTION_BOARD_ON: c_int = 1;
pub const BOARD_GPIO_FUNCTION_BOARD_OFF: c_int = 2;
pub const BOARD_GPIO_FUNCTION_COMPONENT_ON: c_int = 3;
pub const BOARD_GPIO_FUNCTION_COMPONENT_OFF: c_int = 4;
pub const BOARD_GPIO_FUNCTION_SUBBAND_PWM: c_int = 5;
pub const BOARD_GPIO_FUNCTION_SUBBAND_GPIO: c_int = 6;
    pub function: u8,
// mask, direction and value are used specify which GPIO to change GPIO0
// is LSB and possible GPIO31 is MSB.  The same bit-position as in the
// mask is used for the direction and the value. Direction == 1 is OUT,
// 0 == IN. For direction "OUT" value is either 1 or 0, for direction IN
// value has no meaning.
//
// In case of BOARD_GPIO_FUNCTION_PWM mask is giving the GPIO to be
// used to do the PWM. Direction gives the PWModulator to be used.
// Value gives the PWM value in device-dependent scale.
//
    pub mask: u32,
    pub direction: u32,
    pub value: u32,
}

pub const MAX_NB_SUBBANDS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibSubbandSelection {
    pub /: *mut *mut u8 size; / Actual number of subbands.,
    pub f_mhz: u16,
    pub gpio: dibGPIOFunction,
    pub subband: [}; MAX_NB_SUBBANDS],
}

pub const DEMOD_TIMF_SET: c_uint = 0x00;
pub const DEMOD_TIMF_GET: c_uint = 0x01;
pub const DEMOD_TIMF_UPDATE: c_uint = 0x02;
pub const MPEG_ON_DIBTX: c_int = 1;
pub const DIV_ON_DIBTX: c_int = 2;
pub const ADC_ON_DIBTX: c_int = 3;
pub const DEMOUT_ON_HOSTBUS: c_int = 4;
pub const DIBTX_ON_HOSTBUS: c_int = 5;
pub const MPEG_ON_HOSTBUS: c_int = 6;
