//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/abx500/ab8500-sysctrl.h
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
// Copyright (C) ST-Ericsson SA 2010
// Author: Mattias Nilsson <mattias.i.nilsson@stericsson.com> for ST Ericsson.
//

extern "C" {
    pub fn ab8500_sysctrl_read(reg: u16, value: *mut u8) -> c_int;
}
extern "C" {
    pub fn ab8500_sysctrl_write(reg: u16, mask: u8, value: u8) -> c_int;
}

extern "C" {
    pub fn ab8500_sysctrl_write(_arg: reg, _arg: bits, _arg: bits) -> return;
}
extern "C" {
    pub fn ab8500_sysctrl_write(_arg: reg, _arg: bits, _arg: 0) -> return;
}
// Registers
pub const AB8500_TURNONSTATUS: c_uint = 0x100;
pub const AB8500_RESETSTATUS: c_uint = 0x101;
pub const AB8500_PONKEY1PRESSSTATUS: c_uint = 0x102;
pub const AB8500_SYSCLKREQSTATUS: c_uint = 0x142;
pub const AB8500_STW4500CTRL1: c_uint = 0x180;
pub const AB8500_STW4500CTRL2: c_uint = 0x181;
pub const AB8500_STW4500CTRL3: c_uint = 0x200;
pub const AB8500_MAINWDOGCTRL: c_uint = 0x201;
pub const AB8500_MAINWDOGTIMER: c_uint = 0x202;
pub const AB8500_LOWBAT: c_uint = 0x203;
pub const AB8500_BATTOK: c_uint = 0x204;
pub const AB8500_SYSCLKTIMER: c_uint = 0x205;
pub const AB8500_SMPSCLKCTRL: c_uint = 0x206;
pub const AB8500_SMPSCLKSEL1: c_uint = 0x207;
pub const AB8500_SMPSCLKSEL2: c_uint = 0x208;
pub const AB8500_SMPSCLKSEL3: c_uint = 0x209;
pub const AB8500_SYSULPCLKCONF: c_uint = 0x20A;
pub const AB8500_SYSULPCLKCTRL1: c_uint = 0x20B;
pub const AB8500_SYSCLKCTRL: c_uint = 0x20C;
pub const AB8500_SYSCLKREQ1VALID: c_uint = 0x20D;
pub const AB8500_SYSTEMCTRLSUP: c_uint = 0x20F;
pub const AB8500_SYSCLKREQ1RFCLKBUF: c_uint = 0x210;
pub const AB8500_SYSCLKREQ2RFCLKBUF: c_uint = 0x211;
pub const AB8500_SYSCLKREQ3RFCLKBUF: c_uint = 0x212;
pub const AB8500_SYSCLKREQ4RFCLKBUF: c_uint = 0x213;
pub const AB8500_SYSCLKREQ5RFCLKBUF: c_uint = 0x214;
pub const AB8500_SYSCLKREQ6RFCLKBUF: c_uint = 0x215;
pub const AB8500_SYSCLKREQ7RFCLKBUF: c_uint = 0x216;
pub const AB8500_SYSCLKREQ8RFCLKBUF: c_uint = 0x217;
pub const AB8500_DITHERCLKCTRL: c_uint = 0x220;
pub const AB8500_SWATCTRL: c_uint = 0x230;
pub const AB8500_HIQCLKCTRL: c_uint = 0x232;
pub const AB8500_VSIMSYSCLKCTRL: c_uint = 0x233;
pub const AB9540_SYSCLK12BUFCTRL: c_uint = 0x234;
pub const AB9540_SYSCLK12CONFCTRL: c_uint = 0x235;
pub const AB9540_SYSCLK12BUFCTRL2: c_uint = 0x236;
pub const AB9540_SYSCLK12BUF1VALID: c_uint = 0x237;
pub const AB9540_SYSCLK12BUF2VALID: c_uint = 0x238;
pub const AB9540_SYSCLK12BUF3VALID: c_uint = 0x239;
pub const AB9540_SYSCLK12BUF4VALID: c_uint = 0x23A;
// Bits

pub const AB8500_PONKEY1PRESSSTATUS_PONKEY1PRESSTIME_MASK: c_uint = 0x7F;
pub const AB8500_PONKEY1PRESSSTATUS_PONKEY1PRESSTIME_SHIFT: c_int = 0;

pub const AB8500_MAINWDOGTIMER_MAINWDOGTIMER_MASK: c_uint = 0x7F;
pub const AB8500_MAINWDOGTIMER_MAINWDOGTIMER_SHIFT: c_int = 0;

