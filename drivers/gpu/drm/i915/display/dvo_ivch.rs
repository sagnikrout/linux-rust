//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/display/dvo_ivch.c
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
// Thomas Richter <thor@math.tu-berlin.de>
//
// Minor modifications (Dithering enable):
// Thomas Richter <thor@math.tu-berlin.de>
//

//
// register definitions for the i82807aa.
//
// Documentation on this chipset can be found in datasheet #29069001 at
// intel.com.
//
// VCH Revision & GMBus Base Addr
//
pub const VR00: c_uint = 0x00;

//
// Functionality Enable
//
pub const VR01: c_uint = 0x01;
//
// Enable the panel fitter
//

//
// Enables the LCD display.
//
// This must not be set while VR01_DVO_BYPASS_ENABLE is set.
//

// Enables the DVO repeater.

// Enables the DVO clock

// Enable dithering for 18bpp panels. Not documented.

//
// LCD Interface Format
//
pub const VR10: c_uint = 0x10;
// Enables LVDS output instead of CMOS

// Enables 18-bit LVDS output.

// Enables 24-bit LVDS or CMOS output

// Enables 2x18-bit LVDS or CMOS output.

// Enables 2x24-bit LVDS output

// Mask that defines the depth of the pipeline

//
// VR20 LCD Horizontal Display Size
//
pub const VR20: c_uint = 0x20;
//
// LCD Vertical Display Size
//
pub const VR21: c_uint = 0x21;
//
// Panel power down status
//
pub const VR30: c_uint = 0x30;
// Read only bit indicating that the panel is not in a safe poweroff state.

pub const VR40: c_uint = 0x40;

//
// Panel Fitting Vertical Ratio
// (((image_height - 1) << 16) / ((panel_height - 1))) >> 2
//
pub const VR41: c_uint = 0x41;
//
// Panel Fitting Horizontal Ratio
// (((image_width - 1) << 16) / ((panel_width - 1))) >> 2
//
pub const VR42: c_uint = 0x42;
//
// Horizontal Image Size
//
pub const VR43: c_uint = 0x43;
// VR80 GPIO 0
//
pub const VR80: c_uint = 0x80;
pub const VR81: c_uint = 0x81;
pub const VR82: c_uint = 0x82;
pub const VR83: c_uint = 0x83;
pub const VR84: c_uint = 0x84;
pub const VR85: c_uint = 0x85;
pub const VR86: c_uint = 0x86;
pub const VR87: c_uint = 0x87;
// VR88 GPIO 8
//
pub const VR88: c_uint = 0x88;
// Graphics BIOS scratch 0
//
pub const VR8E: c_uint = 0x8E;

// Graphics BIOS scratch 1
//
pub const VR8F: c_uint = 0x8F;

