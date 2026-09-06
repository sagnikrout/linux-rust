//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/radio-gemtek.c
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
// GemTek radio card driver
//
// Copyright 1998 Jonas Munsin <jmunsin@iki.fi>
//
// GemTek hasn't released any specs on the card, so the protocol had to
// be reverse engineered with dosemu.
//
// Besides the protocol changes, this is mostly a copy of:
//
// RadioTrack II driver for Linux radio support (C) 1998 Ben Pfaff
//
// Based on RadioTrack I/RadioReveal (C) 1997 M. Kirkwood
// Converted to new API by Alan Cox <alan@lxorguk.ukuu.org.uk>
// Various bugfixes and enhancements by Russell Kroll <rkroll@exploits.org>
//
// Converted to the radio-isa framework by Hans Verkuil <hverkuil@kernel.org>
// Converted to V4L2 API by Mauro Carvalho Chehab <mchehab@kernel.org>
//
// Note: this card seems to swap the left and right audio channels!
//
// Fully tested with the Keene USB FM Transmitter and the v4l2-compliance tool.
//

//
// Module info.
//
    MODULE_AUTHOR("Jonas Munsin, Pekka Seppänen <pexu@kapsi.fi>");
    MODULE_DESCRIPTION("A driver for the GemTek Radio card.");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("1.0.0");
//
// Module params.
//

pub const CONFIG_RADIO_GEMTEK_PROBE: c_int = 1;

