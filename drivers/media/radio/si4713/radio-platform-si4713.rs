//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/si4713/radio-platform-si4713.c
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
// drivers/media/radio/radio-si4713.c
//
// Platform Driver for Silicon Labs Si4713 FM Radio Transmitter:
//
// Copyright (c) 2008 Instituto Nokia de Tecnologia - INdT
// Contact: Eduardo Valentin <eduardo.valentin@nokia.com>
//

// module parameters
    static int radio_nr = -1;	/* radio device minor (-1 ==> auto assign) */
    module_param(radio_nr, int, 0);
    MODULE_PARM_DESC(radio_nr,
    "Minor number for radio device (-1 ==> auto assign)");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Eduardo Valentin <eduardo.valentin@nokia.com>");
    MODULE_DESCRIPTION("Platform driver for Si4713 FM Radio Transmitter");
    MODULE_VERSION("0.0.1");
    MODULE_ALIAS("platform:radio-si4713");
// Driver state struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radio_si4713_device {
    pub v4l2_dev: v4l2_device,
    pub radio_dev: video_device,
    pub lock: mutex,
}

// radio_si4713_fops - file operations interface
    static const struct v4l2_file_operations radio_si4713_fops = {
    .owner		= THIS_MODULE,
    .open = v4l2_fh_open,
    .release = v4l2_fh_release,
    .poll = v4l2_ctrl_poll,
// Note: locking is done at the subdev level in the i2c driver.
    .unlocked_ioctl	= video_ioctl2,
    };
// Video4Linux Interface
// radio_si4713_querycap - query device capabilities
    static int radio_si4713_querycap(struct file *file, void *priv,
    struct v4l2_capability *capability)
    {
    strscpy(capability.driver, "radio-si4713", sizeof(capability.driver));
    strscpy(capability.card, "Silicon Labs Si4713 Modulator",
    sizeof(capability.card));
    strscpy(capability.bus_info, "platform:radio-si4713",
    sizeof(capability.bus_info));
    return 0;
    }
//
// v4l2 ioctl call backs.
// we are just a wrapper for v4l2_sub_devs.
//
    static inline struct v4l2_device *get_v4l2_dev(struct file *file)
    {
    return &((struct radio_si4713_device *)video_drvdata(file)).v4l2_dev;
    }
    static int radio_si4713_g_modulator(struct file *file, void *priv,
    struct v4l2_modulator *vm)
    {
    return v4l2_device_call_until_err(get_v4l2_dev(file), 0, tuner,
    g_modulator, vm);
    }
    static int radio_si4713_s_modulator(struct file *file, void *priv,
    const struct v4l2_modulator *vm)
    {
    return v4l2_device_call_until_err(get_v4l2_dev(file), 0, tuner,
    s_modulator, vm);
    }
    static int radio_si4713_g_frequency(struct file *file, void *priv,
    struct v4l2_frequency *vf)
    {
    return v4l2_device_call_until_err(get_v4l2_dev(file), 0, tuner,
    g_frequency, vf);
    }
    static int radio_si4713_s_frequency(struct file *file, void *priv,
    const struct v4l2_frequency *vf)
    {
    return v4l2_device_call_until_err(get_v4l2_dev(file), 0, tuner,
    s_frequency, vf);
    }
    static long radio_si4713_default(struct file *file, void *priv,
    bool valid_prio, unsigned int cmd, void *arg)
    {
    return v4l2_device_call_until_err(get_v4l2_dev(file), 0, core,
    ioctl, cmd, arg);
    }
    static const struct v4l2_ioctl_ops radio_si4713_ioctl_ops = {
    .vidioc_querycap	= radio_si4713_querycap,
    .vidioc_g_modulator	= radio_si4713_g_modulator,
    .vidioc_s_modulator	= radio_si4713_s_modulator,
    .vidioc_g_frequency	= radio_si4713_g_frequency,
    .vidioc_s_frequency	= radio_si4713_s_frequency,
    .vidioc_log_status      = v4l2_ctrl_log_status,
    .vidioc_subscribe_event = v4l2_ctrl_subscribe_event,
    .vidioc_unsubscribe_event = v4l2_event_unsubscribe,
    .vidioc_default		= radio_si4713_default,
    };
