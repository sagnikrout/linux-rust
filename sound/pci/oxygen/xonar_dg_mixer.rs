//! Automatically rewritten from C to Rust
//! Source: sound/pci/oxygen/xonar_dg_mixer.c
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
// Mixer controls for the Xonar DG/DGX
//
// Copyright (c) Clemens Ladisch <clemens@ladisch.de>
// Copyright (c) Roman Volkov <v1ron@mail.ru>
//

// analog output select
#[no_mangle]
unsafe extern "C" fn output_select_apply(chip: *mut oxygen) -> c_int {
    static int output_select_apply(struct oxygen *chip)
    {
    struct dg *data = chip.model_data;
    data.cs4245_shadow[CS4245_SIGNAL_SEL] &= ~CS4245_A_OUT_SEL_MASK;
    if (data.output_sel == PLAYBACK_DST_HP) {
// mute FP (aux output) amplifier, switch rear jack to CS4245
    oxygen_set_bits8(chip, OXYGEN_GPIO_DATA, GPIO_HP_REAR);
    } else if (data.output_sel == PLAYBACK_DST_HP_FP) {
//
// Unmute FP amplifier, switch rear jack to CS4361;
// I2S channels 2,3,4 should be inactive.
//
    oxygen_clear_bits8(chip, OXYGEN_GPIO_DATA, GPIO_HP_REAR);
    data.cs4245_shadow[CS4245_SIGNAL_SEL] |= CS4245_A_OUT_SEL_DAC;
    } else {
//
// 2.0, 4.0, 5.1: switch to CS4361, mute FP amp.,
// and change playback routing.
//
    oxygen_clear_bits8(chip, OXYGEN_GPIO_DATA, GPIO_HP_REAR);
    }
    return cs4245_write_spi(chip, CS4245_SIGNAL_SEL);
    }
    static int output_select_info(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_info *info)
    {
    static const char *const names[3] = {
    "Stereo Headphones",
    "Stereo Headphones FP",
    "Multichannel",
    };
    return snd_ctl_enum_info(info, 1, 3, names);
    }
    static int output_select_get(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    guard(mutex)(&chip.mutex);
    value.value.enumerated.item[0] = data.output_sel;
    return 0;
    }
    static int output_select_put(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    let mut new: c_uint = value.value.enumerated.item[0];
    let mut changed: c_int = 0;
    int ret;
    guard(mutex)(&chip.mutex);
    if (data.output_sel != new) {
    data.output_sel = new;
    ret = output_select_apply(chip);
    changed = ret >= 0 ? 1 : ret;
    oxygen_update_dac_routing(chip);
    }
    return changed;
    }
// CS4245 Headphone Channels A&B Volume Control
    static int hp_stereo_volume_info(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_info *info)
    {
    info.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    info.count = 2;
    info.value.integer.min = 0;
    info.value.integer.max = 255;
    return 0;
    }
    static int hp_stereo_volume_get(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *val)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    unsigned int tmp;
    guard(mutex)(&chip.mutex);
    tmp = (~data.cs4245_shadow[CS4245_DAC_A_CTRL]) & 255;
    val.value.integer.value[0] = tmp;
    tmp = (~data.cs4245_shadow[CS4245_DAC_B_CTRL]) & 255;
    val.value.integer.value[1] = tmp;
    return 0;
    }
    static int hp_stereo_volume_put(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *val)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    int ret;
    let mut changed: c_int = 0;
    let mut new1: c_long = val.value.integer.value[0];
    let mut new2: c_long = val.value.integer.value[1];
    if ((new1 > 255) || (new1 < 0) || (new2 > 255) || (new2 < 0))
    return -EINVAL;
    guard(mutex)(&chip.mutex);
    if ((data.cs4245_shadow[CS4245_DAC_A_CTRL] != ~new1) ||
    (data.cs4245_shadow[CS4245_DAC_B_CTRL] != ~new2)) {
    data.cs4245_shadow[CS4245_DAC_A_CTRL] = ~new1;
    data.cs4245_shadow[CS4245_DAC_B_CTRL] = ~new2;
    ret = cs4245_write_spi(chip, CS4245_DAC_A_CTRL);
    if (ret >= 0)
    ret = cs4245_write_spi(chip, CS4245_DAC_B_CTRL);
    changed = ret >= 0 ? 1 : ret;
    }
    return changed;
    }
