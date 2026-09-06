//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/aw2/saa7146.h
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
// Copyright (C) 2008 Cedric Bregardis <cedric.bregardis@free.fr> and
// Jean-Christian Hassler <jhassler@free.fr>
//
// This file is part of the Audiowerk2 ALSA driver
//
// SAA7146 registers
pub const PCI_BT_A: c_uint = 0x4C;
pub const IICTFR: c_uint = 0x8C;
pub const IICSTA: c_uint = 0x90;
pub const BaseA1_in: c_uint = 0x94;
pub const ProtA1_in: c_uint = 0x98;
pub const PageA1_in: c_uint = 0x9C;
pub const BaseA1_out: c_uint = 0xA0;
pub const ProtA1_out: c_uint = 0xA4;
pub const PageA1_out: c_uint = 0xA8;
pub const BaseA2_in: c_uint = 0xAC;
pub const ProtA2_in: c_uint = 0xB0;
pub const PageA2_in: c_uint = 0xB4;
pub const BaseA2_out: c_uint = 0xB8;
pub const ProtA2_out: c_uint = 0xBC;
pub const PageA2_out: c_uint = 0xC0;
pub const IER: c_uint = 0xDC;
pub const GPIO_CTRL: c_uint = 0xE0;
pub const ACON1: c_uint = 0xF4;
pub const ACON2: c_uint = 0xF8;
pub const MC1: c_uint = 0xFC;
pub const MC2: c_uint = 0x100;
pub const ISR: c_uint = 0x10C;
pub const PSR: c_uint = 0x110;
pub const SSR: c_uint = 0x114;
pub const PCI_ADP1: c_uint = 0x12C;
pub const PCI_ADP2: c_uint = 0x130;
pub const PCI_ADP3: c_uint = 0x134;
pub const PCI_ADP4: c_uint = 0x138;
pub const LEVEL_REP: c_uint = 0x140;
pub const FB_BUFFER1: c_uint = 0x144;
pub const FB_BUFFER2: c_uint = 0x148;
pub const TSL1: c_uint = 0x180;
pub const TSL2: c_uint = 0x1C0;

// PSR/ISR/IER

// SSR

// PCI_BT_A

// MC1

// MC2

// ACON1

// ACON2

// IICSTA

// IICTFR

pub const START: c_int = 3;
pub const CONT: c_int = 2;
pub const STOP: c_int = 1;
pub const NOP: c_int = 0;
