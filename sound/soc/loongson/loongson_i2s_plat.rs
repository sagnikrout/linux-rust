//! Automatically rewritten from C to Rust
//! Source: sound/soc/loongson/loongson_i2s_plat.c
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
// Loongson I2S controller master mode dirver(platform device)
//
// Copyright (C) 2023-2026 Loongson Technology Corporation Limited
//
// Author: Yingkun Meng <mengyingkun@loongson.cn>
// Binbin Zhou <zhoubinbin@loongson.cn>

// Loongson-2K1000 APBDMA routing
pub const LOONGSON_I2S_RX_DMA_OFFSET: c_int = 21;
pub const LOONGSON_I2S_TX_DMA_OFFSET: c_int = 18;
pub const LOONGSON_DMA0_CONF: c_uint = 0x0;
pub const LOONGSON_DMA1_CONF: c_uint = 0x1;
pub const LOONGSON_DMA2_CONF: c_uint = 0x2;
pub const LOONGSON_DMA3_CONF: c_uint = 0x3;
pub const LOONGSON_DMA4_CONF: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_i2s_plat_config {
    pub rev_id: c_int,
    pub pdev): *mut *mut int (i2s_dma_config)(struct platform_device,
}

#[no_mangle]
unsafe extern "C" fn loongson_i2s_apbdma_config(pdev: *mut platform_device) -> c_int {
    static int loongson_i2s_apbdma_config(struct platform_device *pdev)
    {
    int val;
    void __iomem *regs;
    regs = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    val = readl(regs);
    val |= LOONGSON_DMA2_CONF << LOONGSON_I2S_TX_DMA_OFFSET;
    val |= LOONGSON_DMA3_CONF << LOONGSON_I2S_RX_DMA_OFFSET;
    writel(val, regs);
    return 0;
    }
    static const struct loongson_i2s_plat_config ls2k0300_i2s_plat_config = {
    .rev_id = 1,
    };
    static const struct loongson_i2s_plat_config ls2k1000_i2s_plat_config = {
    .rev_id = 0,
    .i2s_dma_config = loongson_i2s_apbdma_config,
    };
#[no_mangle]
unsafe extern "C" fn loongson_i2s_plat_probe(pdev: *mut platform_device) -> c_int {
    static int loongson_i2s_plat_probe(struct platform_device *pdev)
    {
    const struct loongson_i2s_plat_config *plat_config;
    struct device *dev = &pdev.dev;
    struct loongson_i2s *i2s;
    struct resource *res;
    struct clk *i2s_clk;
    int ret;
    i2s = devm_kzalloc(dev, sizeof(*i2s), GFP_KERNEL);
    if (!i2s)
    return -ENOMEM;
    plat_config = device_get_match_data(dev);
    if (!plat_config)
    return -EINVAL;
    if (plat_config.i2s_dma_config) {
    ret = plat_config.i2s_dma_config(pdev);
    if (ret)
    return ret;
    }
    i2s.reg_base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(i2s.reg_base))
    return dev_err_probe(dev, PTR_ERR(i2s.reg_base),
    "devm_ioremap_resource failed\n");
    i2s.regmap = devm_regmap_init_mmio(dev, i2s.reg_base,
    &loongson_i2s_regmap_config);
    if (IS_ERR(i2s.regmap))
    return dev_err_probe(dev, PTR_ERR(i2s.regmap),
    "devm_regmap_init_mmio failed\n");
    i2s.playback_dma_data.addr = res.start + LS_I2S_TX_DATA;
    i2s.playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    i2s.playback_dma_data.maxburst = 4;
    i2s.capture_dma_data.addr = res.start + LS_I2S_RX_DATA;
    i2s.capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    i2s.capture_dma_data.maxburst = 4;
    i2s_clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(i2s_clk))
    return dev_err_probe(dev, PTR_ERR(i2s_clk), "clock property invalid\n");
    i2s.clk_rate = clk_get_rate(i2s_clk);
    i2s.rev_id = plat_config.rev_id;
    dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    dev_set_name(dev, LS_I2S_DRVNAME);
    dev_set_drvdata(dev, i2s);
    if (i2s.rev_id == 1) {
    regmap_update_bits(i2s.regmap, LS_I2S_CTRL, I2S_CTRL_RESET, I2S_CTRL_RESET);
    fsleep(200);
    }
    ret = devm_snd_soc_register_component(dev, &loongson_i2s_edma_component,
    &loongson_i2s_dai, 1);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register DAI\n");
    return devm_snd_dmaengine_pcm_register(dev, &loongson_dmaengine_pcm_config,
    SND_DMAENGINE_PCM_FLAG_COMPAT);
    }
    static const struct of_device_id loongson_i2s_ids[] = {
    { .compatible = "loongson,ls2k0300-i2s", .data = &ls2k0300_i2s_plat_config },
    { .compatible = "loongson,ls2k1000-i2s", .data = &ls2k1000_i2s_plat_config },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, loongson_i2s_ids);
    static struct platform_driver loongson_i2s_driver = {
    .probe = loongson_i2s_plat_probe,
    .driver = {
    .name = "loongson-i2s-plat",
    .pm = pm_sleep_ptr(&loongson_i2s_pm),
    .of_match_table = loongson_i2s_ids,
    },
    };
    module_platform_driver(loongson_i2s_driver);
    MODULE_DESCRIPTION("Loongson I2S Master Mode ASoC Driver");
    MODULE_AUTHOR("Loongson Technology Corporation Limited");
    MODULE_LICENSE("GPL");
