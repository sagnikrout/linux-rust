//! Automatically rewritten from C to Rust
//! Source: sound/soc/meson/axg-pdm.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

pub const PDM_CTRL: c_uint = 0x00;

pub const PDM_HCIC_CTRL1: c_uint = 0x04;

pub const PDM_HCIC_CTRL2: c_uint = 0x08;
pub const PDM_F1_CTRL: c_uint = 0x0c;

pub const PDM_LPF_MAX_STAGE: c_int = 336;
pub const PDM_LPF_NUM: c_int = 3;
pub const PDM_F2_CTRL: c_uint = 0x10;
pub const PDM_F3_CTRL: c_uint = 0x14;
pub const PDM_HPF_CTRL: c_uint = 0x18;

pub const PDM_CHAN_CTRL: c_uint = 0x1c;
pub const PDM_CHAN_CTRL_POINTER_WIDTH: c_int = 8;

pub const PDM_CHAN_CTRL_NUM: c_int = 4;
pub const PDM_CHAN_CTRL1: c_uint = 0x20;
pub const PDM_COEFF_ADDR: c_uint = 0x24;
pub const PDM_COEFF_DATA: c_uint = 0x28;
pub const PDM_CLKG_CTRL: c_uint = 0x2c;
pub const PDM_STS: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_pdm_lpf {
    pub ds: c_uint,
    pub round_mode: c_uint,
    pub tap: *const c_uint,
    pub tap_num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_pdm_hcic {
    pub shift: c_uint,
    pub mult: c_uint,
    pub steps: c_uint,
    pub ds: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_pdm_hpf {
    pub out_factor: c_uint,
    pub steps: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_pdm_filters {
    pub hcic: axg_pdm_hcic,
    pub hpf: axg_pdm_hpf,
    pub lpf: [axg_pdm_lpf; PDM_LPF_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_pdm_cfg {
    pub filters: *const axg_pdm_filters,
    pub sys_rate: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_pdm {
    pub cfg: *const axg_pdm_cfg,
    pub map: *mut regmap,
    pub dclk: *mut clk,
    pub sysclk: *mut clk,
    pub pclk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn axg_pdm_enable(map: *mut regmap) {
    static void axg_pdm_enable(struct regmap *map)
    {
// Reset AFIFO
    regmap_update_bits(map, PDM_CTRL, PDM_CTRL_RST_FIFO, PDM_CTRL_RST_FIFO);
    regmap_update_bits(map, PDM_CTRL, PDM_CTRL_RST_FIFO, 0);
// Enable PDM
    regmap_update_bits(map, PDM_CTRL, PDM_CTRL_EN, PDM_CTRL_EN);
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_disable(map: *mut regmap) {
    static void axg_pdm_disable(struct regmap *map)
    {
    regmap_update_bits(map, PDM_CTRL, PDM_CTRL_EN, 0);
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_filters_enable(map: *mut regmap, enable: bool) {
    static void axg_pdm_filters_enable(struct regmap *map, bool enable)
    {
    let mut val: c_uint = enable ? PDM_FILTER_EN : 0;
    regmap_update_bits(map, PDM_HCIC_CTRL1, PDM_FILTER_EN, val);
    regmap_update_bits(map, PDM_F1_CTRL, PDM_FILTER_EN, val);
    regmap_update_bits(map, PDM_F2_CTRL, PDM_FILTER_EN, val);
    regmap_update_bits(map, PDM_F3_CTRL, PDM_FILTER_EN, val);
    regmap_update_bits(map, PDM_HPF_CTRL, PDM_FILTER_EN, val);
    }
    static int axg_pdm_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct axg_pdm *priv = snd_soc_dai_get_drvdata(dai);
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    axg_pdm_enable(priv.map);
    return 0;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    axg_pdm_disable(priv.map);
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_get_os(priv: *mut axg_pdm) -> c_uint {
    static unsigned int axg_pdm_get_os(struct axg_pdm *priv)
    {
    const struct axg_pdm_filters *filters = priv.cfg.filters;
    let mut os: c_uint = filters.hcic.ds;
    int i;
//
// The global oversampling factor is defined by the down sampling
// factor applied by each filter (HCIC and LPFs)
//
    for (i = 0; i < PDM_LPF_NUM; i++)
    os *= filters.lpf[i].ds;
    return os;
    }
    static int axg_pdm_set_sysclk(struct axg_pdm *priv, unsigned int os,
    unsigned int rate)
    {
    let mut sys_rate: c_uint = os * 2 * rate * PDM_CHAN_CTRL_POINTER_MAX;
//
// Set the default system clock rate unless it is too fast for
// the requested sample rate. In this case, the sample pointer
// counter could overflow so set a lower system clock rate
//
    if (sys_rate < priv.cfg.sys_rate)
    return clk_set_rate(priv.sysclk, sys_rate);
    return clk_set_rate(priv.sysclk, priv.cfg.sys_rate);
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_set_sample_pointer(priv: *mut axg_pdm) -> c_int {
    static int axg_pdm_set_sample_pointer(struct axg_pdm *priv)
    {
    unsigned int spmax, sp, val;
    int i;
// Max sample counter value per half period of dclk
    spmax = DIV_ROUND_UP_ULL((u64)clk_get_rate(priv.sysclk),
    clk_get_rate(priv.dclk) * 2);
// Check if sysclk is not too fast - should not happen
    if (WARN_ON(spmax > PDM_CHAN_CTRL_POINTER_MAX))
    return -EINVAL;
// Capture the data when we are at 75% of the half period
    sp = spmax * 3 / 4;
    for (i = 0, val = 0; i < PDM_CHAN_CTRL_NUM; i++)
    val |= sp << (PDM_CHAN_CTRL_POINTER_WIDTH * i);
    regmap_write(priv.map, PDM_CHAN_CTRL, val);
    regmap_write(priv.map, PDM_CHAN_CTRL1, val);
    return 0;
    }
    static void axg_pdm_set_channel_mask(struct axg_pdm *priv,
    unsigned int channels)
    {
    let mut mask: c_uint = GENMASK(channels - 1, 0);
// Put all channel in reset
    regmap_update_bits(priv.map, PDM_CTRL,
    PDM_CTRL_CHAN_RSTN_MASK, 0);
// Take the necessary channels out of reset and enable them
    regmap_update_bits(priv.map, PDM_CTRL,
    PDM_CTRL_CHAN_RSTN_MASK |
    PDM_CTRL_CHAN_EN_MASK,
    PDM_CTRL_CHAN_RSTN(mask) |
    PDM_CTRL_CHAN_EN(mask));
    }
    static int axg_pdm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct axg_pdm *priv = snd_soc_dai_get_drvdata(dai);
    let mut os: c_uint = axg_pdm_get_os(priv);
    let mut rate: c_uint = params_rate(params);
    unsigned int val;
    int ret;
    switch (params_width(params)) {
    case 24:
    val = PDM_CTRL_OUT_MODE;
    break;
    case 32:
    val = 0;
    break;
    default:
    dev_err(dai.dev, "unsupported sample width\n");
    return -EINVAL;
    }
    regmap_update_bits(priv.map, PDM_CTRL, PDM_CTRL_OUT_MODE, val);
    ret = axg_pdm_set_sysclk(priv, os, rate);
    if (ret) {
    dev_err(dai.dev, "failed to set system clock\n");
    return ret;
    }
    ret = clk_set_rate(priv.dclk, rate * os);
    if (ret) {
    dev_err(dai.dev, "failed to set dclk\n");
    return ret;
    }
    ret = axg_pdm_set_sample_pointer(priv);
    if (ret) {
    dev_err(dai.dev, "invalid clock setting\n");
    return ret;
    }
    axg_pdm_set_channel_mask(priv, params_channels(params));
    return 0;
    }
    static int axg_pdm_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct axg_pdm *priv = snd_soc_dai_get_drvdata(dai);
    int ret;
    ret = clk_prepare_enable(priv.dclk);
    if (ret) {
    dev_err(dai.dev, "enabling dclk failed\n");
    return ret;
    }
// Enable the filters
    axg_pdm_filters_enable(priv.map, true);
    return ret;
    }
    static void axg_pdm_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct axg_pdm *priv = snd_soc_dai_get_drvdata(dai);
    axg_pdm_filters_enable(priv.map, false);
    clk_disable_unprepare(priv.dclk);
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_set_hcic_ctrl(priv: *mut axg_pdm) {
    static void axg_pdm_set_hcic_ctrl(struct axg_pdm *priv)
    {
    const struct axg_pdm_hcic *hcic = &priv.cfg.filters.hcic;
    unsigned int val;
    val = PDM_HCIC_CTRL1_STAGE_NUM(hcic.steps);
    val |= PDM_HCIC_CTRL1_DSR(hcic.ds);
    val |= PDM_HCIC_CTRL1_GAIN_MULT(hcic.mult);
    val |= PDM_HCIC_CTRL1_GAIN_SFT(hcic.shift);
    regmap_update_bits(priv.map, PDM_HCIC_CTRL1,
    PDM_HCIC_CTRL1_STAGE_NUM_MASK |
    PDM_HCIC_CTRL1_DSR_MASK |
    PDM_HCIC_CTRL1_GAIN_MULT_MASK |
    PDM_HCIC_CTRL1_GAIN_SFT_MASK,
    val);
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_set_lpf_ctrl(priv: *mut axg_pdm, index: c_uint) {
    static void axg_pdm_set_lpf_ctrl(struct axg_pdm *priv, unsigned int index)
    {
    const struct axg_pdm_lpf *lpf = &priv.cfg.filters.lpf[index];
    unsigned int offset = index * regmap_get_reg_stride(priv.map)
    + PDM_F1_CTRL;
    unsigned int val;
    val = PDM_LPF_STAGE_NUM(lpf.tap_num);
    val |= PDM_LPF_DSR(lpf.ds);
    val |= PDM_LPF_ROUND_MODE(lpf.round_mode);
    regmap_update_bits(priv.map, offset,
    PDM_LPF_STAGE_NUM_MASK |
    PDM_LPF_DSR_MASK |
    PDM_LPF_ROUND_MODE_MASK,
    val);
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_set_hpf_ctrl(priv: *mut axg_pdm) {
    static void axg_pdm_set_hpf_ctrl(struct axg_pdm *priv)
    {
    const struct axg_pdm_hpf *hpf = &priv.cfg.filters.hpf;
    unsigned int val;
    val = PDM_HPF_OUT_FACTOR(hpf.out_factor);
    val |= PDM_HPF_SFT_STEPS(hpf.steps);
    regmap_update_bits(priv.map, PDM_HPF_CTRL,
    PDM_HPF_OUT_FACTOR_MASK |
    PDM_HPF_SFT_STEPS_MASK,
    val);
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_set_lpf_filters(priv: *mut axg_pdm) -> c_int {
    static int axg_pdm_set_lpf_filters(struct axg_pdm *priv)
    {
    const struct axg_pdm_lpf *lpf = priv.cfg.filters.lpf;
    let mut count: c_uint = 0;
    int i, j;
    for (i = 0; i < PDM_LPF_NUM; i++)
    count += lpf[i].tap_num;
// Make sure the coeffs fit in the memory
    if (count >= PDM_LPF_MAX_STAGE)
    return -EINVAL;
// Set the initial APB bus register address
    regmap_write(priv.map, PDM_COEFF_ADDR, 0);
// Set the tap filter values of all 3 filters
    for (i = 0; i < PDM_LPF_NUM; i++) {
    axg_pdm_set_lpf_ctrl(priv, i);
    for (j = 0; j < lpf[i].tap_num; j++)
    regmap_write(priv.map, PDM_COEFF_DATA, lpf[i].tap[j]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    static int axg_pdm_dai_probe(struct snd_soc_dai *dai)
    {
    struct axg_pdm *priv = snd_soc_dai_get_drvdata(dai);
    int ret;
    ret = clk_prepare_enable(priv.pclk);
    if (ret) {
    dev_err(dai.dev, "enabling pclk failed\n");
    return ret;
    }
//
// sysclk must be set and enabled as well to access the pdm registers
// Accessing the register w/o it will give a bus error.
//
    ret = clk_set_rate(priv.sysclk, priv.cfg.sys_rate);
    if (ret) {
    dev_err(dai.dev, "setting sysclk failed\n");
    goto err_pclk;
    }
    ret = clk_prepare_enable(priv.sysclk);
    if (ret) {
    dev_err(dai.dev, "enabling sysclk failed\n");
    goto err_pclk;
    }
// Make sure the device is initially disabled
    axg_pdm_disable(priv.map);
// Make sure filter bypass is disabled
    regmap_update_bits(priv.map, PDM_CTRL, PDM_CTRL_BYPASS_MODE, 0);
// Load filter settings
    axg_pdm_set_hcic_ctrl(priv);
    axg_pdm_set_hpf_ctrl(priv);
    ret = axg_pdm_set_lpf_filters(priv);
    if (ret) {
    dev_err(dai.dev, "invalid filter configuration\n");
    goto err_sysclk;
    }
    return 0;
    err_sysclk:
    clk_disable_unprepare(priv.sysclk);
    err_pclk:
    clk_disable_unprepare(priv.pclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn axg_pdm_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    static int axg_pdm_dai_remove(struct snd_soc_dai *dai)
    {
    struct axg_pdm *priv = snd_soc_dai_get_drvdata(dai);
    clk_disable_unprepare(priv.sysclk);
    clk_disable_unprepare(priv.pclk);
    return 0;
    }
    static const struct snd_soc_dai_ops axg_pdm_dai_ops = {
    .probe		= axg_pdm_dai_probe,
    .remove		= axg_pdm_dai_remove,
    .trigger	= axg_pdm_trigger,
    .hw_params	= axg_pdm_hw_params,
    .startup	= axg_pdm_startup,
    .shutdown	= axg_pdm_shutdown,
    };
    static struct snd_soc_dai_driver axg_pdm_dai_drv = {
    .name = "PDM",
    .capture = {
    .stream_name	= "Capture",
    .channels_min	= 1,
    .channels_max	= 8,
    .rates		= SNDRV_PCM_RATE_CONTINUOUS,
    .rate_min	= 5512,
    .rate_max	= 48000,
    .formats	= (SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32_LE),
    },
    .ops		= &axg_pdm_dai_ops,
    };
    static const struct snd_soc_component_driver axg_pdm_component_drv = {
    .legacy_dai_naming = 1,
    };
    static const struct regmap_config axg_pdm_regmap_cfg = {
    .reg_bits	= 32,
    .val_bits	= 32,
    .reg_stride	= 4,
    .max_register	= PDM_STS,
    };
    static const unsigned int lpf1_default_tap[] = {
    0x000014, 0xffffb2, 0xfffed9, 0xfffdce, 0xfffd45,
    0xfffe32, 0x000147, 0x000645, 0x000b86, 0x000e21,
    0x000ae3, 0x000000, 0xffeece, 0xffdca8, 0xffd212,
    0xffd7d1, 0xfff2a7, 0x001f4c, 0x0050c2, 0x0072aa,
    0x006ff1, 0x003c32, 0xffdc4e, 0xff6a18, 0xff0fef,
    0xfefbaf, 0xff4c40, 0x000000, 0x00ebc8, 0x01c077,
    0x02209e, 0x01c1a4, 0x008e60, 0xfebe52, 0xfcd690,
    0xfb8fa5, 0xfba498, 0xfd9812, 0x0181ce, 0x06f5f3,
    0x0d112f, 0x12a958, 0x169686, 0x18000e, 0x169686,
    0x12a958, 0x0d112f, 0x06f5f3, 0x0181ce, 0xfd9812,
    0xfba498, 0xfb8fa5, 0xfcd690, 0xfebe52, 0x008e60,
    0x01c1a4, 0x02209e, 0x01c077, 0x00ebc8, 0x000000,
    0xff4c40, 0xfefbaf, 0xff0fef, 0xff6a18, 0xffdc4e,
    0x003c32, 0x006ff1, 0x0072aa, 0x0050c2, 0x001f4c,
    0xfff2a7, 0xffd7d1, 0xffd212, 0xffdca8, 0xffeece,
    0x000000, 0x000ae3, 0x000e21, 0x000b86, 0x000645,
    0x000147, 0xfffe32, 0xfffd45, 0xfffdce, 0xfffed9,
    0xffffb2, 0x000014,
    };
    static const unsigned int lpf2_default_tap[] = {
    0x00050a, 0xfff004, 0x0002c1, 0x003c12, 0xffa818,
    0xffc87d, 0x010aef, 0xff5223, 0xfebd93, 0x028f41,
    0xff5c0e, 0xfc63f8, 0x055f81, 0x000000, 0xf478a0,
    0x11c5e3, 0x2ea74d, 0x11c5e3, 0xf478a0, 0x000000,
    0x055f81, 0xfc63f8, 0xff5c0e, 0x028f41, 0xfebd93,
    0xff5223, 0x010aef, 0xffc87d, 0xffa818, 0x003c12,
    0x0002c1, 0xfff004, 0x00050a,
    };
    static const unsigned int lpf3_default_tap[] = {
    0x000000, 0x000081, 0x000000, 0xfffedb, 0x000000,
    0x00022d, 0x000000, 0xfffc46, 0x000000, 0x0005f7,
    0x000000, 0xfff6eb, 0x000000, 0x000d4e, 0x000000,
    0xffed1e, 0x000000, 0x001a1c, 0x000000, 0xffdcb0,
    0x000000, 0x002ede, 0x000000, 0xffc2d1, 0x000000,
    0x004ebe, 0x000000, 0xff9beb, 0x000000, 0x007dd7,
    0x000000, 0xff633a, 0x000000, 0x00c1d2, 0x000000,
    0xff11d5, 0x000000, 0x012368, 0x000000, 0xfe9c45,
    0x000000, 0x01b252, 0x000000, 0xfdebf6, 0x000000,
    0x0290b8, 0x000000, 0xfcca0d, 0x000000, 0x041d7c,
    0x000000, 0xfa8152, 0x000000, 0x07e9c6, 0x000000,
    0xf28fb5, 0x000000, 0x28b216, 0x3fffde, 0x28b216,
    0x000000, 0xf28fb5, 0x000000, 0x07e9c6, 0x000000,
    0xfa8152, 0x000000, 0x041d7c, 0x000000, 0xfcca0d,
    0x000000, 0x0290b8, 0x000000, 0xfdebf6, 0x000000,
    0x01b252, 0x000000, 0xfe9c45, 0x000000, 0x012368,
    0x000000, 0xff11d5, 0x000000, 0x00c1d2, 0x000000,
    0xff633a, 0x000000, 0x007dd7, 0x000000, 0xff9beb,
    0x000000, 0x004ebe, 0x000000, 0xffc2d1, 0x000000,
    0x002ede, 0x000000, 0xffdcb0, 0x000000, 0x001a1c,
    0x000000, 0xffed1e, 0x000000, 0x000d4e, 0x000000,
    0xfff6eb, 0x000000, 0x0005f7, 0x000000, 0xfffc46,
    0x000000, 0x00022d, 0x000000, 0xfffedb, 0x000000,
    0x000081, 0x000000,
    };
//
// These values are sane defaults for the axg platform:
// - OS = 64
// - Latency = 38700 (?)
//
// TODO: There is a lot of different HCIC, LPFs and HPF configurations possible.
// the configuration may depend on the dmic used by the platform, the
// expected tradeoff between latency and quality, etc ... If/When other
// settings are required, we should add a fw interface to this driver to
// load new filter settings.
//
    static const struct axg_pdm_filters axg_default_filters = {
    .hcic = {
    .shift = 0x15,
    .mult = 0x80,
    .steps = 7,
    .ds = 8,
    },
    .hpf = {
    .out_factor = 0x8000,
    .steps = 13,
    },
    .lpf = {
    [0] = {
    .ds = 2,
    .round_mode = 1,
    .tap = lpf1_default_tap,
    .tap_num = ARRAY_SIZE(lpf1_default_tap),
    },
    [1] = {
    .ds = 2,
    .round_mode = 0,
    .tap = lpf2_default_tap,
    .tap_num = ARRAY_SIZE(lpf2_default_tap),
    },
    [2] = {
    .ds = 2,
    .round_mode = 1,
    .tap = lpf3_default_tap,
    .tap_num = ARRAY_SIZE(lpf3_default_tap)
    },
    },
    };
    static const struct axg_pdm_cfg axg_pdm_config = {
    .filters = &axg_default_filters,
    .sys_rate = 250000000,
    };
    static const struct of_device_id axg_pdm_of_match[] = {
    {
    .compatible = "amlogic,axg-pdm",
    .data = &axg_pdm_config,
    }, {}
    };
    MODULE_DEVICE_TABLE(of, axg_pdm_of_match);
#[no_mangle]
unsafe extern "C" fn axg_pdm_probe(pdev: *mut platform_device) -> c_int {
    static int axg_pdm_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct axg_pdm *priv;
    void __iomem *regs;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    priv.cfg = of_device_get_match_data(dev);
    if (!priv.cfg) {
    dev_err(dev, "failed to match device\n");
    return -ENODEV;
    }
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    priv.map = devm_regmap_init_mmio(dev, regs, &axg_pdm_regmap_cfg);
    if (IS_ERR(priv.map)) {
    dev_err(dev, "failed to init regmap: %ld\n",
    PTR_ERR(priv.map));
    return PTR_ERR(priv.map);
    }
    priv.pclk = devm_clk_get(dev, "pclk");
    if (IS_ERR(priv.pclk))
    return dev_err_probe(dev, PTR_ERR(priv.pclk), "failed to get pclk\n");
    priv.dclk = devm_clk_get(dev, "dclk");
    if (IS_ERR(priv.dclk))
    return dev_err_probe(dev, PTR_ERR(priv.dclk), "failed to get dclk\n");
    priv.sysclk = devm_clk_get(dev, "sysclk");
    if (IS_ERR(priv.sysclk))
    return dev_err_probe(dev, PTR_ERR(priv.sysclk), "failed to get dclk\n");
    return devm_snd_soc_register_component(dev, &axg_pdm_component_drv,
    &axg_pdm_dai_drv, 1);
    }
    static struct platform_driver axg_pdm_pdrv = {
    .probe = axg_pdm_probe,
    .driver = {
    .name = "axg-pdm",
    .of_match_table = axg_pdm_of_match,
    },
    };
    module_platform_driver(axg_pdm_pdrv);
    MODULE_DESCRIPTION("Amlogic AXG PDM Input driver");
    MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
    MODULE_LICENSE("GPL v2");
