//! Automatically rewritten from C to Rust
//! Source: sound/soc/bcm/bcm63xx-i2s-whistler.c
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
// linux/sound/bcm/bcm63xx-i2s-whistler.c
// BCM63xx whistler i2s driver
// Copyright (c) 2020 Broadcom Corporation
// Author: Kevin-Ke Li <kevin-ke.li@broadcom.com>

#[no_mangle]
unsafe extern "C" fn brcm_i2s_wr_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool brcm_i2s_wr_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case I2S_TX_CFG ... I2S_TX_DESC_IFF_LEN:
    case I2S_TX_CFG_2 ... I2S_RX_DESC_IFF_LEN:
    case I2S_RX_CFG_2 ... I2S_REG_MAX:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn brcm_i2s_rd_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool brcm_i2s_rd_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case I2S_TX_CFG ... I2S_REG_MAX:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn brcm_i2s_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool brcm_i2s_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case I2S_TX_CFG:
    case I2S_TX_IRQ_CTL:
    case I2S_TX_DESC_IFF_ADDR:
    case I2S_TX_DESC_IFF_LEN:
    case I2S_TX_DESC_OFF_ADDR:
    case I2S_TX_DESC_OFF_LEN:
    case I2S_TX_CFG_2:
    case I2S_RX_CFG:
    case I2S_RX_IRQ_CTL:
    case I2S_RX_DESC_OFF_ADDR:
    case I2S_RX_DESC_OFF_LEN:
    case I2S_RX_DESC_IFF_LEN:
    case I2S_RX_DESC_IFF_ADDR:
    case I2S_RX_CFG_2:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config brcm_i2s_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = I2S_REG_MAX,
    .writeable_reg = brcm_i2s_wr_reg,
    .readable_reg = brcm_i2s_rd_reg,
    .volatile_reg = brcm_i2s_volatile_reg,
    .cache_type = REGCACHE_FLAT,
    };
    static int bcm63xx_i2s_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    let mut ret: c_int = 0;
    struct bcm_i2s_priv *i2s_priv = snd_soc_dai_get_drvdata(dai);
    ret = clk_set_rate(i2s_priv.i2s_clk, params_rate(params));
    if (ret < 0)
    dev_err(i2s_priv.dev,
    "Can't set sample rate, err: %d\n", ret);
    return ret;
    }
    static int bcm63xx_i2s_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    unsigned int slavemode;
    struct bcm_i2s_priv *i2s_priv = snd_soc_dai_get_drvdata(dai);
    struct regmap *regmap_i2s = i2s_priv.regmap_i2s;
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    regmap_update_bits(regmap_i2s, I2S_TX_CFG,
    I2S_TX_OUT_R | I2S_TX_DATA_ALIGNMENT |
    I2S_TX_DATA_ENABLE | I2S_TX_CLOCK_ENABLE,
    I2S_TX_OUT_R | I2S_TX_DATA_ALIGNMENT |
    I2S_TX_DATA_ENABLE | I2S_TX_CLOCK_ENABLE);
    regmap_write(regmap_i2s, I2S_TX_IRQ_CTL, 0);
    regmap_write(regmap_i2s, I2S_TX_IRQ_IFF_THLD, 0);
    regmap_write(regmap_i2s, I2S_TX_IRQ_OFF_THLD, 1);
