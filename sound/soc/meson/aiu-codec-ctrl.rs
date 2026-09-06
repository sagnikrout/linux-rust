//! Automatically rewritten from C to Rust
//! Source: sound/soc/meson/aiu-codec-ctrl.c
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
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

pub const CTRL_DATA_SEL_SHIFT: c_int = 4;

    static const char * const aiu_codec_ctrl_mux_texts[] = {
    "DISABLED", "PCM", "I2S",
    };
    static int aiu_codec_ctrl_mux_put_enum(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    struct snd_soc_dapm_context *dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    struct soc_enum *e = (struct soc_enum *)kcontrol.private_value;
    unsigned int mux, changed;
    if (ucontrol.value.enumerated.item[0] >= e.items)
    return -EINVAL;
    mux = snd_soc_enum_item_to_val(e, ucontrol.value.enumerated.item[0]);
    changed = snd_soc_component_test_bits(component, e.reg,
    CTRL_DATA_SEL,
    FIELD_PREP(CTRL_DATA_SEL, mux));
    if (!changed)
    return 0;
// Force disconnect of the mux while updating
    snd_soc_dapm_mux_update_power(dapm, kcontrol, 0, core::ptr::null_mut(), core::ptr::null_mut());
// Reset the source first
    snd_soc_component_update_bits(component, e.reg,
    CTRL_CLK_SEL |
    CTRL_DATA_SEL,
    FIELD_PREP(CTRL_CLK_SEL, 0) |
    FIELD_PREP(CTRL_DATA_SEL, 0));
// Set the appropriate source
    snd_soc_component_update_bits(component, e.reg,
    CTRL_CLK_SEL |
    CTRL_DATA_SEL,
    FIELD_PREP(CTRL_CLK_SEL, mux) |
    FIELD_PREP(CTRL_DATA_SEL, mux));
    snd_soc_dapm_mux_update_power(dapm, kcontrol, mux, e, core::ptr::null_mut());
    return 1;
    }
    static SOC_ENUM_SINGLE_DECL(aiu_hdmi_ctrl_mux_enum, AIU_HDMI_CLK_DATA_CTRL,
    CTRL_DATA_SEL_SHIFT,
    aiu_codec_ctrl_mux_texts);
    static const struct snd_kcontrol_new aiu_hdmi_ctrl_mux =
    SOC_DAPM_ENUM_EXT("HDMI Source", aiu_hdmi_ctrl_mux_enum,
    snd_soc_dapm_get_enum_double,
    aiu_codec_ctrl_mux_put_enum);
    static const struct snd_soc_dapm_widget aiu_hdmi_ctrl_widgets[] = {
    SND_SOC_DAPM_MUX("HDMI CTRL SRC", SND_SOC_NOPM, 0, 0,
    &aiu_hdmi_ctrl_mux),
    };
    static const struct snd_soc_dai_ops aiu_codec_ctrl_input_ops = {
    .probe		= meson_codec_glue_input_dai_probe,
    .remove		= meson_codec_glue_input_dai_remove,
    .hw_params	= meson_codec_glue_input_hw_params,
    .set_fmt	= meson_codec_glue_input_set_fmt,
    };
    static const struct snd_soc_dai_ops aiu_codec_ctrl_output_ops = {
    .startup	= meson_codec_glue_output_startup,
    };

    (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE |	\
    SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_LE |	\
    SNDRV_PCM_FMTBIT_S32_LE)

    {								\
    .stream_name	= xname " " xsuffix,			\
    .channels_min	= 1,					\
    .channels_max	= 8,					\
    .rate_min       = 5512,					\
    .rate_max	= 192000,				\
    .formats	= AIU_CODEC_CTRL_FORMATS,		\
    }

    .name = "CODEC CTRL " xname,				\
    .playback = AIU_CODEC_CTRL_STREAM(xname, "Playback"),	\
    .ops = &aiu_codec_ctrl_input_ops,			\
    }

    .name = "CODEC CTRL " xname,				\
    .capture = AIU_CODEC_CTRL_STREAM(xname, "Capture"),	\
    .ops = &aiu_codec_ctrl_output_ops,			\
    }
    static struct snd_soc_dai_driver aiu_hdmi_ctrl_dai_drv[] = {
    [CTRL_I2S] = AIU_CODEC_CTRL_INPUT("HDMI I2S IN"),
    [CTRL_PCM] = AIU_CODEC_CTRL_INPUT("HDMI PCM IN"),
    [CTRL_OUT] = AIU_CODEC_CTRL_OUTPUT("HDMI OUT"),
    };
    static const struct snd_soc_dapm_route aiu_hdmi_ctrl_routes[] = {
    { "HDMI CTRL SRC", "I2S", "HDMI I2S IN Playback" },
    { "HDMI CTRL SRC", "PCM", "HDMI PCM IN Playback" },
    { "HDMI OUT Capture", core::ptr::null_mut(), "HDMI CTRL SRC" },
    };
    static int aiu_hdmi_of_xlate_dai_name(struct snd_soc_component *component,
    const struct of_phandle_args *args,
    const char **dai_name)
    {
    return aiu_of_xlate_dai_name(component, args, dai_name, AIU_HDMI);
    }
    static const struct snd_soc_component_driver aiu_hdmi_ctrl_component = {
    .name			= "AIU HDMI Codec Control",
    .dapm_widgets		= aiu_hdmi_ctrl_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(aiu_hdmi_ctrl_widgets),
    .dapm_routes		= aiu_hdmi_ctrl_routes,
    .num_dapm_routes	= ARRAY_SIZE(aiu_hdmi_ctrl_routes),
    .of_xlate_dai_name	= aiu_hdmi_of_xlate_dai_name,
    .endianness		= 1,

    .debugfs_prefix		= "hdmi",

    };
#[no_mangle]
pub unsafe extern "C" fn aiu_hdmi_ctrl_register_component(dev: *mut device) -> c_int {
    int aiu_hdmi_ctrl_register_component(struct device *dev)
    {
    return snd_soc_register_component(dev, &aiu_hdmi_ctrl_component,
    aiu_hdmi_ctrl_dai_drv,
    ARRAY_SIZE(aiu_hdmi_ctrl_dai_drv));
    }
