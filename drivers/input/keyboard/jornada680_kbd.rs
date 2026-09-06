//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/jornada680_kbd.c
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
// drivers/input/keyboard/jornada680_kbd.c
//
// HP Jornada 620/660/680/690 scan keyboard platform driver
// Copyright (C) 2007  Kristoffer Ericson <Kristoffer.Ericson@gmail.com>
//
// Based on hp680_keyb.c
// Copyright (C) 2006 Paul Mundt
// Copyright (C) 2005 Andriy Skulysh
// Split from drivers/input/keyboard/hp600_keyb.c
// Copyright (C) 2000 Yaegashi Takeshi (hp6xx kbd scan routine and translation table)
// Copyright (C) 2000 Niibe Yutaka (HP620 Keyb translation table)
//

pub const PCCR: c_uint = 0xa4000104;
pub const PDCR: c_uint = 0xa4000106;
pub const PECR: c_uint = 0xa4000108;
pub const PFCR: c_uint = 0xa400010a;
pub const PCDR: c_uint = 0xa4000124;
pub const PDDR: c_uint = 0xa4000126;
pub const PEDR: c_uint = 0xa4000128;
pub const PFDR: c_uint = 0xa400012a;
pub const PGDR: c_uint = 0xa400012c;
pub const PHDR: c_uint = 0xa400012e;
pub const PJDR: c_uint = 0xa4000130;
pub const PKDR: c_uint = 0xa4000132;
pub const PLDR: c_uint = 0xa4000134;
    static const unsigned short jornada_scancodes[] = {
// PTD1 */	KEY_CAPSLOCK, KEY_MACRO, KEY_LEFTCTRL, 0, KEY_ESC, KEY_KP5, 0, 0,			/*  1  -> 8
    KEY_F1, KEY_F2, KEY_F3, KEY_F8, KEY_F7, KEY_F6, KEY_F4, KEY_F5,				/*  9  . 16  */
// PTD5 */	KEY_SLASH, KEY_APOSTROPHE, KEY_ENTER, 0, KEY_Z, 0, 0, 0,				/*  17 -> 24
    KEY_X, KEY_C, KEY_V, KEY_DOT, KEY_COMMA, KEY_M, KEY_B, KEY_N,				/*  25 . 32  */
// PTD7 */	KEY_KP2, KEY_KP6, KEY_KP3, 0, 0, 0, 0, 0,						/*  33 -> 40
    KEY_F10, KEY_RO, KEY_F9, KEY_KP4, KEY_NUMLOCK, KEY_SCROLLLOCK, KEY_LEFTALT, KEY_HANJA,	/*  41 . 48  */
// PTE0 */	KEY_KATAKANA, KEY_KP0, KEY_GRAVE, 0, KEY_FINANCE, 0, 0, 0,				/*  49 -> 56
    KEY_KPMINUS, KEY_HIRAGANA, KEY_SPACE, KEY_KPDOT, KEY_VOLUMEUP, 249, 0, 0,		/*  57 . 64  */
// PTE1 */	KEY_SEMICOLON, KEY_RIGHTBRACE, KEY_BACKSLASH, 0, KEY_A, 0, 0, 0,			/*  65 -> 72
    KEY_S, KEY_D, KEY_F, KEY_L, KEY_K, KEY_J, KEY_G, KEY_H,					/*  73 . 80  */
// PTE3 */	KEY_KP8, KEY_LEFTMETA, KEY_RIGHTSHIFT, 0, KEY_TAB, 0, 0, 0,				/*  81 -> 88
    0, KEY_LEFTSHIFT, KEY_KP7, KEY_KP9, KEY_KP1, KEY_F11, KEY_KPPLUS, KEY_KPASTERISK,	/*  89 . 96  */
// PTE6 */	KEY_P, KEY_LEFTBRACE, KEY_BACKSPACE, 0, KEY_Q, 0, 0, 0,					/*  97 -> 104
    KEY_W, KEY_E, KEY_R, KEY_O, KEY_I, KEY_U, KEY_T, KEY_Y,					/* 105 . 112 */
// PTE7 */	KEY_0, KEY_MINUS, KEY_EQUAL, 0, KEY_1, 0, 0, 0,						/* 113 -> 120
    KEY_2, KEY_3, KEY_4, KEY_9, KEY_8, KEY_7, KEY_5, KEY_6,					/* 121 . 128 */
// **** */	0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0
    };
pub const JORNADA_SCAN_SIZE: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jornadakbd {
    pub input: *mut input_dev,
    pub keymap: [c_ushort; ARRAY_SIZE(jornada_scancodes)],
    pub length: c_uchar,
    pub old_scan: [c_uchar; JORNADA_SCAN_SIZE],
    pub new_scan: [c_uchar; JORNADA_SCAN_SIZE],
}

