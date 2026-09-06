//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-tpiu.c
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
// Copyright (c) 2011-2012, The Linux Foundation. All rights reserved.
//
// Description: CoreSight Trace Port Interface Unit driver
//

pub const TPIU_SUPP_PORTSZ: c_uint = 0x000;
pub const TPIU_CURR_PORTSZ: c_uint = 0x004;
pub const TPIU_SUPP_TRIGMODES: c_uint = 0x100;
pub const TPIU_TRIG_CNTRVAL: c_uint = 0x104;
pub const TPIU_TRIG_MULT: c_uint = 0x108;
pub const TPIU_SUPP_TESTPATM: c_uint = 0x200;
pub const TPIU_CURR_TESTPATM: c_uint = 0x204;
pub const TPIU_TEST_PATREPCNTR: c_uint = 0x208;
pub const TPIU_FFSR: c_uint = 0x300;
pub const TPIU_FFCR: c_uint = 0x304;
pub const TPIU_FSYNC_CNTR: c_uint = 0x308;
pub const TPIU_EXTCTL_INPORT: c_uint = 0x400;
pub const TPIU_EXTCTL_OUTPORT: c_uint = 0x404;
pub const TPIU_ITTRFLINACK: c_uint = 0xee4;
pub const TPIU_ITTRFLIN: c_uint = 0xee8;
pub const TPIU_ITATBDATA0: c_uint = 0xeec;
pub const TPIU_ITATBCTR2: c_uint = 0xef0;
pub const TPIU_ITATBCTR1: c_uint = 0xef4;
pub const TPIU_ITATBCTR0: c_uint = 0xef8;
// register definition
// FFSR - 0x300
pub const FFSR_FT_STOPPED_BIT: c_int = 1;
// FFCR - 0x304
pub const FFCR_FON_MAN_BIT: c_int = 6;

