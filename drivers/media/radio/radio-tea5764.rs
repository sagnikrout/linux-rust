//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/radio-tea5764.c
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
// driver/media/radio/radio-tea5764.c
//
// Driver for TEA5764 radio chip for linux 2.6.
// This driver is for TEA5764 chip from NXP, used in EZX phones from Motorola.
// The I2C protocol is used for communicate with chip.
//
// Based in radio-tea5761.c Copyright (C) 2005 Nokia Corporation
//
// Copyright (c) 2008 Fabio Belavenuto <belavenuto@gmail.com>
//
// History:
// 2008-12-06   Fabio Belavenuto <belavenuto@gmail.com>
// initial code
//
// TODO:
// add platform_data support for IRQs platform dependencies
// add RDS support
//

    printk(KERN_INFO KBUILD_MODNAME ": "\
    DRIVER_VERSION ": " format "\n", ## __VA_ARGS__)

    printk(KERN_WARNING KBUILD_MODNAME ": "\
    DRIVER_VERSION ": " format "\n", ## __VA_ARGS__)

    printk(KERN_DEBUG KBUILD_MODNAME ": "\
    DRIVER_VERSION ": " format "\n", ## __VA_ARGS__)
// Frequency limits in MHz -- these are European values.  For Japanese
    devices, that would be 76000 and 91000.  */

pub const FREQ_MUL: c_int = 16;
// TEA5764 registers
pub const TEA5764_MANID: c_uint = 0x002b;
pub const TEA5764_CHIPID: c_uint = 0x5764;
pub const TEA5764_INTREG_BLMSK: c_uint = 0x0001;
pub const TEA5764_INTREG_FRRMSK: c_uint = 0x0002;
pub const TEA5764_INTREG_LEVMSK: c_uint = 0x0008;
pub const TEA5764_INTREG_IFMSK: c_uint = 0x0010;
pub const TEA5764_INTREG_BLMFLAG: c_uint = 0x0100;
pub const TEA5764_INTREG_FRRFLAG: c_uint = 0x0200;
pub const TEA5764_INTREG_LEVFLAG: c_uint = 0x0800;
pub const TEA5764_INTREG_IFFLAG: c_uint = 0x1000;
pub const TEA5764_FRQSET_SUD: c_uint = 0x8000;
pub const TEA5764_FRQSET_SM: c_uint = 0x4000;
pub const TEA5764_TNCTRL_PUPD1: c_uint = 0x8000;
pub const TEA5764_TNCTRL_PUPD0: c_uint = 0x4000;
pub const TEA5764_TNCTRL_BLIM: c_uint = 0x2000;
pub const TEA5764_TNCTRL_SWPM: c_uint = 0x1000;
pub const TEA5764_TNCTRL_IFCTC: c_uint = 0x0800;
pub const TEA5764_TNCTRL_AFM: c_uint = 0x0400;
pub const TEA5764_TNCTRL_SMUTE: c_uint = 0x0200;
pub const TEA5764_TNCTRL_SNC: c_uint = 0x0100;
pub const TEA5764_TNCTRL_MU: c_uint = 0x0080;
pub const TEA5764_TNCTRL_SSL1: c_uint = 0x0040;
pub const TEA5764_TNCTRL_SSL0: c_uint = 0x0020;
pub const TEA5764_TNCTRL_HLSI: c_uint = 0x0010;
pub const TEA5764_TNCTRL_MST: c_uint = 0x0008;
pub const TEA5764_TNCTRL_SWP: c_uint = 0x0004;
pub const TEA5764_TNCTRL_DTC: c_uint = 0x0002;
pub const TEA5764_TNCTRL_AHLSI: c_uint = 0x0001;

pub const TEA5764_TUNCHK_TUNTO: c_uint = 0x0100;
pub const TEA5764_TUNCHK_LD: c_uint = 0x0008;
pub const TEA5764_TUNCHK_STEREO: c_uint = 0x0004;
pub const TEA5764_TESTREG_TRIGFR: c_uint = 0x0800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tea5764_regs {
    pub /: *mut *mut u16 intreg; / INTFLAG & INTMSK,
    pub /: *mut *mut u16 frqset; / FRQSETMSB & FRQSETLSB,
    pub /: *mut *mut u16 tnctrl; / TNCTRL1 & TNCTRL2,
    pub /: *mut *mut u16 frqchk; / FRQCHKMSB & FRQCHKLSB,
    pub /: *mut *mut u16 tunchk; / IFCHK & LEVCHK,
    pub /: *mut *mut u16 testreg; / TESTBITS & TESTMODE,
    pub /: *mut *mut u16 rdsstat; / RDSSTAT1 & RDSSTAT2,
    pub /: *mut *mut u16 rdslb; / RDSLBMSB & RDSLBLSB,
    pub /: *mut *mut u16 rdspb; / RDSPBMSB & RDSPBLSB,
    pub /: *mut *mut u16 rdsbc; / RDSBBC & RDSGBC,
    pub /: *mut *mut u16 rdsctrl; / RDSCTRL1 & RDSCTRL2,
    pub /: *mut *mut u16 rdsbbl; / PAUSEDET & RDSBBL,
    pub /: *mut *mut u16 manid; / MANID1 & MANID2,
    pub /: *mut *mut u16 chipid; / CHIPID1 & CHIPID2,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tea5764_write_regs {
    pub /: *mut *mut u8 intreg; / INTMSK,
    pub /: *mut *mut __be16 frqset; / FRQSETMSB & FRQSETLSB,
    pub /: *mut *mut __be16 tnctrl; / TNCTRL1 & TNCTRL2,
    pub /: *mut *mut __be16 testreg; / TESTBITS & TESTMODE,
    pub /: *mut *mut __be16 rdsctrl; / RDSCTRL1 & RDSCTRL2,
    pub /: *mut *mut __be16 rdsbbl; / PAUSEDET & RDSBBL,
// C attribute field omitted

pub const RADIO_TEA5764_XTAL: c_int = 1;

pub const RADIO_TEA5764_XTAL: c_int = 0;

    pub -1: static int radio_nr =,
    pub RADIO_TEA5764_XTAL: static int use_xtal =,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tea5764_device {
    pub v4l2_dev: v4l2_device,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub i2c_client: *mut i2c_client,
    pub vdev: video_device,
    pub regs: tea5764_regs,
    pub mutex: mutex,
}

// I2C code related
#[no_mangle]
unsafe extern "C" fn tea5764_i2c_read(radio: *mut tea5764_device) -> c_int {
    static int tea5764_i2c_read(struct tea5764_device *radio)
    {
    int i;
    u16 *p = (u16 *) &radio.regs;
    struct i2c_msg msgs[1] = {
    {	.addr = radio.i2c_client.addr,
    .flags = I2C_M_RD,
    .len = sizeof(radio.regs),
    .buf = (void *)&radio.regs
    },
    };
    if (i2c_transfer(radio.i2c_client.adapter, msgs, 1) != 1)
    return -EIO;
    for (i = 0; i < sizeof(struct tea5764_regs) / sizeof(u16); i++)
    p[i] = __be16_to_cpu(( __be16)p[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tea5764_i2c_write(radio: *mut tea5764_device) -> c_int {
    static int tea5764_i2c_write(struct tea5764_device *radio)
    {
    struct tea5764_write_regs wr;
    struct tea5764_regs *r = &radio.regs;
    struct i2c_msg msgs[1] = {
    {
    .addr = radio.i2c_client.addr,
    .len = sizeof(wr),
    .buf = (void *)&wr
    },
    };
    wr.intreg  = r.intreg & 0xff;
    wr.frqset  = __cpu_to_be16(r.frqset);
    wr.tnctrl  = __cpu_to_be16(r.tnctrl);
    wr.testreg = __cpu_to_be16(r.testreg);
    wr.rdsctrl = __cpu_to_be16(r.rdsctrl);
    wr.rdsbbl  = __cpu_to_be16(r.rdsbbl);
    if (i2c_transfer(radio.i2c_client.adapter, msgs, 1) != 1)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tea5764_power_up(radio: *mut tea5764_device) {
    static void tea5764_power_up(struct tea5764_device *radio)
    {
    struct tea5764_regs *r = &radio.regs;
    if (!(r.tnctrl & TEA5764_TNCTRL_PUPD0)) {
    r.tnctrl &= ~(TEA5764_TNCTRL_AFM | TEA5764_TNCTRL_MU |
    TEA5764_TNCTRL_HLSI);
    if (!use_xtal)
    r.testreg |= TEA5764_TESTREG_TRIGFR;
    else
    r.testreg &= ~TEA5764_TESTREG_TRIGFR;
    r.tnctrl |= TEA5764_TNCTRL_PUPD0;
    tea5764_i2c_write(radio);
    }
    }
#[no_mangle]
unsafe extern "C" fn tea5764_power_down(radio: *mut tea5764_device) {
    static void tea5764_power_down(struct tea5764_device *radio)
    {
    struct tea5764_regs *r = &radio.regs;
    if (r.tnctrl & TEA5764_TNCTRL_PUPD0) {
    r.tnctrl &= ~TEA5764_TNCTRL_PUPD0;
    tea5764_i2c_write(radio);
    }
    }
#[no_mangle]
unsafe extern "C" fn tea5764_set_freq(radio: *mut tea5764_device, freq: c_int) {
    static void tea5764_set_freq(struct tea5764_device *radio, int freq)
    {
    struct tea5764_regs *r = &radio.regs;
// formula: (freq [+ or -] 225000) / 8192
    if (r.tnctrl & TEA5764_TNCTRL_HLSI)
    r.frqset = (freq + 225000) / 8192;
    else
    r.frqset = (freq - 225000) / 8192;
    }
#[no_mangle]
unsafe extern "C" fn tea5764_get_freq(radio: *mut tea5764_device) -> c_int {
    static int tea5764_get_freq(struct tea5764_device *radio)
    {
    struct tea5764_regs *r = &radio.regs;
    if (r.tnctrl & TEA5764_TNCTRL_HLSI)
    return (r.frqchk * 8192) - 225000;
    else
    return (r.frqchk * 8192) + 225000;
    }
// tune an frequency, freq is defined by v4l's TUNER_LOW, i.e. 1/16th kHz
#[no_mangle]
unsafe extern "C" fn tea5764_tune(radio: *mut tea5764_device, freq: c_int) {
    static void tea5764_tune(struct tea5764_device *radio, int freq)
    {
    tea5764_set_freq(radio, freq);
    if (tea5764_i2c_write(radio))
    PWARN("Could not set frequency!");
    }
#[no_mangle]
unsafe extern "C" fn tea5764_set_audout_mode(radio: *mut tea5764_device, audmode: c_int) {
    static void tea5764_set_audout_mode(struct tea5764_device *radio, int audmode)
    {
    struct tea5764_regs *r = &radio.regs;
    let mut tnctrl: c_int = r.tnctrl;
    if (audmode == V4L2_TUNER_MODE_MONO)
    r.tnctrl |= TEA5764_TNCTRL_MST;
    else
    r.tnctrl &= ~TEA5764_TNCTRL_MST;
    if (tnctrl != r.tnctrl)
    tea5764_i2c_write(radio);
    }
#[no_mangle]
unsafe extern "C" fn tea5764_get_audout_mode(radio: *mut tea5764_device) -> c_int {
    static int tea5764_get_audout_mode(struct tea5764_device *radio)
    {
    struct tea5764_regs *r = &radio.regs;
    if (r.tnctrl & TEA5764_TNCTRL_MST)
    return V4L2_TUNER_MODE_MONO;
    else
    return V4L2_TUNER_MODE_STEREO;
    }
#[no_mangle]
unsafe extern "C" fn tea5764_mute(radio: *mut tea5764_device, on: c_int) {
    static void tea5764_mute(struct tea5764_device *radio, int on)
    {
    struct tea5764_regs *r = &radio.regs;
    let mut tnctrl: c_int = r.tnctrl;
    if (on)
    r.tnctrl |= TEA5764_TNCTRL_MU;
    else
    r.tnctrl &= ~TEA5764_TNCTRL_MU;
    if (tnctrl != r.tnctrl)
    tea5764_i2c_write(radio);
    }
// V4L2 vidioc
    static int vidioc_querycap(struct file *file, void  *priv,
    struct v4l2_capability *v)
    {
    struct tea5764_device *radio = video_drvdata(file);
    struct video_device *dev = &radio.vdev;
    strscpy(v.driver, dev.dev.driver.name, sizeof(v.driver));
    strscpy(v.card, dev.name, sizeof(v.card));
    snprintf(v.bus_info, sizeof(v.bus_info),
    "I2C:%s", dev_name(&dev.dev));
    return 0;
    }
    static int vidioc_g_tuner(struct file *file, void *priv,
    struct v4l2_tuner *v)
    {
    struct tea5764_device *radio = video_drvdata(file);
    struct tea5764_regs *r = &radio.regs;
    if (v.index > 0)
    return -EINVAL;
    strscpy(v.name, "FM", sizeof(v.name));
    v.type = V4L2_TUNER_RADIO;
    tea5764_i2c_read(radio);
    v.rangelow   = FREQ_MIN * FREQ_MUL;
    v.rangehigh  = FREQ_MAX * FREQ_MUL;
    v.capability = V4L2_TUNER_CAP_LOW | V4L2_TUNER_CAP_STEREO;
    if (r.tunchk & TEA5764_TUNCHK_STEREO)
    v.rxsubchans = V4L2_TUNER_SUB_STEREO;
    else
    v.rxsubchans = V4L2_TUNER_SUB_MONO;
    v.audmode = tea5764_get_audout_mode(radio);
    v.signal = TEA5764_TUNCHK_LEVEL(r.tunchk) * 0xffff / 0xf;
    v.afc = TEA5764_TUNCHK_IFCNT(r.tunchk);
    return 0;
    }
    static int vidioc_s_tuner(struct file *file, void *priv,
    const struct v4l2_tuner *v)
    {
    struct tea5764_device *radio = video_drvdata(file);
    if (v.index > 0)
    return -EINVAL;
    tea5764_set_audout_mode(radio, v.audmode);
    return 0;
    }
    static int vidioc_s_frequency(struct file *file, void *priv,
    const struct v4l2_frequency *f)
    {
    struct tea5764_device *radio = video_drvdata(file);
    let mut freq: unsigned = f.frequency;
    if (f.tuner != 0 || f.type != V4L2_TUNER_RADIO)
    return -EINVAL;
    if (freq == 0) {
// We special case this as a power down control.
    tea5764_power_down(radio);
// Yes, that's what is returned in this case. This
    whole special case is non-compliant and should really
    be replaced with something better, but changing this
    might well break code that depends on this behavior.
    So we keep it as-is. */
    return -EINVAL;
    }
    freq = clamp(freq, FREQ_MIN * FREQ_MUL, FREQ_MAX * FREQ_MUL);
    tea5764_power_up(radio);
    tea5764_tune(radio, (freq * 125) / 2);
    return 0;
    }
    static int vidioc_g_frequency(struct file *file, void *priv,
    struct v4l2_frequency *f)
    {
    struct tea5764_device *radio = video_drvdata(file);
    struct tea5764_regs *r = &radio.regs;
    if (f.tuner != 0)
    return -EINVAL;
    tea5764_i2c_read(radio);
    f.type = V4L2_TUNER_RADIO;
    if (r.tnctrl & TEA5764_TNCTRL_PUPD0)
    f.frequency = (tea5764_get_freq(radio) * 2) / 125;
    else
    f.frequency = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tea5764_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int tea5764_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct tea5764_device *radio =
    container_of(ctrl.handler, struct tea5764_device, ctrl_handler);
    switch (ctrl.id) {
    case V4L2_CID_AUDIO_MUTE:
    tea5764_mute(radio, ctrl.val);
    return 0;
    }
    return -EINVAL;
    }
    static const struct v4l2_ctrl_ops tea5764_ctrl_ops = {
    .s_ctrl = tea5764_s_ctrl,
    };
// File system interface
    static const struct v4l2_file_operations tea5764_fops = {
    .owner		= THIS_MODULE,
    .open		= v4l2_fh_open,
    .release	= v4l2_fh_release,
    .poll		= v4l2_ctrl_poll,
    .unlocked_ioctl	= video_ioctl2,
    };
    static const struct v4l2_ioctl_ops tea5764_ioctl_ops = {
    .vidioc_querycap    = vidioc_querycap,
    .vidioc_g_tuner     = vidioc_g_tuner,
    .vidioc_s_tuner     = vidioc_s_tuner,
    .vidioc_g_frequency = vidioc_g_frequency,
    .vidioc_s_frequency = vidioc_s_frequency,
    .vidioc_log_status  = v4l2_ctrl_log_status,
    .vidioc_subscribe_event = v4l2_ctrl_subscribe_event,
    .vidioc_unsubscribe_event = v4l2_event_unsubscribe,
    };
// V4L2 interface
    static const struct video_device tea5764_radio_template = {
    .name		= "TEA5764 FM-Radio",
    .fops           = &tea5764_fops,
    .ioctl_ops	= &tea5764_ioctl_ops,
    .release	= video_device_release_empty,
    };
// I2C probe: check if the device exists and register with v4l if it is
#[no_mangle]
unsafe extern "C" fn tea5764_i2c_probe(client: *mut i2c_client) -> c_int {
    static int tea5764_i2c_probe(struct i2c_client *client)
    {
    struct tea5764_device *radio;
    struct v4l2_device *v4l2_dev;
    struct v4l2_ctrl_handler *hdl;
    struct tea5764_regs *r;
    int ret;
    PDEBUG("probe");
    radio = kzalloc_obj(struct tea5764_device);
    if (!radio)
    return -ENOMEM;
    v4l2_dev = &radio.v4l2_dev;
    ret = v4l2_device_register(&client.dev, v4l2_dev);
    if (ret < 0) {
    v4l2_err(v4l2_dev, "could not register v4l2_device\n");
    goto errfr;
    }
    hdl = &radio.ctrl_handler;
    v4l2_ctrl_handler_init(hdl, 1);
    v4l2_ctrl_new_std(hdl, &tea5764_ctrl_ops,
    V4L2_CID_AUDIO_MUTE, 0, 1, 1, 1);
    v4l2_dev.ctrl_handler = hdl;
    if (hdl.error) {
    ret = hdl.error;
    v4l2_err(v4l2_dev, "Could not register controls\n");
    goto errunreg;
    }
    mutex_init(&radio.mutex);
    radio.i2c_client = client;
    ret = tea5764_i2c_read(radio);
    if (ret)
    goto errunreg;
    r = &radio.regs;
    PDEBUG("chipid = %04X, manid = %04X", r.chipid, r.manid);
    if (r.chipid != TEA5764_CHIPID ||
    (r.manid & 0x0fff) != TEA5764_MANID) {
    PWARN("This chip is not a TEA5764!");
    ret = -EINVAL;
    goto errunreg;
    }
    radio.vdev = tea5764_radio_template;
    i2c_set_clientdata(client, radio);
    video_set_drvdata(&radio.vdev, radio);
    radio.vdev.lock = &radio.mutex;
    radio.vdev.v4l2_dev = v4l2_dev;
    radio.vdev.device_caps = V4L2_CAP_TUNER | V4L2_CAP_RADIO;
// initialize and power off the chip
    tea5764_i2c_read(radio);
    tea5764_set_audout_mode(radio, V4L2_TUNER_MODE_STEREO);
    tea5764_mute(radio, 1);
    tea5764_power_down(radio);
    ret = video_register_device(&radio.vdev, VFL_TYPE_RADIO, radio_nr);
    if (ret < 0) {
    PWARN("Could not register video device!");
    goto errunreg;
    }
    PINFO("registered.");
    return 0;
    errunreg:
    v4l2_ctrl_handler_free(hdl);
    v4l2_device_unregister(v4l2_dev);
    errfr:
    kfree(radio);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tea5764_i2c_remove(client: *mut i2c_client) {
    static void tea5764_i2c_remove(struct i2c_client *client)
    {
    struct tea5764_device *radio = i2c_get_clientdata(client);
    PDEBUG("remove");
    if (radio) {
    tea5764_power_down(radio);
    video_unregister_device(&radio.vdev);
    v4l2_ctrl_handler_free(&radio.ctrl_handler);
    v4l2_device_unregister(&radio.v4l2_dev);
    kfree(radio);
    }
    }
// I2C subsystem interface
    static const struct i2c_device_id tea5764_id[] = {
    { .name = "radio-tea5764" },
    { }					/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(i2c, tea5764_id);
    static struct i2c_driver tea5764_i2c_driver = {
    .driver = {
    .name = "radio-tea5764",
    },
    .probe = tea5764_i2c_probe,
    .remove = tea5764_i2c_remove,
    .id_table = tea5764_id,
    };
    module_i2c_driver(tea5764_i2c_driver);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
    MODULE_VERSION(DRIVER_VERSION);
    module_param(use_xtal, int, 0);
    MODULE_PARM_DESC(use_xtal, "Chip have a xtal connected in board");
    module_param(radio_nr, int, 0);
    MODULE_PARM_DESC(radio_nr, "video4linux device number to use");
