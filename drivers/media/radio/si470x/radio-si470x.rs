//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/radio/si470x/radio-si470x.h
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
// drivers/media/radio/si470x/radio-si470x.h
//
// Driver for radios with Silicon Labs Si470x FM Radio Receivers
//
// Copyright (c) 2009 Tobias Lorenz <tobias.lorenz@gmx.net>
//
// driver definitions

// kernel includes

//
// Register Definitions
//

pub const DEVICEID_PN: c_uint = 0xf000	/* bits 15..12: Part Number */;
pub const DEVICEID_MFGID: c_uint = 0x0fff	/* bits 11..00: Manufacturer ID */;

pub const SI_CHIPID_REV: c_uint = 0xfc00	/* bits 15..10: Chip Version */;
pub const SI_CHIPID_DEV: c_uint = 0x0200	/* bits 09..09: Device */;
pub const SI_CHIPID_FIRMWARE: c_uint = 0x01ff	/* bits 08..00: Firmware Version */;

pub const POWERCFG_DSMUTE: c_uint = 0x8000	/* bits 15..15: Softmute Disable */;
pub const POWERCFG_DMUTE: c_uint = 0x4000	/* bits 14..14: Mute Disable */;
pub const POWERCFG_MONO: c_uint = 0x2000	/* bits 13..13: Mono Select */;
pub const POWERCFG_RDSM: c_uint = 0x0800	/* bits 11..11: RDS Mode (Si4701 only) */;
pub const POWERCFG_SKMODE: c_uint = 0x0400	/* bits 10..10: Seek Mode */;
pub const POWERCFG_SEEKUP: c_uint = 0x0200	/* bits 09..09: Seek Direction */;
pub const POWERCFG_SEEK: c_uint = 0x0100	/* bits 08..08: Seek */;
pub const POWERCFG_DISABLE: c_uint = 0x0040	/* bits 06..06: Powerup Disable */;
pub const POWERCFG_ENABLE: c_uint = 0x0001	/* bits 00..00: Powerup Enable */;

pub const CHANNEL_TUNE: c_uint = 0x8000	/* bits 15..15: Tune */;
pub const CHANNEL_CHAN: c_uint = 0x03ff	/* bits 09..00: Channel Select */;

pub const SYSCONFIG1_RDSIEN: c_uint = 0x8000	/* bits 15..15: RDS Interrupt Enable (Si4701 only) */;
pub const SYSCONFIG1_STCIEN: c_uint = 0x4000	/* bits 14..14: Seek/Tune Complete Interrupt Enable */;
pub const SYSCONFIG1_RDS: c_uint = 0x1000	/* bits 12..12: RDS Enable (Si4701 only) */;
pub const SYSCONFIG1_DE: c_uint = 0x0800	/* bits 11..11: De-emphasis (0=75us 1=50us) */;
pub const SYSCONFIG1_AGCD: c_uint = 0x0400	/* bits 10..10: AGC Disable */;
pub const SYSCONFIG1_BLNDADJ: c_uint = 0x00c0	/* bits 07..06: Stereo/Mono Blend Level Adjustment */;
pub const SYSCONFIG1_GPIO3: c_uint = 0x0030	/* bits 05..04: General Purpose I/O 3 */;
pub const SYSCONFIG1_GPIO2: c_uint = 0x000c	/* bits 03..02: General Purpose I/O 2 */;
pub const SYSCONFIG1_GPIO2_DIS: c_uint = 0x0000	/* Disable GPIO 2 interrupt */;
pub const SYSCONFIG1_GPIO2_INT: c_uint = 0x0004	/* Enable STC/RDS interrupt */;
pub const SYSCONFIG1_GPIO1: c_uint = 0x0003	/* bits 01..00: General Purpose I/O 1 */;

pub const SYSCONFIG2_SEEKTH: c_uint = 0xff00	/* bits 15..08: RSSI Seek Threshold */;
pub const SYSCONFIG2_BAND: c_uint = 0x00c0	/* bits 07..06: Band Select */;
pub const SYSCONFIG2_SPACE: c_uint = 0x0030	/* bits 05..04: Channel Spacing */;
pub const SYSCONFIG2_VOLUME: c_uint = 0x000f	/* bits 03..00: Volume */;

pub const SYSCONFIG3_SMUTER: c_uint = 0xc000	/* bits 15..14: Softmute Attack/Recover Rate */;
pub const SYSCONFIG3_SMUTEA: c_uint = 0x3000	/* bits 13..12: Softmute Attenuation */;
pub const SYSCONFIG3_SKSNR: c_uint = 0x00f0	/* bits 07..04: Seek SNR Threshold */;
pub const SYSCONFIG3_SKCNT: c_uint = 0x000f	/* bits 03..00: Seek FM Impulse Detection Threshold */;

pub const TEST1_AHIZEN: c_uint = 0x4000	/* bits 14..14: Audio High-Z Enable */;

