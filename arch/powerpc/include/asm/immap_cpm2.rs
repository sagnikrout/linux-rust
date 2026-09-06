//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/immap_cpm2.h
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
// CPM2 Internal Memory Map
// Copyright (c) 1999 Dan Malek (dmalek@jlc.net)
//
// The Internal Memory Map for devices with CPM2 on them.  This
// is the superset of all CPM2 devices (8260, 8266, 8280, 8272,
// 8560).
//

// System configuration registers.
//
// Memory controller registers.
//
// System Integration Timers.
//

// PCI Controller.
//
// Interrupt Controller.
//
// Clocks and Reset.
//
// Input/Output Port control/status registers.
// Names consistent with processor manual, although they are different
// from the original 8xx names.......
//
// Communication Processor Module Timers
//
// DMA control/status registers.
//
// Fast controllers
//
// Fast controllers continued
//
// TC Layer
//
// I2C
//
// Serial Peripheral Interface.
//
// CPM Mux.
//
// SIRAM control
//
// USB Controller.
//
// ...and the whole thing wrapped up....
//
// Some references are into the unique and known dpram spaces,
// others are from the generic base.
//

// First set of baud rate generators.
//
// Second set of baud rate generators.
//