// TX and RX block each have an independent bit to indicate
// if it is generating the clock for the I2S bus. The bus
// clocks need to be generated from either the TX or RX block,
// but not both
//
    regmap_read(regmap_i2s, I2S_RX_CFG_2, &slavemode);
    if (slavemode & I2S_RX_SLAVE_MODE_MASK)
    regmap_update_bits(regmap_i2s, I2S_TX_CFG_2,
    I2S_TX_SLAVE_MODE_MASK,
    I2S_TX_MASTER_MODE);
    else
    regmap_update_bits(regmap_i2s, I2S_TX_CFG_2,
    I2S_TX_SLAVE_MODE_MASK,
    I2S_TX_SLAVE_MODE);
    } else {
    regmap_update_bits(regmap_i2s, I2S_RX_CFG,
    I2S_RX_IN_R | I2S_RX_DATA_ALIGNMENT |
    I2S_RX_CLOCK_ENABLE,
    I2S_RX_IN_R | I2S_RX_DATA_ALIGNMENT |
    I2S_RX_CLOCK_ENABLE);
    regmap_write(regmap_i2s, I2S_RX_IRQ_CTL, 0);
    regmap_write(regmap_i2s, I2S_RX_IRQ_IFF_THLD, 0);
    regmap_write(regmap_i2s, I2S_RX_IRQ_OFF_THLD, 1);
    regmap_read(regmap_i2s, I2S_TX_CFG_2, &slavemode);
    if (slavemode & I2S_TX_SLAVE_MODE_MASK)
    regmap_update_bits(regmap_i2s, I2S_RX_CFG_2,
    I2S_RX_SLAVE_MODE_MASK, 0);
    else
    regmap_update_bits(regmap_i2s, I2S_RX_CFG_2,
    I2S_RX_SLAVE_MODE_MASK,
    I2S_RX_SLAVE_MODE);
    }
    return 0;
    }
    static void bcm63xx_i2s_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    unsigned int enabled, slavemode;
    struct bcm_i2s_priv *i2s_priv = snd_soc_dai_get_drvdata(dai);
    struct regmap *regmap_i2s = i2s_priv.regmap_i2s;
    if (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) {
    regmap_update_bits(regmap_i2s, I2S_TX_CFG,
    I2S_TX_OUT_R | I2S_TX_DATA_ALIGNMENT |
    I2S_TX_DATA_ENABLE | I2S_TX_CLOCK_ENABLE, 0);
    regmap_write(regmap_i2s, I2S_TX_IRQ_CTL, 1);
    regmap_write(regmap_i2s, I2S_TX_IRQ_IFF_THLD, 4);
    regmap_write(regmap_i2s, I2S_TX_IRQ_OFF_THLD, 4);
    regmap_read(regmap_i2s, I2S_TX_CFG_2, &slavemode);
    slavemode = slavemode & I2S_TX_SLAVE_MODE_MASK;
    if (!slavemode) {
    regmap_read(regmap_i2s, I2S_RX_CFG, &enabled);
    enabled = enabled & I2S_RX_ENABLE_MASK;
    if (enabled)
    regmap_update_bits(regmap_i2s, I2S_RX_CFG_2,
    I2S_RX_SLAVE_MODE_MASK,
    I2S_RX_MASTER_MODE);
    }
    regmap_update_bits(regmap_i2s, I2S_TX_CFG_2,
    I2S_TX_SLAVE_MODE_MASK,
    I2S_TX_SLAVE_MODE);
    } else {
    regmap_update_bits(regmap_i2s, I2S_RX_CFG,
    I2S_RX_IN_R | I2S_RX_DATA_ALIGNMENT |
    I2S_RX_CLOCK_ENABLE, 0);
    regmap_write(regmap_i2s, I2S_RX_IRQ_CTL, 1);
    regmap_write(regmap_i2s, I2S_RX_IRQ_IFF_THLD, 4);
    regmap_write(regmap_i2s, I2S_RX_IRQ_OFF_THLD, 4);
    regmap_read(regmap_i2s, I2S_RX_CFG_2, &slavemode);
    slavemode = slavemode & I2S_RX_SLAVE_MODE_MASK;
    if (!slavemode) {
    regmap_read(regmap_i2s, I2S_TX_CFG, &enabled);
    enabled = enabled & I2S_TX_ENABLE_MASK;
    if (enabled)
    regmap_update_bits(regmap_i2s, I2S_TX_CFG_2,
    I2S_TX_SLAVE_MODE_MASK,
    I2S_TX_MASTER_MODE);
    }
    regmap_update_bits(regmap_i2s, I2S_RX_CFG_2,
    I2S_RX_SLAVE_MODE_MASK, I2S_RX_SLAVE_MODE);
    }
    }
    static const struct snd_soc_dai_ops bcm63xx_i2s_dai_ops = {
    .startup = bcm63xx_i2s_startup,
    .shutdown = bcm63xx_i2s_shutdown,
    .hw_params = bcm63xx_i2s_hw_params,
    };
    static struct snd_soc_dai_driver bcm63xx_i2s_dai = {
    .name = DRV_NAME,
    .playback = {
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S32_LE,
    },
    .capture = {
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_192000,
    .formats = SNDRV_PCM_FMTBIT_S32_LE,
    },
    .ops = &bcm63xx_i2s_dai_ops,
    .symmetric_rate = 1,
    .symmetric_channels = 1,
    };
    static const struct snd_soc_component_driver bcm63xx_i2s_component = {
    .name = "bcm63xx",
    .legacy_dai_naming = 1,
    };
