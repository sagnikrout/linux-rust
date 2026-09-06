//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/suspend.c
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
// MPC83xx suspend support
//
// Author: Scott Wood <scottwood@freescale.com>
//
// Copyright (c) 2006-2007 Freescale Semiconductor, Inc.
//

pub const PMCCR1_NEXT_STATE: c_uint = 0x0C /* Next state for power management */;
pub const PMCCR1_NEXT_STATE_SHIFT: c_int = 2;
pub const PMCCR1_CURR_STATE: c_uint = 0x03 /* Current state for power management*/;
pub const IMMR_SYSCR_OFFSET: c_uint = 0x100;
pub const IMMR_RCW_OFFSET: c_uint = 0x900;
pub const RCW_PCI_HOST: c_uint = 0x80000000;
    void mpc83xx_enter_deep_sleep(phys_addr_t immrbase);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc83xx_pmc {
    pub config: u32,

    pub event: u32,
    pub mask: u32,
// All but PMCI are deep-sleep only
pub const PMCER_GPIO: c_uint = 0x100;
pub const PMCER_PCI: c_uint = 0x080;
pub const PMCER_USB: c_uint = 0x040;
pub const PMCER_ETSEC1: c_uint = 0x020;
pub const PMCER_ETSEC2: c_uint = 0x010;
pub const PMCER_TIMER: c_uint = 0x008;
pub const PMCER_INT1: c_uint = 0x004;
pub const PMCER_INT2: c_uint = 0x002;
pub const PMCER_PMCI: c_uint = 0x001;
pub const PMCER_ALL: c_uint = 0x1FF;
// deep-sleep only
    pub config1: u32,
pub const PMCCR1_USE_STATE: c_uint = 0x80000000;
pub const PMCCR1_PME_EN: c_uint = 0x00000080;
pub const PMCCR1_ASSERT_PME: c_uint = 0x00000040;
pub const PMCCR1_POWER_OFF: c_uint = 0x00000020;
// deep-sleep only
    pub config2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc83xx_rcw {
    pub rcwlr: u32,
    pub rcwhr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc83xx_clock {
    pub spmr: u32,
    pub occr: u32,
    pub sccr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc83xx_syscr {
    pub sgprl: __be32,
    pub sgprh: __be32,
    pub spridr: __be32,
    pub :32: __be32,
    pub spcr: __be32,
    pub sicrl: __be32,
    pub sicrh: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc83xx_saved {
    pub sicrl: u32,
    pub sicrh: u32,
    pub sccr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_type {
    pub has_deep_sleep: c_int,
}

    static int has_deep_sleep, deep_sleeping;
    static int pmc_irq;
    static struct mpc83xx_pmc __iomem *pmc_regs;
    static struct mpc83xx_clock __iomem *clock_regs;
    static struct mpc83xx_syscr __iomem *syscr_regs;
    static struct mpc83xx_saved saved_regs;
    static int is_pci_agent, wake_from_pci;
    static phys_addr_t immrbase;
    static int pci_pm_state;
    static DECLARE_WAIT_QUEUE_HEAD(agent_wq);
#[no_mangle]
pub unsafe extern "C" fn fsl_deep_sleep() -> c_int {
    int fsl_deep_sleep(void)
    {
    return deep_sleeping;
    }
    EXPORT_SYMBOL(fsl_deep_sleep);
#[no_mangle]
unsafe extern "C" fn mpc83xx_change_state() -> c_int {
    static int mpc83xx_change_state(void)
    {
    u32 curr_state;
    let mut reg_cfg1: u32 = in_be32(&pmc_regs.config1);
    if (is_pci_agent) {
    pci_pm_state = (reg_cfg1 & PMCCR1_NEXT_STATE) >>
    PMCCR1_NEXT_STATE_SHIFT;
    curr_state = reg_cfg1 & PMCCR1_CURR_STATE;
    if (curr_state != pci_pm_state) {
    reg_cfg1 &= ~PMCCR1_CURR_STATE;
    reg_cfg1 |= pci_pm_state;
    out_be32(&pmc_regs.config1, reg_cfg1);
    wake_up(&agent_wq);
    return 1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pmc_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pmc_irq_handler(int irq, void *dev_id)
    {
    let mut event: u32 = in_be32(&pmc_regs.event);
    let mut ret: c_int = IRQ_NONE;
    if (mpc83xx_change_state())
    ret = IRQ_HANDLED;
    if (event) {
    out_be32(&pmc_regs.event, event);
    ret = IRQ_HANDLED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mpc83xx_suspend_restore_regs() {
    static void mpc83xx_suspend_restore_regs(void)
    {
    out_be32(&syscr_regs.sicrl, saved_regs.sicrl);
    out_be32(&syscr_regs.sicrh, saved_regs.sicrh);
    out_be32(&clock_regs.sccr, saved_regs.sccr);
    }
#[no_mangle]
unsafe extern "C" fn mpc83xx_suspend_save_regs() {
    static void mpc83xx_suspend_save_regs(void)
    {
    saved_regs.sicrl = in_be32(&syscr_regs.sicrl);
    saved_regs.sicrh = in_be32(&syscr_regs.sicrh);
    saved_regs.sccr = in_be32(&clock_regs.sccr);
    }
#[no_mangle]
unsafe extern "C" fn mpc83xx_suspend_enter(state: suspend_state_t) -> c_int {
    static int mpc83xx_suspend_enter(suspend_state_t state)
    {
    let mut ret: c_int = -EAGAIN;
// Don't go to sleep if there's a race where pci_pm_state changes
// between the agent thread checking it and the PM code disabling
// interrupts.
//
    if (wake_from_pci) {
    if (pci_pm_state != (deep_sleeping ? 3 : 2))
    goto out;
    out_be32(&pmc_regs.config1,
    in_be32(&pmc_regs.config1) | PMCCR1_PME_EN);
    }
// Put the system into low-power mode and the RAM
// into self-refresh mode once the core goes to
// sleep.
//
    out_be32(&pmc_regs.config, PMCCR_SLPEN | PMCCR_DLPEN);
// If it has deep sleep (i.e. it's an 831x or compatible),
// disable power to the core upon entering sleep mode.  This will
// require going through the boot firmware upon a wakeup event.
//
    if (deep_sleeping) {
    mpc83xx_suspend_save_regs();
    out_be32(&pmc_regs.mask, PMCER_ALL);
    out_be32(&pmc_regs.config1,
    in_be32(&pmc_regs.config1) | PMCCR1_POWER_OFF);
    if (IS_ENABLED(CONFIG_PPC_FPU))
    enable_kernel_fp();
    mpc83xx_enter_deep_sleep(immrbase);
    out_be32(&pmc_regs.config1,
    in_be32(&pmc_regs.config1) & ~PMCCR1_POWER_OFF);
    out_be32(&pmc_regs.mask, PMCER_PMCI);
    mpc83xx_suspend_restore_regs();
    } else {
    out_be32(&pmc_regs.mask, PMCER_PMCI);
    mpc6xx_enter_standby();
    }
    ret = 0;
    out:
    out_be32(&pmc_regs.config1,
    in_be32(&pmc_regs.config1) & ~PMCCR1_PME_EN);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mpc83xx_suspend_end() {
    static void mpc83xx_suspend_end(void)
    {
    deep_sleeping = 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc83xx_suspend_valid(state: suspend_state_t) -> c_int {
    static int mpc83xx_suspend_valid(suspend_state_t state)
    {
    let mut state: return = = PM_SUSPEND_STANDBY || state == PM_SUSPEND_MEM;
    }
#[no_mangle]
unsafe extern "C" fn mpc83xx_suspend_begin(state: suspend_state_t) -> c_int {
    static int mpc83xx_suspend_begin(suspend_state_t state)
    {
    switch (state) {
    case PM_SUSPEND_STANDBY:
    deep_sleeping = 0;
    return 0;
    case PM_SUSPEND_MEM:
    if (has_deep_sleep)
    deep_sleeping = 1;
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn agent_thread_fn(data: *mut c_void) -> c_int {
    static int agent_thread_fn(void *data)
    {
    set_freezable();
    while (1) {
    wait_event_freezable(agent_wq, pci_pm_state >= 2);
    if (signal_pending(current) || pci_pm_state < 2)
    continue;
// With a preemptible kernel (or SMP), this could race with
// a userspace-driven suspend request.  It's probably best
// to avoid mixing the two with such a configuration (or
// else fix it by adding a mutex to state_store that we can
// synchronize with).
//
    wake_from_pci = 1;
    pm_suspend(pci_pm_state == 3 ? PM_SUSPEND_MEM :
    PM_SUSPEND_STANDBY);
    wake_from_pci = 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc83xx_set_agent() {
    static void mpc83xx_set_agent(void)
    {
    out_be32(&pmc_regs.config1, PMCCR1_USE_STATE);
    out_be32(&pmc_regs.mask, PMCER_PMCI);
    kthread_run(agent_thread_fn, core::ptr::null_mut(), "PCI power mgt");
    }
#[no_mangle]
unsafe extern "C" fn mpc83xx_is_pci_agent() -> c_int {
    static int mpc83xx_is_pci_agent(void)
    {
    struct mpc83xx_rcw __iomem *rcw_regs;
    int ret;
    rcw_regs = ioremap(get_immrbase() + IMMR_RCW_OFFSET,
    sizeof(struct mpc83xx_rcw));
    if (!rcw_regs)
    return -ENOMEM;
    ret = !(in_be32(&rcw_regs.rcwhr) & RCW_PCI_HOST);
    iounmap(rcw_regs);
    return ret;
    }
    static const struct platform_suspend_ops mpc83xx_suspend_ops = {
    .valid = mpc83xx_suspend_valid,
    .begin = mpc83xx_suspend_begin,
    .enter = mpc83xx_suspend_enter,
    .end = mpc83xx_suspend_end,
    };
    static struct pmc_type pmc_types[] = {
    {
    .has_deep_sleep = 1,
    },
    {
    .has_deep_sleep = 0,
    }
    };
    static const struct of_device_id pmc_match[] = {
    {
    .compatible = "fsl,mpc8313-pmc",
    .data = &pmc_types[0],
    },
    {
    .compatible = "fsl,mpc8349-pmc",
    .data = &pmc_types[1],
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn pmc_probe(ofdev: *mut platform_device) -> c_int {
    static int pmc_probe(struct platform_device *ofdev)
    {
    struct device_node *np = ofdev.dev.of_node;
    struct resource res;
    const struct pmc_type *type;
    let mut ret: c_int = 0;
    type = of_device_get_match_data(&ofdev.dev);
    if (!type)
    return -EINVAL;
    if (!of_device_is_available(np))
    return -ENODEV;
    has_deep_sleep = type.has_deep_sleep;
    immrbase = get_immrbase();
    is_pci_agent = mpc83xx_is_pci_agent();
    if (is_pci_agent < 0)
    return is_pci_agent;
    ret = of_address_to_resource(np, 0, &res);
    if (ret)
    return -ENODEV;
    pmc_irq = irq_of_parse_and_map(np, 0);
    if (pmc_irq) {
    ret = request_irq(pmc_irq, pmc_irq_handler, IRQF_SHARED,
    "pmc", ofdev);
    if (ret)
    return -EBUSY;
    }
    pmc_regs = ioremap(res.start, sizeof(*pmc_regs));
    if (!pmc_regs) {
    ret = -ENOMEM;
    goto out;
    }
    ret = of_address_to_resource(np, 1, &res);
    if (ret) {
    ret = -ENODEV;
    goto out_pmc;
    }
    clock_regs = ioremap(res.start, sizeof(*clock_regs));
    if (!clock_regs) {
    ret = -ENOMEM;
    goto out_pmc;
    }
    if (has_deep_sleep) {
    syscr_regs = ioremap(immrbase + IMMR_SYSCR_OFFSET,
    sizeof(*syscr_regs));
    if (!syscr_regs) {
    ret = -ENOMEM;
    goto out_syscr;
    }
    }
    if (is_pci_agent)
    mpc83xx_set_agent();
    suspend_set_ops(&mpc83xx_suspend_ops);
    return 0;
    out_syscr:
    iounmap(clock_regs);
    out_pmc:
    iounmap(pmc_regs);
    out:
    if (pmc_irq)
    free_irq(pmc_irq, ofdev);
    return ret;
    }
    static struct platform_driver pmc_driver = {
    .driver = {
    .name = "mpc83xx-pmc",
    .of_match_table = pmc_match,
    .suppress_bind_attrs = true,
    },
    .probe = pmc_probe,
    };
    builtin_platform_driver(pmc_driver);