//
// @base:	memory mapped base address for this component.
// @atclk:	optional clock for the core parts of the TPIU.
// @pclk:	APB clock if present, otherwise NULL
// @csdev:	component vitals needed by the framework.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpiu_drvdata {
    pub base: *mut void __iomem,
    pub atclk: *mut clk,
    pub pclk: *mut clk,
    pub csdev: *mut coresight_device,
    pub spinlock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn tpiu_enable_hw(csa: *mut csdev_access) {
    static void tpiu_enable_hw(struct csdev_access *csa)
    {
    CS_UNLOCK(csa.base);
// TODO: fill this up
    CS_LOCK(csa.base);
    }
    static int tpiu_enable(struct coresight_device *csdev, enum cs_mode mode,
    struct coresight_path *path)
    {
    struct tpiu_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    guard(spinlock)(&drvdata.spinlock);
    tpiu_enable_hw(&csdev.access);
    csdev.refcnt++;
    dev_dbg(&csdev.dev, "TPIU enabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpiu_disable_hw(csa: *mut csdev_access) {
    static void tpiu_disable_hw(struct csdev_access *csa)
    {
    CS_UNLOCK(csa.base);
// Clear formatter and stop on flush
    csdev_access_relaxed_write32(csa, FFCR_STOP_FI, TPIU_FFCR);
// Generate manual flush
    csdev_access_relaxed_write32(csa, FFCR_STOP_FI | FFCR_FON_MAN, TPIU_FFCR);
// Wait for flush to complete
    coresight_timeout(csa, TPIU_FFCR, FFCR_FON_MAN_BIT, 0);
// Wait for formatter to stop
    coresight_timeout(csa, TPIU_FFSR, FFSR_FT_STOPPED_BIT, 1);
    CS_LOCK(csa.base);
    }
#[no_mangle]
unsafe extern "C" fn tpiu_disable(csdev: *mut coresight_device) -> c_int {
    static int tpiu_disable(struct coresight_device *csdev)
    {
    struct tpiu_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    guard(spinlock)(&drvdata.spinlock);
    csdev.refcnt--;
    if (csdev.refcnt)
    return -EBUSY;
    tpiu_disable_hw(&csdev.access);
    dev_dbg(&csdev.dev, "TPIU disabled\n");
    return 0;
    }
    static const struct coresight_ops_sink tpiu_sink_ops = {
    .enable		= tpiu_enable,
    .disable	= tpiu_disable,
    };
    static const struct coresight_ops tpiu_cs_ops = {
    .sink_ops	= &tpiu_sink_ops,
    };
#[no_mangle]
unsafe extern "C" fn __tpiu_probe(dev: *mut device, res: *mut resource) -> c_int {
    static int __tpiu_probe(struct device *dev, struct resource *res)
    {
    void __iomem *base;
    struct coresight_platform_data *pdata = core::ptr::null_mut();
    struct tpiu_drvdata *drvdata;
    let mut desc: coresight_desc = { 0 };
    int ret;
    desc.name = coresight_alloc_device_name("tpiu", dev);
    if (!desc.name)
    return -ENOMEM;
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    spin_lock_init(&drvdata.spinlock);
    ret = coresight_get_enable_clocks(dev, &drvdata.pclk, &drvdata.atclk);
    if (ret)
    return ret;
    dev_set_drvdata(dev, drvdata);
// Validity for the resource is already checked by the AMBA core
    base = devm_ioremap_resource(dev, res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    drvdata.base = base;
    desc.access = CSDEV_ACCESS_IOMEM(base);
// Disable tpiu to support older devices
    tpiu_disable_hw(&desc.access);
    pdata = coresight_get_platform_data(dev);
    if (IS_ERR(pdata))
    return PTR_ERR(pdata);
    dev.platform_data = pdata;
    desc.type = CORESIGHT_DEV_TYPE_SINK;
    desc.subtype.sink_subtype = CORESIGHT_DEV_SUBTYPE_SINK_PORT;
    desc.ops = &tpiu_cs_ops;
    desc.pdata = pdata;
    desc.dev = dev;
    drvdata.csdev = coresight_register(&desc);
    if (!IS_ERR(drvdata.csdev))
    return 0;
    return PTR_ERR(drvdata.csdev);
    }
#[no_mangle]
unsafe extern "C" fn tpiu_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int tpiu_probe(struct amba_device *adev, const struct amba_id *id)
    {
    int ret;
    ret = __tpiu_probe(&adev.dev, &adev.res);
    if (!ret)
    pm_runtime_put(&adev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __tpiu_remove(dev: *mut device) {
    static void __tpiu_remove(struct device *dev)
    {
    struct tpiu_drvdata *drvdata = dev_get_drvdata(dev);
    coresight_unregister(drvdata.csdev);
    }
#[no_mangle]
unsafe extern "C" fn tpiu_remove(adev: *mut amba_device) {
    static void tpiu_remove(struct amba_device *adev)
    {
    __tpiu_remove(&adev.dev);
    }

#[no_mangle]
unsafe extern "C" fn tpiu_runtime_suspend(dev: *mut device) -> c_int {
    static int tpiu_runtime_suspend(struct device *dev)
    {
    struct tpiu_drvdata *drvdata = dev_get_drvdata(dev);
    clk_disable_unprepare(drvdata.atclk);
    clk_disable_unprepare(drvdata.pclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpiu_runtime_resume(dev: *mut device) -> c_int {
    static int tpiu_runtime_resume(struct device *dev)
    {
    struct tpiu_drvdata *drvdata = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(drvdata.pclk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(drvdata.atclk);
    if (ret)
    clk_disable_unprepare(drvdata.pclk);
    return ret;
    }

    static const struct dev_pm_ops tpiu_dev_pm_ops = {
    SET_RUNTIME_PM_OPS(tpiu_runtime_suspend, tpiu_runtime_resume, core::ptr::null_mut())
    };
    static const struct amba_id tpiu_ids[] = {
    {
    .id	= 0x000bb912,
    .mask	= 0x000fffff,
    },
    {
    .id	= 0x0004b912,
    .mask	= 0x0007ffff,
    },
    {
// Coresight SoC-600
    .id	= 0x000bb9e7,
    .mask	= 0x000fffff,
    },
    { 0, 0, core::ptr::null_mut() },
    };
    MODULE_DEVICE_TABLE(amba, tpiu_ids);
    static struct amba_driver tpiu_driver = {
    .drv = {
    .name	= "coresight-tpiu",
    .pm	= &tpiu_dev_pm_ops,
    .suppress_bind_attrs = true,
    },
    .probe		= tpiu_probe,
    .remove         = tpiu_remove,
    .id_table	= tpiu_ids,
    };
#[no_mangle]
unsafe extern "C" fn tpiu_platform_probe(pdev: *mut platform_device) -> c_int {
    static int tpiu_platform_probe(struct platform_device *pdev)
    {
    struct resource *res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    int ret;
    pm_runtime_get_noresume(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ret = __tpiu_probe(&pdev.dev, res);
    pm_runtime_put(&pdev.dev);
    if (ret)
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tpiu_platform_remove(pdev: *mut platform_device) {
    static void tpiu_platform_remove(struct platform_device *pdev)
    {
    struct tpiu_drvdata *drvdata = dev_get_drvdata(&pdev.dev);
    if (WARN_ON(!drvdata))
    return;
    __tpiu_remove(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }

    static const struct acpi_device_id tpiu_acpi_ids[] = {
    {"ARMHC979", 0, 0, 0}, /* ARM CoreSight TPIU */
    {}
    };
    MODULE_DEVICE_TABLE(acpi, tpiu_acpi_ids);

    static struct platform_driver tpiu_platform_driver = {
    .probe	= tpiu_platform_probe,
    .remove = tpiu_platform_remove,
    .driver = {
    .name			= "coresight-tpiu-platform",
    .acpi_match_table	= ACPI_PTR(tpiu_acpi_ids),
    .suppress_bind_attrs	= true,
    .pm			= &tpiu_dev_pm_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn tpiu_init() -> int __init {
    static int __init tpiu_init(void)
    {
    return coresight_init_driver("tpiu", &tpiu_driver, &tpiu_platform_driver);
    }
#[no_mangle]
unsafe extern "C" fn tpiu_exit() -> void __exit {
    static void __exit tpiu_exit(void)
    {
    coresight_remove_driver(&tpiu_driver, &tpiu_platform_driver);
    }
    module_init(tpiu_init);
    module_exit(tpiu_exit);
    MODULE_AUTHOR("Pratik Patel <pratikp@codeaurora.org>");
    MODULE_AUTHOR("Mathieu Poirier <mathieu.poirier@linaro.org>");
    MODULE_DESCRIPTION("Arm CoreSight TPIU (Trace Port Interface Unit) driver");
    MODULE_LICENSE("GPL v2");
