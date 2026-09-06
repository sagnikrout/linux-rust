//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/upd64031a.c
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
// upd64031A - NEC Electronics Ghost Reduction for NTSC in Japan
//
// 2003 by T.Adachi <tadachi@tadachi-net.com>
// 2003 by Takeru KOMORIYA <komoriya@paken.org>
// 2006 by Hans Verkuil <hverkuil@kernel.org>
//

// --------------------- read registers functions define --------------------
// bit masks
pub const GR_MODE_MASK: c_uint = 0xc0;
pub const DIRECT_3DYCS_CONNECT_MASK: c_uint = 0xc0;
pub const SYNC_CIRCUIT_MASK: c_uint = 0xa0;
// --------------------------------------------------------------------------
    MODULE_DESCRIPTION("uPD64031A driver");
    MODULE_AUTHOR("T. Adachi, Takeru KOMORIYA, Hans Verkuil");
    MODULE_LICENSE("GPL");
    static int debug;
    module_param(debug, int, 0644);
    MODULE_PARM_DESC(debug, "Debug level (0-1)");
    enum {
    R00 = 0, R01, R02, R03, R04,
    R05, R06, R07, R08, R09,
    R0A, R0B, R0C, R0D, R0E, R0F,
// unused registers
    R10, R11, R12, R13, R14,
    R15, R16, R17,
//
    TOT_REGS
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upd64031a_state {
    pub sd: v4l2_subdev,
    pub regs: [u8; TOT_REGS],
    pub gr_mode: u8,
    pub direct_3dycs_connect: u8,
    pub ext_comp_sync: u8,
    pub ext_vert_sync: u8,
}

    static inline struct upd64031a_state *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct upd64031a_state, sd);
    }
    static u8 upd64031a_init[] = {
    0x00, 0xb8, 0x48, 0xd2, 0xe6,
    0x03, 0x10, 0x0b, 0xaf, 0x7f,
    0x00, 0x00, 0x1d, 0x5e, 0x00,
    0xd0
    };
// ------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn upd64031a_read(sd: *mut v4l2_subdev, reg: u8) -> u8 {
    static u8 upd64031a_read(struct v4l2_subdev *sd, u8 reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    u8 buf[2];
    if (reg >= sizeof(buf))
    return 0xff;
    i2c_master_recv(client, buf, 2);
    return buf[reg];
    }
// ------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn upd64031a_write(sd: *mut v4l2_subdev, reg: u8, val: u8) {
    static void upd64031a_write(struct v4l2_subdev *sd, u8 reg, u8 val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    u8 buf[2];
    buf[0] = reg;
    buf[1] = val;
    v4l2_dbg(1, debug, sd, "write reg: %02X val: %02X\n", reg, val);
    if (i2c_master_send(client, buf, 2) != 2)
    v4l2_err(sd, "I/O error write 0x%02x/0x%02x\n", reg, val);
    }
