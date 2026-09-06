//! Automatically rewritten from C to Rust
//! Source: drivers/edac/highbank_mc_edac.c
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
// Copyright 2011-2012 Calxeda, Inc.
//

// DDR Ctrlr Error Registers
pub const HB_DDR_ECC_ERR_BASE: c_uint = 0x128;
pub const MW_DDR_ECC_ERR_BASE: c_uint = 0x1b4;
pub const HB_DDR_ECC_OPT: c_uint = 0x00;
pub const HB_DDR_ECC_U_ERR_ADDR: c_uint = 0x08;
pub const HB_DDR_ECC_U_ERR_STAT: c_uint = 0x0c;
pub const HB_DDR_ECC_U_ERR_DATAL: c_uint = 0x10;
pub const HB_DDR_ECC_U_ERR_DATAH: c_uint = 0x14;
pub const HB_DDR_ECC_C_ERR_ADDR: c_uint = 0x18;
pub const HB_DDR_ECC_C_ERR_STAT: c_uint = 0x1c;
pub const HB_DDR_ECC_C_ERR_DATAL: c_uint = 0x20;
pub const HB_DDR_ECC_C_ERR_DATAH: c_uint = 0x24;
pub const HB_DDR_ECC_OPT_MODE_MASK: c_uint = 0x3;
pub const HB_DDR_ECC_OPT_FWC: c_uint = 0x100;
pub const HB_DDR_ECC_OPT_XOR_SHIFT: c_int = 16;
// DDR Ctrlr Interrupt Registers
pub const HB_DDR_ECC_INT_BASE: c_uint = 0x180;
pub const MW_DDR_ECC_INT_BASE: c_uint = 0x218;
pub const HB_DDR_ECC_INT_STATUS: c_uint = 0x00;
pub const HB_DDR_ECC_INT_ACK: c_uint = 0x04;
pub const HB_DDR_ECC_INT_STAT_CE: c_uint = 0x8;
pub const HB_DDR_ECC_INT_STAT_DOUBLE_CE: c_uint = 0x10;
pub const HB_DDR_ECC_INT_STAT_UE: c_uint = 0x20;
pub const HB_DDR_ECC_INT_STAT_DOUBLE_UE: c_uint = 0x40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hb_mc_drvdata {
    pub mc_err_base: *mut void __iomem,
    pub mc_int_base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn highbank_mc_err_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t highbank_mc_err_handler(int irq, void *dev_id)
    {
    struct mem_ctl_info *mci = dev_id;
    struct hb_mc_drvdata *drvdata = mci.pvt_info;
    u32 status, err_addr;
// Read the interrupt status register
    status = readl(drvdata.mc_int_base + HB_DDR_ECC_INT_STATUS);
    if (status & HB_DDR_ECC_INT_STAT_UE) {
    err_addr = readl(drvdata.mc_err_base + HB_DDR_ECC_U_ERR_ADDR);
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1,
    err_addr >> PAGE_SHIFT,
    err_addr & ~PAGE_MASK, 0,
    0, 0, -1,
    mci.ctl_name, "");
    }
    if (status & HB_DDR_ECC_INT_STAT_CE) {
    let mut syndrome: u32 = readl(drvdata.mc_err_base + HB_DDR_ECC_C_ERR_STAT);
    syndrome = (syndrome >> 8) & 0xff;
    err_addr = readl(drvdata.mc_err_base + HB_DDR_ECC_C_ERR_ADDR);
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1,
    err_addr >> PAGE_SHIFT,
    err_addr & ~PAGE_MASK, syndrome,
    0, 0, -1,
    mci.ctl_name, "");
    }
