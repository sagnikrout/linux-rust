//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/at91/atmel_tcb.h
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


//
// Timer/Counter Unit (TC) registers.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//

//
// Many 32-bit Atmel SOCs include one or more TC blocks, each of which holds
// three general-purpose 16-bit timers.  These timers share one register bank.
// Depending on the SOC, each timer may have its own clock and IRQ, or those
// may be shared by the whole TC block.
//
// These TC blocks may have up to nine external pins:  TCLK0..2 signals for
// clocks or clock gates, and per-timer TIOA and TIOB signals used for PWM
// or triggering.  Those pins need to be set up for use with the TC block,
// else they will be used as GPIOs or for a different controller.
//
// Although we expect each TC block to have a platform_device node, those
// nodes are not what drivers bind to.  Instead, they ask for a specific
// TC block, by number ... which is a common approach on systems with many
// timers.  Then they use clk_get() and platform_get_irq() to get clock and
// IRQ resources.
//
// struct atmel_tcb_config - SoC data for a Timer/Counter Block
// @counter_width: size in bits of a timer counter register
// @has_gclk: boolean indicating if a timer counter has a generic clock
// @has_qdec: boolean indicating if a timer counter has a quadrature
// decoder.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tcb_config {
    pub counter_width: usize,
    pub has_gclk: bool,
    pub has_qdec: bool,
}

//
// struct atmel_tc - information about a Timer/Counter Block
// @pdev: physical device
// @regs: mapping through which the I/O registers can be accessed
// @id: block id
// @tcb_config: configuration data from SoC
// @irq: irq for each of the three channels
// @clk: internal clock source for each of the three channels
// @node: list node, for tclib internal use
// @allocated: if already used, for tclib internal use
//
// On some platforms, each TC channel has its own clocks and IRQs,
// while on others, all TC channels share the same clock and IRQ.
// Drivers should clk_enable() all the clocks they need even though
// all the entries in @clk may point to the same physical clock.
// Likewise, drivers should request irqs independently for each
// channel, but they must use IRQF_SHARED in case some of the entries
// in @irq are actually the same IRQ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_tc {
    pub pdev: *mut platform_device,
    pub regs: *mut void __iomem,
    pub id: c_int,
    pub tcb_config: *const atmel_tcb_config,
    pub irq: [c_int; 3],
    pub clk: [*mut clk; 3],
    pub slow_clk: *mut clk,
    pub node: list_head,
    pub allocated: bool,
}

// platform-specific ATMEL_TC_TIMER_CLOCKx divisors (0 means 32KiHz)
//
// Two registers have block-wide controls.  These are: configuring the three
// "external" clocks (or event sources) used by the timer channels; and
// synchronizing the timers by resetting them all at once.
//
// "External" can mean "external to chip" using the TCLK0, TCLK1, or TCLK2
// signals.  Or, it can mean "external to timer", using the TIOA output from
// one of the other two timers that's being run in waveform mode.
//
pub const ATMEL_TC_BCR: c_uint = 0xc0		/* TC Block Control Register */;

pub const ATMEL_TC_BMR: c_uint = 0xc4		/* TC Block Mode Register */;

//
// Each TC block has three "channels", each with one counter and controls.
//
// Note that the semantics of ATMEL_TC_TIMER_CLOCKx (input clock selection
// when it's not "external") is silicon-specific.  AT91 platforms use one
// set of definitions; AVR32 platforms use a different set.  Don't hard-wire
// such knowledge into your code, use the global "atmel_tc_divisors" ...
// where index N is the divisor for clock N+1, else zero to indicate it uses
// the 32 KiHz clock.
//
// The timers can be chained in various ways, and operated in "waveform"
// generation mode (including PWM) or "capture" mode (to time events).  In
// both modes, behavior can be configured in many ways.
//
// Each timer has two I/O pins, TIOA and TIOB.  Waveform mode uses TIOA as a
// PWM output, and TIOB as either another PWM or as a trigger.  Capture mode
// uses them only as inputs.
//

pub const ATMEL_TC_CCR: c_uint = 0x00		/* Channel Control Register */;

pub const ATMEL_TC_CMR: c_uint = 0x04		/* Channel Mode Register */;
// Both modes share some CMR bits

// CAPTURE mode CMR bits

// WAVEFORM mode CMR bits

pub const ATMEL_TC_CV: c_uint = 0x10		/* counter Value */;
pub const ATMEL_TC_RA: c_uint = 0x14		/* register A */;
pub const ATMEL_TC_RB: c_uint = 0x18		/* register B */;
pub const ATMEL_TC_RC: c_uint = 0x1c		/* register C */;
pub const ATMEL_TC_SR: c_uint = 0x20		/* status (read-only) */;
// Status-only flags

pub const ATMEL_TC_IER: c_uint = 0x24		/* interrupt enable (write-only) */;
pub const ATMEL_TC_IDR: c_uint = 0x28		/* interrupt disable (write-only) */;
pub const ATMEL_TC_IMR: c_uint = 0x2c		/* interrupt mask (read-only) */;
// Status and IRQ flags

// all IRQs
