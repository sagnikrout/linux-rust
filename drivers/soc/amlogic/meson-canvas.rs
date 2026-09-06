//! Automatically rewritten from C to Rust
//! Source: drivers/soc/amlogic/meson-canvas.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2018 BayLibre, SAS
// Copyright (C) 2015 Amlogic, Inc. All rights reserved.
// Copyright (C) 2014 Endless Mobile
//

pub const NUM_CANVAS: c_int = 256;
// DMC Registers
pub const DMC_CAV_LUT_DATAL: c_uint = 0x00;
pub const CANVAS_WIDTH_LBIT: c_int = 29;
pub const CANVAS_WIDTH_LWID: c_int = 3;
pub const DMC_CAV_LUT_DATAH: c_uint = 0x04;
pub const CANVAS_WIDTH_HBIT: c_int = 0;
pub const CANVAS_HEIGHT_BIT: c_int = 9;
pub const CANVAS_WRAP_BIT: c_int = 22;
pub const CANVAS_BLKMODE_BIT: c_int = 24;
pub const CANVAS_ENDIAN_BIT: c_int = 26;
pub const DMC_CAV_LUT_ADDR: c_uint = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_canvas {
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub /: *mut *mut spinlock_t lock; / canvas device lock,
    pub used: [u8; NUM_CANVAS],
    pub supports_endianness: bool,
}

