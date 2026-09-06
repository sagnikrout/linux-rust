//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/codecs/cirrus/cs8409.h
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
// HD audio codec driver for Cirrus Logic CS8409 HDA bridge chip
//
// Copyright (C) 2021 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

// CS8409 Specific Definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs8409_pins {
    CS8409_PIN_ROOT,
    CS8409_PIN_AFG,
    CS8409_PIN_ASP1_OUT_A,
    CS8409_PIN_ASP1_OUT_B,
    CS8409_PIN_ASP1_OUT_C,
    CS8409_PIN_ASP1_OUT_D,
    CS8409_PIN_ASP1_OUT_E,
    CS8409_PIN_ASP1_OUT_F,
    CS8409_PIN_ASP1_OUT_G,
    CS8409_PIN_ASP1_OUT_H,
    CS8409_PIN_ASP2_OUT_A,
    CS8409_PIN_ASP2_OUT_B,
    CS8409_PIN_ASP2_OUT_C,
    CS8409_PIN_ASP2_OUT_D,
    CS8409_PIN_ASP2_OUT_E,
    CS8409_PIN_ASP2_OUT_F,
    CS8409_PIN_ASP2_OUT_G,
    CS8409_PIN_ASP2_OUT_H,
    CS8409_PIN_ASP1_IN_A,
    CS8409_PIN_ASP1_IN_B,
    CS8409_PIN_ASP1_IN_C,
    CS8409_PIN_ASP1_IN_D,
    CS8409_PIN_ASP1_IN_E,
    CS8409_PIN_ASP1_IN_F,
    CS8409_PIN_ASP1_IN_G,
    CS8409_PIN_ASP1_IN_H,
    CS8409_PIN_ASP2_IN_A,
    CS8409_PIN_ASP2_IN_B,
    CS8409_PIN_ASP2_IN_C,
    CS8409_PIN_ASP2_IN_D,
    CS8409_PIN_ASP2_IN_E,
    CS8409_PIN_ASP2_IN_F,
    CS8409_PIN_ASP2_IN_G,
    CS8409_PIN_ASP2_IN_H,
    CS8409_PIN_DMIC1,
    CS8409_PIN_DMIC2,
    CS8409_PIN_ASP1_TRANSMITTER_A,
    CS8409_PIN_ASP1_TRANSMITTER_B,
    CS8409_PIN_ASP1_TRANSMITTER_C,
    CS8409_PIN_ASP1_TRANSMITTER_D,
    CS8409_PIN_ASP1_TRANSMITTER_E,
    CS8409_PIN_ASP1_TRANSMITTER_F,
    CS8409_PIN_ASP1_TRANSMITTER_G,
    CS8409_PIN_ASP1_TRANSMITTER_H,
    CS8409_PIN_ASP2_TRANSMITTER_A,
    CS8409_PIN_ASP2_TRANSMITTER_B,
    CS8409_PIN_ASP2_TRANSMITTER_C,
    CS8409_PIN_ASP2_TRANSMITTER_D,
    CS8409_PIN_ASP2_TRANSMITTER_E,
    CS8409_PIN_ASP2_TRANSMITTER_F,
    CS8409_PIN_ASP2_TRANSMITTER_G,
    CS8409_PIN_ASP2_TRANSMITTER_H,
    CS8409_PIN_ASP1_RECEIVER_A,
    CS8409_PIN_ASP1_RECEIVER_B,
    CS8409_PIN_ASP1_RECEIVER_C,
    CS8409_PIN_ASP1_RECEIVER_D,
    CS8409_PIN_ASP1_RECEIVER_E,
    CS8409_PIN_ASP1_RECEIVER_F,
    CS8409_PIN_ASP1_RECEIVER_G,
    CS8409_PIN_ASP1_RECEIVER_H,
    CS8409_PIN_ASP2_RECEIVER_A,
    CS8409_PIN_ASP2_RECEIVER_B,
    CS8409_PIN_ASP2_RECEIVER_C,
    CS8409_PIN_ASP2_RECEIVER_D,
    CS8409_PIN_ASP2_RECEIVER_E,
    CS8409_PIN_ASP2_RECEIVER_F,
    CS8409_PIN_ASP2_RECEIVER_G,
    CS8409_PIN_ASP2_RECEIVER_H,
    CS8409_PIN_DMIC1_IN,
    CS8409_PIN_DMIC2_IN,
    CS8409_PIN_BEEP_GEN,
    CS8409_PIN_VENDOR_WIDGET
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs8409_coefficient_index_registers {
    CS8409_DEV_CFG1,
    CS8409_DEV_CFG2,
    CS8409_DEV_CFG3,
    CS8409_ASP1_CLK_CTRL1,
    CS8409_ASP1_CLK_CTRL2,
    CS8409_ASP1_CLK_CTRL3,
    CS8409_ASP2_CLK_CTRL1,
    CS8409_ASP2_CLK_CTRL2,
    CS8409_ASP2_CLK_CTRL3,
    CS8409_DMIC_CFG,
    CS8409_BEEP_CFG,
    ASP1_RX_NULL_INS_RMV,
    ASP1_Rx_RATE1,
    ASP1_Rx_RATE2,
    ASP1_Tx_NULL_INS_RMV,
    ASP1_Tx_RATE1,
    ASP1_Tx_RATE2,
    ASP2_Rx_NULL_INS_RMV,
    ASP2_Rx_RATE1,
    ASP2_Rx_RATE2,
    ASP2_Tx_NULL_INS_RMV,
    ASP2_Tx_RATE1,
    ASP2_Tx_RATE2,
    ASP1_SYNC_CTRL,
    ASP2_SYNC_CTRL,
    ASP1_A_TX_CTRL1,
    ASP1_A_TX_CTRL2,
    ASP1_B_TX_CTRL1,
    ASP1_B_TX_CTRL2,
    ASP1_C_TX_CTRL1,
    ASP1_C_TX_CTRL2,
    ASP1_D_TX_CTRL1,
    ASP1_D_TX_CTRL2,
    ASP1_E_TX_CTRL1,
    ASP1_E_TX_CTRL2,
    ASP1_F_TX_CTRL1,
    ASP1_F_TX_CTRL2,
    ASP1_G_TX_CTRL1,
    ASP1_G_TX_CTRL2,
    ASP1_H_TX_CTRL1,
    ASP1_H_TX_CTRL2,
    ASP2_A_TX_CTRL1,
    ASP2_A_TX_CTRL2,
    ASP2_B_TX_CTRL1,
    ASP2_B_TX_CTRL2,
    ASP2_C_TX_CTRL1,
    ASP2_C_TX_CTRL2,
    ASP2_D_TX_CTRL1,
    ASP2_D_TX_CTRL2,
    ASP2_E_TX_CTRL1,
    ASP2_E_TX_CTRL2,
    ASP2_F_TX_CTRL1,
    ASP2_F_TX_CTRL2,
    ASP2_G_TX_CTRL1,
    ASP2_G_TX_CTRL2,
    ASP2_H_TX_CTRL1,
    ASP2_H_TX_CTRL2,
    ASP1_A_RX_CTRL1,
    ASP1_A_RX_CTRL2,
    ASP1_B_RX_CTRL1,
    ASP1_B_RX_CTRL2,
    ASP1_C_RX_CTRL1,
    ASP1_C_RX_CTRL2,
    ASP1_D_RX_CTRL1,
    ASP1_D_RX_CTRL2,
    ASP1_E_RX_CTRL1,
    ASP1_E_RX_CTRL2,
    ASP1_F_RX_CTRL1,
    ASP1_F_RX_CTRL2,
    ASP1_G_RX_CTRL1,
    ASP1_G_RX_CTRL2,
    ASP1_H_RX_CTRL1,
    ASP1_H_RX_CTRL2,
    ASP2_A_RX_CTRL1,
    ASP2_A_RX_CTRL2,
    ASP2_B_RX_CTRL1,
    ASP2_B_RX_CTRL2,
    ASP2_C_RX_CTRL1,
    ASP2_C_RX_CTRL2,
    ASP2_D_RX_CTRL1,
    ASP2_D_RX_CTRL2,
    ASP2_E_RX_CTRL1,
    ASP2_E_RX_CTRL2,
    ASP2_F_RX_CTRL1,
    ASP2_F_RX_CTRL2,
    ASP2_G_RX_CTRL1,
    ASP2_G_RX_CTRL2,
    ASP2_H_RX_CTRL1,
    ASP2_H_RX_CTRL2,
    CS8409_I2C_ADDR,
    CS8409_I2C_DATA,
    CS8409_I2C_CTRL,
    CS8409_I2C_STS,
    CS8409_I2C_QWRITE,
    CS8409_I2C_QREAD,
    CS8409_SPI_CTRL,
    CS8409_SPI_TX_DATA,
    CS8409_SPI_RX_DATA,
    CS8409_SPI_STS,
    CS8409_PFE_COEF_W1, /* Parametric filter engine coefficient write 1*/
    CS8409_PFE_COEF_W2,
    CS8409_PFE_CTRL1,
    CS8409_PFE_CTRL2,
    CS8409_PRE_SCALE_ATTN1,
    CS8409_PRE_SCALE_ATTN2,
    CS8409_PFE_COEF_MON1, /* Parametric filter engine coefficient monitor 1*/
    CS8409_PFE_COEF_MON2,
    CS8409_ASP1_INTRN_STS,
    CS8409_ASP2_INTRN_STS,
    CS8409_ASP1_RX_SCLK_COUNT,
    CS8409_ASP1_TX_SCLK_COUNT,
    CS8409_ASP2_RX_SCLK_COUNT,
    CS8409_ASP2_TX_SCLK_COUNT,
    CS8409_ASP_UNS_RESP_MASK,
    CS8409_LOOPBACK_CTRL = 0x80,
    CS8409_PAD_CFG_SLW_RATE_CTRL = 0x82, /* Pad Config and Slew Rate Control (CIR = 0x0082) */
}

