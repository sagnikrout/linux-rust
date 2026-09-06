//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/tc358743.c
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
// tc358743 - Toshiba HDMI to CSI-2 bridge
//
// Copyright 2015 Cisco Systems, Inc. and/or its affiliates. All rights
// reserved.
//
// References (c = chapter, p = page):
// REF_01 - Toshiba, TC358743XBG (H2C), Functional Specification, Rev 0.60
// REF_02 - Toshiba, TC358743XBG_HDMI-CSI_Tv11p_nm.xls
//

    static int debug;
    module_param(debug, int, 0644);
    MODULE_PARM_DESC(debug, " debug level (0-3)");
    let mut packet_type: static int = 0x87;
    module_param(packet_type, int, 0644);
    MODULE_PARM_DESC(packet_type,
    " Programmable Packet Type. Possible values:\n"
    "\t\t    0x87: DRM InfoFrame (Default).\n"
    "\t\t    0x01: Audio Clock Regeneration Packet\n"
    "\t\t    0x02: Audio Sample Packet\n"
    "\t\t    0x03: General Control Packet\n"
    "\t\t    0x04: ACP Packet\n"
    "\t\t    0x07: One Bit Audio Sample Packet\n"
    "\t\t    0x08: DST Audio Packet\n"
    "\t\t    0x09: High Bitrate Audio Stream Packet\n"
    "\t\t    0x0a: Gamut Metadata Packet\n");
    MODULE_DESCRIPTION("Toshiba TC358743 HDMI to CSI-2 bridge driver");
    MODULE_AUTHOR("Ramakrishnan Muthukrishnan <ram@rkrishnan.org>");
    MODULE_AUTHOR("Mikhail Khelik <mkhelik@cisco.com>");
    MODULE_AUTHOR("Mats Randgaard <matrandg@cisco.com>");
    MODULE_LICENSE("GPL");
pub const EDID_NUM_BLOCKS_MAX: c_int = 8;
pub const EDID_BLOCK_SIZE: c_int = 128;

pub const POLL_INTERVAL_CEC_MS: c_int = 10;
pub const POLL_INTERVAL_MS: c_int = 1000;
    static const struct v4l2_dv_timings_cap tc358743_timings_cap = {
    .type = V4L2_DV_BT_656_1120,
// keep this initialization for compatibility with GCC < 4.4.6
    .reserved = { 0 },
// Pixel clock from REF_01 p. 20. Min/max height/width are unknown
    V4L2_INIT_BT_TIMINGS(640, 1920, 350, 1200, 13000000, 165000000,
    V4L2_DV_BT_STD_CEA861 | V4L2_DV_BT_STD_DMT |
    V4L2_DV_BT_STD_GTF | V4L2_DV_BT_STD_CVT,
    V4L2_DV_BT_CAP_PROGRESSIVE |
    V4L2_DV_BT_CAP_REDUCED_BLANKING |
    V4L2_DV_BT_CAP_CUSTOM)
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc358743_state {
    pub pdata: tc358743_platform_data,
    pub bus: v4l2_mbus_config_mipi_csi2,
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub hdl: v4l2_ctrl_handler,
    pub i2c_client: *mut i2c_client,
// CONFCTL is modified in ops and tc358743_hdmi_sys_int_handler
    pub confctl_mutex: mutex,
// controls
    pub detect_tx_5v_ctrl: *mut v4l2_ctrl,
    pub audio_sampling_rate_ctrl: *mut v4l2_ctrl,
    pub audio_present_ctrl: *mut v4l2_ctrl,
    pub delayed_work_enable_hotplug: delayed_work,
    pub timer: timer_list,
    pub work_i2c_poll: work_struct,
// debugfs
    pub debugfs_dir: *mut dentry,
    pub infoframes: *mut v4l2_debugfs_if,
// edid
    pub edid_blocks_written: u8,
    pub timings: v4l2_dv_timings,
    pub mbus_fmt_code: u32,
    pub csi_lanes_in_use: u8,
    pub reset_gpio: *mut gpio_desc,
    pub cec_adap: *mut cec_adapter,
}

    static void tc358743_enable_interrupts(struct v4l2_subdev *sd,
    bool cable_connected);
    static int tc358743_s_ctrl_detect_tx_5v(struct v4l2_subdev *sd);
    static inline struct tc358743_state *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct tc358743_state, sd);
    }
// --------------- I2C ---------------
#[no_mangle]
unsafe extern "C" fn i2c_rd(sd: *mut v4l2_subdev, reg: u16, values: *mut u8, n: u32) -> c_int {
    static int i2c_rd(struct v4l2_subdev *sd, u16 reg, u8 *values, u32 n)
    {
    struct tc358743_state *state = to_state(sd);
    struct i2c_client *client = state.i2c_client;
    int err;
    u8 buf[2] = { reg >> 8, reg & 0xff };
    struct i2c_msg msgs[] = {
    {
    .addr = client.addr,
    .flags = 0,
    .len = 2,
    .buf = buf,
    },
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = n,
    .buf = values,
    },
    };
    err = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (err != ARRAY_SIZE(msgs)) {
    v4l2_err(sd, "%s: reading register 0x%x from 0x%x failed: %d\n",
    __func__, reg, client.addr, err);
    }
    return err != ARRAY_SIZE(msgs);
    }
