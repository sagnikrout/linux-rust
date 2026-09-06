//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fb.h
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

// Definitions of frame buffers

// ioctls
pub const FBIOGET_VSCREENINFO: c_uint = 0x4600;
pub const FBIOPUT_VSCREENINFO: c_uint = 0x4601;
pub const FBIOGET_FSCREENINFO: c_uint = 0x4602;
pub const FBIOGETCMAP: c_uint = 0x4604;
pub const FBIOPUTCMAP: c_uint = 0x4605;
pub const FBIOPAN_DISPLAY: c_uint = 0x4606;

// 0x4607-0x460B are defined below
// #define FBIOGET_MONITORSPEC	0x460C
// #define FBIOPUT_MONITORSPEC	0x460D
// #define FBIOSWITCH_MONIBIT	0x460E
pub const FBIOGET_CON2FBMAP: c_uint = 0x460F;
pub const FBIOPUT_CON2FBMAP: c_uint = 0x4610;
pub const FBIOBLANK: c_uint = 0x4611		/* arg: 0 or vesa level + 1 */;

pub const FBIO_ALLOC: c_uint = 0x4613;
pub const FBIO_FREE: c_uint = 0x4614;
pub const FBIOGET_GLYPH: c_uint = 0x4615;
pub const FBIOGET_HWCINFO: c_uint = 0x4616;
pub const FBIOPUT_MODEINFO: c_uint = 0x4617;
pub const FBIOGET_DISPINFO: c_uint = 0x4618;

pub const FB_ACCEL_SAVAGE4: c_uint = 0x80	/* S3 Savage4                   */;
pub const FB_ACCEL_SAVAGE3D: c_uint = 0x81	/* S3 Savage3D                  */;
pub const FB_ACCEL_SAVAGE3D_MV: c_uint = 0x82	/* S3 Savage3D-MV               */;
pub const FB_ACCEL_SAVAGE2000: c_uint = 0x83	/* S3 Savage2000                */;
pub const FB_ACCEL_SAVAGE_MX_MV: c_uint = 0x84	/* S3 Savage/MX-MV              */;
pub const FB_ACCEL_SAVAGE_MX: c_uint = 0x85	/* S3 Savage/MX                 */;
pub const FB_ACCEL_SAVAGE_IX_MV: c_uint = 0x86	/* S3 Savage/IX-MV              */;
pub const FB_ACCEL_SAVAGE_IX: c_uint = 0x87	/* S3 Savage/IX                 */;
pub const FB_ACCEL_PROSAVAGE_PM: c_uint = 0x88	/* S3 ProSavage PM133           */;
pub const FB_ACCEL_PROSAVAGE_KM: c_uint = 0x89	/* S3 ProSavage KM133           */;
pub const FB_ACCEL_S3TWISTER_P: c_uint = 0x8a	/* S3 Twister                   */;
pub const FB_ACCEL_S3TWISTER_K: c_uint = 0x8b	/* S3 TwisterK                  */;
pub const FB_ACCEL_SUPERSAVAGE: c_uint = 0x8c    /* S3 Supersavage               */;
pub const FB_ACCEL_PROSAVAGE_DDR: c_uint = 0x8d	/* S3 ProSavage DDR             */;
pub const FB_ACCEL_PROSAVAGE_DDRK: c_uint = 0x8e	/* S3 ProSavage DDR-K           */;
pub const FB_ACCEL_PUV3_UNIGFX: c_uint = 0xa0	/* PKUnity-v3 Unigfx		*/;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_fix_screeninfo {
    pub /: *mut *mut char id[16]; / identification string eg "TT Builtin",
    pub /: *mut *mut unsigned long smem_start; / Start of frame buffer mem,
// (physical address)
    pub /: *mut *mut __u32 smem_len; / Length of frame buffer mem,
    pub /: *mut *mut *mut __u32 type; / see FB_TYPE_,
    pub /: *mut *mut __u32 type_aux; / Interleave for interleaved Planes,
    pub /: *mut *mut *mut __u32 visual; / see FB_VISUAL_,
    pub /: *mut *mut __u16 xpanstep; / zero if no hardware panning,
    pub /: *mut *mut __u16 ypanstep; / zero if no hardware panning,
    pub /: *mut *mut __u16 ywrapstep; / zero if no hardware ywrap,
    pub /: *mut *mut __u32 line_length; / length of a line in bytes,
    pub /: *mut *mut unsigned long mmio_start; / Start of Memory Mapped I/O,
// (physical address)
    pub /: *mut *mut __u32 mmio_len; / Length of Memory Mapped I/O,
    pub /: *mut *mut __u32 accel; / Indicate to driver which,
// specific chip/card we have
    pub /: *mut *mut *mut __u16 capabilities; / see FB_CAP_,
    pub /: *mut *mut __u16 reserved[2]; / Reserved for future compatibility,
}