// Some Bios implementations do not restore the DVO state upon
// resume from standby. Thus, this driver has to handle it
// instead. The following list contains all registers that
// require saving.
//
    static const u16 backup_addresses[] = {
    0x11, 0x12,
    0x18, 0x19, 0x1a, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
    0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
    0x8e, 0x8f,
    0x10		/* this must come last */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivch_priv {
    pub quiet: bool,
    pub height: u16 width,,
// Register backup
    pub reg_backup: [u16; ARRAY_SIZE(backup_addresses)],
}

    static void ivch_dump_regs(struct intel_dvo_device *dvo);
//
// Reads a register on the ivch.
//
// Each of the 256 registers are 16 bits long.
//
#[no_mangle]
unsafe extern "C" fn ivch_read(dvo: *mut intel_dvo_device, addr: c_int, data: *mut u16) -> bool {
    static bool ivch_read(struct intel_dvo_device *dvo, int addr, u16 *data)
    {
    struct ivch_priv *priv = dvo.dev_priv;
    struct i2c_adapter *adapter = dvo.i2c_bus;
    u8 out_buf[1];
    u8 in_buf[2];
    struct i2c_msg msgs[] = {
    {
    .addr = dvo.target_addr,
    .flags = I2C_M_RD,
    .len = 0,
    },
    {
    .addr = 0,
    .flags = I2C_M_NOSTART,
    .len = 1,
    .buf = out_buf,
    },
    {
    .addr = dvo.target_addr,
    .flags = I2C_M_RD | I2C_M_NOSTART,
    .len = 2,
    .buf = in_buf,
    }
    };
    out_buf[0] = addr;
    if (i2c_transfer(adapter, msgs, 3) == 3) {
// data = (in_buf[1] << 8) | in_buf[0];
    return true;
    }
    if (!priv.quiet) {
    DRM_DEBUG_KMS("Unable to read register 0x%02x from "
    "%s:%02x.\n",
    addr, adapter.name, dvo.target_addr);
    }
    return false;
    }
// Writes a 16-bit register on the ivch
#[no_mangle]
unsafe extern "C" fn ivch_write(dvo: *mut intel_dvo_device, addr: c_int, data: u16) -> bool {
    static bool ivch_write(struct intel_dvo_device *dvo, int addr, u16 data)
    {
    struct ivch_priv *priv = dvo.dev_priv;
    struct i2c_adapter *adapter = dvo.i2c_bus;
    u8 out_buf[3];
    struct i2c_msg msg = {
    .addr = dvo.target_addr,
    .flags = 0,
    .len = 3,
    .buf = out_buf,
    };
    out_buf[0] = addr;
    out_buf[1] = data & 0xff;
    out_buf[2] = data >> 8;
    if (i2c_transfer(adapter, &msg, 1) == 1)
    return true;
    if (!priv.quiet) {
    DRM_DEBUG_KMS("Unable to write register 0x%02x to %s:%d.\n",
    addr, adapter.name, dvo.target_addr);
    }
    return false;
    }
// Probes the given bus and target address for an ivch
    static bool ivch_init(struct intel_dvo_device *dvo,
    struct i2c_adapter *adapter)
    {
    struct ivch_priv *priv;
    u16 temp;
    int i;
    priv = kzalloc_obj(*priv);
    if (priv == core::ptr::null_mut())
    return false;
    dvo.i2c_bus = adapter;
    dvo.dev_priv = priv;
    priv.quiet = true;
    if (!ivch_read(dvo, VR00, &temp))
    goto out;
    priv.quiet = false;
// Since the identification bits are probably zeroes, which doesn't seem
// very unique, check that the value in the base address field matches
// the address it's responding on.
//
    if ((temp & VR00_BASE_ADDRESS_MASK) != dvo.target_addr) {
    DRM_DEBUG_KMS("ivch detect failed due to address mismatch "
    "(%d vs %d)\n",
    (temp & VR00_BASE_ADDRESS_MASK), dvo.target_addr);
    goto out;
    }
    ivch_read(dvo, VR20, &priv.width);
    ivch_read(dvo, VR21, &priv.height);
// Make a backup of the registers to be able to restore them
// upon suspend.
//
    for (i = 0; i < ARRAY_SIZE(backup_addresses); i++)
    ivch_read(dvo, backup_addresses[i], priv.reg_backup + i);
    ivch_dump_regs(dvo);
    return true;
    out:
    kfree(priv);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ivch_detect(dvo: *mut intel_dvo_device) -> enum drm_connector_status {
    static enum drm_connector_status ivch_detect(struct intel_dvo_device *dvo)
    {
    return connector_status_connected;
    }
    static enum drm_mode_status ivch_mode_valid(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode)
    {
    if (mode.clock > 112000)
    return MODE_CLOCK_HIGH;
    return MODE_OK;
    }
// Restore the DVO registers after a resume
// from RAM. Registers have been saved during
// the initialization.
//
#[no_mangle]
unsafe extern "C" fn ivch_reset(dvo: *mut intel_dvo_device) {
    static void ivch_reset(struct intel_dvo_device *dvo)
    {
    struct ivch_priv *priv = dvo.dev_priv;
    int i;
    DRM_DEBUG_KMS("Resetting the IVCH registers\n");
    ivch_write(dvo, VR10, 0x0000);
    for (i = 0; i < ARRAY_SIZE(backup_addresses); i++)
    ivch_write(dvo, backup_addresses[i], priv.reg_backup[i]);
    }
// Sets the power state of the panel connected to the ivch
#[no_mangle]
unsafe extern "C" fn ivch_dpms(dvo: *mut intel_dvo_device, enable: bool) {
    static void ivch_dpms(struct intel_dvo_device *dvo, bool enable)
    {
    int i;
    u16 vr01, vr30, backlight;
    ivch_reset(dvo);
// Set the new power state of the panel.
    if (!ivch_read(dvo, VR01, &vr01))
    return;
    if (enable)
    backlight = 1;
    else
    backlight = 0;
    ivch_write(dvo, VR80, backlight);
    if (enable)
    vr01 |= VR01_LCD_ENABLE | VR01_DVO_ENABLE;
    else
    vr01 &= ~(VR01_LCD_ENABLE | VR01_DVO_ENABLE);
    ivch_write(dvo, VR01, vr01);
// Wait for the panel to make its state transition
    for (i = 0; i < 100; i++) {
    if (!ivch_read(dvo, VR30, &vr30))
    break;
    if (((vr30 & VR30_PANEL_ON) != 0) == enable)
    break;
    udelay(1000);
    }
// wait some more; vch may fail to resync sometimes without this
    udelay(16 * 1000);
    }
#[no_mangle]
unsafe extern "C" fn ivch_get_hw_state(dvo: *mut intel_dvo_device) -> bool {
    static bool ivch_get_hw_state(struct intel_dvo_device *dvo)
    {
    u16 vr01;
    ivch_reset(dvo);
// Set the new power state of the panel.
    if (!ivch_read(dvo, VR01, &vr01))
    return false;
    if (vr01 & VR01_LCD_ENABLE)
    return true;
    else
    return false;
    }
    static void ivch_mode_set(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
    struct ivch_priv *priv = dvo.dev_priv;
    let mut vr40: u16 = 0;
    let mut vr01: u16 = 0;
    u16 vr10;
    ivch_reset(dvo);
    vr10 = priv.reg_backup[ARRAY_SIZE(backup_addresses) - 1];
// Enable dithering for 18 bpp pipelines
    vr10 &= VR10_INTERFACE_DEPTH_MASK;
    if (vr10 == VR10_INTERFACE_2X18 || vr10 == VR10_INTERFACE_1X18)
    vr01 = VR01_DITHER_ENABLE;
    vr40 = (VR40_STALL_ENABLE | VR40_VERTICAL_INTERP_ENABLE |
    VR40_HORIZONTAL_INTERP_ENABLE);
    if (mode.hdisplay != adjusted_mode.crtc_hdisplay ||
    mode.vdisplay != adjusted_mode.crtc_vdisplay) {
    u16 x_ratio, y_ratio;
    vr01 |= VR01_PANEL_FIT_ENABLE;
    vr40 |= VR40_CLOCK_GATING_ENABLE;
    x_ratio = (((mode.hdisplay - 1) << 16) /
    (adjusted_mode.crtc_hdisplay - 1)) >> 2;
    y_ratio = (((mode.vdisplay - 1) << 16) /
    (adjusted_mode.crtc_vdisplay - 1)) >> 2;
    ivch_write(dvo, VR42, x_ratio);
    ivch_write(dvo, VR41, y_ratio);
    } else {
    vr01 &= ~VR01_PANEL_FIT_ENABLE;
    vr40 &= ~VR40_CLOCK_GATING_ENABLE;
    }
    vr40 &= ~VR40_AUTO_RATIO_ENABLE;
    ivch_write(dvo, VR01, vr01);
    ivch_write(dvo, VR40, vr40);
    }
#[no_mangle]
unsafe extern "C" fn ivch_dump_regs(dvo: *mut intel_dvo_device) {
    static void ivch_dump_regs(struct intel_dvo_device *dvo)
    {
    u16 val;
    ivch_read(dvo, VR00, &val);
    DRM_DEBUG_KMS("VR00: 0x%04x\n", val);
    ivch_read(dvo, VR01, &val);
    DRM_DEBUG_KMS("VR01: 0x%04x\n", val);
    ivch_read(dvo, VR10, &val);
    DRM_DEBUG_KMS("VR10: 0x%04x\n", val);
    ivch_read(dvo, VR30, &val);
    DRM_DEBUG_KMS("VR30: 0x%04x\n", val);
    ivch_read(dvo, VR40, &val);
    DRM_DEBUG_KMS("VR40: 0x%04x\n", val);
// GPIO registers
    ivch_read(dvo, VR80, &val);
    DRM_DEBUG_KMS("VR80: 0x%04x\n", val);
    ivch_read(dvo, VR81, &val);
    DRM_DEBUG_KMS("VR81: 0x%04x\n", val);
    ivch_read(dvo, VR82, &val);
    DRM_DEBUG_KMS("VR82: 0x%04x\n", val);
    ivch_read(dvo, VR83, &val);
    DRM_DEBUG_KMS("VR83: 0x%04x\n", val);
    ivch_read(dvo, VR84, &val);
    DRM_DEBUG_KMS("VR84: 0x%04x\n", val);
    ivch_read(dvo, VR85, &val);
    DRM_DEBUG_KMS("VR85: 0x%04x\n", val);
    ivch_read(dvo, VR86, &val);
    DRM_DEBUG_KMS("VR86: 0x%04x\n", val);
    ivch_read(dvo, VR87, &val);
    DRM_DEBUG_KMS("VR87: 0x%04x\n", val);
    ivch_read(dvo, VR88, &val);
    DRM_DEBUG_KMS("VR88: 0x%04x\n", val);
// Scratch register 0 - AIM Panel type
    ivch_read(dvo, VR8E, &val);
    DRM_DEBUG_KMS("VR8E: 0x%04x\n", val);
// Scratch register 1 - Status register
    ivch_read(dvo, VR8F, &val);
    DRM_DEBUG_KMS("VR8F: 0x%04x\n", val);
    }
#[no_mangle]
unsafe extern "C" fn ivch_destroy(dvo: *mut intel_dvo_device) {
    static void ivch_destroy(struct intel_dvo_device *dvo)
    {
    struct ivch_priv *priv = dvo.dev_priv;
    if (priv) {
    kfree(priv);
    dvo.dev_priv = core::ptr::null_mut();
    }
    }
    const struct intel_dvo_dev_ops ivch_ops = {
    .init = ivch_init,
    .dpms = ivch_dpms,
    .get_hw_state = ivch_get_hw_state,
    .mode_valid = ivch_mode_valid,
    .mode_set = ivch_mode_set,
    .detect = ivch_detect,
    .dump_regs = ivch_dump_regs,
    .destroy = ivch_destroy,
    };
