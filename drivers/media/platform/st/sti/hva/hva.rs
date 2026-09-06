//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/hva/hva.h
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
// Copyright (C) STMicroelectronics SA 2015
// Authors: Yannick Fertre <yannick.fertre@st.com>
// Hugues Fruchet <hugues.fruchet@st.com>
//

//
// struct hva_frameinfo - information about hva frame
//
// @pixelformat:    fourcc code for uncompressed video format
// @width:          width of frame
// @height:         height of frame
// @aligned_width:  width of frame (with encoder alignment constraint)
// @aligned_height: height of frame (with encoder alignment constraint)
// @size:           maximum size in bytes required for data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_frameinfo {
    pub pixelformat: u32,
    pub width: u32,
    pub height: u32,
    pub aligned_width: u32,
    pub aligned_height: u32,
    pub size: u32,
}

//
// struct hva_streaminfo - information about hva stream
//
// @streamformat: fourcc code of compressed video format (H.264...)
// @width:        width of stream
// @height:       height of stream
// @profile:      profile string
// @level:        level string
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_streaminfo {
    pub streamformat: u32,
    pub width: u32,
    pub height: u32,
    pub profile: [u8; 32],
    pub level: [u8; 32],
}

//
// struct hva_controls - hva controls set
//
// @time_per_frame: time per frame in seconds
// @bitrate_mode:   bitrate mode (constant bitrate or variable bitrate)
// @gop_size:       groupe of picture size
// @bitrate:        bitrate (in bps)
// @aspect:         video aspect
// @profile:        H.264 profile
// @level:          H.264 level
// @entropy_mode:   H.264 entropy mode (CABAC or CVLC)
// @cpb_size:       coded picture buffer size (in kB)
// @dct8x8:         transform mode 8x8 enable
// @qpmin:          minimum quantizer
// @qpmax:          maximum quantizer
// @vui_sar:        pixel aspect ratio enable
// @vui_sar_idc:    pixel aspect ratio identifier
// @sei_fp:         sei frame packing arrangement enable
// @sei_fp_type:    sei frame packing arrangement type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_controls {
    pub time_per_frame: v4l2_fract,
    pub bitrate_mode: v4l2_mpeg_video_bitrate_mode,
    pub gop_size: u32,
    pub bitrate: u32,
    pub aspect: v4l2_mpeg_video_aspect,
    pub profile: v4l2_mpeg_video_h264_profile,
    pub level: v4l2_mpeg_video_h264_level,
    pub entropy_mode: v4l2_mpeg_video_h264_entropy_mode,
    pub cpb_size: u32,
    pub dct8x8: bool,
    pub qpmin: u32,
    pub qpmax: u32,
    pub vui_sar: bool,
    pub vui_sar_idc: v4l2_mpeg_video_h264_vui_sar_idc,
    pub sei_fp: bool,
    pub sei_fp_type: v4l2_mpeg_video_h264_sei_fp_arrangement_type,
}

//
// struct hva_frame - hva frame buffer (output)
//
// @vbuf:     video buffer information for V4L2
// @list:     V4L2 m2m list that the frame belongs to
// @info:     frame information (width, height, format, alignment...)
// @paddr:    physical address (for hardware)
// @vaddr:    virtual address (kernel can read/write)
// @prepared: true if vaddr/paddr are resolved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_frame {
    pub vbuf: vb2_v4l2_buffer,
    pub list: list_head,
    pub info: hva_frameinfo,
    pub paddr: dma_addr_t,
    pub vaddr: *mut c_void,
    pub prepared: bool,
}

//
// to_hva_frame() - cast struct vb2_v4l2_buffer * to struct hva_frame
//

//
// struct hva_stream - hva stream buffer (capture)
//
// @vbuf:       video buffer information for V4L2
// @list:       V4L2 m2m list that the frame belongs to
// @paddr:      physical address (for hardware)
// @vaddr:      virtual address (kernel can read/write)
// @prepared:   true if vaddr/paddr are resolved
// @size:       size of the buffer in bytes
// @bytesused:  number of bytes occupied by data in the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_stream {
    pub vbuf: vb2_v4l2_buffer,
    pub list: list_head,
    pub paddr: dma_addr_t,
    pub vaddr: *mut c_void,
    pub prepared: bool,
    pub size: c_uint,
    pub bytesused: c_uint,
}

//
// to_hva_stream() - cast struct vb2_v4l2_buffer * to struct hva_stream
//

