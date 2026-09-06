//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/wm8739.c
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
// wm8739
//
// Copyright (C) 2005 T. Adachi <tadachi@tadachi-net.com>
//
// Copyright (C) 2005 Hans Verkuil <hverkuil@kernel.org>
// - Cleanup
//

    MODULE_DESCRIPTION("wm8739 driver");
    MODULE_AUTHOR("T. Adachi, Hans Verkuil");
    MODULE_LICENSE("GPL");
    static int debug;
    module_param(debug, int, 0644);
    MODULE_PARM_DESC(debug, "Debug level (0-1)");
// ------------------------------------------------------------------------
    enum {
    R0 = 0, R1,
    R5 = 5, R6, R7, R8, R9, R15 = 15,
    TOT_REGS
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8739_state {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    struct {
// audio cluster
    pub volume: *mut v4l2_ctrl,
    pub mute: *mut v4l2_ctrl,
    pub balance: *mut v4l2_ctrl,
}

    u32 clock_freq;
    };
    static inline struct wm8739_state *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct wm8739_state, sd);
    }
    static inline struct v4l2_subdev *to_sd(struct v4l2_ctrl *ctrl)
    {
    return &container_of(ctrl.handler, struct wm8739_state, hdl).sd;
    }
// ------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn wm8739_write(sd: *mut v4l2_subdev, reg: c_int, val: u16) -> c_int {
    static int wm8739_write(struct v4l2_subdev *sd, int reg, u16 val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    int i;
    if (reg < 0 || reg >= TOT_REGS) {
    v4l2_err(sd, "Invalid register R%d\n", reg);
    return -1;
    }
    v4l2_dbg(1, debug, sd, "write: %02x %02x\n", reg, val);
    for (i = 0; i < 3; i++)
    if (i2c_smbus_write_byte_data(client,
    (reg << 1) | (val >> 8), val & 0xff) == 0)
    return 0;
    v4l2_err(sd, "I2C: cannot write %03x to register R%d\n", val, reg);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn wm8739_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int wm8739_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *sd = to_sd(ctrl);
    struct wm8739_state *state = to_state(sd);
    unsigned int work_l, work_r;
    u8 vol_l;	/* +12dB to -34.5dB 1.5dB step (5bit) def:0dB */
    u8 vol_r;	/* +12dB to -34.5dB 1.5dB step (5bit) def:0dB */
    u16 mute;
    switch (ctrl.id) {
    case V4L2_CID_AUDIO_VOLUME:
    break;
    default:
    return -EINVAL;
    }
// normalize ( 65535 to 0 -> 31 to 0 (12dB to -34.5dB) )
    work_l = (min(65536 - state.balance.val, 32768) * state.volume.val) / 32768;
    work_r = (min(state.balance.val, 32768) * state.volume.val) / 32768;
    vol_l = (long)work_l * 31 / 65535;
    vol_r = (long)work_r * 31 / 65535;
// set audio volume etc.
    mute = state.mute.val ? 0x80 : 0;
// Volume setting: bits 0-4, 0x1f = 12 dB, 0x00 = -34.5 dB
// Default setting: 0x17 = 0 dB
//
    wm8739_write(sd, R0, (vol_l & 0x1f) | mute);
    wm8739_write(sd, R1, (vol_r & 0x1f) | mute);
    return 0;
    }
