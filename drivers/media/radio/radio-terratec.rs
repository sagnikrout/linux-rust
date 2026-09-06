//! Automatically rewritten from C to Rust
//! Source: drivers/media/radio/radio-terratec.c
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
// Terratec ActiveRadio ISA Standalone card driver for Linux radio support
// (c) 1999 R. Offermanns (rolf@offermanns.de)
// based on the aimslab radio driver from M. Kirkwood
// many thanks to Michael Becker and Friedhelm Birth (from TerraTec)
//
// History:
// 1999-05-21	First preview release
//
// Notes on the hardware:
// There are two "main" chips on the card:
// - Philips OM5610 (http://www-us.semiconductors.philips.com/acrobat/datasheets/OM5610_2.pdf)
// - Philips SAA6588 (http://www-us.semiconductors.philips.com/acrobat/datasheets/SAA6588_1.pdf)
// (you can get the datasheet at the above links)
//
// Frequency control is done digitally -- ie out(port,encodefreq(95.8));
// Volume Control is done digitally
//
// Converted to the radio-isa framework by Hans Verkuil <hverkuil@kernel.org>
// Converted to V4L2 API by Mauro Carvalho Chehab <mchehab@kernel.org>
//

    MODULE_AUTHOR("R. Offermans & others");
    MODULE_DESCRIPTION("A driver for the TerraTec ActiveRadio Standalone radio card.");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("0.1.99");
// Note: there seems to be only one possible port (0x590), but without
    hardware this is hard to verify. For now, this is the only one we will
    support. */
    let mut io: static int = 0x590;
    let mut radio_nr: static int = -1;
    module_param(radio_nr, int, 0444);
    MODULE_PARM_DESC(radio_nr, "Radio device number");
pub const WRT_DIS: c_uint = 0x00;
pub const CLK_OFF: c_uint = 0x00;
pub const IIC_DATA: c_uint = 0x01;
pub const IIC_CLK: c_uint = 0x02;
pub const DATA: c_uint = 0x04;
pub const CLK_ON: c_uint = 0x08;
pub const WRT_EN: c_uint = 0x10;
    static struct radio_isa_card *terratec_alloc(void)
    {
    return kzalloc_obj(struct radio_isa_card);
    }
#[no_mangle]
unsafe extern "C" fn terratec_s_mute_volume(isa: *mut radio_isa_card, mute: bool, vol: c_int) -> c_int {
    static int terratec_s_mute_volume(struct radio_isa_card *isa, bool mute, int vol)
    {
    int i;
    if (mute)
    vol = 0;
    vol = vol + (vol * 32); /* change both channels */
    for (i = 0; i < 8; i++) {
    if (vol & (0x80 >> i))
    outb(0x80, isa.io + 1);
    else
    outb(0x00, isa.io + 1);
    }
    return 0;
    }
// this is the worst part in this driver
// many more or less strange things are going on here, but hey, it works :)
#[no_mangle]
unsafe extern "C" fn terratec_s_frequency(isa: *mut radio_isa_card, freq: u32) -> c_int {
    static int terratec_s_frequency(struct radio_isa_card *isa, u32 freq)
    {
    int i;
    int temp;
    long rest;
    unsigned char buffer[25];		/* we have to bit shift 25 registers */
    freq = freq / 160;			/* convert the freq. to a nice to handle value */
    memset(buffer, 0, sizeof(buffer));
    rest = freq * 10 + 10700;	/* I once had understood what is going on here */
// maybe some wise guy (friedhelm?) can comment this stuff
    i = 13;
    temp = 102400;
    while (rest != 0) {
    if (rest % temp  == rest)
    buffer[i] = 0;
    else {
    buffer[i] = 1;
    rest = rest - temp;
    }
    i--;
    temp = temp / 2;
    }
    for (i = 24; i > -1; i--) {	/* bit shift the values to the radiocard */
    if (buffer[i] == 1) {
    outb(WRT_EN | DATA, isa.io);
    outb(WRT_EN | DATA | CLK_ON, isa.io);
    outb(WRT_EN | DATA, isa.io);
    } else {
    outb(WRT_EN | 0x00, isa.io);
    outb(WRT_EN | 0x00 | CLK_ON, isa.io);
    }
    }
    outb(0x00, isa.io);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn terratec_g_signal(isa: *mut radio_isa_card) -> u32 {
    static u32 terratec_g_signal(struct radio_isa_card *isa)
    {
// bit set = no signal present
    return (inb(isa.io) & 2) ? 0 : 0xffff;
    }
    static const struct radio_isa_ops terratec_ops = {
    .alloc = terratec_alloc,
    .s_mute_volume = terratec_s_mute_volume,
    .s_frequency = terratec_s_frequency,
    .g_signal = terratec_g_signal,
    };
    static const int terratec_ioports[] = { 0x590 };
    static struct radio_isa_driver terratec_driver = {
    .driver = {
    .match		= radio_isa_match,
    .probe		= radio_isa_probe,
    .remove		= radio_isa_remove,
    .driver		= {
    .name	= "radio-terratec",
    },
    },
    .io_params = &io,
    .radio_nr_params = &radio_nr,
    .io_ports = terratec_ioports,
    .num_of_io_ports = ARRAY_SIZE(terratec_ioports),
    .region_size = 2,
    .card = "TerraTec ActiveRadio",
    .ops = &terratec_ops,
    .has_stereo = true,
    .max_volume = 10,
    };
#[no_mangle]
unsafe extern "C" fn terratec_init() -> int __init {
    static int __init terratec_init(void)
    {
    return isa_register_driver(&terratec_driver.driver, 1);
    }
#[no_mangle]
unsafe extern "C" fn terratec_exit() -> void __exit {
    static void __exit terratec_exit(void)
    {
    isa_unregister_driver(&terratec_driver.driver);
    }
    module_init(terratec_init);
    module_exit(terratec_exit);
