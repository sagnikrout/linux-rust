//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/gpio_types.h
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


//
// Copyright 2012-15 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//
pub const BUNDLE_A_MASK: c_uint = 0x00FFF000L;
pub const BUNDLE_B_MASK: c_uint = 0x00000FFFL;
//
// gpio_result
//
// @brief
// The possible return codes that the GPIO object can return.
// These return codes can be generated
// directly by the GPIO object or from the GPIOPin object.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_result {
    GPIO_RESULT_OK,
    GPIO_RESULT_NULL_HANDLE,
    GPIO_RESULT_INVALID_DATA,
    GPIO_RESULT_DEVICE_BUSY,
    GPIO_RESULT_OPEN_FAILED,
    GPIO_RESULT_ALREADY_OPENED,
    GPIO_RESULT_NON_SPECIFIC_ERROR
}

//
// @brief
// Used to identify the specific GPIO device
//
// @notes
// These constants are used as indices in a vector.
// Thus they should start from zero and be contiguous.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_id {
    GPIO_ID_UNKNOWN = (-1),
    GPIO_ID_DDC_DATA,
    GPIO_ID_DDC_CLOCK,
    GPIO_ID_GENERIC,
    GPIO_ID_HPD,
    GPIO_ID_GPIO_PAD,
    GPIO_ID_VIP_PAD,
    GPIO_ID_SYNC,
    GPIO_ID_GSL, /* global swap lock */
    GPIO_ID_COUNT,
    GPIO_ID_MIN = GPIO_ID_DDC_DATA,
    GPIO_ID_MAX = GPIO_ID_GSL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_pin_info {
    pub offset: u32,
    pub offset_y: u32,
    pub offset_en: u32,
    pub offset_mask: u32,
    pub mask: u32,
    pub mask_y: u32,
    pub mask_en: u32,
    pub mask_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_pin_output_state {
    GPIO_PIN_OUTPUT_STATE_ACTIVE_LOW,
    GPIO_PIN_OUTPUT_STATE_ACTIVE_HIGH,
    GPIO_PIN_OUTPUT_STATE_DEFAULT = GPIO_PIN_OUTPUT_STATE_ACTIVE_LOW
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_generic {
    GPIO_GENERIC_UNKNOWN = (-1),
    GPIO_GENERIC_A,
    GPIO_GENERIC_B,
    GPIO_GENERIC_C,
    GPIO_GENERIC_D,
    GPIO_GENERIC_E,
    GPIO_GENERIC_F,
    GPIO_GENERIC_G,
    GPIO_GENERIC_COUNT,
    GPIO_GENERIC_MIN = GPIO_GENERIC_A,
    GPIO_GENERIC_MAX = GPIO_GENERIC_B
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_hpd {
    GPIO_HPD_UNKNOWN = (-1),
    GPIO_HPD_1,
    GPIO_HPD_2,
    GPIO_HPD_3,
    GPIO_HPD_4,
    GPIO_HPD_5,
    GPIO_HPD_6,
    GPIO_HPD_COUNT,
    GPIO_HPD_MIN = GPIO_HPD_1,
    GPIO_HPD_MAX = GPIO_HPD_6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_gpio_pad {
    GPIO_GPIO_PAD_UNKNOWN = (-1),
    GPIO_GPIO_PAD_0,
    GPIO_GPIO_PAD_1,
    GPIO_GPIO_PAD_2,
    GPIO_GPIO_PAD_3,
    GPIO_GPIO_PAD_4,
    GPIO_GPIO_PAD_5,
    GPIO_GPIO_PAD_6,
    GPIO_GPIO_PAD_7,
    GPIO_GPIO_PAD_8,
    GPIO_GPIO_PAD_9,
    GPIO_GPIO_PAD_10,
    GPIO_GPIO_PAD_11,
    GPIO_GPIO_PAD_12,
    GPIO_GPIO_PAD_13,
    GPIO_GPIO_PAD_14,
    GPIO_GPIO_PAD_15,
    GPIO_GPIO_PAD_16,
    GPIO_GPIO_PAD_17,
    GPIO_GPIO_PAD_18,
    GPIO_GPIO_PAD_19,
    GPIO_GPIO_PAD_20,
    GPIO_GPIO_PAD_21,
    GPIO_GPIO_PAD_22,
    GPIO_GPIO_PAD_23,
    GPIO_GPIO_PAD_24,
    GPIO_GPIO_PAD_25,
    GPIO_GPIO_PAD_26,
    GPIO_GPIO_PAD_27,
    GPIO_GPIO_PAD_28,
    GPIO_GPIO_PAD_29,
    GPIO_GPIO_PAD_30,
    GPIO_GPIO_PAD_COUNT,
    GPIO_GPIO_PAD_MIN = GPIO_GPIO_PAD_0,
    GPIO_GPIO_PAD_MAX = GPIO_GPIO_PAD_30
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_vip_pad {
    GPIO_VIP_PAD_UNKNOWN = (-1),
// following never used -
// GPIO_ID_DDC_CLOCK::GPIO_DDC_LINE_VIP_PAD defined instead
    GPIO_VIP_PAD_SCL,
// following never used -
// GPIO_ID_DDC_DATA::GPIO_DDC_LINE_VIP_PAD defined instead
    GPIO_VIP_PAD_SDA,
    GPIO_VIP_PAD_VHAD,
    GPIO_VIP_PAD_VPHCTL,
    GPIO_VIP_PAD_VIPCLK,
    GPIO_VIP_PAD_VID,
    GPIO_VIP_PAD_VPCLK0,
    GPIO_VIP_PAD_DVALID,
    GPIO_VIP_PAD_PSYNC,
    GPIO_VIP_PAD_COUNT,
    GPIO_VIP_PAD_MIN = GPIO_VIP_PAD_SCL,
    GPIO_VIP_PAD_MAX = GPIO_VIP_PAD_PSYNC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_sync {
    GPIO_SYNC_UNKNOWN = (-1),
    GPIO_SYNC_HSYNC_A,
    GPIO_SYNC_VSYNC_A,
    GPIO_SYNC_HSYNC_B,
    GPIO_SYNC_VSYNC_B,
    GPIO_SYNC_COUNT,
    GPIO_SYNC_MIN = GPIO_SYNC_HSYNC_A,
    GPIO_SYNC_MAX = GPIO_SYNC_VSYNC_B
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_gsl {
    GPIO_GSL_UNKNOWN = (-1),
    GPIO_GSL_GENLOCK_CLOCK,
    GPIO_GSL_GENLOCK_VSYNC,
    GPIO_GSL_SWAPLOCK_A,
    GPIO_GSL_SWAPLOCK_B,
    GPIO_GSL_COUNT,
    GPIO_GSL_MIN = GPIO_GSL_GENLOCK_CLOCK,
    GPIO_GSL_MAX = GPIO_GSL_SWAPLOCK_B
}

//
// @brief
// Unique Id for DDC handle.
// Values are meaningful (used as indexes to array)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_ddc_line {
    GPIO_DDC_LINE_UNKNOWN = (-1),
    GPIO_DDC_LINE_DDC1,
    GPIO_DDC_LINE_DDC2,
    GPIO_DDC_LINE_DDC3,
    GPIO_DDC_LINE_DDC4,
    GPIO_DDC_LINE_DDC5,
    GPIO_DDC_LINE_DDC6,
    GPIO_DDC_LINE_DDC_VGA,
    GPIO_DDC_LINE_VIP_PAD,
    GPIO_DDC_LINE_I2C_PAD = GPIO_DDC_LINE_VIP_PAD,
    GPIO_DDC_LINE_COUNT,
    GPIO_DDC_LINE_MIN = GPIO_DDC_LINE_DDC1,
    GPIO_DDC_LINE_MAX = GPIO_DDC_LINE_I2C_PAD
}

//
// @brief
// Identifies the mode of operation to open a GPIO device.
// A GPIO device (pin) can be programmed in only one of these modes at a time.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_mode {
    GPIO_MODE_UNKNOWN = (-1),
    GPIO_MODE_INPUT,
    GPIO_MODE_OUTPUT,
    GPIO_MODE_FAST_OUTPUT,
    GPIO_MODE_HARDWARE,
    GPIO_MODE_INTERRUPT
}

//
// @brief
// Identifies the source of the signal when GPIO is in HW mode.
// get_signal_source() will return GPIO_SYGNAL_SOURCE__UNKNOWN
// when one of the following holds:
// 1. GPIO is input GPIO
// 2. GPIO is not opened in HW mode
// 3. GPIO does not have fixed signal source
// (like DC_GenericA have mux instead fixed)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_signal_source {
    GPIO_SIGNAL_SOURCE_UNKNOWN = (-1),
    GPIO_SIGNAL_SOURCE_DACA_STEREO_SYNC,
    GPIO_SIGNAL_SOURCE_PASS_THROUGH_STEREO_SYNC,
    GPIO_SIGNAL_SOURCE_DACB_STEREO_SYNC,
    GPIO_SIGNAL_SOURCE_DACA_HSYNC,
    GPIO_SIGNAL_SOURCE_DACB_HSYNC,
    GPIO_SIGNAL_SOURCE_DACA_VSYNC,
    GPIO_SIGNAL_SOURCE_DACB_VSYNC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_stereo_source {
    GPIO_STEREO_SOURCE_UNKNOWN = (-1),
    GPIO_STEREO_SOURCE_D1,
    GPIO_STEREO_SOURCE_D2,
    GPIO_STEREO_SOURCE_D3,
    GPIO_STEREO_SOURCE_D4,
    GPIO_STEREO_SOURCE_D5,
    GPIO_STEREO_SOURCE_D6
}

//
// GPIO config
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_config_type {
    GPIO_CONFIG_TYPE_NONE,
    GPIO_CONFIG_TYPE_DDC,
    GPIO_CONFIG_TYPE_HPD,
    GPIO_CONFIG_TYPE_GENERIC_MUX,
    GPIO_CONFIG_TYPE_GSL_MUX,
    GPIO_CONFIG_TYPE_I2C_AUX_DUAL_MODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_id_offset_entry {
    pub offset: u32,
    pub mask: u32,
    pub check_mask: bool,
    pub id: gpio_id,
    pub en: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_pin_entry {
    pub id: gpio_id,
    pub en: u32,
    pub offset: u32,
    pub mask: u32,
}

// DDC configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_ddc_config_type {
    GPIO_DDC_CONFIG_TYPE_MODE_AUX,
    GPIO_DDC_CONFIG_TYPE_MODE_I2C,
    GPIO_DDC_CONFIG_TYPE_POLL_FOR_CONNECT,
    GPIO_DDC_CONFIG_TYPE_POLL_FOR_DISCONNECT,
    GPIO_DDC_CONFIG_TYPE_DISABLE_POLLING
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_ddc_config {
    pub type: gpio_ddc_config_type,
    pub data_en_bit_present: bool,
    pub clock_en_bit_present: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_ddc_offset_entry {
    pub offset: u32,
    pub en: u32,
}

// HPD configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_hpd_config {
    pub /: *mut *mut uint32_t delay_on_connect; / milliseconds,
    pub /: *mut *mut uint32_t delay_on_disconnect; / milliseconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_generic_mux_config {
    pub enable_output_from_mux: bool,
    pub mux_select: gpio_signal_source,
    pub stereo_select: gpio_stereo_source,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_gsl_mux_config_type {
    GPIO_GSL_MUX_CONFIG_TYPE_DISABLE,
    GPIO_GSL_MUX_CONFIG_TYPE_TIMING_SYNC,
    GPIO_GSL_MUX_CONFIG_TYPE_FLIP_SYNC
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_gsl_mux_config {
    pub type: gpio_gsl_mux_config_type,
// Actually sync_source type,
// however we want to avoid inter-component includes here
    pub gsl_group: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_config_data {
    pub type: gpio_config_type,
    pub ddc: gpio_ddc_config,
    pub hpd: gpio_hpd_config,
    pub generic_mux: gpio_generic_mux_config,
    pub gsl_mux: gpio_gsl_mux_config,
    pub config: },
}
