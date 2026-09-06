//! Automatically rewritten from C to Rust
//! Source: drivers/dma/dw/acpi.c
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
// Copyright (C) 2013,2019 Intel Corporation

#[no_mangle]
unsafe extern "C" fn dw_dma_acpi_filter(chan: *mut dma_chan, param: *mut c_void) -> bool {
    static bool dw_dma_acpi_filter(struct dma_chan *chan, void *param)
    {
    struct dw_dma *dw = to_dw_dma(chan.device);
    struct dw_dma_chip_pdata *data = dev_get_drvdata(dw.dma.dev);
    struct acpi_dma_spec *dma_spec = param;
    struct dw_dma_slave slave = {
    .dma_dev = dma_spec.dev,
    .src_id = dma_spec.slave_id,
    .dst_id = dma_spec.slave_id,
    .m_master = data.m_master,
    .p_master = data.p_master,
    };
    return dw_dma_filter(chan, &slave);
    }
#[no_mangle]
pub unsafe extern "C" fn dw_dma_acpi_controller_register(dw: *mut dw_dma) {
    void dw_dma_acpi_controller_register(struct dw_dma *dw)
    {
    struct device *dev = dw.dma.dev;
    struct acpi_dma_filter_info *info;
    int ret;
    if (!has_acpi_companion(dev))
    return;
    info = devm_kzalloc(dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return;
    dma_cap_zero(info.dma_cap);
    dma_cap_set(DMA_SLAVE, info.dma_cap);
    info.filter_fn = dw_dma_acpi_filter;
    ret = acpi_dma_controller_register(dev, acpi_dma_simple_xlate, info);
    if (ret)
    dev_err(dev, "could not register acpi_dma_controller\n");
    }
    EXPORT_SYMBOL_GPL(dw_dma_acpi_controller_register);
#[no_mangle]
pub unsafe extern "C" fn dw_dma_acpi_controller_free(dw: *mut dw_dma) {
    void dw_dma_acpi_controller_free(struct dw_dma *dw)
    {
    struct device *dev = dw.dma.dev;
    if (!has_acpi_companion(dev))
    return;
    acpi_dma_controller_free(dev);
    }
    EXPORT_SYMBOL_GPL(dw_dma_acpi_controller_free);
