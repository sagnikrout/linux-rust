//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/dsbr100.c
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
// A driver for the D-Link DSB-R100 USB radio and Gemtek USB Radio 21.
// The device plugs into both the USB and an analog audio input, so this thing
// only deals with initialisation and frequency setting, the
// audio data has to be handled by a sound driver.
//
// Major issue: I can't find out where the device reports the signal
// strength, and indeed the windows software appearantly just looks
// at the stereo indicator as well.  So, scanning will only find
// stereo stations.  Sad, but I can't help it.
//
// Also, the windows program sends oodles of messages over to the
// device, and I couldn't figure out their meaning.  My suspicion
// is that they don't have any:-)
//
// You might find some interesting stuff about this module at
// http://unimut.fsk.uni-heidelberg.de/unimut/demi/dsbr
//
// Fully tested with the Keene USB FM Transmitter and the v4l2-compliance tool.
//
// Copyright (c) 2000 Markus Demleitner <msdemlei@cl.uni-heidelberg.de>
//

//
// Version Information
//
    MODULE_AUTHOR("Markus Demleitner <msdemlei@tucana.harvard.edu>");
    MODULE_DESCRIPTION("D-Link DSB-R100 USB FM radio driver");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("1.1.0");
pub const DSB100_VENDOR: c_uint = 0x04b4;
pub const DSB100_PRODUCT: c_uint = 0x1002;
// Commands the device appears to understand
pub const DSB100_TUNE: c_int = 1;
pub const DSB100_ONOFF: c_int = 2;
pub const TB_LEN: c_int = 16;
// Frequency limits in MHz -- these are European values.  For Japanese
    devices, that would be 76 and 91.  */

pub const FREQ_MUL: c_int = 16000;

    let mut radio_nr: static int = -1;
    module_param(radio_nr, int, 0);
// Data for one (physical) device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsbr100_device {
    pub usbdev: *mut usb_device,
    pub videodev: video_device,
    pub v4l2_dev: v4l2_device,
    pub hdl: v4l2_ctrl_handler,
    pub transfer_buffer: *mut u8,
    pub v4l2_lock: mutex,
    pub curfreq: c_int,
    pub stereo: bool,
    pub muted: bool,
}

