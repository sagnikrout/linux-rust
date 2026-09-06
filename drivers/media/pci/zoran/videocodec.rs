//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/zoran/videocodec.h
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
// VIDEO MOTION CODECs internal API for video devices
//
// Interface for MJPEG (and maybe later MPEG/WAVELETS) codec's
// bound to a master device.
//
// (c) 2002 Wolfgang Scherr <scherr@net4you.at>
//
// ===================
// general description
// ===================
//
// Should ease the (re-)usage of drivers supporting cards with (different)
// video codecs. The codecs register to this module their functionality,
// and the processors (masters) can attach to them if they fit.
//
// The codecs are typically have a "strong" binding to their master - so I
// don't think it makes sense to have a full blown interfacing as with e.g.
// i2c. If you have an other opinion, let's discuss & implement it :-)))
//
// Usage:
//
// The slave has just to setup the videocodec structure and use two functions:
// videocodec_register(codecdata);
// videocodec_unregister(codecdata);
// The best is just calling them at module (de-)initialisation.
//
// The master sets up the structure videocodec_master and calls:
// codecdata=videocodec_attach(master_codecdata);
// videocodec_detach(codecdata);
//
// The slave is called during attach/detach via functions setup previously
// during register. At that time, the master_data pointer is set up
// and the slave can access any io registers of the master device (in the case
// the slave is bound to it). Otherwise it doesn't need this functions and
// therefor they may not be initialized.
//
// The other functions are just for convenience, as they are for sure used by
// most/all of the codecs. The last ones may be omitted, too.
//
// See the structure declaration below for more information and which data has
// to be set up for the master and the slave.
//
// ----------------------------------------------------------------------------
// The master should have "knowledge" of the slave and vice versa.  So the data
// structures sent to/from slave via set_data/get_data set_image/get_image are
// device dependent and vary between MJPEG/MPEG/WAVELET/... devices. (!!!!)
// ----------------------------------------------------------------------------
//
// ==========================================
// description of the videocodec_io structure
// ==========================================
//
// ==== master setup ====
// name -> name of the device structure for reference and debugging
// master_data ->  data ref. for the master (e.g. the zr36055,57,67)
// readreg -> ref. to read-fn from register (setup by master, used by slave)
// writereg -> ref. to write-fn to register (setup by master, used by slave)
// this two functions do the lowlevel I/O job
//
// ==== slave functionality setup ====
// slave_data -> data ref. for the slave (e.g. the zr36050,60)
// check -> fn-ref. checks availability of an device, returns -EIO on failure or
// the type on success
// this makes espcecially sense if a driver module supports more than
// one codec which may be quite similar to access, nevertheless it
// is good for a first functionality check
//
// -- main functions you always need for compression/decompression --
//
// set_mode -> this fn-ref. resets the entire codec, and sets up the mode
// with the last defined norm/size (or device default if not
// available) - it returns 0 if the mode is possible
// set_size -> this fn-ref. sets the norm and image size for
// compression/decompression (returns 0 on success)
// the norm param is defined in videodev2.h (V4L2_STD_*)
//
// additional setup may be available, too - but the codec should work with
// some default values even without this
//
// set_data -> sets device-specific data (tables, quality etc.)
// get_data -> query device-specific data (tables, quality etc.)
//
// if the device delivers interrupts, they may be setup/handled here
// setup_interrupt -> codec irq setup (not needed for 36050/60)
// handle_interrupt -> codec irq handling (not needed for 36050/60)
// if the device delivers pictures, they may be handled here
// put_image -> puts image data to the codec (not needed for 36050/60)
// get_image -> gets image data from the codec (not needed for 36050/60)
// the calls include frame numbers and flags (even/odd/...)
// if needed and a flag which allows blocking until its ready
//
// ==============
// user interface
// ==============
//
// Currently there is only a information display planned, as the layer
// is not visible for the user space at all.
//
// Information is available via procfs. The current entry is "/proc/videocodecs"
// but it makes sense to "hide" it in the /proc/video tree of v4l(2) --TODO--.
//
// A example for such an output is:
//
// <S>lave or attached <M>aster name  type flags    magic    (connected as)
// S                          zr36050 0002 0000d001 00000000 (TEMPLATE)
// M                       zr36055[0] 0001 0000c001 00000000 (zr36050[0])
// M                       zr36055[1] 0001 0000c001 00000000 (zr36050[1])
//
// ===============================================
// special defines for the videocodec_io structure
// ===============================================

