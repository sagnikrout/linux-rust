//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-atmel-pit.c
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
// at91sam926x_time.c - Periodic Interval Timer (PIT) for at91sam926x
//
// Copyright (C) 2005-2006 M. Amine SAYA, ATMEL Rousset, France
// Revision	 2005 M. Nicolas Diremdjian, ATMEL Rousset, France
// Converted to ClockSource/ClockEvents by David Brownell.
//

pub const AT91_PIT_MR: c_uint = 0x00			/* Mode Register */;

pub const AT91_PIT_SR: c_uint = 0x04			/* Status Register */;

pub const AT91_PIT_PIVR: c_uint = 0x08			/* Periodic Interval Value Register */;
pub const AT91_PIT_PIIR: c_uint = 0x0c			/* Periodic Interval Image Register */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pit_data {
    pub clkevt: clock_event_device,
    pub clksrc: clocksource,
    pub base: *mut void __iomem,
    pub cycle: u32,
    pub cnt: u32,
    pub irq: c_uint,
    pub mck: *mut clk,
}

    static inline struct pit_data *clksrc_to_pit_data(struct clocksource *clksrc)
    {
    return container_of(clksrc, struct pit_data, clksrc);
    }
    static inline struct pit_data *clkevt_to_pit_data(struct clock_event_device *clkevt)
    {
    return container_of(clkevt, struct pit_data, clkevt);
    }
