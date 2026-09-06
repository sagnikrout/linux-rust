//! Automatically rewritten from C to Rust
//! Source: drivers/dma/cv1800b-dmamux.c
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
// Copyright (C) 2025 Inochi Amaoto <inochiama@gmail.com>
//

pub const REG_DMA_CHANNEL_REMAP0: c_uint = 0x154;
pub const REG_DMA_CHANNEL_REMAP1: c_uint = 0x158;
pub const REG_DMA_INT_MUX: c_uint = 0x298;
pub const DMAMUX_NCELLS: c_int = 2;
pub const MAX_DMA_MAPPING_ID: c_int = 42;
pub const MAX_DMA_CPU_ID: c_int = 2;
pub const MAX_DMA_CH_ID: c_int = 7;
pub const DMAMUX_INTMUX_REGISTER_LEN: c_int = 4;
pub const DMAMUX_NR_CH_PER_REGISTER: c_int = 4;
pub const DMAMUX_BIT_PER_CH: c_int = 8;

pub const DMAMUX_INT_BIT_PER_CPU: c_int = 10;

    ((chid) / DMAMUX_NR_CH_PER_REGISTER)

    ((chid) % DMAMUX_NR_CH_PER_REGISTER)

    ((DMAMUX_CH_REGPOS(chid) * sizeof(u32)) + \
    REG_DMA_CHANNEL_REMAP0)

    (((val) << (DMAMUX_CH_REGOFF(chid) * DMAMUX_BIT_PER_CH)) | \
    DMAMUX_CH_UPDATE_BIT)

    DMAMUX_CH_SET(chid, DMAMUX_CH_MASk)

    BIT((cpuid) * DMAMUX_INT_BIT_PER_CPU + (chid))

    DMAMUX_INT_BIT(8, cpuid)

    (DMAMUX_INT_BIT(chid, cpuid) | DMAMUX_INTEN_BIT(cpuid))

    (DMAMUX_INT_BIT(chid, 0) | \
    DMAMUX_INT_BIT(chid, 1) | \
    DMAMUX_INT_BIT(chid, 2))

    (DMAMUX_INT_MASK(chid) | DMAMUX_INTEN_BIT(cpuid))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_dmamux_data {
    pub dmarouter: dma_router,
    pub regmap: *mut regmap,
    pub lock: spinlock_t,
    pub free_maps: llist_head,
    pub reserve_maps: llist_head,
    pub MAX_DMA_MAPPING_ID): DECLARE_BITMAP(mapped_peripherals,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_dmamux_map {
    pub node: llist_node,
    pub channel: c_uint,
    pub peripheral: c_uint,
    pub cpu: c_uint,
}

#[no_mangle]
unsafe extern "C" fn cv1800_dmamux_free(dev: *mut device, route_data: *mut c_void) {
    static void cv1800_dmamux_free(struct device *dev, void *route_data)
    {
    struct cv1800_dmamux_data *dmamux = dev_get_drvdata(dev);
    struct cv1800_dmamux_map *map = route_data;
    guard(spinlock_irqsave)(&dmamux.lock);
    regmap_update_bits(dmamux.regmap,
    DMAMUX_CH_REG(map.channel),
    DMAMUX_CH_MASK(map.channel),
    DMAMUX_CH_UPDATE_BIT);
    regmap_update_bits(dmamux.regmap, REG_DMA_INT_MUX,
    DMAMUX_INT_CH_MASK(map.channel, map.cpu),
    DMAMUX_INTEN_BIT(map.cpu));
    dev_dbg(dev, "free channel %u for req %u (cpu %u)\n",
    map.channel, map.peripheral, map.cpu);
    }
    static void *cv1800_dmamux_route_allocate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct platform_device *pdev = of_find_device_by_node(ofdma.of_node);
    struct cv1800_dmamux_data *dmamux = platform_get_drvdata(pdev);
    struct cv1800_dmamux_map *map;
    struct llist_node *node;
    unsigned long flags;
    unsigned int chid, devid, cpuid;
    let mut ret: c_int = -EINVAL;
    if (dma_spec.args_count != DMAMUX_NCELLS) {
    dev_err(&pdev.dev, "invalid number of dma mux args\n");
    goto err_put_pdev;
    }
    devid = dma_spec.args[0];
    cpuid = dma_spec.args[1];
    dma_spec.args_count = 1;
    if (devid > MAX_DMA_MAPPING_ID) {
    dev_err(&pdev.dev, "invalid device id: %u\n", devid);
    goto err_put_pdev;
    }
    if (cpuid > MAX_DMA_CPU_ID) {
    dev_err(&pdev.dev, "invalid cpu id: %u\n", cpuid);
    goto err_put_pdev;
    }
    dma_spec.np = of_parse_phandle(ofdma.of_node, "dma-masters", 0);
    if (!dma_spec.np) {
    dev_err(&pdev.dev, "can't get dma master\n");
    goto err_put_pdev;
    }
    spin_lock_irqsave(&dmamux.lock, flags);
    if (test_bit(devid, dmamux.mapped_peripherals)) {
    llist_for_each_entry(map, dmamux.reserve_maps.first, node) {
    if (map.peripheral == devid && map.cpu == cpuid)
    goto found;
    }
    goto failed;
    } else {
    node = llist_del_first(&dmamux.free_maps);
    if (!node) {
    ret = -ENODEV;
    goto failed;
    }
    map = llist_entry(node, struct cv1800_dmamux_map, node);
    llist_add(&map.node, &dmamux.reserve_maps);
    set_bit(devid, dmamux.mapped_peripherals);
    }
    found:
    chid = map.channel;
    map.peripheral = devid;
    map.cpu = cpuid;
    regmap_set_bits(dmamux.regmap,
    DMAMUX_CH_REG(chid),
    DMAMUX_CH_SET(chid, devid));
    regmap_update_bits(dmamux.regmap, REG_DMA_INT_MUX,
    DMAMUX_INT_CH_MASK(chid, cpuid),
    DMAMUX_INT_CH_BIT(chid, cpuid));
    spin_unlock_irqrestore(&dmamux.lock, flags);
    dma_spec.args[0] = chid;
    dev_dbg(&pdev.dev, "register channel %u for req %u (cpu %u)\n",
    chid, devid, cpuid);
    put_device(&pdev.dev);
    return map;
    failed:
    spin_unlock_irqrestore(&dmamux.lock, flags);
    of_node_put(dma_spec.np);
    dev_err(&pdev.dev, "errno %d\n", ret);
    err_put_pdev:
    put_device(&pdev.dev);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn cv1800_dmamux_probe(pdev: *mut platform_device) -> c_int {
    static int cv1800_dmamux_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *mux_node = dev.of_node;
    struct cv1800_dmamux_data *data;
    struct cv1800_dmamux_map *tmp;
    struct device *parent = dev.parent;
    struct regmap *regmap = core::ptr::null_mut();
    unsigned int i;
    if (!parent)
    return -ENODEV;
    regmap = device_node_to_regmap(parent.of_node);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    spin_lock_init(&data.lock);
    init_llist_head(&data.free_maps);
    init_llist_head(&data.reserve_maps);
    for (i = 0; i <= MAX_DMA_CH_ID; i++) {
    tmp = devm_kmalloc(dev, sizeof(*tmp), GFP_KERNEL);
    if (!tmp) {
// It is OK for not allocating all channel
    dev_warn(dev, "can not allocate channel %u\n", i);
    continue;
    }
    init_llist_node(&tmp.node);
    tmp.channel = i;
    llist_add(&tmp.node, &data.free_maps);
    }
// if no channel is allocated, the probe must fail
    if (llist_empty(&data.free_maps))
    return -ENOMEM;
    data.regmap = regmap;
    data.dmarouter.dev = dev;
    data.dmarouter.route_free = cv1800_dmamux_free;
    platform_set_drvdata(pdev, data);
    return of_dma_router_register(mux_node,
    cv1800_dmamux_route_allocate,
    &data.dmarouter);
    }
#[no_mangle]
unsafe extern "C" fn cv1800_dmamux_remove(pdev: *mut platform_device) {
    static void cv1800_dmamux_remove(struct platform_device *pdev)
    {
    of_dma_controller_free(pdev.dev.of_node);
    }
    static const struct of_device_id cv1800_dmamux_ids[] = {
    { .compatible = "sophgo,cv1800b-dmamux", },
    { }
    };
    MODULE_DEVICE_TABLE(of, cv1800_dmamux_ids);
    static struct platform_driver cv1800_dmamux_driver = {
    .probe = cv1800_dmamux_probe,
    .remove = cv1800_dmamux_remove,
    .driver = {
    .name = "cv1800-dmamux",
    .of_match_table = cv1800_dmamux_ids,
    },
    };
    module_platform_driver(cv1800_dmamux_driver);
    MODULE_AUTHOR("Inochi Amaoto <inochiama@gmail.com>");
    MODULE_DESCRIPTION("Sophgo CV1800/SG2000 Series SoC DMAMUX driver");
    MODULE_LICENSE("GPL");
