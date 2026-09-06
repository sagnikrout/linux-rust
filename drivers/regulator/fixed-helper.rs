//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/fixed-helper.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fixed_regulator_data {
    pub cfg: fixed_voltage_config,
    pub init_data: regulator_init_data,
    pub pdev: platform_device,
}

#[no_mangle]
unsafe extern "C" fn regulator_fixed_release(dev: *mut device) {
    static void regulator_fixed_release(struct device *dev)
    {
    struct fixed_regulator_data *data = container_of(dev,
    struct fixed_regulator_data, pdev.dev);
    kfree_const(data.cfg.supply_name);
    kfree(data);
    }
//
// regulator_register_always_on - register an always-on regulator with a fixed name
// @id: platform device id
// @name: name to be used for the regulator
// @supplies: consumers for this regulator
// @num_supplies: number of consumers
// @uv: voltage in microvolts
//
// Return: Pointer to registered platform device, or %NULL if memory allocation fails.
//
    struct platform_device *regulator_register_always_on(int id, const char *name,
    struct regulator_consumer_supply *supplies, int num_supplies, int uv)
    {
    struct fixed_regulator_data *data;
    data = kzalloc_obj(*data);
    if (!data)
    return core::ptr::null_mut();
    data.cfg.supply_name = kstrdup_const(name, GFP_KERNEL);
    if (!data.cfg.supply_name) {
    kfree(data);
    return core::ptr::null_mut();
    }
    data.cfg.microvolts = uv;
    data.cfg.enabled_at_boot = 1;
    data.cfg.init_data = &data.init_data;
    data.init_data.constraints.always_on = 1;
    data.init_data.consumer_supplies = supplies;
    data.init_data.num_consumer_supplies = num_supplies;
    data.pdev.name = "reg-fixed-voltage";
    data.pdev.id = id;
    data.pdev.dev.platform_data = &data.cfg;
    data.pdev.dev.release = regulator_fixed_release;
    platform_device_register(&data.pdev);
    return &data.pdev;
    }