pub const CODEC_DO_COMPRESSION: c_int = 0;
pub const CODEC_DO_EXPANSION: c_int = 1;
// this are the current codec flags I think they are needed
// -> type value in structure
pub const CODEC_FLAG_JPEG: c_uint = 0x00000001L	// JPEG codec;
pub const CODEC_FLAG_MPEG: c_uint = 0x00000002L	// MPEG1/2/4 codec;
pub const CODEC_FLAG_DIVX: c_uint = 0x00000004L	// DIVX codec;
pub const CODEC_FLAG_WAVELET: c_uint = 0x00000008L	// WAVELET codec;
// room for other types
pub const CODEC_FLAG_MAGIC: c_uint = 0x00000800L	// magic key must match;
pub const CODEC_FLAG_HARDWARE: c_uint = 0x00001000L	// is a hardware codec;
pub const CODEC_FLAG_VFE: c_uint = 0x00002000L	// has direct video frontend;
pub const CODEC_FLAG_ENCODER: c_uint = 0x00004000L	// compression capability;
pub const CODEC_FLAG_DECODER: c_uint = 0x00008000L	// decompression capability;
pub const CODEC_FLAG_NEEDIRQ: c_uint = 0x00010000L	// needs irq handling;
pub const CODEC_FLAG_RDWRPIC: c_uint = 0x00020000L	// handles picture I/O;
// a list of modes, some are just examples (is there any HW?)
pub const CODEC_MODE_BJPG: c_uint = 0x0001	// Baseline JPEG;
pub const CODEC_MODE_LJPG: c_uint = 0x0002	// Lossless JPEG;
pub const CODEC_MODE_MPEG1: c_uint = 0x0003	// MPEG 1;
pub const CODEC_MODE_MPEG2: c_uint = 0x0004	// MPEG 2;
pub const CODEC_MODE_MPEG4: c_uint = 0x0005	// MPEG 4;
pub const CODEC_MODE_MSDIVX: c_uint = 0x0006	// MS DivX;
pub const CODEC_MODE_ODIVX: c_uint = 0x0007	// Open DivX;
pub const CODEC_MODE_WAVELET: c_uint = 0x0008	// Wavelet;
// this are the current codec types I want to implement
// -> type value in structure
pub const CODEC_TYPE_NONE: c_int = 0;
pub const CODEC_TYPE_L64702: c_int = 1;
pub const CODEC_TYPE_ZR36050: c_int = 2;
pub const CODEC_TYPE_ZR36016: c_int = 3;
pub const CODEC_TYPE_ZR36060: c_int = 4;
// the type of data may be enhanced by future implementations (data-fn.'s)
// -> used in command
pub const CODEC_G_STATUS: c_uint = 0x0000	/* codec status (query only) */;
pub const CODEC_S_CODEC_MODE: c_uint = 0x0001	/* codec mode (baseline JPEG, MPEG1,... */;
pub const CODEC_G_CODEC_MODE: c_uint = 0x8001;
pub const CODEC_S_VFE: c_uint = 0x0002	/* additional video frontend setup */;
pub const CODEC_G_VFE: c_uint = 0x8002;
pub const CODEC_S_MMAP: c_uint = 0x0003	/* MMAP setup (if available) */;
pub const CODEC_S_JPEG_TDS_BYTE: c_uint = 0x0010	/* target data size in bytes */;
pub const CODEC_G_JPEG_TDS_BYTE: c_uint = 0x8010;
pub const CODEC_S_JPEG_SCALE: c_uint = 0x0011	/* scaling factor for quant. tables */;
pub const CODEC_G_JPEG_SCALE: c_uint = 0x8011;
pub const CODEC_S_JPEG_HDT_DATA: c_uint = 0x0018	/* huffman-tables */;
pub const CODEC_G_JPEG_HDT_DATA: c_uint = 0x8018;
pub const CODEC_S_JPEG_QDT_DATA: c_uint = 0x0019	/* quantizing-tables */;
pub const CODEC_G_JPEG_QDT_DATA: c_uint = 0x8019;
pub const CODEC_S_JPEG_APP_DATA: c_uint = 0x001A	/* APP marker */;
pub const CODEC_G_JPEG_APP_DATA: c_uint = 0x801A;
pub const CODEC_S_JPEG_COM_DATA: c_uint = 0x001B	/* COM marker */;
pub const CODEC_G_JPEG_COM_DATA: c_uint = 0x801B;
pub const CODEC_S_PRIVATE: c_uint = 0x1000	/* "private" commands start here */;
pub const CODEC_G_PRIVATE: c_uint = 0x9000;
pub const CODEC_G_FLAG: c_uint = 0x8000	/* this is how 'get' is detected */;
// types of transfer, directly user space or a kernel buffer (image-fn.'s)
// -> used in get_image, put_image