//
// struct hva_ctx_dbg - instance context debug info
//
// @debugfs_entry:      debugfs entry
// @is_valid_period:    true if the sequence is valid for performance
// @begin:              start time of last HW task
// @total_duration:     total HW processing durations in 0.1ms
// @cnt_duration:       number of HW processings
// @min_duration:       minimum HW processing duration in 0.1ms
// @max_duration:       maximum HW processing duration in 0.1ms
// @avg_duration:       average HW processing duration in 0.1ms
// @max_fps:            maximum frames encoded per second (in 0.1Hz)
// @total_period:       total encoding periods in 0.1ms
// @cnt_period:         number of periods
// @min_period:         minimum encoding period in 0.1ms
// @max_period:         maximum encoding period in 0.1ms
// @avg_period:         average encoding period in 0.1ms
// @total_stream_size:  total number of encoded bytes
// @avg_fps:            average frames encoded per second (in 0.1Hz)
// @window_duration:    duration of the sampling window in 0.1ms
// @cnt_window:         number of samples in the window
// @window_stream_size: number of encoded bytes upon the sampling window
// @last_bitrate:       bitrate upon the last sampling window
// @min_bitrate:        minimum bitrate in kbps
// @max_bitrate:        maximum bitrate in kbps
// @avg_bitrate:        average bitrate in kbps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_ctx_dbg {
    pub debugfs_entry: *mut dentry,
    pub is_valid_period: bool,
    pub begin: ktime_t,
    pub total_duration: u32,
    pub cnt_duration: u32,
    pub min_duration: u32,
    pub max_duration: u32,
    pub avg_duration: u32,
    pub max_fps: u32,
    pub total_period: u32,
    pub cnt_period: u32,
    pub min_period: u32,
    pub max_period: u32,
    pub avg_period: u32,
    pub total_stream_size: u32,
    pub avg_fps: u32,
    pub window_duration: u32,
    pub cnt_window: u32,
    pub window_stream_size: u32,
    pub last_bitrate: u32,
    pub min_bitrate: u32,
    pub max_bitrate: u32,
    pub avg_bitrate: u32,
}

//
// struct hva_ctx - context of hva instance
//
// @hva_dev:         the device that this instance is associated with
// @fh:              V4L2 file handle
// @ctrl_handler:    V4L2 controls handler
// @ctrls:           hva controls set
// @id:              instance identifier
// @aborting:        true if current job aborted
// @name:            instance name (debug purpose)
// @run_work:        encode work
// @lock:            mutex used to lock access of this context
// @flags:           validity of streaminfo and frameinfo fields
// @frame_num:       frame number
// @stream_num:      stream number
// @max_stream_size: maximum size in bytes required for stream data
// @colorspace:      colorspace identifier
// @xfer_func:       transfer function identifier
// @ycbcr_enc:       Y'CbCr encoding identifier
// @quantization:    quantization identifier
// @streaminfo:      stream properties
// @frameinfo:       frame properties
// @enc:             current encoder
// @priv:            private codec data for this instance, allocated
// by encoder @open time
// @hw_err:          true if hardware error detected
// @encoded_frames:  number of encoded frames
// @sys_errors:      number of system errors (memory, resource, pm...)
// @encode_errors:   number of encoding errors (hw/driver errors)
// @frame_errors:    number of frame errors (format, size, header...)
// @dbg:             context debug info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_ctx {
    pub hva_dev: *mut hva_dev,
    pub fh: v4l2_fh,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub ctrls: hva_controls,
    pub id: u8,
    pub aborting: bool,
    pub name: [c_char; 100],
    pub run_work: work_struct,
// mutex protecting this data structure
    pub lock: mutex,
    pub flags: u32,
    pub frame_num: u32,
    pub stream_num: u32,
    pub max_stream_size: u32,
    pub colorspace: v4l2_colorspace,
    pub xfer_func: v4l2_xfer_func,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
    pub streaminfo: hva_streaminfo,
    pub frameinfo: hva_frameinfo,
    pub enc: *mut hva_enc,
    pub priv: *mut c_void,
    pub hw_err: bool,
    pub encoded_frames: u32,
    pub sys_errors: u32,
    pub encode_errors: u32,
    pub frame_errors: u32,

    pub dbg: hva_ctx_dbg,

}

pub const HVA_FLAG_STREAMINFO: c_uint = 0x0001;
pub const HVA_FLAG_FRAMEINFO: c_uint = 0x0002;

