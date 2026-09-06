//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/meson_mx_ao_arc.c
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
// Copyright (C) 2020 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//

pub const AO_REMAP_REG0: c_uint = 0x0;

pub const AO_REMAP_REG1: c_uint = 0x4;

pub const AO_CPU_CNTL: c_uint = 0x0;

pub const AO_CPU_STAT: c_uint = 0x4;
pub const AO_SECURE_REG0: c_uint = 0x0;

// Only bits [31:20] and [17:14] are usable, all other bits must be zero
pub const MESON_AO_RPROC_SRAM_USABLE_BITS: c_uint = 0xfff3c000ULL;
pub const MESON_AO_RPROC_MEMORY_OFFSET: c_uint = 0x10000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_mx_ao_arc_rproc_priv {
    pub remap_base: *mut void __iomem,
    pub cpu_base: *mut void __iomem,
    pub sram_va: c_ulong,
    pub sram_pa: phys_addr_t,
    pub sram_size: usize,
    pub sram_pool: *mut gen_pool,
    pub arc_reset: *mut reset_control,
    pub arc_pclk: *mut clk,
    pub secbus2_regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn meson_mx_ao_arc_rproc_start(rproc: *mut rproc) -> c_int {
    static int meson_mx_ao_arc_rproc_start(struct rproc *rproc)
    {
    struct meson_mx_ao_arc_rproc_priv *priv = rproc.priv;
    phys_addr_t translated_sram_addr;
    u32 tmp;
    int ret;
    ret = clk_prepare_enable(priv.arc_pclk);
    if (ret)
    return ret;
    tmp = FIELD_PREP(AO_REMAP_REG0_REMAP_AHB_SRAM_BITS_17_14_FOR_ARM_CPU,
    priv.sram_pa >> 14);
    writel(tmp, priv.remap_base + AO_REMAP_REG0);
//
// The SRAM content as seen by the ARC core always starts at 0x0
// regardless of the value given here (this was discovered by trial and
// error). For SoCs older than Meson6 we probably have to set
// AO_REMAP_REG1_MOVE_AHB_SRAM_TO_0X0_INSTEAD_OF_DDR to achieve the
// same. (At least) For Meson8 and newer that bit must not be set.
//
    writel(0x0, priv.remap_base + AO_REMAP_REG1);
    regmap_update_bits(priv.secbus2_regmap, AO_SECURE_REG0,
    AO_SECURE_REG0_AHB_SRAM_BITS_19_12,
    FIELD_PREP(AO_SECURE_REG0_AHB_SRAM_BITS_19_12,
    priv.sram_pa >> 12));
    ret = reset_control_reset(priv.arc_reset);
    if (ret) {
    clk_disable_unprepare(priv.arc_pclk);
    return ret;
    }
    usleep_range(10, 100);
//
// Convert from 0xd9000000 to 0xc9000000 as the vendor driver does.
// This only seems to be relevant for the AO_CPU_CNTL register. It is
// unknown why this is needed.
//
    translated_sram_addr = priv.sram_pa - MESON_AO_RPROC_MEMORY_OFFSET;
    tmp = FIELD_PREP(AO_CPU_CNTL_AHB_SRAM_BITS_31_20,
    translated_sram_addr >> 20);
    tmp |= AO_CPU_CNTL_UNKNONWN | AO_CPU_CNTL_RUN;
    writel(tmp, priv.cpu_base + AO_CPU_CNTL);
    usleep_range(20, 200);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_mx_ao_arc_rproc_stop(rproc: *mut rproc) -> c_int {
    static int meson_mx_ao_arc_rproc_stop(struct rproc *rproc)
    {
    struct meson_mx_ao_arc_rproc_priv *priv = rproc.priv;
    writel(AO_CPU_CNTL_HALT, priv.cpu_base + AO_CPU_CNTL);
    clk_disable_unprepare(priv.arc_pclk);
    return 0;
    }
    static void *meson_mx_ao_arc_rproc_da_to_va(struct rproc *rproc, u64 da,
    size_t len, bool *is_iomem)
    {
    struct meson_mx_ao_arc_rproc_priv *priv = rproc.priv;
// The memory from the ARC core's perspective always starts at 0x0.
    if ((da + len) > priv.sram_size)
    return core::ptr::null_mut();
    return (void *)priv.sram_va + da;
    }
    static struct rproc_ops meson_mx_ao_arc_rproc_ops = {
    .start		= meson_mx_ao_arc_rproc_start,
    .stop		= meson_mx_ao_arc_rproc_stop,
    .da_to_va	= meson_mx_ao_arc_rproc_da_to_va,
    .get_boot_addr	= rproc_elf_get_boot_addr,
    .load		= rproc_elf_load_segments,
    .sanity_check	= rproc_elf_sanity_check,
    };
#[no_mangle]
unsafe extern "C" fn meson_mx_ao_arc_rproc_probe(pdev: *mut platform_device) -> c_int {
    static int meson_mx_ao_arc_rproc_probe(struct platform_device *pdev)
    {
    struct meson_mx_ao_arc_rproc_priv *priv;
    struct device *dev = &pdev.dev;
    const char *fw_name = core::ptr::null_mut();
    struct rproc *rproc;
    int ret;
    device_property_read_string(dev, "firmware-name", &fw_name);
    rproc = devm_rproc_alloc(dev, "meson-mx-ao-arc",
    &meson_mx_ao_arc_rproc_ops, fw_name,
    sizeof(*priv));
    if (!rproc)
    return -ENOMEM;
    rproc.has_iommu = false;
    priv = rproc.priv;
    priv.sram_pool = of_gen_pool_get(dev.of_node, "sram", 0);
    if (!priv.sram_pool) {
    dev_err(dev, "Could not get SRAM pool\n");
    return -ENODEV;
    }
    priv.sram_size = gen_pool_avail(priv.sram_pool);
    priv.sram_va = gen_pool_alloc(priv.sram_pool, priv.sram_size);
    if (!priv.sram_va) {
    dev_err(dev, "Could not alloc memory in SRAM pool\n");
    return -ENOMEM;
    }
    priv.sram_pa = gen_pool_virt_to_phys(priv.sram_pool, priv.sram_va);
    if (priv.sram_pa & ~MESON_AO_RPROC_SRAM_USABLE_BITS) {
    dev_err(dev, "SRAM address contains unusable bits\n");
    ret = -EINVAL;
    goto err_free_genpool;
    }
    priv.secbus2_regmap = syscon_regmap_lookup_by_phandle(dev.of_node,
    "amlogic,secbus2");
    if (IS_ERR(priv.secbus2_regmap)) {
    dev_err(dev, "Failed to find SECBUS2 regmap\n");
    ret = PTR_ERR(priv.secbus2_regmap);
    goto err_free_genpool;
    }
    priv.remap_base = devm_platform_ioremap_resource_byname(pdev, "remap");
    if (IS_ERR(priv.remap_base)) {
    ret = PTR_ERR(priv.remap_base);
    goto err_free_genpool;
    }
    priv.cpu_base = devm_platform_ioremap_resource_byname(pdev, "cpu");
    if (IS_ERR(priv.cpu_base)) {
    ret = PTR_ERR(priv.cpu_base);
    goto err_free_genpool;
    }
    priv.arc_reset = devm_reset_control_get_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(priv.arc_reset)) {
    dev_err(dev, "Failed to get ARC reset\n");
    ret = PTR_ERR(priv.arc_reset);
    goto err_free_genpool;
    }
    priv.arc_pclk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.arc_pclk)) {
    dev_err(dev, "Failed to get the ARC PCLK\n");
    ret = PTR_ERR(priv.arc_pclk);
    goto err_free_genpool;
    }
    platform_set_drvdata(pdev, rproc);
    ret = rproc_add(rproc);
    if (ret)
    goto err_free_genpool;
    return 0;
    err_free_genpool:
    gen_pool_free(priv.sram_pool, priv.sram_va, priv.sram_size);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn meson_mx_ao_arc_rproc_remove(pdev: *mut platform_device) {
    static void meson_mx_ao_arc_rproc_remove(struct platform_device *pdev)
    {
    struct rproc *rproc = platform_get_drvdata(pdev);
    struct meson_mx_ao_arc_rproc_priv *priv = rproc.priv;
    rproc_del(rproc);
    gen_pool_free(priv.sram_pool, priv.sram_va, priv.sram_size);
    }
    static const struct of_device_id meson_mx_ao_arc_rproc_match[] = {
    { .compatible = "amlogic,meson8-ao-arc" },
    { .compatible = "amlogic,meson8b-ao-arc" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, meson_mx_ao_arc_rproc_match);
    static struct platform_driver meson_mx_ao_arc_rproc_driver = {
    .probe = meson_mx_ao_arc_rproc_probe,
    .remove = meson_mx_ao_arc_rproc_remove,
    .driver = {
    .name = "meson-mx-ao-arc-rproc",
    .of_match_table = meson_mx_ao_arc_rproc_match,
    },
    };
    module_platform_driver(meson_mx_ao_arc_rproc_driver);
    MODULE_DESCRIPTION("Amlogic Meson6/8/8b/8m2 AO ARC remote processor driver");
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_LICENSE("GPL v2");
