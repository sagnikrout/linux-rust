//! Automatically rewritten from C to Rust
//! Source: drivers/dma/dw/rzn1-dmamux.c
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
// Copyright (C) 2022 Schneider-Electric
// Author: Miquel Raynal <miquel.raynal@bootlin.com
// Based on TI crossbar driver written by Peter Ujfalusi <peter.ujfalusi@ti.com>
//

pub const RNZ1_DMAMUX_NCELLS: c_int = 6;
pub const RZN1_DMAMUX_MAX_LINES: c_int = 64;
pub const RZN1_DMAMUX_LINES_PER_CTLR: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzn1_dmamux_data {
    pub dmarouter: dma_router,
    pub RZN1_DMAMUX_LINES_PER_CTLR): *mut *mut DECLARE_BITMAP(used_chans, 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzn1_dmamux_map {
    pub req_idx: c_uint,
}

#[no_mangle]
unsafe extern "C" fn rzn1_dmamux_free(dev: *mut device, route_data: *mut c_void) {
    static void rzn1_dmamux_free(struct device *dev, void *route_data)
    {
    struct rzn1_dmamux_data *dmamux = dev_get_drvdata(dev);
    struct rzn1_dmamux_map *map = route_data;
    dev_dbg(dev, "Unmapping DMAMUX request %u\n", map.req_idx);
    clear_bit(map.req_idx, dmamux.used_chans);
    kfree(map);
    }
    static void *rzn1_dmamux_route_allocate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct platform_device *pdev = of_find_device_by_node(ofdma.of_node);
    struct rzn1_dmamux_data *dmamux = platform_get_drvdata(pdev);
    struct rzn1_dmamux_map *map;
    unsigned int dmac_idx, chan, val;
    u32 mask;
    int ret;
    if (dma_spec.args_count != RNZ1_DMAMUX_NCELLS) {
    ret = -EINVAL;
    goto put_device;
    }
    map = kzalloc_obj(*map);
    if (!map) {
    ret = -ENOMEM;
    goto put_device;
    }
    chan = dma_spec.args[0];
    map.req_idx = dma_spec.args[4];
    val = dma_spec.args[5];
    dma_spec.args_count -= 2;
    if (chan >= RZN1_DMAMUX_LINES_PER_CTLR) {
    dev_err(&pdev.dev, "Invalid DMA request line: %u\n", chan);
    ret = -EINVAL;
    goto free_map;
    }
    if (map.req_idx >= RZN1_DMAMUX_MAX_LINES ||
    (map.req_idx % RZN1_DMAMUX_LINES_PER_CTLR) != chan) {
    dev_err(&pdev.dev, "Invalid MUX request line: %u\n", map.req_idx);
    ret = -EINVAL;
    goto free_map;
    }
    dmac_idx = map.req_idx >= RZN1_DMAMUX_LINES_PER_CTLR ? 1 : 0;
    dma_spec.np = of_parse_phandle(ofdma.of_node, "dma-masters", dmac_idx);
    if (!dma_spec.np) {
    dev_err(&pdev.dev, "Can't get DMA master\n");
    ret = -EINVAL;
    goto free_map;
    }
    dev_dbg(&pdev.dev, "Mapping DMAMUX request %u to DMAC%u request %u\n",
    map.req_idx, dmac_idx, chan);
    if (test_and_set_bit(map.req_idx, dmamux.used_chans)) {
    ret = -EBUSY;
    goto put_dma_spec_np;
    }
    mask = BIT(map.req_idx);
    ret = r9a06g032_sysctrl_set_dmamux(mask, val ? mask : 0);
    if (ret)
    goto clear_bitmap;
    put_device(&pdev.dev);
    return map;
    clear_bitmap:
    clear_bit(map.req_idx, dmamux.used_chans);
    put_dma_spec_np:
    of_node_put(dma_spec.np);
    free_map:
    kfree(map);
    put_device:
    put_device(&pdev.dev);
    return ERR_PTR(ret);
    }

    static const struct of_device_id rzn1_dmac_match[] = {
    { .compatible = "renesas,rzn1-dma" },
    {}
    };

#[no_mangle]
unsafe extern "C" fn rzn1_dmamux_probe(pdev: *mut platform_device) -> c_int {
    static int rzn1_dmamux_probe(struct platform_device *pdev)
    {
    struct device_node *mux_node = pdev.dev.of_node;
    const struct of_device_id *match;
    struct device_node *dmac_node;
    struct rzn1_dmamux_data *dmamux;
    dmamux = devm_kzalloc(&pdev.dev, sizeof(*dmamux), GFP_KERNEL);
    if (!dmamux)
    return -ENOMEM;
    dmac_node = of_parse_phandle(mux_node, "dma-masters", 0);
    if (!dmac_node)
    return dev_err_probe(&pdev.dev, -ENODEV, "Can't get DMA master node\n");
    match = of_match_node(rzn1_dmac_match, dmac_node);
    of_node_put(dmac_node);
    if (!match)
    return dev_err_probe(&pdev.dev, -EINVAL, "DMA master is not supported\n");
    dmamux.dmarouter.dev = &pdev.dev;
    dmamux.dmarouter.route_free = rzn1_dmamux_free;
    platform_set_drvdata(pdev, dmamux);
    return of_dma_router_register(mux_node, rzn1_dmamux_route_allocate,
    &dmamux.dmarouter);
    }
    static const struct of_device_id rzn1_dmamux_match[] = {
    { .compatible = "renesas,rzn1-dmamux" },
    {}
    };
    MODULE_DEVICE_TABLE(of, rzn1_dmamux_match);
    static struct platform_driver rzn1_dmamux_driver = {
    .driver = {
    .name = "renesas,rzn1-dmamux",
    .of_match_table = rzn1_dmamux_match,
    },
    .probe	= rzn1_dmamux_probe,
    };
    module_platform_driver(rzn1_dmamux_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Miquel Raynal <miquel.raynal@bootlin.com");
    MODULE_DESCRIPTION("Renesas RZ/N1 DMAMUX driver");
