//! Automatically rewritten from C Header to Rust Module
//! Source: sound/arm/aaci.h
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
// linux/sound/arm/aaci.c - ARM PrimeCell AACI PL041 driver
//
// Copyright (C) 2003 Deep Blue Solutions, Ltd, All Rights Reserved.
//
// Control and status register offsets
// P39.
//
pub const AACI_CSCH1: c_uint = 0x000;
pub const AACI_CSCH2: c_uint = 0x014;
pub const AACI_CSCH3: c_uint = 0x028;
pub const AACI_CSCH4: c_uint = 0x03c;
pub const AACI_RXCR: c_uint = 0x000	/* 29 bits Control Rx FIFO */;
pub const AACI_TXCR: c_uint = 0x004	/* 17 bits Control Tx FIFO */;
pub const AACI_SR: c_uint = 0x008	/* 12 bits Status */;
pub const AACI_ISR: c_uint = 0x00c	/* 7 bits  Int Status */;
pub const AACI_IE: c_uint = 0x010	/* 7 bits  Int Enable */;
//
// Other registers
//
pub const AACI_SL1RX: c_uint = 0x050;
pub const AACI_SL1TX: c_uint = 0x054;
pub const AACI_SL2RX: c_uint = 0x058;
pub const AACI_SL2TX: c_uint = 0x05c;
pub const AACI_SL12RX: c_uint = 0x060;
pub const AACI_SL12TX: c_uint = 0x064;
pub const AACI_SLFR: c_uint = 0x068	/* slot flags */;
pub const AACI_SLISTAT: c_uint = 0x06c	/* slot interrupt status */;
pub const AACI_SLIEN: c_uint = 0x070	/* slot interrupt enable */;
pub const AACI_INTCLR: c_uint = 0x074	/* interrupt clear */;
pub const AACI_MAINCR: c_uint = 0x078	/* main control */;
pub const AACI_RESET: c_uint = 0x07c	/* reset control */;
pub const AACI_SYNC: c_uint = 0x080	/* sync control */;
pub const AACI_ALLINTS: c_uint = 0x084	/* all fifo interrupt status */;
pub const AACI_MAINFR: c_uint = 0x088	/* main flag register */;
pub const AACI_DR1: c_uint = 0x090	/* data read/written fifo 1 */;
pub const AACI_DR2: c_uint = 0x0b0	/* data read/written fifo 2 */;
pub const AACI_DR3: c_uint = 0x0d0	/* data read/written fifo 3 */;
pub const AACI_DR4: c_uint = 0x0f0	/* data read/written fifo 4 */;
//
// TX/RX fifo control register (CR). P48
//

//
// status register bits. P49
//

//
// interrupt status register bits.
//

//
// interrupt enable register bits.
//

//
// interrupt status. P51
//

//
// interrupt enable. P52
//

//
// slot flag register bits. P56
//

//
// Interrupt clear register.
//

//
// Main control register bits. P62
//

//
// Reset register bits. P65
//

//
// Sync register bits. P65
//

//
// Main flag register bits. P66
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aaci_runtime {
    pub base: *mut void __iomem,
    pub fifo: *mut void __iomem,
    pub lock: spinlock_t,
    pub pcm: *mut ac97_pcm,
    pub pcm_open: c_int,
    pub cr: u32,
    pub substream: *mut snd_pcm_substream,
    pub /: *mut *mut unsigned int period; / byte size of a "period",
//
// PIO support
//
    pub start: *mut c_void,
    pub end: *mut c_void,
    pub ptr: *mut c_void,
    pub bytes: c_int,
    pub fifo_bytes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aaci {
    pub dev: *mut amba_device,
    pub card: *mut snd_card,
    pub base: *mut void __iomem,
    pub fifo_depth: c_uint,
    pub users: c_uint,
    pub irq_lock: mutex,
// AC'97
    pub ac97_sem: mutex,
    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub maincr: u32,
    pub playback: aaci_runtime,
    pub capture: aaci_runtime,
    pub pcm: *mut snd_pcm,
}

pub const ACSTREAM_FRONT: c_int = 0;
pub const ACSTREAM_SURROUND: c_int = 1;
pub const ACSTREAM_LFE: c_int = 2;
