//! Automatically rewritten from C to Rust
//! Source: sound/soc/mediatek/mt8365/mt8365-dai-dmic.c
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
// MediaTek 8365 ALSA SoC Audio DAI DMIC Control
//
// Copyright (c) 2024 MediaTek Inc.
// Authors: Jia Zeng <jia.zeng@mediatek.com>
// Alexandre Mergnat <amergnat@baylibre.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt8365_dmic_data {
    pub two_wire_mode: bool,
    pub clk_phase_sel_ch1: c_uint,
    pub clk_phase_sel_ch2: c_uint,
    pub iir_on: bool,
    pub irr_mode: c_uint,
    pub dmic_mode: c_uint,
    pub dmic_channel: c_uint,
}

#[no_mangle]
unsafe extern "C" fn get_chan_reg(channel: c_uint) -> c_int {
    static int get_chan_reg(unsigned int channel)
    {
    switch (channel) {
    case 8:
    fallthrough;
    case 7:
    return AFE_DMIC3_UL_SRC_CON0;
    case 6:
    fallthrough;
    case 5:
    return AFE_DMIC2_UL_SRC_CON0;
    case 4:
    fallthrough;
    case 3:
    return AFE_DMIC1_UL_SRC_CON0;
    case 2:
    fallthrough;
    case 1:
    return AFE_DMIC0_UL_SRC_CON0;
    default:
    return -EINVAL;
    }
    }
// DAI Drivers
#[no_mangle]
unsafe extern "C" fn audio_dmic_adda_enable(afe: *mut mtk_base_afe) {
    static void audio_dmic_adda_enable(struct mtk_base_afe *afe)
    {
    mt8365_dai_enable_adda_on(afe);
    regmap_update_bits(afe.regmap, AFE_ADDA_UL_DL_CON0,
    AFE_ADDA_UL_DL_DMIC_CLKDIV_ON,
    AFE_ADDA_UL_DL_DMIC_CLKDIV_ON);
    }
#[no_mangle]
unsafe extern "C" fn audio_dmic_adda_disable(afe: *mut mtk_base_afe) {
    static void audio_dmic_adda_disable(struct mtk_base_afe *afe)
    {
    regmap_update_bits(afe.regmap, AFE_ADDA_UL_DL_CON0,
    AFE_ADDA_UL_DL_DMIC_CLKDIV_ON,
    ~AFE_ADDA_UL_DL_DMIC_CLKDIV_ON);
    mt8365_dai_disable_adda_on(afe);
    }
    static void mt8365_dai_enable_dmic(struct mtk_base_afe *afe,
    struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mt8365_afe_private *afe_priv = afe.platform_priv;
    struct mt8365_dmic_data *dmic_data = afe_priv.dai_priv[MT8365_AFE_IO_DMIC];
    unsigned int val_mask;
    let mut reg: c_int = get_chan_reg(dmic_data.dmic_channel);
    if (reg < 0)
    return;
// val and mask will be always same to enable
    val_mask = DMIC_TOP_CON_CH1_ON |
    DMIC_TOP_CON_CH2_ON |
    DMIC_TOP_CON_SRC_ON;
    regmap_update_bits(afe.regmap, reg, val_mask, val_mask);
    }
    static void mt8365_dai_disable_dmic(struct mtk_base_afe *afe,
    struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mt8365_afe_private *afe_priv = afe.platform_priv;
    struct mt8365_dmic_data *dmic_data = afe_priv.dai_priv[MT8365_AFE_IO_DMIC];
    unsigned int mask;
    let mut reg: c_int = get_chan_reg(dmic_data.dmic_channel);
    if (reg < 0)
    return;
    dev_dbg(afe.dev, "%s dmic_channel %d\n", __func__, dmic_data.dmic_channel);
    mask = DMIC_TOP_CON_CH1_ON |
    DMIC_TOP_CON_CH2_ON |
    DMIC_TOP_CON_SRC_ON |
    DMIC_TOP_CON_SDM3_LEVEL_MODE;
// Set all masked values to 0
    regmap_update_bits(afe.regmap, reg, mask, 0);
    }
    static int mt8365_dai_configure_dmic(struct mtk_base_afe *afe,
    struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mt8365_afe_private *afe_priv = afe.platform_priv;
    struct mt8365_dmic_data *dmic_data = afe_priv.dai_priv[MT8365_AFE_IO_DMIC];
    let mut two_wire_mode: bool = dmic_data.two_wire_mode;
    let mut clk_phase_sel_ch1: c_uint = dmic_data.clk_phase_sel_ch1;
    let mut clk_phase_sel_ch2: c_uint = dmic_data.clk_phase_sel_ch2;
    let mut val: c_uint = 0;
    let mut rate: c_uint = dai.symmetric_rate;
    let mut reg: c_int = get_chan_reg(dai.symmetric_channels);
    if (reg < 0)
    return -EINVAL;
    dmic_data.dmic_channel = dai.symmetric_channels;
    val |= DMIC_TOP_CON_SDM3_LEVEL_MODE;
    if (two_wire_mode) {
    val |= DMIC_TOP_CON_TWO_WIRE_MODE;
    } else {
    val |= FIELD_PREP(DMIC_TOP_CON_CK_PHASE_SEL_CH1,
    clk_phase_sel_ch1);
    val |= FIELD_PREP(DMIC_TOP_CON_CK_PHASE_SEL_CH2,
    clk_phase_sel_ch2);
    }
    switch (rate) {
    case 48000:
    val |= DMIC_TOP_CON_VOICE_MODE_48K;
    break;
    case 32000:
    val |= DMIC_TOP_CON_VOICE_MODE_32K;
    break;
    case 16000:
    val |= DMIC_TOP_CON_VOICE_MODE_16K;
    break;
    case 8000:
    val |= DMIC_TOP_CON_VOICE_MODE_8K;
    break;
    default:
    return -EINVAL;
    }
    regmap_update_bits(afe.regmap, reg, DMIC_TOP_CON_CONFIG_MASK, val);
    return 0;
    }
    static int mt8365_dai_dmic_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    mt8365_afe_enable_main_clk(afe);
    mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DMIC0_ADC);
    mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DMIC1_ADC);
    mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DMIC2_ADC);
    mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DMIC3_ADC);
    audio_dmic_adda_enable(afe);
    return 0;
    }
    static void mt8365_dai_dmic_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    mt8365_dai_disable_dmic(afe, substream, dai);
    audio_dmic_adda_disable(afe);
