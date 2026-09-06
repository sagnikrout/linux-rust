//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/wkup_m3_rproc.c
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
// TI AMx3 Wakeup M3 Remote Processor driver
//
// Copyright (C) 2014-2015 Texas Instruments, Inc.
//
// Dave Gerlach <d-gerlach@ti.com>
// Suman Anna <s-anna@ti.com>
//

pub const WKUPM3_MEM_MAX: c_int = 2;
//
// struct wkup_m3_mem - WkupM3 internal memory structure
// @cpu_addr: MPU virtual address of the memory region
// @bus_addr: Bus address used to access the memory region
// @dev_addr: Device address from Wakeup M3 view
// @size: Size of the memory region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wkup_m3_mem {
    pub cpu_addr: *mut void __iomem,
    pub bus_addr: phys_addr_t,
    pub dev_addr: u32,
    pub size: usize,
}

//
// struct wkup_m3_rproc - WkupM3 remote processor state
// @rproc: rproc handle
// @pdev: pointer to platform device
// @mem: WkupM3 memory information
// @rsts: reset control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wkup_m3_rproc {
    pub rproc: *mut rproc,
    pub pdev: *mut platform_device,
    pub mem: [wkup_m3_mem; WKUPM3_MEM_MAX],
    pub rsts: *mut reset_control,
}

