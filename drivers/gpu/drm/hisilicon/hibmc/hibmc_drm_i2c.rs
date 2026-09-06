//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/hisilicon/hibmc/hibmc_drm_i2c.c
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
// Hisilicon Hibmc SoC drm driver
//
// Based on the bochs drm driver.
//
// Copyright (c) 2016 Huawei Limited.
//
// Author:
// Tian Tao <tiantao6@hisilicon.com>
//

pub const GPIO_DATA: c_uint = 0x0802A0;
pub const GPIO_DATA_DIRECTION: c_uint = 0x0802A4;

#[no_mangle]
unsafe extern "C" fn hibmc_set_i2c_signal(data: *mut c_void, mask: u32, value: c_int) {
    static void hibmc_set_i2c_signal(void *data, u32 mask, int value)
    {
    struct hibmc_vdac *vdac = data;
    struct hibmc_drm_private *priv = to_hibmc_drm_private(vdac.connector.dev);
    let mut tmp_dir: u32 = readl(priv.mmio + GPIO_DATA_DIRECTION);
    if (value) {
    tmp_dir &= ~mask;
    writel(tmp_dir, priv.mmio + GPIO_DATA_DIRECTION);
    } else {
    let mut tmp_data: u32 = readl(priv.mmio + GPIO_DATA);
    tmp_data &= ~mask;
    writel(tmp_data, priv.mmio + GPIO_DATA);
    tmp_dir |= mask;
    writel(tmp_dir, priv.mmio + GPIO_DATA_DIRECTION);
    }
    }
#[no_mangle]
unsafe extern "C" fn hibmc_get_i2c_signal(data: *mut c_void, mask: u32) -> c_int {
    static int hibmc_get_i2c_signal(void *data, u32 mask)
    {
    struct hibmc_vdac *vdac = data;
    struct hibmc_drm_private *priv = to_hibmc_drm_private(vdac.connector.dev);
    let mut tmp_dir: u32 = readl(priv.mmio + GPIO_DATA_DIRECTION);
    if ((tmp_dir & mask) != mask) {
    tmp_dir &= ~mask;
    writel(tmp_dir, priv.mmio + GPIO_DATA_DIRECTION);
    }
    return (readl(priv.mmio + GPIO_DATA) & mask) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn hibmc_ddc_setsda(data: *mut c_void, state: c_int) {
    static void hibmc_ddc_setsda(void *data, int state)
    {
    hibmc_set_i2c_signal(data, I2C_SDA_MASK, state);
    }
#[no_mangle]
unsafe extern "C" fn hibmc_ddc_setscl(data: *mut c_void, state: c_int) {
    static void hibmc_ddc_setscl(void *data, int state)
    {
    hibmc_set_i2c_signal(data, I2C_SCL_MASK, state);
    }
#[no_mangle]
unsafe extern "C" fn hibmc_ddc_getsda(data: *mut c_void) -> c_int {
    static int hibmc_ddc_getsda(void *data)
    {
    return hibmc_get_i2c_signal(data, I2C_SDA_MASK);
    }
#[no_mangle]
unsafe extern "C" fn hibmc_ddc_getscl(data: *mut c_void) -> c_int {
    static int hibmc_ddc_getscl(void *data)
    {
    return hibmc_get_i2c_signal(data, I2C_SCL_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn hibmc_ddc_create(drm_dev: *mut drm_device, vdac: *mut hibmc_vdac) -> c_int {
    int hibmc_ddc_create(struct drm_device *drm_dev, struct hibmc_vdac *vdac)
    {
    vdac.adapter.owner = THIS_MODULE;
    snprintf(vdac.adapter.name, I2C_NAME_SIZE, "HIS i2c bit bus");
    vdac.adapter.dev.parent = drm_dev.dev;
    i2c_set_adapdata(&vdac.adapter, vdac);
    vdac.adapter.algo_data = &vdac.bit_data;
    vdac.bit_data.udelay = 20;
    vdac.bit_data.timeout = usecs_to_jiffies(2000);
    vdac.bit_data.data = vdac;
    vdac.bit_data.setsda = hibmc_ddc_setsda;
    vdac.bit_data.setscl = hibmc_ddc_setscl;
    vdac.bit_data.getsda = hibmc_ddc_getsda;
    vdac.bit_data.getscl = hibmc_ddc_getscl;
    return i2c_bit_add_bus(&vdac.adapter);
    }
#[no_mangle]
pub unsafe extern "C" fn hibmc_ddc_del(vdac: *mut hibmc_vdac) {
    void hibmc_ddc_del(struct hibmc_vdac *vdac)
    {
    i2c_del_adapter(&vdac.adapter);
    }