// HW Request delay 125us before CG off
    usleep_range(125, 300);
    mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DMIC3_ADC);
    mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DMIC2_ADC);
    mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DMIC1_ADC);
    mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DMIC0_ADC);
    mt8365_afe_disable_main_clk(afe);
    }
    static int mt8365_dai_dmic_prepare(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct mtk_base_afe *afe = snd_soc_dai_get_drvdata(dai);
    mt8365_dai_configure_dmic(afe, substream, dai);
    mt8365_dai_enable_dmic(afe, substream, dai);
    return 0;
    }
    static const struct snd_soc_dai_ops mt8365_afe_dmic_ops = {
    .startup	= mt8365_dai_dmic_startup,
    .shutdown	= mt8365_dai_dmic_shutdown,
    .prepare	= mt8365_dai_dmic_prepare,
    };
    static struct snd_soc_dai_driver mtk_dai_dmic_driver[] = {
    {
    .name = "DMIC",
    .id = MT8365_AFE_IO_DMIC,
    .capture = {
    .stream_name = "DMIC Capture",
    .channels_min = 1,
    .channels_max = 8,
    .rates = SNDRV_PCM_RATE_16000 |
    SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE,
    },
    .ops = &mt8365_afe_dmic_ops,
    }
    };
// DAI Controls
// Values for 48kHz mode
    static const char * const iir_mode_src[] = {
    "SW custom", "5Hz", "10Hz", "25Hz", "50Hz", "65Hz"
    };
    static SOC_ENUM_SINGLE_DECL(iir_mode, AFE_DMIC0_UL_SRC_CON0, 7, iir_mode_src);
    static const struct snd_kcontrol_new mtk_dai_dmic_controls[] = {
    SOC_SINGLE("DMIC IIR Switch", AFE_DMIC0_UL_SRC_CON0, DMIC_TOP_CON_IIR_ON, 1, 0),
    SOC_ENUM("DMIC IIR Mode", iir_mode),
    };
