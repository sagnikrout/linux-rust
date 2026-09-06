//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/sof/tokens.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
// Keyon Jie <yang.jie@linux.intel.com>
//
// Topology IDs and tokens.
//
// ** MUST BE ALIGNED WITH TOPOLOGY CONFIGURATION TOKEN VALUES
//
// Kcontrol IDs
//
pub const SOF_TPLG_KCTL_VOL_ID: c_int = 256;
pub const SOF_TPLG_KCTL_ENUM_ID: c_int = 257;
pub const SOF_TPLG_KCTL_BYTES_ID: c_int = 258;
pub const SOF_TPLG_KCTL_SWITCH_ID: c_int = 259;
pub const SOF_TPLG_KCTL_BYTES_VOLATILE_RO: c_int = 260;
pub const SOF_TPLG_KCTL_BYTES_VOLATILE_RW: c_int = 261;
pub const SOF_TPLG_KCTL_BYTES_WO_ID: c_int = 262;
//
// Tokens - must match values in topology configurations
//
// buffers
pub const SOF_TKN_BUF_SIZE: c_int = 100;
pub const SOF_TKN_BUF_CAPS: c_int = 101;
pub const SOF_TKN_BUF_FLAGS: c_int = 102;
// DAI
// Token retired with ABI 3.2, do not use for new capabilities
// #define	SOF_TKN_DAI_DMAC_CONFIG			153
//
pub const SOF_TKN_DAI_TYPE: c_int = 154;
pub const SOF_TKN_DAI_INDEX: c_int = 155;
pub const SOF_TKN_DAI_DIRECTION: c_int = 156;
// scheduling
pub const SOF_TKN_SCHED_PERIOD: c_int = 200;
pub const SOF_TKN_SCHED_PRIORITY: c_int = 201;
pub const SOF_TKN_SCHED_MIPS: c_int = 202;
pub const SOF_TKN_SCHED_CORE: c_int = 203;
pub const SOF_TKN_SCHED_FRAMES: c_int = 204;
pub const SOF_TKN_SCHED_TIME_DOMAIN: c_int = 205;
pub const SOF_TKN_SCHED_DYNAMIC_PIPELINE: c_int = 206;
pub const SOF_TKN_SCHED_LP_MODE: c_int = 207;
pub const SOF_TKN_SCHED_MEM_USAGE: c_int = 208;
pub const SOF_TKN_SCHED_USE_CHAIN_DMA: c_int = 209;
pub const SOF_TKN_SCHED_KCPS: c_int = 210;
pub const SOF_TKN_SCHED_DIRECTION: c_int = 211;
pub const SOF_TKN_SCHED_DIRECTION_VALID: c_int = 212;
// volume
pub const SOF_TKN_VOLUME_RAMP_STEP_TYPE: c_int = 250;
pub const SOF_TKN_VOLUME_RAMP_STEP_MS: c_int = 251;
pub const SOF_TKN_GAIN_RAMP_TYPE: c_int = 260;
pub const SOF_TKN_GAIN_RAMP_DURATION: c_int = 261;
pub const SOF_TKN_GAIN_VAL: c_int = 262;
// SRC
pub const SOF_TKN_SRC_RATE_IN: c_int = 300;
pub const SOF_TKN_SRC_RATE_OUT: c_int = 301;
// ASRC
pub const SOF_TKN_ASRC_RATE_IN: c_int = 320;
pub const SOF_TKN_ASRC_RATE_OUT: c_int = 321;
pub const SOF_TKN_ASRC_ASYNCHRONOUS_MODE: c_int = 322;
pub const SOF_TKN_ASRC_OPERATION_MODE: c_int = 323;
// PCM
pub const SOF_TKN_PCM_DMAC_CONFIG: c_int = 353;
// Generic components
pub const SOF_TKN_COMP_PERIOD_SINK_COUNT: c_int = 400;
pub const SOF_TKN_COMP_PERIOD_SOURCE_COUNT: c_int = 401;
pub const SOF_TKN_COMP_FORMAT: c_int = 402;
// Token retired with ABI 3.2, do not use for new capabilities
// #define SOF_TKN_COMP_PRELOAD_COUNT		403
//
pub const SOF_TKN_COMP_CORE_ID: c_int = 404;
pub const SOF_TKN_COMP_UUID: c_int = 405;
pub const SOF_TKN_COMP_CPC: c_int = 406;
pub const SOF_TKN_COMP_IS_PAGES: c_int = 409;
pub const SOF_TKN_COMP_NUM_AUDIO_FORMATS: c_int = 410;
pub const SOF_TKN_COMP_NUM_INPUT_PINS: c_int = 411;
pub const SOF_TKN_COMP_NUM_OUTPUT_PINS: c_int = 412;
//
// The token for input/output pin binding, it specifies the widget
// name that the input/output pin is connected from/to.
//
pub const SOF_TKN_COMP_INPUT_PIN_BINDING_WNAME: c_int = 413;
pub const SOF_TKN_COMP_OUTPUT_PIN_BINDING_WNAME: c_int = 414;
pub const SOF_TKN_COMP_NUM_INPUT_AUDIO_FORMATS: c_int = 415;
pub const SOF_TKN_COMP_NUM_OUTPUT_AUDIO_FORMATS: c_int = 416;
//
// The token value is copied to the dapm_widget's
// no_wname_in_kcontrol_name.
//
pub const SOF_TKN_COMP_NO_WNAME_IN_KCONTROL_NAME: c_int = 417;
pub const SOF_TKN_COMP_SCHED_DOMAIN: c_int = 418;
pub const SOF_TKN_COMP_DOMAIN_ID: c_int = 419;
pub const SOF_TKN_COMP_STACK_BYTES_REQUIREMENT: c_int = 420;
pub const SOF_TKN_COMP_HEAP_BYTES_REQUIREMENT: c_int = 421;
// SSP
pub const SOF_TKN_INTEL_SSP_CLKS_CONTROL: c_int = 500;
pub const SOF_TKN_INTEL_SSP_MCLK_ID: c_int = 501;
pub const SOF_TKN_INTEL_SSP_SAMPLE_BITS: c_int = 502;
pub const SOF_TKN_INTEL_SSP_FRAME_PULSE_WIDTH: c_int = 503;
pub const SOF_TKN_INTEL_SSP_QUIRKS: c_int = 504;
pub const SOF_TKN_INTEL_SSP_TDM_PADDING_PER_SLOT: c_int = 505;
pub const SOF_TKN_INTEL_SSP_BCLK_DELAY: c_int = 506;
// DMIC
pub const SOF_TKN_INTEL_DMIC_DRIVER_VERSION: c_int = 600;
pub const SOF_TKN_INTEL_DMIC_CLK_MIN: c_int = 601;
pub const SOF_TKN_INTEL_DMIC_CLK_MAX: c_int = 602;
pub const SOF_TKN_INTEL_DMIC_DUTY_MIN: c_int = 603;
pub const SOF_TKN_INTEL_DMIC_DUTY_MAX: c_int = 604;
pub const SOF_TKN_INTEL_DMIC_NUM_PDM_ACTIVE: c_int = 605;
pub const SOF_TKN_INTEL_DMIC_SAMPLE_RATE: c_int = 608;
pub const SOF_TKN_INTEL_DMIC_FIFO_WORD_LENGTH: c_int = 609;
pub const SOF_TKN_INTEL_DMIC_UNMUTE_RAMP_TIME_MS: c_int = 610;
// DMIC PDM
pub const SOF_TKN_INTEL_DMIC_PDM_CTRL_ID: c_int = 700;
pub const SOF_TKN_INTEL_DMIC_PDM_MIC_A_Enable: c_int = 701;
pub const SOF_TKN_INTEL_DMIC_PDM_MIC_B_Enable: c_int = 702;
pub const SOF_TKN_INTEL_DMIC_PDM_POLARITY_A: c_int = 703;
pub const SOF_TKN_INTEL_DMIC_PDM_POLARITY_B: c_int = 704;
pub const SOF_TKN_INTEL_DMIC_PDM_CLK_EDGE: c_int = 705;
pub const SOF_TKN_INTEL_DMIC_PDM_SKEW: c_int = 706;
// Tone
pub const SOF_TKN_TONE_SAMPLE_RATE: c_int = 800;
// Processing Components
pub const SOF_TKN_PROCESS_TYPE: c_int = 900;
// for backward compatibility

