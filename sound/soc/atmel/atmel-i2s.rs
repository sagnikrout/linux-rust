//! Automatically rewritten from C to Rust
//! Source: sound/soc/atmel/atmel-i2s.c
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
// Driver for Atmel I2S controller
//
// Copyright (C) 2015 Atmel Corporation
//
// Author: Cyrille Pitchen <cyrille.pitchen@atmel.com>
//

pub const ATMEL_I2SC_MAX_TDM_CHANNELS: c_int = 8;
//
// ---- I2S Controller Register map ----
//
pub const ATMEL_I2SC_CR: c_uint = 0x0000	/* Control Register */;
pub const ATMEL_I2SC_MR: c_uint = 0x0004	/* Mode Register */;
pub const ATMEL_I2SC_SR: c_uint = 0x0008	/* Status Register */;
pub const ATMEL_I2SC_SCR: c_uint = 0x000c	/* Status Clear Register */;
pub const ATMEL_I2SC_SSR: c_uint = 0x0010	/* Status Set Register */;
pub const ATMEL_I2SC_IER: c_uint = 0x0014	/* Interrupt Enable Register */;
pub const ATMEL_I2SC_IDR: c_uint = 0x0018	/* Interrupt Disable Register */;
pub const ATMEL_I2SC_IMR: c_uint = 0x001c	/* Interrupt Mask Register */;
pub const ATMEL_I2SC_RHR: c_uint = 0x0020	/* Receiver Holding Register */;
pub const ATMEL_I2SC_THR: c_uint = 0x0024	/* Transmitter Holding Register */;
pub const ATMEL_I2SC_VERSION: c_uint = 0x0028	/* Version Register */;
//
// ---- Control Register (Write-only) ----
//

//
// ---- Mode Register (Read/Write) ----
//

// Left audio samples duplicated to right audio channel

// Receiver uses one DMA channel ...

// I2SDO output of I2SC is internally connected to I2SDI input

// Left audio samples duplicated to right audio channel

// Transmitter uses one DMA channel ...

// x sample transmitted when underrun

// Audio Clock to I2SC Master Clock ratio

    (((div) << 16) & ATMEL_I2SC_MR_IMCKDIV_MASK)
// Master Clock to fs ratio

    (((fs) << 24) & ATMEL_I2SC_MR_IMCKFS_MASK)
// Master Clock mode

// 0: No master clock generated (selected clock drives I2SCK pin)

// 1: master clock generated (internally generated clock drives I2SMCK pin)

// Slot Width
// 0: slot is 32 bits wide for DATALENGTH = 18/20/24 bits.
// 1: slot is 24 bits wide for DATALENGTH = 18/20/24 bits.

//
// ---- Status Registers ----
//

// Receive Overrun Channel

// Transmit Underrun Channel

//
// ---- Interrupt Enable/Disable/Mask Registers ----
//

    static const struct regmap_config atmel_i2s_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = ATMEL_I2SC_VERSION,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_i2s_gck_param {
    pub fs: c_int,
    pub mck: c_ulong,
    pub imckdiv: c_int,
    pub imckfs: c_int,
}

