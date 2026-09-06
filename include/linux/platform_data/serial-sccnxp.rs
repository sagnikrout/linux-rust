//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/serial-sccnxp.h
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
// NXP (Philips) SCC+++(SCN+++) serial driver
//
// Copyright (C) 2012 Alexander Shiyan <shc_work@mail.ru>
//
// Based on sc26xx.c, by Thomas Bogendörfer (tsbogend@alpha.franken.de)
//
pub const SCCNXP_MAX_UARTS: c_int = 2;
// Output lines
pub const LINE_OP0: c_int = 1;
pub const LINE_OP1: c_int = 2;
pub const LINE_OP2: c_int = 3;
pub const LINE_OP3: c_int = 4;
pub const LINE_OP4: c_int = 5;
pub const LINE_OP5: c_int = 6;
pub const LINE_OP6: c_int = 7;
pub const LINE_OP7: c_int = 8;
// Input lines
pub const LINE_IP0: c_int = 9;
pub const LINE_IP1: c_int = 10;
pub const LINE_IP2: c_int = 11;
pub const LINE_IP3: c_int = 12;
pub const LINE_IP4: c_int = 13;
pub const LINE_IP5: c_int = 14;
pub const LINE_IP6: c_int = 15;
// Signals

// Goes high when transmit,
// then goes low.
//
// Routing control signal 'sig' to line 'line'

//
// Example board initialization data:
//
// static struct resource sc2892_resources[] = {
// DEFINE_RES_MEM(UART_PHYS_START, 0x10),
// DEFINE_RES_IRQ(IRQ_EXT2),
// };
//
// static struct sccnxp_pdata sc2892_info = {
// .mctrl_cfg[0]	= MCTRL_SIG(DIR_OP, LINE_OP0),
// .mctrl_cfg[1]	= MCTRL_SIG(DIR_OP, LINE_OP1),
// };
//
// static struct platform_device sc2892 = {
// .name		= "sc2892",
// .id		= -1,
// .resource	= sc2892_resources,
// .num_resources	= ARRAY_SIZE(sc2892_resources),
// .dev = {
// .platform_data	= &sc2892_info,
// },
// };
//
// SCCNXP platform data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sccnxp_pdata {
// Shift for A0 line
    pub reg_shift: u8,
// Modem control lines configuration
    pub mctrl_cfg: [u32; SCCNXP_MAX_UARTS],
// Timer value for polling mode (usecs)
    pub poll_time_us: c_uint,
}
