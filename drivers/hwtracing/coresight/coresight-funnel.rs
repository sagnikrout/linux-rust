//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-funnel.c
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
// Description: CoreSight Funnel driver
//

pub const FUNNEL_FUNCTL: c_uint = 0x000;
pub const FUNNEL_PRICTL: c_uint = 0x004;
pub const FUNNEL_HOLDTIME_MASK: c_uint = 0xf00;
pub const FUNNEL_HOLDTIME_SHFT: c_uint = 0x8;

pub const FUNNEL_ENSx_MASK: c_uint = 0xff;
//
// struct funnel_drvdata - specifics associated to a funnel component
// @base:	memory mapped base address for this component.
// @atclk:	optional clock for the core parts of the funnel.
// @pclk:	APB clock if present, otherwise NULL
// @csdev:	component vitals needed by the framework.
// @priority:	port selection order.
// @spinlock:	serialize enable/disable operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct funnel_drvdata {
    pub base: *mut void __iomem,
    pub atclk: *mut clk,
    pub pclk: *mut clk,
    pub csdev: *mut coresight_device,
    pub priority: c_ulong,
    pub spinlock: raw_spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn dynamic_funnel_enable_hw(drvdata: *mut funnel_drvdata, port: c_int) -> c_int {
    static int dynamic_funnel_enable_hw(struct funnel_drvdata *drvdata, int port)
    {
    u32 functl;
    let mut rc: c_int = 0;
    struct coresight_device *csdev = drvdata.csdev;
    CS_UNLOCK(drvdata.base);
    functl = readl_relaxed(drvdata.base + FUNNEL_FUNCTL);
// Claim the device only when we enable the first slave
    if (!(functl & FUNNEL_ENSx_MASK)) {
    rc = coresight_claim_device_unlocked(csdev);
    if (rc)
    goto done;
    }
    functl &= ~FUNNEL_HOLDTIME_MASK;
    functl |= FUNNEL_HOLDTIME;
    functl |= (1 << port);
    writel_relaxed(functl, drvdata.base + FUNNEL_FUNCTL);
    writel_relaxed(drvdata.priority, drvdata.base + FUNNEL_PRICTL);
    done:
    CS_LOCK(drvdata.base);
    return rc;
    }
    static int funnel_enable(struct coresight_device *csdev,
    struct coresight_connection *in,
    struct coresight_connection *out)
    {
    let mut rc: c_int = 0;
    struct funnel_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    unsigned long flags;
    let mut first_enable: bool = false;
    raw_spin_lock_irqsave(&drvdata.spinlock, flags);
    if (in.dest_refcnt == 0) {
    if (drvdata.base)
    rc = dynamic_funnel_enable_hw(drvdata, in.dest_port);
    if (!rc)
    first_enable = true;
    }
    if (!rc)
    in.dest_refcnt++;
    raw_spin_unlock_irqrestore(&drvdata.spinlock, flags);
    if (first_enable)
    dev_dbg(&csdev.dev, "FUNNEL inport %d enabled\n",
    in.dest_port);
    return rc;
    }
    static void dynamic_funnel_disable_hw(struct funnel_drvdata *drvdata,
    int inport)
    {
    u32 functl;
    struct coresight_device *csdev = drvdata.csdev;
    CS_UNLOCK(drvdata.base);
    functl = readl_relaxed(drvdata.base + FUNNEL_FUNCTL);
    functl &= ~(1 << inport);
    writel_relaxed(functl, drvdata.base + FUNNEL_FUNCTL);
// Disclaim the device if none of the slaves are now active
    if (!(functl & FUNNEL_ENSx_MASK))
    coresight_disclaim_device_unlocked(csdev);
    CS_LOCK(drvdata.base);
    }
    static void funnel_disable(struct coresight_device *csdev,
    struct coresight_connection *in,
    struct coresight_connection *out)
    {
    struct funnel_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    unsigned long flags;
    let mut last_disable: bool = false;
    raw_spin_lock_irqsave(&drvdata.spinlock, flags);
    if (--in.dest_refcnt == 0) {
    if (drvdata.base)
    dynamic_funnel_disable_hw(drvdata, in.dest_port);
    last_disable = true;
    }
    raw_spin_unlock_irqrestore(&drvdata.spinlock, flags);
    if (last_disable)
    dev_dbg(&csdev.dev, "FUNNEL inport %d disabled\n",
    in.dest_port);
    }
    static const struct coresight_ops_link funnel_link_ops = {
    .enable		= funnel_enable,
    .disable	= funnel_disable,
    };
    static const struct coresight_ops funnel_cs_ops = {
    .link_ops	= &funnel_link_ops,
    };
    static ssize_t priority_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct funnel_drvdata *drvdata = dev_get_drvdata(dev.parent);
    let mut val: c_ulong = drvdata.priority;
    return sprintf(buf, "%#lx\n", val);
    }
    static ssize_t priority_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    int ret;
    unsigned long val;
    struct funnel_drvdata *drvdata = dev_get_drvdata(dev.parent);
    ret = kstrtoul(buf, 16, &val);
    if (ret)
    return ret;
    drvdata.priority = val;
    return size;
    }
    static DEVICE_ATTR_RW(priority);