// TEST2 only contains reserved bits

// BOOTCONFIG only contains reserved bits

pub const STATUSRSSI_RDSR: c_uint = 0x8000	/* bits 15..15: RDS Ready (Si4701 only) */;
pub const STATUSRSSI_STC: c_uint = 0x4000	/* bits 14..14: Seek/Tune Complete */;
pub const STATUSRSSI_SF: c_uint = 0x2000	/* bits 13..13: Seek Fail/Band Limit */;
pub const STATUSRSSI_AFCRL: c_uint = 0x1000	/* bits 12..12: AFC Rail */;
pub const STATUSRSSI_RDSS: c_uint = 0x0800	/* bits 11..11: RDS Synchronized (Si4701 only) */;
pub const STATUSRSSI_BLERA: c_uint = 0x0600	/* bits 10..09: RDS Block A Errors (Si4701 only) */;
pub const STATUSRSSI_ST: c_uint = 0x0100	/* bits 08..08: Stereo Indicator */;
pub const STATUSRSSI_RSSI: c_uint = 0x00ff	/* bits 07..00: RSSI (Received Signal Strength Indicator) */;

pub const READCHAN_BLERB: c_uint = 0xc000	/* bits 15..14: RDS Block D Errors (Si4701 only) */;
pub const READCHAN_BLERC: c_uint = 0x3000	/* bits 13..12: RDS Block C Errors (Si4701 only) */;
pub const READCHAN_BLERD: c_uint = 0x0c00	/* bits 11..10: RDS Block B Errors (Si4701 only) */;
pub const READCHAN_READCHAN: c_uint = 0x03ff	/* bits 09..00: Read Channel */;

pub const RDSA_RDSA: c_uint = 0xffff	/* bits 15..00: RDS Block A Data (Si4701 only) */;

pub const RDSB_RDSB: c_uint = 0xffff	/* bits 15..00: RDS Block B Data (Si4701 only) */;

pub const RDSC_RDSC: c_uint = 0xffff	/* bits 15..00: RDS Block C Data (Si4701 only) */;

pub const RDSD_RDSD: c_uint = 0xffff	/* bits 15..00: RDS Block D Data (Si4701 only) */;
//
// General Driver Definitions
//
// si470x_device - private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si470x_device {
    pub v4l2_dev: v4l2_device,
    pub videodev: video_device,
    pub hdl: v4l2_ctrl_handler,
    pub band: c_int,
// Silabs internal registers (0..15)
    pub registers: [c_ushort; RADIO_REGISTER_NUM],
// RDS receive buffer
    pub read_queue: wait_queue_head_t,
    pub /: *mut *mut mutex lock; / buffer locking,
    pub /: *mut *mut *mut unsigned char buffer; / size is always multiple of three,
    pub buf_size: c_uint,
    pub rd_index: c_uint,
    pub wr_index: c_uint,
    pub completion: completion,
    pub /: *mut *mut bool status_rssi_auto_update; / Does RSSI get updated automatic?,
// si470x ops
    pub regnr): *mut *mut *mut int (get_register)(struct si470x_device radio, int,
    pub regnr): *mut *mut *mut int (set_register)(struct si470x_device radio, int,
    pub file): *mut *mut int (fops_open)(struct file,
    pub file): *mut *mut int (fops_release)(struct file,
    pub capability): *mut v4l2_capability,

// reference to USB and video device
    pub usbdev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub usb_buf: *mut c_char,
// Interrupt endpoint handling
    pub int_in_buffer: *mut c_char,
    pub int_in_endpoint: *mut usb_endpoint_descriptor,
    pub int_in_urb: *mut urb,
    pub int_in_running: c_int,
// scratch page
    pub software_version: c_uchar,
    pub hardware_version: c_uchar,

    pub client: *mut i2c_client,
    pub gpio_reset: *mut gpio_desc,

}

//
// Firmware Versions
//
pub const RADIO_FW_VERSION: c_int = 12;
//
// Frequency Multiplicator
//
// The frequency is set in units of 62.5 Hz when using V4L2_TUNER_CAP_LOW,
// 62.5 kHz otherwise.
// The tuner is able to have a channel spacing of 50, 100 or 200 kHz.
// tuner->capability is therefore set to V4L2_TUNER_CAP_LOW
// The FREQ_MUL is then: 1 MHz / 62.5 Hz = 16000
//

//
// Common Functions
//
extern "C" {
    pub fn si470x_disconnect_check(radio: *mut si470x_device) -> c_int;
}
extern "C" {
    pub fn si470x_set_freq(radio: *mut si470x_device, freq: c_uint) -> c_int;
}
extern "C" {
    pub fn si470x_start(radio: *mut si470x_device) -> c_int;
}
extern "C" {
    pub fn si470x_stop(radio: *mut si470x_device) -> c_int;
}
