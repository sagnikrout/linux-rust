//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-pxa2xx-dma.c
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
// PXA2xx SPI DMA engine support.
//
// Copyright (C) 2013, 2021 Intel Corporation
// Author: Mika Westerberg <mika.westerberg@linux.intel.com>
//

    struct device;
    static void pxa2xx_spi_dma_transfer_complete(struct driver_data *drv_data,
    bool error)
    {
    struct spi_message *msg = drv_data.controller.cur_msg;
//
// It is possible that one CPU is handling ROR interrupt and other
// just gets DMA completion. Calling spi_finalize_current_transfer()
// twice for the same transfer leads to problems thus we prevent
// concurrent calls by using dma_running.
//
    if (atomic_dec_and_test(&drv_data.dma_running)) {
//
// If the other CPU is still handling the ROR interrupt we
// might not know about the error yet. So we re-check the
// ROR bit here before we clear the status register.
//
    if (!error)
    error = read_SSSR_bits(drv_data, drv_data.mask_sr) & SSSR_ROR;
// Clear status & disable interrupts
    clear_SSCR1_bits(drv_data, drv_data.dma_cr1);
    write_SSSR_CS(drv_data, drv_data.clear_sr);
    if (!pxa25x_ssp_comp(drv_data))
    pxa2xx_spi_write(drv_data, SSTO, 0);
    if (error) {
// In case we got an error we disable the SSP now
    pxa_ssp_disable(drv_data.ssp);
    msg.status = -EIO;
    }
    spi_finalize_current_transfer(drv_data.controller);
    }
    }
#[no_mangle]
unsafe extern "C" fn pxa2xx_spi_dma_callback(data: *mut c_void) {
    static void pxa2xx_spi_dma_callback(void *data)
    {
    pxa2xx_spi_dma_transfer_complete(data, false);
    }
    static struct dma_async_tx_descriptor *
    pxa2xx_spi_dma_prepare_one(struct driver_data *drv_data,
    enum dma_transfer_direction dir,
    struct spi_transfer *xfer)
    {
    enum dma_slave_buswidth width;
    struct dma_slave_config cfg;
    struct dma_chan *chan;
    struct sg_table *sgt;
    int ret;
    switch (drv_data.n_bytes) {
    case 1:
    width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    break;
    case 2:
    width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    break;
    default:
    width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    break;
    }
    memset(&cfg, 0, sizeof(cfg));
    cfg.direction = dir;
    if (dir == DMA_MEM_TO_DEV) {
    cfg.dst_addr = drv_data.ssp.phys_base + SSDR;
    cfg.dst_addr_width = width;
    cfg.dst_maxburst = drv_data.controller_info.dma_burst_size;
    sgt = &xfer.tx_sg;
    chan = drv_data.controller.dma_tx;
    } else {
    cfg.src_addr = drv_data.ssp.phys_base + SSDR;
    cfg.src_addr_width = width;
    cfg.src_maxburst = drv_data.controller_info.dma_burst_size;
    sgt = &xfer.rx_sg;
    chan = drv_data.controller.dma_rx;
    }
    ret = dmaengine_slave_config(chan, &cfg);
    if (ret) {
    dev_warn(drv_data.ssp.dev, "DMA slave config failed\n");
    return core::ptr::null_mut();
    }
    return dmaengine_prep_slave_sg(chan, sgt.sgl, sgt.nents, dir,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_spi_dma_transfer(drv_data: *mut driver_data) -> irqreturn_t {
    irqreturn_t pxa2xx_spi_dma_transfer(struct driver_data *drv_data)
    {
    u32 status;
    status = read_SSSR_bits(drv_data, drv_data.mask_sr);
    if (status & SSSR_ROR) {
    dev_err(drv_data.ssp.dev, "FIFO overrun\n");
    dmaengine_terminate_async(drv_data.controller.dma_rx);
    dmaengine_terminate_async(drv_data.controller.dma_tx);
    pxa2xx_spi_dma_transfer_complete(drv_data, true);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
    int pxa2xx_spi_dma_prepare(struct driver_data *drv_data,
    struct spi_transfer *xfer)
    {
    struct dma_async_tx_descriptor *tx_desc, *rx_desc;
    int err;
    tx_desc = pxa2xx_spi_dma_prepare_one(drv_data, DMA_MEM_TO_DEV, xfer);
    if (!tx_desc) {
    dev_err(drv_data.ssp.dev, "failed to get DMA TX descriptor\n");
    err = -EBUSY;
    goto err_tx;
    }
    rx_desc = pxa2xx_spi_dma_prepare_one(drv_data, DMA_DEV_TO_MEM, xfer);
    if (!rx_desc) {
    dev_err(drv_data.ssp.dev, "failed to get DMA RX descriptor\n");
    err = -EBUSY;
    goto err_rx;
    }
// We are ready when RX completes
    rx_desc.callback = pxa2xx_spi_dma_callback;
    rx_desc.callback_param = drv_data;
    dmaengine_submit(rx_desc);
    dmaengine_submit(tx_desc);
    return 0;
    err_rx:
    dmaengine_terminate_async(drv_data.controller.dma_tx);
    err_tx:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_spi_dma_start(drv_data: *mut driver_data) {
    void pxa2xx_spi_dma_start(struct driver_data *drv_data)
    {
    dma_async_issue_pending(drv_data.controller.dma_rx);
    dma_async_issue_pending(drv_data.controller.dma_tx);
    atomic_set(&drv_data.dma_running, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_spi_dma_stop(drv_data: *mut driver_data) {
    void pxa2xx_spi_dma_stop(struct driver_data *drv_data)
    {
    atomic_set(&drv_data.dma_running, 0);
    dmaengine_terminate_sync(drv_data.controller.dma_rx);
    dmaengine_terminate_sync(drv_data.controller.dma_tx);
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_spi_dma_setup(drv_data: *mut driver_data) -> c_int {
    int pxa2xx_spi_dma_setup(struct driver_data *drv_data)
    {
    struct pxa2xx_spi_controller *pdata = drv_data.controller_info;
    struct spi_controller *controller = drv_data.controller;
    struct device *dev = drv_data.ssp.dev;
    dma_cap_mask_t mask;
    dma_cap_zero(mask);
    dma_cap_set(DMA_SLAVE, mask);
    controller.dma_tx = dma_request_slave_channel_compat(mask,
    pdata.dma_filter, pdata.tx_param, dev, "tx");
    if (!controller.dma_tx)
    return -ENODEV;
    controller.dma_rx = dma_request_slave_channel_compat(mask,
    pdata.dma_filter, pdata.rx_param, dev, "rx");
    if (!controller.dma_rx) {
    dma_release_channel(controller.dma_tx);
    controller.dma_tx = core::ptr::null_mut();
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pxa2xx_spi_dma_release(drv_data: *mut driver_data) {
    void pxa2xx_spi_dma_release(struct driver_data *drv_data)
    {
    struct spi_controller *controller = drv_data.controller;
    if (controller.dma_rx) {
    dmaengine_terminate_sync(controller.dma_rx);
    dma_release_channel(controller.dma_rx);
    controller.dma_rx = core::ptr::null_mut();
    }
    if (controller.dma_tx) {
    dmaengine_terminate_sync(controller.dma_tx);
    dma_release_channel(controller.dma_tx);
    controller.dma_tx = core::ptr::null_mut();
    }
    }
