//! Automatically rewritten from C to Rust
//! Source: drivers/dma/lpc32xx-dmamux.c
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
// Copyright 2024 Timesys Corporation <piotr.wojtaszczyk@timesys.com>
//
// Based on TI DMA Crossbar driver by:
// Copyright (C) 2015 Texas Instruments Incorporated - http://www.ti.com
// Author: Peter Ujfalusi <peter.ujfalusi@ti.com>

pub const LPC32XX_SSP_CLK_CTRL: c_uint = 0x78;
pub const LPC32XX_I2S_CLK_CTRL: c_uint = 0x7c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_dmamux {
    pub signal: c_int,
    pub name_sel0: *mut c_char,
    pub name_sel1: *mut c_char,
    pub muxval: c_int,
    pub muxreg: c_int,
    pub bit: c_int,
    pub busy: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_dmamux_data {
    pub dmarouter: dma_router,
    pub reg: *mut regmap,
    pub /: *mut *mut spinlock_t lock; / protects busy status flag,
}

// From LPC32x0 User manual "3.2.1 DMA request signals"
    static struct lpc32xx_dmamux lpc32xx_muxes[] = {
    {
    .signal = 3,
    .name_sel0 = "spi2-rx-tx",
    .name_sel1 = "ssp1-rx",
    .muxreg = LPC32XX_SSP_CLK_CTRL,
    .bit = 5,
    },
    {
    .signal = 10,
    .name_sel0 = "uart7-rx",
    .name_sel1 = "i2s1-dma1",
    .muxreg = LPC32XX_I2S_CLK_CTRL,
    .bit = 4,
    },
    {
    .signal = 11,
    .name_sel0 = "spi1-rx-tx",
    .name_sel1 = "ssp1-tx",
    .muxreg = LPC32XX_SSP_CLK_CTRL,
    .bit = 4,
    },
    {
    .signal = 14,
    .name_sel0 = "none",
    .name_sel1 = "ssp0-rx",
    .muxreg = LPC32XX_SSP_CLK_CTRL,
    .bit = 3,
    },
    {
    .signal = 15,
    .name_sel0 = "none",
    .name_sel1 = "ssp0-tx",
    .muxreg = LPC32XX_SSP_CLK_CTRL,
    .bit = 2,
    },
    };
#[no_mangle]
unsafe extern "C" fn lpc32xx_dmamux_release(dev: *mut device, route_data: *mut c_void) {
    static void lpc32xx_dmamux_release(struct device *dev, void *route_data)
    {
    struct lpc32xx_dmamux_data *dmamux = dev_get_drvdata(dev);
    struct lpc32xx_dmamux *mux = route_data;
    dev_dbg(dev, "releasing dma request signal %d routed to %s\n",
    mux.signal, mux.muxval ? mux.name_sel1 : mux.name_sel1);
    guard(spinlock)(&dmamux.lock);
    mux.busy = false;
    }
    static void *lpc32xx_dmamux_reserve(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct platform_device *pdev = of_find_device_by_node(ofdma.of_node);
    struct device *dev = &pdev.dev;
    struct lpc32xx_dmamux_data *dmamux = platform_get_drvdata(pdev);
    unsigned long flags;
    struct lpc32xx_dmamux *mux = core::ptr::null_mut();
    let mut ret: c_int = -EINVAL;
    int i;
    if (dma_spec.args_count != 3) {
    dev_err(&pdev.dev, "invalid number of dma mux args\n");
    goto err_put_pdev;
    }
    for (i = 0; i < ARRAY_SIZE(lpc32xx_muxes); i++) {
    if (lpc32xx_muxes[i].signal == dma_spec.args[0]) {
    mux = &lpc32xx_muxes[i];
    break;
    }
    }
    if (!mux) {
    dev_err(&pdev.dev, "invalid mux request number: %d\n",
    dma_spec.args[0]);
    goto err_put_pdev;
    }
    if (dma_spec.args[2] > 1) {
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
    if (mux.busy) {
    spin_unlock_irqrestore(&dmamux.lock, flags);
    dev_err(dev, "dma request signal %d busy, routed to %s\n",
    mux.signal, mux.muxval ? mux.name_sel1 : mux.name_sel1);
    of_node_put(dma_spec.np);
    ret = -EBUSY;
    goto err_put_pdev;
    }
    mux.busy = true;
    mux.muxval = dma_spec.args[2] ? BIT(mux.bit) : 0;
    regmap_update_bits(dmamux.reg, mux.muxreg, BIT(mux.bit), mux.muxval);
    spin_unlock_irqrestore(&dmamux.lock, flags);
    dma_spec.args[2] = 0;
    dma_spec.args_count = 2;
    dev_dbg(dev, "dma request signal %d routed to %s\n",
    mux.signal, mux.muxval ? mux.name_sel1 : mux.name_sel1);
    put_device(&pdev.dev);
    return mux;
    err_put_pdev:
    put_device(&pdev.dev);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_dmamux_probe(pdev: *mut platform_device) -> c_int {
    static int lpc32xx_dmamux_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct lpc32xx_dmamux_data *dmamux;
    dmamux = devm_kzalloc(&pdev.dev, sizeof(*dmamux), GFP_KERNEL);
    if (!dmamux)
    return -ENOMEM;
    dmamux.reg = syscon_node_to_regmap(np.parent);
    if (IS_ERR(dmamux.reg)) {
    dev_err(&pdev.dev, "syscon lookup failed\n");
    return PTR_ERR(dmamux.reg);
    }
    spin_lock_init(&dmamux.lock);
    platform_set_drvdata(pdev, dmamux);
    dmamux.dmarouter.dev = &pdev.dev;
    dmamux.dmarouter.route_free = lpc32xx_dmamux_release;
    return of_dma_router_register(np, lpc32xx_dmamux_reserve,
    &dmamux.dmarouter);
    }
    static const struct of_device_id lpc32xx_dmamux_match[] = {
    { .compatible = "nxp,lpc3220-dmamux" },
    {},
    };
    static struct platform_driver lpc32xx_dmamux_driver = {
    .probe	= lpc32xx_dmamux_probe,
    .driver = {
    .name = "lpc32xx-dmamux",
    .of_match_table = lpc32xx_dmamux_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn lpc32xx_dmamux_init() -> int __init {
    static int __init lpc32xx_dmamux_init(void)
    {
    return platform_driver_register(&lpc32xx_dmamux_driver);
    }
    arch_initcall(lpc32xx_dmamux_init);
