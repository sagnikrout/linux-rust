//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/ctucanfd/ctucanfd_platform.c
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
// CTU CAN FD IP Core
//
// Copyright (C) 2015-2018 Ondrej Ille <ondrej.ille@gmail.com> FEE CTU
// Copyright (C) 2018-2021 Ondrej Ille <ondrej.ille@gmail.com> self-funded
// Copyright (C) 2018-2019 Martin Jerabek <martin.jerabek01@gmail.com> FEE CTU
// Copyright (C) 2018-2022 Pavel Pisa <pisa@cmp.felk.cvut.cz> FEE CTU/self-funded
//
// Project advisors:
// Jiri Novak <jnovak@fel.cvut.cz>
// Pavel Pisa <pisa@cmp.felk.cvut.cz>
//
// Department of Measurement         (http://meas.fel.cvut.cz/)
// Faculty of Electrical Engineering (http://www.fel.cvut.cz)
// Czech Technical University        (http://www.cvut.cz/)
//

    static void ctucan_platform_set_drvdata(struct device *dev,
    struct net_device *ndev)
    {
    struct platform_device *pdev = container_of(dev, struct platform_device,
    dev);
    platform_set_drvdata(pdev, ndev);
    }
//
// ctucan_platform_probe - Platform registration call
// @pdev:	Handle to the platform device structure
//
// This function does all the memory allocation and registration for the CAN
// device.
//
// Return: 0 on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn ctucan_platform_probe(pdev: *mut platform_device) -> c_int {
    static int ctucan_platform_probe(struct platform_device *pdev)
    {
    struct device	*dev = &pdev.dev;
    void __iomem *addr;
    int ret;
    unsigned int ntxbufs;
    int irq;
// Get the virtual base address for the device
    addr = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(addr)) {
    ret = PTR_ERR(addr);
    goto err;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    ret = irq;
    goto err;
    }
// Number of tx bufs might be change in HW for future. If so,
// it will be passed as property via device tree
//
    ntxbufs = 4;
    ret = ctucan_probe_common(dev, addr, irq, ntxbufs, 0,
    1, ctucan_platform_set_drvdata);
    if (ret < 0)
    platform_set_drvdata(pdev, core::ptr::null_mut());
    err:
    return ret;
    }
//
// ctucan_platform_remove - Unregister the device after releasing the resources
// @pdev:	Handle to the platform device structure
//
// This function frees all the resources allocated to the device.
// Return: 0 always
//
#[no_mangle]
unsafe extern "C" fn ctucan_platform_remove(pdev: *mut platform_device) {
    static void ctucan_platform_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct ctucan_priv *priv = netdev_priv(ndev);
    netdev_dbg(ndev, "ctucan_remove");
    unregister_candev(ndev);
    pm_runtime_disable(&pdev.dev);
    netif_napi_del(&priv.napi);
    free_candev(ndev);
    }
    static SIMPLE_DEV_PM_OPS(ctucan_platform_pm_ops, ctucan_suspend, ctucan_resume);
// Match table for OF platform binding
    static const struct of_device_id ctucan_of_match[] = {
    { .compatible = "ctu,ctucanfd-2", },
    { .compatible = "ctu,ctucanfd", },
    { /* end of list */ },
    };
    MODULE_DEVICE_TABLE(of, ctucan_of_match);
    static struct platform_driver ctucanfd_driver = {
    .probe	= ctucan_platform_probe,
    .remove = ctucan_platform_remove,
    .driver	= {
    .name = DRV_NAME,
    .pm = &ctucan_platform_pm_ops,
    .of_match_table	= ctucan_of_match,
    },
    };
    module_platform_driver(ctucanfd_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Martin Jerabek");
    MODULE_DESCRIPTION("CTU CAN FD for platform");
