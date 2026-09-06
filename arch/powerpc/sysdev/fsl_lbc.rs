//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/fsl_lbc.c
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
// Freescale LBC and UPM routines.
//
// Copyright © 2007-2008  MontaVista Software, Inc.
// Copyright © 2010 Freescale Semiconductor
//
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
// Author: Jack Lan <Jack.Lan@freescale.com>
// Author: Roy Zang <tie-fei.zang@freescale.com>
//

    static DEFINE_SPINLOCK(fsl_lbc_lock);
    struct fsl_lbc_ctrl *fsl_lbc_ctrl_dev;
    EXPORT_SYMBOL(fsl_lbc_ctrl_dev);
//
// fsl_lbc_addr - convert the base address
// @addr_base:	base address of the memory bank
//
// This function converts a base address of lbc into the right format for the
// BR register. If the SOC has eLBC then it returns 32bit physical address
// else it converts a 34bit local bus physical address to correct format of
// 32bit address for BR register (Example: MPC8641).
//
#[no_mangle]
pub unsafe extern "C" fn fsl_lbc_addr(addr_base: phys_addr_t) -> u32 {
    u32 fsl_lbc_addr(phys_addr_t addr_base)
    {
    struct device_node *np = fsl_lbc_ctrl_dev.dev.of_node;
    let mut addr: u32 = addr_base & 0xffff8000;
    if (of_device_is_compatible(np, "fsl,elbc"))
    return addr;
    return addr | ((addr_base & 0x300000000ull) >> 19);
    }
    EXPORT_SYMBOL(fsl_lbc_addr);
//
// fsl_lbc_find - find Localbus bank
// @addr_base:	base address of the memory bank
//
// This function walks LBC banks comparing "Base address" field of the BR
// registers with the supplied addr_base argument. When bases match this
// function returns bank number (starting with 0), otherwise it returns
// appropriate errno value.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_lbc_find(addr_base: phys_addr_t) -> c_int {
    int fsl_lbc_find(phys_addr_t addr_base)
    {
    int i;
    struct fsl_lbc_regs __iomem *lbc;
    if (!fsl_lbc_ctrl_dev || !fsl_lbc_ctrl_dev.regs)
    return -ENODEV;
    lbc = fsl_lbc_ctrl_dev.regs;
    for (i = 0; i < ARRAY_SIZE(lbc.bank); i++) {
    let mut br: u32 = in_be32(&lbc.bank[i].br);
    let mut or: u32 = in_be32(&lbc.bank[i].or);
    if (br & BR_V && (br & or & BR_BA) == fsl_lbc_addr(addr_base))
    return i;
    }
    return -ENOENT;
    }
    EXPORT_SYMBOL(fsl_lbc_find);
//
// fsl_upm_find - find pre-programmed UPM via base address
// @addr_base:	base address of the memory bank controlled by the UPM
// @upm:	pointer to the allocated fsl_upm structure
//
// This function fills fsl_upm structure so you can use it with the rest of
// UPM API. On success this function returns 0, otherwise it returns
// appropriate errno value.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_upm_find(addr_base: phys_addr_t, upm: *mut fsl_upm) -> c_int {
    int fsl_upm_find(phys_addr_t addr_base, struct fsl_upm *upm)
    {
    int bank;
    u32 br;
    struct fsl_lbc_regs __iomem *lbc;
    bank = fsl_lbc_find(addr_base);
    if (bank < 0)
    return bank;
    if (!fsl_lbc_ctrl_dev || !fsl_lbc_ctrl_dev.regs)
    return -ENODEV;
    lbc = fsl_lbc_ctrl_dev.regs;
    br = in_be32(&lbc.bank[bank].br);
    switch (br & BR_MSEL) {
    case BR_MS_UPMA:
    upm.mxmr = &lbc.mamr;
    break;
    case BR_MS_UPMB:
    upm.mxmr = &lbc.mbmr;
    break;
    case BR_MS_UPMC:
    upm.mxmr = &lbc.mcmr;
    break;
    default:
    return -EINVAL;
    }
    switch (br & BR_PS) {
    case BR_PS_8:
    upm.width = 8;
    break;
    case BR_PS_16:
    upm.width = 16;
    break;
    case BR_PS_32:
    upm.width = 32;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    EXPORT_SYMBOL(fsl_upm_find);
//
// fsl_upm_run_pattern - actually run an UPM pattern
// @upm:	pointer to the fsl_upm structure obtained via fsl_upm_find
// @io_base:	remapped pointer to where memory access should happen
// @mar:	MAR register content during pattern execution
//
// This function triggers dummy write to the memory specified by the io_base,
// thus UPM pattern actually executed. Note that mar usage depends on the
// pre-programmed AMX bits in the UPM RAM.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_upm_run_pattern(upm: *mut fsl_upm, io_base: *mut void __iomem, mar: u32) -> c_int {
    int fsl_upm_run_pattern(struct fsl_upm *upm, void __iomem *io_base, u32 mar)
    {
    let mut ret: c_int = 0;
    unsigned long flags;
    if (!fsl_lbc_ctrl_dev || !fsl_lbc_ctrl_dev.regs)
    return -ENODEV;
    spin_lock_irqsave(&fsl_lbc_lock, flags);
    out_be32(&fsl_lbc_ctrl_dev.regs.mar, mar);
    switch (upm.width) {
    case 8:
    out_8(io_base, 0x0);
    break;
    case 16:
    out_be16(io_base, 0x0);
    break;
    case 32:
    out_be32(io_base, 0x0);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    spin_unlock_irqrestore(&fsl_lbc_lock, flags);
    return ret;
    }
    EXPORT_SYMBOL(fsl_upm_run_pattern);
    static int fsl_lbc_ctrl_init(struct fsl_lbc_ctrl *ctrl,
    struct device_node *node)
    {
    struct fsl_lbc_regs __iomem *lbc = ctrl.regs;
// clear event registers
    setbits32(&lbc.ltesr, LTESR_CLEAR);
    out_be32(&lbc.lteatr, 0);
    out_be32(&lbc.ltear, 0);
    out_be32(&lbc.lteccr, LTECCR_CLEAR);
    out_be32(&lbc.ltedr, LTEDR_ENABLE);
// Set the monitor timeout value to the maximum for erratum A001
    if (of_device_is_compatible(node, "fsl,elbc"))
    clrsetbits_be32(&lbc.lbcr, LBCR_BMT, LBCR_BMTPS);
    return 0;
    }
//
// NOTE: This interrupt is used to report localbus events of various kinds,
// such as transaction errors on the chipselects.
//
#[no_mangle]
unsafe extern "C" fn fsl_lbc_ctrl_irq(irqno: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fsl_lbc_ctrl_irq(int irqno, void *data)
    {
    struct fsl_lbc_ctrl *ctrl = data;
    struct fsl_lbc_regs __iomem *lbc = ctrl.regs;
    u32 status;
    unsigned long flags;
    spin_lock_irqsave(&fsl_lbc_lock, flags);
    status = in_be32(&lbc.ltesr);
    if (!status) {
    spin_unlock_irqrestore(&fsl_lbc_lock, flags);
    return IRQ_NONE;
    }
    out_be32(&lbc.ltesr, LTESR_CLEAR);
    out_be32(&lbc.lteatr, 0);
    out_be32(&lbc.ltear, 0);
    ctrl.irq_status = status;
    if (status & LTESR_BM)
    dev_err(ctrl.dev, "Local bus monitor time-out: "
    "LTESR 0x%08X\n", status);
    if (status & LTESR_WP)
    dev_err(ctrl.dev, "Write protect error: "
    "LTESR 0x%08X\n", status);
    if (status & LTESR_ATMW)
    dev_err(ctrl.dev, "Atomic write error: "
    "LTESR 0x%08X\n", status);
    if (status & LTESR_ATMR)
    dev_err(ctrl.dev, "Atomic read error: "
    "LTESR 0x%08X\n", status);
    if (status & LTESR_CS)
    dev_err(ctrl.dev, "Chip select error: "
    "LTESR 0x%08X\n", status);
    if (status & LTESR_FCT) {
    dev_err(ctrl.dev, "FCM command time-out: "
    "LTESR 0x%08X\n", status);
    smp_wmb();
    wake_up(&ctrl.irq_wait);
    }
    if (status & LTESR_PAR) {
    dev_err(ctrl.dev, "Parity or Uncorrectable ECC error: "
    "LTESR 0x%08X\n", status);
    smp_wmb();
    wake_up(&ctrl.irq_wait);
    }
    if (status & LTESR_CC) {
    smp_wmb();
    wake_up(&ctrl.irq_wait);
    }
    if (status & ~LTESR_MASK)
    dev_err(ctrl.dev, "Unknown error: "
    "LTESR 0x%08X\n", status);
    spin_unlock_irqrestore(&fsl_lbc_lock, flags);
    return IRQ_HANDLED;
    }
//
// fsl_lbc_ctrl_probe
//
// called by device layer when it finds a device matching
// one our driver can handled. This code allocates all of
// the resources needed for the controller only.  The
// resources for the NAND banks themselves are allocated
// in the chip probe function.
//
#[no_mangle]
unsafe extern "C" fn fsl_lbc_ctrl_probe(dev: *mut platform_device) -> c_int {
    static int fsl_lbc_ctrl_probe(struct platform_device *dev)
    {
    int ret;
    if (!dev.dev.of_node) {
    dev_err(&dev.dev, "Device OF-Node is core::ptr::null_mut()");
    return -EFAULT;
    }
    fsl_lbc_ctrl_dev = kzalloc_obj(*fsl_lbc_ctrl_dev);
    if (!fsl_lbc_ctrl_dev)
    return -ENOMEM;
    dev_set_drvdata(&dev.dev, fsl_lbc_ctrl_dev);
    spin_lock_init(&fsl_lbc_ctrl_dev.lock);
    init_waitqueue_head(&fsl_lbc_ctrl_dev.irq_wait);
    fsl_lbc_ctrl_dev.regs = of_iomap(dev.dev.of_node, 0);
    if (!fsl_lbc_ctrl_dev.regs) {
    dev_err(&dev.dev, "failed to get memory region\n");
    ret = -ENODEV;
    goto err;
    }
    fsl_lbc_ctrl_dev.irq[0] = irq_of_parse_and_map(dev.dev.of_node, 0);
    if (!fsl_lbc_ctrl_dev.irq[0]) {
    dev_err(&dev.dev, "failed to get irq resource\n");
    ret = -ENODEV;
    goto err;
    }
    fsl_lbc_ctrl_dev.dev = &dev.dev;
    ret = fsl_lbc_ctrl_init(fsl_lbc_ctrl_dev, dev.dev.of_node);
    if (ret < 0)
    goto err;
    ret = request_irq(fsl_lbc_ctrl_dev.irq[0], fsl_lbc_ctrl_irq, 0,
    "fsl-lbc", fsl_lbc_ctrl_dev);
    if (ret != 0) {
    dev_err(&dev.dev, "failed to install irq (%d)\n",
    fsl_lbc_ctrl_dev.irq[0]);
    ret = fsl_lbc_ctrl_dev.irq[0];
    goto err;
    }
    fsl_lbc_ctrl_dev.irq[1] = irq_of_parse_and_map(dev.dev.of_node, 1);
    if (fsl_lbc_ctrl_dev.irq[1]) {
    ret = request_irq(fsl_lbc_ctrl_dev.irq[1], fsl_lbc_ctrl_irq,
    IRQF_SHARED, "fsl-lbc-err", fsl_lbc_ctrl_dev);
    if (ret) {
    dev_err(&dev.dev, "failed to install irq (%d)\n",
    fsl_lbc_ctrl_dev.irq[1]);
    ret = fsl_lbc_ctrl_dev.irq[1];
    goto err1;
    }
    }
// Enable interrupts for any detected events
    out_be32(&fsl_lbc_ctrl_dev.regs.lteir, LTEIR_ENABLE);
    return 0;
    err1:
    free_irq(fsl_lbc_ctrl_dev.irq[0], fsl_lbc_ctrl_dev);
    err:
    iounmap(fsl_lbc_ctrl_dev.regs);
    kfree(fsl_lbc_ctrl_dev);
    fsl_lbc_ctrl_dev = core::ptr::null_mut();
    return ret;
    }

// save lbc registers
#[no_mangle]
unsafe extern "C" fn fsl_lbc_syscore_suspend(data: *mut c_void) -> c_int {
    static int fsl_lbc_syscore_suspend(void *data)
    {
    struct fsl_lbc_ctrl *ctrl;
    struct fsl_lbc_regs __iomem *lbc;
    ctrl = fsl_lbc_ctrl_dev;
    if (!ctrl)
    goto out;
    lbc = ctrl.regs;
    if (!lbc)
    goto out;
    ctrl.saved_regs = kmalloc_obj(struct fsl_lbc_regs);
    if (!ctrl.saved_regs)
    return -ENOMEM;
    _memcpy_fromio(ctrl.saved_regs, lbc, sizeof(struct fsl_lbc_regs));
    out:
    return 0;
    }
// restore lbc registers
#[no_mangle]
unsafe extern "C" fn fsl_lbc_syscore_resume(data: *mut c_void) {
    static void fsl_lbc_syscore_resume(void *data)
    {
    struct fsl_lbc_ctrl *ctrl;
    struct fsl_lbc_regs __iomem *lbc;
    ctrl = fsl_lbc_ctrl_dev;
    if (!ctrl)
    goto out;
    lbc = ctrl.regs;
    if (!lbc)
    goto out;
    if (ctrl.saved_regs) {
    _memcpy_toio(lbc, ctrl.saved_regs,
    sizeof(struct fsl_lbc_regs));
    kfree(ctrl.saved_regs);
    ctrl.saved_regs = core::ptr::null_mut();
    }
    out:
    return;
    }

    static const struct of_device_id fsl_lbc_match[] = {
    { .compatible = "fsl,elbc", },
    { .compatible = "fsl,pq3-localbus", },
    { .compatible = "fsl,pq2-localbus", },
    { .compatible = "fsl,pq2pro-localbus", },
    {},
    };

    static const struct syscore_ops lbc_syscore_pm_ops = {
    .suspend = fsl_lbc_syscore_suspend,
    .resume = fsl_lbc_syscore_resume,
    };
    static struct syscore lbc_syscore_pm = {
    .ops = &lbc_syscore_pm_ops,
    };

    static struct platform_driver fsl_lbc_ctrl_driver = {
    .driver = {
    .name = "fsl-lbc",
    .of_match_table = fsl_lbc_match,
    },
    .probe = fsl_lbc_ctrl_probe,
    };
#[no_mangle]
unsafe extern "C" fn fsl_lbc_init() -> int __init {
    static int __init fsl_lbc_init(void)
    {

    register_syscore(&lbc_syscore_pm);

    return platform_driver_register(&fsl_lbc_ctrl_driver);
    }
    subsys_initcall(fsl_lbc_init);
