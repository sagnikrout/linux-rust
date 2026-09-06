//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/delta/delta.h
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
// Author: Hugues Fruchet <hugues.fruchet@st.com> for STMicroelectronics.
//

//
// enum delta_state - state of decoding instance
//
// @DELTA_STATE_WF_FORMAT:
// Wait for compressed format to be set by V4L2 client in order
// to know what is the relevant decoder to open.
//
// @DELTA_STATE_WF_STREAMINFO:
// Wait for stream information to be available (bitstream
// header parsing is done).
//
// @DELTA_STATE_READY:
// Decoding instance is ready to decode compressed access unit.
//
// @DELTA_STATE_WF_EOS:
// Decoding instance is waiting for EOS (End Of Stream) completion.
//
// @DELTA_STATE_EOS:
// EOS (End Of Stream) is completed (signaled to user). Decoding instance
// should then be closed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum delta_state {
    DELTA_STATE_WF_FORMAT,
    DELTA_STATE_WF_STREAMINFO,
    DELTA_STATE_READY,
    DELTA_STATE_WF_EOS,
    DELTA_STATE_EOS
}

//
// struct delta_streaminfo - information about stream to decode
//
// @flags:		validity of fields (crop, pixelaspect, other)
// @width:		width of video stream
// @height:		height ""
// @streamformat:	fourcc compressed format of video (MJPEG, MPEG2, ...)
// @dpb:		number of frames needed to decode a single frame
// (h264 dpb, up to 16)
// @crop:		cropping window inside decoded frame (1920x1080@0,0
// inside 1920x1088 frame for ex.)
// @pixelaspect:	pixel aspect ratio of video (4/3, 5/4)
// @field:		interlaced or not
// @profile:		profile string
// @level:		level string
// @other:		other string information from codec
// @colorspace:		colorspace identifier
// @xfer_func:		transfer function identifier
// @ycbcr_enc:		Y'CbCr encoding identifier
// @quantization:	quantization identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_streaminfo {
    pub flags: u32,
    pub streamformat: u32,
    pub width: u32,
    pub height: u32,
    pub dpb: u32,
    pub crop: v4l2_rect,
    pub pixelaspect: v4l2_fract,
    pub field: v4l2_field,
    pub profile: [u8; 32],
    pub level: [u8; 32],
    pub other: [u8; 32],
    pub colorspace: v4l2_colorspace,
    pub xfer_func: v4l2_xfer_func,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
}

pub const DELTA_STREAMINFO_FLAG_CROP: c_uint = 0x0001;
pub const DELTA_STREAMINFO_FLAG_PIXELASPECT: c_uint = 0x0002;
pub const DELTA_STREAMINFO_FLAG_OTHER: c_uint = 0x0004;
//
// struct delta_au - access unit structure.
//
// @vbuf:	video buffer information for V4L2
// @list:	V4L2 m2m list that the frame belongs to
// @prepared:	if set vaddr/paddr are resolved
// @vaddr:	virtual address (kernel can read/write)
// @paddr:	physical address (for hardware)
// @flags:	access unit type (V4L2_BUF_FLAG_KEYFRAME/PFRAME/BFRAME)
// @dts:	decoding timestamp of this access unit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_au {
    pub /: *mut *mut vb2_v4l2_buffer vbuf; / keep first,
    pub /: *mut *mut list_head list; / keep second,
    pub prepared: bool,
    pub size: u32,
    pub vaddr: *mut c_void,
    pub paddr: dma_addr_t,
    pub flags: u32,
    pub dts: u64,
}

//
// struct delta_frameinfo - information about decoded frame
//
// @flags:		validity of fields (crop, pixelaspect)
// @pixelformat:	fourcc code for uncompressed video format
// @width:		width of frame
// @height:		height of frame
// @aligned_width:	width of frame (with encoder or decoder alignment
// constraint)
// @aligned_height:	height of frame (with encoder or decoder alignment
// constraint)
// @size:		maximum size in bytes required for data
// @crop:		cropping window inside frame (1920x1080@0,0
// inside 1920x1088 frame for ex.)
// @pixelaspect:	pixel aspect ratio of video (4/3, 5/4)
// @field:		interlaced mode
// @colorspace:		colorspace identifier
// @xfer_func:		transfer function identifier
// @ycbcr_enc:		Y'CbCr encoding identifier
// @quantization:	quantization identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_frameinfo {
    pub flags: u32,
    pub pixelformat: u32,
    pub width: u32,
    pub height: u32,
    pub aligned_width: u32,
    pub aligned_height: u32,
    pub size: u32,
    pub crop: v4l2_rect,
    pub pixelaspect: v4l2_fract,
    pub field: v4l2_field,
    pub colorspace: v4l2_colorspace,
    pub xfer_func: v4l2_xfer_func,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub quantization: v4l2_quantization,
}

