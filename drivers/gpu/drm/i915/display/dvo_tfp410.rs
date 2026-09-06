//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/display/dvo_tfp410.c
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
// Copyright © 2007 Dave Mueller
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
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//
// Authors:
// Dave Mueller <dave.mueller@gmx.ch>
//

// register definitions according to the TFP410 data sheet
pub const TFP410_VID: c_uint = 0x014C;
pub const TFP410_DID: c_uint = 0x0410;
pub const TFP410_VID_LO: c_uint = 0x00;
pub const TFP410_VID_HI: c_uint = 0x01;
pub const TFP410_DID_LO: c_uint = 0x02;
pub const TFP410_DID_HI: c_uint = 0x03;
pub const TFP410_REV: c_uint = 0x04;
pub const TFP410_CTL_1: c_uint = 0x08;

pub const TFP410_CTL_2: c_uint = 0x09;

pub const TFP410_CTL_3: c_uint = 0x0A;

pub const TFP410_USERCFG: c_uint = 0x0B;
pub const TFP410_DE_DLY: c_uint = 0x32;
pub const TFP410_DE_CTL: c_uint = 0x33;

pub const TFP410_DE_TOP: c_uint = 0x34;
pub const TFP410_DE_CNT_LO: c_uint = 0x36;
pub const TFP410_DE_CNT_HI: c_uint = 0x37;
pub const TFP410_DE_LIN_LO: c_uint = 0x38;
pub const TFP410_DE_LIN_HI: c_uint = 0x39;
pub const TFP410_H_RES_LO: c_uint = 0x3A;
pub const TFP410_H_RES_HI: c_uint = 0x3B;
pub const TFP410_V_RES_LO: c_uint = 0x3C;
pub const TFP410_V_RES_HI: c_uint = 0x3D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tfp410_priv {
    pub quiet: bool,
}

