//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/sh_tmu.c
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
// SuperH Timer Support - TMU
//
// Copyright (C) 2009 Magnus Damm
//

    enum sh_tmu_model {
    SH_TMU,
    SH_TMU_SH3,
    };
    struct sh_tmu_device;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_tmu_channel {
    pub tmu: *mut sh_tmu_device,
    pub index: c_uint,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub periodic: c_ulong,
    pub ced: clock_event_device,
    pub cs: clocksource,
    pub cs_enabled: bool,
    pub enable_count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_tmu_device {
    pub pdev: *mut platform_device,
    pub mapbase: *mut void __iomem,
    pub clk: *mut clk,
    pub rate: c_ulong,
    pub model: enum sh_tmu_model,
    pub /: *mut *mut raw_spinlock_t lock; / Protect the shared start/stop register,
    pub channels: *mut sh_tmu_channel,
    pub num_channels: c_uint,
    pub has_clockevent: bool,
    pub has_clocksource: bool,
}

#[no_mangle]
pub unsafe extern "C" fn sh_tmu_read(ch: *mut sh_tmu_channel, reg_nr: c_int) -> c_ulong {
    static inline unsigned long sh_tmu_read(struct sh_tmu_channel *ch, int reg_nr)
    {
    unsigned long offs;
    if (reg_nr == TSTR) {
    switch (ch.tmu.model) {
    case SH_TMU_SH3:
    return ioread8(ch.tmu.mapbase + 2);
    case SH_TMU:
    return ioread8(ch.tmu.mapbase + 4);
    }
    }
    offs = reg_nr << 2;
    if (reg_nr == TCR)
    return ioread16(ch.base + offs);
    else
    return ioread32(ch.base + offs);
    }
    static inline void sh_tmu_write(struct sh_tmu_channel *ch, int reg_nr,
    unsigned long value)
    {
    unsigned long offs;
    if (reg_nr == TSTR) {
    switch (ch.tmu.model) {
    case SH_TMU_SH3:
    return iowrite8(value, ch.tmu.mapbase + 2);
    case SH_TMU:
    return iowrite8(value, ch.tmu.mapbase + 4);
    }
    }
    offs = reg_nr << 2;
    if (reg_nr == TCR)
    iowrite16(value, ch.base + offs);
    else
    iowrite32(value, ch.base + offs);
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_start_stop_ch(ch: *mut sh_tmu_channel, start: c_int) {
    static void sh_tmu_start_stop_ch(struct sh_tmu_channel *ch, int start)
    {
    unsigned long flags, value;
// start stop register shared by multiple timer channels
    raw_spin_lock_irqsave(&ch.tmu.lock, flags);
    value = sh_tmu_read(ch, TSTR);
    if (start)
    value |= 1 << ch.index;
    else
    value &= ~(1 << ch.index);
    sh_tmu_write(ch, TSTR, value);
    raw_spin_unlock_irqrestore(&ch.tmu.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn __sh_tmu_enable(ch: *mut sh_tmu_channel) -> c_int {
    static int __sh_tmu_enable(struct sh_tmu_channel *ch)
    {
// make sure channel is disabled
    sh_tmu_start_stop_ch(ch, 0);
// maximum timeout
    sh_tmu_write(ch, TCOR, 0xffffffff);
    sh_tmu_write(ch, TCNT, 0xffffffff);
// configure channel to parent clock / 4, irq off
    sh_tmu_write(ch, TCR, TCR_TPSC_CLK4);
// enable channel
    sh_tmu_start_stop_ch(ch, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_enable(ch: *mut sh_tmu_channel) -> c_int {
    static int sh_tmu_enable(struct sh_tmu_channel *ch)
    {
    if (ch.enable_count++ > 0)
    return 0;
    dev_pm_syscore_device(&ch.tmu.pdev.dev, true);
    return __sh_tmu_enable(ch);
    }
#[no_mangle]
unsafe extern "C" fn __sh_tmu_disable(ch: *mut sh_tmu_channel) {
    static void __sh_tmu_disable(struct sh_tmu_channel *ch)
    {
// disable channel
    sh_tmu_start_stop_ch(ch, 0);
// disable interrupts in TMU block
    sh_tmu_write(ch, TCR, TCR_TPSC_CLK4);
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_disable(ch: *mut sh_tmu_channel) {
    static void sh_tmu_disable(struct sh_tmu_channel *ch)
    {
    if (WARN_ON(ch.enable_count == 0))
    return;
    if (--ch.enable_count > 0)
    return;
    __sh_tmu_disable(ch);
    dev_pm_syscore_device(&ch.tmu.pdev.dev, false);
    }
    static void sh_tmu_set_next(struct sh_tmu_channel *ch, unsigned long delta,
    int periodic)
    {
// stop timer
    sh_tmu_start_stop_ch(ch, 0);
// acknowledge interrupt
    sh_tmu_read(ch, TCR);
// enable interrupt
    sh_tmu_write(ch, TCR, TCR_UNIE | TCR_TPSC_CLK4);
// reload delta value in case of periodic timer
    if (periodic)
    sh_tmu_write(ch, TCOR, delta);
    else
    sh_tmu_write(ch, TCOR, 0xffffffff);
    sh_tmu_write(ch, TCNT, delta);
// start timer
    sh_tmu_start_stop_ch(ch, 1);
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sh_tmu_interrupt(int irq, void *dev_id)
    {
    struct sh_tmu_channel *ch = dev_id;
// disable or acknowledge interrupt
    if (clockevent_state_oneshot(&ch.ced))
    sh_tmu_write(ch, TCR, TCR_TPSC_CLK4);
    else
    sh_tmu_write(ch, TCR, TCR_UNIE | TCR_TPSC_CLK4);
// notify clockevent layer
    ch.ced.event_handler(&ch.ced);
    return IRQ_HANDLED;
    }
    static struct sh_tmu_channel *cs_to_sh_tmu(struct clocksource *cs)
    {
    return container_of(cs, struct sh_tmu_channel, cs);
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clocksource_read(cs: *mut clocksource) -> u64 {
    static u64 sh_tmu_clocksource_read(struct clocksource *cs)
    {
    struct sh_tmu_channel *ch = cs_to_sh_tmu(cs);
    return sh_tmu_read(ch, TCNT) ^ 0xffffffff;
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clocksource_enable(cs: *mut clocksource) -> c_int {
    static int sh_tmu_clocksource_enable(struct clocksource *cs)
    {
    struct sh_tmu_channel *ch = cs_to_sh_tmu(cs);
    int ret;
    if (WARN_ON(ch.cs_enabled))
    return 0;
    ret = sh_tmu_enable(ch);
    if (!ret)
    ch.cs_enabled = true;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clocksource_disable(cs: *mut clocksource) {
    static void sh_tmu_clocksource_disable(struct clocksource *cs)
    {
    struct sh_tmu_channel *ch = cs_to_sh_tmu(cs);
    if (WARN_ON(!ch.cs_enabled))
    return;
    sh_tmu_disable(ch);
    ch.cs_enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clocksource_suspend(cs: *mut clocksource) {
    static void sh_tmu_clocksource_suspend(struct clocksource *cs)
    {
    struct sh_tmu_channel *ch = cs_to_sh_tmu(cs);
    if (!ch.cs_enabled)
    return;
    if (--ch.enable_count == 0) {
    __sh_tmu_disable(ch);
    dev_pm_genpd_suspend(&ch.tmu.pdev.dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clocksource_resume(cs: *mut clocksource) {
    static void sh_tmu_clocksource_resume(struct clocksource *cs)
    {
    struct sh_tmu_channel *ch = cs_to_sh_tmu(cs);
    if (!ch.cs_enabled)
    return;
    if (ch.enable_count++ == 0) {
    dev_pm_genpd_resume(&ch.tmu.pdev.dev);
    __sh_tmu_enable(ch);
    }
    }
    static int sh_tmu_register_clocksource(struct sh_tmu_channel *ch,
    const char *name)
    {
    struct clocksource *cs = &ch.cs;
    cs.name = name;
    cs.rating = 200;
    cs.read = sh_tmu_clocksource_read;
    cs.enable = sh_tmu_clocksource_enable;
    cs.disable = sh_tmu_clocksource_disable;
    cs.suspend = sh_tmu_clocksource_suspend;
    cs.resume = sh_tmu_clocksource_resume;
    cs.mask = CLOCKSOURCE_MASK(32);
    cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    dev_info(&ch.tmu.pdev.dev, "ch%u: used as clock source\n",
    ch.index);
    clocksource_register_hz(cs, ch.tmu.rate);
    return 0;
    }
    static struct sh_tmu_channel *ced_to_sh_tmu(struct clock_event_device *ced)
    {
    return container_of(ced, struct sh_tmu_channel, ced);
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clock_event_start(ch: *mut sh_tmu_channel, periodic: c_int) {
    static void sh_tmu_clock_event_start(struct sh_tmu_channel *ch, int periodic)
    {
    sh_tmu_enable(ch);
    if (periodic) {
    ch.periodic = (ch.tmu.rate + HZ/2) / HZ;
    sh_tmu_set_next(ch, ch.periodic, 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clock_event_shutdown(ced: *mut clock_event_device) -> c_int {
    static int sh_tmu_clock_event_shutdown(struct clock_event_device *ced)
    {
    struct sh_tmu_channel *ch = ced_to_sh_tmu(ced);
    if (clockevent_state_oneshot(ced) || clockevent_state_periodic(ced))
    sh_tmu_disable(ch);
    return 0;
    }
    static int sh_tmu_clock_event_set_state(struct clock_event_device *ced,
    int periodic)
    {
    struct sh_tmu_channel *ch = ced_to_sh_tmu(ced);
// deal with old setting first
    if (clockevent_state_oneshot(ced) || clockevent_state_periodic(ced))
    sh_tmu_disable(ch);
    dev_info(&ch.tmu.pdev.dev, "ch%u: used for %s clock events\n",
    ch.index, periodic ? "periodic" : "oneshot");
    sh_tmu_clock_event_start(ch, periodic);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clock_event_set_oneshot(ced: *mut clock_event_device) -> c_int {
    static int sh_tmu_clock_event_set_oneshot(struct clock_event_device *ced)
    {
    return sh_tmu_clock_event_set_state(ced, 0);
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clock_event_set_periodic(ced: *mut clock_event_device) -> c_int {
    static int sh_tmu_clock_event_set_periodic(struct clock_event_device *ced)
    {
    return sh_tmu_clock_event_set_state(ced, 1);
    }
    static int sh_tmu_clock_event_next(unsigned long delta,
    struct clock_event_device *ced)
    {
    struct sh_tmu_channel *ch = ced_to_sh_tmu(ced);
    BUG_ON(!clockevent_state_oneshot(ced));
// program new delta value
    sh_tmu_set_next(ch, delta, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clock_event_suspend(ced: *mut clock_event_device) {
    static void sh_tmu_clock_event_suspend(struct clock_event_device *ced)
    {
    dev_pm_genpd_suspend(&ced_to_sh_tmu(ced).tmu.pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_clock_event_resume(ced: *mut clock_event_device) {
    static void sh_tmu_clock_event_resume(struct clock_event_device *ced)
    {
    dev_pm_genpd_resume(&ced_to_sh_tmu(ced).tmu.pdev.dev);
    }
    static void sh_tmu_register_clockevent(struct sh_tmu_channel *ch,
    const char *name)
    {
    struct clock_event_device *ced = &ch.ced;
    int ret;
    ced.name = name;
    ced.features = CLOCK_EVT_FEAT_PERIODIC;
    ced.features |= CLOCK_EVT_FEAT_ONESHOT;
    ced.rating = 200;
    ced.cpumask = cpu_possible_mask;
    ced.set_next_event = sh_tmu_clock_event_next;
    ced.set_state_shutdown = sh_tmu_clock_event_shutdown;
    ced.set_state_periodic = sh_tmu_clock_event_set_periodic;
    ced.set_state_oneshot = sh_tmu_clock_event_set_oneshot;
    ced.suspend = sh_tmu_clock_event_suspend;
    ced.resume = sh_tmu_clock_event_resume;
    dev_info(&ch.tmu.pdev.dev, "ch%u: used for clock events\n",
    ch.index);
    clockevents_config_and_register(ced, ch.tmu.rate, 0x300, 0xffffffff);
    ret = request_irq(ch.irq, sh_tmu_interrupt,
    IRQF_TIMER | IRQF_IRQPOLL | IRQF_NOBALANCING,
    dev_name(&ch.tmu.pdev.dev), ch);
    if (ret) {
    dev_err(&ch.tmu.pdev.dev, "ch%u: failed to request irq %d\n",
    ch.index, ch.irq);
    return;
    }
    }
    static int sh_tmu_register(struct sh_tmu_channel *ch, const char *name,
    bool clockevent, bool clocksource)
    {
    if (clockevent) {
    ch.tmu.has_clockevent = true;
    sh_tmu_register_clockevent(ch, name);
    } else if (clocksource) {
    ch.tmu.has_clocksource = true;
    sh_tmu_register_clocksource(ch, name);
    }
    return 0;
    }
    static int sh_tmu_channel_setup(struct sh_tmu_channel *ch, unsigned int index,
    bool clockevent, bool clocksource,
    struct sh_tmu_device *tmu)
    {
// Skip unused channels.
    if (!clockevent && !clocksource)
    return 0;
    ch.tmu = tmu;
    ch.index = index;
    if (tmu.model == SH_TMU_SH3)
    ch.base = tmu.mapbase + 4 + ch.index * 12;
    else
    ch.base = tmu.mapbase + 8 + ch.index * 12;
    ch.irq = platform_get_irq(tmu.pdev, index);
    if (ch.irq < 0)
    return ch.irq;
    ch.cs_enabled = false;
    ch.enable_count = 0;
    return sh_tmu_register(ch, dev_name(&tmu.pdev.dev),
    clockevent, clocksource);
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_map_memory(tmu: *mut sh_tmu_device) -> c_int {
    static int sh_tmu_map_memory(struct sh_tmu_device *tmu)
    {
    struct resource *res;
    res = platform_get_resource(tmu.pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&tmu.pdev.dev, "failed to get I/O memory\n");
    return -ENXIO;
    }
    tmu.mapbase = ioremap(res.start, resource_size(res));
    if (tmu.mapbase == core::ptr::null_mut())
    return -ENXIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_parse_dt(tmu: *mut sh_tmu_device) -> c_int {
    static int sh_tmu_parse_dt(struct sh_tmu_device *tmu)
    {
    struct device_node *np = tmu.pdev.dev.of_node;
    tmu.model = SH_TMU;
    tmu.num_channels = 3;
    of_property_read_u32(np, "#renesas,channels", &tmu.num_channels);
    if (tmu.num_channels != 2 && tmu.num_channels != 3) {
    dev_err(&tmu.pdev.dev, "invalid number of channels %u\n",
    tmu.num_channels);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_setup(tmu: *mut sh_tmu_device, pdev: *mut platform_device) -> c_int {
    static int sh_tmu_setup(struct sh_tmu_device *tmu, struct platform_device *pdev)
    {
    unsigned int i;
    int ret;
    tmu.pdev = pdev;
    raw_spin_lock_init(&tmu.lock);
    if (IS_ENABLED(CONFIG_OF) && pdev.dev.of_node) {
    ret = sh_tmu_parse_dt(tmu);
    if (ret < 0)
    return ret;
    } else if (pdev.dev.platform_data) {
    const struct platform_device_id *id = pdev.id_entry;
    struct sh_timer_config *cfg = pdev.dev.platform_data;
    tmu.model = id.driver_data;
    tmu.num_channels = hweight8(cfg.channels_mask);
    } else {
    dev_err(&tmu.pdev.dev, "missing platform data\n");
    return -ENXIO;
    }
// Get hold of clock.
    tmu.clk = clk_get(&tmu.pdev.dev, "fck");
    if (IS_ERR(tmu.clk)) {
    dev_err(&tmu.pdev.dev, "cannot get clock\n");
    return PTR_ERR(tmu.clk);
    }
    ret = clk_prepare(tmu.clk);
    if (ret < 0)
    goto err_clk_put;
// Determine clock rate.
    ret = clk_enable(tmu.clk);
    if (ret < 0)
    goto err_clk_unprepare;
    tmu.rate = clk_get_rate(tmu.clk) / 4;
// Map the memory resource.
    ret = sh_tmu_map_memory(tmu);
    if (ret < 0) {
    dev_err(&tmu.pdev.dev, "failed to remap I/O memory\n");
    goto err_clk_unprepare;
    }
// Allocate and setup the channels.
    tmu.channels = kzalloc_objs(*tmu.channels, tmu.num_channels);
    if (tmu.channels == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err_unmap;
    }
//
// Use the first channel as a clock event device and the second channel
// as a clock source.
//
    for (i = 0; i < tmu.num_channels; ++i) {
    ret = sh_tmu_channel_setup(&tmu.channels[i], i,
    i == 0, i == 1, tmu);
    if (ret < 0)
    goto err_unmap;
    }
    platform_set_drvdata(pdev, tmu);
    return 0;
    err_unmap:
    kfree(tmu.channels);
    iounmap(tmu.mapbase);
    err_clk_unprepare:
    clk_unprepare(tmu.clk);
    err_clk_put:
    clk_put(tmu.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_probe(pdev: *mut platform_device) -> c_int {
    static int sh_tmu_probe(struct platform_device *pdev)
    {
    struct sh_tmu_device *tmu = platform_get_drvdata(pdev);
    int ret;
    if (!is_sh_early_platform_device(pdev)) {
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    }
    if (tmu) {
    dev_info(&pdev.dev, "kept as earlytimer\n");
    goto out;
    }
    tmu = kzalloc_obj(*tmu);
    if (tmu == core::ptr::null_mut())
    return -ENOMEM;
    ret = sh_tmu_setup(tmu, pdev);
    if (ret) {
    kfree(tmu);
    pm_runtime_idle(&pdev.dev);
    return ret;
    }
    if (is_sh_early_platform_device(pdev))
    return 0;
    out:
    if (tmu.has_clockevent || tmu.has_clocksource)
    pm_runtime_irq_safe(&pdev.dev);
    return 0;
    }
    static const struct platform_device_id sh_tmu_id_table[] = {
    { .name = "sh-tmu", .driver_data = SH_TMU },
    { .name = "sh-tmu-sh3", .driver_data = SH_TMU_SH3 },
    { }
    };
    MODULE_DEVICE_TABLE(platform, sh_tmu_id_table);
    static const struct of_device_id sh_tmu_of_table[] __maybe_unused = {
    { .compatible = "renesas,tmu" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sh_tmu_of_table);
    static struct platform_driver sh_tmu_device_driver = {
    .probe		= sh_tmu_probe,
    .driver		= {
    .name	= "sh_tmu",
    .of_match_table = of_match_ptr(sh_tmu_of_table),
    .suppress_bind_attrs = true,
    },
    .id_table	= sh_tmu_id_table,
    };
#[no_mangle]
unsafe extern "C" fn sh_tmu_init() -> int __init {
    static int __init sh_tmu_init(void)
    {
    return platform_driver_register(&sh_tmu_device_driver);
    }
#[no_mangle]
unsafe extern "C" fn sh_tmu_exit() -> void __exit {
    static void __exit sh_tmu_exit(void)
    {
    platform_driver_unregister(&sh_tmu_device_driver);
    }

    sh_early_platform_init("earlytimer", &sh_tmu_device_driver);

    subsys_initcall(sh_tmu_init);
    module_exit(sh_tmu_exit);
    MODULE_AUTHOR("Magnus Damm");
    MODULE_DESCRIPTION("SuperH TMU Timer Driver");
