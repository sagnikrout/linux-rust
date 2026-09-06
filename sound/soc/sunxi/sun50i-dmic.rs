//! Automatically rewritten from C to Rust
//! Source: sound/soc/sunxi/sun50i-dmic.c
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
// This driver supports the DMIC in Allwinner's H6 SoCs.
//
// Copyright 2021 Ban Tao <fengzheng923@gmail.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun50i_dmic_dev {
    pub dmic_clk: *mut clk,
    pub bus_clk: *mut clk,
    pub rst: *mut reset_control,
    pub regmap: *mut regmap,
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmic_rate {
    pub samplerate: c_uint,
    pub rate_bit: c_uint,
}

    static const struct dmic_rate dmic_rate_s[] = {
    {48000, 0x0},
    {44100, 0x0},
    {32000, 0x1},
    {24000, 0x2},
    {22050, 0x2},
    {16000, 0x3},
    {12000, 0x4},
    {11025, 0x4},
    {8000,  0x5},
    };
    static int sun50i_dmic_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *cpu_dai)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct sun50i_dmic_dev *host = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0));
// only support capture
    if (substream.stream != SNDRV_PCM_STREAM_CAPTURE)
    return -EINVAL;
    regmap_update_bits(host.regmap, SUN50I_DMIC_RXFIFO_CTL,
    SUN50I_DMIC_RXFIFO_CTL_FLUSH,
    SUN50I_DMIC_RXFIFO_CTL_FLUSH);
    regmap_write(host.regmap, SUN50I_DMIC_CNT, SUN50I_DMIC_CNT_N);
    return 0;
    }
    static int sun50i_dmic_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *cpu_dai)
    {
    let mut i: c_int = 0;
    let mut rate: c_ulong = params_rate(params);
    let mut mclk: c_uint = 0;
    let mut channels: c_uint = params_channels(params);
    let mut chan_en: c_uint = (1 << channels) - 1;
    struct sun50i_dmic_dev *host = snd_soc_dai_get_drvdata(cpu_dai);
// DMIC num is N+1
    regmap_update_bits(host.regmap, SUN50I_DMIC_CH_NUM,
    SUN50I_DMIC_CH_NUM_N_MASK,
    SUN50I_DMIC_CH_NUM_N(channels - 1));
    regmap_write(host.regmap, SUN50I_DMIC_HPF_CTRL, chan_en);
    regmap_update_bits(host.regmap, SUN50I_DMIC_EN_CTL,
    SUN50I_DMIC_EN_CTL_CHAN_MASK,
    SUN50I_DMIC_EN_CTL_CHAN(chan_en));
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    regmap_update_bits(host.regmap, SUN50I_DMIC_RXFIFO_CTL,
    SUN50I_DMIC_RXFIFO_CTL_SAMPLE_MASK,
    SUN50I_DMIC_RXFIFO_CTL_SAMPLE_16);
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    regmap_update_bits(host.regmap, SUN50I_DMIC_RXFIFO_CTL,
    SUN50I_DMIC_RXFIFO_CTL_SAMPLE_MASK,
    SUN50I_DMIC_RXFIFO_CTL_SAMPLE_24);
    break;
    default:
    dev_err(cpu_dai.dev, "Invalid format!\n");
    return -EINVAL;
    }
// The hardware supports FIFO mode 1 for 24-bit samples
    regmap_update_bits(host.regmap, SUN50I_DMIC_RXFIFO_CTL,
    SUN50I_DMIC_RXFIFO_CTL_MODE_MASK,
    SUN50I_DMIC_RXFIFO_CTL_MODE_MSB);
    switch (rate) {
    case 11025:
    case 22050:
    case 44100:
    mclk = 22579200;
    break;
    case 8000:
    case 12000:
    case 16000:
    case 24000:
    case 32000:
    case 48000:
    mclk = 24576000;
    break;
    default:
    dev_err(cpu_dai.dev, "Invalid rate!\n");
    return -EINVAL;
    }
    if (clk_set_rate(host.dmic_clk, mclk)) {
    dev_err(cpu_dai.dev, "mclk : %u not support\n", mclk);
    return -EINVAL;
    }
    for (i = 0; i < ARRAY_SIZE(dmic_rate_s); i++) {
    if (dmic_rate_s[i].samplerate == rate) {
    regmap_update_bits(host.regmap, SUN50I_DMIC_SR,
    SUN50I_DMIC_SR_SAMPLE_RATE_MASK,
    SUN50I_DMIC_SR_SAMPLE_RATE(dmic_rate_s[i].rate_bit));
    break;
    }
    }
    switch (params_physical_width(params)) {
    case 16:
    host.dma_params_rx.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    break;
    case 32:
    host.dma_params_rx.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    break;
    default:
    dev_err(cpu_dai.dev, "Unsupported physical sample width: %d\n",
    params_physical_width(params));
    return -EINVAL;
    }
