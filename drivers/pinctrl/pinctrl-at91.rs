//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/pinctrl-at91.h
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
// Copyright (C) 2005 Ivan Kokshaysky
// Copyright (C) SAN People
//
// Parallel I/O Controller (PIO) - System peripherals registers.
//
pub const PIO_PER: c_uint = 0x00	/* Enable Register */;
pub const PIO_PDR: c_uint = 0x04	/* Disable Register */;
pub const PIO_PSR: c_uint = 0x08	/* Status Register */;
pub const PIO_OER: c_uint = 0x10	/* Output Enable Register */;
pub const PIO_ODR: c_uint = 0x14	/* Output Disable Register */;
pub const PIO_OSR: c_uint = 0x18	/* Output Status Register */;
pub const PIO_IFER: c_uint = 0x20	/* Glitch Input Filter Enable */;
pub const PIO_IFDR: c_uint = 0x24	/* Glitch Input Filter Disable */;
pub const PIO_IFSR: c_uint = 0x28	/* Glitch Input Filter Status */;
pub const PIO_SODR: c_uint = 0x30	/* Set Output Data Register */;
pub const PIO_CODR: c_uint = 0x34	/* Clear Output Data Register */;
pub const PIO_ODSR: c_uint = 0x38	/* Output Data Status Register */;
pub const PIO_PDSR: c_uint = 0x3c	/* Pin Data Status Register */;
pub const PIO_IER: c_uint = 0x40	/* Interrupt Enable Register */;
pub const PIO_IDR: c_uint = 0x44	/* Interrupt Disable Register */;
pub const PIO_IMR: c_uint = 0x48	/* Interrupt Mask Register */;
pub const PIO_ISR: c_uint = 0x4c	/* Interrupt Status Register */;
pub const PIO_MDER: c_uint = 0x50	/* Multi-driver Enable Register */;
pub const PIO_MDDR: c_uint = 0x54	/* Multi-driver Disable Register */;
pub const PIO_MDSR: c_uint = 0x58	/* Multi-driver Status Register */;
pub const PIO_PUDR: c_uint = 0x60	/* Pull-up Disable Register */;
pub const PIO_PUER: c_uint = 0x64	/* Pull-up Enable Register */;
pub const PIO_PUSR: c_uint = 0x68	/* Pull-up Status Register */;
pub const PIO_ASR: c_uint = 0x70	/* Peripheral A Select Register */;
pub const PIO_ABCDSR1: c_uint = 0x70	/* Peripheral ABCD Select Register 1 [some sam9 only] */;
pub const PIO_BSR: c_uint = 0x74	/* Peripheral B Select Register */;
pub const PIO_ABCDSR2: c_uint = 0x74	/* Peripheral ABCD Select Register 2 [some sam9 only] */;
pub const PIO_ABSR: c_uint = 0x78	/* AB Status Register */;
pub const PIO_IFSCDR: c_uint = 0x80	/* Input Filter Slow Clock Disable Register */;
pub const PIO_IFSCER: c_uint = 0x84	/* Input Filter Slow Clock Enable Register */;
pub const PIO_IFSCSR: c_uint = 0x88	/* Input Filter Slow Clock Status Register */;
pub const PIO_SCDR: c_uint = 0x8c	/* Slow Clock Divider Debouncing Register */;

pub const PIO_PPDDR: c_uint = 0x90	/* Pad Pull-down Disable Register */;
pub const PIO_PPDER: c_uint = 0x94	/* Pad Pull-down Enable Register */;
pub const PIO_PPDSR: c_uint = 0x98	/* Pad Pull-down Status Register */;
pub const PIO_OWER: c_uint = 0xa0	/* Output Write Enable Register */;
pub const PIO_OWDR: c_uint = 0xa4	/* Output Write Disable Register */;
pub const PIO_OWSR: c_uint = 0xa8	/* Output Write Status Register */;
pub const PIO_AIMER: c_uint = 0xb0	/* Additional Interrupt Modes Enable Register */;
pub const PIO_AIMDR: c_uint = 0xb4	/* Additional Interrupt Modes Disable Register */;
pub const PIO_AIMMR: c_uint = 0xb8	/* Additional Interrupt Modes Mask Register */;
pub const PIO_ESR: c_uint = 0xc0	/* Edge Select Register */;
pub const PIO_LSR: c_uint = 0xc4	/* Level Select Register */;
pub const PIO_ELSR: c_uint = 0xc8	/* Edge/Level Status Register */;
pub const PIO_FELLSR: c_uint = 0xd0	/* Falling Edge/Low Level Select Register */;
pub const PIO_REHLSR: c_uint = 0xd4	/* Rising Edge/ High Level Select Register */;
pub const PIO_FRLHSR: c_uint = 0xd8	/* Fall/Rise - Low/High Status Register */;
pub const PIO_SCHMITT: c_uint = 0x100	/* Schmitt Trigger Register */;
pub const SAMA5D3_PIO_DRIVER1: c_uint = 0x118  /*PIO Driver 1 register offset*/;
pub const SAMA5D3_PIO_DRIVER2: c_uint = 0x11C  /*PIO Driver 2 register offset*/;
pub const AT91SAM9X5_PIO_DRIVER1: c_uint = 0x114  /*PIO Driver 1 register offset*/;
pub const AT91SAM9X5_PIO_DRIVER2: c_uint = 0x118  /*PIO Driver 2 register offset*/;
pub const SAM9X60_PIO_SLEWR: c_uint = 0x110	/* PIO Slew Rate Control Register */;
pub const SAM9X60_PIO_DRIVER1: c_uint = 0x118	/* PIO Driver 1 register offset */;
