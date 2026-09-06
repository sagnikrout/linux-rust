//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/uvc/uvcvideo.h
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

// --------------------------------------------------------------------------
// UVC constants
//
pub const UVC_TERM_INPUT: c_uint = 0x0000;
pub const UVC_TERM_OUTPUT: c_uint = 0x8000;

pub const UVC_EXT_GPIO_UNIT: c_uint = 0x7ffe;
pub const UVC_EXT_GPIO_UNIT_ID: c_uint = 0x100;

pub const UVC_INVALID_ENTITY_ID: c_uint = 0xffff;
// ------------------------------------------------------------------------
// Driver specific constants.
//

// Number of isochronous URBs.
pub const UVC_URBS: c_int = 5;
// Maximum number of packets per URB.
pub const UVC_MAX_PACKETS: c_int = 32;
pub const UVC_CTRL_CONTROL_TIMEOUT: c_int = 5000;
pub const UVC_CTRL_STREAMING_TIMEOUT: c_int = 5000;
// Maximum allowed number of control mappings per device
pub const UVC_MAX_CONTROL_MAPPINGS: c_int = 1024;
pub const UVC_MAX_CONTROL_MENU_ENTRIES: c_int = 32;
// Devices quirks
pub const UVC_QUIRK_STATUS_INTERVAL: c_uint = 0x00000001;
pub const UVC_QUIRK_PROBE_MINMAX: c_uint = 0x00000002;
pub const UVC_QUIRK_PROBE_EXTRAFIELDS: c_uint = 0x00000004;
pub const UVC_QUIRK_BUILTIN_ISIGHT: c_uint = 0x00000008;
pub const UVC_QUIRK_STREAM_NO_FID: c_uint = 0x00000010;
pub const UVC_QUIRK_IGNORE_SELECTOR_UNIT: c_uint = 0x00000020;
pub const UVC_QUIRK_FIX_BANDWIDTH: c_uint = 0x00000080;
pub const UVC_QUIRK_PROBE_DEF: c_uint = 0x00000100;
pub const UVC_QUIRK_RESTRICT_FRAME_RATE: c_uint = 0x00000200;
pub const UVC_QUIRK_RESTORE_CTRLS_ON_INIT: c_uint = 0x00000400;
pub const UVC_QUIRK_FORCE_Y8: c_uint = 0x00000800;
pub const UVC_QUIRK_FORCE_BPP: c_uint = 0x00001000;
pub const UVC_QUIRK_WAKE_AUTOSUSPEND: c_uint = 0x00002000;
pub const UVC_QUIRK_NO_RESET_RESUME: c_uint = 0x00004000;
pub const UVC_QUIRK_DISABLE_AUTOSUSPEND: c_uint = 0x00008000;
pub const UVC_QUIRK_INVALID_DEVICE_SOF: c_uint = 0x00010000;
pub const UVC_QUIRK_MJPEG_NO_EOF: c_uint = 0x00020000;
pub const UVC_QUIRK_MSXU_META: c_uint = 0x00040000;
// Format flags
pub const UVC_FMT_FLAG_COMPRESSED: c_uint = 0x00000001;
pub const UVC_FMT_FLAG_STREAM: c_uint = 0x00000002;
// ------------------------------------------------------------------------
// Structures.
//
// TODO: Put the most frequently accessed fields at the beginning of
// structures to maximize cache efficiency.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_control_info {
    pub mappings: list_head,
    pub entity: [u8; 16],
    pub /: *mut *mut u8 index; / Bit index in bmControls,
    pub selector: u8,
    pub size: u16,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_control_mapping {
    pub list: list_head,
    pub ev_subs: list_head,
    pub id: u32,
    pub name: *mut c_char,
    pub entity: [u8; 16],
    pub selector: u8,
//
// Size of the control data in the payload of the UVC control GET and
// SET requests, expressed in bits.
//
    pub size: u8,
    pub offset: u8,
    pub v4l2_type: v4l2_ctrl_type,
    pub data_type: u32,
    pub menu_mapping: *const u32,
    pub (*menu_names)[UVC_MENU_NAME_LEN]: *const c_char,
    pub menu_mask: c_ulong,
    pub master_id: u32,
    pub master_manual: i32,
    pub slave_ids: [u32; 2],
    pub disabled: bool,
    pub ctrl): *mut uvc_control,
    pub v4l2_out): *const *const void uvc_in, size_t v4l2_size, void,
    pub uvc_out): *const *const void v4l2_in, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_control {
    pub entity: *mut uvc_entity,
    pub info: uvc_control_info,
    pub /: *mut *mut u8 index; / Used to match the uvc_control entry with a uvc_control_info.,
    pub uvc_data: *mut u8,
    pub /: *mut *mut *mut uvc_fh handle; / File handle that last changed the control.,
}

