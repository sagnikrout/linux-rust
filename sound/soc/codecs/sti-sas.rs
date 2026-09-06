//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/sti-sas.c
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
// Copyright (C) STMicroelectronics SA 2015
// Authors: Arnaud Pouliquen <arnaud.pouliquen@st.com>
// for STMicroelectronics.
//

// DAC definitions
// stih407 DAC registers
// sysconf 5041: Audio-Gue-Control
pub const STIH407_AUDIO_GLUE_CTRL: c_uint = 0x000000A4;
// sysconf 5042: Audio-DAC-Control
pub const STIH407_AUDIO_DAC_CTRL: c_uint = 0x000000A8;
// DAC definitions
pub const STIH407_DAC_SOFTMUTE: c_uint = 0x0;
pub const STIH407_DAC_STANDBY_ANA: c_uint = 0x1;
pub const STIH407_DAC_STANDBY: c_uint = 0x2;

// SPDIF definitions
pub const SPDIF_BIPHASE_ENABLE: c_uint = 0x6;
pub const SPDIF_BIPHASE_IDLE: c_uint = 0x7;

    enum {
    STI_SAS_DAI_SPDIF_OUT,
    STI_SAS_DAI_ANALOG_OUT,
    };
    static const struct reg_default stih407_sas_reg_defaults[] = {
    { STIH407_AUDIO_GLUE_CTRL, 0x00000040 },
    { STIH407_AUDIO_DAC_CTRL, 0x000000000 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_dac_audio {
    pub regmap: *mut regmap,
    pub virt_regmap: *mut regmap,
    pub mclk: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_spdif_audio {
    pub regmap: *mut regmap,
    pub mclk: c_int,
}

// device data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_sas_dev_data {
    pub regmap: *const regmap_config,
    pub /: *const *const *const snd_soc_dai_ops dac_ops; / DAC function callbacks,
}

// driver data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_sas_data {
    pub dev: *mut device,
    pub dev_data: *const sti_sas_dev_data,
    pub dac: sti_dac_audio,
    pub spdif: sti_spdif_audio,
}

// Read a register from the sysconf reg bank
    static int sti_sas_read_reg(void *context, unsigned int reg,
    unsigned int *value)
    {
    struct sti_sas_data *drvdata = context;
    int status;
    u32 val;
    status = regmap_read(drvdata.dac.regmap, reg, &val);
// value = (unsigned int)val;
    return status;
    }
// Read a register from the sysconf reg bank
    static int sti_sas_write_reg(void *context, unsigned int reg,
    unsigned int value)
    {
    struct sti_sas_data *drvdata = context;
    return regmap_write(drvdata.dac.regmap, reg, value);
    }
    static int  sti_sas_init_sas_registers(struct snd_soc_component *component,
    struct sti_sas_data *data)
    {
    int ret;
//
// DAC and SPDIF are activated by default
// put them in IDLE to save power
//
// Initialise bi-phase formatter to disabled
    ret = snd_soc_component_update_bits(component, STIH407_AUDIO_GLUE_CTRL,
    SPDIF_BIPHASE_ENABLE_MASK, 0);
    if (!ret)
// Initialise bi-phase formatter idle value to 0
    ret = snd_soc_component_update_bits(component, STIH407_AUDIO_GLUE_CTRL,
    SPDIF_BIPHASE_IDLE_MASK, 0);
    if (ret < 0) {
    dev_err(component.dev, "Failed to update SPDIF registers\n");
    return ret;
    }
// Init DAC configuration
// init configuration
    ret =  snd_soc_component_update_bits(component, STIH407_AUDIO_DAC_CTRL,
    STIH407_DAC_STANDBY_MASK,
    STIH407_DAC_STANDBY_MASK);
    if (!ret)
    ret = snd_soc_component_update_bits(component, STIH407_AUDIO_DAC_CTRL,
    STIH407_DAC_STANDBY_ANA_MASK,
    STIH407_DAC_STANDBY_ANA_MASK);
    if (!ret)
    ret = snd_soc_component_update_bits(component, STIH407_AUDIO_DAC_CTRL,
    STIH407_DAC_SOFTMUTE_MASK,
    STIH407_DAC_SOFTMUTE_MASK);
    if (ret < 0) {
    dev_err(component.dev, "Failed to update DAC registers\n");
    return ret;
    }
    return ret;
    }
//
// DAC
//
#[no_mangle]
unsafe extern "C" fn sti_sas_dac_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int sti_sas_dac_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
// Sanity check only
    if ((fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC) {
    dev_err(dai.component.dev,
    "%s: ERROR: Unsupported clocking 0x%x\n",
    __func__, fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK);
    return -EINVAL;
    }
    return 0;
    }
    static const struct snd_soc_dapm_widget stih407_sas_dapm_widgets[] = {
    SND_SOC_DAPM_OUT_DRV("DAC standby ana", STIH407_AUDIO_DAC_CTRL,
    STIH407_DAC_STANDBY_ANA, 1, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_DAC("DAC standby",  "dac_p", STIH407_AUDIO_DAC_CTRL,
    STIH407_DAC_STANDBY, 1),
    SND_SOC_DAPM_OUTPUT("DAC Output"),
    };
    static const struct snd_soc_dapm_route stih407_sas_route[] = {
    {"DAC Output", core::ptr::null_mut(), "DAC standby ana"},
    {"DAC standby ana", core::ptr::null_mut(), "DAC standby"},
    };
#[no_mangle]
unsafe extern "C" fn stih407_sas_dac_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    static int stih407_sas_dac_mute(struct snd_soc_dai *dai, int mute, int stream)
    {
    struct snd_soc_component *component = dai.component;
    if (mute) {
    return snd_soc_component_update_bits(component, STIH407_AUDIO_DAC_CTRL,
    STIH407_DAC_SOFTMUTE_MASK,
    STIH407_DAC_SOFTMUTE_MASK);
    } else {
    return snd_soc_component_update_bits(component, STIH407_AUDIO_DAC_CTRL,
    STIH407_DAC_SOFTMUTE_MASK,
    0);
    }
    }
//
// SPDIF
//
    static int sti_sas_spdif_set_fmt(struct snd_soc_dai *dai,
    unsigned int fmt)
    {
    if ((fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC) {
    dev_err(dai.component.dev,
    "%s: ERROR: Unsupported clocking mask 0x%x\n",
    __func__, fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK);
    return -EINVAL;
    }
    return 0;
    }
//
// sti_sas_spdif_trigger:
// Trigger function is used to ensure that BiPhase Formater is disabled
// before CPU dai is stopped.
// This is mandatory to avoid that BPF is stalled
//
    static int sti_sas_spdif_trigger(struct snd_pcm_substream *substream, int cmd,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    return snd_soc_component_update_bits(component, STIH407_AUDIO_GLUE_CTRL,
    SPDIF_BIPHASE_ENABLE_MASK,
    SPDIF_BIPHASE_ENABLE_MASK);
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    return snd_soc_component_update_bits(component, STIH407_AUDIO_GLUE_CTRL,
    SPDIF_BIPHASE_ENABLE_MASK,
    0);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn sti_sas_volatile_register(dev: *mut device, reg: c_uint) -> bool {
    static bool sti_sas_volatile_register(struct device *dev, unsigned int reg)
    {
    if (reg == STIH407_AUDIO_GLUE_CTRL)
    return true;
    return false;
    }
//
// CODEC DAIS
//
// sti_sas_set_sysclk:
// get MCLK input frequency to check that MCLK-FS ratio is coherent
//
    static int sti_sas_set_sysclk(struct snd_soc_dai *dai, int clk_id,
    unsigned int freq, int dir)
    {
    struct snd_soc_component *component = dai.component;
    struct sti_sas_data *drvdata = dev_get_drvdata(component.dev);
    if (dir == SND_SOC_CLOCK_OUT)
    return 0;
    if (clk_id != 0)
    return -EINVAL;
    switch (dai.id) {
    case STI_SAS_DAI_SPDIF_OUT:
    drvdata.spdif.mclk = freq;
    break;
    case STI_SAS_DAI_ANALOG_OUT:
    drvdata.dac.mclk = freq;
    break;
    }
    return 0;
    }
    static int sti_sas_prepare(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct sti_sas_data *drvdata = dev_get_drvdata(component.dev);
    struct snd_pcm_runtime *runtime = substream.runtime;
    switch (dai.id) {
    case STI_SAS_DAI_SPDIF_OUT:
    if ((drvdata.spdif.mclk / runtime.rate) != 128) {
    dev_err(component.dev, "unexpected mclk-fs ratio\n");
    return -EINVAL;
    }
    break;
    case STI_SAS_DAI_ANALOG_OUT:
    if ((drvdata.dac.mclk / runtime.rate) != 256) {
    dev_err(component.dev, "unexpected mclk-fs ratio\n");
    return -EINVAL;
    }
    break;
    }
    return 0;
    }
    static const struct snd_soc_dai_ops stih407_dac_ops = {
    .set_fmt = sti_sas_dac_set_fmt,
    .mute_stream = stih407_sas_dac_mute,
    .prepare = sti_sas_prepare,
    .set_sysclk = sti_sas_set_sysclk,
    };
    static const struct regmap_config stih407_sas_regmap = {
    .reg_bits = 32,
    .val_bits = 32,
    .fast_io = true,
    .max_register = STIH407_AUDIO_DAC_CTRL,
    .reg_defaults = stih407_sas_reg_defaults,
    .num_reg_defaults = ARRAY_SIZE(stih407_sas_reg_defaults),
    .volatile_reg = sti_sas_volatile_register,
    .cache_type = REGCACHE_MAPLE,
    .reg_read = sti_sas_read_reg,
    .reg_write = sti_sas_write_reg,
    };
    static const struct sti_sas_dev_data stih407_data = {
    .regmap = &stih407_sas_regmap,
    .dac_ops = &stih407_dac_ops,
    };
    static struct snd_soc_dai_driver sti_sas_dai[] = {
    {
    .name = "sas-dai-spdif-out",
    .id = STI_SAS_DAI_SPDIF_OUT,
    .playback = {
    .stream_name = "spdif_p",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_64000 |
    SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 |
    SNDRV_PCM_RATE_192000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE,
    },
    .ops = (struct snd_soc_dai_ops[]) {
    {
    .set_fmt = sti_sas_spdif_set_fmt,
    .trigger = sti_sas_spdif_trigger,
    .set_sysclk = sti_sas_set_sysclk,
    .prepare = sti_sas_prepare,
    }
    },
    },
    {
    .name = "sas-dai-dac",
    .id = STI_SAS_DAI_ANALOG_OUT,
    .playback = {
    .stream_name = "dac_p",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S32_LE,
    },
    },
    };

#[no_mangle]
unsafe extern "C" fn sti_sas_resume(component: *mut snd_soc_component) -> c_int {
    static int sti_sas_resume(struct snd_soc_component *component)
    {
    struct sti_sas_data *drvdata = dev_get_drvdata(component.dev);
    return sti_sas_init_sas_registers(component, drvdata);
    }

#[no_mangle]
unsafe extern "C" fn sti_sas_component_probe(component: *mut snd_soc_component) -> c_int {
    static int sti_sas_component_probe(struct snd_soc_component *component)
    {
    struct sti_sas_data *drvdata = dev_get_drvdata(component.dev);
    return sti_sas_init_sas_registers(component, drvdata);
    }
    static const struct snd_soc_component_driver sti_sas_driver = {
    .probe			= sti_sas_component_probe,
    .resume			= sti_sas_resume,
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    .dapm_widgets		= stih407_sas_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(stih407_sas_dapm_widgets),
    .dapm_routes		= stih407_sas_route,
    .num_dapm_routes	= ARRAY_SIZE(stih407_sas_route),
    };
    static const struct of_device_id sti_sas_dev_match[] = {
    {
    .compatible = "st,stih407-sas-codec",
    .data = &stih407_data,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, sti_sas_dev_match);
#[no_mangle]
unsafe extern "C" fn sti_sas_driver_probe(pdev: *mut platform_device) -> c_int {
    static int sti_sas_driver_probe(struct platform_device *pdev)
    {
    struct device_node *pnode = pdev.dev.of_node;
    struct sti_sas_data *drvdata;
    const struct of_device_id *of_id;
// Allocate device structure
    drvdata = devm_kzalloc(&pdev.dev, sizeof(struct sti_sas_data),
    GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
// Populate data structure depending on compatibility
    of_id = of_match_node(sti_sas_dev_match, pnode);
    if (!of_id.data) {
    dev_err(&pdev.dev, "data associated to device is missing\n");
    return -EINVAL;
    }
    drvdata.dev_data = (struct sti_sas_dev_data *)of_id.data;
// Initialise device structure
    drvdata.dev = &pdev.dev;
// Request the DAC & SPDIF registers memory region
    drvdata.dac.virt_regmap = devm_regmap_init(&pdev.dev, core::ptr::null_mut(), drvdata,
    drvdata.dev_data.regmap);
    if (IS_ERR(drvdata.dac.virt_regmap)) {
    dev_err(&pdev.dev, "audio registers not enabled\n");
    return PTR_ERR(drvdata.dac.virt_regmap);
    }
// Request the syscon region
    drvdata.dac.regmap =
    syscon_regmap_lookup_by_phandle(pnode, "st,syscfg");
    if (IS_ERR(drvdata.dac.regmap)) {
    dev_err(&pdev.dev, "syscon registers not available\n");
    return PTR_ERR(drvdata.dac.regmap);
    }
    drvdata.spdif.regmap = drvdata.dac.regmap;
    sti_sas_dai[STI_SAS_DAI_ANALOG_OUT].ops = drvdata.dev_data.dac_ops;
// Store context
    dev_set_drvdata(&pdev.dev, drvdata);
    return devm_snd_soc_register_component(&pdev.dev, &sti_sas_driver,
    sti_sas_dai,
    ARRAY_SIZE(sti_sas_dai));
    }
    static struct platform_driver sti_sas_platform_driver = {
    .driver = {
    .name = "sti-sas-codec",
    .of_match_table = sti_sas_dev_match,
    },
    .probe = sti_sas_driver_probe,
    };
    module_platform_driver(sti_sas_platform_driver);
    MODULE_DESCRIPTION("audio codec for STMicroelectronics sti platforms");
    MODULE_AUTHOR("Arnaud.pouliquen@st.com");
    MODULE_LICENSE("GPL v2");
