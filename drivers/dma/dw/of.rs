//! Automatically rewritten from C to Rust
//! Source: drivers/dma/dw/of.c
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
// Platform driver for the Synopsys DesignWare DMA Controller
//
// Copyright (C) 2007-2008 Atmel Corporation
// Copyright (C) 2010-2011 ST Microelectronics
// Copyright (C) 2013 Intel Corporation
//

    static struct dma_chan *dw_dma_of_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct dw_dma *dw = ofdma.of_dma_data;
    struct dw_dma_slave slave = {
    .dma_dev = dw.dma.dev,
    };
    dma_cap_mask_t cap;
    if (dma_spec.args_count < 3 || dma_spec.args_count > 4)
    return core::ptr::null_mut();
    slave.src_id = dma_spec.args[0];
    slave.dst_id = dma_spec.args[0];
    slave.m_master = dma_spec.args[1];
    slave.p_master = dma_spec.args[2];
    if (dma_spec.args_count >= 4)
    slave.channels = dma_spec.args[3];
    if (WARN_ON(slave.src_id >= DW_DMA_MAX_NR_REQUESTS ||
    slave.dst_id >= DW_DMA_MAX_NR_REQUESTS ||
    slave.m_master >= dw.pdata.nr_masters ||
    slave.p_master >= dw.pdata.nr_masters ||
    slave.channels >= BIT(dw.pdata.nr_channels)))
    return core::ptr::null_mut();
    dma_cap_zero(cap);
    dma_cap_set(DMA_SLAVE, cap);
// TODO: there should be a simpler way to do this
    return dma_request_channel(cap, dw_dma_filter, &slave);
    }
    struct dw_dma_platform_data *dw_dma_parse_dt(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct dw_dma_platform_data *pdata;
    u32 tmp, arr[DW_DMA_MAX_NR_MASTERS];
    u32 nr_masters;
    u32 nr_channels;
    if (of_property_read_u32(np, "dma-masters", &nr_masters))
    return core::ptr::null_mut();
    if (nr_masters < 1 || nr_masters > DW_DMA_MAX_NR_MASTERS)
    return core::ptr::null_mut();
    if (of_property_read_u32(np, "dma-channels", &nr_channels))
    return core::ptr::null_mut();
    if (nr_channels > DW_DMA_MAX_NR_CHANNELS)
    return core::ptr::null_mut();
    pdata = devm_kzalloc(&pdev.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return core::ptr::null_mut();
    pdata.nr_masters = nr_masters;
    pdata.nr_channels = nr_channels;
    of_property_read_u32(np, "chan_allocation_order", &pdata.chan_allocation_order);
    of_property_read_u32(np, "chan_priority", &pdata.chan_priority);
    of_property_read_u32(np, "block_size", &pdata.block_size);
// Try deprecated property first
    if (!of_property_read_u32_array(np, "data_width", arr, nr_masters)) {
    for (tmp = 0; tmp < nr_masters; tmp++)
    pdata.data_width[tmp] = BIT(arr[tmp] & 0x07);
    }
// If "data_width" and "data-width" both provided use the latter one
    of_property_read_u32_array(np, "data-width", pdata.data_width, nr_masters);
    memset32(pdata.multi_block, 1, nr_channels);
    of_property_read_u32_array(np, "multi-block", pdata.multi_block, nr_channels);
    memset32(pdata.max_burst, DW_DMA_MAX_BURST, nr_channels);
    of_property_read_u32_array(np, "snps,max-burst-len", pdata.max_burst, nr_channels);
    of_property_read_u32(np, "snps,dma-protection-control", &pdata.protctl);
    if (pdata.protctl > CHAN_PROTCTL_MASK)
    return core::ptr::null_mut();
    return pdata;
    }
#[no_mangle]
pub unsafe extern "C" fn dw_dma_of_controller_register(dw: *mut dw_dma) {
    void dw_dma_of_controller_register(struct dw_dma *dw)
    {
    struct device *dev = dw.dma.dev;
    int ret;
    if (!dev.of_node)
    return;
    ret = of_dma_controller_register(dev.of_node, dw_dma_of_xlate, dw);
    if (ret)
    dev_err(dev, "could not register of_dma_controller\n");
    }
#[no_mangle]
pub unsafe extern "C" fn dw_dma_of_controller_free(dw: *mut dw_dma) {
    void dw_dma_of_controller_free(struct dw_dma *dw)
    {
    struct device *dev = dw.dma.dev;
    if (!dev.of_node)
    return;
    of_dma_controller_free(dev.of_node);
    }
