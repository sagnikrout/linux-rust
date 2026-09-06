//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/analogix/analogix_dp_core.h
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
// Header file for Analogix DP (Display Port) core interface driver.
//
// Copyright (C) 2012 Samsung Electronics Co., Ltd.
// Author: Jingoo Han <jg1.han@samsung.com>
//

pub const DP_TIMEOUT_LOOP_COUNT: c_int = 100;
pub const MAX_CR_LOOP: c_int = 5;
pub const MAX_EQ_LOOP: c_int = 5;
pub const MAX_PLL_LOCK_LOOP: c_int = 5;
// Training takes 22ms if AUX channel comm fails. Use this as retry interval
pub const DP_TIMEOUT_TRAINING_US: c_int = 22000;
pub const DP_TIMEOUT_PSR_LOOP_MS: c_int = 300;
// DP_MAX_LANE_COUNT

// DP_LANE_COUNT_SET

// DP_TRAINING_LANE0_SET

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_lane_count_type {
    LANE_COUNT1 = 1,
    LANE_COUNT2 = 2,
    LANE_COUNT4 = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_training_state {
    START,
    CLOCK_RECOVERY,
    EQUALIZER_TRAINING,
    FINISHED,
    FAILED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum voltage_swing_level {
    VOLTAGE_LEVEL_0,
    VOLTAGE_LEVEL_1,
    VOLTAGE_LEVEL_2,
    VOLTAGE_LEVEL_3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pre_emphasis_level {
    PRE_EMPHASIS_LEVEL_0,
    PRE_EMPHASIS_LEVEL_1,
    PRE_EMPHASIS_LEVEL_2,
    PRE_EMPHASIS_LEVEL_3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pattern_set {
    PRBS7,
    D10_2,
    TRAINING_PTN1,
    TRAINING_PTN2,
    DP_NONE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum color_space {
    COLOR_RGB,
    COLOR_YCBCR422,
    COLOR_YCBCR444
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum color_depth {
    COLOR_6,
    COLOR_8,
    COLOR_10,
    COLOR_12
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum color_coefficient {
    COLOR_YCBCR601,
    COLOR_YCBCR709
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dynamic_range {
    VESA,
    CEA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clock_recovery_m_value_type {
    CALCULATED_M,
    REGISTER_M
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum video_timing_recognition_type {
    VIDEO_TIMING_FROM_CAPTURE,
    VIDEO_TIMING_FROM_REGISTER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum analog_power_block {
    AUX_BLOCK,
    CH0_BLOCK,
    CH1_BLOCK,
    CH2_BLOCK,
    CH3_BLOCK,
    ANALOG_TOTAL,
    POWER_ALL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_irq_type {
    DP_IRQ_TYPE_HP_CABLE_IN  = BIT(0),
    DP_IRQ_TYPE_HP_CABLE_OUT = BIT(1),
    DP_IRQ_TYPE_HP_CHANGE    = BIT(2),
    DP_IRQ_TYPE_UNKNOWN      = BIT(3),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_info {
    pub name: *mut c_char,
    pub h_sync_polarity: bool,
    pub v_sync_polarity: bool,
    pub interlaced: bool,
    pub color_space: color_space,
    pub dynamic_range: dynamic_range,
    pub ycbcr_coeff: color_coefficient,
    pub color_depth: color_depth,
    pub max_link_rate: c_int,
    pub max_lane_count: link_lane_count_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_train {
    pub eq_loop: c_int,
    pub cr_loop: [c_int; 4],
    pub link_rate: u8,
    pub lane_count: u8,
    pub training_lane: [u8; 4],
    pub lt_state: link_training_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct analogix_dp_device {
    pub encoder: *mut drm_encoder,
    pub dev: *mut device,
    pub drm_dev: *mut drm_device,
    pub bridge: drm_bridge,
    pub aux: drm_dp_aux,
    pub clock: *mut clk,
    pub irq: c_uint,
    pub reg_base: *mut void __iomem,
    pub video_info: video_info,
    pub link_train: link_train,
    pub phy: *mut phy,
    pub dpms_mode: c_int,
    pub hpd_gpiod: *mut gpio_desc,
    pub force_hpd: bool,
    pub fast_train_enable: bool,
    pub psr_supported: bool,
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
    pub plat_data: *mut analogix_dp_plat_data,
}

// analogix_dp_reg.c
extern "C" {
    pub fn analogix_dp_enable_video_mute(dp: *mut analogix_dp_device, enable: bool);
}
extern "C" {
    pub fn analogix_dp_stop_video(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_lane_swap(dp: *mut analogix_dp_device, enable: bool);
}
extern "C" {
    pub fn analogix_dp_init_analog_param(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_init_interrupt(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_reset(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_swreset(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_config_interrupt(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_mute_hpd_interrupt(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_unmute_hpd_interrupt(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_wait_pll_locked(dp: *mut analogix_dp_device) -> c_int;
}
extern "C" {
    pub fn analogix_dp_set_pll_power_down(dp: *mut analogix_dp_device, enable: bool);
}
extern "C" {
    pub fn analogix_dp_init_analog_func(dp: *mut analogix_dp_device) -> c_int;
}
extern "C" {
    pub fn analogix_dp_init_hpd(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_force_hpd(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_get_irq_type(dp: *mut analogix_dp_device) -> dp_irq_type;
}
extern "C" {
    pub fn analogix_dp_clear_hotplug_interrupts(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_reset_aux(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_init_aux(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_get_plug_in_status(dp: *mut analogix_dp_device) -> c_int;
}
extern "C" {
    pub fn analogix_dp_enable_sw_function(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_set_link_bandwidth(dp: *mut analogix_dp_device, bwtype: u32);
}
extern "C" {
    pub fn analogix_dp_get_link_bandwidth(dp: *mut analogix_dp_device, bwtype: *mut u32);
}
extern "C" {
    pub fn analogix_dp_set_lane_count(dp: *mut analogix_dp_device, count: u32);
}
extern "C" {
    pub fn analogix_dp_get_lane_count(dp: *mut analogix_dp_device, count: *mut u32);
}
extern "C" {
    pub fn analogix_dp_set_lane_link_training(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_get_lane_link_training(dp: *mut analogix_dp_device, lane: u8) -> u32;
}
extern "C" {
    pub fn analogix_dp_reset_macro(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_init_video(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_set_video_color_format(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_is_slave_video_stream_clock_on(dp: *mut analogix_dp_device) -> bool;
}
extern "C" {
    pub fn analogix_dp_set_video_timing_mode(dp: *mut analogix_dp_device, type: u32);
}
extern "C" {
    pub fn analogix_dp_start_video(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_is_video_stream_on(dp: *mut analogix_dp_device) -> bool;
}
extern "C" {
    pub fn analogix_dp_config_video_slave_mode(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_enable_scrambling(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_disable_scrambling(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_enable_psr_crc(dp: *mut analogix_dp_device);
}
