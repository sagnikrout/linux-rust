//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/exynos_mct.c
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
// linux/arch/arm/mach-exynos4/mct.c
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Exynos4 MCT(Multi-Core Timer) support
//

pub const TICK_BASE_CNT: c_int = 1;

// Use values higher than ARM arch timer. See 6282edb72bed.
pub const MCT_CLKSOURCE_RATING: c_int = 450;
pub const MCT_CLKEVENTS_RATING: c_int = 500;

pub const MCT_CLKSOURCE_RATING: c_int = 350;
pub const MCT_CLKEVENTS_RATING: c_int = 350;

// There are four Global timers starting with 0 offset
pub const MCT_G0_IRQ: c_int = 0;
// Local timers count starts after global timer count
pub const MCT_L0_IRQ: c_int = 4;
// Max number of IRQ as per DT binding document
pub const MCT_NR_IRQS: c_int = 20;
// Max number of local timers

    enum {
    MCT_INT_SPI,
    MCT_INT_PPI
    };
    static void __iomem *reg_base;
    static unsigned long clk_rate;
    static unsigned int mct_int_type;
    static int mct_irqs[MCT_NR_IRQS];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mct_clock_event_device {
    pub evt: clock_event_device,
    pub base: c_ulong,
//
// The length of the name must be adjusted if number of
// local timer interrupts grow over two digits
//
    pub name: [c_char; 11],
}

