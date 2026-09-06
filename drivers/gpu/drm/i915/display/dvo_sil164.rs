//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/display/dvo_sil164.c
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
    Copyright © 2006 Dave Airlie
    All Rights Reserved.
    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the
    "Software"), to deal in the Software without restriction, including
    without limitation the rights to use, copy, modify, merge, publish,
    distribute, sub license, and/or sell copies of the Software, and to
    permit persons to whom the Software is furnished to do so, subject to
    the following conditions:
    The above copyright notice and this permission notice (including the
    next paragraph) shall be included in all copies or substantial portions
    of the Software.
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
    OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
    IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
    ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
    TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub const SIL164_VID: c_uint = 0x0001;
pub const SIL164_DID: c_uint = 0x0006;
pub const SIL164_VID_LO: c_uint = 0x00;
pub const SIL164_VID_HI: c_uint = 0x01;
pub const SIL164_DID_LO: c_uint = 0x02;
pub const SIL164_DID_HI: c_uint = 0x03;
pub const SIL164_REV: c_uint = 0x04;
pub const SIL164_RSVD: c_uint = 0x05;
pub const SIL164_FREQ_LO: c_uint = 0x06;
pub const SIL164_FREQ_HI: c_uint = 0x07;
pub const SIL164_REG8: c_uint = 0x08;

pub const SIL164_REG9: c_uint = 0x09;

pub const SIL164_REGC: c_uint = 0x0c;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sil164_priv {
// I2CDevRec d;
    pub quiet: bool,
}

