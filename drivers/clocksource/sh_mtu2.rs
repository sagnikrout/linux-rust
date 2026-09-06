//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/sh_mtu2.c
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
// SuperH Timer Support - MTU2
//
// Copyright (C) 2009 Magnus Damm
//

    struct sh_mtu2_device;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mtu2_channel {
    pub mtu: *mut sh_mtu2_device,
    pub index: c_uint,
    pub base: *mut void __iomem,
    pub ced: clock_event_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mtu2_device {
    pub pdev: *mut platform_device,
    pub mapbase: *mut void __iomem,
    pub clk: *mut clk,
    pub /: *mut *mut raw_spinlock_t lock; / Protect the shared registers,
    pub channels: *mut sh_mtu2_channel,
    pub num_channels: c_uint,
    pub has_clockevent: bool,
}

// Values 4 to 7 are channel-dependent

    static unsigned long mtu2_reg_offs[] = {
    [TCR] = 0,
    [TMDR] = 1,
    [TIOR] = 2,
    [TIER] = 4,
    [TSR] = 5,
    [TCNT] = 6,
    [TGR] = 8,
    };
#[no_mangle]
pub unsafe extern "C" fn sh_mtu2_read(ch: *mut sh_mtu2_channel, reg_nr: c_int) -> c_ulong {
    static inline unsigned long sh_mtu2_read(struct sh_mtu2_channel *ch, int reg_nr)
    {
    unsigned long offs;
    if (reg_nr == TSTR)
    return ioread8(ch.mtu.mapbase + 0x280);
    offs = mtu2_reg_offs[reg_nr];
    if ((reg_nr == TCNT) || (reg_nr == TGR))
    return ioread16(ch.base + offs);
    else
    return ioread8(ch.base + offs);
    }
    static inline void sh_mtu2_write(struct sh_mtu2_channel *ch, int reg_nr,
    unsigned long value)
    {
    unsigned long offs;
    if (reg_nr == TSTR)
    return iowrite8(value, ch.mtu.mapbase + 0x280);
    offs = mtu2_reg_offs[reg_nr];
    if ((reg_nr == TCNT) || (reg_nr == TGR))
    iowrite16(value, ch.base + offs);
    else
    iowrite8(value, ch.base + offs);
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_start_stop_ch(ch: *mut sh_mtu2_channel, start: c_int) {
    static void sh_mtu2_start_stop_ch(struct sh_mtu2_channel *ch, int start)
    {
    unsigned long flags, value;
// start stop register shared by multiple timer channels
    raw_spin_lock_irqsave(&ch.mtu.lock, flags);
    value = sh_mtu2_read(ch, TSTR);
    if (start)
    value |= 1 << ch.index;
    else
    value &= ~(1 << ch.index);
    sh_mtu2_write(ch, TSTR, value);
    raw_spin_unlock_irqrestore(&ch.mtu.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_enable(ch: *mut sh_mtu2_channel) -> c_int {
    static int sh_mtu2_enable(struct sh_mtu2_channel *ch)
    {
    unsigned long periodic;
    unsigned long rate;
    int ret;
    pm_runtime_get_sync(&ch.mtu.pdev.dev);
    dev_pm_syscore_device(&ch.mtu.pdev.dev, true);
// enable clock
    ret = clk_enable(ch.mtu.clk);
    if (ret) {
    dev_err(&ch.mtu.pdev.dev, "ch%u: cannot enable clock\n",
    ch.index);
    return ret;
    }
// make sure channel is disabled
    sh_mtu2_start_stop_ch(ch, 0);
    rate = clk_get_rate(ch.mtu.clk) / 64;
    periodic = (rate + HZ/2) / HZ;
//
// "Periodic Counter Operation"
// Clear on TGRA compare match, divide clock by 64.
//
    sh_mtu2_write(ch, TCR, TCR_CCLR_TGRA | TCR_TPSC_P64);
    sh_mtu2_write(ch, TIOR, TIOC_IOCH(TIOR_OC_0_CLEAR) |
    TIOC_IOCL(TIOR_OC_0_CLEAR));
    sh_mtu2_write(ch, TGR, periodic);
    sh_mtu2_write(ch, TCNT, 0);
    sh_mtu2_write(ch, TMDR, TMDR_MD_NORMAL);
    sh_mtu2_write(ch, TIER, TIER_TGIEA);
// enable channel
    sh_mtu2_start_stop_ch(ch, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_disable(ch: *mut sh_mtu2_channel) {
    static void sh_mtu2_disable(struct sh_mtu2_channel *ch)
    {
// disable channel
    sh_mtu2_start_stop_ch(ch, 0);
// stop clock
    clk_disable(ch.mtu.clk);
    dev_pm_syscore_device(&ch.mtu.pdev.dev, false);
    pm_runtime_put(&ch.mtu.pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sh_mtu2_interrupt(int irq, void *dev_id)
    {
    struct sh_mtu2_channel *ch = dev_id;
// acknowledge interrupt
    sh_mtu2_read(ch, TSR);
    sh_mtu2_write(ch, TSR, ~TSR_TGFA);
// notify clockevent layer
    ch.ced.event_handler(&ch.ced);
    return IRQ_HANDLED;
    }
    static struct sh_mtu2_channel *ced_to_sh_mtu2(struct clock_event_device *ced)
    {
    return container_of(ced, struct sh_mtu2_channel, ced);
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_clock_event_shutdown(ced: *mut clock_event_device) -> c_int {
    static int sh_mtu2_clock_event_shutdown(struct clock_event_device *ced)
    {
    struct sh_mtu2_channel *ch = ced_to_sh_mtu2(ced);
    if (clockevent_state_periodic(ced))
    sh_mtu2_disable(ch);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_clock_event_set_periodic(ced: *mut clock_event_device) -> c_int {
    static int sh_mtu2_clock_event_set_periodic(struct clock_event_device *ced)
    {
    struct sh_mtu2_channel *ch = ced_to_sh_mtu2(ced);
    if (clockevent_state_periodic(ced))
    sh_mtu2_disable(ch);
    dev_info(&ch.mtu.pdev.dev, "ch%u: used for periodic clock events\n",
    ch.index);
    sh_mtu2_enable(ch);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_clock_event_suspend(ced: *mut clock_event_device) {
    static void sh_mtu2_clock_event_suspend(struct clock_event_device *ced)
    {
    dev_pm_genpd_suspend(&ced_to_sh_mtu2(ced).mtu.pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_clock_event_resume(ced: *mut clock_event_device) {
    static void sh_mtu2_clock_event_resume(struct clock_event_device *ced)
    {
    dev_pm_genpd_resume(&ced_to_sh_mtu2(ced).mtu.pdev.dev);
    }
    static void sh_mtu2_register_clockevent(struct sh_mtu2_channel *ch,
    const char *name)
    {
    struct clock_event_device *ced = &ch.ced;
    ced.name = name;
    ced.features = CLOCK_EVT_FEAT_PERIODIC;
    ced.rating = 200;
    ced.cpumask = cpu_possible_mask;
    ced.set_state_shutdown = sh_mtu2_clock_event_shutdown;
    ced.set_state_periodic = sh_mtu2_clock_event_set_periodic;
    ced.suspend = sh_mtu2_clock_event_suspend;
    ced.resume = sh_mtu2_clock_event_resume;
    dev_info(&ch.mtu.pdev.dev, "ch%u: used for clock events\n",
    ch.index);
    clockevents_register_device(ced);
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_register(ch: *mut sh_mtu2_channel, name: *const c_char) -> c_int {
    static int sh_mtu2_register(struct sh_mtu2_channel *ch, const char *name)
    {
    ch.mtu.has_clockevent = true;
    sh_mtu2_register_clockevent(ch, name);
    return 0;
    }
    static const unsigned int sh_mtu2_channel_offsets[] = {
    0x300, 0x380, 0x000,
    };
    static int sh_mtu2_setup_channel(struct sh_mtu2_channel *ch, unsigned int index,
    struct sh_mtu2_device *mtu)
    {
    char name[6];
    int irq;
    int ret;
    ch.mtu = mtu;
    sprintf(name, "tgi%ua", index);
    irq = platform_get_irq_byname(mtu.pdev, name);
    if (irq < 0) {
// Skip channels with no declared interrupt.
    return 0;
    }
    ret = request_irq(irq, sh_mtu2_interrupt,
    IRQF_TIMER | IRQF_IRQPOLL | IRQF_NOBALANCING,
    dev_name(&ch.mtu.pdev.dev), ch);
    if (ret) {
    dev_err(&ch.mtu.pdev.dev, "ch%u: failed to request irq %d\n",
    index, irq);
    return ret;
    }
    ch.base = mtu.mapbase + sh_mtu2_channel_offsets[index];
    ch.index = index;
    return sh_mtu2_register(ch, dev_name(&mtu.pdev.dev));
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_map_memory(mtu: *mut sh_mtu2_device) -> c_int {
    static int sh_mtu2_map_memory(struct sh_mtu2_device *mtu)
    {
    struct resource *res;
    res = platform_get_resource(mtu.pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&mtu.pdev.dev, "failed to get I/O memory\n");
    return -ENXIO;
    }
    mtu.mapbase = ioremap(res.start, resource_size(res));
    if (mtu.mapbase == core::ptr::null_mut())
    return -ENXIO;
    return 0;
    }
    static int sh_mtu2_setup(struct sh_mtu2_device *mtu,
    struct platform_device *pdev)
    {
    unsigned int i;
    int ret;
    mtu.pdev = pdev;
    raw_spin_lock_init(&mtu.lock);
// Get hold of clock.
    mtu.clk = clk_get(&mtu.pdev.dev, "fck");
    if (IS_ERR(mtu.clk)) {
    dev_err(&mtu.pdev.dev, "cannot get clock\n");
    return PTR_ERR(mtu.clk);
    }
    ret = clk_prepare(mtu.clk);
    if (ret < 0)
    goto err_clk_put;
// Map the memory resource.
    ret = sh_mtu2_map_memory(mtu);
    if (ret < 0) {
    dev_err(&mtu.pdev.dev, "failed to remap I/O memory\n");
    goto err_clk_unprepare;
    }
// Allocate and setup the channels.
    ret = platform_irq_count(pdev);
    if (ret < 0)
    goto err_unmap;
    mtu.num_channels = min_t(unsigned int, ret,
    ARRAY_SIZE(sh_mtu2_channel_offsets));
    mtu.channels = kzalloc_objs(*mtu.channels, mtu.num_channels);
    if (mtu.channels == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err_unmap;
    }
    for (i = 0; i < mtu.num_channels; ++i) {
    ret = sh_mtu2_setup_channel(&mtu.channels[i], i, mtu);
    if (ret < 0)
    goto err_unmap;
    }
    platform_set_drvdata(pdev, mtu);
    return 0;
    err_unmap:
    kfree(mtu.channels);
    iounmap(mtu.mapbase);
    err_clk_unprepare:
    clk_unprepare(mtu.clk);
    err_clk_put:
    clk_put(mtu.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_probe(pdev: *mut platform_device) -> c_int {
    static int sh_mtu2_probe(struct platform_device *pdev)
    {
    struct sh_mtu2_device *mtu = platform_get_drvdata(pdev);
    int ret;
    if (!is_sh_early_platform_device(pdev)) {
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    }
    if (mtu) {
    dev_info(&pdev.dev, "kept as earlytimer\n");
    goto out;
    }
    mtu = kzalloc_obj(*mtu);
    if (mtu == core::ptr::null_mut())
    return -ENOMEM;
    ret = sh_mtu2_setup(mtu, pdev);
    if (ret) {
    kfree(mtu);
    pm_runtime_idle(&pdev.dev);
    return ret;
    }
    if (is_sh_early_platform_device(pdev))
    return 0;
    out:
    if (mtu.has_clockevent)
    pm_runtime_irq_safe(&pdev.dev);
    else
    pm_runtime_idle(&pdev.dev);
    return 0;
    }
    static const struct platform_device_id sh_mtu2_id_table[] = {
    { .name = "sh-mtu2" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, sh_mtu2_id_table);
    static const struct of_device_id sh_mtu2_of_table[] __maybe_unused = {
    { .compatible = "renesas,mtu2" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sh_mtu2_of_table);
    static struct platform_driver sh_mtu2_device_driver = {
    .probe		= sh_mtu2_probe,
    .driver		= {
    .name	= "sh_mtu2",
    .of_match_table = of_match_ptr(sh_mtu2_of_table),
    .suppress_bind_attrs = true,
    },
    .id_table	= sh_mtu2_id_table,
    };
#[no_mangle]
unsafe extern "C" fn sh_mtu2_init() -> int __init {
    static int __init sh_mtu2_init(void)
    {
    return platform_driver_register(&sh_mtu2_device_driver);
    }
#[no_mangle]
unsafe extern "C" fn sh_mtu2_exit() -> void __exit {
    static void __exit sh_mtu2_exit(void)
    {
    platform_driver_unregister(&sh_mtu2_device_driver);
    }

    sh_early_platform_init("earlytimer", &sh_mtu2_device_driver);

    subsys_initcall(sh_mtu2_init);
    module_exit(sh_mtu2_exit);
    MODULE_AUTHOR("Magnus Damm");
    MODULE_DESCRIPTION("SuperH MTU2 Timer Driver");
