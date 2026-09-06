//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ivtv/ivtv-driver.h
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

// Internal header for ivtv project:
// Driver for the cx23415/6 chip.
// Author: Kevin Thayer (nufan_wfk at yahoo.com)
// License: GPL
//
// -----
// MPG600/MPG160 support by  T.Adachi <tadachi@tadachi-net.com>
// and Takeru KOMORIYA<komoriya@paken.org>
//
// AVerMedia M179 GPIO info by Chris Pinkham <cpinkham@bc2va.org>
// using information provided by Jiun-Kuei Jung @ AVerMedia.
//

// Memory layout
pub const IVTV_ENCODER_OFFSET: c_uint = 0x00000000;
pub const IVTV_ENCODER_SIZE: c_uint = 0x00800000	/* Total size is 0x01000000, but only first half is used */;
pub const IVTV_DECODER_OFFSET: c_uint = 0x01000000;
pub const IVTV_DECODER_SIZE: c_uint = 0x00800000	/* Total size is 0x01000000, but only first half is used */;
pub const IVTV_REG_OFFSET: c_uint = 0x02000000;
pub const IVTV_REG_SIZE: c_uint = 0x00010000;
// Maximum ivtv driver instances. Some people have a huge number of
pub const IVTV_MAX_CARDS: c_int = 32;
pub const IVTV_ENC_STREAM_TYPE_MPG: c_int = 0;
pub const IVTV_ENC_STREAM_TYPE_YUV: c_int = 1;
pub const IVTV_ENC_STREAM_TYPE_VBI: c_int = 2;
pub const IVTV_ENC_STREAM_TYPE_PCM: c_int = 3;
pub const IVTV_ENC_STREAM_TYPE_RAD: c_int = 4;
pub const IVTV_DEC_STREAM_TYPE_MPG: c_int = 5;
pub const IVTV_DEC_STREAM_TYPE_VBI: c_int = 6;
pub const IVTV_DEC_STREAM_TYPE_VOUT: c_int = 7;
pub const IVTV_DEC_STREAM_TYPE_YUV: c_int = 8;
pub const IVTV_MAX_STREAMS: c_int = 9;

// DMA Registers

// Setup Registers

// Other registers

// debugging

// Flag to turn on high volume debugging

// Standard kernel messages

// output modes (cx23415 only)
pub const OUT_NONE: c_int = 0;
pub const OUT_MPG: c_int = 1;
pub const OUT_YUV: c_int = 2;
pub const OUT_UDMA_YUV: c_int = 3;
pub const OUT_PASSTHROUGH: c_int = 4;

// Default I2C SCL period in microseconds
pub const IVTV_DEFAULT_I2C_CLOCK_PERIOD: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_options {
    pub /: *mut *mut int kilobytes[IVTV_MAX_STREAMS]; / size in kilobytes of each stream,
    pub /: *mut *mut int cardtype; / force card type on load,
    pub /: *mut *mut int tuner; / set tuner on load,
    pub /: *mut *mut int radio; / enable/disable radio,
    pub /: *mut *mut int newi2c; / new I2C algorithm,
    pub /: *mut *mut int i2c_clock_period; / period of SCL for I2C bus,
}

