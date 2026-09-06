//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pwrctrl/generic.c
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
// Copyright (C) 2024 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot_pwrctrl {
    pub pwrctrl: pci_pwrctrl,
    pub supplies: *mut regulator_bulk_data,
    pub num_supplies: c_int,
    pub clk: *mut clk,
    pub pwrseq: *mut pwrseq_desc,
}

#[no_mangle]
unsafe extern "C" fn slot_pwrctrl_power_on(pwrctrl: *mut pci_pwrctrl) -> c_int {
    static int slot_pwrctrl_power_on(struct pci_pwrctrl *pwrctrl)
    {
    struct slot_pwrctrl *slot = container_of(pwrctrl,
    struct slot_pwrctrl, pwrctrl);
    int ret;
    if (slot.pwrseq) {
    pwrseq_enable(slot.pwrseq);
    return 0;
    }
    ret = regulator_bulk_enable(slot.num_supplies, slot.supplies);
    if (ret < 0) {
    dev_err(slot.pwrctrl.dev, "Failed to enable slot regulators\n");
    return ret;
    }
    return clk_prepare_enable(slot.clk);
    }
#[no_mangle]
unsafe extern "C" fn slot_pwrctrl_power_off(pwrctrl: *mut pci_pwrctrl) -> c_int {
    static int slot_pwrctrl_power_off(struct pci_pwrctrl *pwrctrl)
    {
    struct slot_pwrctrl *slot = container_of(pwrctrl,
    struct slot_pwrctrl, pwrctrl);
    if (slot.pwrseq) {
    pwrseq_disable(slot.pwrseq);
    return 0;
    }
    regulator_bulk_disable(slot.num_supplies, slot.supplies);
    clk_disable_unprepare(slot.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn devm_slot_pwrctrl_release(data: *mut c_void) {
    static void devm_slot_pwrctrl_release(void *data)
    {
    struct slot_pwrctrl *slot = data;
    regulator_bulk_free(slot.num_supplies, slot.supplies);
    }
#[no_mangle]
unsafe extern "C" fn slot_pwrctrl_probe(pdev: *mut platform_device) -> c_int {
    static int slot_pwrctrl_probe(struct platform_device *pdev)
    {
    struct slot_pwrctrl *slot;
    struct device *dev = &pdev.dev;
    int ret;
    slot = devm_kzalloc(dev, sizeof(*slot), GFP_KERNEL);
    if (!slot)
    return -ENOMEM;
    if (of_graph_is_present(dev_of_node(dev))) {
    slot.pwrseq = devm_pwrseq_get(dev, "pcie");
    if (IS_ERR(slot.pwrseq))
    return dev_err_probe(dev, PTR_ERR(slot.pwrseq),
    "Failed to get the power sequencer\n");
    goto skip_resources;
    }
    ret = of_regulator_bulk_get_all(dev, dev_of_node(dev),
    &slot.supplies);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to get slot regulators\n");
    slot.num_supplies = ret;
    slot.clk = devm_clk_get_optional(dev, core::ptr::null_mut());
    if (IS_ERR(slot.clk))
    return dev_err_probe(dev, PTR_ERR(slot.clk),
    "Failed to enable slot clock\n");
    skip_resources:
    slot.pwrctrl.power_on = slot_pwrctrl_power_on;
    slot.pwrctrl.power_off = slot_pwrctrl_power_off;
    ret = devm_add_action_or_reset(dev, devm_slot_pwrctrl_release, slot);
    if (ret)
    return ret;
    pci_pwrctrl_init(&slot.pwrctrl, dev);
    ret = devm_pci_pwrctrl_device_set_ready(dev, &slot.pwrctrl);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register pwrctrl driver\n");
    return 0;
    }
    static const struct of_device_id slot_pwrctrl_of_match[] = {
    {
    .compatible = "pciclass,0604",
    },
// Renesas UPD720201/UPD720202 USB 3.0 xHCI Host Controller
    {
    .compatible = "pci1912,0014",
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, slot_pwrctrl_of_match);
    static struct platform_driver slot_pwrctrl_driver = {
    .driver = {
    .name = "pci-pwrctrl-slot",
    .of_match_table = slot_pwrctrl_of_match,
    },
    .probe = slot_pwrctrl_probe,
    };
    module_platform_driver(slot_pwrctrl_driver);
    MODULE_AUTHOR("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
    MODULE_DESCRIPTION("Generic PCI Power Control driver for PCI Slots");
    MODULE_LICENSE("GPL");
