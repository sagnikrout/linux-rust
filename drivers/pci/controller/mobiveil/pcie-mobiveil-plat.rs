//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/mobiveil/pcie-mobiveil-plat.c
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
// PCIe host controller driver for Mobiveil PCIe Host controller
//
// Copyright (c) 2018 Mobiveil Inc.
// Copyright 2019 NXP
//
// Author: Subrahmanya Lingappa <l.subrahmanya@mobiveil.co.in>
// Hou Zhiqiang <Zhiqiang.Hou@nxp.com>
//

#[no_mangle]
unsafe extern "C" fn mobiveil_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int mobiveil_pcie_probe(struct platform_device *pdev)
    {
    struct mobiveil_pcie *pcie;
    struct pci_host_bridge *bridge;
    struct device *dev = &pdev.dev;
// allocate the PCIe port
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(*pcie));
    if (!bridge)
    return -ENOMEM;
    pcie = pci_host_bridge_priv(bridge);
    pcie.rp.bridge = bridge;
    pcie.pdev = pdev;
    return mobiveil_pcie_host_probe(pcie);
    }
    static const struct of_device_id mobiveil_pcie_of_match[] = {
    {.compatible = "mbvl,gpex40-pcie",},
    {},
    };
    MODULE_DEVICE_TABLE(of, mobiveil_pcie_of_match);
    static struct platform_driver mobiveil_pcie_driver = {
    .probe = mobiveil_pcie_probe,
    .driver = {
    .name = "mobiveil-pcie",
    .of_match_table = mobiveil_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(mobiveil_pcie_driver);
    MODULE_DESCRIPTION("Mobiveil PCIe host controller driver");
    MODULE_AUTHOR("Subrahmanya Lingappa <l.subrahmanya@mobiveil.co.in>");
