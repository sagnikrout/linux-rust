//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/arm64/amba.c
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
// ACPI support for platform bus type.
//
// Copyright (C) 2015, Linaro Ltd
// Author: Graeme Gregory <graeme.gregory@linaro.org>
//

    static const struct acpi_device_id amba_id_list[] = {
    {"ARMH0061", 0}, /* PL061 GPIO Device */
    {"ARMH0330", 0}, /* ARM DMA Controller DMA-330 */
    {"", 0},
    };
#[no_mangle]
unsafe extern "C" fn amba_register_dummy_clk() {
    static void amba_register_dummy_clk(void)
    {
    struct clk *amba_dummy_clk;
    amba_dummy_clk = clk_register_fixed_rate(core::ptr::null_mut(), "apb_pclk", core::ptr::null_mut(), 0, 0);
    clk_register_clkdev(amba_dummy_clk, "apb_pclk", core::ptr::null_mut());
    }
    static int amba_handler_attach(struct acpi_device *adev,
    const struct acpi_device_id *id)
    {
    struct acpi_device *parent = acpi_dev_parent(adev);
    struct amba_device *dev;
    struct resource_entry *rentry;
    struct list_head resource_list;
    let mut address_found: bool = false;
    let mut irq_no: c_int = 0;
    int ret;
// If the ACPI node already has a physical device attached, skip it.
    if (adev.physical_node_count)
    return 0;
    dev = amba_device_alloc(dev_name(&adev.dev), 0, 0);
    if (!dev) {
    dev_err(&adev.dev, "%s(): amba_device_alloc() failed\n",
    __func__);
    return -ENOMEM;
    }
    INIT_LIST_HEAD(&resource_list);
    ret = acpi_dev_get_resources(adev, &resource_list, core::ptr::null_mut(), core::ptr::null_mut());
    if (ret < 0)
    goto err_free;
    list_for_each_entry(rentry, &resource_list, node) {
    switch (resource_type(rentry.res)) {
    case IORESOURCE_MEM:
    if (!address_found) {
    dev.res = *rentry.res;
    dev.res.name = dev_name(&dev.dev);
    address_found = true;
    }
    break;
    case IORESOURCE_IRQ:
    if (irq_no < AMBA_NR_IRQS)
    dev.irq[irq_no++] = rentry.res.start;
    break;
    default:
    dev_warn(&adev.dev, "Invalid resource\n");
    break;
    }
    }
    acpi_dev_free_resource_list(&resource_list);
//
// If the ACPI node has a parent and that parent has a physical device
// attached to it, that physical device should be the parent of
// the amba device we are about to create.
//
    if (parent)
    dev.dev.parent = acpi_get_first_physical_node(parent);
    device_set_node(&dev.dev, acpi_fwnode_handle(adev));
    ret = amba_device_add(dev, &iomem_resource);
    if (ret) {
    dev_err(&adev.dev, "%s(): amba_device_add() failed (%d)\n",
    __func__, ret);
    goto err_free;
    }
    return 1;
    err_free:
    amba_device_put(dev);
    return ret;
    }
    static struct acpi_scan_handler amba_handler = {
    .ids = amba_id_list,
    .attach = amba_handler_attach,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_amba_init() -> void __init {
    void __init acpi_amba_init(void)
    {
    amba_register_dummy_clk();
    acpi_scan_add_handler(&amba_handler);
    }
