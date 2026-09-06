//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fb.h
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

// Definitions below are used in the parsed monitor specs
pub const FB_DPMS_ACTIVE_OFF: c_int = 1;
pub const FB_DPMS_SUSPEND: c_int = 2;
pub const FB_DPMS_STANDBY: c_int = 4;
pub const FB_DISP_DDI: c_int = 1;
pub const FB_DISP_ANA_700_300: c_int = 2;
pub const FB_DISP_ANA_714_286: c_int = 4;
pub const FB_DISP_ANA_1000_400: c_int = 8;
pub const FB_DISP_ANA_700_000: c_int = 16;
pub const FB_DISP_MONO: c_int = 32;
pub const FB_DISP_RGB: c_int = 64;
pub const FB_DISP_MULTI: c_int = 128;
pub const FB_DISP_UNKNOWN: c_int = 256;
pub const FB_SIGNAL_NONE: c_int = 0;
pub const FB_SIGNAL_BLANK_BLANK: c_int = 1;
pub const FB_SIGNAL_SEPARATE: c_int = 2;
pub const FB_SIGNAL_COMPOSITE: c_int = 4;
pub const FB_SIGNAL_SYNC_ON_GREEN: c_int = 8;
pub const FB_SIGNAL_SERRATION_ON: c_int = 16;
pub const FB_MISC_PRIM_COLOR: c_int = 1;

