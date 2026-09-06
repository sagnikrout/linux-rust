//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/radio-aimslab.c
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
// AimsLab RadioTrack (aka RadioVeveal) driver
//
// Copyright 1997 M. Kirkwood
//
// Converted to the radio-isa framework by Hans Verkuil <hverkuil@kernel.org>
// Converted to V4L2 API by Mauro Carvalho Chehab <mchehab@kernel.org>
// Converted to new API by Alan Cox <alan@lxorguk.ukuu.org.uk>
// Various bugfixes and enhancements by Russell Kroll <rkroll@exploits.org>
//
// Notes on the hardware (reverse engineered from other peoples'
// reverse engineering of AIMS' code :-)
//
// Frequency control is done digitally -- ie out(port,encodefreq(95.8));
//
// The signal strength query is unsurprisingly inaccurate.  And it seems
// to indicate that (on my card, at least) the frequency setting isn't
// too great.  (I have to tune up .025MHz from what the freq should be
// to get a report that the thing is tuned.)
//
// Volume control is (ugh) analogue:
// out(port, start_increasing_volume);
// wait(a_wee_while);
// out(port, stop_changing_the_volume);
//
// Fully tested with the Keene USB FM Transmitter and the v4l2-compliance tool.
//

    MODULE_AUTHOR("M. Kirkwood");
    MODULE_DESCRIPTION("A driver for the RadioTrack/RadioReveal radio card.");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("1.0.0");

pub const RTRACK_MAX: c_int = 2;
    static int io[RTRACK_MAX] = { [0] = CONFIG_RADIO_RTRACK_PORT,
    [1 ... (RTRACK_MAX - 1)] = -1 };
    static int radio_nr[RTRACK_MAX]	= { [0 ... (RTRACK_MAX - 1)] = -1 };
    module_param_array(io, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(io, "I/O addresses of the RadioTrack card (0x20f or 0x30f)");
    module_param_array(radio_nr, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(radio_nr, "Radio device numbers");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtrack {
    pub isa: radio_isa_card,
    pub curvol: c_int,
}

    static struct radio_isa_card *rtrack_alloc(void)
    {
    struct rtrack *rt = kzalloc_obj(struct rtrack);
    if (rt)
    rt.curvol = 0xff;
    return rt ? &rt.isa : core::ptr::null_mut();
    }

// bit 5 is not connected

#[no_mangle]
unsafe extern "C" fn rtrack_set_pins(handle: *mut c_void, pins: u8) {
    static void rtrack_set_pins(void *handle, u8 pins)
    {
    struct radio_isa_card *isa = handle;
    struct rtrack *rt = container_of(isa, struct rtrack, isa);
    let mut bits: u8 = AIMS_BIT_VOL_DN | AIMS_BIT_VOL_UP | AIMS_BIT_TUN_STRQ;
    if (!v4l2_ctrl_g_ctrl(rt.isa.mute))
    bits |= AIMS_BIT_VOL_CE;
    if (pins & LM7000_DATA)
    bits |= AIMS_BIT_TUN_DATA;
    if (pins & LM7000_CLK)
    bits |= AIMS_BIT_TUN_CLK;
    if (pins & LM7000_CE)
    bits |= AIMS_BIT_TUN_CE;
    outb_p(bits, rt.isa.io);
    }
#[no_mangle]
unsafe extern "C" fn rtrack_s_frequency(isa: *mut radio_isa_card, freq: u32) -> c_int {
    static int rtrack_s_frequency(struct radio_isa_card *isa, u32 freq)
    {
    lm7000_set_freq(freq, isa, rtrack_set_pins);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtrack_g_signal(isa: *mut radio_isa_card) -> u32 {
    static u32 rtrack_g_signal(struct radio_isa_card *isa)
    {
// bit set = no signal present
    return 0xffff * !(inb(isa.io) & 2);
    }
#[no_mangle]
unsafe extern "C" fn rtrack_s_mute_volume(isa: *mut radio_isa_card, mute: bool, vol: c_int) -> c_int {
    static int rtrack_s_mute_volume(struct radio_isa_card *isa, bool mute, int vol)
    {
    struct rtrack *rt = container_of(isa, struct rtrack, isa);
    let mut curvol: c_int = rt.curvol;
    if (mute) {
    outb(0xd0, isa.io);	/* volume steady + sigstr + off	*/
    return 0;
    }
    if (vol == 0) {			/* volume = 0 means mute the card */
    outb(0x48, isa.io);	/* volume down but still "on"	*/
    msleep(curvol * 3);	/* make sure it's totally down	*/
    } else if (curvol < vol) {
    outb(0x98, isa.io);	/* volume up + sigstr + on	*/
    for (; curvol < vol; curvol++)
    mdelay(3);
    } else if (curvol > vol) {
    outb(0x58, isa.io);	/* volume down + sigstr + on	*/
    for (; curvol > vol; curvol--)
    mdelay(3);
    }
    outb(0xd8, isa.io);		/* volume steady + sigstr + on	*/
    rt.curvol = vol;
    return 0;
    }
// Mute card - prevents noisy bootups
#[no_mangle]
unsafe extern "C" fn rtrack_initialize(isa: *mut radio_isa_card) -> c_int {
    static int rtrack_initialize(struct radio_isa_card *isa)
    {
// this ensures that the volume is all the way up
    outb(0x90, isa.io);	/* volume up but still "on"	*/
    msleep(3000);		/* make sure it's totally up	*/
    outb(0xc0, isa.io);	/* steady volume, mute card	*/
    return 0;
    }
    static const struct radio_isa_ops rtrack_ops = {
    .alloc = rtrack_alloc,
    .init = rtrack_initialize,
    .s_mute_volume = rtrack_s_mute_volume,
    .s_frequency = rtrack_s_frequency,
    .g_signal = rtrack_g_signal,
    };
    static const int rtrack_ioports[] = { 0x20f, 0x30f };
    static struct radio_isa_driver rtrack_driver = {
    .driver = {
    .match		= radio_isa_match,
    .probe		= radio_isa_probe,
    .remove		= radio_isa_remove,
    .driver		= {
    .name	= "radio-aimslab",
    },
    },
    .io_params = io,
    .radio_nr_params = radio_nr,
    .io_ports = rtrack_ioports,
    .num_of_io_ports = ARRAY_SIZE(rtrack_ioports),
    .region_size = 2,
    .card = "AIMSlab RadioTrack/RadioReveal",
    .ops = &rtrack_ops,
    .has_stereo = true,
    .max_volume = 0xff,
    };
#[no_mangle]
unsafe extern "C" fn rtrack_init() -> int __init {
    static int __init rtrack_init(void)
    {
    return isa_register_driver(&rtrack_driver.driver, RTRACK_MAX);
    }
#[no_mangle]
unsafe extern "C" fn rtrack_exit() -> void __exit {
    static void __exit rtrack_exit(void)
    {
    isa_unregister_driver(&rtrack_driver.driver);
    }
    module_init(rtrack_init);
    module_exit(rtrack_exit);
