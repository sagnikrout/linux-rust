//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tas5086.c
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
// TAS5086 ASoC codec driver
//
// Copyright (c) 2013 Daniel Mack <zonque@gmail.com>
//
// TODO:
// - implement DAPM and input muxing
// - implement modulation limit
// - implement non-default PWM start
//
// Note that this chip has a very unusual register layout, specifically
// because the registers are of unequal size, and multi-byte registers
// require bulk writes to take effect. Regmap does not support that kind
// of devices.
//
// Currently, the driver does not touch any of the registers >= 0x20, so
// it doesn't matter because the entire map can be accessed as 8-bit
// array. In case more features will be added in the future
// that require access to higher registers, the entire regmap H/W I/O
// routines have to be open-coded.
//

    SNDRV_PCM_FMTBIT_S20_3LE |		\
    SNDRV_PCM_FMTBIT_S24_3LE)

    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200  | \
    SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 | \
    SNDRV_PCM_RATE_192000)
//
// TAS5086 registers
//
pub const TAS5086_CLOCK_CONTROL: c_uint = 0x00	/* Clock control register  */;

pub const TAS5086_DEEMPH_MASK: c_uint = 0x03;
pub const TAS5086_SOFT_MUTE_ALL: c_uint = 0x3f;
pub const TAS5086_DEV_ID: c_uint = 0x01	/* Device ID register */;
pub const TAS5086_ERROR_STATUS: c_uint = 0x02	/* Error status register */;
pub const TAS5086_SYS_CONTROL_1: c_uint = 0x03	/* System control register 1 */;
pub const TAS5086_SERIAL_DATA_IF: c_uint = 0x04	/* Serial data interface register  */;
pub const TAS5086_SYS_CONTROL_2: c_uint = 0x05	/* System control register 2 */;
pub const TAS5086_SOFT_MUTE: c_uint = 0x06	/* Soft mute register */;
pub const TAS5086_MASTER_VOL: c_uint = 0x07	/* Master volume  */;

pub const TAS5086_VOLUME_CONTROL: c_uint = 0x09	/* Volume control register */;
pub const TAS5086_MOD_LIMIT: c_uint = 0x10	/* Modulation limit register */;
pub const TAS5086_PWM_START: c_uint = 0x18	/* PWM start register */;
pub const TAS5086_SURROUND: c_uint = 0x19	/* Surround register */;
pub const TAS5086_SPLIT_CAP_CHARGE: c_uint = 0x1a	/* Split cap charge period register */;
pub const TAS5086_OSC_TRIM: c_uint = 0x1b	/* Oscillator trim register */;
pub const TAS5086_BKNDERR: c_uint = 0x1c;
pub const TAS5086_INPUT_MUX: c_uint = 0x20;
pub const TAS5086_PWM_OUTPUT_MUX: c_uint = 0x25;

//
// Default TAS5086 power-up configuration
//
    static const struct reg_default tas5086_reg_defaults[] = {
    { 0x00,	0x6c },
    { 0x01,	0x03 },
    { 0x02,	0x00 },
    { 0x03,	0xa0 },
    { 0x04,	0x05 },
    { 0x05,	0x60 },
    { 0x06,	0x00 },
    { 0x07,	0xff },
    { 0x08,	0x30 },
    { 0x09,	0x30 },
    { 0x0a,	0x30 },
    { 0x0b,	0x30 },
    { 0x0c,	0x30 },
    { 0x0d,	0x30 },
    { 0x0e,	0xb1 },
    { 0x0f,	0x00 },
    { 0x10,	0x02 },
    { 0x11,	0x00 },
    { 0x12,	0x00 },
    { 0x13,	0x00 },
    { 0x14,	0x00 },
    { 0x15,	0x00 },
    { 0x16,	0x00 },
    { 0x17,	0x00 },
    { 0x18,	0x3f },
    { 0x19,	0x00 },
    { 0x1a,	0x18 },
    { 0x1b,	0x82 },
    { 0x1c,	0x05 },
    };
#[no_mangle]
unsafe extern "C" fn tas5086_register_size(dev: *mut device, reg: c_uint) -> c_int {
    static int tas5086_register_size(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case TAS5086_CLOCK_CONTROL ... TAS5086_BKNDERR:
    return 1;
    case TAS5086_INPUT_MUX:
    case TAS5086_PWM_OUTPUT_MUX:
    return 4;
    }
    dev_err(dev, "Unsupported register address: %d\n", reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tas5086_accessible_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool tas5086_accessible_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case 0x0f:
    case 0x11 ... 0x17:
    case 0x1d ... 0x1f:
    return false;
    default:
    return true;
    }
    }