// ------------------------------------------------------------------------
// The input changed due to new input or channel changed
#[no_mangle]
unsafe extern "C" fn upd64031a_s_frequency(sd: *mut v4l2_subdev, freq: *const v4l2_frequency) -> c_int {
    static int upd64031a_s_frequency(struct v4l2_subdev *sd, const struct v4l2_frequency *freq)
    {
    struct upd64031a_state *state = to_state(sd);
    let mut reg: u8 = state.regs[R00];
    v4l2_dbg(1, debug, sd, "changed input or channel\n");
    upd64031a_write(sd, R00, reg | 0x10);
    upd64031a_write(sd, R00, reg & ~0x10);
    return 0;
    }
// ------------------------------------------------------------------------
    static int upd64031a_s_routing(struct v4l2_subdev *sd,
    u32 input, u32 output, u32 config)
    {
    struct upd64031a_state *state = to_state(sd);
    u8 r00, r05, r08;
    state.gr_mode = (input & 3) << 6;
    state.direct_3dycs_connect = (input & 0xc) << 4;
    state.ext_comp_sync =
    (input & UPD64031A_COMPOSITE_EXTERNAL) << 1;
    state.ext_vert_sync =
    (input & UPD64031A_VERTICAL_EXTERNAL) << 2;
    r00 = (state.regs[R00] & ~GR_MODE_MASK) | state.gr_mode;
    r05 = (state.regs[R00] & ~SYNC_CIRCUIT_MASK) |
    state.ext_comp_sync | state.ext_vert_sync;
    r08 = (state.regs[R08] & ~DIRECT_3DYCS_CONNECT_MASK) |
    state.direct_3dycs_connect;
    upd64031a_write(sd, R00, r00);
    upd64031a_write(sd, R05, r05);
    upd64031a_write(sd, R08, r08);
    return upd64031a_s_frequency(sd, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn upd64031a_log_status(sd: *mut v4l2_subdev) -> c_int {
    static int upd64031a_log_status(struct v4l2_subdev *sd)
    {
    v4l2_info(sd, "Status: SA00=0x%02x SA01=0x%02x\n",
    upd64031a_read(sd, 0), upd64031a_read(sd, 1));
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn upd64031a_g_register(sd: *mut v4l2_subdev, reg: *mut v4l2_dbg_register) -> c_int {
    static int upd64031a_g_register(struct v4l2_subdev *sd, struct v4l2_dbg_register *reg)
    {
    reg.val = upd64031a_read(sd, reg.reg & 0xff);
    reg.size = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn upd64031a_s_register(sd: *mut v4l2_subdev, reg: *const v4l2_dbg_register) -> c_int {
    static int upd64031a_s_register(struct v4l2_subdev *sd, const struct v4l2_dbg_register *reg)
    {
    upd64031a_write(sd, reg.reg & 0xff, reg.val & 0xff);
    return 0;
    }

// -----------------------------------------------------------------------
    static const struct v4l2_subdev_core_ops upd64031a_core_ops = {
    .log_status = upd64031a_log_status,

    .g_register = upd64031a_g_register,
    .s_register = upd64031a_s_register,

    };
    static const struct v4l2_subdev_tuner_ops upd64031a_tuner_ops = {
    .s_frequency = upd64031a_s_frequency,
    };
    static const struct v4l2_subdev_video_ops upd64031a_video_ops = {
    .s_routing = upd64031a_s_routing,
    };
    static const struct v4l2_subdev_ops upd64031a_ops = {
    .core = &upd64031a_core_ops,
    .tuner = &upd64031a_tuner_ops,
    .video = &upd64031a_video_ops,
    };
// ------------------------------------------------------------------------
// i2c implementation
#[no_mangle]
unsafe extern "C" fn upd64031a_probe(client: *mut i2c_client) -> c_int {
    static int upd64031a_probe(struct i2c_client *client)
    {
    struct upd64031a_state *state;
    struct v4l2_subdev *sd;
    int i;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    v4l_info(client, "chip found @ 0x%x (%s)\n",
    client.addr << 1, client.adapter.name);
    state = devm_kzalloc(&client.dev, sizeof(*state), GFP_KERNEL);
    if (state == core::ptr::null_mut())
    return -ENOMEM;
    sd = &state.sd;
    v4l2_i2c_subdev_init(sd, client, &upd64031a_ops);
    memcpy(state.regs, upd64031a_init, sizeof(state.regs));
    state.gr_mode = UPD64031A_GR_ON << 6;
    state.direct_3dycs_connect = UPD64031A_3DYCS_COMPOSITE << 4;
    state.ext_comp_sync = state.ext_vert_sync = 0;
    for (i = 0; i < TOT_REGS; i++)
    upd64031a_write(sd, i, state.regs[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn upd64031a_remove(client: *mut i2c_client) {
    static void upd64031a_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    v4l2_device_unregister_subdev(sd);
    }
// -----------------------------------------------------------------------
    static const struct i2c_device_id upd64031a_id[] = {
    { .name = "upd64031a" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, upd64031a_id);
    static struct i2c_driver upd64031a_driver = {
    .driver = {
    .name	= "upd64031a",
    },
    .probe		= upd64031a_probe,
    .remove		= upd64031a_remove,
    .id_table	= upd64031a_id,
    };
    module_i2c_driver(upd64031a_driver);
