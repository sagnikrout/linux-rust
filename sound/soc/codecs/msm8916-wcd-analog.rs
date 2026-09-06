//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/msm8916-wcd-analog.c
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
// Copyright (c) 2016, The Linux Foundation. All rights reserved.

pub const RST_CTL_DIG_SW_RST_N_RESET: c_int = 0;

pub const CONN_TX1_SERIAL_TX1_ADC_1: c_uint = 0x0;
pub const CONN_TX1_SERIAL_TX1_RX_PDM_LB: c_uint = 0x1;
pub const CONN_TX1_SERIAL_TX1_ZERO: c_uint = 0x2;

pub const CONN_TX2_SERIAL_TX2_ADC_2: c_uint = 0x0;
pub const CONN_TX2_SERIAL_TX2_RX_PDM_LB: c_uint = 0x1;
pub const CONN_TX2_SERIAL_TX2_ZERO: c_uint = 0x2;

pub const MICB_1_EN_EXT_BYP_CAP: c_int = 0;

pub const MICB_1_EN_TX3_GND_SEL_TX_GND: c_int = 0;

pub const MICB_MIN_VAL: c_int = 1600;
pub const MICB_STEP_SIZE: c_int = 50;

pub const MICB_1_INT_TX1_INT_RBIAS_EN_DISABLE: c_int = 0;

pub const MICB_1_INT_TX1_INT_PULLUP_EN_TX1N_TO_GND: c_int = 0;

pub const MICB_1_INT_TX2_INT_RBIAS_EN_DISABLE: c_int = 0;

pub const MICB_1_INT_TX2_INT_PULLUP_EN_TX1N_TO_GND: c_int = 0;

pub const MICB_1_INT_TX3_INT_RBIAS_EN_DISABLE: c_int = 0;

pub const MICB_1_INT_TX3_INT_PULLUP_EN_TX1N_TO_GND: c_int = 0;

    CDC_A_MBHC_BTN_VREF_FINE_MASK)

pub const SPKR_DAC_CTL_DAC_RESET_NORMAL: c_int = 0;

pub const SPKR_DRV_CTL_DEF_MASK: c_uint = 0xEF;

    SPKR_DRV_CAL_EN | SPKR_DRV_SETTLE_EN | \
    SPKR_DRV_FW_EN | SPKR_DRV_BOOST_SET | \
    SPKR_DRV_CMFB_SET | SPKR_DRV_GAIN_SET)

