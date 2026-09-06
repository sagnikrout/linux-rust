//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/tuner-types.h
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
// descriptions for simple tuners.
//
// enum param_type - type of the tuner pameters
//
// @TUNER_PARAM_TYPE_RADIO:	Tuner params are for FM and/or AM radio
// @TUNER_PARAM_TYPE_PAL:	Tuner params are for PAL color TV standard
// @TUNER_PARAM_TYPE_SECAM:	Tuner params are for SECAM color TV standard
// @TUNER_PARAM_TYPE_NTSC:	Tuner params are for NTSC color TV standard
// @TUNER_PARAM_TYPE_DIGITAL:	Tuner params are for digital TV
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum param_type {
    TUNER_PARAM_TYPE_RADIO,
    TUNER_PARAM_TYPE_PAL,
    TUNER_PARAM_TYPE_SECAM,
    TUNER_PARAM_TYPE_NTSC,
    TUNER_PARAM_TYPE_DIGITAL,
}

//
// struct tuner_range - define the frequencies supported by the tuner
//
// @limit:		Max frequency supported by that range, in 62.5 kHz
// (TV) or 62.5 Hz (Radio), as defined by
// V4L2_TUNER_CAP_LOW.
// @config:		Value of the band switch byte (BB) to setup this mode.
// @cb:			Value of the CB byte to setup this mode.
//
// Please notice that digital tuners like xc3028/xc4000/xc5000 don't use
// those ranges, as they're defined inside the driver. This is used by
// analog tuners that are compatible with the "Philips way" to setup the
// tuners. On those devices, the tuner set is done via 4 bytes:
//
// #) divider byte1 (DB1)
// #) divider byte 2 (DB2)
// #) Control byte (CB)
// #) band switch byte (BB)
//
// Some tuners also have an additional optional Auxiliary byte (AB).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tuner_range {
    pub limit: c_ushort,
    pub config: c_uchar,
    pub cb: c_uchar,
}

