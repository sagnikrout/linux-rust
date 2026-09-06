//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/fs-amp-lib.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// fs-amp-lib.h --- Common library for FourSemi Audio Amplifiers
//
// Copyright (C) 2016-2025 Shanghai FourSemi Semiconductor Co.,Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fs_index_type {
    FS_INDEX_INFO = 0,
    FS_INDEX_STCOEF,
    FS_INDEX_SCENE,
    FS_INDEX_MODEL,
    FS_INDEX_REG,
    FS_INDEX_EFFECT,
    FS_INDEX_STRING,
    FS_INDEX_WOOFER,
    FS_INDEX_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_reg_val {
    pub reg: u8,
    pub val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_reg_bits {
    pub /: *mut *mut u8 cmd; / FS_CMD_UPDATE,
    pub reg: u8,
    pub val: u16,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_cmd_pkg {
    pub cmd: u8,
    pub regv: fs_reg_val,
    pub regb: fs_reg_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_fwm_index {
// Index type
    pub type: u16,
// Offset address starting from the end of header
    pub offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_fwm_table {
    pub name: [c_char; FS_TABLE_NAME_LEN],
    pub /: *mut *mut u16 size; / size of buf,
    pub buf: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_scene_index {
// Offset address(scene name) in string table
    pub name: u16,
// Offset address(scene reg) in register table
    pub reg: u16,
// Offset address(scene model) in model table
    pub model: u16,
// Offset address(scene effect) in effect table
    pub effect: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_reg_table {
    pub /: *mut *mut u16 size; / size of buf,
    pub buf: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_file_table {
    pub name: u16,
    pub /: *mut *mut u16 size; / size of buf,
    pub buf: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_fwm_date {
    pub year:12: u32,
    pub month:4: u32,
    pub day:5: u32,
    pub hour:5: u32,
    pub minute:6: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_fwm_header {
    pub version: u16,
    pub /: *mut *mut u16 project; / Offset address(project name) in string table,
    pub /: *mut *mut u16 device; / Offset address(device name) in string table,
    pub date: fs_fwm_date,
    pub crc16: u16,
    pub /: *mut *mut u16 crc_size; / Starting position for CRC checking,
    pub chip_type: u16,
    pub /: *mut *mut u16 addr; / 7-bit i2c address,
    pub spkid: u16,
    pub rsvd: [u16; 6],
    pub params: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_i2s_srate {
    pub /: *mut *mut u32 srate; / Sample rate,
    pub /: *mut *mut u16 i2ssr; / Value of Bit field[I2SSR],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_pll_div {
    pub /: *mut *mut unsigned int bclk; / Rate of bit clock,
    pub pll1: u16,
    pub pll2: u16,
    pub pll3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_amp_scene {
    pub name: *const c_char,
    pub reg: *const fs_reg_table,
    pub model: *const fs_file_table,
    pub effect: *const fs_file_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_amp_lib {
    pub hdr: *const fs_fwm_header,
    pub table: [*const fs_fwm_table; FS_INDEX_MAX],
    pub scene: *mut fs_amp_scene,
    pub dev: *mut device,
    pub scene_count: c_int,
    pub devid: u16,
}

extern "C" {
    pub fn fs_amp_load_firmware(amp_lib: *mut fs_amp_lib, name: *const c_char) -> c_int;
}
