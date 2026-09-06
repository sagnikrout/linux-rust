//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-socfpga.c
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
// Copyright (C) 2018, Intel Corporation
// Copied from reset-sunxi.c
//

pub const SOCFPGA_NR_BANKS: c_int = 8;
#[no_mangle]
unsafe extern "C" fn a10_reset_init(np: *mut device_node) -> c_int {
    static int a10_reset_init(struct device_node *np)
    {
    struct reset_simple_data *data;
    struct resource res;
    resource_size_t size;
    int ret;
    let mut reg_offset: u32 = 0x10;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    ret = of_address_to_resource(np, 0, &res);
    if (ret)
    goto err_alloc;
    size = resource_size(&res);
    if (!request_mem_region(res.start, size, np.name)) {
    ret = -EBUSY;
    goto err_alloc;
    }
    data.membase = ioremap(res.start, size);
    if (!data.membase) {
    ret = -ENOMEM;
    goto release_region;
    }
    if (of_property_read_u32(np, "altr,modrst-offset", &reg_offset))
    pr_warn("missing altr,modrst-offset property, assuming 0x10\n");
    data.membase += reg_offset;
    spin_lock_init(&data.lock);
    data.rcdev.owner = THIS_MODULE;
    data.rcdev.nr_resets = SOCFPGA_NR_BANKS * 32;
    data.rcdev.ops = &reset_simple_ops;
    data.rcdev.of_node = np;
    data.status_active_low = true;
    ret = reset_controller_register(&data.rcdev);
    if (ret)
    pr_err("unable to register device\n");
    return ret;
    release_region:
    release_mem_region(res.start, size);
    err_alloc:
    kfree(data);
    return ret;
    };
//
// These are the reset controller we need to initialize early on in
// our system, before we can even think of using a regular device
// driver for it.
// The controllers that we can register through the regular device
// model are handled by the simple reset driver directly.
//
    static const struct of_device_id socfpga_early_reset_dt_ids[] __initconst = {
    { .compatible = "altr,rst-mgr", },
    { /* sentinel */ },
    };
#[no_mangle]
pub unsafe extern "C" fn socfpga_reset_init() -> void __init {
    void __init socfpga_reset_init(void)
    {
    struct device_node *np;
    for_each_matching_node(np, socfpga_early_reset_dt_ids)
    a10_reset_init(np);
    }
//
// The early driver is problematic, because it doesn't register
// itself as a driver. This causes certain device links to prevent
// consumer devices from probing. The hacky solution is to register
// an empty driver, whose only job is to attach itself to the reset
// manager and call probe.
//
    static const struct of_device_id socfpga_reset_dt_ids[] = {
    { .compatible = "altr,rst-mgr", },
    { /* sentinel */ },
    };
#[no_mangle]
unsafe extern "C" fn reset_simple_probe(pdev: *mut platform_device) -> c_int {
    static int reset_simple_probe(struct platform_device *pdev)
    {
    return 0;
    }
    static struct platform_driver reset_socfpga_driver = {
    .probe	= reset_simple_probe,
    .driver = {
    .name		= "socfpga-reset",
    .of_match_table	= socfpga_reset_dt_ids,
    },
    };
    builtin_platform_driver(reset_socfpga_driver);
