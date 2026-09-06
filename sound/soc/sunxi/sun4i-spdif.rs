//! Automatically rewritten from C to Rust
//! Source: sound/soc/sunxi/sun4i-spdif.c
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
// ALSA SoC SPDIF Audio Layer
//
// Copyright 2015 Andrea Venturi <be17068@iperbole.bo.it>
// Copyright 2015 Marcus Cooper <codekipper@gmail.com>
//
// Based on the Allwinner SDK driver, released under the GPL.
//

// Defines for Sampling Frequency
pub const SUN4I_SPDIF_SAMFREQ_44_1KHZ: c_uint = 0x0;
pub const SUN4I_SPDIF_SAMFREQ_NOT_INDICATED: c_uint = 0x1;
pub const SUN4I_SPDIF_SAMFREQ_48KHZ: c_uint = 0x2;
pub const SUN4I_SPDIF_SAMFREQ_32KHZ: c_uint = 0x3;
pub const SUN4I_SPDIF_SAMFREQ_22_05KHZ: c_uint = 0x4;
pub const SUN4I_SPDIF_SAMFREQ_24KHZ: c_uint = 0x6;
pub const SUN4I_SPDIF_SAMFREQ_88_2KHZ: c_uint = 0x8;
pub const SUN4I_SPDIF_SAMFREQ_76_8KHZ: c_uint = 0x9;
pub const SUN4I_SPDIF_SAMFREQ_96KHZ: c_uint = 0xa;
pub const SUN4I_SPDIF_SAMFREQ_176_4KHZ: c_uint = 0xc;
pub const SUN4I_SPDIF_SAMFREQ_192KHZ: c_uint = 0xe;
//
// struct sun4i_spdif_quirks - Differences between SoC variants.
//
// @reg_dac_txdata: TX FIFO offset for DMA config.
// @has_reset: SoC needs reset deasserted.
// @val_fctl_ftx: TX FIFO flush bitmask.
// @mclk_multiplier: ratio of internal MCLK divider
// @tx_clk_name: name of TX module clock if split clock design
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_spdif_quirks {
    pub reg_dac_txdata: c_uint,
    pub has_reset: bool,
    pub val_fctl_ftx: c_uint,
    pub mclk_multiplier: c_uint,
    pub tx_clk_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_spdif_dev {
    pub pdev: *mut platform_device,
    pub spdif_clk: *mut clk,
    pub apb_clk: *mut clk,
    pub rst: *mut reset_control,
    pub cpu_dai_drv: snd_soc_dai_driver,
    pub regmap: *mut regmap,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
    pub quirks: *const sun4i_spdif_quirks,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn sun4i_spdif_configure(host: *mut sun4i_spdif_dev) {
    static void sun4i_spdif_configure(struct sun4i_spdif_dev *host)
    {
    const struct sun4i_spdif_quirks *quirks = host.quirks;
// soft reset SPDIF
    regmap_write(host.regmap, SUN4I_SPDIF_CTL, SUN4I_SPDIF_CTL_RESET);
// flush TX FIFO
    regmap_update_bits(host.regmap, SUN4I_SPDIF_FCTL,
    quirks.val_fctl_ftx, quirks.val_fctl_ftx);
// Valid data at the MSB of TXFIFO Register
    regmap_update_bits(host.regmap, SUN4I_SPDIF_FCTL,
    SUN4I_SPDIF_FCTL_TXIM, 0);
// clear TX counter
    regmap_write(host.regmap, SUN4I_SPDIF_TXCNT, 0);
    }
    static void sun4i_snd_txctrl_on(struct snd_pcm_substream *substream,
    struct sun4i_spdif_dev *host)
    {
    if (substream.runtime.channels == 1)
    regmap_update_bits(host.regmap, SUN4I_SPDIF_TXCFG,
    SUN4I_SPDIF_TXCFG_SINGLEMOD,
    SUN4I_SPDIF_TXCFG_SINGLEMOD);
// SPDIF TX ENABLE
    regmap_update_bits(host.regmap, SUN4I_SPDIF_TXCFG,
    SUN4I_SPDIF_TXCFG_TXEN, SUN4I_SPDIF_TXCFG_TXEN);
// DRQ ENABLE
    regmap_update_bits(host.regmap, SUN4I_SPDIF_INT,
    SUN4I_SPDIF_INT_TXDRQEN, SUN4I_SPDIF_INT_TXDRQEN);
// Global enable
    regmap_update_bits(host.regmap, SUN4I_SPDIF_CTL,
    SUN4I_SPDIF_CTL_GEN, SUN4I_SPDIF_CTL_GEN);
    }
    static void sun4i_snd_txctrl_off(struct snd_pcm_substream *substream,
    struct sun4i_spdif_dev *host)
    {
// SPDIF TX DISABLE
    regmap_update_bits(host.regmap, SUN4I_SPDIF_TXCFG,
    SUN4I_SPDIF_TXCFG_TXEN, 0);
// DRQ DISABLE
    regmap_update_bits(host.regmap, SUN4I_SPDIF_INT,
    SUN4I_SPDIF_INT_TXDRQEN, 0);
// Global disable
    regmap_update_bits(host.regmap, SUN4I_SPDIF_CTL,
    SUN4I_SPDIF_CTL_GEN, 0);
    }
    static int sun4i_spdif_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *cpu_dai)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct sun4i_spdif_dev *host = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0));
    if (substream.stream != SNDRV_PCM_STREAM_PLAYBACK)
    return -EINVAL;
    sun4i_spdif_configure(host);
    return 0;
    }
    static int sun4i_spdif_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *cpu_dai)
    {
    let mut ret: c_int = 0;
    int fmt;
    let mut rate: c_ulong = params_rate(params);
    let mut mclk_div: u32 = 0;
    let mut mclk: c_uint = 0;
    u32 reg_val;
    struct sun4i_spdif_dev *host = snd_soc_dai_get_drvdata(cpu_dai);
    struct platform_device *pdev = host.pdev;
// Add the PCM and raw data select interface
    switch (params_channels(params)) {
    case 1: /* PCM mode */
    case 2:
    fmt = 0;
    break;
    case 4: /* raw data mode */
    fmt = SUN4I_SPDIF_TXCFG_NONAUDIO;
    break;
    default:
    return -EINVAL;
    }
    host.dma_params_tx.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S16_LE:
    fmt |= SUN4I_SPDIF_TXCFG_FMT16BIT;
    host.dma_params_tx.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    break;
    case SNDRV_PCM_FORMAT_S20_3LE:
    fmt |= SUN4I_SPDIF_TXCFG_FMT20BIT;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    case SNDRV_PCM_FORMAT_S32_LE:
    fmt |= SUN4I_SPDIF_TXCFG_FMT24BIT;
    break;
    default:
    return -EINVAL;
    }
    switch (rate) {
    case 22050:
    case 44100:
    case 88200:
    case 176400:
    mclk = 22579200;
    break;
    case 24000:
    case 32000:
    case 48000:
    case 96000:
    case 192000:
    mclk = 24576000;
    break;
    default:
    return -EINVAL;
    }
    mclk *= host.quirks.mclk_multiplier;
    ret = clk_set_rate(host.spdif_clk, mclk);
    if (ret < 0) {
    dev_err(&pdev.dev,
    "Setting SPDIF clock rate for %d Hz failed!\n", mclk);
    return ret;
    }
    switch (rate) {
    case 22050:
    case 24000:
    mclk_div = 8;
    break;
    case 32000:
    mclk_div = 6;
    break;
    case 44100:
    case 48000:
    mclk_div = 4;
    break;
    case 88200:
    case 96000:
    mclk_div = 2;
    break;
    case 176400:
    case 192000:
    mclk_div = 1;
    break;
    default:
    return -EINVAL;
    }
    mclk_div *= host.quirks.mclk_multiplier;
    reg_val = 0;
    reg_val |= SUN4I_SPDIF_TXCFG_ASS;
    reg_val |= fmt; /* set non audio and bit depth */
    reg_val |= SUN4I_SPDIF_TXCFG_CHSTMODE;
    reg_val |= SUN4I_SPDIF_TXCFG_TXRATIO(mclk_div - 1);
    regmap_write(host.regmap, SUN4I_SPDIF_TXCFG, reg_val);
    return 0;
    }
    static int sun4i_spdif_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    let mut ret: c_int = 0;
    struct sun4i_spdif_dev *host = snd_soc_dai_get_drvdata(dai);
    if (substream.stream != SNDRV_PCM_STREAM_PLAYBACK)
    return -EINVAL;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    sun4i_snd_txctrl_on(substream, host);
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    sun4i_snd_txctrl_off(substream, host);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static int sun4i_spdif_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *uinfo)
    {
    uinfo.type = SNDRV_CTL_ELEM_TYPE_IEC958;
    uinfo.count = 1;
    return 0;
    }
    static int sun4i_spdif_get_status_mask(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    u8 *status = ucontrol.value.iec958.status;
    status[0] = 0xff;
    status[1] = 0xff;
    status[2] = 0xff;
    status[3] = 0xff;
    status[4] = 0xff;
    status[5] = 0x03;
    return 0;
    }
    static int sun4i_spdif_get_status(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_dai *cpu_dai = snd_kcontrol_chip(kcontrol);
    struct sun4i_spdif_dev *host = snd_soc_dai_get_drvdata(cpu_dai);
    u8 *status = ucontrol.value.iec958.status;
    unsigned long flags;
    unsigned int reg;
    spin_lock_irqsave(&host.lock, flags);
    regmap_read(host.regmap, SUN4I_SPDIF_TXCHSTA0, &reg);
    status[0] = reg & 0xff;
    status[1] = (reg >> 8) & 0xff;
    status[2] = (reg >> 16) & 0xff;
    status[3] = (reg >> 24) & 0xff;
    regmap_read(host.regmap, SUN4I_SPDIF_TXCHSTA1, &reg);
    status[4] = reg & 0xff;
    status[5] = (reg >> 8) & 0x3;
    spin_unlock_irqrestore(&host.lock, flags);
    return 0;
    }
    static int sun4i_spdif_set_status(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_dai *cpu_dai = snd_kcontrol_chip(kcontrol);
    struct sun4i_spdif_dev *host = snd_soc_dai_get_drvdata(cpu_dai);
    u8 *status = ucontrol.value.iec958.status;
    unsigned long flags;
    unsigned int reg;
    bool chg0, chg1;
    spin_lock_irqsave(&host.lock, flags);
    reg = (u32)status[3] << 24;
    reg |= (u32)status[2] << 16;
    reg |= (u32)status[1] << 8;
    reg |= (u32)status[0];
    regmap_update_bits_check(host.regmap, SUN4I_SPDIF_TXCHSTA0,
    GENMASK(31,0), reg, &chg0);
    reg = (u32)status[5] << 8;
    reg |= (u32)status[4];
    regmap_update_bits_check(host.regmap, SUN4I_SPDIF_TXCHSTA1,
    GENMASK(9,0), reg, &chg1);
    reg = SUN4I_SPDIF_TXCFG_CHSTMODE;
    if (status[0] & IEC958_AES0_NONAUDIO)
    reg |= SUN4I_SPDIF_TXCFG_NONAUDIO;
    regmap_update_bits(host.regmap, SUN4I_SPDIF_TXCFG,
    SUN4I_SPDIF_TXCFG_CHSTMODE |
    SUN4I_SPDIF_TXCFG_NONAUDIO, reg);
    spin_unlock_irqrestore(&host.lock, flags);
    return chg0 || chg1;
    }
    static struct snd_kcontrol_new sun4i_spdif_controls[] = {
    {
    .access = SNDRV_CTL_ELEM_ACCESS_READ,
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name = SNDRV_CTL_NAME_IEC958("", PLAYBACK, MASK),
    .info = sun4i_spdif_info,
    .get = sun4i_spdif_get_status_mask
    },
    {
    .iface = SNDRV_CTL_ELEM_IFACE_PCM,
    .name = SNDRV_CTL_NAME_IEC958("", PLAYBACK, DEFAULT),
    .info = sun4i_spdif_info,
    .get = sun4i_spdif_get_status,
    .put = sun4i_spdif_set_status
    }
    };
