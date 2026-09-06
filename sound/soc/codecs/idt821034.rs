//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/idt821034.c
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
// IDT821034 ALSA SoC driver
//
// Copyright 2022 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>

pub const IDT821034_NB_CHANNEL: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt821034_amp {
    pub gain: u16,
    pub is_muted: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt821034 {
    pub spi: *mut spi_device,
    pub mutex: mutex,
    pub /: *mut *mut u8 spi_tx_buf; / Cannot use stack area for SPI (dma-safe memory),
    pub /: *mut *mut u8 spi_rx_buf; / Cannot use stack area for SPI (dma-safe memory),
    struct {
    pub codec_conf: u8,
    struct {
    pub power: u8,
    pub tx_slot: u8,
    pub rx_slot: u8,
    pub slic_conf: u8,
    pub slic_control: u8,
    pub ch: [}; IDT821034_NB_CHANNEL],
    pub cache: },
    struct {
    struct {
    pub amp_out: idt821034_amp,
    pub amp_in: idt821034_amp,
    pub ch: [}; IDT821034_NB_CHANNEL],
    pub amps: },
    pub max_ch_playback: c_int,
    pub max_ch_capture: c_int,
    pub gpio_chip: gpio_chip,
}

#[no_mangle]
unsafe extern "C" fn idt821034_8bit_write(idt821034: *mut idt821034, val: u8) -> c_int {
    static int idt821034_8bit_write(struct idt821034 *idt821034, u8 val)
    {
    struct spi_transfer xfer[] = {
    {
    .tx_buf = &idt821034.spi_tx_buf,
    .len = 1,
    }, {
    .cs_off = 1,
    .tx_buf = &idt821034.spi_tx_buf,
    .len = 1,
    }
    };
    idt821034.spi_tx_buf = val;
    dev_vdbg(&idt821034.spi.dev, "spi xfer wr 0x%x\n", val);
    return spi_sync_transfer(idt821034.spi, xfer, 2);
    }
#[no_mangle]
unsafe extern "C" fn idt821034_2x8bit_write(idt821034: *mut idt821034, val1: u8, val2: u8) -> c_int {
    static int idt821034_2x8bit_write(struct idt821034 *idt821034, u8 val1, u8 val2)
    {
    int ret;
    ret = idt821034_8bit_write(idt821034, val1);
    if (ret)
    return ret;
    return idt821034_8bit_write(idt821034, val2);
    }
#[no_mangle]
unsafe extern "C" fn idt821034_8bit_read(idt821034: *mut idt821034, valw: u8, valr: *mut u8) -> c_int {
    static int idt821034_8bit_read(struct idt821034 *idt821034, u8 valw, u8 *valr)
    {
    struct spi_transfer xfer[] = {
    {
    .tx_buf = &idt821034.spi_tx_buf,
    .rx_buf = &idt821034.spi_rx_buf,
    .len = 1,
    }, {
    .cs_off = 1,
    .tx_buf = &idt821034.spi_tx_buf,
    .len = 1,
    }
    };
    int ret;
    idt821034.spi_tx_buf = valw;
    ret = spi_sync_transfer(idt821034.spi, xfer, 2);
    if (ret)
    return ret;
// valr = idt821034->spi_rx_buf;
    dev_vdbg(&idt821034.spi.dev, "spi xfer wr 0x%x, rd 0x%x\n",
    valw, *valr);
    return 0;
    }
// Available mode for the programming sequence

// Power values that can be used in 'power' (can be ORed)

#[no_mangle]
unsafe extern "C" fn idt821034_set_channel_power(idt821034: *mut idt821034, ch: u8, power: u8) -> c_int {
    static int idt821034_set_channel_power(struct idt821034 *idt821034, u8 ch, u8 power)
    {
    u8 conf;
    int ret;
    dev_dbg(&idt821034.spi.dev, "set_channel_power(%u, 0x%x)\n", ch, power);
    conf = IDT821034_MODE_CODEC(ch) | idt821034.cache.codec_conf;
    if (power & IDT821034_CONF_PWRUP_RX) {
    ret = idt821034_2x8bit_write(idt821034,
    conf | IDT821034_CONF_PWRUP_RX,
    idt821034.cache.ch[ch].rx_slot);
    if (ret)
    return ret;
    }
    if (power & IDT821034_CONF_PWRUP_TX) {
    ret = idt821034_2x8bit_write(idt821034,
    conf | IDT821034_CONF_PWRUP_TX,
    idt821034.cache.ch[ch].tx_slot);
    if (ret)
    return ret;
    }
    if (!(power & (IDT821034_CONF_PWRUP_TX | IDT821034_CONF_PWRUP_RX))) {
    ret = idt821034_2x8bit_write(idt821034, conf, 0);
    if (ret)
    return ret;
    }
    idt821034.cache.ch[ch].power = power;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_get_channel_power(idt821034: *mut idt821034, ch: u8) -> u8 {
    static u8 idt821034_get_channel_power(struct idt821034 *idt821034, u8 ch)
    {
    return idt821034.cache.ch[ch].power;
    }
// Codec configuration values that can be used in 'codec_conf' (can be ORed)

#[no_mangle]
unsafe extern "C" fn idt821034_set_codec_conf(idt821034: *mut idt821034, codec_conf: u8) -> c_int {
    static int idt821034_set_codec_conf(struct idt821034 *idt821034, u8 codec_conf)
    {
    u8 conf;
    u8 ts;
    int ret;
    dev_dbg(&idt821034.spi.dev, "set_codec_conf(0x%x)\n", codec_conf);
// codec conf fields are common to all channel.
// Arbitrary use of channel 0 for this configuration.
//
// Set Configuration Register
    conf = IDT821034_MODE_CODEC(0) | codec_conf;
// Update conf value and timeslot register value according
// to cache values
//
    if (idt821034.cache.ch[0].power & IDT821034_CONF_PWRUP_RX) {
    conf |= IDT821034_CONF_PWRUP_RX;
    ts = idt821034.cache.ch[0].rx_slot;
    } else if (idt821034.cache.ch[0].power & IDT821034_CONF_PWRUP_TX) {
    conf |= IDT821034_CONF_PWRUP_TX;
    ts = idt821034.cache.ch[0].tx_slot;
    } else {
    ts = 0x00;
    }
// Write configuration register and time-slot register
    ret = idt821034_2x8bit_write(idt821034, conf, ts);
    if (ret)
    return ret;
    idt821034.cache.codec_conf = codec_conf;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_get_codec_conf(idt821034: *mut idt821034) -> u8 {
    static u8 idt821034_get_codec_conf(struct idt821034 *idt821034)
    {
    return idt821034.cache.codec_conf;
    }
// Channel direction values that can be used in 'ch_dir' (can be ORed)

#[no_mangle]
unsafe extern "C" fn idt821034_set_channel_ts(idt821034: *mut idt821034, ch: u8, ch_dir: u8, ts_num: u8) -> c_int {
    static int idt821034_set_channel_ts(struct idt821034 *idt821034, u8 ch, u8 ch_dir, u8 ts_num)
    {
    u8 conf;
    int ret;
    dev_dbg(&idt821034.spi.dev, "set_channel_ts(%u, 0x%x, %d)\n", ch, ch_dir, ts_num);
    conf = IDT821034_MODE_CODEC(ch) | idt821034.cache.codec_conf;
    if (ch_dir & IDT821034_CH_RX) {
    if (idt821034.cache.ch[ch].power & IDT821034_CONF_PWRUP_RX) {
    ret = idt821034_2x8bit_write(idt821034,
    conf | IDT821034_CONF_PWRUP_RX,
    ts_num);
    if (ret)
    return ret;
    }
    idt821034.cache.ch[ch].rx_slot = ts_num;
    }
    if (ch_dir & IDT821034_CH_TX) {
    if (idt821034.cache.ch[ch].power & IDT821034_CONF_PWRUP_TX) {
    ret = idt821034_2x8bit_write(idt821034,
    conf | IDT821034_CONF_PWRUP_TX,
    ts_num);
    if (ret)
    return ret;
    }
    idt821034.cache.ch[ch].tx_slot = ts_num;
    }
    return 0;
    }
// SLIC direction values that can be used in 'slic_dir' (can be ORed)

#[no_mangle]
unsafe extern "C" fn idt821034_set_slic_conf(idt821034: *mut idt821034, ch: u8, slic_dir: u8) -> c_int {
    static int idt821034_set_slic_conf(struct idt821034 *idt821034, u8 ch, u8 slic_dir)
    {
    u8 conf;
    int ret;
    dev_dbg(&idt821034.spi.dev, "set_slic_conf(%u, 0x%x)\n", ch, slic_dir);
    conf = IDT821034_MODE_SLIC(ch) | slic_dir;
    ret = idt821034_2x8bit_write(idt821034, conf, idt821034.cache.ch[ch].slic_control);
    if (ret)
    return ret;
    idt821034.cache.ch[ch].slic_conf = slic_dir;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_get_slic_conf(idt821034: *mut idt821034, ch: u8) -> u8 {
    static u8 idt821034_get_slic_conf(struct idt821034 *idt821034, u8 ch)
    {
    return idt821034.cache.ch[ch].slic_conf;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_write_slic_raw(idt821034: *mut idt821034, ch: u8, slic_raw: u8) -> c_int {
    static int idt821034_write_slic_raw(struct idt821034 *idt821034, u8 ch, u8 slic_raw)
    {
    u8 conf;
    int ret;
    dev_dbg(&idt821034.spi.dev, "write_slic_raw(%u, 0x%x)\n", ch, slic_raw);
//
// On write, slic_raw is mapped as follow :
// b4: O_4
// b3: O_3
// b2: O_2
// b1: I/O_1
// b0: I/O_0
//
    conf = IDT821034_MODE_SLIC(ch) | idt821034.cache.ch[ch].slic_conf;
    ret = idt821034_2x8bit_write(idt821034, conf, slic_raw);
    if (ret)
    return ret;
    idt821034.cache.ch[ch].slic_control = slic_raw;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_get_written_slic_raw(idt821034: *mut idt821034, ch: u8) -> u8 {
    static u8 idt821034_get_written_slic_raw(struct idt821034 *idt821034, u8 ch)
    {
    return idt821034.cache.ch[ch].slic_control;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_read_slic_raw(idt821034: *mut idt821034, ch: u8, slic_raw: *mut u8) -> c_int {
    static int idt821034_read_slic_raw(struct idt821034 *idt821034, u8 ch, u8 *slic_raw)
    {
    u8 val;
    int ret;
//
// On read, slic_raw is mapped as follow :
// b7: I/O_0
// b6: I/O_1
// b5: O_2
// b4: O_3
// b3: O_4
// b2: I/O1_0, I/O_0 from channel 1 (no matter ch value)
// b1: I/O2_0, I/O_0 from channel 2 (no matter ch value)
// b2: I/O3_0, I/O_0 from channel 3 (no matter ch value)
//
    val = IDT821034_MODE_SLIC(ch) | idt821034.cache.ch[ch].slic_conf;
    ret = idt821034_8bit_write(idt821034, val);
    if (ret)
    return ret;
    ret = idt821034_8bit_read(idt821034, idt821034.cache.ch[ch].slic_control, slic_raw);
    if (ret)
    return ret;
    dev_dbg(&idt821034.spi.dev, "read_slic_raw(%i) 0x%x\n", ch, *slic_raw);
    return 0;
    }
// Gain type values that can be used in 'gain_type' (cannot be ORed)

    static int idt821034_set_gain_channel(struct idt821034 *idt821034, u8 ch,
    u8 gain_type, u16 gain_val)
    {
    u8 conf;
    int ret;
    dev_dbg(&idt821034.spi.dev, "set_gain_channel(%u, 0x%x, 0x%x-%d)\n",
    ch, gain_type, gain_val, gain_val);
//
// The gain programming coefficients should be calculated as:
// Transmit : Coeff_X = round [ gain_X0dB × gain_X ]
// Receive: Coeff_R = round [ gain_R0dB × gain_R ]
// where:
// gain_X0dB = 1820;
// gain_X is the target gain;
// Coeff_X should be in the range of 0 to 8192.
// gain_R0dB = 2506;
// gain_R is the target gain;
// Coeff_R should be in the range of 0 to 8192.
//
// A gain programming coefficient is 14-bit wide and in binary format.
// The 7 Most Significant Bits of the coefficient is called
// GA_MSB_Transmit for transmit path, or is called GA_MSB_Receive for
// receive path; The 7 Least Significant Bits of the coefficient is
// called GA_LSB_ Transmit for transmit path, or is called
// GA_LSB_Receive for receive path.
//
// An example is given below to clarify the calculation of the
// coefficient. To program a +3 dB gain in transmit path and a -3.5 dB
// gain in receive path:
//
// Linear Code of +3dB = 10^(3/20)= 1.412537545
// Coeff_X = round (1820 × 1.412537545) = 2571
// = 0b001010_00001011
// GA_MSB_Transmit = 0b0010100
// GA_LSB_Transmit = 0b0001011
//
// Linear Code of -3.5dB = 10^(-3.5/20) = 0.668343917
// Coeff_R= round (2506 × 0.668343917) = 1675
// = 0b0001101_0001011
// GA_MSB_Receive = 0b0001101
// GA_LSB_Receive = 0b0001011
//
    conf = IDT821034_MODE_GAIN(ch) | gain_type;
    ret = idt821034_2x8bit_write(idt821034, conf | 0x00, gain_val & 0x007F);
    if (ret)
    return ret;
    ret = idt821034_2x8bit_write(idt821034, conf | 0x01, (gain_val >> 7) & 0x7F);
    if (ret)
    return ret;
    return 0;
    }
// Id helpers used in controls and dapm

    static int idt821034_kctrl_gain_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_mixer_control *mc = (struct soc_mixer_control *)kcontrol.private_value;
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(component);
    let mut min: c_int = mc.min;
    let mut max: c_int = mc.max;
    let mut mask: c_uint = (1 << fls(max)) - 1;
    let mut invert: c_uint = mc.invert;
    int val;
    u8 ch;
    ch = IDT821034_ID_GET_CHAN(mc.reg);
    scoped_guard(mutex, &idt821034.mutex) {
    if (IDT821034_ID_IS_OUT(mc.reg))
    val = idt821034.amps.ch[ch].amp_out.gain;
    else
    val = idt821034.amps.ch[ch].amp_in.gain;
    }
    ucontrol.value.integer.value[0] = val & mask;
    if (invert)
    ucontrol.value.integer.value[0] = max - ucontrol.value.integer.value[0];
    else
    ucontrol.value.integer.value[0] = ucontrol.value.integer.value[0] - min;
    return 0;
    }
    static int idt821034_kctrl_gain_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct soc_mixer_control *mc = (struct soc_mixer_control *)kcontrol.private_value;
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(component);
    struct idt821034_amp *amp;
    let mut min: c_int = mc.min;
    let mut max: c_int = mc.max;
    let mut mask: c_uint = (1 << fls(max)) - 1;
    let mut invert: c_uint = mc.invert;
    unsigned int val;
    int ret;
    u8 gain_type;
    u8 ch;
    val = ucontrol.value.integer.value[0];
    if (val > max - min)
    return -EINVAL;
    if (invert)
    val = (max - val) & mask;
    else
    val = (val + min) & mask;
    ch = IDT821034_ID_GET_CHAN(mc.reg);
    guard(mutex)(&idt821034.mutex);
    if (IDT821034_ID_IS_OUT(mc.reg)) {
    amp = &idt821034.amps.ch[ch].amp_out;
    gain_type = IDT821034_GAIN_RX;
    } else {
    amp = &idt821034.amps.ch[ch].amp_in;
    gain_type = IDT821034_GAIN_TX;
    }
    if (amp.gain == val)
    return 0;
    if (!amp.is_muted) {
    ret = idt821034_set_gain_channel(idt821034, ch, gain_type, val);
    if (ret)
    return ret;
    }
    amp.gain = val;
    return 1;
    }
    static int idt821034_kctrl_mute_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(component);
    let mut id: c_int = kcontrol.private_value;
    bool is_muted;
    u8 ch;
    ch = IDT821034_ID_GET_CHAN(id);
    scoped_guard(mutex, &idt821034.mutex) {
    is_muted = IDT821034_ID_IS_OUT(id) ?
    idt821034.amps.ch[ch].amp_out.is_muted :
    idt821034.amps.ch[ch].amp_in.is_muted;
    }
    ucontrol.value.integer.value[0] = !is_muted;
    return 0;
    }
    static int idt821034_kctrl_mute_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(component);
    let mut id: c_int = kcontrol.private_value;
    struct idt821034_amp *amp;
    bool is_mute;
    u8 gain_type;
    int ret;
    u8 ch;
    ch = IDT821034_ID_GET_CHAN(id);
    is_mute = !ucontrol.value.integer.value[0];
    guard(mutex)(&idt821034.mutex);
    if (IDT821034_ID_IS_OUT(id)) {
    amp = &idt821034.amps.ch[ch].amp_out;
    gain_type = IDT821034_GAIN_RX;
    } else {
    amp = &idt821034.amps.ch[ch].amp_in;
    gain_type = IDT821034_GAIN_TX;
    }
    if (amp.is_muted == is_mute)
    return 0;
    ret = idt821034_set_gain_channel(idt821034, ch, gain_type,
    is_mute ? 0 : amp.gain);
    if (ret)
    return ret;
    amp.is_muted = is_mute;
    return 1;
    }
    static const DECLARE_TLV_DB_LINEAR(idt821034_gain_in, -300, 1300);

    static const DECLARE_TLV_DB_LINEAR(idt821034_gain_out, -1300, 300);

    static const struct snd_kcontrol_new idt821034_controls[] = {
// DAC volume control
    SOC_SINGLE_RANGE_EXT_TLV("DAC0 Playback Volume", IDT821034_ID_OUT(0), 0,
    IDT821034_GAIN_OUT_MIN_RAW, IDT821034_GAIN_OUT_MAX_RAW,
    0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put,
    idt821034_gain_out),
    SOC_SINGLE_RANGE_EXT_TLV("DAC1 Playback Volume", IDT821034_ID_OUT(1), 0,
    IDT821034_GAIN_OUT_MIN_RAW, IDT821034_GAIN_OUT_MAX_RAW,
    0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put,
    idt821034_gain_out),
    SOC_SINGLE_RANGE_EXT_TLV("DAC2 Playback Volume", IDT821034_ID_OUT(2), 0,
    IDT821034_GAIN_OUT_MIN_RAW, IDT821034_GAIN_OUT_MAX_RAW,
    0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put,
    idt821034_gain_out),
    SOC_SINGLE_RANGE_EXT_TLV("DAC3 Playback Volume", IDT821034_ID_OUT(3), 0,
    IDT821034_GAIN_OUT_MIN_RAW, IDT821034_GAIN_OUT_MAX_RAW,
    0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put,
    idt821034_gain_out),
// DAC mute control
    SOC_SINGLE_BOOL_EXT("DAC0 Playback Switch", IDT821034_ID_OUT(0),
    idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT("DAC1 Playback Switch", IDT821034_ID_OUT(1),
    idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT("DAC2 Playback Switch", IDT821034_ID_OUT(2),
    idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT("DAC3 Playback Switch", IDT821034_ID_OUT(3),
    idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
// ADC volume control
    SOC_SINGLE_RANGE_EXT_TLV("ADC0 Capture Volume", IDT821034_ID_IN(0), 0,
    IDT821034_GAIN_IN_MIN_RAW, IDT821034_GAIN_IN_MAX_RAW,
    0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put,
    idt821034_gain_in),
    SOC_SINGLE_RANGE_EXT_TLV("ADC1 Capture Volume", IDT821034_ID_IN(1), 0,
    IDT821034_GAIN_IN_MIN_RAW, IDT821034_GAIN_IN_MAX_RAW,
    0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put,
    idt821034_gain_in),
    SOC_SINGLE_RANGE_EXT_TLV("ADC2 Capture Volume", IDT821034_ID_IN(2), 0,
    IDT821034_GAIN_IN_MIN_RAW, IDT821034_GAIN_IN_MAX_RAW,
    0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put,
    idt821034_gain_in),
    SOC_SINGLE_RANGE_EXT_TLV("ADC3 Capture Volume", IDT821034_ID_IN(3), 0,
    IDT821034_GAIN_IN_MIN_RAW, IDT821034_GAIN_IN_MAX_RAW,
    0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put,
    idt821034_gain_in),
// ADC mute control
    SOC_SINGLE_BOOL_EXT("ADC0 Capture Switch", IDT821034_ID_IN(0),
    idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT("ADC1 Capture Switch", IDT821034_ID_IN(1),
    idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT("ADC2 Capture Switch", IDT821034_ID_IN(2),
    idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT("ADC3 Capture Switch", IDT821034_ID_IN(3),
    idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    };
    static int idt821034_power_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(component);
    let mut id: c_uint = w.shift;
    u8 power, mask;
    u8 ch;
    ch = IDT821034_ID_GET_CHAN(id);
    mask = IDT821034_ID_IS_OUT(id) ? IDT821034_CONF_PWRUP_RX : IDT821034_CONF_PWRUP_TX;
    guard(mutex)(&idt821034.mutex);
    power = idt821034_get_channel_power(idt821034, ch);
    if (SND_SOC_DAPM_EVENT_ON(event))
    power |= mask;
    else
    power &= ~mask;
    return idt821034_set_channel_power(idt821034, ch, power);
    }
    static const struct snd_soc_dapm_widget idt821034_dapm_widgets[] = {
    SND_SOC_DAPM_DAC_E("DAC0", "Playback", SND_SOC_NOPM, IDT821034_ID_OUT(0), 0,
    idt821034_power_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E("DAC1", "Playback", SND_SOC_NOPM, IDT821034_ID_OUT(1), 0,
    idt821034_power_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E("DAC2", "Playback", SND_SOC_NOPM, IDT821034_ID_OUT(2), 0,
    idt821034_power_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E("DAC3", "Playback", SND_SOC_NOPM, IDT821034_ID_OUT(3), 0,
    idt821034_power_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_OUTPUT("OUT0"),
    SND_SOC_DAPM_OUTPUT("OUT1"),
    SND_SOC_DAPM_OUTPUT("OUT2"),
    SND_SOC_DAPM_OUTPUT("OUT3"),
    SND_SOC_DAPM_DAC_E("ADC0", "Capture", SND_SOC_NOPM, IDT821034_ID_IN(0), 0,
    idt821034_power_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E("ADC1", "Capture", SND_SOC_NOPM, IDT821034_ID_IN(1), 0,
    idt821034_power_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E("ADC2", "Capture", SND_SOC_NOPM, IDT821034_ID_IN(2), 0,
    idt821034_power_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E("ADC3", "Capture", SND_SOC_NOPM, IDT821034_ID_IN(3), 0,
    idt821034_power_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_INPUT("IN0"),
    SND_SOC_DAPM_INPUT("IN1"),
    SND_SOC_DAPM_INPUT("IN2"),
    SND_SOC_DAPM_INPUT("IN3"),
    };
    static const struct snd_soc_dapm_route idt821034_dapm_routes[] = {
    { "OUT0", core::ptr::null_mut(), "DAC0" },
    { "OUT1", core::ptr::null_mut(), "DAC1" },
    { "OUT2", core::ptr::null_mut(), "DAC2" },
    { "OUT3", core::ptr::null_mut(), "DAC3" },
    { "ADC0", core::ptr::null_mut(), "IN0" },
    { "ADC1", core::ptr::null_mut(), "IN1" },
    { "ADC2", core::ptr::null_mut(), "IN2" },
    { "ADC3", core::ptr::null_mut(), "IN3" },
    };
    static int idt821034_dai_set_tdm_slot(struct snd_soc_dai *dai,
    unsigned int tx_mask, unsigned int rx_mask,
    int slots, int width)
    {
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(dai.component);
    unsigned int mask;
    u8 slot;
    int ret;
    u8 ch;
    switch (width) {
    case 0: /* Not set . default 8 */
    case 8:
    break;
    default:
    dev_err(dai.dev, "tdm slot width %d not supported\n", width);
    return -EINVAL;
    }
    mask = tx_mask;
    slot = 0;
    ch = 0;
    while (mask && ch < IDT821034_NB_CHANNEL) {
    if (mask & 0x1) {
    scoped_guard(mutex, &idt821034.mutex)
    ret = idt821034_set_channel_ts(idt821034, ch,
    IDT821034_CH_RX, slot);
    if (ret) {
    dev_err(dai.dev, "ch%u set tx tdm slot failed (%d)\n",
    ch, ret);
    return ret;
    }
    ch++;
    }
    mask >>= 1;
    slot++;
    }
    if (mask) {
    dev_err(dai.dev, "too much tx slots defined (mask = 0x%x) support max %d\n",
    tx_mask, IDT821034_NB_CHANNEL);
    return -EINVAL;
    }
    idt821034.max_ch_playback = ch;
    mask = rx_mask;
    slot = 0;
    ch = 0;
    while (mask && ch < IDT821034_NB_CHANNEL) {
    if (mask & 0x1) {
    scoped_guard(mutex, &idt821034.mutex)
    ret = idt821034_set_channel_ts(idt821034, ch,
    IDT821034_CH_TX, slot);
    if (ret) {
    dev_err(dai.dev, "ch%u set rx tdm slot failed (%d)\n",
    ch, ret);
    return ret;
    }
    ch++;
    }
    mask >>= 1;
    slot++;
    }
    if (mask) {
    dev_err(dai.dev, "too much rx slots defined (mask = 0x%x) support max %d\n",
    rx_mask, IDT821034_NB_CHANNEL);
    return -EINVAL;
    }
    idt821034.max_ch_capture = ch;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    static int idt821034_dai_set_fmt(struct snd_soc_dai *dai, unsigned int fmt)
    {
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(dai.component);
    u8 conf;
    guard(mutex)(&idt821034.mutex);
    conf = idt821034_get_codec_conf(idt821034);
    switch (fmt & SND_SOC_DAIFMT_FORMAT_MASK) {
    case SND_SOC_DAIFMT_DSP_A:
    conf |= IDT821034_CONF_DELAY_MODE;
    break;
    case SND_SOC_DAIFMT_DSP_B:
    conf &= ~IDT821034_CONF_DELAY_MODE;
    break;
    default:
    dev_err(dai.dev, "Unsupported DAI format 0x%x\n",
    fmt & SND_SOC_DAIFMT_FORMAT_MASK);
    return -EINVAL;
    }
    return idt821034_set_codec_conf(idt821034, conf);
    }
    static int idt821034_dai_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(dai.component);
    u8 conf;
    guard(mutex)(&idt821034.mutex);
    conf = idt821034_get_codec_conf(idt821034);
    switch (params_format(params)) {
    case SNDRV_PCM_FORMAT_A_LAW:
    conf |= IDT821034_CONF_ALAW_MODE;
    break;
    case SNDRV_PCM_FORMAT_MU_LAW:
    conf &= ~IDT821034_CONF_ALAW_MODE;
    break;
    default:
    dev_err(dai.dev, "Unsupported PCM format 0x%x\n",
    params_format(params));
    return -EINVAL;
    }
    return idt821034_set_codec_conf(idt821034, conf);
    }
    static const unsigned int idt821034_sample_bits[] = {8};
    static struct snd_pcm_hw_constraint_list idt821034_sample_bits_constr = {
    .list = idt821034_sample_bits,
    .count = ARRAY_SIZE(idt821034_sample_bits),
    };
    static int idt821034_dai_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(dai.component);
    let mut max_ch: c_uint = 0;
    int ret;
    max_ch = (substream.stream == SNDRV_PCM_STREAM_PLAYBACK) ?
    idt821034.max_ch_playback : idt821034.max_ch_capture;
//
// Disable stream support (min = 0, max = 0) if no timeslots were
// configured otherwise, limit the number of channels to those
// configured.
//
    ret = snd_pcm_hw_constraint_minmax(substream.runtime, SNDRV_PCM_HW_PARAM_CHANNELS,
    max_ch ? 1 : 0, max_ch);
    if (ret < 0)
    return ret;
    ret = snd_pcm_hw_constraint_list(substream.runtime, 0, SNDRV_PCM_HW_PARAM_SAMPLE_BITS,
    &idt821034_sample_bits_constr);
    if (ret)
    return ret;
    return 0;
    }
    static const u64 idt821034_dai_formats =
    SND_SOC_POSSIBLE_DAIFMT_DSP_A	|
    SND_SOC_POSSIBLE_DAIFMT_DSP_B;
    static const struct snd_soc_dai_ops idt821034_dai_ops = {
    .startup      = idt821034_dai_startup,
    .hw_params    = idt821034_dai_hw_params,
    .set_tdm_slot = idt821034_dai_set_tdm_slot,
    .set_fmt      = idt821034_dai_set_fmt,
    .auto_selectable_formats     = &idt821034_dai_formats,
    .num_auto_selectable_formats = 1,
    };
    static struct snd_soc_dai_driver idt821034_dai_driver = {
    .name = "idt821034",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 1,
    .channels_max = IDT821034_NB_CHANNEL,
    .rates = SNDRV_PCM_RATE_8000,
    .formats = SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
    },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 1,
    .channels_max = IDT821034_NB_CHANNEL,
    .rates = SNDRV_PCM_RATE_8000,
    .formats = SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
    },
    .ops = &idt821034_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn idt821034_reset_audio(idt821034: *mut idt821034) -> c_int {
    static int idt821034_reset_audio(struct idt821034 *idt821034)
    {
    int ret;
    u8 i;
    guard(mutex)(&idt821034.mutex);
    ret = idt821034_set_codec_conf(idt821034, 0);
    if (ret)
    return ret;
    for (i = 0; i < IDT821034_NB_CHANNEL; i++) {
    idt821034.amps.ch[i].amp_out.gain = IDT821034_GAIN_OUT_INIT_RAW;
    idt821034.amps.ch[i].amp_out.is_muted = false;
    ret = idt821034_set_gain_channel(idt821034, i, IDT821034_GAIN_RX,
    idt821034.amps.ch[i].amp_out.gain);
    if (ret)
    return ret;
    idt821034.amps.ch[i].amp_in.gain = IDT821034_GAIN_IN_INIT_RAW;
    idt821034.amps.ch[i].amp_in.is_muted = false;
    ret = idt821034_set_gain_channel(idt821034, i, IDT821034_GAIN_TX,
    idt821034.amps.ch[i].amp_in.gain);
    if (ret)
    return ret;
    ret = idt821034_set_channel_power(idt821034, i, 0);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_component_probe(component: *mut snd_soc_component) -> c_int {
    static int idt821034_component_probe(struct snd_soc_component *component)
    {
    struct idt821034 *idt821034 = snd_soc_component_get_drvdata(component);
    int ret;
// reset idt821034 audio part
    ret = idt821034_reset_audio(idt821034);
    if (ret)
    return ret;
    return 0;
    }
    static const struct snd_soc_component_driver idt821034_component_driver = {
    .probe			= idt821034_component_probe,
    .controls		= idt821034_controls,
    .num_controls		= ARRAY_SIZE(idt821034_controls),
    .dapm_widgets		= idt821034_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(idt821034_dapm_widgets),
    .dapm_routes		= idt821034_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(idt821034_dapm_routes),
    .endianness		= 1,
    };

    static int idt821034_chip_gpio_set(struct gpio_chip *c, unsigned int offset,
    int val)
    {
    let mut ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mut mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    struct idt821034 *idt821034 = gpiochip_get_data(c);
    u8 slic_raw;
    int ret;
    guard(mutex)(&idt821034.mutex);
    slic_raw = idt821034_get_written_slic_raw(idt821034, ch);
    if (val)
    slic_raw |= mask;
    else
    slic_raw &= ~mask;
    ret = idt821034_write_slic_raw(idt821034, ch, slic_raw);
    if (ret)
    dev_err(&idt821034.spi.dev, "set gpio %d (%u, 0x%x) failed (%d)\n",
    offset, ch, mask, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_chip_gpio_get(c: *mut gpio_chip, offset: c_uint) -> c_int {
    static int idt821034_chip_gpio_get(struct gpio_chip *c, unsigned int offset)
    {
    let mut ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mut mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    struct idt821034 *idt821034 = gpiochip_get_data(c);
    u8 slic_raw;
    int ret;
    scoped_guard(mutex, &idt821034.mutex)
    ret = idt821034_read_slic_raw(idt821034, ch, &slic_raw);
    if (ret) {
    dev_err(&idt821034.spi.dev, "get gpio %d (%u, 0x%x) failed (%d)\n",
    offset, ch, mask, ret);
    return ret;
    }
//
// SLIC IOs are read in reverse order compared to write.
// Reverse the read value here in order to have IO0 at lsb (ie same
// order as write)
//
    return !!(bitrev8(slic_raw) & mask);
    }
#[no_mangle]
unsafe extern "C" fn idt821034_chip_get_direction(c: *mut gpio_chip, offset: c_uint) -> c_int {
    static int idt821034_chip_get_direction(struct gpio_chip *c, unsigned int offset)
    {
    let mut ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mut mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    struct idt821034 *idt821034 = gpiochip_get_data(c);
    u8 slic_dir;
    guard(mutex)(&idt821034.mutex);
    slic_dir = idt821034_get_slic_conf(idt821034, ch);
    return slic_dir & mask ? GPIO_LINE_DIRECTION_IN : GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_chip_direction_input(c: *mut gpio_chip, offset: c_uint) -> c_int {
    static int idt821034_chip_direction_input(struct gpio_chip *c, unsigned int offset)
    {
    let mut ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mut mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    struct idt821034 *idt821034 = gpiochip_get_data(c);
    u8 slic_conf;
    int ret;
// Only IO0 and IO1 can be set as input
    if (mask & ~(IDT821034_SLIC_IO1_IN | IDT821034_SLIC_IO0_IN))
    return -EPERM;
    guard(mutex)(&idt821034.mutex);
    slic_conf = idt821034_get_slic_conf(idt821034, ch) | mask;
    ret = idt821034_set_slic_conf(idt821034, ch, slic_conf);
    if (ret) {
    dev_err(&idt821034.spi.dev, "dir in gpio %d (%u, 0x%x) failed (%d)\n",
    offset, ch, mask, ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_chip_direction_output(c: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int idt821034_chip_direction_output(struct gpio_chip *c, unsigned int offset, int val)
    {
    let mut ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mut mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    struct idt821034 *idt821034 = gpiochip_get_data(c);
    u8 slic_conf;
    int ret;
    ret = idt821034_chip_gpio_set(c, offset, val);
    if (ret)
    return ret;
    guard(mutex)(&idt821034.mutex);
    slic_conf = idt821034_get_slic_conf(idt821034, ch) & ~mask;
    ret = idt821034_set_slic_conf(idt821034, ch, slic_conf);
    if (ret) {
    dev_err(&idt821034.spi.dev, "dir out gpio %d (%u, 0x%x) failed (%d)\n",
    offset, ch, mask, ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_reset_gpio(idt821034: *mut idt821034) -> c_int {
    static int idt821034_reset_gpio(struct idt821034 *idt821034)
    {
    int ret;
    u8 i;
    guard(mutex)(&idt821034.mutex);
// IO0 and IO1 as input for all channels and output IO set to 0
    for (i = 0; i < IDT821034_NB_CHANNEL; i++) {
    ret = idt821034_set_slic_conf(idt821034, i,
    IDT821034_SLIC_IO1_IN | IDT821034_SLIC_IO0_IN);
    if (ret)
    return ret;
    ret = idt821034_write_slic_raw(idt821034, i, 0);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn idt821034_gpio_init(idt821034: *mut idt821034) -> c_int {
    static int idt821034_gpio_init(struct idt821034 *idt821034)
    {
    int ret;
    ret = idt821034_reset_gpio(idt821034);
    if (ret)
    return ret;
    idt821034.gpio_chip.owner = THIS_MODULE;
    idt821034.gpio_chip.label = dev_name(&idt821034.spi.dev);
    idt821034.gpio_chip.parent = &idt821034.spi.dev;
    idt821034.gpio_chip.base = -1;
    idt821034.gpio_chip.ngpio = 5 * 4; /* 5 GPIOs on 4 channels */
    idt821034.gpio_chip.get_direction = idt821034_chip_get_direction;
    idt821034.gpio_chip.direction_input = idt821034_chip_direction_input;
    idt821034.gpio_chip.direction_output = idt821034_chip_direction_output;
    idt821034.gpio_chip.get = idt821034_chip_gpio_get;
    idt821034.gpio_chip.set = idt821034_chip_gpio_set;
    idt821034.gpio_chip.can_sleep = true;
    return devm_gpiochip_add_data(&idt821034.spi.dev, &idt821034.gpio_chip,
    idt821034);
    }
#[no_mangle]
unsafe extern "C" fn idt821034_spi_probe(spi: *mut spi_device) -> c_int {
    static int idt821034_spi_probe(struct spi_device *spi)
    {
    struct idt821034 *idt821034;
    int ret;
    spi.bits_per_word = 8;
    ret = spi_setup(spi);
    if (ret < 0)
    return ret;
    idt821034 = devm_kzalloc(&spi.dev, sizeof(*idt821034), GFP_KERNEL);
    if (!idt821034)
    return -ENOMEM;
    idt821034.spi = spi;
    mutex_init(&idt821034.mutex);
    spi_set_drvdata(spi, idt821034);
    ret = devm_snd_soc_register_component(&spi.dev, &idt821034_component_driver,
    &idt821034_dai_driver, 1);
    if (ret)
    return ret;
    if (IS_ENABLED(CONFIG_GPIOLIB))
    return idt821034_gpio_init(idt821034);
    return 0;
    }
    static const struct of_device_id idt821034_of_match[] = {
    { .compatible = "renesas,idt821034", },
    { }
    };
    MODULE_DEVICE_TABLE(of, idt821034_of_match);
    static const struct spi_device_id idt821034_id_table[] = {
    { "idt821034", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, idt821034_id_table);
    static struct spi_driver idt821034_spi_driver = {
    .driver  = {
    .name   = "idt821034",
    .of_match_table = idt821034_of_match,
    },
    .id_table = idt821034_id_table,
    .probe  = idt821034_spi_probe,
    };
    module_spi_driver(idt821034_spi_driver);
    MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>");
    MODULE_DESCRIPTION("IDT821034 ALSA SoC driver");
    MODULE_LICENSE("GPL");
