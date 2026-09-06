//! Automatically rewritten from C to Rust
//! Source: drivers/dma/lpc18xx-dmamux.c
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
// DMA Router driver for LPC18xx/43xx DMA MUX
//
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//
// Based on TI DMA Crossbar driver by:
// Copyright (C) 2015 Texas Instruments Incorporated - http://www.ti.com
// Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
//

// CREG register offset and macros for mux manipulation
pub const LPC18XX_CREG_DMAMUX: c_uint = 0x11c;

pub const LPC18XX_DMAMUX_MAX_VAL: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc18xx_dmamux {
    pub value: u32,
    pub busy: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc18xx_dmamux_data {
    pub dmarouter: dma_router,
    pub muxes: *mut lpc18xx_dmamux,
    pub dma_master_requests: u32,
    pub dma_mux_requests: u32,
    pub reg: *mut regmap,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn lpc18xx_dmamux_free(dev: *mut device, route_data: *mut c_void) {
    static void lpc18xx_dmamux_free(struct device *dev, void *route_data)
    {
    struct lpc18xx_dmamux_data *dmamux = dev_get_drvdata(dev);
    struct lpc18xx_dmamux *mux = route_data;
    unsigned long flags;
    spin_lock_irqsave(&dmamux.lock, flags);
    mux.busy = false;
    spin_unlock_irqrestore(&dmamux.lock, flags);
    }
    static void *lpc18xx_dmamux_reserve(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct platform_device *pdev = of_find_device_by_node(ofdma.of_node);
    struct lpc18xx_dmamux_data *dmamux = platform_get_drvdata(pdev);
    unsigned long flags;
    unsigned mux;
    let mut ret: c_int = -EINVAL;
    if (dma_spec.args_count != 3) {
    dev_err(&pdev.dev, "invalid number of dma mux args\n");
    goto err_put_pdev;
    }
    mux = dma_spec.args[0];
    if (mux >= dmamux.dma_master_requests) {
    dev_err(&pdev.dev, "invalid mux number: %d\n",
    dma_spec.args[0]);
    goto err_put_pdev;
    }
    if (dma_spec.args[1] > LPC18XX_DMAMUX_MAX_VAL) {
    dev_err(&pdev.dev, "invalid dma mux value: %d\n",
    dma_spec.args[1]);
    goto err_put_pdev;
    }
// The of_node_put() will be done in the core for the node
    dma_spec.np = of_parse_phandle(ofdma.of_node, "dma-masters", 0);
    if (!dma_spec.np) {
    dev_err(&pdev.dev, "can't get dma master\n");
    goto err_put_pdev;
    }
    spin_lock_irqsave(&dmamux.lock, flags);
    if (dmamux.muxes[mux].busy) {
    spin_unlock_irqrestore(&dmamux.lock, flags);
    dev_err(&pdev.dev, "dma request %u busy with %u.%u\n",
    mux, mux, dmamux.muxes[mux].value);
    of_node_put(dma_spec.np);
    ret = -EBUSY;
    goto err_put_pdev;
    }
    dmamux.muxes[mux].busy = true;
    dmamux.muxes[mux].value = dma_spec.args[1];
    regmap_update_bits(dmamux.reg, LPC18XX_CREG_DMAMUX,
    LPC18XX_DMAMUX_MASK(mux),
    LPC18XX_DMAMUX_VAL(dmamux.muxes[mux].value, mux));
    spin_unlock_irqrestore(&dmamux.lock, flags);
    dma_spec.args[1] = dma_spec.args[2];
    dma_spec.args_count = 2;
    dev_dbg(&pdev.dev, "mapping dmamux %u.%u to dma request %u\n", mux,
    dmamux.muxes[mux].value, mux);
    put_device(&pdev.dev);
    return &dmamux.muxes[mux];
    err_put_pdev:
    put_device(&pdev.dev);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_dmamux_probe(pdev: *mut platform_device) -> c_int {
    static int lpc18xx_dmamux_probe(struct platform_device *pdev)
    {
    struct device_node *dma_np, *np = pdev.dev.of_node;
    struct lpc18xx_dmamux_data *dmamux;
    int ret;
    dmamux = devm_kzalloc(&pdev.dev, sizeof(*dmamux), GFP_KERNEL);
    if (!dmamux)
    return -ENOMEM;
    dmamux.reg = syscon_regmap_lookup_by_compatible("nxp,lpc1850-creg");
    if (IS_ERR(dmamux.reg)) {
    dev_err(&pdev.dev, "syscon lookup failed\n");
    return PTR_ERR(dmamux.reg);
    }
    ret = of_property_read_u32(np, "dma-requests",
    &dmamux.dma_mux_requests);
    if (ret) {
    dev_err(&pdev.dev, "missing dma-requests property\n");
    return ret;
    }
    dma_np = of_parse_phandle(np, "dma-masters", 0);
    if (!dma_np) {
    dev_err(&pdev.dev, "can't get dma master\n");
    return -ENODEV;
    }
    ret = of_property_read_u32(dma_np, "dma-requests",
    &dmamux.dma_master_requests);
    of_node_put(dma_np);
    if (ret) {
    dev_err(&pdev.dev, "missing master dma-requests property\n");
    return ret;
    }
    dmamux.muxes = devm_kcalloc(&pdev.dev, dmamux.dma_master_requests,
    sizeof(struct lpc18xx_dmamux),
    GFP_KERNEL);
    if (!dmamux.muxes)
    return -ENOMEM;
    spin_lock_init(&dmamux.lock);
    platform_set_drvdata(pdev, dmamux);
    dmamux.dmarouter.dev = &pdev.dev;
    dmamux.dmarouter.route_free = lpc18xx_dmamux_free;
    return of_dma_router_register(np, lpc18xx_dmamux_reserve,
    &dmamux.dmarouter);
    }
    static const struct of_device_id lpc18xx_dmamux_match[] = {
    { .compatible = "nxp,lpc1850-dmamux" },
    {},
    };
    static struct platform_driver lpc18xx_dmamux_driver = {
    .probe	= lpc18xx_dmamux_probe,
    .driver = {
    .name = "lpc18xx-dmamux",
    .of_match_table = lpc18xx_dmamux_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_dmamux_init() -> int __init {
    static int __init lpc18xx_dmamux_init(void)
    {
    return platform_driver_register(&lpc18xx_dmamux_driver);
    }
    arch_initcall(lpc18xx_dmamux_init);
