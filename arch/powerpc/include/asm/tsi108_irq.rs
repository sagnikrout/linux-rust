//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/tsi108_irq.h
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
// (C) Copyright 2005 Tundra Semiconductor Corp.
// Alex Bounine, <alexandreb at tundra.com).
//
// See file CREDITS for list of people who contributed to this
// project.
//
// definitions for interrupt controller initialization and external interrupt
// demultiplexing on TSI108EMU/SVB boards.
//
// Tsi108 interrupts
//

pub const TSI108_IRQ_REG_BASE: c_int = 0;

pub const MAX_TASK_PRIO: c_uint = 0xF;

// Interrupt vectors assignment to external and internal
// sources of requests.
// EXTERNAL INTERRUPT SOURCES

// INTERNAL INTERRUPT SOURCES

//
// PCI bus INTA# - INTD# lines demultiplexor
//

// number of entries in vector dispatch table

// Mapping of MPIC outputs to processors' interrupt pins
pub const IDIR_INT_OUT0: c_uint = 0x1;
pub const IDIR_INT_OUT1: c_uint = 0x2;
pub const IDIR_INT_OUT2: c_uint = 0x4;
pub const IDIR_INT_OUT3: c_uint = 0x8;
// ---------------------------------------------------------------
// IRQ line configuration parameters
// Interrupt delivery modes
