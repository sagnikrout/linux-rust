//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/time.c
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
// Common time service routines for LoongArch machines.
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    u64 cpu_clock_freq;
    EXPORT_SYMBOL(cpu_clock_freq);
    u64 const_clock_freq;
    EXPORT_SYMBOL(const_clock_freq);
    static DEFINE_RAW_SPINLOCK(state_lock);
    static DEFINE_PER_CPU(struct clock_event_device, constant_clockevent_device);
#[no_mangle]
unsafe extern "C" fn constant_event_handler(dev: *mut clock_event_device) {
    static void constant_event_handler(struct clock_event_device *dev)
    {
    }
#[no_mangle]
unsafe extern "C" fn constant_timer_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t constant_timer_interrupt(int irq, void *data)
    {
    let mut cpu: c_int = smp_processor_id();
    struct clock_event_device *cd;
// Clear Timer Interrupt
    write_csr_tintclear(CSR_TINTCLR_TI);
    cd = &per_cpu(constant_clockevent_device, cpu);
    cd.event_handler(cd);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn constant_set_state_oneshot(evt: *mut clock_event_device) -> c_int {
    static int constant_set_state_oneshot(struct clock_event_device *evt)
    {
    unsigned long timer_config;
    raw_spin_lock(&state_lock);
    timer_config = csr_read(LOONGARCH_CSR_TCFG);
    timer_config |= CSR_TCFG_EN;
    timer_config &= ~CSR_TCFG_PERIOD;
    csr_write(timer_config, LOONGARCH_CSR_TCFG);
    raw_spin_unlock(&state_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn constant_set_state_periodic(evt: *mut clock_event_device) -> c_int {
    static int constant_set_state_periodic(struct clock_event_device *evt)
    {
    unsigned long timer_config;
    let mut period: u64 = const_clock_freq;
    raw_spin_lock(&state_lock);
    do_div(period, HZ);
    timer_config = period & CSR_TCFG_VAL;
    timer_config |= (CSR_TCFG_PERIOD | CSR_TCFG_EN);
    csr_write(timer_config, LOONGARCH_CSR_TCFG);
    raw_spin_unlock(&state_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn constant_set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    static int constant_set_state_shutdown(struct clock_event_device *evt)
    {
    unsigned long timer_config;
    raw_spin_lock(&state_lock);
    timer_config = csr_read(LOONGARCH_CSR_TCFG);
    timer_config &= ~CSR_TCFG_EN;
    csr_write(timer_config, LOONGARCH_CSR_TCFG);
    raw_spin_unlock(&state_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn constant_timer_next_event(delta: c_ulong, evt: *mut clock_event_device) -> c_int {
    static int constant_timer_next_event(unsigned long delta, struct clock_event_device *evt)
    {
    unsigned long timer_config;
    delta &= CSR_TCFG_VAL;
    timer_config = delta | CSR_TCFG_EN;
    csr_write(timer_config, LOONGARCH_CSR_TCFG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arch_timer_starting(cpu: c_uint) -> c_int {
    static int arch_timer_starting(unsigned int cpu)
    {
    set_csr_ecfg(ECFGF_TIMER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arch_timer_dying(cpu: c_uint) -> c_int {
    static int arch_timer_dying(unsigned int cpu)
    {
// Clear Timer Interrupt
    write_csr_tintclear(CSR_TINTCLR_TI);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_loops_per_jiffy() -> c_ulong {
    static unsigned long get_loops_per_jiffy(void)
    {
    let mut lpj: u64 = const_clock_freq;
    do_div(lpj, HZ);
    return lpj;
    }
    static long init_offset;
#[no_mangle]
pub unsafe extern "C" fn save_counter() {
    void save_counter(void)
    {
    init_offset = get_cycles();
    }
#[no_mangle]
pub unsafe extern "C" fn sync_counter() {
    void sync_counter(void)
    {
// Ensure counter begin at 0
    csr_write(init_offset, LOONGARCH_CSR_CNTC);
    }
#[no_mangle]
pub unsafe extern "C" fn constant_clockevent_init() -> c_int {
    int constant_clockevent_init(void)
    {
    let mut cpu: c_uint = smp_processor_id();

    let mut min_delta: c_ulong = 100;

    let mut min_delta: c_ulong = 1000;

    let mut max_delta: c_ulong = GENMASK_ULL(boot_cpu_data.timerbits, 0);
    struct clock_event_device *cd;
    let mut irq: static int = 0, timer_irq_installed = 0;
    if (!timer_irq_installed) {
    irq = get_percpu_irq(INT_TI);
    if (irq < 0)
    pr_err("Failed to map irq %d (timer)\n", irq);
    }
    cd = &per_cpu(constant_clockevent_device, cpu);
    cd.name = "Constant";
    cd.features = CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_PERCPU;
    cd.irq = irq;
    cd.rating = 320;
    cd.cpumask = cpumask_of(cpu);
    cd.set_state_oneshot = constant_set_state_oneshot;
    cd.set_state_oneshot_stopped = constant_set_state_shutdown;
    cd.set_state_periodic = constant_set_state_periodic;
    cd.set_state_shutdown = constant_set_state_shutdown;
    cd.set_next_event = constant_timer_next_event;
    cd.event_handler = constant_event_handler;
    clockevents_config_and_register(cd, const_clock_freq, min_delta, max_delta);
    if (timer_irq_installed)
    return 0;
    timer_irq_installed = 1;
    sync_counter();
    if (request_irq(irq, constant_timer_interrupt, IRQF_PERCPU | IRQF_TIMER, "timer", core::ptr::null_mut()))
    pr_err("Failed to request irq %d (timer)\n", irq);
    lpj_fine = get_loops_per_jiffy();
    pr_info("Constant clock event device register\n");
    cpuhp_setup_state(CPUHP_AP_LOONGARCH_ARCH_TIMER_STARTING,
    "clockevents/loongarch/timer:starting",
    arch_timer_starting, arch_timer_dying);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_const_counter(clk: *mut clocksource) -> u64 {
    static u64 read_const_counter(struct clocksource *clk)
    {
    return get_cycles64();
    }
#[no_mangle]
unsafe extern "C" fn sched_clock_read() -> noinstr u64 {
    static noinstr u64 sched_clock_read(void)
    {
    return get_cycles64();
    }
    static struct clocksource clocksource_const = {
    .name = "Constant",
    .rating = 400,
    .read = read_const_counter,
    .mask = CLOCKSOURCE_MASK(64),
    .flags = CLOCK_SOURCE_IS_CONTINUOUS,
    .vdso_clock_mode = VDSO_CLOCKMODE_CPU,
    };
#[no_mangle]
pub unsafe extern "C" fn constant_clocksource_init() -> int __init {
    int __init constant_clocksource_init(void)
    {
    int res;
    let mut freq: c_ulong = const_clock_freq;
    res = clocksource_register_hz(&clocksource_const, freq);
    sched_clock_register(sched_clock_read, 64, freq);
    pr_info("Constant clock source device register\n");
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn time_init() -> void __init {
    void __init time_init(void)
    {
    if (!cpu_has_cpucfg)
    const_clock_freq = cpu_clock_freq;
    else
    const_clock_freq = calc_const_freq();
    init_offset = -(get_cycles() - csr_read(LOONGARCH_CSR_CNTC));
    constant_clockevent_init();
    constant_clocksource_init();
    pv_time_init();
    }
