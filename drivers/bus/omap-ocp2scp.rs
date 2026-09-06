//! Automatically rewritten from C to Rust
//! Source: drivers/bus/omap-ocp2scp.c
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
// omap-ocp2scp.c - transform ocp interface protocol to scp protocol
//
// Copyright (C) 2012 Texas Instruments Incorporated - http://www.ti.com
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

pub const OCP2SCP_TIMING: c_uint = 0x18;
pub const SYNC2_MASK: c_uint = 0xf;
#[no_mangle]
unsafe extern "C" fn omap_ocp2scp_probe(pdev: *mut platform_device) -> c_int {
    static int omap_ocp2scp_probe(struct platform_device *pdev)
    {
    int ret;
    u32 reg;
    void __iomem *regs;
    struct resource *res;
    struct device_node *np = pdev.dev.of_node;
    if (np) {
    ret = of_platform_populate(np, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    if (ret) {
    dev_err(&pdev.dev,
    "failed to add resources for ocp2scp child\n");
    goto err0;
    }
    }
    pm_runtime_enable(&pdev.dev);
//
// As per AM572x TRM: http://www.ti.com/lit/ug/spruhz6/spruhz6.pdf
// under section 26.3.2.2, table 26-26 OCP2SCP TIMING Caution;
// As per OMAP4430 TRM: http://www.ti.com/lit/ug/swpu231ap/swpu231ap.pdf
// under section 23.12.6.2.2 , Table 23-1213 OCP2SCP TIMING Caution;
// As per OMAP4460 TRM: http://www.ti.com/lit/ug/swpu235ab/swpu235ab.pdf
// under section 23.12.6.2.2, Table 23-1213 OCP2SCP TIMING Caution;
// As per OMAP543x TRM http://www.ti.com/lit/pdf/swpu249
// under section 27.3.2.2, Table 27-27 OCP2SCP TIMING Caution;
//
// Read path of OCP2SCP is not working properly due to low reset value
// of SYNC2 parameter in OCP2SCP. Suggested reset value is 0x6 or more.
//
    if (!of_device_is_compatible(np, "ti,am437x-ocp2scp")) {
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    regs = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(regs)) {
    ret = PTR_ERR(regs);
    goto err1;
    }
    pm_runtime_get_sync(&pdev.dev);
    reg = readl_relaxed(regs + OCP2SCP_TIMING);
    reg &= ~(SYNC2_MASK);
    reg |= 0x6;
    writel_relaxed(reg, regs + OCP2SCP_TIMING);
    pm_runtime_put_sync(&pdev.dev);
    }
    return 0;
    err1:
    pm_runtime_disable(&pdev.dev);
    err0:
    of_platform_depopulate(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_ocp2scp_remove(pdev: *mut platform_device) {
    static void omap_ocp2scp_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    of_platform_depopulate(&pdev.dev);
    }

    static const struct of_device_id omap_ocp2scp_id_table[] = {
    { .compatible = "ti,omap-ocp2scp" },
    { .compatible = "ti,am437x-ocp2scp" },
    {}
    };
    MODULE_DEVICE_TABLE(of, omap_ocp2scp_id_table);

    static struct platform_driver omap_ocp2scp_driver = {
    .probe		= omap_ocp2scp_probe,
    .remove		= omap_ocp2scp_remove,
    .driver		= {
    .name	= "omap-ocp2scp",
    .of_match_table = of_match_ptr(omap_ocp2scp_id_table),
    },
    };
    module_platform_driver(omap_ocp2scp_driver);
    MODULE_ALIAS("platform:omap-ocp2scp");
    MODULE_AUTHOR("Texas Instruments Inc.");
    MODULE_DESCRIPTION("OMAP OCP2SCP driver");
    MODULE_LICENSE("GPL v2");