// clear the error, clears the interrupt
    writel(status, drvdata.mc_int_base + HB_DDR_ECC_INT_ACK);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn highbank_mc_err_inject(mci: *mut mem_ctl_info, synd: u8) {
    static void highbank_mc_err_inject(struct mem_ctl_info *mci, u8 synd)
    {
    struct hb_mc_drvdata *pdata = mci.pvt_info;
    u32 reg;
    reg = readl(pdata.mc_err_base + HB_DDR_ECC_OPT);
    reg &= HB_DDR_ECC_OPT_MODE_MASK;
    reg |= (synd << HB_DDR_ECC_OPT_XOR_SHIFT) | HB_DDR_ECC_OPT_FWC;
    writel(reg, pdata.mc_err_base + HB_DDR_ECC_OPT);
    }

    static ssize_t highbank_mc_inject_ctrl(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct mem_ctl_info *mci = to_mci(dev);
    u8 synd;
    if (kstrtou8(buf, 16, &synd))
    return -EINVAL;
    highbank_mc_err_inject(mci, synd);
    return count;
    }
    static DEVICE_ATTR(inject_ctrl, S_IWUSR, core::ptr::null_mut(), highbank_mc_inject_ctrl);
    static struct attribute *highbank_dev_attrs[] = {
    &dev_attr_inject_ctrl.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(highbank_dev);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hb_mc_settings {
    pub err_offset: c_int,
    pub int_offset: c_int,
}

    static struct hb_mc_settings hb_settings = {
    .err_offset = HB_DDR_ECC_ERR_BASE,
    .int_offset = HB_DDR_ECC_INT_BASE,
    };
    static struct hb_mc_settings mw_settings = {
    .err_offset = MW_DDR_ECC_ERR_BASE,
    .int_offset = MW_DDR_ECC_INT_BASE,
    };
    static const struct of_device_id hb_ddr_ctrl_of_match[] = {
    { .compatible = "calxeda,hb-ddr-ctrl",		.data = &hb_settings },
    { .compatible = "calxeda,ecx-2000-ddr-ctrl",	.data = &mw_settings },
    {},
    };
    MODULE_DEVICE_TABLE(of, hb_ddr_ctrl_of_match);
#[no_mangle]
unsafe extern "C" fn highbank_mc_probe(pdev: *mut platform_device) -> c_int {
    static int highbank_mc_probe(struct platform_device *pdev)
    {
    const struct of_device_id *id;
    const struct hb_mc_settings *settings;
    struct edac_mc_layer layers[2];
    struct mem_ctl_info *mci;
    struct hb_mc_drvdata *drvdata;
    struct dimm_info *dimm;
    struct resource *r;
    void __iomem *base;
    u32 control;
    int irq;
    let mut res: c_int = 0;
    id = of_match_device(hb_ddr_ctrl_of_match, &pdev.dev);
    if (!id)
    return -ENODEV;
    layers[0].type = EDAC_MC_LAYER_CHIP_SELECT;
    layers[0].size = 1;
    layers[0].is_virt_csrow = true;
    layers[1].type = EDAC_MC_LAYER_CHANNEL;
    layers[1].size = 1;
    layers[1].is_virt_csrow = false;
    mci = edac_mc_alloc(0, ARRAY_SIZE(layers), layers,
    sizeof(struct hb_mc_drvdata));
    if (!mci)
    return -ENOMEM;
    mci.pdev = &pdev.dev;
    drvdata = mci.pvt_info;
    platform_set_drvdata(pdev, mci);
    if (!devres_open_group(&pdev.dev, core::ptr::null_mut(), GFP_KERNEL)) {
    res = -ENOMEM;
    goto free;
    }
    r = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!r) {
    dev_err(&pdev.dev, "Unable to get mem resource\n");
    res = -ENODEV;
    goto err;
    }
    if (!devm_request_mem_region(&pdev.dev, r.start,
    resource_size(r), dev_name(&pdev.dev))) {
    dev_err(&pdev.dev, "Error while requesting mem region\n");
    res = -EBUSY;
    goto err;
    }
    base = devm_ioremap(&pdev.dev, r.start, resource_size(r));
    if (!base) {
    dev_err(&pdev.dev, "Unable to map regs\n");
    res = -ENOMEM;
    goto err;
    }
    settings = id.data;
    drvdata.mc_err_base = base + settings.err_offset;
    drvdata.mc_int_base = base + settings.int_offset;
    control = readl(drvdata.mc_err_base + HB_DDR_ECC_OPT) & 0x3;
    if (!control || (control == 0x2)) {
    dev_err(&pdev.dev, "No ECC present, or ECC disabled\n");
    res = -ENODEV;
    goto err;
    }
    mci.mtype_cap = MEM_FLAG_DDR3;
    mci.edac_ctl_cap = EDAC_FLAG_NONE | EDAC_FLAG_SECDED;
    mci.edac_cap = EDAC_FLAG_SECDED;
    mci.mod_name = pdev.dev.driver.name;
    mci.ctl_name = id.compatible;
    mci.dev_name = dev_name(&pdev.dev);
    mci.scrub_mode = SCRUB_SW_SRC;
// Only a single 4GB DIMM is supported
    dimm = *mci.dimms;
    dimm.nr_pages = (~0UL >> PAGE_SHIFT) + 1;
    dimm.grain = 8;
    dimm.dtype = DEV_X8;
    dimm.mtype = MEM_DDR3;
    dimm.edac_mode = EDAC_SECDED;
    res = edac_mc_add_mc_with_groups(mci, highbank_dev_groups);
    if (res < 0)
    goto err;
    irq = platform_get_irq(pdev, 0);
    res = devm_request_irq(&pdev.dev, irq, highbank_mc_err_handler,
    0, dev_name(&pdev.dev), mci);
    if (res < 0)
    goto err2;
    devres_close_group(&pdev.dev, core::ptr::null_mut());
    return 0;
    err2:
    edac_mc_del_mc(&pdev.dev);
    err:
    devres_release_group(&pdev.dev, core::ptr::null_mut());
    free:
    edac_mc_free(mci);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn highbank_mc_remove(pdev: *mut platform_device) {
    static void highbank_mc_remove(struct platform_device *pdev)
    {
    struct mem_ctl_info *mci = platform_get_drvdata(pdev);
    edac_mc_del_mc(&pdev.dev);
    edac_mc_free(mci);
    }
    static struct platform_driver highbank_mc_edac_driver = {
    .probe = highbank_mc_probe,
    .remove = highbank_mc_remove,
    .driver = {
    .name = "hb_mc_edac",
    .of_match_table = hb_ddr_ctrl_of_match,
    },
    };
    module_platform_driver(highbank_mc_edac_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Calxeda, Inc.");
    MODULE_DESCRIPTION("EDAC Driver for Calxeda Highbank");
