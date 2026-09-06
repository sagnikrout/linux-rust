//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/rdacm21.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// IMI RDACM21 GMSL Camera Driver
//
// Copyright (C) 2017-2020 Jacopo Mondi
// Copyright (C) 2017-2019 Kieran Bingham
// Copyright (C) 2017-2019 Laurent Pinchart
// Copyright (C) 2017-2019 Niklas Söderlund
// Copyright (C) 2016 Renesas Electronics Corporation
// Copyright (C) 2015 Cogent Embedded, Inc.
//

pub const MAX9271_RESET_CYCLES: c_int = 10;
pub const OV490_I2C_ADDRESS: c_uint = 0x24;
pub const OV490_PAGE_HIGH_REG: c_uint = 0xfffd;
pub const OV490_PAGE_LOW_REG: c_uint = 0xfffe;
//
// The SCCB slave handling is undocumented; the registers naming scheme is
// totally arbitrary.
//
pub const OV490_SCCB_SLAVE_WRITE: c_uint = 0x00;
pub const OV490_SCCB_SLAVE_READ: c_uint = 0x01;
pub const OV490_SCCB_SLAVE0_DIR: c_uint = 0x80195000;
pub const OV490_SCCB_SLAVE0_ADDR_HIGH: c_uint = 0x80195001;
pub const OV490_SCCB_SLAVE0_ADDR_LOW: c_uint = 0x80195002;
pub const OV490_DVP_CTRL3: c_uint = 0x80286009;
pub const OV490_ODS_CTRL_FRAME_OUTPUT_EN: c_uint = 0x0c;
pub const OV490_ODS_CTRL: c_uint = 0x8029d000;
pub const OV490_HOST_CMD: c_uint = 0x808000c0;
pub const OV490_HOST_CMD_TRIGGER: c_uint = 0xc1;
pub const OV490_ID_VAL: c_uint = 0x0490;

pub const OV490_PID: c_uint = 0x8080300a;
pub const OV490_VER: c_uint = 0x8080300b;
pub const OV490_PID_TIMEOUT: c_int = 20;
pub const OV490_OUTPUT_EN_TIMEOUT: c_int = 300;

pub const OV490_GPIO_SEL0: c_uint = 0x80800050;
pub const OV490_GPIO_SEL1: c_uint = 0x80800051;
pub const OV490_GPIO_DIRECTION0: c_uint = 0x80800054;
pub const OV490_GPIO_DIRECTION1: c_uint = 0x80800055;
pub const OV490_GPIO_OUTPUT_VALUE0: c_uint = 0x80800058;
pub const OV490_GPIO_OUTPUT_VALUE1: c_uint = 0x80800059;
pub const OV490_ISP_HSIZE_LOW: c_uint = 0x80820060;
pub const OV490_ISP_HSIZE_HIGH: c_uint = 0x80820061;
pub const OV490_ISP_VSIZE_LOW: c_uint = 0x80820062;
pub const OV490_ISP_VSIZE_HIGH: c_uint = 0x80820063;
pub const OV10640_PID_TIMEOUT: c_int = 20;
pub const OV10640_ID_HIGH: c_uint = 0xa6;
pub const OV10640_CHIP_ID: c_uint = 0x300a;
pub const OV10640_PIXEL_RATE: c_int = 55000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdacm21_device {
    pub dev: *mut device,
    pub serializer: max9271_device,
    pub isp: *mut i2c_client,
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub fmt: v4l2_mbus_framefmt,
    pub ctrls: v4l2_ctrl_handler,
    pub addrs: [u32; 2],
    pub last_page: u16,
}

    static inline struct rdacm21_device *sd_to_rdacm21(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct rdacm21_device, sd);
    }
    static const struct ov490_reg {
    u16 reg;
    u8 val;
    } ov490_regs_wizard[] = {
    {0xfffd, 0x80},
    {0xfffe, 0x82},
    {0x0071, 0x11},
    {0x0075, 0x11},
    {0xfffe, 0x29},
    {0x6010, 0x01},
//
// OV490 EMB line disable in YUV and RAW data,
// NOTE: EMB line is still used in ISP and sensor
//
    {0xe000, 0x14},
    {0xfffe, 0x28},
    {0x6000, 0x04},
    {0x6004, 0x00},
//
// PCLK polarity - useless due to silicon bug.
// Use 0x808000bb register instead.
//
    {0x6008, 0x00},
    {0xfffe, 0x80},
    {0x0091, 0x00},
// bit[3]=0 - PCLK polarity workaround.
    {0x00bb, 0x1d},
// Ov490 FSIN: app_fsin_from_fsync
    {0xfffe, 0x85},
    {0x0008, 0x00},
    {0x0009, 0x01},
// FSIN0 source.
    {0x000A, 0x05},
    {0x000B, 0x00},
// FSIN0 delay.
    {0x0030, 0x02},
    {0x0031, 0x00},
    {0x0032, 0x00},
    {0x0033, 0x00},
// FSIN1 delay.
    {0x0038, 0x02},
    {0x0039, 0x00},
    {0x003A, 0x00},
    {0x003B, 0x00},
// FSIN0 length.
    {0x0070, 0x2C},
    {0x0071, 0x01},
    {0x0072, 0x00},
    {0x0073, 0x00},
// FSIN1 length.
    {0x0074, 0x64},
    {0x0075, 0x00},
    {0x0076, 0x00},
    {0x0077, 0x00},
    {0x0000, 0x14},
    {0x0001, 0x00},
    {0x0002, 0x00},
    {0x0003, 0x00},
//
// Load fsin0,load fsin1,load other,
// It will be cleared automatically.
//
    {0x0004, 0x32},
    {0x0005, 0x00},
    {0x0006, 0x00},
    {0x0007, 0x00},
    {0xfffe, 0x80},
// Sensor FSIN.
    {0x0081, 0x00},
// ov10640 FSIN enable
    {0xfffe, 0x19},
    {0x5000, 0x00},
    {0x5001, 0x30},
    {0x5002, 0x8c},
    {0x5003, 0xb2},
    {0xfffe, 0x80},
    {0x00c0, 0xc1},
// ov10640 HFLIP=1 by default
    {0xfffe, 0x19},
    {0x5000, 0x01},
    {0x5001, 0x00},
    {0xfffe, 0x80},
    {0x00c0, 0xdc},
    };