pub const SPKR_PWRSTG_CTL_MASK: c_uint = 0xE0;

    SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000)

    SNDRV_PCM_FMTBIT_S32_LE)
    static int btn_mask = SND_JACK_BTN_0 | SND_JACK_BTN_1 |
    SND_JACK_BTN_2 | SND_JACK_BTN_3 | SND_JACK_BTN_4;
    let mut hs_jack_mask: static int = SND_JACK_HEADPHONE | SND_JACK_HEADSET;
    static const char * const supply_names[] = {
    "vdd-cdc-io",
    "vdd-cdc-tx-rx-cx",
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_reg_seq {
    pub seq: *const reg_default,
    pub seq_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8916_wcd_analog_priv {
    pub pmic_rev: u16,
    pub codec_version: u16,
    pub mbhc_btn_enabled: bool,
// special event to detect accessory type
    pub mbhc_btn0_released: c_int,
    pub detect_accessory_type: bool,
    pub mclk: *mut clk,
    pub component: *mut snd_soc_component,
    pub supplies: [regulator_bulk_data; ARRAY_SIZE(supply_names)],
    pub jack: *mut snd_soc_jack,
    pub hphl_jack_type_normally_open: bool,
    pub gnd_jack_type_normally_open: bool,
// Voltage threshold when internal current source of 100uA is used
    pub vref_btn_cs: [u32; MBHC_MAX_BUTTONS],
// Voltage threshold when microphone bias is ON
    pub vref_btn_micb: [u32; MBHC_MAX_BUTTONS],
    pub micbias1_cap_mode: c_uint,
    pub micbias2_cap_mode: c_uint,
    pub micbias_mv: c_uint,
}

    static const char *const adc2_mux_text[] = { "ZERO", "INP2", "INP3" };
    static const char *const rdac2_mux_text[] = { "RX1", "RX2" };
    static const char *const hph_text[] = { "ZERO", "Switch", };
    static const struct soc_enum hph_enum = SOC_ENUM_SINGLE_VIRT(
    ARRAY_SIZE(hph_text), hph_text);
    let mut ear_mux: static struct snd_kcontrol_new = SOC_DAPM_ENUM("EAR_S", hph_enum);
    let mut hphl_mux: static struct snd_kcontrol_new = SOC_DAPM_ENUM("HPHL", hph_enum);
    let mut hphr_mux: static struct snd_kcontrol_new = SOC_DAPM_ENUM("HPHR", hph_enum);
// ADC2 MUX
    static const struct soc_enum adc2_enum = SOC_ENUM_SINGLE_VIRT(
    ARRAY_SIZE(adc2_mux_text), adc2_mux_text);
// RDAC2 MUX
    static const struct soc_enum rdac2_mux_enum = SOC_ENUM_SINGLE(
    CDC_D_CDC_CONN_HPHR_DAC_CTL, 0, 2, rdac2_mux_text);
    static const struct snd_kcontrol_new spkr_switch[] = {
    SOC_DAPM_SINGLE("Switch", CDC_A_SPKR_DAC_CTL, 7, 1, 0)
    };
    static const struct snd_kcontrol_new rdac2_mux = SOC_DAPM_ENUM(
    "RDAC2 MUX Mux", rdac2_mux_enum);
    static const struct snd_kcontrol_new tx_adc2_mux = SOC_DAPM_ENUM(
    "ADC2 MUX Mux", adc2_enum);
// Analog Gain control 0 dB to +24 dB in 6 dB steps
    static const DECLARE_TLV_DB_SCALE(analog_gain, 0, 600, 0);
    static const struct snd_kcontrol_new pm8916_wcd_analog_snd_controls[] = {
    SOC_SINGLE_TLV("ADC1 Volume", CDC_A_TX_1_EN, 3, 8, 0, analog_gain),
    SOC_SINGLE_TLV("ADC2 Volume", CDC_A_TX_2_EN, 3, 8, 0, analog_gain),
    SOC_SINGLE_TLV("ADC3 Volume", CDC_A_TX_3_EN, 3, 8, 0, analog_gain),
    };
#[no_mangle]
unsafe extern "C" fn pm8916_wcd_analog_micbias_enable(component: *mut snd_soc_component) {
    static void pm8916_wcd_analog_micbias_enable(struct snd_soc_component *component)
    {
    struct pm8916_wcd_analog_priv *wcd = snd_soc_component_get_drvdata(component);
    snd_soc_component_update_bits(component, CDC_A_MICB_1_CTL,
    MICB_1_CTL_EXT_PRECHARG_EN_MASK |
    MICB_1_CTL_INT_PRECHARG_BYP_MASK,
    MICB_1_CTL_INT_PRECHARG_BYP_EXT_PRECHRG_SEL
    | MICB_1_CTL_EXT_PRECHARG_EN_ENABLE);
    if (wcd.micbias_mv) {
    snd_soc_component_update_bits(component, CDC_A_MICB_1_VAL,
    MICB_1_VAL_MICB_OUT_VAL_MASK,
    MICB_VOLTAGE_REGVAL(wcd.micbias_mv));
//
// Special headset needs MICBIAS as 2.7V so wait for
// 50 msec for the MICBIAS to reach 2.7 volts.
//
    if (wcd.micbias_mv >= 2700)
    msleep(50);
    }
    snd_soc_component_update_bits(component, CDC_A_MICB_1_CTL,
    MICB_1_CTL_EXT_PRECHARG_EN_MASK |
    MICB_1_CTL_INT_PRECHARG_BYP_MASK, 0);
    }
    static int pm8916_wcd_analog_enable_micbias(struct snd_soc_component *component,
    int event, unsigned int cap_mode)
    {
    switch (event) {
    case SND_SOC_DAPM_POST_PMU:
    pm8916_wcd_analog_micbias_enable(component);
    snd_soc_component_update_bits(component, CDC_A_MICB_1_EN,
    MICB_1_EN_BYP_CAP_MASK, cap_mode);
    break;
    }
    return 0;
    }
    static int pm8916_wcd_analog_enable_micbias_int(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    snd_soc_component_update_bits(component, CDC_A_MICB_1_EN,
    MICB_1_EN_OPA_STG2_TAIL_CURR_MASK,
    MICB_1_EN_OPA_STG2_TAIL_CURR_1_60UA);
    break;
    }
    return 0;
    }
    static int pm8916_wcd_analog_enable_micbias1(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct pm8916_wcd_analog_priv *wcd = snd_soc_component_get_drvdata(component);
    return pm8916_wcd_analog_enable_micbias(component, event,
    wcd.micbias1_cap_mode);
    }
    static int pm8916_wcd_analog_enable_micbias2(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct pm8916_wcd_analog_priv *wcd = snd_soc_component_get_drvdata(component);
    return pm8916_wcd_analog_enable_micbias(component, event,
    wcd.micbias2_cap_mode);
    }
    static int pm8916_mbhc_configure_bias(struct pm8916_wcd_analog_priv *priv,
    bool micbias2_enabled)
    {
    struct snd_soc_component *component = priv.component;
    u32 coarse, fine, reg_val, reg_addr;
    int *vrefs, i;
    if (!micbias2_enabled) { /* use internal 100uA Current source */
// Enable internal 2.2k Internal Rbias Resistor
    snd_soc_component_update_bits(component, CDC_A_MICB_1_INT_RBIAS,
    MICB_1_INT_TX2_INT_RBIAS_EN_MASK,
    MICB_1_INT_TX2_INT_RBIAS_EN_ENABLE);
// Remove pull down on MIC BIAS2
    snd_soc_component_update_bits(component, CDC_A_MICB_2_EN,
    CDC_A_MICB_2_PULL_DOWN_EN_MASK,
    0);
// enable 100uA internal current source
    snd_soc_component_update_bits(component, CDC_A_MBHC_FSM_CTL,
    CDC_A_MBHC_FSM_CTL_BTN_ISRC_CTRL_MASK,
    CDC_A_MBHC_FSM_CTL_BTN_ISRC_CTRL_I_100UA);
    }
    snd_soc_component_update_bits(component, CDC_A_MBHC_FSM_CTL,
    CDC_A_MBHC_FSM_CTL_MBHC_FSM_EN_MASK,
    CDC_A_MBHC_FSM_CTL_MBHC_FSM_EN);
    if (micbias2_enabled)
    vrefs = &priv.vref_btn_micb[0];
    else
    vrefs = &priv.vref_btn_cs[0];
// program vref ranges for all the buttons
    reg_addr = CDC_A_MBHC_BTN0_ZDET_CTL_0;
    for (i = 0; i <  MBHC_MAX_BUTTONS; i++) {
// split mv in to coarse parts of 100mv & fine parts of 12mv
    coarse = (vrefs[i] / 100);
    fine = ((vrefs[i] % 100) / 12);
    reg_val = (coarse << CDC_A_MBHC_BTN_VREF_COARSE_SHIFT) |
    (fine << CDC_A_MBHC_BTN_VREF_FINE_SHIFT);
    snd_soc_component_update_bits(component, reg_addr,
    CDC_A_MBHC_BTN_VREF_MASK,
    reg_val);
    reg_addr++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8916_wcd_setup_mbhc(wcd: *mut pm8916_wcd_analog_priv) {
    static void pm8916_wcd_setup_mbhc(struct pm8916_wcd_analog_priv *wcd)
    {
    struct snd_soc_component *component = wcd.component;
    let mut micbias_enabled: bool = false;
    let mut plug_type: u32 = 0;
    u32 int_en_mask;
    snd_soc_component_write(component, CDC_A_MBHC_DET_CTL_1,
    CDC_A_MBHC_DET_CTL_L_DET_EN |
    CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_INSERTION |
    CDC_A_MBHC_DET_CTL_MIC_CLAMP_CTL_AUTO |
    CDC_A_MBHC_DET_CTL_MBHC_BIAS_EN);
    if (wcd.hphl_jack_type_normally_open)
    plug_type |= CDC_A_HPHL_PLUG_TYPE_NO;
    if (wcd.gnd_jack_type_normally_open)
    plug_type |= CDC_A_GND_PLUG_TYPE_NO;
    snd_soc_component_write(component, CDC_A_MBHC_DET_CTL_2,
    CDC_A_MBHC_DET_CTL_HS_L_DET_PULL_UP_CTRL_I_3P0 |
    CDC_A_MBHC_DET_CTL_HS_L_DET_COMPA_CTRL_V0P9_VDD |
    plug_type |
    CDC_A_MBHC_DET_CTL_HPHL_100K_TO_GND_EN);
    snd_soc_component_write(component, CDC_A_MBHC_DBNC_TIMER,
    CDC_A_MBHC_DBNC_TIMER_INSREM_DBNC_T_256_MS |
    CDC_A_MBHC_DBNC_TIMER_BTN_DBNC_T_16MS);
// enable MBHC clock
    snd_soc_component_update_bits(component, CDC_D_CDC_DIG_CLK_CTL,
    DIG_CLK_CTL_D_MBHC_CLK_EN_MASK,
    DIG_CLK_CTL_D_MBHC_CLK_EN);
    if (snd_soc_component_read(component, CDC_A_MICB_2_EN) & CDC_A_MICB_2_EN_ENABLE)
    micbias_enabled = true;
    pm8916_mbhc_configure_bias(wcd, micbias_enabled);
    int_en_mask = MBHC_SWITCH_INT;
    if (wcd.mbhc_btn_enabled)
    int_en_mask |= MBHC_BUTTON_PRESS_DET | MBHC_BUTTON_RELEASE_DET;
    snd_soc_component_update_bits(component, CDC_D_INT_EN_CLR, int_en_mask, 0);
    snd_soc_component_update_bits(component, CDC_D_INT_EN_SET, int_en_mask, int_en_mask);
    wcd.mbhc_btn0_released = false;
    wcd.detect_accessory_type = true;
    }
    static int pm8916_wcd_analog_enable_micbias_int2(struct
    snd_soc_dapm_widget
// w, struct snd_kcontrol
// kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct pm8916_wcd_analog_priv *wcd = snd_soc_component_get_drvdata(component);
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    snd_soc_component_update_bits(component, CDC_A_MICB_2_EN,
    CDC_A_MICB_2_PULL_DOWN_EN_MASK, 0);
    break;
    case SND_SOC_DAPM_POST_PMU:
    pm8916_mbhc_configure_bias(wcd, true);
    break;
    case SND_SOC_DAPM_POST_PMD:
    pm8916_mbhc_configure_bias(wcd, false);
    break;
    }
    return pm8916_wcd_analog_enable_micbias_int(w, kcontrol, event);
    }
    static int pm8916_wcd_analog_enable_adc(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    let mut adc_reg: u16 = CDC_A_TX_1_2_TEST_CTL_2;
    u8 init_bit_shift;
    if (w.reg == CDC_A_TX_1_EN)
    init_bit_shift = 5;
    else
    init_bit_shift = 4;
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    if (w.reg == CDC_A_TX_2_EN)
    snd_soc_component_update_bits(component, CDC_A_MICB_1_CTL,
    MICB_1_CTL_CFILT_REF_SEL_MASK,
    MICB_1_CTL_CFILT_REF_SEL_HPF_REF);
//
// Add delay of 10 ms to give sufficient time for the voltage
// to shoot up and settle so that the txfe init does not
// happen when the input voltage is changing too much.
//
    usleep_range(10000, 10010);
    snd_soc_component_update_bits(component, adc_reg, 1 << init_bit_shift,
    1 << init_bit_shift);
    switch (w.reg) {
    case CDC_A_TX_1_EN:
    snd_soc_component_update_bits(component, CDC_D_CDC_CONN_TX1_CTL,
    CONN_TX1_SERIAL_TX1_MUX,
    CONN_TX1_SERIAL_TX1_ADC_1);
    break;
    case CDC_A_TX_2_EN:
    case CDC_A_TX_3_EN:
    snd_soc_component_update_bits(component, CDC_D_CDC_CONN_TX2_CTL,
    CONN_TX2_SERIAL_TX2_MUX,
    CONN_TX2_SERIAL_TX2_ADC_2);
    break;
    }
    break;
    case SND_SOC_DAPM_POST_PMU:
//
// Add delay of 12 ms before deasserting the init
// to reduce the tx pop
//
    usleep_range(12000, 12010);
    snd_soc_component_update_bits(component, adc_reg, 1 << init_bit_shift, 0x00);
    break;
    case SND_SOC_DAPM_POST_PMD:
    switch (w.reg) {
    case CDC_A_TX_1_EN:
    snd_soc_component_update_bits(component, CDC_D_CDC_CONN_TX1_CTL,
    CONN_TX1_SERIAL_TX1_MUX,
    CONN_TX1_SERIAL_TX1_ZERO);
    break;
    case CDC_A_TX_2_EN:
    snd_soc_component_update_bits(component, CDC_A_MICB_1_CTL,
    MICB_1_CTL_CFILT_REF_SEL_MASK, 0);
    fallthrough;
    case CDC_A_TX_3_EN:
    snd_soc_component_update_bits(component, CDC_D_CDC_CONN_TX2_CTL,
    CONN_TX2_SERIAL_TX2_MUX,
    CONN_TX2_SERIAL_TX2_ZERO);
    break;
    }
    break;
    }
    return 0;
    }
    static int pm8916_wcd_analog_enable_spk_pa(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    snd_soc_component_update_bits(component, CDC_A_SPKR_PWRSTG_CTL,
    SPKR_PWRSTG_CTL_DAC_EN_MASK |
    SPKR_PWRSTG_CTL_BBM_MASK |
    SPKR_PWRSTG_CTL_HBRDGE_EN_MASK |
    SPKR_PWRSTG_CTL_CLAMP_EN_MASK,
    SPKR_PWRSTG_CTL_DAC_EN|
    SPKR_PWRSTG_CTL_BBM_EN |
    SPKR_PWRSTG_CTL_HBRDGE_EN |
    SPKR_PWRSTG_CTL_CLAMP_EN);
    snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL,
    RX_EAR_CTL_SPK_VBAT_LDO_EN_MASK,
    RX_EAR_CTL_SPK_VBAT_LDO_EN_ENABLE);
    break;
    case SND_SOC_DAPM_POST_PMU:
    snd_soc_component_update_bits(component, CDC_A_SPKR_DRV_CTL,
    SPKR_DRV_CTL_DEF_MASK,
    SPKR_DRV_CTL_DEF_VAL);
    snd_soc_component_update_bits(component, w.reg,
    SPKR_DRV_CLASSD_PA_EN_MASK,
    SPKR_DRV_CLASSD_PA_EN_ENABLE);
    break;
    case SND_SOC_DAPM_POST_PMD:
    snd_soc_component_update_bits(component, CDC_A_SPKR_PWRSTG_CTL,
    SPKR_PWRSTG_CTL_DAC_EN_MASK|
    SPKR_PWRSTG_CTL_BBM_MASK |
    SPKR_PWRSTG_CTL_HBRDGE_EN_MASK |
    SPKR_PWRSTG_CTL_CLAMP_EN_MASK, 0);
    snd_soc_component_update_bits(component, CDC_A_SPKR_DAC_CTL,
    SPKR_DAC_CTL_DAC_RESET_MASK,
    SPKR_DAC_CTL_DAC_RESET_NORMAL);
    snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL,
    RX_EAR_CTL_SPK_VBAT_LDO_EN_MASK, 0);
    break;
    }
    return 0;
    }
    static int pm8916_wcd_analog_enable_ear_pa(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL,
    RX_EAR_CTL_PA_SEL_MASK, RX_EAR_CTL_PA_SEL);
    break;
    case SND_SOC_DAPM_POST_PMU:
    snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL,
    RX_EAR_CTL_PA_EAR_PA_EN_MASK,
    RX_EAR_CTL_PA_EAR_PA_EN_ENABLE);
    break;
    case SND_SOC_DAPM_POST_PMD:
    snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL,
    RX_EAR_CTL_PA_EAR_PA_EN_MASK, 0);