#[no_mangle]
unsafe extern "C" fn canvas_write(canvas: *mut meson_canvas, reg: u32, val: u32) {
    static void canvas_write(struct meson_canvas *canvas, u32 reg, u32 val)
    {
    writel_relaxed(val, canvas.reg_base + reg);
    }
#[no_mangle]
unsafe extern "C" fn canvas_read(canvas: *mut meson_canvas, reg: u32) -> u32 {
    static u32 canvas_read(struct meson_canvas *canvas, u32 reg)
    {
    return readl_relaxed(canvas.reg_base + reg);
    }
    struct meson_canvas *meson_canvas_get(struct device *dev)
    {
    struct device_node *canvas_node;
    struct platform_device *canvas_pdev;
    struct meson_canvas *canvas;
    canvas_node = of_parse_phandle(dev.of_node, "amlogic,canvas", 0);
    if (!canvas_node)
    return ERR_PTR(-ENODEV);
    canvas_pdev = of_find_device_by_node(canvas_node);
    of_node_put(canvas_node);
    if (!canvas_pdev)
    return ERR_PTR(-EPROBE_DEFER);
//
// If priv is NULL, it's probably because the canvas hasn't
// properly initialized. Bail out with -EINVAL because, in the
// current state, this driver probe cannot return -EPROBE_DEFER
//
    canvas = dev_get_drvdata(&canvas_pdev.dev);
    put_device(&canvas_pdev.dev);
    if (!canvas)
    return ERR_PTR(-EINVAL);
    return canvas;
    }
    EXPORT_SYMBOL_GPL(meson_canvas_get);
    int meson_canvas_config(struct meson_canvas *canvas, u8 canvas_index,
    u32 addr, u32 stride, u32 height,
    unsigned int wrap,
    unsigned int blkmode,
    unsigned int endian)
    {
    unsigned long flags;
    if (endian && !canvas.supports_endianness) {
    dev_err(canvas.dev,
    "Endianness is not supported on this SoC\n");
    return -EINVAL;
    }
    spin_lock_irqsave(&canvas.lock, flags);
    if (!canvas.used[canvas_index]) {
    dev_err(canvas.dev,
    "Trying to setup non allocated canvas %u\n",
    canvas_index);
    spin_unlock_irqrestore(&canvas.lock, flags);
    return -EINVAL;
    }
    canvas_write(canvas, DMC_CAV_LUT_DATAL,
    ((addr + 7) >> 3) |
    (((stride + 7) >> 3) << CANVAS_WIDTH_LBIT));
    canvas_write(canvas, DMC_CAV_LUT_DATAH,
    ((((stride + 7) >> 3) >> CANVAS_WIDTH_LWID) <<
    CANVAS_WIDTH_HBIT) |
    (height << CANVAS_HEIGHT_BIT) |
    (wrap << CANVAS_WRAP_BIT) |
    (blkmode << CANVAS_BLKMODE_BIT) |
    (endian << CANVAS_ENDIAN_BIT));
    canvas_write(canvas, DMC_CAV_LUT_ADDR,
    CANVAS_LUT_WR_EN | canvas_index);
// Force a read-back to make sure everything is flushed.
    canvas_read(canvas, DMC_CAV_LUT_DATAH);
    spin_unlock_irqrestore(&canvas.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(meson_canvas_config);
#[no_mangle]
pub unsafe extern "C" fn meson_canvas_alloc(canvas: *mut meson_canvas, canvas_index: *mut u8) -> c_int {
    int meson_canvas_alloc(struct meson_canvas *canvas, u8 *canvas_index)
    {
    int i;
    unsigned long flags;
    spin_lock_irqsave(&canvas.lock, flags);
    for (i = 0; i < NUM_CANVAS; ++i) {
    if (!canvas.used[i]) {
    canvas.used[i] = 1;
    spin_unlock_irqrestore(&canvas.lock, flags);
// canvas_index = i;
    return 0;
    }
    }
    spin_unlock_irqrestore(&canvas.lock, flags);
    dev_err(canvas.dev, "No more canvas available\n");
    return -ENODEV;
    }
    EXPORT_SYMBOL_GPL(meson_canvas_alloc);
#[no_mangle]
pub unsafe extern "C" fn meson_canvas_free(canvas: *mut meson_canvas, canvas_index: u8) -> c_int {
    int meson_canvas_free(struct meson_canvas *canvas, u8 canvas_index)
    {
    unsigned long flags;
    spin_lock_irqsave(&canvas.lock, flags);
    if (!canvas.used[canvas_index]) {
    dev_err(canvas.dev,
    "Trying to free unused canvas %u\n", canvas_index);
    spin_unlock_irqrestore(&canvas.lock, flags);
    return -EINVAL;
    }
    canvas.used[canvas_index] = 0;
    spin_unlock_irqrestore(&canvas.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(meson_canvas_free);
#[no_mangle]
unsafe extern "C" fn meson_canvas_probe(pdev: *mut platform_device) -> c_int {
    static int meson_canvas_probe(struct platform_device *pdev)
    {
    struct meson_canvas *canvas;
    struct device *dev = &pdev.dev;
    canvas = devm_kzalloc(dev, sizeof(*canvas), GFP_KERNEL);
    if (!canvas)
    return -ENOMEM;
    canvas.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(canvas.reg_base))
    return PTR_ERR(canvas.reg_base);
    canvas.supports_endianness = of_device_get_match_data(dev);
    canvas.dev = dev;
    spin_lock_init(&canvas.lock);
    dev_set_drvdata(dev, canvas);
    return 0;
    }
    static const struct of_device_id canvas_dt_match[] = {
    { .compatible = "amlogic,meson8-canvas", .data = (void *)false, },
    { .compatible = "amlogic,meson8b-canvas", .data = (void *)false, },
    { .compatible = "amlogic,meson8m2-canvas", .data = (void *)false, },
    { .compatible = "amlogic,canvas", .data = (void *)true, },
    {}
    };
    MODULE_DEVICE_TABLE(of, canvas_dt_match);
    static struct platform_driver meson_canvas_driver = {
    .probe = meson_canvas_probe,
    .driver = {
    .name = "amlogic-canvas",
    .of_match_table = canvas_dt_match,
    },
    };
    module_platform_driver(meson_canvas_driver);
    MODULE_DESCRIPTION("Amlogic Canvas driver");
    MODULE_AUTHOR("Maxime Jourdan <mjourdan@baylibre.com>");
    MODULE_LICENSE("GPL");
