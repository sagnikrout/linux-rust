//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/ingenic-sysost.c
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
// Ingenic XBurst SoCs SYSOST clocks driver
// Copyright (c) 2020 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//

// OST register offsets
pub const OST_REG_OSTCCR: c_uint = 0x00;
pub const OST_REG_OSTCR: c_uint = 0x08;
pub const OST_REG_OSTFR: c_uint = 0x0c;
pub const OST_REG_OSTMR: c_uint = 0x10;
pub const OST_REG_OST1DFR: c_uint = 0x14;
pub const OST_REG_OST1CNT: c_uint = 0x18;
pub const OST_REG_OST2CNTL: c_uint = 0x20;
pub const OST_REG_OSTCNT2HBUF: c_uint = 0x24;
pub const OST_REG_OSTESR: c_uint = 0x34;
pub const OST_REG_OSTECR: c_uint = 0x38;
// bits within the OSTCCR register
pub const OSTCCR_PRESCALE1_MASK: c_uint = 0x3;
pub const OSTCCR_PRESCALE2_MASK: c_uint = 0xc;
// bits within the OSTCR register

// bits within the OSTFR register

// bits within the OSTMR register

// bits within the OSTESR register

// bits within the OSTECR register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_soc_info {
    pub num_channels: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_ost_clk_info {
    pub init_data: clk_init_data,
    pub ostccr_reg: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_ost_clk {
    pub hw: clk_hw,
    pub idx: c_uint,
    pub ost: *mut ingenic_ost,
    pub info: *const ingenic_ost_clk_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_ost {
    pub base: *mut void __iomem,
    pub soc_info: *const ingenic_soc_info,
    pub global_timer_clk: *mut *mut *mut clk clk, percpu_timer_clk,,
    pub cevt: clock_event_device,
    pub cs: clocksource,
    pub name: [c_char; 20],
    pub clocks: *mut clk_hw_onecell_data,
}

    static struct ingenic_ost *ingenic_ost;
    static inline struct ingenic_ost_clk *to_ost_clk(struct clk_hw *hw)
    {
    return container_of(hw, struct ingenic_ost_clk, hw);
    }
    static unsigned long ingenic_ost_percpu_timer_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct ingenic_ost_clk *ost_clk = to_ost_clk(hw);
    const struct ingenic_ost_clk_info *info = ost_clk.info;
    unsigned int prescale;
    prescale = readl(ost_clk.ost.base + info.ostccr_reg);
    prescale = FIELD_GET(OSTCCR_PRESCALE1_MASK, prescale);
    return parent_rate >> (prescale * 2);
    }
    static unsigned long ingenic_ost_global_timer_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct ingenic_ost_clk *ost_clk = to_ost_clk(hw);
    const struct ingenic_ost_clk_info *info = ost_clk.info;
    unsigned int prescale;
    prescale = readl(ost_clk.ost.base + info.ostccr_reg);
    prescale = FIELD_GET(OSTCCR_PRESCALE2_MASK, prescale);
    return parent_rate >> (prescale * 2);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_get_prescale(rate: c_ulong, req_rate: c_ulong) -> u8 {
    static u8 ingenic_ost_get_prescale(unsigned long rate, unsigned long req_rate)
    {
    u8 prescale;
    for (prescale = 0; prescale < 2; prescale++)
    if ((rate >> (prescale * 2)) <= req_rate)
    return prescale;
    return 2; /* /16 divider */
    }
    static int ingenic_ost_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut rate: c_ulong = req.best_parent_rate;
    u8 prescale;
    if (req.rate > rate) {
    req.rate = rate;
    return 0;
    }
    prescale = ingenic_ost_get_prescale(rate, req.rate);
    req.rate = rate >> (prescale * 2);
    return 0;
    }
    static int ingenic_ost_percpu_timer_set_rate(struct clk_hw *hw, unsigned long req_rate,
    unsigned long parent_rate)
    {
    struct ingenic_ost_clk *ost_clk = to_ost_clk(hw);
    const struct ingenic_ost_clk_info *info = ost_clk.info;
    let mut prescale: u8 = ingenic_ost_get_prescale(parent_rate, req_rate);
    int val;
    val = readl(ost_clk.ost.base + info.ostccr_reg);
    val &= ~OSTCCR_PRESCALE1_MASK;
    val |= FIELD_PREP(OSTCCR_PRESCALE1_MASK, prescale);
    writel(val, ost_clk.ost.base + info.ostccr_reg);
    return 0;
    }
    static int ingenic_ost_global_timer_set_rate(struct clk_hw *hw, unsigned long req_rate,
    unsigned long parent_rate)
    {
    struct ingenic_ost_clk *ost_clk = to_ost_clk(hw);
    const struct ingenic_ost_clk_info *info = ost_clk.info;
    let mut prescale: u8 = ingenic_ost_get_prescale(parent_rate, req_rate);
    int val;
    val = readl(ost_clk.ost.base + info.ostccr_reg);
    val &= ~OSTCCR_PRESCALE2_MASK;
    val |= FIELD_PREP(OSTCCR_PRESCALE2_MASK, prescale);
    writel(val, ost_clk.ost.base + info.ostccr_reg);
    return 0;
    }
    static const struct clk_ops ingenic_ost_percpu_timer_ops = {
    .recalc_rate	= ingenic_ost_percpu_timer_recalc_rate,
    .determine_rate = ingenic_ost_determine_rate,
    .set_rate	= ingenic_ost_percpu_timer_set_rate,
    };
    static const struct clk_ops ingenic_ost_global_timer_ops = {
    .recalc_rate	= ingenic_ost_global_timer_recalc_rate,
    .determine_rate = ingenic_ost_determine_rate,
    .set_rate	= ingenic_ost_global_timer_set_rate,
    };
    static const char * const ingenic_ost_clk_parents[] = { "ext" };
    static const struct ingenic_ost_clk_info x1000_ost_clk_info[] = {
    [OST_CLK_PERCPU_TIMER] = {
    .init_data = {
    .name = "percpu timer",
    .parent_names = ingenic_ost_clk_parents,
    .num_parents = ARRAY_SIZE(ingenic_ost_clk_parents),
    .ops = &ingenic_ost_percpu_timer_ops,
    .flags = CLK_SET_RATE_UNGATE,
    },
    .ostccr_reg = OST_REG_OSTCCR,
    },
    [OST_CLK_GLOBAL_TIMER] = {
    .init_data = {
    .name = "global timer",
    .parent_names = ingenic_ost_clk_parents,
    .num_parents = ARRAY_SIZE(ingenic_ost_clk_parents),
    .ops = &ingenic_ost_global_timer_ops,
    .flags = CLK_SET_RATE_UNGATE,
    },
    .ostccr_reg = OST_REG_OSTCCR,
    },
    };
#[no_mangle]
unsafe extern "C" fn ingenic_ost_global_timer_read_cntl() -> u64 notrace {
    static u64 notrace ingenic_ost_global_timer_read_cntl(void)
    {
    struct ingenic_ost *ost = ingenic_ost;
    unsigned int count;
    count = readl(ost.base + OST_REG_OST2CNTL);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_clocksource_read(cs: *mut clocksource) -> u64 notrace {
    static u64 notrace ingenic_ost_clocksource_read(struct clocksource *cs)
    {
    return ingenic_ost_global_timer_read_cntl();
    }
    static inline struct ingenic_ost *to_ingenic_ost(struct clock_event_device *evt)
    {
    return container_of(evt, struct ingenic_ost, cevt);
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_cevt_set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    static int ingenic_ost_cevt_set_state_shutdown(struct clock_event_device *evt)
    {
    struct ingenic_ost *ost = to_ingenic_ost(evt);
    writel(OSTECR_OST1ENC, ost.base + OST_REG_OSTECR);
    return 0;
    }
    static int ingenic_ost_cevt_set_next(unsigned long next,
    struct clock_event_device *evt)
    {
    struct ingenic_ost *ost = to_ingenic_ost(evt);
    writel((u32)~OSTFR_FFLAG, ost.base + OST_REG_OSTFR);
    writel(next, ost.base + OST_REG_OST1DFR);
    writel(OSTCR_OST1CLR, ost.base + OST_REG_OSTCR);
    writel(OSTESR_OST1ENS, ost.base + OST_REG_OSTESR);
    writel((u32)~OSTMR_FMASK, ost.base + OST_REG_OSTMR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_cevt_cb(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ingenic_ost_cevt_cb(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    struct ingenic_ost *ost = to_ingenic_ost(evt);
    writel(OSTECR_OST1ENC, ost.base + OST_REG_OSTECR);
    if (evt.event_handler)
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
    static int __init ingenic_ost_register_clock(struct ingenic_ost *ost,
    unsigned int idx, const struct ingenic_ost_clk_info *info,
    struct clk_hw_onecell_data *clocks)
    {
    struct ingenic_ost_clk *ost_clk;
    int val, err;
    ost_clk = kzalloc_obj(*ost_clk);
    if (!ost_clk)
    return -ENOMEM;
    ost_clk.hw.init = &info.init_data;
    ost_clk.idx = idx;
    ost_clk.info = info;
    ost_clk.ost = ost;
// Reset clock divider
    val = readl(ost.base + info.ostccr_reg);
    val &= ~(OSTCCR_PRESCALE1_MASK | OSTCCR_PRESCALE2_MASK);
    writel(val, ost.base + info.ostccr_reg);
    err = clk_hw_register(core::ptr::null_mut(), &ost_clk.hw);
    if (err) {
    kfree(ost_clk);
    return err;
    }
    clocks.hws[idx] = &ost_clk.hw;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_get_clock(np: *mut device_node, id: c_int) -> *mut clk  __init {
    static struct clk * __init ingenic_ost_get_clock(struct device_node *np, int id)
    {
    struct of_phandle_args args;
    args.np = np;
    args.args_count = 1;
    args.args[0] = id;
    return of_clk_get_from_provider(&args);
    }
    static int __init ingenic_ost_percpu_timer_init(struct device_node *np,
    struct ingenic_ost *ost)
    {
    unsigned int timer_virq, channel = OST_CLK_PERCPU_TIMER;
    unsigned long rate;
    int err;
    ost.percpu_timer_clk = ingenic_ost_get_clock(np, channel);
    if (IS_ERR(ost.percpu_timer_clk))
    return PTR_ERR(ost.percpu_timer_clk);
    err = clk_prepare_enable(ost.percpu_timer_clk);
    if (err)
    goto err_clk_put;
    rate = clk_get_rate(ost.percpu_timer_clk);
    if (!rate) {
    err = -EINVAL;
    goto err_clk_disable;
    }
    timer_virq = of_irq_get(np, 0);
    if (!timer_virq) {
    err = -EINVAL;
    goto err_clk_disable;
    }
    snprintf(ost.name, sizeof(ost.name), "OST percpu timer");
    err = request_irq(timer_virq, ingenic_ost_cevt_cb, IRQF_TIMER,
    ost.name, &ost.cevt);
    if (err)
    goto err_irq_dispose_mapping;
    ost.cevt.cpumask = cpumask_of(smp_processor_id());
    ost.cevt.features = CLOCK_EVT_FEAT_ONESHOT;
    ost.cevt.name = ost.name;
    ost.cevt.rating = 400;
    ost.cevt.set_state_shutdown = ingenic_ost_cevt_set_state_shutdown;
    ost.cevt.set_next_event = ingenic_ost_cevt_set_next;
    clockevents_config_and_register(&ost.cevt, rate, 4, 0xffffffff);
    return 0;
    err_irq_dispose_mapping:
    irq_dispose_mapping(timer_virq);
    err_clk_disable:
    clk_disable_unprepare(ost.percpu_timer_clk);
    err_clk_put:
    clk_put(ost.percpu_timer_clk);
    return err;
    }
    static int __init ingenic_ost_global_timer_init(struct device_node *np,
    struct ingenic_ost *ost)
    {
    let mut channel: c_uint = OST_CLK_GLOBAL_TIMER;
    struct clocksource *cs = &ost.cs;
    unsigned long rate;
    int err;
    ost.global_timer_clk = ingenic_ost_get_clock(np, channel);
    if (IS_ERR(ost.global_timer_clk))
    return PTR_ERR(ost.global_timer_clk);
    err = clk_prepare_enable(ost.global_timer_clk);
    if (err)
    goto err_clk_put;
    rate = clk_get_rate(ost.global_timer_clk);
    if (!rate) {
    err = -EINVAL;
    goto err_clk_disable;
    }
// Clear counter CNT registers
    writel(OSTCR_OST2CLR, ost.base + OST_REG_OSTCR);
// Enable OST channel
    writel(OSTESR_OST2ENS, ost.base + OST_REG_OSTESR);
    cs.name = "ingenic-ost";
    cs.rating = 400;
    cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    cs.mask = CLOCKSOURCE_MASK(32);
    cs.read = ingenic_ost_clocksource_read;
    err = clocksource_register_hz(cs, rate);
    if (err)
    goto err_clk_disable;
    return 0;
    err_clk_disable:
    clk_disable_unprepare(ost.global_timer_clk);
    err_clk_put:
    clk_put(ost.global_timer_clk);
    return err;
    }
    static const struct ingenic_soc_info x1000_soc_info = {
    .num_channels = 2,
    };
    static const struct of_device_id __maybe_unused ingenic_ost_of_matches[] __initconst = {
    { .compatible = "ingenic,x1000-ost", .data = &x1000_soc_info },
    { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn ingenic_ost_probe(np: *mut device_node) -> int __init {
    static int __init ingenic_ost_probe(struct device_node *np)
    {
    const struct of_device_id *id = of_match_node(ingenic_ost_of_matches, np);
    struct ingenic_ost *ost;
    unsigned int i;
    int ret;
    ost = kzalloc_obj(*ost);
    if (!ost)
    return -ENOMEM;
    ost.base = of_io_request_and_map(np, 0, of_node_full_name(np));
    if (IS_ERR(ost.base)) {
    pr_err("%s: Failed to map OST registers\n", __func__);
    ret = PTR_ERR(ost.base);
    goto err_free_ost;
    }
    ost.clk = of_clk_get_by_name(np, "ost");
    if (IS_ERR(ost.clk)) {
    ret = PTR_ERR(ost.clk);
    pr_crit("%s: Cannot get OST clock\n", __func__);
    goto err_free_ost;
    }
    ret = clk_prepare_enable(ost.clk);
    if (ret) {
    pr_crit("%s: Unable to enable OST clock\n", __func__);
    goto err_put_clk;
    }
    ost.soc_info = id.data;
    ost.clocks = kzalloc_flex(*ost.clocks, hws,
    ost.soc_info.num_channels);
    if (!ost.clocks) {
    ret = -ENOMEM;
    goto err_clk_disable;
    }
    ost.clocks.num = ost.soc_info.num_channels;
    for (i = 0; i < ost.clocks.num; i++) {
    ret = ingenic_ost_register_clock(ost, i, &x1000_ost_clk_info[i], ost.clocks);
    if (ret) {
    pr_crit("%s: Cannot register clock %d\n", __func__, i);
    goto err_unregister_ost_clocks;
    }
    }
    ret = of_clk_add_hw_provider(np, of_clk_hw_onecell_get, ost.clocks);
    if (ret) {
    pr_crit("%s: Cannot add OF clock provider\n", __func__);
    goto err_unregister_ost_clocks;
    }
    ingenic_ost = ost;
    return 0;
    err_unregister_ost_clocks:
    for (i = 0; i < ost.clocks.num; i++)
    if (ost.clocks.hws[i])
    clk_hw_unregister(ost.clocks.hws[i]);
    kfree(ost.clocks);
    err_clk_disable:
    clk_disable_unprepare(ost.clk);
    err_put_clk:
    clk_put(ost.clk);
    err_free_ost:
    kfree(ost);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ingenic_ost_init(np: *mut device_node) -> int __init {
    static int __init ingenic_ost_init(struct device_node *np)
    {
    struct ingenic_ost *ost;
    unsigned long rate;
    int ret;
    ret = ingenic_ost_probe(np);
    if (ret) {
    pr_crit("%s: Failed to initialize OST clocks: %d\n", __func__, ret);
    return ret;
    }
    of_node_clear_flag(np, OF_POPULATED);
    ost = ingenic_ost;
    if (IS_ERR(ost))
    return PTR_ERR(ost);
    ret = ingenic_ost_global_timer_init(np, ost);
    if (ret) {
    pr_crit("%s: Unable to init global timer: %x\n", __func__, ret);
    goto err_free_ingenic_ost;
    }
    ret = ingenic_ost_percpu_timer_init(np, ost);
    if (ret)
    goto err_ost_global_timer_cleanup;
// Register the sched_clock at the end as there's no way to undo it
    rate = clk_get_rate(ost.global_timer_clk);
    sched_clock_register(ingenic_ost_global_timer_read_cntl, 32, rate);
    return 0;
    err_ost_global_timer_cleanup:
    clocksource_unregister(&ost.cs);
    clk_disable_unprepare(ost.global_timer_clk);
    clk_put(ost.global_timer_clk);
    err_free_ingenic_ost:
    kfree(ost);
    return ret;
    }
    TIMER_OF_DECLARE(x1000_ost,  "ingenic,x1000-ost",  ingenic_ost_init);
