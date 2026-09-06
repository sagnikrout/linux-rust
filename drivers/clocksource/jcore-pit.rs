//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/jcore-pit.c
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
// J-Core SoC PIT/clocksource driver
//
// Copyright (C) 2015-2016 Smart Energy Instruments, Inc.
//

pub const PIT_IRQ_SHIFT: c_int = 12;
pub const PIT_PRIO_SHIFT: c_int = 20;
pub const PIT_ENABLE_SHIFT: c_int = 26;
pub const PIT_PRIO_MASK: c_uint = 0xf;
pub const REG_PITEN: c_uint = 0x00;
pub const REG_THROT: c_uint = 0x10;
pub const REG_COUNT: c_uint = 0x14;
pub const REG_BUSPD: c_uint = 0x18;
pub const REG_SECHI: c_uint = 0x20;
pub const REG_SECLO: c_uint = 0x24;
pub const REG_NSEC: c_uint = 0x28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jcore_pit {
    pub ced: clock_event_device,
    pub base: *mut void __iomem,
    pub periodic_delta: c_ulong,
    pub enable_val: u32,
}

    static void __iomem *jcore_pit_base;
    static struct jcore_pit __percpu *jcore_pit_percpu;
#[no_mangle]
unsafe extern "C" fn jcore_sched_clock_read() -> notrace u64 {
    static notrace u64 jcore_sched_clock_read(void)
    {
    u32 seclo, nsec, seclo0;
    __iomem void *base = jcore_pit_base;
    seclo = readl(base + REG_SECLO);
    do {
    seclo0 = seclo;
    nsec  = readl(base + REG_NSEC);
    seclo = readl(base + REG_SECLO);
    } while (seclo0 != seclo);
    return seclo * NSEC_PER_SEC + nsec;
    }
#[no_mangle]
unsafe extern "C" fn jcore_clocksource_read(cs: *mut clocksource) -> u64 {
    static u64 jcore_clocksource_read(struct clocksource *cs)
    {
    return jcore_sched_clock_read();
    }
#[no_mangle]
unsafe extern "C" fn jcore_pit_disable(pit: *mut jcore_pit) -> c_int {
    static int jcore_pit_disable(struct jcore_pit *pit)
    {
    writel(0, pit.base + REG_PITEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jcore_pit_set(delta: c_ulong, pit: *mut jcore_pit) -> c_int {
    static int jcore_pit_set(unsigned long delta, struct jcore_pit *pit)
    {
    jcore_pit_disable(pit);
    writel(delta, pit.base + REG_THROT);
    writel(pit.enable_val, pit.base + REG_PITEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jcore_pit_set_state_shutdown(ced: *mut clock_event_device) -> c_int {
    static int jcore_pit_set_state_shutdown(struct clock_event_device *ced)
    {
    struct jcore_pit *pit = container_of(ced, struct jcore_pit, ced);
    return jcore_pit_disable(pit);
    }
#[no_mangle]
unsafe extern "C" fn jcore_pit_set_state_oneshot(ced: *mut clock_event_device) -> c_int {
    static int jcore_pit_set_state_oneshot(struct clock_event_device *ced)
    {
    struct jcore_pit *pit = container_of(ced, struct jcore_pit, ced);
    return jcore_pit_disable(pit);
    }
#[no_mangle]
unsafe extern "C" fn jcore_pit_set_state_periodic(ced: *mut clock_event_device) -> c_int {
    static int jcore_pit_set_state_periodic(struct clock_event_device *ced)
    {
    struct jcore_pit *pit = container_of(ced, struct jcore_pit, ced);
    return jcore_pit_set(pit.periodic_delta, pit);
    }
    static int jcore_pit_set_next_event(unsigned long delta,
    struct clock_event_device *ced)
    {
    struct jcore_pit *pit = container_of(ced, struct jcore_pit, ced);
    return jcore_pit_set(delta, pit);
    }
#[no_mangle]
unsafe extern "C" fn jcore_pit_local_init(cpu: unsigned) -> c_int {
    static int jcore_pit_local_init(unsigned cpu)
    {
    struct jcore_pit *pit = this_cpu_ptr(jcore_pit_percpu);
    unsigned buspd, freq;
    pr_info("Local J-Core PIT init on cpu %u\n", cpu);
    buspd = readl(pit.base + REG_BUSPD);
    freq = DIV_ROUND_CLOSEST(NSEC_PER_SEC, buspd);
    pit.periodic_delta = DIV_ROUND_CLOSEST(NSEC_PER_SEC, HZ * buspd);
    clockevents_config_and_register(&pit.ced, freq, 1, ULONG_MAX);
    enable_percpu_irq(pit.ced.irq, IRQ_TYPE_NONE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jcore_pit_local_teardown(cpu: unsigned) -> c_int {
    static int jcore_pit_local_teardown(unsigned cpu)
    {
    struct jcore_pit *pit = this_cpu_ptr(jcore_pit_percpu);
    pr_info("Local J-Core PIT teardown on cpu %u\n", cpu);
    disable_percpu_irq(pit.ced.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jcore_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t jcore_timer_interrupt(int irq, void *dev_id)
    {
    struct jcore_pit *pit = dev_id;
    if (clockevent_state_oneshot(&pit.ced))
    jcore_pit_disable(pit);
    pit.ced.event_handler(&pit.ced);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn jcore_pit_init(node: *mut device_node) -> int __init {
    static int __init jcore_pit_init(struct device_node *node)
    {
    int err;
    unsigned pit_irq, cpu;
    unsigned long hwirq;
    u32 irqprio, enable_val;
    jcore_pit_base = of_iomap(node, 0);
    if (!jcore_pit_base) {
    pr_err("Error: Cannot map base address for J-Core PIT\n");
    return -ENXIO;
    }
    pit_irq = irq_of_parse_and_map(node, 0);
    if (!pit_irq) {
    pr_err("Error: J-Core PIT has no IRQ\n");
    return -ENXIO;
    }
    pr_info("Initializing J-Core PIT at %p IRQ %d\n",
    jcore_pit_base, pit_irq);
    err = clocksource_mmio_init(jcore_pit_base, "jcore_pit_cs",
    NSEC_PER_SEC, 400, 32,
    jcore_clocksource_read);
    if (err) {
    pr_err("Error registering clocksource device: %d\n", err);
    return err;
    }
    sched_clock_register(jcore_sched_clock_read, 32, NSEC_PER_SEC);
    jcore_pit_percpu = alloc_percpu(struct jcore_pit);
    if (!jcore_pit_percpu) {
    pr_err("Failed to allocate memory for clock event device\n");
    return -ENOMEM;
    }
    irq_set_percpu_devid(pit_irq);
    err = request_percpu_irq(pit_irq, jcore_timer_interrupt,
    "jcore_pit", jcore_pit_percpu);
    if (err) {
    pr_err("pit irq request failed: %d\n", err);
    free_percpu(jcore_pit_percpu);
    return err;
    }
//
// The J-Core PIT is not hard-wired to a particular IRQ, but
// integrated with the interrupt controller such that the IRQ it
// generates is programmable, as follows:
//
// The bit layout of the PIT enable register is:
//
// .....e..ppppiiiiiiii............
//
// where the .'s indicate unrelated/unused bits, e is enable,
// p is priority, and i is hard irq number.
//
// For the PIT included in AIC1 (obsolete but still in use),
// any hard irq (trap number) can be programmed via the 8
// iiiiiiii bits, and a priority (0-15) is programmable
// separately in the pppp bits.
//
// For the PIT included in AIC2 (current), the programming
// interface is equivalent modulo interrupt mapping. This is
// why a different compatible tag was not used. However only
// traps 64-127 (the ones actually intended to be used for
// interrupts, rather than syscalls/exceptions/etc.) can be
// programmed (the high 2 bits of i are ignored) and the
// priority pppp is <<2'd and or'd onto the irq number. This
// choice seems to have been made on the hardware engineering
// side under an assumption that preserving old AIC1 priority
// mappings was important. Future models will likely ignore
// the pppp field.
//
    hwirq = irq_get_irq_data(pit_irq).hwirq;
    irqprio = (hwirq >> 2) & PIT_PRIO_MASK;
    enable_val = (1U << PIT_ENABLE_SHIFT)
    | (hwirq << PIT_IRQ_SHIFT)
    | (irqprio << PIT_PRIO_SHIFT);
    for_each_present_cpu(cpu) {
    struct jcore_pit *pit = per_cpu_ptr(jcore_pit_percpu, cpu);
    pit.base = of_iomap(node, cpu);
    if (!pit.base) {
    pr_err("Unable to map PIT for cpu %u\n", cpu);
    continue;
    }
    pit.ced.name = "jcore_pit";
    pit.ced.features = CLOCK_EVT_FEAT_PERIODIC
    | CLOCK_EVT_FEAT_ONESHOT
    | CLOCK_EVT_FEAT_PERCPU;
    pit.ced.cpumask = cpumask_of(cpu);
    pit.ced.rating = 400;
    pit.ced.irq = pit_irq;
    pit.ced.set_state_shutdown = jcore_pit_set_state_shutdown;
    pit.ced.set_state_periodic = jcore_pit_set_state_periodic;
    pit.ced.set_state_oneshot = jcore_pit_set_state_oneshot;
    pit.ced.set_next_event = jcore_pit_set_next_event;
    pit.enable_val = enable_val;
    }
    cpuhp_setup_state(CPUHP_AP_JCORE_TIMER_STARTING,
    "clockevents/jcore:starting",
    jcore_pit_local_init, jcore_pit_local_teardown);
    return 0;
    }
    TIMER_OF_DECLARE(jcore_pit, "jcore,pit", jcore_pit_init);
