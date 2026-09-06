//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/lpass-rx-macro.c
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
// Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.

pub const CDC_RX_DC_COEFF_SEL_TWO: c_uint = 0x2;

    (0x0434 + (rx.rxn_reg_stride * n) + ((n > 1) ? rx.rxn_reg_stride2 : 0))

pub const CDC_RX_DSM_OUT_DELAY_TWO_SAMPLE: c_uint = 0x2;

    (0x0440 + (rx.rxn_reg_stride * n) + ((n > 1) ? rx.rxn_reg_stride2 : 0))

// RX offsets prior to 2.5 codec version

// LPASS CODEC version 2.5 rx reg offsets

pub const MCLK_FREQ: c_int = 19200000;

    SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 |\
    SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000 |\
    SNDRV_PCM_RATE_384000)
// Fractional Rates

    SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_352800)

    SNDRV_PCM_FMTBIT_S24_LE |\
    SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE)

    SNDRV_PCM_RATE_48000)

    SNDRV_PCM_FMTBIT_S24_LE |\
    SNDRV_PCM_FMTBIT_S24_3LE)
pub const RX_MACRO_MAX_DMA_CH_PER_PORT: c_int = 2;
pub const RX_MACRO_EC_MIX_TX0_MASK: c_uint = 0xf0;
pub const RX_MACRO_EC_MIX_TX1_MASK: c_uint = 0x0f;
pub const RX_MACRO_EC_MIX_TX2_MASK: c_uint = 0x0f;
pub const COMP_MAX_COEFF: c_int = 25;
pub const RX_NUM_CLKS_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comp_coeff_val {
    pub lsb: u8,
    pub msb: u8,
}

    enum {
    HPH_ULP,
    HPH_LOHIFI,
    HPH_MODE_MAX,
    };
    static const struct comp_coeff_val comp_coeff_table[HPH_MODE_MAX][COMP_MAX_COEFF] = {
    {
    {0x40, 0x00},
    {0x4C, 0x00},
    {0x5A, 0x00},
    {0x6B, 0x00},
    {0x7F, 0x00},
    {0x97, 0x00},
    {0xB3, 0x00},
    {0xD5, 0x00},
    {0xFD, 0x00},
    {0x2D, 0x01},
    {0x66, 0x01},
    {0xA7, 0x01},
    {0xF8, 0x01},
    {0x57, 0x02},
    {0xC7, 0x02},
    {0x4B, 0x03},
    {0xE9, 0x03},
    {0xA3, 0x04},
    {0x7D, 0x05},
    {0x90, 0x06},
    {0xD1, 0x07},
    {0x49, 0x09},
    {0x00, 0x0B},
    {0x01, 0x0D},
    {0x59, 0x0F},
    },
    {
    {0x40, 0x00},
    {0x4C, 0x00},
    {0x5A, 0x00},
    {0x6B, 0x00},
    {0x80, 0x00},
    {0x98, 0x00},
    {0xB4, 0x00},
    {0xD5, 0x00},
    {0xFE, 0x00},
    {0x2E, 0x01},
    {0x66, 0x01},
    {0xA9, 0x01},
    {0xF8, 0x01},
    {0x56, 0x02},
    {0xC4, 0x02},
    {0x4F, 0x03},
    {0xF0, 0x03},
    {0xAE, 0x04},
    {0x8B, 0x05},
    {0x8E, 0x06},
    {0xBC, 0x07},
    {0x56, 0x09},
    {0x0F, 0x0B},
    {0x13, 0x0D},
    {0x6F, 0x0F},
    },
    };
    enum {
    INTERP_HPHL,
    INTERP_HPHR,
    INTERP_AUX,
    INTERP_MAX
    };
    enum {
    RX_MACRO_RX0,
    RX_MACRO_RX1,
    RX_MACRO_RX2,
    RX_MACRO_RX3,
    RX_MACRO_RX4,
    RX_MACRO_RX5,
    RX_MACRO_PORTS_MAX
    };
    enum {
    RX_MACRO_COMP1, /* HPH_L */
    RX_MACRO_COMP2, /* HPH_R */
    RX_MACRO_COMP_MAX
    };
    enum {
    RX_MACRO_EC0_MUX = 0,
    RX_MACRO_EC1_MUX,
    RX_MACRO_EC2_MUX,
    RX_MACRO_EC_MUX_MAX,
    };
    enum {
    INTn_1_INP_SEL_ZERO = 0,
    INTn_1_INP_SEL_DEC0,
    INTn_1_INP_SEL_DEC1,
    INTn_1_INP_SEL_IIR0,
    INTn_1_INP_SEL_IIR1,
    INTn_1_INP_SEL_RX0,
    INTn_1_INP_SEL_RX1,
    INTn_1_INP_SEL_RX2,
    INTn_1_INP_SEL_RX3,
    INTn_1_INP_SEL_RX4,
    INTn_1_INP_SEL_RX5,
    };
    enum {
    INTn_2_INP_SEL_ZERO = 0,
    INTn_2_INP_SEL_RX0,
    INTn_2_INP_SEL_RX1,
    INTn_2_INP_SEL_RX2,
    INTn_2_INP_SEL_RX3,
    INTn_2_INP_SEL_RX4,
    INTn_2_INP_SEL_RX5,
    };
    enum {
    INTERP_MAIN_PATH,
    INTERP_MIX_PATH,
    };
// Codec supports 2 IIR filters
    enum {
    IIR0 = 0,
    IIR1,
    IIR_MAX,
    };