// Delay to reduce ear turn off pop
    usleep_range(7000, 7100);
    snd_soc_component_update_bits(component, CDC_A_RX_EAR_CTL,
    RX_EAR_CTL_PA_SEL_MASK, 0);
    break;
    }
    return 0;
    }
    static int pm8916_wcd_analog_enable_hphl_pa(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct pm8916_wcd_analog_priv *priv = dev_get_drvdata(component.dev);
// This quirk is not required for revisions prior to CAJON_2_0
    if (priv.codec_version < 4)
    return 0;
    switch (event) {
    case SND_SOC_DAPM_POST_PMU:
    usleep_range(7000, 7100);
    snd_soc_component_update_bits(component, CDC_A_RX_HPH_L_TEST,
    0x04, 0x04);
    break;
    case SND_SOC_DAPM_POST_PMD:
// wait 20 ms after the digital codec has powered down
    msleep(20);
    snd_soc_component_update_bits(component, CDC_A_RX_HPH_L_TEST,
    0x04, 0x00);
    break;
    }
    return 0;
    }
    static int pm8916_wcd_analog_enable_hphr_pa(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct pm8916_wcd_analog_priv *priv = dev_get_drvdata(component.dev);
// This quirk is not required for revisions prior to CAJON_2_0
    if (priv.codec_version < 4)
    return 0;
    switch (event) {
    case SND_SOC_DAPM_POST_PMU:
    usleep_range(7000, 7100);
    snd_soc_component_update_bits(component, CDC_A_RX_HPH_R_TEST,
    0x04, 0x04);
    break;
    case SND_SOC_DAPM_POST_PMD:
    msleep(20);
    snd_soc_component_update_bits(component, CDC_A_RX_HPH_R_TEST,
    0x04, 0x00);
    break;
    }
    return 0;
    }
    static const struct reg_default wcd_reg_defaults_2_0[] = {
    {CDC_A_RX_COM_OCP_CTL, 0xD1},
    {CDC_A_RX_COM_OCP_COUNT, 0xFF},
    {CDC_D_SEC_ACCESS, 0xA5},
    {CDC_D_PERPH_RESET_CTL3, 0x0F},
    {CDC_A_TX_1_2_OPAMP_BIAS, 0x4F},
    {CDC_A_NCP_FBCTRL, 0x28},
    {CDC_A_SPKR_DRV_CTL, 0x69},
    {CDC_A_SPKR_DRV_DBG, 0x01},
    {CDC_A_BOOST_EN_CTL, 0x5F},
    {CDC_A_SLOPE_COMP_IP_ZERO, 0x88},
    {CDC_A_SEC_ACCESS, 0xA5},
    {CDC_A_PERPH_RESET_CTL3, 0x0F},
    {CDC_A_CURRENT_LIMIT, 0x82},
    {CDC_A_SPKR_DAC_CTL, 0x03},
    {CDC_A_SPKR_OCP_CTL, 0xE1},
    {CDC_A_MASTER_BIAS_CTL, 0x30},
    };
    static const struct wcd_reg_seq pm8916_data = {
    .seq = wcd_reg_defaults_2_0,
    .seq_size = ARRAY_SIZE(wcd_reg_defaults_2_0),
    };
    static const struct reg_default wcd_reg_defaults_pm8950[] = {
    {CDC_A_RX_COM_OCP_CTL, 0xd1},
    {CDC_A_RX_COM_OCP_COUNT, 0xff},
    {CDC_D_SEC_ACCESS, 0xa5},
    {CDC_D_PERPH_RESET_CTL3, 0x0f},
    {CDC_A_TX_1_2_OPAMP_BIAS, 0x4c},
    {CDC_A_NCP_FBCTRL, 0xa8},
    {CDC_A_NCP_VCTRL, 0xa4},
    {CDC_A_SPKR_DRV_CTL, 0x69},
    {CDC_A_SPKR_DRV_DBG, 0x01},
    {CDC_A_SEC_ACCESS, 0xa5},
    {CDC_A_PERPH_RESET_CTL3, 0x0f},
    {CDC_A_CURRENT_LIMIT, 0x82},
    {CDC_A_SPKR_ANA_BIAS_SET, 0x41},
    {CDC_A_SPKR_DAC_CTL, 0x03},
    {CDC_A_SPKR_OCP_CTL, 0xe1},
    {CDC_A_RX_HPH_BIAS_PA, 0xfa},
    {CDC_A_MASTER_BIAS_CTL, 0x30},
    {CDC_A_MICB_1_INT_RBIAS, 0x00},
    };
    static const struct wcd_reg_seq pm8950_data = {
    .seq = wcd_reg_defaults_pm8950,
    .seq_size = ARRAY_SIZE(wcd_reg_defaults_pm8950),
    };
    static const struct reg_default wcd_reg_defaults_pm8953[] = {
    {CDC_A_RX_COM_OCP_CTL, 0xd1},
    {CDC_A_RX_COM_OCP_COUNT, 0xff},
    {CDC_D_SEC_ACCESS, 0xa5},
    {CDC_D_PERPH_RESET_CTL3, 0x0f},
    {CDC_A_TX_1_2_OPAMP_BIAS, 0x4c},
    {CDC_A_NCP_FBCTRL, 0xa8},
    {CDC_A_NCP_VCTRL, 0xa4},
    {CDC_A_SPKR_DRV_CTL, 0x69},
    {CDC_A_SPKR_DRV_DBG, 0x01},
    {CDC_A_SEC_ACCESS, 0xa5},
    {CDC_A_PERPH_RESET_CTL3, 0x0f},
    {CDC_A_CURRENT_LIMIT, 0xa2},
    {CDC_A_BYPASS_MODE, 0x18},
    {CDC_A_SPKR_ANA_BIAS_SET, 0x41},
    {CDC_A_SPKR_DAC_CTL, 0x03},
    {CDC_A_SPKR_OCP_CTL, 0xe1},
    {CDC_A_RX_HPH_BIAS_PA, 0xfa},
    {CDC_A_RX_EAR_STATUS, 0x10},
    {CDC_A_MASTER_BIAS_CTL, 0x30},
    {CDC_A_MICB_1_INT_RBIAS, 0x00},
    };
    static const struct wcd_reg_seq pm8953_data = {
    .seq = wcd_reg_defaults_pm8953,
    .seq_size = ARRAY_SIZE(wcd_reg_defaults_pm8953),
    };
