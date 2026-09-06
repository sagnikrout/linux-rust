//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/drx39xyj/drx_dap_fasi.h
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


//
// Redistributions of source code must retain the above copyright notice,
// Redistributions in binary form must reproduce the above copyright notice,
// Neither the name of Trident Microsystems nor Hauppauge Computer Works
//
// FILENAME: $Id: drx_dap_fasi.h,v 1.5 2009/07/07 14:21:40 justin Exp $
//
// DESCRIPTION:
// Part of DRX driver.
// Data access protocol: Fast Access Sequential Interface (fasi)
// Fast access, because of short addressing format (16 instead of 32 bits addr)
// Sequential, because of I2C.
//
// USAGE:
// Include.
//
// NOTES:
//
// -------- compilation control switches --------------------------------------
// -------- Required includes -------------------------------------------------

// -------- Defines, configuring the API --------------------------------------
//
// Allowed address formats
//
// Comments about short/long addressing format:
//
// The DAP FASI offers long address format (4 bytes) and short address format
// (2 bytes). The DAP can operate in 3 modes:
// (1) only short
// (2) only long
// (3) both long and short but short preferred and long only when necessary
//
// These modes must be selected compile time via compile switches.
// Compile switch settings for the different modes:
// (1) DRXDAPFASI_LONG_ADDR_ALLOWED=0, DRXDAPFASI_SHORT_ADDR_ALLOWED=1
// (2) DRXDAPFASI_LONG_ADDR_ALLOWED=1, DRXDAPFASI_SHORT_ADDR_ALLOWED=0
// (3) DRXDAPFASI_LONG_ADDR_ALLOWED=1, DRXDAPFASI_SHORT_ADDR_ALLOWED=1
//
// The default setting will be (3) both long and short.
// The default setting will need no compile switches.
// The default setting must be overridden if compile switches are already
// defined.
//
// set default

pub const DRXDAPFASI_LONG_ADDR_ALLOWED: c_int = 1;

// set default

pub const DRXDAPFASI_SHORT_ADDR_ALLOWED: c_int = 1;

// check

// ;				/* illegal statement to force compiler error

//
// Single/master multi master setting
//
// Comments about SINGLE MASTER/MULTI MASTER  modes:
//
// Consider the two sides:1) the master and 2)the slave.
//
// Master:
// Single/multimaster operation set via DRXDAP_SINGLE_MASTER compile switch
// + single master mode means no use of repeated starts
// + multi master mode means use of repeated starts
// Default is single master.
// Default can be overridden by setting the compile switch DRXDAP_SINGLE_MASTER.
//
// Slave:
// Single/multi master selected via the flags in the FASI protocol.
// + single master means remember memory address between i2c packets
// + multimaster means flush memory address between i2c packets
// Default is single master, DAP FASI changes multi-master setting silently
// into single master setting. This cannot be overridden.
//
// set default

pub const DRXDAP_SINGLE_MASTER: c_int = 0;

//
// Chunk/mode checking
//
// Comments about DRXDAP_MAX_WCHUNKSIZE in single or multi master mode and
// in combination with short and long addressing format. All text below
// assumes long addressing format. The table also includes information
// for short ADDRessing format.
//
// In single master mode, data can be written by sending the register address
// first, then two or four bytes of data in the next packet.
// Because the device address plus a register address equals five bytes,
// the minimum chunk size must be five.
// If ten-bit I2C device addresses are used, the minimum chunk size must be six,
// because the I2C device address will then occupy two bytes when writing.
//
// Data in single master mode is transferred as follows:
// <S> <devW>  a0  a1  a2  a3  <P>
// <S> <devW>  d0  d1 [d2  d3] <P>
// ..
// or
// ..
// <S> <devW>  a0  a1  a2  a3  <P>
// <S> <devR> --- <P>
//
// In multi-master mode, the data must immediately follow the address (an I2C
// stop resets the internal address), and hence the minimum chunk size is
// 1 <I2C address> + 4 (register address) + 2 (data to send) = 7 bytes (8 if
// 10-bit I2C device addresses are used).
//
// The 7-bit or 10-bit i2c address parameters is a runtime parameter.
// The other parameters can be limited via compile time switches.
//
// -------------------------------------------------------------------------------
//
// Minimum chunk size table (in bytes):
//
// +----------------+----------------+
// | 7b i2c addr    | 10b i2c addr   |
// +----------------+----------------+
// | single | multi | single | multi |
// ------+--------+-------+--------+-------+
// short | 3      | 5     | 4      | 6     |
// long  | 5      | 7     | 6      | 8     |
// ------+--------+-------+--------+-------+
//
// set default

pub const DRXDAP_MAX_WCHUNKSIZE: c_int = 254;

// check

pub const DRXDAP_MAX_WCHUNKSIZE_MIN: c_int = 3;

pub const DRXDAP_MAX_WCHUNKSIZE_MIN: c_int = 5;

pub const DRXDAP_MAX_WCHUNKSIZE_MIN: c_int = 5;

pub const DRXDAP_MAX_WCHUNKSIZE_MIN: c_int = 7;

// ;				/* illegal statement to force compiler error

// ;				/* illegal statement to force compiler error

// ;				/* illegal statement to force compiler error

// ;				/* illegal statement to force compiler error

// set default

pub const DRXDAP_MAX_RCHUNKSIZE: c_int = 254;

// check

// ;				/* illegal statement to force compiler error

// check

// ;				/* illegal statement to force compiler error

// -------- Public API functions ----------------------------------------------
pub const DRXDAP_FASI_RMW: c_uint = 0x10000000;
pub const DRXDAP_FASI_BROADCAST: c_uint = 0x20000000;
pub const DRXDAP_FASI_CLEARCRC: c_uint = 0x80000000;
pub const DRXDAP_FASI_SINGLE_MASTER: c_uint = 0xC0000000;
pub const DRXDAP_FASI_MULTI_MASTER: c_uint = 0x40000000;
pub const DRXDAP_FASI_SMM_SWITCH: c_uint = 0x40000000	/* single/multi master switch */;
pub const DRXDAP_FASI_MODEFLAGS: c_uint = 0xC0000000;
pub const DRXDAP_FASI_FLAGS: c_uint = 0xF0000000;

