//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-cs5535.c
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
// Clock event driver for the CS5535/CS5536
//
// Copyright (C) 2006, Advanced Micro Devices, Inc.
// Copyright (C) 2007  Andres Salomon <dilinger@debian.org>
// Copyright (C) 2009  Andres Salomon <dilinger@collabora.co.uk>
//
// The MFGPTs are documented in AMD Geode CS5536 Companion Device Data Book.
//

    static int timer_irq;
    module_param_hw_named(irq, timer_irq, int, irq, 0644);
    MODULE_PARM_DESC(irq, "Which IRQ to use for the clock source MFGPT ticks.");
//
// We are using the 32.768kHz input clock - it's the only one that has the
// ranges we find desirable.  The following table lists the suitable
// divisors and the associated Hz, minimum interval and the maximum interval:
//
// Divisor   Hz      Min Delta (s)  Max Delta (s)
// 1        32768   .00048828125      2.000
// 2        16384   .0009765625       4.000
// 4         8192   .001953125        8.000
// 8         4096   .00390625        16.000
// 16        2048   .0078125         32.000
// 32        1024   .015625          64.000
// 64         512   .03125          128.000
// 128         256   .0625           256.000
// 256         128   .125            512.000
//
    static struct cs5535_mfgpt_timer *cs5535_event_clock;
// Selected from the table above
pub const MFGPT_DIVISOR: c_int = 16;

//
// The MFGPT timers on the CS5536 provide us with suitable timers to use
// as clock event sources - not as good as a HPET or APIC, but certainly
// better than the PIT.  This isn't a general purpose MFGPT driver, but
// a simplified one designed specifically to act as a clock event source.
// For full details about the MFGPT, please consult the CS5536 data sheet.
//
#[no_mangle]
unsafe extern "C" fn disable_timer(timer: *mut cs5535_mfgpt_timer) {
    static void disable_timer(struct cs5535_mfgpt_timer *timer)
    {
// avoid races by clearing CMP1 and CMP2 unconditionally
    cs5535_mfgpt_write(timer, MFGPT_REG_SETUP,
    (uint16_t) ~MFGPT_SETUP_CNTEN | MFGPT_SETUP_CMP1 |
    MFGPT_SETUP_CMP2);
    }
#[no_mangle]
unsafe extern "C" fn start_timer(timer: *mut cs5535_mfgpt_timer, delta: u16) {
    static void start_timer(struct cs5535_mfgpt_timer *timer, uint16_t delta)
    {
    cs5535_mfgpt_write(timer, MFGPT_REG_CMP2, delta);
    cs5535_mfgpt_write(timer, MFGPT_REG_COUNTER, 0);
    cs5535_mfgpt_write(timer, MFGPT_REG_SETUP,
    MFGPT_SETUP_CNTEN | MFGPT_SETUP_CMP2);
    }
#[no_mangle]
unsafe extern "C" fn mfgpt_shutdown(evt: *mut clock_event_device) -> c_int {
    static int mfgpt_shutdown(struct clock_event_device *evt)
    {
    disable_timer(cs5535_event_clock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mfgpt_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int mfgpt_set_periodic(struct clock_event_device *evt)
    {
    disable_timer(cs5535_event_clock);
    start_timer(cs5535_event_clock, MFGPT_PERIODIC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mfgpt_next_event(delta: c_ulong, evt: *mut clock_event_device) -> c_int {
    static int mfgpt_next_event(unsigned long delta, struct clock_event_device *evt)
    {
    start_timer(cs5535_event_clock, delta);
    return 0;
    }
    static struct clock_event_device cs5535_clockevent = {
    .name = DRV_NAME,
    .features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    .set_state_shutdown = mfgpt_shutdown,
    .set_state_periodic = mfgpt_set_periodic,
    .set_state_oneshot = mfgpt_shutdown,
    .tick_resume = mfgpt_shutdown,
    .set_next_event = mfgpt_next_event,
    .rating = 250,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn mfgpt_tick(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mfgpt_tick(int irq, void *dev_id)
    {
    let mut val: u16 = cs5535_mfgpt_read(cs5535_event_clock, MFGPT_REG_SETUP);
// See if the interrupt was for us
    if (!(val & (MFGPT_SETUP_SETUP | MFGPT_SETUP_CMP2 | MFGPT_SETUP_CMP1)))
    return IRQ_NONE;
// Turn off the clock (and clear the event)
    disable_timer(cs5535_event_clock);
    if (clockevent_state_detached(&cs5535_clockevent) ||
    clockevent_state_shutdown(&cs5535_clockevent))
    return IRQ_HANDLED;
// Clear the counter
    cs5535_mfgpt_write(cs5535_event_clock, MFGPT_REG_COUNTER, 0);
// Restart the clock in periodic mode
    if (clockevent_state_periodic(&cs5535_clockevent))
    cs5535_mfgpt_write(cs5535_event_clock, MFGPT_REG_SETUP,
    MFGPT_SETUP_CNTEN | MFGPT_SETUP_CMP2);
    cs5535_clockevent.event_handler(&cs5535_clockevent);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cs5535_mfgpt_init() -> int __init {
    static int __init cs5535_mfgpt_init(void)
    {
    let mut flags: c_ulong = IRQF_NOBALANCING | IRQF_TIMER | IRQF_SHARED;
    struct cs5535_mfgpt_timer *timer;
    int ret;
    uint16_t val;
    timer = cs5535_mfgpt_alloc_timer(MFGPT_TIMER_ANY, MFGPT_DOMAIN_WORKING);
    if (!timer) {
    printk(KERN_ERR DRV_NAME ": Could not allocate MFGPT timer\n");
    return -ENODEV;
    }
    cs5535_event_clock = timer;
// Set up the IRQ on the MFGPT side
    if (cs5535_mfgpt_setup_irq(timer, MFGPT_CMP2, &timer_irq)) {
    printk(KERN_ERR DRV_NAME ": Could not set up IRQ %d\n",
    timer_irq);
    goto err_timer;
    }
// And register it with the kernel
    ret = request_irq(timer_irq, mfgpt_tick, flags, DRV_NAME, timer);
    if (ret) {
    printk(KERN_ERR DRV_NAME ": Unable to set up the interrupt.\n");
    goto err_irq;
    }
// Set the clock scale and enable the event mode for CMP2
    val = MFGPT_SCALE | (3 << 8);
    cs5535_mfgpt_write(cs5535_event_clock, MFGPT_REG_SETUP, val);
// Set up the clock event
    printk(KERN_INFO DRV_NAME
    ": Registering MFGPT timer as a clock event, using IRQ %d\n",
    timer_irq);
    clockevents_config_and_register(&cs5535_clockevent, MFGPT_HZ,
    0xF, 0xFFFE);
    return 0;
    err_irq:
    cs5535_mfgpt_release_irq(cs5535_event_clock, MFGPT_CMP2, &timer_irq);
    err_timer:
    cs5535_mfgpt_free_timer(cs5535_event_clock);
    printk(KERN_ERR DRV_NAME ": Unable to set up the MFGPT clock source\n");
    return -EIO;
    }
    module_init(cs5535_mfgpt_init);
    MODULE_AUTHOR("Andres Salomon <dilinger@queued.net>");
    MODULE_DESCRIPTION("CS5535/CS5536 MFGPT clock event driver");
    MODULE_LICENSE("GPL");