#[no_mangle]
unsafe extern "C" fn pm8916_wcd_analog_probe(component: *mut snd_soc_component) -> c_int {
    static int pm8916_wcd_analog_probe(struct snd_soc_component *component)
    {
    struct pm8916_wcd_analog_priv *priv = dev_get_drvdata(component.dev);
    const struct wcd_reg_seq *wcd_reg_init_data;
    int err, reg;
    err = regulator_bulk_enable(ARRAY_SIZE(priv.supplies), priv.supplies);
    if (err != 0) {
    dev_err(component.dev, "failed to enable regulators (%d)\n", err);
    return err;
    }
    snd_soc_component_init_regmap(component,
    dev_get_regmap(component.dev.parent, core::ptr::null_mut()));
    snd_soc_component_set_drvdata(component, priv);
    priv.pmic_rev = snd_soc_component_read(component, CDC_D_REVISION1);
    priv.codec_version = snd_soc_component_read(component, CDC_D_PERPH_SUBTYPE);
    dev_info(component.dev, "PMIC REV: %d\t CODEC Version: %d\n",
    priv.pmic_rev, priv.codec_version);
    snd_soc_component_write(component, CDC_D_PERPH_RESET_CTL4, 0x01);
    snd_soc_component_write(component, CDC_A_PERPH_RESET_CTL4, 0x01);
    wcd_reg_init_data = of_device_get_match_data(component.dev);
    for (reg = 0; reg < wcd_reg_init_data.seq_size; reg++)
    snd_soc_component_write(component, wcd_reg_init_data.seq[reg].reg,
    wcd_reg_init_data.seq[reg].def);
    priv.component = component;
    snd_soc_component_update_bits(component, CDC_D_CDC_RST_CTL,
    RST_CTL_DIG_SW_RST_N_MASK,
    RST_CTL_DIG_SW_RST_N_REMOVE_RESET);
    pm8916_wcd_setup_mbhc(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8916_wcd_analog_remove(component: *mut snd_soc_component) {
    static void pm8916_wcd_analog_remove(struct snd_soc_component *component)
    {
    struct pm8916_wcd_analog_priv *priv = dev_get_drvdata(component.dev);
    snd_soc_component_update_bits(component, CDC_D_CDC_RST_CTL,
    RST_CTL_DIG_SW_RST_N_MASK, 0);
    regulator_bulk_disable(ARRAY_SIZE(priv.supplies),
    priv.supplies);
    }
    static const struct snd_soc_dapm_route pm8916_wcd_analog_audio_map[] = {
    {"PDM_RX1", core::ptr::null_mut(), "PDM Playback"},
    {"PDM_RX2", core::ptr::null_mut(), "PDM Playback"},
    {"PDM_RX3", core::ptr::null_mut(), "PDM Playback"},
    {"PDM Capture", core::ptr::null_mut(), "PDM_TX"},
// ADC Connections
    {"PDM_TX", core::ptr::null_mut(), "ADC2"},
    {"PDM_TX", core::ptr::null_mut(), "ADC3"},
    {"ADC2", core::ptr::null_mut(), "ADC2 MUX"},
    {"ADC3", core::ptr::null_mut(), "ADC2 MUX"},
    {"ADC2 MUX", "INP2", "ADC2_INP2"},
    {"ADC2 MUX", "INP3", "ADC2_INP3"},
    {"PDM_TX", core::ptr::null_mut(), "ADC1"},
    {"ADC1", core::ptr::null_mut(), "AMIC1"},
    {"ADC2_INP2", core::ptr::null_mut(), "AMIC2"},
    {"ADC2_INP3", core::ptr::null_mut(), "AMIC3"},
// RDAC Connections
    {"HPHR DAC", core::ptr::null_mut(), "RDAC2 MUX"},
    {"RDAC2 MUX", "RX1", "PDM_RX1"},
    {"RDAC2 MUX", "RX2", "PDM_RX2"},
    {"HPHL DAC", core::ptr::null_mut(), "PDM_RX1"},
    {"PDM_RX1", core::ptr::null_mut(), "RXD1_CLK"},
    {"PDM_RX2", core::ptr::null_mut(), "RXD2_CLK"},
    {"PDM_RX3", core::ptr::null_mut(), "RXD3_CLK"},
    {"PDM_RX1", core::ptr::null_mut(), "RXD_PDM_CLK"},
    {"PDM_RX2", core::ptr::null_mut(), "RXD_PDM_CLK"},
    {"PDM_RX3", core::ptr::null_mut(), "RXD_PDM_CLK"},
    {"ADC1", core::ptr::null_mut(), "TXD_CLK"},
    {"ADC2", core::ptr::null_mut(), "TXD_CLK"},
    {"ADC3", core::ptr::null_mut(), "TXD_CLK"},
    {"ADC1", core::ptr::null_mut(), "TXA_CLK25"},
    {"ADC2", core::ptr::null_mut(), "TXA_CLK25"},
    {"ADC3", core::ptr::null_mut(), "TXA_CLK25"},
    {"PDM_RX1", core::ptr::null_mut(), "A_MCLK2"},
    {"PDM_RX2", core::ptr::null_mut(), "A_MCLK2"},
    {"PDM_RX3", core::ptr::null_mut(), "A_MCLK2"},
    {"PDM_TX", core::ptr::null_mut(), "A_MCLK2"},
    {"A_MCLK2", core::ptr::null_mut(), "A_MCLK"},
// Earpiece (RX MIX1)
    {"EAR", core::ptr::null_mut(), "EAR_S"},
    {"EAR_S", "Switch", "EAR PA"},
    {"EAR PA", core::ptr::null_mut(), "RX_BIAS"},
    {"EAR PA", core::ptr::null_mut(), "HPHL DAC"},
    {"EAR PA", core::ptr::null_mut(), "HPHR DAC"},
    {"EAR PA", core::ptr::null_mut(), "EAR CP"},
// Headset (RX MIX1 and RX MIX2)
    {"HPH_L", core::ptr::null_mut(), "HPHL PA"},
    {"HPH_R", core::ptr::null_mut(), "HPHR PA"},
    {"HPHL DAC", core::ptr::null_mut(), "EAR_HPHL_CLK"},
    {"HPHR DAC", core::ptr::null_mut(), "EAR_HPHR_CLK"},
    {"CP", core::ptr::null_mut(), "NCP_CLK"},
    {"HPHL PA", core::ptr::null_mut(), "HPHL"},
    {"HPHR PA", core::ptr::null_mut(), "HPHR"},
    {"HPHL PA", core::ptr::null_mut(), "CP"},
    {"HPHL PA", core::ptr::null_mut(), "RX_BIAS"},
    {"HPHR PA", core::ptr::null_mut(), "CP"},
    {"HPHR PA", core::ptr::null_mut(), "RX_BIAS"},
    {"HPHL", "Switch", "HPHL DAC"},
    {"HPHR", "Switch", "HPHR DAC"},
    {"RX_BIAS", core::ptr::null_mut(), "DAC_REF"},
    {"SPK_OUT", core::ptr::null_mut(), "SPK PA"},
    {"SPK PA", core::ptr::null_mut(), "RX_BIAS"},
    {"SPK PA", core::ptr::null_mut(), "SPKR_CLK"},
    {"SPK PA", core::ptr::null_mut(), "SPK DAC"},
    {"SPK DAC", "Switch", "PDM_RX3"},
    {"MIC_BIAS1", core::ptr::null_mut(), "INT_LDO_H"},
    {"MIC_BIAS2", core::ptr::null_mut(), "INT_LDO_H"},
    {"MIC_BIAS1", core::ptr::null_mut(), "vdd-micbias"},
    {"MIC_BIAS2", core::ptr::null_mut(), "vdd-micbias"},
    {"MIC BIAS External1", core::ptr::null_mut(), "MIC_BIAS1"},
    {"MIC BIAS Internal1", core::ptr::null_mut(), "MIC_BIAS1"},
    {"MIC BIAS External2", core::ptr::null_mut(), "MIC_BIAS2"},
    {"MIC BIAS Internal2", core::ptr::null_mut(), "MIC_BIAS2"},
    {"MIC BIAS Internal3", core::ptr::null_mut(), "MIC_BIAS1"},
    };
    static const struct snd_soc_dapm_widget pm8916_wcd_analog_dapm_widgets[] = {
    SND_SOC_DAPM_AIF_IN("PDM_RX1", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("PDM_RX2", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("PDM_RX3", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT("PDM_TX", core::ptr::null_mut(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_INPUT("AMIC1"),
    SND_SOC_DAPM_INPUT("AMIC3"),
    SND_SOC_DAPM_INPUT("AMIC2"),
    SND_SOC_DAPM_OUTPUT("EAR"),
    SND_SOC_DAPM_OUTPUT("HPH_L"),
    SND_SOC_DAPM_OUTPUT("HPH_R"),
// RX stuff
    SND_SOC_DAPM_SUPPLY("INT_LDO_H", SND_SOC_NOPM, 1, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA_E("EAR PA", SND_SOC_NOPM,
    0, 0, core::ptr::null_mut(), 0,
    pm8916_wcd_analog_enable_ear_pa,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX("EAR_S", SND_SOC_NOPM, 0, 0, &ear_mux),
    SND_SOC_DAPM_SUPPLY("EAR CP", CDC_A_NCP_EN, 4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_PGA_E("HPHL PA", CDC_A_RX_HPH_CNP_EN, 5, 0, core::ptr::null_mut(), 0,
    pm8916_wcd_analog_enable_hphl_pa,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX("HPHL", SND_SOC_NOPM, 0, 0, &hphl_mux),
    SND_SOC_DAPM_MIXER("HPHL DAC", CDC_A_RX_HPH_L_PA_DAC_CTL, 3, 0, core::ptr::null_mut(),
    0),
    SND_SOC_DAPM_PGA_E("HPHR PA", CDC_A_RX_HPH_CNP_EN, 4, 0, core::ptr::null_mut(), 0,
    pm8916_wcd_analog_enable_hphr_pa,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX("HPHR", SND_SOC_NOPM, 0, 0, &hphr_mux),
    SND_SOC_DAPM_MIXER("HPHR DAC", CDC_A_RX_HPH_R_PA_DAC_CTL, 3, 0, core::ptr::null_mut(),
    0),
    SND_SOC_DAPM_MIXER("SPK DAC", SND_SOC_NOPM, 0, 0,
    spkr_switch, ARRAY_SIZE(spkr_switch)),
// Speaker
    SND_SOC_DAPM_OUTPUT("SPK_OUT"),
    SND_SOC_DAPM_PGA_E("SPK PA", CDC_A_SPKR_DRV_CTL,
    6, 0, core::ptr::null_mut(), 0,
    pm8916_wcd_analog_enable_spk_pa,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_REGULATOR_SUPPLY("vdd-micbias", 0, 0),
    SND_SOC_DAPM_SUPPLY("CP", CDC_A_NCP_EN, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("DAC_REF", CDC_A_RX_COM_BIAS_DAC, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("RX_BIAS", CDC_A_RX_COM_BIAS_DAC, 7, 0, core::ptr::null_mut(), 0),
// TX
    SND_SOC_DAPM_SUPPLY("MIC_BIAS1", CDC_A_MICB_1_EN, 7, 0,
    pm8916_wcd_analog_enable_micbias1,
    SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_SUPPLY("MIC_BIAS2", CDC_A_MICB_2_EN, 7, 0,
    pm8916_wcd_analog_enable_micbias2,
    SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_SUPPLY("MIC BIAS External1", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("MIC BIAS External2", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("MIC BIAS Internal1", CDC_A_MICB_1_INT_RBIAS, 7, 0,
    pm8916_wcd_analog_enable_micbias_int,
    SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_SUPPLY("MIC BIAS Internal2", CDC_A_MICB_1_INT_RBIAS, 4, 0,
    pm8916_wcd_analog_enable_micbias_int2,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY("MIC BIAS Internal3", CDC_A_MICB_1_INT_RBIAS, 1, 0,
    pm8916_wcd_analog_enable_micbias_int,
    SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_ADC_E("ADC1", core::ptr::null_mut(), CDC_A_TX_1_EN, 7, 0,
    pm8916_wcd_analog_enable_adc,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_ADC_E("ADC2_INP2", core::ptr::null_mut(), CDC_A_TX_2_EN, 7, 0,
    pm8916_wcd_analog_enable_adc,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_ADC_E("ADC2_INP3", core::ptr::null_mut(), CDC_A_TX_3_EN, 7, 0,
    pm8916_wcd_analog_enable_adc,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER("ADC2", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("ADC3", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MUX("ADC2 MUX", SND_SOC_NOPM, 0, 0, &tx_adc2_mux),
    SND_SOC_DAPM_MUX("RDAC2 MUX", SND_SOC_NOPM, 0, 0, &rdac2_mux),
// Analog path clocks
    SND_SOC_DAPM_SUPPLY("EAR_HPHR_CLK", CDC_D_CDC_ANA_CLK_CTL, 0, 0, core::ptr::null_mut(),
    0),
    SND_SOC_DAPM_SUPPLY("EAR_HPHL_CLK", CDC_D_CDC_ANA_CLK_CTL, 1, 0, core::ptr::null_mut(),
    0),
    SND_SOC_DAPM_SUPPLY("SPKR_CLK", CDC_D_CDC_ANA_CLK_CTL, 4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("TXA_CLK25", CDC_D_CDC_ANA_CLK_CTL, 5, 0, core::ptr::null_mut(), 0),
// Digital path clocks
    SND_SOC_DAPM_SUPPLY("RXD1_CLK", CDC_D_CDC_DIG_CLK_CTL, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("RXD2_CLK", CDC_D_CDC_DIG_CLK_CTL, 1, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("RXD3_CLK", CDC_D_CDC_DIG_CLK_CTL, 2, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("TXD_CLK", CDC_D_CDC_DIG_CLK_CTL, 4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("NCP_CLK", CDC_D_CDC_DIG_CLK_CTL, 6, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_SUPPLY("RXD_PDM_CLK", CDC_D_CDC_DIG_CLK_CTL, 7, 0, core::ptr::null_mut(),
    0),
// System Clock source
    SND_SOC_DAPM_SUPPLY("A_MCLK", CDC_D_CDC_TOP_CLK_CTL, 2, 0, core::ptr::null_mut(), 0),
// TX ADC and RX DAC Clock source.
    SND_SOC_DAPM_SUPPLY("A_MCLK2", CDC_D_CDC_TOP_CLK_CTL, 3, 0, core::ptr::null_mut(), 0),
    };
    static int pm8916_wcd_analog_set_jack(struct snd_soc_component *component,
    struct snd_soc_jack *jack,
    void *data)
    {
    struct pm8916_wcd_analog_priv *wcd = snd_soc_component_get_drvdata(component);
    wcd.jack = jack;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mbhc_btn_release_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t mbhc_btn_release_irq_handler(int irq, void *arg)
    {
    struct pm8916_wcd_analog_priv *priv = arg;
    if (priv.detect_accessory_type) {
    struct snd_soc_component *component = priv.component;
    let mut val: u32 = snd_soc_component_read(component, CDC_A_MBHC_RESULT_1);
// check if its BTN0 thats released
    if ((val != -1) && !(val & CDC_A_MBHC_RESULT_1_BTN_RESULT_MASK))
    priv.mbhc_btn0_released = true;
    } else {
    snd_soc_jack_report(priv.jack, 0, btn_mask);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mbhc_btn_press_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t mbhc_btn_press_irq_handler(int irq, void *arg)
    {
    struct pm8916_wcd_analog_priv *priv = arg;
    struct snd_soc_component *component = priv.component;
    u32 btn_result;
    btn_result = snd_soc_component_read(component, CDC_A_MBHC_RESULT_1) &
    CDC_A_MBHC_RESULT_1_BTN_RESULT_MASK;
    switch (btn_result) {
    case 0xf:
    snd_soc_jack_report(priv.jack, SND_JACK_BTN_4, btn_mask);
    break;
    case 0x7:
    snd_soc_jack_report(priv.jack, SND_JACK_BTN_3, btn_mask);
    break;
    case 0x3:
    snd_soc_jack_report(priv.jack, SND_JACK_BTN_2, btn_mask);
    break;
    case 0x1:
    snd_soc_jack_report(priv.jack, SND_JACK_BTN_1, btn_mask);
    break;
    case 0x0:
// handle BTN_0 specially for type detection
    if (!priv.detect_accessory_type)
    snd_soc_jack_report(priv.jack,
    SND_JACK_BTN_0, btn_mask);
    break;
    default:
    dev_err(component.dev,
    "Unexpected button press result (%x)", btn_result);
    break;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm8916_mbhc_switch_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm8916_mbhc_switch_irq_handler(int irq, void *arg)
    {
    struct pm8916_wcd_analog_priv *priv = arg;
    struct snd_soc_component *component = priv.component;
    let mut ins: bool = false;
    if (snd_soc_component_read(component, CDC_A_MBHC_DET_CTL_1) &
    CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_MASK)
    ins = true;
// Set the detection type appropriately
    snd_soc_component_update_bits(component, CDC_A_MBHC_DET_CTL_1,
    CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_MASK,
    (!ins << CDC_A_MBHC_DET_CTL_MECH_DET_TYPE_SHIFT));
    if (ins) { /* hs insertion */
    let mut micbias_enabled: bool = false;
    if (snd_soc_component_read(component, CDC_A_MICB_2_EN) &
    CDC_A_MICB_2_EN_ENABLE)
    micbias_enabled = true;
    pm8916_mbhc_configure_bias(priv, micbias_enabled);
//
// if only a btn0 press event is receive just before
// insert event then its a 3 pole headphone else if
// both press and release event received then its
// a headset.
//
    if (priv.mbhc_btn0_released)
    snd_soc_jack_report(priv.jack,
    SND_JACK_HEADSET, hs_jack_mask);
    else
    snd_soc_jack_report(priv.jack,
    SND_JACK_HEADPHONE, hs_jack_mask);
    priv.detect_accessory_type = false;
    } else { /* removal */
    snd_soc_jack_report(priv.jack, 0, hs_jack_mask);
    priv.detect_accessory_type = true;
    priv.mbhc_btn0_released = false;
    }
    return IRQ_HANDLED;
    }
    static struct snd_soc_dai_driver pm8916_wcd_analog_dai[] = {
    [0] = {
    .name = "pm8916_wcd_analog_pdm_rx",
    .id = 0,
    .playback = {
    .stream_name = "PDM Playback",
    .rates = MSM8916_WCD_ANALOG_RATES,
    .formats = MSM8916_WCD_ANALOG_FORMATS,
    .channels_min = 1,
    .channels_max = 3,
    },
    },
    [1] = {
    .name = "pm8916_wcd_analog_pdm_tx",
    .id = 1,
    .capture = {
    .stream_name = "PDM Capture",
    .rates = MSM8916_WCD_ANALOG_RATES,
    .formats = MSM8916_WCD_ANALOG_FORMATS,
    .channels_min = 1,
    .channels_max = 4,
    },
    },
    };
    static const struct snd_soc_component_driver pm8916_wcd_analog = {
    .probe			= pm8916_wcd_analog_probe,
    .remove			= pm8916_wcd_analog_remove,
    .set_jack		= pm8916_wcd_analog_set_jack,
    .controls		= pm8916_wcd_analog_snd_controls,
    .num_controls		= ARRAY_SIZE(pm8916_wcd_analog_snd_controls),
    .dapm_widgets		= pm8916_wcd_analog_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(pm8916_wcd_analog_dapm_widgets),
    .dapm_routes		= pm8916_wcd_analog_audio_map,
    .num_dapm_routes	= ARRAY_SIZE(pm8916_wcd_analog_audio_map),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static int pm8916_wcd_analog_parse_dt(struct device *dev,
    struct pm8916_wcd_analog_priv *priv)
    {
    int rval;
    if (of_property_read_bool(dev.of_node, "qcom,micbias1-ext-cap"))
    priv.micbias1_cap_mode = MICB_1_EN_EXT_BYP_CAP;
    else
    priv.micbias1_cap_mode = MICB_1_EN_NO_EXT_BYP_CAP;
    if (of_property_read_bool(dev.of_node, "qcom,micbias2-ext-cap"))
    priv.micbias2_cap_mode = MICB_1_EN_EXT_BYP_CAP;
    else
    priv.micbias2_cap_mode = MICB_1_EN_NO_EXT_BYP_CAP;
    of_property_read_u32(dev.of_node, "qcom,micbias-lvl",
    &priv.micbias_mv);
    if (of_property_read_bool(dev.of_node,
    "qcom,hphl-jack-type-normally-open"))
    priv.hphl_jack_type_normally_open = true;
    else
    priv.hphl_jack_type_normally_open = false;
    if (of_property_read_bool(dev.of_node,
    "qcom,gnd-jack-type-normally-open"))
    priv.gnd_jack_type_normally_open = true;
    else
    priv.gnd_jack_type_normally_open = false;
    priv.mbhc_btn_enabled = true;
    rval = of_property_read_u32_array(dev.of_node,
    "qcom,mbhc-vthreshold-low",
    &priv.vref_btn_cs[0],
    MBHC_MAX_BUTTONS);
    if (rval < 0) {
    priv.mbhc_btn_enabled = false;
    } else {
    rval = of_property_read_u32_array(dev.of_node,
    "qcom,mbhc-vthreshold-high",
    &priv.vref_btn_micb[0],
    MBHC_MAX_BUTTONS);
    if (rval < 0)
    priv.mbhc_btn_enabled = false;
    }
    if (!priv.mbhc_btn_enabled)
    dev_err(dev,
    "DT property missing, MBHC btn detection disabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8916_wcd_analog_spmi_probe(pdev: *mut platform_device) -> c_int {
    static int pm8916_wcd_analog_spmi_probe(struct platform_device *pdev)
    {
    struct pm8916_wcd_analog_priv *priv;
    struct device *dev = &pdev.dev;
    int ret, i, irq;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ret = pm8916_wcd_analog_parse_dt(dev, priv);
    if (ret < 0)
    return ret;
    for (i = 0; i < ARRAY_SIZE(supply_names); i++)
    priv.supplies[i].supply = supply_names[i];
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(priv.supplies),
    priv.supplies);
    if (ret) {
    dev_err(dev, "Failed to get regulator supplies %d\n", ret);
    return ret;
    }
    irq = platform_get_irq_byname(pdev, "mbhc_switch_int");
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    pm8916_mbhc_switch_irq_handler,
    IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT,
    "mbhc switch irq", priv);
    if (ret) {
    dev_err(dev, "cannot request mbhc switch irq\n");
    return ret;
    }
    if (priv.mbhc_btn_enabled) {
    irq = platform_get_irq_byname(pdev, "mbhc_but_press_det");
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    mbhc_btn_press_irq_handler,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "mbhc btn press irq", priv);
    if (ret) {
    dev_err(dev, "cannot request mbhc button press irq\n");
    return ret;
    }
    irq = platform_get_irq_byname(pdev, "mbhc_but_rel_det");
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    mbhc_btn_release_irq_handler,
    IRQF_TRIGGER_RISING |
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "mbhc btn release irq", priv);
    if (ret) {
    dev_err(dev, "cannot request mbhc button release irq\n");
    return ret;
    }
    }
    dev_set_drvdata(dev, priv);
    return devm_snd_soc_register_component(dev, &pm8916_wcd_analog,
    pm8916_wcd_analog_dai,
    ARRAY_SIZE(pm8916_wcd_analog_dai));
    }
    static const struct of_device_id pm8916_wcd_analog_spmi_match_table[] = {
    { .compatible = "qcom,pm8916-wcd-analog-codec", .data = &pm8916_data },
    { .compatible = "qcom,pm8950-wcd-analog-codec", .data = &pm8950_data },
    { .compatible = "qcom,pm8953-wcd-analog-codec", .data = &pm8953_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, pm8916_wcd_analog_spmi_match_table);
    static struct platform_driver pm8916_wcd_analog_spmi_driver = {
    .driver = {
    .name = "qcom,pm8916-wcd-spmi-codec",
    .of_match_table = pm8916_wcd_analog_spmi_match_table,
    },
    .probe = pm8916_wcd_analog_spmi_probe,
    };
    module_platform_driver(pm8916_wcd_analog_spmi_driver);
    MODULE_AUTHOR("Srinivas Kandagatla <srinivas.kandagatla@linaro.org>");
    MODULE_DESCRIPTION("PMIC PM8916 WCD Analog Codec driver");
    MODULE_LICENSE("GPL v2");
