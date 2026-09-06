//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/pcxhr/pcxhr_core.h
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
// Driver for Digigram pcxhr compatible soundcards
//
// low level interface with interrupt and message handling
//
// Copyright (c) 2004 by Digigram <alsa@digigram.com>
//
// init and firmware download commands
extern "C" {
    pub fn pcxhr_reset_xilinx_com(mgr: *mut pcxhr_mgr);
}
extern "C" {
    pub fn pcxhr_reset_dsp(mgr: *mut pcxhr_mgr);
}
extern "C" {
    pub fn pcxhr_enable_dsp(mgr: *mut pcxhr_mgr);
}
extern "C" {
    pub fn pcxhr_load_xilinx_binary(mgr: *mut pcxhr_mgr, xilinx: *const firmware, second: c_int) -> c_int;
}
extern "C" {
    pub fn pcxhr_load_eeprom_binary(mgr: *mut pcxhr_mgr, eeprom: *const firmware) -> c_int;
}
extern "C" {
    pub fn pcxhr_load_boot_binary(mgr: *mut pcxhr_mgr, boot: *const firmware) -> c_int;
}
extern "C" {
    pub fn pcxhr_load_dsp_binary(mgr: *mut pcxhr_mgr, dsp: *const firmware) -> c_int;
}
// DSP time available on MailBox4 register : 24 bit time samples()
pub const PCXHR_DSP_TIME_MASK: c_uint = 0x00ffffff;
pub const PCXHR_DSP_TIME_INVALID: c_uint = 0x10000000;
pub const PCXHR_SIZE_MAX_CMD: c_int = 8;
pub const PCXHR_SIZE_MAX_STATUS: c_int = 16;
pub const PCXHR_SIZE_MAX_LONG_STATUS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcxhr_rmh {
    pub /: *mut *mut u16 cmd_len; / length of the command to send (WORDs),
    pub /: *mut *mut u16 stat_len; / length of the status received (WORDs),
    pub /: *mut *mut u16 dsp_stat; / status type, RMP_SSIZE_XXX,
    pub /: *mut *mut u16 cmd_idx; / index of the command,
    pub cmd: [u32; PCXHR_SIZE_MAX_CMD],
    pub stat: [u32; PCXHR_SIZE_MAX_STATUS],
}

pub const MASK_DSP_WORD: c_uint = 0x00ffffff;
pub const MASK_ALL_STREAM: c_uint = 0x00ffffff;
pub const MASK_DSP_WORD_LEVEL: c_uint = 0x000001ff;
pub const MASK_FIRST_FIELD: c_uint = 0x0000001f;
pub const FIELD_SIZE: c_int = 5;
//
extern "C" {
    pub fn pcxhr_init_rmh(rmh: *mut pcxhr_rmh, cmd: c_int);
}

