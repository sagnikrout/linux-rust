//! Automatically rewritten from C Header to Rust Module
//! Source: include/clocksource/arm_arch_timer.h
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
// Copyright (C) 2012 ARM Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arch_timer_reg {
    ARCH_TIMER_REG_CTRL,
    ARCH_TIMER_REG_CVAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arch_timer_ppi_nr {
    ARCH_TIMER_PHYS_SECURE_PPI,
    ARCH_TIMER_PHYS_NONSECURE_PPI,
    ARCH_TIMER_VIRT_PPI,
    ARCH_TIMER_HYP_PPI,
    ARCH_TIMER_HYP_VIRT_PPI,
    ARCH_TIMER_MAX_TIMER_PPI
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arch_timer_spi_nr {
    ARCH_TIMER_PHYS_SPI,
    ARCH_TIMER_VIRT_SPI,
    ARCH_TIMER_MAX_TIMER_SPI
}

pub const ARCH_TIMER_PHYS_ACCESS: c_int = 0;
pub const ARCH_TIMER_VIRT_ACCESS: c_int = 1;
pub const ARCH_TIMER_MEM_MAX_FRAMES: c_int = 8;

pub const ARCH_TIMER_EVT_STREAM_PERIOD_US: c_int = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_timer_kvm_info {
    pub timecounter: timecounter,
    pub virtual_irq: c_int,
    pub physical_irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_timer_mem_frame {
    pub valid: bool,
    pub cntbase: phys_addr_t,
    pub size: usize,
    pub phys_irq: c_int,
    pub virt_irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_timer_mem {
    pub cntctlbase: phys_addr_t,
    pub size: usize,
    pub frame: [arch_timer_mem_frame; ARCH_TIMER_MEM_MAX_FRAMES],
}

extern "C" {
    pub fn arch_timer_get_rate() -> u32;
}
extern "C" {
    pub fn u64(_arg: *mut arch_timer_read_counter)(void) -> extern;
}
extern "C" {
    pub fn arch_timer_evtstrm_available() -> bool;
}