// Headphone Mute
    static int hp_mute_get(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *val)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    guard(mutex)(&chip.mutex);
    val.value.integer.value[0] =
    !(data.cs4245_shadow[CS4245_DAC_CTRL_1] & CS4245_MUTE_DAC);
    return 0;
    }
    static int hp_mute_put(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *val)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    int ret;
    int changed;
    if (val.value.integer.value[0] > 1)
    return -EINVAL;
    guard(mutex)(&chip.mutex);
    data.cs4245_shadow[CS4245_DAC_CTRL_1] &= ~CS4245_MUTE_DAC;
    data.cs4245_shadow[CS4245_DAC_CTRL_1] |=
    (~val.value.integer.value[0] << 2) & CS4245_MUTE_DAC;
    ret = cs4245_write_spi(chip, CS4245_DAC_CTRL_1);
    changed = ret >= 0 ? 1 : ret;
    return changed;
    }
// capture volume for all sources
#[no_mangle]
unsafe extern "C" fn input_volume_apply(chip: *mut oxygen, left: c_char, right: c_char) -> c_int {
    static int input_volume_apply(struct oxygen *chip, char left, char right)
    {
    struct dg *data = chip.model_data;
    int ret;
    data.cs4245_shadow[CS4245_PGA_A_CTRL] = left;
    data.cs4245_shadow[CS4245_PGA_B_CTRL] = right;
    ret = cs4245_write_spi(chip, CS4245_PGA_A_CTRL);
    if (ret < 0)
    return ret;
    return cs4245_write_spi(chip, CS4245_PGA_B_CTRL);
    }
    static int input_vol_info(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_info *info)
    {
    info.type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    info.count = 2;
    info.value.integer.min = 2 * -12;
    info.value.integer.max = 2 * 12;
    return 0;
    }
    static int input_vol_get(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    let mut idx: c_uint = ctl.private_value;
    guard(mutex)(&chip.mutex);
    value.value.integer.value[0] = data.input_vol[idx][0];
    value.value.integer.value[1] = data.input_vol[idx][1];
    return 0;
    }
    static int input_vol_put(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    let mut idx: c_uint = ctl.private_value;
    let mut changed: c_int = 0;
    let mut ret: c_int = 0;
    if (value.value.integer.value[0] < 2 * -12 ||
    value.value.integer.value[0] > 2 * 12 ||
    value.value.integer.value[1] < 2 * -12 ||
    value.value.integer.value[1] > 2 * 12)
    return -EINVAL;
    guard(mutex)(&chip.mutex);
    changed = data.input_vol[idx][0] != value.value.integer.value[0] ||
    data.input_vol[idx][1] != value.value.integer.value[1];
    if (changed) {
    data.input_vol[idx][0] = value.value.integer.value[0];
    data.input_vol[idx][1] = value.value.integer.value[1];
    if (idx == data.input_sel) {
    ret = input_volume_apply(chip,
    data.input_vol[idx][0],
    data.input_vol[idx][1]);
    }
    changed = ret >= 0 ? 1 : ret;
    }
    return changed;
    }
// Capture Source
#[no_mangle]
unsafe extern "C" fn input_source_apply(chip: *mut oxygen) -> c_int {
    static int input_source_apply(struct oxygen *chip)
    {
    struct dg *data = chip.model_data;
    data.cs4245_shadow[CS4245_ANALOG_IN] &= ~CS4245_SEL_MASK;
    if (data.input_sel == CAPTURE_SRC_FP_MIC)
    data.cs4245_shadow[CS4245_ANALOG_IN] |= CS4245_SEL_INPUT_2;
#[no_mangle]
pub unsafe extern "C" fn if(CAPTURE_SRC_LINE: data->input_sel ==) -> else {
    else if (data.input_sel == CAPTURE_SRC_LINE)
    data.cs4245_shadow[CS4245_ANALOG_IN] |= CS4245_SEL_INPUT_4;
#[no_mangle]
pub unsafe extern "C" fn if(CAPTURE_SRC_MIC: data->input_sel !=) -> else {
    else if (data.input_sel != CAPTURE_SRC_MIC)
    data.cs4245_shadow[CS4245_ANALOG_IN] |= CS4245_SEL_INPUT_1;
    return cs4245_write_spi(chip, CS4245_ANALOG_IN);
    }
    static int input_sel_info(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_info *info)
    {
    static const char *const names[4] = {
    "Mic", "Front Mic", "Line", "Aux"
    };
    return snd_ctl_enum_info(info, 1, 4, names);
    }
    static int input_sel_get(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    guard(mutex)(&chip.mutex);
    value.value.enumerated.item[0] = data.input_sel;
    return 0;
    }
    static int input_sel_put(struct snd_kcontrol *ctl,
    struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    int changed;
    int ret;
    if (value.value.enumerated.item[0] > 3)
    return -EINVAL;
    guard(mutex)(&chip.mutex);
    changed = value.value.enumerated.item[0] != data.input_sel;
    if (changed) {
    data.input_sel = value.value.enumerated.item[0];
    ret = input_source_apply(chip);
    if (ret >= 0)
    ret = input_volume_apply(chip,
    data.input_vol[data.input_sel][0],
    data.input_vol[data.input_sel][1]);
    changed = ret >= 0 ? 1 : ret;
    }
    return changed;
    }
// ADC high-pass filter
#[no_mangle]
unsafe extern "C" fn hpf_info(ctl: *mut snd_kcontrol, info: *mut snd_ctl_elem_info) -> c_int {
    static int hpf_info(struct snd_kcontrol *ctl, struct snd_ctl_elem_info *info)
    {
    static const char *const names[2] = { "Active", "Frozen" };
    return snd_ctl_enum_info(info, 1, 2, names);
    }
#[no_mangle]
unsafe extern "C" fn hpf_get(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    static int hpf_get(struct snd_kcontrol *ctl, struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    value.value.enumerated.item[0] =
    !!(data.cs4245_shadow[CS4245_ADC_CTRL] & CS4245_HPF_FREEZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpf_put(ctl: *mut snd_kcontrol, value: *mut snd_ctl_elem_value) -> c_int {
    static int hpf_put(struct snd_kcontrol *ctl, struct snd_ctl_elem_value *value)
    {
    struct oxygen *chip = ctl.private_data;
    struct dg *data = chip.model_data;
    u8 reg;
    int changed;
    guard(mutex)(&chip.mutex);
    reg = data.cs4245_shadow[CS4245_ADC_CTRL] & ~CS4245_HPF_FREEZE;
    if (value.value.enumerated.item[0])
    reg |= CS4245_HPF_FREEZE;
    changed = reg != data.cs4245_shadow[CS4245_ADC_CTRL];
    if (changed) {
    data.cs4245_shadow[CS4245_ADC_CTRL] = reg;
    cs4245_write_spi(chip, CS4245_ADC_CTRL);
    }
    return changed;
    }

    .iface = SNDRV_CTL_ELEM_IFACE_MIXER, \
    .name = xname, \
    .access = SNDRV_CTL_ELEM_ACCESS_READWRITE | \
    SNDRV_CTL_ELEM_ACCESS_TLV_READ, \
    .info = input_vol_info, \
    .get = input_vol_get, \
    .put = input_vol_put, \
    .tlv = { .p = pga_db_scale }, \
    .private_value = index, \
    }
    static const DECLARE_TLV_DB_MINMAX(hp_db_scale, -12550, 0);
    static const DECLARE_TLV_DB_MINMAX(pga_db_scale, -1200, 1200);
    static const struct snd_kcontrol_new dg_controls[] = {
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Analog Output Playback Enum",
    .info = output_select_info,
    .get = output_select_get,
    .put = output_select_put,
    },
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Headphone Playback Volume",
    .access = SNDRV_CTL_ELEM_ACCESS_READWRITE |
    SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    .info = hp_stereo_volume_info,
    .get = hp_stereo_volume_get,
    .put = hp_stereo_volume_put,
    .tlv = { .p = hp_db_scale, },
    },
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Headphone Playback Switch",
    .access = SNDRV_CTL_ELEM_ACCESS_READWRITE,
    .info = snd_ctl_boolean_mono_info,
    .get = hp_mute_get,
    .put = hp_mute_put,
    },
    INPUT_VOLUME("Mic Capture Volume", CAPTURE_SRC_MIC),
    INPUT_VOLUME("Front Mic Capture Volume", CAPTURE_SRC_FP_MIC),
    INPUT_VOLUME("Line Capture Volume", CAPTURE_SRC_LINE),
    INPUT_VOLUME("Aux Capture Volume", CAPTURE_SRC_AUX),
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "Capture Source",
    .info = input_sel_info,
    .get = input_sel_get,
    .put = input_sel_put,
    },
    {
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER,
    .name = "ADC High-pass Filter Capture Enum",
    .info = hpf_info,
    .get = hpf_get,
    .put = hpf_put,
    },
    };
#[no_mangle]
unsafe extern "C" fn dg_control_filter(template: *mut snd_kcontrol_new) -> c_int {
    static int dg_control_filter(struct snd_kcontrol_new *template)
    {
    if (!strncmp(template.name, "Master Playback ", 16))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dg_mixer_init(chip: *mut oxygen) -> c_int {
    static int dg_mixer_init(struct oxygen *chip)
    {
    unsigned int i;
    int err;
    output_select_apply(chip);
    input_source_apply(chip);
    oxygen_update_dac_routing(chip);
    for (i = 0; i < ARRAY_SIZE(dg_controls); ++i) {
    err = snd_ctl_add(chip.card,
    snd_ctl_new1(&dg_controls[i], chip));
    if (err < 0)
    return err;
    }
    return 0;
    }
    const struct oxygen_model model_xonar_dg = {
    .longname = "C-Media Oxygen HD Audio",
    .chip = "CMI8786",
    .init = dg_init,
    .control_filter = dg_control_filter,
    .mixer_init = dg_mixer_init,
    .cleanup = dg_cleanup,
    .suspend = dg_suspend,
    .resume = dg_resume,
    .set_dac_params = set_cs4245_dac_params,
    .set_adc_params = set_cs4245_adc_params,
    .adjust_dac_routing = adjust_dg_dac_routing,
    .dump_registers = dump_cs4245_registers,
    .model_data_size = sizeof(struct dg),
    .device_config = PLAYBACK_0_TO_I2S |
    PLAYBACK_1_TO_SPDIF |
    CAPTURE_0_FROM_I2S_1 |
    CAPTURE_1_FROM_SPDIF,
    .dac_channels_pcm = 6,
    .dac_channels_mixer = 0,
    .function_flags = OXYGEN_FUNCTION_SPI,
    .dac_mclks = OXYGEN_MCLKS(256, 128, 128),
    .adc_mclks = OXYGEN_MCLKS(256, 128, 128),
    .dac_i2s_format = OXYGEN_I2S_FORMAT_LJUST,
    .adc_i2s_format = OXYGEN_I2S_FORMAT_LJUST,
    };
