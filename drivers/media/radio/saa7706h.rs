//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/saa7706h.c
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
// saa7706.c Philips SAA7706H Car Radio DSP driver
// Copyright (c) 2009 Intel Corporation
//

// the I2C memory map looks like this
    $1C00 - $FFFF Not Used
    $2200 - $3FFF Reserved YRAM (DSP2) space
    $2000 - $21FF YRAM (DSP2)
    $1FF0 - $1FFF Hardware Registers
    $1280 - $1FEF Reserved XRAM (DSP2) space
    $1000 - $127F XRAM (DSP2)
    $0FFF        DSP CONTROL
    $0A00 - $0FFE Reserved
    $0980 - $09FF Reserved YRAM (DSP1) space
    $0800 - $097F YRAM (DSP1)
    $0200 - $07FF Not Used
    $0180 - $01FF Reserved XRAM (DSP1) space
    $0000 - $017F XRAM (DSP1)
//
pub const SAA7706H_REG_CTRL: c_uint = 0x0fff;
pub const SAA7706H_CTRL_BYP_PLL: c_uint = 0x0001;
pub const SAA7706H_CTRL_PLL_DIV_MASK: c_uint = 0x003e;
pub const SAA7706H_CTRL_PLL3_62975MHZ: c_uint = 0x003e;
pub const SAA7706H_CTRL_DSP_TURBO: c_uint = 0x0040;
pub const SAA7706H_CTRL_PC_RESET_DSP1: c_uint = 0x0080;
pub const SAA7706H_CTRL_PC_RESET_DSP2: c_uint = 0x0100;
pub const SAA7706H_CTRL_DSP1_ROM_EN_MASK: c_uint = 0x0600;
pub const SAA7706H_CTRL_DSP1_FUNC_PROM: c_uint = 0x0000;
pub const SAA7706H_CTRL_DSP2_ROM_EN_MASK: c_uint = 0x1800;
pub const SAA7706H_CTRL_DSP2_FUNC_PROM: c_uint = 0x0000;
pub const SAA7706H_CTRL_DIG_SIL_INTERPOL: c_uint = 0x8000;
pub const SAA7706H_REG_EVALUATION: c_uint = 0x1ff0;
pub const SAA7706H_EVAL_DISABLE_CHARGE_PUMP: c_uint = 0x000001;
pub const SAA7706H_EVAL_DCS_CLOCK: c_uint = 0x000002;
pub const SAA7706H_EVAL_GNDRC1_ENABLE: c_uint = 0x000004;
pub const SAA7706H_EVAL_GNDRC2_ENABLE: c_uint = 0x000008;
pub const SAA7706H_REG_CL_GEN1: c_uint = 0x1ff3;
pub const SAA7706H_CL_GEN1_MIN_LOOPGAIN_MASK: c_uint = 0x00000f;
pub const SAA7706H_CL_GEN1_LOOPGAIN_MASK: c_uint = 0x0000f0;
pub const SAA7706H_CL_GEN1_COARSE_RATION: c_uint = 0xffff00;
pub const SAA7706H_REG_CL_GEN2: c_uint = 0x1ff4;
pub const SAA7706H_CL_GEN2_WSEDGE_FALLING: c_uint = 0x000001;
pub const SAA7706H_CL_GEN2_STOP_VCO: c_uint = 0x000002;
pub const SAA7706H_CL_GEN2_FRERUN: c_uint = 0x000004;
pub const SAA7706H_CL_GEN2_ADAPTIVE: c_uint = 0x000008;
pub const SAA7706H_CL_GEN2_FINE_RATIO_MASK: c_uint = 0x0ffff0;
pub const SAA7706H_REG_CL_GEN4: c_uint = 0x1ff6;
pub const SAA7706H_CL_GEN4_BYPASS_PLL1: c_uint = 0x001000;
pub const SAA7706H_CL_GEN4_PLL1_DIV_MASK: c_uint = 0x03e000;
pub const SAA7706H_CL_GEN4_DSP1_TURBO: c_uint = 0x040000;
pub const SAA7706H_REG_SEL: c_uint = 0x1ff7;
pub const SAA7706H_SEL_DSP2_SRCA_MASK: c_uint = 0x000007;
pub const SAA7706H_SEL_DSP2_FMTA_MASK: c_uint = 0x000031;
pub const SAA7706H_SEL_DSP2_SRCB_MASK: c_uint = 0x0001c0;
pub const SAA7706H_SEL_DSP2_FMTB_MASK: c_uint = 0x000e00;
pub const SAA7706H_SEL_DSP1_SRC_MASK: c_uint = 0x003000;
pub const SAA7706H_SEL_DSP1_FMT_MASK: c_uint = 0x01c003;
pub const SAA7706H_SEL_SPDIF2: c_uint = 0x020000;
pub const SAA7706H_SEL_HOST_IO_FMT_MASK: c_uint = 0x1c0000;
pub const SAA7706H_SEL_EN_HOST_IO: c_uint = 0x200000;
pub const SAA7706H_REG_IAC: c_uint = 0x1ff8;
pub const SAA7706H_REG_CLK_SET: c_uint = 0x1ff9;
pub const SAA7706H_REG_CLK_COEFF: c_uint = 0x1ffa;
pub const SAA7706H_REG_INPUT_SENS: c_uint = 0x1ffb;
pub const SAA7706H_INPUT_SENS_RDS_VOL_MASK: c_uint = 0x0003f;
pub const SAA7706H_INPUT_SENS_FM_VOL_MASK: c_uint = 0x00fc0;
pub const SAA7706H_INPUT_SENS_FM_MPX: c_uint = 0x01000;
pub const SAA7706H_INPUT_SENS_OFF_FILTER_A_EN: c_uint = 0x02000;
pub const SAA7706H_INPUT_SENS_OFF_FILTER_B_EN: c_uint = 0x04000;
pub const SAA7706H_REG_PHONE_NAV_AUDIO: c_uint = 0x1ffc;
pub const SAA7706H_REG_IO_CONF_DSP2: c_uint = 0x1ffd;
pub const SAA7706H_REG_STATUS_DSP2: c_uint = 0x1ffe;
pub const SAA7706H_REG_PC_DSP2: c_uint = 0x1fff;
pub const SAA7706H_DSP1_MOD0: c_uint = 0x0800;
pub const SAA7706H_DSP1_ROM_VER: c_uint = 0x097f;
pub const SAA7706H_DSP2_MPTR0: c_uint = 0x1000;
pub const SAA7706H_DSP1_MODPNTR: c_uint = 0x0000;
pub const SAA7706H_DSP2_XMEM_CONTLLCW: c_uint = 0x113e;
pub const SAA7706H_DSP2_XMEM_BUSAMP: c_uint = 0x114a;
pub const SAA7706H_DSP2_XMEM_FDACPNTR: c_uint = 0x11f9;
pub const SAA7706H_DSP2_XMEM_IIS1PNTR: c_uint = 0x11fb;
pub const SAA7706H_DSP2_YMEM_PVGA: c_uint = 0x212a;
pub const SAA7706H_DSP2_YMEM_PVAT1: c_uint = 0x212b;
pub const SAA7706H_DSP2_YMEM_PVAT: c_uint = 0x212c;
pub const SAA7706H_DSP2_YMEM_ROM_VER: c_uint = 0x21ff;
pub const SUPPORTED_DSP1_ROM_VER: c_uint = 0x667;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7706h_state {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub muted: unsigned,
}

    static inline struct saa7706h_state *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct saa7706h_state, sd);
    }
