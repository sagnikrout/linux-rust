//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/hw.h
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
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//

// VIA output devices
pub const VIA_LDVP0: c_uint = 0x00000001;
pub const VIA_LDVP1: c_uint = 0x00000002;
pub const VIA_DVP0: c_uint = 0x00000004;
pub const VIA_CRT: c_uint = 0x00000010;
pub const VIA_DVP1: c_uint = 0x00000020;
pub const VIA_LVDS1: c_uint = 0x00000040;
pub const VIA_LVDS2: c_uint = 0x00000080;
// VIA output device power states
pub const VIA_STATE_ON: c_int = 0;
pub const VIA_STATE_STANDBY: c_int = 1;
pub const VIA_STATE_SUSPEND: c_int = 2;
pub const VIA_STATE_OFF: c_int = 3;
// VIA output device sync polarity
pub const VIA_HSYNC_NEGATIVE: c_uint = 0x01;
pub const VIA_VSYNC_NEGATIVE: c_uint = 0x02;
//
// Definition IGA2 Design Method of CRTC Shadow Registers
//

// Define Register Number for IGA2 Shadow CRTC Timing
// location: {CR6D,0,7},{CR71,3,3}
pub const IGA2_SHADOW_HOR_TOTAL_REG_NUM: c_int = 2;
// location: {CR6E,0,7}
pub const IGA2_SHADOW_HOR_BLANK_END_REG_NUM: c_int = 1;
// location: {CR6F,0,7},{CR71,0,2}
pub const IGA2_SHADOW_VER_TOTAL_REG_NUM: c_int = 2;
// location: {CR70,0,7},{CR71,4,6}
pub const IGA2_SHADOW_VER_ADDR_REG_NUM: c_int = 2;
// location: {CR72,0,7},{CR74,4,6}
pub const IGA2_SHADOW_VER_BLANK_START_REG_NUM: c_int = 2;
// location: {CR73,0,7},{CR74,0,2}
pub const IGA2_SHADOW_VER_BLANK_END_REG_NUM: c_int = 2;
// location: {CR75,0,7},{CR76,4,6}
pub const IGA2_SHADOW_VER_SYNC_START_REG_NUM: c_int = 2;
// location: {CR76,0,3}
pub const IGA2_SHADOW_VER_SYNC_END_REG_NUM: c_int = 1;
// Define Fetch Count Register
// location: {SR1C,0,7},{SR1D,0,1}
pub const IGA1_FETCH_COUNT_REG_NUM: c_int = 2;
// 16 bytes alignment.
pub const IGA1_FETCH_COUNT_ALIGN_BYTE: c_int = 16;
// x: H resolution, y: color depth
pub const IGA1_FETCH_COUNT_PATCH_VALUE: c_int = 4;

// location: {CR65,0,7},{CR67,2,3}
pub const IGA2_FETCH_COUNT_REG_NUM: c_int = 2;
pub const IGA2_FETCH_COUNT_ALIGN_BYTE: c_int = 16;
pub const IGA2_FETCH_COUNT_PATCH_VALUE: c_int = 0;

