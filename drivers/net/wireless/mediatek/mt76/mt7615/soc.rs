//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt7615/soc.c
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2019 MediaTek Inc.
//
// Author: Ryder Lee <ryder.lee@mediatek.com>
// Felix Fietkau <nbd@nbd.name>
//

#[no_mangle]
pub unsafe extern "C" fn mt7622_wmac_init(dev: *mut mt7615_dev) -> c_int {
    int mt7622_wmac_init(struct mt7615_dev *dev)
    {
    struct device_node *np = dev.mt76.dev.of_node;
    if (!is_mt7622(&dev.mt76))
    return 0;
    dev.infracfg = syscon_regmap_lookup_by_phandle(np, "mediatek,infracfg");
    if (IS_ERR(dev.infracfg)) {
    dev_err(dev.mt76.dev, "Cannot find infracfg controller\n");
    return PTR_ERR(dev.infracfg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt7622_wmac_probe(pdev: *mut platform_device) -> c_int {
    static int mt7622_wmac_probe(struct platform_device *pdev)
    {
    void __iomem *mem_base;
    int irq;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    mem_base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(mem_base))
    return PTR_ERR(mem_base);
    return mt7615_mmio_probe(&pdev.dev, mem_base, irq, mt7615e_reg_map);
    }
#[no_mangle]
unsafe extern "C" fn mt7622_wmac_remove(pdev: *mut platform_device) {
    static void mt7622_wmac_remove(struct platform_device *pdev)
    {
    struct mt7615_dev *dev = platform_get_drvdata(pdev);
    mt7615_unregister_device(dev);
    }
    static const struct of_device_id mt7622_wmac_of_match[] = {
    { .compatible = "mediatek,mt7622-wmac" },
    {},
    };
    struct platform_driver mt7622_wmac_driver = {
    .driver = {
    .name = "mt7622-wmac",
    .of_match_table = mt7622_wmac_of_match,
    },
    .probe = mt7622_wmac_probe,
    .remove = mt7622_wmac_remove,
    };
    MODULE_FIRMWARE(MT7622_FIRMWARE_N9);
    MODULE_FIRMWARE(MT7622_ROM_PATCH);
