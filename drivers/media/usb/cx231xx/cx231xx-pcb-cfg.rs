//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/cx231xx/cx231xx-pcb-cfg.h
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

//
// Class Information
//
pub const CLASS_DEFAULT: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VENDOR_REQUEST_TYPE {
// Set/Get I2C
    VRT_SET_I2C0 = 0x0,
    VRT_SET_I2C1 = 0x1,
    VRT_SET_I2C2 = 0x2,
    VRT_GET_I2C0 = 0x4,
    VRT_GET_I2C1 = 0x5,
    VRT_GET_I2C2 = 0x6,

// Set/Get GPIO
    VRT_SET_GPIO = 0x8,
    VRT_GET_GPIO = 0x9,

// Set/Get GPIE
    VRT_SET_GPIE = 0xA,
    VRT_GET_GPIE = 0xB,

// Set/Get Register Control/Status
    VRT_SET_REGISTER = 0xC,
    VRT_GET_REGISTER = 0xD,

// Get Extended Compat ID Descriptor
    VRT_GET_EXTCID_DESC = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BYTE_ENABLE_MASK {
    ENABLE_ONE_BYTE = 0x1,
    ENABLE_TWE_BYTE = 0x3,
    ENABLE_THREE_BYTE = 0x7,
    ENABLE_FOUR_BYTE = 0xF,
}

pub const SPEED_MASK: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum USB_SPEED {
    FULL_SPEED = 0x0,	/* 0: full speed */
    HIGH_SPEED = 0x1	/* 1: high speed */
}

pub const TS_MASK: c_uint = 0x6;
pub const NO_TS_PORT: c_uint = 0x0	/* 2'b00: Neither port used. PCB not a Hybrid,;
pub const TS1_PORT: c_uint = 0x4	/* 2'b10: TS1 Input (Hybrid mode :;
pub const TS1_TS2_PORT: c_uint = 0x6	/* 2'b11: TS1 & TS2 Inputs;
pub const TS1_EXT_CLOCK: c_uint = 0x6	/* 2'b11: TS1 & TS2 as selector;
pub const TS1VIP_TS2_PORT: c_uint = 0x2	/* 2'b01: TS1 used as 656/VIP Output,;
pub const EAVP_MASK: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EAV_PRESENT {
    NO_EXTERNAL_AV = 0x0,	/* 0: No External A/V inputs
    (no need for i2s block),
    Analog Tuner must be present */
    EXTERNAL_AV = 0x8	/* 1: External A/V inputs
    present (requires i2s blk) */
}

pub const ATM_MASK: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AT_MODE {
    DIF_TUNER = 0x30,	/* 2'b11: IF Tuner (requires use of DIF) */
    BASEBAND_SOUND = 0x20,	/* 2'b10: Baseband Composite &
    Sound-IF Signals present */
    NO_TUNER = 0x10		/* 2'b0x: No Analog Tuner present */
}

pub const PWR_SEL_MASK: c_uint = 0x40;
pub const SELF_POWER: c_uint = 0x0	/* 0: self power */;
pub const BUS_POWER: c_uint = 0x40	/* 1: bus power */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum USB_POWE_TYPE {
    USB_SELF_POWER = 0,
    USB_BUS_POWER
}

pub const BO_0_MASK: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AVDEC_STATUS {
    AVDEC_DISABLE = 0x0,	/* 0: A/V Decoder Disabled */
    AVDEC_ENABLE = 0x80	/* 1: A/V Decoder Enabled */
}

