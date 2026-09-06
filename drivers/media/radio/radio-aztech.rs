//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/radio-aztech.c
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
// radio-aztech.c - Aztech radio card driver
//
// Converted to the radio-isa framework by Hans Verkuil <hverkuil@kernel.org>
// Converted to V4L2 API by Mauro Carvalho Chehab <mchehab@kernel.org>
// Adapted to support the Video for Linux API by
// Russell Kroll <rkroll@exploits.org>.  Based on original tuner code by:
//
// Quay Ly
// Donald Song
// Jason Lewis      (jlewis@twilight.vtc.vsc.edu)
// Scott McGrath    (smcgrath@twilight.vtc.vsc.edu)
// William McGrath  (wmcgrath@twilight.vtc.vsc.edu)
//
// Fully tested with the Keene USB FM Transmitter and the v4l2-compliance tool.
//

    MODULE_AUTHOR("Russell Kroll, Quay Lu, Donald Song, Jason Lewis, Scott McGrath, William McGrath");
    MODULE_DESCRIPTION("A driver for the Aztech radio card.");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("1.0.0");
// acceptable ports: 0x350 (JP3 shorted), 0x358 (JP3 open)

pub const AZTECH_MAX: c_int = 2;
    static int io[AZTECH_MAX] = { [0] = CONFIG_RADIO_AZTECH_PORT,
    [1 ... (AZTECH_MAX - 1)] = -1 };
    static int radio_nr[AZTECH_MAX]	= { [0 ... (AZTECH_MAX - 1)] = -1 };
    module_param_array(io, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(io, "I/O addresses of the Aztech card (0x350 or 0x358)");
    module_param_array(radio_nr, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(radio_nr, "Radio device numbers");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aztech {
    pub isa: radio_isa_card,
    pub curvol: c_int,
}

// bit definitions for register read

// bit definitions for register write

// bits 0 and 2 are volume control, bits 3..5 are not connected
#[no_mangle]
unsafe extern "C" fn aztech_set_pins(handle: *mut c_void, pins: u8) {
    static void aztech_set_pins(void *handle, u8 pins)
    {
    struct radio_isa_card *isa = handle;
    struct aztech *az = container_of(isa, struct aztech, isa);
    let mut bits: u8 = az.curvol;
    if (pins & LM7000_DATA)
    bits |= AZTECH_BIT_TUN_DATA;
    if (pins & LM7000_CLK)
    bits |= AZTECH_BIT_TUN_CLK;
    if (pins & LM7000_CE)
    bits |= AZTECH_BIT_TUN_CE;
    outb_p(bits, az.isa.io);
    }
    static struct radio_isa_card *aztech_alloc(void)
    {
    struct aztech *az = kzalloc_obj(*az);
    return az ? &az.isa : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn aztech_s_frequency(isa: *mut radio_isa_card, freq: u32) -> c_int {
    static int aztech_s_frequency(struct radio_isa_card *isa, u32 freq)
    {
    lm7000_set_freq(freq, isa, aztech_set_pins);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aztech_g_rxsubchans(isa: *mut radio_isa_card) -> u32 {
    static u32 aztech_g_rxsubchans(struct radio_isa_card *isa)
    {
    if (inb(isa.io) & AZTECH_BIT_MONO)
    return V4L2_TUNER_SUB_MONO;
    return V4L2_TUNER_SUB_STEREO;
    }
#[no_mangle]
unsafe extern "C" fn aztech_g_signal(isa: *mut radio_isa_card) -> u32 {
    static u32 aztech_g_signal(struct radio_isa_card *isa)
    {
    return (inb(isa.io) & AZTECH_BIT_NOT_TUNED) ? 0 : 0xffff;
    }
#[no_mangle]
unsafe extern "C" fn aztech_s_mute_volume(isa: *mut radio_isa_card, mute: bool, vol: c_int) -> c_int {
    static int aztech_s_mute_volume(struct radio_isa_card *isa, bool mute, int vol)
    {
    struct aztech *az = container_of(isa, struct aztech, isa);
    if (mute)
    vol = 0;
    az.curvol = (vol & 1) + ((vol & 2) << 1);
    outb(az.curvol, isa.io);
    return 0;
    }
    static const struct radio_isa_ops aztech_ops = {
    .alloc = aztech_alloc,
    .s_mute_volume = aztech_s_mute_volume,
    .s_frequency = aztech_s_frequency,
    .g_rxsubchans = aztech_g_rxsubchans,
    .g_signal = aztech_g_signal,
    };
    static const int aztech_ioports[] = { 0x350, 0x358 };
    static struct radio_isa_driver aztech_driver = {
    .driver = {
    .match		= radio_isa_match,
    .probe		= radio_isa_probe,
    .remove		= radio_isa_remove,
    .driver		= {
    .name	= "radio-aztech",
    },
    },
    .io_params = io,
    .radio_nr_params = radio_nr,
    .io_ports = aztech_ioports,
    .num_of_io_ports = ARRAY_SIZE(aztech_ioports),
    .region_size = 8,
    .card = "Aztech Radio",
    .ops = &aztech_ops,
    .has_stereo = true,
    .max_volume = 3,
    };
#[no_mangle]
unsafe extern "C" fn aztech_init() -> int __init {
    static int __init aztech_init(void)
    {
    return isa_register_driver(&aztech_driver.driver, AZTECH_MAX);
    }
#[no_mangle]
unsafe extern "C" fn aztech_exit() -> void __exit {
    static void __exit aztech_exit(void)
    {
    isa_unregister_driver(&aztech_driver.driver);
    }
    module_init(aztech_init);
    module_exit(aztech_exit);
