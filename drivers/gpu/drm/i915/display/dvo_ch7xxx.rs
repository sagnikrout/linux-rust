//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/display/dvo_ch7xxx.c
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

pub const CH7xxx_REG_VID: c_uint = 0x4a;
pub const CH7xxx_REG_DID: c_uint = 0x4b;
pub const CH7011_VID: c_uint = 0x83 /* 7010 as well */;
pub const CH7010B_VID: c_uint = 0x05;
pub const CH7009A_VID: c_uint = 0x84;
pub const CH7009B_VID: c_uint = 0x85;
pub const CH7301_VID: c_uint = 0x95;
pub const CH7xxx_VID: c_uint = 0x84;
pub const CH7xxx_DID: c_uint = 0x17;
pub const CH7010_DID: c_uint = 0x16;
pub const CH7xxx_NUM_REGS: c_uint = 0x4c;
pub const CH7xxx_CM: c_uint = 0x1c;

pub const CH7xxx_INPUT_CLOCK: c_uint = 0x1d;
pub const CH7xxx_GPIO: c_uint = 0x1e;

pub const CH7xxx_IDF: c_uint = 0x1f;

pub const CH7xxx_CONNECTION_DETECT: c_uint = 0x20;

pub const CH7xxx_DAC_CNTL: c_uint = 0x21;

pub const CH7xxx_CLOCK_OUTPUT: c_uint = 0x22;

pub const CH7301_HOTPLUG: c_uint = 0x23;
pub const CH7xxx_TCTL: c_uint = 0x31;
pub const CH7xxx_TVCO: c_uint = 0x32;
pub const CH7xxx_TPCP: c_uint = 0x33;
pub const CH7xxx_TPD: c_uint = 0x34;
pub const CH7xxx_TPVT: c_uint = 0x35;
pub const CH7xxx_TLPF: c_uint = 0x36;
pub const CH7xxx_TCT: c_uint = 0x37;
pub const CH7301_TEST_PATTERN: c_uint = 0x48;
pub const CH7xxx_PM: c_uint = 0x49;

pub const CH7301_SYNC_POLARITY: c_uint = 0x56;

// @file
// driver for the Chrontel 7xxx DVI chip over DVO.
//
    static struct ch7xxx_id_struct {
    u8 vid;
    char *name;
    } ch7xxx_ids[] = {
    { CH7011_VID, "CH7011" },
    { CH7010B_VID, "CH7010B" },
    { CH7009A_VID, "CH7009A" },
    { CH7009B_VID, "CH7009B" },
    { CH7301_VID, "CH7301" },
    };
    static struct ch7xxx_did_struct {
    u8 did;
    char *name;
    } ch7xxx_dids[] = {
    { CH7xxx_DID, "CH7XXX" },
    { CH7010_DID, "CH7010B" },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch7xxx_priv {
    pub quiet: bool,
}

    static char *ch7xxx_get_id(u8 vid)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(ch7xxx_ids); i++) {
    if (ch7xxx_ids[i].vid == vid)
    return ch7xxx_ids[i].name;
    }
    return core::ptr::null_mut();
    }
    static char *ch7xxx_get_did(u8 did)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(ch7xxx_dids); i++) {
    if (ch7xxx_dids[i].did == did)
    return ch7xxx_dids[i].name;
    }
    return core::ptr::null_mut();
    }
