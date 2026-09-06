//! Automatically rewritten from C to Rust
//! Source: sound/soc/loongson/loongson_i2s_pci.c
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
// loongson_i2s_pci.c -- Loongson I2S controller driver
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
// Author: Yingkun Meng <mengyingkun@loongson.cn>
//

    static int loongson_i2s_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *pid)
    {
    const struct fwnode_handle *fwnode = pdev.dev.fwnode;
    struct loongson_idma_data *tx_data, *rx_data;
    struct device *dev = &pdev.dev;
    struct loongson_i2s *i2s;
    int ret;
    if (pcim_enable_device(pdev)) {
    dev_err(dev, "pci_enable_device failed\n");
    return -ENODEV;
    }
    i2s = devm_kzalloc(dev, sizeof(*i2s), GFP_KERNEL);
    if (!i2s)
    return -ENOMEM;
    i2s.rev_id = pdev.revision;
    i2s.dev = dev;
    pci_set_drvdata(pdev, i2s);
    i2s.reg_base = pcim_iomap_region(pdev, 0, DRIVER_NAME);
    if (IS_ERR(i2s.reg_base)) {
    dev_err(dev, "iomap_region failed\n");
    return PTR_ERR(i2s.reg_base);
    }
    i2s.regmap = devm_regmap_init_mmio(dev, i2s.reg_base,
    &loongson_i2s_regmap_config);
    if (IS_ERR(i2s.regmap))
    return dev_err_probe(dev, PTR_ERR(i2s.regmap), "regmap_init_mmio failed\n");
    tx_data = &i2s.tx_dma_data;
    rx_data = &i2s.rx_dma_data;
    tx_data.dev_addr = pci_resource_start(pdev, 0) + LS_I2S_TX_DATA;
    tx_data.order_addr = i2s.reg_base + LS_I2S_TX_ORDER;
    rx_data.dev_addr = pci_resource_start(pdev, 0) + LS_I2S_RX_DATA;
    rx_data.order_addr = i2s.reg_base + LS_I2S_RX_ORDER;
    tx_data.irq = fwnode_irq_get_byname(fwnode, "tx");
    if (tx_data.irq < 0)
    return dev_err_probe(dev, tx_data.irq, "dma tx irq invalid\n");
    rx_data.irq = fwnode_irq_get_byname(fwnode, "rx");
    if (rx_data.irq < 0)
    return dev_err_probe(dev, rx_data.irq, "dma rx irq invalid\n");
    ret = device_property_read_u32(dev, "clock-frequency", &i2s.clk_rate);
    if (ret)
    return dev_err_probe(dev, ret, "clock-frequency property invalid\n");
    dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    if (i2s.rev_id == 1) {
    regmap_write(i2s.regmap, LS_I2S_CTRL, I2S_CTRL_RESET);
    udelay(200);
    }
    ret = devm_snd_soc_register_component(dev, &loongson_i2s_idma_component,
    &loongson_i2s_dai, 1);
    if (ret)
    return dev_err_probe(dev, ret, "register DAI failed\n");
    return 0;
    }
    static const struct pci_device_id loongson_i2s_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_LOONGSON, 0x7a27) },
    { },
    };
    MODULE_DEVICE_TABLE(pci, loongson_i2s_ids);
    static struct pci_driver loongson_i2s_driver = {
    .name = DRIVER_NAME,
    .id_table = loongson_i2s_ids,
    .probe = loongson_i2s_pci_probe,
    .driver = {
    .pm = pm_sleep_ptr(&loongson_i2s_pm),
    },
    };
    module_pci_driver(loongson_i2s_driver);
    MODULE_DESCRIPTION("Loongson I2S Master Mode ASoC Driver");
    MODULE_AUTHOR("Loongson Technology Corporation Limited");
    MODULE_LICENSE("GPL");