// DAI widget
    static const struct snd_soc_dapm_widget mtk_dai_dmic_widgets[] = {
    SND_SOC_DAPM_INPUT("DMIC In"),
    };
// DAI route
    static const struct snd_soc_dapm_route mtk_dai_dmic_routes[] = {
    {"I14", core::ptr::null_mut(), "DMIC Capture"},
    {"I15", core::ptr::null_mut(), "DMIC Capture"},
    {"I16", core::ptr::null_mut(), "DMIC Capture"},
    {"I17", core::ptr::null_mut(), "DMIC Capture"},
    {"I18", core::ptr::null_mut(), "DMIC Capture"},
    {"I19", core::ptr::null_mut(), "DMIC Capture"},
    {"I20", core::ptr::null_mut(), "DMIC Capture"},
    {"I21", core::ptr::null_mut(), "DMIC Capture"},
    {"DMIC Capture", core::ptr::null_mut(), "DMIC In"},
    };
#[no_mangle]
unsafe extern "C" fn init_dmic_priv_data(afe: *mut mtk_base_afe) -> c_int {
    static int init_dmic_priv_data(struct mtk_base_afe *afe)
    {
    struct mt8365_afe_private *afe_priv = afe.platform_priv;
    struct mt8365_dmic_data *dmic_priv;
    struct device_node *np = afe.dev.of_node;
    unsigned int temps[4];
    int ret;
    dmic_priv = devm_kzalloc(afe.dev, sizeof(*dmic_priv), GFP_KERNEL);
    if (!dmic_priv)
    return -ENOMEM;
    ret = of_property_read_u32_array(np, "mediatek,dmic-mode",
    &temps[0],
    1);
    if (ret == 0)
    dmic_priv.two_wire_mode = !!temps[0];
    if (!dmic_priv.two_wire_mode) {
    dmic_priv.clk_phase_sel_ch1 = 0;
    dmic_priv.clk_phase_sel_ch2 = 4;
    }
    afe_priv.dai_priv[MT8365_AFE_IO_DMIC] = dmic_priv;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mt8365_dai_dmic_register(afe: *mut mtk_base_afe) -> c_int {
    int mt8365_dai_dmic_register(struct mtk_base_afe *afe)
    {
    struct mtk_base_afe_dai *dai;
    dai = devm_kzalloc(afe.dev, sizeof(*dai), GFP_KERNEL);
    if (!dai)
    return -ENOMEM;
    list_add(&dai.list, &afe.sub_dais);
    dai.dai_drivers = mtk_dai_dmic_driver;
    dai.num_dai_drivers = ARRAY_SIZE(mtk_dai_dmic_driver);
    dai.controls = mtk_dai_dmic_controls;
    dai.num_controls = ARRAY_SIZE(mtk_dai_dmic_controls);
    dai.dapm_widgets = mtk_dai_dmic_widgets;
    dai.num_dapm_widgets = ARRAY_SIZE(mtk_dai_dmic_widgets);
    dai.dapm_routes = mtk_dai_dmic_routes;
    dai.num_dapm_routes = ARRAY_SIZE(mtk_dai_dmic_routes);
    return init_dmic_priv_data(afe);
    }
