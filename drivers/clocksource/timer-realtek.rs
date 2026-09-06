//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-realtek.c
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
// Copyright (c) 2025 Realtek Semiconductor Corp.
//

pub const ENBL: c_int = 1;
pub const DSBL: c_int = 0;
pub const SYSTIMER_RATE: c_int = 1000000;
pub const SYSTIMER_MIN_DELTA: c_uint = 0x64;

// SYSTIMER Register Offset (RTK Internal Use)
pub const TS_LW_OFST: c_uint = 0x0;
pub const TS_HW_OFST: c_uint = 0x4;
pub const TS_CMP_VAL_LW_OFST: c_uint = 0x8;
pub const TS_CMP_VAL_HW_OFST: c_uint = 0xC;
pub const TS_CMP_CTRL_OFST: c_uint = 0x10;
pub const TS_CMP_STAT_OFST: c_uint = 0x14;
// SYSTIMER CMP CTRL REG Mask
pub const TS_CMP_EN_MASK: c_uint = 0x1;
pub const TS_WR_EN0_MASK: c_uint = 0x2;
    static void __iomem *systimer_base;
#[no_mangle]
unsafe extern "C" fn rtk_ts64_read() -> u64 {
    static u64 rtk_ts64_read(void)
    {
    u32 low, high;
    u64 ts;
// Caution: Read LSB word (TS_LW_OFST) first then MSB (TS_HW_OFST)
    low = readl(systimer_base + TS_LW_OFST);
    high = readl(systimer_base + TS_HW_OFST);
    ts = ((u64)high << 32) | low;
    return ts;
    }
#[no_mangle]
unsafe extern "C" fn rtk_cmp_value_write(value: u64) {
    static void rtk_cmp_value_write(u64 value)
    {
    u32 high, low;
    low = value & 0xFFFFFFFF;
    high = value >> 32;
    writel(high, systimer_base + TS_CMP_VAL_HW_OFST);
    writel(low, systimer_base + TS_CMP_VAL_LW_OFST);
    }
#[no_mangle]
pub unsafe extern "C" fn rtk_cmp_en_write(cmp_en: bool) {
    static inline void rtk_cmp_en_write(bool cmp_en)
    {
    u32 val;
    val = TS_WR_EN0_MASK;
    if (cmp_en == ENBL)
    val |= TS_CMP_EN_MASK;
    writel(val, systimer_base + TS_CMP_CTRL_OFST);
    }
#[no_mangle]
unsafe extern "C" fn rtk_syst_clkevt_next_event(cycles: c_ulong, clkevt: *mut clock_event_device) -> c_int {
    static int rtk_syst_clkevt_next_event(unsigned long cycles, struct clock_event_device *clkevt)
    {
    u64 cmp_val;
    rtk_cmp_en_write(DSBL);
    cmp_val = rtk_ts64_read();
// Set CMP value to current timestamp plus delta_us
    rtk_cmp_value_write(cmp_val + cycles);
    rtk_cmp_en_write(ENBL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtk_ts_match_intr_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rtk_ts_match_intr_handler(int irq, void *dev_id)
    {
    struct clock_event_device *clkevt = dev_id;
    void __iomem *reg_base;
    u32 val;
// Disable TS CMP Match
    rtk_cmp_en_write(DSBL);
// Clear TS CMP INTR
    reg_base = systimer_base + TS_CMP_STAT_OFST;
    val = readl(reg_base) & TS_CMP_EN_MASK;
    writel(val | TS_CMP_EN_MASK, reg_base);
    clkevt.event_handler(clkevt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rtk_syst_shutdown(clkevt: *mut clock_event_device) -> c_int {
    static int rtk_syst_shutdown(struct clock_event_device *clkevt)
    {
    void __iomem *reg_base;
    let mut cmp_val: u64 = 0;
// Disable TS CMP Match
    rtk_cmp_en_write(DSBL);
// Set compare value to 0
    rtk_cmp_value_write(cmp_val);
// Clear TS CMP INTR
    reg_base = systimer_base + TS_CMP_STAT_OFST;
    writel(TS_CMP_EN_MASK, reg_base);
    return 0;
    }
    static struct timer_of rtk_timer_to = {
    .flags = TIMER_OF_IRQ | TIMER_OF_BASE,
    .clkevt = {
    .name			= "rtk-clkevt",
    .rating			= 300,
    .cpumask		= cpu_possible_mask,
    .features		= CLOCK_EVT_FEAT_DYNIRQ |
    CLOCK_EVT_FEAT_ONESHOT,
    .set_next_event		= rtk_syst_clkevt_next_event,
    .set_state_oneshot	= rtk_syst_shutdown,
    .set_state_shutdown	= rtk_syst_shutdown,
    },
    .of_irq = {
    .flags			= IRQF_TIMER | IRQF_IRQPOLL,
    .handler		= rtk_ts_match_intr_handler,
    },
    };
#[no_mangle]
unsafe extern "C" fn rtk_systimer_init(node: *mut device_node) -> int __init {
    static int __init rtk_systimer_init(struct device_node *node)
    {
    int ret;
    ret = timer_of_init(node, &rtk_timer_to);
    if (ret)
    return ret;
    systimer_base = timer_of_base(&rtk_timer_to);
    clockevents_config_and_register(&rtk_timer_to.clkevt, SYSTIMER_RATE,
    SYSTIMER_MIN_DELTA, SYSTIMER_MAX_DELTA);
    return 0;
    }
    TIMER_OF_DECLARE(rtk_systimer, "realtek,rtd1625-systimer", rtk_systimer_init);