#[no_mangle]
unsafe extern "C" fn sil164_readb(dvo: *mut intel_dvo_device, addr: c_int, ch: *mut u8) -> bool {
    static bool sil164_readb(struct intel_dvo_device *dvo, int addr, u8 *ch)
    {
    struct sil164_priv *sil = dvo.dev_priv;
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
    if (!sil.quiet) {
    DRM_DEBUG_KMS("Unable to read register 0x%02x from %s:%02x.\n",
    addr, adapter.name, dvo.target_addr);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sil164_writeb(dvo: *mut intel_dvo_device, addr: c_int, ch: u8) -> bool {
    static bool sil164_writeb(struct intel_dvo_device *dvo, int addr, u8 ch)
    {
    struct sil164_priv *sil = dvo.dev_priv;
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
    if (!sil.quiet) {
    DRM_DEBUG_KMS("Unable to write register 0x%02x to %s:%d.\n",
    addr, adapter.name, dvo.target_addr);
    }
    return false;
    }
// Silicon Image 164 driver for chip on i2c bus
    static bool sil164_init(struct intel_dvo_device *dvo,
    struct i2c_adapter *adapter)
    {
// this will detect the SIL164 chip on the specified i2c bus
    struct sil164_priv *sil;
    unsigned char ch;
    sil = kzalloc_obj(*sil);
    if (sil == core::ptr::null_mut())
    return false;
    dvo.i2c_bus = adapter;
    dvo.dev_priv = sil;
    sil.quiet = true;
    if (!sil164_readb(dvo, SIL164_VID_LO, &ch))
    goto out;
    if (ch != (SIL164_VID & 0xff)) {
    DRM_DEBUG_KMS("sil164 not detected got %d: from %s Target %d.\n",
    ch, adapter.name, dvo.target_addr);
    goto out;
    }
    if (!sil164_readb(dvo, SIL164_DID_LO, &ch))
    goto out;
    if (ch != (SIL164_DID & 0xff)) {
    DRM_DEBUG_KMS("sil164 not detected got %d: from %s Target %d.\n",
    ch, adapter.name, dvo.target_addr);
    goto out;
    }
    sil.quiet = false;
    DRM_DEBUG_KMS("init sil164 dvo controller successfully!\n");
    return true;
    out:
    kfree(sil);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sil164_detect(dvo: *mut intel_dvo_device) -> enum drm_connector_status {
    static enum drm_connector_status sil164_detect(struct intel_dvo_device *dvo)
    {
    u8 reg9;
    sil164_readb(dvo, SIL164_REG9, &reg9);
    if (reg9 & SIL164_9_HTPLG)
    return connector_status_connected;
    else
    return connector_status_disconnected;
    }
    static enum drm_mode_status sil164_mode_valid(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode)
    {
    return MODE_OK;
    }
    static void sil164_mode_set(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
// As long as the basics are set up, since we don't have clock
// dependencies in the mode setup, we can just leave the
// registers alone and everything will work fine.
//
// recommended programming sequence from doc
// sil164_writeb(sil, 0x08, 0x30);
    sil164_writeb(sil, 0x09, 0x00);
    sil164_writeb(sil, 0x0a, 0x90);
    sil164_writeb(sil, 0x0c, 0x89);
    sil164_writeb(sil, 0x08, 0x31);*/
// don't do much
    sil164_writeb(dvo, SIL164_REG8,
    SIL164_8_VEN | SIL164_8_HEN);
    sil164_writeb(dvo, SIL164_REG9,
    SIL164_9_TSEL);
    sil164_writeb(dvo, SIL164_REGC,
    SIL164_C_PLLF_REC | SIL164_C_PFEN);
    }
// set the SIL164 power state
#[no_mangle]
unsafe extern "C" fn sil164_dpms(dvo: *mut intel_dvo_device, enable: bool) {
    static void sil164_dpms(struct intel_dvo_device *dvo, bool enable)
    {
    int ret;
    unsigned char ch;
    ret = sil164_readb(dvo, SIL164_REG8, &ch);
    if (ret == false)
    return;
    if (enable)
    ch |= SIL164_8_PD;
    else
    ch &= ~SIL164_8_PD;
    sil164_writeb(dvo, SIL164_REG8, ch);
    }
#[no_mangle]
unsafe extern "C" fn sil164_get_hw_state(dvo: *mut intel_dvo_device) -> bool {
    static bool sil164_get_hw_state(struct intel_dvo_device *dvo)
    {
    int ret;
    unsigned char ch;
    ret = sil164_readb(dvo, SIL164_REG8, &ch);
    if (ret == false)
    return false;
    if (ch & SIL164_8_PD)
    return true;
    else
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sil164_dump_regs(dvo: *mut intel_dvo_device) {
    static void sil164_dump_regs(struct intel_dvo_device *dvo)
    {
    u8 val;
    sil164_readb(dvo, SIL164_FREQ_LO, &val);
    DRM_DEBUG_KMS("SIL164_FREQ_LO: 0x%02x\n", val);
    sil164_readb(dvo, SIL164_FREQ_HI, &val);
    DRM_DEBUG_KMS("SIL164_FREQ_HI: 0x%02x\n", val);
    sil164_readb(dvo, SIL164_REG8, &val);
    DRM_DEBUG_KMS("SIL164_REG8: 0x%02x\n", val);
    sil164_readb(dvo, SIL164_REG9, &val);
    DRM_DEBUG_KMS("SIL164_REG9: 0x%02x\n", val);
    sil164_readb(dvo, SIL164_REGC, &val);
    DRM_DEBUG_KMS("SIL164_REGC: 0x%02x\n", val);
    }
#[no_mangle]
unsafe extern "C" fn sil164_destroy(dvo: *mut intel_dvo_device) {
    static void sil164_destroy(struct intel_dvo_device *dvo)
    {
    struct sil164_priv *sil = dvo.dev_priv;
    if (sil) {
    kfree(sil);
    dvo.dev_priv = core::ptr::null_mut();
    }
    }
    const struct intel_dvo_dev_ops sil164_ops = {
    .init = sil164_init,
    .detect = sil164_detect,
    .mode_valid = sil164_mode_valid,
    .mode_set = sil164_mode_set,
    .dpms = sil164_dpms,
    .get_hw_state = sil164_get_hw_state,
    .dump_regs = sil164_dump_regs,
    .destroy = sil164_destroy,
    };
