//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/acpiphp_ampere_altra.c
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
// ACPI PCI Hot Plug Extension for Ampere Altra. Allows control of
// attention LEDs via requests to system firmware.
//
// Copyright (C) 2023 Ampere Computing LLC
//

pub const HANDLE_OPEN: c_uint = 0xb0200000;
pub const HANDLE_CLOSE: c_uint = 0xb0300000;
pub const REQUEST: c_uint = 0xf0700000;
pub const LED_CMD: c_uint = 0x00000004;
pub const LED_ATTENTION: c_uint = 0x00000002;
pub const LED_SET_ON: c_uint = 0x00000001;
pub const LED_SET_OFF: c_uint = 0x00000002;
pub const LED_SET_BLINK: c_uint = 0x00000003;
    static u32 led_service_id[4];
#[no_mangle]
unsafe extern "C" fn led_status(status: u8) -> c_int {
    static int led_status(u8 status)
    {
    switch (status) {
    case 1: return LED_SET_ON;
    case 2: return LED_SET_BLINK;
    default: return LED_SET_OFF;
    }
    }
#[no_mangle]
unsafe extern "C" fn set_attention_status(slot: *mut hotplug_slot, status: u8) -> c_int {
    static int set_attention_status(struct hotplug_slot *slot, u8 status)
    {
    struct arm_smccc_res res;
    struct pci_bus *bus;
    struct pci_dev *root_port;
    unsigned long flags;
    u32 handle;
    let mut ret: c_int = 0;
    bus = slot.pci_slot.bus;
    root_port = pcie_find_root_port(bus.self);
    if (!root_port)
    return -ENODEV;
    local_irq_save(flags);
    arm_smccc_smc(HANDLE_OPEN, led_service_id[0], led_service_id[1],
    led_service_id[2], led_service_id[3], 0, 0, 0, &res);
    if (res.a0) {
    ret = -ENODEV;
    goto out;
    }
    handle = res.a1 & 0xffff0000;
    arm_smccc_smc(REQUEST, LED_CMD, led_status(status), LED_ATTENTION,
    (PCI_SLOT(root_port.devfn) << 4) | (pci_domain_nr(bus) & 0xf),
    0, 0, handle, &res);
    if (res.a0)
    ret = -ENODEV;
    arm_smccc_smc(HANDLE_CLOSE, handle, 0, 0, 0, 0, 0, 0, &res);
    out:
    local_irq_restore(flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_attention_status(slot: *mut hotplug_slot, status: *mut u8) -> c_int {
    static int get_attention_status(struct hotplug_slot *slot, u8 *status)
    {
    return -EINVAL;
    }
    static struct acpiphp_attention_info ampere_altra_attn = {
    .set_attn = set_attention_status,
    .get_attn = get_attention_status,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn altra_led_probe(pdev: *mut platform_device) -> c_int {
    static int altra_led_probe(struct platform_device *pdev)
    {
    struct fwnode_handle *fwnode = dev_fwnode(&pdev.dev);
    int ret;
    ret = fwnode_property_read_u32_array(fwnode, "uuid", led_service_id, 4);
    if (ret) {
    dev_err(&pdev.dev, "can't find uuid\n");
    return ret;
    }
    ret = acpiphp_register_attention(&ampere_altra_attn);
    if (ret) {
    dev_err(&pdev.dev, "can't register driver\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn altra_led_remove(pdev: *mut platform_device) {
    static void altra_led_remove(struct platform_device *pdev)
    {
    acpiphp_unregister_attention(&ampere_altra_attn);
    }
    static const struct acpi_device_id altra_led_ids[] = {
    { "AMPC0008", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, altra_led_ids);
    static struct platform_driver altra_led_driver = {
    .driver = {
    .name = "ampere-altra-leds",
    .acpi_match_table = altra_led_ids,
    },
    .probe = altra_led_probe,
    .remove = altra_led_remove,
    };
    module_platform_driver(altra_led_driver);
    MODULE_AUTHOR("D Scott Phillips <scott@os.amperecomputing.com>");
    MODULE_DESCRIPTION("ACPI PCI Hot Plug Extension for Ampere Altra");
    MODULE_LICENSE("GPL");
