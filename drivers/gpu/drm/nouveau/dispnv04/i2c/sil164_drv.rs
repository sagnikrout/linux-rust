//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv04/i2c/sil164_drv.c
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
// Copyright (C) 2010 Francisco Jerez.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sil164_priv {
    pub config: sil164_encoder_params,
    pub duallink_slave: *mut i2c_client,
    pub saved_state: [u8; 0x10],
    pub saved_slave_state: [u8; 0x10],
}

    ((struct sil164_priv *)to_encoder_i2c(x).encoder_i2c_priv)

    if (drm_debug_enabled(DRM_UT_KMS))			\
    dev_printk(KERN_DEBUG, &client.dev,		\
    "%s: " format, __func__, ## __VA_ARGS__); \
    } while (0)

    dev_info(&client.dev, format, __VA_ARGS__)

    dev_err(&client.dev, format, __VA_ARGS__)
pub const SIL164_I2C_ADDR_MASTER: c_uint = 0x38;
pub const SIL164_I2C_ADDR_SLAVE: c_uint = 0x39;
// HW register definitions
pub const SIL164_VENDOR_LO: c_uint = 0x0;
pub const SIL164_VENDOR_HI: c_uint = 0x1;
pub const SIL164_DEVICE_LO: c_uint = 0x2;
pub const SIL164_DEVICE_HI: c_uint = 0x3;
pub const SIL164_REVISION: c_uint = 0x4;
pub const SIL164_FREQ_MIN: c_uint = 0x6;
pub const SIL164_FREQ_MAX: c_uint = 0x7;
pub const SIL164_CONTROL0: c_uint = 0x8;

pub const SIL164_DETECT: c_uint = 0x9;

pub const SIL164_CONTROL1: c_uint = 0xa;

pub const SIL164_GPIO: c_uint = 0xb;
pub const SIL164_CONTROL2: c_uint = 0xc;

pub const SIL164_DUALLINK: c_uint = 0xd;

pub const SIL164_PLLZONE: c_uint = 0xe;

// HW access functions
    static void
    sil164_write(struct i2c_client *client, uint8_t addr, uint8_t val)
    {
    uint8_t buf[] = {addr, val};
    int ret;
    ret = i2c_master_send(client, buf, ARRAY_SIZE(buf));
    if (ret < 0)
    sil164_err(client, "Error %d writing to subaddress 0x%x\n",
    ret, addr);
    }
    static uint8_t
    sil164_read(struct i2c_client *client, uint8_t addr)
    {
    uint8_t val;
    int ret;
    ret = i2c_master_send(client, &addr, sizeof(addr));
    if (ret < 0)
    goto fail;
    ret = i2c_master_recv(client, &val, sizeof(val));
    if (ret < 0)
    goto fail;
    return val;
    fail:
    sil164_err(client, "Error %d reading from subaddress 0x%x\n",
    ret, addr);
    return 0;
    }
    static void
    sil164_save_state(struct i2c_client *client, uint8_t *state)
    {
    int i;
    for (i = 0x8; i <= 0xe; i++)
    state[i] = sil164_read(client, i);
    }
    static void
    sil164_restore_state(struct i2c_client *client, uint8_t *state)
    {
    int i;
    for (i = 0x8; i <= 0xe; i++)
    sil164_write(client, i, state[i]);
    }
    static void
    sil164_set_power_state(struct i2c_client *client, bool on)
    {
    let mut control0: u8 = sil164_read(client, SIL164_CONTROL0);
    if (on)
    control0 |= SIL164_CONTROL0_POWER_ON;
    else
    control0 &= ~SIL164_CONTROL0_POWER_ON;
    sil164_write(client, SIL164_CONTROL0, control0);
    }
    static void
    sil164_init_state(struct i2c_client *client,
    struct sil164_encoder_params *config,
    bool duallink)
    {
    sil164_write(client, SIL164_CONTROL0,
    SIL164_CONTROL0_HSYNC_ON |
    SIL164_CONTROL0_VSYNC_ON |
    (config.input_edge ? SIL164_CONTROL0_EDGE_RISING : 0) |
    (config.input_width ? SIL164_CONTROL0_INPUT_24BIT : 0) |
    (config.input_dual ? SIL164_CONTROL0_DUAL_EDGE : 0));
    sil164_write(client, SIL164_DETECT,
    SIL164_DETECT_INTR_STAT |
    SIL164_DETECT_OUT_MODE_RECEIVER);
    sil164_write(client, SIL164_CONTROL1,
    (config.input_skew ? SIL164_CONTROL1_DESKEW_ENABLE : 0) |
    (((config.input_skew + 4) & 0x7)
    << SIL164_CONTROL1_DESKEW_INCR_SHIFT));
    sil164_write(client, SIL164_CONTROL2,
    SIL164_CONTROL2_SYNC_CONT |
    (config.pll_filter ? 0 : SIL164_CONTROL2_FILTER_ENABLE) |
    (4 << SIL164_CONTROL2_FILTER_SETTING_SHIFT));
    sil164_write(client, SIL164_PLLZONE, 0);
    if (duallink)
    sil164_write(client, SIL164_DUALLINK,
    SIL164_DUALLINK_ENABLE |
    (((config.duallink_skew + 4) & 0x7)
    << SIL164_DUALLINK_SKEW_SHIFT));
    else
    sil164_write(client, SIL164_DUALLINK, 0);
    }
