//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/raspberrypi/vchiq-mmal/mmal-msg.h
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
// Broadcom BCM2835 V4L2 driver
//
// Copyright © 2013 Raspberry Pi (Trading) Ltd.
//
// Authors: Vincent Sanders @ Collabora
// Dave Stevenson @ Broadcom
// (now dave.stevenson@raspberrypi.org)
// Simon Mellor @ Broadcom
// Luke Diamand @ Broadcom
//
// all the data structures which serialise the MMAL protocol. note
// these are directly mapped onto the received message data.
//
// BEWARE: They seem to *assume* pointers are u32 and that there is no
// structure padding!
//
// NOTE: this implementation uses kernel types to ensure sizes. Rather
// than assigning values to enums to force their size the
// implementation uses fixed size types and not the enums (though the
// comments have the actual enum type
//
pub const VC_MMAL_VER: c_int = 15;
pub const VC_MMAL_MIN_VER: c_int = 10;
// max total message size is 512 bytes
pub const MMAL_MSG_MAX_SIZE: c_int = 512;
// with six 32bit header elements max payload is therefore 488 bytes
pub const MMAL_MSG_MAX_PAYLOAD: c_int = 488;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmal_msg_type {
    MMAL_MSG_TYPE_QUIT = 1,
    MMAL_MSG_TYPE_SERVICE_CLOSED,
    MMAL_MSG_TYPE_GET_VERSION,
    MMAL_MSG_TYPE_COMPONENT_CREATE,
    MMAL_MSG_TYPE_COMPONENT_DESTROY,	/* 5 */
    MMAL_MSG_TYPE_COMPONENT_ENABLE,
    MMAL_MSG_TYPE_COMPONENT_DISABLE,
    MMAL_MSG_TYPE_PORT_INFO_GET,
    MMAL_MSG_TYPE_PORT_INFO_SET,
    MMAL_MSG_TYPE_PORT_ACTION,		/* 10 */
    MMAL_MSG_TYPE_BUFFER_FROM_HOST,
    MMAL_MSG_TYPE_BUFFER_TO_HOST,
    MMAL_MSG_TYPE_GET_STATS,
    MMAL_MSG_TYPE_PORT_PARAMETER_SET,
    MMAL_MSG_TYPE_PORT_PARAMETER_GET,	/* 15 */
    MMAL_MSG_TYPE_EVENT_TO_HOST,
    MMAL_MSG_TYPE_GET_CORE_STATS_FOR_PORT,
    MMAL_MSG_TYPE_OPAQUE_ALLOCATOR,
    MMAL_MSG_TYPE_CONSUME_MEM,
    MMAL_MSG_TYPE_LMK,			/* 20 */
    MMAL_MSG_TYPE_OPAQUE_ALLOCATOR_DESC,
    MMAL_MSG_TYPE_DRM_GET_LHS32,
    MMAL_MSG_TYPE_DRM_GET_TIME,
    MMAL_MSG_TYPE_BUFFER_FROM_HOST_ZEROLEN,
    MMAL_MSG_TYPE_PORT_FLUSH,		/* 25 */
    MMAL_MSG_TYPE_HOST_LOG,
    MMAL_MSG_TYPE_MSG_LAST
}

// port action request messages differ depending on the action type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmal_msg_port_action_type {
    MMAL_MSG_PORT_ACTION_TYPE_UNKNOWN = 0,	/* Unknown action */
    MMAL_MSG_PORT_ACTION_TYPE_ENABLE,	/* Enable a port */
    MMAL_MSG_PORT_ACTION_TYPE_DISABLE,	/* Disable a port */
    MMAL_MSG_PORT_ACTION_TYPE_FLUSH,	/* Flush a port */
    MMAL_MSG_PORT_ACTION_TYPE_CONNECT,	/* Connect ports */
    MMAL_MSG_PORT_ACTION_TYPE_DISCONNECT,	/* Disconnect ports */
    MMAL_MSG_PORT_ACTION_TYPE_SET_REQUIREMENTS, /* Set buffer requirements*/
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_header {
    pub magic: u32,
    pub /: *mut *mut u32 type; / enum mmal_msg_type,
// Opaque handle to the control service
    pub control_service: u32,
    pub /: *mut *mut u32 context; / a u32 per message context,
    pub /: *mut *mut u32 status; / The status of the vchiq operation,
    pub padding: u32,
}

// Send from VC to host to report version
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_version {
    pub flags: u32,
    pub major: u32,
    pub minor: u32,
    pub minimum: u32,
}