// mck = (32 * (imckfs+1) / (imckdiv+1)) * fs
    static const struct atmel_i2s_gck_param gck_params[] = {
// mck = 6.144Mhz
    {  8000, I2S_MCK_6M144,  1, 47},	/* mck =  768 fs */
// mck = 12.288MHz
    { 16000, I2S_MCK_12M288, 1, 47},	/* mck =  768 fs */
    { 24000, I2S_MCK_12M288, 3, 63},	/* mck =  512 fs */
    { 32000, I2S_MCK_12M288, 3, 47},	/* mck =  384 fs */
    { 48000, I2S_MCK_12M288, 7, 63},	/* mck =  256 fs */
    { 64000, I2S_MCK_12M288, 7, 47},	/* mck =  192 fs */
    { 96000, I2S_MCK_12M288, 7, 31},	/* mck =  128 fs */
    {192000, I2S_MCK_12M288, 7, 15},	/* mck =   64 fs */
// mck = 11.2896MHz
    { 11025, I2S_MCK_11M2896, 1, 63},	/* mck = 1024 fs */
    { 22050, I2S_MCK_11M2896, 3, 63},	/* mck =  512 fs */
    { 44100, I2S_MCK_11M2896, 7, 63},	/* mck =  256 fs */
    { 88200, I2S_MCK_11M2896, 7, 31},	/* mck =  128 fs */
    {176400, I2S_MCK_11M2896, 7, 15},	/* mck =   64 fs */
    };
    struct atmel_i2s_dev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_i2s_caps {
    pub np): *mut *mut *mut int (mck_init)(struct atmel_i2s_dev , struct device_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_i2s_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pclk: *mut clk,
    pub gclk: *mut clk,
    pub playback: snd_dmaengine_dai_dma_data,
    pub capture: snd_dmaengine_dai_dma_data,
    pub fmt: c_uint,
    pub gck_param: *const atmel_i2s_gck_param,
    pub caps: *const atmel_i2s_caps,
    pub clk_use_no: c_int,
}

#[no_mangle]
unsafe extern "C" fn atmel_i2s_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t atmel_i2s_interrupt(int irq, void *dev_id)
    {
    struct atmel_i2s_dev *dev = dev_id;
    unsigned int sr, imr, pending, ch, mask;
    let mut ret: irqreturn_t = IRQ_NONE;
    regmap_read(dev.regmap, ATMEL_I2SC_SR, &sr);
    regmap_read(dev.regmap, ATMEL_I2SC_IMR, &imr);
    pending = sr & imr;
    if (!pending)
    return IRQ_NONE;
    if (pending & ATMEL_I2SC_INT_RXOR) {
    mask = ATMEL_I2SC_SR_RXOR;
    for (ch = 0; ch < ATMEL_I2SC_MAX_TDM_CHANNELS; ++ch) {
    if (sr & ATMEL_I2SC_SR_RXORCH(ch)) {
    mask |= ATMEL_I2SC_SR_RXORCH(ch);
    dev_err(dev.dev,
    "RX overrun on channel %d\n", ch);
    }
    }
    regmap_write(dev.regmap, ATMEL_I2SC_SCR, mask);
    ret = IRQ_HANDLED;
    }
    if (pending & ATMEL_I2SC_INT_TXUR) {
    mask = ATMEL_I2SC_SR_TXUR;
    for (ch = 0; ch < ATMEL_I2SC_MAX_TDM_CHANNELS; ++ch) {
    if (sr & ATMEL_I2SC_SR_TXURCH(ch)) {
    mask |= ATMEL_I2SC_SR_TXURCH(ch);
    dev_err(dev.dev,
    "TX underrun on channel %d\n", ch);
    }
    }
    regmap_write(dev.regmap, ATMEL_I2SC_SCR, mask);
    ret = IRQ_HANDLED;
    }
    return ret;
    }

    SNDRV_PCM_FMTBIT_S16_LE |	\
    SNDRV_PCM_FMTBIT_S18_3LE |	\
    SNDRV_PCM_FMTBIT_S20_3LE |	\
    SNDRV_PCM_FMTBIT_S24_3LE |	\
    SNDRV_PCM_FMTBIT_S24_LE |	\
    SNDRV_PCM_FMTBIT_S32_LE)