//
// struct hva_dev_dbg - device debug info
//
// @debugfs_entry: debugfs entry
// @last_ctx:      debug information about last running instance context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_dev_dbg {
    pub debugfs_entry: *mut dentry,
    pub last_ctx: hva_ctx,
}

pub const HVA_MAX_INSTANCES: c_int = 16;
pub const HVA_MAX_ENCODERS: c_int = 10;

//
// struct hva_dev - abstraction for hva entity
//
// @v4l2_dev:            V4L2 device
// @vdev:                video device
// @pdev:                platform device
// @dev:                 device
// @lock:                mutex used for critical sections & V4L2 ops
// serialization
// @m2m_dev:             memory-to-memory V4L2 device information
// @instances:           opened instances
// @nb_of_instances:     number of opened instances
// @instance_id:         rolling counter identifying an instance (debug purpose)
// @regs:                register io memory access
// @esram_addr:          esram address
// @esram_size:          esram size
// @clk:                 hva clock
// @irq_its:             status interruption
// @irq_err:             error interruption
// @work_queue:          work queue to handle the encode jobs
// @protect_mutex:       mutex used to lock access of hardware
// @interrupt:           completion interrupt
// @ip_version:          IP hardware version
// @encoders:            registered encoders
// @nb_of_encoders:      number of registered encoders
// @pixelformats:        supported uncompressed video formats
// @nb_of_pixelformats:  number of supported umcompressed video formats
// @streamformats:       supported compressed video formats
// @nb_of_streamformats: number of supported compressed video formats
// @sfl_reg:             status fifo level register value
// @sts_reg:             status register value
// @lmi_err_reg:         local memory interface error register value
// @emi_err_reg:         external memory interface error register value
// @hec_mif_err_reg:     HEC memory interface error register value
// @dbg:                 device debug info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_dev {
    pub v4l2_dev: v4l2_device,
    pub vdev: *mut video_device,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
// mutex protecting vb2_queue structure
    pub lock: mutex,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub instances: [*mut hva_ctx; HVA_MAX_INSTANCES],
    pub nb_of_instances: c_uint,
    pub instance_id: c_uint,
    pub regs: *mut void __iomem,
    pub esram_addr: u32,
    pub esram_size: u32,
    pub clk: *mut clk,
    pub irq_its: c_int,
    pub irq_err: c_int,
    pub work_queue: *mut workqueue_struct,
// mutex protecting hardware access
    pub protect_mutex: mutex,
    pub interrupt: completion,
    pub ip_version: unsigned long int,
    pub encoders: [*const hva_enc; HVA_MAX_ENCODERS],
    pub nb_of_encoders: u32,
    pub pixelformats: [u32; HVA_MAX_FORMATS],
    pub nb_of_pixelformats: u32,
    pub streamformats: [u32; HVA_MAX_FORMATS],
    pub nb_of_streamformats: u32,
    pub sfl_reg: u32,
    pub sts_reg: u32,
    pub lmi_err_reg: u32,
    pub emi_err_reg: u32,
    pub hec_mif_err_reg: u32,

    pub dbg: hva_dev_dbg,

}

//
// struct hva_enc - hva encoder
//
// @name:         encoder name
// @streamformat: fourcc code for compressed video format (H.264...)
// @pixelformat:  fourcc code for uncompressed video format
// @max_width:    maximum width of frame for this encoder
// @max_height:   maximum height of frame for this encoder
// @open:         open encoder
// @close:        close encoder
// @encode:       encode a frame (struct hva_frame) in a stream
// (struct hva_stream)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hva_enc {
    pub name: *const c_char,
    pub streamformat: u32,
    pub pixelformat: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub ctx): *mut *mut int (open)(struct hva_ctx,
    pub ctx): *mut *mut int (close)(struct hva_ctx,
    pub stream): *mut hva_stream,
}

extern "C" {
    pub fn hva_debugfs_create(hva: *mut hva_dev);
}
extern "C" {
    pub fn hva_debugfs_remove(hva: *mut hva_dev);
}
extern "C" {
    pub fn hva_dbg_ctx_create(ctx: *mut hva_ctx);
}
extern "C" {
    pub fn hva_dbg_ctx_remove(ctx: *mut hva_ctx);
}
extern "C" {
    pub fn hva_dbg_perf_begin(ctx: *mut hva_ctx);
}
extern "C" {
    pub fn hva_dbg_perf_end(ctx: *mut hva_ctx, stream: *mut hva_stream);
}

