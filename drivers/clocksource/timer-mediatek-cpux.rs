//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-mediatek-cpux.c
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
// MediaTek SoCs CPUX General Purpose Timer handling
//
// Based on timer-mediatek.c:
// Copyright (C) 2014 Matthias Brugger <matthias.bgg@gmail.com>
//
// Copyright (C) 2022 Collabora Ltd.
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//

pub const TIMER_SYNC_TICKS: c_int = 3;
// cpux mcusys wrapper
pub const CPUX_CON_REG: c_uint = 0x0;
pub const CPUX_IDX_REG: c_uint = 0x4;
// cpux
pub const CPUX_IDX_GLOBAL_CTRL: c_uint = 0x0;

pub const CPUX_IDX_GLOBAL_IRQ: c_uint = 0x30;
#[no_mangle]
unsafe extern "C" fn mtk_cpux_readl(reg_idx: u32, to: *mut timer_of) -> u32 {
    static u32 mtk_cpux_readl(u32 reg_idx, struct timer_of *to)
    {
    writel(reg_idx, timer_of_base(to) + CPUX_IDX_REG);
    return readl(timer_of_base(to) + CPUX_CON_REG);
    }
#[no_mangle]
unsafe extern "C" fn mtk_cpux_writel(val: u32, reg_idx: u32, to: *mut timer_of) {
    static void mtk_cpux_writel(u32 val, u32 reg_idx, struct timer_of *to)
    {
    writel(reg_idx, timer_of_base(to) + CPUX_IDX_REG);
    writel(val, timer_of_base(to) + CPUX_CON_REG);
    }
#[no_mangle]
unsafe extern "C" fn mtk_cpux_set_irq(to: *mut timer_of, enable: bool) {
    static void mtk_cpux_set_irq(struct timer_of *to, bool enable)
    {
    const unsigned long *irq_mask = cpumask_bits(cpu_possible_mask);
    u32 val;
    val = mtk_cpux_readl(CPUX_IDX_GLOBAL_IRQ, to);
    if (enable)
    val |= *irq_mask;
    else
    val &= ~(*irq_mask);
    mtk_cpux_writel(val, CPUX_IDX_GLOBAL_IRQ, to);
    }
#[no_mangle]
unsafe extern "C" fn mtk_cpux_clkevt_shutdown(clkevt: *mut clock_event_device) -> c_int {
    static int mtk_cpux_clkevt_shutdown(struct clock_event_device *clkevt)
    {
// Clear any irq
    mtk_cpux_set_irq(to_timer_of(clkevt), false);
//
// Disabling CPUXGPT timer will crash the platform, especially
// if Trusted Firmware is using it (usually, for sleep states),
// so we only mask the IRQ and call it a day.
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_cpux_clkevt_resume(clkevt: *mut clock_event_device) -> c_int {
    static int mtk_cpux_clkevt_resume(struct clock_event_device *clkevt)
    {
    mtk_cpux_set_irq(to_timer_of(clkevt), true);
    return 0;
    }
    static struct timer_of to = {
//
// There are per-cpu interrupts for the CPUX General Purpose Timer
// but since this timer feeds the AArch64 System Timer we can rely
// on the CPU timer PPIs as well, so we don't declare TIMER_OF_IRQ.
//
    .flags = TIMER_OF_BASE | TIMER_OF_CLOCK,
    .clkevt = {
    .name = "mtk-cpuxgpt",
    .cpumask = cpu_possible_mask,
    .rating = 10,
    .set_state_shutdown = mtk_cpux_clkevt_shutdown,
    .tick_resume = mtk_cpux_clkevt_resume,
    },
    };
#[no_mangle]
unsafe extern "C" fn mtk_cpux_init(node: *mut device_node) -> int __init {
    static int __init mtk_cpux_init(struct device_node *node)
    {
    u32 freq, val;
    int ret;
// If this fails, bad things are about to happen...
    ret = timer_of_init(node, &to);
    if (ret) {
    WARN(1, "Cannot start CPUX timers.\n");
    return ret;
    }
//
// Check if we're given a clock with the right frequency for this
// timer, otherwise warn but keep going with the setup anyway, as
// that makes it possible to still boot the kernel, even though
// it may not work correctly (random lockups, etc).
// The reason behind this is that having an early UART may not be
// possible for everyone and this gives a chance to retrieve kmsg
// for eventual debugging even on consumer devices.
//
    freq = timer_of_rate(&to);
    if (freq > 13000000)
    WARN(1, "Requested unsupported timer frequency %u\n", freq);
// Clock input is 26MHz, set DIV2 to achieve 13MHz clock
    val = mtk_cpux_readl(CPUX_IDX_GLOBAL_CTRL, &to);
    val &= ~CPUX_CLK_DIV_MASK;
    val |= CPUX_CLK_DIV2;
    mtk_cpux_writel(val, CPUX_IDX_GLOBAL_CTRL, &to);
// Enable all CPUXGPT timers
    val = mtk_cpux_readl(CPUX_IDX_GLOBAL_CTRL, &to);
    mtk_cpux_writel(val | CPUX_ENABLE, CPUX_IDX_GLOBAL_CTRL, &to);
    clockevents_config_and_register(&to.clkevt, timer_of_rate(&to),
    TIMER_SYNC_TICKS, 0xffffffff);
    return 0;
    }
    TIMER_OF_DECLARE(mtk_mt6795, "mediatek,mt6795-systimer", mtk_cpux_init);
