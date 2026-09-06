//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/zoran/zoran.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// zoran - Iomega Buz driver
//
// Copyright (C) 1999 Rainer Johanni <Rainer@Johanni.de>
//
// based on
//
// zoran.0.0.3 Copyright (C) 1998 Dave Perks <dperks@ibm.net>
//
// and
//
// bttv - Bt848 frame grabber driver
// Copyright (C) 1996,97,98 Ralph  Metzler (rjkm@thp.uni-koeln.de)
// & Marcus Metzler (mocm@thp.uni-koeln.de)
//

pub const ZR_NORM_PAL: c_int = 0;
pub const ZR_NORM_NTSC: c_int = 1;
pub const ZR_NORM_SECAM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zr_buffer {
// common v4l buffer stuff -- must be first
    pub vbuf: vb2_v4l2_buffer,
    pub queue: list_head,
}

extern "C" {
    pub fn container_of(_arg: vbuf, zr_buffer: struct, _arg: vbuf) -> return;
}

pub const BUZ_NUM_STAT_COM: c_int = 4;
pub const BUZ_MASK_STAT_COM: c_int = 3;
pub const BUZ_MAX_INPUT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum card_type {
    UNKNOWN = -1,

// Pinnacle/Miro
    DC10_OLD,		/* DC30 like */
    DC10_NEW,		/* DC10_PLUS like */
    DC10_PLUS,
    DC30,
    DC30_PLUS,

// Linux Media Labs
    LML33,
    LML33R10,

// Iomega
    BUZ,

// AverMedia
    AVS6EYES,

// total number of cards
    NUM_CARDS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zoran_codec_mode {
    BUZ_MODE_IDLE,		/* nothing going on */
    BUZ_MODE_MOTION_COMPRESS,	/* grabbing frames */
    BUZ_MODE_MOTION_DECOMPRESS,	/* playing frames */
    BUZ_MODE_STILL_COMPRESS,	/* still frame conversion */
    BUZ_MODE_STILL_DECOMPRESS	/* still frame conversion */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zoran_map_mode {
    ZORAN_MAP_MODE_NONE,
    ZORAN_MAP_MODE_RAW,
    ZORAN_MAP_MODE_JPG_REC,
    ZORAN_MAP_MODE_JPG_PLAY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_type {
    ZR_GPIO_JPEG_SLEEP = 0,
    ZR_GPIO_JPEG_RESET,
    ZR_GPIO_JPEG_FRAME,
    ZR_GPIO_VID_DIR,
    ZR_GPIO_VID_EN,
    ZR_GPIO_VID_RESET,
    ZR_GPIO_CLK_SEL1,
    ZR_GPIO_CLK_SEL2,
    ZR_GPIO_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpcs_type {
    GPCS_JPEG_RESET = 0,
    GPCS_JPEG_START,
    GPCS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoran_format {
    pub name: *mut c_char,
    pub fourcc: __u32,
    pub colorspace: c_int,
    pub depth: c_int,
    pub flags: __u32,
    pub vfespfr: __u32,
}

// flags

// v4l-capture settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoran_v4l_settings {
    pub /: *mut *mut int width, height, bytesperline; / capture size,
    pub /: *const *const *const zoran_format format; / capture format,
}

// jpg-capture/-playback settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoran_jpg_settings {
// this bit is used to set everything to default
    pub decimation: c_int,
// capture decimation settings (tmp_dcm=1 means both fields)
    pub tmp_dcm: int hor_dcm, ver_dcm,,
// field-settings (odd_even=1 (+tmp_dcm=1) means top-field-first)
    pub odd_even: int field_per_buff,,
// crop settings (subframe capture)
    pub img_height: int img_x, img_y, img_width,,
// JPEG-specific capture settings
    pub jpg_comp: v4l2_jpegcompression,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct card_info {
    pub type: card_type,
    pub name: [c_char; 32],
    pub /: *const *const *const char i2c_decoder; / i2c decoder device,
    pub addrs_decoder: *const c_ushort,
    pub /: *const *const *const char i2c_encoder; / i2c encoder device,
    pub addrs_encoder: *const c_ushort,
    pub /: *mut *mut u16 video_vfe, video_codec; / videocodec types,
    pub /: *mut *mut u16 audio_chip; / audio type,
    pub /: *mut *mut int inputs; / number of video inputs,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input {
    pub muxsel: c_int,
    pub name: [c_char; 32],
    pub input: [}; BUZ_MAX_INPUT],
    pub norms: v4l2_std_id,
    pub /: *const *const *const tvnorm tvn[3]; / supported TV norms,
    pub /: *mut *mut u32 jpeg_int; / JPEG interrupt,
    pub /: *mut *mut u32 vsync_int; / VSYNC interrupt,
    pub gpio: [i8; ZR_GPIO_MAX],
    pub gpcs: [u8; GPCS_MAX],
    pub vfe_pol: vfe_polarity,
    pub gpio_pol: [u8; ZR_GPIO_MAX],
// is the /GWS line connected?
    pub gws_not_connected: u8,
// avs6eyes mux setting
    pub input_mux: u8,
    pub zr): *mut *mut void (init)(struct zoran,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoran {
    pub v4l2_dev: v4l2_device,
    pub hdl: v4l2_ctrl_handler,
    pub video_dev: *mut video_device,
    pub vq: vb2_queue,
    pub /: *mut *mut i2c_adapter i2c_adapter; /,
    pub /: *mut *mut i2c_algo_bit_data i2c_algo; /,
    pub i2cbr: u32,
    pub /: *mut *mut *mut v4l2_subdev decoder; / video decoder sub-device,
    pub /: *mut *mut *mut v4l2_subdev encoder; / video encoder sub-device,
    pub /: *mut *mut *mut videocodec codec; / video codec,
    pub /: *mut *mut *mut videocodec vfe; / video front end,
    pub /: *mut *mut mutex lock; / file ops serialize lock,
    pub /: *mut *mut u8 initialized; / flag if zoran has been correctly initialized,
    pub card: card_info,
    pub timing: *const tvnorm,
    pub /: *mut *mut unsigned short id; / number of this device,
    pub /: *mut *mut char name[40]; / name of this device,
    pub /: *mut *mut *mut pci_dev pci_dev; / PCI device,
    pub /: *mut *mut unsigned char revision; / revision of zr36057,
    pub /: *mut *mut *mut unsigned char __iomem zr36057_mem;/ pointer to mapped IO memory,
    pub /: *mut *mut spinlock_t spinlock; / Spinlock,
// Video for Linux parameters
    pub /: *mut *mut int input; / card's norm and input,
    pub norm: v4l2_std_id,
// Current buffer params
    pub buffer_size: c_uint,
    pub /: *mut *mut zoran_v4l_settings v4l_settings; / structure with a lot of things to play with,
// Buz MJPEG parameters
    pub /: *mut *mut zoran_codec_mode codec_mode; / status of codec,
    pub /: *mut *mut zoran_jpg_settings jpg_settings; / structure with a lot of things to play with,
// grab queue counts/indices, mask with BUZ_MASK_STAT_COM before using as index
// (dma_head - dma_tail) is number active in DMA, must be <= BUZ_NUM_STAT_COM
// (value & BUZ_MASK_STAT_COM) corresponds to index in stat_com table
    pub /: *mut *mut unsigned long jpg_que_head; / Index where to put next buffer which is queued,
    pub /: *mut *mut unsigned long jpg_dma_head; / Index of next buffer which goes into stat_com,
    pub /: *mut *mut unsigned long jpg_dma_tail; / Index of last buffer in stat_com,
    pub /: *mut *mut unsigned long jpg_que_tail; / Index of last buffer in queue,
    pub /: *mut *mut unsigned long jpg_seq_num; / count of frames since grab/play started,
    pub /: *mut *mut unsigned long jpg_err_seq; / last seq_num before error,
    pub jpg_err_shift: c_ulong,
    pub /: *mut *mut unsigned long jpg_queued_num; / count of frames queued since grab/play started,
    pub vbseq: c_ulong,
// zr36057's code buffer table
// stat_com[i] is indexed by dma_head/tail & BUZ_MASK_STAT_COM
    pub stat_com: *mut __le32,
// Additional stuff for testing
    pub ghost_int: c_uint,
    pub intr_counter_GIRQ1: c_int,
    pub intr_counter_GIRQ0: c_int,
    pub intr_counter_cod_rep_irq: c_int,
    pub intr_counter_jpeg_rep_irq: c_int,
    pub field_counter: c_int,
    pub irq1_in: c_int,
    pub irq1_out: c_int,
    pub jpeg_in: c_int,
    pub jpeg_out: c_int,
    pub JPEG_0: c_int,
    pub JPEG_1: c_int,
    pub end_event_missed: c_int,
    pub jpeg_missed: c_int,
    pub jpeg_error: c_int,
    pub num_errors: c_int,
    pub jpeg_max_missed: c_int,
    pub jpeg_min_missed: c_int,
    pub prepared: c_uint,
    pub queued: c_uint,
    pub last_isr: u32,
    pub frame_num: c_ulong,
    pub running: c_int,
    pub buf_in_reserve: c_int,
    pub p_sc: dma_addr_t,
    pub stat_comb: *mut __le32,
    pub p_scb: dma_addr_t,
    pub map_mode: zoran_map_mode,
    pub queued_bufs: list_head,
    pub /: *mut *mut spinlock_t queued_bufs_lock; / Protects queued_bufs,
    pub 2]: *mut *mut *mut zr_buffer inuse[BUZ_NUM_STAT_COM,
    pub dbgfs_dir: *mut dentry,
}

extern "C" {
    pub fn container_of(_arg: v4l2_dev, zoran: struct, _arg: v4l2_dev) -> return;
}
//
// There was something called _ALPHA_BUZ that used the PCI address instead of
// the kernel iomapped address for btread/btwrite.
//

//
// Debugging macros
//

extern "C" {
    pub fn zoran_queue_init(zr: *mut zoran, vq: *mut vb2_queue, dir: c_int) -> c_int;
}
extern "C" {
    pub fn zoran_queue_exit(zr: *mut zoran);
}
extern "C" {
    pub fn zr_set_buf(zr: *mut zoran) -> c_int;
}
