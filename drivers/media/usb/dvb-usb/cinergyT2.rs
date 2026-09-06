//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/cinergyT2.h
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
// TerraTec Cinergy T2/qanu USB2 DVB-T adapter.
//
// Copyright (C) 2007 Tomi Orava (tomimo@ncircle.nullnet.fi)
//
// Based on the dvb-usb-framework code and the
// original Terratec Cinergy T2 driver by:
//
// Copyright (C) 2004 Daniel Mack <daniel@qanu.de> and
// Holger Waechtler <holger@qanu.de>
//
// Protocol Spec published on http://qanu.de/specs/terratec_cinergyT2.pdf
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cinergyt2_ep1_cmd {
    CINERGYT2_EP1_PID_TABLE_RESET		= 0x01,
    CINERGYT2_EP1_PID_SETUP			= 0x02,
    CINERGYT2_EP1_CONTROL_STREAM_TRANSFER	= 0x03,
    CINERGYT2_EP1_SET_TUNER_PARAMETERS	= 0x04,
    CINERGYT2_EP1_GET_TUNER_STATUS		= 0x05,
    CINERGYT2_EP1_START_SCAN		= 0x06,
    CINERGYT2_EP1_CONTINUE_SCAN		= 0x07,
    CINERGYT2_EP1_GET_RC_EVENTS		= 0x08,
    CINERGYT2_EP1_SLEEP_MODE		= 0x09,
    CINERGYT2_EP1_GET_FIRMWARE_VERSION	= 0x0A
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvbt_get_status_msg {
    pub freq: u32,
    pub bandwidth: u8,
    pub tps: u16,
    pub flags: u8,
    pub gain: __le16,
    pub snr: u8,
    pub viterbi_error_rate: __le32,
    pub rs_error_rate: u32,
    pub uncorrected_block_count: __le32,
    pub lock_bits: u8,
    pub prev_lock_bits: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvbt_set_parameters_msg {
    pub cmd: u8,
    pub freq: __le32,
    pub bandwidth: u8,
    pub tps: __le16,
    pub flags: u8,
    pub __attribute__((packed)): },
    pub d): *mut *mut extern struct dvb_frontend cinergyt2_fe_attach(struct dvb_usb_device,