#[no_mangle]
unsafe extern "C" fn exynos4_mct_write(value: c_uint, offset: c_ulong) {
    static void exynos4_mct_write(unsigned int value, unsigned long offset)
    {
    unsigned long stat_addr;
    u32 mask;
    u32 i;
    writel_relaxed(value, reg_base + offset);
    if (likely(offset >= EXYNOS4_MCT_L_BASE(0))) {
    stat_addr = (offset & EXYNOS4_MCT_L_MASK) + MCT_L_WSTAT_OFFSET;
    switch (offset & ~EXYNOS4_MCT_L_MASK) {
    case MCT_L_TCON_OFFSET:
    mask = 1 << 3;		/* L_TCON write status */
    break;
    case MCT_L_ICNTB_OFFSET:
    mask = 1 << 1;		/* L_ICNTB write status */
    break;
    case MCT_L_TCNTB_OFFSET:
    mask = 1 << 0;		/* L_TCNTB write status */
    break;
    default:
    return;
    }
    } else {
    switch (offset) {
    case EXYNOS4_MCT_G_TCON:
    stat_addr = EXYNOS4_MCT_G_WSTAT;
    mask = 1 << 16;		/* G_TCON write status */
    break;
    case EXYNOS4_MCT_G_COMP0_L:
    stat_addr = EXYNOS4_MCT_G_WSTAT;
    mask = 1 << 0;		/* G_COMP0_L write status */
    break;
    case EXYNOS4_MCT_G_COMP0_U:
    stat_addr = EXYNOS4_MCT_G_WSTAT;
    mask = 1 << 1;		/* G_COMP0_U write status */
    break;
    case EXYNOS4_MCT_G_COMP0_ADD_INCR:
    stat_addr = EXYNOS4_MCT_G_WSTAT;
    mask = 1 << 2;		/* G_COMP0_ADD_INCR w status */
    break;
    case EXYNOS4_MCT_G_CNT_L:
    stat_addr = EXYNOS4_MCT_G_CNT_WSTAT;
    mask = 1 << 0;		/* G_CNT_L write status */
    break;
    case EXYNOS4_MCT_G_CNT_U:
    stat_addr = EXYNOS4_MCT_G_CNT_WSTAT;
    mask = 1 << 1;		/* G_CNT_U write status */
    break;
    default:
    return;
    }
    }
// Wait maximum 1 ms until written values are applied
    for (i = 0; i < loops_per_jiffy / 1000 * HZ; i++)
    if (readl_relaxed(reg_base + stat_addr) & mask) {
    writel_relaxed(mask, reg_base + stat_addr);
    return;
    }
    panic("MCT hangs after writing %d (offset:0x%lx)\n", value, offset);
    }
// Clocksource handling
#[no_mangle]
unsafe extern "C" fn exynos4_mct_frc_start() {
    static void exynos4_mct_frc_start(void)
    {
    u32 reg;
    reg = readl_relaxed(reg_base + EXYNOS4_MCT_G_TCON);
    reg |= MCT_G_TCON_START;
    exynos4_mct_write(reg, EXYNOS4_MCT_G_TCON);
    }
//
// exynos4_read_count_64 - Read all 64-bits of the global counter
//
// This will read all 64-bits of the global counter taking care to make sure
// that the upper and lower half match.  Note that reading the MCT can be quite
// slow (hundreds of nanoseconds) so you should use the 32-bit (lower half
// only) version when possible.
//
// Returns the number of cycles in the global counter.
//
#[no_mangle]
unsafe extern "C" fn exynos4_read_count_64() -> u64 {
    static u64 exynos4_read_count_64(void)
    {
    unsigned int lo, hi;
    let mut hi2: u32 = readl_relaxed(reg_base + EXYNOS4_MCT_G_CNT_U);
    do {
    hi = hi2;
    lo = readl_relaxed(reg_base + EXYNOS4_MCT_G_CNT_L);
    hi2 = readl_relaxed(reg_base + EXYNOS4_MCT_G_CNT_U);
    } while (hi != hi2);
    return ((u64)hi << 32) | lo;
    }
//
// exynos4_read_count_32 - Read the lower 32-bits of the global counter
//
// This will read just the lower 32-bits of the global counter.  This is marked
// as notrace so it can be used by the scheduler clock.
//
// Returns the number of cycles in the global counter (lower 32 bits).
//
#[no_mangle]
unsafe extern "C" fn exynos4_read_count_32() -> u32 notrace {
    static u32 notrace exynos4_read_count_32(void)
    {
    return readl_relaxed(reg_base + EXYNOS4_MCT_G_CNT_L);
    }
#[no_mangle]
unsafe extern "C" fn exynos4_frc_read(cs: *mut clocksource) -> u64 {
    static u64 exynos4_frc_read(struct clocksource *cs)
    {
    return exynos4_read_count_32();
    }
#[no_mangle]
unsafe extern "C" fn exynos4_frc_resume(cs: *mut clocksource) {
    static void exynos4_frc_resume(struct clocksource *cs)
    {
    exynos4_mct_frc_start();
    }
    static struct clocksource mct_frc = {
    .name		= "mct-frc",
    .rating		= MCT_CLKSOURCE_RATING,
    .read		= exynos4_frc_read,
    .mask		= CLOCKSOURCE_MASK(32),
    .flags		= CLOCK_SOURCE_IS_CONTINUOUS,
    .resume		= exynos4_frc_resume,
    };
#[no_mangle]
unsafe extern "C" fn exynos4_read_sched_clock() -> u64 notrace {
    static u64 notrace exynos4_read_sched_clock(void)
    {
    return exynos4_read_count_32();
    }

    static struct delay_timer exynos4_delay_timer;
#[no_mangle]
unsafe extern "C" fn exynos4_read_current_timer() -> cycles_t {
    static cycles_t exynos4_read_current_timer(void)
    {
    BUILD_BUG_ON_MSG(sizeof(cycles_t) != sizeof(u32),
    "cycles_t needs to move to 32-bit for ARM64 usage");
    return exynos4_read_count_32();
    }

#[no_mangle]
unsafe extern "C" fn exynos4_clocksource_init(frc_shared: bool) -> int __init {
    static int __init exynos4_clocksource_init(bool frc_shared)
    {
//
// When the frc is shared, the main processor should have already
// turned it on and we shouldn't be writing to TCON.
//
    if (frc_shared)
    mct_frc.resume = core::ptr::null_mut();
    else
    exynos4_mct_frc_start();

    exynos4_delay_timer.read_current_timer = &exynos4_read_current_timer;
    exynos4_delay_timer.freq = clk_rate;
    register_current_timer_delay(&exynos4_delay_timer);

    if (clocksource_register_hz(&mct_frc, clk_rate))
    panic("%s: can't register clocksource\n", mct_frc.name);
    sched_clock_register(exynos4_read_sched_clock, 32, clk_rate);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos4_mct_comp0_stop() {
    static void exynos4_mct_comp0_stop(void)
    {
    unsigned int tcon;
    tcon = readl_relaxed(reg_base + EXYNOS4_MCT_G_TCON);
    tcon &= ~(MCT_G_TCON_COMP0_ENABLE | MCT_G_TCON_COMP0_AUTO_INC);
    exynos4_mct_write(tcon, EXYNOS4_MCT_G_TCON);
    exynos4_mct_write(0, EXYNOS4_MCT_G_INT_ENB);
    }
#[no_mangle]
unsafe extern "C" fn exynos4_mct_comp0_start(periodic: bool, cycles: c_ulong) {
    static void exynos4_mct_comp0_start(bool periodic, unsigned long cycles)
    {
    unsigned int tcon;
    u64 comp_cycle;
    tcon = readl_relaxed(reg_base + EXYNOS4_MCT_G_TCON);
    if (periodic) {
    tcon |= MCT_G_TCON_COMP0_AUTO_INC;
    exynos4_mct_write(cycles, EXYNOS4_MCT_G_COMP0_ADD_INCR);
    }
    comp_cycle = exynos4_read_count_64() + cycles;
    exynos4_mct_write((u32)comp_cycle, EXYNOS4_MCT_G_COMP0_L);
    exynos4_mct_write((u32)(comp_cycle >> 32), EXYNOS4_MCT_G_COMP0_U);
    exynos4_mct_write(0x1, EXYNOS4_MCT_G_INT_ENB);
    tcon |= MCT_G_TCON_COMP0_ENABLE;
    exynos4_mct_write(tcon , EXYNOS4_MCT_G_TCON);
    }
    static int exynos4_comp_set_next_event(unsigned long cycles,
    struct clock_event_device *evt)
    {
    exynos4_mct_comp0_start(false, cycles);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mct_set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    static int mct_set_state_shutdown(struct clock_event_device *evt)
    {
    exynos4_mct_comp0_stop();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mct_set_state_periodic(evt: *mut clock_event_device) -> c_int {
    static int mct_set_state_periodic(struct clock_event_device *evt)
    {
    unsigned long cycles_per_jiffy;
    cycles_per_jiffy = (((unsigned long long)NSEC_PER_SEC / HZ * evt.mult)
    >> evt.shift);
    exynos4_mct_comp0_stop();
    exynos4_mct_comp0_start(true, cycles_per_jiffy);
    return 0;
    }
    static struct clock_event_device mct_comp_device = {
    .name			= "mct-comp",
    .features		= CLOCK_EVT_FEAT_PERIODIC |
    CLOCK_EVT_FEAT_ONESHOT,
    .rating			= 250,
    .set_next_event		= exynos4_comp_set_next_event,
    .set_state_periodic	= mct_set_state_periodic,
    .set_state_shutdown	= mct_set_state_shutdown,
    .set_state_oneshot	= mct_set_state_shutdown,
    .set_state_oneshot_stopped = mct_set_state_shutdown,
    .tick_resume		= mct_set_state_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn exynos4_mct_comp_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t exynos4_mct_comp_isr(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    exynos4_mct_write(0x1, EXYNOS4_MCT_G_INT_CSTAT);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn exynos4_clockevent_init() -> c_int {
    static int exynos4_clockevent_init(void)
    {
    mct_comp_device.cpumask = cpumask_of(0);
    clockevents_config_and_register(&mct_comp_device, clk_rate,
    0xf, 0xffffffff);
    if (request_irq(mct_irqs[MCT_G0_IRQ], exynos4_mct_comp_isr,
    IRQF_TIMER | IRQF_IRQPOLL, "mct_comp_irq",
    &mct_comp_device))
    pr_err("%s: request_irq() failed\n", "mct_comp_irq");
    return 0;
    }
    static DEFINE_PER_CPU(struct mct_clock_event_device, percpu_mct_tick);
// Clock event handling
#[no_mangle]
unsafe extern "C" fn exynos4_mct_tick_stop(mevt: *mut mct_clock_event_device) {
    static void exynos4_mct_tick_stop(struct mct_clock_event_device *mevt)
    {
    unsigned long tmp;
    let mut mask: c_ulong = MCT_L_TCON_INT_START | MCT_L_TCON_TIMER_START;
    let mut offset: c_ulong = mevt.base + MCT_L_TCON_OFFSET;
    tmp = readl_relaxed(reg_base + offset);
    if (tmp & mask) {
    tmp &= ~mask;
    exynos4_mct_write(tmp, offset);
    }
    }
    static void exynos4_mct_tick_start(unsigned long cycles,
    struct mct_clock_event_device *mevt)
    {
    unsigned long tmp;
    exynos4_mct_tick_stop(mevt);
    tmp = (1 << 31) | cycles;	/* MCT_L_UPDATE_ICNTB */
// update interrupt count buffer
    exynos4_mct_write(tmp, mevt.base + MCT_L_ICNTB_OFFSET);
// enable MCT tick interrupt
    exynos4_mct_write(0x1, mevt.base + MCT_L_INT_ENB_OFFSET);
    tmp = readl_relaxed(reg_base + mevt.base + MCT_L_TCON_OFFSET);
    tmp |= MCT_L_TCON_INT_START | MCT_L_TCON_TIMER_START |
    MCT_L_TCON_INTERVAL_MODE;
    exynos4_mct_write(tmp, mevt.base + MCT_L_TCON_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn exynos4_mct_tick_clear(mevt: *mut mct_clock_event_device) {
    static void exynos4_mct_tick_clear(struct mct_clock_event_device *mevt)
    {
// Clear the MCT tick interrupt
    if (readl_relaxed(reg_base + mevt.base + MCT_L_INT_CSTAT_OFFSET) & 1)
    exynos4_mct_write(0x1, mevt.base + MCT_L_INT_CSTAT_OFFSET);
    }
    static int exynos4_tick_set_next_event(unsigned long cycles,
    struct clock_event_device *evt)
    {
    struct mct_clock_event_device *mevt;
    mevt = container_of(evt, struct mct_clock_event_device, evt);
    exynos4_mct_tick_start(cycles, mevt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    static int set_state_shutdown(struct clock_event_device *evt)
    {
    struct mct_clock_event_device *mevt;
    mevt = container_of(evt, struct mct_clock_event_device, evt);
    exynos4_mct_tick_stop(mevt);
    exynos4_mct_tick_clear(mevt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_state_periodic(evt: *mut clock_event_device) -> c_int {
    static int set_state_periodic(struct clock_event_device *evt)
    {
    struct mct_clock_event_device *mevt;
    unsigned long cycles_per_jiffy;
    mevt = container_of(evt, struct mct_clock_event_device, evt);
    cycles_per_jiffy = (((unsigned long long)NSEC_PER_SEC / HZ * evt.mult)
    >> evt.shift);
    exynos4_mct_tick_stop(mevt);
    exynos4_mct_tick_start(cycles_per_jiffy, mevt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos4_mct_tick_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t exynos4_mct_tick_isr(int irq, void *dev_id)
    {
    struct mct_clock_event_device *mevt = dev_id;
    struct clock_event_device *evt = &mevt.evt;
//
// This is for supporting oneshot mode.
// Mct would generate interrupt periodically
// without explicit stopping.
//
    if (!clockevent_state_periodic(&mevt.evt))
    exynos4_mct_tick_stop(mevt);
    exynos4_mct_tick_clear(mevt);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn exynos4_mct_starting_cpu(cpu: c_uint) -> c_int {
    static int exynos4_mct_starting_cpu(unsigned int cpu)
    {
    struct mct_clock_event_device *mevt =
    per_cpu_ptr(&percpu_mct_tick, cpu);
    struct clock_event_device *evt = &mevt.evt;
    snprintf(mevt.name, sizeof(mevt.name), "mct_tick%d", cpu);
    evt.name = mevt.name;
    evt.cpumask = cpumask_of(cpu);
    evt.set_next_event = exynos4_tick_set_next_event;
    evt.set_state_periodic = set_state_periodic;
    evt.set_state_shutdown = set_state_shutdown;
    evt.set_state_oneshot = set_state_shutdown;
    evt.set_state_oneshot_stopped = set_state_shutdown;
    evt.tick_resume = set_state_shutdown;
    evt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_PERCPU;
    evt.rating = MCT_CLKEVENTS_RATING;
    exynos4_mct_write(TICK_BASE_CNT, mevt.base + MCT_L_TCNTB_OFFSET);
    if (mct_int_type == MCT_INT_SPI) {
    if (evt.irq == -1)
    return -EIO;
    irq_force_affinity(evt.irq, cpumask_of(cpu));
    enable_irq(evt.irq);
    } else {
    enable_percpu_irq(mct_irqs[MCT_L0_IRQ], 0);
    }
    clockevents_config_and_register(evt, clk_rate / (TICK_BASE_CNT + 1),
    0xf, 0x7fffffff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos4_mct_dying_cpu(cpu: c_uint) -> c_int {
    static int exynos4_mct_dying_cpu(unsigned int cpu)
    {
    struct mct_clock_event_device *mevt =
    per_cpu_ptr(&percpu_mct_tick, cpu);
    struct clock_event_device *evt = &mevt.evt;
    if (mct_int_type == MCT_INT_SPI) {
    if (evt.irq != -1)
    disable_irq_nosync(evt.irq);
    exynos4_mct_write(0x1, mevt.base + MCT_L_INT_CSTAT_OFFSET);
    } else {
    disable_percpu_irq(mct_irqs[MCT_L0_IRQ]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exynos4_timer_resources(np: *mut device_node) -> int __init {
    static int __init exynos4_timer_resources(struct device_node *np)
    {
    struct clk *mct_clk, *tick_clk;
    reg_base = of_iomap(np, 0);
    if (!reg_base)
    panic("%s: unable to ioremap mct address space\n", __func__);
    tick_clk = of_clk_get_by_name(np, "fin_pll");
    if (IS_ERR(tick_clk))
    panic("%s: unable to determine tick clock rate\n", __func__);
    clk_rate = clk_get_rate(tick_clk);
    mct_clk = of_clk_get_by_name(np, "mct");
    if (IS_ERR(mct_clk))
    panic("%s: unable to retrieve mct clock instance\n", __func__);
    clk_prepare_enable(mct_clk);
    return 0;
    }
//
// exynos4_timer_interrupts - initialize MCT interrupts
// @np: device node for MCT
// @int_type: interrupt type, MCT_INT_PPI or MCT_INT_SPI
// @local_idx: array mapping CPU numbers to local timer indices
// @nr_local: size of @local_idx array
//
    static int __init exynos4_timer_interrupts(struct device_node *np,
    unsigned int int_type,
    const u32 *local_idx,
    size_t nr_local)
    {
    int nr_irqs, i, err, cpu;
    mct_int_type = int_type;
// This driver uses only one global timer interrupt
    mct_irqs[MCT_G0_IRQ] = irq_of_parse_and_map(np, MCT_G0_IRQ);
//
// Find out the number of local irqs specified. The local
// timer irqs are specified after the four global timer
// irqs are specified.
//
    nr_irqs = of_irq_count(np);
    if (nr_irqs > ARRAY_SIZE(mct_irqs)) {
    pr_err("exynos-mct: too many (%d) interrupts configured in DT\n",
    nr_irqs);
    nr_irqs = ARRAY_SIZE(mct_irqs);
    }
    for (i = MCT_L0_IRQ; i < nr_irqs; i++)
    mct_irqs[i] = irq_of_parse_and_map(np, i);
    if (mct_int_type == MCT_INT_PPI) {
    err = request_percpu_irq(mct_irqs[MCT_L0_IRQ],
    exynos4_mct_tick_isr, "MCT",
    &percpu_mct_tick);
    WARN(err, "MCT: can't request IRQ %d (%d)\n",
    mct_irqs[MCT_L0_IRQ], err);
    } else {
    for_each_possible_cpu(cpu) {
    int mct_irq;
    unsigned int irq_idx;
    struct mct_clock_event_device *pcpu_mevt =
    per_cpu_ptr(&percpu_mct_tick, cpu);
    if (cpu >= nr_local) {
    err = -EINVAL;
    goto out_irq;
    }
    irq_idx = MCT_L0_IRQ + local_idx[cpu];
    pcpu_mevt.evt.irq = -1;
    if (irq_idx >= ARRAY_SIZE(mct_irqs))
    break;
    mct_irq = mct_irqs[irq_idx];
    irq_set_status_flags(mct_irq, IRQ_NOAUTOEN);
    if (request_irq(mct_irq,
    exynos4_mct_tick_isr,
    IRQF_TIMER | IRQF_NOBALANCING,
    pcpu_mevt.name, pcpu_mevt)) {
    pr_err("exynos-mct: cannot register IRQ (cpu%d)\n",
    cpu);
    continue;
    }
    pcpu_mevt.evt.irq = mct_irq;
    }
    }
    for_each_possible_cpu(cpu) {
    struct mct_clock_event_device *mevt = per_cpu_ptr(&percpu_mct_tick, cpu);
    if (cpu >= nr_local) {
    err = -EINVAL;
    goto out_irq;
    }
    mevt.base = EXYNOS4_MCT_L_BASE(local_idx[cpu]);
    }
// Install hotplug callbacks which configure the timer on this CPU
    err = cpuhp_setup_state(CPUHP_AP_EXYNOS4_MCT_TIMER_STARTING,
    "clockevents/exynos4/mct_timer:starting",
    exynos4_mct_starting_cpu,
    exynos4_mct_dying_cpu);
    if (err)
    goto out_irq;
    return 0;
    out_irq:
    if (mct_int_type == MCT_INT_PPI) {
    free_percpu_irq(mct_irqs[MCT_L0_IRQ], &percpu_mct_tick);
    } else {
    for_each_possible_cpu(cpu) {
    struct mct_clock_event_device *pcpu_mevt =
    per_cpu_ptr(&percpu_mct_tick, cpu);
    if (pcpu_mevt.evt.irq != -1) {
    free_irq(pcpu_mevt.evt.irq, pcpu_mevt);
    pcpu_mevt.evt.irq = -1;
    }
    }
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mct_init_dt(np: *mut device_node, int_type: c_uint) -> int __init {
    static int __init mct_init_dt(struct device_node *np, unsigned int int_type)
    {
    let mut frc_shared: bool = of_property_read_bool(np, "samsung,frc-shared");
    u32 local_idx[MCT_NR_LOCAL] = {0};
    int nr_local;
    int ret;
    nr_local = of_property_count_u32_elems(np, "samsung,local-timers");
    if (nr_local == 0)
    return -EINVAL;
    if (nr_local > 0) {
    if (nr_local > ARRAY_SIZE(local_idx))
    return -EINVAL;
    ret = of_property_read_u32_array(np, "samsung,local-timers",
    local_idx, nr_local);
    if (ret)
    return ret;
    } else {
    int i;
    nr_local = ARRAY_SIZE(local_idx);
    for (i = 0; i < nr_local; i++)
    local_idx[i] = i;
    }
    ret = exynos4_timer_resources(np);
    if (ret)
    return ret;
    ret = exynos4_timer_interrupts(np, int_type, local_idx, nr_local);
    if (ret)
    return ret;
    ret = exynos4_clocksource_init(frc_shared);
    if (ret)
    return ret;
//
// When the FRC is shared with a main processor, this secondary
// processor cannot use the global comparator.
//
    if (frc_shared)
    return 0;
    return exynos4_clockevent_init();
    }
#[no_mangle]
unsafe extern "C" fn mct_init_spi(np: *mut device_node) -> int __init {
    static int __init mct_init_spi(struct device_node *np)
    {
    return mct_init_dt(np, MCT_INT_SPI);
    }
#[no_mangle]
unsafe extern "C" fn mct_init_ppi(np: *mut device_node) -> int __init {
    static int __init mct_init_ppi(struct device_node *np)
    {
    return mct_init_dt(np, MCT_INT_PPI);
    }
    TIMER_OF_DECLARE(exynos4210, "samsung,exynos4210-mct", mct_init_spi);
    TIMER_OF_DECLARE(exynos4412, "samsung,exynos4412-mct", mct_init_ppi);