#[no_mangle]
unsafe extern "C" fn ov490_read(dev: *mut rdacm21_device, reg: u16, val: *mut u8) -> c_int {
    static int ov490_read(struct rdacm21_device *dev, u16 reg, u8 *val)
    {
    u8 buf[2] = { reg >> 8, reg };
    int ret;
    ret = i2c_master_send(dev.isp, buf, 2);
    if (ret == 2)
    ret = i2c_master_recv(dev.isp, val, 1);
    if (ret < 0) {
    dev_dbg(dev.dev, "%s: register 0x%04x read failed (%d)\n",
    __func__, reg, ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov490_write(dev: *mut rdacm21_device, reg: u16, val: u8) -> c_int {
    static int ov490_write(struct rdacm21_device *dev, u16 reg, u8 val)
    {
    u8 buf[3] = { reg >> 8, reg, val };
    int ret;
    ret = i2c_master_send(dev.isp, buf, 3);
    if (ret < 0) {
    dev_err(dev.dev, "%s: register 0x%04x write failed (%d)\n",
    __func__, reg, ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov490_set_page(dev: *mut rdacm21_device, page: u16) -> c_int {
    static int ov490_set_page(struct rdacm21_device *dev, u16 page)
    {
    let mut page_high: u8 = page >> 8;
    let mut page_low: u8 = page;
    int ret;
    if (page == dev.last_page)
    return 0;
    if (page_high != (dev.last_page >> 8)) {
    ret = ov490_write(dev, OV490_PAGE_HIGH_REG, page_high);
    if (ret)
    return ret;
    }
    if (page_low != (u8)dev.last_page) {
    ret = ov490_write(dev, OV490_PAGE_LOW_REG, page_low);
    if (ret)
    return ret;
    }
    dev.last_page = page;
    usleep_range(100, 150);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov490_read_reg(dev: *mut rdacm21_device, reg: u32, val: *mut u8) -> c_int {
    static int ov490_read_reg(struct rdacm21_device *dev, u32 reg, u8 *val)
    {
    int ret;
    ret = ov490_set_page(dev, reg >> 16);
    if (ret)
    return ret;
    ret = ov490_read(dev, (u16)reg, val);
    if (ret)
    return ret;
    dev_dbg(dev.dev, "%s: 0x%08x = 0x%02x\n", __func__, reg, *val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov490_write_reg(dev: *mut rdacm21_device, reg: u32, val: u8) -> c_int {
    static int ov490_write_reg(struct rdacm21_device *dev, u32 reg, u8 val)
    {
    int ret;
    ret = ov490_set_page(dev, reg >> 16);
    if (ret)
    return ret;
    ret = ov490_write(dev, (u16)reg, val);
    if (ret)
    return ret;
    dev_dbg(dev.dev, "%s: 0x%08x = 0x%02x\n", __func__, reg, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rdacm21_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int rdacm21_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct rdacm21_device *dev = sd_to_rdacm21(sd);
//
// Enable serial link now that the ISP provides a valid pixel clock
// to start serializing video data on the GMSL link.
//
    return max9271_set_serial_link(&dev.serializer, enable);
    }
    static int rdacm21_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index > 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_YUYV8_1X16;
    return 0;
    }
    static int rdacm21_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *mf = &format.format;
    struct rdacm21_device *dev = sd_to_rdacm21(sd);
    if (format.pad)
    return -EINVAL;
    mf.width		= dev.fmt.width;
    mf.height		= dev.fmt.height;
    mf.code		= MEDIA_BUS_FMT_YUYV8_1X16;
    mf.colorspace		= V4L2_COLORSPACE_SRGB;
    mf.field		= V4L2_FIELD_NONE;
    mf.ycbcr_enc		= V4L2_YCBCR_ENC_601;
    mf.quantization	= V4L2_QUANTIZATION_FULL_RANGE;
    mf.xfer_func		= V4L2_XFER_FUNC_NONE;
    return 0;
    }
    static const struct v4l2_subdev_video_ops rdacm21_video_ops = {
    .s_stream	= rdacm21_s_stream,
    };
    static const struct v4l2_subdev_pad_ops rdacm21_subdev_pad_ops = {
    .enum_mbus_code = rdacm21_enum_mbus_code,
    .get_fmt	= rdacm21_get_fmt,
    .set_fmt	= rdacm21_get_fmt,
    };
    static const struct v4l2_subdev_ops rdacm21_subdev_ops = {
    .video		= &rdacm21_video_ops,
    .pad		= &rdacm21_subdev_pad_ops,
    };
#[no_mangle]
unsafe extern "C" fn ov10640_power_up(dev: *mut rdacm21_device) {
    static void ov10640_power_up(struct rdacm21_device *dev)
    {
// Enable GPIO0#0 (reset) and GPIO1#0 (pwdn) as output lines.
    ov490_write_reg(dev, OV490_GPIO_SEL0, OV490_GPIO0);
    ov490_write_reg(dev, OV490_GPIO_SEL1, OV490_SPWDN0);
    ov490_write_reg(dev, OV490_GPIO_DIRECTION0, OV490_GPIO0);
    ov490_write_reg(dev, OV490_GPIO_DIRECTION1, OV490_SPWDN0);
// Power up OV10640 and then reset it.
    ov490_write_reg(dev, OV490_GPIO_OUTPUT_VALUE1, OV490_SPWDN0);
    usleep_range(1500, 3000);
    ov490_write_reg(dev, OV490_GPIO_OUTPUT_VALUE0, 0x00);
    usleep_range(1500, 3000);
    ov490_write_reg(dev, OV490_GPIO_OUTPUT_VALUE0, OV490_GPIO0);
    usleep_range(3000, 5000);
    }
#[no_mangle]
unsafe extern "C" fn ov10640_check_id(dev: *mut rdacm21_device) -> c_int {
    static int ov10640_check_id(struct rdacm21_device *dev)
    {
    unsigned int i;
    let mut val: u8 = 0;
// Read OV10640 ID to test communications.
    for (i = 0; i < OV10640_PID_TIMEOUT; ++i) {
    ov490_write_reg(dev, OV490_SCCB_SLAVE0_DIR,
    OV490_SCCB_SLAVE_READ);
    ov490_write_reg(dev, OV490_SCCB_SLAVE0_ADDR_HIGH,
    OV10640_CHIP_ID >> 8);
    ov490_write_reg(dev, OV490_SCCB_SLAVE0_ADDR_LOW,
    OV10640_CHIP_ID & 0xff);
//
// Trigger SCCB slave transaction and give it some time
// to complete.
//
    ov490_write_reg(dev, OV490_HOST_CMD, OV490_HOST_CMD_TRIGGER);
    usleep_range(1000, 1500);
    ov490_read_reg(dev, OV490_SCCB_SLAVE0_DIR, &val);
    if (val == OV10640_ID_HIGH)
    break;
    usleep_range(1000, 1500);
    }
    if (i == OV10640_PID_TIMEOUT) {
    dev_err(dev.dev, "OV10640 ID mismatch: (0x%02x)\n", val);
    return -ENODEV;
    }
    dev_dbg(dev.dev, "OV10640 ID = 0x%2x\n", val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov490_initialize(dev: *mut rdacm21_device) -> c_int {
    static int ov490_initialize(struct rdacm21_device *dev)
    {
    u8 pid, ver, val;
    unsigned int i;
    int ret;
    ov10640_power_up(dev);
//
// Read OV490 Id to test communications. Give it up to 40msec to
// exit from reset.
//
    for (i = 0; i < OV490_PID_TIMEOUT; ++i) {
    ret = ov490_read_reg(dev, OV490_PID, &pid);
    if (ret == 0)
    break;
    usleep_range(1000, 2000);
    }
    if (i == OV490_PID_TIMEOUT) {
    dev_err(dev.dev, "OV490 PID read failed (%d)\n", ret);
    return ret;
    }
    ret = ov490_read_reg(dev, OV490_VER, &ver);
    if (ret < 0)
    return ret;
    if (OV490_ID(pid, ver) != OV490_ID_VAL) {
    dev_err(dev.dev, "OV490 ID mismatch (0x%04x)\n",
    OV490_ID(pid, ver));
    return -ENODEV;
    }
// Wait for firmware boot by reading streamon status.
    for (i = 0; i < OV490_OUTPUT_EN_TIMEOUT; ++i) {
    ov490_read_reg(dev, OV490_ODS_CTRL, &val);
    if (val == OV490_ODS_CTRL_FRAME_OUTPUT_EN)
    break;
    usleep_range(1000, 2000);
    }
    if (i == OV490_OUTPUT_EN_TIMEOUT) {
    dev_err(dev.dev, "Timeout waiting for firmware boot\n");
    return -ENODEV;
    }
    ret = ov10640_check_id(dev);
    if (ret)
    return ret;
// Program OV490 with register-value table.
    for (i = 0; i < ARRAY_SIZE(ov490_regs_wizard); ++i) {
    ret = ov490_write(dev, ov490_regs_wizard[i].reg,
    ov490_regs_wizard[i].val);
    if (ret < 0) {
    dev_err(dev.dev,
    "%s: register %u (0x%04x) write failed (%d)\n",
    __func__, i, ov490_regs_wizard[i].reg, ret);
    return -EIO;
    }
    usleep_range(100, 150);
    }
//
// The ISP is programmed with the content of a serial flash memory.
// Read the firmware configuration to reflect it through the V4L2 APIs.
//
    ov490_read_reg(dev, OV490_ISP_HSIZE_HIGH, &val);
    dev.fmt.width = (val & 0xf) << 8;
    ov490_read_reg(dev, OV490_ISP_HSIZE_LOW, &val);
    dev.fmt.width |= (val & 0xff);
    ov490_read_reg(dev, OV490_ISP_VSIZE_HIGH, &val);
    dev.fmt.height = (val & 0xf) << 8;
    ov490_read_reg(dev, OV490_ISP_VSIZE_LOW, &val);
    dev.fmt.height |= val & 0xff;
// Set bus width to 12 bits with [0:11] ordering.
    ov490_write_reg(dev, OV490_DVP_CTRL3, 0x10);
    dev_info(dev.dev, "Identified RDACM21 camera module\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rdacm21_initialize(dev: *mut rdacm21_device) -> c_int {
    static int rdacm21_initialize(struct rdacm21_device *dev)
    {
    int ret;
    max9271_wake_up(&dev.serializer);
// Enable reverse channel and disable the serial link.
    ret = max9271_set_serial_link(&dev.serializer, false);
    if (ret)
    return ret;
// Configure I2C bus at 105Kbps speed and configure GMSL.
    ret = max9271_configure_i2c(&dev.serializer,
    MAX9271_I2CSLVSH_469NS_234NS |
    MAX9271_I2CSLVTO_1024US |
    MAX9271_I2CMSTBT_105KBPS);
    if (ret)
    return ret;
    ret = max9271_verify_id(&dev.serializer);
    if (ret)
    return ret;
//
// Enable GPIO1 and hold OV490 in reset during max9271 configuration.
// The reset signal has to be asserted for at least 250 useconds.
//
    ret = max9271_enable_gpios(&dev.serializer, MAX9271_GPIO1OUT);
    if (ret)
    return ret;
    ret = max9271_clear_gpios(&dev.serializer, MAX9271_GPIO1OUT);
    if (ret)
    return ret;
    usleep_range(250, 500);
    ret = max9271_configure_gmsl_link(&dev.serializer);
    if (ret)
    return ret;
    ret = max9271_set_address(&dev.serializer, dev.addrs[0]);
    if (ret)
    return ret;
    dev.serializer.client.addr = dev.addrs[0];
    ret = max9271_set_translation(&dev.serializer, dev.addrs[1],
    OV490_I2C_ADDRESS);
    if (ret)
    return ret;
    dev.isp.addr = dev.addrs[1];
// Release OV490 from reset and initialize it.
    ret = max9271_set_gpios(&dev.serializer, MAX9271_GPIO1OUT);
    if (ret)
    return ret;
    usleep_range(3000, 5000);
    ret = ov490_initialize(dev);
    if (ret)
    return ret;
//
// Set reverse channel high threshold to increase noise immunity.
//
// This should be compensated by increasing the reverse channel
// amplitude on the remote deserializer side.
//
    return max9271_set_high_threshold(&dev.serializer, true);
    }
#[no_mangle]
unsafe extern "C" fn rdacm21_probe(client: *mut i2c_client) -> c_int {
    static int rdacm21_probe(struct i2c_client *client)
    {
    struct rdacm21_device *dev;
    int ret;
    dev = devm_kzalloc(&client.dev, sizeof(*dev), GFP_KERNEL);
    if (!dev)
    return -ENOMEM;
    dev.dev = &client.dev;
    dev.serializer.client = client;
    ret = device_property_read_u32_array(dev.dev, "reg", dev.addrs, 2);
    if (ret < 0) {
    dev_err(dev.dev, "Invalid FW reg property: %d\n", ret);
    return -EINVAL;
    }
// Create the dummy I2C client for the sensor.
    dev.isp = i2c_new_dummy_device(client.adapter, OV490_I2C_ADDRESS);
    if (IS_ERR(dev.isp))
    return PTR_ERR(dev.isp);
    ret = rdacm21_initialize(dev);
    if (ret < 0)
    goto error;
// Initialize and register the subdevice.
    v4l2_i2c_subdev_init(&dev.sd, client, &rdacm21_subdev_ops);
    dev.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    v4l2_ctrl_handler_init(&dev.ctrls, 1);
    v4l2_ctrl_new_std(&dev.ctrls, core::ptr::null_mut(), V4L2_CID_PIXEL_RATE,
    OV10640_PIXEL_RATE, OV10640_PIXEL_RATE, 1,
    OV10640_PIXEL_RATE);
    dev.sd.ctrl_handler = &dev.ctrls;
    ret = dev.ctrls.error;
    if (ret)
    goto error_free_ctrls;
    dev.pad.flags = MEDIA_PAD_FL_SOURCE;
    dev.sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&dev.sd.entity, 1, &dev.pad);
    if (ret < 0)
    goto error_free_ctrls;
    ret = v4l2_async_register_subdev(&dev.sd);
    if (ret)
    goto error_entity_cleanup;
    return 0;
    error_entity_cleanup:
    media_entity_cleanup(&dev.sd.entity);
    error_free_ctrls:
    v4l2_ctrl_handler_free(&dev.ctrls);
    error:
    i2c_unregister_device(dev.isp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rdacm21_remove(client: *mut i2c_client) {
    static void rdacm21_remove(struct i2c_client *client)
    {
    struct rdacm21_device *dev = sd_to_rdacm21(i2c_get_clientdata(client));
    v4l2_async_unregister_subdev(&dev.sd);
    v4l2_ctrl_handler_free(&dev.ctrls);
    media_entity_cleanup(&dev.sd.entity);
    i2c_unregister_device(dev.isp);
    }
    static const struct of_device_id rdacm21_of_ids[] = {
    { .compatible = "imi,rdacm21" },
    { }
    };
    MODULE_DEVICE_TABLE(of, rdacm21_of_ids);
    static struct i2c_driver rdacm21_i2c_driver = {
    .driver	= {
    .name	= "rdacm21",
    .of_match_table = rdacm21_of_ids,
    },
    .probe		= rdacm21_probe,
    .remove		= rdacm21_remove,
    };
    module_i2c_driver(rdacm21_i2c_driver);
    MODULE_DESCRIPTION("GMSL Camera driver for RDACM21");
    MODULE_AUTHOR("Jacopo Mondi");
    MODULE_LICENSE("GPL v2");