//
extern "C" {
    pub fn pcxhr_send_msg(mgr: *mut pcxhr_mgr, rmh: *mut pcxhr_rmh) -> c_int;
}
// values used for CMD_ACCESS_IO_WRITE and CMD_ACCESS_IO_READ
pub const IO_NUM_REG_CONT: c_int = 0;
pub const IO_NUM_REG_GENCLK: c_int = 1;
pub const IO_NUM_REG_MUTE_OUT: c_int = 2;
pub const IO_NUM_SPEED_RATIO: c_int = 4;
pub const IO_NUM_REG_STATUS: c_int = 5;
pub const IO_NUM_REG_CUER: c_int = 10;
pub const IO_NUM_UER_CHIP_REG: c_int = 11;
pub const IO_NUM_REG_CONFIG_SRC: c_int = 12;
pub const IO_NUM_REG_OUT_ANA_LEVEL: c_int = 20;
pub const IO_NUM_REG_IN_ANA_LEVEL: c_int = 21;
pub const REG_CONT_VALSMPTE: c_uint = 0x000800;
pub const REG_CONT_UNMUTE_INPUTS: c_uint = 0x020000;
// parameters used with register IO_NUM_REG_STATUS
pub const REG_STATUS_OPTIONS: c_int = 0;
pub const REG_STATUS_AES_SYNC: c_int = 8;
pub const REG_STATUS_AES_1: c_int = 9;
pub const REG_STATUS_AES_2: c_int = 10;
pub const REG_STATUS_AES_3: c_int = 11;
pub const REG_STATUS_AES_4: c_int = 12;
pub const REG_STATUS_WORD_CLOCK: c_int = 13;
pub const REG_STATUS_INTER_SYNC: c_int = 14;
pub const REG_STATUS_CURRENT: c_uint = 0x80;
// results
pub const REG_STATUS_OPT_NO_VIDEO_SIGNAL: c_uint = 0x01;
pub const REG_STATUS_OPT_DAUGHTER_MASK: c_uint = 0x1c;
pub const REG_STATUS_OPT_ANALOG_BOARD: c_uint = 0x00;
pub const REG_STATUS_OPT_NO_DAUGHTER: c_uint = 0x1c;
pub const REG_STATUS_OPT_COMPANION_MASK: c_uint = 0xe0;
pub const REG_STATUS_OPT_NO_COMPANION: c_uint = 0xe0;
pub const REG_STATUS_SYNC_32000: c_uint = 0x00;
pub const REG_STATUS_SYNC_44100: c_uint = 0x01;
pub const REG_STATUS_SYNC_48000: c_uint = 0x02;
pub const REG_STATUS_SYNC_64000: c_uint = 0x03;
pub const REG_STATUS_SYNC_88200: c_uint = 0x04;
pub const REG_STATUS_SYNC_96000: c_uint = 0x05;
pub const REG_STATUS_SYNC_128000: c_uint = 0x06;
pub const REG_STATUS_SYNC_176400: c_uint = 0x07;
pub const REG_STATUS_SYNC_192000: c_uint = 0x08;
extern "C" {
    pub fn pcxhr_set_pipe_state(mgr: *mut pcxhr_mgr, playback_mask: c_int, capture_mask: c_int, start: c_int) -> c_int;
}
// codec parameters
pub const CS8416_RUN: c_uint = 0x200401;
pub const CS8416_FORMAT_DETECT: c_uint = 0x200b00;
pub const CS8416_CSB0: c_uint = 0x201900;
pub const CS8416_CSB1: c_uint = 0x201a00;
pub const CS8416_CSB2: c_uint = 0x201b00;
pub const CS8416_CSB3: c_uint = 0x201c00;
pub const CS8416_CSB4: c_uint = 0x201d00;
pub const CS8416_VERSION: c_uint = 0x207f00;
pub const CS8420_DATA_FLOW_CTL: c_uint = 0x200301;
pub const CS8420_CLOCK_SRC_CTL: c_uint = 0x200401;
pub const CS8420_RECEIVER_ERRORS: c_uint = 0x201000;
pub const CS8420_SRC_RATIO: c_uint = 0x201e00;
pub const CS8420_CSB0: c_uint = 0x202000;
pub const CS8420_CSB1: c_uint = 0x202100;
pub const CS8420_CSB2: c_uint = 0x202200;
pub const CS8420_CSB3: c_uint = 0x202300;
pub const CS8420_CSB4: c_uint = 0x202400;
pub const CS8420_VERSION: c_uint = 0x207f00;
pub const CS4271_MODE_CTL_1: c_uint = 0x200101;
pub const CS4271_DAC_CTL: c_uint = 0x200201;
pub const CS4271_VOLMIX: c_uint = 0x200301;
pub const CS4271_VOLMUTE_LEFT: c_uint = 0x200401;
pub const CS4271_VOLMUTE_RIGHT: c_uint = 0x200501;
pub const CS4271_ADC_CTL: c_uint = 0x200601;
pub const CS4271_MODE_CTL_2: c_uint = 0x200701;
pub const CHIP_SIG_AND_MAP_SPI: c_uint = 0xff7f00;
// codec selection
pub const CS4271_01_CS: c_uint = 0x160018;
pub const CS4271_23_CS: c_uint = 0x160019;
pub const CS4271_45_CS: c_uint = 0x16001a;
pub const CS4271_67_CS: c_uint = 0x16001b;
pub const CS4271_89_CS: c_uint = 0x16001c;
pub const CS4271_AB_CS: c_uint = 0x16001d;
pub const CS8420_01_CS: c_uint = 0x080090;
pub const CS8420_23_CS: c_uint = 0x080092;
pub const CS8420_45_CS: c_uint = 0x080094;
pub const CS8420_67_CS: c_uint = 0x080096;
pub const CS8416_01_CS: c_uint = 0x080098;
// interrupt handling
extern "C" {
    pub fn pcxhr_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn pcxhr_threaded_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
