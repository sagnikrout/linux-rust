//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb/umt-010.c
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
// DVB USB framework compliant Linux driver for the HanfTek UMT-010 USB2.0
// DVB-T receiver.
//
// Copyright (C) 2004-5 Patrick Boettcher (patrick.boettcher@posteo.de)
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

    DVB_DEFINE_MOD_OPT_ADAPTER_NR(adapter_nr);
#[no_mangle]
unsafe extern "C" fn umt_mt352_demod_init(fe: *mut dvb_frontend) -> c_int {
    static int umt_mt352_demod_init(struct dvb_frontend *fe)
    {
    static u8 mt352_clock_config[] = { 0x89, 0xb8, 0x2d };
    static u8 mt352_reset[] = { 0x50, 0x80 };
    static u8 mt352_mclk_ratio[] = { 0x8b, 0x00 };
    static u8 mt352_adc_ctl_1_cfg[] = { 0x8E, 0x40 };
    static u8 mt352_agc_cfg[] = { 0x67, 0x10, 0xa0 };
    static u8 mt352_sec_agc_cfg1[] = { 0x6a, 0xff };
    static u8 mt352_sec_agc_cfg2[] = { 0x6d, 0xff };
    static u8 mt352_sec_agc_cfg3[] = { 0x70, 0x40 };
    static u8 mt352_sec_agc_cfg4[] = { 0x7b, 0x03 };
    static u8 mt352_sec_agc_cfg5[] = { 0x7d, 0x0f };
    static u8 mt352_acq_ctl[] = { 0x53, 0x50 };
    static u8 mt352_input_freq_1[] = { 0x56, 0x31, 0x06 };
    mt352_write(fe, mt352_clock_config, sizeof(mt352_clock_config));
    udelay(2000);
    mt352_write(fe, mt352_reset, sizeof(mt352_reset));
    mt352_write(fe, mt352_mclk_ratio, sizeof(mt352_mclk_ratio));
    mt352_write(fe, mt352_adc_ctl_1_cfg, sizeof(mt352_adc_ctl_1_cfg));
    mt352_write(fe, mt352_agc_cfg, sizeof(mt352_agc_cfg));
    mt352_write(fe, mt352_sec_agc_cfg1, sizeof(mt352_sec_agc_cfg1));
    mt352_write(fe, mt352_sec_agc_cfg2, sizeof(mt352_sec_agc_cfg2));
    mt352_write(fe, mt352_sec_agc_cfg3, sizeof(mt352_sec_agc_cfg3));
    mt352_write(fe, mt352_sec_agc_cfg4, sizeof(mt352_sec_agc_cfg4));
    mt352_write(fe, mt352_sec_agc_cfg5, sizeof(mt352_sec_agc_cfg5));
    mt352_write(fe, mt352_acq_ctl, sizeof(mt352_acq_ctl));
    mt352_write(fe, mt352_input_freq_1, sizeof(mt352_input_freq_1));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn umt_mt352_frontend_attach(adap: *mut dvb_usb_adapter) -> c_int {
    static int umt_mt352_frontend_attach(struct dvb_usb_adapter *adap)
    {
    struct mt352_config umt_config;
    memset(&umt_config,0,sizeof(struct mt352_config));
    umt_config.demod_init = umt_mt352_demod_init;
    umt_config.demod_address = 0xf;
    adap.fe_adap[0].fe = dvb_attach(mt352_attach, &umt_config, &adap.dev.i2c_adap);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn umt_tuner_attach(adap: *mut dvb_usb_adapter) -> c_int {
    static int umt_tuner_attach (struct dvb_usb_adapter *adap)
    {
    dvb_attach(dvb_pll_attach, adap.fe_adap[0].fe, 0x61, core::ptr::null_mut(), DVB_PLL_TUA6034);
    return 0;
    }
// USB Driver stuff
    static struct dvb_usb_device_properties umt_properties;
    static int umt_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    if (0 == dvb_usb_device_init(intf, &umt_properties,
    THIS_MODULE, core::ptr::null_mut(), adapter_nr))
    return 0;
    return -EINVAL;
    }
// do not change the order of the ID table
    enum {
    HANFTEK_UMT_010_COLD,
    HANFTEK_UMT_010_WARM,
    };
    static const struct usb_device_id umt_table[] = {
    DVB_USB_DEV(HANFTEK, HANFTEK_UMT_010_COLD),
    DVB_USB_DEV(HANFTEK, HANFTEK_UMT_010_WARM),
    { }
    };
    MODULE_DEVICE_TABLE (usb, umt_table);
    static struct dvb_usb_device_properties umt_properties = {
    .caps = DVB_USB_IS_AN_I2C_ADAPTER,
    .usb_ctrl = CYPRESS_FX2,
    .firmware = "dvb-usb-umt-010-02.fw",
    .num_adapters = 1,
    .adapter = {
    {
    .num_frontends = 1,
    .fe = {{
    .streaming_ctrl   = dibusb2_0_streaming_ctrl,
    .frontend_attach  = umt_mt352_frontend_attach,
    .tuner_attach     = umt_tuner_attach,
// parameter for the MPEG2-data transfer
    .stream = {
    .type = USB_BULK,
    .count = MAX_NO_URBS_FOR_DATA_STREAM,
    .endpoint = 0x06,
    .u = {
    .bulk = {
    .buffersize = 512,
    }
    }
    },
    }},
    .size_of_priv     = sizeof(struct dibusb_state),
    }
    },
    .power_ctrl       = dibusb_power_ctrl,
    .i2c_algo         = &dibusb_i2c_algo,
    .generic_bulk_ctrl_endpoint = 0x01,
    .num_device_descs = 1,
    .devices = {
    {	"Hanftek UMT-010 DVB-T USB2.0",
    { &umt_table[HANFTEK_UMT_010_COLD], core::ptr::null_mut() },
    { &umt_table[HANFTEK_UMT_010_WARM], core::ptr::null_mut() },
    },
    }
    };
    static struct usb_driver umt_driver = {
    .name		= "dvb_usb_umt_010",
    .probe		= umt_probe,
    .disconnect = dvb_usb_device_exit,
    .id_table	= umt_table,
    };
    module_usb_driver(umt_driver);
    MODULE_AUTHOR("Patrick Boettcher <patrick.boettcher@posteo.de>");
    MODULE_DESCRIPTION("Driver for HanfTek UMT 010 USB2.0 DVB-T device");
    MODULE_VERSION("1.0");
    MODULE_LICENSE("GPL");