// oversamplerate adjust
    if (params_rate(params) >= 24000)
    regmap_update_bits(host.regmap, SUN50I_DMIC_CTL,
    SUN50I_DMIC_CTL_OVERSAMPLE_RATE,
    SUN50I_DMIC_CTL_OVERSAMPLE_RATE);
    else
    regmap_update_bits(host.regmap, SUN50I_DMIC_CTL,
    SUN50I_DMIC_CTL_OVERSAMPLE_RATE, 0);
    return 0;
    }
    static int sun50i_dmic_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    let mut ret: c_int = 0;
    struct sun50i_dmic_dev *host = snd_soc_dai_get_drvdata(dai);
    if (substream.stream != SNDRV_PCM_STREAM_CAPTURE)
    return -EINVAL;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
// DRQ ENABLE
    regmap_update_bits(host.regmap, SUN50I_DMIC_INTC,
    SUN50I_DMIC_FIFO_DRQ_EN,
    SUN50I_DMIC_FIFO_DRQ_EN);
// Global enable
    regmap_update_bits(host.regmap, SUN50I_DMIC_EN_CTL,
    SUN50I_DMIC_EN_CTL_GLOBE,
    SUN50I_DMIC_EN_CTL_GLOBE);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
// DRQ DISABLE
    regmap_update_bits(host.regmap, SUN50I_DMIC_INTC,
    SUN50I_DMIC_FIFO_DRQ_EN, 0);