// ------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn wm8739_s_clock_freq(sd: *mut v4l2_subdev, audiofreq: u32) -> c_int {
    static int wm8739_s_clock_freq(struct v4l2_subdev *sd, u32 audiofreq)
    {
    struct wm8739_state *state = to_state(sd);
    state.clock_freq = audiofreq;
// de-activate
    wm8739_write(sd, R9, 0x000);
    switch (audiofreq) {
    case 44100:
// 256fps, fs=44.1k
    wm8739_write(sd, R8, 0x020);
    break;
    case 48000:
// 256fps, fs=48k
    wm8739_write(sd, R8, 0x000);
    break;
    case 32000:
// 256fps, fs=32k
    wm8739_write(sd, R8, 0x018);
    break;
    default:
    break;
    }
// activate
    wm8739_write(sd, R9, 0x001);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wm8739_log_status(sd: *mut v4l2_subdev) -> c_int {
    static int wm8739_log_status(struct v4l2_subdev *sd)
    {
    struct wm8739_state *state = to_state(sd);
    v4l2_info(sd, "Frequency: %u Hz\n", state.clock_freq);
    v4l2_ctrl_handler_log_status(&state.hdl, sd.name);
    return 0;
    }
// -----------------------------------------------------------------------
    static const struct v4l2_ctrl_ops wm8739_ctrl_ops = {
    .s_ctrl = wm8739_s_ctrl,
    };
    static const struct v4l2_subdev_core_ops wm8739_core_ops = {
    .log_status = wm8739_log_status,
    };
    static const struct v4l2_subdev_audio_ops wm8739_audio_ops = {
    .s_clock_freq = wm8739_s_clock_freq,
    };
    static const struct v4l2_subdev_ops wm8739_ops = {
    .core = &wm8739_core_ops,
    .audio = &wm8739_audio_ops,
    };
// ------------------------------------------------------------------------
// i2c implementation
#[no_mangle]
unsafe extern "C" fn wm8739_probe(client: *mut i2c_client) -> c_int {
    static int wm8739_probe(struct i2c_client *client)
    {
    struct wm8739_state *state;
    struct v4l2_subdev *sd;
// Check if the adapter supports the needed features
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    v4l_info(client, "chip found @ 0x%x (%s)\n",
    client.addr << 1, client.adapter.name);
    state = devm_kzalloc(&client.dev, sizeof(*state), GFP_KERNEL);
    if (state == core::ptr::null_mut())
    return -ENOMEM;
    sd = &state.sd;
    v4l2_i2c_subdev_init(sd, client, &wm8739_ops);
    v4l2_ctrl_handler_init(&state.hdl, 2);
    state.volume = v4l2_ctrl_new_std(&state.hdl, &wm8739_ctrl_ops,
    V4L2_CID_AUDIO_VOLUME, 0, 65535, 65535 / 100, 50736);
    state.mute = v4l2_ctrl_new_std(&state.hdl, &wm8739_ctrl_ops,
    V4L2_CID_AUDIO_MUTE, 0, 1, 1, 0);
    state.balance = v4l2_ctrl_new_std(&state.hdl, &wm8739_ctrl_ops,
    V4L2_CID_AUDIO_BALANCE, 0, 65535, 65535 / 100, 32768);
    sd.ctrl_handler = &state.hdl;
    if (state.hdl.error) {
    let mut err: c_int = state.hdl.error;
    v4l2_ctrl_handler_free(&state.hdl);
    return err;
    }
    v4l2_ctrl_cluster(3, &state.volume);
    state.clock_freq = 48000;
// Initialize wm8739
// reset
    wm8739_write(sd, R15, 0x00);
// filter setting, high path, offet clear
    wm8739_write(sd, R5, 0x000);
// ADC, OSC, Power Off mode Disable
    wm8739_write(sd, R6, 0x000);
// Digital Audio interface format:
    Enable Master mode, 24 bit, MSB first/left justified */
    wm8739_write(sd, R7, 0x049);
// sampling control: normal, 256fs, 48KHz sampling rate
    wm8739_write(sd, R8, 0x000);
// activate
    wm8739_write(sd, R9, 0x001);
// set volume/mute
    v4l2_ctrl_handler_setup(&state.hdl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wm8739_remove(client: *mut i2c_client) {
    static void wm8739_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct wm8739_state *state = to_state(sd);
    v4l2_device_unregister_subdev(sd);
    v4l2_ctrl_handler_free(&state.hdl);
    }
    static const struct i2c_device_id wm8739_id[] = {
    { .name = "wm8739" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, wm8739_id);
    static struct i2c_driver wm8739_driver = {
    .driver = {
    .name	= "wm8739",
    },
    .probe		= wm8739_probe,
    .remove		= wm8739_remove,
    .id_table	= wm8739_id,
    };
    module_i2c_driver(wm8739_driver);
