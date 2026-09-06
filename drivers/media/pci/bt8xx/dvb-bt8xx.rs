//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/bt8xx/dvb-bt8xx.h
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
// Bt8xx based DVB adapter driver
//
// Copyright (C) 2002,2003 Florian Schirmer <jolt@tuxbox.org>
// Copyright (C) 2002 Peter Hettkamp <peter.hettkamp@htp-tel.de>
// Copyright (C) 1999-2001 Ralph  Metzler & Marcus Metzler for convergence integrated media GmbH
// Copyright (C) 1998,1999 Christian Theiss <mistert@rz.fh-augsburg.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_bt8xx_card {
    pub lock: mutex,
    pub nfeeds: c_int,
    pub card_name: [c_char; 32],
    pub dvb_adapter: dvb_adapter,
    pub bt: *mut bt878,
    pub bttv_nr: c_uint,
    pub demux: dvb_demux,
    pub dmxdev: dmxdev,
    pub fe_hw: dmx_frontend,
    pub fe_mem: dmx_frontend,
    pub gpio_mode: u32,
    pub op_sync_orin: u32,
    pub irq_err_ignore: u32,
    pub i2c_adapter: *mut i2c_adapter,
    pub dvbnet: dvb_net,
    pub fe: *mut *mut dvb_frontend,
}