// DRM encoder functions
    static void
    sil164_encoder_set_config(struct drm_encoder *encoder, void *params)
    {
    struct sil164_priv *priv = to_sil164_priv(encoder);
    priv.config = *(struct sil164_encoder_params *)params;
    }
    static void
    sil164_encoder_dpms(struct drm_encoder *encoder, int mode)
    {
    struct sil164_priv *priv = to_sil164_priv(encoder);
    let mut on: bool = (mode == DRM_MODE_DPMS_ON);
    let mut duallink: bool = (on && encoder.crtc.mode.clock > 165000);
    sil164_set_power_state(nouveau_i2c_encoder_get_client(encoder), on);
    if (priv.duallink_slave)
    sil164_set_power_state(priv.duallink_slave, duallink);
    }
    static void
    sil164_encoder_save(struct drm_encoder *encoder)
    {
    struct sil164_priv *priv = to_sil164_priv(encoder);
    sil164_save_state(nouveau_i2c_encoder_get_client(encoder),
    priv.saved_state);
    if (priv.duallink_slave)
    sil164_save_state(priv.duallink_slave,
    priv.saved_slave_state);
    }
    static void
    sil164_encoder_restore(struct drm_encoder *encoder)
    {
    struct sil164_priv *priv = to_sil164_priv(encoder);
    sil164_restore_state(nouveau_i2c_encoder_get_client(encoder),
    priv.saved_state);
    if (priv.duallink_slave)
    sil164_restore_state(priv.duallink_slave,
    priv.saved_slave_state);
    }
    static int
    sil164_encoder_mode_valid(struct drm_encoder *encoder,
    const struct drm_display_mode *mode)
    {
    struct sil164_priv *priv = to_sil164_priv(encoder);
    if (mode.clock < 32000)
    return MODE_CLOCK_LOW;
    if (mode.clock > 330000 ||
    (mode.clock > 165000 && !priv.duallink_slave))
    return MODE_CLOCK_HIGH;
    return MODE_OK;
    }
    static void
    sil164_encoder_mode_set(struct drm_encoder *encoder,
    struct drm_display_mode *mode,
    struct drm_display_mode *adjusted_mode)
    {
    struct sil164_priv *priv = to_sil164_priv(encoder);
    let mut duallink: bool = adjusted_mode.clock > 165000;
    sil164_init_state(nouveau_i2c_encoder_get_client(encoder),
    &priv.config, duallink);
    if (priv.duallink_slave)
    sil164_init_state(priv.duallink_slave,
    &priv.config, duallink);
    sil164_encoder_dpms(encoder, DRM_MODE_DPMS_ON);
    }
    static enum drm_connector_status
    sil164_encoder_detect(struct drm_encoder *encoder,
    struct drm_connector *connector)
    {
    struct i2c_client *client = nouveau_i2c_encoder_get_client(encoder);
    if (sil164_read(client, SIL164_DETECT) & SIL164_DETECT_HOTPLUG_STAT)
    return connector_status_connected;
    else
    return connector_status_disconnected;
    }
    static int
    sil164_encoder_get_modes(struct drm_encoder *encoder,
    struct drm_connector *connector)
    {
    return 0;
    }
    static int
    sil164_encoder_create_resources(struct drm_encoder *encoder,
    struct drm_connector *connector)
    {
    return 0;
    }
    static int
    sil164_encoder_set_property(struct drm_encoder *encoder,
    struct drm_connector *connector,
    struct drm_property *property,
    uint64_t val)
    {
    return 0;
    }
    static void
    sil164_encoder_destroy(struct drm_encoder *encoder)
    {
    struct sil164_priv *priv = to_sil164_priv(encoder);
    i2c_unregister_device(priv.duallink_slave);
    kfree(priv);
    nouveau_i2c_encoder_destroy(encoder);
    }
    static const struct nouveau_i2c_encoder_funcs sil164_encoder_funcs = {
    .set_config = sil164_encoder_set_config,
    .destroy = sil164_encoder_destroy,
    .dpms = sil164_encoder_dpms,
    .save = sil164_encoder_save,
    .restore = sil164_encoder_restore,
    .mode_valid = sil164_encoder_mode_valid,
    .mode_set = sil164_encoder_mode_set,
    .detect = sil164_encoder_detect,
    .get_modes = sil164_encoder_get_modes,
    .create_resources = sil164_encoder_create_resources,
    .set_property = sil164_encoder_set_property,
    };