pub const DELTA_FRAMEINFO_FLAG_CROP: c_uint = 0x0001;
pub const DELTA_FRAMEINFO_FLAG_PIXELASPECT: c_uint = 0x0002;
//
// struct delta_frame - frame structure.
//
// @vbuf:	video buffer information for V4L2
// @list:	V4L2 m2m list that the frame belongs to
// @info:	frame information (width, height, format, alignment...)
// @prepared:	if set pix/vaddr/paddr are resolved
// @index:	frame index, aligned on V4L2 wow
// @vaddr:	virtual address (kernel can read/write)
// @paddr:	physical address (for hardware)
// @state:	frame state for frame lifecycle tracking
// (DELTA_FRAME_FREE/DEC/OUT/REC/...)
// @flags:	frame type (V4L2_BUF_FLAG_KEYFRAME/PFRAME/BFRAME)
// @dts:	decoding timestamp of this frame
// @field:	field order for interlaced frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_frame {
    pub /: *mut *mut vb2_v4l2_buffer vbuf; / keep first,
    pub /: *mut *mut list_head list; / keep second,
    pub info: delta_frameinfo,
    pub prepared: bool,
    pub index: u32,
    pub vaddr: *mut c_void,
    pub paddr: dma_addr_t,
    pub state: u32,
    pub flags: u32,
    pub dts: u64,
    pub field: v4l2_field,
}

// frame state for frame lifecycle tracking
pub const DELTA_FRAME_FREE: c_uint = 0x00 /* is free and can be used for decoding */;
pub const DELTA_FRAME_REF: c_uint = 0x01 /* is a reference frame */;
pub const DELTA_FRAME_BSY: c_uint = 0x02 /* is owned by decoder and busy */;
pub const DELTA_FRAME_DEC: c_uint = 0x04 /* contains decoded content */;
pub const DELTA_FRAME_OUT: c_uint = 0x08 /* has been given to user */;
pub const DELTA_FRAME_RDY: c_uint = 0x10 /* is ready but still held by decoder */;
pub const DELTA_FRAME_M2M: c_uint = 0x20 /* is owned by mem2mem framework */;
//
// struct delta_dts - decoding timestamp.
//
// @list:	list to chain timestamps
// @val:	timestamp in microseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_dts {
    pub list: list_head,
    pub val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_buf {
    pub size: u32,
    pub vaddr: *mut c_void,
    pub paddr: dma_addr_t,
    pub name: *const c_char,
    pub attrs: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_ipc_ctx {
    pub cb_err: c_int,
    pub copro_hdl: u32,
    pub done: completion,
    pub ipc_buf_struct: delta_buf,
    pub ipc_buf: *mut delta_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_ipc_param {
    pub size: u32,
    pub data: *mut c_void,
}

//
// struct delta_dec - decoder structure.
//
// @name:		name of this decoder
// @streamformat:	input stream format that this decoder support
// @pixelformat:	pixel format of decoded frame that this decoder support
// @max_width:		(optional) maximum width that can decode this decoder
// if not set, maximum width is DELTA_MAX_WIDTH
// @max_height:		(optional) maximum height that can decode this decoder
// if not set, maximum height is DELTA_MAX_HEIGHT
// @pm:			(optional) if set, decoder will manage power on its own
// @open:		open this decoder
// @close:		close this decoder
// @setup_frame:	setup frame to be used by decoder, see below
// @get_streaminfo:	get stream related infos, see below
// @get_frameinfo:	get decoded frame related infos, see below
// @set_frameinfo:	(optional) set decoded frame related infos, see below
// @setup_frame:	setup frame to be used by decoder, see below
// @decode:		decode a single access unit, see below
// @get_frame:		get the next decoded frame available, see below
// @recycle:		recycle the given frame, see below
// @flush:		(optional) flush decoder, see below
// @drain:		(optional) drain decoder, see below
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_dec {
    pub name: *const c_char,
    pub streamformat: u32,
    pub pixelformat: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub pm: bool,
//
// decoder ops
//
    pub ctx): *mut *mut int (open)(struct delta_ctx,
    pub ctx): *mut *mut int (close)(struct delta_ctx,
//
// setup_frame() - setup frame to be used by decoder
// @ctx:	(in) instance
// @frame:	(in) frame to use
// @frame.index	(in) identifier of frame
// @frame.vaddr	(in) virtual address (kernel can read/write)
// @frame.paddr	(in) physical address (for hardware)
//
// Frame is to be allocated by caller, then given
// to decoder through this call.
// Several frames must be given to decoder (dpb),
// each frame is identified using its index.
//
    pub frame): *mut *mut *mut int (setup_frame)(struct delta_ctx ctx, struct delta_frame,
//
// get_streaminfo() - get stream related infos
// @ctx:	(in) instance
// @streaminfo:	(out) width, height, dpb,...
//
// Precondition: stream header must have been successfully
// parsed to have this call successful & @streaminfo valid.
// Header parsing must be done using decode(), giving
// explicitly header access unit or first access unit of bitstream.
// If no valid header is found, get_streaminfo will return -ENODATA,
// in this case the next bitstream access unit must be decoded till
// get_streaminfo becomes successful.
//
    pub streaminfo): *mut delta_streaminfo,
//
// get_frameinfo() - get decoded frame related infos
// @ctx:	(in) instance
// @frameinfo:	(out) width, height, alignment, crop, ...
//
// Precondition: get_streaminfo() must be successful
//
    pub frameinfo): *mut delta_frameinfo,
//
// set_frameinfo() - set decoded frame related infos
// @ctx:	(in) instance
// @frameinfo:	(out) width, height, alignment, crop, ...
//
// Optional.
// Typically used to negotiate with decoder the output
// frame if decoder can do post-processing.
//
    pub frameinfo): *mut delta_frameinfo,
