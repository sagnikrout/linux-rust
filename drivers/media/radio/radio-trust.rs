//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/radio-trust.c
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
// radio-trust.c - Trust FM Radio card driver for Linux 2.2
// by Eric Lammerts <eric@scintilla.utwente.nl>
//
// Based on radio-aztech.c. Original notes:
//
// Adapted to support the Video for Linux API by
// Russell Kroll <rkroll@exploits.org>.  Based on original tuner code by:
//
// Quay Ly
// Donald Song
// Jason Lewis      (jlewis@twilight.vtc.vsc.edu)
// Scott McGrath    (smcgrath@twilight.vtc.vsc.edu)
// William McGrath  (wmcgrath@twilight.vtc.vsc.edu)
//
// Converted to V4L2 API by Mauro Carvalho Chehab <mchehab@kernel.org>
//

    MODULE_AUTHOR("Eric Lammerts, Russell Kroll, Quay Lu, Donald Song, Jason Lewis, Scott McGrath, William McGrath");
    MODULE_DESCRIPTION("A driver for the Trust FM Radio card.");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("0.1.99");
// acceptable ports: 0x350 (JP3 shorted), 0x358 (JP3 open)

pub const TRUST_MAX: c_int = 2;
    static int io[TRUST_MAX] = { [0] = CONFIG_RADIO_TRUST_PORT,
    [1 ... (TRUST_MAX - 1)] = -1 };
    static int radio_nr[TRUST_MAX] = { [0 ... (TRUST_MAX - 1)] = -1 };
    module_param_array(io, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(io, "I/O addresses of the Trust FM Radio card (0x350 or 0x358)");
    module_param_array(radio_nr, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(radio_nr, "Radio device numbers");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trust {
    pub isa: radio_isa_card,
    pub ioval: c_int,
}

    static struct radio_isa_card *trust_alloc(void)
    {
    struct trust *tr = kzalloc_obj(*tr);
    return tr ? &tr.isa : core::ptr::null_mut();
    }
// i2c addresses
pub const TDA7318_ADDR: c_uint = 0x88;
pub const TSA6060T_ADDR: c_uint = 0xc4;

#[no_mangle]
unsafe extern "C" fn write_i2c(tr: *mut trust, n: c_int, ...) {
    static void write_i2c(struct trust *tr, int n, ...)
    {
    unsigned char val, mask;
    va_list args;
    va_start(args, n);
// start condition
    TR_SET_SDA;
    TR_SET_SCL;
    TR_DELAY;
    TR_CLR_SDA;
    TR_CLR_SCL;
    TR_DELAY;
    for (; n; n--) {
    val = va_arg(args, unsigned);
    for (mask = 0x80; mask; mask >>= 1) {
    if (val & mask)
    TR_SET_SDA;
    else
    TR_CLR_SDA;
    TR_SET_SCL;
    TR_DELAY;
    TR_CLR_SCL;
    TR_DELAY;
    }
// acknowledge bit
    TR_SET_SDA;
    TR_SET_SCL;
    TR_DELAY;
    TR_CLR_SCL;
    TR_DELAY;
    }
// stop condition
    TR_CLR_SDA;
    TR_DELAY;
    TR_SET_SCL;
    TR_DELAY;
    TR_SET_SDA;
    TR_DELAY;
    va_end(args);
    }
#[no_mangle]
unsafe extern "C" fn trust_s_mute_volume(isa: *mut radio_isa_card, mute: bool, vol: c_int) -> c_int {
    static int trust_s_mute_volume(struct radio_isa_card *isa, bool mute, int vol)
    {
    struct trust *tr = container_of(isa, struct trust, isa);
    tr.ioval = (tr.ioval & 0xf7) | (mute << 3);
    outb(tr.ioval, isa.io);
    write_i2c(tr, 2, TDA7318_ADDR, vol ^ 0x1f);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trust_s_stereo(isa: *mut radio_isa_card, stereo: bool) -> c_int {
    static int trust_s_stereo(struct radio_isa_card *isa, bool stereo)
    {
    struct trust *tr = container_of(isa, struct trust, isa);
    tr.ioval = (tr.ioval & 0xfb) | (!stereo << 2);
    outb(tr.ioval, isa.io);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn trust_g_signal(isa: *mut radio_isa_card) -> u32 {
    static u32 trust_g_signal(struct radio_isa_card *isa)
    {
    int i, v;
    for (i = 0, v = 0; i < 100; i++)
    v |= inb(isa.io);
    return (v & 1) ? 0 : 0xffff;
    }
#[no_mangle]
unsafe extern "C" fn trust_s_frequency(isa: *mut radio_isa_card, freq: u32) -> c_int {
    static int trust_s_frequency(struct radio_isa_card *isa, u32 freq)
    {
    struct trust *tr = container_of(isa, struct trust, isa);
    freq /= 160;	/* Convert to 10 kHz units	*/
    freq += 1070;	/* Add 10.7 MHz IF		*/
    write_i2c(tr, 5, TSA6060T_ADDR, (freq << 1) | 1,
    freq >> 7, 0x60 | ((freq >> 15) & 1), 0);
    return 0;
    }
    static int basstreble2chip[15] = {
    0, 1, 2, 3, 4, 5, 6, 7, 14, 13, 12, 11, 10, 9, 8
    };
#[no_mangle]
unsafe extern "C" fn trust_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int trust_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct radio_isa_card *isa =
    container_of(ctrl.handler, struct radio_isa_card, hdl);
    struct trust *tr = container_of(isa, struct trust, isa);
    switch (ctrl.id) {
    case V4L2_CID_AUDIO_BASS:
    write_i2c(tr, 2, TDA7318_ADDR, 0x60 | basstreble2chip[ctrl.val]);
    return 0;
    case V4L2_CID_AUDIO_TREBLE:
    write_i2c(tr, 2, TDA7318_ADDR, 0x70 | basstreble2chip[ctrl.val]);
    return 0;
    }
    return -EINVAL;
    }
    static const struct v4l2_ctrl_ops trust_ctrl_ops = {
    .s_ctrl = trust_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn trust_initialize(isa: *mut radio_isa_card) -> c_int {
    static int trust_initialize(struct radio_isa_card *isa)
    {
    struct trust *tr = container_of(isa, struct trust, isa);
    tr.ioval = 0xf;
    write_i2c(tr, 2, TDA7318_ADDR, 0x80);	/* speaker att. LF = 0 dB */
    write_i2c(tr, 2, TDA7318_ADDR, 0xa0);	/* speaker att. RF = 0 dB */
    write_i2c(tr, 2, TDA7318_ADDR, 0xc0);	/* speaker att. LR = 0 dB */
    write_i2c(tr, 2, TDA7318_ADDR, 0xe0);	/* speaker att. RR = 0 dB */
    write_i2c(tr, 2, TDA7318_ADDR, 0x40);	/* stereo 1 input, gain = 18.75 dB */
    v4l2_ctrl_new_std(&isa.hdl, &trust_ctrl_ops,
    V4L2_CID_AUDIO_BASS, 0, 15, 1, 8);
    v4l2_ctrl_new_std(&isa.hdl, &trust_ctrl_ops,
    V4L2_CID_AUDIO_TREBLE, 0, 15, 1, 8);
    return isa.hdl.error;
    }
    static const struct radio_isa_ops trust_ops = {
    .init = trust_initialize,
    .alloc = trust_alloc,
    .s_mute_volume = trust_s_mute_volume,
    .s_frequency = trust_s_frequency,
    .s_stereo = trust_s_stereo,
    .g_signal = trust_g_signal,
    };
    static const int trust_ioports[] = { 0x350, 0x358 };
    static struct radio_isa_driver trust_driver = {
    .driver = {
    .match		= radio_isa_match,
    .probe		= radio_isa_probe,
    .remove		= radio_isa_remove,
    .driver		= {
    .name	= "radio-trust",
    },
    },
    .io_params = io,
    .radio_nr_params = radio_nr,
    .io_ports = trust_ioports,
    .num_of_io_ports = ARRAY_SIZE(trust_ioports),
    .region_size = 2,
    .card = "Trust FM Radio",
    .ops = &trust_ops,
    .has_stereo = true,
    .max_volume = 31,
    };
#[no_mangle]
unsafe extern "C" fn trust_init() -> int __init {
    static int __init trust_init(void)
    {
    return isa_register_driver(&trust_driver.driver, TRUST_MAX);
    }
#[no_mangle]
unsafe extern "C" fn trust_exit() -> void __exit {
    static void __exit trust_exit(void)
    {
    isa_unregister_driver(&trust_driver.driver);
    }
    module_init(trust_init);
    module_exit(trust_exit);
