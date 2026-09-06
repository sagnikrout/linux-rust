//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/mtk-efuse.c
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
// Copyright (c) 2015 MediaTek Inc.
// Author: Andrew-CT Chen <andrew-ct.chen@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_efuse_pdata {
    pub uses_post_processing: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_efuse_priv {
    pub base: *mut void __iomem,
}

    static int mtk_reg_read(void *context,
    unsigned int reg, void *_val, size_t bytes)
    {
    struct mtk_efuse_priv *priv = context;
    void __iomem *addr = priv.base + reg;
    u8 *val = _val;
    int i;
    for (i = 0; i < bytes; i++, val++)
// val = readb(addr + i);
    return 0;
    }
    static int mtk_efuse_gpu_speedbin_pp(void *context, const char *id, int index,
    unsigned int offset, void *data, size_t bytes)
    {
    u8 *val = data;
    if (val[0] < 8)
    val[0] = BIT(val[0]);
    return 0;
    }
    static void mtk_efuse_fixup_dt_cell_info(struct nvmem_device *nvmem,
    struct nvmem_cell_info *cell)
    {
    let mut sz: usize = strlen(cell.name);
//
// On some SoCs, the GPU speedbin is not read as bitmask but as
// a number with range [0-7] (max 3 bits): post process to use
// it in OPP tables to describe supported-hw.
//
    if (cell.nbits <= 3 &&
    strncmp(cell.name, "gpu-speedbin", min(sz, strlen("gpu-speedbin"))) == 0)
    cell.read_post_process = mtk_efuse_gpu_speedbin_pp;
    }
#[no_mangle]
unsafe extern "C" fn mtk_efuse_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_efuse_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct resource *res;
    struct nvmem_device *nvmem;
    let mut econfig: nvmem_config = {};
    struct mtk_efuse_priv *priv;
    const struct mtk_efuse_pdata *pdata;
    struct platform_device *socinfo;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    pdata = device_get_match_data(dev);
    econfig.add_legacy_fixed_of_cells = true;
    econfig.stride = 1;
    econfig.word_size = 1;
    econfig.reg_read = mtk_reg_read;
    econfig.size = resource_size(res);
    econfig.priv = priv;
    econfig.dev = dev;
    if (pdata.uses_post_processing)
    econfig.fixup_dt_cell_info = &mtk_efuse_fixup_dt_cell_info;
    nvmem = devm_nvmem_register(dev, &econfig);
    if (IS_ERR(nvmem))
    return PTR_ERR(nvmem);
    socinfo = platform_device_register_data(&pdev.dev, "mtk-socinfo",
    PLATFORM_DEVID_AUTO, core::ptr::null_mut(), 0);
    if (IS_ERR(socinfo))
    dev_info(dev, "MediaTek SoC Information will be unavailable\n");
    platform_set_drvdata(pdev, socinfo);
    return 0;
    }
    static const struct mtk_efuse_pdata mtk_mt8186_efuse_pdata = {
    .uses_post_processing = true,
    };
    static const struct mtk_efuse_pdata mtk_efuse_pdata = {
    .uses_post_processing = false,
    };
    static const struct of_device_id mtk_efuse_of_match[] = {
    { .compatible = "mediatek,mt8173-efuse", .data = &mtk_efuse_pdata },
    { .compatible = "mediatek,mt8186-efuse", .data = &mtk_mt8186_efuse_pdata },
    { .compatible = "mediatek,efuse", .data = &mtk_efuse_pdata },
    {/* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, mtk_efuse_of_match);
#[no_mangle]
unsafe extern "C" fn mtk_efuse_remove(pdev: *mut platform_device) {
    static void mtk_efuse_remove(struct platform_device *pdev)
    {
    struct platform_device *socinfo = platform_get_drvdata(pdev);
    if (!IS_ERR_OR_NULL(socinfo))
    platform_device_unregister(socinfo);
    }
    static struct platform_driver mtk_efuse_driver = {
    .probe = mtk_efuse_probe,
    .remove = mtk_efuse_remove,
    .driver = {
    .name = "mediatek,efuse",
    .of_match_table = mtk_efuse_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn mtk_efuse_init() -> int __init {
    static int __init mtk_efuse_init(void)
    {
    int ret;
    ret = platform_driver_register(&mtk_efuse_driver);
    if (ret) {
    pr_err("Failed to register efuse driver\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_efuse_exit() -> void __exit {
    static void __exit mtk_efuse_exit(void)
    {
    return platform_driver_unregister(&mtk_efuse_driver);
    }
    subsys_initcall(mtk_efuse_init);
    module_exit(mtk_efuse_exit);
    MODULE_AUTHOR("Andrew-CT Chen <andrew-ct.chen@mediatek.com>");
    MODULE_DESCRIPTION("Mediatek EFUSE driver");
    MODULE_LICENSE("GPL v2");