// Reads an 8 bit register
#[no_mangle]
unsafe extern "C" fn ch7xxx_readb(dvo: *mut intel_dvo_device, addr: c_int, ch: *mut u8) -> bool {
    static bool ch7xxx_readb(struct intel_dvo_device *dvo, int addr, u8 *ch)
    {
    struct ch7xxx_priv *ch7xxx = dvo.dev_priv;
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
    if (!ch7xxx.quiet) {
    DRM_DEBUG_KMS("Unable to read register 0x%02x from %s:%02x.\n",
    addr, adapter.name, dvo.target_addr);
    }
    return false;
    }
// Writes an 8 bit register
#[no_mangle]
unsafe extern "C" fn ch7xxx_writeb(dvo: *mut intel_dvo_device, addr: c_int, ch: u8) -> bool {
    static bool ch7xxx_writeb(struct intel_dvo_device *dvo, int addr, u8 ch)
    {
    struct ch7xxx_priv *ch7xxx = dvo.dev_priv;
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
    if (!ch7xxx.quiet) {
    DRM_DEBUG_KMS("Unable to write register 0x%02x to %s:%d.\n",
    addr, adapter.name, dvo.target_addr);
    }
    return false;
    }
    static bool ch7xxx_init(struct intel_dvo_device *dvo,
    struct i2c_adapter *adapter)
    {
// this will detect the CH7xxx chip on the specified i2c bus
    struct ch7xxx_priv *ch7xxx;
    u8 vendor, device;
    char *name, *devid;
    ch7xxx = kzalloc_obj(*ch7xxx);
    if (ch7xxx == core::ptr::null_mut())
    return false;
    dvo.i2c_bus = adapter;
    dvo.dev_priv = ch7xxx;
    ch7xxx.quiet = true;
    if (!ch7xxx_readb(dvo, CH7xxx_REG_VID, &vendor))
    goto out;
    name = ch7xxx_get_id(vendor);
    if (!name) {
    DRM_DEBUG_KMS("ch7xxx not detected; got VID 0x%02x from %s target %d.\n",
    vendor, adapter.name, dvo.target_addr);
    goto out;
    }
    if (!ch7xxx_readb(dvo, CH7xxx_REG_DID, &device))
    goto out;
    devid = ch7xxx_get_did(device);
    if (!devid) {
    DRM_DEBUG_KMS("ch7xxx not detected; got DID 0x%02x from %s target %d.\n",
    device, adapter.name, dvo.target_addr);
    goto out;
    }
    ch7xxx.quiet = false;
    DRM_DEBUG_KMS("Detected %s chipset, vendor/device ID 0x%02x/0x%02x\n",
    name, vendor, device);
    return true;
    out:
    kfree(ch7xxx);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ch7xxx_detect(dvo: *mut intel_dvo_device) -> enum drm_connector_status {
    static enum drm_connector_status ch7xxx_detect(struct intel_dvo_device *dvo)
    {
    u8 cdet, orig_pm, pm;
    ch7xxx_readb(dvo, CH7xxx_PM, &orig_pm);
    pm = orig_pm;
    pm &= ~CH7xxx_PM_FPD;
    pm |= CH7xxx_PM_DVIL | CH7xxx_PM_DVIP;
    ch7xxx_writeb(dvo, CH7xxx_PM, pm);
    ch7xxx_readb(dvo, CH7xxx_CONNECTION_DETECT, &cdet);
    ch7xxx_writeb(dvo, CH7xxx_PM, orig_pm);
    if (cdet & CH7xxx_CDET_DVI)
    return connector_status_connected;
    return connector_status_disconnected;
    }
    static enum drm_mode_status ch7xxx_mode_valid(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode)
    {
    if (mode.clock > 165000)
    return MODE_CLOCK_HIGH;
    return MODE_OK;
    }
    static void ch7xxx_mode_set(struct intel_dvo_device *dvo,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
    u8 tvco, tpcp, tpd, tlpf, idf;
    if (mode.clock <= 65000) {
    tvco = 0x23;
    tpcp = 0x08;
    tpd = 0x16;
    tlpf = 0x60;
    } else {
    tvco = 0x2d;
    tpcp = 0x06;
    tpd = 0x26;
    tlpf = 0xa0;
    }
    ch7xxx_writeb(dvo, CH7xxx_TCTL, 0x00);
    ch7xxx_writeb(dvo, CH7xxx_TVCO, tvco);
    ch7xxx_writeb(dvo, CH7xxx_TPCP, tpcp);
    ch7xxx_writeb(dvo, CH7xxx_TPD, tpd);
    ch7xxx_writeb(dvo, CH7xxx_TPVT, 0x30);
    ch7xxx_writeb(dvo, CH7xxx_TLPF, tlpf);
    ch7xxx_writeb(dvo, CH7xxx_TCT, 0x00);
    ch7xxx_readb(dvo, CH7xxx_IDF, &idf);
    idf |= CH7xxx_IDF_IBS;
    idf &= ~(CH7xxx_IDF_HSP | CH7xxx_IDF_VSP);
    if (mode.flags & DRM_MODE_FLAG_PHSYNC)
    idf |= CH7xxx_IDF_HSP;
    if (mode.flags & DRM_MODE_FLAG_PVSYNC)
    idf |= CH7xxx_IDF_VSP;
    ch7xxx_writeb(dvo, CH7xxx_IDF, idf);
    ch7xxx_writeb(dvo, CH7xxx_DAC_CNTL,
    CH7xxx_SYNCO_VGA_HSYNC);
    ch7xxx_writeb(dvo, CH7xxx_CLOCK_OUTPUT,
    CH7xxx_BCOEN | CH7xxx_BCO_VGA_VSYNC);
    }
// set the CH7xxx power state
#[no_mangle]
unsafe extern "C" fn ch7xxx_dpms(dvo: *mut intel_dvo_device, enable: bool) {
    static void ch7xxx_dpms(struct intel_dvo_device *dvo, bool enable)
    {
    if (enable)
    ch7xxx_writeb(dvo, CH7xxx_PM, CH7xxx_PM_DVIL | CH7xxx_PM_DVIP);
    else
    ch7xxx_writeb(dvo, CH7xxx_PM, CH7xxx_PM_FPD);
    }
#[no_mangle]
unsafe extern "C" fn ch7xxx_get_hw_state(dvo: *mut intel_dvo_device) -> bool {
    static bool ch7xxx_get_hw_state(struct intel_dvo_device *dvo)
    {
    u8 val;
    ch7xxx_readb(dvo, CH7xxx_PM, &val);
    if (val & (CH7xxx_PM_DVIL | CH7xxx_PM_DVIP))
    return true;
    else
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ch7xxx_dump_regs(dvo: *mut intel_dvo_device) {
    static void ch7xxx_dump_regs(struct intel_dvo_device *dvo)
    {
    int i;
    for (i = 0; i < CH7xxx_NUM_REGS; i++) {
    u8 val;
    if ((i % 8) == 0)
    DRM_DEBUG_KMS("\n %02X: ", i);
    ch7xxx_readb(dvo, i, &val);
    DRM_DEBUG_KMS("%02X ", val);
    }
    }
#[no_mangle]
unsafe extern "C" fn ch7xxx_destroy(dvo: *mut intel_dvo_device) {
    static void ch7xxx_destroy(struct intel_dvo_device *dvo)
    {
    struct ch7xxx_priv *ch7xxx = dvo.dev_priv;
    if (ch7xxx) {
    kfree(ch7xxx);
    dvo.dev_priv = core::ptr::null_mut();
    }
    }
    const struct intel_dvo_dev_ops ch7xxx_ops = {
    .init = ch7xxx_init,
    .detect = ch7xxx_detect,
    .mode_valid = ch7xxx_mode_valid,
    .mode_set = ch7xxx_mode_set,
    .dpms = ch7xxx_dpms,
    .get_hw_state = ch7xxx_get_hw_state,
    .dump_regs = ch7xxx_dump_regs,
    .destroy = ch7xxx_destroy,
    };
