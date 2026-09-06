//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/kd.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// 0x4B is 'K', to avoid collision with termios and vt
pub const GIO_FONT: c_uint = 0x4B60	/* gets font in expanded form */;
pub const PIO_FONT: c_uint = 0x4B61	/* use font in expanded form */;
pub const GIO_FONTX: c_uint = 0x4B6B	/* get font using struct consolefontdesc */;
pub const PIO_FONTX: c_uint = 0x4B6C	/* set font using struct consolefontdesc */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct consolefontdesc {
    pub /: *mut *mut unsigned short charcount; / characters in font (256 or 512),
    pub /: *mut *mut unsigned short charheight; / scan lines per character (1-32),
    pub /: *mut *mut *mut char __user chardata; / font data in expanded form,
}

pub const PIO_FONTRESET: c_uint = 0x4B6D	/* reset to default font */;
pub const GIO_CMAP: c_uint = 0x4B70	/* gets colour palette on VGA+ */;
pub const PIO_CMAP: c_uint = 0x4B71	/* sets colour palette on VGA+ */;
pub const KIOCSOUND: c_uint = 0x4B2F	/* start sound generation (0 for off) */;
pub const KDMKTONE: c_uint = 0x4B30	/* generate tone */;
pub const KDGETLED: c_uint = 0x4B31	/* return current led state */;
pub const KDSETLED: c_uint = 0x4B32	/* set led state [lights, not flags] */;
pub const LED_SCR: c_uint = 0x01	/* scroll lock led */;
pub const LED_NUM: c_uint = 0x02	/* num lock led */;
pub const LED_CAP: c_uint = 0x04	/* caps lock led */;
pub const KDGKBTYPE: c_uint = 0x4B33	/* get keyboard type */;
pub const KB_84: c_uint = 0x01;
pub const KB_101: c_uint = 0x02 	/* this is what we always answer */;
pub const KB_OTHER: c_uint = 0x03;
pub const KDADDIO: c_uint = 0x4B34	/* add i/o port as valid */;
pub const KDDELIO: c_uint = 0x4B35	/* del i/o port as valid */;
pub const KDENABIO: c_uint = 0x4B36	/* enable i/o to video board */;
pub const KDDISABIO: c_uint = 0x4B37	/* disable i/o to video board */;
pub const KDSETMODE: c_uint = 0x4B3A	/* set text/graphics mode */;
pub const KD_TEXT: c_uint = 0x00;
pub const KD_GRAPHICS: c_uint = 0x01;
pub const KD_TEXT0: c_uint = 0x02	/* obsolete */;
pub const KD_TEXT1: c_uint = 0x03	/* obsolete */;
pub const KDGETMODE: c_uint = 0x4B3B	/* get current mode */;
pub const KDMAPDISP: c_uint = 0x4B3C	/* map display into address space */;
pub const KDUNMAPDISP: c_uint = 0x4B3D	/* unmap display from address space */;
pub type scrnmap_t = c_char;
pub const E_TABSZ: c_int = 256;
pub const GIO_SCRNMAP: c_uint = 0x4B40	/* get screen mapping from kernel */;
pub const PIO_SCRNMAP: c_uint = 0x4B41	/* put screen mapping table in kernel */;
pub const GIO_UNISCRNMAP: c_uint = 0x4B69	/* get full Unicode screen mapping */;
pub const PIO_UNISCRNMAP: c_uint = 0x4B6A  /* set full Unicode screen mapping */;
pub const GIO_UNIMAP: c_uint = 0x4B66	/* get unicode-to-font mapping from kernel */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unipair {
    pub unicode: c_ushort,
    pub fontpos: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unimapdesc {
    pub entry_ct: c_ushort,
    pub entries: *mut unipair __user,
}

pub const PIO_UNIMAP: c_uint = 0x4B67	/* put unicode-to-font mapping in kernel */;
pub const PIO_UNIMAPCLR: c_uint = 0x4B68	/* clear table, possibly advise hash algorithm */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unimapinit {
    pub /: *mut *mut unsigned short advised_hashsize; / 0 if no opinion,
    pub /: *mut *mut unsigned short advised_hashstep; / 0 if no opinion,
    pub /: *mut *mut unsigned short advised_hashlevel; / 0 if no opinion,
}

pub const UNI_DIRECT_BASE: c_uint = 0xF000	/* start of Direct Font Region */;
pub const UNI_DIRECT_MASK: c_uint = 0x01FF	/* Direct Font Region bitmask */;
pub const K_RAW: c_uint = 0x00;
pub const K_XLATE: c_uint = 0x01;
pub const K_MEDIUMRAW: c_uint = 0x02;
pub const K_UNICODE: c_uint = 0x03;
pub const K_OFF: c_uint = 0x04;
pub const KDGKBMODE: c_uint = 0x4B44	/* gets current keyboard mode */;
pub const KDSKBMODE: c_uint = 0x4B45	/* sets current keyboard mode */;
pub const K_METABIT: c_uint = 0x03;
pub const K_ESCPREFIX: c_uint = 0x04;
pub const KDGKBMETA: c_uint = 0x4B62	/* gets meta key handling mode */;
pub const KDSKBMETA: c_uint = 0x4B63	/* sets meta key handling mode */;
pub const K_SCROLLLOCK: c_uint = 0x01;
pub const K_NUMLOCK: c_uint = 0x02;
pub const K_CAPSLOCK: c_uint = 0x04;
pub const KDGKBLED: c_uint = 0x4B64	/* get led flags (not lights) */;
pub const KDSKBLED: c_uint = 0x4B65	/* set led flags (not lights) */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbentry {
    pub kb_table: c_uchar,
    pub kb_index: c_uchar,
    pub kb_value: c_ushort,
}

pub const K_NORMTAB: c_uint = 0x00;
pub const K_SHIFTTAB: c_uint = 0x01;
pub const K_ALTTAB: c_uint = 0x02;
pub const K_ALTSHIFTTAB: c_uint = 0x03;
pub const KDGKBENT: c_uint = 0x4B46	/* gets one entry in translation table */;
pub const KDSKBENT: c_uint = 0x4B47	/* sets one entry in translation table */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbsentry {
    pub kb_func: c_uchar,
    pub kb_string: [c_uchar; 512],
}

pub const KDGKBSENT: c_uint = 0x4B48	/* gets one function key string entry */;
pub const KDSKBSENT: c_uint = 0x4B49	/* sets one function key string entry */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbdiacr {
    pub result: unsigned char diacr, base,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbdiacrs {
    pub /: *mut *mut unsigned int kb_cnt; / number of entries in following array,
    pub /: *mut *mut kbdiacr kbdiacr[256]; / MAX_DIACR from keyboard.h,
}

pub const KDGKBDIACR: c_uint = 0x4B4A  /* read kernel accent table */;
pub const KDSKBDIACR: c_uint = 0x4B4B  /* write kernel accent table */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbdiacruc {
    pub result: unsigned int diacr, base,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbdiacrsuc {
    pub /: *mut *mut unsigned int kb_cnt; / number of entries in following array,
    pub /: *mut *mut kbdiacruc kbdiacruc[256]; / MAX_DIACR from keyboard.h,
}

pub const KDGKBDIACRUC: c_uint = 0x4BFA  /* read kernel accent table - UCS */;
pub const KDSKBDIACRUC: c_uint = 0x4BFB  /* write kernel accent table - UCS */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbkeycode {
    pub keycode: unsigned int scancode,,
}

pub const KDGETKEYCODE: c_uint = 0x4B4C	/* read kernel keycode table entry */;
pub const KDSETKEYCODE: c_uint = 0x4B4D	/* write kernel keycode table entry */;
pub const KDSIGACCEPT: c_uint = 0x4B4E	/* accept kbd generated signals */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbd_repeat {
    pub /: *mut *mut int delay; / in msec; <= 0: don't change,
    pub /: *mut *mut int period; / in msec; <= 0: don't change,
// earlier this field was misnamed "rate"
}

pub const KDKBDREP: c_uint = 0x4B52  /* set keyboard delay/repeat rate;;
// actually used values are returned
pub const KDFONTOP: c_uint = 0x4B72	/* font operations */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct console_font_op {
    pub /: *mut *mut *mut unsigned int op; / operation code KD_FONT_OP_,
    pub /: *mut *mut *mut unsigned int flags; / KD_FONT_FLAG_,
    pub /: *mut *mut unsigned int width, height; / font size,
    pub charcount: c_uint,
    pub for: *mut *mut *mut unsigned char __user data; / font data with vpitch fixed to 32,
// KD_FONT_OP_SET/GET
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct console_font {
    pub /: *mut *mut unsigned int width, height; / font size,
    pub charcount: c_uint,
    pub for: *mut *mut *mut unsigned char data; / font data with vpitch fixed to 32,
// KD_FONT_OP_SET/GET
//
}

// note: 0x4B00-0x4B4E all have had a value at some time;
// note: 0x4B60-0x4B6D, 0x4B70-0x4B72 used above
