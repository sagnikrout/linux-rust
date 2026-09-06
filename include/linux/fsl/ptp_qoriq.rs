//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/ptp_qoriq.h
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
// Copyright (C) 2010 OMICRON electronics GmbH
// Copyright 2018 NXP
//

//
// qoriq ptp registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctrl_regs {
    pub /: *mut *mut u32 tmr_ctrl; / Timer control register,
    pub /: *mut *mut u32 tmr_tevent; / Timestamp event register,
    pub /: *mut *mut u32 tmr_temask; / Timer event mask register,
    pub /: *mut *mut u32 tmr_pevent; / Timestamp event register,
    pub /: *mut *mut u32 tmr_pemask; / Timer event mask register,
    pub /: *mut *mut u32 tmr_stat; / Timestamp status register,
    pub /: *mut *mut u32 tmr_cnt_h; / Timer counter high register,
    pub /: *mut *mut u32 tmr_cnt_l; / Timer counter low register,
    pub /: *mut *mut u32 tmr_add; / Timer drift compensation addend register,
    pub /: *mut *mut u32 tmr_acc; / Timer accumulator register,
    pub /: *mut *mut u32 tmr_prsc; / Timer prescale,
    pub res1: [u8; 4],
    pub /: *mut *mut u32 tmroff_h; / Timer offset high,
    pub /: *mut *mut u32 tmroff_l; / Timer offset low,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alarm_regs {
    pub /: *mut *mut u32 tmr_alarm1_h; / Timer alarm 1 high register,
    pub /: *mut *mut u32 tmr_alarm1_l; / Timer alarm 1 high register,
    pub /: *mut *mut u32 tmr_alarm2_h; / Timer alarm 2 high register,
    pub /: *mut *mut u32 tmr_alarm2_l; / Timer alarm 2 high register,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fiper_regs {
    pub /: *mut *mut u32 tmr_fiper1; / Timer fixed period interval,
    pub /: *mut *mut u32 tmr_fiper2; / Timer fixed period interval,
    pub /: *mut *mut u32 tmr_fiper3; / Timer fixed period interval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etts_regs {
    pub /: *mut *mut u32 tmr_etts1_h; / Timestamp of general purpose external trigger,
    pub /: *mut *mut u32 tmr_etts1_l; / Timestamp of general purpose external trigger,
    pub /: *mut *mut u32 tmr_etts2_h; / Timestamp of general purpose external trigger,
    pub /: *mut *mut u32 tmr_etts2_l; / Timestamp of general purpose external trigger,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_qoriq_registers {
    pub ctrl_regs: *mut ctrl_regs __iomem,
    pub alarm_regs: *mut alarm_regs __iomem,
    pub fiper_regs: *mut fiper_regs __iomem,
    pub etts_regs: *mut etts_regs __iomem,
}

// Offset definitions for the four register groups
pub const ETSEC_CTRL_REGS_OFFSET: c_uint = 0x0;
pub const ETSEC_ALARM_REGS_OFFSET: c_uint = 0x40;
pub const ETSEC_FIPER_REGS_OFFSET: c_uint = 0x80;
pub const ETSEC_ETTS_REGS_OFFSET: c_uint = 0xa0;
pub const CTRL_REGS_OFFSET: c_uint = 0x80;
pub const ALARM_REGS_OFFSET: c_uint = 0xb8;
pub const FIPER_REGS_OFFSET: c_uint = 0xd0;
pub const ETTS_REGS_OFFSET: c_uint = 0xe0;
// Bit definitions for the TMR_CTRL register

// Bit definitions for the TMR_TEVENT register

// Bit definitions for the TMR_TEMASK register

// Bit definitions for the TMR_PEVENT register

// Bit definitions for the TMR_PEMASK register

// Bit definitions for the TMR_STAT register

// Bit definitions for the TMR_PRSC register

pub const N_EXT_TS: c_int = 2;
pub const DEFAULT_CKSEL: c_int = 1;
pub const DEFAULT_TMR_PRSC: c_int = 2;
pub const DEFAULT_FIPER1_PERIOD: c_int = 1000000000;
pub const DEFAULT_FIPER2_PERIOD: c_int = 1000000000;
pub const DEFAULT_FIPER3_PERIOD: c_int = 1000000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_qoriq {
    pub base: *mut void __iomem,
    pub regs: ptp_qoriq_registers,
    pub /: *mut *mut spinlock_t lock; / protects regs,
    pub clock: *mut ptp_clock,
    pub caps: ptp_clock_info,
    pub rsrc: *mut resource,
    pub dev: *mut device,
    pub extts_fifo_support: bool,
    pub fiper3_support: bool,
    pub etsec: bool,
    pub irq: c_int,
    pub phc_index: c_int,
    pub /: *mut *mut u32 tclk_period; / nanoseconds,
    pub tmr_prsc: u32,
    pub tmr_add: u32,
    pub cksel: u32,
    pub tmr_fiper1: u32,
    pub tmr_fiper2: u32,
    pub tmr_fiper3: u32,
    pub addr): *mut *mut u32 (read)(unsigned __iomem,
    pub val): *mut *mut *mut void (write)(unsigned __iomem addr, u32,
}

extern "C" {
    pub fn ioread32be(_arg: addr) -> return;
}
extern "C" {
    pub fn ioread32(_arg: addr) -> return;
}
extern "C" {
    pub fn ptp_qoriq_isr(irq: c_int, priv: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ptp_qoriq_free(ptp_qoriq: *mut ptp_qoriq);
}
extern "C" {
    pub fn ptp_qoriq_adjfine(ptp: *mut ptp_clock_info, scaled_ppm: c_long) -> c_int;
}
extern "C" {
    pub fn ptp_qoriq_adjtime(ptp: *mut ptp_clock_info, delta: i64) -> c_int;
}
extern "C" {
    pub fn ptp_qoriq_gettime(ptp: *mut ptp_clock_info, ts: *mut timespec64) -> c_int;
}
extern "C" {
    pub fn extts_clean_up(ptp_qoriq: *mut ptp_qoriq, index: c_int, update_event: bool) -> c_int;
}
