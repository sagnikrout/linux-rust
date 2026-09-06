//! Automatically rewritten from C to Rust
//! Source: drivers/memory/pl172.c
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
// Memory controller driver for ARM PrimeCell PL172
// PrimeCell MultiPort Memory Controller (PL172)
//
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//
// Based on:
// TI AEMIF driver, Copyright (C) 2010 - 2013 Texas Instruments Inc.
//

pub const MPMC_STATIC_CFG_MW_8BIT: c_uint = 0x0;
pub const MPMC_STATIC_CFG_MW_16BIT: c_uint = 0x1;
pub const MPMC_STATIC_CFG_MW_32BIT: c_uint = 0x2;

pub const MPMC_STATIC_WAIT_WEN_MAX: c_uint = 0x0f;

pub const MPMC_STATIC_WAIT_OEN_MAX: c_uint = 0x0f;

pub const MPMC_STATIC_WAIT_RD_MAX: c_uint = 0x1f;

pub const MPMC_STATIC_WAIT_PAGE_MAX: c_uint = 0x1f;

pub const MPMC_STATIC_WAIT_WR_MAX: c_uint = 0x1f;

pub const MPMC_STATIC_WAIT_TURN_MAX: c_uint = 0x0f;
// Maximum number of static chip selects
pub const PL172_MAX_CS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pl172_data {
    pub base: *mut void __iomem,
    pub rate: c_ulong,
    pub clk: *mut clk,
}

    static int pl172_timing_prop(struct amba_device *adev,
    const struct device_node *np, const char *name,
    u32 reg_offset, u32 max, int start)
    {
    struct pl172_data *pl172 = amba_get_drvdata(adev);
    int cycles;
    u32 val;
    if (!of_property_read_u32(np, name, &val)) {
    cycles = DIV_ROUND_UP(val * pl172.rate, NSEC_PER_MSEC) - start;
    if (cycles < 0) {
    cycles = 0;
    } else if (cycles > max) {
    dev_err(&adev.dev, "%s timing too tight\n", name);
    return -EINVAL;
    }
    writel(cycles, pl172.base + reg_offset);
    }
    dev_dbg(&adev.dev, "%s: %u cycle(s)\n", name, start +
    readl(pl172.base + reg_offset));
    return 0;
    }
    static int pl172_setup_static(struct amba_device *adev,
    struct device_node *np, u32 cs)
    {
    struct pl172_data *pl172 = amba_get_drvdata(adev);
    u32 cfg;
    int ret;
// MPMC static memory configuration
    if (!of_property_read_u32(np, "mpmc,memory-width", &cfg)) {
    if (cfg == 8) {
    cfg = MPMC_STATIC_CFG_MW_8BIT;
    } else if (cfg == 16) {
    cfg = MPMC_STATIC_CFG_MW_16BIT;
    } else if (cfg == 32) {
    cfg = MPMC_STATIC_CFG_MW_32BIT;
    } else {
    dev_err(&adev.dev, "invalid memory width cs%u\n", cs);
    return -EINVAL;
    }
    } else {
    dev_err(&adev.dev, "memory-width property required\n");
    return -EINVAL;
    }
    if (of_property_read_bool(np, "mpmc,async-page-mode"))
    cfg |= MPMC_STATIC_CFG_PM;
    if (of_property_read_bool(np, "mpmc,cs-active-high"))
    cfg |= MPMC_STATIC_CFG_PC;
    if (of_property_read_bool(np, "mpmc,byte-lane-low"))
    cfg |= MPMC_STATIC_CFG_PB;
    if (of_property_read_bool(np, "mpmc,extended-wait"))
    cfg |= MPMC_STATIC_CFG_EW;
    if (amba_part(adev) == 0x172 &&
    of_property_read_bool(np, "mpmc,buffer-enable"))
    cfg |= MPMC_STATIC_CFG_B;
    if (of_property_read_bool(np, "mpmc,write-protect"))
    cfg |= MPMC_STATIC_CFG_P;
    writel(cfg, pl172.base + MPMC_STATIC_CFG(cs));
    dev_dbg(&adev.dev, "mpmc static config cs%u: 0x%08x\n", cs, cfg);
// MPMC static memory timing
    ret = pl172_timing_prop(adev, np, "mpmc,write-enable-delay",
    MPMC_STATIC_WAIT_WEN(cs),
    MPMC_STATIC_WAIT_WEN_MAX, 1);
    if (ret)
    goto fail;
    ret = pl172_timing_prop(adev, np, "mpmc,output-enable-delay",
    MPMC_STATIC_WAIT_OEN(cs),
    MPMC_STATIC_WAIT_OEN_MAX, 0);
    if (ret)
    goto fail;
    ret = pl172_timing_prop(adev, np, "mpmc,read-access-delay",
    MPMC_STATIC_WAIT_RD(cs),
    MPMC_STATIC_WAIT_RD_MAX, 1);
    if (ret)
    goto fail;
    ret = pl172_timing_prop(adev, np, "mpmc,page-mode-read-delay",
    MPMC_STATIC_WAIT_PAGE(cs),
    MPMC_STATIC_WAIT_PAGE_MAX, 1);
    if (ret)
    goto fail;
    ret = pl172_timing_prop(adev, np, "mpmc,write-access-delay",
    MPMC_STATIC_WAIT_WR(cs),
    MPMC_STATIC_WAIT_WR_MAX, 2);
    if (ret)
    goto fail;
    ret = pl172_timing_prop(adev, np, "mpmc,turn-round-delay",
    MPMC_STATIC_WAIT_TURN(cs),
    MPMC_STATIC_WAIT_TURN_MAX, 1);
    if (ret)
    goto fail;
    return 0;
    fail:
    dev_err(&adev.dev, "failed to configure cs%u\n", cs);
    return ret;
    }
    static int pl172_parse_cs_config(struct amba_device *adev,
    struct device_node *np)
    {
    u32 cs;
    if (!of_property_read_u32(np, "mpmc,cs", &cs)) {
    if (cs >= PL172_MAX_CS) {
    dev_err(&adev.dev, "cs%u invalid\n", cs);
    return -EINVAL;
    }
    return pl172_setup_static(adev, np, cs);
    }
    dev_err(&adev.dev, "cs property required\n");
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn pl172_amba_release_regions(data: *mut c_void) {
    static void pl172_amba_release_regions(void *data)
    {
    struct amba_device *adev = data;
    amba_release_regions(adev);
    }
    static const char * const pl172_revisions[] = {"r1", "r2", "r2p3", "r2p4"};
    static const char * const pl175_revisions[] = {"r1"};
    static const char * const pl176_revisions[] = {"r0"};
#[no_mangle]
unsafe extern "C" fn pl172_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int pl172_probe(struct amba_device *adev, const struct amba_id *id)
    {
    struct device_node *child_np, *np = adev.dev.of_node;
    struct device *dev = &adev.dev;
    static const char *rev = "?";
    struct pl172_data *pl172;
    int ret;
    if (amba_part(adev) == 0x172) {
    if (amba_rev(adev) < ARRAY_SIZE(pl172_revisions))
    rev = pl172_revisions[amba_rev(adev)];
    } else if (amba_part(adev) == 0x175) {
    if (amba_rev(adev) < ARRAY_SIZE(pl175_revisions))
    rev = pl175_revisions[amba_rev(adev)];
    } else if (amba_part(adev) == 0x176) {
    if (amba_rev(adev) < ARRAY_SIZE(pl176_revisions))
    rev = pl176_revisions[amba_rev(adev)];
    }
    dev_info(dev, "ARM PL%x revision %s\n", amba_part(adev), rev);
    pl172 = devm_kzalloc(dev, sizeof(*pl172), GFP_KERNEL);
    if (!pl172)
    return -ENOMEM;
    pl172.clk = devm_clk_get_enabled(dev, "mpmcclk");
    if (IS_ERR(pl172.clk))
    return dev_err_probe(dev, PTR_ERR(pl172.clk),
    "no mpmcclk provided clock\n");
    pl172.rate = clk_get_rate(pl172.clk) / MSEC_PER_SEC;
    if (!pl172.rate)
    return dev_err_probe(dev, -EINVAL,
    "unable to get mpmcclk clock rate\n");
    ret = amba_request_regions(adev, core::ptr::null_mut());
    if (ret) {
    dev_err(dev, "unable to request AMBA regions\n");
    return ret;
    }
    ret = devm_add_action_or_reset(dev, pl172_amba_release_regions, adev);
    if (ret)
    return ret;
    pl172.base = devm_ioremap(dev, adev.res.start,
    resource_size(&adev.res));
    if (!pl172.base)
    return dev_err_probe(dev, -ENOMEM, "ioremap failed\n");
    amba_set_drvdata(adev, pl172);
//
// Loop through each child node, which represent a chip select, and
// configure parameters and timing. If successful; populate devices
// under that node.
//
    for_each_available_child_of_node(np, child_np) {
    ret = pl172_parse_cs_config(adev, child_np);
    if (ret)
    continue;
    of_platform_populate(child_np, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    }
    return 0;
    }
    static const struct amba_id pl172_ids[] = {
// PrimeCell MPMC PL172, EMC found on NXP LPC18xx and LPC43xx
    {
    .id	= 0x07041172,
    .mask	= 0x3f0fffff,
    },
// PrimeCell MPMC PL175, EMC found on NXP LPC32xx
    {
    .id	= 0x07041175,
    .mask	= 0x3f0fffff,
    },
// PrimeCell MPMC PL176
    {
    .id	= 0x89041176,
    .mask	= 0xff0fffff,
    },
    { 0, 0 },
    };
    MODULE_DEVICE_TABLE(amba, pl172_ids);
    static struct amba_driver pl172_driver = {
    .drv = {
    .name	= "memory-pl172",
    },
    .probe		= pl172_probe,
    .id_table	= pl172_ids,
    };
    module_amba_driver(pl172_driver);
    MODULE_AUTHOR("Joachim Eastwood <manabian@gmail.com>");
    MODULE_DESCRIPTION("PL172 Memory Controller Driver");
    MODULE_LICENSE("GPL v2");