pub const AB8500_LOWBAT_LOWBAT_MASK: c_uint = 0x7E;
pub const AB8500_LOWBAT_LOWBAT_SHIFT: c_int = 1;
pub const AB8500_BATTOK_BATTOKSEL0THF_MASK: c_uint = 0x0F;
pub const AB8500_BATTOK_BATTOKSEL0THF_SHIFT: c_int = 0;
pub const AB8500_BATTOK_BATTOKSEL1THF_MASK: c_uint = 0xF0;
pub const AB8500_BATTOK_BATTOKSEL1THF_SHIFT: c_int = 4;
pub const AB8500_SYSCLKTIMER_SYSCLKTIMER_MASK: c_uint = 0x0F;
pub const AB8500_SYSCLKTIMER_SYSCLKTIMER_SHIFT: c_int = 0;
pub const AB8500_SYSCLKTIMER_SYSCLKTIMERADJ_MASK: c_uint = 0xF0;
pub const AB8500_SYSCLKTIMER_SYSCLKTIMERADJ_SHIFT: c_int = 4;
pub const AB8500_SMPSCLKCTRL_SMPSCLKINTSEL_MASK: c_uint = 0x03;
pub const AB8500_SMPSCLKCTRL_SMPSCLKINTSEL_SHIFT: c_int = 0;

pub const AB8500_SMPSCLKSEL1_VARMCLKSEL_MASK: c_uint = 0x07;
pub const AB8500_SMPSCLKSEL1_VARMCLKSEL_SHIFT: c_int = 0;
pub const AB8500_SMPSCLKSEL1_VAPECLKSEL_MASK: c_uint = 0x38;
pub const AB8500_SMPSCLKSEL1_VAPECLKSEL_SHIFT: c_int = 3;
pub const AB8500_SMPSCLKSEL2_VMODCLKSEL_MASK: c_uint = 0x07;
pub const AB8500_SMPSCLKSEL2_VMODCLKSEL_SHIFT: c_int = 0;
pub const AB8500_SMPSCLKSEL2_VSMPS1CLKSEL_MASK: c_uint = 0x38;
pub const AB8500_SMPSCLKSEL2_VSMPS1CLKSEL_SHIFT: c_int = 3;
pub const AB8500_SMPSCLKSEL3_VSMPS2CLKSEL_MASK: c_uint = 0x07;
pub const AB8500_SMPSCLKSEL3_VSMPS2CLKSEL_SHIFT: c_int = 0;
pub const AB8500_SMPSCLKSEL3_VSMPS3CLKSEL_MASK: c_uint = 0x38;
pub const AB8500_SMPSCLKSEL3_VSMPS3CLKSEL_SHIFT: c_int = 3;
pub const AB8500_SYSULPCLKCONF_ULPCLKCONF_MASK: c_uint = 0x03;
pub const AB8500_SYSULPCLKCONF_ULPCLKCONF_SHIFT: c_int = 0;

pub const AB8500_SYSULPCLKCTRL1_SYSULPCLKINTSEL_MASK: c_uint = 0x03;
pub const AB8500_SYSULPCLKCTRL1_SYSULPCLKINTSEL_SHIFT: c_int = 0;

pub const AB8500_SYSTEMCTRLSUP_EXTSUP12LPNCLKSEL_MASK: c_uint = 0x03;
pub const AB8500_SYSTEMCTRLSUP_EXTSUP12LPNCLKSEL_SHIFT: c_int = 0;
pub const AB8500_SYSTEMCTRLSUP_EXTSUP3LPNCLKSEL_MASK: c_uint = 0x0C;
pub const AB8500_SYSTEMCTRLSUP_EXTSUP3LPNCLKSEL_SHIFT: c_int = 2;

pub const AB8500_DITHERCLKCTRL_DITHERDEL_MASK: c_uint = 0xC0;
pub const AB8500_DITHERCLKCTRL_DITHERDEL_SHIFT: c_int = 6;

pub const AB8500_SWATCTRL_RFOFFTIMER_MASK: c_uint = 0x1C;
pub const AB8500_SWATCTRL_RFOFFTIMER_SHIFT: c_int = 2;

pub const AB9540_SYSCLK12BUFCTRL_SYSCLK12BUFENA_MASK: c_uint = 0x0F;

pub const AB9540_SYSCLK12BUFCTRL_SYSCLK12BUFSTRE_MASK: c_uint = 0xF0;

pub const AB9540_SYSCLK12BUF1VALID_SYSCLK12BUF1VALID_MASK: c_uint = 0xFF;
pub const AB9540_SYSCLK12BUF1VALID_SYSCLK12BUF1VALID_SHIFT: c_int = 0;
pub const AB9540_SYSCLK12BUF2VALID_SYSCLK12BUF2VALID_MASK: c_uint = 0xFF;
pub const AB9540_SYSCLK12BUF2VALID_SYSCLK12BUF2VALID_SHIFT: c_int = 0;
pub const AB9540_SYSCLK12BUF3VALID_SYSCLK12BUF3VALID_MASK: c_uint = 0xFF;
pub const AB9540_SYSCLK12BUF3VALID_SYSCLK12BUF3VALID_SHIFT: c_int = 0;
pub const AB9540_SYSCLK12BUF4VALID_SYSCLK12BUF4VALID_MASK: c_uint = 0xFF;
pub const AB9540_SYSCLK12BUF4VALID_SYSCLK12BUF4VALID_SHIFT: c_int = 0;
pub const AB8500_ENABLE_WD: c_uint = 0x1;
pub const AB8500_KICK_WD: c_uint = 0x2;
pub const AB8500_WD_RESTART_ON_EXPIRE: c_uint = 0x10;
