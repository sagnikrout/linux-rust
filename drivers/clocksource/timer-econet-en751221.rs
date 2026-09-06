//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-econet-en751221.c
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
// Timer present on EcoNet EN75xx MIPS based SoCs.
//
// Copyright (C) 2025 by Caleb James DeLisle <cjd@cjdns.fr>
//

pub const ECONET_BITS: c_int = 32;
pub const ECONET_MIN_DELTA: c_uint = 0x00001000;

// 34Kc hardware has 1 block and 1004Kc has 2.

    static struct {
    void __iomem	*membase[ECONET_NUM_BLOCKS];
    u32		freq_hz;
    } econet_timer __ro_after_init;
    static DEFINE_PER_CPU(struct clock_event_device, econet_timer_pcpu);
// Each memory block has 2 timers, the order of registers is:
// CTL, CMR0, CNT0, CMR1, CNT1
//
    static inline void __iomem *reg_ctl(u32 timer_n)
    {
    return econet_timer.membase[timer_n >> 1];
    }
    static inline void __iomem *reg_compare(u32 timer_n)
    {
    return econet_timer.membase[timer_n >> 1] + (timer_n & 1) * 0x08 + 0x04;
    }
    static inline void __iomem *reg_count(u32 timer_n)
    {
    return econet_timer.membase[timer_n >> 1] + (timer_n & 1) * 0x08 + 0x08;
    }
#[no_mangle]
pub unsafe extern "C" fn ctl_bit_enabled(timer_n: u32) -> u32 {
    static inline u32 ctl_bit_enabled(u32 timer_n)
    {
    return 1U << (timer_n & 1);
    }
#[no_mangle]
pub unsafe extern "C" fn ctl_bit_pending(timer_n: u32) -> u32 {
    static inline u32 ctl_bit_pending(u32 timer_n)
    {
    return 1U << ((timer_n & 1) + 16);
    }
#[no_mangle]
unsafe extern "C" fn cevt_is_pending(cpu_id: c_int) -> bool {
    static bool cevt_is_pending(int cpu_id)
    {
    return ioread32(reg_ctl(cpu_id)) & ctl_bit_pending(cpu_id);
    }
#[no_mangle]
unsafe extern "C" fn cevt_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t cevt_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *dev = this_cpu_ptr(&econet_timer_pcpu);
    let mut cpu: c_int = cpumask_first(dev.cpumask);
// Each VPE has its own events,
// so this will only happen on spurious interrupt.
//
    if (!cevt_is_pending(cpu))
    return IRQ_NONE;
    iowrite32(ioread32(reg_count(cpu)), reg_compare(cpu));
    dev.event_handler(dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cevt_set_next_event(delta: c_ulong, dev: *mut clock_event_device) -> c_int {
    static int cevt_set_next_event(ulong delta, struct clock_event_device *dev)
    {
    u32 next;
    int cpu;
    cpu = cpumask_first(dev.cpumask);
    next = ioread32(reg_count(cpu)) + delta;
    iowrite32(next, reg_compare(cpu));
    if ((s32)(next - ioread32(reg_count(cpu))) < ECONET_MIN_DELTA / 2)
    return -ETIME;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cevt_init_cpu(cpu: c_uint) -> c_int {
    static int cevt_init_cpu(uint cpu)
    {
    struct clock_event_device *cd = &per_cpu(econet_timer_pcpu, cpu);
    u32 reg;
    pr_debug("%s: Setting up clockevent for CPU %d\n", cd.name, cpu);
    reg = ioread32(reg_ctl(cpu)) | ctl_bit_enabled(cpu);
    iowrite32(reg, reg_ctl(cpu));
    enable_percpu_irq(cd.irq, IRQ_TYPE_NONE);
// Do this last because it synchronously configures the timer
    clockevents_config_and_register(cd, econet_timer.freq_hz,
    ECONET_MIN_DELTA, ECONET_MAX_DELTA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sched_clock_read() -> u64 notrace {
    static u64 notrace sched_clock_read(void)
    {
// Always read from clock zero no matter the CPU
    return (u64)ioread32(reg_count(0));
    }
// Init
#[no_mangle]
unsafe extern "C" fn cevt_dev_init(cpu: c_uint) -> void __init {
    static void __init cevt_dev_init(uint cpu)
    {
    iowrite32(0, reg_count(cpu));
    iowrite32(U32_MAX, reg_compare(cpu));
    }
#[no_mangle]
unsafe extern "C" fn cevt_init(np: *mut device_node) -> int __init {
    static int __init cevt_init(struct device_node *np)
    {
    int i, irq, ret;
    irq = irq_of_parse_and_map(np, 0);
    if (irq <= 0) {
    pr_err("%pOFn: irq_of_parse_and_map failed", np);
    return -EINVAL;
    }
    ret = request_percpu_irq(irq, cevt_interrupt, np.name, &econet_timer_pcpu);
    if (ret < 0) {
    pr_err("%pOFn: IRQ %d setup failed (%d)\n", np, irq, ret);
    goto err_unmap_irq;
    }
    for_each_possible_cpu(i) {
    struct clock_event_device *cd = &per_cpu(econet_timer_pcpu, i);
    cd.rating		= 310;
    cd.features		= CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_C3STOP |
    CLOCK_EVT_FEAT_PERCPU;
    cd.set_next_event	= cevt_set_next_event;
    cd.irq			= irq;
    cd.cpumask		= cpumask_of(i);
    cd.name		= np.name;
    cevt_dev_init(i);
    }
    cpuhp_setup_state(CPUHP_AP_ONLINE_DYN,
    "clockevents/econet/timer:starting",
    cevt_init_cpu, core::ptr::null_mut());
    return 0;
    err_unmap_irq:
    irq_dispose_mapping(irq);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn timer_init(np: *mut device_node) -> int __init {
    static int __init timer_init(struct device_node *np)
    {
    let mut num_blocks: c_int = DIV_ROUND_UP(num_possible_cpus(), 2);
    struct clk *clk;
    int ret;
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    pr_err("%pOFn: Failed to get CPU clock from DT %ld\n", np, PTR_ERR(clk));
    return PTR_ERR(clk);
    }
    econet_timer.freq_hz = clk_get_rate(clk);
    for (int i = 0; i < num_blocks; i++) {
    econet_timer.membase[i] = of_iomap(np, i);
    if (!econet_timer.membase[i]) {
    pr_err("%pOFn: failed to map register [%d]\n", np, i);
    return -ENXIO;
    }
    }
// For clocksource purposes always read clock zero, whatever the CPU
    ret = clocksource_mmio_init(reg_count(0), np.name,
    econet_timer.freq_hz, 301, ECONET_BITS,
    clocksource_mmio_readl_up);
    if (ret) {
    pr_err("%pOFn: clocksource_mmio_init failed: %d", np, ret);
    return ret;
    }
    ret = cevt_init(np);
    if (ret < 0)
    return ret;
    sched_clock_register(sched_clock_read, ECONET_BITS,
    econet_timer.freq_hz);
    pr_info("%pOFn: using %u.%03u MHz high precision timer\n", np,
    econet_timer.freq_hz / 1000000,
    (econet_timer.freq_hz / 1000) % 1000);
    return 0;
    }
    TIMER_OF_DECLARE(econet_timer_hpt, "econet,en751221-timer", timer_init);
