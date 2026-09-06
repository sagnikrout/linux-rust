//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv6110x.h
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

// Macro flag: #define __STV6110x_H
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv6110x_config {
    pub addr: u8,
    pub refclk: u32,
    pub /: *mut *mut u8 clk_div; / divisor value for the output clock,
    pub frontend: *mut dvb_frontend,
    pub i2c): *mut *mut *mut stv6110x_devctl (get_devctl)(i2c_client,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tuner_mode {
    TUNER_SLEEP = 1,
    TUNER_WAKE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tuner_status {
    TUNER_PHASELOCKED = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv6110x_devctl {
    pub fe): *mut *mut int (tuner_init) (struct dvb_frontend,
    pub fe): *mut *mut int (tuner_sleep) (struct dvb_frontend,
    pub mode): *mut *mut *mut int (tuner_set_mode) (struct dvb_frontend fe, enum tuner_mode,
    pub frequency): *mut *mut *mut int (tuner_set_frequency) (struct dvb_frontend fe, u32,
    pub frequency): *mut *mut *mut int (tuner_get_frequency) (struct dvb_frontend fe, u32,
    pub bandwidth): *mut *mut *mut int (tuner_set_bandwidth) (struct dvb_frontend fe, u32,
    pub bandwidth): *mut *mut *mut int (tuner_get_bandwidth) (struct dvb_frontend fe, u32,
    pub gain): *mut *mut *mut int (tuner_set_bbgain) (struct dvb_frontend fe, u32,
    pub gain): *mut *mut *mut int (tuner_get_bbgain) (struct dvb_frontend fe, u32,
    pub refclk): *mut *mut *mut int (tuner_set_refclk) (struct dvb_frontend fe, u32,
    pub status): *mut *mut *mut int (tuner_get_status) (struct dvb_frontend fe, u32,
}