//
// The term 'entity' refers to both UVC units and UVC terminals.
//
// The type field is either the terminal type (wTerminalType in the terminal
// descriptor), or the unit type (bDescriptorSubtype in the unit descriptor).
// As the bDescriptorSubtype field is one byte long, the type value will
// always have a null MSB for units. All terminal types defined by the UVC
// specification have a non-null MSB, so it is safe to use the MSB to
// differentiate between units and terminals as long as the descriptor parsing
// code makes sure terminal types have a non-null MSB.
//
// For terminals, the type's most significant bit stores the terminal
// direction (either UVC_TERM_INPUT or UVC_TERM_OUTPUT). The type field should
// always be accessed with the UVC_ENTITY_* macros and never directly.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_entity {
    pub /: *mut *mut list_head list; / Entity as part of a UVC device.,
    pub /: *mut *mut list_head chain; / Entity as part of a video device chain.,
    pub flags: c_uint,
//
// Entities exposed by the UVC device use IDs 0-255, extra entities
// implemented by the driver (such as the GPIO entity) use IDs 256 and
// up.
//
    pub id: u16,
    pub type: u16,
    pub name: [c_char; 64],
    pub guid: [u8; 16],
// Media controller-related fields.
    pub vdev: *mut video_device,
    pub subdev: v4l2_subdev,
    pub num_pads: c_uint,
    pub num_links: c_uint,
    pub pads: *mut media_pad,
    pub wObjectiveFocalLengthMin: u16,
    pub wObjectiveFocalLengthMax: u16,
    pub wOcularFocalLength: u16,
    pub bControlSize: u8,
    pub bmControls: *mut u8,
    pub camera: },
    pub bControlSize: u8,
    pub bmControls: *mut u8,
    pub bTransportModeSize: u8,
    pub bmTransportModes: *mut u8,
    pub media: },
    pub output: },
    pub wMaxMultiplier: u16,
    pub bControlSize: u8,
    pub bmControls: *mut u8,
    pub bmVideoStandards: u8,
    pub processing: },
    pub selector: },
    pub bNumControls: u8,
    pub bControlSize: u8,
    pub bmControls: *mut u8,
    pub bmControlsType: *mut u8,
    pub extension: },
    pub bControlSize: u8,
    pub bmControls: *mut u8,
    pub gpio_privacy: *mut gpio_desc,
    pub irq: c_int,
    pub initialized: bool,
    pub gpio: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_frame {
    pub bFrameIndex: u8,
    pub bmCapabilities: u8,
    pub wWidth: u16,
    pub wHeight: u16,
    pub dwMinBitRate: u32,
    pub dwMaxBitRate: u32,
    pub dwMaxVideoFrameBufferSize: u32,
    pub bFrameIntervalType: u8,
    pub dwDefaultFrameInterval: u32,
    pub dwFrameInterval: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_format {
    pub type: u8,
    pub index: u8,
    pub bpp: u8,
    pub colorspace: v4l2_colorspace,
    pub xfer_func: v4l2_xfer_func,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub fcc: u32,
    pub flags: u32,
    pub nframes: c_uint,
    pub frames: *const uvc_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_streaming_header {
    pub bNumFormats: u8,
    pub bEndpointAddress: u8,
    pub bTerminalLink: u8,
    pub bControlSize: u8,
    pub bmaControls: *mut u8,
// The following fields are used by input headers only.
    pub bmInfo: u8,
    pub bStillCaptureMethod: u8,
    pub bTriggerSupport: u8,
    pub bTriggerUsage: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uvc_buffer_state {
    UVC_BUF_STATE_IDLE	= 0,
    UVC_BUF_STATE_QUEUED	= 1,
    UVC_BUF_STATE_ACTIVE	= 2,
    UVC_BUF_STATE_READY	= 3,
    UVC_BUF_STATE_DONE	= 4,
    UVC_BUF_STATE_ERROR	= 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_buffer {
    pub buf: vb2_v4l2_buffer,
    pub queue: list_head,
    pub state: uvc_buffer_state,
    pub error: c_uint,
    pub mem: *mut c_void,
    pub length: c_uint,
    pub bytesused: c_uint,
    pub pts: u32,
// Asynchronous buffer handling.
    pub ref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_video_queue {
    pub stream: *mut uvc_streaming,
    pub vdev: video_device,
    pub queue: vb2_queue,
    pub /*: *mut mutex mutex;,
// Serializes vb2_queue and
// fops
//
    pub flags: c_uint,
    pub buf_used: c_uint,
    pub /: *mut *mut spinlock_t irqlock; / Protects irqqueue,
    pub irqqueue: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_video_chain {
    pub dev: *mut uvc_device,
    pub list: list_head,
    pub /: *mut *mut list_head entities; / All entities,
    pub /: *mut *mut *mut uvc_entity processing; / Processing unit,
    pub /: *mut *mut *mut uvc_entity selector; / Selector unit,
    pub /*: *mut mutex ctrl_mutex;,
// Protects ctrl.info,
// ctrl.handle and
// uvc_fh.pending_async_ctrls
//
    pub /: *mut *mut v4l2_prio_state prio; / V4L2 priority state,
    pub /: *mut *mut u32 caps; / V4L2 chain-wide caps,
    pub /: *mut *mut u8 ctrl_class_bitmap; / Bitmap of valid classes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_stats_frame {
    pub /: *mut *mut unsigned int size; / Number of bytes captured,
    pub /: *mut *mut unsigned int first_data; / Index of the first non-empty packet,
    pub /: *mut *mut unsigned int nb_packets; / Number of packets,
    pub /: *mut *mut unsigned int nb_empty; / Number of empty packets,
    pub /: *mut *mut unsigned int nb_invalid; / Number of packets with an invalid header,
    pub /: *mut *mut unsigned int nb_errors; / Number of packets with the error bit set,
    pub /: *mut *mut unsigned int nb_pts; / Number of packets with a PTS timestamp,
    pub /: *mut *mut unsigned int nb_pts_diffs; / Number of PTS differences inside a frame,
    pub /: *mut *mut unsigned int last_pts_diff; / Index of the last PTS difference,
    pub /: *mut *mut bool has_initial_pts; / Whether the first non-empty packet has a PTS,
    pub /: *mut *mut bool has_early_pts; / Whether a PTS is present before the first non-empty packet,
    pub /: *mut *mut u32 pts; / PTS of the last packet,
    pub /: *mut *mut unsigned int nb_scr; / Number of packets with a SCR timestamp,
    pub /: *mut *mut unsigned int nb_scr_diffs; / Number of SCR.STC differences inside a frame,
    pub /: *mut *mut u16 scr_sof; / SCR.SOF of the last packet,
    pub /: *mut *mut u32 scr_stc; / SCR.STC of the last packet,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_stats_stream {
    pub /: *mut *mut ktime_t start_ts; / Stream start timestamp,
    pub /: *mut *mut ktime_t stop_ts; / Stream stop timestamp,
    pub /: *mut *mut unsigned int nb_frames; / Number of frames,
    pub /: *mut *mut unsigned int nb_packets; / Number of packets,
    pub /: *mut *mut unsigned int nb_empty; / Number of empty packets,
    pub /: *mut *mut unsigned int nb_invalid; / Number of packets with an invalid header,
    pub /: *mut *mut unsigned int nb_errors; / Number of packets with the error bit set,
    pub /: *mut *mut unsigned int nb_pts_constant; / Number of frames with constant PTS,
    pub /: *mut *mut unsigned int nb_pts_early; / Number of frames with early PTS,
    pub /: *mut *mut unsigned int nb_pts_initial; / Number of frames with initial PTS,
    pub /: *mut *mut unsigned int nb_scr_count_ok; / Number of frames with at least one SCR per non empty packet,
    pub /: *mut *mut unsigned int nb_scr_diffs_ok; / Number of frames with varying SCR.STC,
    pub /: *mut *mut unsigned int scr_sof_count; / STC.SOF counter accumulated since stream start,
    pub /: *mut *mut unsigned int scr_sof; / STC.SOF of the last packet,
    pub /: *mut *mut unsigned int min_sof; / Minimum STC.SOF value,
    pub /: *mut *mut unsigned int max_sof; / Maximum STC.SOF value,
}

pub const UVC_METADATA_BUF_MIN_SIZE: c_int = 10240;
//
// struct uvc_copy_op: Context structure to schedule asynchronous memcpy
//
// @buf: active buf object for this operation
// @dst: copy destination address
// @src: copy source address
// @len: copy length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_copy_op {
    pub buf: *mut uvc_buffer,
    pub dst: *mut c_void,
    pub src: *const __u8,
    pub len: usize,
}

//
// struct uvc_urb - URB context management structure
//
// @urb: the URB described by this context structure
// @stream: UVC streaming context
// @buffer: memory storage for the URB
// @dma: Allocated DMA handle
// @sgt: sgt_table with the urb locations in memory
// @async_operations: counter to indicate the number of copy operations
// @copy_operations: work descriptors for asynchronous copy operations
// @work: work queue entry for asynchronous decode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_urb {
    pub urb: *mut urb,
    pub stream: *mut uvc_streaming,
    pub buffer: *mut c_char,
    pub dma: dma_addr_t,
    pub sgt: *mut sg_table,
    pub async_operations: c_uint,
    pub copy_operations: [uvc_copy_op; UVC_MAX_PACKETS],
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_streaming {
    pub list: list_head,
    pub dev: *mut uvc_device,
    pub chain: *mut uvc_video_chain,
    pub active: core::sync::atomic::AtomicI32,
    pub intf: *mut usb_interface,
    pub intfnum: c_int,
    pub maxpsize: u32,
    pub header: uvc_streaming_header,
    pub type: v4l2_buf_type,
    pub nformats: c_uint,
    pub formats: *const uvc_format,
    pub ctrl: uvc_streaming_control,
    pub def_format: *const uvc_format,
    pub cur_format: *const uvc_format,
    pub cur_frame: *const uvc_frame,
// Buffers queue.
    pub 1: unsigned int frozen :,
    pub queue: uvc_video_queue,
    pub async_wq: *mut workqueue_struct,
    pub meta_buf): *mut uvc_buffer,
    pub queue: uvc_video_queue,
    pub format: u32,
    pub buffersize: u32,
    pub meta: },
// Context data used by the bulk completion handler.
    pub header: [u8; 256],
    pub header_size: c_uint,
    pub skip_payload: c_int,
    pub payload_size: u32,
    pub max_payload_size: u32,
    pub bulk: },
    pub uvc_urb: [uvc_urb; UVC_URBS],
    pub urb_size: c_uint,
    pub sequence: u32,
    pub last_fid: u8,
// debugfs
    pub debugfs_dir: *mut dentry,
    pub frame: uvc_stats_frame,
    pub stream: uvc_stats_stream,
    pub stats: },
// Timestamps support.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_clock {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_clock_sample {
    pub dev_stc: u32,
    pub dev_sof: u16,
    pub host_sof: u16,
    pub host_time: ktime_t,
    pub samples: *mut },
    pub head: c_uint,
    pub count: c_uint,
    pub size: c_uint,
    pub last_sof_overflow: c_uint,
    pub last_sof_processed: u16,
    pub last_sof_raw: u16,
    pub sof_offset: u16,
    pub last_scr: [u8; 6],
    pub lock: spinlock_t,
    pub clock: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_device_info {
    pub quirks: u32,
    pub meta_format: u32,
    pub uvc_version: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_rect {
    pub top: u16,
    pub left: u16,
    pub bottom: u16,
    pub right: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_status_streaming {
    pub button: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_status_control {
    pub bSelector: u8,
    pub bAttribute: u8,
    pub bValue: [u8; 11],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_status {
    pub bStatusType: u8,
    pub bOriginator: u8,
    pub bEvent: u8,
    pub control: uvc_status_control,
    pub streaming: uvc_status_streaming,
}

pub const UVC_MAX_META_DATA_FORMATS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_device {
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub warnings: c_ulong,
    pub quirks: u32,
    pub intfnum: c_int,
    pub name: [c_char; 32],
    pub info: *const uvc_device_info,
    pub meta_formats: [u32; UVC_MAX_META_DATA_FORMATS],
    pub nmeta_formats: c_uint,
    pub nmappings: core::sync::atomic::AtomicI32,
// Video control interface

    pub mdev: media_device,

    pub vdev: v4l2_device,
    pub uvc_version: u16,
    pub clock_frequency: u32,
    pub entities: list_head,
    pub chains: list_head,
// Video Streaming interfaces
    pub streams: list_head,
    pub ref: kref,
// Status Interrupt Endpoint
    pub int_ep: *mut usb_host_endpoint,
    pub int_urb: *mut urb,
    pub status: *mut uvc_status,
    pub /: *mut *mut mutex status_lock; / Protects status_users,
    pub status_users: c_uint,
    pub flush_status: bool,
    pub input: *mut input_dev,
    pub input_phys: [c_char; 64],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_ctrl_work {
    pub work: work_struct,
    pub urb: *mut urb,
    pub chain: *mut uvc_video_chain,
    pub ctrl: *mut uvc_control,
    pub data: *const c_void,
    pub async_ctrl: },
    pub gpio_unit: *mut uvc_entity,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_fh {
    pub vfh: v4l2_fh,
    pub chain: *mut uvc_video_chain,
    pub stream: *mut uvc_streaming,
    pub pending_async_ctrls: c_uint,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), uvc_fh: struct, _arg: vfh) -> return;
}
// ------------------------------------------------------------------------
// Debugging, printing and logging
//

pub const UVC_WARN_MINMAX: c_int = 0;
pub const UVC_WARN_PROBE_DEF: c_int = 1;
pub const UVC_WARN_XU_GET_RES: c_int = 2;

// --------------------------------------------------------------------------
// Internal functions.
//
// Video buffers queue management.
extern "C" {
    pub fn uvc_queue_cancel(queue: *mut uvc_video_queue, disconnect: c_int);
}
extern "C" {
    pub fn uvc_queue_buffer_release(buf: *mut uvc_buffer);
}
extern "C" {
    pub fn vb2_is_streaming(_arg: &queue->queue) -> return;
}
// V4L2 interface
// Media controller
extern "C" {
    pub fn uvc_mc_register_entities(chain: *mut uvc_video_chain) -> c_int;
}
extern "C" {
    pub fn uvc_mc_cleanup_entity(entity: *mut uvc_entity);
}
// Video
extern "C" {
    pub fn uvc_video_init(stream: *mut uvc_streaming) -> c_int;
}
extern "C" {
    pub fn uvc_video_suspend(stream: *mut uvc_streaming) -> c_int;
}
extern "C" {
    pub fn uvc_video_resume(stream: *mut uvc_streaming, reset: c_int) -> c_int;
}
extern "C" {
    pub fn uvc_video_start_streaming(stream: *mut uvc_streaming) -> c_int;
}
extern "C" {
    pub fn uvc_video_stop_streaming(stream: *mut uvc_streaming);
}
extern "C" {
    pub fn uvc_meta_init(dev: *mut uvc_device) -> c_int;
}
extern "C" {
    pub fn uvc_meta_register(stream: *mut uvc_streaming) -> c_int;
}
// Status
extern "C" {
    pub fn uvc_status_init(dev: *mut uvc_device) -> c_int;
}
extern "C" {
    pub fn uvc_status_unregister(dev: *mut uvc_device);
}
extern "C" {
    pub fn uvc_status_cleanup(dev: *mut uvc_device);
}
extern "C" {
    pub fn uvc_status_resume(dev: *mut uvc_device) -> c_int;
}
extern "C" {
    pub fn uvc_status_suspend(dev: *mut uvc_device);
}
extern "C" {
    pub fn uvc_status_get(dev: *mut uvc_device) -> c_int;
}
extern "C" {
    pub fn uvc_status_put(dev: *mut uvc_device);
}
// PM
extern "C" {
    pub fn uvc_pm_get(dev: *mut uvc_device) -> c_int;
}
extern "C" {
    pub fn uvc_pm_put(dev: *mut uvc_device);
}
// Controls
extern "C" {
    pub fn uvc_ctrl_init_device(dev: *mut uvc_device) -> c_int;
}
extern "C" {
    pub fn uvc_ctrl_cleanup_device(dev: *mut uvc_device);
}
extern "C" {
    pub fn uvc_ctrl_restore_values(dev: *mut uvc_device) -> c_int;
}
extern "C" {
    pub fn uvc_ctrl_begin(chain: *mut uvc_video_chain) -> c_int;
}
extern "C" {
    pub fn __uvc_ctrl_commit(_arg: handle, _arg: 0, _arg: ctrls) -> return;
}
extern "C" {
    pub fn __uvc_ctrl_commit(_arg: handle, _arg: 1, _arg: NULL) -> return;
}
extern "C" {
    pub fn uvc_ctrl_set(handle: *mut uvc_fh, xctrl: *mut v4l2_ext_control) -> c_int;
}
extern "C" {
    pub fn uvc_ctrl_cleanup_fh(handle: *mut uvc_fh);
}
extern "C" {
    pub fn uvc_ctrl_is_privacy_control(entity[16]: u8, selector: u8) -> bool;
}
// Utility functions
// Quirks support
// debugfs and statistics
extern "C" {
    pub fn uvc_debugfs_init();
}
extern "C" {
    pub fn uvc_debugfs_cleanup();
}
extern "C" {
    pub fn uvc_debugfs_init_stream(stream: *mut uvc_streaming);
}
extern "C" {
    pub fn uvc_debugfs_cleanup_stream(stream: *mut uvc_streaming);
}