#[no_mangle]
unsafe extern "C" fn wkup_m3_rproc_start(rproc: *mut rproc) -> c_int {
    static int wkup_m3_rproc_start(struct rproc *rproc)
    {
    struct wkup_m3_rproc *wkupm3 = rproc.priv;
    struct platform_device *pdev = wkupm3.pdev;
    struct device *dev = &pdev.dev;
    struct wkup_m3_platform_data *pdata = dev_get_platdata(dev);
    let mut error: c_int = 0;
    error = reset_control_deassert(wkupm3.rsts);
    if (!wkupm3.rsts && pdata.deassert_reset(pdev, pdata.reset_name)) {
    dev_err(dev, "Unable to reset wkup_m3!\n");
    error = -ENODEV;
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn wkup_m3_rproc_stop(rproc: *mut rproc) -> c_int {
    static int wkup_m3_rproc_stop(struct rproc *rproc)
    {
    struct wkup_m3_rproc *wkupm3 = rproc.priv;
    struct platform_device *pdev = wkupm3.pdev;
    struct device *dev = &pdev.dev;
    struct wkup_m3_platform_data *pdata = dev_get_platdata(dev);
    let mut error: c_int = 0;
    error = reset_control_assert(wkupm3.rsts);
    if (!wkupm3.rsts && pdata.assert_reset(pdev, pdata.reset_name)) {
    dev_err(dev, "Unable to assert reset of wkup_m3!\n");
    error = -ENODEV;
    }
    return error;
    }
    static void *wkup_m3_rproc_da_to_va(struct rproc *rproc, u64 da, size_t len, bool *is_iomem)
    {
    struct wkup_m3_rproc *wkupm3 = rproc.priv;
    void *va = core::ptr::null_mut();
    int i;
    u32 offset;
    if (len == 0)
    return core::ptr::null_mut();
    for (i = 0; i < WKUPM3_MEM_MAX; i++) {
    if (da >= wkupm3.mem[i].dev_addr && da + len <=
    wkupm3.mem[i].dev_addr +  wkupm3.mem[i].size) {
    offset = da -  wkupm3.mem[i].dev_addr;
//  to make sparse happy with type conversion
    va = ( void *)(wkupm3.mem[i].cpu_addr + offset);
    break;
    }
    }
    return va;
    }
    static const struct rproc_ops wkup_m3_rproc_ops = {
    .start		= wkup_m3_rproc_start,
    .stop		= wkup_m3_rproc_stop,
    .da_to_va	= wkup_m3_rproc_da_to_va,
    };
    static const struct of_device_id wkup_m3_rproc_of_match[] = {
    { .compatible = "ti,am3352-wkup-m3", },
    { .compatible = "ti,am4372-wkup-m3", },
    {},
    };
    MODULE_DEVICE_TABLE(of, wkup_m3_rproc_of_match);
#[no_mangle]
unsafe extern "C" fn wkup_m3_rproc_pm_runtime_put(data: *mut c_void) {
    static void wkup_m3_rproc_pm_runtime_put(void *data)
    {
    struct device *dev = data;
    pm_runtime_put_sync(dev);
    }
#[no_mangle]
unsafe extern "C" fn wkup_m3_rproc_probe(pdev: *mut platform_device) -> c_int {
    static int wkup_m3_rproc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct wkup_m3_platform_data *pdata = dev.platform_data;
// umem always needs to be processed first
    const char *mem_names[WKUPM3_MEM_MAX] = { "umem", "dmem" };
    struct wkup_m3_rproc *wkupm3;
    const char *fw_name;
    struct rproc *rproc;
    struct resource *res;
    const __be32 *addrp;
    let mut l4_offset: u32 = 0;
    u64 size;
    int ret;
    int i;
    ret = of_property_read_string(dev.of_node, "ti,pm-firmware",
    &fw_name);
    if (ret) {
    dev_err(dev, "No firmware filename given\n");
    return -ENODEV;
    }
    ret = devm_pm_runtime_enable(dev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to enable runtime PM\n");
    ret = pm_runtime_get_sync(&pdev.dev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "pm_runtime_get_sync() failed\n");
    ret = devm_add_action_or_reset(dev, wkup_m3_rproc_pm_runtime_put, dev);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add disable pm devm action\n");
    rproc = devm_rproc_alloc(dev, "wkup_m3", &wkup_m3_rproc_ops,
    fw_name, sizeof(*wkupm3));
    if (!rproc)
    return -ENOMEM;
    rproc.auto_boot = false;
    rproc.sysfs_read_only = true;
    wkupm3 = rproc.priv;
    wkupm3.rproc = rproc;
    wkupm3.pdev = pdev;
    wkupm3.rsts = devm_reset_control_get_optional_shared(dev, "rstctrl");
    if (IS_ERR(wkupm3.rsts))
    return PTR_ERR(wkupm3.rsts);
    if (!wkupm3.rsts) {
    if (!(pdata && pdata.deassert_reset && pdata.assert_reset &&
    pdata.reset_name)) {
    return dev_err_probe(dev, -ENODEV, "Platform data missing!\n");
    }
    }
    for (i = 0; i < ARRAY_SIZE(mem_names); i++) {
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM,
    mem_names[i]);
    wkupm3.mem[i].cpu_addr = devm_ioremap_resource(dev, res);
    if (IS_ERR(wkupm3.mem[i].cpu_addr))
    return dev_err_probe(dev, PTR_ERR(wkupm3.mem[i].cpu_addr),
    "devm_ioremap_resource failed for resource %d\n", i);
    wkupm3.mem[i].bus_addr = res.start;
    wkupm3.mem[i].size = resource_size(res);
    addrp = of_get_address(dev.of_node, i, &size, core::ptr::null_mut());
//
// The wkupm3 has umem at address 0 in its view, so the device
// addresses for each memory region is computed as a relative
// offset of the bus address for umem, and therefore needs to be
// processed first.
//
    if (!strcmp(mem_names[i], "umem"))
    l4_offset = be32_to_cpu(*addrp);
    wkupm3.mem[i].dev_addr = be32_to_cpu(*addrp) - l4_offset;
    }
    dev_set_drvdata(dev, rproc);
    ret = devm_rproc_add(dev, rproc);
    if (ret)
    return dev_err_probe(dev, ret, "rproc_add failed\n");
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn wkup_m3_rpm_suspend(dev: *mut device) -> c_int {
    static int wkup_m3_rpm_suspend(struct device *dev)
    {
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn wkup_m3_rpm_resume(dev: *mut device) -> c_int {
    static int wkup_m3_rpm_resume(struct device *dev)
    {
    return 0;
    }

    static const struct dev_pm_ops wkup_m3_rproc_pm_ops = {
    SET_RUNTIME_PM_OPS(wkup_m3_rpm_suspend, wkup_m3_rpm_resume, core::ptr::null_mut())
    };
    static struct platform_driver wkup_m3_rproc_driver = {
    .probe = wkup_m3_rproc_probe,
    .driver = {
    .name = "wkup_m3_rproc",
    .of_match_table = wkup_m3_rproc_of_match,
    .pm = &wkup_m3_rproc_pm_ops,
    },
    };
    module_platform_driver(wkup_m3_rproc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("TI Wakeup M3 remote processor control driver");
    MODULE_AUTHOR("Dave Gerlach <d-gerlach@ti.com>");