// request to VC to create component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_component_create {
    pub /: *mut *mut u32 client_component; / component context,
    pub name: [c_char; 128],
    pub /: *mut *mut u32 pid; / For debug,
}

// reply from VC to component creation request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_component_create_reply {
    pub to: *mut *mut u32 status; / enum mmal_msg_status - how does this differ,
// the one in the header?
//
    pub /: *mut *mut u32 component_handle; / VideoCore handle for component,
    pub /: *mut *mut u32 input_num; / Number of input ports,
    pub /: *mut *mut u32 output_num; / Number of output ports,
    pub /: *mut *mut u32 clock_num; / Number of clock ports,
}

// request to VC to destroy a component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_component_destroy {
    pub component_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_component_destroy_reply {
    pub /: *mut *mut u32 status; / The component destruction status,
}

// request and reply to VC to enable a component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_component_enable {
    pub component_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_component_enable_reply {
    pub /: *mut *mut u32 status; / The component enable status,
}

// request and reply to VC to disable a component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_component_disable {
    pub component_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_component_disable_reply {
    pub /: *mut *mut u32 status; / The component disable status,
}

// request to VC to get port information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_info_get {
    pub /: *mut *mut u32 component_handle; / component handle port is associated with,
    pub /: *mut *mut u32 port_type; / enum mmal_msg_port_type,
    pub /: *mut *mut u32 index; / port index to query,
}

// reply from VC to get port info request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_info_get_reply {
    pub /: *mut *mut u32 status; / enum mmal_msg_status,
    pub /: *mut *mut u32 component_handle; / component handle port is associated with,
    pub /: *mut *mut u32 port_type; / enum mmal_msg_port_type,
    pub /: *mut *mut u32 port_index; / port indexed in query,
    pub /: *mut *mut s32 found; / unused,
    pub /: *mut *mut u32 port_handle; / Handle to use for this port,
    pub port: mmal_port,
    pub /: *mut *mut mmal_es_format format; / elementary stream format,
    pub /: *mut *mut mmal_es_specific_format es; / es type specific data,
    pub /: *mut *mut u8 extradata[MMAL_FORMAT_EXTRADATA_MAX_SIZE]; / es extra data,
}

// request to VC to set port information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_info_set {
    pub component_handle: u32,
    pub /: *mut *mut u32 port_type; / enum mmal_msg_port_type,
    pub /: *mut *mut u32 port_index; / port indexed in query,
    pub port: mmal_port,
    pub format: mmal_es_format,
    pub es: mmal_es_specific_format,
    pub extradata: [u8; MMAL_FORMAT_EXTRADATA_MAX_SIZE],
}

// reply from VC to port info set request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_info_set_reply {
    pub status: u32,
    pub /: *mut *mut u32 component_handle; / component handle port is associated with,
    pub /: *mut *mut u32 port_type; / enum mmal_msg_port_type,
    pub /: *mut *mut u32 index; / port indexed in query,
    pub /: *mut *mut s32 found; / unused,
    pub /: *mut *mut u32 port_handle; / Handle to use for this port,
    pub port: mmal_port,
    pub format: mmal_es_format,
    pub es: mmal_es_specific_format,
    pub extradata: [u8; MMAL_FORMAT_EXTRADATA_MAX_SIZE],
}

// port action requests that take a mmal_port as a parameter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_action_port {
    pub component_handle: u32,
    pub port_handle: u32,
    pub /: *mut *mut u32 action; / enum mmal_msg_port_action_type,
    pub port: mmal_port,
}

// port action requests that take handles as a parameter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_action_handle {
    pub component_handle: u32,
    pub port_handle: u32,
    pub /: *mut *mut u32 action; / enum mmal_msg_port_action_type,
    pub connect_component_handle: u32,
    pub connect_port_handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_action_reply {
    pub /: *mut *mut u32 status; / The port action operation status,
}

// MMAL buffer transfer
// Size of space reserved in a buffer message for short messages.
pub const MMAL_VC_SHORT_DATA: c_int = 128;
// Signals that the current payload is the end of the stream of data

// Signals that the start of the current payload starts a frame

// Signals that the end of the current payload ends a frame

// Signals that the current payload contains only complete frames (>1)

// Signals that the current payload is a keyframe (i.e. self decodable)

//
// Signals a discontinuity in the stream of data (e.g. after a seek).
// Can be used for instance by a decoder to reset its state
//

//
// Signals a buffer containing some kind of config data for the component
// (e.g. codec config data)
//

// Signals an encrypted payload

// Signals a buffer containing side information

//
// Signals a buffer which is the snapshot/postview image from a stills
// capture
//

