//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-fw-isys.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2013--2024 Intel Corporation

// Max number of Input/Output Pins
pub const IPU6_MAX_IPINS: c_int = 4;

pub const IPU6_STREAM_ID_MAX: c_int = 16;
pub const IPU6_NONSECURE_STREAM_ID_MAX: c_int = 12;

pub const IPU6SE_STREAM_ID_MAX: c_int = 8;
pub const IPU6SE_NONSECURE_STREAM_ID_MAX: c_int = 4;

// Single return queue for all streams/commands type
pub const IPU6_N_MAX_MSG_RECV_QUEUES: c_int = 1;
// Single device queue for high priority commands (bypass in-order queue)
pub const IPU6_N_MAX_DEV_SEND_QUEUES: c_int = 1;
// Single dedicated send queue for proxy interface
pub const IPU6_N_MAX_PROXY_SEND_QUEUES: c_int = 1;
// Single dedicated recv queue for proxy interface
pub const IPU6_N_MAX_PROXY_RECV_QUEUES: c_int = 1;
// Send queues layout
pub const IPU6_BASE_PROXY_SEND_QUEUES: c_int = 0;

// Recv queues layout
pub const IPU6_BASE_PROXY_RECV_QUEUES: c_int = 0;

// Max number of planes for frame formats supported by the FW
pub const IPU6_PIN_PLANES_MAX: c_int = 4;
pub const IPU6_FW_ISYS_SENSOR_TYPE_START: c_int = 14;
pub const IPU6_FW_ISYS_SENSOR_TYPE_END: c_int = 19;
pub const IPU6SE_FW_ISYS_SENSOR_TYPE_START: c_int = 6;
pub const IPU6SE_FW_ISYS_SENSOR_TYPE_END: c_int = 11;
//
// Device close takes some time from last ack message to actual stopping
// of the SP processor. As long as the SP processor runs we can't proceed with
// clean up of resources.
//
pub const IPU6_ISYS_OPEN_RETRY: c_int = 2000;
pub const IPU6_ISYS_CLOSE_RETRY: c_int = 2000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_resp_type {
    IPU6_FW_ISYS_RESP_TYPE_STREAM_OPEN_DONE = 0,
    IPU6_FW_ISYS_RESP_TYPE_STREAM_START_ACK,
    IPU6_FW_ISYS_RESP_TYPE_STREAM_START_AND_CAPTURE_ACK,
    IPU6_FW_ISYS_RESP_TYPE_STREAM_CAPTURE_ACK,
    IPU6_FW_ISYS_RESP_TYPE_STREAM_STOP_ACK,
    IPU6_FW_ISYS_RESP_TYPE_STREAM_FLUSH_ACK,
    IPU6_FW_ISYS_RESP_TYPE_STREAM_CLOSE_ACK,
    IPU6_FW_ISYS_RESP_TYPE_PIN_DATA_READY,
    IPU6_FW_ISYS_RESP_TYPE_PIN_DATA_WATERMARK,
    IPU6_FW_ISYS_RESP_TYPE_FRAME_SOF,
    IPU6_FW_ISYS_RESP_TYPE_FRAME_EOF,
    IPU6_FW_ISYS_RESP_TYPE_STREAM_START_AND_CAPTURE_DONE,
    IPU6_FW_ISYS_RESP_TYPE_STREAM_CAPTURE_DONE,
    IPU6_FW_ISYS_RESP_TYPE_PIN_DATA_SKIPPED,
    IPU6_FW_ISYS_RESP_TYPE_STREAM_CAPTURE_SKIPPED,
    IPU6_FW_ISYS_RESP_TYPE_FRAME_SOF_DISCARDED,
    IPU6_FW_ISYS_RESP_TYPE_FRAME_EOF_DISCARDED,
    IPU6_FW_ISYS_RESP_TYPE_STATS_DATA_READY,
    N_IPU6_FW_ISYS_RESP_TYPE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_send_type {
    IPU6_FW_ISYS_SEND_TYPE_STREAM_OPEN = 0,
    IPU6_FW_ISYS_SEND_TYPE_STREAM_START,
    IPU6_FW_ISYS_SEND_TYPE_STREAM_START_AND_CAPTURE,
    IPU6_FW_ISYS_SEND_TYPE_STREAM_CAPTURE,
    IPU6_FW_ISYS_SEND_TYPE_STREAM_STOP,
    IPU6_FW_ISYS_SEND_TYPE_STREAM_FLUSH,
    IPU6_FW_ISYS_SEND_TYPE_STREAM_CLOSE,
    N_IPU6_FW_ISYS_SEND_TYPE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_queue_type {
    IPU6_FW_ISYS_QUEUE_TYPE_PROXY = 0,
    IPU6_FW_ISYS_QUEUE_TYPE_DEV,
    IPU6_FW_ISYS_QUEUE_TYPE_MSG,
    N_IPU6_FW_ISYS_QUEUE_TYPE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_stream_source {
    IPU6_FW_ISYS_STREAM_SRC_PORT_0 = 0,
    IPU6_FW_ISYS_STREAM_SRC_PORT_1,
    IPU6_FW_ISYS_STREAM_SRC_PORT_2,
    IPU6_FW_ISYS_STREAM_SRC_PORT_3,
    IPU6_FW_ISYS_STREAM_SRC_PORT_4,
    IPU6_FW_ISYS_STREAM_SRC_PORT_5,
    IPU6_FW_ISYS_STREAM_SRC_PORT_6,
    IPU6_FW_ISYS_STREAM_SRC_PORT_7,
    IPU6_FW_ISYS_STREAM_SRC_PORT_8,
    IPU6_FW_ISYS_STREAM_SRC_PORT_9,
    IPU6_FW_ISYS_STREAM_SRC_PORT_10,
    IPU6_FW_ISYS_STREAM_SRC_PORT_11,
    IPU6_FW_ISYS_STREAM_SRC_PORT_12,
    IPU6_FW_ISYS_STREAM_SRC_PORT_13,
    IPU6_FW_ISYS_STREAM_SRC_PORT_14,
    IPU6_FW_ISYS_STREAM_SRC_PORT_15,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_0,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_1,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_2,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_3,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_4,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_5,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_6,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_7,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_8,
    IPU6_FW_ISYS_STREAM_SRC_MIPIGEN_9,
    N_IPU6_FW_ISYS_STREAM_SRC
}

//
// enum ipu6_fw_isys_mipi_vc: MIPI csi2 spec
// supports up to 4 virtual per physical channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_mipi_vc {
    IPU6_FW_ISYS_MIPI_VC_0 = 0,
    IPU6_FW_ISYS_MIPI_VC_1,
    IPU6_FW_ISYS_MIPI_VC_2,
    IPU6_FW_ISYS_MIPI_VC_3,
    N_IPU6_FW_ISYS_MIPI_VC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_frame_format_type {
    IPU6_FW_ISYS_FRAME_FORMAT_NV11 = 0, /* 12 bit YUV 411, Y, UV plane */
    IPU6_FW_ISYS_FRAME_FORMAT_NV12,	/* 12 bit YUV 420, Y, UV plane */
    IPU6_FW_ISYS_FRAME_FORMAT_NV12_16, /* 16 bit YUV 420, Y, UV plane */
// 12 bit YUV 420, Intel proprietary tiled format
    IPU6_FW_ISYS_FRAME_FORMAT_NV12_TILEY,

    IPU6_FW_ISYS_FRAME_FORMAT_NV16,	/* 16 bit YUV 422, Y, UV plane */
    IPU6_FW_ISYS_FRAME_FORMAT_NV21,	/* 12 bit YUV 420, Y, VU plane */
    IPU6_FW_ISYS_FRAME_FORMAT_NV61,	/* 16 bit YUV 422, Y, VU plane */
    IPU6_FW_ISYS_FRAME_FORMAT_YV12,	/* 12 bit YUV 420, Y, V, U plane */
    IPU6_FW_ISYS_FRAME_FORMAT_YV16,	/* 16 bit YUV 422, Y, V, U plane */
    IPU6_FW_ISYS_FRAME_FORMAT_YUV420, /* 12 bit YUV 420, Y, U, V plane */
    IPU6_FW_ISYS_FRAME_FORMAT_YUV420_10, /* yuv420, 10 bits per subpixel */
    IPU6_FW_ISYS_FRAME_FORMAT_YUV420_12, /* yuv420, 12 bits per subpixel */
    IPU6_FW_ISYS_FRAME_FORMAT_YUV420_14, /* yuv420, 14 bits per subpixel */
    IPU6_FW_ISYS_FRAME_FORMAT_YUV420_16, /* yuv420, 16 bits per subpixel */
    IPU6_FW_ISYS_FRAME_FORMAT_YUV422, /* 16 bit YUV 422, Y, U, V plane */
    IPU6_FW_ISYS_FRAME_FORMAT_YUV422_16, /* yuv422, 16 bits per subpixel */
    IPU6_FW_ISYS_FRAME_FORMAT_UYVY,	/* 16 bit YUV 422, UYVY interleaved */
    IPU6_FW_ISYS_FRAME_FORMAT_YUYV,	/* 16 bit YUV 422, YUYV interleaved */
    IPU6_FW_ISYS_FRAME_FORMAT_YUV444, /* 24 bit YUV 444, Y, U, V plane */
// Internal format, 2 y lines followed by a uvinterleaved line
    IPU6_FW_ISYS_FRAME_FORMAT_YUV_LINE,
    IPU6_FW_ISYS_FRAME_FORMAT_RAW8,	/* RAW8, 1 plane */
    IPU6_FW_ISYS_FRAME_FORMAT_RAW10, /* RAW10, 1 plane */
    IPU6_FW_ISYS_FRAME_FORMAT_RAW12, /* RAW12, 1 plane */
    IPU6_FW_ISYS_FRAME_FORMAT_RAW14, /* RAW14, 1 plane */
    IPU6_FW_ISYS_FRAME_FORMAT_RAW16, /* RAW16, 1 plane */
//
// 16 bit RGB, 1 plane. Each 3 sub pixels are packed into one 16 bit
// value, 5 bits for R, 6 bits for G and 5 bits for B.
//
    IPU6_FW_ISYS_FRAME_FORMAT_RGB565,
    IPU6_FW_ISYS_FRAME_FORMAT_PLANAR_RGB888, /* 24 bit RGB, 3 planes */
    IPU6_FW_ISYS_FRAME_FORMAT_RGBA888, /* 32 bit RGBA, 1 plane, A=Alpha */
    IPU6_FW_ISYS_FRAME_FORMAT_QPLANE6, /* Internal, for advanced ISP */
    IPU6_FW_ISYS_FRAME_FORMAT_BINARY_8, /* byte stream, used for jpeg. */
    N_IPU6_FW_ISYS_FRAME_FORMAT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_pin_type {
// captured as MIPI packets
    IPU6_FW_ISYS_PIN_TYPE_MIPI = 0,
// captured through the SoC path
    IPU6_FW_ISYS_PIN_TYPE_RAW_SOC = 3,
}

//
// enum ipu6_fw_isys_mipi_store_mode. Describes if long MIPI packets reach
// MIPI SRAM with the long packet header or
// if not, then only option is to capture it with pin type MIPI.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_mipi_store_mode {
    IPU6_FW_ISYS_MIPI_STORE_MODE_NORMAL = 0,
    IPU6_FW_ISYS_MIPI_STORE_MODE_DISCARD_LONG_HEADER,
    N_IPU6_FW_ISYS_MIPI_STORE_MODE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_capture_mode {
    IPU6_FW_ISYS_CAPTURE_MODE_REGULAR = 0,
    IPU6_FW_ISYS_CAPTURE_MODE_BURST,
    N_IPU6_FW_ISYS_CAPTURE_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_sensor_mode {
    IPU6_FW_ISYS_SENSOR_MODE_NORMAL = 0,
    IPU6_FW_ISYS_SENSOR_MODE_TOBII,
    N_IPU6_FW_ISYS_SENSOR_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_isys_error {
    IPU6_FW_ISYS_ERROR_NONE = 0,
    IPU6_FW_ISYS_ERROR_FW_INTERNAL_CONSISTENCY,
    IPU6_FW_ISYS_ERROR_HW_CONSISTENCY,
    IPU6_FW_ISYS_ERROR_DRIVER_INVALID_COMMAND_SEQUENCE,
    IPU6_FW_ISYS_ERROR_DRIVER_INVALID_DEVICE_CONFIGURATION,
    IPU6_FW_ISYS_ERROR_DRIVER_INVALID_STREAM_CONFIGURATION,
    IPU6_FW_ISYS_ERROR_DRIVER_INVALID_FRAME_CONFIGURATION,
    IPU6_FW_ISYS_ERROR_INSUFFICIENT_RESOURCES,
    IPU6_FW_ISYS_ERROR_HW_REPORTED_STR2MMIO,
    IPU6_FW_ISYS_ERROR_HW_REPORTED_SIG2CIO,
    IPU6_FW_ISYS_ERROR_SENSOR_FW_SYNC,
    IPU6_FW_ISYS_ERROR_STREAM_IN_SUSPENSION,
    IPU6_FW_ISYS_ERROR_RESPONSE_QUEUE_FULL,
    N_IPU6_FW_ISYS_ERROR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_fw_proxy_error {
    IPU6_FW_PROXY_ERROR_NONE = 0,
    IPU6_FW_PROXY_ERROR_INVALID_WRITE_REGION,
    IPU6_FW_PROXY_ERROR_INVALID_WRITE_OFFSET,
    N_IPU6_FW_PROXY_ERROR
}

// firmware ABI structure below are aligned in firmware, no need pack
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_buffer_partition_abi {
    pub num_gda_pages: [u32; IPU6_STREAM_ID_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_fw_config {
    pub buffer_partition: ipu6_fw_isys_buffer_partition_abi,
    pub num_send_queues: [u32; N_IPU6_FW_ISYS_QUEUE_TYPE],
    pub num_recv_queues: [u32; N_IPU6_FW_ISYS_QUEUE_TYPE],
}

//
// struct ipu6_fw_isys_resolution_abi: Generic resolution structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_resolution_abi {
    pub width: u32,
    pub height: u32,
}

//
// struct ipu6_fw_isys_output_pin_payload_abi - ISYS output pin buffer
// @out_buf_id: Points to output pin buffer - buffer identifier
// @addr: Points to output pin buffer - CSS Virtual Address
// @compress: Request frame compression (1), or  not (0)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_output_pin_payload_abi {
    pub out_buf_id: u64,
    pub addr: u32,
    pub compress: u32,
}

//
// struct ipu6_fw_isys_output_pin_info_abi - ISYS output pin info
// @output_res: output pin resolution
// @stride: output stride in Bytes (not valid for statistics)
// @watermark_in_lines: pin watermark level in lines
// @payload_buf_size: minimum size in Bytes of all buffers that will be
// supplied for capture on this pin
// @ts_offsets: ts_offsets
// @s2m_pixel_soc_pixel_remapping: pixel soc remapping (see the definition of
// S2M_PIXEL_SOC_PIXEL_REMAPPING_FLAG_NO_REMAPPING)
// @csi_be_soc_pixel_remapping: see s2m_pixel_soc_pixel_remapping
// @send_irq: assert if pin event should trigger irq
// @input_pin_id: related input pin id
// @pt: pin type -real format "enum ipu6_fw_isys_pin_type"
// @ft: frame format type -real format "enum ipu6_fw_isys_frame_format_type"
// @reserved: a reserved field
// @reserve_compression: reserve compression resources for pin
// @snoopable: snoopable
// @error_handling_enable: enable error handling
// @sensor_type: sensor_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_output_pin_info_abi {
    pub output_res: ipu6_fw_isys_resolution_abi,
    pub stride: u32,
    pub watermark_in_lines: u32,
    pub payload_buf_size: u32,
    pub ts_offsets: [u32; IPU6_PIN_PLANES_MAX],
    pub s2m_pixel_soc_pixel_remapping: u32,
    pub csi_be_soc_pixel_remapping: u32,
    pub send_irq: u8,
    pub input_pin_id: u8,
    pub pt: u8,
    pub ft: u8,
    pub reserved: u8,
    pub reserve_compression: u8,
    pub snoopable: u8,
    pub error_handling_enable: u8,
    pub sensor_type: u32,
}

//
// struct ipu6_fw_isys_input_pin_info_abi - ISYS input pin info
// @input_res: input resolution
// @dt: mipi data type ((enum ipu6_fw_isys_mipi_data_type)
// @mipi_store_mode: defines if legacy long packet header will be stored or
// discarded if discarded, output pin type for this
// input pin can only be MIPI
// (enum ipu6_fw_isys_mipi_store_mode)
// @bits_per_pix: native bits per pixel
// @mapped_dt: actual data type from sensor
// @mipi_decompression: defines which compression will be in mipi backend
// @crop_first_and_last_lines: Control whether to crop the first and last line
// of the input image. Crop done by HW device.
// @capture_mode: mode of capture, regular or burst, default value is regular
// @reserved: a reserved field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_input_pin_info_abi {
    pub input_res: ipu6_fw_isys_resolution_abi,
    pub dt: u8,
    pub mipi_store_mode: u8,
    pub bits_per_pix: u8,
    pub mapped_dt: u8,
    pub mipi_decompression: u8,
    pub crop_first_and_last_lines: u8,
    pub capture_mode: u8,
    pub reserved: u8,
}

//
// struct ipu6_fw_isys_cropping_abi - ISYS cropping coordinates
// @top_offset: Top offset
// @left_offset: Left offset
// @bottom_offset: Bottom offset
// @right_offset: Right offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_cropping_abi {
    pub top_offset: i32,
    pub left_offset: i32,
    pub bottom_offset: i32,
    pub right_offset: i32,
}

//
// struct ipu6_fw_isys_stream_cfg_data_abi - ISYS stream configuration data
// ISYS stream configuration data structure
// @crop: for extended use and is not used in FW currently
// @input_pins: input pin descriptors
// @output_pins: output pin descriptors
// @compfmt: de-compression setting for User Defined Data
// @nof_input_pins: number of input pins
// @nof_output_pins: number of output pins
// @send_irq_sof_discarded: send irq on discarded frame sof response
// - if '1' it will override the send_resp_sof_discarded
// and send the response
// - if '0' the send_resp_sof_discarded will determine
// whether to send the response
// @send_irq_eof_discarded: send irq on discarded frame eof response
// - if '1' it will override the send_resp_eof_discarded
// and send the response
// - if '0' the send_resp_eof_discarded will determine
// whether to send the response
// @send_resp_sof_discarded: send response for discarded frame sof detected,
// used only when send_irq_sof_discarded is '0'
// @send_resp_eof_discarded: send response for discarded frame eof detected,
// used only when send_irq_eof_discarded is '0'
// @src: Stream source index e.g. MIPI_generator_0, CSI2-rx_1
// @vc: MIPI Virtual Channel (up to 4 virtual per physical channel)
// @isl_use: indicates whether stream requires ISL and how
// @sensor_type: type of connected sensor, tobii or others, default is 0
// @reserved: a reserved field
// @reserved2: a reserved field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_stream_cfg_data_abi {
    pub crop: ipu6_fw_isys_cropping_abi,
    pub input_pins: [ipu6_fw_isys_input_pin_info_abi; IPU6_MAX_IPINS],
    pub output_pins: [ipu6_fw_isys_output_pin_info_abi; IPU6_MAX_OPINS],
    pub compfmt: u32,
    pub nof_input_pins: u8,
    pub nof_output_pins: u8,
    pub send_irq_sof_discarded: u8,
    pub send_irq_eof_discarded: u8,
    pub send_resp_sof_discarded: u8,
    pub send_resp_eof_discarded: u8,
    pub src: u8,
    pub vc: u8,
    pub isl_use: u8,
    pub sensor_type: u8,
    pub reserved: u8,
    pub reserved2: u8,
}

//
// struct ipu6_fw_isys_frame_buff_set_abi - ISYS frame buffer set (request)
// @output_pins: output pin addresses
// @send_irq_sof: send irq on frame sof response
// - if '1' it will override the send_resp_sof and
// send the response
// - if '0' the send_resp_sof will determine whether to
// send the response
// @send_irq_eof: send irq on frame eof response
// - if '1' it will override the send_resp_eof and
// send the response
// - if '0' the send_resp_eof will determine whether to
// send the response
// @send_irq_capture_ack: send irq on capture ack
// @send_irq_capture_done: send irq on capture done
// @send_resp_sof: send response for frame sof detected,
// used only when send_irq_sof is '0'
// @send_resp_eof: send response for frame eof detected,
// used only when send_irq_eof is '0'
// @send_resp_capture_ack: send response for capture ack event
// @send_resp_capture_done: send response for capture done event
// @reserved: a reserved field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_frame_buff_set_abi {
    pub output_pins: [ipu6_fw_isys_output_pin_payload_abi; IPU6_MAX_OPINS],
    pub send_irq_sof: u8,
    pub send_irq_eof: u8,
    pub send_irq_capture_ack: u8,
    pub send_irq_capture_done: u8,
    pub send_resp_sof: u8,
    pub send_resp_eof: u8,
    pub send_resp_capture_ack: u8,
    pub send_resp_capture_done: u8,
    pub reserved: [u8; 8],
}

//
// struct ipu6_fw_isys_error_info_abi - ISYS error information
// @error: error code if something went wrong
// @error_details: depending on error code, it may contain additional error info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_error_info_abi {
    pub error: u32,
    pub error_details: u32,
}

//
// struct ipu6_fw_isys_resp_info_abi - ISYS firmware response
// @buf_id: buffer ID
// @pin: this var is only valid for pin event related responses,
// contains pin addresses
// @error_info: error information from the FW
// @timestamp: Time information for event if available
// @stream_handle: stream id the response corresponds to
// @type: response type (enum ipu6_fw_isys_resp_type)
// @pin_id: pin id that the pin payload corresponds to
// @reserved: a reserved field
// @reserved2: a reserved field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_resp_info_abi {
    pub buf_id: u64,
    pub pin: ipu6_fw_isys_output_pin_payload_abi,
    pub error_info: ipu6_fw_isys_error_info_abi,
    pub timestamp: [u32; 2],
    pub stream_handle: u8,
    pub type: u8,
    pub pin_id: u8,
    pub reserved: u8,
    pub reserved2: u32,
}

