//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/hdpvr/hdpvr-i2c.c
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
// Hauppauge HD PVR USB driver
//
// Copyright (C) 2008      Janne Grunau (j@jannau.net)
//
// IR device registration code is
// Copyright (C) 2010	Andy Walls <awalls@md.metrocast.net>
//

pub const CTRL_READ_REQUEST: c_uint = 0xb8;
pub const CTRL_WRITE_REQUEST: c_uint = 0x38;
pub const REQTYPE_I2C_READ: c_uint = 0xb1;
pub const REQTYPE_I2C_WRITE: c_uint = 0xb0;
pub const REQTYPE_I2C_WRITE_STATT: c_uint = 0xd0;
pub const Z8F0811_IR_TX_I2C_ADDR: c_uint = 0x70;
pub const Z8F0811_IR_RX_I2C_ADDR: c_uint = 0x71;
    struct i2c_client *hdpvr_register_ir_i2c(struct hdpvr_device *dev)
    {
    struct IR_i2c_init_data *init_data = &dev.ir_i2c_init_data;
    struct i2c_board_info info = {
    I2C_BOARD_INFO("ir_z8f0811_hdpvr", Z8F0811_IR_RX_I2C_ADDR),
    };
// Our default information for ir-kbd-i2c.c to use
    init_data.ir_codes = RC_MAP_HAUPPAUGE;
    init_data.internal_get_key_func = IR_KBD_GET_KEY_HAUP_XVR;
    init_data.type = RC_PROTO_BIT_RC5 | RC_PROTO_BIT_RC6_MCE |
    RC_PROTO_BIT_RC6_6A_32;
    init_data.name = "HD-PVR";
    init_data.polling_interval = 405; /* ms, duplicated from Windows */
    info.platform_data = init_data;
    return i2c_new_client_device(&dev.i2c_adapter, &info);
    }
    static int hdpvr_i2c_read(struct hdpvr_device *dev, int bus,
    unsigned char addr, char *wdata, int wlen,
    char *data, int len)
    {
    int ret;
    if ((len > sizeof(dev.i2c_buf)) || (wlen > sizeof(dev.i2c_buf)))
    return -EINVAL;
    if (wlen) {
    memcpy(dev.i2c_buf, wdata, wlen);
    ret = usb_control_msg(dev.udev, usb_sndctrlpipe(dev.udev, 0),
    REQTYPE_I2C_WRITE, CTRL_WRITE_REQUEST,
    (bus << 8) | addr, 0, dev.i2c_buf,
    wlen, 1000);
    if (ret < 0)
    return ret;
    }
    ret = usb_control_msg(dev.udev, usb_rcvctrlpipe(dev.udev, 0),
    REQTYPE_I2C_READ, CTRL_READ_REQUEST,
    (bus << 8) | addr, 0, dev.i2c_buf, len, 1000);
    if (ret == len) {
    memcpy(data, dev.i2c_buf, len);
    ret = 0;
    } else if (ret >= 0)
    ret = -EIO;
    return ret;
    }
    static int hdpvr_i2c_write(struct hdpvr_device *dev, int bus,
    unsigned char addr, char *data, int len)
    {
    int ret;
    if (len > sizeof(dev.i2c_buf))
    return -EINVAL;
    memcpy(dev.i2c_buf, data, len);
    ret = usb_control_msg(dev.udev, usb_sndctrlpipe(dev.udev, 0),
    REQTYPE_I2C_WRITE, CTRL_WRITE_REQUEST,
    (bus << 8) | addr, 0, dev.i2c_buf, len, 1000);
    if (ret < 0)
    return ret;
    ret = usb_control_msg(dev.udev, usb_rcvctrlpipe(dev.udev, 0),
    REQTYPE_I2C_WRITE_STATT, CTRL_READ_REQUEST,
    0, 0, dev.i2c_buf, 2, 1000);
    if ((ret == 2) && (dev.i2c_buf[1] == (len - 1)))
    ret = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret >=) -> else {
    else if (ret >= 0)
    ret = -EIO;
    return ret;
    }
    static int hdpvr_transfer(struct i2c_adapter *i2c_adapter, struct i2c_msg *msgs,
    int num)
    {
    struct hdpvr_device *dev = i2c_get_adapdata(i2c_adapter);
    let mut retval: c_int = 0, addr;
    mutex_lock(&dev.i2c_mutex);
    addr = msgs[0].addr << 1;
    if (num == 1) {
    if (msgs[0].flags & I2C_M_RD)
    retval = hdpvr_i2c_read(dev, 1, addr, core::ptr::null_mut(), 0,
    msgs[0].buf, msgs[0].len);
    else
    retval = hdpvr_i2c_write(dev, 1, addr, msgs[0].buf,
    msgs[0].len);
    } else {
// do write-then-read
    retval = hdpvr_i2c_read(dev, 1, addr, msgs[0].buf, msgs[0].len,
    msgs[1].buf, msgs[1].len);
    }
    mutex_unlock(&dev.i2c_mutex);
    return retval ? retval : num;
    }
#[no_mangle]
unsafe extern "C" fn hdpvr_functionality(adapter: *mut i2c_adapter) -> u32 {
    static u32 hdpvr_functionality(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm hdpvr_algo = {
    .master_xfer   = hdpvr_transfer,
    .functionality = hdpvr_functionality,
    };
// prevent invalid 0-length usb_control_msg and support only write-then-read
    static const struct i2c_adapter_quirks hdpvr_quirks = {
    .flags = I2C_AQ_NO_ZERO_LEN_READ | I2C_AQ_COMB_WRITE_THEN_READ,
    };
    static const struct i2c_adapter hdpvr_i2c_adapter_template = {
    .name   = "Hauppauge HD PVR I2C",
    .owner  = THIS_MODULE,
    .algo   = &hdpvr_algo,
    .quirks = &hdpvr_quirks,
    };
#[no_mangle]
unsafe extern "C" fn hdpvr_activate_ir(dev: *mut hdpvr_device) -> c_int {
    static int hdpvr_activate_ir(struct hdpvr_device *dev)
    {
    char buffer[2];
    mutex_lock(&dev.i2c_mutex);
    hdpvr_i2c_read(dev, 0, 0x54, core::ptr::null_mut(), 0, buffer, 1);
    buffer[0] = 0;
    buffer[1] = 0x8;
    hdpvr_i2c_write(dev, 1, 0x54, buffer, 2);
    buffer[1] = 0x18;
    hdpvr_i2c_write(dev, 1, 0x54, buffer, 2);
    mutex_unlock(&dev.i2c_mutex);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hdpvr_register_i2c_adapter(dev: *mut hdpvr_device) -> c_int {
    int hdpvr_register_i2c_adapter(struct hdpvr_device *dev)
    {
    hdpvr_activate_ir(dev);
    dev.i2c_adapter = hdpvr_i2c_adapter_template;
    dev.i2c_adapter.dev.parent = &dev.udev.dev;
    i2c_set_adapdata(&dev.i2c_adapter, dev);
    return i2c_add_adapter(&dev.i2c_adapter);
    }
