//! Automatically rewritten from C to Rust
//! Source: drivers/memory/fsl_ifc.c
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
// Copyright 2011 Freescale Semiconductor, Inc
//
// Freescale Integrated Flash Controller
//
// Author: Dipen Dudhat <Dipen.Dudhat@freescale.com>
//

    struct fsl_ifc_ctrl *fsl_ifc_ctrl_dev;
    EXPORT_SYMBOL(fsl_ifc_ctrl_dev);
//
// convert_ifc_address - convert the base address
// @addr_base:	base address of the memory bank
//
#[no_mangle]
pub unsafe extern "C" fn convert_ifc_address(addr_base: phys_addr_t) -> c_uint {
    unsigned int convert_ifc_address(phys_addr_t addr_base)
    {
    return addr_base & CSPR_BA;
    }
    EXPORT_SYMBOL(convert_ifc_address);
//
// fsl_ifc_find - find IFC bank
// @addr_base:	base address of the memory bank
//
// This function walks IFC banks comparing "Base address" field of the CSPR
// registers with the supplied addr_base argument. When bases match this
// function returns bank number (starting with 0), otherwise it returns
// appropriate errno value.
//
#[no_mangle]
pub unsafe extern "C" fn fsl_ifc_find(addr_base: phys_addr_t) -> c_int {
    int fsl_ifc_find(phys_addr_t addr_base)
    {
    let mut i: c_int = 0;
    if (!fsl_ifc_ctrl_dev || !fsl_ifc_ctrl_dev.gregs)
    return -ENODEV;
    for (i = 0; i < fsl_ifc_ctrl_dev.banks; i++) {
    let mut cspr: u32 = ifc_in32(&fsl_ifc_ctrl_dev.gregs.cspr_cs[i].cspr);
    if (cspr & CSPR_V && (cspr & CSPR_BA) ==
    convert_ifc_address(addr_base))
    return i;
    }
    return -ENOENT;
    }
    EXPORT_SYMBOL(fsl_ifc_find);
#[no_mangle]
unsafe extern "C" fn fsl_ifc_ctrl_init(ctrl: *mut fsl_ifc_ctrl) -> c_int {
    static int fsl_ifc_ctrl_init(struct fsl_ifc_ctrl *ctrl)
    {
    struct fsl_ifc_global __iomem *ifc = ctrl.gregs;
//
// Clear all the common status and event registers
//
    if (ifc_in32(&ifc.cm_evter_stat) & IFC_CM_EVTER_STAT_CSER)
    ifc_out32(IFC_CM_EVTER_STAT_CSER, &ifc.cm_evter_stat);
// enable all error and events
    ifc_out32(IFC_CM_EVTER_EN_CSEREN, &ifc.cm_evter_en);
// enable all error and event interrupts
    ifc_out32(IFC_CM_EVTER_INTR_EN_CSERIREN, &ifc.cm_evter_intr_en);
    ifc_out32(0x0, &ifc.cm_erattr0);
    ifc_out32(0x0, &ifc.cm_erattr1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsl_ifc_ctrl_remove(dev: *mut platform_device) {
    static void fsl_ifc_ctrl_remove(struct platform_device *dev)
    {
    struct fsl_ifc_ctrl *ctrl = dev_get_drvdata(&dev.dev);
    of_platform_depopulate(&dev.dev);
    free_irq(ctrl.nand_irq, ctrl);
    free_irq(ctrl.irq, ctrl);
    irq_dispose_mapping(ctrl.nand_irq);
    irq_dispose_mapping(ctrl.irq);
    iounmap(ctrl.gregs);
    dev_set_drvdata(&dev.dev, core::ptr::null_mut());
    }
//
// NAND events are split between an operational interrupt which only
// receives OPC, and an error interrupt that receives everything else,
// including non-NAND errors.  Whichever interrupt gets to it first
// records the status and wakes the wait queue.
//
    static DEFINE_SPINLOCK(nand_irq_lock);
#[no_mangle]
unsafe extern "C" fn check_nand_stat(ctrl: *mut fsl_ifc_ctrl) -> u32 {
    static u32 check_nand_stat(struct fsl_ifc_ctrl *ctrl)
    {
    struct fsl_ifc_runtime __iomem *ifc = ctrl.rregs;
    unsigned long flags;
    u32 stat;
    spin_lock_irqsave(&nand_irq_lock, flags);
    stat = ifc_in32(&ifc.ifc_nand.nand_evter_stat);
    if (stat) {
    ifc_out32(stat, &ifc.ifc_nand.nand_evter_stat);
    ctrl.nand_stat = stat;
    wake_up(&ctrl.nand_wait);
    }
    spin_unlock_irqrestore(&nand_irq_lock, flags);
    return stat;
    }
#[no_mangle]
unsafe extern "C" fn fsl_ifc_nand_irq(irqno: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fsl_ifc_nand_irq(int irqno, void *data)
    {
    struct fsl_ifc_ctrl *ctrl = data;
    if (check_nand_stat(ctrl))
    return IRQ_HANDLED;
    return IRQ_NONE;
    }
//
// NOTE: This interrupt is used to report ifc events of various kinds,
// such as transaction errors on the chipselects.
//
#[no_mangle]
unsafe extern "C" fn fsl_ifc_ctrl_irq(irqno: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t fsl_ifc_ctrl_irq(int irqno, void *data)
    {
    struct fsl_ifc_ctrl *ctrl = data;
    struct fsl_ifc_global __iomem *ifc = ctrl.gregs;
    u32 err_axiid, err_srcid, status, cs_err, err_addr;
    let mut ret: irqreturn_t = IRQ_NONE;
// read for chip select error
    cs_err = ifc_in32(&ifc.cm_evter_stat);
    if (cs_err) {
    dev_err(ctrl.dev, "transaction sent to IFC is not mapped to any memory bank 0x%08X\n",
    cs_err);
// clear the chip select error
    ifc_out32(IFC_CM_EVTER_STAT_CSER, &ifc.cm_evter_stat);
// read error attribute registers print the error information
    status = ifc_in32(&ifc.cm_erattr0);
    err_addr = ifc_in32(&ifc.cm_erattr1);
    if (status & IFC_CM_ERATTR0_ERTYP_READ)
    dev_err(ctrl.dev, "Read transaction error CM_ERATTR0 0x%08X\n",
    status);
    else
    dev_err(ctrl.dev, "Write transaction error CM_ERATTR0 0x%08X\n",
    status);
    err_axiid = (status & IFC_CM_ERATTR0_ERAID) >>
    IFC_CM_ERATTR0_ERAID_SHIFT;
    dev_err(ctrl.dev, "AXI ID of the error transaction 0x%08X\n",
    err_axiid);
    err_srcid = (status & IFC_CM_ERATTR0_ESRCID) >>
    IFC_CM_ERATTR0_ESRCID_SHIFT;
    dev_err(ctrl.dev, "SRC ID of the error transaction 0x%08X\n",
    err_srcid);
    dev_err(ctrl.dev, "Transaction Address corresponding to error ERADDR 0x%08X\n",
    err_addr);
    ret = IRQ_HANDLED;
    }
    if (check_nand_stat(ctrl))
    ret = IRQ_HANDLED;
    return ret;
    }
//
// fsl_ifc_ctrl_probe
//
// called by device layer when it finds a device matching
// one our driver can handled. This code allocates all of
// the resources needed for the controller only.  The
// resources for the NAND banks themselves are allocated
// in the chip probe function.
//
#[no_mangle]
unsafe extern "C" fn fsl_ifc_ctrl_probe(dev: *mut platform_device) -> c_int {
    static int fsl_ifc_ctrl_probe(struct platform_device *dev)
    {
    let mut ret: c_int = 0;
    int version, banks;
    void __iomem *addr;
    dev_info(&dev.dev, "Freescale Integrated Flash Controller\n");
    fsl_ifc_ctrl_dev = devm_kzalloc(&dev.dev, sizeof(*fsl_ifc_ctrl_dev),
    GFP_KERNEL);
    if (!fsl_ifc_ctrl_dev)
    return -ENOMEM;
    dev_set_drvdata(&dev.dev, fsl_ifc_ctrl_dev);
// IOMAP the entire IFC region
    fsl_ifc_ctrl_dev.gregs = of_iomap(dev.dev.of_node, 0);
    if (!fsl_ifc_ctrl_dev.gregs) {
    dev_err(&dev.dev, "failed to get memory region\n");
    return -ENODEV;
    }
    if (of_property_read_bool(dev.dev.of_node, "little-endian")) {
    fsl_ifc_ctrl_dev.little_endian = true;
    dev_dbg(&dev.dev, "IFC REGISTERS are LITTLE endian\n");
    } else {
    fsl_ifc_ctrl_dev.little_endian = false;
    dev_dbg(&dev.dev, "IFC REGISTERS are BIG endian\n");
    }
    version = ifc_in32(&fsl_ifc_ctrl_dev.gregs.ifc_rev) &
    FSL_IFC_VERSION_MASK;
    banks = (version == FSL_IFC_VERSION_1_0_0) ? 4 : 8;
    dev_info(&dev.dev, "IFC version %d.%d, %d banks\n",
    version >> 24, (version >> 16) & 0xf, banks);
    fsl_ifc_ctrl_dev.version = version;
    fsl_ifc_ctrl_dev.banks = banks;
    addr = fsl_ifc_ctrl_dev.gregs;
    if (version >= FSL_IFC_VERSION_2_0_0)
    addr += PGOFFSET_64K;
    else
    addr += PGOFFSET_4K;
    fsl_ifc_ctrl_dev.rregs = addr;
// get the Controller level irq
    fsl_ifc_ctrl_dev.irq = irq_of_parse_and_map(dev.dev.of_node, 0);
    if (fsl_ifc_ctrl_dev.irq == 0) {
    dev_err(&dev.dev, "failed to get irq resource for IFC\n");
    ret = -ENODEV;
    goto err;
    }
// get the nand machine irq
    fsl_ifc_ctrl_dev.nand_irq =
    irq_of_parse_and_map(dev.dev.of_node, 1);
    fsl_ifc_ctrl_dev.dev = &dev.dev;
    ret = fsl_ifc_ctrl_init(fsl_ifc_ctrl_dev);
    if (ret < 0)
    goto err_unmap_nandirq;
    init_waitqueue_head(&fsl_ifc_ctrl_dev.nand_wait);
    ret = request_irq(fsl_ifc_ctrl_dev.irq, fsl_ifc_ctrl_irq, IRQF_SHARED,
    "fsl-ifc", fsl_ifc_ctrl_dev);
    if (ret != 0) {
    dev_err(&dev.dev, "failed to install irq (%d)\n",
    fsl_ifc_ctrl_dev.irq);
    goto err_unmap_nandirq;
    }
    if (fsl_ifc_ctrl_dev.nand_irq) {
    ret = request_irq(fsl_ifc_ctrl_dev.nand_irq, fsl_ifc_nand_irq,
    0, "fsl-ifc-nand", fsl_ifc_ctrl_dev);
    if (ret != 0) {
    dev_err(&dev.dev, "failed to install irq (%d)\n",
    fsl_ifc_ctrl_dev.nand_irq);
    goto err_free_irq;
    }
    }
// legacy dts may still use "simple-bus" compatible
    ret = of_platform_default_populate(dev.dev.of_node, core::ptr::null_mut(), &dev.dev);
    if (ret)
    goto err_free_nandirq;
    return 0;
    err_free_nandirq:
    free_irq(fsl_ifc_ctrl_dev.nand_irq, fsl_ifc_ctrl_dev);
    err_free_irq:
    free_irq(fsl_ifc_ctrl_dev.irq, fsl_ifc_ctrl_dev);
    err_unmap_nandirq:
    irq_dispose_mapping(fsl_ifc_ctrl_dev.nand_irq);
    irq_dispose_mapping(fsl_ifc_ctrl_dev.irq);
    err:
    iounmap(fsl_ifc_ctrl_dev.gregs);
    return ret;
    }
    static const struct of_device_id fsl_ifc_match[] = {
    {
    .compatible = "fsl,ifc",
    },
    {},
    };
    static struct platform_driver fsl_ifc_ctrl_driver = {
    .driver = {
    .name	= "fsl-ifc",
    .of_match_table = fsl_ifc_match,
    },
    .probe       = fsl_ifc_ctrl_probe,
    .remove  = fsl_ifc_ctrl_remove,
    };
#[no_mangle]
unsafe extern "C" fn fsl_ifc_init() -> int __init {
    static int __init fsl_ifc_init(void)
    {
    return platform_driver_register(&fsl_ifc_ctrl_driver);
    }
    subsys_initcall(fsl_ifc_init);
    MODULE_AUTHOR("Freescale Semiconductor");
    MODULE_DESCRIPTION("Freescale Integrated Flash Controller driver");
