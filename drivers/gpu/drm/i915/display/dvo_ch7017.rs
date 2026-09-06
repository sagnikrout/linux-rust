//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/display/dvo_ch7017.c
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
// Copyright © 2006 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Authors:
// Eric Anholt <eric@anholt.net>
//

pub const CH7017_TV_DISPLAY_MODE: c_uint = 0x00;
pub const CH7017_FLICKER_FILTER: c_uint = 0x01;
pub const CH7017_VIDEO_BANDWIDTH: c_uint = 0x02;
pub const CH7017_TEXT_ENHANCEMENT: c_uint = 0x03;
pub const CH7017_START_ACTIVE_VIDEO: c_uint = 0x04;
pub const CH7017_HORIZONTAL_POSITION: c_uint = 0x05;
pub const CH7017_VERTICAL_POSITION: c_uint = 0x06;
pub const CH7017_BLACK_LEVEL: c_uint = 0x07;
pub const CH7017_CONTRAST_ENHANCEMENT: c_uint = 0x08;
pub const CH7017_TV_PLL: c_uint = 0x09;
pub const CH7017_TV_PLL_M: c_uint = 0x0a;
pub const CH7017_TV_PLL_N: c_uint = 0x0b;
pub const CH7017_SUB_CARRIER_0: c_uint = 0x0c;
pub const CH7017_CIV_CONTROL: c_uint = 0x10;
pub const CH7017_CIV_0: c_uint = 0x11;
pub const CH7017_CHROMA_BOOST: c_uint = 0x14;
pub const CH7017_CLOCK_MODE: c_uint = 0x1c;
pub const CH7017_INPUT_CLOCK: c_uint = 0x1d;
pub const CH7017_GPIO_CONTROL: c_uint = 0x1e;
pub const CH7017_INPUT_DATA_FORMAT: c_uint = 0x1f;
pub const CH7017_CONNECTION_DETECT: c_uint = 0x20;
pub const CH7017_DAC_CONTROL: c_uint = 0x21;
pub const CH7017_BUFFERED_CLOCK_OUTPUT: c_uint = 0x22;
pub const CH7017_DEFEAT_VSYNC: c_uint = 0x47;
pub const CH7017_TEST_PATTERN: c_uint = 0x48;
pub const CH7017_POWER_MANAGEMENT: c_uint = 0x49;
// Enables the TV output path.

// Powers down the TV out block, and DAC0-3

pub const CH7017_VERSION_ID: c_uint = 0x4a;
pub const CH7017_DEVICE_ID: c_uint = 0x4b;
pub const CH7017_DEVICE_ID_VALUE: c_uint = 0x1b;
pub const CH7018_DEVICE_ID_VALUE: c_uint = 0x1a;
pub const CH7019_DEVICE_ID_VALUE: c_uint = 0x19;
pub const CH7017_XCLK_D2_ADJUST: c_uint = 0x53;
pub const CH7017_UP_SCALER_COEFF_0: c_uint = 0x55;
pub const CH7017_UP_SCALER_COEFF_1: c_uint = 0x56;
pub const CH7017_UP_SCALER_COEFF_2: c_uint = 0x57;
pub const CH7017_UP_SCALER_COEFF_3: c_uint = 0x58;
pub const CH7017_UP_SCALER_COEFF_4: c_uint = 0x59;
pub const CH7017_UP_SCALER_VERTICAL_INC_0: c_uint = 0x5a;
pub const CH7017_UP_SCALER_VERTICAL_INC_1: c_uint = 0x5b;
pub const CH7017_GPIO_INVERT: c_uint = 0x5c;
pub const CH7017_UP_SCALER_HORIZONTAL_INC_0: c_uint = 0x5d;
pub const CH7017_UP_SCALER_HORIZONTAL_INC_1: c_uint = 0x5e;
pub const CH7017_HORIZONTAL_ACTIVE_PIXEL_INPUT: c_uint = 0x5f;
// < Low bits of horizontal active pixel input
pub const CH7017_ACTIVE_INPUT_LINE_OUTPUT: c_uint = 0x60;
// High bits of horizontal active pixel input

// High bits of vertical active line output

pub const CH7017_VERTICAL_ACTIVE_LINE_OUTPUT: c_uint = 0x61;
// < Low bits of vertical active line output
pub const CH7017_HORIZONTAL_ACTIVE_PIXEL_OUTPUT: c_uint = 0x62;
// < Low bits of horizontal active pixel output
pub const CH7017_LVDS_POWER_DOWN: c_uint = 0x63;
// High bits of horizontal active pixel output