// =========================
// the structures itself ...
// =========================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfe_polarity {
    pub vsync_pol:1: c_uint,
    pub hsync_pol:1: c_uint,
    pub field_pol:1: c_uint,
    pub blank_pol:1: c_uint,
    pub subimg_pol:1: c_uint,
    pub poe_pol:1: c_uint,
    pub pvalid_pol:1: c_uint,
    pub vclk_pol:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfe_settings {
    pub /: *mut *mut __u32 x, y; / Offsets into image,
    pub /: *mut *mut __u32 width, height; / Area to capture,
    pub /: *mut *mut __u16 decimation; / Decimation divider,
    pub /: *mut *mut __u16 flags; / Flags for capture,
    pub /: *mut *mut __u16 quality; / quality of the video,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tvnorm {
    pub v_start: u16 wt, wa, h_start, h_sync_start, ht, ha,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_com_marker {
    pub /: *mut *mut int len; / number of usable bytes in data,
    pub data: [c_char; 60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_app_marker {
    pub /: *mut *mut int appn; / number app segment,
    pub /: *mut *mut int len; / number of usable bytes in data,
    pub data: [c_char; 60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct videocodec {
// -- filled in by slave device during register --
    pub name: [c_char; 32],
    pub /: *mut *mut unsigned long magic; / may be used for client<->master attaching,
    pub /: *mut *mut unsigned long flags; / functionality flags,
    pub /: *mut *mut unsigned int type; / codec type,
// -- these is filled in later during master device attach --
    pub master_data: *mut videocodec_master,
// -- these are filled in by the slave device during register --
    pub /: *mut *mut *mut void data; / private slave data,
// attach/detach client functions (indirect call)
    pub codec): *mut *mut int (setup)(struct videocodec,
    pub codec): *mut *mut int (unset)(struct videocodec,
// main functions, every client needs them for sure!
// set compression or decompression (or freeze, stop, standby, etc)
    pub mode): *mut *mut *mut int (set_mode)(struct videocodec codec, int,
// setup picture size and norm (for the codec's video frontend)
    pub pol): *mut *mut vfe_settings cap, vfe_polarity,
// other control commands, also mmap setup etc.
    pub data): *mut *mut *mut int (control)(struct videocodec codec, int type, int size, void,
// additional setup/query/processing (may be NULL pointer)
// interrupt setup / handling (for irq's delivered by master)
    pub mode): *mut *mut *mut int (setup_interrupt)(struct videocodec codec, long,
    pub flag): *mut *mut *mut int (handle_interrupt)(struct videocodec codec, int source, long,
// picture interface (if any)
    pub buf): *mut *mut *mut long fr_num, long flag, long size, void,
    pub buf): *mut *mut *mut long fr_num, long flag, long size, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct videocodec_master {
// -- filled in by master device for registration --
    pub name: [c_char; 32],
    pub /: *mut *mut unsigned long magic; / may be used for client<->master attaching,
    pub /: *mut *mut unsigned long flags; / functionality flags,
    pub /: *mut *mut unsigned int type; / master type,
    pub /: *mut *mut *mut void data; / private master data,
    pub reg): *mut *mut *mut __u32 (readreg)(struct videocodec codec, __u16,
    pub value): *mut *mut *mut void (writereg)(struct videocodec codec, __u16 reg, __u32,
}

// =================================================
// function prototypes of the master/slave interface
// =================================================
// attach and detach commands for the master
// * master structure needs to be kmalloc'ed before calling attach
// and free'd after calling detach
// * returns pointer on success, NULL on failure
// * 0 on success, <0 (errno) on failure
extern "C" {
    pub fn videocodec_detach(codec: *mut videocodec) -> c_int;
}
// register and unregister commands for the slaves
// * 0 on success, <0 (errno) on failure
extern "C" {
    pub fn videocodec_register(codec: *const videocodec) -> c_int;
}
// * 0 on success, <0 (errno) on failure
extern "C" {
    pub fn videocodec_unregister(codec: *const videocodec) -> c_int;
}
// the other calls are directly done via the videocodec structure!
extern "C" {
    pub fn videocodec_debugfs_show(m: *mut seq_file) -> c_int;
}

extern "C" {
    pub fn videocodec_master_to_zoran(_arg: master) -> return;
}