//
// struct ipu6_fw_isys_proxy_error_info_abi - ISYS proxy error
// @error: error code if something went wrong
// @error_details: depending on error code, it may contain additional error info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_proxy_error_info_abi {
    pub error: u32,
    pub error_details: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_isys_proxy_resp_info_abi {
    pub request_id: u32,
    pub error_info: ipu6_fw_isys_proxy_error_info_abi,
}

//
// struct ipu6_fw_proxy_write_queue_token - ISYS proxy write queue token
// @request_id: update id for the specific proxy write request
// @region_index: Region id for the proxy write request
// @offset: Offset of the write request according to the base address
// of the region
// @value: Value that is requested to be written with the proxy write request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_proxy_write_queue_token {
    pub request_id: u32,
    pub region_index: u32,
    pub offset: u32,
    pub value: u32,
}

//
// struct ipu6_fw_resp_queue_token - ISYS response queue token
// @resp_info: response info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_resp_queue_token {
    pub resp_info: ipu6_fw_isys_resp_info_abi,
}

//
// struct ipu6_fw_send_queue_token - ISYS send queue token
// @buf_handle: buffer handle
// @payload: payload
// @send_type: send_type
// @stream_id: stream_id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_send_queue_token {
    pub buf_handle: u64,
    pub payload: u32,
    pub send_type: u16,
    pub stream_id: u16,
}

//
// struct ipu6_fw_proxy_resp_queue_token - ISYS proxy response queue token
// @proxy_resp_info: proxy response info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_proxy_resp_queue_token {
    pub proxy_resp_info: ipu6_fw_isys_proxy_resp_info_abi,
}

//
// struct ipu6_fw_proxy_send_queue_token - SYS proxy send queue token
// @request_id: request_id
// @region_index: region_index
// @offset: offset
// @value: value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_fw_proxy_send_queue_token {
    pub request_id: u32,
    pub region_index: u32,
    pub offset: u32,
    pub value: u32,
}

extern "C" {
    pub fn ipu6_fw_isys_init(isys: *mut ipu6_isys, num_streams: c_uint) -> c_int;
}
extern "C" {
    pub fn ipu6_fw_isys_close(isys: *mut ipu6_isys) -> c_int;
}
extern "C" {
    pub fn ipu6_fw_isys_cleanup(isys: *mut ipu6_isys);
}
extern "C" {
    pub fn ipu6_fw_isys_put_resp(context: *mut c_void, queue: c_uint);
}
