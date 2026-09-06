//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/tda7432.c
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


// SPDX-License-Identifier: GPL-2.0
//
// For the STS-Thompson TDA7432 audio processor chip
//
// Handles audio functions: volume, balance, tone, loudness
// This driver will not complain if used with any
// other i2c device with the same address.
//
// Muting and tone control by Jonathan Isom <jisom@ematic.com>
//
// Copyright (c) 2000 Eric Sandeen <eric_sandeen@bigfoot.com>
// Copyright (c) 2006 Mauro Carvalho Chehab <mchehab@kernel.org>
//
// Based on tda9855.c by Steve VanDeBogart (vandebo@uclink.berkeley.edu)
// Which was based on tda8425.c by Greg Alexander (c) 1998
//
// OPTIONS:
// debug    - set to 1 if you'd like to see debug messages
// set to 2 if you'd like to be inundated with debug messages
//
// loudness - set between 0 and 15 for varying degrees of loudness effect
//
// maxvol   - set maximum volume to +20db (1), default is 0db(0)
//

    MODULE_AUTHOR("Eric Sandeen <eric_sandeen@bigfoot.com>");
    MODULE_DESCRIPTION("bttv driver for the tda7432 audio processor chip");
    MODULE_LICENSE("GPL");
    static int maxvol;
    static int loudness; /* disable loudness by default */
    static int debug;	 /* insmod parameter */
    module_param(debug, int, S_IRUGO | S_IWUSR);
    MODULE_PARM_DESC(debug, "Set debugging level from 0 to 3. Default is off(0).");
    module_param(loudness, int, S_IRUGO);
    MODULE_PARM_DESC(loudness, "Turn loudness on(1) else off(0). Default is off(0).");
    module_param(maxvol, int, S_IRUGO | S_IWUSR);
    MODULE_PARM_DESC(maxvol, "Set maximum volume to +20dB(0) else +0dB(1). Default is +20dB(0).");
// Structure of address and subaddresses for the tda7432
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda7432 {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    struct {
// bass/treble cluster
    pub bass: *mut v4l2_ctrl,
    pub treble: *mut v4l2_ctrl,
}

    struct {
// mute/balance cluster
    struct v4l2_ctrl *mute;
    struct v4l2_ctrl *balance;
    };
    };
    static inline struct tda7432 *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct tda7432, sd);
    }
    static inline struct v4l2_subdev *to_sd(struct v4l2_ctrl *ctrl)
    {
    return &container_of(ctrl.handler, struct tda7432, hdl).sd;
    }
// The TDA7432 is made by STS-Thompson
// http://www.st.com
// http://us.st.com/stonline/books/pdf/docs/4056.pdf
//
// TDA7432: I2C-bus controlled basic audio processor
//
// The TDA7432 controls basic audio functions like volume, balance,
// and tone control (including loudness).  It also has four channel
// output (for front and rear).  Since most vidcap cards probably
// don't have 4 channel output, this driver will set front & rear
// together (no independent control).
//
// Subaddresses for TDA7432
pub const TDA7432_IN: c_uint = 0x00 /* Input select                 */;
pub const TDA7432_VL: c_uint = 0x01 /* Volume                       */;
pub const TDA7432_TN: c_uint = 0x02 /* Bass, Treble (Tone)          */;
pub const TDA7432_LF: c_uint = 0x03 /* Attenuation LF (Left Front)  */;
pub const TDA7432_LR: c_uint = 0x04 /* Attenuation LR (Left Rear)   */;
pub const TDA7432_RF: c_uint = 0x05 /* Attenuation RF (Right Front) */;
pub const TDA7432_RR: c_uint = 0x06 /* Attenuation RR (Right Rear)  */;
pub const TDA7432_LD: c_uint = 0x07 /* Loudness                     */;
// Masks for bits in TDA7432 subaddresses
// Many of these not used - just for documentation
// Subaddress 0x00 - Input selection and bass control
// Bits 0,1,2 control input:
// 0x00 - Stereo input
// 0x02 - Mono input
// 0x03 - Mute  (Using Attenuators Plays better with modules)
// Mono probably isn't used - I'm guessing only the stereo
// input is connected on most cards, so we'll set it to stereo.
//
// Bit 3 controls bass cut: 0/1 is non-symmetric/symmetric bass cut
// Bit 4 controls bass range: 0/1 is extended/standard bass range
//
// Highest 3 bits not used
//
pub const TDA7432_STEREO_IN: c_int = 0;

// Subaddress 0x01 - Volume
// Lower 7 bits control volume from -79dB to +32dB in 1dB steps
// Recommended maximum is +20 dB
//
// +32dB: 0x00
// +20dB: 0x0c
// 0dB: 0x20
// -79dB: 0x6f
//
// MSB (bit 7) controls loudness: 1/0 is loudness on/off
//
pub const TDA7432_VOL_0DB: c_uint = 0x20;

