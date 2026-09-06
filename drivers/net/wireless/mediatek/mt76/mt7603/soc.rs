//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt7603/soc.c
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

    static int
    mt76_wmac_probe(struct platform_device *pdev)
    {
    struct mt7603_dev *dev;
    void __iomem *mem_base;
    struct mt76_dev *mdev;
    int irq;
    int ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    mem_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mem_base))
    return PTR_ERR(mem_base);
    mdev = mt76_alloc_device(&pdev.dev, sizeof(*dev), &mt7603_ops,
    &mt7603_drv_ops);
    if (!mdev)
    return -ENOMEM;
    dev = container_of(mdev, struct mt7603_dev, mt76);
    mt76_mmio_init(mdev, mem_base);
    mdev.rev = (mt76_rr(dev, MT_HW_CHIPID) << 16) |
    (mt76_rr(dev, MT_HW_REV) & 0xff);
    dev_info(mdev.dev, "ASIC revision: %04x\n", mdev.rev);
    mt76_wr(dev, MT_INT_MASK_CSR, 0);
    ret = devm_request_irq(mdev.dev, irq, mt7603_irq_handler,
    IRQF_SHARED, KBUILD_MODNAME, dev);
    if (ret)
    goto error;
    ret = mt7603_register_device(dev);
    if (ret)
    goto error;
    return 0;
    error:
    mt76_free_device(mdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt76_wmac_remove(pdev: *mut platform_device) {
    static void mt76_wmac_remove(struct platform_device *pdev)
    {
    struct mt76_dev *mdev = platform_get_drvdata(pdev);
    struct mt7603_dev *dev = container_of(mdev, struct mt7603_dev, mt76);
    mt7603_unregister_device(dev);
    }
    static const struct of_device_id of_wmac_match[] = {
    { .compatible = "mediatek,mt7628-wmac" },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_wmac_match);
    MODULE_FIRMWARE(MT7628_FIRMWARE_E1);
    MODULE_FIRMWARE(MT7628_FIRMWARE_E2);
    struct platform_driver mt76_wmac_driver = {
    .probe		= mt76_wmac_probe,
    .remove		= mt76_wmac_remove,
    .driver = {
    .name = "mt76_wmac",
    .of_match_table = of_wmac_match,
    },
    };