pub const FB_MISC_HDMI: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_chroma {
    pub /: *mut *mut __u32 redx; / in fraction of 1024,
    pub greenx: __u32,
    pub bluex: __u32,
    pub whitex: __u32,
    pub redy: __u32,
    pub greeny: __u32,
    pub bluey: __u32,
    pub whitey: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_monspecs {
    pub chroma: fb_chroma,
    pub /: *mut *mut *mut fb_videomode modedb; / mode database,
    pub /: *mut *mut __u8 manufacturer[4]; / Manufacturer,
    pub /: *mut *mut __u8 monitor[14]; / Monitor String,
    pub /: *mut *mut __u8 serial_no[14]; / Serial Number,
    pub /: *mut *mut __u8 ascii[14]; / ?,
    pub /: *mut *mut __u32 modedb_len; / mode database length,
    pub /: *mut *mut __u32 model; / Monitor Model,
    pub /: *mut *mut __u32 serial; / Serial Number - Integer,
    pub /: *mut *mut __u32 year; / Year manufactured,
    pub /: *mut *mut __u32 week; / Week Manufactured,
    pub /: *mut *mut __u32 hfmin; / hfreq lower limit (Hz),
    pub /: *mut *mut __u32 hfmax; / hfreq upper limit (Hz),
    pub /: *mut *mut __u32 dclkmin; / pixelclock lower limit (Hz),
    pub /: *mut *mut __u32 dclkmax; / pixelclock upper limit (Hz),
    pub /: *mut *mut *mut __u16 input; / display type - see FB_DISP_,
    pub /: *mut *mut __u16 dpms; / DPMS support - see FB_DPMS_,
    pub /: *mut *mut *mut __u16 signal; / Signal Type - see FB_SIGNAL_,
    pub /: *mut *mut __u16 vfmin; / vfreq lower limit (Hz),
    pub /: *mut *mut __u16 vfmax; / vfreq upper limit (Hz),
    pub /: *mut *mut __u16 gamma; / Gamma - in fractions of 100,
    pub /: *mut *mut __u16 gtf : 1; / supports GTF,
    pub /: *mut *mut *mut __u16 misc; / Misc flags - see FB_MISC_,
    pub /: *mut *mut __u8 version; / EDID version...,
    pub /: *mut *mut __u8 revision; / ...and revision,
    pub /: *mut *mut __u8 max_x; / Maximum horizontal size (cm),
    pub /: *mut *mut __u8 max_y; / Maximum vertical size (cm),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_cmap_user {
    pub /: *mut *mut __u32 start; / First entry,
    pub /: *mut *mut __u32 len; / Number of entries,
    pub /: *mut *mut *mut __u16 __user red; / Red values,
    pub green: *mut __u16 __user,
    pub blue: *mut __u16 __user,
    pub /: *mut *mut *mut __u16 __user transp; / transparency, can be NULL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_image_user {
    pub /: *mut *mut __u32 dx; / Where to place image,
    pub dy: __u32,
    pub /: *mut *mut __u32 width; / Size of image,
    pub height: __u32,
    pub /: *mut *mut __u32 fg_color; / Only used when a mono bitmap,
    pub bg_color: __u32,
    pub /: *mut *mut __u8 depth; / Depth of the image,
    pub /: *const *const *const char __user data; / Pointer to image data,
    pub /: *mut *mut fb_cmap_user cmap; / color map info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_cursor_user {
    pub /: *mut *mut __u16 set; / what to set,
    pub /: *mut *mut __u16 enable; / cursor on/off,
    pub /: *mut *mut __u16 rop; / bitop operation,
    pub /: *const *const *const char __user mask; / cursor mask bits,
    pub /: *mut *mut fbcurpos hot; / cursor hot spot,
    pub /: *mut *mut fb_image_user image; / Cursor image,
}

//
// Register/unregister for framebuffer events
//

// only used by mach-pxa/am200epd.c
pub const FB_EVENT_FB_REGISTERED: c_uint = 0x05;
pub const FB_EVENT_FB_UNREGISTERED: c_uint = 0x06;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_event {
    pub info: *mut fb_info,
    pub data: *mut c_void,
}

// Enough for the VT console needs, see its max_font_width/height
pub const FB_MAX_BLIT_WIDTH: c_int = 64;
pub const FB_MAX_BLIT_HEIGHT: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_blit_caps {
    pub FB_MAX_BLIT_WIDTH): DECLARE_BITMAP(x,,
    pub FB_MAX_BLIT_HEIGHT): DECLARE_BITMAP(y,,
    pub len: u32,
    pub flags: u32,
}

extern "C" {
    pub fn fb_register_client(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn fb_unregister_client(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn fb_notifier_call_chain(val: c_ulong, v: *mut c_void) -> c_int;
}

//
// Pixmap structure definition
//
// The purpose of this structure is to translate data
// from the hardware independent format of fbdev to what
// format the hardware needs.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_pixmap {
    pub /: *mut *mut *mut u8 addr; / pointer to memory,
    pub /: *mut *mut u32 size; / size of buffer in bytes,
    pub /: *mut *mut u32 offset; / current offset to buffer,
    pub /: *mut *mut u32 buf_align; / byte alignment of each bitmap,
    pub /: *mut *mut u32 scan_align; / alignment per scanline,
    pub /: *mut *mut u32 access_align; / alignment per read/write (bits),
    pub /: *mut *mut *mut u32 flags; / see FB_PIXMAP_,
// supported bit block dimensions
// Format: test_bit(width - 1, blit_x)
// test_bit(height - 1, blit_y)
// if zero, will be set to full (all)
    pub FB_MAX_BLIT_WIDTH): DECLARE_BITMAP(blit_x,,
    pub FB_MAX_BLIT_HEIGHT): DECLARE_BITMAP(blit_y,,
// access methods
    pub size): *mut *mut *mut *mut *mut void (writeio)(struct fb_info info, void __iomem dst, void src, unsigned int,
    pub size): *mut *mut *mut *mut *mut void (readio) (struct fb_info info, void dst, void __iomem src, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_deferred_io_pageref {
    pub page: *mut page,
    pub offset: c_ulong,
// private
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_deferred_io {
// delay between mkwrite and deferred handler
    pub delay: c_ulong,
    pub /: *mut *mut bool sort_pagereflist; / sort pagelist by offset,
// callback
    pub offset): *mut *mut *mut *mut page (get_page)(fb_info info, unsigned long,
    pub pagelist): *mut *mut *mut void (deferred_io)(struct fb_info info, struct list_head,
}

//
// Frame buffer operations
//
// LOCKING NOTE: those functions must _ALL_ be called with the console
// semaphore held, this is the only suitable locking mechanism we have
// in 2.6. Some may be called at interrupt time at this point though.
//
// The exception to this is the debug related hooks.  Putting the fb
// into a debug state (e.g. flipping to the kernel console) and restoring
// it must be done in a lock-free manner, so low level drivers should
// keep track of the initial console (if applicable) and may need to
// perform direct, unlocked hardware writes in these hooks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_ops {
// open/release and usage marking
    pub owner: *mut module,
    pub user): *mut *mut *mut int (fb_open)(struct fb_info info, int,
    pub user): *mut *mut *mut int (fb_release)(struct fb_info info, int,
// For framebuffers with strange non linear layouts or that do not
// work with normal memory mapped access
//
    pub ppos): *mut size_t count, loff_t,
    pub ppos): *mut size_t count, loff_t,
// checks var and eventually tweaks it to something supported,
// DO NOT MODIFY PAR
    pub info): *mut *mut *mut int (fb_check_var)(struct fb_var_screeninfo var, struct fb_info,
// set the video mode according to info->var
    pub info): *mut *mut int (fb_set_par)(struct fb_info,
// set color register
    pub info): *mut unsigned blue, unsigned transp, struct fb_info,
// set color registers in batch
    pub info): *mut *mut *mut int (fb_setcmap)(struct fb_cmap cmap, struct fb_info,
// blank display
    pub info): *mut *mut int (fb_blank)(int blank, struct fb_info,
// pan display
    pub info): *mut *mut *mut int (fb_pan_display)(struct fb_var_screeninfo var, struct fb_info,
// Draws a rectangle
    pub rect): *const *const *const void (fb_fillrect) (struct fb_info info, struct fb_fillrect,
// Copy data from area to another
    pub region): *const *const *const void (fb_copyarea) (struct fb_info info, struct fb_copyarea,
// Draws a image to the display
    pub image): *const *const *const void (fb_imageblit) (struct fb_info info, struct fb_image,
// Draws cursor
    pub cursor): *mut *mut *mut int (fb_cursor) (struct fb_info info, struct fb_cursor,
// wait for blit idle, optional
    pub info): *mut *mut int (fb_sync)(struct fb_info,
// perform fb specific ioctl (optional)
    pub arg): c_ulong,
// Handle 32bit compat ioctl (optional)
    pub arg): c_ulong,
// perform fb specific mmap
    pub vma): *mut *mut *mut int (fb_mmap)(struct fb_info info, struct vm_area_struct,
// get capability given var
    pub var): *mut fb_var_screeninfo,
// teardown any resources to do with this framebuffer
    pub info): *mut *mut void (fb_destroy)(struct fb_info,
}

