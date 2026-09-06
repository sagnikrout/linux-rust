//! Automatically rewritten from C to Rust
//! Source: drivers/auxdisplay/img-ascii-lcd.c
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
//
// Copyright (C) 2016 Imagination Technologies
// Author: Paul Burton <paul.burton@mips.com>
//

    struct img_ascii_lcd_ctx;
//
// struct img_ascii_lcd_config - Configuration information about an LCD model
// @num_chars: the number of characters the LCD can display
// @external_regmap: true if registers are in a system controller, else false
// @ops: character line display operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ascii_lcd_config {
    pub num_chars: c_uint,
    pub external_regmap: bool,
    pub ops: linedisp_ops,
}

//
// struct img_ascii_lcd_ctx - Private data structure
// @linedisp: line display structure
// @base: the base address of the LCD registers
// @regmap: the regmap through which LCD registers are accessed
// @offset: the offset within regmap to the start of the LCD registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ascii_lcd_ctx {
    pub linedisp: linedisp,
    union {
    pub base: *mut void __iomem,
    pub regmap: *mut regmap,
}

    u32 offset;
    };
//
// MIPS Boston development board
//
#[no_mangle]
unsafe extern "C" fn boston_update(linedisp: *mut linedisp) {
    static void boston_update(struct linedisp *linedisp)
    {
    struct img_ascii_lcd_ctx *ctx =
    container_of(linedisp, struct img_ascii_lcd_ctx, linedisp);
    ulong val;

    val = *((u64 *)&linedisp.buf[0]);
    __raw_writeq(val, ctx.base);

    val = *((u32 *)&linedisp.buf[0]);
    __raw_writel(val, ctx.base);
    val = *((u32 *)&linedisp.buf[4]);
    __raw_writel(val, ctx.base + 4);

    }
    static const struct img_ascii_lcd_config boston_config = {
    .num_chars = 8,
    .ops = {
    .update = boston_update,
    },
    };
//
// MIPS Malta development board
//
#[no_mangle]
unsafe extern "C" fn malta_update(linedisp: *mut linedisp) {
    static void malta_update(struct linedisp *linedisp)
    {
    struct img_ascii_lcd_ctx *ctx =
    container_of(linedisp, struct img_ascii_lcd_ctx, linedisp);
    unsigned int i;
    let mut err: c_int = 0;
    for (i = 0; i < linedisp.num_chars; i++) {
    err = regmap_write(ctx.regmap,
    ctx.offset + (i * 8), linedisp.buf[i]);
    if (err)
    break;
    }
    if (unlikely(err))
    pr_err_ratelimited("Failed to update LCD display: %d\n", err);
    }
    static const struct img_ascii_lcd_config malta_config = {
    .num_chars = 8,
    .external_regmap = true,
    .ops = {
    .update = malta_update,
    },
    };
//
// MIPS SEAD3 development board
//
    enum {
    SEAD3_REG_LCD_CTRL		= 0x00,

    SEAD3_REG_LCD_DATA		= 0x08,
    SEAD3_REG_CPLD_STATUS		= 0x10,

    SEAD3_REG_CPLD_DATA		= 0x18,

    };
#[no_mangle]
unsafe extern "C" fn sead3_wait_sm_idle(ctx: *mut img_ascii_lcd_ctx) -> c_int {
    static int sead3_wait_sm_idle(struct img_ascii_lcd_ctx *ctx)
    {
    unsigned int status;
    int err;
    do {
    err = regmap_read(ctx.regmap,
    ctx.offset + SEAD3_REG_CPLD_STATUS,
    &status);
    if (err)
    return err;
    } while (status & SEAD3_REG_CPLD_STATUS_BUSY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sead3_wait_lcd_idle(ctx: *mut img_ascii_lcd_ctx) -> c_int {
    static int sead3_wait_lcd_idle(struct img_ascii_lcd_ctx *ctx)
    {
    unsigned int cpld_data;
    int err;
    err = sead3_wait_sm_idle(ctx);
    if (err)
    return err;
    do {
    err = regmap_read(ctx.regmap,
    ctx.offset + SEAD3_REG_LCD_CTRL,
    &cpld_data);
    if (err)
    return err;
    err = sead3_wait_sm_idle(ctx);
    if (err)
    return err;
    err = regmap_read(ctx.regmap,
    ctx.offset + SEAD3_REG_CPLD_DATA,
    &cpld_data);
    if (err)
    return err;
    } while (cpld_data & SEAD3_REG_CPLD_DATA_BUSY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sead3_update(linedisp: *mut linedisp) {
    static void sead3_update(struct linedisp *linedisp)
    {
    struct img_ascii_lcd_ctx *ctx =
    container_of(linedisp, struct img_ascii_lcd_ctx, linedisp);
    unsigned int i;
    let mut err: c_int = 0;
    for (i = 0; i < linedisp.num_chars; i++) {
    err = sead3_wait_lcd_idle(ctx);
    if (err)
    break;
    err = regmap_write(ctx.regmap,
    ctx.offset + SEAD3_REG_LCD_CTRL,
    SEAD3_REG_LCD_CTRL_SETDRAM | i);
    if (err)
    break;
    err = sead3_wait_lcd_idle(ctx);
    if (err)
    break;
    err = regmap_write(ctx.regmap,
    ctx.offset + SEAD3_REG_LCD_DATA,
    linedisp.buf[i]);
    if (err)
    break;
    }
    if (unlikely(err))
    pr_err_ratelimited("Failed to update LCD display: %d\n", err);
    }
    static const struct img_ascii_lcd_config sead3_config = {
    .num_chars = 16,
    .external_regmap = true,
    .ops = {
    .update = sead3_update,
    },
    };
    static const struct of_device_id img_ascii_lcd_matches[] = {
    { .compatible = "img,boston-lcd", .data = &boston_config },
    { .compatible = "mti,malta-lcd", .data = &malta_config },
    { .compatible = "mti,sead3-lcd", .data = &sead3_config },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, img_ascii_lcd_matches);
//
// img_ascii_lcd_probe() - probe an LCD display device
// @pdev: the LCD platform device
//
// Probe an LCD display device, ensuring that we have the required resources in
// order to access the LCD & setting up private data as well as sysfs files.
//
// Return: 0 on success, else -ERRNO
//
#[no_mangle]
unsafe extern "C" fn img_ascii_lcd_probe(pdev: *mut platform_device) -> c_int {
    static int img_ascii_lcd_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct img_ascii_lcd_config *cfg = device_get_match_data(dev);
    struct img_ascii_lcd_ctx *ctx;
    int err;
    ctx = devm_kzalloc(dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    if (cfg.external_regmap) {
    ctx.regmap = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(ctx.regmap))
    return PTR_ERR(ctx.regmap);
    if (of_property_read_u32(dev.of_node, "offset", &ctx.offset))
    return -EINVAL;
    } else {
    ctx.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctx.base))
    return PTR_ERR(ctx.base);
    }
    err = linedisp_register(&ctx.linedisp, dev, cfg.num_chars, &cfg.ops);
    if (err)
    return err;
// for backwards compatibility
    err = compat_only_sysfs_link_entry_to_kobj(&dev.kobj,
    &ctx.linedisp.dev.kobj,
    "message", core::ptr::null_mut());
    if (err)
    goto err_unregister;
    platform_set_drvdata(pdev, ctx);
    return 0;
    err_unregister:
    linedisp_unregister(&ctx.linedisp);
    return err;
    }
//
// img_ascii_lcd_remove() - remove an LCD display device
// @pdev: the LCD platform device
//
// Remove an LCD display device, freeing private resources & ensuring that the
// driver stops using the LCD display registers.
//
#[no_mangle]
unsafe extern "C" fn img_ascii_lcd_remove(pdev: *mut platform_device) {
    static void img_ascii_lcd_remove(struct platform_device *pdev)
    {
    struct img_ascii_lcd_ctx *ctx = platform_get_drvdata(pdev);
    sysfs_remove_link(&pdev.dev.kobj, "message");
    linedisp_unregister(&ctx.linedisp);
    }
    static struct platform_driver img_ascii_lcd_driver = {
    .driver = {
    .name		= "img-ascii-lcd",
    .of_match_table	= img_ascii_lcd_matches,
    },
    .probe	= img_ascii_lcd_probe,
    .remove = img_ascii_lcd_remove,
    };
    module_platform_driver(img_ascii_lcd_driver);
    MODULE_DESCRIPTION("Imagination Technologies ASCII LCD Display");
    MODULE_AUTHOR("Paul Burton <paul.burton@mips.com>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("LINEDISP");
