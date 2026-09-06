//! Automatically rewritten from C to Rust
//! Source: drivers/bus/fsl-mc/fsl-mc-msi.c
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
// Freescale Management Complex (MC) bus driver MSI support
//
// Copyright (C) 2015-2016 Freescale Semiconductor, Inc.
// Author: German Rivera <German.Rivera@freescale.com>
//

#[no_mangle]
unsafe extern "C" fn fsl_mc_write_msi_msg(msi_desc: *mut msi_desc, msg: *mut msi_msg) {
    static void fsl_mc_write_msi_msg(struct msi_desc *msi_desc, struct msi_msg *msg)
    {
    struct fsl_mc_device *mc_bus_dev = to_fsl_mc_device(msi_desc.dev);
    struct fsl_mc_bus *mc_bus = to_fsl_mc_bus(mc_bus_dev);
    struct fsl_mc_device_irq *mc_dev_irq = &mc_bus.irq_resources[msi_desc.msi_index];
    struct fsl_mc_device *owner_mc_dev = mc_dev_irq.mc_dev;
    struct dprc_irq_cfg irq_cfg;
    int error;
    msi_desc.msg = *msg;
//
// msi_desc->msg.address is 0x0 when this function is invoked in
// the free_irq() code path. In this case, for the MC, we don't
// really need to "unprogram" the MSI, so we just return.
//
    if (msi_desc.msg.address_lo == 0x0 && msi_desc.msg.address_hi == 0x0)
    return;
    if (!owner_mc_dev)
    return;
    irq_cfg.paddr = ((u64)msi_desc.msg.address_hi << 32) |
    msi_desc.msg.address_lo;
    irq_cfg.val = msi_desc.msg.data;
    irq_cfg.irq_num = msi_desc.irq;
    if (owner_mc_dev == mc_bus_dev) {
//
// IRQ is for the mc_bus_dev's DPRC itself
//
    error = dprc_set_irq(mc_bus_dev.mc_io,
    MC_CMD_FLAG_INTR_DIS | MC_CMD_FLAG_PRI,
    mc_bus_dev.mc_handle,
    mc_dev_irq.dev_irq_index,
    &irq_cfg);
    if (error < 0) {
    dev_err(&owner_mc_dev.dev,
    "dprc_set_irq() failed: %d\n", error);
    }
    } else {
//
// IRQ is for for a child device of mc_bus_dev
//
    error = dprc_set_obj_irq(mc_bus_dev.mc_io,
    MC_CMD_FLAG_INTR_DIS | MC_CMD_FLAG_PRI,
    mc_bus_dev.mc_handle,
    owner_mc_dev.obj_desc.type,
    owner_mc_dev.obj_desc.id,
    mc_dev_irq.dev_irq_index,
    &irq_cfg);
    if (error < 0) {
    dev_err(&owner_mc_dev.dev,
    "dprc_obj_set_irq() failed: %d\n", error);
    }
    }
    }
    struct irq_domain *fsl_mc_get_msi_parent(struct device *dev)
    {
    struct fsl_mc_device *mc_dev = to_fsl_mc_device(dev);
    struct device *root_dprc_dev;
    struct device *bus_dev;
    fsl_mc_get_root_dprc(dev, &root_dprc_dev);
    bus_dev = root_dprc_dev.parent;
    return (bus_dev.of_node ?
    of_msi_get_domain(bus_dev, bus_dev.of_node, DOMAIN_BUS_NEXUS) :
    iort_get_device_domain(bus_dev, mc_dev.icid, DOMAIN_BUS_NEXUS));
    }
#[no_mangle]
pub unsafe extern "C" fn fsl_mc_msi_domain_alloc_irqs(dev: *mut device, irq_count: c_uint) -> c_int {
    int fsl_mc_msi_domain_alloc_irqs(struct device *dev,  unsigned int irq_count)
    {
    let mut error: c_int = msi_setup_device_data(dev);
    if (error)
    return error;
    error = platform_device_msi_init_and_alloc_irqs(dev, irq_count, fsl_mc_write_msi_msg);
    if (error)
    dev_err(dev, "Failed to allocate IRQs\n");
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn fsl_mc_msi_domain_free_irqs(dev: *mut device) {
    void fsl_mc_msi_domain_free_irqs(struct device *dev)
    {
    msi_domain_free_irqs_all(dev, MSI_DEFAULT_DOMAIN);
    }
#[no_mangle]
pub unsafe extern "C" fn fsl_mc_get_msi_id(dev: *mut device) -> u32 {
    u32 fsl_mc_get_msi_id(struct device *dev)
    {
    struct fsl_mc_device *mc_dev = to_fsl_mc_device(dev);
    struct device *root_dprc_dev;
    fsl_mc_get_root_dprc(dev, &root_dprc_dev);
    return (root_dprc_dev.parent.of_node ?
    of_msi_xlate(dev, core::ptr::null_mut(), mc_dev.icid) :
    iort_msi_map_id(dev, mc_dev.icid));
    }