//
// decode() - decode a single access unit
// @ctx:	(in) instance
// @au:		(in/out) access unit
// @au.size	(in) size of au to decode
// @au.vaddr	(in) virtual address (kernel can read/write)
// @au.paddr	(in) physical address (for hardware)
// @au.flags	(out) au type (V4L2_BUF_FLAG_KEYFRAME
// PFRAME/BFRAME)
//
// Decode the access unit given. Decode is synchronous;
// access unit memory is no more needed after this call.
// After this call, none, one or several frames could
// have been decoded, which can be retrieved using
// get_frame().
//
    pub au): *mut *mut *mut int (decode)(struct delta_ctx ctx, struct delta_au,
//
// get_frame() - get the next decoded frame available
// @ctx:	(in) instance
// @frame:	(out) frame with decoded data:
// @frame.index	(out) identifier of frame
// @frame.field	(out) field order for interlaced frame
// @frame.state	(out) frame state for frame lifecycle tracking
// @frame.flags	(out) frame type (V4L2_BUF_FLAG_KEYFRAME
// PFRAME/BFRAME)
//
// Get the next available decoded frame.
// If no frame is available, -ENODATA is returned.
// If a frame is available, frame structure is filled with
// relevant data, frame.index identifying this exact frame.
// When this frame is no more needed by upper layers,
// recycle() must be called giving this frame identifier.
//
    pub frame): *mut *mut *mut int (get_frame)(struct delta_ctx ctx, struct delta_frame,
//
// recycle() - recycle the given frame
// @ctx:	(in) instance
// @frame:	(in) frame to recycle:
// @frame.index	(in) identifier of frame
//
// recycle() is to be called by user when the decoded frame
// is no more needed (composition/display done).
// This frame will then be reused by decoder to proceed
// with next frame decoding.
// If not enough frames have been provided through setup_frame(),
// or recycle() is not called fast enough, the decoder can run out
// of available frames to proceed with decoding (starvation).
// This case is guarded by wq_recycle wait queue which ensures that
// decoder is called only if at least one frame is available.
//
    pub frame): *mut *mut *mut int (recycle)(struct delta_ctx ctx, struct delta_frame,
//
// flush() - flush decoder
// @ctx:	(in) instance
//
// Optional.
// Reset decoder context and discard all internal buffers.
// This allows implementation of seek, which leads to discontinuity
// of input bitstream that decoder must know to restart its internal
// decoding logic.
//
    pub ctx): *mut *mut int (flush)(struct delta_ctx,
//
// drain() - drain decoder
// @ctx:	(in) instance
//
// Optional.
// Mark decoder pending frames (decoded but not yet output) as ready
// so that they can be output to client at EOS (End Of Stream).
// get_frame() is to be called in a loop right after drain() to
// get all those pending frames.
//
    pub ctx): *mut *mut int (drain)(struct delta_ctx,
}

