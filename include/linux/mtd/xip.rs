//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/xip.h
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
// MTD primitives for XIP support
//
// Author:	Nicolas Pitre
// Created:	Nov 2, 2004
// Copyright:	(C) 2004 MontaVista Software, Inc.
//
// This XIP support for MTD has been loosely inspired
// by an earlier patch authored by David Woodhouse.
//

//
// We really don't want gcc to guess anything.
// We absolutely _need_ proper inlining.
//

//
// Function that are modifying the flash state away from array mode must
// obviously not be running from flash.  The __xipram is therefore marking
// those functions so they get relocated to ram.
//

//
// Each architecture has to provide the following macros.  They must access
// the hardware directly and not rely on any other (XIP) functions since they
// won't be available when used (flash not in array mode).
//
// xip_irqpending()
//
// return non zero when any hardware interrupt is pending.
//
// xip_currtime()
//
// return a platform specific time reference to be used with
// xip_elapsed_since().
//
// xip_elapsed_since(x)
//
// return in usecs the elapsed timebetween now and the reference x as
// returned by xip_currtime().
//
// note 1: conversion to usec can be approximated, as long as the
// returned value is <= the real elapsed time.
// note 2: this should be able to cope with a few seconds without
// overflowing.
//
// xip_iprefetch()
//
// Macro to fill instruction prefetch
// e.g. a series of nops:  asm volatile (".rep 8; nop; .endr");
//

//
// xip_cpu_idle() is used when waiting for a delay equal or larger than
// the system timer tick period.  This should put the CPU into idle mode
// to save power and to be woken up only when some interrupts are pending.
// This should not rely upon standard kernel code.
//

// Macro flag: #define __xipram