// Each IIR has 5 Filter Stages
    enum {
    BAND1 = 0,
    BAND2,
    BAND3,
    BAND4,
    BAND5,
    BAND_MAX,
    };

    { \
    .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, \
    .info = rx_macro_iir_filter_info, \
    .get = rx_macro_get_iir_band_audio_mixer, \
    .put = rx_macro_put_iir_band_audio_mixer, \
    .private_value = (unsigned long)&(struct wcd_iir_filter_ctl) { \
    .iir_idx = iidx, \
    .band_idx = bidx, \
    .bytes_ext = {.max = RX_MACRO_IIR_FILTER_SIZE, }, \
    } \
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct interp_sample_rate {
    pub sample_rate: c_int,
    pub rate_val: c_int,
}

    static struct interp_sample_rate sr_val_tbl[] = {
    {8000, 0x0}, {16000, 0x1}, {32000, 0x3}, {48000, 0x4}, {96000, 0x5},
    {192000, 0x6}, {384000, 0x7}, {44100, 0x9}, {88200, 0xA},
    {176400, 0xB}, {352800, 0xC},
    };
// Matches also rx_macro_mux_text
    enum {
    RX_MACRO_AIF1_PB,
    RX_MACRO_AIF2_PB,
    RX_MACRO_AIF3_PB,
    RX_MACRO_AIF4_PB,
    RX_MACRO_AIF_ECHO,
    RX_MACRO_MAX_DAIS,
    };
    enum {
    RX_MACRO_AIF1_CAP = 0,
    RX_MACRO_AIF2_CAP,
    RX_MACRO_AIF3_CAP,
    RX_MACRO_MAX_AIF_CAP_DAIS
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_macro {
    pub dev: *mut device,
    pub comp_enabled: [c_int; RX_MACRO_COMP_MAX],
// Main path clock users count
    pub main_clk_users: [c_int; INTERP_MAX],
    pub rx_port_value: [c_int; RX_MACRO_PORTS_MAX],
    pub prim_int_users: [u16; INTERP_MAX],
    pub rx_mclk_users: c_int,
    pub clsh_users: c_int,
    pub rx_mclk_cnt: c_int,
    pub codec_version: enum lpass_codec_version,
    pub rxn_reg_stride: c_int,
    pub rxn_reg_stride2: c_int,
    pub is_ear_mode_on: bool,
    pub hph_pwr_mode: bool,
    pub hph_hd2_mode: bool,
    pub component: *mut snd_soc_component,
    pub active_ch_mask: [c_ulong; RX_MACRO_MAX_DAIS],
    pub active_ch_cnt: [c_ulong; RX_MACRO_MAX_DAIS],
    pub bit_width: [u16; RX_MACRO_MAX_DAIS],
    pub is_softclip_on: c_int,
    pub is_aux_hpf_on: c_int,
    pub softclip_clk_users: c_int,
    pub pds: *mut lpass_macro,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub npl: *mut clk,
    pub macro: *mut clk,
    pub dcodec: *mut clk,
    pub fsgen: *mut clk,
    pub hw: clk_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcd_iir_filter_ctl {
    pub iir_idx: c_uint,
    pub band_idx: c_uint,
    pub bytes_ext: soc_bytes_ext,
}

    static const DECLARE_TLV_DB_SCALE(digital_gain, -8400, 100, -8400);
    static const char * const rx_int_mix_mux_text[] = {
    "ZERO", "RX0", "RX1", "RX2", "RX3", "RX4", "RX5"
    };
    static const char * const rx_prim_mix_text[] = {
    "ZERO", "DEC0", "DEC1", "IIR0", "IIR1", "RX0", "RX1", "RX2",
    "RX3", "RX4", "RX5"
    };
    static const char * const rx_sidetone_mix_text[] = {
    "ZERO", "SRC0", "SRC1", "SRC_SUM"
    };
    static const char * const iir_inp_mux_text[] = {
    "ZERO", "DEC0", "DEC1", "DEC2", "DEC3",
    "RX0", "RX1", "RX2", "RX3", "RX4", "RX5"
    };
    static const char * const rx_int_dem_inp_mux_text[] = {
    "NORMAL_DSM_OUT", "CLSH_DSM_OUT",
    };
    static const char * const rx_int0_1_interp_mux_text[] = {
    "ZERO", "RX INT0_1 MIX1",
    };
    static const char * const rx_int1_1_interp_mux_text[] = {
    "ZERO", "RX INT1_1 MIX1",
    };
    static const char * const rx_int2_1_interp_mux_text[] = {
    "ZERO", "RX INT2_1 MIX1",
    };
    static const char * const rx_int0_2_interp_mux_text[] = {
    "ZERO", "RX INT0_2 MUX",
    };
    static const char * const rx_int1_2_interp_mux_text[] = {
    "ZERO", "RX INT1_2 MUX",
    };
    static const char * const rx_int2_2_interp_mux_text[] = {
    "ZERO", "RX INT2_2 MUX",
    };
// Order must match RX_MACRO_MAX_DAIS enum (offset by 1)
    static const char *const rx_macro_mux_text[] = {
    "ZERO", "AIF1_PB", "AIF2_PB", "AIF3_PB", "AIF4_PB"
    };
    static const char *const rx_macro_hph_pwr_mode_text[] = {
    "ULP", "LOHIFI"
    };
    static const char * const rx_echo_mux_text[] = {
    "ZERO", "RX_MIX0", "RX_MIX1", "RX_MIX2"
    };
    static const struct soc_enum rx_macro_hph_pwr_mode_enum =
    SOC_ENUM_SINGLE_EXT(2, rx_macro_hph_pwr_mode_text);
    static const struct soc_enum rx_mix_tx2_mux_enum =
    SOC_ENUM_SINGLE(CDC_RX_INP_MUX_RX_MIX_CFG5, 0, 4, rx_echo_mux_text);
    static const struct soc_enum rx_mix_tx1_mux_enum =
    SOC_ENUM_SINGLE(CDC_RX_INP_MUX_RX_MIX_CFG4, 0, 4, rx_echo_mux_text);
    static const struct soc_enum rx_mix_tx0_mux_enum =
    SOC_ENUM_SINGLE(CDC_RX_INP_MUX_RX_MIX_CFG4, 4, 4, rx_echo_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int0_2_enum, CDC_RX_INP_MUX_RX_INT0_CFG1, 0,
    rx_int_mix_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int1_2_enum, CDC_RX_INP_MUX_RX_INT1_CFG1, 0,
    rx_int_mix_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int2_2_enum, CDC_RX_INP_MUX_RX_INT2_CFG1, 0,
    rx_int_mix_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int0_1_mix_inp0_enum, CDC_RX_INP_MUX_RX_INT0_CFG0, 0,
    rx_prim_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int0_1_mix_inp1_enum, CDC_RX_INP_MUX_RX_INT0_CFG0, 4,
    rx_prim_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int0_1_mix_inp2_enum, CDC_RX_INP_MUX_RX_INT0_CFG1, 4,
    rx_prim_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int1_1_mix_inp0_enum, CDC_RX_INP_MUX_RX_INT1_CFG0, 0,
    rx_prim_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int1_1_mix_inp1_enum, CDC_RX_INP_MUX_RX_INT1_CFG0, 4,
    rx_prim_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int1_1_mix_inp2_enum, CDC_RX_INP_MUX_RX_INT1_CFG1, 4,
    rx_prim_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int2_1_mix_inp0_enum, CDC_RX_INP_MUX_RX_INT2_CFG0, 0,
    rx_prim_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int2_1_mix_inp1_enum, CDC_RX_INP_MUX_RX_INT2_CFG0, 4,
    rx_prim_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int2_1_mix_inp2_enum, CDC_RX_INP_MUX_RX_INT2_CFG1, 4,
    rx_prim_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int0_mix2_inp_enum, CDC_RX_INP_MUX_SIDETONE_SRC_CFG0, 2,
    rx_sidetone_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int1_mix2_inp_enum, CDC_RX_INP_MUX_SIDETONE_SRC_CFG0, 4,
    rx_sidetone_mix_text);
    static SOC_ENUM_SINGLE_DECL(rx_int2_mix2_inp_enum, CDC_RX_INP_MUX_SIDETONE_SRC_CFG0, 6,
    rx_sidetone_mix_text);
    static SOC_ENUM_SINGLE_DECL(iir0_inp0_enum, CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG0, 0,
    iir_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(iir0_inp1_enum, CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG1, 0,
    iir_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(iir0_inp2_enum, CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG2, 0,
    iir_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(iir0_inp3_enum, CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG3, 0,
    iir_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(iir1_inp0_enum, CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG0, 0,
    iir_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(iir1_inp1_enum, CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG1, 0,
    iir_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(iir1_inp2_enum, CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG2, 0,
    iir_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(iir1_inp3_enum, CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG3, 0,
    iir_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int0_1_interp_enum, SND_SOC_NOPM, 0,
    rx_int0_1_interp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int1_1_interp_enum, SND_SOC_NOPM, 0,
    rx_int1_1_interp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int2_1_interp_enum, SND_SOC_NOPM, 0,
    rx_int2_1_interp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int0_2_interp_enum, SND_SOC_NOPM, 0,
    rx_int0_2_interp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int1_2_interp_enum, SND_SOC_NOPM, 0,
    rx_int1_2_interp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int2_2_interp_enum, SND_SOC_NOPM, 0,
    rx_int2_2_interp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int0_dem_inp_enum, CDC_RX_RX0_RX_PATH_CFG1, 0,
    rx_int_dem_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_int1_dem_inp_enum, CDC_RX_RX1_RX_PATH_CFG1, 0,
    rx_int_dem_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_2_5_int1_dem_inp_enum, CDC_2_5_RX_RX1_RX_PATH_CFG1, 0,
    rx_int_dem_inp_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_macro_rx0_enum, SND_SOC_NOPM, 0, rx_macro_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_macro_rx1_enum, SND_SOC_NOPM, 0, rx_macro_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_macro_rx2_enum, SND_SOC_NOPM, 0, rx_macro_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_macro_rx3_enum, SND_SOC_NOPM, 0, rx_macro_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_macro_rx4_enum, SND_SOC_NOPM, 0, rx_macro_mux_text);
    static SOC_ENUM_SINGLE_DECL(rx_macro_rx5_enum, SND_SOC_NOPM, 0, rx_macro_mux_text);
    static const struct snd_kcontrol_new rx_mix_tx1_mux =
    SOC_DAPM_ENUM("RX MIX TX1_MUX Mux", rx_mix_tx1_mux_enum);
    static const struct snd_kcontrol_new rx_mix_tx2_mux =
    SOC_DAPM_ENUM("RX MIX TX2_MUX Mux", rx_mix_tx2_mux_enum);
    static const struct snd_kcontrol_new rx_int0_2_mux =
    SOC_DAPM_ENUM("rx_int0_2", rx_int0_2_enum);
    static const struct snd_kcontrol_new rx_int1_2_mux =
    SOC_DAPM_ENUM("rx_int1_2", rx_int1_2_enum);
    static const struct snd_kcontrol_new rx_int2_2_mux =
    SOC_DAPM_ENUM("rx_int2_2", rx_int2_2_enum);
    static const struct snd_kcontrol_new rx_int0_1_mix_inp0_mux =
    SOC_DAPM_ENUM("rx_int0_1_mix_inp0", rx_int0_1_mix_inp0_enum);
    static const struct snd_kcontrol_new rx_int0_1_mix_inp1_mux =
    SOC_DAPM_ENUM("rx_int0_1_mix_inp1", rx_int0_1_mix_inp1_enum);
    static const struct snd_kcontrol_new rx_int0_1_mix_inp2_mux =
    SOC_DAPM_ENUM("rx_int0_1_mix_inp2", rx_int0_1_mix_inp2_enum);
    static const struct snd_kcontrol_new rx_int1_1_mix_inp0_mux =
    SOC_DAPM_ENUM("rx_int1_1_mix_inp0", rx_int1_1_mix_inp0_enum);
    static const struct snd_kcontrol_new rx_int1_1_mix_inp1_mux =
    SOC_DAPM_ENUM("rx_int1_1_mix_inp1", rx_int1_1_mix_inp1_enum);
    static const struct snd_kcontrol_new rx_int1_1_mix_inp2_mux =
    SOC_DAPM_ENUM("rx_int1_1_mix_inp2", rx_int1_1_mix_inp2_enum);
    static const struct snd_kcontrol_new rx_int2_1_mix_inp0_mux =
    SOC_DAPM_ENUM("rx_int2_1_mix_inp0", rx_int2_1_mix_inp0_enum);
    static const struct snd_kcontrol_new rx_int2_1_mix_inp1_mux =
    SOC_DAPM_ENUM("rx_int2_1_mix_inp1", rx_int2_1_mix_inp1_enum);
    static const struct snd_kcontrol_new rx_int2_1_mix_inp2_mux =
    SOC_DAPM_ENUM("rx_int2_1_mix_inp2", rx_int2_1_mix_inp2_enum);
    static const struct snd_kcontrol_new rx_int0_mix2_inp_mux =
    SOC_DAPM_ENUM("rx_int0_mix2_inp", rx_int0_mix2_inp_enum);
    static const struct snd_kcontrol_new rx_int1_mix2_inp_mux =
    SOC_DAPM_ENUM("rx_int1_mix2_inp", rx_int1_mix2_inp_enum);
    static const struct snd_kcontrol_new rx_int2_mix2_inp_mux =
    SOC_DAPM_ENUM("rx_int2_mix2_inp", rx_int2_mix2_inp_enum);
    static const struct snd_kcontrol_new iir0_inp0_mux =
    SOC_DAPM_ENUM("iir0_inp0", iir0_inp0_enum);
    static const struct snd_kcontrol_new iir0_inp1_mux =
    SOC_DAPM_ENUM("iir0_inp1", iir0_inp1_enum);
    static const struct snd_kcontrol_new iir0_inp2_mux =
    SOC_DAPM_ENUM("iir0_inp2", iir0_inp2_enum);
    static const struct snd_kcontrol_new iir0_inp3_mux =
    SOC_DAPM_ENUM("iir0_inp3", iir0_inp3_enum);
    static const struct snd_kcontrol_new iir1_inp0_mux =
    SOC_DAPM_ENUM("iir1_inp0", iir1_inp0_enum);
    static const struct snd_kcontrol_new iir1_inp1_mux =
    SOC_DAPM_ENUM("iir1_inp1", iir1_inp1_enum);
    static const struct snd_kcontrol_new iir1_inp2_mux =
    SOC_DAPM_ENUM("iir1_inp2", iir1_inp2_enum);
    static const struct snd_kcontrol_new iir1_inp3_mux =
    SOC_DAPM_ENUM("iir1_inp3", iir1_inp3_enum);
    static const struct snd_kcontrol_new rx_int0_1_interp_mux =
    SOC_DAPM_ENUM("rx_int0_1_interp", rx_int0_1_interp_enum);
    static const struct snd_kcontrol_new rx_int1_1_interp_mux =
    SOC_DAPM_ENUM("rx_int1_1_interp", rx_int1_1_interp_enum);
    static const struct snd_kcontrol_new rx_int2_1_interp_mux =
    SOC_DAPM_ENUM("rx_int2_1_interp", rx_int2_1_interp_enum);
    static const struct snd_kcontrol_new rx_int0_2_interp_mux =
    SOC_DAPM_ENUM("rx_int0_2_interp", rx_int0_2_interp_enum);
    static const struct snd_kcontrol_new rx_int1_2_interp_mux =
    SOC_DAPM_ENUM("rx_int1_2_interp", rx_int1_2_interp_enum);
    static const struct snd_kcontrol_new rx_int2_2_interp_mux =
    SOC_DAPM_ENUM("rx_int2_2_interp", rx_int2_2_interp_enum);
    static const struct snd_kcontrol_new rx_mix_tx0_mux =
    SOC_DAPM_ENUM("RX MIX TX0_MUX Mux", rx_mix_tx0_mux_enum);
    static const struct reg_default rx_defaults[] = {
// RX Macro
    { CDC_RX_TOP_TOP_CFG0, 0x00 },
    { CDC_RX_TOP_SWR_CTRL, 0x00 },
    { CDC_RX_TOP_DEBUG, 0x00 },
    { CDC_RX_TOP_DEBUG_BUS, 0x00 },
    { CDC_RX_TOP_DEBUG_EN0, 0x00 },
    { CDC_RX_TOP_DEBUG_EN1, 0x00 },
    { CDC_RX_TOP_DEBUG_EN2, 0x00 },
    { CDC_RX_TOP_HPHL_COMP_WR_LSB, 0x00 },
    { CDC_RX_TOP_HPHL_COMP_WR_MSB, 0x00 },
    { CDC_RX_TOP_HPHL_COMP_LUT, 0x00 },
    { CDC_RX_TOP_HPHL_COMP_RD_LSB, 0x00 },
    { CDC_RX_TOP_HPHL_COMP_RD_MSB, 0x00 },
    { CDC_RX_TOP_HPHR_COMP_WR_LSB, 0x00 },
    { CDC_RX_TOP_HPHR_COMP_WR_MSB, 0x00 },
    { CDC_RX_TOP_HPHR_COMP_LUT, 0x00 },
    { CDC_RX_TOP_HPHR_COMP_RD_LSB, 0x00 },
    { CDC_RX_TOP_HPHR_COMP_RD_MSB, 0x00 },
    { CDC_RX_TOP_DSD0_DEBUG_CFG0, 0x11 },
    { CDC_RX_TOP_DSD0_DEBUG_CFG1, 0x20 },
    { CDC_RX_TOP_DSD0_DEBUG_CFG2, 0x00 },
    { CDC_RX_TOP_DSD0_DEBUG_CFG3, 0x00 },
    { CDC_RX_TOP_DSD1_DEBUG_CFG0, 0x11 },
    { CDC_RX_TOP_DSD1_DEBUG_CFG1, 0x20 },
    { CDC_RX_TOP_DSD1_DEBUG_CFG2, 0x00 },
    { CDC_RX_TOP_DSD1_DEBUG_CFG3, 0x00 },
    { CDC_RX_TOP_RX_I2S_CTL, 0x0C },
    { CDC_RX_TOP_TX_I2S2_CTL, 0x0C },
    { CDC_RX_TOP_I2S_CLK, 0x0C },
    { CDC_RX_TOP_I2S_RESET, 0x00 },
    { CDC_RX_TOP_I2S_MUX, 0x00 },
    { CDC_RX_CLK_RST_CTRL_MCLK_CONTROL, 0x00 },
    { CDC_RX_CLK_RST_CTRL_FS_CNT_CONTROL, 0x00 },
    { CDC_RX_CLK_RST_CTRL_SWR_CONTROL, 0x00 },
    { CDC_RX_CLK_RST_CTRL_DSD_CONTROL, 0x00 },
    { CDC_RX_CLK_RST_CTRL_ASRC_SHARE_CONTROL, 0x08 },
    { CDC_RX_SOFTCLIP_CRC, 0x00 },
    { CDC_RX_SOFTCLIP_SOFTCLIP_CTRL, 0x38 },
    { CDC_RX_INP_MUX_RX_INT0_CFG0, 0x00 },
    { CDC_RX_INP_MUX_RX_INT0_CFG1, 0x00 },
    { CDC_RX_INP_MUX_RX_INT1_CFG0, 0x00 },
    { CDC_RX_INP_MUX_RX_INT1_CFG1, 0x00 },
    { CDC_RX_INP_MUX_RX_INT2_CFG0, 0x00 },
    { CDC_RX_INP_MUX_RX_INT2_CFG1, 0x00 },
    { CDC_RX_INP_MUX_RX_MIX_CFG4, 0x00 },
    { CDC_RX_INP_MUX_RX_MIX_CFG5, 0x00 },
    { CDC_RX_INP_MUX_SIDETONE_SRC_CFG0, 0x00 },
    { CDC_RX_CLSH_CRC, 0x00 },
    { CDC_RX_CLSH_DLY_CTRL, 0x03 },
    { CDC_RX_CLSH_DECAY_CTRL, 0x02 },
    { CDC_RX_CLSH_HPH_V_PA, 0x1C },
    { CDC_RX_CLSH_EAR_V_PA, 0x39 },
    { CDC_RX_CLSH_HPH_V_HD, 0x0C },
    { CDC_RX_CLSH_EAR_V_HD, 0x0C },
    { CDC_RX_CLSH_K1_MSB, 0x01 },
    { CDC_RX_CLSH_K1_LSB, 0x00 },
    { CDC_RX_CLSH_K2_MSB, 0x00 },
    { CDC_RX_CLSH_K2_LSB, 0x80 },
    { CDC_RX_CLSH_IDLE_CTRL, 0x00 },
    { CDC_RX_CLSH_IDLE_HPH, 0x00 },
    { CDC_RX_CLSH_IDLE_EAR, 0x00 },
    { CDC_RX_CLSH_TEST0, 0x07 },
    { CDC_RX_CLSH_TEST1, 0x00 },
    { CDC_RX_CLSH_OVR_VREF, 0x00 },
    { CDC_RX_CLSH_CLSG_CTL, 0x02 },
    { CDC_RX_CLSH_CLSG_CFG1, 0x9A },
    { CDC_RX_CLSH_CLSG_CFG2, 0x10 },
    { CDC_RX_BCL_VBAT_PATH_CTL, 0x00 },
    { CDC_RX_BCL_VBAT_CFG, 0x10 },
    { CDC_RX_BCL_VBAT_ADC_CAL1, 0x00 },
    { CDC_RX_BCL_VBAT_ADC_CAL2, 0x00 },
    { CDC_RX_BCL_VBAT_ADC_CAL3, 0x04 },
    { CDC_RX_BCL_VBAT_PK_EST1, 0xE0 },
    { CDC_RX_BCL_VBAT_PK_EST2, 0x01 },
    { CDC_RX_BCL_VBAT_PK_EST3, 0x40 },
    { CDC_RX_BCL_VBAT_RF_PROC1, 0x2A },
    { CDC_RX_BCL_VBAT_RF_PROC2, 0x00 },
    { CDC_RX_BCL_VBAT_TAC1, 0x00 },
    { CDC_RX_BCL_VBAT_TAC2, 0x18 },
    { CDC_RX_BCL_VBAT_TAC3, 0x18 },
    { CDC_RX_BCL_VBAT_TAC4, 0x03 },
    { CDC_RX_BCL_VBAT_GAIN_UPD1, 0x01 },
    { CDC_RX_BCL_VBAT_GAIN_UPD2, 0x00 },
    { CDC_RX_BCL_VBAT_GAIN_UPD3, 0x00 },
    { CDC_RX_BCL_VBAT_GAIN_UPD4, 0x64 },
    { CDC_RX_BCL_VBAT_GAIN_UPD5, 0x01 },
    { CDC_RX_BCL_VBAT_DEBUG1, 0x00 },
    { CDC_RX_BCL_VBAT_GAIN_UPD_MON, 0x00 },
    { CDC_RX_BCL_VBAT_GAIN_MON_VAL, 0x00 },
    { CDC_RX_BCL_VBAT_BAN, 0x0C },
    { CDC_RX_BCL_VBAT_BCL_GAIN_UPD1, 0x00 },
    { CDC_RX_BCL_VBAT_BCL_GAIN_UPD2, 0x77 },
    { CDC_RX_BCL_VBAT_BCL_GAIN_UPD3, 0x01 },
    { CDC_RX_BCL_VBAT_BCL_GAIN_UPD4, 0x00 },
    { CDC_RX_BCL_VBAT_BCL_GAIN_UPD5, 0x4B },
    { CDC_RX_BCL_VBAT_BCL_GAIN_UPD6, 0x00 },
    { CDC_RX_BCL_VBAT_BCL_GAIN_UPD7, 0x01 },
    { CDC_RX_BCL_VBAT_BCL_GAIN_UPD8, 0x00 },
    { CDC_RX_BCL_VBAT_BCL_GAIN_UPD9, 0x00 },
    { CDC_RX_BCL_VBAT_ATTN1, 0x04 },
    { CDC_RX_BCL_VBAT_ATTN2, 0x08 },
    { CDC_RX_BCL_VBAT_ATTN3, 0x0C },
    { CDC_RX_BCL_VBAT_DECODE_CTL1, 0xE0 },
    { CDC_RX_BCL_VBAT_DECODE_CTL2, 0x00 },
    { CDC_RX_BCL_VBAT_DECODE_CFG1, 0x00 },
    { CDC_RX_BCL_VBAT_DECODE_CFG2, 0x00 },
    { CDC_RX_BCL_VBAT_DECODE_CFG3, 0x00 },
    { CDC_RX_BCL_VBAT_DECODE_CFG4, 0x00 },
    { CDC_RX_BCL_VBAT_DECODE_ST, 0x00 },
    { CDC_RX_INTR_CTRL_CFG, 0x00 },
    { CDC_RX_INTR_CTRL_CLR_COMMIT, 0x00 },
    { CDC_RX_INTR_CTRL_PIN1_MASK0, 0xFF },
    { CDC_RX_INTR_CTRL_PIN1_STATUS0, 0x00 },
    { CDC_RX_INTR_CTRL_PIN1_CLEAR0, 0x00 },
    { CDC_RX_INTR_CTRL_PIN2_MASK0, 0xFF },
    { CDC_RX_INTR_CTRL_PIN2_STATUS0, 0x00 },
    { CDC_RX_INTR_CTRL_PIN2_CLEAR0, 0x00 },
    { CDC_RX_INTR_CTRL_LEVEL0, 0x00 },
    { CDC_RX_INTR_CTRL_BYPASS0, 0x00 },
    { CDC_RX_INTR_CTRL_SET0, 0x00 },
    { CDC_RX_RX0_RX_PATH_CTL, 0x04 },
    { CDC_RX_RX0_RX_PATH_CFG0, 0x00 },
    { CDC_RX_RX0_RX_PATH_CFG1, 0x64 },
    { CDC_RX_RX0_RX_PATH_CFG2, 0x8F },
    { CDC_RX_RX0_RX_PATH_CFG3, 0x00 },
    { CDC_RX_RX0_RX_VOL_CTL, 0x00 },
    { CDC_RX_RX0_RX_PATH_MIX_CTL, 0x04 },
    { CDC_RX_RX0_RX_PATH_MIX_CFG, 0x7E },
    { CDC_RX_RX0_RX_VOL_MIX_CTL, 0x00 },
    { CDC_RX_RX0_RX_PATH_SEC1, 0x08 },
    { CDC_RX_RX0_RX_PATH_SEC2, 0x00 },
    { CDC_RX_RX0_RX_PATH_SEC3, 0x00 },
    { CDC_RX_RX0_RX_PATH_SEC4, 0x00 },
    { CDC_RX_RX0_RX_PATH_SEC7, 0x00 },
    { CDC_RX_RX0_RX_PATH_MIX_SEC0, 0x08 },
    { CDC_RX_RX0_RX_PATH_MIX_SEC1, 0x00 },
    { CDC_RX_RX0_RX_PATH_DSM_CTL, 0x08 },
    { CDC_RX_RX0_RX_PATH_DSM_DATA1, 0x00 },
    { CDC_RX_RX0_RX_PATH_DSM_DATA2, 0x00 },
    { CDC_RX_RX0_RX_PATH_DSM_DATA3, 0x00 },
    { CDC_RX_RX0_RX_PATH_DSM_DATA4, 0x55 },
    { CDC_RX_RX0_RX_PATH_DSM_DATA5, 0x55 },
    { CDC_RX_RX0_RX_PATH_DSM_DATA6, 0x55 },
    { CDC_RX_IDLE_DETECT_PATH_CTL, 0x00 },
    { CDC_RX_IDLE_DETECT_CFG0, 0x07 },
    { CDC_RX_IDLE_DETECT_CFG1, 0x3C },
    { CDC_RX_IDLE_DETECT_CFG2, 0x00 },
    { CDC_RX_IDLE_DETECT_CFG3, 0x00 },
    { CDC_RX_COMPANDER0_CTL0, 0x60 },
    { CDC_RX_COMPANDER0_CTL1, 0xDB },
    { CDC_RX_COMPANDER0_CTL2, 0xFF },
    { CDC_RX_COMPANDER0_CTL3, 0x35 },
    { CDC_RX_COMPANDER0_CTL4, 0xFF },
    { CDC_RX_COMPANDER0_CTL5, 0x00 },
    { CDC_RX_COMPANDER0_CTL6, 0x01 },
    { CDC_RX_COMPANDER0_CTL7, 0x28 },
    { CDC_RX_COMPANDER1_CTL0, 0x60 },
    { CDC_RX_COMPANDER1_CTL1, 0xDB },
    { CDC_RX_COMPANDER1_CTL2, 0xFF },
    { CDC_RX_COMPANDER1_CTL3, 0x35 },
    { CDC_RX_COMPANDER1_CTL4, 0xFF },
    { CDC_RX_COMPANDER1_CTL5, 0x00 },
    { CDC_RX_COMPANDER1_CTL6, 0x01 },
    { CDC_RX_COMPANDER1_CTL7, 0x28 },
    { CDC_RX_SIDETONE_IIR0_IIR_PATH_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_GAIN_B1_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_GAIN_B2_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_GAIN_B3_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_GAIN_B4_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_GAIN_B5_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_GAIN_B6_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_GAIN_B7_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_GAIN_B8_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_CTL, 0x40 },
    { CDC_RX_SIDETONE_IIR0_IIR_GAIN_TIMER_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_COEF_B1_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR0_IIR_COEF_B2_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_PATH_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_GAIN_B1_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_GAIN_B2_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_GAIN_B3_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_GAIN_B4_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_GAIN_B5_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_GAIN_B6_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_GAIN_B7_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_GAIN_B8_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_CTL, 0x40 },
    { CDC_RX_SIDETONE_IIR1_IIR_GAIN_TIMER_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_COEF_B1_CTL, 0x00 },
    { CDC_RX_SIDETONE_IIR1_IIR_COEF_B2_CTL, 0x00 },
    { CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG0, 0x00 },
    { CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG1, 0x00 },
    { CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG2, 0x00 },
    { CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG3, 0x00 },
    { CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG0, 0x00 },
    { CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG1, 0x00 },
    { CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG2, 0x00 },
    { CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG3, 0x00 },
    { CDC_RX_SIDETONE_SRC0_ST_SRC_PATH_CTL, 0x04 },
    { CDC_RX_SIDETONE_SRC0_ST_SRC_PATH_CFG1, 0x00 },
    { CDC_RX_SIDETONE_SRC1_ST_SRC_PATH_CTL, 0x04 },
    { CDC_RX_SIDETONE_SRC1_ST_SRC_PATH_CFG1, 0x00 },
    { CDC_RX_EC_REF_HQ0_EC_REF_HQ_PATH_CTL, 0x00 },
    { CDC_RX_EC_REF_HQ0_EC_REF_HQ_CFG0, 0x01 },
    { CDC_RX_EC_REF_HQ1_EC_REF_HQ_PATH_CTL, 0x00 },
    { CDC_RX_EC_REF_HQ1_EC_REF_HQ_CFG0, 0x01 },
    { CDC_RX_EC_REF_HQ2_EC_REF_HQ_PATH_CTL, 0x00 },
    { CDC_RX_EC_REF_HQ2_EC_REF_HQ_CFG0, 0x01 },
    { CDC_RX_EC_ASRC0_CLK_RST_CTL, 0x00 },
    { CDC_RX_EC_ASRC0_CTL0, 0x00 },
    { CDC_RX_EC_ASRC0_CTL1, 0x00 },
    { CDC_RX_EC_ASRC0_FIFO_CTL, 0xA8 },
    { CDC_RX_EC_ASRC0_STATUS_FMIN_CNTR_LSB, 0x00 },
    { CDC_RX_EC_ASRC0_STATUS_FMIN_CNTR_MSB, 0x00 },
    { CDC_RX_EC_ASRC0_STATUS_FMAX_CNTR_LSB, 0x00 },
    { CDC_RX_EC_ASRC0_STATUS_FMAX_CNTR_MSB, 0x00 },
    { CDC_RX_EC_ASRC0_STATUS_FIFO, 0x00 },
    { CDC_RX_EC_ASRC1_CLK_RST_CTL, 0x00 },
    { CDC_RX_EC_ASRC1_CTL0, 0x00 },
    { CDC_RX_EC_ASRC1_CTL1, 0x00 },
    { CDC_RX_EC_ASRC1_FIFO_CTL, 0xA8 },
    { CDC_RX_EC_ASRC1_STATUS_FMIN_CNTR_LSB, 0x00 },
    { CDC_RX_EC_ASRC1_STATUS_FMIN_CNTR_MSB, 0x00 },
    { CDC_RX_EC_ASRC1_STATUS_FMAX_CNTR_LSB, 0x00 },
    { CDC_RX_EC_ASRC1_STATUS_FMAX_CNTR_MSB, 0x00 },
    { CDC_RX_EC_ASRC1_STATUS_FIFO, 0x00 },
    { CDC_RX_EC_ASRC2_CLK_RST_CTL, 0x00 },
    { CDC_RX_EC_ASRC2_CTL0, 0x00 },
    { CDC_RX_EC_ASRC2_CTL1, 0x00 },
    { CDC_RX_EC_ASRC2_FIFO_CTL, 0xA8 },
    { CDC_RX_EC_ASRC2_STATUS_FMIN_CNTR_LSB, 0x00 },
    { CDC_RX_EC_ASRC2_STATUS_FMIN_CNTR_MSB, 0x00 },
    { CDC_RX_EC_ASRC2_STATUS_FMAX_CNTR_LSB, 0x00 },
    { CDC_RX_EC_ASRC2_STATUS_FMAX_CNTR_MSB, 0x00 },
    { CDC_RX_EC_ASRC2_STATUS_FIFO, 0x00 },
    { CDC_RX_DSD0_PATH_CTL, 0x00 },
    { CDC_RX_DSD0_CFG0, 0x00 },
    { CDC_RX_DSD0_CFG1, 0x62 },
    { CDC_RX_DSD0_CFG2, 0x96 },
    { CDC_RX_DSD1_PATH_CTL, 0x00 },
    { CDC_RX_DSD1_CFG0, 0x00 },
    { CDC_RX_DSD1_CFG1, 0x62 },
    { CDC_RX_DSD1_CFG2, 0x96 },
    };
    static const struct reg_default rx_2_5_defaults[] = {
    { CDC_2_5_RX_RX1_RX_PATH_CTL, 0x04 },
    { CDC_2_5_RX_RX1_RX_PATH_CFG0, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_CFG1, 0x64 },
    { CDC_2_5_RX_RX1_RX_PATH_CFG2, 0x8F },
    { CDC_2_5_RX_RX1_RX_PATH_CFG3, 0x00 },
    { CDC_2_5_RX_RX1_RX_VOL_CTL, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_MIX_CTL, 0x04 },
    { CDC_2_5_RX_RX1_RX_PATH_MIX_CFG, 0x7E },
    { CDC_2_5_RX_RX1_RX_VOL_MIX_CTL, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_SEC1, 0x08 },
    { CDC_2_5_RX_RX1_RX_PATH_SEC2, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_SEC3, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_SEC4, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_SEC7, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_MIX_SEC0, 0x08 },
    { CDC_2_5_RX_RX1_RX_PATH_MIX_SEC1, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_DSM_CTL, 0x08 },
    { CDC_2_5_RX_RX1_RX_PATH_DSM_DATA1, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_DSM_DATA2, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_DSM_DATA3, 0x00 },
    { CDC_2_5_RX_RX1_RX_PATH_DSM_DATA4, 0x55 },
    { CDC_2_5_RX_RX1_RX_PATH_DSM_DATA5, 0x55 },
    { CDC_2_5_RX_RX1_RX_PATH_DSM_DATA6, 0x55 },
    { CDC_2_5_RX_RX2_RX_PATH_CTL, 0x04 },
    { CDC_2_5_RX_RX2_RX_PATH_CFG0, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_CFG1, 0x64 },
    { CDC_2_5_RX_RX2_RX_PATH_CFG2, 0x8F },
    { CDC_2_5_RX_RX2_RX_PATH_CFG3, 0x00 },
    { CDC_2_5_RX_RX2_RX_VOL_CTL, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_MIX_CTL, 0x04 },
    { CDC_2_5_RX_RX2_RX_PATH_MIX_CFG, 0x7E },
    { CDC_2_5_RX_RX2_RX_VOL_MIX_CTL, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_SEC0, 0x04 },
    { CDC_2_5_RX_RX2_RX_PATH_SEC1, 0x08 },
    { CDC_2_5_RX_RX2_RX_PATH_SEC2, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_SEC3, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_SEC4, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_SEC5, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_SEC6, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_SEC7, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_MIX_SEC0, 0x08 },
    { CDC_2_5_RX_RX2_RX_PATH_MIX_SEC1, 0x00 },
    { CDC_2_5_RX_RX2_RX_PATH_DSM_CTL, 0x00 },
    };
    static const struct reg_default rx_pre_2_5_defaults[] = {
    { CDC_RX_RX1_RX_PATH_CTL, 0x04 },
    { CDC_RX_RX1_RX_PATH_CFG0, 0x00 },
    { CDC_RX_RX1_RX_PATH_CFG1, 0x64 },
    { CDC_RX_RX1_RX_PATH_CFG2, 0x8F },
    { CDC_RX_RX1_RX_PATH_CFG3, 0x00 },
    { CDC_RX_RX1_RX_VOL_CTL, 0x00 },
    { CDC_RX_RX1_RX_PATH_MIX_CTL, 0x04 },
    { CDC_RX_RX1_RX_PATH_MIX_CFG, 0x7E },
    { CDC_RX_RX1_RX_VOL_MIX_CTL, 0x00 },
    { CDC_RX_RX1_RX_PATH_SEC1, 0x08 },
    { CDC_RX_RX1_RX_PATH_SEC2, 0x00 },
    { CDC_RX_RX1_RX_PATH_SEC3, 0x00 },
    { CDC_RX_RX1_RX_PATH_SEC4, 0x00 },
    { CDC_RX_RX1_RX_PATH_SEC7, 0x00 },
    { CDC_RX_RX1_RX_PATH_MIX_SEC0, 0x08 },
    { CDC_RX_RX1_RX_PATH_MIX_SEC1, 0x00 },
    { CDC_RX_RX1_RX_PATH_DSM_CTL, 0x08 },
    { CDC_RX_RX1_RX_PATH_DSM_DATA1, 0x00 },
    { CDC_RX_RX1_RX_PATH_DSM_DATA2, 0x00 },
    { CDC_RX_RX1_RX_PATH_DSM_DATA3, 0x00 },
    { CDC_RX_RX1_RX_PATH_DSM_DATA4, 0x55 },
    { CDC_RX_RX1_RX_PATH_DSM_DATA5, 0x55 },
    { CDC_RX_RX1_RX_PATH_DSM_DATA6, 0x55 },
    { CDC_RX_RX2_RX_PATH_CTL, 0x04 },
    { CDC_RX_RX2_RX_PATH_CFG0, 0x00 },
    { CDC_RX_RX2_RX_PATH_CFG1, 0x64 },
    { CDC_RX_RX2_RX_PATH_CFG2, 0x8F },
    { CDC_RX_RX2_RX_PATH_CFG3, 0x00 },
    { CDC_RX_RX2_RX_VOL_CTL, 0x00 },
    { CDC_RX_RX2_RX_PATH_MIX_CTL, 0x04 },
    { CDC_RX_RX2_RX_PATH_MIX_CFG, 0x7E },
    { CDC_RX_RX2_RX_VOL_MIX_CTL, 0x00 },
    { CDC_RX_RX2_RX_PATH_SEC0, 0x04 },
    { CDC_RX_RX2_RX_PATH_SEC1, 0x08 },
    { CDC_RX_RX2_RX_PATH_SEC2, 0x00 },
    { CDC_RX_RX2_RX_PATH_SEC3, 0x00 },
    { CDC_RX_RX2_RX_PATH_SEC4, 0x00 },
    { CDC_RX_RX2_RX_PATH_SEC5, 0x00 },
    { CDC_RX_RX2_RX_PATH_SEC6, 0x00 },
    { CDC_RX_RX2_RX_PATH_SEC7, 0x00 },
    { CDC_RX_RX2_RX_PATH_MIX_SEC0, 0x08 },
    { CDC_RX_RX2_RX_PATH_MIX_SEC1, 0x00 },
    { CDC_RX_RX2_RX_PATH_DSM_CTL, 0x00 },
    };
    static bool rx_is_wronly_register(struct device *dev,
    unsigned int reg)
    {
    switch (reg) {
    case CDC_RX_BCL_VBAT_GAIN_UPD_MON:
    case CDC_RX_INTR_CTRL_CLR_COMMIT:
    case CDC_RX_INTR_CTRL_PIN1_CLEAR0:
    case CDC_RX_INTR_CTRL_PIN2_CLEAR0:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rx_is_volatile_register(dev: *mut device, reg: c_uint) -> bool {
    static bool rx_is_volatile_register(struct device *dev, unsigned int reg)
    {
// Update volatile list for rx/tx macros
    switch (reg) {
    case CDC_RX_TOP_HPHL_COMP_RD_LSB:
    case CDC_RX_TOP_HPHL_COMP_WR_LSB:
    case CDC_RX_TOP_HPHL_COMP_RD_MSB:
    case CDC_RX_TOP_HPHL_COMP_WR_MSB:
    case CDC_RX_TOP_HPHR_COMP_RD_LSB:
    case CDC_RX_TOP_HPHR_COMP_WR_LSB:
    case CDC_RX_TOP_HPHR_COMP_RD_MSB:
    case CDC_RX_TOP_HPHR_COMP_WR_MSB:
    case CDC_RX_TOP_DSD0_DEBUG_CFG2:
    case CDC_RX_TOP_DSD1_DEBUG_CFG2:
    case CDC_RX_BCL_VBAT_GAIN_MON_VAL:
    case CDC_RX_BCL_VBAT_DECODE_ST:
    case CDC_RX_INTR_CTRL_PIN1_STATUS0:
    case CDC_RX_INTR_CTRL_PIN2_STATUS0:
    case CDC_RX_COMPANDER0_CTL6:
    case CDC_RX_COMPANDER1_CTL6:
    case CDC_RX_EC_ASRC0_STATUS_FMIN_CNTR_LSB:
    case CDC_RX_EC_ASRC0_STATUS_FMIN_CNTR_MSB:
    case CDC_RX_EC_ASRC0_STATUS_FMAX_CNTR_LSB:
    case CDC_RX_EC_ASRC0_STATUS_FMAX_CNTR_MSB:
    case CDC_RX_EC_ASRC0_STATUS_FIFO:
    case CDC_RX_EC_ASRC1_STATUS_FMIN_CNTR_LSB:
    case CDC_RX_EC_ASRC1_STATUS_FMIN_CNTR_MSB:
    case CDC_RX_EC_ASRC1_STATUS_FMAX_CNTR_LSB:
    case CDC_RX_EC_ASRC1_STATUS_FMAX_CNTR_MSB:
    case CDC_RX_EC_ASRC1_STATUS_FIFO:
    case CDC_RX_EC_ASRC2_STATUS_FMIN_CNTR_LSB:
    case CDC_RX_EC_ASRC2_STATUS_FMIN_CNTR_MSB:
    case CDC_RX_EC_ASRC2_STATUS_FMAX_CNTR_LSB:
    case CDC_RX_EC_ASRC2_STATUS_FMAX_CNTR_MSB:
    case CDC_RX_EC_ASRC2_STATUS_FIFO:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rx_pre_2_5_is_rw_register(dev: *mut device, reg: c_uint) -> bool {
    static bool rx_pre_2_5_is_rw_register(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case CDC_RX_RX1_RX_PATH_CTL:
    case CDC_RX_RX1_RX_PATH_CFG0:
    case CDC_RX_RX1_RX_PATH_CFG1:
    case CDC_RX_RX1_RX_PATH_CFG2:
    case CDC_RX_RX1_RX_PATH_CFG3:
    case CDC_RX_RX1_RX_VOL_CTL:
    case CDC_RX_RX1_RX_PATH_MIX_CTL:
    case CDC_RX_RX1_RX_PATH_MIX_CFG:
    case CDC_RX_RX1_RX_VOL_MIX_CTL:
    case CDC_RX_RX1_RX_PATH_SEC1:
    case CDC_RX_RX1_RX_PATH_SEC2:
    case CDC_RX_RX1_RX_PATH_SEC3:
    case CDC_RX_RX1_RX_PATH_SEC4:
    case CDC_RX_RX1_RX_PATH_SEC7:
    case CDC_RX_RX1_RX_PATH_MIX_SEC0:
    case CDC_RX_RX1_RX_PATH_MIX_SEC1:
    case CDC_RX_RX1_RX_PATH_DSM_CTL:
    case CDC_RX_RX1_RX_PATH_DSM_DATA1:
    case CDC_RX_RX1_RX_PATH_DSM_DATA2:
    case CDC_RX_RX1_RX_PATH_DSM_DATA3:
    case CDC_RX_RX1_RX_PATH_DSM_DATA4:
    case CDC_RX_RX1_RX_PATH_DSM_DATA5:
    case CDC_RX_RX1_RX_PATH_DSM_DATA6:
    case CDC_RX_RX2_RX_PATH_CTL:
    case CDC_RX_RX2_RX_PATH_CFG0:
    case CDC_RX_RX2_RX_PATH_CFG1:
    case CDC_RX_RX2_RX_PATH_CFG2:
    case CDC_RX_RX2_RX_PATH_CFG3:
    case CDC_RX_RX2_RX_VOL_CTL:
    case CDC_RX_RX2_RX_PATH_MIX_CTL:
    case CDC_RX_RX2_RX_PATH_MIX_CFG:
    case CDC_RX_RX2_RX_VOL_MIX_CTL:
    case CDC_RX_RX2_RX_PATH_SEC0:
    case CDC_RX_RX2_RX_PATH_SEC1:
    case CDC_RX_RX2_RX_PATH_SEC2:
    case CDC_RX_RX2_RX_PATH_SEC3:
    case CDC_RX_RX2_RX_PATH_SEC4:
    case CDC_RX_RX2_RX_PATH_SEC5:
    case CDC_RX_RX2_RX_PATH_SEC6:
    case CDC_RX_RX2_RX_PATH_SEC7:
    case CDC_RX_RX2_RX_PATH_MIX_SEC0:
    case CDC_RX_RX2_RX_PATH_MIX_SEC1:
    case CDC_RX_RX2_RX_PATH_DSM_CTL:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rx_2_5_is_rw_register(dev: *mut device, reg: c_uint) -> bool {
    static bool rx_2_5_is_rw_register(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case CDC_2_5_RX_RX1_RX_PATH_CTL:
    case CDC_2_5_RX_RX1_RX_PATH_CFG0:
    case CDC_2_5_RX_RX1_RX_PATH_CFG1:
    case CDC_2_5_RX_RX1_RX_PATH_CFG2:
    case CDC_2_5_RX_RX1_RX_PATH_CFG3:
    case CDC_2_5_RX_RX1_RX_VOL_CTL:
    case CDC_2_5_RX_RX1_RX_PATH_MIX_CTL:
    case CDC_2_5_RX_RX1_RX_PATH_MIX_CFG:
    case CDC_2_5_RX_RX1_RX_VOL_MIX_CTL:
    case CDC_2_5_RX_RX1_RX_PATH_SEC1:
    case CDC_2_5_RX_RX1_RX_PATH_SEC2:
    case CDC_2_5_RX_RX1_RX_PATH_SEC3:
    case CDC_2_5_RX_RX1_RX_PATH_SEC4:
    case CDC_2_5_RX_RX1_RX_PATH_SEC7:
    case CDC_2_5_RX_RX1_RX_PATH_MIX_SEC0:
    case CDC_2_5_RX_RX1_RX_PATH_MIX_SEC1:
    case CDC_2_5_RX_RX1_RX_PATH_DSM_CTL:
    case CDC_2_5_RX_RX1_RX_PATH_DSM_DATA1:
    case CDC_2_5_RX_RX1_RX_PATH_DSM_DATA2:
    case CDC_2_5_RX_RX1_RX_PATH_DSM_DATA3:
    case CDC_2_5_RX_RX1_RX_PATH_DSM_DATA4:
    case CDC_2_5_RX_RX1_RX_PATH_DSM_DATA5:
    case CDC_2_5_RX_RX1_RX_PATH_DSM_DATA6:
    case CDC_2_5_RX_RX2_RX_PATH_CTL:
    case CDC_2_5_RX_RX2_RX_PATH_CFG0:
    case CDC_2_5_RX_RX2_RX_PATH_CFG1:
    case CDC_2_5_RX_RX2_RX_PATH_CFG2:
    case CDC_2_5_RX_RX2_RX_PATH_CFG3:
    case CDC_2_5_RX_RX2_RX_VOL_CTL:
    case CDC_2_5_RX_RX2_RX_PATH_MIX_CTL:
    case CDC_2_5_RX_RX2_RX_PATH_MIX_CFG:
    case CDC_2_5_RX_RX2_RX_VOL_MIX_CTL:
    case CDC_2_5_RX_RX2_RX_PATH_SEC0:
    case CDC_2_5_RX_RX2_RX_PATH_SEC1:
    case CDC_2_5_RX_RX2_RX_PATH_SEC2:
    case CDC_2_5_RX_RX2_RX_PATH_SEC3:
    case CDC_2_5_RX_RX2_RX_PATH_SEC4:
    case CDC_2_5_RX_RX2_RX_PATH_SEC5:
    case CDC_2_5_RX_RX2_RX_PATH_SEC6:
    case CDC_2_5_RX_RX2_RX_PATH_SEC7:
    case CDC_2_5_RX_RX2_RX_PATH_MIX_SEC0:
    case CDC_2_5_RX_RX2_RX_PATH_MIX_SEC1:
    case CDC_2_5_RX_RX2_RX_PATH_DSM_CTL:
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rx_is_rw_register(dev: *mut device, reg: c_uint) -> bool {
    static bool rx_is_rw_register(struct device *dev, unsigned int reg)
    {
    struct rx_macro *rx = dev_get_drvdata(dev);
    switch (reg) {
    case CDC_RX_TOP_TOP_CFG0:
    case CDC_RX_TOP_SWR_CTRL:
    case CDC_RX_TOP_DEBUG:
    case CDC_RX_TOP_DEBUG_BUS:
    case CDC_RX_TOP_DEBUG_EN0:
    case CDC_RX_TOP_DEBUG_EN1:
    case CDC_RX_TOP_DEBUG_EN2:
    case CDC_RX_TOP_HPHL_COMP_WR_LSB:
    case CDC_RX_TOP_HPHL_COMP_WR_MSB:
    case CDC_RX_TOP_HPHL_COMP_LUT:
    case CDC_RX_TOP_HPHR_COMP_WR_LSB:
    case CDC_RX_TOP_HPHR_COMP_WR_MSB:
    case CDC_RX_TOP_HPHR_COMP_LUT:
    case CDC_RX_TOP_DSD0_DEBUG_CFG0:
    case CDC_RX_TOP_DSD0_DEBUG_CFG1:
    case CDC_RX_TOP_DSD0_DEBUG_CFG3:
    case CDC_RX_TOP_DSD1_DEBUG_CFG0:
    case CDC_RX_TOP_DSD1_DEBUG_CFG1:
    case CDC_RX_TOP_DSD1_DEBUG_CFG3:
    case CDC_RX_TOP_RX_I2S_CTL:
    case CDC_RX_TOP_TX_I2S2_CTL:
    case CDC_RX_TOP_I2S_CLK:
    case CDC_RX_TOP_I2S_RESET:
    case CDC_RX_TOP_I2S_MUX:
    case CDC_RX_CLK_RST_CTRL_MCLK_CONTROL:
    case CDC_RX_CLK_RST_CTRL_FS_CNT_CONTROL:
    case CDC_RX_CLK_RST_CTRL_SWR_CONTROL:
    case CDC_RX_CLK_RST_CTRL_DSD_CONTROL:
    case CDC_RX_CLK_RST_CTRL_ASRC_SHARE_CONTROL:
    case CDC_RX_SOFTCLIP_CRC:
    case CDC_RX_SOFTCLIP_SOFTCLIP_CTRL:
    case CDC_RX_INP_MUX_RX_INT0_CFG0:
    case CDC_RX_INP_MUX_RX_INT0_CFG1:
    case CDC_RX_INP_MUX_RX_INT1_CFG0:
    case CDC_RX_INP_MUX_RX_INT1_CFG1:
    case CDC_RX_INP_MUX_RX_INT2_CFG0:
    case CDC_RX_INP_MUX_RX_INT2_CFG1:
    case CDC_RX_INP_MUX_RX_MIX_CFG4:
    case CDC_RX_INP_MUX_RX_MIX_CFG5:
    case CDC_RX_INP_MUX_SIDETONE_SRC_CFG0:
    case CDC_RX_CLSH_CRC:
    case CDC_RX_CLSH_DLY_CTRL:
    case CDC_RX_CLSH_DECAY_CTRL:
    case CDC_RX_CLSH_HPH_V_PA:
    case CDC_RX_CLSH_EAR_V_PA:
    case CDC_RX_CLSH_HPH_V_HD:
    case CDC_RX_CLSH_EAR_V_HD:
    case CDC_RX_CLSH_K1_MSB:
    case CDC_RX_CLSH_K1_LSB:
    case CDC_RX_CLSH_K2_MSB:
    case CDC_RX_CLSH_K2_LSB:
    case CDC_RX_CLSH_IDLE_CTRL:
    case CDC_RX_CLSH_IDLE_HPH:
    case CDC_RX_CLSH_IDLE_EAR:
    case CDC_RX_CLSH_TEST0:
    case CDC_RX_CLSH_TEST1:
    case CDC_RX_CLSH_OVR_VREF:
    case CDC_RX_CLSH_CLSG_CTL:
    case CDC_RX_CLSH_CLSG_CFG1:
    case CDC_RX_CLSH_CLSG_CFG2:
    case CDC_RX_BCL_VBAT_PATH_CTL:
    case CDC_RX_BCL_VBAT_CFG:
    case CDC_RX_BCL_VBAT_ADC_CAL1:
    case CDC_RX_BCL_VBAT_ADC_CAL2:
    case CDC_RX_BCL_VBAT_ADC_CAL3:
    case CDC_RX_BCL_VBAT_PK_EST1:
    case CDC_RX_BCL_VBAT_PK_EST2:
    case CDC_RX_BCL_VBAT_PK_EST3:
    case CDC_RX_BCL_VBAT_RF_PROC1:
    case CDC_RX_BCL_VBAT_RF_PROC2:
    case CDC_RX_BCL_VBAT_TAC1:
    case CDC_RX_BCL_VBAT_TAC2:
    case CDC_RX_BCL_VBAT_TAC3:
    case CDC_RX_BCL_VBAT_TAC4:
    case CDC_RX_BCL_VBAT_GAIN_UPD1:
    case CDC_RX_BCL_VBAT_GAIN_UPD2:
    case CDC_RX_BCL_VBAT_GAIN_UPD3:
    case CDC_RX_BCL_VBAT_GAIN_UPD4:
    case CDC_RX_BCL_VBAT_GAIN_UPD5:
    case CDC_RX_BCL_VBAT_DEBUG1:
    case CDC_RX_BCL_VBAT_BAN:
    case CDC_RX_BCL_VBAT_BCL_GAIN_UPD1:
    case CDC_RX_BCL_VBAT_BCL_GAIN_UPD2:
    case CDC_RX_BCL_VBAT_BCL_GAIN_UPD3:
    case CDC_RX_BCL_VBAT_BCL_GAIN_UPD4:
    case CDC_RX_BCL_VBAT_BCL_GAIN_UPD5:
    case CDC_RX_BCL_VBAT_BCL_GAIN_UPD6:
    case CDC_RX_BCL_VBAT_BCL_GAIN_UPD7:
    case CDC_RX_BCL_VBAT_BCL_GAIN_UPD8:
    case CDC_RX_BCL_VBAT_BCL_GAIN_UPD9:
    case CDC_RX_BCL_VBAT_ATTN1:
    case CDC_RX_BCL_VBAT_ATTN2:
    case CDC_RX_BCL_VBAT_ATTN3:
    case CDC_RX_BCL_VBAT_DECODE_CTL1:
    case CDC_RX_BCL_VBAT_DECODE_CTL2:
    case CDC_RX_BCL_VBAT_DECODE_CFG1:
    case CDC_RX_BCL_VBAT_DECODE_CFG2:
    case CDC_RX_BCL_VBAT_DECODE_CFG3:
    case CDC_RX_BCL_VBAT_DECODE_CFG4:
    case CDC_RX_INTR_CTRL_CFG:
    case CDC_RX_INTR_CTRL_PIN1_MASK0:
    case CDC_RX_INTR_CTRL_PIN2_MASK0:
    case CDC_RX_INTR_CTRL_LEVEL0:
    case CDC_RX_INTR_CTRL_BYPASS0:
    case CDC_RX_INTR_CTRL_SET0:
    case CDC_RX_RX0_RX_PATH_CTL:
    case CDC_RX_RX0_RX_PATH_CFG0:
    case CDC_RX_RX0_RX_PATH_CFG1:
    case CDC_RX_RX0_RX_PATH_CFG2:
    case CDC_RX_RX0_RX_PATH_CFG3:
    case CDC_RX_RX0_RX_VOL_CTL:
    case CDC_RX_RX0_RX_PATH_MIX_CTL:
    case CDC_RX_RX0_RX_PATH_MIX_CFG:
    case CDC_RX_RX0_RX_VOL_MIX_CTL:
    case CDC_RX_RX0_RX_PATH_SEC1:
    case CDC_RX_RX0_RX_PATH_SEC2:
    case CDC_RX_RX0_RX_PATH_SEC3:
    case CDC_RX_RX0_RX_PATH_SEC4:
    case CDC_RX_RX0_RX_PATH_SEC7:
    case CDC_RX_RX0_RX_PATH_MIX_SEC0:
    case CDC_RX_RX0_RX_PATH_MIX_SEC1:
    case CDC_RX_RX0_RX_PATH_DSM_CTL:
    case CDC_RX_RX0_RX_PATH_DSM_DATA1:
    case CDC_RX_RX0_RX_PATH_DSM_DATA2:
    case CDC_RX_RX0_RX_PATH_DSM_DATA3:
    case CDC_RX_RX0_RX_PATH_DSM_DATA4:
    case CDC_RX_RX0_RX_PATH_DSM_DATA5:
    case CDC_RX_RX0_RX_PATH_DSM_DATA6:
    case CDC_RX_IDLE_DETECT_PATH_CTL:
    case CDC_RX_IDLE_DETECT_CFG0:
    case CDC_RX_IDLE_DETECT_CFG1:
    case CDC_RX_IDLE_DETECT_CFG2:
    case CDC_RX_IDLE_DETECT_CFG3:
    case CDC_RX_COMPANDER0_CTL0:
    case CDC_RX_COMPANDER0_CTL1:
    case CDC_RX_COMPANDER0_CTL2:
    case CDC_RX_COMPANDER0_CTL3:
    case CDC_RX_COMPANDER0_CTL4:
    case CDC_RX_COMPANDER0_CTL5:
    case CDC_RX_COMPANDER0_CTL7:
    case CDC_RX_COMPANDER1_CTL0:
    case CDC_RX_COMPANDER1_CTL1:
    case CDC_RX_COMPANDER1_CTL2:
    case CDC_RX_COMPANDER1_CTL3:
    case CDC_RX_COMPANDER1_CTL4:
    case CDC_RX_COMPANDER1_CTL5:
    case CDC_RX_COMPANDER1_CTL7:
    case CDC_RX_SIDETONE_IIR0_IIR_PATH_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_GAIN_B1_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_GAIN_B2_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_GAIN_B3_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_GAIN_B4_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_GAIN_B5_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_GAIN_B6_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_GAIN_B7_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_GAIN_B8_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_GAIN_TIMER_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_COEF_B1_CTL:
    case CDC_RX_SIDETONE_IIR0_IIR_COEF_B2_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_PATH_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_GAIN_B1_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_GAIN_B2_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_GAIN_B3_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_GAIN_B4_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_GAIN_B5_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_GAIN_B6_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_GAIN_B7_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_GAIN_B8_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_GAIN_TIMER_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_COEF_B1_CTL:
    case CDC_RX_SIDETONE_IIR1_IIR_COEF_B2_CTL:
    case CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG0:
    case CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG1:
    case CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG2:
    case CDC_RX_IIR_INP_MUX_IIR0_MIX_CFG3:
    case CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG0:
    case CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG1:
    case CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG2:
    case CDC_RX_IIR_INP_MUX_IIR1_MIX_CFG3:
    case CDC_RX_SIDETONE_SRC0_ST_SRC_PATH_CTL:
    case CDC_RX_SIDETONE_SRC0_ST_SRC_PATH_CFG1:
    case CDC_RX_SIDETONE_SRC1_ST_SRC_PATH_CTL:
    case CDC_RX_SIDETONE_SRC1_ST_SRC_PATH_CFG1:
    case CDC_RX_EC_REF_HQ0_EC_REF_HQ_PATH_CTL:
    case CDC_RX_EC_REF_HQ0_EC_REF_HQ_CFG0:
    case CDC_RX_EC_REF_HQ1_EC_REF_HQ_PATH_CTL:
    case CDC_RX_EC_REF_HQ1_EC_REF_HQ_CFG0:
    case CDC_RX_EC_REF_HQ2_EC_REF_HQ_PATH_CTL:
    case CDC_RX_EC_REF_HQ2_EC_REF_HQ_CFG0:
    case CDC_RX_EC_ASRC0_CLK_RST_CTL:
    case CDC_RX_EC_ASRC0_CTL0:
    case CDC_RX_EC_ASRC0_CTL1:
    case CDC_RX_EC_ASRC0_FIFO_CTL:
    case CDC_RX_EC_ASRC1_CLK_RST_CTL:
    case CDC_RX_EC_ASRC1_CTL0:
    case CDC_RX_EC_ASRC1_CTL1:
    case CDC_RX_EC_ASRC1_FIFO_CTL:
    case CDC_RX_EC_ASRC2_CLK_RST_CTL:
    case CDC_RX_EC_ASRC2_CTL0:
    case CDC_RX_EC_ASRC2_CTL1:
    case CDC_RX_EC_ASRC2_FIFO_CTL:
    case CDC_RX_DSD0_PATH_CTL:
    case CDC_RX_DSD0_CFG0:
    case CDC_RX_DSD0_CFG1:
    case CDC_RX_DSD0_CFG2:
    case CDC_RX_DSD1_PATH_CTL:
    case CDC_RX_DSD1_CFG0:
    case CDC_RX_DSD1_CFG1:
    case CDC_RX_DSD1_CFG2:
    return true;
    }
    switch (rx.codec_version) {
    case LPASS_CODEC_VERSION_1_0:
    case LPASS_CODEC_VERSION_1_1:
    case LPASS_CODEC_VERSION_1_2:
    case LPASS_CODEC_VERSION_2_0:
    case LPASS_CODEC_VERSION_2_1:
    return rx_pre_2_5_is_rw_register(dev, reg);
    case LPASS_CODEC_VERSION_2_5:
    case LPASS_CODEC_VERSION_2_6:
    case LPASS_CODEC_VERSION_2_7:
    case LPASS_CODEC_VERSION_2_8:
    return rx_2_5_is_rw_register(dev, reg);
    default:
    break;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn rx_is_writeable_register(dev: *mut device, reg: c_uint) -> bool {
    static bool rx_is_writeable_register(struct device *dev, unsigned int reg)
    {
    bool ret;
    ret = rx_is_rw_register(dev, reg);
    if (!ret)
    return rx_is_wronly_register(dev, reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rx_is_readable_register(dev: *mut device, reg: c_uint) -> bool {
    static bool rx_is_readable_register(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case CDC_RX_TOP_HPHL_COMP_RD_LSB:
    case CDC_RX_TOP_HPHL_COMP_RD_MSB:
    case CDC_RX_TOP_HPHR_COMP_RD_LSB:
    case CDC_RX_TOP_HPHR_COMP_RD_MSB:
    case CDC_RX_TOP_DSD0_DEBUG_CFG2:
    case CDC_RX_TOP_DSD1_DEBUG_CFG2:
    case CDC_RX_BCL_VBAT_GAIN_MON_VAL:
    case CDC_RX_BCL_VBAT_DECODE_ST:
    case CDC_RX_INTR_CTRL_PIN1_STATUS0:
    case CDC_RX_INTR_CTRL_PIN2_STATUS0:
    case CDC_RX_COMPANDER0_CTL6:
    case CDC_RX_COMPANDER1_CTL6:
    case CDC_RX_EC_ASRC0_STATUS_FMIN_CNTR_LSB:
    case CDC_RX_EC_ASRC0_STATUS_FMIN_CNTR_MSB:
    case CDC_RX_EC_ASRC0_STATUS_FMAX_CNTR_LSB:
    case CDC_RX_EC_ASRC0_STATUS_FMAX_CNTR_MSB:
    case CDC_RX_EC_ASRC0_STATUS_FIFO:
    case CDC_RX_EC_ASRC1_STATUS_FMIN_CNTR_LSB:
    case CDC_RX_EC_ASRC1_STATUS_FMIN_CNTR_MSB:
    case CDC_RX_EC_ASRC1_STATUS_FMAX_CNTR_LSB:
    case CDC_RX_EC_ASRC1_STATUS_FMAX_CNTR_MSB:
    case CDC_RX_EC_ASRC1_STATUS_FIFO:
    case CDC_RX_EC_ASRC2_STATUS_FMIN_CNTR_LSB:
    case CDC_RX_EC_ASRC2_STATUS_FMIN_CNTR_MSB:
    case CDC_RX_EC_ASRC2_STATUS_FMAX_CNTR_LSB:
    case CDC_RX_EC_ASRC2_STATUS_FMAX_CNTR_MSB:
    case CDC_RX_EC_ASRC2_STATUS_FIFO:
    return true;
    }
    return rx_is_rw_register(dev, reg);
    }
    static const struct regmap_config rx_regmap_config = {
    .name = "rx_macro",
    .reg_bits = 16,
    .val_bits = 32, /* 8 but with 32 bit read/write */
    .reg_stride = 4,
    .cache_type = REGCACHE_FLAT,
    .max_register = RX_MAX_OFFSET,
    .writeable_reg = rx_is_writeable_register,
    .volatile_reg = rx_is_volatile_register,
    .readable_reg = rx_is_readable_register,
    };
    static int rx_macro_int_dem_inp_mux_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_dapm_widget *widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    struct snd_soc_component *component = snd_soc_dapm_to_component(widget.dapm);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    struct soc_enum *e = (struct soc_enum *)kcontrol.private_value;
    unsigned short look_ahead_dly_reg;
    unsigned int val;
    val = ucontrol.value.enumerated.item[0];
    if (e.reg == CDC_RX_RXn_RX_PATH_CFG1(rx, 0))
    look_ahead_dly_reg = CDC_RX_RXn_RX_PATH_CFG0(rx, 0);
#[no_mangle]
pub unsafe extern "C" fn if(CDC_RX_RXn_RX_PATH_CFG1(rx: e->reg ==, _arg: 1)) -> else {
    else if (e.reg == CDC_RX_RXn_RX_PATH_CFG1(rx, 1))
    look_ahead_dly_reg = CDC_RX_RXn_RX_PATH_CFG0(rx, 1);
// Set Look Ahead Delay
    if (val)
    snd_soc_component_update_bits(component, look_ahead_dly_reg,
    CDC_RX_DLY_ZN_EN_MASK,
    CDC_RX_DLY_ZN_ENABLE);
    else
    snd_soc_component_update_bits(component, look_ahead_dly_reg,
    CDC_RX_DLY_ZN_EN_MASK, 0);
// Set DEM INP Select
    return snd_soc_dapm_put_enum_double(kcontrol, ucontrol);
    }
    static const struct snd_kcontrol_new rx_int0_dem_inp_mux =
    SOC_DAPM_ENUM_EXT("rx_int0_dem_inp", rx_int0_dem_inp_enum,
    snd_soc_dapm_get_enum_double, rx_macro_int_dem_inp_mux_put);
    static const struct snd_kcontrol_new rx_int1_dem_inp_mux =
    SOC_DAPM_ENUM_EXT("rx_int1_dem_inp", rx_int1_dem_inp_enum,
    snd_soc_dapm_get_enum_double, rx_macro_int_dem_inp_mux_put);
    static const struct snd_kcontrol_new rx_2_5_int1_dem_inp_mux =
    SOC_DAPM_ENUM_EXT("rx_int1_dem_inp", rx_2_5_int1_dem_inp_enum,
    snd_soc_dapm_get_enum_double, rx_macro_int_dem_inp_mux_put);
    static int rx_macro_set_prim_interpolator_rate(struct snd_soc_dai *dai,
    int rate_reg_val, u32 sample_rate)
    {
    u8 int_1_mix1_inp;
    u32 j, port;
    u16 int_mux_cfg0, int_mux_cfg1;
    u16 int_fs_reg;
    u8 inp0_sel, inp1_sel, inp2_sel;
    struct snd_soc_component *component = dai.component;
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    for_each_set_bit(port, &rx.active_ch_mask[dai.id], RX_MACRO_PORTS_MAX) {
    int_1_mix1_inp = port;
    int_mux_cfg0 = CDC_RX_INP_MUX_RX_INT0_CFG0;
//
// Loop through all interpolator MUX inputs and find out
// to which interpolator input, the rx port
// is connected
//
    for (j = 0; j < INTERP_MAX; j++) {
    int_mux_cfg1 = int_mux_cfg0 + 4;
    inp0_sel = snd_soc_component_read_field(component, int_mux_cfg0,
    CDC_RX_INTX_1_MIX_INP0_SEL_MASK);
    inp1_sel = snd_soc_component_read_field(component, int_mux_cfg0,
    CDC_RX_INTX_1_MIX_INP1_SEL_MASK);
    inp2_sel = snd_soc_component_read_field(component, int_mux_cfg1,
    CDC_RX_INTX_1_MIX_INP2_SEL_MASK);
    if ((inp0_sel == int_1_mix1_inp + INTn_1_INP_SEL_RX0) ||
    (inp1_sel == int_1_mix1_inp + INTn_1_INP_SEL_RX0) ||
    (inp2_sel == int_1_mix1_inp + INTn_1_INP_SEL_RX0)) {
    int_fs_reg = CDC_RX_RXn_RX_PATH_CTL(rx, j);
// sample_rate is in Hz
    snd_soc_component_update_bits(component, int_fs_reg,
    CDC_RX_PATH_PCM_RATE_MASK,
    rate_reg_val);
    }
    int_mux_cfg0 += 8;
    }
    }
    return 0;
    }
    static int rx_macro_set_mix_interpolator_rate(struct snd_soc_dai *dai,
    int rate_reg_val, u32 sample_rate)
    {
    u8 int_2_inp;
    u32 j, port;
    u16 int_mux_cfg1, int_fs_reg;
    u8 int_mux_cfg1_val;
    struct snd_soc_component *component = dai.component;
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    for_each_set_bit(port, &rx.active_ch_mask[dai.id], RX_MACRO_PORTS_MAX) {
    int_2_inp = port;
    int_mux_cfg1 = CDC_RX_INP_MUX_RX_INT0_CFG1;
    for (j = 0; j < INTERP_MAX; j++) {
    int_mux_cfg1_val = snd_soc_component_read_field(component, int_mux_cfg1,
    CDC_RX_INTX_2_SEL_MASK);
    if (int_mux_cfg1_val == int_2_inp + INTn_2_INP_SEL_RX0) {
    int_fs_reg = CDC_RX_RXn_RX_PATH_MIX_CTL(rx, j);
    snd_soc_component_update_bits(component, int_fs_reg,
    CDC_RX_RXn_MIX_PCM_RATE_MASK,
    rate_reg_val);
    }
    int_mux_cfg1 += 8;
    }
    }
    return 0;
    }
    static int rx_macro_set_interpolator_rate(struct snd_soc_dai *dai,
    u32 sample_rate)
    {
    let mut rate_val: c_int = 0;
    int i, ret;
    for (i = 0; i < ARRAY_SIZE(sr_val_tbl); i++)
    if (sample_rate == sr_val_tbl[i].sample_rate)
    rate_val = sr_val_tbl[i].rate_val;
    ret = rx_macro_set_prim_interpolator_rate(dai, rate_val, sample_rate);
    if (ret)
    return ret;
    ret = rx_macro_set_mix_interpolator_rate(dai, rate_val, sample_rate);
    return ret;
    }
    static int rx_macro_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    int ret;
    switch (substream.stream) {
    case SNDRV_PCM_STREAM_PLAYBACK:
    ret = rx_macro_set_interpolator_rate(dai, params_rate(params));
    if (ret) {
    dev_err(component.dev, "%s: cannot set sample rate: %u\n",
    __func__, params_rate(params));
    return ret;
    }
    rx.bit_width[dai.id] = params_width(params);
    break;
    default:
    break;
    }
    return 0;
    }
    static int rx_macro_get_channel_map(const struct snd_soc_dai *dai,
    unsigned int *tx_num, unsigned int *tx_slot,
    unsigned int *rx_num, unsigned int *rx_slot)
    {
    struct snd_soc_component *component = dai.component;
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    u16 val, mask = 0, cnt = 0, temp;
    switch (dai.id) {
    case RX_MACRO_AIF1_PB:
    case RX_MACRO_AIF2_PB:
    case RX_MACRO_AIF3_PB:
    case RX_MACRO_AIF4_PB:
    for_each_set_bit(temp, &rx.active_ch_mask[dai.id],
    RX_MACRO_PORTS_MAX) {
    mask |= (1 << temp);
    if (++cnt == RX_MACRO_MAX_DMA_CH_PER_PORT)
    break;
    }
//
// CDC_DMA_RX_0 port drives RX0/RX1 -- ch_mask 0x1/0x2/0x3
// CDC_DMA_RX_1 port drives RX2/RX3 -- ch_mask 0x1/0x2/0x3
// CDC_DMA_RX_2 port drives RX4     -- ch_mask 0x1
// CDC_DMA_RX_3 port drives RX5     -- ch_mask 0x1
// AIFn can pair to any CDC_DMA_RX_n port.
// In general, below convention is used::
// CDC_DMA_RX_0(AIF1)/CDC_DMA_RX_1(AIF2)
// CDC_DMA_RX_2(AIF3)/CDC_DMA_RX_3(AIF4)
//
    if (mask & 0x0C)
    mask = mask >> 2;
    if ((mask & 0x10) || (mask & 0x20))
    mask = 0x1;
// rx_slot = mask;
// rx_num = rx->active_ch_cnt[dai->id];
    break;
    case RX_MACRO_AIF_ECHO:
    val = snd_soc_component_read(component,	CDC_RX_INP_MUX_RX_MIX_CFG4);
    if (val & RX_MACRO_EC_MIX_TX0_MASK) {
    mask |= 0x1;
    cnt++;
    }
    if (val & RX_MACRO_EC_MIX_TX1_MASK) {
    mask |= 0x2;
    cnt++;
    }
    val = snd_soc_component_read(component,
    CDC_RX_INP_MUX_RX_MIX_CFG5);
    if (val & RX_MACRO_EC_MIX_TX2_MASK) {
    mask |= 0x4;
    cnt++;
    }
// tx_slot = mask;
// tx_num = cnt;
    break;
    default:
    dev_err(component.dev, "%s: Invalid AIF\n", __func__);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rx_macro_digital_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    static int rx_macro_digital_mute(struct snd_soc_dai *dai, int mute, int stream)
    {
    struct snd_soc_component *component = dai.component;
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    u32 port, j, reg, mix_reg, int_mux_cfg0, int_mux_cfg1;
    u32 mask, val;
    u8 int_mux_cfg0_val, int_mux_cfg1_val;
    if (stream != SNDRV_PCM_STREAM_PLAYBACK)
    return 0;
    for (j = 0; j < INTERP_MAX; j++) {
    reg = CDC_RX_RXn_RX_PATH_CTL(rx, j);
    mix_reg = CDC_RX_RXn_RX_PATH_MIX_CTL(rx, j);
    mask = CDC_RX_PATH_PGA_MUTE_MASK;
    val = 0;
    if (mute)
    val |= CDC_RX_PATH_PGA_MUTE_ENABLE;
    if (rx.main_clk_users[j] > 0) {
    mask |= CDC_RX_PATH_CLK_EN_MASK;
    val |= CDC_RX_PATH_CLK_ENABLE;
    }
    int_mux_cfg0 = CDC_RX_INP_MUX_RX_INT0_CFG0 + j * 8;
    int_mux_cfg1 = int_mux_cfg0 + 4;
    int_mux_cfg0_val = snd_soc_component_read(component, int_mux_cfg0);
    int_mux_cfg1_val = snd_soc_component_read(component, int_mux_cfg1);
    for_each_set_bit(port, &rx.active_ch_mask[dai.id], RX_MACRO_PORTS_MAX) {
    if (((int_mux_cfg0_val & 0x0f) == port + INTn_1_INP_SEL_RX0) ||
    ((int_mux_cfg0_val >> 4) == port + INTn_1_INP_SEL_RX0) ||
    ((int_mux_cfg1_val >> 4) == port + INTn_1_INP_SEL_RX0)) {
    snd_soc_component_update_bits(component, reg, mask, val);
    }
    if ((int_mux_cfg1_val & 0x0f) == port + INTn_2_INP_SEL_RX0) {
    snd_soc_component_update_bits(component, mix_reg, mask, val);
// main clock needs to be enabled for mix to be useful:
    if (rx.main_clk_users[j] > 0) {
    snd_soc_component_update_bits(component, reg,
    CDC_RX_PATH_CLK_EN_MASK,
    CDC_RX_PATH_CLK_ENABLE);
    }
    }
    }
    }
    return 0;
    }
    static const struct snd_soc_dai_ops rx_macro_dai_ops = {
    .hw_params = rx_macro_hw_params,
    .get_channel_map = rx_macro_get_channel_map,
    .mute_stream = rx_macro_digital_mute,
    };
    static struct snd_soc_dai_driver rx_macro_dai[] = {
    {
    .name = "rx_macro_rx1",
    .id = RX_MACRO_AIF1_PB,
    .playback = {
    .stream_name = "RX_MACRO_AIF1 Playback",
    .rates = RX_MACRO_RATES | RX_MACRO_FRAC_RATES,
    .formats = RX_MACRO_FORMATS,
    .rate_max = 384000,
    .rate_min = 8000,
    .channels_min = 1,
    .channels_max = 2,
    },
    .ops = &rx_macro_dai_ops,
    },
    {
    .name = "rx_macro_rx2",
    .id = RX_MACRO_AIF2_PB,
    .playback = {
    .stream_name = "RX_MACRO_AIF2 Playback",
    .rates = RX_MACRO_RATES | RX_MACRO_FRAC_RATES,
    .formats = RX_MACRO_FORMATS,
    .rate_max = 384000,
    .rate_min = 8000,
    .channels_min = 1,
    .channels_max = 2,
    },
    .ops = &rx_macro_dai_ops,
    },
    {
    .name = "rx_macro_rx3",
    .id = RX_MACRO_AIF3_PB,
    .playback = {
    .stream_name = "RX_MACRO_AIF3 Playback",
    .rates = RX_MACRO_RATES | RX_MACRO_FRAC_RATES,
    .formats = RX_MACRO_FORMATS,
    .rate_max = 384000,
    .rate_min = 8000,
    .channels_min = 1,
    .channels_max = 2,
    },
    .ops = &rx_macro_dai_ops,
    },
    {
    .name = "rx_macro_rx4",
    .id = RX_MACRO_AIF4_PB,
    .playback = {
    .stream_name = "RX_MACRO_AIF4 Playback",
    .rates = RX_MACRO_RATES | RX_MACRO_FRAC_RATES,
    .formats = RX_MACRO_FORMATS,
    .rate_max = 384000,
    .rate_min = 8000,
    .channels_min = 1,
    .channels_max = 2,
    },
    .ops = &rx_macro_dai_ops,
    },
    {
    .name = "rx_macro_echo",
    .id = RX_MACRO_AIF_ECHO,
    .capture = {
    .stream_name = "RX_AIF_ECHO Capture",
    .rates = RX_MACRO_ECHO_RATES,
    .formats = RX_MACRO_ECHO_FORMATS,
    .rate_max = 48000,
    .rate_min = 8000,
    .channels_min = 1,
    .channels_max = 3,
    },
    .ops = &rx_macro_dai_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn rx_macro_mclk_enable(rx: *mut rx_macro, mclk_enable: bool) -> c_int {
    static int rx_macro_mclk_enable(struct rx_macro *rx, bool mclk_enable)
    {
    struct regmap *regmap = rx.regmap;
    int ret;
    if (mclk_enable) {
    if (rx.rx_mclk_users == 0) {
    regmap_update_bits(regmap, CDC_RX_CLK_RST_CTRL_MCLK_CONTROL,
    CDC_RX_CLK_MCLK_EN_MASK |
    CDC_RX_CLK_MCLK2_EN_MASK,
    CDC_RX_CLK_MCLK_ENABLE |
    CDC_RX_CLK_MCLK2_ENABLE);
    regmap_update_bits(regmap, CDC_RX_CLK_RST_CTRL_FS_CNT_CONTROL,
    CDC_RX_FS_MCLK_CNT_CLR_MASK, 0x00);
    regmap_update_bits(regmap, CDC_RX_CLK_RST_CTRL_FS_CNT_CONTROL,
    CDC_RX_FS_MCLK_CNT_EN_MASK,
    CDC_RX_FS_MCLK_CNT_ENABLE);
    regcache_mark_dirty(regmap);
    ret = regcache_sync(regmap);
    if (ret)
    return ret;
    }
    rx.rx_mclk_users++;
    } else {
    if (rx.rx_mclk_users <= 0) {
    dev_err(rx.dev, "%s: clock already disabled\n", __func__);
    rx.rx_mclk_users = 0;
    return 0;
    }
    rx.rx_mclk_users--;
    if (rx.rx_mclk_users == 0) {
    regmap_update_bits(regmap, CDC_RX_CLK_RST_CTRL_FS_CNT_CONTROL,
    CDC_RX_FS_MCLK_CNT_EN_MASK, 0x0);
    regmap_update_bits(regmap, CDC_RX_CLK_RST_CTRL_FS_CNT_CONTROL,
    CDC_RX_FS_MCLK_CNT_CLR_MASK,
    CDC_RX_FS_MCLK_CNT_CLR);
    regmap_update_bits(regmap, CDC_RX_CLK_RST_CTRL_MCLK_CONTROL,
    CDC_RX_CLK_MCLK_EN_MASK |
    CDC_RX_CLK_MCLK2_EN_MASK, 0x0);
    }
    }
    return 0;
    }
    static int rx_macro_mclk_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    let mut ret: c_int = 0;
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    return rx_macro_mclk_enable(rx, true);
    case SND_SOC_DAPM_POST_PMD:
    return rx_macro_mclk_enable(rx, false);
    default:
    dev_err(component.dev, "%s: invalid DAPM event %d\n", __func__, event);
    ret = -EINVAL;
    }
    return ret;
    }
    static bool rx_macro_adie_lb(struct snd_soc_component *component,
    int interp_idx)
    {
    u16 int_mux_cfg0, int_mux_cfg1;
    u8 int_n_inp0, int_n_inp1, int_n_inp2;
    int_mux_cfg0 = CDC_RX_INP_MUX_RX_INT0_CFG0 + interp_idx * 8;
    int_mux_cfg1 = int_mux_cfg0 + 4;
    int_n_inp0 = snd_soc_component_read_field(component, int_mux_cfg0,
    CDC_RX_INTX_1_MIX_INP0_SEL_MASK);
    int_n_inp1 = snd_soc_component_read_field(component, int_mux_cfg0,
    CDC_RX_INTX_1_MIX_INP1_SEL_MASK);
    int_n_inp2 = snd_soc_component_read_field(component, int_mux_cfg1,
    CDC_RX_INTX_1_MIX_INP2_SEL_MASK);
    if (int_n_inp0 == INTn_1_INP_SEL_DEC0 ||
    int_n_inp0 == INTn_1_INP_SEL_DEC1 ||
    int_n_inp0 == INTn_1_INP_SEL_IIR0 ||
    int_n_inp0 == INTn_1_INP_SEL_IIR1)
    return true;
    if (int_n_inp1 == INTn_1_INP_SEL_DEC0 ||
    int_n_inp1 == INTn_1_INP_SEL_DEC1 ||
    int_n_inp1 == INTn_1_INP_SEL_IIR0 ||
    int_n_inp1 == INTn_1_INP_SEL_IIR1)
    return true;
    if (int_n_inp2 == INTn_1_INP_SEL_DEC0 ||
    int_n_inp2 == INTn_1_INP_SEL_DEC1 ||
    int_n_inp2 == INTn_1_INP_SEL_IIR0 ||
    int_n_inp2 == INTn_1_INP_SEL_IIR1)
    return true;
    return false;
    }
    static int rx_macro_enable_interp_clk(struct snd_soc_component *component,
    int event, int interp_idx);
    static int rx_macro_enable_main_path(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    u16 gain_reg, reg;
    reg = CDC_RX_RXn_RX_PATH_CTL(rx, w.shift);
    gain_reg = CDC_RX_RXn_RX_VOL_CTL(rx, w.shift);
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    rx_macro_enable_interp_clk(component, event, w.shift);
    if (rx_macro_adie_lb(component, w.shift))
    snd_soc_component_update_bits(component, reg,
    CDC_RX_PATH_CLK_EN_MASK,
    CDC_RX_PATH_CLK_ENABLE);
    break;
    case SND_SOC_DAPM_POST_PMU:
    snd_soc_component_write(component, gain_reg,
    snd_soc_component_read(component, gain_reg));
    break;
    case SND_SOC_DAPM_POST_PMD:
    rx_macro_enable_interp_clk(component, event, w.shift);
    break;
    }
    return 0;
    }
    static int rx_macro_config_compander(struct snd_soc_component *component,
    struct rx_macro *rx,
    int comp, int event)
    {
    u8 pcm_rate, val;
// AUX does not have compander
    if (comp == INTERP_AUX)
    return 0;
    pcm_rate = snd_soc_component_read(component, CDC_RX_RXn_RX_PATH_CTL(rx, comp)) & 0x0F;
    if (pcm_rate < 0x06)
    val = 0x03;
#[no_mangle]
pub unsafe extern "C" fn if(0x08: pcm_rate <) -> else {
    else if (pcm_rate < 0x08)
    val = 0x01;
#[no_mangle]
pub unsafe extern "C" fn if(0x0B: pcm_rate <) -> else {
    else if (pcm_rate < 0x0B)
    val = 0x02;
    else
    val = 0x00;
    if (SND_SOC_DAPM_EVENT_ON(event))
    snd_soc_component_update_bits(component, CDC_RX_RXn_RX_PATH_CFG3(rx, comp),
    CDC_RX_DC_COEFF_SEL_MASK, val);
    if (SND_SOC_DAPM_EVENT_OFF(event))
    snd_soc_component_update_bits(component, CDC_RX_RXn_RX_PATH_CFG3(rx, comp),
    CDC_RX_DC_COEFF_SEL_MASK, 0x3);
    if (!rx.comp_enabled[comp])
    return 0;
    if (SND_SOC_DAPM_EVENT_ON(event)) {
// Enable Compander Clock
    snd_soc_component_write_field(component, CDC_RX_COMPANDERn_CTL0(comp),
    CDC_RX_COMPANDERn_CLK_EN_MASK, 0x1);
    snd_soc_component_write_field(component, CDC_RX_COMPANDERn_CTL0(comp),
    CDC_RX_COMPANDERn_SOFT_RST_MASK, 0x1);
    snd_soc_component_write_field(component, CDC_RX_COMPANDERn_CTL0(comp),
    CDC_RX_COMPANDERn_SOFT_RST_MASK, 0x0);
    snd_soc_component_write_field(component, CDC_RX_RXn_RX_PATH_CFG0(rx, comp),
    CDC_RX_RXn_COMP_EN_MASK, 0x1);
    }
    if (SND_SOC_DAPM_EVENT_OFF(event)) {
    snd_soc_component_write_field(component, CDC_RX_COMPANDERn_CTL0(comp),
    CDC_RX_COMPANDERn_HALT_MASK, 0x1);
    snd_soc_component_write_field(component, CDC_RX_RXn_RX_PATH_CFG0(rx, comp),
    CDC_RX_RXn_COMP_EN_MASK, 0x0);
    snd_soc_component_write_field(component, CDC_RX_COMPANDERn_CTL0(comp),
    CDC_RX_COMPANDERn_CLK_EN_MASK, 0x0);
    snd_soc_component_write_field(component, CDC_RX_COMPANDERn_CTL0(comp),
    CDC_RX_COMPANDERn_HALT_MASK, 0x0);
    }
    return 0;
    }
    static int rx_macro_load_compander_coeff(struct snd_soc_component *component,
    struct rx_macro *rx,
    int comp, int event)
    {
    u16 comp_coeff_lsb_reg, comp_coeff_msb_reg;
    int i;
    int hph_pwr_mode;
// AUX does not have compander
    if (comp == INTERP_AUX)
    return 0;
    if (!rx.comp_enabled[comp])
    return 0;
    if (comp == INTERP_HPHL) {
    comp_coeff_lsb_reg = CDC_RX_TOP_HPHL_COMP_WR_LSB;
    comp_coeff_msb_reg = CDC_RX_TOP_HPHL_COMP_WR_MSB;
    } else if (comp == INTERP_HPHR) {
    comp_coeff_lsb_reg = CDC_RX_TOP_HPHR_COMP_WR_LSB;
    comp_coeff_msb_reg = CDC_RX_TOP_HPHR_COMP_WR_MSB;
    } else {
// compander coefficients are loaded only for hph path
    return 0;
    }
    hph_pwr_mode = rx.hph_pwr_mode;
    if (SND_SOC_DAPM_EVENT_ON(event)) {
// Load Compander Coeff
    for (i = 0; i < COMP_MAX_COEFF; i++) {
    snd_soc_component_write(component, comp_coeff_lsb_reg,
    comp_coeff_table[hph_pwr_mode][i].lsb);
    snd_soc_component_write(component, comp_coeff_msb_reg,
    comp_coeff_table[hph_pwr_mode][i].msb);
    }
    }
    return 0;
    }
    static void rx_macro_enable_softclip_clk(struct snd_soc_component *component,
    struct rx_macro *rx, bool enable)
    {
    if (enable) {
    if (rx.softclip_clk_users == 0)
    snd_soc_component_write_field(component, CDC_RX_SOFTCLIP_CRC,
    CDC_RX_SOFTCLIP_CLK_EN_MASK, 1);
    rx.softclip_clk_users++;
    } else {
    rx.softclip_clk_users--;
    if (rx.softclip_clk_users == 0)
    snd_soc_component_write_field(component, CDC_RX_SOFTCLIP_CRC,
    CDC_RX_SOFTCLIP_CLK_EN_MASK, 0);
    }
    }
    static int rx_macro_config_softclip(struct snd_soc_component *component,
    struct rx_macro *rx, int event)
    {
    if (!rx.is_softclip_on)
    return 0;
    if (SND_SOC_DAPM_EVENT_ON(event)) {
// Enable Softclip clock
    rx_macro_enable_softclip_clk(component, rx, true);
// Enable Softclip control
    snd_soc_component_write_field(component, CDC_RX_SOFTCLIP_SOFTCLIP_CTRL,
    CDC_RX_SOFTCLIP_EN_MASK, 0x01);
    }
    if (SND_SOC_DAPM_EVENT_OFF(event)) {
    snd_soc_component_write_field(component, CDC_RX_SOFTCLIP_SOFTCLIP_CTRL,
    CDC_RX_SOFTCLIP_EN_MASK, 0x0);
    rx_macro_enable_softclip_clk(component, rx, false);
    }
    return 0;
    }
    static int rx_macro_config_aux_hpf(struct snd_soc_component *component,
    struct rx_macro *rx, int event)
    {
    if (SND_SOC_DAPM_EVENT_ON(event)) {
// Update Aux HPF control
    if (!rx.is_aux_hpf_on)
    snd_soc_component_update_bits(component,
    CDC_RX_RXn_RX_PATH_CFG1(rx, 2), 0x04, 0x00);
    }
    if (SND_SOC_DAPM_EVENT_OFF(event)) {
// Reset to default (HPF=ON)
    snd_soc_component_update_bits(component,
    CDC_RX_RXn_RX_PATH_CFG1(rx, 2), 0x04, 0x04);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rx_macro_enable_clsh_block(rx: *mut rx_macro, enable: bool) {
    static inline void rx_macro_enable_clsh_block(struct rx_macro *rx, bool enable)
    {
    if ((enable && ++rx.clsh_users == 1) || (!enable && --rx.clsh_users == 0))
    snd_soc_component_update_bits(rx.component, CDC_RX_CLSH_CRC,
    CDC_RX_CLSH_CLK_EN_MASK, enable);
    if (rx.clsh_users < 0)
    rx.clsh_users = 0;
    }
    static int rx_macro_config_classh(struct snd_soc_component *component,
    struct rx_macro *rx,
    int interp_n, int event)
    {
    if (SND_SOC_DAPM_EVENT_OFF(event)) {
    rx_macro_enable_clsh_block(rx, false);
    return 0;
    }
    if (!SND_SOC_DAPM_EVENT_ON(event))
    return 0;
    rx_macro_enable_clsh_block(rx, true);
    if (interp_n == INTERP_HPHL ||
    interp_n == INTERP_HPHR) {
//
// These K1 values depend on the Headphone Impedance
// For now it is assumed to be 16 ohm
//
    snd_soc_component_write(component, CDC_RX_CLSH_K1_LSB, 0xc0);
    snd_soc_component_write_field(component, CDC_RX_CLSH_K1_MSB,
    CDC_RX_CLSH_K1_MSB_COEFF_MASK, 0);
    }
    switch (interp_n) {
    case INTERP_HPHL:
    if (rx.is_ear_mode_on)
    snd_soc_component_update_bits(component,
    CDC_RX_CLSH_HPH_V_PA,
    CDC_RX_CLSH_HPH_V_PA_MIN_MASK, 0x39);
    else
    snd_soc_component_update_bits(component,
    CDC_RX_CLSH_HPH_V_PA,
    CDC_RX_CLSH_HPH_V_PA_MIN_MASK, 0x1c);
    snd_soc_component_update_bits(component,
    CDC_RX_CLSH_DECAY_CTRL,
    CDC_RX_CLSH_DECAY_RATE_MASK, 0x0);
    snd_soc_component_write_field(component,
    CDC_RX_RXn_RX_PATH_CFG0(rx, 0),
    CDC_RX_RXn_CLSH_EN_MASK, 0x1);
    break;
    case INTERP_HPHR:
    if (rx.is_ear_mode_on)
    snd_soc_component_update_bits(component,
    CDC_RX_CLSH_HPH_V_PA,
    CDC_RX_CLSH_HPH_V_PA_MIN_MASK, 0x39);
    else
    snd_soc_component_update_bits(component,
    CDC_RX_CLSH_HPH_V_PA,
    CDC_RX_CLSH_HPH_V_PA_MIN_MASK, 0x1c);
    snd_soc_component_update_bits(component,
    CDC_RX_CLSH_DECAY_CTRL,
    CDC_RX_CLSH_DECAY_RATE_MASK, 0x0);
    snd_soc_component_write_field(component,
    CDC_RX_RXn_RX_PATH_CFG0(rx, 1),
    CDC_RX_RXn_CLSH_EN_MASK, 0x1);
    break;
    case INTERP_AUX:
    snd_soc_component_update_bits(component,
    CDC_RX_RXn_RX_PATH_CFG0(rx, 2),
    CDC_RX_RX2_DLY_Z_EN_MASK, 1);
    snd_soc_component_write_field(component,
    CDC_RX_RXn_RX_PATH_CFG0(rx, 2),
    CDC_RX_RX2_CLSH_EN_MASK, 1);
    break;
    }
    return 0;
    }
    static void rx_macro_hd2_control(struct snd_soc_component *component,
    u16 interp_idx, int event)
    {
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    u16 hd2_scale_reg, hd2_enable_reg;
    switch (interp_idx) {
    case INTERP_HPHL:
    hd2_scale_reg = CDC_RX_RXn_RX_PATH_SEC3(rx, 0);
    hd2_enable_reg = CDC_RX_RXn_RX_PATH_CFG0(rx, 0);
    break;
    case INTERP_HPHR:
    hd2_scale_reg = CDC_RX_RXn_RX_PATH_SEC3(rx, 1);
    hd2_enable_reg = CDC_RX_RXn_RX_PATH_CFG0(rx, 1);
    break;
    }
    if (hd2_enable_reg && SND_SOC_DAPM_EVENT_ON(event)) {
    snd_soc_component_update_bits(component, hd2_scale_reg,
    CDC_RX_RXn_HD2_ALPHA_MASK, 0x14);
    snd_soc_component_write_field(component, hd2_enable_reg,
    CDC_RX_RXn_HD2_EN_MASK, 1);
    }
    if (hd2_enable_reg && SND_SOC_DAPM_EVENT_OFF(event)) {
    snd_soc_component_write_field(component, hd2_enable_reg,
    CDC_RX_RXn_HD2_EN_MASK, 0);
    snd_soc_component_update_bits(component, hd2_scale_reg,
    CDC_RX_RXn_HD2_ALPHA_MASK, 0x0);
    }
    }
    static int rx_macro_get_compander(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    let mut comp: c_int = ((struct soc_mixer_control *) kcontrol.private_value).shift;
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    ucontrol.value.integer.value[0] = rx.comp_enabled[comp];
    return 0;
    }
    static int rx_macro_set_compander(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    let mut comp: c_int = ((struct soc_mixer_control *)  kcontrol.private_value).shift;
    let mut value: c_int = ucontrol.value.integer.value[0];
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    rx.comp_enabled[comp] = value;
    return 0;
    }
    static int rx_macro_mux_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_dapm_widget *widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    struct snd_soc_component *component = snd_soc_dapm_to_component(widget.dapm);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    ucontrol.value.enumerated.item[0] =
    rx.rx_port_value[widget.shift];
    return 0;
    }
    static int rx_macro_mux_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_dapm_widget *widget = snd_soc_dapm_kcontrol_to_widget(kcontrol);
    struct snd_soc_component *component = snd_soc_dapm_to_component(widget.dapm);
    struct soc_enum *e = (struct soc_enum *)kcontrol.private_value;
    struct snd_soc_dapm_update *update = core::ptr::null_mut();
    let mut rx_port_value: u32 = ucontrol.value.enumerated.item[0];
    unsigned int dai_id;
    u32 aif_rst;
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    aif_rst = rx.rx_port_value[widget.shift];
    if (!rx_port_value) {
    if (aif_rst == 0)
    return 0;
    if (aif_rst > RX_MACRO_AIF4_PB) {
    dev_err(component.dev, "%s: Invalid AIF reset\n", __func__);
    return 0;
    }
    }
    rx.rx_port_value[widget.shift] = rx_port_value;
    switch (rx_port_value) {
    case 0:
//
// active_ch_cnt and active_ch_mask use DAI IDs (RX_MACRO_MAX_DAIS).
// active_ch_cnt == 0 was tested in if() above.
//
    dai_id = aif_rst - 1;
    if (rx.active_ch_cnt[dai_id]) {
    clear_bit(widget.shift, &rx.active_ch_mask[dai_id]);
    rx.active_ch_cnt[dai_id]--;
    }
    break;
    case 1:
    case 2:
    case 3:
    case 4:
// active_ch_cnt and active_ch_mask use DAI IDs (WSA_MACRO_MAX_DAIS).
    dai_id = rx_port_value - 1;
    set_bit(widget.shift, &rx.active_ch_mask[dai_id]);
    rx.active_ch_cnt[dai_id]++;
    break;
    default:
    dev_err(component.dev,
    "%s:Invalid AIF_ID for RX_MACRO MUX %d\n",
    __func__, rx_port_value);
    goto err;
    }
    snd_soc_dapm_mux_update_power(widget.dapm, kcontrol,
    rx_port_value, e, update);
    return 0;
    err:
    return -EINVAL;
    }
    static const struct snd_kcontrol_new rx_macro_rx0_mux =
    SOC_DAPM_ENUM_EXT("rx_macro_rx0", rx_macro_rx0_enum,
    rx_macro_mux_get, rx_macro_mux_put);
    static const struct snd_kcontrol_new rx_macro_rx1_mux =
    SOC_DAPM_ENUM_EXT("rx_macro_rx1", rx_macro_rx1_enum,
    rx_macro_mux_get, rx_macro_mux_put);
    static const struct snd_kcontrol_new rx_macro_rx2_mux =
    SOC_DAPM_ENUM_EXT("rx_macro_rx2", rx_macro_rx2_enum,
    rx_macro_mux_get, rx_macro_mux_put);
    static const struct snd_kcontrol_new rx_macro_rx3_mux =
    SOC_DAPM_ENUM_EXT("rx_macro_rx3", rx_macro_rx3_enum,
    rx_macro_mux_get, rx_macro_mux_put);
    static const struct snd_kcontrol_new rx_macro_rx4_mux =
    SOC_DAPM_ENUM_EXT("rx_macro_rx4", rx_macro_rx4_enum,
    rx_macro_mux_get, rx_macro_mux_put);
    static const struct snd_kcontrol_new rx_macro_rx5_mux =
    SOC_DAPM_ENUM_EXT("rx_macro_rx5", rx_macro_rx5_enum,
    rx_macro_mux_get, rx_macro_mux_put);
    static int rx_macro_get_ear_mode(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    ucontrol.value.integer.value[0] = rx.is_ear_mode_on;
    return 0;
    }
    static int rx_macro_put_ear_mode(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    rx.is_ear_mode_on = (!ucontrol.value.integer.value[0] ? false : true);
    return 0;
    }
    static int rx_macro_get_hph_hd2_mode(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    ucontrol.value.integer.value[0] = rx.hph_hd2_mode;
    return 0;
    }
    static int rx_macro_put_hph_hd2_mode(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    rx.hph_hd2_mode = ucontrol.value.integer.value[0];
    return 0;
    }
    static int rx_macro_get_hph_pwr_mode(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    ucontrol.value.enumerated.item[0] = rx.hph_pwr_mode;
    return 0;
    }
    static int rx_macro_put_hph_pwr_mode(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    rx.hph_pwr_mode = ucontrol.value.enumerated.item[0];
    return 0;
    }
    static int rx_macro_soft_clip_enable_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    ucontrol.value.integer.value[0] = rx.is_softclip_on;
    return 0;
    }
    static int rx_macro_soft_clip_enable_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    rx.is_softclip_on = ucontrol.value.integer.value[0];
    return 0;
    }
    static int rx_macro_aux_hpf_mode_get(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    ucontrol.value.integer.value[0] = rx.is_aux_hpf_on;
    return 0;
    }
    static int rx_macro_aux_hpf_mode_put(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    rx.is_aux_hpf_on = ucontrol.value.integer.value[0];
    return 0;
    }
    static int rx_macro_hphdelay_lutbypass(struct snd_soc_component *component,
    struct rx_macro *rx,
    u16 interp_idx, int event)
    {
    u16 hph_lut_bypass_reg;
    u16 hph_comp_ctrl7;
    switch (interp_idx) {
    case INTERP_HPHL:
    hph_lut_bypass_reg = CDC_RX_TOP_HPHL_COMP_LUT;
    hph_comp_ctrl7 = CDC_RX_COMPANDER0_CTL7;
    break;
    case INTERP_HPHR:
    hph_lut_bypass_reg = CDC_RX_TOP_HPHR_COMP_LUT;
    hph_comp_ctrl7 = CDC_RX_COMPANDER1_CTL7;
    break;
    default:
    return -EINVAL;
    }
    if (hph_lut_bypass_reg && SND_SOC_DAPM_EVENT_ON(event)) {
    if (interp_idx == INTERP_HPHL) {
    if (rx.is_ear_mode_on)
    snd_soc_component_write_field(component,
    CDC_RX_RXn_RX_PATH_CFG1(rx, 0),
    CDC_RX_RX0_HPH_L_EAR_SEL_MASK, 0x1);
    else
    snd_soc_component_write_field(component,
    hph_lut_bypass_reg,
    CDC_RX_TOP_HPH_LUT_BYPASS_MASK, 1);
    } else {
    snd_soc_component_write_field(component, hph_lut_bypass_reg,
    CDC_RX_TOP_HPH_LUT_BYPASS_MASK, 1);
    }
    if (rx.hph_pwr_mode)
    snd_soc_component_write_field(component, hph_comp_ctrl7,
    CDC_RX_COMPANDER1_HPH_LOW_PWR_MODE_MASK, 0x0);
    }
    if (hph_lut_bypass_reg && SND_SOC_DAPM_EVENT_OFF(event)) {
    snd_soc_component_write_field(component,
    CDC_RX_RXn_RX_PATH_CFG1(rx, 0),
    CDC_RX_RX0_HPH_L_EAR_SEL_MASK, 0x0);
    snd_soc_component_update_bits(component, hph_lut_bypass_reg,
    CDC_RX_TOP_HPH_LUT_BYPASS_MASK, 0);
    snd_soc_component_write_field(component, hph_comp_ctrl7,
    CDC_RX_COMPANDER1_HPH_LOW_PWR_MODE_MASK, 0x1);
    }
    return 0;
    }
    static int rx_macro_enable_interp_clk(struct snd_soc_component *component,
    int event, int interp_idx)
    {
    u16 main_reg, dsm_reg, rx_cfg2_reg;
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    main_reg = CDC_RX_RXn_RX_PATH_CTL(rx, interp_idx);
    dsm_reg = CDC_RX_RXn_RX_PATH_DSM_CTL(rx, interp_idx);
    rx_cfg2_reg = CDC_RX_RXn_RX_PATH_CFG2(rx, interp_idx);
    if (SND_SOC_DAPM_EVENT_ON(event)) {
    if (rx.main_clk_users[interp_idx] == 0) {
// Main path PGA mute enable
    snd_soc_component_write_field(component, main_reg,
    CDC_RX_PATH_PGA_MUTE_MASK, 0x1);
    snd_soc_component_write_field(component, dsm_reg,
    CDC_RX_RXn_DSM_CLK_EN_MASK, 0x1);
    snd_soc_component_update_bits(component, rx_cfg2_reg,
    CDC_RX_RXn_HPF_CUT_FREQ_MASK, 0x03);
    rx_macro_load_compander_coeff(component, rx, interp_idx, event);
    if (rx.hph_hd2_mode)
    rx_macro_hd2_control(component, interp_idx, event);
    rx_macro_hphdelay_lutbypass(component, rx, interp_idx, event);
    rx_macro_config_compander(component, rx, interp_idx, event);
    if (interp_idx == INTERP_AUX) {
    rx_macro_config_softclip(component, rx,	event);
    rx_macro_config_aux_hpf(component, rx, event);
    }
    rx_macro_config_classh(component, rx, interp_idx, event);
    }
    rx.main_clk_users[interp_idx]++;
    }
    if (SND_SOC_DAPM_EVENT_OFF(event)) {
    rx.main_clk_users[interp_idx]--;
    if (rx.main_clk_users[interp_idx] <= 0) {
    rx.main_clk_users[interp_idx] = 0;
// Main path PGA mute enable
    snd_soc_component_write_field(component, main_reg,
    CDC_RX_PATH_PGA_MUTE_MASK, 0x1);
// Clk Disable
    snd_soc_component_write_field(component, dsm_reg,
    CDC_RX_RXn_DSM_CLK_EN_MASK, 0);
    snd_soc_component_write_field(component, main_reg,
    CDC_RX_PATH_CLK_EN_MASK, 0);
// Reset enable and disable
    snd_soc_component_write_field(component, main_reg,
    CDC_RX_PATH_RESET_EN_MASK, 1);
    snd_soc_component_write_field(component, main_reg,
    CDC_RX_PATH_RESET_EN_MASK, 0);
// Reset rate to 48K
    snd_soc_component_update_bits(component, main_reg,
    CDC_RX_PATH_PCM_RATE_MASK,
    0x04);
    snd_soc_component_update_bits(component, rx_cfg2_reg,
    CDC_RX_RXn_HPF_CUT_FREQ_MASK, 0x00);
    rx_macro_config_classh(component, rx, interp_idx, event);
    rx_macro_config_compander(component, rx, interp_idx, event);
    if (interp_idx ==  INTERP_AUX) {
    rx_macro_config_softclip(component, rx,	event);
    rx_macro_config_aux_hpf(component, rx, event);
    }
    rx_macro_hphdelay_lutbypass(component, rx, interp_idx, event);
    if (rx.hph_hd2_mode)
    rx_macro_hd2_control(component, interp_idx, event);
    }
    }
    return rx.main_clk_users[interp_idx];
    }
    static int rx_macro_enable_mix_path(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    u16 gain_reg, mix_reg;
    gain_reg = CDC_RX_RXn_RX_VOL_MIX_CTL(rx, w.shift);
    mix_reg = CDC_RX_RXn_RX_PATH_MIX_CTL(rx, w.shift);
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    rx_macro_enable_interp_clk(component, event, w.shift);
    break;
    case SND_SOC_DAPM_POST_PMU:
    snd_soc_component_write(component, gain_reg,
    snd_soc_component_read(component, gain_reg));
    break;
    case SND_SOC_DAPM_POST_PMD:
// Clk Disable
    snd_soc_component_update_bits(component, mix_reg,
    CDC_RX_RXn_MIX_CLK_EN_MASK, 0x00);
    rx_macro_enable_interp_clk(component, event, w.shift);
// Reset enable and disable
    snd_soc_component_update_bits(component, mix_reg,
    CDC_RX_RXn_MIX_RESET_MASK,
    CDC_RX_RXn_MIX_RESET);
    snd_soc_component_update_bits(component, mix_reg,
    CDC_RX_RXn_MIX_RESET_MASK, 0x00);
    break;
    }
    return 0;
    }
    static int rx_macro_enable_rx_path_clk(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    switch (event) {
    case SND_SOC_DAPM_PRE_PMU:
    rx_macro_enable_interp_clk(component, event, w.shift);
    snd_soc_component_write_field(component, CDC_RX_RXn_RX_PATH_CFG1(rx, w.shift),
    CDC_RX_RXn_SIDETONE_EN_MASK, 1);
    snd_soc_component_write_field(component, CDC_RX_RXn_RX_PATH_CTL(rx, w.shift),
    CDC_RX_PATH_CLK_EN_MASK, 1);
    break;
    case SND_SOC_DAPM_POST_PMD:
    snd_soc_component_write_field(component, CDC_RX_RXn_RX_PATH_CFG1(rx, w.shift),
    CDC_RX_RXn_SIDETONE_EN_MASK, 0);
    rx_macro_enable_interp_clk(component, event, w.shift);
    break;
    default:
    break;
    }
    return 0;
    }
    static int rx_macro_set_iir_gain(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    switch (event) {
    case SND_SOC_DAPM_POST_PMU: /* fall through */
    case SND_SOC_DAPM_PRE_PMD:
    if (strnstr(w.name, "IIR0", sizeof("IIR0"))) {
    snd_soc_component_write(component,
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B1_CTL,
    snd_soc_component_read(component,
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B1_CTL));
    snd_soc_component_write(component,
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B2_CTL,
    snd_soc_component_read(component,
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B2_CTL));
    snd_soc_component_write(component,
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B3_CTL,
    snd_soc_component_read(component,
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B3_CTL));
    snd_soc_component_write(component,
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B4_CTL,
    snd_soc_component_read(component,
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B4_CTL));
    } else {
    snd_soc_component_write(component,
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B1_CTL,
    snd_soc_component_read(component,
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B1_CTL));
    snd_soc_component_write(component,
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B2_CTL,
    snd_soc_component_read(component,
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B2_CTL));
    snd_soc_component_write(component,
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B3_CTL,
    snd_soc_component_read(component,
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B3_CTL));
    snd_soc_component_write(component,
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B4_CTL,
    snd_soc_component_read(component,
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B4_CTL));
    }
    break;
    }
    return 0;
    }
    static uint32_t get_iir_band_coeff(struct snd_soc_component *component,
    int iir_idx, int band_idx, int coeff_idx)
    {
    u32 value;
    int reg, b2_reg;
// Address does not automatically update if reading
    reg = CDC_RX_SIDETONE_IIR0_IIR_COEF_B1_CTL + 0x80 * iir_idx;
    b2_reg = CDC_RX_SIDETONE_IIR0_IIR_COEF_B2_CTL + 0x80 * iir_idx;
    snd_soc_component_write(component, reg,
    ((band_idx * BAND_MAX + coeff_idx) *
    sizeof(uint32_t)) & 0x7F);
    value = snd_soc_component_read(component, b2_reg);
    snd_soc_component_write(component, reg,
    ((band_idx * BAND_MAX + coeff_idx)
// sizeof(uint32_t) + 1) & 0x7F);
    value |= (snd_soc_component_read(component, b2_reg) << 8);
    snd_soc_component_write(component, reg,
    ((band_idx * BAND_MAX + coeff_idx)
// sizeof(uint32_t) + 2) & 0x7F);
    value |= (snd_soc_component_read(component, b2_reg) << 16);
    snd_soc_component_write(component, reg,
    ((band_idx * BAND_MAX + coeff_idx)
// sizeof(uint32_t) + 3) & 0x7F);
// Mask bits top 2 bits since they are reserved
    value |= (snd_soc_component_read(component, b2_reg) << 24);
    return value;
    }
    static void set_iir_band_coeff(struct snd_soc_component *component,
    int iir_idx, int band_idx, uint32_t value)
    {
    let mut reg: c_int = CDC_RX_SIDETONE_IIR0_IIR_COEF_B2_CTL + 0x80 * iir_idx;
    snd_soc_component_write(component, reg, (value & 0xFF));
    snd_soc_component_write(component, reg, (value >> 8) & 0xFF);
    snd_soc_component_write(component, reg, (value >> 16) & 0xFF);
// Mask top 2 bits, 7-8 are reserved
    snd_soc_component_write(component, reg, (value >> 24) & 0x3F);
    }
    static int rx_macro_put_iir_band_audio_mixer(
    struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct wcd_iir_filter_ctl *ctl =
    (struct wcd_iir_filter_ctl *)kcontrol.private_value;
    struct soc_bytes_ext *params = &ctl.bytes_ext;
    let mut iir_idx: c_int = ctl.iir_idx;
    let mut band_idx: c_int = ctl.band_idx;
    u32 coeff[BAND_MAX];
    let mut reg: c_int = CDC_RX_SIDETONE_IIR0_IIR_COEF_B1_CTL + 0x80 * iir_idx;
    memcpy(&coeff[0], ucontrol.value.bytes.data, params.max);
// Mask top bit it is reserved
// Updates addr automatically for each B2 write
    snd_soc_component_write(component, reg, (band_idx * BAND_MAX *
    sizeof(uint32_t)) & 0x7F);
    set_iir_band_coeff(component, iir_idx, band_idx, coeff[0]);
    set_iir_band_coeff(component, iir_idx, band_idx, coeff[1]);
    set_iir_band_coeff(component, iir_idx, band_idx, coeff[2]);
    set_iir_band_coeff(component, iir_idx, band_idx, coeff[3]);
    set_iir_band_coeff(component, iir_idx, band_idx, coeff[4]);
    return 0;
    }
    static int rx_macro_get_iir_band_audio_mixer(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_value *ucontrol)
    {
    struct snd_soc_component *component = snd_kcontrol_chip(kcontrol);
    struct wcd_iir_filter_ctl *ctl =
    (struct wcd_iir_filter_ctl *)kcontrol.private_value;
    struct soc_bytes_ext *params = &ctl.bytes_ext;
    let mut iir_idx: c_int = ctl.iir_idx;
    let mut band_idx: c_int = ctl.band_idx;
    u32 coeff[BAND_MAX];
    coeff[0] = get_iir_band_coeff(component, iir_idx, band_idx, 0);
    coeff[1] = get_iir_band_coeff(component, iir_idx, band_idx, 1);
    coeff[2] = get_iir_band_coeff(component, iir_idx, band_idx, 2);
    coeff[3] = get_iir_band_coeff(component, iir_idx, band_idx, 3);
    coeff[4] = get_iir_band_coeff(component, iir_idx, band_idx, 4);
    memcpy(ucontrol.value.bytes.data, &coeff[0], params.max);
    return 0;
    }
    static int rx_macro_iir_filter_info(struct snd_kcontrol *kcontrol,
    struct snd_ctl_elem_info *ucontrol)
    {
    struct wcd_iir_filter_ctl *ctl =
    (struct wcd_iir_filter_ctl *)kcontrol.private_value;
    struct soc_bytes_ext *params = &ctl.bytes_ext;
    ucontrol.type = SNDRV_CTL_ELEM_TYPE_BYTES;
    ucontrol.count = params.max;
    return 0;
    }
    static const struct snd_kcontrol_new rx_macro_def_snd_controls[] = {
    SOC_SINGLE_S8_TLV("RX_RX1 Digital Volume", CDC_RX_RX1_RX_VOL_CTL,
    -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV("RX_RX2 Digital Volume", CDC_RX_RX2_RX_VOL_CTL,
    -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV("RX_RX1 Mix Digital Volume", CDC_RX_RX1_RX_VOL_MIX_CTL,
    -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV("RX_RX2 Mix Digital Volume", CDC_RX_RX2_RX_VOL_MIX_CTL,
    -84, 40, digital_gain),
    };
    static const struct snd_kcontrol_new rx_macro_2_5_snd_controls[] = {
    SOC_SINGLE_S8_TLV("RX_RX1 Digital Volume", CDC_2_5_RX_RX1_RX_VOL_CTL,
    -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV("RX_RX2 Digital Volume", CDC_2_5_RX_RX2_RX_VOL_CTL,
    -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV("RX_RX1 Mix Digital Volume", CDC_2_5_RX_RX1_RX_VOL_MIX_CTL,
    -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV("RX_RX2 Mix Digital Volume", CDC_2_5_RX_RX2_RX_VOL_MIX_CTL,
    -84, 40, digital_gain),
    };
    static const struct snd_kcontrol_new rx_macro_snd_controls[] = {
    SOC_SINGLE_S8_TLV("RX_RX0 Digital Volume", CDC_RX_RX0_RX_VOL_CTL,
    -84, 40, digital_gain),
    SOC_SINGLE_S8_TLV("RX_RX0 Mix Digital Volume", CDC_RX_RX0_RX_VOL_MIX_CTL,
    -84, 40, digital_gain),
    SOC_SINGLE_EXT("RX_COMP1 Switch", SND_SOC_NOPM, RX_MACRO_COMP1, 1, 0,
    rx_macro_get_compander, rx_macro_set_compander),
    SOC_SINGLE_EXT("RX_COMP2 Switch", SND_SOC_NOPM, RX_MACRO_COMP2, 1, 0,
    rx_macro_get_compander, rx_macro_set_compander),
    SOC_SINGLE_EXT("RX_EAR Mode Switch", SND_SOC_NOPM, 0, 1, 0,
    rx_macro_get_ear_mode, rx_macro_put_ear_mode),
    SOC_SINGLE_EXT("RX_HPH HD2 Mode Switch", SND_SOC_NOPM, 0, 1, 0,
    rx_macro_get_hph_hd2_mode, rx_macro_put_hph_hd2_mode),
    SOC_ENUM_EXT("RX_HPH PWR Mode", rx_macro_hph_pwr_mode_enum,
    rx_macro_get_hph_pwr_mode, rx_macro_put_hph_pwr_mode),
    SOC_SINGLE_EXT("RX_Softclip Switch", SND_SOC_NOPM, 0, 1, 0,
    rx_macro_soft_clip_enable_get,
    rx_macro_soft_clip_enable_put),
    SOC_SINGLE_EXT("AUX_HPF Switch", SND_SOC_NOPM, 0, 1, 0,
    rx_macro_aux_hpf_mode_get,
    rx_macro_aux_hpf_mode_put),
    SOC_SINGLE_S8_TLV("IIR0 INP0 Volume",
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B1_CTL, -84, 40,
    digital_gain),
    SOC_SINGLE_S8_TLV("IIR0 INP1 Volume",
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B2_CTL, -84, 40,
    digital_gain),
    SOC_SINGLE_S8_TLV("IIR0 INP2 Volume",
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B3_CTL, -84, 40,
    digital_gain),
    SOC_SINGLE_S8_TLV("IIR0 INP3 Volume",
    CDC_RX_SIDETONE_IIR0_IIR_GAIN_B4_CTL, -84, 40,
    digital_gain),
    SOC_SINGLE_S8_TLV("IIR1 INP0 Volume",
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B1_CTL, -84, 40,
    digital_gain),
    SOC_SINGLE_S8_TLV("IIR1 INP1 Volume",
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B2_CTL, -84, 40,
    digital_gain),
    SOC_SINGLE_S8_TLV("IIR1 INP2 Volume",
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B3_CTL, -84, 40,
    digital_gain),
    SOC_SINGLE_S8_TLV("IIR1 INP3 Volume",
    CDC_RX_SIDETONE_IIR1_IIR_GAIN_B4_CTL, -84, 40,
    digital_gain),
    SOC_SINGLE("IIR1 Band1 Switch", CDC_RX_SIDETONE_IIR0_IIR_CTL,
    0, 1, 0),
    SOC_SINGLE("IIR1 Band2 Switch", CDC_RX_SIDETONE_IIR0_IIR_CTL,
    1, 1, 0),
    SOC_SINGLE("IIR1 Band3 Switch", CDC_RX_SIDETONE_IIR0_IIR_CTL,
    2, 1, 0),
    SOC_SINGLE("IIR1 Band4 Switch", CDC_RX_SIDETONE_IIR0_IIR_CTL,
    3, 1, 0),
    SOC_SINGLE("IIR1 Band5 Switch", CDC_RX_SIDETONE_IIR0_IIR_CTL,
    4, 1, 0),
    SOC_SINGLE("IIR2 Band1 Switch", CDC_RX_SIDETONE_IIR1_IIR_CTL,
    0, 1, 0),
    SOC_SINGLE("IIR2 Band2 Switch", CDC_RX_SIDETONE_IIR1_IIR_CTL,
    1, 1, 0),
    SOC_SINGLE("IIR2 Band3 Switch", CDC_RX_SIDETONE_IIR1_IIR_CTL,
    2, 1, 0),
    SOC_SINGLE("IIR2 Band4 Switch", CDC_RX_SIDETONE_IIR1_IIR_CTL,
    3, 1, 0),
    SOC_SINGLE("IIR2 Band5 Switch", CDC_RX_SIDETONE_IIR1_IIR_CTL,
    4, 1, 0),
    RX_MACRO_IIR_FILTER_CTL("IIR0 Band1", IIR0, BAND1),
    RX_MACRO_IIR_FILTER_CTL("IIR0 Band2", IIR0, BAND2),
    RX_MACRO_IIR_FILTER_CTL("IIR0 Band3", IIR0, BAND3),
    RX_MACRO_IIR_FILTER_CTL("IIR0 Band4", IIR0, BAND4),
    RX_MACRO_IIR_FILTER_CTL("IIR0 Band5", IIR0, BAND5),
    RX_MACRO_IIR_FILTER_CTL("IIR1 Band1", IIR1, BAND1),
    RX_MACRO_IIR_FILTER_CTL("IIR1 Band2", IIR1, BAND2),
    RX_MACRO_IIR_FILTER_CTL("IIR1 Band3", IIR1, BAND3),
    RX_MACRO_IIR_FILTER_CTL("IIR1 Band4", IIR1, BAND4),
    RX_MACRO_IIR_FILTER_CTL("IIR1 Band5", IIR1, BAND5),
    };
    static int rx_macro_enable_echo(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol,
    int event)
    {
    struct snd_soc_component *component = snd_soc_dapm_to_component(w.dapm);
    u16 val, ec_hq_reg;
    let mut ec_tx: c_int = -1;
    val = snd_soc_component_read(component,
    CDC_RX_INP_MUX_RX_MIX_CFG4);
    if (!(snd_soc_dapm_widget_name_cmp(w, "RX MIX TX0 MUX")))
    ec_tx = ((val & 0xf0) >> 0x4) - 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !(snd_soc_dapm_widget_name_cmp(w, MUX")): "RX MIX TX1) -> else {
    else if (!(snd_soc_dapm_widget_name_cmp(w, "RX MIX TX1 MUX")))
    ec_tx = (val & 0x0f) - 1;
    val = snd_soc_component_read(component,
    CDC_RX_INP_MUX_RX_MIX_CFG5);
    if (!(snd_soc_dapm_widget_name_cmp(w, "RX MIX TX2 MUX")))
    ec_tx = (val & 0x0f) - 1;
    if (ec_tx < 0 || (ec_tx >= RX_MACRO_EC_MUX_MAX)) {
    dev_err(component.dev, "%s: EC mix control not set correctly\n",
    __func__);
    return -EINVAL;
    }
    ec_hq_reg = CDC_RX_EC_REF_HQ0_EC_REF_HQ_PATH_CTL +
    0x40 * ec_tx;
    snd_soc_component_update_bits(component, ec_hq_reg, 0x01, 0x01);
    ec_hq_reg = CDC_RX_EC_REF_HQ0_EC_REF_HQ_CFG0 +
    0x40 * ec_tx;
// default set to 48k
    snd_soc_component_update_bits(component, ec_hq_reg, 0x1E, 0x08);
    return 0;
    }
    static const struct snd_soc_dapm_widget rx_macro_2_5_dapm_widgets[] = {
    SND_SOC_DAPM_MUX("RX INT1 DEM MUX", SND_SOC_NOPM, 0, 0,
    &rx_2_5_int1_dem_inp_mux),
    };
    static const struct snd_soc_dapm_widget rx_macro_def_dapm_widgets[] = {
    SND_SOC_DAPM_MUX("RX INT1 DEM MUX", SND_SOC_NOPM, 0, 0,
    &rx_int1_dem_inp_mux),
    };
    static const struct snd_soc_dapm_widget rx_macro_dapm_widgets[] = {
    SND_SOC_DAPM_AIF_IN("RX AIF1 PB", "RX_MACRO_AIF1 Playback", 0,
    SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("RX AIF2 PB", "RX_MACRO_AIF2 Playback", 0,
    SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("RX AIF3 PB", "RX_MACRO_AIF3 Playback", 0,
    SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN("RX AIF4 PB", "RX_MACRO_AIF4 Playback", 0,
    SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT("RX AIF_ECHO", "RX_AIF_ECHO Capture", 0,
    SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX("RX_MACRO RX0 MUX", SND_SOC_NOPM, RX_MACRO_RX0, 0,
    &rx_macro_rx0_mux),
    SND_SOC_DAPM_MUX("RX_MACRO RX1 MUX", SND_SOC_NOPM, RX_MACRO_RX1, 0,
    &rx_macro_rx1_mux),
    SND_SOC_DAPM_MUX("RX_MACRO RX2 MUX", SND_SOC_NOPM, RX_MACRO_RX2, 0,
    &rx_macro_rx2_mux),
    SND_SOC_DAPM_MUX("RX_MACRO RX3 MUX", SND_SOC_NOPM, RX_MACRO_RX3, 0,
    &rx_macro_rx3_mux),
    SND_SOC_DAPM_MUX("RX_MACRO RX4 MUX", SND_SOC_NOPM, RX_MACRO_RX4, 0,
    &rx_macro_rx4_mux),
    SND_SOC_DAPM_MUX("RX_MACRO RX5 MUX", SND_SOC_NOPM, RX_MACRO_RX5, 0,
    &rx_macro_rx5_mux),
    SND_SOC_DAPM_MIXER("RX_RX0", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX_RX1", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX_RX2", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX_RX3", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX_RX4", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX_RX5", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MUX("IIR0 INP0 MUX", SND_SOC_NOPM, 0, 0, &iir0_inp0_mux),
    SND_SOC_DAPM_MUX("IIR0 INP1 MUX", SND_SOC_NOPM, 0, 0, &iir0_inp1_mux),
    SND_SOC_DAPM_MUX("IIR0 INP2 MUX", SND_SOC_NOPM, 0, 0, &iir0_inp2_mux),
    SND_SOC_DAPM_MUX("IIR0 INP3 MUX", SND_SOC_NOPM, 0, 0, &iir0_inp3_mux),
    SND_SOC_DAPM_MUX("IIR1 INP0 MUX", SND_SOC_NOPM, 0, 0, &iir1_inp0_mux),
    SND_SOC_DAPM_MUX("IIR1 INP1 MUX", SND_SOC_NOPM, 0, 0, &iir1_inp1_mux),
    SND_SOC_DAPM_MUX("IIR1 INP2 MUX", SND_SOC_NOPM, 0, 0, &iir1_inp2_mux),
    SND_SOC_DAPM_MUX("IIR1 INP3 MUX", SND_SOC_NOPM, 0, 0, &iir1_inp3_mux),
    SND_SOC_DAPM_MUX_E("RX MIX TX0 MUX", SND_SOC_NOPM,
    RX_MACRO_EC0_MUX, 0,
    &rx_mix_tx0_mux, rx_macro_enable_echo,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E("RX MIX TX1 MUX", SND_SOC_NOPM,
    RX_MACRO_EC1_MUX, 0,
    &rx_mix_tx1_mux, rx_macro_enable_echo,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E("RX MIX TX2 MUX", SND_SOC_NOPM,
    RX_MACRO_EC2_MUX, 0,
    &rx_mix_tx2_mux, rx_macro_enable_echo,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER_E("IIR0", CDC_RX_SIDETONE_IIR0_IIR_PATH_CTL,
    4, 0, core::ptr::null_mut(), 0, rx_macro_set_iir_gain,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MIXER_E("IIR1", CDC_RX_SIDETONE_IIR1_IIR_PATH_CTL,
    4, 0, core::ptr::null_mut(), 0, rx_macro_set_iir_gain,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MIXER("SRC0", CDC_RX_SIDETONE_SRC0_ST_SRC_PATH_CTL,
    4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("SRC1", CDC_RX_SIDETONE_SRC1_ST_SRC_PATH_CTL,
    4, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MUX("RX INT0 DEM MUX", SND_SOC_NOPM, 0, 0,
    &rx_int0_dem_inp_mux),
    SND_SOC_DAPM_MUX_E("RX INT0_2 MUX", SND_SOC_NOPM, INTERP_HPHL, 0,
    &rx_int0_2_mux, rx_macro_enable_mix_path,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E("RX INT1_2 MUX", SND_SOC_NOPM, INTERP_HPHR, 0,
    &rx_int1_2_mux, rx_macro_enable_mix_path,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E("RX INT2_2 MUX", SND_SOC_NOPM, INTERP_AUX, 0,
    &rx_int2_2_mux, rx_macro_enable_mix_path,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX("RX INT0_1 MIX1 INP0", SND_SOC_NOPM, 0, 0, &rx_int0_1_mix_inp0_mux),
    SND_SOC_DAPM_MUX("RX INT0_1 MIX1 INP1", SND_SOC_NOPM, 0, 0, &rx_int0_1_mix_inp1_mux),
    SND_SOC_DAPM_MUX("RX INT0_1 MIX1 INP2", SND_SOC_NOPM, 0, 0, &rx_int0_1_mix_inp2_mux),
    SND_SOC_DAPM_MUX("RX INT1_1 MIX1 INP0", SND_SOC_NOPM, 0, 0, &rx_int1_1_mix_inp0_mux),
    SND_SOC_DAPM_MUX("RX INT1_1 MIX1 INP1", SND_SOC_NOPM, 0, 0, &rx_int1_1_mix_inp1_mux),
    SND_SOC_DAPM_MUX("RX INT1_1 MIX1 INP2", SND_SOC_NOPM, 0, 0, &rx_int1_1_mix_inp2_mux),
    SND_SOC_DAPM_MUX("RX INT2_1 MIX1 INP0", SND_SOC_NOPM, 0, 0, &rx_int2_1_mix_inp0_mux),
    SND_SOC_DAPM_MUX("RX INT2_1 MIX1 INP1", SND_SOC_NOPM, 0, 0, &rx_int2_1_mix_inp1_mux),
    SND_SOC_DAPM_MUX("RX INT2_1 MIX1 INP2", SND_SOC_NOPM, 0, 0, &rx_int2_1_mix_inp2_mux),
    SND_SOC_DAPM_MUX_E("RX INT0_1 INTERP", SND_SOC_NOPM, INTERP_HPHL, 0,
    &rx_int0_1_interp_mux, rx_macro_enable_main_path,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E("RX INT1_1 INTERP", SND_SOC_NOPM, INTERP_HPHR, 0,
    &rx_int1_1_interp_mux, rx_macro_enable_main_path,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E("RX INT2_1 INTERP", SND_SOC_NOPM, INTERP_AUX, 0,
    &rx_int2_1_interp_mux, rx_macro_enable_main_path,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU |
    SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX("RX INT0_2 INTERP", SND_SOC_NOPM, 0, 0,
    &rx_int0_2_interp_mux),
    SND_SOC_DAPM_MUX("RX INT1_2 INTERP", SND_SOC_NOPM, 0, 0,
    &rx_int1_2_interp_mux),
    SND_SOC_DAPM_MUX("RX INT2_2 INTERP", SND_SOC_NOPM, 0, 0,
    &rx_int2_2_interp_mux),
    SND_SOC_DAPM_MIXER("RX INT0_1 MIX1", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX INT0 SEC MIX", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX INT1_1 MIX1", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX INT1 SEC MIX", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX INT2_1 MIX1", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX INT2 SEC MIX", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MUX_E("RX INT0 MIX2 INP", SND_SOC_NOPM, INTERP_HPHL,
    0, &rx_int0_mix2_inp_mux, rx_macro_enable_rx_path_clk,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E("RX INT1 MIX2 INP", SND_SOC_NOPM, INTERP_HPHR,
    0, &rx_int1_mix2_inp_mux, rx_macro_enable_rx_path_clk,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX_E("RX INT2 MIX2 INP", SND_SOC_NOPM, INTERP_AUX,
    0, &rx_int2_mix2_inp_mux, rx_macro_enable_rx_path_clk,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER("RX INT0 MIX2", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX INT1 MIX2", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_MIXER("RX INT2 MIX2", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    SND_SOC_DAPM_OUTPUT("HPHL_OUT"),
    SND_SOC_DAPM_OUTPUT("HPHR_OUT"),
    SND_SOC_DAPM_OUTPUT("AUX_OUT"),
    SND_SOC_DAPM_INPUT("RX_TX DEC0_INP"),
    SND_SOC_DAPM_INPUT("RX_TX DEC1_INP"),
    SND_SOC_DAPM_INPUT("RX_TX DEC2_INP"),
    SND_SOC_DAPM_INPUT("RX_TX DEC3_INP"),
    SND_SOC_DAPM_SUPPLY_S("RX_MCLK", 0, SND_SOC_NOPM, 0, 0,
    rx_macro_mclk_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    };
    static const struct snd_soc_dapm_route rx_audio_map[] = {
    {"RX AIF1 PB", core::ptr::null_mut(), "RX_MCLK"},
    {"RX AIF2 PB", core::ptr::null_mut(), "RX_MCLK"},
    {"RX AIF3 PB", core::ptr::null_mut(), "RX_MCLK"},
    {"RX AIF4 PB", core::ptr::null_mut(), "RX_MCLK"},
    {"RX_MACRO RX0 MUX", "AIF1_PB", "RX AIF1 PB"},
    {"RX_MACRO RX1 MUX", "AIF1_PB", "RX AIF1 PB"},
    {"RX_MACRO RX2 MUX", "AIF1_PB", "RX AIF1 PB"},
    {"RX_MACRO RX3 MUX", "AIF1_PB", "RX AIF1 PB"},
    {"RX_MACRO RX4 MUX", "AIF1_PB", "RX AIF1 PB"},
    {"RX_MACRO RX5 MUX", "AIF1_PB", "RX AIF1 PB"},
    {"RX_MACRO RX0 MUX", "AIF2_PB", "RX AIF2 PB"},
    {"RX_MACRO RX1 MUX", "AIF2_PB", "RX AIF2 PB"},
    {"RX_MACRO RX2 MUX", "AIF2_PB", "RX AIF2 PB"},
    {"RX_MACRO RX3 MUX", "AIF2_PB", "RX AIF2 PB"},
    {"RX_MACRO RX4 MUX", "AIF2_PB", "RX AIF2 PB"},
    {"RX_MACRO RX5 MUX", "AIF2_PB", "RX AIF2 PB"},
    {"RX_MACRO RX0 MUX", "AIF3_PB", "RX AIF3 PB"},
    {"RX_MACRO RX1 MUX", "AIF3_PB", "RX AIF3 PB"},
    {"RX_MACRO RX2 MUX", "AIF3_PB", "RX AIF3 PB"},
    {"RX_MACRO RX3 MUX", "AIF3_PB", "RX AIF3 PB"},
    {"RX_MACRO RX4 MUX", "AIF3_PB", "RX AIF3 PB"},
    {"RX_MACRO RX5 MUX", "AIF3_PB", "RX AIF3 PB"},
    {"RX_MACRO RX0 MUX", "AIF4_PB", "RX AIF4 PB"},
    {"RX_MACRO RX1 MUX", "AIF4_PB", "RX AIF4 PB"},
    {"RX_MACRO RX2 MUX", "AIF4_PB", "RX AIF4 PB"},
    {"RX_MACRO RX3 MUX", "AIF4_PB", "RX AIF4 PB"},
    {"RX_MACRO RX4 MUX", "AIF4_PB", "RX AIF4 PB"},
    {"RX_MACRO RX5 MUX", "AIF4_PB", "RX AIF4 PB"},
    {"RX_RX0", core::ptr::null_mut(), "RX_MACRO RX0 MUX"},
    {"RX_RX1", core::ptr::null_mut(), "RX_MACRO RX1 MUX"},
    {"RX_RX2", core::ptr::null_mut(), "RX_MACRO RX2 MUX"},
    {"RX_RX3", core::ptr::null_mut(), "RX_MACRO RX3 MUX"},
    {"RX_RX4", core::ptr::null_mut(), "RX_MACRO RX4 MUX"},
    {"RX_RX5", core::ptr::null_mut(), "RX_MACRO RX5 MUX"},
    {"RX INT0_1 MIX1 INP0", "RX0", "RX_RX0"},
    {"RX INT0_1 MIX1 INP0", "RX1", "RX_RX1"},
    {"RX INT0_1 MIX1 INP0", "RX2", "RX_RX2"},
    {"RX INT0_1 MIX1 INP0", "RX3", "RX_RX3"},
    {"RX INT0_1 MIX1 INP0", "RX4", "RX_RX4"},
    {"RX INT0_1 MIX1 INP0", "RX5", "RX_RX5"},
    {"RX INT0_1 MIX1 INP0", "IIR0", "IIR0"},
    {"RX INT0_1 MIX1 INP0", "IIR1", "IIR1"},
    {"RX INT0_1 MIX1 INP0", "DEC0", "RX_TX DEC0_INP"},
    {"RX INT0_1 MIX1 INP0", "DEC1", "RX_TX DEC1_INP"},
    {"RX INT0_1 MIX1 INP1", "RX0", "RX_RX0"},
    {"RX INT0_1 MIX1 INP1", "RX1", "RX_RX1"},
    {"RX INT0_1 MIX1 INP1", "RX2", "RX_RX2"},
    {"RX INT0_1 MIX1 INP1", "RX3", "RX_RX3"},
    {"RX INT0_1 MIX1 INP1", "RX4", "RX_RX4"},
    {"RX INT0_1 MIX1 INP1", "RX5", "RX_RX5"},
    {"RX INT0_1 MIX1 INP1", "IIR0", "IIR0"},
    {"RX INT0_1 MIX1 INP1", "IIR1", "IIR1"},
    {"RX INT0_1 MIX1 INP1", "DEC0", "RX_TX DEC0_INP"},
    {"RX INT0_1 MIX1 INP1", "DEC1", "RX_TX DEC1_INP"},
    {"RX INT0_1 MIX1 INP2", "RX0", "RX_RX0"},
    {"RX INT0_1 MIX1 INP2", "RX1", "RX_RX1"},
    {"RX INT0_1 MIX1 INP2", "RX2", "RX_RX2"},
    {"RX INT0_1 MIX1 INP2", "RX3", "RX_RX3"},
    {"RX INT0_1 MIX1 INP2", "RX4", "RX_RX4"},
    {"RX INT0_1 MIX1 INP2", "RX5", "RX_RX5"},
    {"RX INT0_1 MIX1 INP2", "IIR0", "IIR0"},
    {"RX INT0_1 MIX1 INP2", "IIR1", "IIR1"},
    {"RX INT0_1 MIX1 INP2", "DEC0", "RX_TX DEC0_INP"},
    {"RX INT0_1 MIX1 INP2", "DEC1", "RX_TX DEC1_INP"},
    {"RX INT1_1 MIX1 INP0", "RX0", "RX_RX0"},
    {"RX INT1_1 MIX1 INP0", "RX1", "RX_RX1"},
    {"RX INT1_1 MIX1 INP0", "RX2", "RX_RX2"},
    {"RX INT1_1 MIX1 INP0", "RX3", "RX_RX3"},
    {"RX INT1_1 MIX1 INP0", "RX4", "RX_RX4"},
    {"RX INT1_1 MIX1 INP0", "RX5", "RX_RX5"},
    {"RX INT1_1 MIX1 INP0", "IIR0", "IIR0"},
    {"RX INT1_1 MIX1 INP0", "IIR1", "IIR1"},
    {"RX INT1_1 MIX1 INP0", "DEC0", "RX_TX DEC0_INP"},
    {"RX INT1_1 MIX1 INP0", "DEC1", "RX_TX DEC1_INP"},
    {"RX INT1_1 MIX1 INP1", "RX0", "RX_RX0"},
    {"RX INT1_1 MIX1 INP1", "RX1", "RX_RX1"},
    {"RX INT1_1 MIX1 INP1", "RX2", "RX_RX2"},
    {"RX INT1_1 MIX1 INP1", "RX3", "RX_RX3"},
    {"RX INT1_1 MIX1 INP1", "RX4", "RX_RX4"},
    {"RX INT1_1 MIX1 INP1", "RX5", "RX_RX5"},
    {"RX INT1_1 MIX1 INP1", "IIR0", "IIR0"},
    {"RX INT1_1 MIX1 INP1", "IIR1", "IIR1"},
    {"RX INT1_1 MIX1 INP1", "DEC0", "RX_TX DEC0_INP"},
    {"RX INT1_1 MIX1 INP1", "DEC1", "RX_TX DEC1_INP"},
    {"RX INT1_1 MIX1 INP2", "RX0", "RX_RX0"},
    {"RX INT1_1 MIX1 INP2", "RX1", "RX_RX1"},
    {"RX INT1_1 MIX1 INP2", "RX2", "RX_RX2"},
    {"RX INT1_1 MIX1 INP2", "RX3", "RX_RX3"},
    {"RX INT1_1 MIX1 INP2", "RX4", "RX_RX4"},
    {"RX INT1_1 MIX1 INP2", "RX5", "RX_RX5"},
    {"RX INT1_1 MIX1 INP2", "IIR0", "IIR0"},
    {"RX INT1_1 MIX1 INP2", "IIR1", "IIR1"},
    {"RX INT1_1 MIX1 INP2", "DEC0", "RX_TX DEC0_INP"},
    {"RX INT1_1 MIX1 INP2", "DEC1", "RX_TX DEC1_INP"},
    {"RX INT2_1 MIX1 INP0", "RX0", "RX_RX0"},
    {"RX INT2_1 MIX1 INP0", "RX1", "RX_RX1"},
    {"RX INT2_1 MIX1 INP0", "RX2", "RX_RX2"},
    {"RX INT2_1 MIX1 INP0", "RX3", "RX_RX3"},
    {"RX INT2_1 MIX1 INP0", "RX4", "RX_RX4"},
    {"RX INT2_1 MIX1 INP0", "RX5", "RX_RX5"},
    {"RX INT2_1 MIX1 INP0", "IIR0", "IIR0"},
    {"RX INT2_1 MIX1 INP0", "IIR1", "IIR1"},
    {"RX INT2_1 MIX1 INP0", "DEC0", "RX_TX DEC0_INP"},
    {"RX INT2_1 MIX1 INP0", "DEC1", "RX_TX DEC1_INP"},
    {"RX INT2_1 MIX1 INP1", "RX0", "RX_RX0"},
    {"RX INT2_1 MIX1 INP1", "RX1", "RX_RX1"},
    {"RX INT2_1 MIX1 INP1", "RX2", "RX_RX2"},
    {"RX INT2_1 MIX1 INP1", "RX3", "RX_RX3"},
    {"RX INT2_1 MIX1 INP1", "RX4", "RX_RX4"},
    {"RX INT2_1 MIX1 INP1", "RX5", "RX_RX5"},
    {"RX INT2_1 MIX1 INP1", "IIR0", "IIR0"},
    {"RX INT2_1 MIX1 INP1", "IIR1", "IIR1"},
    {"RX INT2_1 MIX1 INP1", "DEC0", "RX_TX DEC0_INP"},
    {"RX INT2_1 MIX1 INP1", "DEC1", "RX_TX DEC1_INP"},
    {"RX INT2_1 MIX1 INP2", "RX0", "RX_RX0"},
    {"RX INT2_1 MIX1 INP2", "RX1", "RX_RX1"},
    {"RX INT2_1 MIX1 INP2", "RX2", "RX_RX2"},
    {"RX INT2_1 MIX1 INP2", "RX3", "RX_RX3"},
    {"RX INT2_1 MIX1 INP2", "RX4", "RX_RX4"},
    {"RX INT2_1 MIX1 INP2", "RX5", "RX_RX5"},
    {"RX INT2_1 MIX1 INP2", "IIR0", "IIR0"},
    {"RX INT2_1 MIX1 INP2", "IIR1", "IIR1"},
    {"RX INT2_1 MIX1 INP2", "DEC0", "RX_TX DEC0_INP"},
    {"RX INT2_1 MIX1 INP2", "DEC1", "RX_TX DEC1_INP"},
    {"RX INT0_1 MIX1", core::ptr::null_mut(), "RX INT0_1 MIX1 INP0"},
    {"RX INT0_1 MIX1", core::ptr::null_mut(), "RX INT0_1 MIX1 INP1"},
    {"RX INT0_1 MIX1", core::ptr::null_mut(), "RX INT0_1 MIX1 INP2"},
    {"RX INT1_1 MIX1", core::ptr::null_mut(), "RX INT1_1 MIX1 INP0"},
    {"RX INT1_1 MIX1", core::ptr::null_mut(), "RX INT1_1 MIX1 INP1"},
    {"RX INT1_1 MIX1", core::ptr::null_mut(), "RX INT1_1 MIX1 INP2"},
    {"RX INT2_1 MIX1", core::ptr::null_mut(), "RX INT2_1 MIX1 INP0"},
    {"RX INT2_1 MIX1", core::ptr::null_mut(), "RX INT2_1 MIX1 INP1"},
    {"RX INT2_1 MIX1", core::ptr::null_mut(), "RX INT2_1 MIX1 INP2"},
    {"RX MIX TX0 MUX", "RX_MIX0", "RX INT0 SEC MIX"},
    {"RX MIX TX0 MUX", "RX_MIX1", "RX INT1 SEC MIX"},
    {"RX MIX TX0 MUX", "RX_MIX2", "RX INT2 SEC MIX"},
    {"RX MIX TX1 MUX", "RX_MIX0", "RX INT0 SEC MIX"},
    {"RX MIX TX1 MUX", "RX_MIX1", "RX INT1 SEC MIX"},
    {"RX MIX TX1 MUX", "RX_MIX2", "RX INT2 SEC MIX"},
    {"RX MIX TX2 MUX", "RX_MIX0", "RX INT0 SEC MIX"},
    {"RX MIX TX2 MUX", "RX_MIX1", "RX INT1 SEC MIX"},
    {"RX MIX TX2 MUX", "RX_MIX2", "RX INT2 SEC MIX"},
    {"RX AIF_ECHO", core::ptr::null_mut(), "RX MIX TX0 MUX"},
    {"RX AIF_ECHO", core::ptr::null_mut(), "RX MIX TX1 MUX"},
    {"RX AIF_ECHO", core::ptr::null_mut(), "RX MIX TX2 MUX"},
    {"RX AIF_ECHO", core::ptr::null_mut(), "RX_MCLK"},
// Mixing path INT0
    {"RX INT0_2 MUX", "RX0", "RX_RX0"},
    {"RX INT0_2 MUX", "RX1", "RX_RX1"},
    {"RX INT0_2 MUX", "RX2", "RX_RX2"},
    {"RX INT0_2 MUX", "RX3", "RX_RX3"},
    {"RX INT0_2 MUX", "RX4", "RX_RX4"},
    {"RX INT0_2 MUX", "RX5", "RX_RX5"},
    {"RX INT0_2 INTERP", core::ptr::null_mut(), "RX INT0_2 MUX"},
    {"RX INT0 SEC MIX", core::ptr::null_mut(), "RX INT0_2 INTERP"},
// Mixing path INT1
    {"RX INT1_2 MUX", "RX0", "RX_RX0"},
    {"RX INT1_2 MUX", "RX1", "RX_RX1"},
    {"RX INT1_2 MUX", "RX2", "RX_RX2"},
    {"RX INT1_2 MUX", "RX3", "RX_RX3"},
    {"RX INT1_2 MUX", "RX4", "RX_RX4"},
    {"RX INT1_2 MUX", "RX5", "RX_RX5"},
    {"RX INT1_2 INTERP", core::ptr::null_mut(), "RX INT1_2 MUX"},
    {"RX INT1 SEC MIX", core::ptr::null_mut(), "RX INT1_2 INTERP"},
// Mixing path INT2
    {"RX INT2_2 MUX", "RX0", "RX_RX0"},
    {"RX INT2_2 MUX", "RX1", "RX_RX1"},
    {"RX INT2_2 MUX", "RX2", "RX_RX2"},
    {"RX INT2_2 MUX", "RX3", "RX_RX3"},
    {"RX INT2_2 MUX", "RX4", "RX_RX4"},
    {"RX INT2_2 MUX", "RX5", "RX_RX5"},
    {"RX INT2_2 INTERP", core::ptr::null_mut(), "RX INT2_2 MUX"},
    {"RX INT2 SEC MIX", core::ptr::null_mut(), "RX INT2_2 INTERP"},
    {"RX INT0_1 INTERP", core::ptr::null_mut(), "RX INT0_1 MIX1"},
    {"RX INT0 SEC MIX", core::ptr::null_mut(), "RX INT0_1 INTERP"},
    {"RX INT0 MIX2", core::ptr::null_mut(), "RX INT0 SEC MIX"},
    {"RX INT0 MIX2", core::ptr::null_mut(), "RX INT0 MIX2 INP"},
    {"RX INT0 DEM MUX", "CLSH_DSM_OUT", "RX INT0 MIX2"},
    {"HPHL_OUT", core::ptr::null_mut(), "RX INT0 DEM MUX"},
    {"HPHL_OUT", core::ptr::null_mut(), "RX_MCLK"},
    {"RX INT1_1 INTERP", core::ptr::null_mut(), "RX INT1_1 MIX1"},
    {"RX INT1 SEC MIX", core::ptr::null_mut(), "RX INT1_1 INTERP"},
    {"RX INT1 MIX2", core::ptr::null_mut(), "RX INT1 SEC MIX"},
    {"RX INT1 MIX2", core::ptr::null_mut(), "RX INT1 MIX2 INP"},
    {"RX INT1 DEM MUX", "CLSH_DSM_OUT", "RX INT1 MIX2"},
    {"HPHR_OUT", core::ptr::null_mut(), "RX INT1 DEM MUX"},
    {"HPHR_OUT", core::ptr::null_mut(), "RX_MCLK"},
    {"RX INT2_1 INTERP", core::ptr::null_mut(), "RX INT2_1 MIX1"},
    {"RX INT2 SEC MIX", core::ptr::null_mut(), "RX INT2_1 INTERP"},
    {"RX INT2 MIX2", core::ptr::null_mut(), "RX INT2 SEC MIX"},
    {"RX INT2 MIX2", core::ptr::null_mut(), "RX INT2 MIX2 INP"},
    {"AUX_OUT", core::ptr::null_mut(), "RX INT2 MIX2"},
    {"AUX_OUT", core::ptr::null_mut(), "RX_MCLK"},
    {"IIR0", core::ptr::null_mut(), "RX_MCLK"},
    {"IIR0", core::ptr::null_mut(), "IIR0 INP0 MUX"},
    {"IIR0 INP0 MUX", "DEC0", "RX_TX DEC0_INP"},
    {"IIR0 INP0 MUX", "DEC1", "RX_TX DEC1_INP"},
    {"IIR0 INP0 MUX", "DEC2", "RX_TX DEC2_INP"},
    {"IIR0 INP0 MUX", "DEC3", "RX_TX DEC3_INP"},
    {"IIR0 INP0 MUX", "RX0", "RX_RX0"},
    {"IIR0 INP0 MUX", "RX1", "RX_RX1"},
    {"IIR0 INP0 MUX", "RX2", "RX_RX2"},
    {"IIR0 INP0 MUX", "RX3", "RX_RX3"},
    {"IIR0 INP0 MUX", "RX4", "RX_RX4"},
    {"IIR0 INP0 MUX", "RX5", "RX_RX5"},
    {"IIR0", core::ptr::null_mut(), "IIR0 INP1 MUX"},
    {"IIR0 INP1 MUX", "DEC0", "RX_TX DEC0_INP"},
    {"IIR0 INP1 MUX", "DEC1", "RX_TX DEC1_INP"},
    {"IIR0 INP1 MUX", "DEC2", "RX_TX DEC2_INP"},
    {"IIR0 INP1 MUX", "DEC3", "RX_TX DEC3_INP"},
    {"IIR0 INP1 MUX", "RX0", "RX_RX0"},
    {"IIR0 INP1 MUX", "RX1", "RX_RX1"},
    {"IIR0 INP1 MUX", "RX2", "RX_RX2"},
    {"IIR0 INP1 MUX", "RX3", "RX_RX3"},
    {"IIR0 INP1 MUX", "RX4", "RX_RX4"},
    {"IIR0 INP1 MUX", "RX5", "RX_RX5"},
    {"IIR0", core::ptr::null_mut(), "IIR0 INP2 MUX"},
    {"IIR0 INP2 MUX", "DEC0", "RX_TX DEC0_INP"},
    {"IIR0 INP2 MUX", "DEC1", "RX_TX DEC1_INP"},
    {"IIR0 INP2 MUX", "DEC2", "RX_TX DEC2_INP"},
    {"IIR0 INP2 MUX", "DEC3", "RX_TX DEC3_INP"},
    {"IIR0 INP2 MUX", "RX0", "RX_RX0"},
    {"IIR0 INP2 MUX", "RX1", "RX_RX1"},
    {"IIR0 INP2 MUX", "RX2", "RX_RX2"},
    {"IIR0 INP2 MUX", "RX3", "RX_RX3"},
    {"IIR0 INP2 MUX", "RX4", "RX_RX4"},
    {"IIR0 INP2 MUX", "RX5", "RX_RX5"},
    {"IIR0", core::ptr::null_mut(), "IIR0 INP3 MUX"},
    {"IIR0 INP3 MUX", "DEC0", "RX_TX DEC0_INP"},
    {"IIR0 INP3 MUX", "DEC1", "RX_TX DEC1_INP"},
    {"IIR0 INP3 MUX", "DEC2", "RX_TX DEC2_INP"},
    {"IIR0 INP3 MUX", "DEC3", "RX_TX DEC3_INP"},
    {"IIR0 INP3 MUX", "RX0", "RX_RX0"},
    {"IIR0 INP3 MUX", "RX1", "RX_RX1"},
    {"IIR0 INP3 MUX", "RX2", "RX_RX2"},
    {"IIR0 INP3 MUX", "RX3", "RX_RX3"},
    {"IIR0 INP3 MUX", "RX4", "RX_RX4"},
    {"IIR0 INP3 MUX", "RX5", "RX_RX5"},
    {"IIR1", core::ptr::null_mut(), "RX_MCLK"},
    {"IIR1", core::ptr::null_mut(), "IIR1 INP0 MUX"},
    {"IIR1 INP0 MUX", "DEC0", "RX_TX DEC0_INP"},
    {"IIR1 INP0 MUX", "DEC1", "RX_TX DEC1_INP"},
    {"IIR1 INP0 MUX", "DEC2", "RX_TX DEC2_INP"},
    {"IIR1 INP0 MUX", "DEC3", "RX_TX DEC3_INP"},
    {"IIR1 INP0 MUX", "RX0", "RX_RX0"},
    {"IIR1 INP0 MUX", "RX1", "RX_RX1"},
    {"IIR1 INP0 MUX", "RX2", "RX_RX2"},
    {"IIR1 INP0 MUX", "RX3", "RX_RX3"},
    {"IIR1 INP0 MUX", "RX4", "RX_RX4"},
    {"IIR1 INP0 MUX", "RX5", "RX_RX5"},
    {"IIR1", core::ptr::null_mut(), "IIR1 INP1 MUX"},
    {"IIR1 INP1 MUX", "DEC0", "RX_TX DEC0_INP"},
    {"IIR1 INP1 MUX", "DEC1", "RX_TX DEC1_INP"},
    {"IIR1 INP1 MUX", "DEC2", "RX_TX DEC2_INP"},
    {"IIR1 INP1 MUX", "DEC3", "RX_TX DEC3_INP"},
    {"IIR1 INP1 MUX", "RX0", "RX_RX0"},
    {"IIR1 INP1 MUX", "RX1", "RX_RX1"},
    {"IIR1 INP1 MUX", "RX2", "RX_RX2"},
    {"IIR1 INP1 MUX", "RX3", "RX_RX3"},
    {"IIR1 INP1 MUX", "RX4", "RX_RX4"},
    {"IIR1 INP1 MUX", "RX5", "RX_RX5"},
    {"IIR1", core::ptr::null_mut(), "IIR1 INP2 MUX"},
    {"IIR1 INP2 MUX", "DEC0", "RX_TX DEC0_INP"},
    {"IIR1 INP2 MUX", "DEC1", "RX_TX DEC1_INP"},
    {"IIR1 INP2 MUX", "DEC2", "RX_TX DEC2_INP"},
    {"IIR1 INP2 MUX", "DEC3", "RX_TX DEC3_INP"},
    {"IIR1 INP2 MUX", "RX0", "RX_RX0"},
    {"IIR1 INP2 MUX", "RX1", "RX_RX1"},
    {"IIR1 INP2 MUX", "RX2", "RX_RX2"},
    {"IIR1 INP2 MUX", "RX3", "RX_RX3"},
    {"IIR1 INP2 MUX", "RX4", "RX_RX4"},
    {"IIR1 INP2 MUX", "RX5", "RX_RX5"},
    {"IIR1", core::ptr::null_mut(), "IIR1 INP3 MUX"},
    {"IIR1 INP3 MUX", "DEC0", "RX_TX DEC0_INP"},
    {"IIR1 INP3 MUX", "DEC1", "RX_TX DEC1_INP"},
    {"IIR1 INP3 MUX", "DEC2", "RX_TX DEC2_INP"},
    {"IIR1 INP3 MUX", "DEC3", "RX_TX DEC3_INP"},
    {"IIR1 INP3 MUX", "RX0", "RX_RX0"},
    {"IIR1 INP3 MUX", "RX1", "RX_RX1"},
    {"IIR1 INP3 MUX", "RX2", "RX_RX2"},
    {"IIR1 INP3 MUX", "RX3", "RX_RX3"},
    {"IIR1 INP3 MUX", "RX4", "RX_RX4"},
    {"IIR1 INP3 MUX", "RX5", "RX_RX5"},
    {"SRC0", core::ptr::null_mut(), "IIR0"},
    {"SRC1", core::ptr::null_mut(), "IIR1"},
    {"RX INT0 MIX2 INP", "SRC0", "SRC0"},
    {"RX INT0 MIX2 INP", "SRC1", "SRC1"},
    {"RX INT1 MIX2 INP", "SRC0", "SRC0"},
    {"RX INT1 MIX2 INP", "SRC1", "SRC1"},
    {"RX INT2 MIX2 INP", "SRC0", "SRC0"},
    {"RX INT2 MIX2 INP", "SRC1", "SRC1"},
    };
#[no_mangle]
unsafe extern "C" fn rx_macro_component_probe(component: *mut snd_soc_component) -> c_int {
    static int rx_macro_component_probe(struct snd_soc_component *component)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    struct rx_macro *rx = snd_soc_component_get_drvdata(component);
    const struct snd_soc_dapm_widget *widgets;
    const struct snd_kcontrol_new *controls;
    unsigned int num_controls, num_widgets;
    int ret;
    snd_soc_component_init_regmap(component, rx.regmap);
    snd_soc_component_update_bits(component, CDC_RX_RXn_RX_PATH_SEC7(rx, 0),
    CDC_RX_DSM_OUT_DELAY_SEL_MASK,
    CDC_RX_DSM_OUT_DELAY_TWO_SAMPLE);
    snd_soc_component_update_bits(component, CDC_RX_RXn_RX_PATH_SEC7(rx, 1),
    CDC_RX_DSM_OUT_DELAY_SEL_MASK,
    CDC_RX_DSM_OUT_DELAY_TWO_SAMPLE);
    snd_soc_component_update_bits(component, CDC_RX_RXn_RX_PATH_SEC7(rx, 2),
    CDC_RX_DSM_OUT_DELAY_SEL_MASK,
    CDC_RX_DSM_OUT_DELAY_TWO_SAMPLE);
    snd_soc_component_update_bits(component, CDC_RX_RXn_RX_PATH_CFG3(rx, 0),
    CDC_RX_DC_COEFF_SEL_MASK,
    CDC_RX_DC_COEFF_SEL_TWO);
    snd_soc_component_update_bits(component, CDC_RX_RXn_RX_PATH_CFG3(rx, 1),
    CDC_RX_DC_COEFF_SEL_MASK,
    CDC_RX_DC_COEFF_SEL_TWO);
    snd_soc_component_update_bits(component, CDC_RX_RXn_RX_PATH_CFG3(rx, 2),
    CDC_RX_DC_COEFF_SEL_MASK,
    CDC_RX_DC_COEFF_SEL_TWO);
    switch (rx.codec_version) {
    case LPASS_CODEC_VERSION_1_0:
    case LPASS_CODEC_VERSION_1_1:
    case LPASS_CODEC_VERSION_1_2:
    case LPASS_CODEC_VERSION_2_0:
    case LPASS_CODEC_VERSION_2_1:
    controls = rx_macro_def_snd_controls;
    num_controls = ARRAY_SIZE(rx_macro_def_snd_controls);
    widgets = rx_macro_def_dapm_widgets;
    num_widgets = ARRAY_SIZE(rx_macro_def_dapm_widgets);
    break;
    case LPASS_CODEC_VERSION_2_5:
    case LPASS_CODEC_VERSION_2_6:
    case LPASS_CODEC_VERSION_2_7:
    case LPASS_CODEC_VERSION_2_8:
    controls = rx_macro_2_5_snd_controls;
    num_controls = ARRAY_SIZE(rx_macro_2_5_snd_controls);
    widgets = rx_macro_2_5_dapm_widgets;
    num_widgets = ARRAY_SIZE(rx_macro_2_5_dapm_widgets);
    break;
    default:
    return -EINVAL;
    }
    rx.component = component;
    ret = snd_soc_add_component_controls(component, controls, num_controls);
    if (ret)
    return ret;
    return snd_soc_dapm_new_controls(dapm, widgets, num_widgets);
    }
#[no_mangle]
unsafe extern "C" fn swclk_gate_enable(hw: *mut clk_hw) -> c_int {
    static int swclk_gate_enable(struct clk_hw *hw)
    {
    struct rx_macro *rx = to_rx_macro(hw);
    int ret;
    ret = pm_runtime_resume_and_get(rx.dev);
    if (ret < 0)
    return ret;
    ret = rx_macro_mclk_enable(rx, true);
    if (ret) {
    clk_disable_unprepare(rx.mclk);
    return ret;
    }
    regmap_update_bits(rx.regmap, CDC_RX_CLK_RST_CTRL_SWR_CONTROL,
    CDC_RX_SWR_CLK_EN_MASK, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn swclk_gate_disable(hw: *mut clk_hw) {
    static void swclk_gate_disable(struct clk_hw *hw)
    {
    struct rx_macro *rx = to_rx_macro(hw);
    regmap_update_bits(rx.regmap, CDC_RX_CLK_RST_CTRL_SWR_CONTROL,
    CDC_RX_SWR_CLK_EN_MASK, 0);
    rx_macro_mclk_enable(rx, false);
    pm_runtime_put_autosuspend(rx.dev);
    }
#[no_mangle]
unsafe extern "C" fn swclk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    static int swclk_gate_is_enabled(struct clk_hw *hw)
    {
    struct rx_macro *rx = to_rx_macro(hw);
    int ret, val;
    regmap_read(rx.regmap, CDC_RX_CLK_RST_CTRL_SWR_CONTROL, &val);
    ret = val & BIT(0);
    return ret;
    }
    static unsigned long swclk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return parent_rate / 2;
    }
    static const struct clk_ops swclk_gate_ops = {
    .prepare = swclk_gate_enable,
    .unprepare = swclk_gate_disable,
    .is_enabled = swclk_gate_is_enabled,
    .recalc_rate = swclk_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn rx_macro_register_mclk_output(rx: *mut rx_macro) -> c_int {
    static int rx_macro_register_mclk_output(struct rx_macro *rx)
    {
    struct device *dev = rx.dev;
    const char *parent_clk_name = core::ptr::null_mut();
    const char *clk_name = "lpass-rx-mclk";
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    if (rx.npl)
    parent_clk_name = __clk_get_name(rx.npl);
    else
    parent_clk_name = __clk_get_name(rx.mclk);
    init.name = clk_name;
    init.ops = &swclk_gate_ops;
    init.flags = 0;
    init.parent_names = &parent_clk_name;
    init.num_parents = 1;
    rx.hw.init = &init;
    hw = &rx.hw;
    ret = devm_clk_hw_register(rx.dev, hw);
    if (ret)
    return ret;
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, hw);
    }
    static const struct snd_soc_component_driver rx_macro_component_drv = {
    .name = "RX-MACRO",
    .probe = rx_macro_component_probe,
    .controls = rx_macro_snd_controls,
    .num_controls = ARRAY_SIZE(rx_macro_snd_controls),
    .dapm_widgets = rx_macro_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(rx_macro_dapm_widgets),
    .dapm_routes = rx_audio_map,
    .num_dapm_routes = ARRAY_SIZE(rx_audio_map),
    };
#[no_mangle]
unsafe extern "C" fn rx_macro_probe(pdev: *mut platform_device) -> c_int {
    static int rx_macro_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    kernel_ulong_t flags;
    struct rx_macro *rx;
    void __iomem *base;
    int ret, def_count;
    flags = (kernel_ulong_t)device_get_match_data(dev);
    rx = devm_kzalloc(dev, sizeof(*rx), GFP_KERNEL);
    if (!rx)
    return -ENOMEM;
    rx.macro = devm_clk_get_optional(dev, "macro");
    if (IS_ERR(rx.macro))
    return dev_err_probe(dev, PTR_ERR(rx.macro), "unable to get macro clock\n");
    rx.dcodec = devm_clk_get_optional(dev, "dcodec");
    if (IS_ERR(rx.dcodec))
    return dev_err_probe(dev, PTR_ERR(rx.dcodec), "unable to get dcodec clock\n");
    rx.mclk = devm_clk_get(dev, "mclk");
    if (IS_ERR(rx.mclk))
    return dev_err_probe(dev, PTR_ERR(rx.mclk), "unable to get mclk clock\n");
    if (flags & LPASS_MACRO_FLAG_HAS_NPL_CLOCK) {
    rx.npl = devm_clk_get(dev, "npl");
    if (IS_ERR(rx.npl))
    return dev_err_probe(dev, PTR_ERR(rx.npl), "unable to get npl clock\n");
    }
    rx.fsgen = devm_clk_get(dev, "fsgen");
    if (IS_ERR(rx.fsgen))
    return dev_err_probe(dev, PTR_ERR(rx.fsgen), "unable to get fsgen clock\n");
    rx.pds = lpass_macro_pds_init(dev);
    if (IS_ERR(rx.pds))
    return PTR_ERR(rx.pds);
    ret = devm_add_action_or_reset(dev, lpass_macro_pds_exit_action, rx.pds);
    if (ret)
    return ret;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    rx.codec_version = lpass_macro_get_codec_version();
    struct reg_default *reg_defaults __free(kfree) = core::ptr::null_mut();
    switch (rx.codec_version) {
    case LPASS_CODEC_VERSION_1_0:
    case LPASS_CODEC_VERSION_1_1:
    case LPASS_CODEC_VERSION_1_2:
    case LPASS_CODEC_VERSION_2_0:
    case LPASS_CODEC_VERSION_2_1:
    rx.rxn_reg_stride = 0x80;
    rx.rxn_reg_stride2 = 0xc;
    def_count = ARRAY_SIZE(rx_defaults) + ARRAY_SIZE(rx_pre_2_5_defaults);
    reg_defaults = kmalloc_objs(struct reg_default, def_count);
    if (!reg_defaults)
    return -ENOMEM;
    memcpy(&reg_defaults[0], rx_defaults, sizeof(rx_defaults));
    memcpy(&reg_defaults[ARRAY_SIZE(rx_defaults)],
    rx_pre_2_5_defaults, sizeof(rx_pre_2_5_defaults));
    break;
    case LPASS_CODEC_VERSION_2_5:
    case LPASS_CODEC_VERSION_2_6:
    case LPASS_CODEC_VERSION_2_7:
    case LPASS_CODEC_VERSION_2_8:
    rx.rxn_reg_stride = 0xc0;
    rx.rxn_reg_stride2 = 0x0;
    def_count = ARRAY_SIZE(rx_defaults) + ARRAY_SIZE(rx_2_5_defaults);
    reg_defaults = kmalloc_objs(struct reg_default, def_count);
    if (!reg_defaults)
    return -ENOMEM;
    memcpy(&reg_defaults[0], rx_defaults, sizeof(rx_defaults));
    memcpy(&reg_defaults[ARRAY_SIZE(rx_defaults)],
    rx_2_5_defaults, sizeof(rx_2_5_defaults));
    break;
    default:
    dev_err(dev, "Unsupported Codec version (%d)\n", rx.codec_version);
    return -EINVAL;
    }
    struct regmap_config *reg_config __free(kfree) = kmemdup(&rx_regmap_config,
    sizeof(*reg_config),
    GFP_KERNEL);
    if (!reg_config)
    return -ENOMEM;
    reg_config.reg_defaults = reg_defaults;
    reg_config.num_reg_defaults = def_count;
    rx.regmap = devm_regmap_init_mmio(dev, base, reg_config);
    if (IS_ERR(rx.regmap))
    return PTR_ERR(rx.regmap);
    dev_set_drvdata(dev, rx);
    rx.dev = dev;
// set MCLK and NPL rates
    ret = clk_set_rate(rx.mclk, MCLK_FREQ);
    if (ret)
    return ret;
    ret = clk_set_rate(rx.npl, MCLK_FREQ);
    if (ret)
    return ret;
    ret = devm_pm_clk_create(dev);
    if (ret)
    return ret;
    ret = of_pm_clk_add_clks(dev);
    if (ret < 0)
    return ret;
    pm_runtime_set_autosuspend_delay(dev, 100);
    pm_runtime_use_autosuspend(dev);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    return ret;
// reset swr block
    regmap_update_bits(rx.regmap, CDC_RX_CLK_RST_CTRL_SWR_CONTROL,
    CDC_RX_SWR_RESET_MASK,
    CDC_RX_SWR_RESET);
    regmap_update_bits(rx.regmap, CDC_RX_CLK_RST_CTRL_SWR_CONTROL,
    CDC_RX_SWR_CLK_EN_MASK, 1);
    regmap_update_bits(rx.regmap, CDC_RX_CLK_RST_CTRL_SWR_CONTROL,
    CDC_RX_SWR_RESET_MASK, 0);
    ret = devm_snd_soc_register_component(dev, &rx_macro_component_drv,
    rx_macro_dai,
    ARRAY_SIZE(rx_macro_dai));
    if (ret)
    goto err_rpm_put;
    ret = rx_macro_register_mclk_output(rx);
    if (ret)
    goto err_rpm_put;
    ret = pm_runtime_put_autosuspend(dev);
    if (ret < 0)
    dev_warn(dev, "runtime PM put failed after probe: %d\n", ret);
    return 0;
    err_rpm_put:
    if (pm_runtime_put_sync_suspend(dev) < 0)
    dev_warn(dev, "runtime PM sync suspend failed in probe unwind\n");
    return ret;
    }
    static const struct of_device_id rx_macro_dt_match[] = {
    {
    .compatible = "qcom,sc7280-lpass-rx-macro",
    .data = (void *)LPASS_MACRO_FLAG_HAS_NPL_CLOCK,
    }, {
    .compatible = "qcom,sm6115-lpass-rx-macro",
    .data = (void *)LPASS_MACRO_FLAG_HAS_NPL_CLOCK,
    }, {
    .compatible = "qcom,sm8250-lpass-rx-macro",
    .data = (void *)LPASS_MACRO_FLAG_HAS_NPL_CLOCK,
    }, {
    .compatible = "qcom,sm8450-lpass-rx-macro",
    .data = (void *)LPASS_MACRO_FLAG_HAS_NPL_CLOCK,
    }, {
    .compatible = "qcom,sm8550-lpass-rx-macro",
    }, {
    .compatible = "qcom,sc8280xp-lpass-rx-macro",
    .data = (void *)LPASS_MACRO_FLAG_HAS_NPL_CLOCK,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, rx_macro_dt_match);
#[no_mangle]
unsafe extern "C" fn rx_macro_runtime_suspend(dev: *mut device) -> c_int {
    static int rx_macro_runtime_suspend(struct device *dev)
    {
    struct rx_macro *rx = dev_get_drvdata(dev);
    int ret;
    regcache_cache_only(rx.regmap, true);
    ret = pm_clk_suspend(dev);
    if (ret) {
    regcache_cache_only(rx.regmap, false);
    return ret;
    }
    regcache_mark_dirty(rx.regmap);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rx_macro_runtime_resume(dev: *mut device) -> c_int {
    static int rx_macro_runtime_resume(struct device *dev)
    {
    struct rx_macro *rx = dev_get_drvdata(dev);
    int ret;
    ret = pm_clk_resume(dev);
    if (ret) {
    regcache_cache_only(rx.regmap, true);
    regcache_mark_dirty(rx.regmap);
    return ret;
    }
    regcache_cache_only(rx.regmap, false);
    ret = regcache_sync(rx.regmap);
    if (ret) {
    regcache_cache_only(rx.regmap, true);
    regcache_mark_dirty(rx.regmap);
    pm_clk_suspend(dev);
    return ret;
    }
    return 0;
    }
    static const struct dev_pm_ops rx_macro_pm_ops = {
    RUNTIME_PM_OPS(rx_macro_runtime_suspend, rx_macro_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver rx_macro_driver = {
    .driver = {
    .name = "rx_macro",
    .of_match_table = rx_macro_dt_match,
    .suppress_bind_attrs = true,
    .pm = pm_ptr(&rx_macro_pm_ops),
    },
    .probe = rx_macro_probe,
    };
    module_platform_driver(rx_macro_driver);
    MODULE_DESCRIPTION("RX macro driver");
    MODULE_LICENSE("GPL");