#[no_mangle]
unsafe extern "C" fn sun4i_spdif_soc_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int sun4i_spdif_soc_dai_probe(struct snd_soc_dai *dai)
    {
    struct sun4i_spdif_dev *host = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_init_dma_data(dai, &host.dma_params_tx, core::ptr::null_mut());
    snd_soc_add_dai_controls(dai, sun4i_spdif_controls,
    ARRAY_SIZE(sun4i_spdif_controls));
    return 0;
    }
    static const struct snd_soc_dai_ops sun4i_spdif_dai_ops = {
    .probe		= sun4i_spdif_soc_dai_probe,
    .startup	= sun4i_spdif_startup,
    .trigger	= sun4i_spdif_trigger,
    .hw_params	= sun4i_spdif_hw_params,
    };
    static const struct regmap_config sun4i_spdif_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = SUN4I_SPDIF_RXCHSTA1,
    };

    SNDRV_PCM_FMTBIT_S20_3LE | \
    SNDRV_PCM_FMTBIT_S24_LE | \
    SNDRV_PCM_FMTBIT_S32_LE)
    static struct snd_soc_dai_driver sun4i_spdif_dai = {
    .playback = {
    .channels_min = 1,
    .channels_max = 2,
    .rates = SUN4I_RATES,
    .formats = SUN4I_FORMATS,
    },
    .ops = &sun4i_spdif_dai_ops,
    .name = "spdif",
    };
    static const struct sun4i_spdif_quirks sun4i_a10_spdif_quirks = {
    .reg_dac_txdata	= SUN4I_SPDIF_TXFIFO,
    .val_fctl_ftx   = SUN4I_SPDIF_FCTL_FTX,
    .mclk_multiplier = 1,
    };
    static const struct sun4i_spdif_quirks sun6i_a31_spdif_quirks = {
    .reg_dac_txdata	= SUN4I_SPDIF_TXFIFO,
    .val_fctl_ftx   = SUN4I_SPDIF_FCTL_FTX,
    .has_reset	= true,
    .mclk_multiplier = 1,
    };
    static const struct sun4i_spdif_quirks sun8i_h3_spdif_quirks = {
    .reg_dac_txdata	= SUN8I_SPDIF_TXFIFO,
    .val_fctl_ftx   = SUN4I_SPDIF_FCTL_FTX,
    .has_reset	= true,
    .mclk_multiplier = 4,
    };
    static const struct sun4i_spdif_quirks sun50i_h6_spdif_quirks = {
    .reg_dac_txdata = SUN8I_SPDIF_TXFIFO,
    .val_fctl_ftx   = SUN50I_H6_SPDIF_FCTL_FTX,
    .has_reset      = true,
    .mclk_multiplier = 1,
    };
    static const struct sun4i_spdif_quirks sun55i_a523_spdif_quirks = {
    .reg_dac_txdata = SUN8I_SPDIF_TXFIFO,
    .val_fctl_ftx   = SUN50I_H6_SPDIF_FCTL_FTX,
    .has_reset      = true,
    .mclk_multiplier = 1,
    .tx_clk_name	= "tx",
    };
    static const struct of_device_id sun4i_spdif_of_match[] = {
    {
    .compatible = "allwinner,sun4i-a10-spdif",
    .data = &sun4i_a10_spdif_quirks,
    },
    {
    .compatible = "allwinner,sun6i-a31-spdif",
    .data = &sun6i_a31_spdif_quirks,
    },
    {
    .compatible = "allwinner,sun8i-h3-spdif",
    .data = &sun8i_h3_spdif_quirks,
    },
    {
    .compatible = "allwinner,sun50i-h6-spdif",
    .data = &sun50i_h6_spdif_quirks,
    },
    {
    .compatible = "allwinner,sun50i-h616-spdif",
// Essentially the same as the H6, but without RX
    .data = &sun50i_h6_spdif_quirks,
    },
    {
    .compatible = "allwinner,sun55i-a523-spdif",
//
// Almost the same as H6, but has split the TX and RX clocks,
// has a separate reset bit for the RX side, and has some
// expanded features for the RX side.
//
    .data = &sun55i_a523_spdif_quirks,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, sun4i_spdif_of_match);
    static const struct snd_soc_component_driver sun4i_spdif_component = {
    .name			= "sun4i-spdif",
    .legacy_dai_naming	= 1,
    };
#[no_mangle]
unsafe extern "C" fn sun4i_spdif_runtime_suspend(dev: *mut device) -> c_int {
    static int sun4i_spdif_runtime_suspend(struct device *dev)
    {
    struct sun4i_spdif_dev *host  = dev_get_drvdata(dev);
    clk_disable_unprepare(host.spdif_clk);
    clk_disable_unprepare(host.apb_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_spdif_runtime_resume(dev: *mut device) -> c_int {
    static int sun4i_spdif_runtime_resume(struct device *dev)
    {
    struct sun4i_spdif_dev *host  = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(host.spdif_clk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(host.apb_clk);
    if (ret)
    clk_disable_unprepare(host.spdif_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_spdif_probe(pdev: *mut platform_device) -> c_int {
    static int sun4i_spdif_probe(struct platform_device *pdev)
    {
    struct sun4i_spdif_dev *host;
    struct resource *res;
    const struct sun4i_spdif_quirks *quirks;
    int ret;
    void __iomem *base;
    const char *tx_clk_name = "spdif";
    dev_dbg(&pdev.dev, "Entered %s\n", __func__);
    host = devm_kzalloc(&pdev.dev, sizeof(*host), GFP_KERNEL);
    if (!host)
    return -ENOMEM;
    host.pdev = pdev;
    spin_lock_init(&host.lock);
// Initialize this copy of the CPU DAI driver structure
    memcpy(&host.cpu_dai_drv, &sun4i_spdif_dai, sizeof(sun4i_spdif_dai));
    host.cpu_dai_drv.name = dev_name(&pdev.dev);
// Get the addresses
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    quirks = of_device_get_match_data(&pdev.dev);
    if (quirks == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "Failed to determine the quirks to use\n");
    return -ENODEV;
    }
    host.quirks = quirks;
    host.regmap = devm_regmap_init_mmio(&pdev.dev, base,
    &sun4i_spdif_regmap_config);
    if (IS_ERR(host.regmap))
    return dev_err_probe(&pdev.dev, PTR_ERR(host.regmap),
    "failed to initialise regmap.\n");
// Clocks
    host.apb_clk = devm_clk_get(&pdev.dev, "apb");
    if (IS_ERR(host.apb_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(host.apb_clk),
    "failed to get a apb clock.\n");
    if (quirks.tx_clk_name)
    tx_clk_name = quirks.tx_clk_name;
    host.spdif_clk = devm_clk_get(&pdev.dev, tx_clk_name);
    if (IS_ERR(host.spdif_clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(host.spdif_clk),
    "failed to get the \"%s\" clock.\n",
    tx_clk_name);
    host.dma_params_tx.addr = res.start + quirks.reg_dac_txdata;
    host.dma_params_tx.maxburst = 8;
    host.dma_params_tx.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    platform_set_drvdata(pdev, host);
    if (quirks.has_reset) {
    host.rst = devm_reset_control_get_exclusive_deasserted(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(host.rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(host.rst),
    "Failed to get reset\n");
    }
    ret = devm_snd_soc_register_component(&pdev.dev,
    &sun4i_spdif_component, &sun4i_spdif_dai, 1);
    if (ret)
    return ret;
    pm_runtime_enable(&pdev.dev);
    if (!pm_runtime_enabled(&pdev.dev)) {
    ret = sun4i_spdif_runtime_resume(&pdev.dev);
    if (ret)
    goto err_unregister;
    }
    ret = devm_snd_dmaengine_pcm_register(&pdev.dev, core::ptr::null_mut(), 0);
    if (ret)
    goto err_suspend;
    return 0;
    err_suspend:
    if (!pm_runtime_status_suspended(&pdev.dev))
    sun4i_spdif_runtime_suspend(&pdev.dev);
    err_unregister:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_spdif_remove(pdev: *mut platform_device) {
    static void sun4i_spdif_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    sun4i_spdif_runtime_suspend(&pdev.dev);
    }
    static const struct dev_pm_ops sun4i_spdif_pm = {
    RUNTIME_PM_OPS(sun4i_spdif_runtime_suspend,
    sun4i_spdif_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver sun4i_spdif_driver = {
    .driver		= {
    .name	= "sun4i-spdif",
    .of_match_table = sun4i_spdif_of_match,
    .pm	= pm_ptr(&sun4i_spdif_pm),
    },
    .probe		= sun4i_spdif_probe,
    .remove		= sun4i_spdif_remove,
    };
    module_platform_driver(sun4i_spdif_driver);
    MODULE_AUTHOR("Marcus Cooper <codekipper@gmail.com>");
    MODULE_AUTHOR("Andrea Venturi <be17068@iperbole.bo.it>");
    MODULE_DESCRIPTION("Allwinner sun4i SPDIF SoC Interface");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:sun4i-spdif");
