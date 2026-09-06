//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/scx200_hrt.c
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
// Copyright (C) 2006 Jim Cromie
//
// This is a clocksource driver for the Geode SCx200's 1 or 27 MHz
// high-resolution timer.  The Geode SC-1100 (at least) has a buggy
// time stamp counter (TSC), which loses time unless 'idle=poll' is
// given as a boot-arg. In its absence, the Generic Timekeeping code
// will detect and de-rate the bad TSC, allowing this timer to take
// over timekeeping duties.
//
// Based on work by John Stultz, and Ted Phelps (in a 2.6.12-rc6 patch)
//

    static int mhz27;
    module_param(mhz27, int, 0);	/* load time only */
    MODULE_PARM_DESC(mhz27, "count at 27.0 MHz (default is 1.0 MHz)");
    static int ppm;
    module_param(ppm, int, 0);	/* load time only */
    MODULE_PARM_DESC(ppm, "+-adjust to actual XO freq (ppm)");
// HiRes Timer configuration register address

// and config settings

// The base timer frequency, * 27 if selected
pub const HRT_FREQ: c_int = 1000000;
#[no_mangle]
unsafe extern "C" fn read_hrt(cs: *mut clocksource) -> u64 {
    static u64 read_hrt(struct clocksource *cs)
    {
// Read the timer value
    return (u64) inl(scx200_cb_base + SCx200_TIMER_OFFSET);
    }
    static struct clocksource cs_hrt = {
    .name		= "scx200_hrt",
    .rating		= 250,
    .read		= read_hrt,
    .mask		= CLOCKSOURCE_MASK(32),
    .flags		= CLOCK_SOURCE_IS_CONTINUOUS,
// mult, shift are set based on mhz27 flag
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn init_hrt_clocksource() -> int __init {
    static int __init init_hrt_clocksource(void)
    {
    u32 freq;
// Make sure scx200 has initialized the configuration block
    if (!scx200_cb_present())
    return -ENODEV;
// Reserve the timer's ISA io-region for ourselves
    if (!request_region(scx200_cb_base + SCx200_TIMER_OFFSET,
    SCx200_TIMER_SIZE,
    "NatSemi SCx200 High-Resolution Timer")) {
    pr_warn("unable to lock timer region\n");
    return -ENODEV;
    }
// write timer config
    outb(HR_TMEN | (mhz27 ? HR_TMCLKSEL : 0),
    scx200_cb_base + SCx200_TMCNFG_OFFSET);
    freq = (HRT_FREQ + ppm);
    if (mhz27)
    freq *= 27;
    pr_info("enabling scx200 high-res timer (%s MHz +%d ppm)\n", mhz27 ? "27":"1", ppm);
    return clocksource_register_hz(&cs_hrt, freq);
    }
    module_init(init_hrt_clocksource);
    MODULE_AUTHOR("Jim Cromie <jim.cromie@gmail.com>");
    MODULE_DESCRIPTION("clocksource on SCx200 HiRes Timer");
    MODULE_LICENSE("GPL");