// Staring Address
// location: {CR0C,0,7},{CR0D,0,7},{CR34,0,7},{CR48,0,1}
pub const IGA1_STARTING_ADDR_REG_NUM: c_int = 4;
// location: {CR62,1,7},{CR63,0,7},{CR64,0,7}
pub const IGA2_STARTING_ADDR_REG_NUM: c_int = 3;
// Define Display OFFSET
// These value are by HW suggested value
// location: {SR17,0,7}
pub const K800_IGA1_FIFO_MAX_DEPTH: c_int = 384;
// location: {SR16,0,5},{SR16,7,7}
pub const K800_IGA1_FIFO_THRESHOLD: c_int = 328;
// location: {SR18,0,5},{SR18,7,7}
pub const K800_IGA1_FIFO_HIGH_THRESHOLD: c_int = 296;
// location: {SR22,0,4}. (128/4) =64, K800 must be set zero,
// because HW only 5 bits
pub const K800_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 0;
// location: {CR68,4,7},{CR94,7,7},{CR95,7,7}
pub const K800_IGA2_FIFO_MAX_DEPTH: c_int = 384;
// location: {CR68,0,3},{CR95,4,6}
pub const K800_IGA2_FIFO_THRESHOLD: c_int = 328;
// location: {CR92,0,3},{CR95,0,2}
pub const K800_IGA2_FIFO_HIGH_THRESHOLD: c_int = 296;
// location: {CR94,0,6}
pub const K800_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 128;
// location: {SR17,0,7}
pub const P880_IGA1_FIFO_MAX_DEPTH: c_int = 192;
// location: {SR16,0,5},{SR16,7,7}
pub const P880_IGA1_FIFO_THRESHOLD: c_int = 128;
// location: {SR18,0,5},{SR18,7,7}
pub const P880_IGA1_FIFO_HIGH_THRESHOLD: c_int = 64;
// location: {SR22,0,4}. (128/4) =64, K800 must be set zero,
// because HW only 5 bits
pub const P880_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 0;
// location: {CR68,4,7},{CR94,7,7},{CR95,7,7}
pub const P880_IGA2_FIFO_MAX_DEPTH: c_int = 96;
// location: {CR68,0,3},{CR95,4,6}
pub const P880_IGA2_FIFO_THRESHOLD: c_int = 64;
// location: {CR92,0,3},{CR95,0,2}
pub const P880_IGA2_FIFO_HIGH_THRESHOLD: c_int = 32;
// location: {CR94,0,6}
pub const P880_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 128;
// VT3314 chipset
// location: {SR17,0,7}
pub const CN700_IGA1_FIFO_MAX_DEPTH: c_int = 96;
// location: {SR16,0,5},{SR16,7,7}
pub const CN700_IGA1_FIFO_THRESHOLD: c_int = 80;
// location: {SR18,0,5},{SR18,7,7}
pub const CN700_IGA1_FIFO_HIGH_THRESHOLD: c_int = 64;
// location: {SR22,0,4}. (128/4) =64, P800 must be set zero,
pub const CN700_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 0;
// location: {CR68,4,7},{CR94,7,7},{CR95,7,7}
pub const CN700_IGA2_FIFO_MAX_DEPTH: c_int = 96;
// location: {CR68,0,3},{CR95,4,6}
pub const CN700_IGA2_FIFO_THRESHOLD: c_int = 80;
// location: {CR92,0,3},{CR95,0,2}
pub const CN700_IGA2_FIFO_HIGH_THRESHOLD: c_int = 32;
// location: {CR94,0,6}
pub const CN700_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 128;
// For VT3324, these values are suggested by HW
// location: {SR17,0,7}
pub const CX700_IGA1_FIFO_MAX_DEPTH: c_int = 192;
// location: {SR16,0,5},{SR16,7,7}
pub const CX700_IGA1_FIFO_THRESHOLD: c_int = 128;
// location: {SR18,0,5},{SR18,7,7}
pub const CX700_IGA1_FIFO_HIGH_THRESHOLD: c_int = 128;
// location: {SR22,0,4}
pub const CX700_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 124;
// location: {CR68,4,7},{CR94,7,7},{CR95,7,7}
pub const CX700_IGA2_FIFO_MAX_DEPTH: c_int = 96;
// location: {CR68,0,3},{CR95,4,6}
pub const CX700_IGA2_FIFO_THRESHOLD: c_int = 64;
// location: {CR92,0,3},{CR95,0,2}
pub const CX700_IGA2_FIFO_HIGH_THRESHOLD: c_int = 32;
// location: {CR94,0,6}
pub const CX700_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 128;
// VT3336 chipset
// location: {SR17,0,7}
pub const K8M890_IGA1_FIFO_MAX_DEPTH: c_int = 360;
// location: {SR16,0,5},{SR16,7,7}
pub const K8M890_IGA1_FIFO_THRESHOLD: c_int = 328;
// location: {SR18,0,5},{SR18,7,7}
pub const K8M890_IGA1_FIFO_HIGH_THRESHOLD: c_int = 296;
// location: {SR22,0,4}.
pub const K8M890_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 124;
// location: {CR68,4,7},{CR94,7,7},{CR95,7,7}
pub const K8M890_IGA2_FIFO_MAX_DEPTH: c_int = 360;
// location: {CR68,0,3},{CR95,4,6}
pub const K8M890_IGA2_FIFO_THRESHOLD: c_int = 328;
// location: {CR92,0,3},{CR95,0,2}
pub const K8M890_IGA2_FIFO_HIGH_THRESHOLD: c_int = 296;
// location: {CR94,0,6}
pub const K8M890_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 124;
// VT3327 chipset
// location: {SR17,0,7}
pub const P4M890_IGA1_FIFO_MAX_DEPTH: c_int = 96;
// location: {SR16,0,5},{SR16,7,7}
pub const P4M890_IGA1_FIFO_THRESHOLD: c_int = 76;
// location: {SR18,0,5},{SR18,7,7}
pub const P4M890_IGA1_FIFO_HIGH_THRESHOLD: c_int = 64;
// location: {SR22,0,4}. (32/4) =8
pub const P4M890_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 32;
// location: {CR68,4,7},{CR94,7,7},{CR95,7,7}
pub const P4M890_IGA2_FIFO_MAX_DEPTH: c_int = 96;
// location: {CR68,0,3},{CR95,4,6}
pub const P4M890_IGA2_FIFO_THRESHOLD: c_int = 76;
// location: {CR92,0,3},{CR95,0,2}
pub const P4M890_IGA2_FIFO_HIGH_THRESHOLD: c_int = 64;
// location: {CR94,0,6}
pub const P4M890_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 32;
// VT3364 chipset
// location: {SR17,0,7}
pub const P4M900_IGA1_FIFO_MAX_DEPTH: c_int = 96;
// location: {SR16,0,5},{SR16,7,7}
pub const P4M900_IGA1_FIFO_THRESHOLD: c_int = 76;
// location: {SR18,0,5},{SR18,7,7}
pub const P4M900_IGA1_FIFO_HIGH_THRESHOLD: c_int = 76;
// location: {SR22,0,4}.
pub const P4M900_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 32;
// location: {CR68,4,7},{CR94,7,7},{CR95,7,7}
pub const P4M900_IGA2_FIFO_MAX_DEPTH: c_int = 96;
// location: {CR68,0,3},{CR95,4,6}
pub const P4M900_IGA2_FIFO_THRESHOLD: c_int = 76;
// location: {CR92,0,3},{CR95,0,2}
pub const P4M900_IGA2_FIFO_HIGH_THRESHOLD: c_int = 76;
// location: {CR94,0,6}
pub const P4M900_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 32;
// For VT3353, these values are suggested by HW
// location: {SR17,0,7}
pub const VX800_IGA1_FIFO_MAX_DEPTH: c_int = 192;
// location: {SR16,0,5},{SR16,7,7}
pub const VX800_IGA1_FIFO_THRESHOLD: c_int = 152;
// location: {SR18,0,5},{SR18,7,7}
pub const VX800_IGA1_FIFO_HIGH_THRESHOLD: c_int = 152;
// location: {SR22,0,4}
pub const VX800_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 64;
// location: {CR68,4,7},{CR94,7,7},{CR95,7,7}
pub const VX800_IGA2_FIFO_MAX_DEPTH: c_int = 96;
// location: {CR68,0,3},{CR95,4,6}
pub const VX800_IGA2_FIFO_THRESHOLD: c_int = 64;
// location: {CR92,0,3},{CR95,0,2}
pub const VX800_IGA2_FIFO_HIGH_THRESHOLD: c_int = 32;
// location: {CR94,0,6}
pub const VX800_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 128;
// For VT3409
pub const VX855_IGA1_FIFO_MAX_DEPTH: c_int = 400;
pub const VX855_IGA1_FIFO_THRESHOLD: c_int = 320;
pub const VX855_IGA1_FIFO_HIGH_THRESHOLD: c_int = 320;
pub const VX855_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 160;
pub const VX855_IGA2_FIFO_MAX_DEPTH: c_int = 200;
pub const VX855_IGA2_FIFO_THRESHOLD: c_int = 160;
pub const VX855_IGA2_FIFO_HIGH_THRESHOLD: c_int = 160;
pub const VX855_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 320;
// For VT3410
pub const VX900_IGA1_FIFO_MAX_DEPTH: c_int = 400;
pub const VX900_IGA1_FIFO_THRESHOLD: c_int = 320;
pub const VX900_IGA1_FIFO_HIGH_THRESHOLD: c_int = 320;
pub const VX900_IGA1_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 160;
pub const VX900_IGA2_FIFO_MAX_DEPTH: c_int = 192;
pub const VX900_IGA2_FIFO_THRESHOLD: c_int = 160;
pub const VX900_IGA2_FIFO_HIGH_THRESHOLD: c_int = 160;
pub const VX900_IGA2_DISPLAY_QUEUE_EXPIRE_NUM: c_int = 320;
pub const IGA1_FIFO_DEPTH_SELECT_REG_NUM: c_int = 1;
pub const IGA1_FIFO_THRESHOLD_REG_NUM: c_int = 2;
pub const IGA1_FIFO_HIGH_THRESHOLD_REG_NUM: c_int = 2;
pub const IGA1_DISPLAY_QUEUE_EXPIRE_NUM_REG_NUM: c_int = 1;
pub const IGA2_FIFO_DEPTH_SELECT_REG_NUM: c_int = 3;
pub const IGA2_FIFO_THRESHOLD_REG_NUM: c_int = 2;
pub const IGA2_FIFO_HIGH_THRESHOLD_REG_NUM: c_int = 2;
pub const IGA2_DISPLAY_QUEUE_EXPIRE_NUM_REG_NUM: c_int = 1;