#[no_mangle]
unsafe extern "C" fn tas5086_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool tas5086_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case TAS5086_DEV_ID:
    case TAS5086_ERROR_STATUS:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn tas5086_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool tas5086_writeable_reg(struct device *dev, unsigned int reg)
    {
    return tas5086_accessible_reg(dev, reg) && (reg != TAS5086_DEV_ID);
    }
    static int tas5086_reg_write(void *context, unsigned int reg,
    unsigned int value)
    {
    struct i2c_client *client = context;
    unsigned int i, size;
    uint8_t buf[5];
    int ret;
    size = tas5086_register_size(&client.dev, reg);
    if (size == 0)
    return -EINVAL;
    buf[0] = reg;
    for (i = size; i >= 1; --i) {
    buf[i] = value;
    value >>= 8;
    }
    ret = i2c_master_send(client, buf, size + 1);
    if (ret == size + 1)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    return ret;
    else
    return -EIO;
    }
    static int tas5086_reg_read(void *context, unsigned int reg,
    unsigned int *value)
    {
    struct i2c_client *client = context;
    uint8_t send_buf, recv_buf[4];
    struct i2c_msg msgs[2];
    unsigned int size;
    unsigned int i;
    int ret;
    size = tas5086_register_size(&client.dev, reg);
    if (size == 0)
    return -EINVAL;
    send_buf = reg;
    msgs[0].addr = client.addr;
    msgs[0].len = sizeof(send_buf);
    msgs[0].buf = &send_buf;
    msgs[0].flags = 0;
    msgs[1].addr = client.addr;
    msgs[1].len = size;
    msgs[1].buf = recv_buf;
    msgs[1].flags = I2C_M_RD;
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(ARRAY_SIZE(msgs): ret !=) -> else {
    else if (ret != ARRAY_SIZE(msgs))
    return -EIO;
// value = 0;
    for (i = 0; i < size; i++) {
// value <<= 8;
// value |= recv_buf[i];
    }
    return 0;
    }
    static const char * const supply_names[] = {
    "dvdd", "avdd"
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tas5086_private {
    pub regmap: *mut regmap,
    pub sclk: unsigned int mclk,,
    pub format: c_uint,
    pub deemph: bool,
    pub charge_period: c_uint,
    pub pwm_start_mid_z: c_uint,
// Current sample rate for de-emphasis control
    pub rate: c_int,
// GPIO driving Reset pin, if any
    pub reset: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; ARRAY_SIZE(supply_names)],
}

    static int tas5086_deemph[] = { 0, 32000, 44100, 48000 };
#[no_mangle]
unsafe extern "C" fn tas5086_set_deemph(component: *mut snd_soc_component) -> c_int {
    static int tas5086_set_deemph(struct snd_soc_component *component)
    {
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    int i, val = 0;
    if (priv.deemph) {
    for (i = 0; i < ARRAY_SIZE(tas5086_deemph); i++) {
    if (tas5086_deemph[i] == priv.rate) {
    val = i;
    break;
    }
    }
    }
    return regmap_update_bits(priv.regmap, TAS5086_SYS_CONTROL_1,
    TAS5086_DEEMPH_MASK, val);
    }
    static int tas5086_get_deemph(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    ucontrol.value.integer.value[0] = priv.deemph;
    return 0;
    }
    static int tas5086_put_deemph(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    priv.deemph = ucontrol.value.integer.value[0];
    return tas5086_set_deemph(component);
    }
    static int tas5086_set_dai_sysclk(struct snd_soc_dai *codec_dai,
    int clk_id, unsigned int freq, int dir)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    switch (clk_id) {
    case TAS5086_CLK_IDX_MCLK:
    priv.mclk = freq;
    break;
    case TAS5086_CLK_IDX_SCLK:
    priv.sclk = freq;
    break;
    }
    return 0;
    }
    static int tas5086_set_dai_fmt(struct snd_soc_dai *codec_dai,
    unsigned int format)
    {
    struct snd_soc_component *component = codec_dai.component;
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
// The TAS5086 can only be slave to all clocks
    if ((format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC) {
    dev_err(component.dev, "Invalid clocking mode\n");
    return -EINVAL;
    }
// we need to refer to the data format from hw_params()
    priv.format = format;
    return 0;
    }
    static const int tas5086_sample_rates[] = {
    32000, 38000, 44100, 48000, 88200, 96000, 176400, 192000
    };
    static const int tas5086_ratios[] = {
    64, 128, 192, 256, 384, 512
    };
#[no_mangle]
unsafe extern "C" fn index_in_array(array: *const c_int, len: c_int, needle: c_int) -> c_int {
    static int index_in_array(const int *array, int len, int needle)
    {
    int i;
    for (i = 0; i < len; i++)
    if (array[i] == needle)
    return i;
    return -ENOENT;
    }
    static int tas5086_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    int val;
    int ret;
    priv.rate = params_rate(params);
// Look up the sample rate and refer to the offset in the list
    val = index_in_array(tas5086_sample_rates,
    ARRAY_SIZE(tas5086_sample_rates), priv.rate);
    if (val < 0) {
    dev_err(component.dev, "Invalid sample rate\n");
    return -EINVAL;
    }
    ret = regmap_update_bits(priv.regmap, TAS5086_CLOCK_CONTROL,
    TAS5086_CLOCK_RATE_MASK,
    TAS5086_CLOCK_RATE(val));
    if (ret < 0)
    return ret;
// MCLK / Fs ratio
    val = index_in_array(tas5086_ratios, ARRAY_SIZE(tas5086_ratios),
    priv.mclk / priv.rate);
    if (val < 0) {
    dev_err(component.dev, "Invalid MCLK / Fs ratio\n");
    return -EINVAL;
    }
    ret = regmap_update_bits(priv.regmap, TAS5086_CLOCK_CONTROL,
    TAS5086_CLOCK_RATIO_MASK,
    TAS5086_CLOCK_RATIO(val));
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(priv.regmap, TAS5086_CLOCK_CONTROL,
    TAS5086_CLOCK_SCLK_RATIO_48,
    (priv.sclk == 48 * priv.rate) ?
    TAS5086_CLOCK_SCLK_RATIO_48 : 0);
    if (ret < 0)
    return ret;
//
// The chip has a very unituitive register mapping and muxes information
// about data format and sample depth into the same register, but not on
// a logical bit-boundary. Hence, we have to refer to the format passed
// in the set_dai_fmt() callback and set up everything from here.
//
// First, determine the 'base' value, using the format ...
//
    switch (priv.format & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_RIGHT_J:
    val = 0x00;
    break;
    case SND_SOC_DAIFMT_I2S:
    val = 0x03;
    break;
    case SND_SOC_DAIFMT_LEFT_J:
    val = 0x06;
    break;
    default:
    dev_err(component.dev, "Invalid DAI format\n");
    return -EINVAL;
    }
// ... then add the offset for the sample bit depth.
    switch (params_width(params)) {
    case 16:
    val += 0;
    break;
    case 20:
    val += 1;
    break;
    case 24:
    val += 2;
    break;
    default:
    dev_err(component.dev, "Invalid bit width\n");
    return -EINVAL;
    }
    ret = regmap_write(priv.regmap, TAS5086_SERIAL_DATA_IF, val);
    if (ret < 0)
    return ret;
// clock is considered valid now
    ret = regmap_update_bits(priv.regmap, TAS5086_CLOCK_CONTROL,
    TAS5086_CLOCK_VALID, TAS5086_CLOCK_VALID);
    if (ret < 0)
    return ret;
    return tas5086_set_deemph(component);
    }
#[no_mangle]
unsafe extern "C" fn tas5086_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    static int tas5086_mute_stream(struct snd_soc_dai *dai, int mute, int stream)
    {
    struct snd_soc_component *component = dai.component;
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    let mut val: c_uint = 0;
    if (mute)
    val = TAS5086_SOFT_MUTE_ALL;
    return regmap_write(priv.regmap, TAS5086_SOFT_MUTE, val);
    }
#[no_mangle]
unsafe extern "C" fn tas5086_reset(priv: *mut tas5086_private) {
    static void tas5086_reset(struct tas5086_private *priv)
    {
    if (priv.reset) {
// Reset codec - minimum assertion time is 400ns
    gpiod_set_value_cansleep(priv.reset, 1);
    udelay(1);
    gpiod_set_value_cansleep(priv.reset, 0);
// Codec needs ~15ms to wake up
    msleep(15);
    }
    }
// charge period values in microseconds
    static const int tas5086_charge_period[] = {
    13000,  16900,   23400,   31200,   41600,   54600,   72800,   96200,
    130000, 156000,  234000,  312000,  416000,  546000,  728000,  962000,
    1300000, 169000, 2340000, 3120000, 4160000, 5460000, 7280000, 9620000,
    };
#[no_mangle]
unsafe extern "C" fn tas5086_init(dev: *mut device, priv: *mut tas5086_private) -> c_int {
    static int tas5086_init(struct device *dev, struct tas5086_private *priv)
    {
    int ret, i;
//
// If any of the channels is configured to start in Mid-Z mode,
// configure 'part 1' of the PWM starts to use Mid-Z, and tell
// all configured mid-z channels to start under 'part 1'.
//
    if (priv.pwm_start_mid_z)
    regmap_write(priv.regmap, TAS5086_PWM_START,
    TAS5086_PWM_START_MIDZ_FOR_START_1 |
    priv.pwm_start_mid_z);
// lookup and set split-capacitor charge period
    if (priv.charge_period == 0) {
    regmap_write(priv.regmap, TAS5086_SPLIT_CAP_CHARGE, 0);
    } else {
    i = index_in_array(tas5086_charge_period,
    ARRAY_SIZE(tas5086_charge_period),
    priv.charge_period);
    if (i >= 0)
    regmap_write(priv.regmap, TAS5086_SPLIT_CAP_CHARGE,
    i + 0x08);
    else
    dev_warn(dev,
    "Invalid split-cap charge period of %d ns.\n",
    priv.charge_period);
    }
// enable factory trim
    ret = regmap_write(priv.regmap, TAS5086_OSC_TRIM, 0x00);
    if (ret < 0)
    return ret;
// start all channels
    ret = regmap_write(priv.regmap, TAS5086_SYS_CONTROL_2, 0x20);
    if (ret < 0)
    return ret;
// mute all channels for now
    ret = regmap_write(priv.regmap, TAS5086_SOFT_MUTE,
    TAS5086_SOFT_MUTE_ALL);
    if (ret < 0)
    return ret;
    return 0;
    }
// TAS5086 controls
    static const DECLARE_TLV_DB_SCALE(tas5086_dac_tlv, -10350, 50, 1);
    static const struct snd_kcontrol_new tas5086_controls[] = {
    SOC_SINGLE_TLV("Master Playback Volume", TAS5086_MASTER_VOL,
    0, 0xff, 1, tas5086_dac_tlv),
    SOC_DOUBLE_R_TLV("Channel 1/2 Playback Volume",
    TAS5086_CHANNEL_VOL(0), TAS5086_CHANNEL_VOL(1),
    0, 0xff, 1, tas5086_dac_tlv),
    SOC_DOUBLE_R_TLV("Channel 3/4 Playback Volume",
    TAS5086_CHANNEL_VOL(2), TAS5086_CHANNEL_VOL(3),
    0, 0xff, 1, tas5086_dac_tlv),
    SOC_DOUBLE_R_TLV("Channel 5/6 Playback Volume",
    TAS5086_CHANNEL_VOL(4), TAS5086_CHANNEL_VOL(5),
    0, 0xff, 1, tas5086_dac_tlv),
    SOC_SINGLE_BOOL_EXT("De-emphasis Switch", 0,
    tas5086_get_deemph, tas5086_put_deemph),
    };
// Input mux controls
    static const char *tas5086_dapm_sdin_texts[] =
    {
    "SDIN1-L", "SDIN1-R", "SDIN2-L", "SDIN2-R",
    "SDIN3-L", "SDIN3-R", "Ground (0)", "nc"
    };
    static const struct soc_enum tas5086_dapm_input_mux_enum[] = {
    SOC_ENUM_SINGLE(TAS5086_INPUT_MUX, 20, 8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE(TAS5086_INPUT_MUX, 16, 8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE(TAS5086_INPUT_MUX, 12, 8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE(TAS5086_INPUT_MUX, 8,  8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE(TAS5086_INPUT_MUX, 4,  8, tas5086_dapm_sdin_texts),
    SOC_ENUM_SINGLE(TAS5086_INPUT_MUX, 0,  8, tas5086_dapm_sdin_texts),
    };
    static const struct snd_kcontrol_new tas5086_dapm_input_mux_controls[] = {
    SOC_DAPM_ENUM("Channel 1 input", tas5086_dapm_input_mux_enum[0]),
    SOC_DAPM_ENUM("Channel 2 input", tas5086_dapm_input_mux_enum[1]),
    SOC_DAPM_ENUM("Channel 3 input", tas5086_dapm_input_mux_enum[2]),
    SOC_DAPM_ENUM("Channel 4 input", tas5086_dapm_input_mux_enum[3]),
    SOC_DAPM_ENUM("Channel 5 input", tas5086_dapm_input_mux_enum[4]),
    SOC_DAPM_ENUM("Channel 6 input", tas5086_dapm_input_mux_enum[5]),
    };
// Output mux controls
    static const char *tas5086_dapm_channel_texts[] =
    { "Channel 1 Mux", "Channel 2 Mux", "Channel 3 Mux",
    "Channel 4 Mux", "Channel 5 Mux", "Channel 6 Mux" };
    static const struct soc_enum tas5086_dapm_output_mux_enum[] = {
    SOC_ENUM_SINGLE(TAS5086_PWM_OUTPUT_MUX, 20, 6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE(TAS5086_PWM_OUTPUT_MUX, 16, 6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE(TAS5086_PWM_OUTPUT_MUX, 12, 6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE(TAS5086_PWM_OUTPUT_MUX, 8,  6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE(TAS5086_PWM_OUTPUT_MUX, 4,  6, tas5086_dapm_channel_texts),
    SOC_ENUM_SINGLE(TAS5086_PWM_OUTPUT_MUX, 0,  6, tas5086_dapm_channel_texts),
    };
    static const struct snd_kcontrol_new tas5086_dapm_output_mux_controls[] = {
    SOC_DAPM_ENUM("PWM1 Output", tas5086_dapm_output_mux_enum[0]),
    SOC_DAPM_ENUM("PWM2 Output", tas5086_dapm_output_mux_enum[1]),
    SOC_DAPM_ENUM("PWM3 Output", tas5086_dapm_output_mux_enum[2]),
    SOC_DAPM_ENUM("PWM4 Output", tas5086_dapm_output_mux_enum[3]),
    SOC_DAPM_ENUM("PWM5 Output", tas5086_dapm_output_mux_enum[4]),
    SOC_DAPM_ENUM("PWM6 Output", tas5086_dapm_output_mux_enum[5]),
    };
    static const struct snd_soc_dapm_widget tas5086_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("SDIN1-L"),
    SND_SOC_DAPM_INPUT("SDIN1-R"),
    SND_SOC_DAPM_INPUT("SDIN2-L"),
    SND_SOC_DAPM_INPUT("SDIN2-R"),
    SND_SOC_DAPM_INPUT("SDIN3-L"),
    SND_SOC_DAPM_INPUT("SDIN3-R"),
    SND_SOC_DAPM_INPUT("SDIN4-L"),
    SND_SOC_DAPM_INPUT("SDIN4-R"),
    SND_SOC_DAPM_OUTPUT("PWM1"),
    SND_SOC_DAPM_OUTPUT("PWM2"),
    SND_SOC_DAPM_OUTPUT("PWM3"),
    SND_SOC_DAPM_OUTPUT("PWM4"),
    SND_SOC_DAPM_OUTPUT("PWM5"),
    SND_SOC_DAPM_OUTPUT("PWM6"),
    SND_SOC_DAPM_MUX("Channel 1 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_input_mux_controls[0]),
    SND_SOC_DAPM_MUX("Channel 2 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_input_mux_controls[1]),
    SND_SOC_DAPM_MUX("Channel 3 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_input_mux_controls[2]),
    SND_SOC_DAPM_MUX("Channel 4 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_input_mux_controls[3]),
    SND_SOC_DAPM_MUX("Channel 5 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_input_mux_controls[4]),
    SND_SOC_DAPM_MUX("Channel 6 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_input_mux_controls[5]),
    SND_SOC_DAPM_MUX("PWM1 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_output_mux_controls[0]),
    SND_SOC_DAPM_MUX("PWM2 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_output_mux_controls[1]),
    SND_SOC_DAPM_MUX("PWM3 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_output_mux_controls[2]),
    SND_SOC_DAPM_MUX("PWM4 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_output_mux_controls[3]),
    SND_SOC_DAPM_MUX("PWM5 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_output_mux_controls[4]),
    SND_SOC_DAPM_MUX("PWM6 Mux", SND_SOC_NOPM, 0, 0,
    &tas5086_dapm_output_mux_controls[5]),
    };
    static const struct snd_soc_dapm_route tas5086_dapm_routes[] = {
// SDIN inputs -> channel muxes
    { "Channel 1 Mux", "SDIN1-L", "SDIN1-L" },
    { "Channel 1 Mux", "SDIN1-R", "SDIN1-R" },
    { "Channel 1 Mux", "SDIN2-L", "SDIN2-L" },
    { "Channel 1 Mux", "SDIN2-R", "SDIN2-R" },
    { "Channel 1 Mux", "SDIN3-L", "SDIN3-L" },
    { "Channel 1 Mux", "SDIN3-R", "SDIN3-R" },
    { "Channel 2 Mux", "SDIN1-L", "SDIN1-L" },
    { "Channel 2 Mux", "SDIN1-R", "SDIN1-R" },
    { "Channel 2 Mux", "SDIN2-L", "SDIN2-L" },
    { "Channel 2 Mux", "SDIN2-R", "SDIN2-R" },
    { "Channel 2 Mux", "SDIN3-L", "SDIN3-L" },
    { "Channel 2 Mux", "SDIN3-R", "SDIN3-R" },
    { "Channel 2 Mux", "SDIN1-L", "SDIN1-L" },
    { "Channel 2 Mux", "SDIN1-R", "SDIN1-R" },
    { "Channel 2 Mux", "SDIN2-L", "SDIN2-L" },
    { "Channel 2 Mux", "SDIN2-R", "SDIN2-R" },
    { "Channel 2 Mux", "SDIN3-L", "SDIN3-L" },
    { "Channel 2 Mux", "SDIN3-R", "SDIN3-R" },
    { "Channel 3 Mux", "SDIN1-L", "SDIN1-L" },
    { "Channel 3 Mux", "SDIN1-R", "SDIN1-R" },
    { "Channel 3 Mux", "SDIN2-L", "SDIN2-L" },
    { "Channel 3 Mux", "SDIN2-R", "SDIN2-R" },
    { "Channel 3 Mux", "SDIN3-L", "SDIN3-L" },
    { "Channel 3 Mux", "SDIN3-R", "SDIN3-R" },
    { "Channel 4 Mux", "SDIN1-L", "SDIN1-L" },
    { "Channel 4 Mux", "SDIN1-R", "SDIN1-R" },
    { "Channel 4 Mux", "SDIN2-L", "SDIN2-L" },
    { "Channel 4 Mux", "SDIN2-R", "SDIN2-R" },
    { "Channel 4 Mux", "SDIN3-L", "SDIN3-L" },
    { "Channel 4 Mux", "SDIN3-R", "SDIN3-R" },
    { "Channel 5 Mux", "SDIN1-L", "SDIN1-L" },
    { "Channel 5 Mux", "SDIN1-R", "SDIN1-R" },
    { "Channel 5 Mux", "SDIN2-L", "SDIN2-L" },
    { "Channel 5 Mux", "SDIN2-R", "SDIN2-R" },
    { "Channel 5 Mux", "SDIN3-L", "SDIN3-L" },
    { "Channel 5 Mux", "SDIN3-R", "SDIN3-R" },
    { "Channel 6 Mux", "SDIN1-L", "SDIN1-L" },
    { "Channel 6 Mux", "SDIN1-R", "SDIN1-R" },
    { "Channel 6 Mux", "SDIN2-L", "SDIN2-L" },
    { "Channel 6 Mux", "SDIN2-R", "SDIN2-R" },
    { "Channel 6 Mux", "SDIN3-L", "SDIN3-L" },
    { "Channel 6 Mux", "SDIN3-R", "SDIN3-R" },
// Channel muxes -> PWM muxes
    { "PWM1 Mux", "Channel 1 Mux", "Channel 1 Mux" },
    { "PWM2 Mux", "Channel 1 Mux", "Channel 1 Mux" },
    { "PWM3 Mux", "Channel 1 Mux", "Channel 1 Mux" },
    { "PWM4 Mux", "Channel 1 Mux", "Channel 1 Mux" },
    { "PWM5 Mux", "Channel 1 Mux", "Channel 1 Mux" },
    { "PWM6 Mux", "Channel 1 Mux", "Channel 1 Mux" },
    { "PWM1 Mux", "Channel 2 Mux", "Channel 2 Mux" },
    { "PWM2 Mux", "Channel 2 Mux", "Channel 2 Mux" },
    { "PWM3 Mux", "Channel 2 Mux", "Channel 2 Mux" },
    { "PWM4 Mux", "Channel 2 Mux", "Channel 2 Mux" },
    { "PWM5 Mux", "Channel 2 Mux", "Channel 2 Mux" },
    { "PWM6 Mux", "Channel 2 Mux", "Channel 2 Mux" },
    { "PWM1 Mux", "Channel 3 Mux", "Channel 3 Mux" },
    { "PWM2 Mux", "Channel 3 Mux", "Channel 3 Mux" },
    { "PWM3 Mux", "Channel 3 Mux", "Channel 3 Mux" },
    { "PWM4 Mux", "Channel 3 Mux", "Channel 3 Mux" },
    { "PWM5 Mux", "Channel 3 Mux", "Channel 3 Mux" },
    { "PWM6 Mux", "Channel 3 Mux", "Channel 3 Mux" },
    { "PWM1 Mux", "Channel 4 Mux", "Channel 4 Mux" },
    { "PWM2 Mux", "Channel 4 Mux", "Channel 4 Mux" },
    { "PWM3 Mux", "Channel 4 Mux", "Channel 4 Mux" },
    { "PWM4 Mux", "Channel 4 Mux", "Channel 4 Mux" },
    { "PWM5 Mux", "Channel 4 Mux", "Channel 4 Mux" },
    { "PWM6 Mux", "Channel 4 Mux", "Channel 4 Mux" },
    { "PWM1 Mux", "Channel 5 Mux", "Channel 5 Mux" },
    { "PWM2 Mux", "Channel 5 Mux", "Channel 5 Mux" },
    { "PWM3 Mux", "Channel 5 Mux", "Channel 5 Mux" },
    { "PWM4 Mux", "Channel 5 Mux", "Channel 5 Mux" },
    { "PWM5 Mux", "Channel 5 Mux", "Channel 5 Mux" },
    { "PWM6 Mux", "Channel 5 Mux", "Channel 5 Mux" },
    { "PWM1 Mux", "Channel 6 Mux", "Channel 6 Mux" },
    { "PWM2 Mux", "Channel 6 Mux", "Channel 6 Mux" },
    { "PWM3 Mux", "Channel 6 Mux", "Channel 6 Mux" },
    { "PWM4 Mux", "Channel 6 Mux", "Channel 6 Mux" },
    { "PWM5 Mux", "Channel 6 Mux", "Channel 6 Mux" },
    { "PWM6 Mux", "Channel 6 Mux", "Channel 6 Mux" },
// The PWM muxes are directly connected to the PWM outputs
    { "PWM1", core::ptr::null_mut(), "PWM1 Mux" },
    { "PWM2", core::ptr::null_mut(), "PWM2 Mux" },
    { "PWM3", core::ptr::null_mut(), "PWM3 Mux" },
    { "PWM4", core::ptr::null_mut(), "PWM4 Mux" },
    { "PWM5", core::ptr::null_mut(), "PWM5 Mux" },
    { "PWM6", core::ptr::null_mut(), "PWM6 Mux" },
    };
    static const struct snd_soc_dai_ops tas5086_dai_ops = {
    .hw_params	= tas5086_hw_params,
    .set_sysclk	= tas5086_set_dai_sysclk,
    .set_fmt	= tas5086_set_dai_fmt,
    .mute_stream	= tas5086_mute_stream,
    };
    static struct snd_soc_dai_driver tas5086_dai = {
    .name = "tas5086-hifi",
    .playback = {
    .stream_name	= "Playback",
    .channels_min	= 2,
    .channels_max	= 6,
    .rates		= TAS5086_PCM_RATES,
    .formats	= TAS5086_PCM_FORMATS,
    },
    .ops = &tas5086_dai_ops,
    };

#[no_mangle]
unsafe extern "C" fn tas5086_soc_suspend(component: *mut snd_soc_component) -> c_int {
    static int tas5086_soc_suspend(struct snd_soc_component *component)
    {
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    int ret;
// Shut down all channels
    ret = regmap_write(priv.regmap, TAS5086_SYS_CONTROL_2, 0x60);
    if (ret < 0)
    return ret;
    regulator_bulk_disable(ARRAY_SIZE(priv.supplies), priv.supplies);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tas5086_soc_resume(component: *mut snd_soc_component) -> c_int {
    static int tas5086_soc_resume(struct snd_soc_component *component)
    {
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(priv.supplies), priv.supplies);
    if (ret < 0)
    return ret;
    tas5086_reset(priv);
    regcache_mark_dirty(priv.regmap);
    ret = tas5086_init(component.dev, priv);
    if (ret < 0)
    return ret;
    ret = regcache_sync(priv.regmap);
    if (ret < 0)
    return ret;
    return 0;
    }

    static const struct of_device_id tas5086_dt_ids[] = {
    { .compatible = "ti,tas5086", },
    { }
    };
    MODULE_DEVICE_TABLE(of, tas5086_dt_ids);

#[no_mangle]
unsafe extern "C" fn tas5086_probe(component: *mut snd_soc_component) -> c_int {
    static int tas5086_probe(struct snd_soc_component *component)
    {
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    int i, ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(priv.supplies), priv.supplies);
    if (ret < 0) {
    dev_err(component.dev, "Failed to enable regulators: %d\n", ret);
    return ret;
    }
    priv.pwm_start_mid_z = 0;
    priv.charge_period = 1300000; /* hardware default is 1300 ms */
    if (of_match_device(of_match_ptr(tas5086_dt_ids), component.dev)) {
    struct device_node *of_node = component.dev.of_node;
    of_property_read_u32(of_node, "ti,charge-period",
    &priv.charge_period);
    for (i = 0; i < 6; i++) {
    char name[25];
    snprintf(name, sizeof(name),
    "ti,mid-z-channel-%d", i + 1);
    if (of_property_read_bool(of_node, name))
    priv.pwm_start_mid_z |= 1 << i;
    }
    }
    tas5086_reset(priv);
    ret = tas5086_init(component.dev, priv);
    if (ret < 0)
    goto exit_disable_regulators;
// set master volume to 0 dB
    ret = regmap_write(priv.regmap, TAS5086_MASTER_VOL, 0x30);
    if (ret < 0)
    goto exit_disable_regulators;
    return 0;
    exit_disable_regulators:
    regulator_bulk_disable(ARRAY_SIZE(priv.supplies), priv.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tas5086_remove(component: *mut snd_soc_component) {
    static void tas5086_remove(struct snd_soc_component *component)
    {
    struct tas5086_private *priv = snd_soc_component_get_drvdata(component);
    if (priv.reset) {
// Set codec to the reset state
    gpiod_set_value_cansleep(priv.reset, 1);
    }
    regulator_bulk_disable(ARRAY_SIZE(priv.supplies), priv.supplies);
    };
    static const struct snd_soc_component_driver soc_component_dev_tas5086 = {
    .probe			= tas5086_probe,
    .remove			= tas5086_remove,
    .suspend		= tas5086_soc_suspend,
    .resume			= tas5086_soc_resume,
    .controls		= tas5086_controls,
    .num_controls		= ARRAY_SIZE(tas5086_controls),
    .dapm_widgets		= tas5086_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(tas5086_dapm_widgets),
    .dapm_routes		= tas5086_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(tas5086_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static const struct i2c_device_id tas5086_i2c_id[] = {
    { .name = "tas5086" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tas5086_i2c_id);
    static const struct regmap_config tas5086_regmap = {
    .reg_bits		= 8,
    .val_bits		= 32,
    .max_register		= TAS5086_MAX_REGISTER,
    .reg_defaults		= tas5086_reg_defaults,
    .num_reg_defaults	= ARRAY_SIZE(tas5086_reg_defaults),
    .cache_type		= REGCACHE_RBTREE,
    .volatile_reg		= tas5086_volatile_reg,
    .writeable_reg		= tas5086_writeable_reg,
    .readable_reg		= tas5086_accessible_reg,
    .reg_read		= tas5086_reg_read,
    .reg_write		= tas5086_reg_write,
    };
#[no_mangle]
unsafe extern "C" fn tas5086_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int tas5086_i2c_probe(struct i2c_client *i2c)
    {
    struct tas5086_private *priv;
    struct device *dev = &i2c.dev;
    int i, ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    for (i = 0; i < ARRAY_SIZE(supply_names); i++)
    priv.supplies[i].supply = supply_names[i];
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(priv.supplies),
    priv.supplies);
    if (ret < 0) {
    dev_err(dev, "Failed to get regulators: %d\n", ret);
    return ret;
    }
    priv.regmap = devm_regmap_init(dev, core::ptr::null_mut(), i2c, &tas5086_regmap);
    if (IS_ERR(priv.regmap)) {
    ret = PTR_ERR(priv.regmap);
    dev_err(&i2c.dev, "Failed to create regmap: %d\n", ret);
    return ret;
    }
    i2c_set_clientdata(i2c, priv);
// Request line asserted
    priv.reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(priv.reset))
    return PTR_ERR(priv.reset);
    gpiod_set_consumer_name(priv.reset, "TAS5086 Reset");
    ret = regulator_bulk_enable(ARRAY_SIZE(priv.supplies), priv.supplies);
    if (ret < 0) {
    dev_err(dev, "Failed to enable regulators: %d\n", ret);
    return ret;
    }
    tas5086_reset(priv);
// The TAS5086 always returns 0x03 in its TAS5086_DEV_ID register
    ret = regmap_read(priv.regmap, TAS5086_DEV_ID, &i);
    if (ret == 0 && i != 0x3) {
    dev_err(dev,
    "Failed to identify TAS5086 codec (got %02x)\n", i);
    ret = -ENODEV;
    }
//
// The chip has been identified, so we can turn off the power
// again until the dai link is set up.
//
    regulator_bulk_disable(ARRAY_SIZE(priv.supplies), priv.supplies);
    if (ret == 0)
    ret = devm_snd_soc_register_component(&i2c.dev,
    &soc_component_dev_tas5086,
    &tas5086_dai, 1);
    return ret;
    }
    static struct i2c_driver tas5086_i2c_driver = {
    .driver = {
    .name = "tas5086",
    .of_match_table = of_match_ptr(tas5086_dt_ids),
    },
    .id_table = tas5086_i2c_id,
    .probe = tas5086_i2c_probe,
    };
    module_i2c_driver(tas5086_i2c_driver);
    MODULE_AUTHOR("Daniel Mack <zonque@gmail.com>");
    MODULE_DESCRIPTION("Texas Instruments TAS5086 ALSA SoC Codec Driver");
    MODULE_LICENSE("GPL");