// Interpretation of offset for color fields: All offsets are from the right,
// inside a "pixel" value, which is exactly 'bits_per_pixel' wide (means: you
// can use the offset as right argument to <<). A pixel afterwards is a bit
// stream and is written to video memory as that unmodified.
//
// For pseudocolor: offset and length should be the same for all color
// components. Offset specifies the position of the least significant bit
// of the palette index in a pixel value. Length indicates the number
// of available palette entries (i.e. # of entries = 1 << length).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_bitfield {
    pub /: *mut *mut __u32 offset; / beginning of bitfield,
    pub /: *mut *mut __u32 length; / length of bitfield,
    pub /: *mut *mut __u32 msb_right; / != 0 : Most significant bit is,
// right
}

pub const FB_ACTIVATE_MASK: c_int = 15;
// values

// vtotal = 144d/288n/576i => PAL
// vtotal = 121d/242n/484i => NTSC

pub const FB_VMODE_MASK: c_int = 255;

//
// Display rotation support
//
pub const FB_ROTATE_UR: c_int = 0;
pub const FB_ROTATE_CW: c_int = 1;
pub const FB_ROTATE_UD: c_int = 2;
pub const FB_ROTATE_CCW: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_var_screeninfo {
    pub /: *mut *mut __u32 xres; / visible resolution,
    pub yres: __u32,
    pub /: *mut *mut __u32 xres_virtual; / virtual resolution,
    pub yres_virtual: __u32,
    pub /: *mut *mut __u32 xoffset; / offset from virtual to visible,
    pub /: *mut *mut __u32 yoffset; / resolution,
    pub /: *mut *mut __u32 bits_per_pixel; / guess what,
    pub /: *mut *mut __u32 grayscale; / 0 = color, 1 = grayscale,,
// >1 = FOURCC
    pub /: *mut *mut fb_bitfield red; / bitfield in fb mem if true color,,
    pub /: *mut *mut fb_bitfield green; / else only length is significant,
    pub blue: fb_bitfield,
    pub /: *mut *mut fb_bitfield transp; / transparency,
    pub /: *mut *mut __u32 nonstd; / != 0 Non standard pixel format,
    pub /: *mut *mut *mut __u32 activate; / see FB_ACTIVATE_,
    pub /: *mut *mut __u32 height; / height of picture in mm,
    pub /: *mut *mut __u32 width; / width of picture in mm,
    pub /: *mut *mut __u32 accel_flags; / (OBSOLETE) see fb_info.flags,
// Timing: All values in pixclocks, except pixclock (of course)
    pub /: *mut *mut __u32 pixclock; / pixel clock in ps (pico seconds),
    pub /: *mut *mut __u32 left_margin; / time from sync to picture,
    pub /: *mut *mut __u32 right_margin; / time from picture to sync,
    pub /: *mut *mut __u32 upper_margin; / time from sync to picture,
    pub lower_margin: __u32,
    pub /: *mut *mut __u32 hsync_len; / length of horizontal sync,
    pub /: *mut *mut __u32 vsync_len; / length of vertical sync,
    pub /: *mut *mut *mut __u32 sync; / see FB_SYNC_,
    pub /: *mut *mut *mut __u32 vmode; / see FB_VMODE_,
    pub /: *mut *mut __u32 rotate; / angle we rotate counter clockwise,
    pub /: *mut *mut __u32 colorspace; / colorspace for FOURCC-based modes,
    pub /: *mut *mut __u32 reserved[4]; / Reserved for future compatibility,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_cmap {
    pub /: *mut *mut __u32 start; / First entry,
    pub /: *mut *mut __u32 len; / Number of entries,
    pub /: *mut *mut *mut __u16 red; / Red values,
    pub green: *mut __u16,
    pub blue: *mut __u16,
    pub /: *mut *mut *mut __u16 transp; / transparency, can be NULL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_con2fbmap {
    pub console: __u32,
    pub framebuffer: __u32,
}

// screen: unblanked, hsync: on,  vsync: on
// screen: blanked,   hsync: on,  vsync: on
// screen: blanked,   hsync: on,  vsync: off
// screen: blanked,   hsync: off, vsync: on
// screen: blanked,   hsync: off, vsync: off
pub const FB_VBLANK_VBLANKING: c_uint = 0x001	/* currently in a vertical blank */;
pub const FB_VBLANK_HBLANKING: c_uint = 0x002	/* currently in a horizontal blank */;
pub const FB_VBLANK_HAVE_VBLANK: c_uint = 0x004	/* vertical blanks can be detected */;
pub const FB_VBLANK_HAVE_HBLANK: c_uint = 0x008	/* horizontal blanks can be detected */;
pub const FB_VBLANK_HAVE_COUNT: c_uint = 0x010	/* global retrace counter is available */;
pub const FB_VBLANK_HAVE_VCOUNT: c_uint = 0x020	/* the vcount field is valid */;
pub const FB_VBLANK_HAVE_HCOUNT: c_uint = 0x040	/* the hcount field is valid */;
pub const FB_VBLANK_VSYNCING: c_uint = 0x080	/* currently in a vsync */;
pub const FB_VBLANK_HAVE_VSYNC: c_uint = 0x100	/* vertical syncs can be detected */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_vblank {
    pub /: *mut *mut __u32 flags; / FB_VBLANK flags,
    pub /: *mut *mut __u32 count; / counter of retraces since boot,
    pub /: *mut *mut __u32 vcount; / current scanline position,
    pub /: *mut *mut __u32 hcount; / current scandot position,
    pub /: *mut *mut __u32 reserved[4]; / reserved for future compatibility,
}

// Internal HW accel
pub const ROP_COPY: c_int = 0;
pub const ROP_XOR: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_copyarea {
    pub dx: __u32,
    pub dy: __u32,
    pub width: __u32,
    pub height: __u32,
    pub sx: __u32,
    pub sy: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_fillrect {
    pub /: *mut *mut __u32 dx; / screen-relative,
    pub dy: __u32,
    pub width: __u32,
    pub height: __u32,
    pub color: __u32,
    pub rop: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_image {
    pub /: *mut *mut __u32 dx; / Where to place image,
    pub dy: __u32,
    pub /: *mut *mut __u32 width; / Size of image,
    pub height: __u32,
    pub /: *mut *mut __u32 fg_color; / Only used when a mono bitmap,
    pub bg_color: __u32,
    pub /: *mut *mut __u8 depth; / Depth of the image,
    pub /: *const *const *const char data; / Pointer to image data,
    pub /: *mut *mut fb_cmap cmap; / color map info,
}

//
// hardware cursor control
//
pub const FB_CUR_SETIMAGE: c_uint = 0x01;
pub const FB_CUR_SETPOS: c_uint = 0x02;
pub const FB_CUR_SETHOT: c_uint = 0x04;
pub const FB_CUR_SETCMAP: c_uint = 0x08;
pub const FB_CUR_SETSHAPE: c_uint = 0x10;
pub const FB_CUR_SETSIZE: c_uint = 0x20;
pub const FB_CUR_SETALL: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbcurpos {
    pub y: __u16 x,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_cursor {
    pub /: *mut *mut __u16 set; / what to set,
    pub /: *mut *mut __u16 enable; / cursor on/off,
    pub /: *mut *mut __u16 rop; / bitop operation,
    pub /: *const *const *const char mask; / cursor mask bits,
    pub /: *mut *mut fbcurpos hot; / cursor hot spot,
    pub /: *mut *mut fb_image image; / Cursor image,
}

// Settings for the generic backlight code
pub const FB_BACKLIGHT_LEVELS: c_int = 128;
pub const FB_BACKLIGHT_MAX: c_uint = 0xFF;