// Global disable
    regmap_update_bits(host.regmap, SUN50I_DMIC_EN_CTL,
    SUN50I_DMIC_EN_CTL_GLOBE, 0);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun50i_dmic_soc_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int sun50i_dmic_soc_dai_probe(struct snd_soc_dai *dai)
    {
    struct sun50i_dmic_dev *host = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_init_dma_data(dai, core::ptr::null_mut(), &host.dma_params_rx);
    return 0;
    }
    static const struct snd_soc_dai_ops sun50i_dmic_dai_ops = {
    .probe		= sun50i_dmic_soc_dai_probe,
    .startup        = sun50i_dmic_startup,
    .trigger        = sun50i_dmic_trigger,
    .hw_params      = sun50i_dmic_hw_params,
    };
    static const struct regmap_config sun50i_dmic_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = SUN50I_DMIC_VERSION,
    .cache_type = REGCACHE_NONE,
    };

    static struct snd_soc_dai_driver sun50i_dmic_dai = {
    .capture = {
    .channels_min = 1,
    .channels_max = 8,
    .rates = SUN50I_DMIC_RATES,
    .formats = SUN50I_DMIC_FORMATS,
    .sig_bits = 21,
    },
    .ops = &sun50i_dmic_dai_ops,
    .name = "dmic",
    };
    static const struct of_device_id sun50i_dmic_of_match[] = {
    {
    .compatible = "allwinner,sun50i-h6-dmic",
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sun50i_dmic_of_match);
    static const SNDRV_CTL_TLVD_DECLARE_DB_SCALE(sun50i_dmic_vol_scale, -12000, 75, 1);
    static const struct snd_kcontrol_new sun50i_dmic_controls[] = {
    SOC_DOUBLE_TLV("DMIC Channel 0 Capture Volume", SUN50I_DMIC_D0D1_VOL_CTR,
    SUN50I_DMIC_D0D1_VOL_CTR_0L, SUN50I_DMIC_D0D1_VOL_CTR_0R,
    0xFF, 0, sun50i_dmic_vol_scale),
    SOC_DOUBLE_TLV("DMIC Channel 1 Capture Volume", SUN50I_DMIC_D0D1_VOL_CTR,
    SUN50I_DMIC_D0D1_VOL_CTR_1L, SUN50I_DMIC_D0D1_VOL_CTR_1R,
    0xFF, 0, sun50i_dmic_vol_scale),
    SOC_DOUBLE_TLV("DMIC Channel 2 Capture Volume", SUN50I_DMIC_D2D3_VOL_CTR,
    SUN50I_DMIC_D2D3_VOL_CTR_2L, SUN50I_DMIC_D2D3_VOL_CTR_2R,
    0xFF, 0, sun50i_dmic_vol_scale),
    SOC_DOUBLE_TLV("DMIC Channel 3 Capture Volume", SUN50I_DMIC_D2D3_VOL_CTR,
    SUN50I_DMIC_D2D3_VOL_CTR_3L, SUN50I_DMIC_D2D3_VOL_CTR_3R,
    0xFF, 0, sun50i_dmic_vol_scale),
    };
    static const struct snd_soc_component_driver sun50i_dmic_component = {
    .name           = "sun50i-dmic",
    .controls	= sun50i_dmic_controls,
    .num_controls	= ARRAY_SIZE(sun50i_dmic_controls),
    };
#[no_mangle]
unsafe extern "C" fn sun50i_dmic_runtime_suspend(dev: *mut device) -> c_int {
    static int sun50i_dmic_runtime_suspend(struct device *dev)
    {
    struct sun50i_dmic_dev *host  = dev_get_drvdata(dev);
    clk_disable_unprepare(host.dmic_clk);
    clk_disable_unprepare(host.bus_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun50i_dmic_runtime_resume(dev: *mut device) -> c_int {
    static int sun50i_dmic_runtime_resume(struct device *dev)
    {
    struct sun50i_dmic_dev *host  = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(host.dmic_clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(host.bus_clk);
    if (ret) {
    clk_disable_unprepare(host.dmic_clk);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun50i_dmic_probe(pdev: *mut platform_device) -> c_int {
    static int sun50i_dmic_probe(struct platform_device *pdev)
    {
    struct sun50i_dmic_dev *host;
    struct resource *res;
    int ret;
    void __iomem *base;
    host = devm_kzalloc(&pdev.dev, sizeof(*host), GFP_KERNEL);
    if (!host)
    return -ENOMEM;
// Get the addresses
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return dev_err_probe(&pdev.dev, PTR_ERR(base),
    "get resource failed.\n");
    host.regmap = devm_regmap_init_mmio(&pdev.dev, base,
    &sun50i_dmic_regmap_config);
    if (IS_ERR(host.regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(host.regmap),
    "failed to initialise regmap\n");
// Clocks
    host.bus_clk = devm_clk_get(&pdev.dev, "bus");
    if (IS_ERR(host.bus_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(host.bus_clk),
    "failed to get bus clock.\n");
    host.dmic_clk = devm_clk_get(&pdev.dev, "mod");
    if (IS_ERR(host.dmic_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(host.dmic_clk),
    "failed to get dmic clock.\n");
    host.dma_params_rx.addr = res.start + SUN50I_DMIC_DATA;
    host.dma_params_rx.maxburst = 8;
    platform_set_drvdata(pdev, host);
    host.rst = devm_reset_control_get_optional_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(host.rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(host.rst),
    "Failed to get reset.\n");
    reset_control_deassert(host.rst);
    ret = devm_snd_soc_register_component(&pdev.dev, &sun50i_dmic_component,
    &sun50i_dmic_dai, 1);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to register component.\n");
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    ret = sun50i_dmic_runtime_resume(&pdev.dev);
    if (ret)
    goto err_disable_runtime_pm;
    }
    ret = devm_snd_dmaengine_pcm_register(&pdev.dev, core::ptr::null_mut(), 0);
    if (ret)
    goto err_suspend;
    return 0;
    err_suspend:
    if (!pm_runtime_status_suspended(&pdev.dev))
    sun50i_dmic_runtime_suspend(&pdev.dev);
    err_disable_runtime_pm:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun50i_dmic_remove(pdev: *mut platform_device) {
    static void sun50i_dmic_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    sun50i_dmic_runtime_suspend(&pdev.dev);
    }
    static const struct dev_pm_ops sun50i_dmic_pm = {
    RUNTIME_PM_OPS(sun50i_dmic_runtime_suspend,
    sun50i_dmic_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver sun50i_dmic_driver = {
    .driver         = {
    .name   = "sun50i-dmic",
    .of_match_table = sun50i_dmic_of_match,
    .pm     = pm_ptr(&sun50i_dmic_pm),
    },
    .probe          = sun50i_dmic_probe,
    .remove         = sun50i_dmic_remove,
    };
    module_platform_driver(sun50i_dmic_driver);
    MODULE_DESCRIPTION("Allwinner sun50i DMIC SoC Interface");
    MODULE_AUTHOR("Ban Tao <fengzheng923@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:sun50i-dmic");