// Subaddress 0x02 - Tone control
// Bits 0,1,2 control absolute treble gain from 0dB to 14dB
// 0x0 is 14dB, 0x7 is 0dB
//
// Bit 3 controls treble attenuation/gain (sign)
// 1 = gain (+)
// 0 = attenuation (-)
//
// Bits 4,5,6 control absolute bass gain from 0dB to 14dB
// (This is only true for normal base range, set in 0x00)
// 0x0 << 4 is 14dB, 0x7 is 0dB
//
// Bit 7 controls bass attenuation/gain (sign)
// 1 << 7 = gain (+)
// 0 << 7 = attenuation (-)
//
// Example:
// 1 1 0 1 0 1 0 1 is +4dB bass, -4dB treble
//
pub const TDA7432_TREBLE_0DB: c_uint = 0xf;
pub const TDA7432_TREBLE: c_int = 7;

pub const TDA7432_BASS_0DB: c_uint = 0xf;

// Subaddress 0x03 - Left  Front attenuation
// Subaddress 0x04 - Left  Rear  attenuation
// Subaddress 0x05 - Right Front attenuation
// Subaddress 0x06 - Right Rear  attenuation
// Bits 0,1,2,3,4 control attenuation from 0dB to -37.5dB
// in 1.5dB steps.
//
// 0x00 is     0dB
// 0x1f is -37.5dB
//
// Bit 5 mutes that channel when set (1 = mute, 0 = unmute)
// We'll use the mute on the input, though (above)
// Bits 6,7 unused
//
pub const TDA7432_ATTEN_0DB: c_uint = 0x00;
pub const TDA7432_MUTE: c_uint = 0x1 << 5;
// Subaddress 0x07 - Loudness Control
// Bits 0,1,2,3 control loudness from 0dB to -15dB in 1dB steps
// when bit 4 is NOT set
//
// 0x0 is   0dB
// 0xf is -15dB
//
// If bit 4 is set, then there is a flat attenuation according to
// the lower 4 bits, as above.
//
// Bits 5,6,7 unused
//
// Begin code
#[no_mangle]
unsafe extern "C" fn tda7432_write(sd: *mut v4l2_subdev, subaddr: c_int, val: c_int) -> c_int {
    static int tda7432_write(struct v4l2_subdev *sd, int subaddr, int val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    unsigned char buffer[2];
    v4l2_dbg(2, debug, sd, "In tda7432_write\n");
    v4l2_dbg(1, debug, sd, "Writing %d 0x%x\n", subaddr, val);
    buffer[0] = subaddr;
    buffer[1] = val;
    if (2 != i2c_master_send(client, buffer, 2)) {
    v4l2_err(sd, "I/O error, trying (write %d 0x%x)\n",
    subaddr, val);
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tda7432_set(sd: *mut v4l2_subdev) -> c_int {
    static int tda7432_set(struct v4l2_subdev *sd)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    unsigned char buf[16];
    buf[0]  = TDA7432_IN;
    buf[1]  = TDA7432_STEREO_IN |  /* Main (stereo) input   */
    TDA7432_BASS_SYM  |  /* Symmetric bass cut    */
    TDA7432_BASS_NORM;   /* Normal bass range     */
    buf[2]  = 0x3b;
    if (loudness)			 /* Turn loudness on?     */
    buf[2] |= TDA7432_LD_ON;
    buf[3]  = TDA7432_TREBLE_0DB | (TDA7432_BASS_0DB << 4);
    buf[4]  = TDA7432_ATTEN_0DB;
    buf[5]  = TDA7432_ATTEN_0DB;
    buf[6]  = TDA7432_ATTEN_0DB;
    buf[7]  = TDA7432_ATTEN_0DB;
    buf[8]  = loudness;
    if (9 != i2c_master_send(client, buf, 9)) {
    v4l2_err(sd, "I/O error, trying tda7432_set\n");
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tda7432_log_status(sd: *mut v4l2_subdev) -> c_int {
    static int tda7432_log_status(struct v4l2_subdev *sd)
    {
    struct tda7432 *state = to_state(sd);
    v4l2_ctrl_handler_log_status(&state.hdl, sd.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tda7432_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int tda7432_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *sd = to_sd(ctrl);
    struct tda7432 *t = to_state(sd);
    u8 bass, treble, volume;
    u8 lf, lr, rf, rr;
    switch (ctrl.id) {
    case V4L2_CID_AUDIO_MUTE:
    if (t.balance.val < 0) {
// shifted to left, attenuate right
    rr = rf = -t.balance.val;
    lr = lf = TDA7432_ATTEN_0DB;
    } else if (t.balance.val > 0) {
// shifted to right, attenuate left
    rr = rf = TDA7432_ATTEN_0DB;
    lr = lf = t.balance.val;
    } else {
// centered
    rr = rf = TDA7432_ATTEN_0DB;
    lr = lf = TDA7432_ATTEN_0DB;
    }
    if (t.mute.val) {
    lf |= TDA7432_MUTE;
    lr |= TDA7432_MUTE;
    rf |= TDA7432_MUTE;
    rr |= TDA7432_MUTE;
    }
// Mute & update balance
    tda7432_write(sd, TDA7432_LF, lf);
    tda7432_write(sd, TDA7432_LR, lr);
    tda7432_write(sd, TDA7432_RF, rf);
    tda7432_write(sd, TDA7432_RR, rr);
    return 0;
    case V4L2_CID_AUDIO_VOLUME:
    volume = 0x6f - ctrl.val;
    if (loudness)		/* Turn on the loudness bit */
    volume |= TDA7432_LD_ON;
    tda7432_write(sd, TDA7432_VL, volume);
    return 0;
    case V4L2_CID_AUDIO_BASS:
    bass = t.bass.val;
    treble = t.treble.val;
    if (bass >= 0x8)
    bass = 14 - (bass - 8);
    if (treble >= 0x8)
    treble = 14 - (treble - 8);
    tda7432_write(sd, TDA7432_TN, 0x10 | (bass << 4) | treble);
    return 0;
    }
    return -EINVAL;
    }
// -----------------------------------------------------------------------
    static const struct v4l2_ctrl_ops tda7432_ctrl_ops = {
    .s_ctrl = tda7432_s_ctrl,
    };
    static const struct v4l2_subdev_core_ops tda7432_core_ops = {
    .log_status = tda7432_log_status,
    };
    static const struct v4l2_subdev_ops tda7432_ops = {
    .core = &tda7432_core_ops,
    };
// -----------------------------------------------------------------------
// ***********************
// i2c interface functions
// ***********************
#[no_mangle]
unsafe extern "C" fn tda7432_probe(client: *mut i2c_client) -> c_int {
    static int tda7432_probe(struct i2c_client *client)
    {
    struct tda7432 *t;
    struct v4l2_subdev *sd;
    v4l_info(client, "chip found @ 0x%02x (%s)\n",
    client.addr << 1, client.adapter.name);
    t = devm_kzalloc(&client.dev, sizeof(*t), GFP_KERNEL);
    if (!t)
    return -ENOMEM;
    sd = &t.sd;
    v4l2_i2c_subdev_init(sd, client, &tda7432_ops);
    v4l2_ctrl_handler_init(&t.hdl, 5);
    v4l2_ctrl_new_std(&t.hdl, &tda7432_ctrl_ops,
    V4L2_CID_AUDIO_VOLUME, 0, maxvol ? 0x68 : 0x4f, 1, maxvol ? 0x5d : 0x47);
    t.mute = v4l2_ctrl_new_std(&t.hdl, &tda7432_ctrl_ops,
    V4L2_CID_AUDIO_MUTE, 0, 1, 1, 0);
    t.balance = v4l2_ctrl_new_std(&t.hdl, &tda7432_ctrl_ops,
    V4L2_CID_AUDIO_BALANCE, -31, 31, 1, 0);
    t.bass = v4l2_ctrl_new_std(&t.hdl, &tda7432_ctrl_ops,
    V4L2_CID_AUDIO_BASS, 0, 14, 1, 7);
    t.treble = v4l2_ctrl_new_std(&t.hdl, &tda7432_ctrl_ops,
    V4L2_CID_AUDIO_TREBLE, 0, 14, 1, 7);
    sd.ctrl_handler = &t.hdl;
    if (t.hdl.error) {
    let mut err: c_int = t.hdl.error;
    v4l2_ctrl_handler_free(&t.hdl);
    return err;
    }
    v4l2_ctrl_cluster(2, &t.bass);
    v4l2_ctrl_cluster(2, &t.mute);
    v4l2_ctrl_handler_setup(&t.hdl);
    if (loudness < 0 || loudness > 15) {
    v4l2_warn(sd, "loudness parameter must be between 0 and 15\n");
    if (loudness < 0)
    loudness = 0;
    if (loudness > 15)
    loudness = 15;
    }
    tda7432_set(sd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tda7432_remove(client: *mut i2c_client) {
    static void tda7432_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct tda7432 *t = to_state(sd);
    tda7432_set(sd);
    v4l2_device_unregister_subdev(sd);
    v4l2_ctrl_handler_free(&t.hdl);
    }
    static const struct i2c_device_id tda7432_id[] = {
    { .name = "tda7432" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tda7432_id);
    static struct i2c_driver tda7432_driver = {
    .driver = {
    .name	= "tda7432",
    },
    .probe		= tda7432_probe,
    .remove		= tda7432_remove,
    .id_table	= tda7432_id,
    };
    module_i2c_driver(tda7432_driver);