#[no_mangle]
unsafe extern "C" fn get_funnel_ctrl_hw(drvdata: *mut funnel_drvdata) -> u32 {
    static u32 get_funnel_ctrl_hw(struct funnel_drvdata *drvdata)
    {
    u32 functl;
    CS_UNLOCK(drvdata.base);
    functl = readl_relaxed(drvdata.base + FUNNEL_FUNCTL);
    CS_LOCK(drvdata.base);
    return functl;
    }
    static ssize_t funnel_ctrl_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    u32 val;
    struct funnel_drvdata *drvdata = dev_get_drvdata(dev.parent);
    pm_runtime_get_sync(dev.parent);
    val = get_funnel_ctrl_hw(drvdata);
    pm_runtime_put(dev.parent);
    return sprintf(buf, "%#x\n", val);
    }
    static DEVICE_ATTR_RO(funnel_ctrl);
    static struct attribute *coresight_funnel_attrs[] = {
    &dev_attr_funnel_ctrl.attr,
    &dev_attr_priority.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(coresight_funnel);
#[no_mangle]
unsafe extern "C" fn funnel_probe(dev: *mut device, res: *mut resource) -> c_int {
    static int funnel_probe(struct device *dev, struct resource *res)
    {
    void __iomem *base;
    struct coresight_platform_data *pdata = core::ptr::null_mut();
    struct funnel_drvdata *drvdata;
    let mut desc: coresight_desc = { 0 };
    int ret;
    if (is_of_node(dev_fwnode(dev)) &&
    of_device_is_compatible(dev.of_node, "arm,coresight-funnel"))
    dev_warn_once(dev, "Uses OBSOLETE CoreSight funnel binding\n");
    desc.name = coresight_alloc_device_name("funnel", dev);
    if (!desc.name)
    return -ENOMEM;
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    ret = coresight_get_enable_clocks(dev, &drvdata.pclk, &drvdata.atclk);
    if (ret)
    return ret;
//
// Map the device base for dynamic-funnel, which has been
// validated by AMBA core.
//
    if (res) {
    base = devm_ioremap_resource(dev, res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    drvdata.base = base;
    desc.groups = coresight_funnel_groups;
    desc.access = CSDEV_ACCESS_IOMEM(base);
    coresight_clear_self_claim_tag(&desc.access);
    }
    dev_set_drvdata(dev, drvdata);
    pdata = coresight_get_platform_data(dev);
    if (IS_ERR(pdata))
    return PTR_ERR(pdata);
    dev.platform_data = pdata;
    raw_spin_lock_init(&drvdata.spinlock);
    desc.type = CORESIGHT_DEV_TYPE_LINK;
    desc.subtype.link_subtype = CORESIGHT_DEV_SUBTYPE_LINK_MERG;
    desc.ops = &funnel_cs_ops;
    desc.pdata = pdata;
    desc.dev = dev;
    drvdata.csdev = coresight_register(&desc);
    if (IS_ERR(drvdata.csdev))
    return PTR_ERR(drvdata.csdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn funnel_remove(dev: *mut device) -> c_int {
    static int funnel_remove(struct device *dev)
    {
    struct funnel_drvdata *drvdata = dev_get_drvdata(dev);
    coresight_unregister(drvdata.csdev);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn funnel_runtime_suspend(dev: *mut device) -> c_int {
    static int funnel_runtime_suspend(struct device *dev)
    {
    struct funnel_drvdata *drvdata = dev_get_drvdata(dev);
    clk_disable_unprepare(drvdata.atclk);
    clk_disable_unprepare(drvdata.pclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn funnel_runtime_resume(dev: *mut device) -> c_int {
    static int funnel_runtime_resume(struct device *dev)
    {
    struct funnel_drvdata *drvdata = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(drvdata.pclk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(drvdata.atclk);
    if (ret)
    clk_disable_unprepare(drvdata.pclk);
    return ret;
    }

    static const struct dev_pm_ops funnel_dev_pm_ops = {
    SET_RUNTIME_PM_OPS(funnel_runtime_suspend, funnel_runtime_resume, core::ptr::null_mut())
    };
#[no_mangle]
unsafe extern "C" fn funnel_platform_probe(pdev: *mut platform_device) -> c_int {
    static int funnel_platform_probe(struct platform_device *pdev)
    {
    struct resource *res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    int ret;
    pm_runtime_get_noresume(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ret = funnel_probe(&pdev.dev, res);
    pm_runtime_put(&pdev.dev);
    if (ret)
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn funnel_platform_remove(pdev: *mut platform_device) {
    static void funnel_platform_remove(struct platform_device *pdev)
    {
    struct funnel_drvdata *drvdata = dev_get_drvdata(&pdev.dev);
    if (WARN_ON(!drvdata))
    return;
    funnel_remove(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id funnel_match[] = {
    {.compatible = "arm,coresight-static-funnel"},
    {}
    };
    MODULE_DEVICE_TABLE(of, funnel_match);

    static const struct acpi_device_id funnel_acpi_ids[] = {
    {"ARMHC9FE", 0, 0, 0}, /* ARM Coresight Static Funnel */
    {"ARMHC9FF", 0, 0, 0}, /* ARM CoreSight Dynamic Funnel */
    {},
    };
    MODULE_DEVICE_TABLE(acpi, funnel_acpi_ids);

    static struct platform_driver funnel_driver = {
    .probe		= funnel_platform_probe,
    .remove		= funnel_platform_remove,
    .driver		= {
    .name   = "coresight-funnel",
// THIS_MODULE is taken care of by platform_driver_register()
    .of_match_table = funnel_match,
    .acpi_match_table = ACPI_PTR(funnel_acpi_ids),
    .pm	= &funnel_dev_pm_ops,
    .suppress_bind_attrs = true,
    },
    };
    static int dynamic_funnel_probe(struct amba_device *adev,
    const struct amba_id *id)
    {
    int ret;
    ret = funnel_probe(&adev.dev, &adev.res);
    if (!ret)
    pm_runtime_put(&adev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dynamic_funnel_remove(adev: *mut amba_device) {
    static void dynamic_funnel_remove(struct amba_device *adev)
    {
    funnel_remove(&adev.dev);
    }
    static const struct amba_id dynamic_funnel_ids[] = {
    {
    .id     = 0x000bb908,
    .mask   = 0x000fffff,
    },
    {
// Coresight SoC-600
    .id     = 0x000bb9eb,
    .mask   = 0x000fffff,
    },
    { 0, 0, core::ptr::null_mut() },
    };
    MODULE_DEVICE_TABLE(amba, dynamic_funnel_ids);
    static struct amba_driver dynamic_funnel_driver = {
    .drv = {
    .name	= "coresight-dynamic-funnel",
    .pm	= &funnel_dev_pm_ops,
    .suppress_bind_attrs = true,
    },
    .probe		= dynamic_funnel_probe,
    .remove		= dynamic_funnel_remove,
    .id_table	= dynamic_funnel_ids,
    };
#[no_mangle]
unsafe extern "C" fn funnel_init() -> int __init {
    static int __init funnel_init(void)
    {
    return coresight_init_driver("funnel", &dynamic_funnel_driver, &funnel_driver);
    }
#[no_mangle]
unsafe extern "C" fn funnel_exit() -> void __exit {
    static void __exit funnel_exit(void)
    {
    coresight_remove_driver(&dynamic_funnel_driver, &funnel_driver);
    }
    module_init(funnel_init);
    module_exit(funnel_exit);
    MODULE_AUTHOR("Pratik Patel <pratikp@codeaurora.org>");
    MODULE_AUTHOR("Mathieu Poirier <mathieu.poirier@linaro.org>");
    MODULE_DESCRIPTION("Arm CoreSight Funnel Driver");
    MODULE_LICENSE("GPL v2");