// Signals a buffer which contains data known to be corrupted

// Signals that a buffer failed to be transmitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_driver_buffer {
    pub magic: u32,
    pub component_handle: u32,
    pub port_handle: u32,
    pub client_context: u32,
}

// buffer header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_buffer_header {
    pub /: *mut *mut u32 next; / next header,
    pub /: *mut *mut u32 priv; / framework private data,
    pub cmd: u32,
    pub data: u32,
    pub alloc_size: u32,
    pub length: u32,
    pub offset: u32,
    pub flags: u32,
    pub pts: i64,
    pub dts: i64,
    pub type: u32,
    pub user_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_buffer_header_type_specific {
    pub planes: u32,
    pub offset: [u32; 4],
    pub pitch: [u32; 4],
    pub flags: u32,
    pub video: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_buffer_from_host {
//
// The front 32 bytes of the buffer header are copied
// back to us in the reply to allow for context. This
// area is used to store two mmal_driver_buffer structures to
// allow for multiple concurrent service users.
//
// control data
    pub drvbuf: mmal_driver_buffer,
// referenced control data for passthrough buffer management
    pub drvbuf_ref: mmal_driver_buffer,
    pub /: *mut *mut mmal_buffer_header buffer_header; / buffer header itself,
    pub buffer_header_type_specific: mmal_buffer_header_type_specific,
    pub is_zero_copy: i32,
    pub has_reference: i32,
// allows short data to be xfered in control message
    pub payload_in_message: u32,
    pub short_data: [u8; MMAL_VC_SHORT_DATA],
}

// port parameter setting
pub const MMAL_WORKER_PORT_PARAMETER_SPACE: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_parameter_set {
    pub /: *mut *mut u32 component_handle; / component,
    pub /: *mut *mut u32 port_handle; / port,
    pub /: *mut *mut u32 id; / Parameter ID,
    pub /: *mut *mut u32 size; / Parameter size,
    pub value: [u32; MMAL_WORKER_PORT_PARAMETER_SPACE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_parameter_set_reply {
    pub this: *mut *mut u32 status; / enum mmal_msg_status todo: how does,
// differ to the one in the header?
//
}

// port parameter getting
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_parameter_get {
    pub /: *mut *mut u32 component_handle; / component,
    pub /: *mut *mut u32 port_handle; / port,
    pub /: *mut *mut u32 id; / Parameter ID,
    pub /: *mut *mut u32 size; / Parameter size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_port_parameter_get_reply {
    pub /: *mut *mut u32 status; / Status of mmal_port_parameter_get call,
    pub /: *mut *mut u32 id; / Parameter ID,
    pub /: *mut *mut u32 size; / Parameter size,
    pub value: [u32; MMAL_WORKER_PORT_PARAMETER_SPACE],
}

// event messages
pub const MMAL_WORKER_EVENT_SPACE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg_event_to_host {
    pub /: *mut *mut u32 client_component; / component context,
    pub port_type: u32,
    pub port_num: u32,
    pub cmd: u32,
    pub length: u32,
    pub data: [u8; MMAL_WORKER_EVENT_SPACE],
    pub delayed_buffer: u32,
}

// all mmal messages are serialised through this structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_msg {
// header
    pub h: mmal_msg_header,
// payload
    pub version: mmal_msg_version,
    pub component_create: mmal_msg_component_create,
    pub component_create_reply: mmal_msg_component_create_reply,
    pub component_destroy: mmal_msg_component_destroy,
    pub component_destroy_reply: mmal_msg_component_destroy_reply,
    pub component_enable: mmal_msg_component_enable,
    pub component_enable_reply: mmal_msg_component_enable_reply,
    pub component_disable: mmal_msg_component_disable,
    pub component_disable_reply: mmal_msg_component_disable_reply,
    pub port_info_get: mmal_msg_port_info_get,
    pub port_info_get_reply: mmal_msg_port_info_get_reply,
    pub port_info_set: mmal_msg_port_info_set,
    pub port_info_set_reply: mmal_msg_port_info_set_reply,
    pub port_action_port: mmal_msg_port_action_port,
    pub port_action_handle: mmal_msg_port_action_handle,
    pub port_action_reply: mmal_msg_port_action_reply,
    pub buffer_from_host: mmal_msg_buffer_from_host,
    pub port_parameter_set: mmal_msg_port_parameter_set,
    pub event_to_host: mmal_msg_event_to_host,
    pub payload: [u8; MMAL_MSG_MAX_PAYLOAD],
    pub u: },
}