//
// LCD Timing
//
// 500 ms = 500000 us
pub const LCD_POWER_SEQ_TD0: c_int = 500000;
// 50 ms = 50000 us
pub const LCD_POWER_SEQ_TD1: c_int = 50000;
// 0 us
pub const LCD_POWER_SEQ_TD2: c_int = 0;
// 210 ms = 210000 us
pub const LCD_POWER_SEQ_TD3: c_int = 210000;
// 2^10 * (1/14.31818M) = 71.475 us (K400.revA)
pub const CLE266_POWER_SEQ_UNIT: c_int = 71;
// 2^11 * (1/14.31818M) = 142.95 us (K400.revB)
pub const K800_POWER_SEQ_UNIT: c_int = 142;
// 2^13 * (1/14.31818M) = 572.1 us
pub const P880_POWER_SEQ_UNIT: c_int = 572;

// location: {CR8B,0,7},{CR8F,0,3}
pub const LCD_POWER_SEQ_TD0_REG_NUM: c_int = 2;
// location: {CR8C,0,7},{CR8F,4,7}
pub const LCD_POWER_SEQ_TD1_REG_NUM: c_int = 2;
// location: {CR8D,0,7},{CR90,0,3}
pub const LCD_POWER_SEQ_TD2_REG_NUM: c_int = 2;
// location: {CR8E,0,7},{CR90,4,7}
pub const LCD_POWER_SEQ_TD3_REG_NUM: c_int = 2;
// LCD Scaling factor
// x: indicate setting horizontal size
// y: indicate panel horizontal size
// Horizontal scaling factor 10 bits (2^10)