#[no_mangle]
unsafe extern "C" fn bcm63xx_i2s_dev_probe(pdev: *mut platform_device) -> c_int {
    static int bcm63xx_i2s_dev_probe(struct platform_device *pdev)
    {
    let mut ret: c_int = 0;
    void __iomem *regs;
    struct bcm_i2s_priv *i2s_priv;
    struct regmap *regmap_i2s;
    struct clk *i2s_clk;
    i2s_priv = devm_kzalloc(&pdev.dev, sizeof(*i2s_priv), GFP_KERNEL);
    if (!i2s_priv)
    return -ENOMEM;
    i2s_clk = devm_clk_get(&pdev.dev, "i2sclk");
    if (IS_ERR(i2s_clk)) {
    dev_err(&pdev.dev, "%s: cannot get a brcm clock: %ld\n",
    __func__, PTR_ERR(i2s_clk));
    return PTR_ERR(i2s_clk);
    }
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs)) {
    ret = PTR_ERR(regs);
    return ret;
    }
    regmap_i2s = devm_regmap_init_mmio(&pdev.dev,
    regs, &brcm_i2s_regmap_config);
    if (IS_ERR(regmap_i2s))
    return PTR_ERR(regmap_i2s);
    regmap_update_bits(regmap_i2s, I2S_MISC_CFG,
    I2S_PAD_LVL_LOOP_DIS_MASK,
    I2S_PAD_LVL_LOOP_DIS_ENABLE);
    ret = devm_snd_soc_register_component(&pdev.dev,
    &bcm63xx_i2s_component,
    &bcm63xx_i2s_dai, 1);
    if (ret) {
    dev_err(&pdev.dev, "failed to register the dai\n");
    return ret;
    }
    i2s_priv.dev = &pdev.dev;
    i2s_priv.i2s_clk = i2s_clk;
    i2s_priv.regmap_i2s = regmap_i2s;
    dev_set_drvdata(&pdev.dev, i2s_priv);
    ret = bcm63xx_soc_platform_probe(pdev, i2s_priv);
    if (ret)
    dev_err(&pdev.dev, "failed to register the pcm\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bcm63xx_i2s_dev_remove(pdev: *mut platform_device) {
    static void bcm63xx_i2s_dev_remove(struct platform_device *pdev)
    {
    bcm63xx_soc_platform_remove(pdev);
    }

    static const struct of_device_id snd_soc_bcm_audio_match[] = {
    {.compatible = "brcm,bcm63xx-i2s"},
    { }
    };

    static struct platform_driver bcm63xx_i2s_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = of_match_ptr(snd_soc_bcm_audio_match),
    },
    .probe = bcm63xx_i2s_dev_probe,
    .remove = bcm63xx_i2s_dev_remove,
    };
    module_platform_driver(bcm63xx_i2s_driver);
    MODULE_AUTHOR("Kevin,Li <kevin-ke.li@broadcom.com>");
    MODULE_DESCRIPTION("Broadcom DSL XPON ASOC I2S Interface");
    MODULE_LICENSE("GPL v2");
