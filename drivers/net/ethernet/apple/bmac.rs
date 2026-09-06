//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apple/bmac.h
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
// mace.h - definitions for the registers in the "Big Mac"
// Ethernet controller found in PowerMac G3 models.
//
// Copyright (C) 1998 Randy Gobbel.
//
// The "Big MAC" appears to have some parts in common with the Sun "Happy Meal"
// (HME) controller.  See sunhme.h
//
// register offsets
// global status and control
pub const XIFC: c_uint = 0x000   /* low-level interface control */;

pub const TXFIFOCSR: c_uint = 0x100   /* transmit FIFO control */;

pub const TXTH: c_uint = 0x110   /* transmit threshold */;

pub const RXFIFOCSR: c_uint = 0x120   /* receive FIFO control */;

pub const MEMADD: c_uint = 0x130   /* memory address, unknown function */;
pub const MEMDATAHI: c_uint = 0x140   /* memory data high, presently unused in driver */;
pub const MEMDATALO: c_uint = 0x150   /* memory data low, presently unused in driver */;
pub const XCVRIF: c_uint = 0x160   /* transceiver interface control */;

pub const CHIPID: c_uint = 0x170   /* chip ID */;
pub const MIFCSR: c_uint = 0x180   /* ??? */;
pub const SROMCSR: c_uint = 0x190   /* SROM control */;

pub const TXPNTR: c_uint = 0x1a0   /* transmit pointer */;
pub const RXPNTR: c_uint = 0x1b0   /* receive pointer */;
pub const STATUS: c_uint = 0x200   /* status--reading this clears it */;
pub const INTDISABLE: c_uint = 0x210   /* interrupt enable/disable control */;
// bits below are the same in both STATUS and INTDISABLE registers

// #	define	NormalIntEvents	~(FrameReceived | FrameSent | TxUnderrun)

// transmit control
pub const TXRST: c_uint = 0x420   /* transmit reset */;

pub const TXCFG: c_uint = 0x430   /* transmit configuration control*/;

pub const IPG1: c_uint = 0x440   /* Inter-packet gap 1 */;
pub const IPG2: c_uint = 0x450   /* Inter-packet gap 2 */;
pub const ALIMIT: c_uint = 0x460   /* Transmit attempt limit */;
pub const SLOT: c_uint = 0x470   /* Transmit slot time */;
pub const PALEN: c_uint = 0x480   /* Size of transmit preamble */;
pub const PAPAT: c_uint = 0x490   /* Pattern for transmit preamble */;
pub const TXSFD: c_uint = 0x4a0   /* Transmit frame delimiter */;
pub const JAM: c_uint = 0x4b0   /* Jam size */;
pub const TXMAX: c_uint = 0x4c0   /* Transmit max pkt size */;
pub const TXMIN: c_uint = 0x4d0   /* Transmit min pkt size */;
pub const PAREG: c_uint = 0x4e0   /* Count of transmit peak attempts */;
pub const DCNT: c_uint = 0x4f0   /* Transmit defer timer */;
pub const NCCNT: c_uint = 0x500   /* Transmit normal-collision counter */;
pub const NTCNT: c_uint = 0x510   /* Transmit first-collision counter */;
pub const EXCNT: c_uint = 0x520   /* Transmit excess-collision counter */;
pub const LTCNT: c_uint = 0x530   /* Transmit late-collision counter */;
pub const RSEED: c_uint = 0x540   /* Transmit random number seed */;
pub const TXSM: c_uint = 0x550   /* Transmit state machine */;
// receive control
pub const RXRST: c_uint = 0x620   /* receive reset */;

pub const RXCFG: c_uint = 0x630   /* receive configuration control */;

pub const RXMAX: c_uint = 0x640   /* Max receive packet size */;
pub const RXMIN: c_uint = 0x650   /* Min receive packet size */;
pub const MADD2: c_uint = 0x660   /* our enet address, high part */;
pub const MADD1: c_uint = 0x670   /* our enet address, middle part */;
pub const MADD0: c_uint = 0x680   /* our enet address, low part */;
pub const FRCNT: c_uint = 0x690   /* receive frame counter */;
pub const LECNT: c_uint = 0x6a0   /* Receive excess length error counter */;
pub const AECNT: c_uint = 0x6b0   /* Receive misaligned error counter */;
pub const FECNT: c_uint = 0x6c0   /* Receive CRC error counter */;
pub const RXSM: c_uint = 0x6d0   /* Receive state machine */;
pub const RXCV: c_uint = 0x6e0   /* Receive code violation */;
pub const BHASH3: c_uint = 0x700   /* multicast hash register */;
pub const BHASH2: c_uint = 0x710   /* multicast hash register */;
pub const BHASH1: c_uint = 0x720   /* multicast hash register */;
pub const BHASH0: c_uint = 0x730   /* multicast hash register */;
pub const AFR2: c_uint = 0x740   /* address filtering setup? */;
pub const AFR1: c_uint = 0x750   /* address filtering setup? */;
pub const AFR0: c_uint = 0x760   /* address filtering setup? */;
pub const AFCR: c_uint = 0x770   /* address filter compare register? */;

// bits in XIFC