// I2C driver functions
    static int
    sil164_probe(struct i2c_client *client)
    {
    int vendor = sil164_read(client, SIL164_VENDOR_HI) << 8 |
    sil164_read(client, SIL164_VENDOR_LO);
    int device = sil164_read(client, SIL164_DEVICE_HI) << 8 |
    sil164_read(client, SIL164_DEVICE_LO);
    let mut rev: c_int = sil164_read(client, SIL164_REVISION);
    if (vendor != 0x1 || device != 0x6) {
    sil164_dbg(client, "Unknown device %x:%x.%x\n",
    vendor, device, rev);
    return -ENODEV;
    }
    sil164_info(client, "Detected device %x:%x.%x\n",
    vendor, device, rev);
    return 0;
    }
    static struct i2c_client *
    sil164_detect_slave(struct i2c_client *client)
    {
    struct i2c_adapter *adap = client.adapter;
    struct i2c_msg msg = {
    .addr = SIL164_I2C_ADDR_SLAVE,
    .len = 0,
    };
    const struct i2c_board_info info = {
    I2C_BOARD_INFO("sil164", SIL164_I2C_ADDR_SLAVE)
    };
    if (i2c_transfer(adap, &msg, 1) != 1) {
    sil164_dbg(adap, "No dual-link slave found.");
    return core::ptr::null_mut();
    }
    return i2c_new_client_device(adap, &info);
    }
    static int
    sil164_encoder_init(struct i2c_client *client,
    struct drm_device *dev,
    struct nouveau_i2c_encoder *encoder)
    {
    struct sil164_priv *priv;
    struct i2c_client *slave_client;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    encoder.encoder_i2c_priv = priv;
    encoder.encoder_i2c_funcs = &sil164_encoder_funcs;
    slave_client = sil164_detect_slave(client);
    if (!IS_ERR(slave_client))
    priv.duallink_slave = slave_client;
    return 0;
    }
    static const struct i2c_device_id sil164_ids[] = {
    { "sil164" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sil164_ids);
    static struct nouveau_i2c_encoder_driver sil164_driver = {
    .i2c_driver = {
    .probe = sil164_probe,
    .driver = {
    .name = "sil164",
    },
    .id_table = sil164_ids,
    },
    .encoder_init = sil164_encoder_init,
    };
// Module initialization
    static int __init
    sil164_init(void)
    {
    return i2c_add_driver(&sil164_driver.i2c_driver);
    }
    static void __exit
    sil164_exit(void)
    {
    i2c_del_driver(&sil164_driver.i2c_driver);
    }
    MODULE_AUTHOR("Francisco Jerez <currojerez@riseup.net>");
    MODULE_DESCRIPTION("Silicon Image sil164 TMDS transmitter driver");
    MODULE_LICENSE("GPL and additional rights");
    module_init(sil164_init);
    module_exit(sil164_exit);