// Vertical scaling factor 10 bits (2^10)

// Horizontal scaling factor 10 bits (2^12)

// Vertical scaling factor 10 bits (2^11)

// location: {CR9F,0,1},{CR77,0,7},{CR79,4,5}
pub const LCD_HOR_SCALING_FACTOR_REG_NUM: c_int = 3;
// location: {CR79,3,3},{CR78,0,7},{CR79,6,7}
pub const LCD_VER_SCALING_FACTOR_REG_NUM: c_int = 3;
// location: {CR77,0,7},{CR79,4,5}
pub const LCD_HOR_SCALING_FACTOR_REG_NUM_CLE: c_int = 2;
// location: {CR78,0,7},{CR79,6,7}
pub const LCD_VER_SCALING_FACTOR_REG_NUM_CLE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_register {
    pub io_addr: u8,
    pub start_bit: u8,
    pub end_bit: u8,
}

//
// Define IGA2 Shadow Display Timing
//
// IGA2 Shadow Horizontal Total
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_shadow_hor_total {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_SHADOW_HOR_TOTAL_REG_NUM],
}

// IGA2 Shadow Horizontal Blank End
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_shadow_hor_blank_end {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_SHADOW_HOR_BLANK_END_REG_NUM],
}

// IGA2 Shadow Vertical Total
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_shadow_ver_total {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_SHADOW_VER_TOTAL_REG_NUM],
}

// IGA2 Shadow Vertical Addressable Video
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_shadow_ver_addr {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_SHADOW_VER_ADDR_REG_NUM],
}

// IGA2 Shadow Vertical Blank Start
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_shadow_ver_blank_start {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_SHADOW_VER_BLANK_START_REG_NUM],
}

// IGA2 Shadow Vertical Blank End
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_shadow_ver_blank_end {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_SHADOW_VER_BLANK_END_REG_NUM],
}

