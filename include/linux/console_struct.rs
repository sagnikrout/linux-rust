//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/console_struct.h
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
//
// console_struct.h
//
// Data structure describing single virtual console except for data
// used by vt.c.
//
// Fields marked with [#] must be set by the low-level driver.
// Fields marked with [!] can be changed by the low-level driver
// to achieve effects such as fast scrolling by changing the origin.
//

pub const NPAR: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc_intensity {
    VCI_HALF_BRIGHT,
    VCI_NORMAL,
    VCI_BOLD,
    VCI_MASK = 0x3,
}

//
// struct vc_state -- state of a VC
// @x: cursor's x-position
// @y: cursor's y-position
// @color: foreground & background colors
// @Gx_charset: what's G0/G1 slot set to (like GRAF_MAP, LAT1_MAP)
// @charset: what character set to use (0=G0 or 1=G1)
// @intensity: see enum vc_intensity for values
// @reverse: reversed foreground/background colors
//
// These members are defined separately from struct vc_data as we save &
// restore them at times.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc_state {
    pub y: unsigned int x,,
    pub color: c_uchar,
    pub Gx_charset: [c_uchar; 2],
    pub 1: unsigned int charset :,
// attribute flags
    pub intensity: vc_intensity,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub reverse: bool,
}

//
// struct vc_font - Describes a font
// @width: The width of a single glyph in bits
// @height: The height of a single glyph in scanlines
// @charcount: The number of glyphs in the font
// @data: The raw font data
//
// Font data is organized as an array of glyphs. Each glyph is a bitmap with
// set bits indicating the foreground color. Unset bits indicate background
// color. The fields @width and @height store a single glyph's number of
// horizontal bits and vertical scanlines. If width is not a multiple of 8,
// there are trailing bits to fill up the byte. These bits should not be drawn.
//
// The field @data points to the first glyph's first byte. The value @charcount
// gives the number of glyphs in the font. There are no empty scanlines between
// two adjacent glyphs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc_font {
    pub width: c_uint,
    pub height: c_uint,
    pub charcount: c_uint,
    pub data: *const c_uchar,
}

