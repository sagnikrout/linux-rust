//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-driver.h
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
// cx18 driver internal defines and structures
//
// Derived from ivtv-driver.h
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//

// DVB

// vb2 YUV support

pub const CX18_MEM_OFFSET: c_uint = 0x00000000;
pub const CX18_MEM_SIZE: c_uint = 0x04000000;
pub const CX18_REG_OFFSET: c_uint = 0x02000000;
// Maximum cx18 driver instances.
pub const CX18_MAX_CARDS: c_int = 32;
// Supported cards

pub const CX18_CARD_LAST: c_int = 9;
pub const CX18_ENC_STREAM_TYPE_MPG: c_int = 0;
pub const CX18_ENC_STREAM_TYPE_TS: c_int = 1;
pub const CX18_ENC_STREAM_TYPE_YUV: c_int = 2;
pub const CX18_ENC_STREAM_TYPE_VBI: c_int = 3;
pub const CX18_ENC_STREAM_TYPE_PCM: c_int = 4;
pub const CX18_ENC_STREAM_TYPE_IDX: c_int = 5;
pub const CX18_ENC_STREAM_TYPE_RAD: c_int = 6;
pub const CX18_MAX_STREAMS: c_int = 7;
// system vendor and device IDs
pub const PCI_VENDOR_ID_CX: c_uint = 0x14f1;
pub const PCI_DEVICE_ID_CX23418: c_uint = 0x5b7a;
// subsystem vendor ID
pub const CX18_PCI_ID_HAUPPAUGE: c_uint = 0x0070;
pub const CX18_PCI_ID_COMPRO: c_uint = 0x185b;
pub const CX18_PCI_ID_YUAN: c_uint = 0x12ab;
pub const CX18_PCI_ID_CONEXANT: c_uint = 0x14f1;
pub const CX18_PCI_ID_TOSHIBA: c_uint = 0x1179;
pub const CX18_PCI_ID_LEADTEK: c_uint = 0x107D;
pub const CX18_PCI_ID_GOTVIEW: c_uint = 0x5854;
// ========================================================================
// ========================== START USER SETTABLE DMA VARIABLES ===========
// ========================================================================
// DMA Buffers, Default size in MB allocated
pub const CX18_DEFAULT_ENC_TS_BUFFERS: c_int = 1;
pub const CX18_DEFAULT_ENC_MPG_BUFFERS: c_int = 2;
pub const CX18_DEFAULT_ENC_IDX_BUFFERS: c_int = 1;
pub const CX18_DEFAULT_ENC_YUV_BUFFERS: c_int = 2;
pub const CX18_DEFAULT_ENC_VBI_BUFFERS: c_int = 1;
pub const CX18_DEFAULT_ENC_PCM_BUFFERS: c_int = 1;
// Maximum firmware DMA buffers per stream
pub const CX18_MAX_FW_MDLS_PER_STREAM: c_int = 63;
// YUV buffer sizes in bytes to ensure integer # of frames per buffer