//
// struct delta_ctx - instance structure.
//
// @flags:		validity of fields (streaminfo)
// @fh:			V4L2 file handle
// @dev:		device context
// @dec:		selected decoder context for this instance
// @ipc_ctx:		context of IPC communication with firmware
// @state:		instance state
// @frame_num:		frame number
// @au_num:		access unit number
// @max_au_size:	max size of an access unit
// @streaminfo:		stream information (width, height, dpb, interlacing...)
// @frameinfo:		frame information (width, height, format, alignment...)
// @nb_of_frames:	number of frames available for decoding
// @frames:		array of decoding frames to keep track of frame
// state and manage frame recycling
// @decoded_frames:	nb of decoded frames from opening
// @output_frames:	nb of output frames from opening
// @dropped_frames:	nb of frames dropped (ie access unit not parsed
// or frame decoded but not output)
// @stream_errors:	nb of stream errors (corrupted, not supported, ...)
// @decode_errors:	nb of decode errors (firmware error)
// @sys_errors:		nb of system errors (memory, ipc, ...)
// @dts:		FIFO of decoding timestamp.
// output frames are timestamped with incoming access
// unit timestamps using this fifo.
// @name:		string naming this instance (debug purpose)
// @run_work:		decoding work
// @lock:		lock for decoding work serialization
// @aborting:		true if current job aborted
// @priv:		private decoder context for this instance, allocated
// by decoder @open time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_ctx {
    pub flags: u32,
    pub fh: v4l2_fh,
    pub dev: *mut delta_dev,
    pub dec: *const delta_dec,
    pub ipc_ctx: delta_ipc_ctx,
    pub state: delta_state,
    pub frame_num: u32,
    pub au_num: u32,
    pub max_au_size: usize,
    pub streaminfo: delta_streaminfo,
    pub frameinfo: delta_frameinfo,
    pub nb_of_frames: u32,
    pub frames: [*mut delta_frame; DELTA_MAX_FRAMES],
    pub decoded_frames: u32,
    pub output_frames: u32,
    pub dropped_frames: u32,
    pub stream_errors: u32,
    pub decode_errors: u32,
    pub sys_errors: u32,
    pub dts: list_head,
    pub name: [c_char; 100],
    pub run_work: work_struct,
    pub lock: mutex,
    pub aborting: bool,
    pub priv: *mut c_void,
}

pub const DELTA_FLAG_STREAMINFO: c_uint = 0x0001;
pub const DELTA_FLAG_FRAMEINFO: c_uint = 0x0002;

//
// struct delta_dev - device struct, 1 per probe (so single one for
// all platform life)
//
// @v4l2_dev:		v4l2 device
// @vdev:		v4l2 video device
// @pdev:		platform device
// @dev:		device
// @m2m_dev:		memory-to-memory V4L2 device
// @lock:		device lock, for crit section & V4L2 ops serialization.
// @clk_delta:		delta main clock
// @clk_st231:		st231 coprocessor main clock
// @clk_flash_promip:	flash promip clock
// @decoders:		list of registered decoders
// @nb_of_decoders:	nb of registered decoders
// @pixelformats:	supported uncompressed video formats
// @nb_of_pixelformats:	number of supported umcompressed video formats
// @streamformats:	supported compressed video formats
// @nb_of_streamformats:number of supported compressed video formats
// @instance_id:	rolling counter identifying an instance (debug purpose)
// @work_queue:		decoding job work queue
// @rpmsg_driver:	rpmsg IPC driver
// @rpmsg_device:	rpmsg IPC device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_dev {
    pub v4l2_dev: v4l2_device,
    pub vdev: *mut video_device,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub lock: mutex,
    pub clk_delta: *mut clk,
    pub clk_st231: *mut clk,
    pub clk_flash_promip: *mut clk,
    pub decoders: [*const delta_dec; DELTA_MAX_DECODERS],
    pub nb_of_decoders: u32,
    pub pixelformats: [u32; DELTA_MAX_FORMATS],
    pub nb_of_pixelformats: u32,
    pub streamformats: [u32; DELTA_MAX_FORMATS],
    pub nb_of_streamformats: u32,
    pub instance_id: u8,
    pub work_queue: *mut workqueue_struct,
    pub rpmsg_driver: rpmsg_driver,
    pub rpmsg_device: *mut rpmsg_device,
}

extern "C" {
    pub fn delta_get_sync(ctx: *mut delta_ctx) -> c_int;
}
extern "C" {
    pub fn delta_put_autosuspend(ctx: *mut delta_ctx);
}
