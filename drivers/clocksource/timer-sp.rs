//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clocksource/timer-sp.h
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
// ARM timer implementation, found in Integrator, Versatile and Realview
// platforms.  Not all platforms support all registers and bits in these
// registers, so we mark them with A for Integrator AP, C for Integrator
// CP, V for Versatile and R for Realview.
//
// Integrator AP has 16-bit timers, Integrator CP, Versatile and Realview
// can have 16-bit or 32-bit selectable via a bit in the control register.
//
// Every SP804 contains two identical timers.
//
pub const NR_TIMERS: c_int = 2;
pub const TIMER_1_BASE: c_uint = 0x00;
pub const TIMER_2_BASE: c_uint = 0x20;
pub const TIMER_LOAD: c_uint = 0x00			/* ACVR rw */;
pub const TIMER_VALUE: c_uint = 0x04			/* ACVR ro */;
pub const TIMER_CTRL: c_uint = 0x08			/* ACVR rw */;

pub const TIMER_INTCLR: c_uint = 0x0c			/* ACVR wo */;
pub const TIMER_RIS: c_uint = 0x10			/*  CVR ro */;
pub const TIMER_MIS: c_uint = 0x14			/*  CVR ro */;
pub const TIMER_BGLOAD: c_uint = 0x18			/*  CVR rw */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp804_timer {
    pub load: c_int,
    pub load_h: c_int,
    pub value: c_int,
    pub value_h: c_int,
    pub ctrl: c_int,
    pub intclr: c_int,
    pub ris: c_int,
    pub mis: c_int,
    pub bgload: c_int,
    pub bgload_h: c_int,
    pub timer_base: [c_int; NR_TIMERS],
    pub width: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp804_clkevt {
    pub base: *mut void __iomem,
    pub load: *mut void __iomem,
    pub load_h: *mut void __iomem,
    pub value: *mut void __iomem,
    pub value_h: *mut void __iomem,
    pub ctrl: *mut void __iomem,
    pub intclr: *mut void __iomem,
    pub ris: *mut void __iomem,
    pub mis: *mut void __iomem,
    pub bgload: *mut void __iomem,
    pub bgload_h: *mut void __iomem,
    pub reload: c_ulong,
    pub width: c_int,
}
