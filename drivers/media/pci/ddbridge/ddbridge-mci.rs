//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ddbridge/ddbridge-mci.h
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
// ddbridge-mci.h: Digital Devices micro code interface
//
// Copyright (C) 2017-2018 Digital Devices GmbH
// Marcus Metzler <mocm@metzlerbros.de>
// Ralph Metzler <rjkm@metzlerbros.de>
//
pub const MCI_DEMOD_MAX: c_int = 8;
pub const MCI_TUNER_MAX: c_int = 4;

//
// IQMode is only available on MaxSX8 on a single tuner
//
// IQ_MODE_SAMPLES
// sampling rate is 1550/24 MHz (64.583 MHz)
// channel agc is frozen, to allow stitching the FFT results together
//
// IQ_MODE_VTM
// sampling rate is the supplied symbolrate
// channel agc is active
//
// in both cases down sampling is done with a RRC Filter (currently fixed to
// alpha = 0.05) which causes some (ca 5%) aliasing at the edges from
// outside the spectrum
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mci_command {
    pub command_word: u32,
    pub command: u8,
    pub tuner: u8,
    pub demod: u8,
    pub output: u8,
}

//
// Bit 0: DVB-S Enabled
// Bit 1: DVB-S2 Enabled
// Bit 7: InputStreamID
//
// Bit 0: QPSK,
// Bit 1: 8PSK/8APSK
// Bit 2: 16APSK
// Bit 3: 32APSK
// Bit 4: 64APSK
// Bit 5: 128APSK
// Bit 6: 256APSK
//
// Bit 0: 0=VTM/1=SCAN
// Bit 1: Set Gain
//
// Bit 1:0 = STVVGLNA Gain.
// 0 = AGC, 1 = 0dB, 2 = Minimum, 3 = Maximum
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mci_result {
    pub status_word: u32,
    pub status: u8,
    pub mode: u8,
    pub time: u16,
}

// 1 = DVB-S, 2 = DVB-S2X
// puncture rate for DVB-S
// 2-0: rolloff
// actual frequency in Hz
// actual symbolrate in Hz
// channel power in dBm x 100
// band power in dBm x 100
//
// SNR in dB x 100
// Note: negative values are valid in DVB-S2
//
// Counter for packet errors
// (set to 0 on start command)
//
// Bit error rate: PreRS in DVB-S, PreBCH in DVB-S2X
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mci_base {
    pub mci_list: list_head,
    pub key: *mut c_void,
    pub link: *mut ddb_link,
    pub completion: completion,
    pub dev: *mut device,
    pub /: *mut *mut mutex tuner_lock; / concurrent tuner access lock,
    pub /: *mut *mut mutex mci_lock; / concurrent MCI access lock,
    pub count: c_int,
    pub type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mci {
    pub base: *mut mci_base,
    pub fe: dvb_frontend,
    pub nr: c_int,
    pub demod: c_int,
    pub tuner: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mci_cfg {
    pub type: c_int,
    pub fe_ops: *mut dvb_frontend_ops,
    pub base_size: u32,
    pub state_size: u32,
    pub mci): *mut *mut int (init)(struct mci,
    pub mci_base): *mut *mut int (base_init)(struct mci_base,
    pub input): *mut *mut *mut int (set_input)(struct dvb_frontend fe, int,
}

// defined in ddbridge-sx8.c
extern "C" {
    pub fn ddb_mci_config(state: *mut mci, config: u32) -> c_int;
}
// ddb_mci_attach(struct ddb_input *input, struct mci_cfg *cfg, int nr,