pub const GEMTEK_MAX: c_int = 4;
    let mut probe: static bool = CONFIG_RADIO_GEMTEK_PROBE;
    static bool hardmute;
    static int io[GEMTEK_MAX] = { [0] = CONFIG_RADIO_GEMTEK_PORT,
    [1 ... (GEMTEK_MAX - 1)] = -1 };
    static int radio_nr[GEMTEK_MAX]	= { [0 ... (GEMTEK_MAX - 1)] = -1 };
    module_param(probe, bool, 0444);
    MODULE_PARM_DESC(probe, "Enable automatic device probing.");
    module_param(hardmute, bool, 0644);
    MODULE_PARM_DESC(hardmute, "Enable 'hard muting' by shutting down PLL, may reduce static noise.");
    module_param_array(io, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(io, "Force I/O ports for the GemTek Radio card if automatic probing is disabled or fails. The most common I/O ports are: 0x20c 0x30c, 0x24c or 0x34c (0x20c, 0x248 and 0x28c have been reported to work for the combined sound/radiocard).");
    module_param_array(radio_nr, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(radio_nr, "Radio device numbers");
//
// Frequency calculation constants.  Intermediate frequency 10.52 MHz (nominal
// value 10.7 MHz), reference divisor 6.39 kHz (nominal 6.25 kHz).
//
pub const FSCALE: c_int = 8;

pub const GEMTEK_CK: c_uint = 0x01	/* Clock signal			*/;
pub const GEMTEK_DA: c_uint = 0x02	/* Serial data			*/;
pub const GEMTEK_CE: c_uint = 0x04	/* Chip enable			*/;
pub const GEMTEK_NS: c_uint = 0x08	/* No signal			*/;
pub const GEMTEK_MT: c_uint = 0x10	/* Line mute			*/;
pub const GEMTEK_STDF_3_125_KHZ: c_uint = 0x01	/* Standard frequency 3.125 kHz	*/;
pub const GEMTEK_PLL_OFF: c_uint = 0x07	/* PLL off			*/;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gemtek {
    pub isa: radio_isa_card,
    pub muted: bool,
    pub bu2614data: u32,
}

pub const BU2614_FREQ_SHIFT: c_int = 0;

    BU2614_##field##_SHIFT)

//
// Set data which will be sent to BU2614FS.
//

    ((dev).bu2614data & ~field##_MASK) | ((data) << field##_SHIFT))
//
// Transmit settings to BU2614FS over GemTek IC.
//
#[no_mangle]
unsafe extern "C" fn gemtek_bu2614_transmit(gt: *mut gemtek) {
    static void gemtek_bu2614_transmit(struct gemtek *gt)
    {
    struct radio_isa_card *isa = &gt.isa;
    int i, bit, q, mute;
    mute = gt.muted ? GEMTEK_MT : 0x00;
    outb_p(mute | GEMTEK_CE | GEMTEK_DA | GEMTEK_CK, isa.io);
    udelay(LONG_DELAY);
    for (i = 0, q = gt.bu2614data; i < 32; i++, q >>= 1) {
    bit = (q & 1) ? GEMTEK_DA : 0;
    outb_p(mute | GEMTEK_CE | bit, isa.io);
    udelay(SHORT_DELAY);
    outb_p(mute | GEMTEK_CE | bit | GEMTEK_CK, isa.io);
    udelay(SHORT_DELAY);
    }
    outb_p(mute | GEMTEK_DA | GEMTEK_CK, isa.io);
    udelay(SHORT_DELAY);
    }
//
// Calculate divisor from FM-frequency for BU2614FS (3.125 KHz STDF expected).
//
#[no_mangle]
unsafe extern "C" fn gemtek_convfreq(freq: c_ulong) -> c_ulong {
    static unsigned long gemtek_convfreq(unsigned long freq)
    {
    return ((freq << FSCALE) + IF_OFFSET + REF_FREQ / 2) / REF_FREQ;
    }
    static struct radio_isa_card *gemtek_alloc(void)
    {
    struct gemtek *gt = kzalloc_obj(*gt);
    if (gt)
    gt.muted = true;
    return gt ? &gt.isa : core::ptr::null_mut();
    }
//
// Set FM-frequency.
//
#[no_mangle]
unsafe extern "C" fn gemtek_s_frequency(isa: *mut radio_isa_card, freq: u32) -> c_int {
    static int gemtek_s_frequency(struct radio_isa_card *isa, u32 freq)
    {
    struct gemtek *gt = container_of(isa, struct gemtek, isa);
    if (hardmute && gt.muted)
    return 0;
    gemtek_bu2614_set(gt, BU2614_PORT, 0);
    gemtek_bu2614_set(gt, BU2614_FMES, 0);
    gemtek_bu2614_set(gt, BU2614_SWIN, 0);	/* FM-mode	*/
    gemtek_bu2614_set(gt, BU2614_SWAL, 0);
    gemtek_bu2614_set(gt, BU2614_FMUN, 1);	/* GT bit set	*/
    gemtek_bu2614_set(gt, BU2614_TEST, 0);
    gemtek_bu2614_set(gt, BU2614_STDF, GEMTEK_STDF_3_125_KHZ);
    gemtek_bu2614_set(gt, BU2614_FREQ, gemtek_convfreq(freq));
    gemtek_bu2614_transmit(gt);
    return 0;
    }
//
// Set mute flag.
//
#[no_mangle]
unsafe extern "C" fn gemtek_s_mute_volume(isa: *mut radio_isa_card, mute: bool, vol: c_int) -> c_int {
    static int gemtek_s_mute_volume(struct radio_isa_card *isa, bool mute, int vol)
    {
    struct gemtek *gt = container_of(isa, struct gemtek, isa);
    int i;
    gt.muted = mute;
    if (hardmute) {
    if (!mute)
    return gemtek_s_frequency(isa, isa.freq);
// Turn off PLL, disable data output
    gemtek_bu2614_set(gt, BU2614_PORT, 0);
    gemtek_bu2614_set(gt, BU2614_FMES, 0);	/* CT bit off	*/
    gemtek_bu2614_set(gt, BU2614_SWIN, 0);	/* FM-mode	*/
    gemtek_bu2614_set(gt, BU2614_SWAL, 0);
    gemtek_bu2614_set(gt, BU2614_FMUN, 0);	/* GT bit off	*/
    gemtek_bu2614_set(gt, BU2614_TEST, 0);
    gemtek_bu2614_set(gt, BU2614_STDF, GEMTEK_PLL_OFF);
    gemtek_bu2614_set(gt, BU2614_FREQ, 0);
    gemtek_bu2614_transmit(gt);
    return 0;
    }
// Read bus contents (CE, CK and DA).
    i = inb_p(isa.io);
// Write it back with mute flag set.
    outb_p((i >> 5) | (mute ? GEMTEK_MT : 0), isa.io);
    udelay(SHORT_DELAY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gemtek_g_rxsubchans(isa: *mut radio_isa_card) -> u32 {
    static u32 gemtek_g_rxsubchans(struct radio_isa_card *isa)
    {
    if (inb_p(isa.io) & GEMTEK_NS)
    return V4L2_TUNER_SUB_MONO;
    return V4L2_TUNER_SUB_STEREO;
    }
//
// Check if requested card acts like GemTek Radio card.
//
#[no_mangle]
unsafe extern "C" fn gemtek_probe(isa: *mut radio_isa_card, io: c_int) -> bool {
    static bool gemtek_probe(struct radio_isa_card *isa, int io)
    {
    int i, q;
    q = inb_p(io);	/* Read bus contents before probing. */
// Try to turn on CE, CK and DA respectively and check if card responds
    properly. */
    for (i = 0; i < 3; ++i) {
    outb_p(1 << i, io);
    udelay(SHORT_DELAY);
    if ((inb_p(io) & ~GEMTEK_NS) != (0x17 | (1 << (i + 5))))
    return false;
    }
    outb_p(q >> 5, io);	/* Write bus contents back. */
    udelay(SHORT_DELAY);
    return true;
    }
    static const struct radio_isa_ops gemtek_ops = {
    .alloc = gemtek_alloc,
    .probe = gemtek_probe,
    .s_mute_volume = gemtek_s_mute_volume,
    .s_frequency = gemtek_s_frequency,
    .g_rxsubchans = gemtek_g_rxsubchans,
    };
    static const int gemtek_ioports[] = { 0x20c, 0x30c, 0x24c, 0x34c, 0x248, 0x28c };

    static const struct pnp_device_id gemtek_pnp_devices[] = {
// AOpen FX-3D/Pro Radio
    { .id = "ADS7183" },
    { }
    };
    MODULE_DEVICE_TABLE(pnp, gemtek_pnp_devices);

    static struct radio_isa_driver gemtek_driver = {
    .driver = {
    .match		= radio_isa_match,
    .probe		= radio_isa_probe,
    .remove		= radio_isa_remove,
    .driver		= {
    .name	= "radio-gemtek",
    },
    },

    .pnp_driver = {
    .name		= "radio-gemtek",
    .id_table	= gemtek_pnp_devices,
    .probe		= radio_isa_pnp_probe,
    .remove		= radio_isa_pnp_remove,
    },

    .io_params = io,
    .radio_nr_params = radio_nr,
    .io_ports = gemtek_ioports,
    .num_of_io_ports = ARRAY_SIZE(gemtek_ioports),
    .region_size = 1,
    .card = "GemTek Radio",
    .ops = &gemtek_ops,
    .has_stereo = true,
    };
#[no_mangle]
unsafe extern "C" fn gemtek_init() -> int __init {
    static int __init gemtek_init(void)
    {
    gemtek_driver.probe = probe;

    pnp_register_driver(&gemtek_driver.pnp_driver);

    return isa_register_driver(&gemtek_driver.driver, GEMTEK_MAX);
    }
#[no_mangle]
unsafe extern "C" fn gemtek_exit() -> void __exit {
    static void __exit gemtek_exit(void)
    {
    hardmute = true;	/* Turn off PLL */

    pnp_unregister_driver(&gemtek_driver.pnp_driver);

    isa_unregister_driver(&gemtek_driver.driver);
    }
    module_init(gemtek_init);
    module_exit(gemtek_exit);