#[no_mangle]
unsafe extern "C" fn tfp410_readb(dvo: *mut intel_dvo_device, addr: c_int, ch: *mut u8) -> bool {
    static bool tfp410_readb(struct intel_dvo_device *dvo, int addr, u8 *ch)
    {
    struct tfp410_priv *tfp = dvo.dev_priv;
    struct i2c_adapter *adapter = dvo.i2c_bus;
    u8 out_buf[2];
    u8 in_buf[2];
    struct i2c_msg msgs[] = {
    {
    .addr = dvo.target_addr,
    .flags = 0,
    .len = 1,
    .buf = out_buf,
    },
    {
    .addr = dvo.target_addr,
    .flags = I2C_M_RD,
    .len = 1,
    .buf = in_buf,
    }
    };
    out_buf[0] = addr;
    out_buf[1] = 0;
    if (i2c_transfer(adapter, msgs, 2) == 2) {
// ch = in_buf[0];
    return true;
    }
    if (!tfp.quiet) {
    DRM_DEBUG_KMS("Unable to read register 0x%02x from %s:%02x.\n",
    addr, adapter.name, dvo.target_addr);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn tfp410_writeb(dvo: *mut intel_dvo_device, addr: c_int, ch: u8) -> bool {
    static bool tfp410_writeb(struct intel_dvo_device *dvo, int addr, u8 ch)
    {
    struct tfp410_priv *tfp = dvo.dev_priv;
    struct i2c_adapter *adapter = dvo.i2c_bus;
    u8 out_buf[2];
    struct i2c_msg msg = {
    .addr = dvo.target_addr,
    .flags = 0,
    .len = 2,
    .buf = out_buf,
    };
    out_buf[0] = addr;
    out_buf[1] = ch;
    if (i2c_transfer(adapter, &msg, 1) == 1)
    return true;
    if (!tfp.quiet) {
    DRM_DEBUG_KMS("Unable to write register 0x%02x to %s:%d.\n",
    addr, adapter.name, dvo.target_addr);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn tfp410_getid(dvo: *mut intel_dvo_device, addr: c_int) -> c_int {
    static int tfp410_getid(struct intel_dvo_device *dvo, int addr)
    {
    u8 ch1, ch2;
    if (tfp410_readb(dvo, addr+0, &ch1) &&
    tfp410_readb(dvo, addr+1, &ch2))
    return ((ch2 << 8) & 0xFF00) | (ch1 & 0x00FF);
    return -1;
    }
// Ti TFP410 driver for chip on i2c bus
    static bool tfp410_init(struct intel_dvo_device *dvo,
    struct i2c_adapter *adapter)
    {
// this will detect the tfp410 chip on the specified i2c bus
    struct tfp410_priv *tfp;
    int id;
    tfp = kzalloc_obj(*tfp);
    if (tfp == core::ptr::null_mut())
    return false;
    dvo.i2c_bus = adapter;
    dvo.dev_priv = tfp;
    tfp.quiet = true;
    if ((id = tfp410_getid(dvo, TFP410_VID_LO)) != TFP410_VID) {
    DRM_DEBUG_KMS("tfp410 not detected got VID %X: from %s "
    "Target %d.\n",
    id, adapter.name, dvo.target_addr);
    goto out;
    }
    if ((id = tfp410_getid(dvo, TFP410_DID_LO)) != TFP410_DID) {
    DRM_DEBUG_KMS("tfp410 not detected got DID %X: from %s "
    "Target %d.\n",
    id, adapter.name, dvo.target_addr);
    goto out;
    }
    tfp.quiet = false;
    return true;
    out:
    kfree(tfp);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn tfp410_detect(dvo: *mut intel_dvo_device) -> enum drm_connector_status {
    static enum drm_connector_status tfp410_detect(struct intel_dvo_device *dvo)
    {
    let mut ret: enum drm_connector_status = connector_status_disconnected;
    u8 ctl2;
    if (tfp410_readb(dvo, TFP410_CTL_2, &ctl2)) {
    if (ctl2 & TFP410_CTL_2_RSEN)
    ret = connector_status_connected;
    else
    ret = connector_status_disconnected;
    }
    return ret;
    }
    static enum drm_mode_status tfp410_mode_valid(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode)
    {
    return MODE_OK;
    }
    static void tfp410_mode_set(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
// As long as the basics are set up, since we don't have clock dependencies
// in the mode setup, we can just leave the registers alone and everything
// will work fine.
//
// don't do much
    return;
    }
// set the tfp410 power state
#[no_mangle]
unsafe extern "C" fn tfp410_dpms(dvo: *mut intel_dvo_device, enable: bool) {
    static void tfp410_dpms(struct intel_dvo_device *dvo, bool enable)
    {
    u8 ctl1;
    if (!tfp410_readb(dvo, TFP410_CTL_1, &ctl1))
    return;
    if (enable)
    ctl1 |= TFP410_CTL_1_PD;
    else
    ctl1 &= ~TFP410_CTL_1_PD;
    tfp410_writeb(dvo, TFP410_CTL_1, ctl1);
    }
#[no_mangle]
unsafe extern "C" fn tfp410_get_hw_state(dvo: *mut intel_dvo_device) -> bool {
    static bool tfp410_get_hw_state(struct intel_dvo_device *dvo)
    {
    u8 ctl1;
    if (!tfp410_readb(dvo, TFP410_CTL_1, &ctl1))
    return false;
    if (ctl1 & TFP410_CTL_1_PD)
    return true;
    else
    return false;
    }
#[no_mangle]
unsafe extern "C" fn tfp410_dump_regs(dvo: *mut intel_dvo_device) {
    static void tfp410_dump_regs(struct intel_dvo_device *dvo)
    {
    u8 val, val2;
    tfp410_readb(dvo, TFP410_REV, &val);
    DRM_DEBUG_KMS("TFP410_REV: 0x%02X\n", val);
    tfp410_readb(dvo, TFP410_CTL_1, &val);
    DRM_DEBUG_KMS("TFP410_CTL1: 0x%02X\n", val);
    tfp410_readb(dvo, TFP410_CTL_2, &val);
    DRM_DEBUG_KMS("TFP410_CTL2: 0x%02X\n", val);
    tfp410_readb(dvo, TFP410_CTL_3, &val);
    DRM_DEBUG_KMS("TFP410_CTL3: 0x%02X\n", val);
    tfp410_readb(dvo, TFP410_USERCFG, &val);
    DRM_DEBUG_KMS("TFP410_USERCFG: 0x%02X\n", val);
    tfp410_readb(dvo, TFP410_DE_DLY, &val);
    DRM_DEBUG_KMS("TFP410_DE_DLY: 0x%02X\n", val);
    tfp410_readb(dvo, TFP410_DE_CTL, &val);
    DRM_DEBUG_KMS("TFP410_DE_CTL: 0x%02X\n", val);
    tfp410_readb(dvo, TFP410_DE_TOP, &val);
    DRM_DEBUG_KMS("TFP410_DE_TOP: 0x%02X\n", val);
    tfp410_readb(dvo, TFP410_DE_CNT_LO, &val);
    tfp410_readb(dvo, TFP410_DE_CNT_HI, &val2);
    DRM_DEBUG_KMS("TFP410_DE_CNT: 0x%02X%02X\n", val2, val);
    tfp410_readb(dvo, TFP410_DE_LIN_LO, &val);
    tfp410_readb(dvo, TFP410_DE_LIN_HI, &val2);
    DRM_DEBUG_KMS("TFP410_DE_LIN: 0x%02X%02X\n", val2, val);
    tfp410_readb(dvo, TFP410_H_RES_LO, &val);
    tfp410_readb(dvo, TFP410_H_RES_HI, &val2);
    DRM_DEBUG_KMS("TFP410_H_RES: 0x%02X%02X\n", val2, val);
    tfp410_readb(dvo, TFP410_V_RES_LO, &val);
    tfp410_readb(dvo, TFP410_V_RES_HI, &val2);
    DRM_DEBUG_KMS("TFP410_V_RES: 0x%02X%02X\n", val2, val);
    }
#[no_mangle]
unsafe extern "C" fn tfp410_destroy(dvo: *mut intel_dvo_device) {
    static void tfp410_destroy(struct intel_dvo_device *dvo)
    {
    struct tfp410_priv *tfp = dvo.dev_priv;
    if (tfp) {
    kfree(tfp);
    dvo.dev_priv = core::ptr::null_mut();
    }
    }
    const struct intel_dvo_dev_ops tfp410_ops = {
    .init = tfp410_init,
    .detect = tfp410_detect,
    .mode_valid = tfp410_mode_valid,
    .mode_set = tfp410_mode_set,
    .dpms = tfp410_dpms,
    .get_hw_state = tfp410_get_hw_state,
    .dump_regs = tfp410_dump_regs,
    .destroy = tfp410_destroy,
    };
