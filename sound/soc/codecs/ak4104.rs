//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ak4104.c
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
// AK4104 ALSA SoC (ASoC) driver
//
// Copyright (c) 2009 Daniel Mack <daniel@caiaq.de>
//

// AK4104 registers addresses
pub const AK4104_REG_CONTROL1: c_uint = 0x00;
pub const AK4104_REG_RESERVED: c_uint = 0x01;
pub const AK4104_REG_CONTROL2: c_uint = 0x02;
pub const AK4104_REG_TX: c_uint = 0x03;

pub const AK4104_NUM_REGS: c_int = 10;
pub const AK4104_REG_MASK: c_uint = 0x1f;
pub const AK4104_READ: c_uint = 0xc0;
pub const AK4104_WRITE: c_uint = 0xe0;
pub const AK4104_RESERVED_VAL: c_uint = 0x5b;
// Bit masks for AK4104 registers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak4104_private {
    pub regmap: *mut regmap,
    pub regulator: *mut regulator,
}

    static const struct snd_soc_dapm_widget ak4104_dapm_widgets[] = {
    SND_SOC_DAPM_PGA("TXE", AK4104_REG_TX, AK4104_TX_TXE, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT("TX"),
    };
    static const struct snd_soc_dapm_route ak4104_dapm_routes[] = {
    { "TXE", core::ptr::null_mut(), "Playback" },
    { "TX", core::ptr::null_mut(), "TXE" },
    };
    static int ak4104_set_dai_fmt(struct snd_soc_dai *codec_dai,
    unsigned int format)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct ak4104_private *ak4104 = snd_soc_component_get_drvdata(component);
    let mut val: c_int = 0;
    int ret;
// set DAI format
    switch (format & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_RIGHT_J:
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    val |= AK4104_CONTROL1_DIF0;
    break;
    case SND_SOC_DAIFMT_I2S:
    val |= AK4104_CONTROL1_DIF0 | AK4104_CONTROL1_DIF1;
    break;
    default:
    dev_err(component.dev, "invalid dai format\n");
    return -EINVAL;
    }
// This device can only be consumer
    if ((format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC)
    return -EINVAL;
    ret = regmap_update_bits(ak4104.regmap, AK4104_REG_CONTROL1,
    AK4104_CONTROL1_DIF0 | AK4104_CONTROL1_DIF1,
    val);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int ak4104_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct ak4104_private *ak4104 = snd_soc_component_get_drvdata(component);
    int ret, val = 0;
// set the IEC958 bits: consumer mode, no copyright bit
    val |= IEC958_AES0_CON_NOT_COPYRIGHT;
    regmap_write(ak4104.regmap, AK4104_REG_CHN_STATUS(0), val);
    val = 0;
    switch (params_rate(params)) {
    case 22050:
    val |= IEC958_AES3_CON_FS_22050;
    break;
    case 24000:
    val |= IEC958_AES3_CON_FS_24000;
    break;
    case 32000:
    val |= IEC958_AES3_CON_FS_32000;
    break;
    case 44100:
    val |= IEC958_AES3_CON_FS_44100;
    break;
    case 48000:
    val |= IEC958_AES3_CON_FS_48000;
    break;
    case 88200:
    val |= IEC958_AES3_CON_FS_88200;
    break;
    case 96000:
    val |= IEC958_AES3_CON_FS_96000;
    break;
    case 176400:
    val |= IEC958_AES3_CON_FS_176400;
    break;
    case 192000:
    val |= IEC958_AES3_CON_FS_192000;
    break;
    default:
    dev_err(component.dev, "unsupported sampling rate\n");
    return -EINVAL;
    }
    ret = regmap_write(ak4104.regmap, AK4104_REG_CHN_STATUS(3), val);
    if (ret < 0)
    return ret;
    return 0;
    }
    static const struct snd_soc_dai_ops ak4101_dai_ops = {
    .hw_params = ak4104_hw_params,
    .set_fmt = ak4104_set_dai_fmt,
    };
    static struct snd_soc_dai_driver ak4104_dai = {
    .name = "ak4104-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 |
    SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 |
    SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE  |
    SNDRV_PCM_FMTBIT_S24_3LE |
    SNDRV_PCM_FMTBIT_S24_LE
    },
    .ops = &ak4101_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn ak4104_probe(component: *mut snd_soc_component) -> c_int {
    static int ak4104_probe(struct snd_soc_component *component)
    {
    struct ak4104_private *ak4104 = snd_soc_component_get_drvdata(component);
    int ret;
    ret = regulator_enable(ak4104.regulator);
    if (ret < 0) {
    dev_err(component.dev, "Unable to enable regulator: %d\n", ret);
    return ret;
    }
// set power-up and non-reset bits
    ret = regmap_update_bits(ak4104.regmap, AK4104_REG_CONTROL1,
    AK4104_CONTROL1_PW | AK4104_CONTROL1_RSTN,
    AK4104_CONTROL1_PW | AK4104_CONTROL1_RSTN);
    if (ret < 0)
    goto exit_disable_regulator;
// enable transmitter
    ret = regmap_update_bits(ak4104.regmap, AK4104_REG_TX,
    AK4104_TX_TXE, AK4104_TX_TXE);
    if (ret < 0)
    goto exit_disable_regulator;
    return 0;
    exit_disable_regulator:
    regulator_disable(ak4104.regulator);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ak4104_remove(component: *mut snd_soc_component) {
    static void ak4104_remove(struct snd_soc_component *component)
    {
    struct ak4104_private *ak4104 = snd_soc_component_get_drvdata(component);
    regmap_update_bits(ak4104.regmap, AK4104_REG_CONTROL1,
    AK4104_CONTROL1_PW | AK4104_CONTROL1_RSTN, 0);
    regulator_disable(ak4104.regulator);
    }

#[no_mangle]
unsafe extern "C" fn ak4104_soc_suspend(component: *mut snd_soc_component) -> c_int {
    static int ak4104_soc_suspend(struct snd_soc_component *component)
    {
    struct ak4104_private *priv = snd_soc_component_get_drvdata(component);
    regulator_disable(priv.regulator);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak4104_soc_resume(component: *mut snd_soc_component) -> c_int {
    static int ak4104_soc_resume(struct snd_soc_component *component)
    {
    struct ak4104_private *priv = snd_soc_component_get_drvdata(component);
    int ret;
    ret = regulator_enable(priv.regulator);
    if (ret < 0)
    return ret;
    return 0;
    }

    static const struct snd_soc_component_driver soc_component_device_ak4104 = {
    .probe			= ak4104_probe,
    .remove			= ak4104_remove,
    .suspend		= ak4104_soc_suspend,
    .resume			= ak4104_soc_resume,
    .dapm_widgets		= ak4104_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(ak4104_dapm_widgets),
    .dapm_routes		= ak4104_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(ak4104_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static const struct regmap_config ak4104_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = AK4104_NUM_REGS - 1,
    .read_flag_mask = AK4104_READ,
    .write_flag_mask = AK4104_WRITE,
    .cache_type = REGCACHE_RBTREE,
    };
#[no_mangle]
unsafe extern "C" fn ak4104_spi_probe(spi: *mut spi_device) -> c_int {
    static int ak4104_spi_probe(struct spi_device *spi)
    {
    struct ak4104_private *ak4104;
    struct gpio_desc *reset_gpiod;
    unsigned int val;
    int ret;
    spi.bits_per_word = 8;
    spi.mode = SPI_MODE_0;
    ret = spi_setup(spi);
    if (ret < 0)
    return ret;
    ak4104 = devm_kzalloc(&spi.dev, sizeof(struct ak4104_private),
    GFP_KERNEL);
    if (ak4104 == core::ptr::null_mut())
    return -ENOMEM;
    ak4104.regulator = devm_regulator_get(&spi.dev, "vdd");
    if (IS_ERR(ak4104.regulator)) {
    ret = PTR_ERR(ak4104.regulator);
    dev_err(&spi.dev, "Unable to get Vdd regulator: %d\n", ret);
    return ret;
    }
    ak4104.regmap = devm_regmap_init_spi(spi, &ak4104_regmap);
    if (IS_ERR(ak4104.regmap)) {
    ret = PTR_ERR(ak4104.regmap);
    return ret;
    }
    reset_gpiod = devm_gpiod_get_optional(&spi.dev, "reset",
    GPIOD_OUT_HIGH);
    if (PTR_ERR(reset_gpiod) == -EPROBE_DEFER)
    return -EPROBE_DEFER;
// read the 'reserved' register - according to the datasheet, it
// should contain 0x5b. Not a good way to verify the presence of
// the device, but there is no hardware ID register.
    ret = regmap_read(ak4104.regmap, AK4104_REG_RESERVED, &val);
    if (ret != 0)
    return ret;
    if (val != AK4104_RESERVED_VAL)
    return -ENODEV;
    spi_set_drvdata(spi, ak4104);
    ret = devm_snd_soc_register_component(&spi.dev,
    &soc_component_device_ak4104, &ak4104_dai, 1);
    return ret;
    }
    static const struct of_device_id ak4104_of_match[] = {
    { .compatible = "asahi-kasei,ak4104", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ak4104_of_match);
    static const struct spi_device_id ak4104_id_table[] = {
    { "ak4104", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ak4104_id_table);
    static struct spi_driver ak4104_spi_driver = {
    .driver  = {
    .name   = "ak4104",
    .of_match_table = ak4104_of_match,
    },
    .id_table = ak4104_id_table,
    .probe  = ak4104_spi_probe,
    };
    module_spi_driver(ak4104_spi_driver);
    MODULE_AUTHOR("Daniel Mack <daniel@caiaq.de>");
    MODULE_DESCRIPTION("Asahi Kasei AK4104 ALSA SoC driver");
    MODULE_LICENSE("GPL");