// SAI
pub const SOF_TKN_IMX_SAI_MCLK_ID: c_int = 1000;
// ESAI
pub const SOF_TKN_IMX_ESAI_MCLK_ID: c_int = 1100;
// Stream
pub const SOF_TKN_STREAM_PLAYBACK_COMPATIBLE_D0I3: c_int = 1200;
pub const SOF_TKN_STREAM_CAPTURE_COMPATIBLE_D0I3: c_int = 1201;
pub const SOF_TKN_STREAM_PLAYBACK_PAUSE_SUPPORTED: c_int = 1202;
pub const SOF_TKN_STREAM_CAPTURE_PAUSE_SUPPORTED: c_int = 1203;
// Led control for mute switches
pub const SOF_TKN_MUTE_LED_USE: c_int = 1300;
pub const SOF_TKN_MUTE_LED_DIRECTION: c_int = 1301;
// ALH
pub const SOF_TKN_INTEL_ALH_RATE: c_int = 1400;
pub const SOF_TKN_INTEL_ALH_CH: c_int = 1401;
// HDA
pub const SOF_TKN_INTEL_HDA_RATE: c_int = 1500;
pub const SOF_TKN_INTEL_HDA_CH: c_int = 1501;
// AFE
pub const SOF_TKN_MEDIATEK_AFE_RATE: c_int = 1600;
pub const SOF_TKN_MEDIATEK_AFE_CH: c_int = 1601;
pub const SOF_TKN_MEDIATEK_AFE_FORMAT: c_int = 1602;
// MIXER
pub const SOF_TKN_MIXER_TYPE: c_int = 1700;
// ACPDMIC
pub const SOF_TKN_AMD_ACPDMIC_RATE: c_int = 1800;
pub const SOF_TKN_AMD_ACPDMIC_CH: c_int = 1801;
// CAVS AUDIO FORMAT
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_RATE: c_int = 1900;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_BIT_DEPTH: c_int = 1901;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_VALID_BIT_DEPTH: c_int = 1902;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_CHANNELS: c_int = 1903;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_CH_MAP: c_int = 1904;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_CH_CFG: c_int = 1905;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_INTERLEAVING_STYLE: c_int = 1906;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_FMT_CFG: c_int = 1907;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IN_SAMPLE_TYPE: c_int = 1908;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_INPUT_PIN_INDEX: c_int = 1909;
// intentional token numbering discontinuity, reserved for future use
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_RATE: c_int = 1930;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_BIT_DEPTH: c_int = 1931;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_VALID_BIT_DEPTH: c_int = 1932;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_CHANNELS: c_int = 1933;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_CH_MAP: c_int = 1934;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_CH_CFG: c_int = 1935;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_INTERLEAVING_STYLE: c_int = 1936;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_FMT_CFG: c_int = 1937;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUT_SAMPLE_TYPE: c_int = 1938;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OUTPUT_PIN_INDEX: c_int = 1939;
// intentional token numbering discontinuity, reserved for future use
pub const SOF_TKN_CAVS_AUDIO_FORMAT_IBS: c_int = 1970;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_OBS: c_int = 1971;
pub const SOF_TKN_CAVS_AUDIO_FORMAT_DMA_BUFFER_SIZE: c_int = 1972;
// COPIER
pub const SOF_TKN_INTEL_COPIER_NODE_TYPE: c_int = 1980;
pub const SOF_TKN_INTEL_COPIER_DEEP_BUFFER_DMA_MS: c_int = 1981;
// ACP I2S
pub const SOF_TKN_AMD_ACPI2S_RATE: c_int = 1700;
pub const SOF_TKN_AMD_ACPI2S_CH: c_int = 1701;
pub const SOF_TKN_AMD_ACPI2S_TDM_MODE: c_int = 1702;
pub const SOF_TKN_AMD_ACPI2S_FORMAT: c_int = 1703;
// MICFIL PDM
pub const SOF_TKN_IMX_MICFIL_RATE: c_int = 2000;
pub const SOF_TKN_IMX_MICFIL_CH: c_int = 2001;
// ACP SDW
pub const SOF_TKN_AMD_ACP_SDW_RATE: c_int = 2100;
pub const SOF_TKN_AMD_ACP_SDW_CH: c_int = 2101;