extern "C" {
    pub fn vc_font_pitch(font: *const vc_font) -> c_uint;
}
extern "C" {
    pub fn vc_font_size(font: *const vc_font) -> c_uint;
}
//
// Example: vc_data of a console that was scrolled 3 lines down.
//
// Console buffer
// vc_screenbuf ---------> +----------------------+-.
// | initializing W       |  \
// | initializing X       |   |
// | initializing Y       |    > scroll-back area
// | initializing Z       |   |
// |                      |
// vc_visible_origin ---> ^+----------------------+-:
// (changes by scroll)    || Welcome to linux     |  \
// ||                      |   |
// vc_rows --->< | login: root          |   |  visible on console
// || password:            |    > (vc_screenbuf_size is
// vc_origin -----------> ||                      |   |   vc_size_row * vc_rows)
// (start when no scroll) || Last login: 12:28    |
// v+----------------------+-:
// | Have a lot of fun... |  \
// vc_pos -----------------|--------v             |   > scroll-front area
// | ~ # cat_             |
// vc_scr_end -----------> +----------------------+-:
// (vc_origin +            |                      |  \ EMPTY, to be filled by
// vc_screenbuf_size)     |                      |  / vc_video_erase_char
// +----------------------+-'
// <---- 2 * vc_cols ----->
// <---- vc_size_row ----->
//
// Note that every character in the console buffer is accompanied with an
// attribute in the buffer right after the character. This is not depicted
// in the figure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc_data {
    pub /: *mut *mut tty_port port; / Upper level data,
    pub saved_state: vc_state state,,
    pub /: *mut *mut unsigned short vc_num; / Console number,
    pub /: *mut *mut unsigned int vc_cols; / [#] Console size,
    pub vc_rows: c_uint,
    pub /: *mut *mut unsigned int vc_size_row; / Bytes per row,
    pub /: *mut *mut unsigned int vc_scan_lines; / # of scan lines,
    pub /: *mut *mut unsigned int vc_cell_height; / CRTC character cell height,
    pub /: *mut *mut unsigned long vc_origin; / [!] Start of real screen,
    pub /: *mut *mut unsigned long vc_scr_end; / [!] End of real screen,
    pub /: *mut *mut unsigned long vc_visible_origin; / [!] Top of visible window,
    pub /: *mut *mut unsigned int vc_top, vc_bottom; / Scrolling region,
    pub vc_sw: *const consw,
    pub /: *mut *mut *mut unsigned short vc_screenbuf; / In-memory character/attribute buffer,
    pub vc_screenbuf_size: c_uint,
    pub /: *mut *mut unsigned char vc_mode; / KD_TEXT, ...,
// attributes for all characters on screen
    pub /: *mut *mut unsigned char vc_attr; / Current attributes,
    pub /: *mut *mut unsigned char vc_def_color; / Default colors,
    pub /: *mut *mut unsigned char vc_ulcolor; / Color for underline mode,
    pub vc_itcolor: c_uchar,
    pub /: *mut *mut unsigned char vc_halfcolor; / Color for half intensity mode,
// cursor
    pub vc_cursor_type: c_uint,
    pub /: *mut *mut unsigned short vc_complement_mask; / [#] Xor mask for mouse pointer,
    pub /: *mut *mut unsigned short vc_s_complement_mask; / Saved mouse pointer mask,
    pub /: *mut *mut unsigned long vc_pos; / Cursor address,
// fonts
    pub /: *mut *mut unsigned short vc_hi_font_mask; / [#] Attribute set for upper 256 chars of font or 0 if not supported,
    pub /: *mut *mut vc_font vc_font; / Current VC font set,
    pub /: *mut *mut unsigned short vc_video_erase_char; / Background erase character,
// VT terminal data
    pub /: *mut *mut unsigned int vc_state; / Escape sequence parser state,
    pub /: *mut *mut unsigned int vc_npar,vc_par[NPAR]; / Parameters of current escape sequence,
// data for manual vt switching
    pub vt_mode: vt_mode,
    pub vt_pid: *mut pid,
    pub vt_newvt: c_int,
    pub paste_wait: wait_queue_head_t,
// mode flags
    pub /: *mut *mut unsigned int vc_disp_ctrl : 1; / Display chars < 32?,
    pub /: *mut *mut unsigned int vc_toggle_meta : 1; / Toggle high bit?,
    pub /: *mut *mut unsigned int vc_decscnm : 1; / Screen Mode,
    pub /: *mut *mut unsigned int vc_decom : 1; / Origin Mode,
    pub /: *mut *mut unsigned int vc_decawm : 1; / Autowrap Mode,
    pub /: *mut *mut unsigned int vc_deccm : 1; / Cursor Visible,
    pub /: *mut *mut unsigned int vc_decim : 1; / Insert Mode,
// misc
    pub 3: unsigned int vc_priv :,
    pub 1: unsigned int vc_need_wrap :,
    pub 1: unsigned int vc_can_do_color :,
    pub 2: unsigned int vc_report_mouse :,
    pub 1: unsigned int vc_bracketed_paste :,
    pub /: *mut *mut unsigned char vc_utf : 1; / Unicode UTF-8 encoding,
    pub vc_utf_count: c_uchar,
    pub vc_utf_char: c_int,
    pub /: *mut *mut DECLARE_BITMAP(vc_tab_stop, VC_TABSTOPS_COUNT); / Tab stops. 256 columns.,
    pub /: *mut *mut *mut unsigned char vc_palette[163]; / Colour palette for VGA+,
    pub vc_translate: *mut *mut c_ushort,
    pub /: *mut *mut unsigned int vc_bell_pitch; / Console bell pitch,
    pub /: *mut *mut unsigned int vc_bell_duration; / Console bell duration,
    pub /: *mut *mut unsigned short vc_cur_blink_ms; / Cursor blink duration,
    pub /: *mut *mut *mut *mut vc_data vc_display_fg; / [!] Ptr to var holding fg console for this display,
    pub uni_pagedict: *mut uni_pagedict,
    pub /: *mut *mut *mut *mut uni_pagedict uni_pagedict_loc; / [!] Location of uni_pagedict variable for this console,
    pub /: *mut *mut *mut *mut u32 vc_uni_lines; / unicode screen content,
    pub vc_saved_screen: *mut u16,
    pub vc_saved_uni_lines: *mut u32,
    pub vc_saved_cols: c_uint,
    pub vc_saved_rows: c_uint,
// additional information is in vt_kern.h
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc {
    pub d: *mut vc_data,
    pub SAK_work: work_struct,
// might add  scrmem, kbd  at some time,
}

extern "C" {
    pub fn vc_SAK(work: *mut work_struct);
}

pub const CUR_SW: c_uint = 0x000010;
pub const CUR_ALWAYS_BG: c_uint = 0x000020;
pub const CUR_INVERT_FG_BG: c_uint = 0x000040;
pub const CUR_FG: c_uint = 0x000700;
pub const CUR_BG: c_uint = 0x007000;

extern "C" {
    pub fn con_is_visible(vc: *const vc_data) -> bool;
}