#[no_mangle]
unsafe extern "C" fn i2c_wr(sd: *mut v4l2_subdev, reg: u16, values: *mut u8, n: u32) {
    static void i2c_wr(struct v4l2_subdev *sd, u16 reg, u8 *values, u32 n)
    {
    struct tc358743_state *state = to_state(sd);
    struct i2c_client *client = state.i2c_client;
    int err, i;
    struct i2c_msg msg;
    u8 data[I2C_MAX_XFER_SIZE];
    if ((2 + n) > I2C_MAX_XFER_SIZE) {
    n = I2C_MAX_XFER_SIZE - 2;
    v4l2_warn(sd, "i2c wr reg=%04x: len=%d is too big!\n",
    reg, 2 + n);
    }
    msg.addr = client.addr;
    msg.buf = data;
    msg.len = 2 + n;
    msg.flags = 0;
    data[0] = reg >> 8;
    data[1] = reg & 0xff;
    for (i = 0; i < n; i++)
    data[2 + i] = values[i];
    err = i2c_transfer(client.adapter, &msg, 1);
    if (err != 1) {
    v4l2_err(sd, "%s: writing register 0x%x from 0x%x failed: %d\n",
    __func__, reg, client.addr, err);
    return;
    }
    if (debug < 3)
    return;
    switch (n) {
    case 1:
    v4l2_info(sd, "I2C write 0x%04x = 0x%02x",
    reg, data[2]);
    break;
    case 2:
    v4l2_info(sd, "I2C write 0x%04x = 0x%02x%02x",
    reg, data[3], data[2]);
    break;
    case 4:
    v4l2_info(sd, "I2C write 0x%04x = 0x%02x%02x%02x%02x",
    reg, data[5], data[4], data[3], data[2]);
    break;
    default:
    v4l2_info(sd, "I2C write %d bytes from address 0x%04x\n",
    n, reg);
    }
    }
    static noinline u32 i2c_rdreg_err(struct v4l2_subdev *sd, u16 reg, u32 n,
    int *err)
    {
    int error;
    let mut val: __le32 = 0;
    error = i2c_rd(sd, reg, (u8  *)&val, n);
    if (err)
// err = error;
    return le32_to_cpu(val);
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_rdreg(sd: *mut v4l2_subdev, reg: u16, n: u32) -> u32 {
    static inline u32 i2c_rdreg(struct v4l2_subdev *sd, u16 reg, u32 n)
    {
    return i2c_rdreg_err(sd, reg, n, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn i2c_wrreg(sd: *mut v4l2_subdev, reg: u16, val: u32, n: u32) -> noinline void {
    static noinline void i2c_wrreg(struct v4l2_subdev *sd, u16 reg, u32 val, u32 n)
    {
    let mut raw: __le32 = cpu_to_le32(val);
    i2c_wr(sd, reg, (u8  *)&raw, n);
    }
#[no_mangle]
unsafe extern "C" fn i2c_rd8(sd: *mut v4l2_subdev, reg: u16) -> u8 {
    static u8 i2c_rd8(struct v4l2_subdev *sd, u16 reg)
    {
    return i2c_rdreg(sd, reg, 1);
    }
#[no_mangle]
unsafe extern "C" fn i2c_wr8(sd: *mut v4l2_subdev, reg: u16, val: u8) {
    static void i2c_wr8(struct v4l2_subdev *sd, u16 reg, u8 val)
    {
    i2c_wrreg(sd, reg, val, 1);
    }
    static void i2c_wr8_and_or(struct v4l2_subdev *sd, u16 reg,
    u8 mask, u8 val)
    {
    i2c_wrreg(sd, reg, (i2c_rdreg(sd, reg, 1) & mask) | val, 1);
    }
#[no_mangle]
unsafe extern "C" fn i2c_rd16(sd: *mut v4l2_subdev, reg: u16) -> u16 {
    static u16 i2c_rd16(struct v4l2_subdev *sd, u16 reg)
    {
    return i2c_rdreg(sd, reg, 2);
    }
#[no_mangle]
unsafe extern "C" fn i2c_rd16_err(sd: *mut v4l2_subdev, reg: u16, value: *mut u16) -> c_int {
    static int i2c_rd16_err(struct v4l2_subdev *sd, u16 reg, u16 *value)
    {
    int err;
// value = i2c_rdreg_err(sd, reg, 2, &err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn i2c_wr16(sd: *mut v4l2_subdev, reg: u16, val: u16) {
    static void i2c_wr16(struct v4l2_subdev *sd, u16 reg, u16 val)
    {
    i2c_wrreg(sd, reg, val, 2);
    }
#[no_mangle]
unsafe extern "C" fn i2c_wr16_and_or(sd: *mut v4l2_subdev, reg: u16, mask: u16, val: u16) {
    static void i2c_wr16_and_or(struct v4l2_subdev *sd, u16 reg, u16 mask, u16 val)
    {
    i2c_wrreg(sd, reg, (i2c_rdreg(sd, reg, 2) & mask) | val, 2);
    }
#[no_mangle]
unsafe extern "C" fn i2c_rd32(sd: *mut v4l2_subdev, reg: u16) -> u32 {
    static u32 i2c_rd32(struct v4l2_subdev *sd, u16 reg)
    {
    return i2c_rdreg(sd, reg, 4);
    }
#[no_mangle]
unsafe extern "C" fn i2c_wr32(sd: *mut v4l2_subdev, reg: u16, val: u32) {
    static void i2c_wr32(struct v4l2_subdev *sd, u16 reg, u32 val)
    {
    i2c_wrreg(sd, reg, val, 4);
    }
// --------------- STATUS ---------------
#[no_mangle]
pub unsafe extern "C" fn is_hdmi(sd: *mut v4l2_subdev) -> bool {
    static inline bool is_hdmi(struct v4l2_subdev *sd)
    {
    return i2c_rd8(sd, SYS_STATUS) & MASK_S_HDMI;
    }
#[no_mangle]
pub unsafe extern "C" fn tx_5v_power_present(sd: *mut v4l2_subdev) -> bool {
    static inline bool tx_5v_power_present(struct v4l2_subdev *sd)
    {
    return i2c_rd8(sd, SYS_STATUS) & MASK_S_DDC5V;
    }
#[no_mangle]
pub unsafe extern "C" fn no_signal(sd: *mut v4l2_subdev) -> bool {
    static inline bool no_signal(struct v4l2_subdev *sd)
    {
    return !(i2c_rd8(sd, SYS_STATUS) & MASK_S_TMDS);
    }
#[no_mangle]
pub unsafe extern "C" fn no_sync(sd: *mut v4l2_subdev) -> bool {
    static inline bool no_sync(struct v4l2_subdev *sd)
    {
    return !(i2c_rd8(sd, SYS_STATUS) & MASK_S_SYNC);
    }
#[no_mangle]
pub unsafe extern "C" fn audio_present(sd: *mut v4l2_subdev) -> bool {
    static inline bool audio_present(struct v4l2_subdev *sd)
    {
    return i2c_rd8(sd, AU_STATUS0) & MASK_S_A_SAMPLE;
    }
#[no_mangle]
unsafe extern "C" fn get_audio_sampling_rate(sd: *mut v4l2_subdev) -> c_int {
    static int get_audio_sampling_rate(struct v4l2_subdev *sd)
    {
    static const int code_to_rate[] = {
    44100, 0, 48000, 32000, 22050, 384000, 24000, 352800,
    88200, 768000, 96000, 705600, 176400, 0, 192000, 0
    };
// Register FS_SET is not cleared when the cable is disconnected
    if (no_signal(sd))
    return 0;
    return code_to_rate[i2c_rd8(sd, FS_SET) & MASK_FS];
    }
// --------------- TIMINGS ---------------
#[no_mangle]
pub unsafe extern "C" fn fps(t: *const v4l2_bt_timings) -> unsigned {
    static inline unsigned fps(const struct v4l2_bt_timings *t)
    {
    if (!V4L2_DV_BT_FRAME_HEIGHT(t) || !V4L2_DV_BT_FRAME_WIDTH(t))
    return 0;
    return DIV_ROUND_CLOSEST((unsigned)t.pixelclock,
    V4L2_DV_BT_FRAME_HEIGHT(t) * V4L2_DV_BT_FRAME_WIDTH(t));
    }
    static int tc358743_get_detected_timings(struct v4l2_subdev *sd,
    struct v4l2_dv_timings *timings)
    {
    struct v4l2_bt_timings *bt = &timings.bt;
    unsigned width, height, frame_width, frame_height, frame_interval, fps;
    memset(timings, 0, sizeof(struct v4l2_dv_timings));
// if HPD is low, ignore any video
    if (!(i2c_rd8(sd, HPD_CTL) & MASK_HPD_OUT0))
    return -ENOLINK;
    if (no_signal(sd)) {
    v4l2_dbg(1, debug, sd, "%s: no valid signal\n", __func__);
    return -ENOLINK;
    }
    if (no_sync(sd)) {
    v4l2_dbg(1, debug, sd, "%s: no sync on signal\n", __func__);
    return -ENOLCK;
    }
    timings.type = V4L2_DV_BT_656_1120;
    bt.interlaced = i2c_rd8(sd, VI_STATUS1) & MASK_S_V_INTERLACE ?
    V4L2_DV_INTERLACED : V4L2_DV_PROGRESSIVE;
    width = ((i2c_rd8(sd, DE_WIDTH_H_HI) & 0x1f) << 8) +
    i2c_rd8(sd, DE_WIDTH_H_LO);
    height = ((i2c_rd8(sd, DE_WIDTH_V_HI) & 0x1f) << 8) +
    i2c_rd8(sd, DE_WIDTH_V_LO);
    frame_width = ((i2c_rd8(sd, H_SIZE_HI) & 0x1f) << 8) +
    i2c_rd8(sd, H_SIZE_LO);
    frame_height = (((i2c_rd8(sd, V_SIZE_HI) & 0x3f) << 8) +
    i2c_rd8(sd, V_SIZE_LO)) / 2;
// frame interval in milliseconds * 10
// Require SYS_FREQ0 and SYS_FREQ1 are precisely set
    frame_interval = ((i2c_rd8(sd, FV_CNT_HI) & 0x3) << 8) +
    i2c_rd8(sd, FV_CNT_LO);
    fps = (frame_interval > 0) ?
    DIV_ROUND_CLOSEST(10000, frame_interval) : 0;
    bt.width = width;
    bt.height = height;
    bt.vsync = frame_height - height;
    bt.hsync = frame_width - width;
    bt.pixelclock = frame_width * frame_height * fps;
    if (bt.interlaced == V4L2_DV_INTERLACED) {
    bt.height *= 2;
    bt.il_vsync = bt.vsync + 1;
    bt.pixelclock /= 2;
    }
    return 0;
    }
// --------------- HOTPLUG / HDCP / EDID ---------------
#[no_mangle]
unsafe extern "C" fn tc358743_delayed_work_enable_hotplug(work: *mut work_struct) {
    static void tc358743_delayed_work_enable_hotplug(struct work_struct *work)
    {
    struct delayed_work *dwork = to_delayed_work(work);
    struct tc358743_state *state = container_of(dwork,
    struct tc358743_state, delayed_work_enable_hotplug);
    struct v4l2_subdev *sd = &state.sd;
    v4l2_dbg(2, debug, sd, "%s:\n", __func__);
    i2c_wr8_and_or(sd, HPD_CTL, ~MASK_HPD_OUT0, MASK_HPD_OUT0);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_set_hdmi_hdcp(sd: *mut v4l2_subdev, enable: bool) {
    static void tc358743_set_hdmi_hdcp(struct v4l2_subdev *sd, bool enable)
    {
    v4l2_dbg(2, debug, sd, "%s: %s\n", __func__, enable ?
    "enable" : "disable");
    if (enable) {
    i2c_wr8_and_or(sd, HDCP_REG3, ~KEY_RD_CMD, KEY_RD_CMD);
    i2c_wr8_and_or(sd, HDCP_MODE, ~MASK_MANUAL_AUTHENTICATION, 0);
    i2c_wr8_and_or(sd, HDCP_REG1, 0xff,
    MASK_AUTH_UNAUTH_SEL_16_FRAMES |
    MASK_AUTH_UNAUTH_AUTO);
    i2c_wr8_and_or(sd, HDCP_REG2, ~MASK_AUTO_P3_RESET,
    SET_AUTO_P3_RESET_FRAMES(0x0f));
    } else {
    i2c_wr8_and_or(sd, HDCP_MODE, ~MASK_MANUAL_AUTHENTICATION,
    MASK_MANUAL_AUTHENTICATION);
    }
    }
#[no_mangle]
unsafe extern "C" fn tc358743_disable_edid(sd: *mut v4l2_subdev) {
    static void tc358743_disable_edid(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    v4l2_dbg(2, debug, sd, "%s:\n", __func__);
    cancel_delayed_work_sync(&state.delayed_work_enable_hotplug);
// DDC access to EDID is also disabled when hotplug is disabled. See
// register DDC_CTL
    i2c_wr8_and_or(sd, HPD_CTL, ~MASK_HPD_OUT0, 0x0);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_enable_edid(sd: *mut v4l2_subdev) {
    static void tc358743_enable_edid(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    if (state.edid_blocks_written == 0) {
    v4l2_dbg(2, debug, sd, "%s: no EDID . no hotplug\n", __func__);
    tc358743_s_ctrl_detect_tx_5v(sd);
    return;
    }
    v4l2_dbg(2, debug, sd, "%s:\n", __func__);
// Enable hotplug after 143 ms. DDC access to EDID is also enabled when
// hotplug is enabled. See register DDC_CTL
    schedule_delayed_work(&state.delayed_work_enable_hotplug, HZ / 7);
    tc358743_enable_interrupts(sd, true);
    tc358743_s_ctrl_detect_tx_5v(sd);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_erase_bksv(sd: *mut v4l2_subdev) {
    static void tc358743_erase_bksv(struct v4l2_subdev *sd)
    {
    int i;
    for (i = 0; i < 5; i++)
    i2c_wr8(sd, BKSV + i, 0);
    }
// --------------- AVI infoframe ---------------
    static ssize_t
    tc358743_debugfs_if_read(u32 type, void *priv, struct file *filp,
    char __user *ubuf, size_t count, loff_t *ppos)
    {
    u8 buf[V4L2_DEBUGFS_IF_MAX_LEN] = {};
    struct v4l2_subdev *sd = priv;
    int len;
    if (!is_hdmi(sd))
    return 0;
    switch (type) {
    case V4L2_DEBUGFS_IF_AVI:
    i2c_rd(sd, PK_AVI_0HEAD, buf, PK_AVI_LEN);
    break;
    case V4L2_DEBUGFS_IF_AUDIO:
    i2c_rd(sd, PK_AUD_0HEAD, buf, PK_AUD_LEN);
    break;
    case V4L2_DEBUGFS_IF_SPD:
    i2c_rd(sd, PK_SPD_0HEAD, buf, PK_SPD_LEN);
    break;
    case V4L2_DEBUGFS_IF_HDMI:
    i2c_rd(sd, PK_VS_0HEAD, buf, PK_VS_LEN);
    break;
    case V4L2_DEBUGFS_IF_DRM:
    i2c_rd(sd, PK_ACP_0HEAD, buf, PK_ACP_LEN);
    break;
    default:
    return 0;
    }
    if (!buf[2])
    return -ENOENT;
    len = buf[2] + 4;
    if (len > V4L2_DEBUGFS_IF_MAX_LEN)
    len = -ENOENT;
    if (len > 0)
    len = simple_read_from_buffer(ubuf, count, ppos, buf, len);
    return len < 0 ? 0 : len;
    }
#[no_mangle]
unsafe extern "C" fn print_infoframes(sd: *mut v4l2_subdev) {
    static void print_infoframes(struct v4l2_subdev *sd)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct device *dev = &client.dev;
    union hdmi_infoframe frame;
    u8 buffer[V4L2_DEBUGFS_IF_MAX_LEN] = {};
//
// Updating the ACP TYPE here allows for dynamically
// changing the type you want to monitor, without having
// to reload the driver with a new packet_type module option value.
//
// Instead you can set it with the new value, then call
// VIDIOC_LOG_STATUS.
//
    i2c_wr8(sd, TYP_ACP_SET, packet_type);
    if (!is_hdmi(sd)) {
    v4l2_info(sd, "DVI-D signal - InfoFrames not supported\n");
    return;
    }
    i2c_rd(sd, PK_AVI_0HEAD, buffer, PK_AVI_LEN);
    if (hdmi_infoframe_unpack(&frame, buffer, sizeof(buffer)) >= 0)
    hdmi_infoframe_log(KERN_INFO, dev, &frame);
    i2c_rd(sd, PK_VS_0HEAD, buffer, PK_VS_LEN);
    if (hdmi_infoframe_unpack(&frame, buffer, sizeof(buffer)) >= 0)
    hdmi_infoframe_log(KERN_INFO, dev, &frame);
    i2c_rd(sd, PK_AUD_0HEAD, buffer, PK_AUD_LEN);
    if (hdmi_infoframe_unpack(&frame, buffer, sizeof(buffer)) >= 0)
    hdmi_infoframe_log(KERN_INFO, dev, &frame);
    i2c_rd(sd, PK_SPD_0HEAD, buffer, PK_SPD_LEN);
    if (hdmi_infoframe_unpack(&frame, buffer, sizeof(buffer)) >= 0)
    hdmi_infoframe_log(KERN_INFO, dev, &frame);
    i2c_rd(sd, PK_ACP_0HEAD, buffer, PK_ACP_LEN);
    if (buffer[0] == packet_type) {
    if (packet_type < 0x80)
    v4l2_info(sd, "Packet: %*ph\n", PK_ACP_LEN, buffer);
#[no_mangle]
pub unsafe extern "C" fn if(0x87: packet_type !=) -> else {
    else if (packet_type != 0x87)
    v4l2_info(sd, "InfoFrame: %*ph\n", PK_ACP_LEN, buffer);
    else if (hdmi_infoframe_unpack(&frame, buffer,
    sizeof(buffer)) >= 0)
    hdmi_infoframe_log(KERN_INFO, dev, &frame);
    }
    i2c_rd(sd, PK_MS_0HEAD, buffer, PK_MS_LEN);
    if (buffer[2] && buffer[2] + 3 <= PK_MS_LEN)
    v4l2_info(sd, "MPEG Source InfoFrame: %*ph\n",
    buffer[2] + 3, buffer);
    i2c_rd(sd, PK_ISRC1_0HEAD, buffer, PK_ISRC1_LEN);
    if (buffer[0] == 0x05)
    v4l2_info(sd, "ISRC1 Packet: %*ph\n",
    PK_ISRC1_LEN, buffer);
    i2c_rd(sd, PK_ISRC2_0HEAD, buffer, PK_ISRC2_LEN);
    if (buffer[0] == 0x06)
    v4l2_info(sd, "ISRC2 Packet: %*ph\n",
    PK_ISRC2_LEN, buffer);
    }
// --------------- CTRLS ---------------
#[no_mangle]
unsafe extern "C" fn tc358743_s_ctrl_detect_tx_5v(sd: *mut v4l2_subdev) -> c_int {
    static int tc358743_s_ctrl_detect_tx_5v(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    return v4l2_ctrl_s_ctrl(state.detect_tx_5v_ctrl,
    tx_5v_power_present(sd));
    }
#[no_mangle]
unsafe extern "C" fn tc358743_s_ctrl_audio_sampling_rate(sd: *mut v4l2_subdev) -> c_int {
    static int tc358743_s_ctrl_audio_sampling_rate(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    return v4l2_ctrl_s_ctrl(state.audio_sampling_rate_ctrl,
    get_audio_sampling_rate(sd));
    }
#[no_mangle]
unsafe extern "C" fn tc358743_s_ctrl_audio_present(sd: *mut v4l2_subdev) -> c_int {
    static int tc358743_s_ctrl_audio_present(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    return v4l2_ctrl_s_ctrl(state.audio_present_ctrl,
    audio_present(sd));
    }
#[no_mangle]
unsafe extern "C" fn tc358743_update_controls(sd: *mut v4l2_subdev) -> c_int {
    static int tc358743_update_controls(struct v4l2_subdev *sd)
    {
    let mut ret: c_int = 0;
    ret |= tc358743_s_ctrl_detect_tx_5v(sd);
    ret |= tc358743_s_ctrl_audio_sampling_rate(sd);
    ret |= tc358743_s_ctrl_audio_present(sd);
    return ret;
    }
// --------------- INIT ---------------
#[no_mangle]
unsafe extern "C" fn tc358743_reset_phy(sd: *mut v4l2_subdev) {
    static void tc358743_reset_phy(struct v4l2_subdev *sd)
    {
    v4l2_dbg(1, debug, sd, "%s:\n", __func__);
    i2c_wr8_and_or(sd, PHY_RST, ~MASK_RESET_CTRL, 0);
    i2c_wr8_and_or(sd, PHY_RST, ~MASK_RESET_CTRL, MASK_RESET_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_reset(sd: *mut v4l2_subdev, mask: u16) {
    static void tc358743_reset(struct v4l2_subdev *sd, uint16_t mask)
    {
    let mut sysctl: u16 = i2c_rd16(sd, SYSCTL);
    i2c_wr16(sd, SYSCTL, sysctl | mask);
    i2c_wr16(sd, SYSCTL, sysctl & ~mask);
    }
#[no_mangle]
pub unsafe extern "C" fn tc358743_sleep_mode(sd: *mut v4l2_subdev, enable: bool) {
    static inline void tc358743_sleep_mode(struct v4l2_subdev *sd, bool enable)
    {
    i2c_wr16_and_or(sd, SYSCTL, ~MASK_SLEEP,
    enable ? MASK_SLEEP : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn enable_stream(sd: *mut v4l2_subdev, enable: bool) {
    static inline void enable_stream(struct v4l2_subdev *sd, bool enable)
    {
    struct tc358743_state *state = to_state(sd);
    v4l2_dbg(3, debug, sd, "%s: %sable\n",
    __func__, enable ? "en" : "dis");
    if (enable) {
// It is critical for CSI receiver to see lane transition
// LP11->HS. Set to non-continuous mode to enable clock lane
// LP11 state.
    i2c_wr32(sd, TXOPTIONCNTRL, 0);
// Set to continuous mode to trigger LP11->HS transition
    i2c_wr32(sd, TXOPTIONCNTRL, MASK_CONTCLKMODE);
// Unmute video
    i2c_wr8(sd, VI_MUTE, MASK_AUTO_MUTE);
    } else {
// Mute video so that all data lanes go to LSP11 state.
// No data is output to CSI Tx block.
    i2c_wr8(sd, VI_MUTE, MASK_AUTO_MUTE | MASK_VI_MUTE);
    }
    mutex_lock(&state.confctl_mutex);
    i2c_wr16_and_or(sd, CONFCTL, ~(MASK_VBUFEN | MASK_ABUFEN),
    enable ? (MASK_VBUFEN | MASK_ABUFEN) : 0x0);
    mutex_unlock(&state.confctl_mutex);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_set_pll(sd: *mut v4l2_subdev) {
    static void tc358743_set_pll(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    struct tc358743_platform_data *pdata = &state.pdata;
    let mut pllctl0: u16 = i2c_rd16(sd, PLLCTL0);
    let mut pllctl1: u16 = i2c_rd16(sd, PLLCTL1);
    u16 pllctl0_new = SET_PLL_PRD(pdata.pll_prd) |
    SET_PLL_FBD(pdata.pll_fbd);
    let mut hsck: u32 = (pdata.refclk_hz / pdata.pll_prd) * pdata.pll_fbd;
    v4l2_dbg(2, debug, sd, "%s:\n", __func__);
// Only rewrite when needed (new value or disabled), since rewriting
// triggers another format change event.
    if ((pllctl0 != pllctl0_new) || ((pllctl1 & MASK_PLL_EN) == 0)) {
    u16 pll_frs;
    if (hsck > 500000000)
    pll_frs = 0x0;
#[no_mangle]
pub unsafe extern "C" fn if(250000000: hsck >) -> else {
    else if (hsck > 250000000)
    pll_frs = 0x1;
#[no_mangle]
pub unsafe extern "C" fn if(125000000: hsck >) -> else {
    else if (hsck > 125000000)
    pll_frs = 0x2;
    else
    pll_frs = 0x3;
    v4l2_dbg(1, debug, sd, "%s: updating PLL clock\n", __func__);
    tc358743_sleep_mode(sd, true);
    i2c_wr16(sd, PLLCTL0, pllctl0_new);
    i2c_wr16_and_or(sd, PLLCTL1,
    ~(MASK_PLL_FRS | MASK_RESETB | MASK_PLL_EN),
    (SET_PLL_FRS(pll_frs) | MASK_RESETB |
    MASK_PLL_EN));
    udelay(10); /* REF_02, Sheet "Source HDMI" */
    i2c_wr16_and_or(sd, PLLCTL1, ~MASK_CKEN, MASK_CKEN);
    tc358743_sleep_mode(sd, false);
    }
    }
#[no_mangle]
unsafe extern "C" fn tc358743_set_ref_clk(sd: *mut v4l2_subdev) {
    static void tc358743_set_ref_clk(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    struct tc358743_platform_data *pdata = &state.pdata;
    u32 sys_freq;
    u32 lockdet_ref;
    u32 cec_freq;
    u16 fh_min;
    u16 fh_max;
    BUG_ON(!(pdata.refclk_hz == 26000000 ||
    pdata.refclk_hz == 27000000 ||
    pdata.refclk_hz == 42000000));
    sys_freq = pdata.refclk_hz / 10000;
    i2c_wr8(sd, SYS_FREQ0, sys_freq & 0x00ff);
    i2c_wr8(sd, SYS_FREQ1, (sys_freq & 0xff00) >> 8);
    i2c_wr8_and_or(sd, PHY_CTL0, ~MASK_PHY_SYSCLK_IND,
    (pdata.refclk_hz == 42000000) ?
    MASK_PHY_SYSCLK_IND : 0x0);
    fh_min = pdata.refclk_hz / 100000;
    i2c_wr8(sd, FH_MIN0, fh_min & 0x00ff);
    i2c_wr8(sd, FH_MIN1, (fh_min & 0xff00) >> 8);
    fh_max = (fh_min * 66) / 10;
    i2c_wr8(sd, FH_MAX0, fh_max & 0x00ff);
    i2c_wr8(sd, FH_MAX1, (fh_max & 0xff00) >> 8);
    lockdet_ref = pdata.refclk_hz / 100;
    i2c_wr8(sd, LOCKDET_REF0, lockdet_ref & 0x0000ff);
    i2c_wr8(sd, LOCKDET_REF1, (lockdet_ref & 0x00ff00) >> 8);
    i2c_wr8(sd, LOCKDET_REF2, (lockdet_ref & 0x0f0000) >> 16);
    i2c_wr8_and_or(sd, NCO_F0_MOD, ~MASK_NCO_F0_MOD,
    (pdata.refclk_hz == 27000000) ?
    MASK_NCO_F0_MOD_27MHZ : 0x0);
//
// Trial and error suggests that the default register value
// of 656 is for a 42 MHz reference clock. Use that to derive
// a new value based on the actual reference clock.
//
    cec_freq = (656 * sys_freq) / 4200;
    i2c_wr16(sd, CECHCLK, cec_freq);
    i2c_wr16(sd, CECLCLK, cec_freq);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_set_csi_color_space(sd: *mut v4l2_subdev) {
    static void tc358743_set_csi_color_space(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    switch (state.mbus_fmt_code) {
    case MEDIA_BUS_FMT_UYVY8_1X16:
    v4l2_dbg(2, debug, sd, "%s: YCbCr 422 16-bit\n", __func__);
    i2c_wr8_and_or(sd, VOUT_SET2,
    ~(MASK_SEL422 | MASK_VOUT_422FIL_100) & 0xff,
    MASK_SEL422 | MASK_VOUT_422FIL_100);
    i2c_wr8_and_or(sd, VI_REP, ~MASK_VOUT_COLOR_SEL & 0xff,
    MASK_VOUT_COLOR_601_YCBCR_LIMITED);
    mutex_lock(&state.confctl_mutex);
    i2c_wr16_and_or(sd, CONFCTL, ~MASK_YCBCRFMT,
    MASK_YCBCRFMT_422_8_BIT);
    mutex_unlock(&state.confctl_mutex);
    break;
    case MEDIA_BUS_FMT_RGB888_1X24:
    v4l2_dbg(2, debug, sd, "%s: RGB 888 24-bit\n", __func__);
    i2c_wr8_and_or(sd, VOUT_SET2,
    ~(MASK_SEL422 | MASK_VOUT_422FIL_100) & 0xff,
    0x00);
    i2c_wr8_and_or(sd, VI_REP, ~MASK_VOUT_COLOR_SEL & 0xff,
    MASK_VOUT_COLOR_RGB_FULL);
    mutex_lock(&state.confctl_mutex);
    i2c_wr16_and_or(sd, CONFCTL, ~MASK_YCBCRFMT, 0);
    mutex_unlock(&state.confctl_mutex);
    break;
    default:
    v4l2_dbg(2, debug, sd, "%s: Unsupported format code 0x%x\n",
    __func__, state.mbus_fmt_code);
    }
    }
#[no_mangle]
unsafe extern "C" fn tc358743_num_csi_lanes_needed(sd: *mut v4l2_subdev) -> unsigned {
    static unsigned tc358743_num_csi_lanes_needed(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    struct v4l2_bt_timings *bt = &state.timings.bt;
    struct tc358743_platform_data *pdata = &state.pdata;
    u32 bits_pr_pixel =
    (state.mbus_fmt_code == MEDIA_BUS_FMT_UYVY8_1X16) ?  16 : 24;
    let mut bps: u32 = bt.width * bt.height * fps(bt) * bits_pr_pixel;
    let mut bps_pr_lane: u32 = (pdata.refclk_hz / pdata.pll_prd) * pdata.pll_fbd;
    return DIV_ROUND_UP(bps, bps_pr_lane);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_set_csi(sd: *mut v4l2_subdev) {
    static void tc358743_set_csi(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    struct tc358743_platform_data *pdata = &state.pdata;
    let mut lanes: unsigned = tc358743_num_csi_lanes_needed(sd);
    v4l2_dbg(3, debug, sd, "%s:\n", __func__);
    state.csi_lanes_in_use = lanes;
    tc358743_reset(sd, MASK_CTXRST);
    if (lanes < 1)
    i2c_wr32(sd, CLW_CNTRL, MASK_CLW_LANEDISABLE);
    if (lanes < 1)
    i2c_wr32(sd, D0W_CNTRL, MASK_D0W_LANEDISABLE);
    if (lanes < 2)
    i2c_wr32(sd, D1W_CNTRL, MASK_D1W_LANEDISABLE);
    if (lanes < 3)
    i2c_wr32(sd, D2W_CNTRL, MASK_D2W_LANEDISABLE);
    if (lanes < 4)
    i2c_wr32(sd, D3W_CNTRL, MASK_D3W_LANEDISABLE);
    i2c_wr32(sd, LINEINITCNT, pdata.lineinitcnt);
    i2c_wr32(sd, LPTXTIMECNT, pdata.lptxtimecnt);
    i2c_wr32(sd, TCLK_HEADERCNT, pdata.tclk_headercnt);
    i2c_wr32(sd, TCLK_TRAILCNT, pdata.tclk_trailcnt);
    i2c_wr32(sd, THS_HEADERCNT, pdata.ths_headercnt);
    i2c_wr32(sd, TWAKEUP, pdata.twakeup);
    i2c_wr32(sd, TCLK_POSTCNT, pdata.tclk_postcnt);
    i2c_wr32(sd, THS_TRAILCNT, pdata.ths_trailcnt);
    i2c_wr32(sd, HSTXVREGCNT, pdata.hstxvregcnt);
    i2c_wr32(sd, HSTXVREGEN,
    ((lanes > 0) ? MASK_CLM_HSTXVREGEN : 0x0) |
    ((lanes > 0) ? MASK_D0M_HSTXVREGEN : 0x0) |
    ((lanes > 1) ? MASK_D1M_HSTXVREGEN : 0x0) |
    ((lanes > 2) ? MASK_D2M_HSTXVREGEN : 0x0) |
    ((lanes > 3) ? MASK_D3M_HSTXVREGEN : 0x0));
    i2c_wr32(sd, TXOPTIONCNTRL, (state.bus.flags &
    V4L2_MBUS_CSI2_NONCONTINUOUS_CLOCK) ? 0 : MASK_CONTCLKMODE);
    i2c_wr32(sd, STARTCNTRL, MASK_START);
    i2c_wr32(sd, CSI_START, MASK_STRT);
    i2c_wr32(sd, CSI_CONFW, MASK_MODE_SET |
    MASK_ADDRESS_CSI_CONTROL |
    MASK_CSI_MODE |
    MASK_TXHSMD |
    ((lanes == 4) ? MASK_NOL_4 :
    (lanes == 3) ? MASK_NOL_3 :
    (lanes == 2) ? MASK_NOL_2 : MASK_NOL_1));
    i2c_wr32(sd, CSI_CONFW, MASK_MODE_SET |
    MASK_ADDRESS_CSI_ERR_INTENA | MASK_TXBRK | MASK_QUNK |
    MASK_WCER | MASK_INER);
    i2c_wr32(sd, CSI_CONFW, MASK_MODE_CLEAR |
    MASK_ADDRESS_CSI_ERR_HALT | MASK_TXBRK | MASK_QUNK);
    i2c_wr32(sd, CSI_CONFW, MASK_MODE_SET |
    MASK_ADDRESS_CSI_INT_ENA | MASK_INTER);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_set_hdmi_phy(sd: *mut v4l2_subdev) {
    static void tc358743_set_hdmi_phy(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    struct tc358743_platform_data *pdata = &state.pdata;
// Default settings from REF_02, sheet "Source HDMI"
// and custom settings as platform data
    i2c_wr8_and_or(sd, PHY_EN, ~MASK_ENABLE_PHY, 0x0);
    i2c_wr8(sd, PHY_CTL1, SET_PHY_AUTO_RST1_US(1600) |
    SET_FREQ_RANGE_MODE_CYCLES(1));
    i2c_wr8_and_or(sd, PHY_CTL2, ~MASK_PHY_AUTO_RSTn,
    (pdata.hdmi_phy_auto_reset_tmds_detected ?
    MASK_PHY_AUTO_RST2 : 0) |
    (pdata.hdmi_phy_auto_reset_tmds_in_range ?
    MASK_PHY_AUTO_RST3 : 0) |
    (pdata.hdmi_phy_auto_reset_tmds_valid ?
    MASK_PHY_AUTO_RST4 : 0));
    i2c_wr8(sd, PHY_BIAS, 0x40);
    i2c_wr8(sd, PHY_CSQ, SET_CSQ_CNT_LEVEL(0x0a));
    i2c_wr8(sd, AVM_CTL, 45);
    i2c_wr8_and_or(sd, HDMI_DET, ~MASK_HDMI_DET_V,
    pdata.hdmi_detection_delay << 4);
    i2c_wr8_and_or(sd, HV_RST, ~(MASK_H_PI_RST | MASK_V_PI_RST),
    (pdata.hdmi_phy_auto_reset_hsync_out_of_range ?
    MASK_H_PI_RST : 0) |
    (pdata.hdmi_phy_auto_reset_vsync_out_of_range ?
    MASK_V_PI_RST : 0));
    i2c_wr8_and_or(sd, PHY_EN, ~MASK_ENABLE_PHY, MASK_ENABLE_PHY);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_set_hdmi_audio(sd: *mut v4l2_subdev) {
    static void tc358743_set_hdmi_audio(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
// Default settings from REF_02, sheet "Source HDMI"
    i2c_wr8(sd, FORCE_MUTE, 0x00);
    i2c_wr8(sd, AUTO_CMD0, MASK_AUTO_MUTE7 | MASK_AUTO_MUTE6 |
    MASK_AUTO_MUTE5 | MASK_AUTO_MUTE4 |
    MASK_AUTO_MUTE1 | MASK_AUTO_MUTE0);
    i2c_wr8(sd, AUTO_CMD1, MASK_AUTO_MUTE9);
    i2c_wr8(sd, AUTO_CMD2, MASK_AUTO_PLAY3 | MASK_AUTO_PLAY2);
    i2c_wr8(sd, BUFINIT_START, SET_BUFINIT_START_MS(500));
    i2c_wr8(sd, FS_MUTE, 0x00);
    i2c_wr8(sd, FS_IMODE, MASK_NLPCM_SMODE | MASK_FS_SMODE);
    i2c_wr8(sd, ACR_MODE, MASK_CTS_MODE);
    i2c_wr8(sd, ACR_MDF0, MASK_ACR_L2MDF_1976_PPM | MASK_ACR_L1MDF_976_PPM);
    i2c_wr8(sd, ACR_MDF1, MASK_ACR_L3MDF_3906_PPM);
    i2c_wr8(sd, SDO_MODE1, MASK_SDO_FMT_I2S);
    i2c_wr8(sd, DIV_MODE, SET_DIV_DLY_MS(100));
    mutex_lock(&state.confctl_mutex);
    i2c_wr16_and_or(sd, CONFCTL, 0xffff, MASK_AUDCHNUM_2 |
    MASK_AUDOUTSEL_I2S | MASK_AUTOINDEX);
    mutex_unlock(&state.confctl_mutex);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_set_hdmi_info_frame_mode(sd: *mut v4l2_subdev) {
    static void tc358743_set_hdmi_info_frame_mode(struct v4l2_subdev *sd)
    {
// Default settings from REF_02, sheet "Source HDMI"
    i2c_wr8(sd, PK_INT_MODE, MASK_ISRC2_INT_MODE | MASK_ISRC_INT_MODE |
    MASK_ACP_INT_MODE | MASK_VS_INT_MODE |
    MASK_SPD_INT_MODE | MASK_MS_INT_MODE |
    MASK_AUD_INT_MODE | MASK_AVI_INT_MODE);
    i2c_wr8(sd, NO_PKT_LIMIT, 0x2c);
    i2c_wr8(sd, NO_PKT_CLR, 0x53);
    i2c_wr8(sd, ERR_PK_LIMIT, 0x01);
    i2c_wr8(sd, NO_PKT_LIMIT2, 0x30);
    i2c_wr8(sd, NO_GDB_LIMIT, 0x10);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_initial_setup(sd: *mut v4l2_subdev) {
    static void tc358743_initial_setup(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    struct tc358743_platform_data *pdata = &state.pdata;
//
// IR is not supported by this driver.
// CEC is only enabled if needed.
//
    i2c_wr16_and_or(sd, SYSCTL, ~(MASK_IRRST | MASK_CECRST),
    (MASK_IRRST | MASK_CECRST));
    tc358743_reset(sd, MASK_CTXRST | MASK_HDMIRST);

    tc358743_reset(sd, MASK_CECRST);

    tc358743_sleep_mode(sd, false);
    i2c_wr16(sd, FIFOCTL, pdata.fifo_level);
    tc358743_set_ref_clk(sd);
    i2c_wr8_and_or(sd, DDC_CTL, ~MASK_DDC5V_MODE,
    pdata.ddc5v_delay & MASK_DDC5V_MODE);
    i2c_wr8_and_or(sd, EDID_MODE, ~MASK_EDID_MODE, MASK_EDID_MODE_E_DDC);
    tc358743_set_hdmi_phy(sd);
    tc358743_set_hdmi_hdcp(sd, pdata.enable_hdcp);
    tc358743_set_hdmi_audio(sd);
    tc358743_set_hdmi_info_frame_mode(sd);
// All CE and IT formats are detected as RGB full range in DVI mode
    i2c_wr8_and_or(sd, VI_MODE, ~MASK_RGB_DVI, 0);
    i2c_wr8_and_or(sd, VOUT_SET2, ~MASK_VOUTCOLORMODE,
    MASK_VOUTCOLORMODE_AUTO);
    i2c_wr8(sd, VOUT_SET3, MASK_VOUT_EXTCNT);
    }
// --------------- CEC ---------------

#[no_mangle]
unsafe extern "C" fn tc358743_cec_adap_enable(adap: *mut cec_adapter, enable: bool) -> c_int {
    static int tc358743_cec_adap_enable(struct cec_adapter *adap, bool enable)
    {
    struct tc358743_state *state = adap.priv;
    struct v4l2_subdev *sd = &state.sd;
    i2c_wr32(sd, CECIMSK, enable ? MASK_CECTIM | MASK_CECRIM : 0);
    i2c_wr32(sd, CECICLR, MASK_CECTICLR | MASK_CECRICLR);
    i2c_wr32(sd, CECEN, enable);
    if (enable)
    i2c_wr32(sd, CECREN, MASK_CECREN);
    return 0;
    }
    static int tc358743_cec_adap_monitor_all_enable(struct cec_adapter *adap,
    bool enable)
    {
    struct tc358743_state *state = adap.priv;
    struct v4l2_subdev *sd = &state.sd;
    u32 reg;
    reg = i2c_rd32(sd, CECRCTL1);
    if (enable)
    reg |= MASK_CECOTH;
    else
    reg &= ~MASK_CECOTH;
    i2c_wr32(sd, CECRCTL1, reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc358743_cec_adap_log_addr(adap: *mut cec_adapter, log_addr: u8) -> c_int {
    static int tc358743_cec_adap_log_addr(struct cec_adapter *adap, u8 log_addr)
    {
    struct tc358743_state *state = adap.priv;
    struct v4l2_subdev *sd = &state.sd;
    let mut la: c_uint = 0;
    if (log_addr != CEC_LOG_ADDR_INVALID) {
    la = i2c_rd32(sd, CECADD);
    la |= 1 << log_addr;
    }
    i2c_wr32(sd, CECADD, la);
    return 0;
    }
    static int tc358743_cec_adap_transmit(struct cec_adapter *adap, u8 attempts,
    u32 signal_free_time, struct cec_msg *msg)
    {
    struct tc358743_state *state = adap.priv;
    struct v4l2_subdev *sd = &state.sd;
    unsigned int i;
    i2c_wr32(sd, CECTCTL,
    (cec_msg_is_broadcast(msg) ? MASK_CECBRD : 0) |
    (signal_free_time - 1));
    for (i = 0; i < msg.len; i++)
    i2c_wr32(sd, CECTBUF1 + i * 4,
    msg.msg[i] | ((i == msg.len - 1) ? MASK_CECTEOM : 0));
    i2c_wr32(sd, CECTEN, MASK_CECTEN);
    return 0;
    }
    static const struct cec_adap_ops tc358743_cec_adap_ops = {
    .adap_enable = tc358743_cec_adap_enable,
    .adap_log_addr = tc358743_cec_adap_log_addr,
    .adap_transmit = tc358743_cec_adap_transmit,
    .adap_monitor_all_enable = tc358743_cec_adap_monitor_all_enable,
    };
    static void tc358743_cec_handler(struct v4l2_subdev *sd, u16 intstatus,
    bool *handled)
    {
    struct tc358743_state *state = to_state(sd);
    unsigned int cec_rxint, cec_txint;
    let mut clr: c_uint = 0;
    cec_rxint = i2c_rd32(sd, CECRSTAT);
    cec_txint = i2c_rd32(sd, CECTSTAT);
    if (intstatus & MASK_CEC_RINT)
    clr |= MASK_CECRICLR;
    if (intstatus & MASK_CEC_TINT)
    clr |= MASK_CECTICLR;
    i2c_wr32(sd, CECICLR, clr);
    if ((intstatus & MASK_CEC_TINT) && cec_txint) {
    if (cec_txint & MASK_CECTIEND)
    cec_transmit_attempt_done(state.cec_adap,
    CEC_TX_STATUS_OK);
#[no_mangle]
pub unsafe extern "C" fn if(MASK_CECTIAL: cec_txint &) -> else {
    else if (cec_txint & MASK_CECTIAL)
    cec_transmit_attempt_done(state.cec_adap,
    CEC_TX_STATUS_ARB_LOST);
#[no_mangle]
pub unsafe extern "C" fn if(MASK_CECTIACK: cec_txint &) -> else {
    else if (cec_txint & MASK_CECTIACK)
    cec_transmit_attempt_done(state.cec_adap,
    CEC_TX_STATUS_NACK);
#[no_mangle]
pub unsafe extern "C" fn if(MASK_CECTIUR: cec_txint &) -> else {
//
// Not sure when this bit is set. Treat
// it as an error for now.
//
    cec_transmit_attempt_done(state.cec_adap,
    CEC_TX_STATUS_ERROR);
    }
    if (handled)
// handled = true;
    }
    if ((intstatus & MASK_CEC_RINT) &&
    (cec_rxint & MASK_CECRIEND)) {
    let mut msg: cec_msg = {};
    unsigned int i;
    unsigned int v;
    v = i2c_rd32(sd, CECRCTR);
    msg.len = v & 0x1f;
    if (msg.len > CEC_MAX_MSG_SIZE)
    msg.len = CEC_MAX_MSG_SIZE;
    for (i = 0; i < msg.len; i++) {
    v = i2c_rd32(sd, CECRBUF1 + i * 4);
    msg.msg[i] = v & 0xff;
    }
    cec_received_msg(state.cec_adap, &msg);
    if (handled)
// handled = true;
    }
    i2c_wr16(sd, INTSTATUS,
    intstatus & (MASK_CEC_RINT | MASK_CEC_TINT));
    }

// --------------- IRQ ---------------
#[no_mangle]
unsafe extern "C" fn tc358743_format_change(sd: *mut v4l2_subdev) {
    static void tc358743_format_change(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    struct v4l2_dv_timings timings;
    const struct v4l2_event tc358743_ev_fmt = {
    .type = V4L2_EVENT_SOURCE_CHANGE,
    .u.src_change.changes = V4L2_EVENT_SRC_CH_RESOLUTION,
    };
    if (tc358743_get_detected_timings(sd, &timings)) {
    enable_stream(sd, false);
    v4l2_dbg(1, debug, sd, "%s: No signal\n",
    __func__);
    } else {
    if (!v4l2_match_dv_timings(&state.timings, &timings, 0, false))
    enable_stream(sd, false);
    if (debug)
    v4l2_print_dv_timings(sd.name,
    "tc358743_format_change: New format: ",
    &timings, false);
    }
    if (sd.devnode)
    v4l2_subdev_notify_event(sd, &tc358743_ev_fmt);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_init_interrupts(sd: *mut v4l2_subdev) {
    static void tc358743_init_interrupts(struct v4l2_subdev *sd)
    {
    u16 i;
// clear interrupt status registers
    for (i = SYS_INT; i <= KEY_INT; i++)
    i2c_wr8(sd, i, 0xff);
    i2c_wr16(sd, INTSTATUS, 0xffff);
    }
    static void tc358743_enable_interrupts(struct v4l2_subdev *sd,
    bool cable_connected)
    {
    v4l2_dbg(2, debug, sd, "%s: cable connected = %d\n", __func__,
    cable_connected);
    if (cable_connected) {
    i2c_wr8(sd, SYS_INTM, ~(MASK_M_DDC | MASK_M_DVI_DET |
    MASK_M_HDMI_DET) & 0xff);
    i2c_wr8(sd, CLK_INTM, ~MASK_M_IN_DE_CHG);
    i2c_wr8(sd, CBIT_INTM, ~(MASK_M_CBIT_FS | MASK_M_AF_LOCK |
    MASK_M_AF_UNLOCK) & 0xff);
    i2c_wr8(sd, AUDIO_INTM, ~MASK_M_BUFINIT_END);
    i2c_wr8(sd, MISC_INTM, ~MASK_M_SYNC_CHG);
    } else {
    i2c_wr8(sd, SYS_INTM, ~MASK_M_DDC & 0xff);
    i2c_wr8(sd, CLK_INTM, 0xff);
    i2c_wr8(sd, CBIT_INTM, 0xff);
    i2c_wr8(sd, AUDIO_INTM, 0xff);
    i2c_wr8(sd, MISC_INTM, 0xff);
    }
    }
    static void tc358743_hdmi_audio_int_handler(struct v4l2_subdev *sd,
    bool *handled)
    {
    let mut audio_int_mask: u8 = i2c_rd8(sd, AUDIO_INTM);
    let mut audio_int: u8 = i2c_rd8(sd, AUDIO_INT) & ~audio_int_mask;
    i2c_wr8(sd, AUDIO_INT, audio_int);
    v4l2_dbg(3, debug, sd, "%s: AUDIO_INT = 0x%02x\n", __func__, audio_int);
    tc358743_s_ctrl_audio_sampling_rate(sd);
    tc358743_s_ctrl_audio_present(sd);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_csi_err_int_handler(sd: *mut v4l2_subdev, handled: *mut bool) {
    static void tc358743_csi_err_int_handler(struct v4l2_subdev *sd, bool *handled)
    {
    v4l2_err(sd, "%s: CSI_ERR = 0x%x\n", __func__, i2c_rd32(sd, CSI_ERR));
    i2c_wr32(sd, CSI_INT_CLR, MASK_ICRER);
    }
    static void tc358743_hdmi_misc_int_handler(struct v4l2_subdev *sd,
    bool *handled)
    {
    let mut misc_int_mask: u8 = i2c_rd8(sd, MISC_INTM);
    let mut misc_int: u8 = i2c_rd8(sd, MISC_INT) & ~misc_int_mask;
    i2c_wr8(sd, MISC_INT, misc_int);
    v4l2_dbg(3, debug, sd, "%s: MISC_INT = 0x%02x\n", __func__, misc_int);
    if (misc_int & MASK_I_SYNC_CHG) {
// Reset the HDMI PHY to try to trigger proper lock on the
// incoming video format. Erase BKSV to prevent that old keys
// are used when a new source is connected.
    if (no_sync(sd) || no_signal(sd)) {
    tc358743_reset_phy(sd);
    tc358743_erase_bksv(sd);
    }
    tc358743_format_change(sd);
    misc_int &= ~MASK_I_SYNC_CHG;
    if (handled)
// handled = true;
    }
    if (misc_int) {
    v4l2_err(sd, "%s: Unhandled MISC_INT interrupts: 0x%02x\n",
    __func__, misc_int);
    }
    }
    static void tc358743_hdmi_cbit_int_handler(struct v4l2_subdev *sd,
    bool *handled)
    {
    let mut cbit_int_mask: u8 = i2c_rd8(sd, CBIT_INTM);
    let mut cbit_int: u8 = i2c_rd8(sd, CBIT_INT) & ~cbit_int_mask;
    i2c_wr8(sd, CBIT_INT, cbit_int);
    v4l2_dbg(3, debug, sd, "%s: CBIT_INT = 0x%02x\n", __func__, cbit_int);
    if (cbit_int & MASK_I_CBIT_FS) {
    v4l2_dbg(1, debug, sd, "%s: Audio sample rate changed\n",
    __func__);
    tc358743_s_ctrl_audio_sampling_rate(sd);
    cbit_int &= ~MASK_I_CBIT_FS;
    if (handled)
// handled = true;
    }
    if (cbit_int & (MASK_I_AF_LOCK | MASK_I_AF_UNLOCK)) {
    v4l2_dbg(1, debug, sd, "%s: Audio present changed\n",
    __func__);
    tc358743_s_ctrl_audio_present(sd);
    cbit_int &= ~(MASK_I_AF_LOCK | MASK_I_AF_UNLOCK);
    if (handled)
// handled = true;
    }
    if (cbit_int) {
    v4l2_err(sd, "%s: Unhandled CBIT_INT interrupts: 0x%02x\n",
    __func__, cbit_int);
    }
    }
#[no_mangle]
unsafe extern "C" fn tc358743_hdmi_clk_int_handler(sd: *mut v4l2_subdev, handled: *mut bool) {
    static void tc358743_hdmi_clk_int_handler(struct v4l2_subdev *sd, bool *handled)
    {
    let mut clk_int_mask: u8 = i2c_rd8(sd, CLK_INTM);
    let mut clk_int: u8 = i2c_rd8(sd, CLK_INT) & ~clk_int_mask;
// Bit 7 and bit 6 are set even when they are masked
    i2c_wr8(sd, CLK_INT, clk_int | 0x80 | MASK_I_OUT_H_CHG);
    v4l2_dbg(3, debug, sd, "%s: CLK_INT = 0x%02x\n", __func__, clk_int);
    if (clk_int & (MASK_I_IN_DE_CHG)) {
    v4l2_dbg(1, debug, sd, "%s: DE size or position has changed\n",
    __func__);
// If the source switch to a new resolution with the same pixel
// frequency as the existing (e.g. 1080p25 -> 720p50), the
// I_SYNC_CHG interrupt is not always triggered, while the
// I_IN_DE_CHG interrupt seems to work fine. Format change
// notifications are only sent when the signal is stable to
// reduce the number of notifications.
    if (!no_signal(sd) && !no_sync(sd))
    tc358743_format_change(sd);
    clk_int &= ~(MASK_I_IN_DE_CHG);
    if (handled)
// handled = true;
    }
    if (clk_int) {
    v4l2_err(sd, "%s: Unhandled CLK_INT interrupts: 0x%02x\n",
    __func__, clk_int);
    }
    }
#[no_mangle]
unsafe extern "C" fn tc358743_hdmi_sys_int_handler(sd: *mut v4l2_subdev, handled: *mut bool) {
    static void tc358743_hdmi_sys_int_handler(struct v4l2_subdev *sd, bool *handled)
    {
    struct tc358743_state *state = to_state(sd);
    let mut sys_int_mask: u8 = i2c_rd8(sd, SYS_INTM);
    let mut sys_int: u8 = i2c_rd8(sd, SYS_INT) & ~sys_int_mask;
    i2c_wr8(sd, SYS_INT, sys_int);
    v4l2_dbg(3, debug, sd, "%s: SYS_INT = 0x%02x\n", __func__, sys_int);
    if (sys_int & MASK_I_DDC) {
    let mut tx_5v: bool = tx_5v_power_present(sd);
    v4l2_dbg(1, debug, sd, "%s: Tx 5V power present: %s\n",
    __func__, tx_5v ?  "yes" : "no");
    if (tx_5v) {
    tc358743_enable_edid(sd);
    } else {
    tc358743_enable_interrupts(sd, false);
    tc358743_disable_edid(sd);
    memset(&state.timings, 0, sizeof(state.timings));
    tc358743_erase_bksv(sd);
    tc358743_update_controls(sd);
    }
    sys_int &= ~MASK_I_DDC;
    if (handled)
// handled = true;
    }
    if (sys_int & MASK_I_DVI) {
    v4l2_dbg(1, debug, sd, "%s: HDMI.DVI change detected\n",
    __func__);
// Reset the HDMI PHY to try to trigger proper lock on the
// incoming video format. Erase BKSV to prevent that old keys
// are used when a new source is connected.
    if (no_sync(sd) || no_signal(sd)) {
    tc358743_reset_phy(sd);
    tc358743_erase_bksv(sd);
    }
    sys_int &= ~MASK_I_DVI;
    if (handled)
// handled = true;
    }
    if (sys_int & MASK_I_HDMI) {
    v4l2_dbg(1, debug, sd, "%s: DVI.HDMI change detected\n",
    __func__);
// Register is reset in DVI mode (REF_01, c. 6.6.41)
    i2c_wr8(sd, ANA_CTL, MASK_APPL_PCSX_NORMAL | MASK_ANALOG_ON);
    sys_int &= ~MASK_I_HDMI;
    if (handled)
// handled = true;
    }
    if (sys_int) {
    v4l2_err(sd, "%s: Unhandled SYS_INT interrupts: 0x%02x\n",
    __func__, sys_int);
    }
    }
// --------------- CORE OPS ---------------
#[no_mangle]
unsafe extern "C" fn tc358743_log_status(sd: *mut v4l2_subdev) -> c_int {
    static int tc358743_log_status(struct v4l2_subdev *sd)
    {
    struct tc358743_state *state = to_state(sd);
    struct v4l2_dv_timings timings;
    let mut hdmi_sys_status: u8 = i2c_rd8(sd, SYS_STATUS);
    let mut sysctl: u16 = i2c_rd16(sd, SYSCTL);
    let mut vi_status3: u8 = i2c_rd8(sd, VI_STATUS3);
    const int deep_color_mode[4] = { 8, 10, 12, 16 };
    static const char * const input_color_space[] = {
    "RGB", "YCbCr 601", "opRGB", "YCbCr 709", "NA (4)",
    "xvYCC 601", "NA(6)", "xvYCC 709", "NA(8)", "sYCC601",
    "NA(10)", "NA(11)", "NA(12)", "opYCC 601"};
    v4l2_info(sd, "-----Chip status-----\n");
    v4l2_info(sd, "Chip ID: 0x%02x\n",
    (i2c_rd16(sd, CHIPID) & MASK_CHIPID) >> 8);
    v4l2_info(sd, "Chip revision: 0x%02x\n",
    i2c_rd16(sd, CHIPID) & MASK_REVID);
    v4l2_info(sd, "Reset: IR: %d, CEC: %d, CSI TX: %d, HDMI: %d\n",
    !!(sysctl & MASK_IRRST),
    !!(sysctl & MASK_CECRST),
    !!(sysctl & MASK_CTXRST),
    !!(sysctl & MASK_HDMIRST));
    v4l2_info(sd, "Sleep mode: %s\n", sysctl & MASK_SLEEP ? "on" : "off");
    v4l2_info(sd, "Cable detected (+5V power): %s\n",
    hdmi_sys_status & MASK_S_DDC5V ? "yes" : "no");
    v4l2_info(sd, "DDC lines enabled: %s\n",
    (i2c_rd8(sd, EDID_MODE) & MASK_EDID_MODE_E_DDC) ?
    "yes" : "no");
    v4l2_info(sd, "Hotplug enabled: %s\n",
    (i2c_rd8(sd, HPD_CTL) & MASK_HPD_OUT0) ?
    "yes" : "no");
    v4l2_info(sd, "CEC enabled: %s\n",
    (i2c_rd16(sd, CECEN) & MASK_CECEN) ?  "yes" : "no");
    v4l2_info(sd, "-----Signal status-----\n");
    v4l2_info(sd, "TMDS signal detected: %s\n",
    hdmi_sys_status & MASK_S_TMDS ? "yes" : "no");
    v4l2_info(sd, "Stable sync signal: %s\n",
    hdmi_sys_status & MASK_S_SYNC ? "yes" : "no");
    v4l2_info(sd, "PHY PLL locked: %s\n",
    hdmi_sys_status & MASK_S_PHY_PLL ? "yes" : "no");
    v4l2_info(sd, "PHY DE detected: %s\n",
    hdmi_sys_status & MASK_S_PHY_SCDT ? "yes" : "no");
    if (tc358743_get_detected_timings(sd, &timings)) {
    v4l2_info(sd, "No video detected\n");
    } else {
    v4l2_print_dv_timings(sd.name, "Detected format: ", &timings,
    true);
    }
    v4l2_print_dv_timings(sd.name, "Configured format: ", &state.timings,
    true);
    v4l2_info(sd, "-----CSI-TX status-----\n");
    v4l2_info(sd, "Lanes needed: %d\n",
    tc358743_num_csi_lanes_needed(sd));
    v4l2_info(sd, "Lanes in use: %d\n",
    state.csi_lanes_in_use);
    v4l2_info(sd, "Waiting for particular sync signal: %s\n",
    (i2c_rd16(sd, CSI_STATUS) & MASK_S_WSYNC) ?
    "yes" : "no");
    v4l2_info(sd, "Transmit mode: %s\n",
    (i2c_rd16(sd, CSI_STATUS) & MASK_S_TXACT) ?
    "yes" : "no");
    v4l2_info(sd, "Receive mode: %s\n",
    (i2c_rd16(sd, CSI_STATUS) & MASK_S_RXACT) ?
    "yes" : "no");
    v4l2_info(sd, "Stopped: %s\n",
    (i2c_rd16(sd, CSI_STATUS) & MASK_S_HLT) ?
    "yes" : "no");
    v4l2_info(sd, "Color space: %s\n",
    state.mbus_fmt_code == MEDIA_BUS_FMT_UYVY8_1X16 ?
    "YCbCr 422 16-bit" :
    state.mbus_fmt_code == MEDIA_BUS_FMT_RGB888_1X24 ?
    "RGB 888 24-bit" : "Unsupported");
    v4l2_info(sd, "-----%s status-----\n", is_hdmi(sd) ? "HDMI" : "DVI-D");
    v4l2_info(sd, "HDCP encrypted content: %s\n",
    hdmi_sys_status & MASK_S_HDCP ? "yes" : "no");
    v4l2_info(sd, "Input color space: %s %s range\n",
    input_color_space[(vi_status3 & MASK_S_V_COLOR) >> 1],
    (vi_status3 & MASK_LIMITED) ? "limited" : "full");
    if (!is_hdmi(sd))
    return 0;
    v4l2_info(sd, "AV Mute: %s\n", hdmi_sys_status & MASK_S_AVMUTE ? "on" :
    "off");
    v4l2_info(sd, "Deep color mode: %d-bits per channel\n",
    deep_color_mode[(i2c_rd8(sd, VI_STATUS1) &
    MASK_S_DEEPCOLOR) >> 2]);
    print_infoframes(sd);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn tc358743_print_register_map(sd: *mut v4l2_subdev) {
    static void tc358743_print_register_map(struct v4l2_subdev *sd)
    {
    v4l2_info(sd, "0x0000-0x00FF: Global Control Register\n");
    v4l2_info(sd, "0x0100-0x01FF: CSI2-TX PHY Register\n");
    v4l2_info(sd, "0x0200-0x03FF: CSI2-TX PPI Register\n");
    v4l2_info(sd, "0x0400-0x05FF: Reserved\n");
    v4l2_info(sd, "0x0600-0x06FF: CEC Register\n");
    v4l2_info(sd, "0x0700-0x84FF: Reserved\n");
    v4l2_info(sd, "0x8500-0x85FF: HDMIRX System Control Register\n");
    v4l2_info(sd, "0x8600-0x86FF: HDMIRX Audio Control Register\n");
    v4l2_info(sd, "0x8700-0x87FF: HDMIRX InfoFrame packet data Register\n");
    v4l2_info(sd, "0x8800-0x88FF: HDMIRX HDCP Port Register\n");
    v4l2_info(sd, "0x8900-0x89FF: HDMIRX Video Output Port & 3D Register\n");
    v4l2_info(sd, "0x8A00-0x8BFF: Reserved\n");
    v4l2_info(sd, "0x8C00-0x8FFF: HDMIRX EDID-RAM (1024bytes)\n");
    v4l2_info(sd, "0x9000-0x90FF: HDMIRX GBD Extraction Control\n");
    v4l2_info(sd, "0x9100-0x92FF: HDMIRX GBD RAM read\n");
    v4l2_info(sd, "0x9300-      : Reserved\n");
    }
#[no_mangle]
unsafe extern "C" fn tc358743_get_reg_size(address: u16) -> c_int {
    static int tc358743_get_reg_size(u16 address)
    {
// REF_01 p. 66-72
    if (address <= 0x00ff)
    return 2;
#[no_mangle]
pub unsafe extern "C" fn if(0x06FF): (address >= 0x0100) && (address <=) -> else {
    else if ((address >= 0x0100) && (address <= 0x06FF))
    return 4;
#[no_mangle]
pub unsafe extern "C" fn if(0x84ff): (address >= 0x0700) && (address <=) -> else {
    else if ((address >= 0x0700) && (address <= 0x84ff))
    return 2;
    else
    return 1;
    }
    static int tc358743_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    if (reg.reg > 0xffff) {
    tc358743_print_register_map(sd);
    return -EINVAL;
    }
    reg.size = tc358743_get_reg_size(reg.reg);
    reg.val = i2c_rdreg(sd, reg.reg, reg.size);
    return 0;
    }
    static int tc358743_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    if (reg.reg > 0xffff) {
    tc358743_print_register_map(sd);
    return -EINVAL;
    }
// It should not be possible for the user to enable HDCP with a simple
// v4l2-dbg command.
//
// DO NOT REMOVE THIS unless all other issues with HDCP have been
// resolved.
//
    if (reg.reg == HDCP_MODE ||
    reg.reg == HDCP_REG1 ||
    reg.reg == HDCP_REG2 ||
    reg.reg == HDCP_REG3 ||
    reg.reg == BCAPS)
    return 0;
    i2c_wrreg(sd, (u16)reg.reg, reg.val,
    tc358743_get_reg_size(reg.reg));
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn tc358743_isr(sd: *mut v4l2_subdev, status: u32, handled: *mut bool) -> c_int {
    static int tc358743_isr(struct v4l2_subdev *sd, u32 status, bool *handled)
    {
    let mut intstatus: u16 = i2c_rd16(sd, INTSTATUS);
    v4l2_dbg(1, debug, sd, "%s: IntStatus = 0x%04x\n", __func__, intstatus);
    if (intstatus & MASK_HDMI_INT) {
    let mut hdmi_int0: u8 = i2c_rd8(sd, HDMI_INT0);
    let mut hdmi_int1: u8 = i2c_rd8(sd, HDMI_INT1);
    if (hdmi_int0 & MASK_I_MISC)
    tc358743_hdmi_misc_int_handler(sd, handled);
    if (hdmi_int1 & MASK_I_CBIT)
    tc358743_hdmi_cbit_int_handler(sd, handled);
    if (hdmi_int1 & MASK_I_CLK)
    tc358743_hdmi_clk_int_handler(sd, handled);
    if (hdmi_int1 & MASK_I_SYS)
    tc358743_hdmi_sys_int_handler(sd, handled);
    if (hdmi_int1 & MASK_I_AUD)
    tc358743_hdmi_audio_int_handler(sd, handled);
    i2c_wr16(sd, INTSTATUS, MASK_HDMI_INT);
    intstatus &= ~MASK_HDMI_INT;
    }

    if (intstatus & (MASK_CEC_RINT | MASK_CEC_TINT)) {
    tc358743_cec_handler(sd, intstatus, handled);
    i2c_wr16(sd, INTSTATUS,
    intstatus & (MASK_CEC_RINT | MASK_CEC_TINT));
    intstatus &= ~(MASK_CEC_RINT | MASK_CEC_TINT);
    }

    if (intstatus & MASK_CSI_INT) {
    let mut csi_int: u32 = i2c_rd32(sd, CSI_INT);
    if (csi_int & MASK_INTER)
    tc358743_csi_err_int_handler(sd, handled);
    i2c_wr16(sd, INTSTATUS, MASK_CSI_INT);
    }
    intstatus = i2c_rd16(sd, INTSTATUS);
    if (intstatus) {
    v4l2_dbg(1, debug, sd,
    "%s: Unhandled IntStatus interrupts: 0x%02x\n",
    __func__, intstatus);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc358743_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tc358743_irq_handler(int irq, void *dev_id)
    {
    struct tc358743_state *state = dev_id;
    let mut handled: bool = false;
    tc358743_isr(&state.sd, 0, &handled);
    return handled ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn tc358743_irq_poll_timer(t: *mut timer_list) {
    static void tc358743_irq_poll_timer(struct timer_list *t)
    {
    struct tc358743_state *state = timer_container_of(state, t, timer);
    unsigned int msecs;
    schedule_work(&state.work_i2c_poll);
//
// If CEC is present, then we need to poll more frequently,
// otherwise we will miss CEC messages.
//
    msecs = state.cec_adap ? POLL_INTERVAL_CEC_MS : POLL_INTERVAL_MS;
    mod_timer(&state.timer, jiffies + msecs_to_jiffies(msecs));
    }
#[no_mangle]
unsafe extern "C" fn tc358743_work_i2c_poll(work: *mut work_struct) {
    static void tc358743_work_i2c_poll(struct work_struct *work)
    {
    struct tc358743_state *state = container_of(work,
    struct tc358743_state, work_i2c_poll);
    bool handled;
    tc358743_isr(&state.sd, 0, &handled);
    }
    static int tc358743_subscribe_event(struct v4l2_subdev *sd, struct v4l2_fh *fh,
    struct v4l2_event_subscription *sub)
    {
    switch (sub.type) {
    case V4L2_EVENT_SOURCE_CHANGE:
    return v4l2_src_change_event_subdev_subscribe(sd, fh, sub);
    case V4L2_EVENT_CTRL:
    return v4l2_ctrl_subdev_subscribe_event(sd, fh, sub);
    default:
    return -EINVAL;
    }
    }
// --------------- VIDEO OPS ---------------
#[no_mangle]
unsafe extern "C" fn tc358743_g_input_status(sd: *mut v4l2_subdev, status: *mut u32) -> c_int {
    static int tc358743_g_input_status(struct v4l2_subdev *sd, u32 *status)
    {
// status = 0;
// status |= no_signal(sd) ? V4L2_IN_ST_NO_SIGNAL : 0;
// status |= no_sync(sd) ? V4L2_IN_ST_NO_SYNC : 0;
    v4l2_dbg(1, debug, sd, "%s: status = 0x%x\n", __func__, *status);
    return 0;
    }
    static int tc358743_s_dv_timings(struct v4l2_subdev *sd, unsigned int pad,
    struct v4l2_dv_timings *timings)
    {
    struct tc358743_state *state = to_state(sd);
    if (pad != 0)
    return -EINVAL;
    if (!timings)
    return -EINVAL;
    if (debug)
    v4l2_print_dv_timings(sd.name, "tc358743_s_dv_timings: ",
    timings, false);
    if (v4l2_match_dv_timings(&state.timings, timings, 0, false)) {
    v4l2_dbg(1, debug, sd, "%s: no change\n", __func__);
    return 0;
    }
    if (!v4l2_valid_dv_timings(timings,
    &tc358743_timings_cap, core::ptr::null_mut(), core::ptr::null_mut())) {
    v4l2_dbg(1, debug, sd, "%s: timings out of range\n", __func__);
    return -ERANGE;
    }
    state.timings = *timings;
    enable_stream(sd, false);
    tc358743_set_pll(sd);
    tc358743_set_csi(sd);
    return 0;
    }
    static int tc358743_g_dv_timings(struct v4l2_subdev *sd, unsigned int pad,
    struct v4l2_dv_timings *timings)
    {
    struct tc358743_state *state = to_state(sd);
    if (pad != 0)
    return -EINVAL;
// timings = state->timings;
    return 0;
    }
    static int tc358743_enum_dv_timings(struct v4l2_subdev *sd,
    struct v4l2_enum_dv_timings *timings)
    {
    if (timings.pad != 0)
    return -EINVAL;
    return v4l2_enum_dv_timings_cap(timings,
    &tc358743_timings_cap, core::ptr::null_mut(), core::ptr::null_mut());
    }
    static int tc358743_query_dv_timings(struct v4l2_subdev *sd, unsigned int pad,
    struct v4l2_dv_timings *timings)
    {
    int ret;
    if (pad != 0)
    return -EINVAL;
    ret = tc358743_get_detected_timings(sd, timings);
    if (ret)
    return ret;
    if (debug)
    v4l2_print_dv_timings(sd.name, "tc358743_query_dv_timings: ",
    timings, false);
    if (!v4l2_valid_dv_timings(timings,
    &tc358743_timings_cap, core::ptr::null_mut(), core::ptr::null_mut())) {
    v4l2_dbg(1, debug, sd, "%s: timings out of range\n", __func__);
    return -ERANGE;
    }
    return 0;
    }
    static int tc358743_dv_timings_cap(struct v4l2_subdev *sd,
    struct v4l2_dv_timings_cap *cap)
    {
    if (cap.pad != 0)
    return -EINVAL;
// cap = tc358743_timings_cap;
    return 0;
    }
    static int tc358743_get_mbus_config(struct v4l2_subdev *sd,
    unsigned int pad,
    struct v4l2_mbus_config *cfg)
    {
    struct tc358743_state *state = to_state(sd);
    cfg.type = V4L2_MBUS_CSI2_DPHY;
// Support for non-continuous CSI-2 clock is missing in the driver
    cfg.bus.mipi_csi2.flags = 0;
    cfg.bus.mipi_csi2.num_data_lanes = state.csi_lanes_in_use;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc358743_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int tc358743_s_stream(struct v4l2_subdev *sd, int enable)
    {
    enable_stream(sd, enable);
    if (!enable) {
// Put all lanes in LP-11 state (STOPSTATE)
    tc358743_set_csi(sd);
    }
    return 0;
    }
// --------------- PAD OPS ---------------
    static int tc358743_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    switch (code.index) {
    case 0:
    code.code = MEDIA_BUS_FMT_RGB888_1X24;
    break;
    case 1:
    code.code = MEDIA_BUS_FMT_UYVY8_1X16;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc358743_g_colorspace(code: u32) -> u32 {
    static u32 tc358743_g_colorspace(u32 code)
    {
    switch (code) {
    case MEDIA_BUS_FMT_RGB888_1X24:
    return V4L2_COLORSPACE_SRGB;
    case MEDIA_BUS_FMT_UYVY8_1X16:
    return V4L2_COLORSPACE_SMPTE170M;
    default:
    return 0;
    }
    }
    static int tc358743_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct tc358743_state *state = to_state(sd);
    if (format.pad != 0)
    return -EINVAL;
    format.format.code = state.mbus_fmt_code;
    format.format.width = state.timings.bt.width;
    format.format.height = state.timings.bt.height;
    format.format.field = V4L2_FIELD_NONE;
    format.format.colorspace = tc358743_g_colorspace(format.format.code);
    return 0;
    }
    static int tc358743_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct tc358743_state *state = to_state(sd);
    u32 code = format.format.code; /* is overwritten by get_fmt */
    let mut ret: c_int = tc358743_get_fmt(sd, sd_state, format);
    if (code == MEDIA_BUS_FMT_RGB888_1X24 ||
    code == MEDIA_BUS_FMT_UYVY8_1X16)
    format.format.code = code;
    format.format.colorspace = tc358743_g_colorspace(format.format.code);
    if (ret)
    return ret;
    if (format.which == V4L2_SUBDEV_FORMAT_TRY)
    return 0;
    state.mbus_fmt_code = format.format.code;
    enable_stream(sd, false);
    tc358743_set_pll(sd);
    tc358743_set_csi(sd);
    tc358743_set_csi_color_space(sd);
    return 0;
    }
    static int tc358743_g_edid(struct v4l2_subdev *sd,
    struct v4l2_subdev_edid *edid)
    {
    struct tc358743_state *state = to_state(sd);
    memset(edid.reserved, 0, sizeof(edid.reserved));
    if (edid.pad != 0)
    return -EINVAL;
    if (edid.start_block == 0 && edid.blocks == 0) {
    edid.blocks = state.edid_blocks_written;
    return 0;
    }
    if (state.edid_blocks_written == 0)
    return -ENODATA;
    if (edid.start_block >= state.edid_blocks_written ||
    edid.blocks == 0)
    return -EINVAL;
    if (edid.start_block + edid.blocks > state.edid_blocks_written)
    edid.blocks = state.edid_blocks_written - edid.start_block;
    i2c_rd(sd, EDID_RAM + (edid.start_block * EDID_BLOCK_SIZE), edid.edid,
    edid.blocks * EDID_BLOCK_SIZE);
    return 0;
    }
    static int tc358743_s_edid(struct v4l2_subdev *sd,
    struct v4l2_subdev_edid *edid)
    {
    struct tc358743_state *state = to_state(sd);
    let mut edid_len: u16 = edid.blocks * EDID_BLOCK_SIZE;
    u16 pa;
    int err;
    int i;
    v4l2_dbg(2, debug, sd, "%s, pad %d, start block %d, blocks %d\n",
    __func__, edid.pad, edid.start_block, edid.blocks);
    memset(edid.reserved, 0, sizeof(edid.reserved));
    if (edid.pad != 0)
    return -EINVAL;
    if (edid.start_block != 0)
    return -EINVAL;
    if (edid.blocks > EDID_NUM_BLOCKS_MAX) {
    edid.blocks = EDID_NUM_BLOCKS_MAX;
    return -E2BIG;
    }
    pa = cec_get_edid_phys_addr(edid.edid, edid.blocks * 128, core::ptr::null_mut());
    err = v4l2_phys_addr_validate(pa, &pa, core::ptr::null_mut());
    if (err)
    return err;
    cec_phys_addr_invalidate(state.cec_adap);
    tc358743_disable_edid(sd);
    i2c_wr8(sd, EDID_LEN1, edid_len & 0xff);
    i2c_wr8(sd, EDID_LEN2, edid_len >> 8);
    if (edid.blocks == 0) {
    state.edid_blocks_written = 0;
    return 0;
    }
    for (i = 0; i < edid_len; i += EDID_BLOCK_SIZE)
    i2c_wr(sd, EDID_RAM + i, edid.edid + i, EDID_BLOCK_SIZE);
    state.edid_blocks_written = edid.blocks;
    cec_s_phys_addr(state.cec_adap, pa, false);
    if (tx_5v_power_present(sd))
    tc358743_enable_edid(sd);
    return 0;
    }
// --------------------------------------------------------------------------
    static const struct v4l2_subdev_core_ops tc358743_core_ops = {
    .log_status = tc358743_log_status,

    .g_register = tc358743_g_register,
    .s_register = tc358743_s_register,

    .interrupt_service_routine = tc358743_isr,
    .subscribe_event = tc358743_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,
    };
    static const struct v4l2_subdev_video_ops tc358743_video_ops = {
    .g_input_status = tc358743_g_input_status,
    .s_stream = tc358743_s_stream,
    };
    static const struct v4l2_subdev_pad_ops tc358743_pad_ops = {
    .enum_mbus_code = tc358743_enum_mbus_code,
    .set_fmt = tc358743_set_fmt,
    .get_fmt = tc358743_get_fmt,
    .get_edid = tc358743_g_edid,
    .set_edid = tc358743_s_edid,
    .s_dv_timings = tc358743_s_dv_timings,
    .g_dv_timings = tc358743_g_dv_timings,
    .query_dv_timings = tc358743_query_dv_timings,
    .enum_dv_timings = tc358743_enum_dv_timings,
    .dv_timings_cap = tc358743_dv_timings_cap,
    .get_mbus_config = tc358743_get_mbus_config,
    };
    static const struct v4l2_subdev_ops tc358743_ops = {
    .core = &tc358743_core_ops,
    .video = &tc358743_video_ops,
    .pad = &tc358743_pad_ops,
    };
// --------------- CUSTOM CTRLS ---------------
    static const struct v4l2_ctrl_config tc358743_ctrl_audio_sampling_rate = {
    .id = TC358743_CID_AUDIO_SAMPLING_RATE,
    .name = "Audio sampling rate",
    .type = V4L2_CTRL_TYPE_INTEGER,
    .min = 0,
    .max = 768000,
    .step = 1,
    .def = 0,
    .flags = V4L2_CTRL_FLAG_READ_ONLY,
    };
    static const struct v4l2_ctrl_config tc358743_ctrl_audio_present = {
    .id = TC358743_CID_AUDIO_PRESENT,
    .name = "Audio present",
    .type = V4L2_CTRL_TYPE_BOOLEAN,
    .min = 0,
    .max = 1,
    .step = 1,
    .def = 0,
    .flags = V4L2_CTRL_FLAG_READ_ONLY,
    };
// --------------- PROBE / REMOVE ---------------

#[no_mangle]
unsafe extern "C" fn tc358743_gpio_reset(state: *mut tc358743_state) {
    static void tc358743_gpio_reset(struct tc358743_state *state)
    {
    usleep_range(5000, 10000);
    gpiod_set_value(state.reset_gpio, 1);
    usleep_range(1000, 2000);
    gpiod_set_value(state.reset_gpio, 0);
    msleep(20);
    }
#[no_mangle]
unsafe extern "C" fn tc358743_probe_of(state: *mut tc358743_state) -> c_int {
    static int tc358743_probe_of(struct tc358743_state *state)
    {
    struct device *dev = &state.i2c_client.dev;
    let mut endpoint: v4l2_fwnode_endpoint = { .bus_type = 0 };
    struct device_node *ep;
    struct clk *refclk;
    u32 bps_pr_lane;
    int ret;
    refclk = devm_clk_get(dev, "refclk");
    if (IS_ERR(refclk))
    return dev_err_probe(dev, PTR_ERR(refclk),
    "failed to get refclk\n");
    ep = of_graph_get_endpoint_by_regs(dev.of_node, 0, -1);
    if (!ep) {
    dev_err(dev, "missing endpoint node\n");
    return -EINVAL;
    }
    ret = v4l2_fwnode_endpoint_alloc_parse(of_fwnode_handle(ep), &endpoint);
    if (ret) {
    dev_err(dev, "failed to parse endpoint\n");
    goto put_node;
    }
    if (endpoint.bus_type != V4L2_MBUS_CSI2_DPHY ||
    endpoint.bus.mipi_csi2.num_data_lanes == 0 ||
    endpoint.nr_of_link_frequencies == 0) {
    dev_err(dev, "missing CSI-2 properties in endpoint\n");
    ret = -EINVAL;
    goto free_endpoint;
    }
    if (endpoint.bus.mipi_csi2.num_data_lanes > 4) {
    dev_err(dev, "invalid number of lanes\n");
    ret = -EINVAL;
    goto free_endpoint;
    }
    state.bus = endpoint.bus.mipi_csi2;
    ret = clk_prepare_enable(refclk);
    if (ret) {
    dev_err(dev, "Failed! to enable clock\n");
    goto free_endpoint;
    }
    state.pdata.refclk_hz = clk_get_rate(refclk);
    state.pdata.ddc5v_delay = DDC5V_DELAY_100_MS;
    state.pdata.enable_hdcp = false;
//
// Ideally the FIFO trigger level should be set based on the input and
// output data rates, but the calculations required are buried in
// Toshiba's register settings spreadsheet.
// A value of 16 works with a 594Mbps data rate for 720p60 (using 2
// lanes) and 1080p60 (using 4 lanes), but fails when the data rate
// is increased, or a lower pixel clock is used that result in CSI
// reading out faster than the data is arriving.
//
// A value of 374 works with both those modes at 594Mbps, and with most
// modes on 972Mbps.
//
    state.pdata.fifo_level = 374;
//
// The PLL input clock is obtained by dividing refclk by pll_prd.
// It must be between 6 MHz and 40 MHz, lower frequency is better.
//
    switch (state.pdata.refclk_hz) {
    case 26000000:
    case 27000000:
    case 42000000:
    state.pdata.pll_prd = state.pdata.refclk_hz / 6000000;
    break;
    default:
    dev_err(dev, "unsupported refclk rate: %u Hz\n",
    state.pdata.refclk_hz);
    goto disable_clk;
    }
//
// The CSI bps per lane must be between 62.5 Mbps and 1 Gbps.
// The default is 594 Mbps for 4-lane 1080p60 or 2-lane 720p60.
// 972 Mbps allows 1080P50 UYVY over 2-lane.
//
    bps_pr_lane = 2 * endpoint.link_frequencies[0];
    if (bps_pr_lane < 62500000U || bps_pr_lane > 1000000000U) {
    dev_err(dev, "unsupported bps per lane: %u bps\n", bps_pr_lane);
    ret = -EINVAL;
    goto disable_clk;
    }
// The CSI speed per lane is refclk / pll_prd * pll_fbd
    state.pdata.pll_fbd = bps_pr_lane /
    state.pdata.refclk_hz * state.pdata.pll_prd;
//
// FIXME: These timings are from REF_02 for 594 or 972 Mbps per lane
// (297 MHz or 486 MHz link frequency).
// In principle it should be possible to calculate
// them based on link frequency and resolution.
//
    switch (bps_pr_lane) {
    default:
    dev_warn(dev, "untested bps per lane: %u bps\n", bps_pr_lane);
    fallthrough;
    case 594000000U:
    state.pdata.lineinitcnt = 0xe80;
    state.pdata.lptxtimecnt = 0x003;
// tclk-preparecnt: 3, tclk-zerocnt: 20
    state.pdata.tclk_headercnt = 0x1403;
    state.pdata.tclk_trailcnt = 0x00;
// ths-preparecnt: 3, ths-zerocnt: 1
    state.pdata.ths_headercnt = 0x0103;
    state.pdata.twakeup = 0x4882;
    state.pdata.tclk_postcnt = 0x008;
    state.pdata.ths_trailcnt = 0x2;
    state.pdata.hstxvregcnt = 0;
    break;
    case 972000000U:
    state.pdata.lineinitcnt = 0x1b58;
    state.pdata.lptxtimecnt = 0x007;
// tclk-preparecnt: 6, tclk-zerocnt: 40
    state.pdata.tclk_headercnt = 0x2806;
    state.pdata.tclk_trailcnt = 0x00;
// ths-preparecnt: 6, ths-zerocnt: 8
    state.pdata.ths_headercnt = 0x0806;
    state.pdata.twakeup = 0x4268;
    state.pdata.tclk_postcnt = 0x008;
    state.pdata.ths_trailcnt = 0x5;
    state.pdata.hstxvregcnt = 0;
    break;
    }
    state.reset_gpio = devm_gpiod_get_optional(dev, "reset",
    GPIOD_OUT_LOW);
    if (IS_ERR(state.reset_gpio)) {
    dev_err(dev, "failed to get reset gpio\n");
    ret = PTR_ERR(state.reset_gpio);
    goto disable_clk;
    }
    if (state.reset_gpio)
    tc358743_gpio_reset(state);
    ret = 0;
    goto free_endpoint;
    disable_clk:
    clk_disable_unprepare(refclk);
    free_endpoint:
    v4l2_fwnode_endpoint_free(&endpoint);
    put_node:
    of_node_put(ep);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn tc358743_probe_of(state: *mut tc358743_state) -> c_int {
    static inline int tc358743_probe_of(struct tc358743_state *state)
    {
    return -ENODEV;
    }

#[no_mangle]
unsafe extern "C" fn tc358743_probe(client: *mut i2c_client) -> c_int {
    static int tc358743_probe(struct i2c_client *client)
    {
    static struct v4l2_dv_timings default_timing =
    V4L2_DV_BT_CEA_640X480P59_94;
    struct tc358743_state *state;
    struct tc358743_platform_data *pdata = client.dev.platform_data;
    struct v4l2_subdev *sd;
    let mut irq_mask: u16 = MASK_HDMI_MSK | MASK_CSI_MSK;
    u16 chipid;
    int err;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    v4l_dbg(1, debug, client, "chip found @ 0x%x (%s)\n",
    client.addr << 1, client.adapter.name);
    state = devm_kzalloc(&client.dev, sizeof(struct tc358743_state),
    GFP_KERNEL);
    if (!state)
    return -ENOMEM;
    state.i2c_client = client;
// platform data
    if (pdata) {
    state.pdata = *pdata;
    state.bus.flags = 0;
    } else {
    err = tc358743_probe_of(state);
    if (err == -ENODEV)
    v4l_err(client, "No platform data!\n");
    if (err)
    return err;
    }
    sd = &state.sd;
    v4l2_i2c_subdev_init(sd, client, &tc358743_ops);
    sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE | V4L2_SUBDEV_FL_HAS_EVENTS;
// i2c access
    if (i2c_rd16_err(sd, CHIPID, &chipid) ||
    (chipid & MASK_CHIPID) != 0) {
    v4l2_info(sd, "not a TC358743 on address 0x%x\n",
    client.addr << 1);
    return -ENODEV;
    }
// control handlers
    v4l2_ctrl_handler_init(&state.hdl, 3);
    state.detect_tx_5v_ctrl = v4l2_ctrl_new_std(&state.hdl, core::ptr::null_mut(),
    V4L2_CID_DV_RX_POWER_PRESENT, 0, 1, 0, 0);
// custom controls
    state.audio_sampling_rate_ctrl = v4l2_ctrl_new_custom(&state.hdl,
    &tc358743_ctrl_audio_sampling_rate, core::ptr::null_mut());
    state.audio_present_ctrl = v4l2_ctrl_new_custom(&state.hdl,
    &tc358743_ctrl_audio_present, core::ptr::null_mut());
    sd.ctrl_handler = &state.hdl;
    if (state.hdl.error) {
    err = state.hdl.error;
    goto err_hdl;
    }
    if (tc358743_update_controls(sd)) {
    err = -ENODEV;
    goto err_hdl;
    }
    state.pad.flags = MEDIA_PAD_FL_SOURCE;
    sd.entity.function = MEDIA_ENT_F_VID_IF_BRIDGE;
    err = media_entity_pads_init(&sd.entity, 1, &state.pad);
    if (err < 0)
    goto err_hdl;
    state.mbus_fmt_code = MEDIA_BUS_FMT_RGB888_1X24;
    sd.dev = &client.dev;
    mutex_init(&state.confctl_mutex);
    INIT_DELAYED_WORK(&state.delayed_work_enable_hotplug,
    tc358743_delayed_work_enable_hotplug);

    state.cec_adap = cec_allocate_adapter(&tc358743_cec_adap_ops,
    state, dev_name(&client.dev),
    CEC_CAP_DEFAULTS | CEC_CAP_MONITOR_ALL, CEC_MAX_LOG_ADDRS);
    if (IS_ERR(state.cec_adap)) {
    err = PTR_ERR(state.cec_adap);
    goto err_hdl;
    }
    irq_mask |= MASK_CEC_RMSK | MASK_CEC_TMSK;

    tc358743_initial_setup(sd);
    tc358743_s_dv_timings(sd, 0, &default_timing);
    tc358743_set_csi_color_space(sd);
    tc358743_init_interrupts(sd);
    if (state.i2c_client.irq) {
    err = devm_request_threaded_irq(&client.dev,
    state.i2c_client.irq,
    core::ptr::null_mut(), tc358743_irq_handler,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
    "tc358743", state);
    if (err)
    goto err_work_queues;
    } else {
    INIT_WORK(&state.work_i2c_poll,
    tc358743_work_i2c_poll);
    timer_setup(&state.timer, tc358743_irq_poll_timer, 0);
    state.timer.expires = jiffies +
    msecs_to_jiffies(POLL_INTERVAL_MS);
    add_timer(&state.timer);
    }
    err = cec_register_adapter(state.cec_adap, &client.dev);
    if (err < 0) {
    pr_err("%s: failed to register the cec device\n", __func__);
    cec_delete_adapter(state.cec_adap);
    state.cec_adap = core::ptr::null_mut();
    goto err_work_queues;
    }
    tc358743_enable_interrupts(sd, tx_5v_power_present(sd));
    i2c_wr16(sd, INTMASK, ~irq_mask);
    err = v4l2_ctrl_handler_setup(sd.ctrl_handler);
    if (err)
    goto err_work_queues;
    err = v4l2_async_register_subdev(sd);
    if (err < 0)
    goto err_work_queues;
    i2c_wr8(sd, TYP_ACP_SET, packet_type);
    i2c_wr8(sd, PK_AUTO_CLR, 0xff);
    i2c_wr8(sd, NO_PKT_CLR, MASK_NO_ACP_CLR);
    state.debugfs_dir = debugfs_create_dir(sd.name, v4l2_debugfs_root());
    state.infoframes = v4l2_debugfs_if_alloc(state.debugfs_dir,
    V4L2_DEBUGFS_IF_AVI | V4L2_DEBUGFS_IF_AUDIO |
    V4L2_DEBUGFS_IF_SPD | V4L2_DEBUGFS_IF_HDMI |
    V4L2_DEBUGFS_IF_DRM, sd, tc358743_debugfs_if_read);
    v4l2_info(sd, "%s found @ 0x%x (%s)\n", client.name,
    client.addr << 1, client.adapter.name);
    return 0;
    err_work_queues:
    cec_unregister_adapter(state.cec_adap);
    if (!state.i2c_client.irq) {
    timer_delete_sync(&state.timer);
    flush_work(&state.work_i2c_poll);
    }
    cancel_delayed_work_sync(&state.delayed_work_enable_hotplug);
    mutex_destroy(&state.confctl_mutex);
    err_hdl:
    media_entity_cleanup(&sd.entity);
    v4l2_ctrl_handler_free(&state.hdl);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tc358743_remove(client: *mut i2c_client) {
    static void tc358743_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct tc358743_state *state = to_state(sd);
    if (!state.i2c_client.irq) {
    timer_delete_sync(&state.timer);
    flush_work(&state.work_i2c_poll);
    }
    cancel_delayed_work_sync(&state.delayed_work_enable_hotplug);
    v4l2_debugfs_if_free(state.infoframes);
    debugfs_remove_recursive(state.debugfs_dir);
    cec_unregister_adapter(state.cec_adap);
    v4l2_async_unregister_subdev(sd);
    v4l2_device_unregister_subdev(sd);
    mutex_destroy(&state.confctl_mutex);
    media_entity_cleanup(&sd.entity);
    v4l2_ctrl_handler_free(&state.hdl);
    }
    static const struct i2c_device_id tc358743_id[] = {
    { .name = "tc358743" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tc358743_id);

    static const struct of_device_id tc358743_of_match[] = {
    { .compatible = "toshiba,tc358743" },
    {},
    };
    MODULE_DEVICE_TABLE(of, tc358743_of_match);

    static struct i2c_driver tc358743_driver = {
    .driver = {
    .name = "tc358743",
    .of_match_table = of_match_ptr(tc358743_of_match),
    },
    .probe = tc358743_probe,
    .remove = tc358743_remove,
    .id_table = tc358743_id,
    };
    module_i2c_driver(tc358743_driver);