pub const BO_1_MASK: c_uint = 0x100;
pub const BUSPOWER_MASK: c_uint = 0xC4	/* for Polaris spec 0.8 */;
pub const SELFPOWER_MASK: c_uint = 0x86;
//
pub const NOT_DECIDE_YET: c_uint = 0xFE;
pub const NOT_SUPPORTED: c_uint = 0xFF;
//
// for mod field use
//
pub const MOD_DIGITAL: c_uint = 0x1;
pub const MOD_ANALOG: c_uint = 0x2;
pub const MOD_DIF: c_uint = 0x4;
pub const MOD_EXTERNAL: c_uint = 0x8;
pub const CAP_ALL_MOD: c_uint = 0x0f;
//
// source define
//
pub const SOURCE_DIGITAL: c_uint = 0x1;
pub const SOURCE_ANALOG: c_uint = 0x2;
pub const SOURCE_DIF: c_uint = 0x4;
pub const SOURCE_EXTERNAL: c_uint = 0x8;
pub const SOURCE_TS_BDA: c_uint = 0x10;
pub const SOURCE_TS_ENCODE: c_uint = 0x20;
pub const SOURCE_TS_EXTERNAL: c_uint = 0x40;
//
// interface information define
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct INTERFACE_INFO {
    pub interrupt_index: u8,
    pub ts1_index: u8,
    pub ts2_index: u8,
    pub audio_index: u8,
    pub video_index: u8,
    pub /: *mut *mut u8 vanc_index; / VBI,
    pub /: *mut *mut u8 hanc_index; / Sliced CC,
    pub ir_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum INDEX_INTERFACE_INFO {
    INDEX_INTERRUPT = 0x0,
    INDEX_TS1,
    INDEX_TS2,
    INDEX_AUDIO,
    INDEX_VIDEO,
    INDEX_VANC,
    INDEX_HANC,
    INDEX_IR,
}

//
// configuration information define
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CONFIG_INFO {
    pub config_index: u8,
    pub interface_info: INTERFACE_INFO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcb_config {
    pub index: u8,
    pub power,: *mut *mut u8 type; / bus power or self,
    pub /: *mut *mut u8 speed; / usb speed, 2.0--1, 1.1--0,
    pub /: *mut *mut u8 mode; / digital , anlog, dif or external A/V,
    pub /: *mut *mut u32 ts1_source; / three source -- BDA,External,encode,
    pub ts2_source: u32,
    pub analog_source: u32,
    pub /: *mut *mut u8 digital_index; / bus-power used,
    pub /: *mut *mut u8 analog_index; / bus-power used,
    pub /: *mut *mut u8 dif_index; / bus-power used,
    pub /: *mut *mut u8 external_index; / bus-power used,
    pub 0,1,2,: *mut *mut u8 config_num; / current config num,,
    pub hs_config_info: [CONFIG_INFO; 3],
    pub fs_config_info: [CONFIG_INFO; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum INDEX_PCB_CONFIG {
    INDEX_SELFPOWER_DIGITAL_ONLY = 0x0,
    INDEX_SELFPOWER_DUAL_DIGITAL,
    INDEX_SELFPOWER_ANALOG_ONLY,
    INDEX_SELFPOWER_DUAL,
    INDEX_SELFPOWER_TRIPLE,
    INDEX_SELFPOWER_COMPRESSOR,
    INDEX_BUSPOWER_DIGITAL_ONLY,
    INDEX_BUSPOWER_ANALOG_ONLY,
    INDEX_BUSPOWER_DIF_ONLY,
    INDEX_BUSPOWER_EXTERNAL_ONLY,
    INDEX_BUSPOWER_EXTERNAL_ANALOG,
    INDEX_BUSPOWER_EXTERNAL_DIF,
    INDEX_BUSPOWER_EXTERNAL_DIGITAL,
    INDEX_BUSPOWER_DIGITAL_ANALOG,
    INDEX_BUSPOWER_DIGITAL_DIF,
    INDEX_BUSPOWER_DIGITAL_ANALOG_EXTERNAL,
    INDEX_BUSPOWER_DIGITAL_DIF_EXTERNAL,
}

//
extern "C" {
    pub fn initialize_cx231xx(p_dev: *mut cx231xx) -> c_int;
}
