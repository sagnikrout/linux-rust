//! Automatically rewritten from C to Rust
//! Source: sound/soc/sophgo/cv1800b-sound-dac.c
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
// Internal DAC codec for cv1800b based CPUs
//

pub const CV1800B_TXDAC_CTRL0: c_uint = 0x00;
pub const CV1800B_TXDAC_CTRL1: c_uint = 0x04;
pub const CV1800B_TXDAC_STATUS: c_uint = 0x08;
pub const CV1800B_TXDAC_AFE0: c_uint = 0x0c;
pub const CV1800B_TXDAC_AFE1: c_uint = 0x10;
pub const CV1800B_TXDAC_ANA0: c_uint = 0x20;
pub const CV1800B_TXDAC_ANA1: c_uint = 0x24;
pub const CV1800B_TXDAC_ANA2: c_uint = 0x28;
// cv1800b_TXDAC_CTRL0

// cv1800b_TXDAC_CTRL1

// cv1800b_TXDAC_AFE0

// cv1800b_TXDAC_ANA2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800b_priv {
    pub regs: *mut void __iomem,
    pub dev: *mut device,
}

    enum decimation_values {
    DECIMATION_64 = 0,
    DECIMATION_128,
    DECIMATION_256,
    DECIMATION_512,
    };
#[no_mangle]
unsafe extern "C" fn cv1800b_dac_enable(priv: *mut cv1800b_priv, enable: bool) {
    static void cv1800b_dac_enable(struct cv1800b_priv *priv, bool enable)
    {
    u32 val;
    val = readl(priv.regs + CV1800B_TXDAC_CTRL0);
    val = u32_replace_bits(val, enable, REG_TXDAC_EN);
    val = u32_replace_bits(val, enable, REG_I2S_RX_EN);
    writel(val, priv.regs + CV1800B_TXDAC_CTRL0);
    }
//
// Control the DAC overwrite bits. When enabled, the DAC outputs the fixed
// overwrite value instead of samples from the I2S input.
//
#[no_mangle]
unsafe extern "C" fn cv1800b_dac_mute(priv: *mut cv1800b_priv, enable: bool) {
    static void cv1800b_dac_mute(struct cv1800b_priv *priv, bool enable)
    {
    u32 val;
    val = readl(priv.regs + CV1800B_TXDAC_ANA2);
    val = u32_replace_bits(val, enable, TXDAC_OW_EN_L_MASK);
    val = u32_replace_bits(val, enable, TXDAC_OW_EN_R_MASK);
    writel(val, priv.regs + CV1800B_TXDAC_ANA2);
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_dac_decimation(priv: *mut cv1800b_priv, dec: u8) -> c_int {
    static int cv1800b_dac_decimation(struct cv1800b_priv *priv, u8 dec)
    {
    u32 val;
    if (dec > 3)
    return -EINVAL;
    val = readl(priv.regs + CV1800B_TXDAC_CTRL1);
    val = u32_replace_bits(val, dec, REG_TXDAC_CIC_OPT);
    writel(val, priv.regs + CV1800B_TXDAC_CTRL1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cv1800b_dac_dly(priv: *mut cv1800b_priv, dly: u32) -> c_int {
    static int cv1800b_dac_dly(struct cv1800b_priv *priv, u32 dly)
    {
    u32 val;
    if (dly > 63)
    return -EINVAL;
    val = readl(priv.regs + CV1800B_TXDAC_AFE0);
    val = u32_replace_bits(val, dly, REG_TXDAC_INIT_DLY_CNT);
    writel(val, priv.regs + CV1800B_TXDAC_AFE0);
    return 0;
    }
    static int cv1800b_dac_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct cv1800b_priv *priv = snd_soc_dai_get_drvdata(dai);
    int ret;
    let mut rate: c_uint = params_rate(params);
    if (rate != 48000) {
    dev_err(priv.dev, "rate %u is not supported\n", rate);
    return -EINVAL;
    }
// Clear DAC overwrite so playback uses I2S data.
    cv1800b_dac_mute(priv, false);
// minimal decimation for 48kHz is 64
    ret = cv1800b_dac_decimation(priv, DECIMATION_64);
    if (ret)
    return ret;
// value is taken from vendors driver 48kHz
// tested on sg2000 and sg2002.
//
    ret = cv1800b_dac_dly(priv, 0x19);
    if (ret)
    return ret;
    return 0;
    }
    static int cv1800b_dac_dai_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct cv1800b_priv *priv = snd_soc_dai_get_drvdata(dai);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    cv1800b_dac_enable(priv, true);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    cv1800b_dac_enable(priv, false);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct snd_soc_dai_ops cv1800b_dac_dai_ops = {
    .hw_params = cv1800b_dac_hw_params,
    .trigger = cv1800b_dac_dai_trigger,
    };
    static struct snd_soc_dai_driver cv1800b_dac_dai = {
    .name = "dac-hifi",
    .playback = { .stream_name = "DAC Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE },
    .ops = &cv1800b_dac_dai_ops,
    };
    static const struct snd_soc_component_driver cv1800b_dac_component = {
    .name = "cv1800b-dac-codec",
    };
#[no_mangle]
unsafe extern "C" fn cv1800b_dac_probe(pdev: *mut platform_device) -> c_int {
    static int cv1800b_dac_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct cv1800b_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.regs))
    return PTR_ERR(priv.regs);
    platform_set_drvdata(pdev, priv);
    return devm_snd_soc_register_component(&pdev.dev,
    &cv1800b_dac_component,
    &cv1800b_dac_dai, 1);
    }
    static const struct of_device_id cv1800b_dac_of_match[] = {
    { .compatible = "sophgo,cv1800b-sound-dac" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, cv1800b_dac_of_match);
    static struct platform_driver cv1800b_dac_driver = {
    .probe = cv1800b_dac_probe,
    .driver = {
    .name = "cv1800b-dac-codec",
    .of_match_table = cv1800b_dac_of_match,
    },
    };
    module_platform_driver(cv1800b_dac_driver);
    MODULE_DESCRIPTION("DAC codec for CV1800B");
    MODULE_AUTHOR("Anton D. Stavinskii <stavinsky@gmail.com>");
    MODULE_LICENSE("GPL");