pub const FB_TILE_CURSOR_NONE: c_int = 0;
pub const FB_TILE_CURSOR_UNDERLINE: c_int = 1;
pub const FB_TILE_CURSOR_LOWER_THIRD: c_int = 2;
pub const FB_TILE_CURSOR_LOWER_HALF: c_int = 3;
pub const FB_TILE_CURSOR_TWO_THIRDS: c_int = 4;
pub const FB_TILE_CURSOR_BLOCK: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_tilemap {
    pub /: *mut *mut __u32 width; / width of each tile in pixels,
    pub /: *mut *mut __u32 height; / height of each tile in scanlines,
    pub /: *mut *mut __u32 depth; / color depth of each tile,
    pub /: *mut *mut __u32 length; / number of tiles in the map,
    pub packed: *const *const *const __u8 data; / actual tile map: a bitmap array,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_tilerect {
    pub /: *mut *mut __u32 sx; / origin in the x-axis,
    pub /: *mut *mut __u32 sy; / origin in the y-axis,
    pub /: *mut *mut __u32 width; / number of tiles in the x-axis,
    pub /: *mut *mut __u32 height; / number of tiles in the y-axis,
    pub /: *mut *mut __u32 index; / what tile to use: index to tile map,
    pub /: *mut *mut __u32 fg; / foreground color,
    pub /: *mut *mut __u32 bg; / background color,
    pub /: *mut *mut __u32 rop; / raster operation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_tilearea {
    pub /: *mut *mut __u32 sx; / source origin in the x-axis,
    pub /: *mut *mut __u32 sy; / source origin in the y-axis,
    pub /: *mut *mut __u32 dx; / destination origin in the x-axis,
    pub /: *mut *mut __u32 dy; / destination origin in the y-axis,
    pub /: *mut *mut __u32 width; / number of tiles in the x-axis,
    pub /: *mut *mut __u32 height; / number of tiles in the y-axis,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_tileblit {
    pub /: *mut *mut __u32 sx; / origin in the x-axis,
    pub /: *mut *mut __u32 sy; / origin in the y-axis,
    pub /: *mut *mut __u32 width; / number of tiles in the x-axis,
    pub /: *mut *mut __u32 height; / number of tiles in the y-axis,
    pub /: *mut *mut __u32 fg; / foreground color,
    pub /: *mut *mut __u32 bg; / background color,
    pub /: *mut *mut __u32 length; / number of tiles to draw,
    pub /: *mut *mut *mut __u32 indices; / array of indices to tile map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_tilecursor {
    pub /: *mut *mut __u32 sx; / cursor position in the x-axis,
    pub /: *mut *mut __u32 sy; / cursor position in the y-axis,
    pub /: *mut *mut __u32 mode; / 0 = erase, 1 = draw,
    pub /: *mut *mut *mut __u32 shape; / see FB_TILE_CURSOR_,
    pub /: *mut *mut __u32 fg; / foreground color,
    pub /: *mut *mut __u32 bg; / background color,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_tile_ops {
// set tile characteristics
    pub map): *mut *mut *mut void (fb_settile)(struct fb_info info, struct fb_tilemap,
// all dimensions from hereon are in terms of tiles
// move a rectangular region of tiles from one area to another
    pub area): *mut *mut *mut void (fb_tilecopy)(struct fb_info info, struct fb_tilearea,
// fill a rectangular region with a tile
    pub rect): *mut *mut *mut void (fb_tilefill)(struct fb_info info, struct fb_tilerect,
// copy an array of tiles
    pub blit): *mut *mut *mut void (fb_tileblit)(struct fb_info info, struct fb_tileblit,
// cursor
    pub cursor): *mut fb_tilecursor,
// get maximum length of the tile map
    pub info): *mut *mut int (fb_get_tilemax)(struct fb_info,
}

// FBINFO_* = fb_info.flags bit flags
pub const FBINFO_HWACCEL_DISABLED: c_uint = 0x0002;
// When FBINFO_HWACCEL_DISABLED is set:
// Hardware acceleration is turned off.  Software implementations
// of required functions (copyarea(), fillrect(), and imageblit())
// takes over; acceleration engine should be in a quiescent state
// hints
pub const FBINFO_VIRTFB: c_uint = 0x0004 /* FB is System RAM, not device. */;
pub const FBINFO_PARTIAL_PAN_OK: c_uint = 0x0040 /* otw use pan only for double-buffering */;
pub const FBINFO_READS_FAST: c_uint = 0x0080 /* soft-copy faster than rendering */;
// hardware supported ops
// semantics: when a bit is set, it indicates that the operation is
// accelerated by hardware.
// required functions will still work even if the bit is not set.
// optional functions may not even exist if the flag bit is not set.
//
pub const FBINFO_HWACCEL_NONE: c_uint = 0x0000;
pub const FBINFO_HWACCEL_COPYAREA: c_uint = 0x0100 /* required */;
pub const FBINFO_HWACCEL_FILLRECT: c_uint = 0x0200 /* required */;
pub const FBINFO_HWACCEL_IMAGEBLIT: c_uint = 0x0400 /* required */;
pub const FBINFO_HWACCEL_ROTATE: c_uint = 0x0800 /* optional */;
pub const FBINFO_HWACCEL_XPAN: c_uint = 0x1000 /* optional */;
pub const FBINFO_HWACCEL_YPAN: c_uint = 0x2000 /* optional */;
pub const FBINFO_HWACCEL_YWRAP: c_uint = 0x4000 /* optional */;
pub const FBINFO_MISC_TILEBLITTING: c_uint = 0x20000 /* use tile blitting */;
// A driver may set this flag to indicate that it does want a set_par to be
// called every time when fbcon_switch is executed. The advantage is that with
// this flag set you can really be sure that set_par is always called before
// any of the functions dependent on the correct hardware state or altering
// that state, even if you are using some broken X releases. The disadvantage
// is that it introduces unwanted delays to every console switch if set_par
// is slow. It is a good idea to try this flag in the drivers initialization
// code whenever there is a bug report related to switching between X and the
// framebuffer console.
//
pub const FBINFO_MISC_ALWAYS_SETPAR: c_uint = 0x40000;
//
// Host and GPU endianness differ.
//
pub const FBINFO_FOREIGN_ENDIAN: c_uint = 0x100000;
//
// Big endian math. This is the same flags as above, but with different
// meaning, it is set by the fb subsystem depending FOREIGN_ENDIAN flag
// and host endianness. Drivers should not use this flag.
//
pub const FBINFO_BE_MATH: c_uint = 0x100000;
//
// Hide smem_start in the FBIOGET_FSCREENINFO IOCTL. This is used by modern DRM
// drivers to stop userspace from trying to share buffers behind the kernel's
// back. Instead dma-buf based buffer sharing should be used.
//
pub const FBINFO_HIDE_SMEM_START: c_uint = 0x200000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_info {
    pub count: refcount_t,
    pub node: c_int,
    pub flags: c_int,
//
// -1 by default, set to a FB_ROTATE_* value by the driver, if it knows
// a lcd is not mounted upright and fbcon should rotate to compensate.
//
    pub fbcon_rotate_hint: c_int,
    pub /: *mut *mut mutex lock; / Lock for open/release/ioctl funcs,
    pub /: *mut *mut *mut mutex mm_lock; / Lock for fb_mmap and smem_ fields,
    pub /: *mut *mut fb_var_screeninfo var; / Current var,
    pub /: *mut *mut fb_fix_screeninfo fix; / Current fix,
    pub /: *mut *mut fb_monspecs monspecs; / Current Monitor specs,
    pub /: *mut *mut fb_pixmap pixmap; / Image hardware mapper,
    pub /: *mut *mut fb_pixmap sprite; / Cursor hardware mapper,
    pub /: *mut *mut fb_cmap cmap; / Current cmap,
    pub /: *mut *mut list_head modelist; / mode list,
    pub /: *mut *mut *mut fb_videomode mode; / current mode,
    pub /: *mut *mut int blank; / current blanking; see FB_BLANK_ constants,

// assigned backlight device
// set before framebuffer registration,
    pub bl_dev: *mut backlight_device,
// Backlight level curve
    pub bl_curve_mutex: mutex,
    pub bl_curve: [u8; FB_BACKLIGHT_LEVELS],
//
// Assigned LCD device; set before framebuffer
// registration, remove after unregister
//
    pub lcd_dev: *mut lcd_device,

    pub deferred_work: delayed_work,
    pub fbdefio: *mut fb_deferred_io,
    pub fbdefio_state: *mut fb_deferred_io_state,

    pub fbops: *const fb_ops,
    pub /: *mut *mut *mut device device; / This is the parent,

    pub /: *mut *mut *mut device dev; / This is this fb device,

    pub /: *mut *mut *mut fb_tile_ops tileops; / Tile Blitting,

    pub /: *mut *mut *mut char __iomem screen_base; / Virtual address,
    pub screen_buffer: *mut c_char,
}

pub const FBINFO_STATE_RUNNING: c_int = 0;
pub const FBINFO_STATE_SUSPENDED: c_int = 1;
// From here on everything is device dependent
// This will go away
// fbset currently hacks in FB_ACCELF_TEXT into var.accel_flags
// when it wants to turn the acceleration engine on.  This is
// really a separate operation, and should be modified via sysfs.
// But for now, we leave it broken with the following define
//
// Macro flag: #define STUPID_ACCELF_TEXT_SHIT

//
// `Generic' versions of the frame buffer device operations
//
extern "C" {
    pub fn fb_set_var(info: *mut fb_info, var: *mut fb_var_screeninfo) -> c_int;
}
extern "C" {
    pub fn fb_pan_display(info: *mut fb_info, var: *mut fb_var_screeninfo) -> c_int;
}
extern "C" {
    pub fn fb_blank(info: *mut fb_info, blank: c_int) -> c_int;
}
extern "C" {
    pub fn fb_set_var_from_user(info: *mut fb_info, var: *mut fb_var_screeninfo) -> c_int;
}
//
// Helpers for framebuffers in I/O memory
//
extern "C" {
    pub fn cfb_fillrect(info: *mut fb_info, rect: *const fb_fillrect);
}
extern "C" {
    pub fn cfb_copyarea(info: *mut fb_info, area: *const fb_copyarea);
}
extern "C" {
    pub fn cfb_imageblit(info: *mut fb_info, image: *const fb_image);
}
extern "C" {
    pub fn fb_io_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int;
}

//
// Helpers for framebuffers in system memory
//
extern "C" {
    pub fn sys_fillrect(info: *mut fb_info, rect: *const fb_fillrect);
}
extern "C" {
    pub fn sys_copyarea(info: *mut fb_info, area: *const fb_copyarea);
}
extern "C" {
    pub fn sys_imageblit(info: *mut fb_info, image: *const fb_image);
}

//
// Helpers for framebuffers in DMA-able memory
//

// fbmem.c
extern "C" {
    pub fn register_framebuffer(fb_info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn unregister_framebuffer(fb_info: *mut fb_info);
}
extern "C" {
    pub fn devm_register_framebuffer(dev: *mut device, fb_info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn fb_get_buffer_offset(info: *mut fb_info, buf: *mut fb_pixmap, size: u32) -> *mut c_char;
}
extern "C" {
    pub fn fb_pad_aligned_buffer(dst: *mut u8, d_pitch: u32, src: *const u8, s_pitch: u32, height: u32);
}
extern "C" {
    pub fn fb_set_suspend(info: *mut fb_info, state: c_int);
}
extern "C" {
    pub fn fb_switch_outputs(info: *mut fb_info);
}
extern "C" {
    pub fn fb_get_options(name: *const c_char, option: *mut c_char) -> c_int;
}
extern "C" {
    pub fn fb_new_modelist(info: *mut fb_info) -> c_int;
}

// s_pitch is a few bytes at the most, memcpy is suboptimal
// dst++ = *src++;
// fb_defio.c
extern "C" {
    pub fn fb_deferred_io_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn fb_deferred_io_init(info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn fb_deferred_io_release(info: *mut fb_info);
}
extern "C" {
    pub fn fb_deferred_io_cleanup(info: *mut fb_info);
}
//
// Generate callbacks for deferred I/O
//

//
// Initializes struct fb_ops for deferred I/O.
//

extern "C" {
    pub fn framebuffer_release(info: *mut fb_info);
}
extern "C" {
    pub fn fb_bl_default_curve(fb_info: *mut fb_info, off: u8, min: u8, max: u8);
}

extern "C" {
    pub fn fb_bl_notify_blank(info: *mut fb_info, old_blank: c_int);
}

// fbmon.c
pub const FB_MAXTIMINGS: c_int = 0;
pub const FB_VSYNCTIMINGS: c_int = 1;
pub const FB_HSYNCTIMINGS: c_int = 2;
pub const FB_DCLKTIMINGS: c_int = 3;
pub const FB_IGNOREMON: c_uint = 0x100;
pub const FB_MODE_IS_UNKNOWN: c_int = 0;
pub const FB_MODE_IS_DETAILED: c_int = 1;
pub const FB_MODE_IS_STANDARD: c_int = 2;
pub const FB_MODE_IS_VESA: c_int = 4;
pub const FB_MODE_IS_CALCULATED: c_int = 8;
pub const FB_MODE_IS_FIRST: c_int = 16;
pub const FB_MODE_IS_FROM_VAR: c_int = 32;
extern "C" {
    pub fn fbmon_dpms(fb_info: *const fb_info) -> c_int;
}
extern "C" {
    pub fn fb_parse_edid(edid: *mut c_uchar, var: *mut fb_var_screeninfo) -> c_int;
}
extern "C" {
    pub fn fb_destroy_modedb(modedb: *mut fb_videomode);
}
extern "C" {
    pub fn fb_find_mode_cvt(mode: *mut fb_videomode, margins: c_int, rb: c_int) -> c_int;
}
// modedb.c
pub const VESA_MODEDB_SIZE: c_int = 43;
pub const DMT_SIZE: c_uint = 0x50;
extern "C" {
    pub fn fb_destroy_modelist(head: *mut list_head);
}
// fbcmap.c
extern "C" {
    pub fn fb_alloc_cmap(cmap: *mut fb_cmap, len: c_int, transp: c_int) -> c_int;
}
extern "C" {
    pub fn fb_alloc_cmap_gfp(cmap: *mut fb_cmap, len: c_int, transp: c_int, flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn fb_dealloc_cmap(cmap: *mut fb_cmap);
}
extern "C" {
    pub fn fb_copy_cmap(from: *const fb_cmap, to: *mut fb_cmap) -> c_int;
}
extern "C" {
    pub fn fb_cmap_to_user(from: *const fb_cmap, to: *mut fb_cmap_user) -> c_int;
}
extern "C" {
    pub fn fb_set_cmap(cmap: *mut fb_cmap, fb_info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn fb_set_user_cmap(cmap: *mut fb_cmap_user, fb_info: *mut fb_info) -> c_int;
}
extern "C" {
    pub fn fb_invert_cmaps();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_videomode {
    pub /: *const *const *const char name; / optional,
    pub /: *mut *mut u32 refresh; / optional,
    pub xres: u32,
    pub yres: u32,
    pub pixclock: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub sync: u32,
    pub vmode: u32,
    pub flag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmt_videomode {
    pub dmt_id: u32,
    pub std_2byte_code: u32,
    pub cvt_3byte_code: u32,
    pub mode: *const fb_videomode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fb_modelist {
    pub list: list_head,
    pub mode: fb_videomode,
}

extern "C" {
    pub fn fb_modesetting_disabled(drvname: *const c_char) -> bool;
}
//
// Convenience logging macros
//