#[no_mangle]
unsafe extern "C" fn saa7706h_i2c_send(client: *mut i2c_client, data: *const u8, len: c_int) -> c_int {
    static int saa7706h_i2c_send(struct i2c_client *client, const u8 *data, int len)
    {
    let mut err: c_int = i2c_master_send(client, data, len);
    if (err == len)
    return 0;
    return err > 0 ? -EIO : err;
    }
    static int saa7706h_i2c_transfer(struct i2c_client *client,
    struct i2c_msg *msgs, int num)
    {
    let mut err: c_int = i2c_transfer(client.adapter, msgs, num);
    if (err == num)
    return 0;
    return err > 0 ? -EIO : err;
    }
#[no_mangle]
unsafe extern "C" fn saa7706h_set_reg24(sd: *mut v4l2_subdev, reg: u16, val: u32) -> c_int {
    static int saa7706h_set_reg24(struct v4l2_subdev *sd, u16 reg, u32 val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    u8 buf[5];
    let mut pos: c_int = 0;
    buf[pos++] = reg >> 8;
    buf[pos++] = reg;
    buf[pos++] = val >> 16;
    buf[pos++] = val >> 8;
    buf[pos++] = val;
    return saa7706h_i2c_send(client, buf, pos);
    }
    static int saa7706h_set_reg24_err(struct v4l2_subdev *sd, u16 reg, u32 val,
    int *err)
    {
    return *err ? *err : saa7706h_set_reg24(sd, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn saa7706h_set_reg16(sd: *mut v4l2_subdev, reg: u16, val: u16) -> c_int {
    static int saa7706h_set_reg16(struct v4l2_subdev *sd, u16 reg, u16 val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    u8 buf[4];
    let mut pos: c_int = 0;
    buf[pos++] = reg >> 8;
    buf[pos++] = reg;
    buf[pos++] = val >> 8;
    buf[pos++] = val;
    return saa7706h_i2c_send(client, buf, pos);
    }
    static int saa7706h_set_reg16_err(struct v4l2_subdev *sd, u16 reg, u16 val,
    int *err)
    {
    return *err ? *err : saa7706h_set_reg16(sd, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn saa7706h_get_reg16(sd: *mut v4l2_subdev, reg: u16) -> c_int {
    static int saa7706h_get_reg16(struct v4l2_subdev *sd, u16 reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    u8 buf[2];
    int err;
    u8 regaddr[] = {reg >> 8, reg};
    struct i2c_msg msg[] = {
    {
    .addr = client.addr,
    .len = sizeof(regaddr),
    .buf = regaddr
    },
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = sizeof(buf),
    .buf = buf
    }
    };
    err = saa7706h_i2c_transfer(client, msg, ARRAY_SIZE(msg));
    if (err)
    return err;
    return buf[0] << 8 | buf[1];
    }
#[no_mangle]
unsafe extern "C" fn saa7706h_unmute(sd: *mut v4l2_subdev) -> c_int {
    static int saa7706h_unmute(struct v4l2_subdev *sd)
    {
    struct saa7706h_state *state = to_state(sd);
    let mut err: c_int = 0;
    err = saa7706h_set_reg16_err(sd, SAA7706H_REG_CTRL,
    SAA7706H_CTRL_PLL3_62975MHZ | SAA7706H_CTRL_PC_RESET_DSP1 |
    SAA7706H_CTRL_PC_RESET_DSP2, &err);
// newer versions of the chip requires a small sleep after reset
    msleep(1);
    err = saa7706h_set_reg16_err(sd, SAA7706H_REG_CTRL,
    SAA7706H_CTRL_PLL3_62975MHZ, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_EVALUATION, 0, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_CL_GEN1, 0x040022, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_CL_GEN2,
    SAA7706H_CL_GEN2_WSEDGE_FALLING, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_CL_GEN4, 0x024080, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_SEL, 0x200080, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_IAC, 0xf4caed, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_CLK_SET, 0x124334, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_CLK_COEFF, 0x004a1a,
    &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_INPUT_SENS, 0x0071c7,
    &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_PHONE_NAV_AUDIO,
    0x0e22ff, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_IO_CONF_DSP2, 0x001ff8,
    &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_STATUS_DSP2, 0x080003,
    &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_REG_PC_DSP2, 0x000004, &err);
    err = saa7706h_set_reg16_err(sd, SAA7706H_DSP1_MOD0, 0x0c6c, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_DSP2_MPTR0, 0x000b4b, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_DSP1_MODPNTR, 0x000600, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_DSP1_MODPNTR, 0x0000c0, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_DSP2_XMEM_CONTLLCW, 0x000819,
    &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_DSP2_XMEM_CONTLLCW, 0x00085a,
    &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_DSP2_XMEM_BUSAMP, 0x7fffff,
    &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_DSP2_XMEM_FDACPNTR, 0x2000cb,
    &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_DSP2_XMEM_IIS1PNTR, 0x2000cb,
    &err);
    err = saa7706h_set_reg16_err(sd, SAA7706H_DSP2_YMEM_PVGA, 0x0f80, &err);
    err = saa7706h_set_reg16_err(sd, SAA7706H_DSP2_YMEM_PVAT1, 0x0800,
    &err);
    err = saa7706h_set_reg16_err(sd, SAA7706H_DSP2_YMEM_PVAT, 0x0800, &err);
    err = saa7706h_set_reg24_err(sd, SAA7706H_DSP2_XMEM_CONTLLCW, 0x000905,
    &err);
    if (!err)
    state.muted = 0;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn saa7706h_mute(sd: *mut v4l2_subdev) -> c_int {
    static int saa7706h_mute(struct v4l2_subdev *sd)
    {
    struct saa7706h_state *state = to_state(sd);
    int err;
    err = saa7706h_set_reg16(sd, SAA7706H_REG_CTRL,
    SAA7706H_CTRL_PLL3_62975MHZ | SAA7706H_CTRL_PC_RESET_DSP1 |
    SAA7706H_CTRL_PC_RESET_DSP2);
    if (!err)
    state.muted = 1;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn saa7706h_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int saa7706h_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct saa7706h_state *state =
    container_of(ctrl.handler, struct saa7706h_state, hdl);
    switch (ctrl.id) {
    case V4L2_CID_AUDIO_MUTE:
    if (ctrl.val)
    return saa7706h_mute(&state.sd);
    return saa7706h_unmute(&state.sd);
    }
    return -EINVAL;
    }
    static const struct v4l2_ctrl_ops saa7706h_ctrl_ops = {
    .s_ctrl = saa7706h_s_ctrl,
    };
    let mut empty_ops: static struct v4l2_subdev_ops = {};
//
// Generic i2c probe
// concerning the addresses: i2c wants 7 bit (without the r/w bit), so '>>1'
//
#[no_mangle]
unsafe extern "C" fn saa7706h_probe(client: *mut i2c_client) -> c_int {
    static int saa7706h_probe(struct i2c_client *client)
    {
    struct saa7706h_state *state;
    struct v4l2_subdev *sd;
    int err;
// Check if the adapter supports the needed features
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    v4l_info(client, "chip found @ 0x%02x (%s)\n",
    client.addr << 1, client.adapter.name);
    state = kzalloc_obj(struct saa7706h_state);
    if (state == core::ptr::null_mut())
    return -ENOMEM;
    sd = &state.sd;
    v4l2_i2c_subdev_init(sd, client, &empty_ops);
    v4l2_ctrl_handler_init(&state.hdl, 4);
    v4l2_ctrl_new_std(&state.hdl, &saa7706h_ctrl_ops,
    V4L2_CID_AUDIO_MUTE, 0, 1, 1, 1);
    sd.ctrl_handler = &state.hdl;
    err = state.hdl.error;
    if (err)
    goto err;
// check the rom versions
    err = saa7706h_get_reg16(sd, SAA7706H_DSP1_ROM_VER);
    if (err < 0)
    goto err;
    if (err != SUPPORTED_DSP1_ROM_VER)
    v4l2_warn(sd, "Unknown DSP1 ROM code version: 0x%x\n", err);
    state.muted = 1;
// startup in a muted state
    err = saa7706h_mute(sd);
    if (err)
    goto err;
    return 0;
    err:
    v4l2_device_unregister_subdev(sd);
    v4l2_ctrl_handler_free(&state.hdl);
    kfree(to_state(sd));
    printk(KERN_ERR DRIVER_NAME ": Failed to probe: %d\n", err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn saa7706h_remove(client: *mut i2c_client) {
    static void saa7706h_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct saa7706h_state *state = to_state(sd);
    saa7706h_mute(sd);
    v4l2_device_unregister_subdev(sd);
    v4l2_ctrl_handler_free(&state.hdl);
    kfree(to_state(sd));
    }
    static const struct i2c_device_id saa7706h_id[] = {
    { .name = DRIVER_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, saa7706h_id);
    static struct i2c_driver saa7706h_driver = {
    .driver = {
    .name	= DRIVER_NAME,
    },
    .probe		= saa7706h_probe,
    .remove		= saa7706h_remove,
    .id_table	= saa7706h_id,
    };
    module_i2c_driver(saa7706h_driver);
    MODULE_DESCRIPTION("SAA7706H Car Radio DSP driver");
    MODULE_AUTHOR("Mocean Laboratories");
    MODULE_LICENSE("GPL v2");
