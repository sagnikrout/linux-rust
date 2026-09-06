//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt7603/pci.c
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

    static const struct pci_device_id mt76pci_device_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_MEDIATEK, 0x7603) },
    { },
    };
    static int
    mt76pci_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct mt7603_dev *dev;
    struct mt76_dev *mdev;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    ret = pcim_iomap_regions(pdev, BIT(0), pci_name(pdev));
    if (ret)
    return ret;
    pci_set_master(pdev);
    ret = dma_set_mask(&pdev.dev, DMA_BIT_MASK(32));
    if (ret)
    return ret;
    mdev = mt76_alloc_device(&pdev.dev, sizeof(*dev), &mt7603_ops,
    &mt7603_drv_ops);
    if (!mdev)
    return -ENOMEM;
    dev = container_of(mdev, struct mt7603_dev, mt76);
    mt76_mmio_init(mdev, pcim_iomap_table(pdev)[0]);
    mdev.rev = (mt76_rr(dev, MT_HW_CHIPID) << 16) |
    (mt76_rr(dev, MT_HW_REV) & 0xff);
    dev_info(mdev.dev, "ASIC revision: %04x\n", mdev.rev);
    mt76_wr(dev, MT_INT_MASK_CSR, 0);
    ret = devm_request_irq(mdev.dev, pdev.irq, mt7603_irq_handler,
    IRQF_SHARED, KBUILD_MODNAME, dev);
    if (ret)
    goto error;
    ret = mt7603_register_device(dev);
    if (ret)
    goto error;
    return 0;
    error:
    mt76_free_device(&dev.mt76);
    return ret;
    }
    static void
    mt76pci_remove(struct pci_dev *pdev)
    {
    struct mt76_dev *mdev = pci_get_drvdata(pdev);
    struct mt7603_dev *dev = container_of(mdev, struct mt7603_dev, mt76);
    mt7603_unregister_device(dev);
    }
    MODULE_DEVICE_TABLE(pci, mt76pci_device_table);
    MODULE_FIRMWARE(MT7603_FIRMWARE_E1);
    MODULE_FIRMWARE(MT7603_FIRMWARE_E2);
    struct pci_driver mt7603_pci_driver = {
    .name		= KBUILD_MODNAME,
    .id_table	= mt76pci_device_table,
    .probe		= mt76pci_probe,
    .remove		= mt76pci_remove,
    };