// IGA2 Shadow Vertical Sync Start
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_shadow_ver_sync_start {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_SHADOW_VER_SYNC_START_REG_NUM],
}

// IGA2 Shadow Vertical Sync End
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_shadow_ver_sync_end {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_SHADOW_VER_SYNC_END_REG_NUM],
}

// IGA1 Fetch Count Register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga1_fetch_count {
    pub reg_num: c_int,
    pub reg: [io_register; IGA1_FETCH_COUNT_REG_NUM],
}

// IGA2 Fetch Count Register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_fetch_count {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_FETCH_COUNT_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fetch_count {
    pub iga1_fetch_count_reg: iga1_fetch_count,
    pub iga2_fetch_count_reg: iga2_fetch_count,
}

// Starting Address Register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga1_starting_addr {
    pub reg_num: c_int,
    pub reg: [io_register; IGA1_STARTING_ADDR_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_starting_addr {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_STARTING_ADDR_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct starting_addr {
    pub iga1_starting_addr_reg: iga1_starting_addr,
    pub iga2_starting_addr_reg: iga2_starting_addr,
}

// LCD Power Sequence Timer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_pwd_seq_td0 {
    pub reg_num: c_int,
    pub reg: [io_register; LCD_POWER_SEQ_TD0_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_pwd_seq_td1 {
    pub reg_num: c_int,
    pub reg: [io_register; LCD_POWER_SEQ_TD1_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_pwd_seq_td2 {
    pub reg_num: c_int,
    pub reg: [io_register; LCD_POWER_SEQ_TD2_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_pwd_seq_td3 {
    pub reg_num: c_int,
    pub reg: [io_register; LCD_POWER_SEQ_TD3_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _lcd_pwd_seq_timer {
    pub td0: lcd_pwd_seq_td0,
    pub td1: lcd_pwd_seq_td1,
    pub td2: lcd_pwd_seq_td2,
    pub td3: lcd_pwd_seq_td3,
}

// LCD Scaling Factor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _lcd_hor_scaling_factor {
    pub reg_num: c_int,
    pub reg: [io_register; LCD_HOR_SCALING_FACTOR_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _lcd_ver_scaling_factor {
    pub reg_num: c_int,
    pub reg: [io_register; LCD_VER_SCALING_FACTOR_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _lcd_scaling_factor {
    pub lcd_hor_scaling_factor: _lcd_hor_scaling_factor,
    pub lcd_ver_scaling_factor: _lcd_ver_scaling_factor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_limit {
    pub multiplier_min: u16,
    pub multiplier_max: u16,
    pub divisor: u8,
    pub rshift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rgbLUT {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lcd_pwd_seq_timer {
    pub td0: u16,
    pub td1: u16,
    pub td2: u16,
    pub td3: u16,
}

// Display FIFO Relation Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga1_fifo_depth_select {
    pub reg_num: c_int,
    pub reg: [io_register; IGA1_FIFO_DEPTH_SELECT_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga1_fifo_threshold_select {
    pub reg_num: c_int,
    pub reg: [io_register; IGA1_FIFO_THRESHOLD_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga1_fifo_high_threshold_select {
    pub reg_num: c_int,
    pub reg: [io_register; IGA1_FIFO_HIGH_THRESHOLD_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga1_display_queue_expire_num {
    pub reg_num: c_int,
    pub reg: [io_register; IGA1_DISPLAY_QUEUE_EXPIRE_NUM_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_fifo_depth_select {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_FIFO_DEPTH_SELECT_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_fifo_threshold_select {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_FIFO_THRESHOLD_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_fifo_high_threshold_select {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_FIFO_HIGH_THRESHOLD_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_display_queue_expire_num {
    pub reg_num: c_int,
    pub reg: [io_register; IGA2_DISPLAY_QUEUE_EXPIRE_NUM_REG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fifo_depth_select {
    pub iga1_fifo_depth_select_reg: iga1_fifo_depth_select,
    pub iga2_fifo_depth_select_reg: iga2_fifo_depth_select,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fifo_threshold_select {
    pub iga1_fifo_threshold_select_reg: iga1_fifo_threshold_select,
    pub iga2_fifo_threshold_select_reg: iga2_fifo_threshold_select,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fifo_high_threshold_select {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_queue_expire_num {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iga2_shadow_crtc_timing {
    pub hor_total_shadow: iga2_shadow_hor_total,
    pub hor_blank_end_shadow: iga2_shadow_hor_blank_end,
    pub ver_total_shadow: iga2_shadow_ver_total,
    pub ver_addr_shadow: iga2_shadow_ver_addr,
    pub ver_blank_start_shadow: iga2_shadow_ver_blank_start,
    pub ver_blank_end_shadow: iga2_shadow_ver_blank_end,
    pub ver_sync_start_shadow: iga2_shadow_ver_sync_start,
    pub ver_sync_end_shadow: iga2_shadow_ver_sync_end,
}

// device ID
pub const CLE266_FUNCTION3: c_uint = 0x3123;
pub const KM400_FUNCTION3: c_uint = 0x3205;
pub const CN400_FUNCTION2: c_uint = 0x2259;
pub const CN400_FUNCTION3: c_uint = 0x3259;
// support VT3314 chipset
pub const CN700_FUNCTION2: c_uint = 0x2314;
pub const CN700_FUNCTION3: c_uint = 0x3208;
// VT3324 chipset
pub const CX700_FUNCTION2: c_uint = 0x2324;
pub const CX700_FUNCTION3: c_uint = 0x3324;
// VT3204 chipset
pub const KM800_FUNCTION3: c_uint = 0x3204;
// VT3336 chipset
pub const KM890_FUNCTION3: c_uint = 0x3336;
// VT3327 chipset
pub const P4M890_FUNCTION3: c_uint = 0x3327;
// VT3293 chipset
pub const CN750_FUNCTION3: c_uint = 0x3208;
// VT3364 chipset
pub const P4M900_FUNCTION3: c_uint = 0x3364;
// VT3353 chipset
pub const VX800_FUNCTION3: c_uint = 0x3353;
// VT3409 chipset
pub const VX855_FUNCTION3: c_uint = 0x3409;
// VT3410 chipset
pub const VX900_FUNCTION3: c_uint = 0x3410;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IODATA {
    pub Index: u8,
    pub Mask: u8,
    pub Data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_device_id_info {
    pub vendor: u32,
    pub device: u32,
    pub chip_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_device_mapping {
    pub device: u32,
    pub name: *const c_char,
}

extern "C" {
    pub fn viafb_set_vclock(CLK: u32, set_iga: c_int);
}
extern "C" {
    pub fn via_set_source(devices: u32, iga: u8);
}
extern "C" {
    pub fn via_set_state(devices: u32, state: u8);
}
extern "C" {
    pub fn via_set_sync_polarity(devices: u32, polarity: u8);
}
extern "C" {
    pub fn via_parse_odev(input: *mut c_char, end: *mut c_char) -> u32;
}
extern "C" {
    pub fn via_odev_to_seq(m: *mut seq_file, odev: u32);
}
extern "C" {
    pub fn init_ad9389();
}
// Access I/O Function
extern "C" {
    pub fn viafb_lock_crt();
}
extern "C" {
    pub fn viafb_unlock_crt();
}
extern "C" {
    pub fn viafb_load_fetch_count_reg(h_addr: c_int, bpp_byte: c_int, set_iga: c_int);
}
extern "C" {
    pub fn viafb_write_regx(RegTable[]: io_reg, ItemNum: c_int);
}
extern "C" {
    pub fn viafb_load_FIFO_reg(set_iga: c_int, hor_active: c_int, ver_active: c_int);
}
// p_gfx_dpa_setting);
extern "C" {
    pub fn viafb_setmode() -> c_int;
}
extern "C" {
    pub fn viafb_init_chip_info(chip_type: c_int);
}
extern "C" {
    pub fn viafb_init_dac(set_iga: c_int);
}
extern "C" {
    pub fn viafb_get_refresh(hres: c_int, vres: c_int, float_refresh: u32) -> c_int;
}
extern "C" {
    pub fn viafb_update_device_setting(hres: c_int, vres: c_int, bpp: c_int, flag: c_int);
}
extern "C" {
    pub fn viafb_set_iga_path();
}
extern "C" {
    pub fn viafb_set_primary_color_register(index: u8, red: u8, green: u8, blue: u8);
}
extern "C" {
    pub fn viafb_set_secondary_color_register(index: u8, red: u8, green: u8, blue: u8);
}
extern "C" {
    pub fn viafb_get_fb_info(fb_base: *mut c_uint, fb_len: *mut c_uint);
}