// Enables the LVDS power down state transition

// Enables the LVDS upscaler

pub const CH7017_LVDS_POWER_DOWN_DEFAULT_RESERVED: c_uint = 0x08;
pub const CH7017_LVDS_ENCODING: c_uint = 0x64;

pub const CH7017_LVDS_ENCODING_2: c_uint = 0x65;
pub const CH7017_LVDS_PLL_CONTROL: c_uint = 0x66;
// Enables the LVDS panel output path

// Enables the LVDS panel backlight

pub const CH7017_POWER_SEQUENCING_T1: c_uint = 0x67;
pub const CH7017_POWER_SEQUENCING_T2: c_uint = 0x68;
pub const CH7017_POWER_SEQUENCING_T3: c_uint = 0x69;
pub const CH7017_POWER_SEQUENCING_T4: c_uint = 0x6a;
pub const CH7017_POWER_SEQUENCING_T5: c_uint = 0x6b;
pub const CH7017_GPIO_DRIVER_TYPE: c_uint = 0x6c;
pub const CH7017_GPIO_DATA: c_uint = 0x6d;
pub const CH7017_GPIO_DIRECTION_CONTROL: c_uint = 0x6e;
pub const CH7017_LVDS_PLL_FEEDBACK_DIV: c_uint = 0x71;

pub const CH7017_LVDS_PLL_VCO_CONTROL: c_uint = 0x72;

pub const CH7017_OUTPUTS_ENABLE: c_uint = 0x73;

pub const CH7017_LVDS_OUTPUT_AMPLITUDE: c_uint = 0x74;
pub const CH7017_LVDS_PLL_EMI_REDUCTION: c_uint = 0x75;
pub const CH7017_LVDS_POWER_DOWN_FLICKER: c_uint = 0x76;
pub const CH7017_LVDS_CONTROL_2: c_uint = 0x78;

pub const CH7017_BANG_LIMIT_CONTROL: c_uint = 0x7f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch7017_priv {
    pub dummy: u8,
}

    static void ch7017_dump_regs(struct intel_dvo_device *dvo);
    static void ch7017_dpms(struct intel_dvo_device *dvo, bool enable);
