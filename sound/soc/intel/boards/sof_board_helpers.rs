//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/boards/sof_board_helpers.h
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
// Copyright(c) 2023 Intel Corporation.
//

//
// Common board quirks: from bit 8 to 31, LSB 8 bits reserved for machine
// drivers
//
// SSP port number for headphone codec: 3 bits
pub const SOF_SSP_PORT_CODEC_SHIFT: c_int = 8;

// SSP port number for speaker amplifier: 3 bits
pub const SOF_SSP_PORT_AMP_SHIFT: c_int = 11;

// SSP port number for BT audio offload: 3 bits
pub const SOF_SSP_PORT_BT_OFFLOAD_SHIFT: c_int = 14;

// SSP port mask for HDMI capture: 6 bits
pub const SOF_SSP_MASK_HDMI_CAPTURE_SHIFT: c_int = 17;

// Number of idisp HDMI BE link: 3 bits
pub const SOF_NUM_IDISP_HDMI_SHIFT: c_int = 23;

// Board uses BT audio offload

//
// sof_da7219_private: private data for da7219 machine driver
//
// @mclk_en: true for mclk pin is connected
// @pll_bypass: true for PLL bypass mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_da7219_private {
    pub mclk_en: bool,
    pub pll_bypass: bool,
}

//
// sof_rt5682_private: private data for rt5682 machine driver
//
// @mclk: mclk clock data
// @is_legacy_cpu: true for BYT/CHT boards
// @mclk_en: true for mclk pin is connected
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_rt5682_private {
    pub mclk: *mut clk,
    pub is_legacy_cpu: bool,
    pub mclk_en: bool,
}

//
// sof_card_private: common data for machine drivers
//
// @headset_jack: headset jack data
// @hdmi: init data for hdmi dai link
// @codec_type: type of headset codec
// @amp_type: type of speaker amplifier
// @dmic_be_num: number of Intel PCH DMIC BE link
// @hdmi_num: number of Intel HDMI BE link
// @ssp_codec: ssp port number of headphone BE link
// @ssp_amp: ssp port number of speaker BE link
// @ssp_bt: ssp port number of BT offload BE link
// @ssp_mask_hdmi_in: ssp port mask of HDMI-IN BE link
// @bt_offload_present: true to create BT offload BE link
// @hda_codec_present: true to create HDA codec BE links
// @codec_link: pointer to headset codec dai link
// @amp_link: pointer to speaker amplifier dai link
// @link_order_overwrite: custom DAI link order
// @link_id_overwrite: custom DAI link ID
// @da7219: private data for da7219 machine driver
// @rt5682: private data for rt5682 machine driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_card_private {
    pub headset_jack: snd_soc_jack,
    pub hdmi: sof_hdmi_private,
    pub codec_type: snd_soc_acpi_intel_codec,
    pub amp_type: snd_soc_acpi_intel_codec,
    pub dmic_be_num: c_int,
    pub hdmi_num: c_int,
    pub ssp_codec: c_int,
    pub ssp_amp: c_int,
    pub ssp_bt: c_int,
    pub ssp_mask_hdmi_in: c_ulong,
    pub bt_offload_present: bool,
    pub hda_codec_present: bool,
    pub codec_link: *mut snd_soc_dai_link,
    pub amp_link: *mut snd_soc_dai_link,
    pub link_order_overwrite: c_ulong,
//
// A variable stores id for all BE DAI links, use SOF_LINK_IDS macro to
// build the value; use DAI link array index as id if zero.
//
    pub link_id_overwrite: c_ulong,
    pub da7219: sof_da7219_private,
    pub rt5682: sof_rt5682_private,
}

extern "C" {
    pub fn sof_intel_board_card_late_probe(card: *mut snd_soc_card) -> c_int;
}
