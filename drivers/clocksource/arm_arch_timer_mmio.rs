//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/arm_arch_timer_mmio.c
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
// ARM Generic Memory Mapped Timer support
//
// Split from drivers/clocksource/arm_arch_timer.c
//
// Copyright (C) 2011 ARM Ltd.
// All Rights Reserved
//

pub const CNTTIDR: c_uint = 0x08;

pub const CNTPCT_LO: c_uint = 0x00;
pub const CNTVCT_LO: c_uint = 0x08;
pub const CNTFRQ: c_uint = 0x10;
pub const CNTP_CVAL_LO: c_uint = 0x20;
pub const CNTP_CTL: c_uint = 0x2c;
pub const CNTV_CVAL_LO: c_uint = 0x30;
pub const CNTV_CTL: c_uint = 0x3c;
    enum arch_timer_access {
    PHYS_ACCESS,
    VIRT_ACCESS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_timer {
    pub evt: clock_event_device,
    pub cs: clocksource,
    pub gt_block: *mut arch_timer_mem,
    pub base: *mut void __iomem,
    pub access: enum arch_timer_access,
    pub rate: u32,
}

    static void arch_timer_mmio_write(struct arch_timer *timer,
    enum arch_timer_reg reg, u64 val)
    {
    switch (timer.access) {
    case PHYS_ACCESS:
    switch (reg) {
    case ARCH_TIMER_REG_CTRL:
    writel_relaxed((u32)val, timer.base + CNTP_CTL);
    return;
    case ARCH_TIMER_REG_CVAL:
//
// Not guaranteed to be atomic, so the timer
// must be disabled at this point.
//
    writeq_relaxed(val, timer.base + CNTP_CVAL_LO);
    return;
    }
    break;
    case VIRT_ACCESS:
    switch (reg) {
    case ARCH_TIMER_REG_CTRL:
    writel_relaxed((u32)val, timer.base + CNTV_CTL);
    return;
    case ARCH_TIMER_REG_CVAL:
// Same restriction as above
    writeq_relaxed(val, timer.base + CNTV_CVAL_LO);
    return;
    }
    break;
    }
// Should never be here
    WARN_ON_ONCE(1);
    }
#[no_mangle]
unsafe extern "C" fn arch_timer_mmio_read(timer: *mut arch_timer, reg: enum arch_timer_reg) -> u32 {
    static u32 arch_timer_mmio_read(struct arch_timer *timer, enum arch_timer_reg reg)
    {
    switch (timer.access) {
    case PHYS_ACCESS:
    switch (reg) {
    case ARCH_TIMER_REG_CTRL:
    return readl_relaxed(timer.base + CNTP_CTL);
    default:
    break;
    }
    break;
    case VIRT_ACCESS:
    switch (reg) {
    case ARCH_TIMER_REG_CTRL:
    return readl_relaxed(timer.base + CNTV_CTL);
    default:
    break;
    }
    break;
    }
// Should never be here
    WARN_ON_ONCE(1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arch_counter_mmio_get_cnt(t: *mut arch_timer) -> noinstr u64 {
    static noinstr u64 arch_counter_mmio_get_cnt(struct arch_timer *t)
    {
    let mut offset_lo: c_int = t.access == VIRT_ACCESS ? CNTVCT_LO : CNTPCT_LO;
    u32 cnt_lo, cnt_hi, tmp_hi;
    do {
    cnt_hi = __le32_to_cpu((__le32 )__raw_readl(t.base + offset_lo + 4));
    cnt_lo = __le32_to_cpu((__le32 )__raw_readl(t.base + offset_lo));
    tmp_hi = __le32_to_cpu((__le32 )__raw_readl(t.base + offset_lo + 4));
    } while (cnt_hi != tmp_hi);
    return ((u64) cnt_hi << 32) | cnt_lo;
    }
#[no_mangle]
unsafe extern "C" fn arch_mmio_counter_read(cs: *mut clocksource) -> u64 {
    static u64 arch_mmio_counter_read(struct clocksource *cs)
    {
    struct arch_timer *at = cs_to_arch_timer(cs);
    return arch_counter_mmio_get_cnt(at);
    }
#[no_mangle]
unsafe extern "C" fn arch_timer_mmio_shutdown(clk: *mut clock_event_device) -> c_int {
    static int arch_timer_mmio_shutdown(struct clock_event_device *clk)
    {
    struct arch_timer *at = evt_to_arch_timer(clk);
    unsigned long ctrl;
    ctrl = arch_timer_mmio_read(at, ARCH_TIMER_REG_CTRL);
    ctrl &= ~ARCH_TIMER_CTRL_ENABLE;
    arch_timer_mmio_write(at, ARCH_TIMER_REG_CTRL, ctrl);
    return 0;
    }
    static int arch_timer_mmio_set_next_event(unsigned long evt,
    struct clock_event_device *clk)
    {
    struct arch_timer *timer = evt_to_arch_timer(clk);
    unsigned long ctrl;
    u64 cnt;
    ctrl = arch_timer_mmio_read(timer, ARCH_TIMER_REG_CTRL);
// Timer must be disabled before programming CVAL
    if (ctrl & ARCH_TIMER_CTRL_ENABLE) {
    ctrl &= ~ARCH_TIMER_CTRL_ENABLE;
    arch_timer_mmio_write(timer, ARCH_TIMER_REG_CTRL, ctrl);
    }
    ctrl |= ARCH_TIMER_CTRL_ENABLE;
    ctrl &= ~ARCH_TIMER_CTRL_IT_MASK;
    cnt = arch_counter_mmio_get_cnt(timer);
    arch_timer_mmio_write(timer, ARCH_TIMER_REG_CVAL, evt + cnt);
    arch_timer_mmio_write(timer, ARCH_TIMER_REG_CTRL, ctrl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arch_timer_mmio_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t arch_timer_mmio_handler(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    struct arch_timer *at = evt_to_arch_timer(evt);
    unsigned long ctrl;
    ctrl = arch_timer_mmio_read(at, ARCH_TIMER_REG_CTRL);
    if (ctrl & ARCH_TIMER_CTRL_IT_STAT) {
    ctrl |= ARCH_TIMER_CTRL_IT_MASK;
    arch_timer_mmio_write(at, ARCH_TIMER_REG_CTRL, ctrl);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
    static struct arch_timer_mem_frame *find_best_frame(struct platform_device *pdev)
    {
    struct arch_timer_mem_frame *frame, *best_frame = core::ptr::null_mut();
    struct arch_timer *at = platform_get_drvdata(pdev);
    void __iomem *cntctlbase;
    u32 cnttidr;
    cntctlbase = ioremap(at.gt_block.cntctlbase, at.gt_block.size);
    if (!cntctlbase) {
    dev_err(&pdev.dev, "Can't map CNTCTLBase @ %pa\n",
    &at.gt_block.cntctlbase);
    return core::ptr::null_mut();
    }
    cnttidr = readl_relaxed(cntctlbase + CNTTIDR);
//
// Try to find a virtual capable frame. Otherwise fall back to a
// physical capable frame.
//
    for (int i = 0; i < ARCH_TIMER_MEM_MAX_FRAMES; i++) {
    u32 cntacr = CNTACR_RFRQ | CNTACR_RWPT | CNTACR_RPCT |
    CNTACR_RWVT | CNTACR_RVOFF | CNTACR_RVCT;
    frame = &at.gt_block.frame[i];
    if (!frame.valid)
    continue;
// Try enabling everything, and see what sticks
    writel_relaxed(cntacr, cntctlbase + CNTACR(i));
    cntacr = readl_relaxed(cntctlbase + CNTACR(i));
// Pick a suitable frame for which we have an IRQ
    if ((cnttidr & CNTTIDR_VIRT(i)) &&
    !(~cntacr & (CNTACR_RWVT | CNTACR_RVCT)) &&
    frame.virt_irq) {
    best_frame = frame;
    at.access = VIRT_ACCESS;
    break;
    }
    if ((~cntacr & (CNTACR_RWPT | CNTACR_RPCT)) ||
    !frame.phys_irq)
    continue;
    at.access = PHYS_ACCESS;
    best_frame = frame;
    }
    iounmap(cntctlbase);
    return best_frame;
    }
#[no_mangle]
unsafe extern "C" fn arch_timer_mmio_setup(at: *mut arch_timer, irq: c_int) {
    static void arch_timer_mmio_setup(struct arch_timer *at, int irq)
    {
    at.evt = (struct clock_event_device) {
    .features		   = (CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_DYNIRQ),
    .name			   = "arch_mem_timer",
    .rating			   = 400,
    .cpumask		   = cpu_possible_mask,
    .irq 			   = irq,
    .set_next_event		   = arch_timer_mmio_set_next_event,
    .set_state_oneshot_stopped = arch_timer_mmio_shutdown,
    .set_state_shutdown	   = arch_timer_mmio_shutdown,
    };
    at.evt.set_state_shutdown(&at.evt);
    clockevents_config_and_register(&at.evt, at.rate, 0xf,
    (unsigned long)CLOCKSOURCE_MASK(56));
    enable_irq(at.evt.irq);
    at.cs = (struct clocksource) {
    .name	= "arch_mmio_counter",
    .rating	= 300,
    .read	= arch_mmio_counter_read,
    .mask	= CLOCKSOURCE_MASK(56),
    .flags	= CLOCK_SOURCE_IS_CONTINUOUS,
    };
    clocksource_register_hz(&at.cs, at.rate);
    }
    static int arch_timer_mmio_frame_register(struct platform_device *pdev,
    struct arch_timer_mem_frame *frame)
    {
    struct arch_timer *at = platform_get_drvdata(pdev);
    struct device_node *np = pdev.dev.of_node;
    int ret, irq;
    u32 rate;
    if (!devm_request_mem_region(&pdev.dev, frame.cntbase, frame.size,
    "arch_mem_timer"))
    return -EBUSY;
    at.base = devm_ioremap(&pdev.dev, frame.cntbase, frame.size);
    if (!at.base) {
    dev_err(&pdev.dev, "Can't map frame's registers\n");
    return -ENXIO;
    }
//
// Allow "clock-frequency" to override the probed rate. If neither
// lead to something useful, use the CPU timer frequency as the
// fallback. The nice thing about that last point is that we woudn't
// made it here if we didn't have a valid frequency.
//
    rate = readl_relaxed(at.base + CNTFRQ);
    if (!np || of_property_read_u32(np, "clock-frequency", &at.rate))
    at.rate = rate;
    if (!at.rate)
    at.rate = arch_timer_get_rate();
    irq = at.access == VIRT_ACCESS ? frame.virt_irq : frame.phys_irq;
    ret = devm_request_irq(&pdev.dev, irq, arch_timer_mmio_handler,
    IRQF_TIMER | IRQF_NO_AUTOEN, "arch_mem_timer",
    &at.evt);
    if (ret)
    return ret;
// Afer this point, we're not allowed to fail anymore
    arch_timer_mmio_setup(at, irq);
    return 0;
    }
    static int of_populate_gt_block(struct platform_device *pdev,
    struct arch_timer *at)
    {
    struct resource res;
    if (of_address_to_resource(pdev.dev.of_node, 0, &res))
    return -EINVAL;
    at.gt_block.cntctlbase = res.start;
    at.gt_block.size = resource_size(&res);
    for_each_available_child_of_node_scoped(pdev.dev.of_node, frame_node) {
    struct arch_timer_mem_frame *frame;
    u32 n;
    if (of_property_read_u32(frame_node, "frame-number", &n)) {
    dev_err(&pdev.dev, FW_BUG "Missing frame-number\n");
    return -EINVAL;
    }
    if (n >= ARCH_TIMER_MEM_MAX_FRAMES) {
    dev_err(&pdev.dev,
    FW_BUG "Wrong frame-number, only 0-%u are permitted\n",
    ARCH_TIMER_MEM_MAX_FRAMES - 1);
    return -EINVAL;
    }
    frame = &at.gt_block.frame[n];
    if (frame.valid) {
    dev_err(&pdev.dev, FW_BUG "Duplicated frame-number\n");
    return -EINVAL;
    }
    if (of_address_to_resource(frame_node, 0, &res))
    return -EINVAL;
    frame.cntbase = res.start;
    frame.size = resource_size(&res);
    frame.phys_irq = irq_of_parse_and_map(frame_node, 0);
    frame.virt_irq = irq_of_parse_and_map(frame_node, 1);
    frame.valid = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arch_timer_mmio_probe(pdev: *mut platform_device) -> c_int {
    static int arch_timer_mmio_probe(struct platform_device *pdev)
    {
    struct arch_timer_mem_frame *frame;
    struct arch_timer *at;
    struct device_node *np;
    int ret;
    np = pdev.dev.of_node;
    at = devm_kmalloc(&pdev.dev, sizeof(*at), GFP_KERNEL | __GFP_ZERO);
    if (!at)
    return -ENOMEM;
    if (np) {
    at.gt_block = devm_kmalloc(&pdev.dev, sizeof(*at.gt_block),
    GFP_KERNEL | __GFP_ZERO);
    if (!at.gt_block)
    return -ENOMEM;
    ret = of_populate_gt_block(pdev, at);
    if (ret)
    return ret;
    } else {
    at.gt_block = dev_get_platdata(&pdev.dev);
    }
    platform_set_drvdata(pdev, at);
    frame = find_best_frame(pdev);
    if (!frame) {
    dev_err(&pdev.dev,
    "Unable to find a suitable frame in timer @ %pa\n",
    &at.gt_block.cntctlbase);
    return -EINVAL;
    }
    ret = arch_timer_mmio_frame_register(pdev, frame);
    if (!ret)
    dev_info(&pdev.dev,
    "mmio timer running at %lu.%02luMHz (%s)\n",
    (unsigned long)at.rate / 1000000,
    (unsigned long)(at.rate / 10000) % 100,
    at.access == VIRT_ACCESS ? "virt" : "phys");
    return ret;
    }
    static const struct of_device_id arch_timer_mmio_of_table[] = {
    { .compatible = "arm,armv7-timer-mem", },
    {}
    };
    static struct platform_driver arch_timer_mmio_drv = {
    .driver	= {
    .name = "arch-timer-mmio",
    .of_match_table	= arch_timer_mmio_of_table,
    .suppress_bind_attrs = true,
    },
    .probe	= arch_timer_mmio_probe,
    };
    builtin_platform_driver(arch_timer_mmio_drv);
    static struct platform_driver arch_timer_mmio_acpi_drv = {
    .driver	= {
    .name = "gtdt-arm-mmio-timer",
    .suppress_bind_attrs = true,
    },
    .probe	= arch_timer_mmio_probe,
    };
    builtin_platform_driver(arch_timer_mmio_acpi_drv);