#[no_mangle]
unsafe extern "C" fn atmel_i2s_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int atmel_i2s_set_dai_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct atmel_i2s_dev *dev = snd_soc_dai_get_drvdata(dai);
    dev.fmt = fmt;
    return 0;
    }
    static int atmel_i2s_prepare(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct atmel_i2s_dev *dev = snd_soc_dai_get_drvdata(dai);
    let mut is_playback: bool = (substream.stream == SNDRV_PCM_STREAM_PLAYBACK);
    unsigned int rhr, sr = 0;
    if (is_playback) {
    regmap_read(dev.regmap, ATMEL_I2SC_SR, &sr);
    if (sr & ATMEL_I2SC_SR_RXRDY) {
//
// The RX Ready flag should not be set. However if here,
// we flush (read) the Receive Holding Register to start
// from a clean state.
//
    dev_dbg(dev.dev, "RXRDY is set\n");
    regmap_read(dev.regmap, ATMEL_I2SC_RHR, &rhr);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_i2s_get_gck_param(dev: *mut atmel_i2s_dev, fs: c_int) -> c_int {
    static int atmel_i2s_get_gck_param(struct atmel_i2s_dev *dev, int fs)
    {
    int i, best;
    if (!dev.gclk) {
    dev_err(dev.dev, "cannot generate the I2S Master Clock\n");
    return -EINVAL;
    }
//
// Find the best possible settings to generate the I2S Master Clock
// from the PLL Audio.
//
    dev.gck_param = core::ptr::null_mut();
    best = INT_MAX;
    for (i = 0; i < ARRAY_SIZE(gck_params); ++i) {
    const struct atmel_i2s_gck_param *gck_param = &gck_params[i];
    let mut val: c_int = abs(fs - gck_param.fs);
    if (val < best) {
    best = val;
    dev.gck_param = gck_param;
    }
    }
    return 0;
    }
    static int atmel_i2s_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct atmel_i2s_dev *dev = snd_soc_dai_get_drvdata(dai);
    let mut is_playback: bool = (substream.stream == SNDRV_PCM_STREAM_PLAYBACK);
    let mut mr: c_uint = 0, mr_mask;
    int ret;
    mr_mask = ATMEL_I2SC_MR_FORMAT_MASK | ATMEL_I2SC_MR_MODE_MASK |
    ATMEL_I2SC_MR_DATALENGTH_MASK;
    if (is_playback)
    mr_mask |= ATMEL_I2SC_MR_TXMONO;
    else
    mr_mask |= ATMEL_I2SC_MR_RXMONO;
    switch (dev.fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_I2S:
    mr |= ATMEL_I2SC_MR_FORMAT_I2S;
    break;
    default:
    dev_err(dev.dev, "unsupported bus format\n");
    return -EINVAL;
    }
    switch (dev.fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) {
    case SND_SOC_DAIFMT_BP_FP:
// codec is slave, so cpu is master
    mr |= ATMEL_I2SC_MR_MODE_MASTER;
    ret = atmel_i2s_get_gck_param(dev, params_rate(params));
    if (ret)
    return ret;
    break;
    case SND_SOC_DAIFMT_BC_FC:
// codec is master, so cpu is slave
    mr |= ATMEL_I2SC_MR_MODE_SLAVE;
    dev.gck_param = core::ptr::null_mut();
    break;
    default:
    dev_err(dev.dev, "unsupported master/slave mode\n");
    return -EINVAL;
    }
    switch (params_channels(params)) {
    case 1:
    if (is_playback)
    mr |= ATMEL_I2SC_MR_TXMONO;
    else
    mr |= ATMEL_I2SC_MR_RXMONO;
    break;
    case 2:
    break;
    default:
    dev_err(dev.dev, "unsupported number of audio channels\n");
    return -EINVAL;
    }
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_S8:
    mr |= ATMEL_I2SC_MR_DATALENGTH_8_BITS;
    break;
    case SNDRV_PCM_FORMAT_S16_LE:
    mr |= ATMEL_I2SC_MR_DATALENGTH_16_BITS;
    break;
    case SNDRV_PCM_FORMAT_S18_3LE:
    mr |= ATMEL_I2SC_MR_DATALENGTH_18_BITS | ATMEL_I2SC_MR_IWS;
    break;
    case SNDRV_PCM_FORMAT_S20_3LE:
    mr |= ATMEL_I2SC_MR_DATALENGTH_20_BITS | ATMEL_I2SC_MR_IWS;
    break;
    case SNDRV_PCM_FORMAT_S24_3LE:
    mr |= ATMEL_I2SC_MR_DATALENGTH_24_BITS | ATMEL_I2SC_MR_IWS;
    break;
    case SNDRV_PCM_FORMAT_S24_LE:
    mr |= ATMEL_I2SC_MR_DATALENGTH_24_BITS;
    break;
    case SNDRV_PCM_FORMAT_S32_LE:
    mr |= ATMEL_I2SC_MR_DATALENGTH_32_BITS;
    break;
    default:
    dev_err(dev.dev, "unsupported size/endianness for audio samples\n");
    return -EINVAL;
    }
    return regmap_update_bits(dev.regmap, ATMEL_I2SC_MR, mr_mask, mr);
    }
    static int atmel_i2s_switch_mck_generator(struct atmel_i2s_dev *dev,
    bool enabled)
    {
    unsigned int mr, mr_mask;
    unsigned long gclk_rate;
    int ret;
    mr = 0;
    mr_mask = (ATMEL_I2SC_MR_IMCKDIV_MASK |
    ATMEL_I2SC_MR_IMCKFS_MASK |
    ATMEL_I2SC_MR_IMCKMODE_MASK);
    if (!enabled) {
// Disable the I2S Master Clock generator.
    ret = regmap_write(dev.regmap, ATMEL_I2SC_CR,
    ATMEL_I2SC_CR_CKDIS);
    if (ret)
    return ret;
// Reset the I2S Master Clock generator settings.
    ret = regmap_update_bits(dev.regmap, ATMEL_I2SC_MR,
    mr_mask, mr);
    if (ret)
    return ret;
// Disable/unprepare the PMC generated clock.
    clk_disable_unprepare(dev.gclk);
    return 0;
    }
    if (!dev.gck_param)
    return -EINVAL;
    gclk_rate = dev.gck_param.mck * (dev.gck_param.imckdiv + 1);
    ret = clk_set_rate(dev.gclk, gclk_rate);
    if (ret)
    return ret;
    ret = clk_prepare_enable(dev.gclk);
    if (ret)
    return ret;
// Update the Mode Register to generate the I2S Master Clock.
    mr |= ATMEL_I2SC_MR_IMCKDIV(dev.gck_param.imckdiv);
    mr |= ATMEL_I2SC_MR_IMCKFS(dev.gck_param.imckfs);
    mr |= ATMEL_I2SC_MR_IMCKMODE_I2SMCK;
    ret = regmap_update_bits(dev.regmap, ATMEL_I2SC_MR, mr_mask, mr);
    if (ret)
    return ret;
// Finally enable the I2S Master Clock generator.
    return regmap_write(dev.regmap, ATMEL_I2SC_CR,
    ATMEL_I2SC_CR_CKEN);
    }
    static int atmel_i2s_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct atmel_i2s_dev *dev = snd_soc_dai_get_drvdata(dai);
    let mut is_playback: bool = (substream.stream == SNDRV_PCM_STREAM_PLAYBACK);
    bool is_master, mck_enabled;
    unsigned int cr, mr;
    int err;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    cr = is_playback ? ATMEL_I2SC_CR_TXEN : ATMEL_I2SC_CR_RXEN;
    mck_enabled = true;
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    cr = is_playback ? ATMEL_I2SC_CR_TXDIS : ATMEL_I2SC_CR_RXDIS;
    mck_enabled = false;
    break;
    default:
    return -EINVAL;
    }
// Read the Mode Register to retrieve the master/slave state.
    err = regmap_read(dev.regmap, ATMEL_I2SC_MR, &mr);
    if (err)
    return err;
    is_master = (mr & ATMEL_I2SC_MR_MODE_MASK) == ATMEL_I2SC_MR_MODE_MASTER;
// If master starts, enable the audio clock.
    if (is_master && mck_enabled) {
    if (!dev.clk_use_no) {
    err = atmel_i2s_switch_mck_generator(dev, true);
    if (err)
    return err;
    }
    dev.clk_use_no++;
    }
    err = regmap_write(dev.regmap, ATMEL_I2SC_CR, cr);
    if (err)
    return err;
// If master stops, disable the audio clock.
    if (is_master && !mck_enabled) {
    if (dev.clk_use_no == 1) {
    err = atmel_i2s_switch_mck_generator(dev, false);
    if (err)
    return err;
    }
    dev.clk_use_no--;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn atmel_i2s_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int atmel_i2s_dai_probe(struct snd_soc_dai *dai)
    {
    struct atmel_i2s_dev *dev = snd_soc_dai_get_drvdata(dai);
    snd_soc_dai_init_dma_data(dai, &dev.playback, &dev.capture);
    return 0;
    }
    let mut atmel_i2s_selectable_formats: static u64 = SND_SOC_POSSIBLE_DAIFMT_I2S;
    static const struct snd_soc_dai_ops atmel_i2s_dai_ops = {
    .probe		= atmel_i2s_dai_probe,
    .prepare	= atmel_i2s_prepare,
    .trigger	= atmel_i2s_trigger,
    .hw_params	= atmel_i2s_hw_params,
    .set_fmt	= atmel_i2s_set_dai_fmt,
    .auto_selectable_formats	= &atmel_i2s_selectable_formats,
    .num_auto_selectable_formats	= 1,
    };
    static struct snd_soc_dai_driver atmel_i2s_dai = {
    .playback = {
    .channels_min = 1,
    .channels_max = 2,
    .rates = ATMEL_I2S_RATES,
    .formats = ATMEL_I2S_FORMATS,
    },
    .capture = {
    .channels_min = 1,
    .channels_max = 2,
    .rates = ATMEL_I2S_RATES,
    .formats = ATMEL_I2S_FORMATS,
    },
    .ops = &atmel_i2s_dai_ops,
    .symmetric_rate = 1,
    .symmetric_sample_bits = 1,
    };
    static const struct snd_soc_component_driver atmel_i2s_component = {
    .name			= "atmel-i2s",
    .legacy_dai_naming	= 1,
    };
    static int atmel_i2s_sama5d2_mck_init(struct atmel_i2s_dev *dev,
    struct device_node *np)
    {
    struct clk *muxclk;
    int err;
    if (!dev.gclk)
    return 0;
// muxclk is optional, so we return error for probe defer only
    muxclk = devm_clk_get(dev.dev, "muxclk");
    if (IS_ERR(muxclk)) {
    err = PTR_ERR(muxclk);
    if (err == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    dev_dbg(dev.dev,
    "failed to get the I2S clock control: %d\n", err);
    return 0;
    }
    return clk_set_parent(muxclk, dev.gclk);
    }
    static const struct atmel_i2s_caps atmel_i2s_sama5d2_caps = {
    .mck_init = atmel_i2s_sama5d2_mck_init,
    };
    static const struct of_device_id atmel_i2s_dt_ids[] = {
    {
    .compatible = "atmel,sama5d2-i2s",
    .data = (void *)&atmel_i2s_sama5d2_caps,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmel_i2s_dt_ids);
#[no_mangle]
unsafe extern "C" fn atmel_i2s_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_i2s_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    const struct of_device_id *match;
    struct atmel_i2s_dev *dev;
    struct resource *mem;
    struct regmap *regmap;
    void __iomem *base;
    int irq;
    int err;
    let mut pcm_flags: c_uint = 0;
    unsigned int version;
// Get memory for driver data.
    dev = devm_kzalloc(&pdev.dev, sizeof(*dev), GFP_KERNEL);
    if (!dev)
    return -ENOMEM;
// Get hardware capabilities.
    match = of_match_node(atmel_i2s_dt_ids, np);
    if (match)
    dev.caps = match.data;
// Map I/O registers.
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mem);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = devm_regmap_init_mmio(&pdev.dev, base,
    &atmel_i2s_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
// Request IRQ.
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    err = devm_request_irq(&pdev.dev, irq, atmel_i2s_interrupt, 0,
    dev_name(&pdev.dev), dev);
    if (err)
    return err;
// Get the peripheral clock.
    dev.pclk = devm_clk_get(&pdev.dev, "pclk");
    if (IS_ERR(dev.pclk)) {
    err = PTR_ERR(dev.pclk);
    dev_err(&pdev.dev,
    "failed to get the peripheral clock: %d\n", err);
    return err;
    }
// Get audio clock to generate the I2S Master Clock (I2S_MCK)
    dev.gclk = devm_clk_get(&pdev.dev, "gclk");
    if (IS_ERR(dev.gclk)) {
    if (PTR_ERR(dev.gclk) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
// Master Mode not supported
    dev.gclk = core::ptr::null_mut();
    }
    dev.dev = &pdev.dev;
    dev.regmap = regmap;
    platform_set_drvdata(pdev, dev);
// Do hardware specific settings to initialize I2S_MCK generator
    if (dev.caps && dev.caps.mck_init) {
    err = dev.caps.mck_init(dev, np);
    if (err)
    return err;
    }
// Enable the peripheral clock.
    err = clk_prepare_enable(dev.pclk);
    if (err)
    return err;
// Get IP version.
    regmap_read(dev.regmap, ATMEL_I2SC_VERSION, &version);
    dev_info(&pdev.dev, "hw version: %#x\n", version);
// Enable error interrupts.
    regmap_write(dev.regmap, ATMEL_I2SC_IER,
    ATMEL_I2SC_INT_RXOR | ATMEL_I2SC_INT_TXUR);
    err = devm_snd_soc_register_component(&pdev.dev,
    &atmel_i2s_component,
    &atmel_i2s_dai, 1);
    if (err) {
    dev_err(&pdev.dev, "failed to register DAI: %d\n", err);
    clk_disable_unprepare(dev.pclk);
    return err;
    }
// Prepare DMA config.
    dev.playback.addr	= (dma_addr_t)mem.start + ATMEL_I2SC_THR;
    dev.playback.maxburst	= 1;
    dev.capture.addr	= (dma_addr_t)mem.start + ATMEL_I2SC_RHR;
    dev.capture.maxburst	= 1;
    if (of_property_match_string(np, "dma-names", "rx-tx") == 0)
    pcm_flags |= SND_DMAENGINE_PCM_FLAG_HALF_DUPLEX;
    err = devm_snd_dmaengine_pcm_register(&pdev.dev, core::ptr::null_mut(), pcm_flags);
    if (err) {
    dev_err(&pdev.dev, "failed to register PCM: %d\n", err);
    clk_disable_unprepare(dev.pclk);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_i2s_remove(pdev: *mut platform_device) {
    static void atmel_i2s_remove(struct platform_device *pdev)
    {
    struct atmel_i2s_dev *dev = platform_get_drvdata(pdev);
    clk_disable_unprepare(dev.pclk);
    }
    static struct platform_driver atmel_i2s_driver = {
    .driver		= {
    .name	= "atmel_i2s",
    .of_match_table	= atmel_i2s_dt_ids,
    },
    .probe		= atmel_i2s_probe,
    .remove		= atmel_i2s_remove,
    };
    module_platform_driver(atmel_i2s_driver);
    MODULE_DESCRIPTION("Atmel I2S Controller driver");
    MODULE_AUTHOR("Cyrille Pitchen <cyrille.pitchen@atmel.com>");
    MODULE_LICENSE("GPL v2");
