//! Automatically rewritten from C to Rust
//! Source: drivers/bus/stm32_etzpc.c
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
// Copyright (C) 2023, STMicroelectronics - All Rights Reserved
//

//
// ETZPC registers
//
pub const ETZPC_DECPROT: c_uint = 0x10;
pub const ETZPC_HWCFGR: c_uint = 0x3F0;
//
// HWCFGR register
//

//
// ETZPC miscellaneous
//

pub const ETZPC_PROT_A7NS: c_uint = 0x3;
pub const ETZPC_DECPROT_SHIFT: c_int = 1;
pub const IDS_PER_DECPROT_REGS: c_int = 16;
#[no_mangle]
unsafe extern "C" fn stm32_etzpc_grant_access(ctrl: *mut stm32_firewall_controller, firewall_id: u32) -> c_int {
    static int stm32_etzpc_grant_access(struct stm32_firewall_controller *ctrl, u32 firewall_id)
    {
    u32 offset, reg_offset, sec_val;
    if (firewall_id >= ctrl.max_entries) {
    dev_err(ctrl.dev, "Invalid sys bus ID %u", firewall_id);
    return -EINVAL;
    }
// Check access configuration, 16 peripherals per register
    reg_offset = ETZPC_DECPROT + 0x4 * (firewall_id / IDS_PER_DECPROT_REGS);
    offset = (firewall_id % IDS_PER_DECPROT_REGS) << ETZPC_DECPROT_SHIFT;
// Verify peripheral is non-secure and attributed to cortex A7
    sec_val = (readl(ctrl.mmio + reg_offset) >> offset) & ETZPC_PROT_MASK;
    if (sec_val != ETZPC_PROT_A7NS) {
    dev_dbg(ctrl.dev, "Invalid bus configuration: reg_offset %#x, value %d\n",
    reg_offset, sec_val);
    return -EACCES;
    }
    return 0;
    }
    static void stm32_etzpc_release_access(struct stm32_firewall_controller *ctrl __maybe_unused,
    u32 firewall_id __maybe_unused)
    {
    }
#[no_mangle]
unsafe extern "C" fn stm32_etzpc_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_etzpc_probe(struct platform_device *pdev)
    {
    struct stm32_firewall_controller *etzpc_controller;
    struct device_node *np = pdev.dev.of_node;
    u32 nb_per, nb_master;
    struct resource *res;
    void __iomem *mmio;
    int rc;
    etzpc_controller = devm_kzalloc(&pdev.dev, sizeof(*etzpc_controller), GFP_KERNEL);
    if (!etzpc_controller)
    return -ENOMEM;
    mmio = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(mmio))
    return PTR_ERR(mmio);
    etzpc_controller.dev = &pdev.dev;
    etzpc_controller.mmio = mmio;
    etzpc_controller.name = dev_driver_string(etzpc_controller.dev);
    etzpc_controller.type = STM32_PERIPHERAL_FIREWALL | STM32_MEMORY_FIREWALL;
    etzpc_controller.grant_access = stm32_etzpc_grant_access;
    etzpc_controller.release_access = stm32_etzpc_release_access;
// Get number of etzpc entries
    nb_per = FIELD_GET(ETZPC_HWCFGR_NUM_PER_SEC,
    readl(etzpc_controller.mmio + ETZPC_HWCFGR));
    nb_master = FIELD_GET(ETZPC_HWCFGR_NUM_AHB_SEC,
    readl(etzpc_controller.mmio + ETZPC_HWCFGR));
    etzpc_controller.max_entries = nb_per + nb_master;
    platform_set_drvdata(pdev, etzpc_controller);
    rc = stm32_firewall_controller_register(etzpc_controller);
    if (rc) {
    dev_err(etzpc_controller.dev, "Couldn't register as a firewall controller: %d",
    rc);
    return rc;
    }
    rc = stm32_firewall_populate_bus(etzpc_controller);
    if (rc) {
    dev_err(etzpc_controller.dev, "Couldn't populate ETZPC bus: %d",
    rc);
    return rc;
    }
// Populate all allowed nodes
    return of_platform_populate(np, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    }
    static const struct of_device_id stm32_etzpc_of_match[] = {
    { .compatible = "st,stm32-etzpc" },
    {}
    };
    MODULE_DEVICE_TABLE(of, stm32_etzpc_of_match);
    static struct platform_driver stm32_etzpc_driver = {
    .probe  = stm32_etzpc_probe,
    .driver = {
    .name = "stm32-etzpc",
    .of_match_table = stm32_etzpc_of_match,
    },
    };
    module_platform_driver(stm32_etzpc_driver);
    MODULE_AUTHOR("Gatien Chevallier <gatien.chevallier@foss.st.com>");
    MODULE_DESCRIPTION("STMicroelectronics ETZPC driver");
    MODULE_LICENSE("GPL");
