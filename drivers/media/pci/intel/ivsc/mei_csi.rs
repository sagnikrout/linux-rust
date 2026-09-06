//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/intel/ivsc/mei_csi.c
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
// Copyright (C) 2023 Intel Corporation. All rights reserved.
// Intel Visual Sensing Controller CSI Linux driver
//
// To set ownership of CSI-2 link and to configure CSI-2 link, there
// are specific commands, which are sent via MEI protocol. The send
// command function uses "completion" as a synchronization mechanism.
// The response for command is received via a mei callback which wakes
// up the caller. There can be only one outstanding command at a time.
//

// the 5s used here is based on experiment

// to setup CSI-2 link an extra delay needed and determined experimentally
pub const CSI_FW_READY_DELAY_MS: c_int = 100;
// link frequency unit is 100kHz

//
// identify the command id supported by firmware
// IPC, as well as the privacy notification id
// used when processing privacy event.
//
    enum csi_cmd_id {
// used to set csi ownership
    CSI_SET_OWNER = 0,
// used to configure CSI-2 link
    CSI_SET_CONF = 2,
// privacy notification id used when privacy state changes
    CSI_PRIVACY_NOTIF = 6,
    };
// CSI-2 link ownership definition
    enum csi_link_owner {
    CSI_LINK_IVSC,
    CSI_LINK_HOST,
    };
// privacy status definition
    enum ivsc_privacy_status {
    CSI_PRIVACY_OFF,
    CSI_PRIVACY_ON,
    CSI_PRIVACY_MAX,
    };
    enum csi_pads {
    CSI_PAD_SINK,
    CSI_PAD_SOURCE,
    CSI_NUM_PADS
    };
// configuration of the CSI-2 link between host and IVSC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csi_link_cfg {
// number of data lanes used on the CSI-2 link
    pub nr_of_lanes: u32,
// frequency of the CSI-2 link
    pub link_freq: u32,
// for future use
    pub rsvd: [u32; 2],
    pub __packed: },
// CSI command structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csi_cmd {
    pub cmd_id: u32,
    union _cmd_param {
    pub param: u32,
    pub conf: csi_link_cfg,
    pub param: },
    pub __packed: },
// CSI notification structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csi_notif {
    pub cmd_id: u32,
    pub status: c_int,
    union _resp_cont {
    pub cont: u32,
    pub conf: csi_link_cfg,
    pub cont: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_csi {
    pub cldev: *mut mei_cl_device,
// command response
    pub cmd_response: csi_notif,
// used to wait for command response from firmware
    pub cmd_completion: completion,
// protect command download
    pub lock: mutex,
    pub subdev: v4l2_subdev,
    pub remote: *mut media_pad,
    pub notifier: v4l2_async_notifier,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub freq_ctrl: *mut v4l2_ctrl,
    pub privacy_ctrl: *mut v4l2_ctrl,
// lock for v4l2 controls
    pub ctrl_lock: mutex,
// start streaming or not
    pub streaming: c_int,
    pub pads: [media_pad; CSI_NUM_PADS],
// number of data lanes used on the CSI-2 link
    pub nr_of_lanes: u32,
// frequency of the CSI-2 link
    pub link_freq: u64,
}

    static const struct v4l2_mbus_framefmt mei_csi_format_mbus_default = {
    .width = 1,
    .height = 1,
    .code = MEDIA_BUS_FMT_Y8_1X8,
    .field = V4L2_FIELD_NONE,
    };
    static inline struct mei_csi *notifier_to_csi(struct v4l2_async_notifier *n)
    {
    return container_of(n, struct mei_csi, notifier);
    }
    static inline struct mei_csi *sd_to_csi(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct mei_csi, subdev);
    }