// Low-level device interface begins here
// set a frequency, freq is defined by v4l's TUNER_LOW, i.e. 1/16th kHz
#[no_mangle]
unsafe extern "C" fn dsbr100_setfreq(radio: *mut dsbr100_device, freq: unsigned) -> c_int {
    static int dsbr100_setfreq(struct dsbr100_device *radio, unsigned freq)
    {
    let mut f: unsigned = (freq / 16 * 80) / 1000 + 856;
    let mut retval: c_int = 0;
    if (!radio.muted) {
    retval = usb_control_msg(radio.usbdev,
    usb_rcvctrlpipe(radio.usbdev, 0),
    DSB100_TUNE,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    (f >> 8) & 0x00ff, f & 0xff,
    radio.transfer_buffer, 8, 300);
    if (retval >= 0)
    mdelay(1);
    }
    if (retval >= 0) {
    radio.curfreq = freq;
    return 0;
    }
    dev_err(&radio.usbdev.dev,
    "%s - usb_control_msg returned %i, request %i\n",
    __func__, retval, DSB100_TUNE);
    return retval;
    }
// switch on radio
#[no_mangle]
unsafe extern "C" fn dsbr100_start(radio: *mut dsbr100_device) -> c_int {
    static int dsbr100_start(struct dsbr100_device *radio)
    {
    int retval = usb_control_msg(radio.usbdev,
    usb_rcvctrlpipe(radio.usbdev, 0),
    DSB100_ONOFF,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    0x01, 0x00, radio.transfer_buffer, 8, 300);
    if (retval >= 0)
    return dsbr100_setfreq(radio, radio.curfreq);
    dev_err(&radio.usbdev.dev,
    "%s - usb_control_msg returned %i, request %i\n",
    __func__, retval, DSB100_ONOFF);
    return retval;
    }
// switch off radio
#[no_mangle]
unsafe extern "C" fn dsbr100_stop(radio: *mut dsbr100_device) -> c_int {
    static int dsbr100_stop(struct dsbr100_device *radio)
    {
    int retval = usb_control_msg(radio.usbdev,
    usb_rcvctrlpipe(radio.usbdev, 0),
    DSB100_ONOFF,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    0x00, 0x00, radio.transfer_buffer, 8, 300);
    if (retval >= 0)
    return 0;
    dev_err(&radio.usbdev.dev,
    "%s - usb_control_msg returned %i, request %i\n",
    __func__, retval, DSB100_ONOFF);
    return retval;
    }
// return the device status.  This is, in effect, just whether it
    sees a stereo signal or not.  Pity. */
#[no_mangle]
unsafe extern "C" fn dsbr100_getstat(radio: *mut dsbr100_device) {
    static void dsbr100_getstat(struct dsbr100_device *radio)
    {
    int retval = usb_control_msg(radio.usbdev,
    usb_rcvctrlpipe(radio.usbdev, 0),
    USB_REQ_GET_STATUS,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    0x00, 0x24, radio.transfer_buffer, 8, 300);
    if (retval < 0) {
    radio.stereo = false;
    dev_err(&radio.usbdev.dev,
    "%s - usb_control_msg returned %i, request %i\n",
    __func__, retval, USB_REQ_GET_STATUS);
    } else {
    radio.stereo = !(radio.transfer_buffer[0] & 0x01);
    }
    }
    static int vidioc_querycap(struct file *file, void *priv,
    struct v4l2_capability *v)
    {
    struct dsbr100_device *radio = video_drvdata(file);
    strscpy(v.driver, "dsbr100", sizeof(v.driver));
    strscpy(v.card, "D-Link R-100 USB FM Radio", sizeof(v.card));
    usb_make_path(radio.usbdev, v.bus_info, sizeof(v.bus_info));
    return 0;
    }
    static int vidioc_g_tuner(struct file *file, void *priv,
    struct v4l2_tuner *v)
    {
    struct dsbr100_device *radio = video_drvdata(file);
    if (v.index > 0)
    return -EINVAL;
    dsbr100_getstat(radio);
    strscpy(v.name, "FM", sizeof(v.name));
    v.type = V4L2_TUNER_RADIO;
    v.rangelow = FREQ_MIN * FREQ_MUL;
    v.rangehigh = FREQ_MAX * FREQ_MUL;
    v.rxsubchans = radio.stereo ? V4L2_TUNER_SUB_STEREO :
    V4L2_TUNER_SUB_MONO;
    v.capability = V4L2_TUNER_CAP_LOW | V4L2_TUNER_CAP_STEREO;
    v.audmode = V4L2_TUNER_MODE_STEREO;
    v.signal = radio.stereo ? 0xffff : 0;     /* We can't get the signal strength */
    return 0;
    }
    static int vidioc_s_tuner(struct file *file, void *priv,
    const struct v4l2_tuner *v)
    {
    return v.index ? -EINVAL : 0;
    }
    static int vidioc_s_frequency(struct file *file, void *priv,
    const struct v4l2_frequency *f)
    {
    struct dsbr100_device *radio = video_drvdata(file);
    if (f.tuner != 0 || f.type != V4L2_TUNER_RADIO)
    return -EINVAL;
    return dsbr100_setfreq(radio, clamp_t(unsigned, f.frequency,
    FREQ_MIN * FREQ_MUL, FREQ_MAX * FREQ_MUL));
    }
    static int vidioc_g_frequency(struct file *file, void *priv,
    struct v4l2_frequency *f)
    {
    struct dsbr100_device *radio = video_drvdata(file);
    if (f.tuner)
    return -EINVAL;
    f.type = V4L2_TUNER_RADIO;
    f.frequency = radio.curfreq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_dsbr100_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int usb_dsbr100_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct dsbr100_device *radio =
    container_of(ctrl.handler, struct dsbr100_device, hdl);
    switch (ctrl.id) {
    case V4L2_CID_AUDIO_MUTE:
    radio.muted = ctrl.val;
    return radio.muted ? dsbr100_stop(radio) : dsbr100_start(radio);
    }
    return -EINVAL;
    }
// USB subsystem interface begins here
//
// Handle unplugging of the device.
// We call video_unregister_device in any case.
// The last function called in this procedure is
// usb_dsbr100_video_device_release
//
#[no_mangle]
unsafe extern "C" fn usb_dsbr100_disconnect(intf: *mut usb_interface) {
    static void usb_dsbr100_disconnect(struct usb_interface *intf)
    {
    struct dsbr100_device *radio = usb_get_intfdata(intf);
    mutex_lock(&radio.v4l2_lock);
//
// Disconnect is also called on unload, and in that case we need to
// mute the device. This call will silently fail if it is called
// after a physical disconnect.
//
    usb_control_msg(radio.usbdev,
    usb_rcvctrlpipe(radio.usbdev, 0),
    DSB100_ONOFF,
    USB_TYPE_VENDOR | USB_RECIP_DEVICE | USB_DIR_IN,
    0x00, 0x00, radio.transfer_buffer, 8, 300);
    usb_set_intfdata(intf, core::ptr::null_mut());
    video_unregister_device(&radio.videodev);
    v4l2_device_disconnect(&radio.v4l2_dev);
    mutex_unlock(&radio.v4l2_lock);
    v4l2_device_put(&radio.v4l2_dev);
    }
// Suspend device - stop device.
#[no_mangle]
unsafe extern "C" fn usb_dsbr100_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int {
    static int usb_dsbr100_suspend(struct usb_interface *intf, pm_message_t message)
    {
    struct dsbr100_device *radio = usb_get_intfdata(intf);
    mutex_lock(&radio.v4l2_lock);
    if (!radio.muted && dsbr100_stop(radio) < 0)
    dev_warn(&intf.dev, "dsbr100_stop failed\n");
    mutex_unlock(&radio.v4l2_lock);
    dev_info(&intf.dev, "going into suspend..\n");
    return 0;
    }
// Resume device - start device.
#[no_mangle]
unsafe extern "C" fn usb_dsbr100_resume(intf: *mut usb_interface) -> c_int {
    static int usb_dsbr100_resume(struct usb_interface *intf)
    {
    struct dsbr100_device *radio = usb_get_intfdata(intf);
    mutex_lock(&radio.v4l2_lock);
    if (!radio.muted && dsbr100_start(radio) < 0)
    dev_warn(&intf.dev, "dsbr100_start failed\n");
    mutex_unlock(&radio.v4l2_lock);
    dev_info(&intf.dev, "coming out of suspend..\n");
    return 0;
    }
// free data structures
#[no_mangle]
unsafe extern "C" fn usb_dsbr100_release(v4l2_dev: *mut v4l2_device) {
    static void usb_dsbr100_release(struct v4l2_device *v4l2_dev)
    {
    struct dsbr100_device *radio = v4l2_dev_to_radio(v4l2_dev);
    v4l2_ctrl_handler_free(&radio.hdl);
    v4l2_device_unregister(&radio.v4l2_dev);
    kfree(radio.transfer_buffer);
    kfree(radio);
    }
    static const struct v4l2_ctrl_ops usb_dsbr100_ctrl_ops = {
    .s_ctrl = usb_dsbr100_s_ctrl,
    };
// File system interface
    static const struct v4l2_file_operations usb_dsbr100_fops = {
    .owner		= THIS_MODULE,
    .unlocked_ioctl	= video_ioctl2,
    .open           = v4l2_fh_open,
    .release        = v4l2_fh_release,
    .poll		= v4l2_ctrl_poll,
    };
    static const struct v4l2_ioctl_ops usb_dsbr100_ioctl_ops = {
    .vidioc_querycap    = vidioc_querycap,
    .vidioc_g_tuner     = vidioc_g_tuner,
    .vidioc_s_tuner     = vidioc_s_tuner,
    .vidioc_g_frequency = vidioc_g_frequency,
    .vidioc_s_frequency = vidioc_s_frequency,
    .vidioc_log_status  = v4l2_ctrl_log_status,
    .vidioc_subscribe_event = v4l2_ctrl_subscribe_event,
    .vidioc_unsubscribe_event = v4l2_event_unsubscribe,
    };
// check if the device is present and register with v4l and usb if it is
    static int usb_dsbr100_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct dsbr100_device *radio;
    struct v4l2_device *v4l2_dev;
    int retval;
    radio = kzalloc_obj(struct dsbr100_device);
    if (!radio)
    return -ENOMEM;
    radio.transfer_buffer = kmalloc(TB_LEN, GFP_KERNEL);
    if (!(radio.transfer_buffer)) {
    kfree(radio);
    return -ENOMEM;
    }
    v4l2_dev = &radio.v4l2_dev;
    v4l2_dev.release = usb_dsbr100_release;
    retval = v4l2_device_register(&intf.dev, v4l2_dev);
    if (retval < 0) {
    v4l2_err(v4l2_dev, "couldn't register v4l2_device\n");
    goto err_reg_dev;
    }
    v4l2_ctrl_handler_init(&radio.hdl, 1);
    v4l2_ctrl_new_std(&radio.hdl, &usb_dsbr100_ctrl_ops,
    V4L2_CID_AUDIO_MUTE, 0, 1, 1, 1);
    if (radio.hdl.error) {
    retval = radio.hdl.error;
    v4l2_err(v4l2_dev, "couldn't register control\n");
    goto err_reg_ctrl;
    }
    mutex_init(&radio.v4l2_lock);
    strscpy(radio.videodev.name, v4l2_dev.name,
    sizeof(radio.videodev.name));
    radio.videodev.v4l2_dev = v4l2_dev;
    radio.videodev.fops = &usb_dsbr100_fops;
    radio.videodev.ioctl_ops = &usb_dsbr100_ioctl_ops;
    radio.videodev.release = video_device_release_empty;
    radio.videodev.lock = &radio.v4l2_lock;
    radio.videodev.ctrl_handler = &radio.hdl;
    radio.videodev.device_caps = V4L2_CAP_RADIO | V4L2_CAP_TUNER;
    radio.usbdev = interface_to_usbdev(intf);
    radio.curfreq = FREQ_MIN * FREQ_MUL;
    radio.muted = true;
    video_set_drvdata(&radio.videodev, radio);
    usb_set_intfdata(intf, radio);
    retval = video_register_device(&radio.videodev, VFL_TYPE_RADIO, radio_nr);
    if (retval == 0)
    return 0;
    v4l2_err(v4l2_dev, "couldn't register video device\n");
    err_reg_ctrl:
    v4l2_ctrl_handler_free(&radio.hdl);
    v4l2_device_unregister(v4l2_dev);
    err_reg_dev:
    kfree(radio.transfer_buffer);
    kfree(radio);
    return retval;
    }
    static const struct usb_device_id usb_dsbr100_device_table[] = {
    { USB_DEVICE(DSB100_VENDOR, DSB100_PRODUCT) },
    { }						/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, usb_dsbr100_device_table);
// USB subsystem interface
    static struct usb_driver usb_dsbr100_driver = {
    .name			= "dsbr100",
    .probe			= usb_dsbr100_probe,
    .disconnect		= usb_dsbr100_disconnect,
    .id_table		= usb_dsbr100_device_table,
    .suspend		= usb_dsbr100_suspend,
    .resume			= usb_dsbr100_resume,
    .reset_resume		= usb_dsbr100_resume,
    };
    module_usb_driver(usb_dsbr100_driver);
