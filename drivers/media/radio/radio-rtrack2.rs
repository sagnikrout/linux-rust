//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/radio-rtrack2.c
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
// RadioTrack II driver
// Copyright 1998 Ben Pfaff
//
// Based on RadioTrack I/RadioReveal (C) 1997 M. Kirkwood
// Converted to new API by Alan Cox <alan@lxorguk.ukuu.org.uk>
// Various bugfixes and enhancements by Russell Kroll <rkroll@exploits.org>
//
// Converted to the radio-isa framework by Hans Verkuil <hverkuil@kernel.org>
// Converted to V4L2 API by Mauro Carvalho Chehab <mchehab@kernel.org>
//
// Fully tested with actual hardware and the v4l2-compliance tool.
//

    MODULE_AUTHOR("Ben Pfaff");
    MODULE_DESCRIPTION("A driver for the RadioTrack II radio card.");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("0.1.99");

pub const RTRACK2_MAX: c_int = 2;
    static int io[RTRACK2_MAX] = { [0] = CONFIG_RADIO_RTRACK2_PORT,
    [1 ... (RTRACK2_MAX - 1)] = -1 };
    static int radio_nr[RTRACK2_MAX] = { [0 ... (RTRACK2_MAX - 1)] = -1 };
    module_param_array(io, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(io, "I/O addresses of the RadioTrack card (0x20f or 0x30f)");
    module_param_array(radio_nr, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(radio_nr, "Radio device numbers");
    static struct radio_isa_card *rtrack2_alloc(void)
    {
    return kzalloc_obj(struct radio_isa_card);
    }
#[no_mangle]
unsafe extern "C" fn zero(isa: *mut radio_isa_card) {
    static void zero(struct radio_isa_card *isa)
    {
    outb_p(1, isa.io);
    outb_p(3, isa.io);
    outb_p(1, isa.io);
    }
#[no_mangle]
unsafe extern "C" fn one(isa: *mut radio_isa_card) {
    static void one(struct radio_isa_card *isa)
    {
    outb_p(5, isa.io);
    outb_p(7, isa.io);
    outb_p(5, isa.io);
    }
#[no_mangle]
unsafe extern "C" fn rtrack2_s_frequency(isa: *mut radio_isa_card, freq: u32) -> c_int {
    static int rtrack2_s_frequency(struct radio_isa_card *isa, u32 freq)
    {
    int i;
    freq = freq / 200 + 856;
    outb_p(0xc8, isa.io);
    outb_p(0xc9, isa.io);
    outb_p(0xc9, isa.io);
    for (i = 0; i < 10; i++)
    zero(isa);
    for (i = 14; i >= 0; i--)
    if (freq & (1 << i))
    one(isa);
    else
    zero(isa);
    outb_p(0xc8, isa.io);
    outb_p(v4l2_ctrl_g_ctrl(isa.mute), isa.io);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtrack2_g_signal(isa: *mut radio_isa_card) -> u32 {
    static u32 rtrack2_g_signal(struct radio_isa_card *isa)
    {
// bit set = no signal present
    return (inb(isa.io) & 2) ? 0 : 0xffff;
    }
#[no_mangle]
unsafe extern "C" fn rtrack2_s_mute_volume(isa: *mut radio_isa_card, mute: bool, vol: c_int) -> c_int {
    static int rtrack2_s_mute_volume(struct radio_isa_card *isa, bool mute, int vol)
    {
    outb(mute, isa.io);
    return 0;
    }
    static const struct radio_isa_ops rtrack2_ops = {
    .alloc = rtrack2_alloc,
    .s_mute_volume = rtrack2_s_mute_volume,
    .s_frequency = rtrack2_s_frequency,
    .g_signal = rtrack2_g_signal,
    };
    static const int rtrack2_ioports[] = { 0x20f, 0x30f };
    static struct radio_isa_driver rtrack2_driver = {
    .driver = {
    .match		= radio_isa_match,
    .probe		= radio_isa_probe,
    .remove		= radio_isa_remove,
    .driver		= {
    .name	= "radio-rtrack2",
    },
    },
    .io_params = io,
    .radio_nr_params = radio_nr,
    .io_ports = rtrack2_ioports,
    .num_of_io_ports = ARRAY_SIZE(rtrack2_ioports),
    .region_size = 4,
    .card = "AIMSlab RadioTrack II",
    .ops = &rtrack2_ops,
    .has_stereo = true,
    };
#[no_mangle]
unsafe extern "C" fn rtrack2_init() -> int __init {
    static int __init rtrack2_init(void)
    {
    return isa_register_driver(&rtrack2_driver.driver, RTRACK2_MAX);
    }
#[no_mangle]
unsafe extern "C" fn rtrack2_exit() -> void __exit {
    static void __exit rtrack2_exit(void)
    {
    isa_unregister_driver(&rtrack2_driver.driver);
    }
    module_init(rtrack2_init);
    module_exit(rtrack2_exit);