#[no_mangle]
unsafe extern "C" fn ch7017_read(dvo: *mut intel_dvo_device, addr: u8, val: *mut u8) -> bool {
    static bool ch7017_read(struct intel_dvo_device *dvo, u8 addr, u8 *val)
    {
    struct i2c_msg msgs[] = {
    {
    .addr = dvo.target_addr,
    .flags = 0,
    .len = 1,
    .buf = &addr,
    },
    {
    .addr = dvo.target_addr,
    .flags = I2C_M_RD,
    .len = 1,
    .buf = val,
    }
    };
    return i2c_transfer(dvo.i2c_bus, msgs, 2) == 2;
    }
#[no_mangle]
unsafe extern "C" fn ch7017_write(dvo: *mut intel_dvo_device, addr: u8, val: u8) -> bool {
    static bool ch7017_write(struct intel_dvo_device *dvo, u8 addr, u8 val)
    {
    u8 buf[2] = { addr, val };
    struct i2c_msg msg = {
    .addr = dvo.target_addr,
    .flags = 0,
    .len = 2,
    .buf = buf,
    };
    return i2c_transfer(dvo.i2c_bus, &msg, 1) == 1;
    }
// Probes for a CH7017 on the given bus and target address.
    static bool ch7017_init(struct intel_dvo_device *dvo,
    struct i2c_adapter *adapter)
    {
    struct ch7017_priv *priv;
    const char *str;
    u8 val;
    priv = kzalloc_obj(*priv);
    if (priv == core::ptr::null_mut())
    return false;
    dvo.i2c_bus = adapter;
    dvo.dev_priv = priv;
    if (!ch7017_read(dvo, CH7017_DEVICE_ID, &val))
    goto fail;
    switch (val) {
    case CH7017_DEVICE_ID_VALUE:
    str = "ch7017";
    break;
    case CH7018_DEVICE_ID_VALUE:
    str = "ch7018";
    break;
    case CH7019_DEVICE_ID_VALUE:
    str = "ch7019";
    break;
    default:
    DRM_DEBUG_KMS("ch701x not detected, got %d: from %s "
    "target %d.\n",
    val, adapter.name, dvo.target_addr);
    goto fail;
    }
    DRM_DEBUG_KMS("%s detected on %s, addr %d\n",
    str, adapter.name, dvo.target_addr);
    return true;
    fail:
    kfree(priv);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ch7017_detect(dvo: *mut intel_dvo_device) -> enum drm_connector_status {
    static enum drm_connector_status ch7017_detect(struct intel_dvo_device *dvo)
    {
    return connector_status_connected;
    }
    static enum drm_mode_status ch7017_mode_valid(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode)
    {
    if (mode.clock > 160000)
    return MODE_CLOCK_HIGH;
    return MODE_OK;
    }
    static void ch7017_mode_set(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
    u8 lvds_pll_feedback_div, lvds_pll_vco_control;
    u8 outputs_enable, lvds_control_2, lvds_power_down;
    u8 horizontal_active_pixel_input;
    u8 horizontal_active_pixel_output, vertical_active_line_output;
    u8 active_input_line_output;
    DRM_DEBUG_KMS("Registers before mode setting\n");
    ch7017_dump_regs(dvo);
// LVDS PLL settings from page 75 of 7017-7017ds.pdf
    if (mode.clock < 100000) {
    outputs_enable = CH7017_LVDS_CHANNEL_A | CH7017_CHARGE_PUMP_LOW;
    lvds_pll_feedback_div = CH7017_LVDS_PLL_FEEDBACK_DEFAULT_RESERVED |
    (2 << CH7017_LVDS_PLL_FEED_BACK_DIVIDER_SHIFT) |
    (13 << CH7017_LVDS_PLL_FEED_FORWARD_DIVIDER_SHIFT);
    lvds_pll_vco_control = CH7017_LVDS_PLL_VCO_DEFAULT_RESERVED |
    (2 << CH7017_LVDS_PLL_VCO_SHIFT) |
    (3 << CH7017_LVDS_PLL_POST_SCALE_DIV_SHIFT);
    lvds_control_2 = (1 << CH7017_LOOP_FILTER_SHIFT) |
    (0 << CH7017_PHASE_DETECTOR_SHIFT);
    } else {
    outputs_enable = CH7017_LVDS_CHANNEL_A | CH7017_CHARGE_PUMP_HIGH;
    lvds_pll_feedback_div =
    CH7017_LVDS_PLL_FEEDBACK_DEFAULT_RESERVED |
    (2 << CH7017_LVDS_PLL_FEED_BACK_DIVIDER_SHIFT) |
    (3 << CH7017_LVDS_PLL_FEED_FORWARD_DIVIDER_SHIFT);
    lvds_control_2 = (3 << CH7017_LOOP_FILTER_SHIFT) |
    (0 << CH7017_PHASE_DETECTOR_SHIFT);
    if (1) { /* XXX: dual channel panel detection.  Assume yes for now. */
    outputs_enable |= CH7017_LVDS_CHANNEL_B;
    lvds_pll_vco_control = CH7017_LVDS_PLL_VCO_DEFAULT_RESERVED |
    (2 << CH7017_LVDS_PLL_VCO_SHIFT) |
    (13 << CH7017_LVDS_PLL_POST_SCALE_DIV_SHIFT);
    } else {
    lvds_pll_vco_control = CH7017_LVDS_PLL_VCO_DEFAULT_RESERVED |
    (1 << CH7017_LVDS_PLL_VCO_SHIFT) |
    (13 << CH7017_LVDS_PLL_POST_SCALE_DIV_SHIFT);
    }
    }
    horizontal_active_pixel_input = mode.hdisplay & 0x00ff;
    vertical_active_line_output = mode.vdisplay & 0x00ff;
    horizontal_active_pixel_output = mode.hdisplay & 0x00ff;
    active_input_line_output = ((mode.hdisplay & 0x0700) >> 8) |
    (((mode.vdisplay & 0x0700) >> 8) << 3);
    lvds_power_down = CH7017_LVDS_POWER_DOWN_DEFAULT_RESERVED |
    (mode.hdisplay & 0x0700) >> 8;
    ch7017_dpms(dvo, false);
    ch7017_write(dvo, CH7017_HORIZONTAL_ACTIVE_PIXEL_INPUT,
    horizontal_active_pixel_input);
    ch7017_write(dvo, CH7017_HORIZONTAL_ACTIVE_PIXEL_OUTPUT,
    horizontal_active_pixel_output);
    ch7017_write(dvo, CH7017_VERTICAL_ACTIVE_LINE_OUTPUT,
    vertical_active_line_output);
    ch7017_write(dvo, CH7017_ACTIVE_INPUT_LINE_OUTPUT,
    active_input_line_output);
    ch7017_write(dvo, CH7017_LVDS_PLL_VCO_CONTROL, lvds_pll_vco_control);
    ch7017_write(dvo, CH7017_LVDS_PLL_FEEDBACK_DIV, lvds_pll_feedback_div);
    ch7017_write(dvo, CH7017_LVDS_CONTROL_2, lvds_control_2);
    ch7017_write(dvo, CH7017_OUTPUTS_ENABLE, outputs_enable);
// Turn the LVDS back on with new settings.
    ch7017_write(dvo, CH7017_LVDS_POWER_DOWN, lvds_power_down);
    DRM_DEBUG_KMS("Registers after mode setting\n");
    ch7017_dump_regs(dvo);
    }
// set the CH7017 power state
#[no_mangle]
unsafe extern "C" fn ch7017_dpms(dvo: *mut intel_dvo_device, enable: bool) {
    static void ch7017_dpms(struct intel_dvo_device *dvo, bool enable)
    {
    u8 val;
    ch7017_read(dvo, CH7017_LVDS_POWER_DOWN, &val);
// Turn off TV/VGA, and never turn it on since we don't support it.
    ch7017_write(dvo, CH7017_POWER_MANAGEMENT,
    CH7017_DAC0_POWER_DOWN |
    CH7017_DAC1_POWER_DOWN |
    CH7017_DAC2_POWER_DOWN |
    CH7017_DAC3_POWER_DOWN |
    CH7017_TV_POWER_DOWN_EN);
    if (enable) {
// Turn on the LVDS
    ch7017_write(dvo, CH7017_LVDS_POWER_DOWN,
    val & ~CH7017_LVDS_POWER_DOWN_EN);
    } else {
// Turn off the LVDS
    ch7017_write(dvo, CH7017_LVDS_POWER_DOWN,
    val | CH7017_LVDS_POWER_DOWN_EN);
    }
// XXX: Should actually wait for update power status somehow
    msleep(20);
    }
#[no_mangle]
unsafe extern "C" fn ch7017_get_hw_state(dvo: *mut intel_dvo_device) -> bool {
    static bool ch7017_get_hw_state(struct intel_dvo_device *dvo)
    {
    u8 val;
    ch7017_read(dvo, CH7017_LVDS_POWER_DOWN, &val);
    if (val & CH7017_LVDS_POWER_DOWN_EN)
    return false;
    else
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ch7017_dump_regs(dvo: *mut intel_dvo_device) {
    static void ch7017_dump_regs(struct intel_dvo_device *dvo)
    {
    u8 val;

    do {							\
    ch7017_read(dvo, reg, &val);			\
    DRM_DEBUG_KMS(#reg ": %02x\n", val);		\
    } while (0)
    DUMP(CH7017_HORIZONTAL_ACTIVE_PIXEL_INPUT);
    DUMP(CH7017_HORIZONTAL_ACTIVE_PIXEL_OUTPUT);
    DUMP(CH7017_VERTICAL_ACTIVE_LINE_OUTPUT);
    DUMP(CH7017_ACTIVE_INPUT_LINE_OUTPUT);
    DUMP(CH7017_LVDS_PLL_VCO_CONTROL);
    DUMP(CH7017_LVDS_PLL_FEEDBACK_DIV);
    DUMP(CH7017_LVDS_CONTROL_2);
    DUMP(CH7017_OUTPUTS_ENABLE);
    DUMP(CH7017_LVDS_POWER_DOWN);
    }
#[no_mangle]
unsafe extern "C" fn ch7017_destroy(dvo: *mut intel_dvo_device) {
    static void ch7017_destroy(struct intel_dvo_device *dvo)
    {
    struct ch7017_priv *priv = dvo.dev_priv;
    if (priv) {
    kfree(priv);
    dvo.dev_priv = core::ptr::null_mut();
    }
    }
    const struct intel_dvo_dev_ops ch7017_ops = {
    .init = ch7017_init,
    .detect = ch7017_detect,
    .mode_valid = ch7017_mode_valid,
    .mode_set = ch7017_mode_set,
    .dpms = ch7017_dpms,
    .get_hw_state = ch7017_get_hw_state,
    .dump_regs = ch7017_dump_regs,
    .destroy = ch7017_destroy,
    };
