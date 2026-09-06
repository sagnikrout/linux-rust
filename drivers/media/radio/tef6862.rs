//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/tef6862.c
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
// tef6862.c Philips TEF6862 Car Radio Enhanced Selectivity Tuner
// Copyright (c) 2009 Intel Corporation
//

pub const FREQ_MUL: c_int = 16000;

// Write mode sub addresses
pub const WM_SUB_BANDWIDTH: c_uint = 0x0;
pub const WM_SUB_PLLM: c_uint = 0x1;
pub const WM_SUB_PLLL: c_uint = 0x2;
pub const WM_SUB_DAA: c_uint = 0x3;
pub const WM_SUB_AGC: c_uint = 0x4;
pub const WM_SUB_BAND: c_uint = 0x5;
pub const WM_SUB_CONTROL: c_uint = 0x6;
pub const WM_SUB_LEVEL: c_uint = 0x7;
pub const WM_SUB_IFCF: c_uint = 0x8;
pub const WM_SUB_IFCAP: c_uint = 0x9;
pub const WM_SUB_ACD: c_uint = 0xA;
pub const WM_SUB_TEST: c_uint = 0xF;
// Different modes of the MSA register
pub const MSA_MODE_BUFFER: c_uint = 0x0;
pub const MSA_MODE_PRESET: c_uint = 0x1;
pub const MSA_MODE_SEARCH: c_uint = 0x2;
pub const MSA_MODE_AF_UPDATE: c_uint = 0x3;
pub const MSA_MODE_JUMP: c_uint = 0x4;
pub const MSA_MODE_CHECK: c_uint = 0x5;
pub const MSA_MODE_LOAD: c_uint = 0x6;
pub const MSA_MODE_END: c_uint = 0x7;
pub const MSA_MODE_SHIFT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tef6862_state {
    pub sd: v4l2_subdev,
    pub freq: c_ulong,
}

    static inline struct tef6862_state *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct tef6862_state, sd);
    }
#[no_mangle]
unsafe extern "C" fn tef6862_sigstr(client: *mut i2c_client) -> u16 {
    static u16 tef6862_sigstr(struct i2c_client *client)
    {
    u8 buf[4];
    let mut err: c_int = i2c_master_recv(client, buf, sizeof(buf));
    if (err == sizeof(buf))
    return buf[3] << 8;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tef6862_g_tuner(sd: *mut v4l2_subdev, v: *mut v4l2_tuner) -> c_int {
    static int tef6862_g_tuner(struct v4l2_subdev *sd, struct v4l2_tuner *v)
    {
    if (v.index > 0)
    return -EINVAL;
// only support FM for now
    strscpy(v.name, "FM", sizeof(v.name));
    v.type = V4L2_TUNER_RADIO;
    v.rangelow = TEF6862_LO_FREQ;
    v.rangehigh = TEF6862_HI_FREQ;
    v.rxsubchans = V4L2_TUNER_SUB_MONO;
    v.capability = V4L2_TUNER_CAP_LOW;
    v.audmode = V4L2_TUNER_MODE_STEREO;
    v.signal = tef6862_sigstr(v4l2_get_subdevdata(sd));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tef6862_s_tuner(sd: *mut v4l2_subdev, v: *const v4l2_tuner) -> c_int {
    static int tef6862_s_tuner(struct v4l2_subdev *sd, const struct v4l2_tuner *v)
    {
    return v.index ? -EINVAL : 0;
    }
#[no_mangle]
unsafe extern "C" fn tef6862_s_frequency(sd: *mut v4l2_subdev, f: *const v4l2_frequency) -> c_int {
    static int tef6862_s_frequency(struct v4l2_subdev *sd, const struct v4l2_frequency *f)
    {
    struct tef6862_state *state = to_state(sd);
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    let mut freq: unsigned = f.frequency;
    u16 pll;
    u8 i2cmsg[3];
    int err;
    if (f.tuner != 0)
    return -EINVAL;
    freq = clamp(freq, TEF6862_LO_FREQ, TEF6862_HI_FREQ);
    pll = 1964 + ((freq - TEF6862_LO_FREQ) * 20) / FREQ_MUL;
    i2cmsg[0] = (MSA_MODE_PRESET << MSA_MODE_SHIFT) | WM_SUB_PLLM;
    i2cmsg[1] = (pll >> 8) & 0xff;
    i2cmsg[2] = pll & 0xff;
    err = i2c_master_send(client, i2cmsg, sizeof(i2cmsg));
    if (err != sizeof(i2cmsg))
    return err < 0 ? err : -EIO;
    state.freq = freq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tef6862_g_frequency(sd: *mut v4l2_subdev, f: *mut v4l2_frequency) -> c_int {
    static int tef6862_g_frequency(struct v4l2_subdev *sd, struct v4l2_frequency *f)
    {
    struct tef6862_state *state = to_state(sd);
    if (f.tuner != 0)
    return -EINVAL;
    f.type = V4L2_TUNER_RADIO;
    f.frequency = state.freq;
    return 0;
    }
    static const struct v4l2_subdev_tuner_ops tef6862_tuner_ops = {
    .g_tuner = tef6862_g_tuner,
    .s_tuner = tef6862_s_tuner,
    .s_frequency = tef6862_s_frequency,
    .g_frequency = tef6862_g_frequency,
    };
    static const struct v4l2_subdev_ops tef6862_ops = {
    .tuner = &tef6862_tuner_ops,
    };
//
// Generic i2c probe
// concerning the addresses: i2c wants 7 bit (without the r/w bit), so '>>1'
//
#[no_mangle]
unsafe extern "C" fn tef6862_probe(client: *mut i2c_client) -> c_int {
    static int tef6862_probe(struct i2c_client *client)
    {
    struct tef6862_state *state;
    struct v4l2_subdev *sd;
// Check if the adapter supports the needed features
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    v4l_info(client, "chip found @ 0x%02x (%s)\n",
    client.addr << 1, client.adapter.name);
    state = kzalloc_obj(struct tef6862_state);
    if (state == core::ptr::null_mut())
    return -ENOMEM;
    state.freq = TEF6862_LO_FREQ;
    sd = &state.sd;
    v4l2_i2c_subdev_init(sd, client, &tef6862_ops);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tef6862_remove(client: *mut i2c_client) {
    static void tef6862_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    v4l2_device_unregister_subdev(sd);
    kfree(to_state(sd));
    }
    static const struct i2c_device_id tef6862_id[] = {
    { .name = DRIVER_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tef6862_id);
    static struct i2c_driver tef6862_driver = {
    .driver = {
    .name	= DRIVER_NAME,
    },
    .probe		= tef6862_probe,
    .remove		= tef6862_remove,
    .id_table	= tef6862_id,
    };
    module_i2c_driver(tef6862_driver);
    MODULE_DESCRIPTION("TEF6862 Car Radio Enhanced Selectivity Tuner");
    MODULE_AUTHOR("Mocean Laboratories");
    MODULE_LICENSE("GPL v2");