// radio_si4713_vdev_template - video device interface
    static const struct video_device radio_si4713_vdev_template = {
    .fops			= &radio_si4713_fops,
    .name			= "radio-si4713",
    .release		= video_device_release_empty,
    .ioctl_ops		= &radio_si4713_ioctl_ops,
    .vfl_dir		= VFL_DIR_TX,
    };
// Platform driver interface
// radio_si4713_pdriver_probe - probe for the device
#[no_mangle]
unsafe extern "C" fn radio_si4713_pdriver_probe(pdev: *mut platform_device) -> c_int {
    static int radio_si4713_pdriver_probe(struct platform_device *pdev)
    {
    struct radio_si4713_platform_data *pdata = pdev.dev.platform_data;
    struct radio_si4713_device *rsdev;
    struct v4l2_subdev *sd;
    let mut rval: c_int = 0;
    if (!pdata) {
    dev_err(&pdev.dev, "Cannot proceed without platform data.\n");
    rval = -EINVAL;
    goto exit;
    }
    rsdev = devm_kzalloc(&pdev.dev, sizeof(*rsdev), GFP_KERNEL);
    if (!rsdev) {
    dev_err(&pdev.dev, "Failed to alloc video device.\n");
    rval = -ENOMEM;
    goto exit;
    }
    mutex_init(&rsdev.lock);
    rval = v4l2_device_register(&pdev.dev, &rsdev.v4l2_dev);
    if (rval) {
    dev_err(&pdev.dev, "Failed to register v4l2 device.\n");
    goto exit;
    }
    sd = i2c_get_clientdata(pdata.subdev);
    rval = v4l2_device_register_subdev(&rsdev.v4l2_dev, sd);
    if (rval) {
    dev_err(&pdev.dev, "Cannot get v4l2 subdevice\n");
    goto unregister_v4l2_dev;
    }
    rsdev.radio_dev = radio_si4713_vdev_template;
    rsdev.radio_dev.v4l2_dev = &rsdev.v4l2_dev;
    rsdev.radio_dev.ctrl_handler = sd.ctrl_handler;
// Serialize all access to the si4713
    rsdev.radio_dev.lock = &rsdev.lock;
    rsdev.radio_dev.device_caps = V4L2_CAP_MODULATOR | V4L2_CAP_RDS_OUTPUT;
    video_set_drvdata(&rsdev.radio_dev, rsdev);
    if (video_register_device(&rsdev.radio_dev, VFL_TYPE_RADIO, radio_nr)) {
    dev_err(&pdev.dev, "Could not register video device.\n");
    rval = -EIO;
    goto unregister_v4l2_dev;
    }
    dev_info(&pdev.dev, "New device successfully probed\n");
    goto exit;
    unregister_v4l2_dev:
    v4l2_device_unregister(&rsdev.v4l2_dev);
    exit:
    return rval;
    }
// radio_si4713_pdriver_remove - remove the device
#[no_mangle]
unsafe extern "C" fn radio_si4713_pdriver_remove(pdev: *mut platform_device) {
    static void radio_si4713_pdriver_remove(struct platform_device *pdev)
    {
    struct v4l2_device *v4l2_dev = platform_get_drvdata(pdev);
    struct radio_si4713_device *rsdev;
    rsdev = container_of(v4l2_dev, struct radio_si4713_device, v4l2_dev);
    video_unregister_device(&rsdev.radio_dev);
    v4l2_device_unregister(&rsdev.v4l2_dev);
    }
    static struct platform_driver radio_si4713_pdriver = {
    .driver		= {
    .name	= "radio-si4713",
    },
    .probe		= radio_si4713_pdriver_probe,
    .remove         = radio_si4713_pdriver_remove,
    };
    module_platform_driver(radio_si4713_pdriver);
