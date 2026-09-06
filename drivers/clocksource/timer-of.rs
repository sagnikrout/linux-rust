//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clocksource/timer-of.h
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

pub const TIMER_OF_BASE: c_uint = 0x1;
pub const TIMER_OF_CLOCK: c_uint = 0x2;
pub const TIMER_OF_IRQ: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_timer_irq {
    pub irq: c_int,
    pub index: c_int,
    pub name: *const c_char,
    pub flags: c_ulong,
    pub handler: irq_handler_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_timer_base {
    pub base: *mut void __iomem,
    pub name: *const c_char,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_timer_clk {
    pub clk: *mut clk,
    pub name: *const c_char,
    pub index: c_int,
    pub rate: c_ulong,
    pub period: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_of {
    pub flags: c_uint,
    pub np: *mut device_node,
    pub clkevt: clock_event_device,
    pub of_base: of_timer_base,
    pub of_irq: of_timer_irq,
    pub of_clk: of_timer_clk,
    pub private_data: *mut c_void,
}

extern "C" {
    pub fn container_of(_arg: clkevt, timer_of: struct, _arg: clkevt) -> return;
}
extern "C" {
    pub fn timer_of_init(np: *mut device_node, to: *mut timer_of) -> c_int;
}
extern "C" {
    pub fn timer_of_cleanup(to: *mut timer_of);
}
