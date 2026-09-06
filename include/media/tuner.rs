//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/tuner.h
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
// tuner.h - definition for different tuners
//
// Copyright (C) 1997 Markus Schroeder (schroedm@uni-duesseldorf.de)
// minor modifications by Ralph Metzler (rjkm@thp.uni-koeln.de)
//

pub const TUNER_PHILIPS_PAL_I: c_int = 1;
pub const TUNER_PHILIPS_NTSC: c_int = 2;

pub const TUNER_ABSENT: c_int = 4;
pub const TUNER_PHILIPS_PAL: c_int = 5;

pub const TUNER_ALPS_TSBH1_NTSC: c_int = 9;
pub const TUNER_ALPS_TSBE1_PAL: c_int = 10;
pub const TUNER_ALPS_TSBB5_PAL_I: c_int = 11;
pub const TUNER_ALPS_TSBE5_PAL: c_int = 12;
pub const TUNER_ALPS_TSBC5_PAL: c_int = 13;

pub const TUNER_ALPS_TSHC6_NTSC: c_int = 15;

pub const TUNER_PHILIPS_NTSC_M: c_int = 17;

pub const TUNER_PHILIPS_PAL_DK: c_int = 23;

pub const TUNER_LG_PAL_I_FM: c_int = 25;
pub const TUNER_LG_PAL_I: c_int = 26;
pub const TUNER_LG_NTSC_FM: c_int = 27;
pub const TUNER_LG_PAL_FM: c_int = 28;
pub const TUNER_LG_PAL: c_int = 29;

pub const TUNER_SHARP_2U5JF5540_NTSC: c_int = 31;
pub const TUNER_Samsung_PAL_TCPM9091PD27: c_int = 32;
pub const TUNER_MT2032: c_int = 33;

pub const TUNER_LG_PAL_NEW_TAPC: c_int = 37;
pub const TUNER_PHILIPS_FM1216ME_MK3: c_int = 38;
pub const TUNER_LG_NTSC_NEW_TAPC: c_int = 39;
pub const TUNER_HITACHI_NTSC: c_int = 40;
pub const TUNER_PHILIPS_PAL_MK: c_int = 41;
pub const TUNER_PHILIPS_FCV1236D: c_int = 42;
pub const TUNER_PHILIPS_FM1236_MK3: c_int = 43;

//
// Microtune merged with Temic 12/31/1999 partially financed by Alps.
// these may be similar to Temic
//
pub const TUNER_MICROTUNE_4049FM5: c_int = 45;
pub const TUNER_PANASONIC_VP27: c_int = 46;
pub const TUNER_LG_NTSC_TAPE: c_int = 47;
pub const TUNER_TNF_8831BGFF: c_int = 48;

pub const TUNER_TCL_2002N: c_int = 50;
pub const TUNER_PHILIPS_FM1256_IH3: c_int = 51;
pub const TUNER_THOMSON_DTT7610: c_int = 52;
pub const TUNER_PHILIPS_FQ1286: c_int = 53;
pub const TUNER_PHILIPS_TDA8290: c_int = 54;

pub const TUNER_YMEC_TVF_8531MF: c_int = 58;

pub const TUNER_TENA_9533_DI: c_int = 61;

pub const TUNER_PHILIPS_FMD1216ME_MK3: c_int = 63;

pub const TUNER_LG_TALN: c_int = 66;
pub const TUNER_PHILIPS_TD1316: c_int = 67;

pub const TUNER_XC2028: c_int = 71;

pub const TUNER_PHILIPS_FMD1216MEX_MK3: c_int = 78;
pub const TUNER_PHILIPS_FM1216MK5: c_int = 79;

pub const TUNER_PARTSNIC_PTI_5NF05: c_int = 81;
pub const TUNER_PHILIPS_CU1216L: c_int = 82;
pub const TUNER_NXP_TDA18271: c_int = 83;
pub const TUNER_SONY_BTF_PXN01Z: c_int = 84;

pub const TUNER_TENA_TNF_5337: c_int = 86;

pub const TUNER_SI2157: c_int = 92;
pub const TUNER_TENA_TNF_931D_DFDR1: c_int = 93;
// tv card specific

// Tuner takeover point adjustment, in dB, -16 <= top <= 15

// config options

//
// enum tuner_mode      - Mode of the tuner
//
// @T_RADIO:        Tuner core will work in radio mode
// @T_ANALOG_TV:    Tuner core will work in analog TV mode
//
// Older boards only had a single tuner device, but some devices have a
// separate tuner for radio. In any case, the tuner-core needs to know if
// the tuner chip(s) will be used in radio mode or analog TV mode, as, on
// radio mode, frequencies are specified on a different range than on TV
// mode. This enum is used by the tuner core in order to work with the
// proper tuner range and eventually use a different tuner chip while in
// radio mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tuner_mode {
    T_RADIO		= 1 << V4L2_TUNER_RADIO,
    T_ANALOG_TV     = 1 << V4L2_TUNER_ANALOG_TV,
// Don't map V4L2_TUNER_DIGITAL_TV, as tuner-core won't use it
}

//
// struct tuner_setup   - setup the tuner chipsets
//
// @addr:		I2C address used to control the tuner device/chipset
// @type:		Type of the tuner, as defined at the TUNER_* macros.
// Each different tuner model should have an unique
// identifier.
// @mode_mask:		Mask with the allowed tuner modes: V4L2_TUNER_RADIO,
// V4L2_TUNER_ANALOG_TV and/or V4L2_TUNER_DIGITAL_TV,
// describing if the tuner should be used to support
// Radio, analog TV and/or digital TV.
// @config:		Used to send tuner-specific configuration for complex
// tuners that require extra parameters to be set.
// Only a very few tuners require it and its usage on
// newer tuners should be avoided.
// @tuner_callback:	Some tuners require to call back the bridge driver,
// in order to do some tasks like rising a GPIO at the
// bridge chipset, in order to do things like resetting
// the device.
//
// Older boards only had a single tuner device. Nowadays multiple tuner
// devices may be present on a single board. Using TUNER_SET_TYPE_ADDR
// to pass the tuner_setup structure it is possible to setup each tuner
// device in turn.
//
// Since multiple devices may be present it is no longer sufficient to
// send a command to a single i2c device. Instead you should broadcast
// the command to all i2c devices.
//
// By setting the mode_mask correctly you can select which commands are
// accepted by a specific tuner device. For example, set mode_mask to
// T_RADIO if the device is a radio-only tuner. That specific tuner will
// only accept commands when the tuner is in radio mode and ignore them
// when the tuner is set to TV mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tuner_setup {
    pub addr: c_ushort,
    pub type: c_uint,
    pub mode_mask: c_uint,
    pub config: *mut c_void,
    pub arg): *mut *mut *mut int (tuner_callback)(void dev, int component, int cmd, int,
}