#[no_mangle]
pub unsafe extern "C" fn pit_read(base: *mut void __iomem, reg_offset: c_uint) -> c_uint {
    static inline unsigned int pit_read(void __iomem *base, unsigned int reg_offset)
    {
    return readl_relaxed(base + reg_offset);
    }
#[no_mangle]
pub unsafe extern "C" fn pit_write(base: *mut void __iomem, reg_offset: c_uint, value: c_ulong) {
    static inline void pit_write(void __iomem *base, unsigned int reg_offset, unsigned long value)
    {
    writel_relaxed(value, base + reg_offset);
    }
//
// Clocksource:  just a monotonic counter of MCK/16 cycles.
// We don't care whether or not PIT irqs are enabled.
//
#[no_mangle]
unsafe extern "C" fn read_pit_clk(cs: *mut clocksource) -> u64 {
    static u64 read_pit_clk(struct clocksource *cs)
    {
    struct pit_data *data = clksrc_to_pit_data(cs);
    unsigned long flags;
    u32 elapsed;
    u32 t;
    raw_local_irq_save(flags);
    elapsed = data.cnt;
    t = pit_read(data.base, AT91_PIT_PIIR);
    raw_local_irq_restore(flags);
    elapsed += PIT_PICNT(t) * data.cycle;
    elapsed += PIT_CPIV(t);
    return elapsed;
    }
#[no_mangle]
unsafe extern "C" fn pit_clkevt_shutdown(dev: *mut clock_event_device) -> c_int {
    static int pit_clkevt_shutdown(struct clock_event_device *dev)
    {
    struct pit_data *data = clkevt_to_pit_data(dev);
// disable irq, leaving the clocksource active
    pit_write(data.base, AT91_PIT_MR, (data.cycle - 1) | AT91_PIT_PITEN);
    return 0;
    }
//
// Clockevent device:  interrupts every 1/HZ (== pit_cycles * MCK/16)
//
#[no_mangle]
unsafe extern "C" fn pit_clkevt_set_periodic(dev: *mut clock_event_device) -> c_int {
    static int pit_clkevt_set_periodic(struct clock_event_device *dev)
    {
    struct pit_data *data = clkevt_to_pit_data(dev);
// update clocksource counter
    data.cnt += data.cycle * PIT_PICNT(pit_read(data.base, AT91_PIT_PIVR));
    pit_write(data.base, AT91_PIT_MR,
    (data.cycle - 1) | AT91_PIT_PITEN | AT91_PIT_PITIEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91sam926x_pit_suspend(cedev: *mut clock_event_device) {
    static void at91sam926x_pit_suspend(struct clock_event_device *cedev)
    {
    struct pit_data *data = clkevt_to_pit_data(cedev);
// Disable timer
    pit_write(data.base, AT91_PIT_MR, 0);
    }
#[no_mangle]
unsafe extern "C" fn at91sam926x_pit_reset(data: *mut pit_data) {
    static void at91sam926x_pit_reset(struct pit_data *data)
    {
// Disable timer and irqs
    pit_write(data.base, AT91_PIT_MR, 0);
// Clear any pending interrupts, wait for PIT to stop counting
    while (PIT_CPIV(pit_read(data.base, AT91_PIT_PIVR)) != 0)
    cpu_relax();
// Start PIT but don't enable IRQ
    pit_write(data.base, AT91_PIT_MR,
    (data.cycle - 1) | AT91_PIT_PITEN);
    }
#[no_mangle]
unsafe extern "C" fn at91sam926x_pit_resume(cedev: *mut clock_event_device) {
    static void at91sam926x_pit_resume(struct clock_event_device *cedev)
    {
    struct pit_data *data = clkevt_to_pit_data(cedev);
    at91sam926x_pit_reset(data);
    }
//
// IRQ handler for the timer.
//
#[no_mangle]
unsafe extern "C" fn at91sam926x_pit_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t at91sam926x_pit_interrupt(int irq, void *dev_id)
    {
    struct pit_data *data = dev_id;
// The PIT interrupt may be disabled, and is shared
    if (clockevent_state_periodic(&data.clkevt) &&
    (pit_read(data.base, AT91_PIT_SR) & AT91_PIT_PITS)) {
// Get number of ticks performed before irq, and ack it
    data.cnt += data.cycle * PIT_PICNT(pit_read(data.base,
    AT91_PIT_PIVR));
    data.clkevt.event_handler(&data.clkevt);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
//
// Set up both clocksource and clockevent support.
//
#[no_mangle]
unsafe extern "C" fn at91sam926x_pit_dt_init(node: *mut device_node) -> int __init {
    static int __init at91sam926x_pit_dt_init(struct device_node *node)
    {
    unsigned long   pit_rate;
    unsigned        bits;
    int             ret;
    struct pit_data *data;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    data.base = of_iomap(node, 0);
    if (!data.base) {
    pr_err("Could not map PIT address\n");
    ret = -ENXIO;
    goto exit;
    }
    data.mck = of_clk_get(node, 0);
    if (IS_ERR(data.mck)) {
    pr_err("Unable to get mck clk\n");
    ret = PTR_ERR(data.mck);
    goto exit;
    }
    ret = clk_prepare_enable(data.mck);
    if (ret) {
    pr_err("Unable to enable mck\n");
    goto exit;
    }
// Get the interrupts property
    data.irq = irq_of_parse_and_map(node, 0);
    if (!data.irq) {
    pr_err("Unable to get IRQ from DT\n");
    ret = -EINVAL;
    goto exit;
    }
//
// Use our actual MCK to figure out how many MCK/16 ticks per
// 1/HZ period (instead of a compile-time constant LATCH).
//
    pit_rate = clk_get_rate(data.mck) / 16;
    data.cycle = DIV_ROUND_CLOSEST(pit_rate, HZ);
    WARN_ON(((data.cycle - 1) & ~AT91_PIT_PIV) != 0);
// Initialize and enable the timer
    at91sam926x_pit_reset(data);
//
// Register clocksource.  The high order bits of PIV are unused,
// so this isn't a 32-bit counter unless we get clockevent irqs.
//
    bits = 12 /* PICNT */ + ilog2(data.cycle) /* PIV */;
    data.clksrc.mask = CLOCKSOURCE_MASK(bits);
    data.clksrc.name = "pit";
    data.clksrc.rating = 175;
    data.clksrc.read = read_pit_clk;
    data.clksrc.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    ret = clocksource_register_hz(&data.clksrc, pit_rate);
    if (ret) {
    pr_err("Failed to register clocksource\n");
    goto exit;
    }
// Set up irq handler
    ret = request_irq(data.irq, at91sam926x_pit_interrupt,
    IRQF_SHARED | IRQF_TIMER | IRQF_IRQPOLL,
    "at91_tick", data);
    if (ret) {
    pr_err("Unable to setup IRQ\n");
    clocksource_unregister(&data.clksrc);
    goto exit;
    }
// Set up and register clockevents
    data.clkevt.name = "pit";
    data.clkevt.features = CLOCK_EVT_FEAT_PERIODIC;
    data.clkevt.shift = 32;
    data.clkevt.mult = div_sc(pit_rate, NSEC_PER_SEC, data.clkevt.shift);
    data.clkevt.rating = 100;
    data.clkevt.cpumask = cpumask_of(0);
    data.clkevt.set_state_shutdown = pit_clkevt_shutdown;
    data.clkevt.set_state_periodic = pit_clkevt_set_periodic;
    data.clkevt.resume = at91sam926x_pit_resume;
    data.clkevt.suspend = at91sam926x_pit_suspend;
    clockevents_register_device(&data.clkevt);
    return 0;
    exit:
    kfree(data);
    return ret;
    }
    TIMER_OF_DECLARE(at91sam926x_pit, "atmel,at91sam9260-pit",
    at91sam926x_pit_dt_init);