#[no_mangle]
unsafe extern "C" fn jornada_parse_kbd(jornadakbd: *mut jornadakbd) {
    static void jornada_parse_kbd(struct jornadakbd *jornadakbd)
    {
    struct input_dev *input_dev = jornadakbd.input;
    unsigned short *keymap = jornadakbd.keymap;
    let mut sync_me: c_uint = 0;
    unsigned int i, j;
    for (i = 0; i < JORNADA_SCAN_SIZE; i++) {
    let mut new: c_uchar = jornadakbd.new_scan[i];
    let mut old: c_uchar = jornadakbd.old_scan[i];
    let mut xor: c_uint = new ^ old;
    if (xor == 0)
    continue;
    for (j = 0; j < 8; j++) {
    let mut bit: c_uint = 1 << j;
    if (xor & bit) {
    let mut scancode: c_uint = (i << 3) + j;
    input_event(input_dev,
    EV_MSC, MSC_SCAN, scancode);
    input_report_key(input_dev,
    keymap[scancode],
    !(new & bit));
    sync_me = 1;
    }
    }
    }
    if (sync_me)
    input_sync(input_dev);
    }
#[no_mangle]
unsafe extern "C" fn jornada_scan_keyb(s: *mut c_uchar) {
    static void jornada_scan_keyb(unsigned char *s)
    {
    int i;
    unsigned short ec_static, dc_static; /* = UINT16_t */
    unsigned char matrix_switch[] = {
    0xfd, 0xff,   /* PTD1 PD(1) */
    0xdf, 0xff,   /* PTD5 PD(5) */
    0x7f, 0xff,   /* PTD7 PD(7) */
    0xff, 0xfe,   /* PTE0 PE(0) */
    0xff, 0xfd,   /* PTE1 PE(1) */
    0xff, 0xf7,   /* PTE3 PE(3) */
    0xff, 0xbf,   /* PTE6 PE(6) */
    0xff, 0x7f,   /* PTE7 PE(7) */
    }, *t = matrix_switch;
// PD(x) :
    1.   0xcc0c & (1~(1 << (2*(x)+1)))))
    2.   (0xf0cf & 0xfffff) */
// PE(x) :
    1.   0xcc0c & 0xffff
    2.   0xf0cf & (1~(1 << (2*(x)+1))))) */
    unsigned short matrix_PDE[] = {
    0xcc04, 0xf0cf,  /* PD(1) */
    0xc40c, 0xf0cf,	 /* PD(5) */
    0x4c0c, 0xf0cf,  /* PD(7) */
    0xcc0c, 0xf0cd,  /* PE(0) */
    0xcc0c, 0xf0c7,	 /* PE(1) */
    0xcc0c, 0xf04f,  /* PE(3) */
    0xcc0c, 0xd0cf,	 /* PE(6) */
    0xcc0c, 0x70cf,	 /* PE(7) */
    }, *y = matrix_PDE;
// Save these control reg bits
    dc_static = (__raw_readw(PDCR) & (~0xcc0c));
    ec_static = (__raw_readw(PECR) & (~0xf0cf));
    for (i = 0; i < 8; i++) {
// disable output for all but the one we want to scan
    __raw_writew((dc_static | *y++), PDCR);
    __raw_writew((ec_static | *y++), PECR);
    udelay(5);
// Get scanline row
    __raw_writeb(*t++, PDDR);
    __raw_writeb(*t++, PEDR);
    udelay(50);
// Read data
// s++ = __raw_readb(PCDR);
// s++ = __raw_readb(PFDR);
    }
// Scan no lines
    __raw_writeb(0xff, PDDR);
    __raw_writeb(0xff, PEDR);
// Enable all scanlines
    __raw_writew((dc_static | (0x5555 & 0xcc0c)),PDCR);
    __raw_writew((ec_static | (0x5555 & 0xf0cf)),PECR);
// Ignore extra keys and events
// s++ = __raw_readb(PGDR);
// s++ = __raw_readb(PHDR);
    }
#[no_mangle]
unsafe extern "C" fn jornadakbd680_poll(input: *mut input_dev) {
    static void jornadakbd680_poll(struct input_dev *input)
    {
    struct jornadakbd *jornadakbd = input_get_drvdata(input);
    jornada_scan_keyb(jornadakbd.new_scan);
    jornada_parse_kbd(jornadakbd);
    memcpy(jornadakbd.old_scan, jornadakbd.new_scan, JORNADA_SCAN_SIZE);
    }
#[no_mangle]
unsafe extern "C" fn jornada680kbd_probe(pdev: *mut platform_device) -> c_int {
    static int jornada680kbd_probe(struct platform_device *pdev)
    {
    struct jornadakbd *jornadakbd;
    struct input_dev *input_dev;
    int i, error;
    jornadakbd = devm_kzalloc(&pdev.dev, sizeof(struct jornadakbd),
    GFP_KERNEL);
    if (!jornadakbd)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(&pdev.dev);
    if (!input_dev) {
    dev_err(&pdev.dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    jornadakbd.input = input_dev;
    memcpy(jornadakbd.keymap, jornada_scancodes,
    sizeof(jornadakbd.keymap));
    input_set_drvdata(input_dev, jornadakbd);
    input_dev.evbit[0] = BIT(EV_KEY) | BIT(EV_REP);
    input_dev.name = "HP Jornada 680 keyboard";
    input_dev.phys = "jornadakbd/input0";
    input_dev.keycode = jornadakbd.keymap;
    input_dev.keycodesize = sizeof(unsigned short);
    input_dev.keycodemax = ARRAY_SIZE(jornada_scancodes);
    input_dev.id.bustype = BUS_HOST;
    for (i = 0; i < 128; i++)
    if (jornadakbd.keymap[i])
    __set_bit(jornadakbd.keymap[i], input_dev.keybit);
    __clear_bit(KEY_RESERVED, input_dev.keybit);
    input_set_capability(input_dev, EV_MSC, MSC_SCAN);
    error = input_setup_polling(input_dev, jornadakbd680_poll);
    if (error) {
    dev_err(&pdev.dev, "failed to set up polling\n");
    return error;
    }
    input_set_poll_interval(input_dev, 50 /* msec */);
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&pdev.dev, "failed to register input device\n");
    return error;
    }
    return 0;
    }
    static struct platform_driver jornada680kbd_driver = {
    .driver	= {
    .name	= "jornada680_kbd",
    },
    .probe	= jornada680kbd_probe,
    };
    module_platform_driver(jornada680kbd_driver);
    MODULE_AUTHOR("Kristoffer Ericson <kristoffer.ericson@gmail.com>");
    MODULE_DESCRIPTION("HP Jornada 620/660/680/690 Keyboard Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:jornada680_kbd");
