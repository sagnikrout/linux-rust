//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/m5602/m5602_bridge.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// USB Driver for ALi m5602 based webcams
//
// Copyright (C) 2008 Erik Andrén
// Copyright (C) 2007 Ilyes Gouta. Based on the m5603x Linux Driver Project.
// Copyright (C) 2005 m5603x Linux Driver Project <m5602@x3ng.com.br>
//
// Portions of code to USB interface and ALi driver software,
// Copyright (c) 2006 Willem Duinker
// v4l2 interface modeled after the V4L2 driver
// for SN9C10x PC Camera Controllers
//

//
pub const M5602_XB_SENSOR_TYPE: c_uint = 0x00;
pub const M5602_XB_SENSOR_CTRL: c_uint = 0x01;
pub const M5602_XB_LINE_OF_FRAME_H: c_uint = 0x02;
pub const M5602_XB_LINE_OF_FRAME_L: c_uint = 0x03;
pub const M5602_XB_PIX_OF_LINE_H: c_uint = 0x04;
pub const M5602_XB_PIX_OF_LINE_L: c_uint = 0x05;
pub const M5602_XB_VSYNC_PARA: c_uint = 0x06;
pub const M5602_XB_HSYNC_PARA: c_uint = 0x07;
pub const M5602_XB_TEST_MODE_1: c_uint = 0x08;
pub const M5602_XB_TEST_MODE_2: c_uint = 0x09;
pub const M5602_XB_SIG_INI: c_uint = 0x0a;
pub const M5602_XB_DS_PARA: c_uint = 0x0e;
pub const M5602_XB_TRIG_PARA: c_uint = 0x0f;
pub const M5602_XB_CLK_PD: c_uint = 0x10;
pub const M5602_XB_MCU_CLK_CTRL: c_uint = 0x12;
pub const M5602_XB_MCU_CLK_DIV: c_uint = 0x13;
pub const M5602_XB_SEN_CLK_CTRL: c_uint = 0x14;
pub const M5602_XB_SEN_CLK_DIV: c_uint = 0x15;
pub const M5602_XB_AUD_CLK_CTRL: c_uint = 0x16;
pub const M5602_XB_AUD_CLK_DIV: c_uint = 0x17;
pub const M5602_OB_AC_LINK_STATE: c_uint = 0x22;
pub const M5602_OB_PCM_SLOT_INDEX: c_uint = 0x24;
pub const M5602_OB_GPIO_SLOT_INDEX: c_uint = 0x25;
pub const M5602_OB_ACRX_STATUS_ADDRESS_H: c_uint = 0x28;
pub const M5602_OB_ACRX_STATUS_DATA_L: c_uint = 0x29;
pub const M5602_OB_ACRX_STATUS_DATA_H: c_uint = 0x2a;
pub const M5602_OB_ACTX_COMMAND_ADDRESS: c_uint = 0x31;
pub const M5602_OB_ACRX_COMMAND_DATA_L: c_uint = 0x32;

pub const M5602_XB_DEVCTR1: c_uint = 0x41;
pub const M5602_XB_EPSETR0: c_uint = 0x42;
pub const M5602_XB_EPAFCTR: c_uint = 0x47;
pub const M5602_XB_EPBFCTR: c_uint = 0x49;
pub const M5602_XB_EPEFCTR: c_uint = 0x4f;
pub const M5602_XB_TEST_REG: c_uint = 0x53;
pub const M5602_XB_ALT2SIZE: c_uint = 0x54;
pub const M5602_XB_ALT3SIZE: c_uint = 0x55;
pub const M5602_XB_OBSFRAME: c_uint = 0x56;
pub const M5602_XB_PWR_CTL: c_uint = 0x59;
pub const M5602_XB_ADC_CTRL: c_uint = 0x60;
pub const M5602_XB_ADC_DATA: c_uint = 0x61;
pub const M5602_XB_MISC_CTRL: c_uint = 0x62;
pub const M5602_XB_SNAPSHOT: c_uint = 0x63;
pub const M5602_XB_SCRATCH_1: c_uint = 0x64;
pub const M5602_XB_SCRATCH_2: c_uint = 0x65;
pub const M5602_XB_SCRATCH_3: c_uint = 0x66;
pub const M5602_XB_SCRATCH_4: c_uint = 0x67;
pub const M5602_XB_I2C_CTRL: c_uint = 0x68;
pub const M5602_XB_I2C_CLK_DIV: c_uint = 0x69;
pub const M5602_XB_I2C_DEV_ADDR: c_uint = 0x6a;
pub const M5602_XB_I2C_REG_ADDR: c_uint = 0x6b;
pub const M5602_XB_I2C_DATA: c_uint = 0x6c;
pub const M5602_XB_I2C_STATUS: c_uint = 0x6d;
pub const M5602_XB_GPIO_DAT_H: c_uint = 0x70;
pub const M5602_XB_GPIO_DAT_L: c_uint = 0x71;
pub const M5602_XB_GPIO_DIR_H: c_uint = 0x72;
pub const M5602_XB_GPIO_DIR_L: c_uint = 0x73;
pub const M5602_XB_GPIO_EN_H: c_uint = 0x74;
pub const M5602_XB_GPIO_EN_L: c_uint = 0x75;
pub const M5602_XB_GPIO_DAT: c_uint = 0x76;
pub const M5602_XB_GPIO_DIR: c_uint = 0x77;
pub const M5602_XB_SEN_CLK_CONTROL: c_uint = 0x80;
pub const M5602_XB_SEN_CLK_DIVISION: c_uint = 0x81;
pub const M5602_XB_CPR_CLK_CONTROL: c_uint = 0x82;
pub const M5602_XB_CPR_CLK_DIVISION: c_uint = 0x83;
pub const M5602_XB_MCU_CLK_CONTROL: c_uint = 0x84;
pub const M5602_XB_MCU_CLK_DIVISION: c_uint = 0x85;
pub const M5602_XB_DCT_CLK_CONTROL: c_uint = 0x86;
pub const M5602_XB_DCT_CLK_DIVISION: c_uint = 0x87;
pub const M5602_XB_EC_CLK_CONTROL: c_uint = 0x88;
pub const M5602_XB_EC_CLK_DIVISION: c_uint = 0x89;
pub const M5602_XB_LBUF_CLK_CONTROL: c_uint = 0x8a;
pub const M5602_XB_LBUF_CLK_DIVISION: c_uint = 0x8b;
pub const I2C_BUSY: c_uint = 0x80;
//
// Driver info

pub const M5602_ISOC_ENDPOINT_ADDR: c_uint = 0x81;
pub const M5602_INTR_ENDPOINT_ADDR: c_uint = 0x82;
pub const M5602_URB_MSG_TIMEOUT: c_int = 5000;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd {
    pub gspca_dev: gspca_dev,
// A pointer to the currently connected sensor
    pub sensor: *const m5602_sensor,
// The current frame's id, used to detect frame boundaries
    pub frame_id: u8,
// The current frame count
    pub frame_count: u32,
// Camera rotation polling thread for "flipable" cams
    pub rotation_thread: *mut task_struct,
    pub auto_white_bal: *mut v4l2_ctrl,
    pub red_bal: *mut v4l2_ctrl,
    pub blue_bal: *mut v4l2_ctrl,
    pub green_bal: *mut v4l2_ctrl,
}