//
// struct tuner_params - Parameters to be used to setup the tuner. Those
// are used by drivers/media/tuners/tuner-types.c in
// order to specify the tuner properties. Most of
// the parameters are for tuners based on tda9887 IF-PLL
// multi-standard analog TV/Radio demodulator, with is
// very common on legacy analog tuners.
//
// @type:			Type of the tuner parameters, as defined at
// enum param_type. If the tuner supports multiple
// standards, an array should be used, with one
// row per different standard.
// @cb_first_if_lower_freq:	Many Philips-based tuners have a comment in
// their datasheet like
// "For channel selection involving band
// switching, and to ensure smooth tuning to the
// desired channel without causing unnecessary
// charge pump action, it is recommended to
// consider the difference between wanted channel
// frequency and the current channel frequency.
// Unnecessary charge pump action will result
// in very low tuning voltage which may drive the
// oscillator to extreme conditions".
// Set cb_first_if_lower_freq to 1, if this check
// is required for this tuner. I tested this for
// PAL by first setting the TV frequency to
// 203 MHz and then switching to 96.6 MHz FM
// radio. The result was static unless the
// control byte was sent first.
// @has_tda9887:		Set to 1 if this tuner uses a tda9887
// @port1_fm_high_sensitivity:	Many Philips tuners use tda9887 PORT1 to select
// the FM radio sensitivity. If this setting is 1,
// then set PORT1 to 1 to get proper FM reception.
// @port2_fm_high_sensitivity:	Some Philips tuners use tda9887 PORT2 to select
// the FM radio sensitivity. If this setting is 1,
// then set PORT2 to 1 to get proper FM reception.
// @fm_gain_normal:		Some Philips tuners use tda9887 cGainNormal to
// select the FM radio sensitivity. If this
// setting is 1, e register will use cGainNormal
// instead of cGainLow.
// @intercarrier_mode:		Most tuners with a tda9887 use QSS mode.
// Some (cheaper) tuners use Intercarrier mode.
// If this setting is 1, then the tuner needs to
// be set to intercarrier mode.
// @port1_active:		This setting sets the default value for PORT1.
// 0 means inactive, 1 means active. Note: the
// actual bit value written to the tda9887 is
// inverted. So a 0 here means a 1 in the B6 bit.
// @port2_active:		This setting sets the default value for PORT2.
// 0 means inactive, 1 means active. Note: the
// actual bit value written to the tda9887 is
// inverted. So a 0 here means a 1 in the B7 bit.
// @port1_invert_for_secam_lc:	Sometimes PORT1 is inverted when the SECAM-L'
// standard is selected. Set this bit to 1 if this
// is needed.
// @port2_invert_for_secam_lc:	Sometimes PORT2 is inverted when the SECAM-L'
// standard is selected. Set this bit to 1 if this
// is needed.
// @port1_set_for_fm_mono:	Some cards require PORT1 to be 1 for mono Radio
// FM and 0 for stereo.
// @default_pll_gating_18:	Select 18% (or according to datasheet 0%)
// L standard PLL gating, vs the driver default
// of 36%.
// @radio_if:			IF to use in radio mode.  Tuners with a
// separate radio IF filter seem to use 10.7,
// while those without use 33.3 for PAL/SECAM
// tuners and 41.3 for NTSC tuners.
// 0 = 10.7, 1 = 33.3, 2 = 41.3
// @default_top_low:		Default tda9887 TOP value in dB for the low
// band. Default is 0. Range: -16:+15
// @default_top_mid:		Default tda9887 TOP value in dB for the mid
// band. Default is 0. Range: -16:+15
// @default_top_high:		Default tda9887 TOP value in dB for the high
// band. Default is 0. Range: -16:+15
// @default_top_secam_low:	Default tda9887 TOP value in dB for SECAM-L/L'
// for the low band. Default is 0. Several tuners
// require a different TOP value for the
// SECAM-L/L' standards. Range: -16:+15
// @default_top_secam_mid:	Default tda9887 TOP value in dB for SECAM-L/L'
// for the mid band. Default is 0. Several tuners
// require a different TOP value for the
// SECAM-L/L' standards. Range: -16:+15
// @default_top_secam_high:	Default tda9887 TOP value in dB for SECAM-L/L'
// for the high band. Default is 0. Several tuners
// require a different TOP value for the
// SECAM-L/L' standards. Range: -16:+15
// @iffreq:			Intermediate frequency (IF) used by the tuner
// on digital mode.
// @count:			Size of the ranges array.
// @ranges:			Array with the frequency ranges supported by
// the tuner.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tuner_params {
    pub type: param_type,
    pub cb_first_if_lower_freq:1: c_uint,
    pub has_tda9887:1: c_uint,
    pub port1_fm_high_sensitivity:1: c_uint,
    pub port2_fm_high_sensitivity:1: c_uint,
    pub fm_gain_normal:1: c_uint,
    pub intercarrier_mode:1: c_uint,
    pub port1_active:1: c_uint,
    pub port2_active:1: c_uint,
    pub port1_invert_for_secam_lc:1: c_uint,
    pub port2_invert_for_secam_lc:1: c_uint,
    pub port1_set_for_fm_mono:1: c_uint,
    pub default_pll_gating_18:1: c_uint,
    pub radio_if:2: c_uint,
    pub default_top_low:5: signed int,
    pub default_top_mid:5: signed int,
    pub default_top_high:5: signed int,
    pub default_top_secam_low:5: signed int,
    pub default_top_secam_mid:5: signed int,
    pub default_top_secam_high:5: signed int,
    pub iffreq: u16,
    pub count: c_uint,
    pub ranges: *const tuner_range,
}

//
// struct tunertype - describes the known tuners.
//
// @name:	string with the tuner's name.
// @count:	size of &struct tuner_params array.
// @params:	pointer to &struct tuner_params array.
//
// @min:	minimal tuner frequency, in 62.5 kHz step.
// should be multiplied to 16 to convert to MHz.
// @max:	minimal tuner frequency, in 62.5 kHz step.
// Should be multiplied to 16 to convert to MHz.
// @stepsize:	frequency step, in Hz.
// @initdata:	optional byte sequence to initialize the tuner.
// @sleepdata:	optional byte sequence to power down the tuner.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tunertype {
    pub name: *mut c_char,
    pub count: c_uint,
    pub params: *const tuner_params,
    pub min: u16,
    pub max: u16,
    pub stepsize: u32,
    pub initdata: *mut u8,
    pub sleepdata: *mut u8,
}
