//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/rcpm.c
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
// rcpm.c - Freescale QorIQ RCPM driver
//
// Copyright 2019-2020 NXP
//
// Author: Ran Wang <ran.wang_1@nxp.com>

pub const RCPM_WAKEUP_CELL_MAX_SIZE: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcpm {
    pub wakeup_cells: c_uint,
    pub ippdexpcr_base: *mut void __iomem,
    pub little_endian: bool,
}

pub const SCFG_SPARECR8: c_uint = 0x051c;
#[no_mangle]
unsafe extern "C" fn copy_ippdexpcr1_setting(val: u32) {
    static void copy_ippdexpcr1_setting(u32 val)
    {
    struct device_node *np;
    void __iomem *regs;
    u32 reg_val;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,ls1021a-scfg");
    if (!np)
    return;
    regs = of_iomap(np, 0);
    of_node_put(np);
    if (!regs)
    return;
    reg_val = ioread32be(regs + SCFG_SPARECR8);
    iowrite32be(val | reg_val, regs + SCFG_SPARECR8);
    iounmap(regs);
    }
//
// rcpm_pm_prepare - performs device-level tasks associated with power
// management, such as programming related to the wakeup source control.
// @dev: Device to handle.
//
#[no_mangle]
unsafe extern "C" fn rcpm_pm_prepare(dev: *mut device) -> c_int {
    static int rcpm_pm_prepare(struct device *dev)
    {
    int i, ret, idx;
    void __iomem *base;
    struct wakeup_source	*ws;
    struct rcpm		*rcpm;
    struct device_node	*np = dev.of_node;
    u32 value[RCPM_WAKEUP_CELL_MAX_SIZE + 1];
    u32 setting[RCPM_WAKEUP_CELL_MAX_SIZE] = {0};
    rcpm = dev_get_drvdata(dev);
    if (!rcpm)
    return -EINVAL;
    base = rcpm.ippdexpcr_base;
    idx = wakeup_sources_read_lock();
// Begin with first registered wakeup source
    for_each_wakeup_source(ws) {
// skip object which is not attached to device
    if (!ws.dev || !ws.dev.parent)
    continue;
    ret = device_property_read_u32_array(ws.dev.parent,
    "fsl,rcpm-wakeup", value,
    rcpm.wakeup_cells + 1);
    if (ret)
    continue;
//
// For DT mode, would handle devices with "fsl,rcpm-wakeup"
// pointing to the current RCPM node.
//
// For ACPI mode, currently we assume there is only one
// RCPM controller existing.
//
    if (is_of_node(dev.fwnode))
    if (np.phandle != value[0])
    continue;
// Property "#fsl,rcpm-wakeup-cells" of rcpm node defines the
// number of IPPDEXPCR register cells, and "fsl,rcpm-wakeup"
// of wakeup source IP contains an integer array: <phandle to
// RCPM node, IPPDEXPCR0 setting, IPPDEXPCR1 setting,
// IPPDEXPCR2 setting, etc>.
//
// So we will go thought them to collect setting data.
//
    for (i = 0; i < rcpm.wakeup_cells; i++)
    setting[i] |= value[i + 1];
    }
    wakeup_sources_read_unlock(idx);
// Program all IPPDEXPCRn once
    for (i = 0; i < rcpm.wakeup_cells; i++) {
    let mut tmp: u32 = setting[i];
    void __iomem *address = base + i * 4;
    if (!tmp)
    continue;
// We can only OR related bits
    if (rcpm.little_endian) {
    tmp |= ioread32(address);
    iowrite32(tmp, address);
    } else {
    tmp |= ioread32be(address);
    iowrite32be(tmp, address);
    }
//
// Workaround of errata A-008646 on SoC LS1021A:
// There is a bug of register ippdexpcr1.
// Reading configuration register RCPM_IPPDEXPCR1
// always return zero. So save ippdexpcr1's value
// to register SCFG_SPARECR8.And the value of
// ippdexpcr1 will be read from SCFG_SPARECR8.
//
    if (dev_of_node(dev) && (i == 1))
    if (of_device_is_compatible(np, "fsl,ls1021a-rcpm"))
    copy_ippdexpcr1_setting(tmp);
    }
    return 0;
    }
    static const struct dev_pm_ops rcpm_pm_ops = {
    .prepare =  rcpm_pm_prepare,
    };
#[no_mangle]
unsafe extern "C" fn rcpm_probe(pdev: *mut platform_device) -> c_int {
    static int rcpm_probe(struct platform_device *pdev)
    {
    struct device	*dev = &pdev.dev;
    struct rcpm	*rcpm;
    int ret;
    rcpm = devm_kzalloc(dev, sizeof(*rcpm), GFP_KERNEL);
    if (!rcpm)
    return -ENOMEM;
    rcpm.ippdexpcr_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rcpm.ippdexpcr_base)) {
    ret =  PTR_ERR(rcpm.ippdexpcr_base);
    return ret;
    }
    rcpm.little_endian = device_property_read_bool(
    &pdev.dev, "little-endian");
    ret = device_property_read_u32(&pdev.dev,
    "#fsl,rcpm-wakeup-cells", &rcpm.wakeup_cells);
    if (ret)
    return ret;
    dev_set_drvdata(&pdev.dev, rcpm);
    return 0;
    }
    static const struct of_device_id rcpm_of_match[] = {
    { .compatible = "fsl,qoriq-rcpm-2.1+", },
    {}
    };
    MODULE_DEVICE_TABLE(of, rcpm_of_match);

    static const struct acpi_device_id rcpm_acpi_ids[] = {
    {"NXP0015",},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, rcpm_acpi_ids);

    static struct platform_driver rcpm_driver = {
    .driver = {
    .name = "rcpm",
    .of_match_table = rcpm_of_match,
    .acpi_match_table = ACPI_PTR(rcpm_acpi_ids),
    .pm	= &rcpm_pm_ops,
    },
    .probe = rcpm_probe,
    };
    module_platform_driver(rcpm_driver);