// ivtv-specific mailbox template
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_mailbox {
    pub flags: u32,
    pub cmd: u32,
    pub retval: u32,
    pub timeout: u32,
    pub data: [u32; CX2341X_MBOX_MAX_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_api_cache {
    pub /: *mut *mut unsigned long last_jiffies; / when last command was issued,
    pub /: *mut *mut u32 data[CX2341X_MBOX_MAX_DATA]; / last sent api data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_mailbox_data {
    pub mbox: *mut volatile struct ivtv_mailbox __iomem,
// Bits 0-2 are for the encoder mailboxes, 0-1 are for the decoder mailboxes.
    pub busy: c_ulong,
    pub max_mbox: u8,
}

// per-buffer bit flags

// per-stream, s_flags

// per-ivtv, i_flags

// Event notifications

// Scatter-Gather array element, used in DMA transfers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_sg_element {
    pub src: __le32,
    pub dst: __le32,
    pub size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_sg_host_element {
    pub src: u32,
    pub dst: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_user_dma {
    pub lock: mutex,
    pub page_count: c_int,
    pub map: [*mut page; IVTV_DMA_SG_OSD_ENT],
// Needed when dealing with highmem userspace buffers
    pub bouncemap: [*mut page; IVTV_DMA_SG_OSD_ENT],
// Base Dev SG Array for cx23415/6
    pub SGarray: [ivtv_sg_element; IVTV_DMA_SG_OSD_ENT],
    pub SG_handle: dma_addr_t,
    pub SG_length: c_int,
// SG List of Buffers
    pub SGlist: [scatterlist; IVTV_DMA_SG_OSD_ENT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_dma_page_info {
    pub uaddr: c_ulong,
    pub first: c_ulong,
    pub last: c_ulong,
    pub offset: c_uint,
    pub tail: c_uint,
    pub page_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_buffer {
    pub list: list_head,
    pub dma_handle: dma_addr_t,
    pub b_flags: c_ushort,
    pub dma_xfer_cnt: c_ushort,
    pub buf: *mut c_char,
    pub bytesused: u32,
    pub readpos: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_queue {
    pub /: *mut *mut list_head list; / the list of buffers in this queue,
    pub /: *mut *mut u32 buffers; / number of buffers in this queue,
    pub /: *mut *mut u32 length; / total number of bytes of available buffer space,
    pub /: *mut *mut u32 bytesused; / total number of bytes used in this queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_stream {
// These first four fields are always set, even if the stream
    pub /: *mut *mut video_device vdev; / vdev.v4l2_dev is NULL if there is no device,
    pub /: *mut *mut *mut ivtv itv; / for ease of use,
    pub /: *const *const *const char name; / name of the stream,
    pub /: *mut *mut int type; / stream type,
    pub /: *mut *mut *mut ivtv_open_id id; / pointer to the streaming ivtv_open_id,
    pub /: *mut *mut spinlock_t qlock; / locks access to the queues,
    pub /: *mut *mut unsigned long s_flags; / status flags, see above,
    pub /: *mut *mut int dma; / can be PCI_DMA_TODEVICE, PCI_DMA_FROMDEVICE or PCI_DMA_NONE,
    pub pending_offset: u32,
    pub pending_backup: u32,
    pub pending_pts: u64,
    pub dma_offset: u32,
    pub dma_backup: u32,
    pub dma_pts: u64,
    pub subtype: c_int,
    pub waitq: wait_queue_head_t,
    pub dma_last_offset: u32,
// Buffer Stats
    pub buffers: u32,
    pub buf_size: u32,
    pub buffers_stolen: u32,
// Buffer Queues
    pub /: *mut *mut ivtv_queue q_free; / free buffers,
    pub /: *mut *mut ivtv_queue q_full; / full buffers,
    pub /: *mut *mut ivtv_queue q_io; / waiting for I/O,
    pub /: *mut *mut ivtv_queue q_dma; / waiting for DMA,
    pub /: *mut *mut ivtv_queue q_predma; / waiting for DMA,
// DMA xfer counter, buffers belonging to the same DMA
    pub dma_xfer_cnt: u16,
// Base Dev SG Array for cx23415/6
    pub sg_pending: *mut ivtv_sg_host_element,
    pub sg_processing: *mut ivtv_sg_host_element,
    pub sg_dma: *mut ivtv_sg_element,
    pub sg_handle: dma_addr_t,
    pub sg_pending_size: c_int,
    pub sg_processing_size: c_int,
    pub sg_processed: c_int,
// SG List of Buffers
    pub SGlist: *mut scatterlist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_open_id {
    pub fh: v4l2_fh,
    pub /: *mut *mut int type; / stream type,
    pub /: *mut *mut int yuv_frames; / 1: started OUT_UDMA_YUV output mode,
    pub itv: *mut ivtv,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), ivtv_open_id: struct, _arg: fh) -> return;
}
pub const IVTV_YUV_MODE_INTERLACED: c_uint = 0x00;
pub const IVTV_YUV_MODE_PROGRESSIVE: c_uint = 0x01;
pub const IVTV_YUV_MODE_AUTO: c_uint = 0x02;
pub const IVTV_YUV_MODE_MASK: c_uint = 0x03;
pub const IVTV_YUV_SYNC_EVEN: c_uint = 0x00;
pub const IVTV_YUV_SYNC_ODD: c_uint = 0x04;
pub const IVTV_YUV_SYNC_MASK: c_uint = 0x04;
pub const IVTV_YUV_BUFFERS: c_int = 8;
pub const IVTV_VBI_FRAMES: c_int = 32;
// VBI data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbi_cc {
    pub /: *mut *mut u8 odd[2]; / two-byte payload of odd field,
    pub /: *mut *mut u8 even[2]; / two-byte payload of even field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbi_vps {
    pub /: *mut *mut u8 data[5]; / five-byte VPS payload,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbi_info {
// VBI general data, does not change during streaming
    pub /: *mut *mut u32 raw_decoder_line_size; / raw VBI line size from digitizer,
    pub /: *mut *mut u8 raw_decoder_sav_odd_field; / raw VBI Start Active Video digitizer code of odd field,
    pub /: *mut *mut u8 raw_decoder_sav_even_field; / raw VBI Start Active Video digitizer code of even field,
    pub /: *mut *mut u32 sliced_decoder_line_size; / sliced VBI line size from digitizer,
    pub /: *mut *mut u8 sliced_decoder_sav_odd_field; / sliced VBI Start Active Video digitizer code of odd field,
    pub /: *mut *mut u8 sliced_decoder_sav_even_field; / sliced VBI Start Active Video digitizer code of even field,
    pub /: *mut *mut u32 start[2]; / start of first VBI line in the odd/even fields,
    pub /: *mut *mut u32 count; / number of VBI lines per field,
    pub /: *mut *mut u32 raw_size; / size of raw VBI line from the digitizer,
    pub /: *mut *mut u32 sliced_size; / size of sliced VBI line from the digitizer,
    pub /: *mut *mut u32 dec_start; / start in decoder memory of VBI re-insertion buffers,
    pub /: *mut *mut u32 enc_start; / start in encoder memory of VBI capture buffers,
    pub /: *mut *mut u32 enc_size; / size of VBI capture area,
    pub /: *mut *mut int fpi; / number of VBI frames per interrupt,
    pub /: *mut *mut v4l2_format in; / current VBI capture format,
    pub /: *mut *mut *mut v4l2_sliced_vbi_format sliced_in; / convenience pointer to sliced in vbi.in union,
    pub /: *mut *mut int insert_mpeg; / if non-zero, then embed VBI data in MPEG stream,
// Raw VBI compatibility hack
    pub compatibility: *mut *mut u32 frame; / frame counter hack needed for backwards,
// Sliced VBI output data
    pub to: *mut *mut vbi_cc cc_payload[256]; / sliced VBI CC payload array: it is an array,
    pub /: *mut *mut int cc_payload_idx; / index in cc_payload,
    pub /: *mut *mut u8 cc_missing_cnt; / counts number of frames without CC for passthrough mode,
    pub /: *mut *mut int wss_payload; / sliced VBI WSS payload,
    pub /: *mut *mut u8 wss_missing_cnt; / counts number of frames without WSS for passthrough mode,
    pub /: *mut *mut vbi_vps vps_payload; / sliced VBI VPS payload,
// Sliced VBI capture data
    pub /: *mut *mut v4l2_sliced_vbi_data sliced_data[36]; / sliced VBI storage for VBI encoder stream,
    pub /: *mut *mut v4l2_sliced_vbi_data sliced_dec_data[36];/ sliced VBI storage for VBI decoder stream,
// VBI Embedding data
// Buffer for VBI data inserted into MPEG stream.
    pub sliced_mpeg_data: [*mut u8; IVTV_VBI_FRAMES],
    pub sliced_mpeg_size: [u32; IVTV_VBI_FRAMES],
    pub /: *mut *mut ivtv_buffer sliced_mpeg_buf; / temporary buffer holding data from sliced_mpeg_data,
    pub data: *mut *mut u32 inserted_frame; / index in sliced_mpeg_size of next sliced,
}

// forward declaration of struct defined in ivtv-cards.h
// Struct to hold info about ivtv cards
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv {
// General fixed card data
    pub /: *mut *mut *mut pci_dev pdev; / PCI device,
    pub /: *const *const *const ivtv_card card; / card information,
    pub /: *const *const *const char card_name; / full name of the card,
    pub /: *const *const *const ivtv_card_tuner_i2c card_i2c; / i2c addresses to probe for tuner,
    pub /: *mut *mut u8 has_cx23415; / 1 if it is a cx23415 based card, 0 for cx23416,
    pub /: *mut *mut u8 pvr150_workaround; / 1 if the cx25840 needs to workaround a PVR150 bug,
    pub /: *mut *mut u8 nof_inputs; / number of video inputs,
    pub /: *mut *mut u8 nof_audio_inputs; / number of audio inputs,
    pub /: *mut *mut u32 v4l2_cap; / V4L2 capabilities of card,
    pub /: *mut *mut u32 hw_flags; / hardware description of the board,
    pub /: *mut *mut v4l2_std_id tuner_std; / the norm of the card's tuner (fixed),
    pub /: *mut *mut *mut v4l2_subdev sd_video; / controlling video decoder subdev,
    pub /: *mut *mut bool sd_video_is_streaming; / is video already streaming?,
    pub /: *mut *mut *mut v4l2_subdev sd_audio; / controlling audio subdev,
    pub /: *mut *mut *mut v4l2_subdev sd_muxer; / controlling audio muxer subdev,
    pub /: *mut *mut resource_size_t base_addr; / PCI resource base address,
    pub /: *mut *mut *mut volatile void __iomem enc_mem; / pointer to mapped encoder memory,
    pub /: *mut *mut *mut volatile void __iomem dec_mem; / pointer to mapped decoder memory,
    pub /: *mut *mut *mut volatile void __iomem reg_mem; / pointer to mapped registers,
    pub /: *mut *mut ivtv_options options; / user options,
    pub v4l2_dev: v4l2_device,
    pub cxhdl: cx2341x_handler,
// PTS/Frame count control cluster
    pub ctrl_pts: *mut v4l2_ctrl,
    pub ctrl_frame: *mut v4l2_ctrl,
}

// Audio Playback control cluster
// High-level state info
// Locking
// Streams
// ALSA interface for PCM capture stream
// Used for ivtv-alsa module loading
// Interrupts & DMA
// Mailbox
// I2C
// Program Index information
// Miscellaneous
// VBI state info
// YUV playback
// OSD support
extern "C" {
    pub fn container_of(_arg: v4l2_dev, ivtv: struct, _arg: v4l2_dev) -> return;
}
// ivtv extensions to be loaded
extern "C" {
    pub fn int(: *mut *mut ivtv_ext_init)(struct ivtv) -> extern;
}
// Globals
// ==============Prototypes==================
// Hardware/IRQ
extern "C" {
    pub fn ivtv_set_irq_mask(itv: *mut ivtv, mask: u32);
}
extern "C" {
    pub fn ivtv_clear_irq_mask(itv: *mut ivtv, mask: u32);
}
// try to set output mode, return current mode.
extern "C" {
    pub fn ivtv_set_output_mode(itv: *mut ivtv, mode: c_int) -> c_int;
}
// return current output stream based on current mode
// Return non-zero if a signal is pending
extern "C" {
    pub fn ivtv_msleep_timeout(msecs: c_uint, intr: c_int) -> c_int;
}
// Wait on queue, returns -EINTR if interrupted
extern "C" {
    pub fn ivtv_waitq(waitq: *mut wait_queue_head_t) -> c_int;
}
// Read Hauppauge eeprom
extern "C" {
    pub fn ivtv_read_eeprom(itv: *mut ivtv, tv: *mut tveeprom);
}
// First-open initialization: load firmware, init cx25840, etc.
extern "C" {
    pub fn ivtv_init_on_first_open(itv: *mut ivtv) -> c_int;
}
// Test if the current VBI mode is raw (1) or sliced (0)
// This is a PCI post thing, where if the pci register is not read, then

// Call the specified callback for all subdevs matching hw (if 0, then

// Call the specified callback for all subdevs matching hw (if 0, then

