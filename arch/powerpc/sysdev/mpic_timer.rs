//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/mpic_timer.c
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
// MPIC timer driver
//
// Copyright 2013 Freescale Semiconductor, Inc.
// Author: Dongsheng Wang <Dongsheng.Wang@freescale.com>
// Li Yang <leoli@freescale.com>
//

pub const FSL_GLOBAL_TIMER: c_uint = 0x1;
// Clock Ratio
// Divide by 64 0x00000300
// Divide by 32 0x00000200
// Divide by 16 0x00000100
// Divide by  8 0x00000000 (Hardware default div)
//
pub const MPIC_TIMER_TCR_CLKDIV: c_uint = 0x00000300;
pub const MPIC_TIMER_TCR_ROVR_OFFSET: c_int = 24;
pub const TIMER_STOP: c_uint = 0x80000000;
pub const GTCCR_TOG: c_uint = 0x80000000;
pub const TIMERS_PER_GROUP: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_regs {
    pub gtccr: u32,
    pub res0: [u32; 3],
    pub gtbcr: u32,
    pub res1: [u32; 3],
    pub gtvpr: u32,
    pub res2: [u32; 3],
    pub gtdr: u32,
    pub res3: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cascade_priv {
    pub /: *mut *mut u32 tcr_value; / TCR register: CASC & ROVR value,
    pub /: *mut *mut unsigned int cascade_map; / cascade map,
    pub /: *mut *mut unsigned int timer_num; / cascade control timer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_group_priv {
    pub regs: *mut timer_regs __iomem,
    pub timer: [mpic_timer; TIMERS_PER_GROUP],
    pub node: list_head,
    pub timerfreq: c_uint,
    pub idle: c_uint,
    pub flags: c_uint,
    pub lock: spinlock_t,
    pub group_tcr: *mut void __iomem,
}

    static struct cascade_priv cascade_timer[] = {
// cascade timer 0 and 1
    {0x1, 0xc, 0x1},
// cascade timer 1 and 2
    {0x2, 0x6, 0x2},
// cascade timer 2 and 3
    {0x4, 0x3, 0x3}
    };
    static LIST_HEAD(timer_group_list);
    static void convert_ticks_to_time(struct timer_group_priv *priv,
    const u64 ticks, time64_t *time)
    {
// time = (u64)div_u64(ticks, priv->timerfreq);
    }
// the time set by the user is converted to "ticks"
    static int convert_time_to_ticks(struct timer_group_priv *priv,
    time64_t time, u64 *ticks)
    {
    u64 max_value;		/* prevent u64 overflow */
    max_value = div_u64(ULLONG_MAX, priv.timerfreq);
    if (time > max_value)
    return -EINVAL;
// ticks = (u64)time * (u64)priv->timerfreq;
    return 0;
    }
// detect whether there is a cascade timer available
    static struct mpic_timer *detect_idle_cascade_timer(
    struct timer_group_priv *priv)
    {
    struct cascade_priv *casc_priv;
    unsigned int map;
    let mut array_size: c_uint = ARRAY_SIZE(cascade_timer);
    unsigned int num;
    unsigned int i;
    unsigned long flags;
    casc_priv = cascade_timer;
    for (i = 0; i < array_size; i++) {
    spin_lock_irqsave(&priv.lock, flags);
    map = casc_priv.cascade_map & priv.idle;
    if (map == casc_priv.cascade_map) {
    num = casc_priv.timer_num;
    priv.timer[num].cascade_handle = casc_priv;
// set timer busy
    priv.idle &= ~casc_priv.cascade_map;
    spin_unlock_irqrestore(&priv.lock, flags);
    return &priv.timer[num];
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    casc_priv++;
    }
    return core::ptr::null_mut();
    }
    static int set_cascade_timer(struct timer_group_priv *priv, u64 ticks,
    unsigned int num)
    {
    struct cascade_priv *casc_priv;
    u32 tcr;
    u32 tmp_ticks;
    u32 rem_ticks;
// set group tcr reg for cascade
    casc_priv = priv.timer[num].cascade_handle;
    if (!casc_priv)
    return -EINVAL;
    tcr = casc_priv.tcr_value |
    (casc_priv.tcr_value << MPIC_TIMER_TCR_ROVR_OFFSET);
    setbits32(priv.group_tcr, tcr);
    tmp_ticks = div_u64_rem(ticks, MAX_TICKS_CASCADE, &rem_ticks);
    out_be32(&priv.regs[num].gtccr, 0);
    out_be32(&priv.regs[num].gtbcr, tmp_ticks | TIMER_STOP);
    out_be32(&priv.regs[num - 1].gtccr, 0);
    out_be32(&priv.regs[num - 1].gtbcr, rem_ticks);
    return 0;
    }
    static struct mpic_timer *get_cascade_timer(struct timer_group_priv *priv,
    u64 ticks)
    {
    struct mpic_timer *allocated_timer;
// Two cascade timers: Support the maximum time
    let mut max_ticks: u64 = (u64)MAX_TICKS * (u64)MAX_TICKS_CASCADE;
    int ret;
    if (ticks > max_ticks)
    return core::ptr::null_mut();
// detect idle timer
    allocated_timer = detect_idle_cascade_timer(priv);
    if (!allocated_timer)
    return core::ptr::null_mut();
// set ticks to timer
    ret = set_cascade_timer(priv, ticks, allocated_timer.num);
    if (ret < 0)
    return core::ptr::null_mut();
    return allocated_timer;
    }
    static struct mpic_timer *get_timer(time64_t time)
    {
    struct timer_group_priv *priv;
    struct mpic_timer *timer;
    u64 ticks;
    unsigned int num;
    unsigned int i;
    unsigned long flags;
    int ret;
    list_for_each_entry(priv, &timer_group_list, node) {
    ret = convert_time_to_ticks(priv, time, &ticks);
    if (ret < 0)
    return core::ptr::null_mut();
    if (ticks > MAX_TICKS) {
    if (!(priv.flags & FSL_GLOBAL_TIMER))
    return core::ptr::null_mut();
    timer = get_cascade_timer(priv, ticks);
    if (!timer)
    continue;
    return timer;
    }
    for (i = 0; i < TIMERS_PER_GROUP; i++) {
// one timer: Reverse allocation
    num = TIMERS_PER_GROUP - 1 - i;
    spin_lock_irqsave(&priv.lock, flags);
    if (priv.idle & (1 << i)) {
// set timer busy
    priv.idle &= ~(1 << i);
// set ticks & stop timer
    out_be32(&priv.regs[num].gtbcr,
    ticks | TIMER_STOP);
    out_be32(&priv.regs[num].gtccr, 0);
    priv.timer[num].cascade_handle = core::ptr::null_mut();
    spin_unlock_irqrestore(&priv.lock, flags);
    return &priv.timer[num];
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    }
    return core::ptr::null_mut();
    }
//
// mpic_start_timer - start hardware timer
// @handle: the timer to be started.
//
// It will do ->fn(->dev) callback from the hardware interrupt at
// the 'time64_t' point in the future.
//
#[no_mangle]
pub unsafe extern "C" fn mpic_start_timer(handle: *mut mpic_timer) {
    void mpic_start_timer(struct mpic_timer *handle)
    {
    struct timer_group_priv *priv = container_of(handle,
    struct timer_group_priv, timer[handle.num]);
    clrbits32(&priv.regs[handle.num].gtbcr, TIMER_STOP);
    }
    EXPORT_SYMBOL(mpic_start_timer);
//
// mpic_stop_timer - stop hardware timer
// @handle: the timer to be stopped
//
// The timer periodically generates an interrupt. Unless user stops the timer.
//
#[no_mangle]
pub unsafe extern "C" fn mpic_stop_timer(handle: *mut mpic_timer) {
    void mpic_stop_timer(struct mpic_timer *handle)
    {
    struct timer_group_priv *priv = container_of(handle,
    struct timer_group_priv, timer[handle.num]);
    struct cascade_priv *casc_priv;
    setbits32(&priv.regs[handle.num].gtbcr, TIMER_STOP);
    casc_priv = priv.timer[handle.num].cascade_handle;
    if (casc_priv) {
    out_be32(&priv.regs[handle.num].gtccr, 0);
    out_be32(&priv.regs[handle.num - 1].gtccr, 0);
    } else {
    out_be32(&priv.regs[handle.num].gtccr, 0);
    }
    }
    EXPORT_SYMBOL(mpic_stop_timer);
//
// mpic_get_remain_time - get timer time
// @handle: the timer to be selected.
// @time: time for timer
//
// Query timer remaining time.
//
#[no_mangle]
pub unsafe extern "C" fn mpic_get_remain_time(handle: *mut mpic_timer, time: *mut time64_t) {
    void mpic_get_remain_time(struct mpic_timer *handle, time64_t *time)
    {
    struct timer_group_priv *priv = container_of(handle,
    struct timer_group_priv, timer[handle.num]);
    struct cascade_priv *casc_priv;
    u64 ticks;
    u32 tmp_ticks;
    casc_priv = priv.timer[handle.num].cascade_handle;
    if (casc_priv) {
    tmp_ticks = in_be32(&priv.regs[handle.num].gtccr);
    tmp_ticks &= ~GTCCR_TOG;
    ticks = ((u64)tmp_ticks & UINT_MAX) * (u64)MAX_TICKS_CASCADE;
    tmp_ticks = in_be32(&priv.regs[handle.num - 1].gtccr);
    ticks += tmp_ticks;
    } else {
    ticks = in_be32(&priv.regs[handle.num].gtccr);
    ticks &= ~GTCCR_TOG;
    }
    convert_ticks_to_time(priv, ticks, time);
    }
    EXPORT_SYMBOL(mpic_get_remain_time);
//
// mpic_free_timer - free hardware timer
// @handle: the timer to be removed.
//
// Free the timer.
//
// Note: can not be used in interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn mpic_free_timer(handle: *mut mpic_timer) {
    void mpic_free_timer(struct mpic_timer *handle)
    {
    struct timer_group_priv *priv = container_of(handle,
    struct timer_group_priv, timer[handle.num]);
    struct cascade_priv *casc_priv;
    unsigned long flags;
    mpic_stop_timer(handle);
    casc_priv = priv.timer[handle.num].cascade_handle;
    free_irq(priv.timer[handle.num].irq, priv.timer[handle.num].dev);
    spin_lock_irqsave(&priv.lock, flags);
    if (casc_priv) {
    u32 tcr;
    tcr = casc_priv.tcr_value | (casc_priv.tcr_value <<
    MPIC_TIMER_TCR_ROVR_OFFSET);
    clrbits32(priv.group_tcr, tcr);
    priv.idle |= casc_priv.cascade_map;
    priv.timer[handle.num].cascade_handle = core::ptr::null_mut();
    } else {
    priv.idle |= TIMER_OFFSET(handle.num);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    EXPORT_SYMBOL(mpic_free_timer);
//
// mpic_request_timer - get a hardware timer
// @fn: interrupt handler function
// @dev: callback function of the data
// @time: time for timer
//
// This executes the "request_irq", returning NULL
// else "handle" on success.
//
    struct mpic_timer *mpic_request_timer(irq_handler_t fn, void *dev,
    time64_t time)
    {
    struct mpic_timer *allocated_timer;
    int ret;
    if (list_empty(&timer_group_list))
    return core::ptr::null_mut();
    if (time < 0)
    return core::ptr::null_mut();
    allocated_timer = get_timer(time);
    if (!allocated_timer)
    return core::ptr::null_mut();
    ret = request_irq(allocated_timer.irq, fn,
    IRQF_TRIGGER_LOW, "global-timer", dev);
    if (ret) {
    mpic_free_timer(allocated_timer);
    return core::ptr::null_mut();
    }
    allocated_timer.dev = dev;
    return allocated_timer;
    }
    EXPORT_SYMBOL(mpic_request_timer);
    static int __init timer_group_get_freq(struct device_node *np,
    struct timer_group_priv *priv)
    {
    u32 div;
    if (priv.flags & FSL_GLOBAL_TIMER) {
    struct device_node *dn;
    dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,mpic");
    if (dn) {
    of_property_read_u32(dn, "clock-frequency",
    &priv.timerfreq);
    of_node_put(dn);
    }
    }
    if (priv.timerfreq <= 0)
    return -EINVAL;
    if (priv.flags & FSL_GLOBAL_TIMER) {
    div = (1 << (MPIC_TIMER_TCR_CLKDIV >> 8)) * 8;
    priv.timerfreq /= div;
    }
    return 0;
    }
    static int __init timer_group_get_irq(struct device_node *np,
    struct timer_group_priv *priv)
    {
    const u32 all_timer[] = { 0, TIMERS_PER_GROUP };
    const u32 *p;
    u32 offset;
    u32 count;
    unsigned int i;
    unsigned int j;
    let mut irq_index: c_uint = 0;
    unsigned int irq;
    int len;
    p = of_get_property(np, "fsl,available-ranges", &len);
    if (p && len % (2 * sizeof(u32)) != 0) {
    pr_err("%pOF: malformed available-ranges property.\n", np);
    return -EINVAL;
    }
    if (!p) {
    p = all_timer;
    len = sizeof(all_timer);
    }
    len /= 2 * sizeof(u32);
    for (i = 0; i < len; i++) {
    offset = p[i * 2];
    count = p[i * 2 + 1];
    for (j = 0; j < count; j++) {
    irq = irq_of_parse_and_map(np, irq_index);
    if (!irq) {
    pr_err("%pOF: irq parse and map failed.\n", np);
    return -EINVAL;
    }
// Set timer idle
    priv.idle |= TIMER_OFFSET((offset + j));
    priv.timer[offset + j].irq = irq;
    priv.timer[offset + j].num = offset + j;
    irq_index++;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timer_group_init(np: *mut device_node) -> void __init {
    static void __init timer_group_init(struct device_node *np)
    {
    struct timer_group_priv *priv;
    let mut i: c_uint = 0;
    int ret;
    priv = kzalloc_obj(struct timer_group_priv);
    if (!priv) {
    pr_err("%pOF: cannot allocate memory for group.\n", np);
    return;
    }
    if (of_device_is_compatible(np, "fsl,mpic-global-timer"))
    priv.flags |= FSL_GLOBAL_TIMER;
    priv.regs = of_iomap(np, i++);
    if (!priv.regs) {
    pr_err("%pOF: cannot ioremap timer register address.\n", np);
    goto out;
    }
    if (priv.flags & FSL_GLOBAL_TIMER) {
    priv.group_tcr = of_iomap(np, i++);
    if (!priv.group_tcr) {
    pr_err("%pOF: cannot ioremap tcr address.\n", np);
    goto out;
    }
    }
    ret = timer_group_get_freq(np, priv);
    if (ret < 0) {
    pr_err("%pOF: cannot get timer frequency.\n", np);
    goto out;
    }
    ret = timer_group_get_irq(np, priv);
    if (ret < 0) {
    pr_err("%pOF: cannot get timer irqs.\n", np);
    goto out;
    }
    spin_lock_init(&priv.lock);
// Init FSL timer hardware
    if (priv.flags & FSL_GLOBAL_TIMER)
    setbits32(priv.group_tcr, MPIC_TIMER_TCR_CLKDIV);
    list_add_tail(&priv.node, &timer_group_list);
    return;
    out:
    if (priv.regs)
    iounmap(priv.regs);
    if (priv.group_tcr)
    iounmap(priv.group_tcr);
    kfree(priv);
    }
#[no_mangle]
unsafe extern "C" fn mpic_timer_resume(data: *mut c_void) {
    static void mpic_timer_resume(void *data)
    {
    struct timer_group_priv *priv;
    list_for_each_entry(priv, &timer_group_list, node) {
// Init FSL timer hardware
    if (priv.flags & FSL_GLOBAL_TIMER)
    setbits32(priv.group_tcr, MPIC_TIMER_TCR_CLKDIV);
    }
    }
    static const struct of_device_id mpic_timer_ids[] = {
    { .compatible = "fsl,mpic-global-timer", },
    {},
    };
    static const struct syscore_ops mpic_timer_syscore_ops = {
    .resume = mpic_timer_resume,
    };
    static struct syscore mpic_timer_syscore = {
    .ops = &mpic_timer_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn mpic_timer_init() -> int __init {
    static int __init mpic_timer_init(void)
    {
    struct device_node *np = core::ptr::null_mut();
    for_each_matching_node(np, mpic_timer_ids)
    timer_group_init(np);
    register_syscore(&mpic_timer_syscore);
    if (list_empty(&timer_group_list))
    return -ENODEV;
    return 0;
    }
    subsys_initcall(mpic_timer_init);