// send a command to firmware and mutex must be held by caller
#[no_mangle]
unsafe extern "C" fn mei_csi_send(csi: *mut mei_csi, buf: *mut u8, len: usize) -> c_int {
    static int mei_csi_send(struct mei_csi *csi, u8 *buf, size_t len)
    {
    struct csi_cmd *cmd = (struct csi_cmd *)buf;
    int ret;
    reinit_completion(&csi.cmd_completion);
    ret = mei_cldev_send(csi.cldev, buf, len);
    if (ret < 0)
    goto out;
    ret = wait_for_completion_killable_timeout(&csi.cmd_completion,
    CSI_CMD_TIMEOUT);
    if (ret < 0) {
    goto out;
    } else if (!ret) {
    ret = -ETIMEDOUT;
    goto out;
    }
// command response status
    ret = csi.cmd_response.status;
    if (ret == -1) {
// notify privacy on instead of reporting error
    ret = 0;
    v4l2_ctrl_s_ctrl(csi.privacy_ctrl, 1);
    } else if (ret) {
    ret = -EINVAL;
    goto out;
    }
    if (csi.cmd_response.cmd_id != cmd.cmd_id)
    ret = -EINVAL;
    out:
    return ret;
    }
// set CSI-2 link ownership
#[no_mangle]
unsafe extern "C" fn csi_set_link_owner(csi: *mut mei_csi, owner: enum csi_link_owner) -> c_int {
    static int csi_set_link_owner(struct mei_csi *csi, enum csi_link_owner owner)
    {
    let mut cmd: csi_cmd = { 0 };
    size_t cmd_size;
    int ret;
    cmd.cmd_id = CSI_SET_OWNER;
    cmd.param.param = owner;
    cmd_size = sizeof(cmd.cmd_id) + sizeof(cmd.param.param);
    mutex_lock(&csi.lock);
    ret = mei_csi_send(csi, (u8 *)&cmd, cmd_size);
    mutex_unlock(&csi.lock);
    return ret;
    }
// configure CSI-2 link between host and IVSC
#[no_mangle]
unsafe extern "C" fn csi_set_link_cfg(csi: *mut mei_csi) -> c_int {
    static int csi_set_link_cfg(struct mei_csi *csi)
    {
    let mut cmd: csi_cmd = { 0 };
    size_t cmd_size;
    int ret;
    cmd.cmd_id = CSI_SET_CONF;
    cmd.param.conf.nr_of_lanes = csi.nr_of_lanes;
    cmd.param.conf.link_freq = CSI_LINK_FREQ(csi.link_freq);
    cmd_size = sizeof(cmd.cmd_id) + sizeof(cmd.param.conf);
    mutex_lock(&csi.lock);
    ret = mei_csi_send(csi, (u8 *)&cmd, cmd_size);
//
// wait configuration ready if download success. placing
// delay under mutex is to make sure current command flow
// completed before starting a possible new one.
//
    if (!ret)
    msleep(CSI_FW_READY_DELAY_MS);
    mutex_unlock(&csi.lock);
    return ret;
    }
// callback for receive
#[no_mangle]
unsafe extern "C" fn mei_csi_rx(cldev: *mut mei_cl_device) {
    static void mei_csi_rx(struct mei_cl_device *cldev)
    {
    struct mei_csi *csi = mei_cldev_get_drvdata(cldev);
    let mut notif: csi_notif = { 0 };
    int ret;
    ret = mei_cldev_recv(cldev, (u8 *)&notif, sizeof(notif));
    if (ret < 0) {
    dev_err(&cldev.dev, "recv error: %d\n", ret);
    return;
    }
    switch (notif.cmd_id) {
    case CSI_PRIVACY_NOTIF:
    if (notif.cont.cont < CSI_PRIVACY_MAX)
    v4l2_ctrl_s_ctrl(csi.privacy_ctrl,
    notif.cont.cont == CSI_PRIVACY_ON);
    break;
    case CSI_SET_OWNER:
    case CSI_SET_CONF:
    memcpy(&csi.cmd_response, &notif, ret);
    complete(&csi.cmd_completion);
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mei_csi_set_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int mei_csi_set_stream(struct v4l2_subdev *sd, int enable)
    {
    struct mei_csi *csi = sd_to_csi(sd);
    struct v4l2_subdev *remote_sd =
    media_entity_to_v4l2_subdev(csi.remote.entity);
    s64 freq;
    int ret;
    if (enable && csi.streaming == 0) {
    freq = v4l2_get_link_freq(csi.remote, 0, 0);
    if (freq < 0) {
    dev_err(&csi.cldev.dev,
    "error %lld, invalid link_freq\n", freq);
    ret = freq;
    goto err;
    }
    csi.link_freq = freq;
// switch CSI-2 link to host
    ret = csi_set_link_owner(csi, CSI_LINK_HOST);
    if (ret < 0)
    goto err;
// configure CSI-2 link
    ret = csi_set_link_cfg(csi);
    if (ret < 0)
    goto err_switch;
    ret = v4l2_subdev_call(remote_sd, video, s_stream, 1);
    if (ret)
    goto err_switch;
    } else if (!enable && csi.streaming == 1) {
    v4l2_subdev_call(remote_sd, video, s_stream, 0);
// switch CSI-2 link to IVSC
    ret = csi_set_link_owner(csi, CSI_LINK_IVSC);
    if (ret < 0)
    dev_warn(&csi.cldev.dev,
    "failed to switch CSI2 link: %d\n", ret);
    }
    csi.streaming = enable;
    return 0;
    err_switch:
    csi_set_link_owner(csi, CSI_LINK_IVSC);
    err:
    return ret;
    }
    static int mei_csi_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state)
    {
    struct v4l2_mbus_framefmt *mbusformat;
    unsigned int i;
    for (i = 0; i < sd.entity.num_pads; i++) {
    mbusformat = v4l2_subdev_state_get_format(sd_state, i);
// mbusformat = mei_csi_format_mbus_default;
    }
    return 0;
    }
    static int mei_csi_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *source_fmt;
    struct v4l2_mbus_framefmt *sink_fmt;
    sink_fmt = v4l2_subdev_state_get_format(sd_state, CSI_PAD_SINK);
    source_fmt = v4l2_subdev_state_get_format(sd_state, CSI_PAD_SOURCE);
    if (format.pad) {
// source_fmt = *sink_fmt;
    return 0;
    }
    v4l_bound_align_image(&format.format.width, 1, 65536, 0,
    &format.format.height, 1, 65536, 0, 0);
    switch (format.format.code) {
    case MEDIA_BUS_FMT_RGB444_1X12:
    case MEDIA_BUS_FMT_RGB444_2X8_PADHI_BE:
    case MEDIA_BUS_FMT_RGB444_2X8_PADHI_LE:
    case MEDIA_BUS_FMT_RGB555_2X8_PADHI_BE:
    case MEDIA_BUS_FMT_RGB555_2X8_PADHI_LE:
    case MEDIA_BUS_FMT_RGB565_1X16:
    case MEDIA_BUS_FMT_BGR565_2X8_BE:
    case MEDIA_BUS_FMT_BGR565_2X8_LE:
    case MEDIA_BUS_FMT_RGB565_2X8_BE:
    case MEDIA_BUS_FMT_RGB565_2X8_LE:
    case MEDIA_BUS_FMT_RGB666_1X18:
    case MEDIA_BUS_FMT_RBG888_1X24:
    case MEDIA_BUS_FMT_RGB666_1X24_CPADHI:
    case MEDIA_BUS_FMT_BGR888_1X24:
    case MEDIA_BUS_FMT_GBR888_1X24:
    case MEDIA_BUS_FMT_RGB888_1X24:
    case MEDIA_BUS_FMT_RGB888_2X12_BE:
    case MEDIA_BUS_FMT_RGB888_2X12_LE:
    case MEDIA_BUS_FMT_ARGB8888_1X32:
    case MEDIA_BUS_FMT_RGB888_1X32_PADHI:
    case MEDIA_BUS_FMT_RGB101010_1X30:
    case MEDIA_BUS_FMT_RGB121212_1X36:
    case MEDIA_BUS_FMT_RGB161616_1X48:
    case MEDIA_BUS_FMT_Y8_1X8:
    case MEDIA_BUS_FMT_UV8_1X8:
    case MEDIA_BUS_FMT_UYVY8_1_5X8:
    case MEDIA_BUS_FMT_VYUY8_1_5X8:
    case MEDIA_BUS_FMT_YUYV8_1_5X8:
    case MEDIA_BUS_FMT_YVYU8_1_5X8:
    case MEDIA_BUS_FMT_UYVY8_2X8:
    case MEDIA_BUS_FMT_VYUY8_2X8:
    case MEDIA_BUS_FMT_YUYV8_2X8:
    case MEDIA_BUS_FMT_YVYU8_2X8:
    case MEDIA_BUS_FMT_Y10_1X10:
    case MEDIA_BUS_FMT_UYVY10_2X10:
    case MEDIA_BUS_FMT_VYUY10_2X10:
    case MEDIA_BUS_FMT_YUYV10_2X10:
    case MEDIA_BUS_FMT_YVYU10_2X10:
    case MEDIA_BUS_FMT_Y12_1X12:
    case MEDIA_BUS_FMT_UYVY12_2X12:
    case MEDIA_BUS_FMT_VYUY12_2X12:
    case MEDIA_BUS_FMT_YUYV12_2X12:
    case MEDIA_BUS_FMT_YVYU12_2X12:
    case MEDIA_BUS_FMT_UYVY8_1X16:
    case MEDIA_BUS_FMT_VYUY8_1X16:
    case MEDIA_BUS_FMT_YUYV8_1X16:
    case MEDIA_BUS_FMT_YVYU8_1X16:
    case MEDIA_BUS_FMT_YDYUYDYV8_1X16:
    case MEDIA_BUS_FMT_UYVY10_1X20:
    case MEDIA_BUS_FMT_VYUY10_1X20:
    case MEDIA_BUS_FMT_YUYV10_1X20:
    case MEDIA_BUS_FMT_YVYU10_1X20:
    case MEDIA_BUS_FMT_VUY8_1X24:
    case MEDIA_BUS_FMT_YUV8_1X24:
    case MEDIA_BUS_FMT_UYYVYY8_0_5X24:
    case MEDIA_BUS_FMT_UYVY12_1X24:
    case MEDIA_BUS_FMT_VYUY12_1X24:
    case MEDIA_BUS_FMT_YUYV12_1X24:
    case MEDIA_BUS_FMT_YVYU12_1X24:
    case MEDIA_BUS_FMT_YUV10_1X30:
    case MEDIA_BUS_FMT_UYYVYY10_0_5X30:
    case MEDIA_BUS_FMT_AYUV8_1X32:
    case MEDIA_BUS_FMT_UYYVYY12_0_5X36:
    case MEDIA_BUS_FMT_YUV12_1X36:
    case MEDIA_BUS_FMT_YUV16_1X48:
    case MEDIA_BUS_FMT_UYYVYY16_0_5X48:
    case MEDIA_BUS_FMT_JPEG_1X8:
    case MEDIA_BUS_FMT_AHSV8888_1X32:
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    case MEDIA_BUS_FMT_SGBRG8_1X8:
    case MEDIA_BUS_FMT_SGRBG8_1X8:
    case MEDIA_BUS_FMT_SRGGB8_1X8:
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    case MEDIA_BUS_FMT_SGBRG10_1X10:
    case MEDIA_BUS_FMT_SGRBG10_1X10:
    case MEDIA_BUS_FMT_SRGGB10_1X10:
    case MEDIA_BUS_FMT_SBGGR12_1X12:
    case MEDIA_BUS_FMT_SGBRG12_1X12:
    case MEDIA_BUS_FMT_SGRBG12_1X12:
    case MEDIA_BUS_FMT_SRGGB12_1X12:
    case MEDIA_BUS_FMT_SBGGR14_1X14:
    case MEDIA_BUS_FMT_SGBRG14_1X14:
    case MEDIA_BUS_FMT_SGRBG14_1X14:
    case MEDIA_BUS_FMT_SRGGB14_1X14:
    case MEDIA_BUS_FMT_SBGGR16_1X16:
    case MEDIA_BUS_FMT_SGBRG16_1X16:
    case MEDIA_BUS_FMT_SGRBG16_1X16:
    case MEDIA_BUS_FMT_SRGGB16_1X16:
    break;
    default:
    format.format.code = MEDIA_BUS_FMT_Y8_1X8;
    break;
    }
    if (format.format.field == V4L2_FIELD_ANY)
    format.format.field = V4L2_FIELD_NONE;
// sink_fmt = format->format;
// source_fmt = *sink_fmt;
    return 0;
    }
    static int mei_csi_get_mbus_config(struct v4l2_subdev *sd, unsigned int pad,
    struct v4l2_mbus_config *mbus_config)
    {
    struct mei_csi *csi = sd_to_csi(sd);
    unsigned int i;
    s64 freq;
    mbus_config.type = V4L2_MBUS_CSI2_DPHY;
    for (i = 0; i < V4L2_MBUS_CSI2_MAX_DATA_LANES; i++)
    mbus_config.bus.mipi_csi2.data_lanes[i] = i + 1;
    mbus_config.bus.mipi_csi2.num_data_lanes = csi.nr_of_lanes;
    freq = v4l2_get_link_freq(csi.remote, 0, 0);
    if (freq < 0) {
    dev_err(&csi.cldev.dev,
    "error %lld, invalid link_freq\n", freq);
    return -EINVAL;
    }
    csi.link_freq = freq;
    mbus_config.link_freq = freq;
    return 0;
    }
    static const struct v4l2_subdev_video_ops mei_csi_video_ops = {
    .s_stream = mei_csi_set_stream,
    };
    static const struct v4l2_subdev_pad_ops mei_csi_pad_ops = {
    .get_fmt = v4l2_subdev_get_fmt,
    .set_fmt = mei_csi_set_fmt,
    .get_mbus_config = mei_csi_get_mbus_config,
    };
    static const struct v4l2_subdev_ops mei_csi_subdev_ops = {
    .video = &mei_csi_video_ops,
    .pad = &mei_csi_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops mei_csi_internal_ops = {
    .init_state = mei_csi_init_state,
    };
    static const struct media_entity_operations mei_csi_entity_ops = {
    .link_validate = v4l2_subdev_link_validate,
    };
    static int mei_csi_notify_bound(struct v4l2_async_notifier *notifier,
    struct v4l2_subdev *subdev,
    struct v4l2_async_connection *asd)
    {
    struct mei_csi *csi = notifier_to_csi(notifier);
    int pad;
    pad = media_entity_get_fwnode_pad(&subdev.entity, asd.match.fwnode,
    MEDIA_PAD_FL_SOURCE);
    if (pad < 0)
    return pad;
    csi.remote = &subdev.entity.pads[pad];
    return media_create_pad_link(&subdev.entity, pad,
    &csi.subdev.entity, CSI_PAD_SINK,
    MEDIA_LNK_FL_ENABLED |
    MEDIA_LNK_FL_IMMUTABLE);
    }
    static void mei_csi_notify_unbind(struct v4l2_async_notifier *notifier,
    struct v4l2_subdev *subdev,
    struct v4l2_async_connection *asd)
    {
    struct mei_csi *csi = notifier_to_csi(notifier);
    csi.remote = core::ptr::null_mut();
    }
    static const struct v4l2_async_notifier_operations mei_csi_notify_ops = {
    .bound = mei_csi_notify_bound,
    .unbind = mei_csi_notify_unbind,
    };
#[no_mangle]
unsafe extern "C" fn mei_csi_init_controls(csi: *mut mei_csi) -> c_int {
    static int mei_csi_init_controls(struct mei_csi *csi)
    {
    int ret;
    mutex_init(&csi.ctrl_lock);
    ret = v4l2_ctrl_handler_init(&csi.ctrl_handler, 1);
    if (ret)
    return ret;
    csi.ctrl_handler.lock = &csi.ctrl_lock;
    csi.privacy_ctrl = v4l2_ctrl_new_std(&csi.ctrl_handler, core::ptr::null_mut(),
    V4L2_CID_PRIVACY, 0, 1, 1, 0);
    if (csi.privacy_ctrl)
    csi.privacy_ctrl.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    if (csi.ctrl_handler.error)
    return csi.ctrl_handler.error;
    csi.subdev.ctrl_handler = &csi.ctrl_handler;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mei_csi_parse_firmware(csi: *mut mei_csi) -> c_int {
    static int mei_csi_parse_firmware(struct mei_csi *csi)
    {
    struct v4l2_fwnode_endpoint v4l2_ep = {
    .bus_type = V4L2_MBUS_CSI2_DPHY,
    };
    struct device *dev = &csi.cldev.dev;
    struct v4l2_async_connection *asd;
    struct fwnode_handle *sink_ep, *source_ep;
    int ret;
    sink_ep = fwnode_graph_get_endpoint_by_id(dev_fwnode(dev), 0, 0, 0);
    if (!sink_ep) {
    dev_err(dev, "can't obtain sink endpoint\n");
    return -EINVAL;
    }
    v4l2_async_subdev_nf_init(&csi.notifier, &csi.subdev);
    csi.notifier.ops = &mei_csi_notify_ops;
    ret = v4l2_fwnode_endpoint_parse(sink_ep, &v4l2_ep);
    if (ret) {
    dev_err(dev, "could not parse v4l2 sink endpoint\n");
    goto out_nf_cleanup;
    }
    csi.nr_of_lanes = v4l2_ep.bus.mipi_csi2.num_data_lanes;
    source_ep = fwnode_graph_get_endpoint_by_id(dev_fwnode(dev), 1, 0, 0);
    if (!source_ep) {
    ret = -ENOTCONN;
    dev_err(dev, "can't obtain source endpoint\n");
    goto out_nf_cleanup;
    }
    ret = v4l2_fwnode_endpoint_parse(source_ep, &v4l2_ep);
    fwnode_handle_put(source_ep);
    if (ret) {
    dev_err(dev, "could not parse v4l2 source endpoint\n");
    goto out_nf_cleanup;
    }
    if (csi.nr_of_lanes != v4l2_ep.bus.mipi_csi2.num_data_lanes) {
    ret = -EINVAL;
    dev_err(dev,
    "the number of lanes does not match (%u vs. %u)\n",
    csi.nr_of_lanes, v4l2_ep.bus.mipi_csi2.num_data_lanes);
    goto out_nf_cleanup;
    }
    asd = v4l2_async_nf_add_fwnode_remote(&csi.notifier, sink_ep,
    struct v4l2_async_connection);
    if (IS_ERR(asd)) {
    ret = PTR_ERR(asd);
    goto out_nf_cleanup;
    }
    ret = v4l2_async_nf_register(&csi.notifier);
    if (ret)
    goto out_nf_cleanup;
    fwnode_handle_put(sink_ep);
    return 0;
    out_nf_cleanup:
    v4l2_async_nf_cleanup(&csi.notifier);
    fwnode_handle_put(sink_ep);
    return ret;
    }
    static int mei_csi_probe(struct mei_cl_device *cldev,
    const struct mei_cl_device_id *id)
    {
    struct device *dev = &cldev.dev;
    struct pci_dev *ipu;
    struct mei_csi *csi;
    unsigned int i;
    int ret;
    for (i = 0, ipu = core::ptr::null_mut(); !ipu && ipu6_pci_tbl[i].vendor; i++)
    ipu = pci_get_device(ipu6_pci_tbl[i].vendor,
    ipu6_pci_tbl[i].device, core::ptr::null_mut());
    if (!ipu)
    return -ENODEV;
    ret = ipu_bridge_init(&ipu.dev, ipu_bridge_parse_ssdb);
    put_device(&ipu.dev);
    if (ret < 0)
    return ret;
    if (!dev_fwnode(dev)) {
    dev_err(dev, "mei-csi probed without device fwnode!\n");
    return -ENXIO;
    }
    csi = devm_kzalloc(dev, sizeof(struct mei_csi), GFP_KERNEL);
    if (!csi)
    return -ENOMEM;
    csi.cldev = cldev;
    mutex_init(&csi.lock);
    init_completion(&csi.cmd_completion);
    mei_cldev_set_drvdata(cldev, csi);
    ret = mei_cldev_enable(cldev);
    if (ret < 0) {
    dev_err(dev, "mei_cldev_enable failed: %d\n", ret);
    goto destroy_mutex;
    }
    ret = mei_cldev_register_rx_cb(cldev, mei_csi_rx);
    if (ret) {
    dev_err(dev, "event cb registration failed: %d\n", ret);
    goto err_disable;
    }
    ret = mei_csi_parse_firmware(csi);
    if (ret)
    goto err_disable;
    csi.subdev.dev = &cldev.dev;
    csi.subdev.state_lock = &csi.lock;
    v4l2_subdev_init(&csi.subdev, &mei_csi_subdev_ops);
    csi.subdev.internal_ops = &mei_csi_internal_ops;
    v4l2_set_subdevdata(&csi.subdev, csi);
    csi.subdev.flags = V4L2_SUBDEV_FL_HAS_DEVNODE |
    V4L2_SUBDEV_FL_HAS_EVENTS;
    csi.subdev.entity.function = MEDIA_ENT_F_VID_IF_BRIDGE;
    csi.subdev.entity.ops = &mei_csi_entity_ops;
    snprintf(csi.subdev.name, sizeof(csi.subdev.name),
    MEI_CSI_ENTITY_NAME);
    ret = mei_csi_init_controls(csi);
    if (ret)
    goto err_ctrl_handler;
    csi.pads[CSI_PAD_SOURCE].flags = MEDIA_PAD_FL_SOURCE;
    csi.pads[CSI_PAD_SINK].flags = MEDIA_PAD_FL_SINK;
    ret = media_entity_pads_init(&csi.subdev.entity, CSI_NUM_PADS,
    csi.pads);
    if (ret)
    goto err_ctrl_handler;
    ret = v4l2_subdev_init_finalize(&csi.subdev);
    if (ret < 0)
    goto err_entity;
    ret = v4l2_async_register_subdev(&csi.subdev);
    if (ret < 0)
    goto err_subdev;
    pm_runtime_enable(&cldev.dev);
    return 0;
    err_subdev:
    v4l2_subdev_cleanup(&csi.subdev);
    err_entity:
    media_entity_cleanup(&csi.subdev.entity);
    err_ctrl_handler:
    v4l2_ctrl_handler_free(&csi.ctrl_handler);
    mutex_destroy(&csi.ctrl_lock);
    v4l2_async_nf_unregister(&csi.notifier);
    v4l2_async_nf_cleanup(&csi.notifier);
    err_disable:
    mei_cldev_disable(cldev);
    destroy_mutex:
    mutex_destroy(&csi.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mei_csi_remove(cldev: *mut mei_cl_device) {
    static void mei_csi_remove(struct mei_cl_device *cldev)
    {
    struct mei_csi *csi = mei_cldev_get_drvdata(cldev);
    v4l2_async_nf_unregister(&csi.notifier);
    v4l2_async_nf_cleanup(&csi.notifier);
    v4l2_ctrl_handler_free(&csi.ctrl_handler);
    mutex_destroy(&csi.ctrl_lock);
    v4l2_async_unregister_subdev(&csi.subdev);
    v4l2_subdev_cleanup(&csi.subdev);
    media_entity_cleanup(&csi.subdev.entity);
    pm_runtime_disable(&cldev.dev);
    mei_cldev_disable(cldev);
    mutex_destroy(&csi.lock);
    }

    0xAF, 0x93, 0x7b, 0x44, 0x53, 0xAC, 0x29, 0xDA)
    static const struct mei_cl_device_id mei_csi_tbl[] = {
    { .uuid = MEI_CSI_UUID, .version = MEI_CL_VERSION_ANY },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(mei, mei_csi_tbl);
    static struct mei_cl_driver mei_csi_driver = {
    .id_table = mei_csi_tbl,
    .name = KBUILD_MODNAME,
    .probe = mei_csi_probe,
    .remove = mei_csi_remove,
    };
    module_mei_cl_driver(mei_csi_driver);
    MODULE_IMPORT_NS("INTEL_IPU_BRIDGE");
    MODULE_AUTHOR("Wentong Wu");
    MODULE_AUTHOR("Zhifeng Wang <zhifeng.wang@intel.com>");
    MODULE_DESCRIPTION("Device driver for IVSC CSI");
    MODULE_LICENSE("GPL");
