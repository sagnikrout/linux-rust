//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/lx6464es/lx_core.h
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
// -*- linux-c -*-
//
// ALSA driver for the digigram lx6464es interface
// low-level interface
//
// Copyright (c) 2009 Tim Blechmann <tim@klingt.org>
//

pub const REG_CRM_NUMBER: c_int = 12;
// low-level register access
// dsp register access
extern "C" {
    pub fn lx_dsp_reg_read(chip: *mut lx6464es, port: c_int) -> c_ulong;
}
extern "C" {
    pub fn lx_dsp_reg_write(chip: *mut lx6464es, port: c_int, data: unsigned);
}
// plx register access
extern "C" {
    pub fn lx_plx_reg_read(chip: *mut lx6464es, port: c_int) -> c_ulong;
}
extern "C" {
    pub fn lx_plx_reg_write(chip: *mut lx6464es, port: c_int, data: u32);
}
// rhm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lx_rmh {
    pub /: *mut *mut u16 cmd_len; / length of the command to send (WORDs),
    pub /: *mut *mut u16 stat_len; / length of the status received (WORDs),
    pub /: *mut *mut u16 dsp_stat; / status type, RMP_SSIZE_XXX,
    pub /: *mut *mut u16 cmd_idx; / index of the command,
    pub cmd: [u32; REG_CRM_NUMBER],
    pub stat: [u32; REG_CRM_NUMBER],
}

// low-level dsp access
extern "C" {
    pub fn lx_dsp_get_version(chip: *mut lx6464es, rdsp_version: *mut u32) -> c_int;
}
extern "C" {
    pub fn lx_dsp_get_clock_frequency(chip: *mut lx6464es, rfreq: *mut u32) -> c_int;
}
extern "C" {
    pub fn lx_dsp_set_granularity(chip: *mut lx6464es, gran: u32) -> c_int;
}
extern "C" {
    pub fn lx_dsp_read_async_events(chip: *mut lx6464es, data: *mut u32) -> c_int;
}
extern "C" {
    pub fn lx_dsp_get_mac(chip: *mut lx6464es) -> c_int;
}
// low-level pipe handling
extern "C" {
    pub fn lx_pipe_release(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int;
}
extern "C" {
    pub fn lx_pipe_state(chip: *mut lx6464es, pipe: u32, is_capture: c_int, rstate: *mut u16) -> c_int;
}
extern "C" {
    pub fn lx_pipe_stop(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int;
}
extern "C" {
    pub fn lx_pipe_start(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int;
}
extern "C" {
    pub fn lx_pipe_pause(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int;
}
extern "C" {
    pub fn lx_pipe_wait_for_start(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int;
}
extern "C" {
    pub fn lx_pipe_wait_for_idle(chip: *mut lx6464es, pipe: u32, is_capture: c_int) -> c_int;
}
// low-level stream handling
extern "C" {
    pub fn lx_stream_set_state(_arg: chip, _arg: pipe, _arg: is_capture, _arg: SSTATE_RUN) -> return;
}
extern "C" {
    pub fn lx_stream_set_state(_arg: chip, _arg: pipe, _arg: is_capture, _arg: SSTATE_PAUSE) -> return;
}
extern "C" {
    pub fn lx_stream_set_state(_arg: chip, _arg: pipe, _arg: is_capture, _arg: SSTATE_STOP) -> return;
}
// low-level buffer handling
// low-level gain/peak handling
extern "C" {
    pub fn lx_level_unmute(chip: *mut lx6464es, is_capture: c_int, unmute: c_int) -> c_int;
}
// interrupt handling
extern "C" {
    pub fn lx_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lx_threaded_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lx_irq_enable(chip: *mut lx6464es);
}
extern "C" {
    pub fn lx_irq_disable(chip: *mut lx6464es);
}
// Stream Format Header Defines (for LIN and IEEE754)

pub const HEADER_FMT_BASE_LIN: c_uint = 0xFED00000;
pub const HEADER_FMT_BASE_FLOAT: c_uint = 0xFAD00000;
pub const HEADER_FMT_MONO: c_uint = 0x00000080 /* bit 23 in header_lo. WARNING: old;
// bit 22 is ignored in float
// format
pub const HEADER_FMT_INTEL: c_uint = 0x00008000;
pub const HEADER_FMT_16BITS: c_uint = 0x00002000;
pub const HEADER_FMT_24BITS: c_uint = 0x00004000;
pub const HEADER_FMT_UPTO11: c_uint = 0x00000200 /* frequency is less or equ. to 11k.;
//
pub const HEADER_FMT_UPTO32: c_uint = 0x00000100 /* frequency is over 11k and less;
// then 32k.
pub const BIT_FMP_HEADER: c_int = 23;
pub const BIT_FMP_SD: c_int = 22;
pub const BIT_FMP_MULTICHANNEL: c_int = 19;
pub const START_STATE: c_int = 1;
pub const PAUSE_STATE: c_int = 0;
// from PcxAll_e.h
// Start/Pause condition for pipes (PCXStartPipe, PCXPausePipe)
pub const START_PAUSE_IMMEDIATE: c_int = 0;
pub const START_PAUSE_ON_SYNCHRO: c_int = 1;
pub const START_PAUSE_ON_TIME_CODE: c_int = 2;
// Pipe / Stream state
pub const START_STATE: c_int = 1;
pub const PAUSE_STATE: c_int = 0;
// r_low = (u32)(ptr & 0xffffffff);

// r_high = 0;

// r_high = (u32)((u64)ptr>>32);