// IDX buffer size should be a multiple of the index entry size from the chip
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_enc_idx_entry {
    pub length: __le32,
    pub offset_low: __le32,
    pub offset_high: __le32,
    pub flags: __le32,
    pub pts_low: __le32,
    pub pts_high: __le32,
// C attribute field omitted

// DMA buffer, default size in kB allocated
pub const CX18_DEFAULT_ENC_TS_BUFSIZE: c_int = 32;
pub const CX18_DEFAULT_ENC_MPG_BUFSIZE: c_int = 32;

pub const CX18_DEFAULT_ENC_PCM_BUFSIZE: c_int = 4;
// i2c stuff
pub const I2C_CLIENTS_MAX: c_int = 16;
// debugging
// Flag to turn on high volume debugging

// Flag to turn on high volume debugging

// NOTE: extra space before comma in 'fmt , ## args' is required for

    pub \: v4l2_info(&cx->v4l2_dev, " " type ": " fmt , ## args);,

    pub \: v4l2_info(&cx->v4l2_dev, " " type ": " fmt , ## args);,

// Standard kernel messages

// Messages for internal subdevs to use

    pub \: v4l2_info(dev, " " type ": " fmt , ## args);,

    pub \: v4l2_info(dev, " " type ": " fmt , ## args);,

    pub cx18_debug: extern int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_options {
    pub /: *mut *mut int megabytes[CX18_MAX_STREAMS]; / Size in megabytes of each stream,
    pub /: *mut *mut int cardtype; / force card type on load,
    pub /: *mut *mut int tuner; / set tuner on load,
    pub /: *mut *mut int radio; / enable/disable radio,
}

// per-mdl bit flags

// per-stream, s_flags

// per-cx18, i_flags

// These are the VBI types as they appear in the embedded VBI private packets.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_vb2_buffer {
// Common video buffer sub-system struct
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub /: *mut *mut v4l2_std_id tvnorm; / selected tv norm,
    pub bytes_used: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_buffer {
    pub list: list_head,
    pub dma_handle: dma_addr_t,
    pub buf: *mut c_char,
    pub bytesused: u32,
    pub readpos: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_mdl {
    pub list: list_head,
    pub /: *mut *mut u32 id; / index into cx->scb->cpu_mdl[] of 1st cx18_mdl_ent,
    pub skipped: c_uint,
    pub m_flags: c_ulong,
    pub buf_list: list_head,
    pub /: *mut *mut *mut cx18_buffer curr_buf; / current buffer in list for reading,
    pub bytesused: u32,
    pub readpos: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_queue {
    pub list: list_head,
    pub depth: core::sync::atomic::AtomicI32,
    pub bytesused: u32,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_dvb {
    pub stream: *mut cx18_stream,
    pub hw_frontend: dmx_frontend,
    pub mem_frontend: dmx_frontend,
    pub dmxdev: dmxdev,
    pub dvb_adapter: dvb_adapter,
    pub demux: dvb_demux,
    pub fe: *mut dvb_frontend,
    pub dvbnet: dvb_net,
    pub enabled: c_int,
    pub feeding: c_int,
    pub feedlock: mutex,
}

pub const CX18_MAX_MDL_ACKS: c_int = 2;

// CPU_DE_RELEASE_MDL can burst CX18_MAX_FW_MDLS_PER_STREAM orders in a group
pub const CX18_F_EWO_MB_STALE_UPON_RECEIPT: c_uint = 0x1;
pub const CX18_F_EWO_MB_STALE_WHILE_PROC: c_uint = 0x2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_in_work_order {
    pub work: work_struct,
    pub pending: core::sync::atomic::AtomicI32,
    pub cx: *mut cx18,
    pub flags: c_ulong,
    pub rpu: c_int,
    pub mb: cx18_mailbox,
    pub mdl_ack: [cx18_mdl_ack; CX18_MAX_MDL_ACKS],
    pub str: *mut c_char,
}

pub const CX18_INVALID_TASK_HANDLE: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_stream {
// These first five fields are always set, even if the stream
    pub /: *mut *mut video_device video_dev; / v4l2_dev is NULL when stream not created,
    pub /: *mut *mut *mut cx18_dvb dvb; / DVB / Digital Transport,
    pub /: *mut *mut *mut cx18 cx; / for ease of use,
    pub /: *const *const *const char name; / name of the stream,
    pub /: *mut *mut int type; / stream type,
    pub /: *mut *mut u32 handle; / task handle,
    pub /: *mut *mut u32 v4l2_dev_caps; / device capabilities,
    pub mdl_base_idx: c_uint,
    pub id: u32,
    pub /: *mut *mut unsigned long s_flags; / status flags, see above,
    pub PCI_DMA_TODEVICE,: *mut *mut int dma; / can be,
    pub waitq: wait_queue_head_t,
// Buffers
    pub /: *mut *mut list_head buf_pool; / buffers not attached to an MDL,
    pub /: *mut *mut u32 buffers; / total buffers owned by this stream,
    pub /: *mut *mut u32 buf_size; / size in bytes of a single buffer,
// MDL sizes - all stream MDLs are the same size
    pub bufs_per_mdl: u32,
    pub /: *mut *mut u32 mdl_size; / total bytes in all buffers in a mdl,
// MDL Queues
    pub /: *mut *mut cx18_queue q_free; / free - in rotation, not committed,
    pub /: *mut *mut cx18_queue q_busy; / busy - in use by firmware,
    pub /: *mut *mut cx18_queue q_full; / full - data for user apps,
    pub /: *mut *mut cx18_queue q_idle; / idle - not in rotation,
    pub out_work_order: work_struct,
// Videobuf for YUV video
    pub pixelformat: u32,
    pub vb_bytes_per_frame: u32,
    pub vb_bytes_per_line: u32,
    pub /: *mut *mut list_head vb_capture; / video capture queue,
    pub vb_lock: spinlock_t,
    pub vb_timeout: timer_list,
    pub sequence: u32,
    pub vidq: vb2_queue,
    pub vb_type: v4l2_buf_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_open_id {
    pub fh: v4l2_fh,
    pub open_id: u32,
    pub type: c_int,
    pub cx: *mut cx18,
}

extern "C" {
    pub fn container_of(_arg: fh, cx18_open_id: struct, _arg: fh) -> return;
}
extern "C" {
    pub fn fh2id(_arg: file_to_v4l2_fh(file)) -> return;
}
// forward declaration of struct defined in cx18-cards.h
//
// A note about "sliced" VBI data as implemented in this driver:
//
// Currently we collect the sliced VBI in the form of Ancillary Data
// packets, inserted by the AV core decoder/digitizer/slicer in the
// horizontal blanking region of the VBI lines, in "raw" mode as far as
// the Encoder is concerned.  We don't ever tell the Encoder itself
// to provide sliced VBI. (AV Core: sliced mode - Encoder: raw mode)
//
// We then process the ancillary data ourselves to send the sliced data
// to the user application directly or build up MPEG-2 private stream 1
// packets to splice into (only!) MPEG-2 PS streams for the user app.
//
// (That's how ivtv essentially does it.)
//
// The Encoder should be able to extract certain sliced VBI data for
// us and provide it in a separate stream or splice it into any type of
// MPEG PS or TS stream, but this isn't implemented yet.
//
// Number of "raw" VBI samples per horizontal line we tell the Encoder to
// grab from the decoder/digitizer/slicer output for raw or sliced VBI.
// It depends on the pixel clock and the horiz rate:
//
// (1/Fh)*(2*Fp) = Samples/line
// = 4 bytes EAV + Anc data in hblank + 4 bytes SAV + active samples
//
// Sliced VBI data is sent as ancillary data during horizontal blanking
// Raw VBI is sent as active video samples during vertcal blanking
//
// We use a  BT.656 pxiel clock of 13.5 MHz and a BT.656 active line
// length of 720 pixels @ 4:2:2 sampling.  Thus...
//
// For systems that use a 15.734 kHz horizontal rate, such as
// NTSC-M, PAL-M, PAL-60, and other 60 Hz/525 line systems, we have:
//
// (1/15.734 kHz) * 2 * 13.5 MHz = 1716 samples/line =
// 4 bytes SAV + 268 bytes anc data + 4 bytes SAV + 1440 active samples
//
// For systems that use a 15.625 kHz horizontal rate, such as
// PAL-B/G/H, PAL-I, SECAM-L and other 50 Hz/625 line systems, we have:
//
// (1/15.625 kHz) * 2 * 13.5 MHz = 1728 samples/line =
// 4 bytes SAV + 280 bytes anc data + 4 bytes SAV + 1440 active samples
//

pub const CX18_VBI_FRAMES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbi_info {
// Current state of v4l2 VBI settings for this device
    pub in: v4l2_format,
    pub /: *mut *mut *mut v4l2_sliced_vbi_format sliced_in; / pointer to in.fmt.sliced,
    pub /: *mut *mut u32 count; / Count of VBI data lines: 60 Hz: 12 or 50 Hz: 18,
    pub /: *mut *mut u32 start[2]; / First VBI data line per field: 10 & 273 or 6 & 318,
    pub /: *mut *mut u32 frame; / Count of VBI buffers/frames received from Encoder,
//
// Vars for creation and insertion of MPEG Private Stream 1 packets
// of sliced VBI data into an MPEG PS
//
// Boolean: create and insert Private Stream 1 packets into the PS
    pub insert_mpeg: c_int,
//
// Buffer for the maximum of 2 * 18 * packet_size sliced VBI lines.
// Used in cx18-vbi.c only for collecting sliced data, and as a source
// during conversion of sliced VBI data into MPEG Priv Stream 1 packets.
// We don't need to save state here, but the array may have been a bit
// too big (2304 bytes) to alloc from the stack.
//
    pub sliced_data: [v4l2_sliced_vbi_data; 36],
//
// A ring buffer of driver-generated MPEG-2 PS
// Program Pack/Private Stream 1 packets for sliced VBI data insertion
// into the MPEG PS stream.
//
// In each sliced_mpeg_data[] buffer is:
// 16 byte MPEG-2 PS Program Pack Header
// 16 byte MPEG-2 Private Stream 1 PES Header
// 4 byte magic number: "itv0" or "ITV0"
// 4 byte first  field line mask, if "itv0"
// 4 byte second field line mask, if "itv0"
// 36 lines, if "ITV0"; or <36 lines, if "itv0"; of sliced VBI data
//
// Each line in the payload is
// 1 byte line header derived from the SDID (WSS, CC, VPS, etc.)
// 42 bytes of line data
//
// That's a maximum 1552 bytes of payload in the Private Stream 1 packet
// which is the payload size a PVR-350 (CX23415) MPEG decoder will
// accept for VBI data. So, including the headers, it's a maximum 1584
// bytes total.
//
pub const CX18_SLICED_MPEG_DATA_MAXSZ: c_int = 1584;
// copy_vbi_buf() needs 8 temp bytes on the end for the worst case
    pub sliced_mpeg_data: [*mut u8; CX18_VBI_FRAMES],
    pub sliced_mpeg_size: [u32; CX18_VBI_FRAMES],
// Count of Program Pack/Program Stream 1 packets inserted into PS
    pub inserted_frame: u32,
//
// A dummy driver stream transfer mdl & buffer with a copy of the next
// sliced_mpeg_data[] buffer for output to userland apps.
// Only used in cx18-fileops.c, but its state needs to persist at times.
//
    pub sliced_mpeg_mdl: cx18_mdl,
    pub sliced_mpeg_buf: cx18_buffer,
}

// Per cx23418, per I2C bus private algo callback data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18_i2c_algo_callback_data {
    pub cx: *mut cx18,
    pub /: *mut *mut int bus_index; / 0 or 1 for the cx23418's 1st or 2nd I2C bus,
}

pub const CX18_MAX_MMIO_WR_RETRIES: c_int = 10;
// Struct to hold info about cx18 cards
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx18 {
    pub instance: c_int,
    pub pci_dev: *mut pci_dev,
    pub v4l2_dev: v4l2_device,
    pub /: *mut *mut *mut v4l2_subdev sd_av; / A/V decoder/digitizer sub-device,
    pub /: *mut *mut *mut v4l2_subdev sd_extmux; / External multiplexer sub-dev,
    pub /: *const *const *const cx18_card card; / card information,
    pub /: *const *const *const char card_name; / full name of the card,
    pub /: *const *const *const cx18_card_tuner_i2c card_i2c; / i2c addresses to probe for tuner,
    pub is_50hz: u8,
    pub is_60hz: u8,
    pub /: *mut *mut u8 nof_inputs; / number of video inputs,
    pub /: *mut *mut u8 nof_audio_inputs; / number of audio inputs,
    pub /: *mut *mut u32 v4l2_cap; / V4L2 capabilities of card,
    pub /: *mut *mut u32 hw_flags; / Hardware description of the board,
    pub free_mdl_idx: c_uint,
    pub /: *mut *mut *mut cx18_scb __iomem scb; / pointer to SCB,
    pub SCB*/: *mut *mut mutex epu2apu_mb_lock; / protect driver to chip mailbox in,
    pub SCB*/: *mut *mut mutex epu2cpu_mb_lock; / protect driver to chip mailbox in,
    pub av_state: cx18_av_state,
// codec settings
    pub cxhdl: cx2341x_handler,
    pub filter_mode: u32,
    pub temporal_strength: u32,
    pub spatial_strength: u32,
// dualwatch
    pub dualwatch_jiffies: c_ulong,
    pub dualwatch_stereo_mode: u32,
    pub /: *mut *mut mutex serialize_lock; / mutex used to serialize open/close/start/stop/ioctl operations,
    pub /: *mut *mut cx18_options options; / User options,
    pub /: *mut *mut int stream_buffers[CX18_MAX_STREAMS]; / # of buffers for each stream,
    pub /: *mut *mut int stream_buf_size[CX18_MAX_STREAMS]; / Stream buffer size,
    pub /: *mut *mut cx18_stream streams[CX18_MAX_STREAMS]; / Stream data,
    pub /: *mut *mut *mut snd_cx18_card alsa; / ALSA interface for PCM capture stream,
    pub num_bytes): usize,
    pub /: *mut *mut unsigned long i_flags; / global cx18 flags,
    pub /: *mut *mut atomic_t ana_capturing; / count number of active analog capture streams,
    pub /: *mut *mut atomic_t tot_capturing; / total count number of active capture streams,
    pub search_pack_header: c_int,
    pub as: *mut *mut int open_id; / incremented each time an open occurs, used,
    pub base_addr: resource_size_t,
    pub card_rev: u8,
    pub reg_mem: *mut *mut void __iomem enc_mem,,
    pub vbi: vbi_info,
    pub mpg_data_received: u64,
    pub vbi_data_inserted: u64,
    pub mb_apu_waitq: wait_queue_head_t,
    pub mb_cpu_waitq: wait_queue_head_t,
    pub cap_w: wait_queue_head_t,
// when the current DMA is finished this queue is woken up
    pub dma_waitq: wait_queue_head_t,
    pub sw1_irq_mask: u32,
    pub sw2_irq_mask: u32,
    pub hw2_irq_mask: u32,
    pub in_work_queue: *mut workqueue_struct,
    pub /: *mut *mut char in_workq_name[39]; / "cx18-NN-in",
    pub in_work_order: [cx18_in_work_order; CX18_MAX_IN_WORK_ORDERS],
    pub /: *mut *mut char epu_debug_str[256]; / CX18_EPU_DEBUG is rare: use shared space,
// i2c
    pub i2c_adap: [i2c_adapter; 2],
    pub i2c_algo: [i2c_algo_bit_data; 2],
    pub i2c_algo_cb_data: [cx18_i2c_algo_callback_data; 2],
    pub ir_i2c_init_data: IR_i2c_init_data,
// gpio
    pub gpio_dir: u32,
    pub gpio_val: u32,
    pub gpio_lock: mutex,
    pub sd_gpiomux: v4l2_subdev,
    pub sd_resetctrl: v4l2_subdev,
// v4l2 and User settings
// codec settings
    pub audio_input: u32,
    pub active_input: u32,
    pub std: v4l2_std_id,
    pub /: *mut *mut v4l2_std_id tuner_std; / The norm of the tuner (fixed),
// Used for cx18-alsa module loading
    pub request_module_wk: work_struct,
}

extern "C" {
    pub fn container_of(_arg: v4l2_dev, cx18: struct, _arg: v4l2_dev) -> return;
}
// cx18 extensions to be loaded
extern "C" {
    pub fn int(: *mut *mut cx18_ext_init)(struct cx18) -> extern;
}
// Globals
// ==============Prototypes==================
// Return non-zero if a signal is pending
extern "C" {
    pub fn cx18_msleep_timeout(msecs: c_uint, intr: c_int) -> c_int;
}
// Read Hauppauge eeprom
extern "C" {
    pub fn cx18_read_eeprom(cx: *mut cx18, tv: *mut tveeprom);
}
// First-open initialization: load firmware, etc.
extern "C" {
    pub fn cx18_init_on_first_open(cx: *mut cx18) -> c_int;
}
// Test if the current VBI mode is raw (1) or sliced (0)
// Call the specified callback for all subdevs with a grp_id bit matching the
// mask in hw (if 0, then match them all). Ignore any errors.

// Call the specified callback for all subdevs with a grp_id bit matching the
// mask in hw (if 0, then match them all). If the callback returns an error
// other than 0 or -ENOIOCTLCMD, then return with that error code.