// CS42L42 Specific Definitions
pub const CS8409_MAX_CODECS: c_int = 8;

// Dell BULLSEYE / WARLOCK / CYBORG Specific Definitions

// Dolphin

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs8409_i2c_param {
    pub addr: c_uint,
    pub value: c_uint,
    pub delay: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs8409_cir_param {
    pub nid: c_uint,
    pub cir: c_uint,
    pub coeff: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sub_codec {
    pub codec: *mut hda_codec,
    pub addr: c_uint,
    pub reset_gpio: c_uint,
    pub irq_mask: c_uint,
    pub init_seq: *const cs8409_i2c_param,
    pub init_seq_num: c_uint,
    pub hp_jack_in:1: c_uint,
    pub mic_jack_in:1: c_uint,
    pub suspended:1: c_uint,
    pub paged:1: c_uint,
    pub last_page: c_uint,
    pub hsbias_hiz: c_uint,
    pub full_scale_vol:1: c_uint,
    pub no_type_dect:1: c_uint,
    pub vol: [i8; CS42L42_VOLUMES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs8409_spec {
    pub gen: hda_gen_spec,
    pub codec: *mut hda_codec,
    pub scodecs: [*mut sub_codec; CS8409_MAX_CODECS],
    pub num_scodecs: c_uint,
    pub gpio_mask: c_uint,
    pub gpio_dir: c_uint,
    pub gpio_data: c_uint,
    pub speaker_pdn_gpio: c_int,
    pub i2c_mux: mutex,
    pub i2c_clck_enabled: c_uint,
    pub dev_addr: c_uint,
    pub i2c_clk_work: delayed_work,
    pub playback_started:1: c_uint,
    pub capture_started:1: c_uint,
    pub init_done:1: c_uint,
    pub build_ctrl_done:1: c_uint,
    pub speaker_muted:1: c_uint,
// verb exec op override
    pub res): *mut c_uint,
// unsol_event op override
    pub res): *mut *mut *mut void (unsol_event)(struct hda_codec codec, unsigned int,
// component binding
    pub match: *mut component_match,
    pub comps: hda_component_parent,
}

extern "C" {
    pub fn cs42l42_volume_info(kctrl: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
}
extern "C" {
    pub fn cs42l42_volume_get(kctrl: *mut snd_kcontrol, uctrl: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn cs42l42_volume_put(kctrl: *mut snd_kcontrol, uctrl: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn cs8409_cs42l42_fixups(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
}
extern "C" {
    pub fn dolphin_fixups(codec: *mut hda_codec, fix: *const hda_fixup, action: c_int);
}
