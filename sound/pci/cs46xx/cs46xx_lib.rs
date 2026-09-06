//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/cs46xx/cs46xx_lib.h
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
// The driver for the Cirrus Logic's Sound Fusion CS46XX based soundcards
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//
// constants
//
pub const CS46XX_BA0_SIZE: c_uint = 0x1000;
pub const CS46XX_BA1_DATA0_SIZE: c_uint = 0x3000;
pub const CS46XX_BA1_DATA1_SIZE: c_uint = 0x3800;
pub const CS46XX_BA1_PRG_SIZE: c_uint = 0x7000;
pub const CS46XX_BA1_REG_SIZE: c_uint = 0x0100;

pub const CS46XX_MIN_PERIOD_SIZE: c_int = 64;

pub const CS46XX_MIN_PERIOD_SIZE: c_int = 2048;
pub const CS46XX_MAX_PERIOD_SIZE: c_int = 2048;

pub const CS46XX_FRAGS: c_int = 2;
// #define CS46XX_BUFFER_SIZE CS46XX_MAX_PERIOD_SIZE * CS46XX_FRAGS
pub const SCB_NO_PARENT: c_int = 0;
pub const SCB_ON_PARENT_NEXT_SCB: c_int = 1;
pub const SCB_ON_PARENT_SUBLIST_SCB: c_int = 2;
// 3*1024 parameter, 3.5*1024 sample, 2*3.5*1024 code

pub const BA1_MEMORY_COUNT: c_int = 3;
//
// common I/O routines
//
extern "C" {
    pub fn readl(offset: chip->region.idx[bank+1].remap_addr +) -> return;
}
extern "C" {
    pub fn readl(offset: chip->region.name.ba0.remap_addr +) -> return;
}
extern "C" {
    pub fn cs46xx_dsp_spos_destroy(chip: *mut *mut snd_cs46xx);
}
extern "C" {
    pub fn cs46xx_dsp_load_module(chip: *mut *mut snd_cs46xx, module: *mut *mut dsp_module_desc) -> c_int;
}

extern "C" {
    pub fn cs46xx_dsp_resume(chip: *mut *mut snd_cs46xx) -> c_int;
}

extern "C" {
    pub fn cs46xx_dsp_proc_init(card: *mut snd_card, chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_proc_done(chip: *mut snd_cs46xx) -> c_int;
}

// Macro flag: #define cs46xx_dsp_proc_done(chip)

extern "C" {
    pub fn cs46xx_dsp_scb_and_task_init(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn snd_cs46xx_clear_BA1(chip: *mut snd_cs46xx, offset: c_ulong, len: c_ulong) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_enable_spdif_out(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_enable_spdif_hw(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_disable_spdif_out(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_enable_spdif_in(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_disable_spdif_in(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_enable_pcm_capture(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_disable_pcm_capture(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_enable_adc_capture(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_disable_adc_capture(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_poke_via_dsp(chip: *mut snd_cs46xx, address: u32, data: u32) -> c_int;
}

extern "C" {
    pub fn cs46xx_dsp_proc_free_scb_desc(scb: *mut *mut dsp_scb_descriptor);
}

// Macro flag: #define cs46xx_dsp_proc_free_scb_desc(scb)

extern "C" {
    pub fn cs46xx_dsp_create_timing_master_scb(chip: *mut snd_cs46xx) -> *mut dsp_scb_descriptor;
}
extern "C" {
    pub fn cs46xx_src_unlink(chip: *mut snd_cs46xx, src: *mut *mut dsp_scb_descriptor) -> c_int;
}
extern "C" {
    pub fn cs46xx_src_link(chip: *mut snd_cs46xx, src: *mut *mut dsp_scb_descriptor) -> c_int;
}
extern "C" {
    pub fn cs46xx_iec958_pre_open(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_iec958_post_close(chip: *mut snd_cs46xx) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_pcm_ostream_set_period(chip: *mut *mut snd_cs46xx, period_size: c_int) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_set_dac_volume(chip: *mut *mut snd_cs46xx, left: u16, right: u16) -> c_int;
}
extern "C" {
    pub fn cs46xx_dsp_set_iec958_volume(chip: *mut *mut snd_cs46xx, left: u16, right: u16) -> c_int;
}
