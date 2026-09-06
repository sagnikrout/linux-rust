//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/drxk.h
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
// struct drxk_config - Configure the initial parameters for DRX-K
//
// @adr:		I2C address of the DRX-K
// @parallel_ts:	True means that the device uses parallel TS,
// Serial otherwise.
// @dynamic_clk:	True means that the clock will be dynamically
// adjusted. Static clock otherwise.
// @enable_merr_cfg:	Enable SIO_PDR_PERR_CFG/SIO_PDR_MVAL_CFG.
// @single_master:	Device is on the single master mode
// @no_i2c_bridge:	Don't switch the I2C bridge to talk with tuner
// @antenna_gpio:	GPIO bit used to control the antenna
// @antenna_dvbt:	GPIO bit for changing antenna to DVB-C. A value of 1
// means that 1=DVBC, 0 = DVBT. Zero means the opposite.
// @mpeg_out_clk_strength: DRXK Mpeg output clock drive strength.
// @chunk_size:		maximum size for I2C messages
// @microcode_name:	Name of the firmware file with the microcode
// @qam_demod_parameter_count:	The number of parameters used for the command
// to set the demodulator parameters. All
// firmwares are using the 2-parameter command.
// An exception is the ``drxk_a3.mc`` firmware,
// which uses the 4-parameter command.
// A value of 0 (default) or lower indicates that
// the correct number of parameters will be
// automatically detected.
//
// On the ``*_gpio`` vars, bit 0 is UIO-1, bit 1 is UIO-2 and bit 2 is
// UIO-3.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxk_config {
    pub adr: u8,
    pub single_master: bool,
    pub no_i2c_bridge: bool,
    pub parallel_ts: bool,
    pub dynamic_clk: bool,
    pub enable_merr_cfg: bool,
    pub antenna_dvbt: bool,
    pub antenna_gpio: u16,
    pub mpeg_out_clk_strength: u8,
    pub chunk_size: c_int,
    pub microcode_name: *const c_char,
    pub qam_demod_parameter_count: c_int,
}

//
// drxk_attach - Attach a drxk demod
//
// @config: pointer to &struct drxk_config with demod configuration.
// @i2c: i2c adapter to use.
//
// return: FE pointer on success, NULL on failure.
//

