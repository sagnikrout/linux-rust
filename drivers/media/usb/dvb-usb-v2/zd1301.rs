//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb-v2/zd1301.c
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
// ZyDAS ZD1301 driver (USB interface)
//
// Copyright (C) 2015 Antti Palosaari <crope@iki.fi>
//

    DVB_DEFINE_MOD_OPT_ADAPTER_NR(adapter_nr);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd1301_dev {
pub const BUF_LEN: c_int = 8;
    pub /: *mut *mut u8 buf[BUF_LEN]; / bulk USB control message,
    pub demod_pdata: zd1301_demod_platform_data,
    pub mt2060_pdata: mt2060_platform_data,
    pub platform_device_demod: *mut platform_device,
    pub i2c_client_tuner: *mut i2c_client,
}

    static int zd1301_ctrl_msg(struct dvb_usb_device *d, const u8 *wbuf,
    unsigned int wlen, u8 *rbuf, unsigned int rlen)
    {
    struct zd1301_dev *dev = d_to_priv(d);
    struct usb_interface *intf = d.intf;
    int ret, actual_length;
    mutex_lock(&d.usb_mutex);
    memcpy(&dev.buf, wbuf, wlen);
    dev_dbg(&intf.dev, ">>> %*ph\n", wlen, dev.buf);
    ret = usb_bulk_msg(d.udev, usb_sndbulkpipe(d.udev, 0x04), dev.buf,
    wlen, &actual_length, 1000);
    if (ret) {
    dev_err(&intf.dev, "1st usb_bulk_msg() failed %d\n", ret);
    goto err_mutex_unlock;
    }
    if (rlen) {
    ret = usb_bulk_msg(d.udev, usb_rcvbulkpipe(d.udev, 0x83),
    dev.buf, rlen, &actual_length, 1000);
    if (ret) {
    dev_err(&intf.dev,
    "2nd usb_bulk_msg() failed %d\n", ret);
    goto err_mutex_unlock;
    }
    dev_dbg(&intf.dev, "<<< %*ph\n", actual_length, dev.buf);
    if (actual_length != rlen) {
//
// Chip replies often with 3 byte len stub. On that case
// we have to query new reply.
//
    dev_dbg(&intf.dev, "repeating reply message\n");
    ret = usb_bulk_msg(d.udev,
    usb_rcvbulkpipe(d.udev, 0x83),
    dev.buf, rlen, &actual_length,
    1000);
    if (ret) {
    dev_err(&intf.dev,
    "3rd usb_bulk_msg() failed %d\n", ret);
    goto err_mutex_unlock;
    }
    dev_dbg(&intf.dev,
    "<<< %*ph\n", actual_length, dev.buf);
    }
    memcpy(rbuf, dev.buf, rlen);
    }
    err_mutex_unlock:
    mutex_unlock(&d.usb_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zd1301_demod_wreg(reg_priv: *mut c_void, reg: u16, val: u8) -> c_int {
    static int zd1301_demod_wreg(void *reg_priv, u16 reg, u8 val)
    {
    struct dvb_usb_device *d = reg_priv;
    struct usb_interface *intf = d.intf;
    int ret;
    u8 buf[7] = {0x07, 0x00, 0x03, 0x01,
    (reg >> 0) & 0xff, (reg >> 8) & 0xff, val};
    ret = zd1301_ctrl_msg(d, buf, 7, core::ptr::null_mut(), 0);
    if (ret)
    goto err;
    return 0;
    err:
    dev_dbg(&intf.dev, "failed=%d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zd1301_demod_rreg(reg_priv: *mut c_void, reg: u16, val: *mut u8) -> c_int {
    static int zd1301_demod_rreg(void *reg_priv, u16 reg, u8 *val)
    {
    struct dvb_usb_device *d = reg_priv;
    struct usb_interface *intf = d.intf;
    int ret;
    u8 buf[7] = {0x07, 0x00, 0x04, 0x01,
    (reg >> 0) & 0xff, (reg >> 8) & 0xff, 0};
    ret = zd1301_ctrl_msg(d, buf, 7, buf, 7);
    if (ret)
    goto err;
// val = buf[6];
    return 0;
    err:
    dev_dbg(&intf.dev, "failed=%d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zd1301_frontend_attach(adap: *mut dvb_usb_adapter) -> c_int {
    static int zd1301_frontend_attach(struct dvb_usb_adapter *adap)
    {
    struct dvb_usb_device *d = adap_to_d(adap);
    struct zd1301_dev *dev = adap_to_priv(adap);
    struct usb_interface *intf = d.intf;
    struct platform_device *pdev;
    struct i2c_client *client;
    struct i2c_board_info board_info;
    struct i2c_adapter *adapter;
    struct dvb_frontend *frontend;
    int ret;
    dev_dbg(&intf.dev, "\n");
// Add platform demod
    dev.demod_pdata.reg_priv = d;
    dev.demod_pdata.reg_read = zd1301_demod_rreg;
    dev.demod_pdata.reg_write = zd1301_demod_wreg;
    request_module("%s", "zd1301_demod");
    pdev = platform_device_register_data(&intf.dev,
    "zd1301_demod",
    PLATFORM_DEVID_AUTO,
    &dev.demod_pdata,
    sizeof(dev.demod_pdata));
    if (IS_ERR(pdev)) {
    ret = PTR_ERR(pdev);
    goto err;
    }
    if (!pdev.dev.driver) {
    ret = -ENODEV;
    goto err_platform_device_unregister;
    }
    if (!try_module_get(pdev.dev.driver.owner)) {
    ret = -ENODEV;
    goto err_platform_device_unregister;
    }
    adapter = zd1301_demod_get_i2c_adapter(pdev);
    frontend = zd1301_demod_get_dvb_frontend(pdev);
    if (!adapter || !frontend) {
    ret = -ENODEV;
    goto err_module_put_demod;
    }
// Add I2C tuner
    dev.mt2060_pdata.i2c_write_max = 9;
    dev.mt2060_pdata.dvb_frontend = frontend;
    memset(&board_info, 0, sizeof(board_info));
    strscpy(board_info.type, "mt2060", I2C_NAME_SIZE);
    board_info.addr = 0x60;
    board_info.platform_data = &dev.mt2060_pdata;
    request_module("%s", "mt2060");
    client = i2c_new_client_device(adapter, &board_info);
    if (!i2c_client_has_driver(client)) {
    ret = -ENODEV;
    goto err_module_put_demod;
    }
    if (!try_module_get(client.dev.driver.owner)) {
    ret = -ENODEV;
    goto err_i2c_unregister_device;
    }
    dev.platform_device_demod = pdev;
    dev.i2c_client_tuner = client;
    adap.fe[0] = frontend;
    return 0;
    err_i2c_unregister_device:
    i2c_unregister_device(client);
    err_module_put_demod:
    module_put(pdev.dev.driver.owner);
    err_platform_device_unregister:
    platform_device_unregister(pdev);
    err:
    dev_dbg(&intf.dev, "failed=%d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zd1301_frontend_detach(adap: *mut dvb_usb_adapter) -> c_int {
    static int zd1301_frontend_detach(struct dvb_usb_adapter *adap)
    {
    struct dvb_usb_device *d = adap_to_d(adap);
    struct zd1301_dev *dev = d_to_priv(d);
    struct usb_interface *intf = d.intf;
    struct platform_device *pdev;
    struct i2c_client *client;
    dev_dbg(&intf.dev, "\n");
    client = dev.i2c_client_tuner;
    pdev = dev.platform_device_demod;
// Remove I2C tuner
    if (client) {
    module_put(client.dev.driver.owner);
    i2c_unregister_device(client);
    }
// Remove platform demod
    if (pdev) {
    module_put(pdev.dev.driver.owner);
    platform_device_unregister(pdev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zd1301_streaming_ctrl(fe: *mut dvb_frontend, onoff: c_int) -> c_int {
    static int zd1301_streaming_ctrl(struct dvb_frontend *fe, int onoff)
    {
    struct dvb_usb_device *d = fe_to_d(fe);
    struct usb_interface *intf = d.intf;
    int ret;
    u8 buf[3] = {0x03, 0x00, onoff ? 0x07 : 0x08};
    dev_dbg(&intf.dev, "onoff=%d\n", onoff);
    ret = zd1301_ctrl_msg(d, buf, 3, core::ptr::null_mut(), 0);
    if (ret)
    goto err;
    return 0;
    err:
    dev_dbg(&intf.dev, "failed=%d\n", ret);
    return ret;
    }
    static const struct dvb_usb_device_properties zd1301_props = {
    .driver_name = KBUILD_MODNAME,
    .owner = THIS_MODULE,
    .adapter_nr = adapter_nr,
    .size_of_priv = sizeof(struct zd1301_dev),
    .frontend_attach = zd1301_frontend_attach,
    .frontend_detach = zd1301_frontend_detach,
    .streaming_ctrl  = zd1301_streaming_ctrl,
    .num_adapters = 1,
    .adapter = {
    {
    .stream = DVB_USB_STREAM_BULK(0x81, 6, 21 * 188),
    },
    },
    };
    static const struct usb_device_id zd1301_id_table[] = {
    {DVB_USB_DEVICE(USB_VID_ZYDAS, 0x13a1, &zd1301_props,
    "ZyDAS ZD1301 reference design", core::ptr::null_mut())},
    {}
    };
    MODULE_DEVICE_TABLE(usb, zd1301_id_table);
// Usb specific object needed to register this driver with the usb subsystem
    static struct usb_driver zd1301_usb_driver = {
    .name = KBUILD_MODNAME,
    .id_table = zd1301_id_table,
    .probe = dvb_usbv2_probe,
    .disconnect = dvb_usbv2_disconnect,
    .suspend = dvb_usbv2_suspend,
    .resume = dvb_usbv2_resume,
    .reset_resume = dvb_usbv2_reset_resume,
    .no_dynamic_id = 1,
    .soft_unbind = 1,
    };
    module_usb_driver(zd1301_usb_driver);
    MODULE_AUTHOR("Antti Palosaari <crope@iki.fi>");
    MODULE_DESCRIPTION("ZyDAS ZD1301 driver");
    MODULE_LICENSE("GPL");
