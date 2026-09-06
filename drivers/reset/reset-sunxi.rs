//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-sunxi.c
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
// Allwinner SoCs Reset Controller driver
//
// Copyright 2013 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

#[no_mangle]
unsafe extern "C" fn sunxi_reset_init(np: *mut device_node) -> c_int {
    static int sunxi_reset_init(struct device_node *np)
    {
    struct reset_simple_data *data;
    struct resource res;
    resource_size_t size;
    int ret;
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
    goto err_mem_region;
    }
    spin_lock_init(&data.lock);
    data.rcdev.owner = THIS_MODULE;
    data.rcdev.nr_resets = size * 8;
    data.rcdev.ops = &reset_simple_ops;
    data.rcdev.of_node = np;
    data.active_low = true;
    return reset_controller_register(&data.rcdev);
    err_mem_region:
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
    static const struct of_device_id sunxi_early_reset_dt_ids[] __initconst = {
    { .compatible = "allwinner,sun6i-a31-ahb1-reset", },
    { /* sentinel */ },
    };
#[no_mangle]
pub unsafe extern "C" fn sun6i_reset_init() -> void __init {
    void __init sun6i_reset_init(void)
    {
    struct device_node *np;
    for_each_matching_node(np, sunxi_early_reset_dt_ids)
    sunxi_reset_init(np);
    }
