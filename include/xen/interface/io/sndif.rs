//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/sndif.h
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


// SPDX-License-Identifier: MIT
//
// sndif.h
//
// Unified sound-device I/O interface for Xen guest OSes.
//
// Copyright (C) 2013-2015 GlobalLogic Inc.
// Copyright (C) 2016-2017 EPAM Systems Inc.
//
// Authors: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
// Oleksandr Grytsov <oleksandr_grytsov@epam.com>
// Oleksandr Dmytryshyn <oleksandr.dmytryshyn@globallogic.com>
// Iurii Konovalenko <iurii.konovalenko@globallogic.com>
//

//
// Protocol version
//
pub const XENSND_PROTOCOL_VERSION: c_int = 2;
//
// Feature and Parameter Negotiation
//
// Front->back notifications: when enqueuing a new request, sending a
// notification can be made conditional on xensnd_req (i.e., the generic
// hold-off mechanism provided by the ring macros). Backends must set
// xensnd_req appropriately (e.g., using RING_FINAL_CHECK_FOR_REQUESTS()).
//
// Back->front notifications: when enqueuing a new response, sending a
// notification can be made conditional on xensnd_resp (i.e., the generic
// hold-off mechanism provided by the ring macros). Frontends must set
// xensnd_resp appropriately (e.g., using RING_FINAL_CHECK_FOR_RESPONSES()).
//
// The two halves of a para-virtual sound card driver utilize nodes within
// XenStore to communicate capabilities and to negotiate operating parameters.
// This section enumerates these nodes which reside in the respective front and
// backend portions of XenStore, following the XenBus convention.
//
// All data in XenStore is stored as strings. Nodes specifying numeric
// values are encoded in decimal. Integer value ranges listed below are
// expressed as fixed sized integer types capable of storing the conversion
// of a properly formated node string, without loss of information.
//
// Example configuration
//
// Note: depending on the use-case backend can expose more sound cards and
// PCM devices/streams than the underlying HW physically has by employing
// SW mixers, configuring virtual sound streams, channels etc.
//
// This is an example of backend and frontend configuration:
//
// --------------------------------- Backend -----------------------------------
//
// /local/domain/0/backend/vsnd/1/0/frontend-id = "1"
// /local/domain/0/backend/vsnd/1/0/frontend = "/local/domain/1/device/vsnd/0"
// /local/domain/0/backend/vsnd/1/0/state = "4"
// /local/domain/0/backend/vsnd/1/0/versions = "1,2"
//
// --------------------------------- Frontend ----------------------------------
//
// /local/domain/1/device/vsnd/0/backend-id = "0"
// /local/domain/1/device/vsnd/0/backend = "/local/domain/0/backend/vsnd/1/0"
// /local/domain/1/device/vsnd/0/state = "4"
// /local/domain/1/device/vsnd/0/version = "1"
//
// ----------------------------- Card configuration ----------------------------
//
// /local/domain/1/device/vsnd/0/short-name = "Card short name"
// /local/domain/1/device/vsnd/0/long-name = "Card long name"
// /local/domain/1/device/vsnd/0/sample-rates = "8000,32000,44100,48000,96000"
// /local/domain/1/device/vsnd/0/sample-formats = "s8,u8,s16_le,s16_be"
// /local/domain/1/device/vsnd/0/buffer-size = "262144"
//
// ------------------------------- PCM device 0 --------------------------------
//
// /local/domain/1/device/vsnd/0/0/name = "General analog"
// /local/domain/1/device/vsnd/0/0/channels-max = "5"
//
// ----------------------------- Stream 0, playback ----------------------------
//
// /local/domain/1/device/vsnd/0/0/0/type = "p"
// /local/domain/1/device/vsnd/0/0/0/sample-formats = "s8,u8"
// /local/domain/1/device/vsnd/0/0/0/unique-id = "0"
//
// /local/domain/1/device/vsnd/0/0/0/ring-ref = "386"
// /local/domain/1/device/vsnd/0/0/0/event-channel = "15"
// /local/domain/1/device/vsnd/0/0/0/evt-ring-ref = "1386"
// /local/domain/1/device/vsnd/0/0/0/evt-event-channel = "215"
//
// ------------------------------ Stream 1, capture ----------------------------
//
// /local/domain/1/device/vsnd/0/0/1/type = "c"
// /local/domain/1/device/vsnd/0/0/1/channels-max = "2"
// /local/domain/1/device/vsnd/0/0/1/unique-id = "1"
//
// /local/domain/1/device/vsnd/0/0/1/ring-ref = "384"
// /local/domain/1/device/vsnd/0/0/1/event-channel = "13"
// /local/domain/1/device/vsnd/0/0/1/evt-ring-ref = "1384"
// /local/domain/1/device/vsnd/0/0/1/evt-event-channel = "213"
//
// ------------------------------- PCM device 1 --------------------------------
//
// /local/domain/1/device/vsnd/0/1/name = "HDMI-0"
// /local/domain/1/device/vsnd/0/1/sample-rates = "8000,32000,44100"
//
// ------------------------------ Stream 0, capture ----------------------------
//
// /local/domain/1/device/vsnd/0/1/0/type = "c"
// /local/domain/1/device/vsnd/0/1/0/unique-id = "2"
//
// /local/domain/1/device/vsnd/0/1/0/ring-ref = "387"
// /local/domain/1/device/vsnd/0/1/0/event-channel = "151"
// /local/domain/1/device/vsnd/0/1/0/evt-ring-ref = "1387"
// /local/domain/1/device/vsnd/0/1/0/evt-event-channel = "351"
//
// ------------------------------- PCM device 2 --------------------------------
//
// /local/domain/1/device/vsnd/0/2/name = "SPDIF"
//
// ----------------------------- Stream 0, playback ----------------------------
//
// /local/domain/1/device/vsnd/0/2/0/type = "p"
// /local/domain/1/device/vsnd/0/2/0/unique-id = "3"
//
// /local/domain/1/device/vsnd/0/2/0/ring-ref = "389"
// /local/domain/1/device/vsnd/0/2/0/event-channel = "152"
// /local/domain/1/device/vsnd/0/2/0/evt-ring-ref = "1389"
// /local/domain/1/device/vsnd/0/2/0/evt-event-channel = "452"
//
// Backend XenBus Nodes
//
// ----------------------------- Protocol version ------------------------------
//
// versions
// Values:         <string>
//
// List of XENSND_LIST_SEPARATOR separated protocol versions supported
// by the backend. For example "1,2,3".
//
// Frontend XenBus Nodes
//
// -------------------------------- Addressing ---------------------------------
//
// dom-id
// Values:         <uint16_t>
//
// Domain identifier.
//
// dev-id
// Values:         <uint16_t>
//
// Device identifier.
//
// pcm-dev-idx
// Values:         <uint8_t>
//
// Zero based contigous index of the PCM device.
//
// stream-idx
// Values:         <uint8_t>
//
// Zero based contigous index of the stream of the PCM device.
//
// The following pattern is used for addressing:
// /local/domain/<dom-id>/device/vsnd/<dev-id>/<pcm-dev-idx>/<stream-idx>/...
//
// ----------------------------- Protocol version ------------------------------
//
// version
// Values:         <string>
//
// Protocol version, chosen among the ones supported by the backend.
//
// ------------------------------- PCM settings --------------------------------
//
// Every virtualized sound frontend has a set of PCM devices and streams, each
// could be individually configured. Part of the PCM configuration can be
// defined at higher level of the hierarchy and be fully or partially re-used
// by the underlying layers. These configuration values are:
// o number of channels (min/max)
// o supported sample rates
// o supported sample formats.
// E.g. one can define these values for the whole card, device or stream.
// Every underlying layer in turn can re-define some or all of them to better
// fit its needs. For example, card may define number of channels to be
// in [1; 8] range, and some particular stream may be limited to [1; 2] only.
// The rule is that the underlying layer must be a subset of the upper layer
// range.
//
// channels-min
// Values:         <uint8_t>
//
// The minimum amount of channels that is supported, [1; channels-max].
// Optional, if not set or omitted a value of 1 is used.
//
// channels-max
// Values:         <uint8_t>
//
// The maximum amount of channels that is supported.
// Must be at least <channels-min>.
//
// sample-rates
// Values:         <list of uint32_t>
//
// List of supported sample rates separated by XENSND_LIST_SEPARATOR.
// Sample rates are expressed as a list of decimal values w/o any
// ordering requirement.
//
// sample-formats
// Values:         <list of XENSND_PCM_FORMAT_XXX_STR>
//
// List of supported sample formats separated by XENSND_LIST_SEPARATOR.
// Items must not exceed XENSND_SAMPLE_FORMAT_MAX_LEN length.
//
// buffer-size
// Values:         <uint32_t>
//
// The maximum size in octets of the buffer to allocate per stream.
//
// ----------------------- Virtual sound card settings -------------------------
// short-name
// Values:         <char[32]>
//
// Short name of the virtual sound card. Optional.
//
// long-name
// Values:         <char[80]>
//
// Long name of the virtual sound card. Optional.
//
// ----------------------------- Device settings -------------------------------
// name
// Values:         <char[80]>
//
// Name of the sound device within the virtual sound card. Optional.
//
// ----------------------------- Stream settings -------------------------------
//
// type
// Values:         "p", "c"
//
// Stream type: "p" - playback stream, "c" - capture stream
//
// If both capture and playback are needed then two streams need to be
// defined under the same device.
//
// unique-id
// Values:         <string>
//
// After stream initialization it is assigned a unique ID, so every
// stream of the frontend can be identified by the backend by this ID.
// This can be UUID or such.
//
// -------------------- Stream Request Transport Parameters --------------------
//
// event-channel
// Values:         <uint32_t>
//
// The identifier of the Xen event channel used to signal activity
// in the ring buffer.
//
// ring-ref
// Values:         <uint32_t>
//
// The Xen grant reference granting permission for the backend to map
// a sole page in a single page sized ring buffer.
//
// --------------------- Stream Event Transport Parameters ---------------------
//
// This communication path is used to deliver asynchronous events from backend
// to frontend, set up per stream.
//
// evt-event-channel
// Values:         <uint32_t>
//
// The identifier of the Xen event channel used to signal activity
// in the ring buffer.
//
// evt-ring-ref
// Values:         <uint32_t>
//
// The Xen grant reference granting permission for the backend to map
// a sole page in a single page sized ring buffer.
//
// STATE DIAGRAMS
//
// Tool stack creates front and back state nodes with initial state
// XenbusStateInitialising.
// Tool stack creates and sets up frontend sound configuration nodes per domain.
//
// Front                                Back
// =================================    =====================================
// XenbusStateInitialising              XenbusStateInitialising
// o Query backend device identification
// data.
// o Open and validate backend device.
// |
// V
// XenbusStateInitWait
//
// o Query frontend configuration
// o Allocate and initialize
// event channels per configured
// playback/capture stream.
// o Publish transport parameters
// that will be in effect during
// this connection.
// |
// V
// XenbusStateInitialised
//
// o Query frontend transport parameters.
// o Connect to the event channels.
// |
// V
// XenbusStateConnected
//
// o Create and initialize OS
// virtual sound device instances
// as per configuration.
// |
// V
// XenbusStateConnected
//
// XenbusStateUnknown
// XenbusStateClosed
// XenbusStateClosing
// o Remove virtual sound device
// o Remove event channels
// |
// V
// XenbusStateClosed
//
// ------------------------------- Recovery flow -------------------------------
//
// In case of frontend unrecoverable errors backend handles that as
// if frontend goes into the XenbusStateClosed state.
//
// In case of backend unrecoverable errors frontend tries removing
// the virtualized device. If this is possible at the moment of error,
// then frontend goes into the XenbusStateInitialising state and is ready for
// new connection with backend. If the virtualized device is still in use and
// cannot be removed, then frontend goes into the XenbusStateReconfiguring state
// until either the virtualized device removed or backend initiates a new
// connection. On the virtualized device removal frontend goes into the
// XenbusStateInitialising state.
//
// Note on XenbusStateReconfiguring state of the frontend: if backend has
// unrecoverable errors then frontend cannot send requests to the backend
// and thus cannot provide functionality of the virtualized device anymore.
// After backend is back to normal the virtualized device may still hold some
// state: configuration in use, allocated buffers, client application state etc.
// So, in most cases, this will require frontend to implement complex recovery
// reconnect logic. Instead, by going into XenbusStateReconfiguring state,
// frontend will make sure no new clients of the virtualized device are
// accepted, allow existing client(s) to exit gracefully by signaling error
// state etc.
// Once all the clients are gone frontend can reinitialize the virtualized
// device and get into XenbusStateInitialising state again signaling the
// backend that a new connection can be made.
//
// There are multiple conditions possible under which frontend will go from
// XenbusStateReconfiguring into XenbusStateInitialising, some of them are OS
// specific. For example:
// 1. The underlying OS framework may provide callbacks to signal that the last
// client of the virtualized device has gone and the device can be removed
// 2. Frontend can schedule a deferred work (timer/tasklet/workqueue)
// to periodically check if this is the right time to re-try removal of
// the virtualized device.
// 3. By any other means.
//
// PCM FORMATS
//
// XENSND_PCM_FORMAT_<format>[_<endian>]
//
// format: <S/U/F><bits> or <name>
// S - signed, U - unsigned, F - float
// bits - 8, 16, 24, 32
// name - MU_LAW, GSM, etc.
//
// endian: <LE/BE>, may be absent
// LE - Little endian, BE - Big endian
//
pub const XENSND_PCM_FORMAT_S8: c_int = 0;
pub const XENSND_PCM_FORMAT_U8: c_int = 1;
pub const XENSND_PCM_FORMAT_S16_LE: c_int = 2;
pub const XENSND_PCM_FORMAT_S16_BE: c_int = 3;
pub const XENSND_PCM_FORMAT_U16_LE: c_int = 4;
pub const XENSND_PCM_FORMAT_U16_BE: c_int = 5;
pub const XENSND_PCM_FORMAT_S24_LE: c_int = 6;
pub const XENSND_PCM_FORMAT_S24_BE: c_int = 7;
pub const XENSND_PCM_FORMAT_U24_LE: c_int = 8;
pub const XENSND_PCM_FORMAT_U24_BE: c_int = 9;
pub const XENSND_PCM_FORMAT_S32_LE: c_int = 10;
pub const XENSND_PCM_FORMAT_S32_BE: c_int = 11;
pub const XENSND_PCM_FORMAT_U32_LE: c_int = 12;
pub const XENSND_PCM_FORMAT_U32_BE: c_int = 13;

pub const XENSND_PCM_FORMAT_IEC958_SUBFRAME_LE: c_int = 18;
pub const XENSND_PCM_FORMAT_IEC958_SUBFRAME_BE: c_int = 19;
pub const XENSND_PCM_FORMAT_MU_LAW: c_int = 20;
pub const XENSND_PCM_FORMAT_A_LAW: c_int = 21;
pub const XENSND_PCM_FORMAT_IMA_ADPCM: c_int = 22;
pub const XENSND_PCM_FORMAT_MPEG: c_int = 23;
pub const XENSND_PCM_FORMAT_GSM: c_int = 24;
//
// REQUEST CODES
//
pub const XENSND_OP_OPEN: c_int = 0;
pub const XENSND_OP_CLOSE: c_int = 1;
pub const XENSND_OP_READ: c_int = 2;
pub const XENSND_OP_WRITE: c_int = 3;
pub const XENSND_OP_SET_VOLUME: c_int = 4;
pub const XENSND_OP_GET_VOLUME: c_int = 5;
pub const XENSND_OP_MUTE: c_int = 6;
pub const XENSND_OP_UNMUTE: c_int = 7;
pub const XENSND_OP_TRIGGER: c_int = 8;
pub const XENSND_OP_HW_PARAM_QUERY: c_int = 9;
pub const XENSND_OP_TRIGGER_START: c_int = 0;
pub const XENSND_OP_TRIGGER_PAUSE: c_int = 1;
pub const XENSND_OP_TRIGGER_STOP: c_int = 2;
pub const XENSND_OP_TRIGGER_RESUME: c_int = 3;
//
// EVENT CODES
//
pub const XENSND_EVT_CUR_POS: c_int = 0;
//
// XENSTORE FIELD AND PATH NAME STRINGS, HELPERS
//

// Field names

// Stream type field values.

// Sample rate max string length
pub const XENSND_SAMPLE_RATE_MAX_LEN: c_int = 11;
// Sample format field values
pub const XENSND_SAMPLE_FORMAT_MAX_LEN: c_int = 24;

//
// STATUS RETURN CODES
//
// Status return code is zero on success and -XEN_EXX on failure.
//
// Assumptions
//
// o usage of grant reference 0 as invalid grant reference:
// grant reference 0 is valid, but never exposed to a PV driver,
// because of the fact it is already in use/reserved by the PV console.
// o all references in this document to page sizes must be treated
// as pages of size XEN_PAGE_SIZE unless otherwise noted.
//
// Description of the protocol between frontend and backend driver
//
// The two halves of a Para-virtual sound driver communicate with
// each other using shared pages and event channels.
// Shared page contains a ring with request/response packets.
//
// Packets, used for input/output operations, e.g. read/write, set/get volume,
// etc., provide offset/length fields in order to allow asynchronous protocol
// operation with buffer space sharing: part of the buffer allocated at
// XENSND_OP_OPEN can be used for audio samples and part, for example,
// for volume control.
//
// All reserved fields in the structures below must be 0.
//
// ---------------------------------- Requests ---------------------------------
//
// All request packets have the same length (64 octets)
// All request packets have common header:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |    operation   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// id - uint16_t, private guest value, echoed in response
// operation - uint8_t, operation code, XENSND_OP_???
//
// For all packets which use offset and length:
// offset - uint32_t, read or write data offset within the shared buffer,
// passed with XENSND_OP_OPEN request, octets,
// [0; XENSND_OP_OPEN.buffer_sz - 1].
// length - uint32_t, read or write data length, octets
//
// Request open - open a PCM stream for playback or capture:
//
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                | XENSND_OP_OPEN |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |                             pcm_rate                              | 12
// +----------------+----------------+----------------+----------------+
// |  pcm_format    |  pcm_channels  |             reserved            | 16
// +----------------+----------------+----------------+----------------+
// |                             buffer_sz                             | 20
// +----------------+----------------+----------------+----------------+
// |                           gref_directory                          | 24
// +----------------+----------------+----------------+----------------+
// |                             period_sz                             | 28
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 32
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 64
// +----------------+----------------+----------------+----------------+
//
// pcm_rate - uint32_t, stream data rate, Hz
// pcm_format - uint8_t, XENSND_PCM_FORMAT_XXX value
// pcm_channels - uint8_t, number of channels of this stream,
// [channels-min; channels-max]
// buffer_sz - uint32_t, buffer size to be allocated, octets
// period_sz - uint32_t, event period size, octets
// This is the requested value of the period at which frontend would
// like to receive XENSND_EVT_CUR_POS notifications from the backend when
// stream position advances during playback/capture.
// It shows how many octets are expected to be played/captured before
// sending such an event.
// If set to 0 no XENSND_EVT_CUR_POS events are sent by the backend.
//
// gref_directory - grant_ref_t, a reference to the first shared page
// describing shared buffer references. At least one page exists. If shared
// buffer size  (buffer_sz) exceeds what can be addressed by this single page,
// then reference to the next page must be supplied (see gref_dir_next_page
// below)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_open_req {
    pub pcm_rate: u32,
    pub pcm_format: u8,
    pub pcm_channels: u8,
    pub reserved: u16,
    pub buffer_sz: u32,
    pub gref_directory: grant_ref_t,
    pub period_sz: u32,
}

//
// Shared page for XENSND_OP_OPEN buffer descriptor (gref_directory in the
// request) employs a list of pages, describing all pages of the shared data
// buffer:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |                        gref_dir_next_page                         | 4
// +----------------+----------------+----------------+----------------+
// |                              gref[0]                              | 8
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                              gref[i]                              | i*4+8
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             gref[N - 1]                           | N*4+8
// +----------------+----------------+----------------+----------------+
//
// gref_dir_next_page - grant_ref_t, reference to the next page describing
// page directory. Must be 0 if there are no more pages in the list.
// gref[i] - grant_ref_t, reference to a shared page of the buffer
// allocated at XENSND_OP_OPEN
//
// Number of grant_ref_t entries in the whole page directory is not
// passed, but instead can be calculated as:
// num_grefs_total = (XENSND_OP_OPEN.buffer_sz + XEN_PAGE_SIZE - 1)
// XEN_PAGE_SIZE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_page_directory {
    pub gref_dir_next_page: grant_ref_t,
    pub gref: [grant_ref_t; ],
}

//
// Request close - close an opened pcm stream:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                | XENSND_OP_CLOSE|    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 64
// +----------------+----------------+----------------+----------------+
//
// Request read/write - used for read (for capture) or write (for playback):
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |   operation    |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |                              offset                               | 12
// +----------------+----------------+----------------+----------------+
// |                              length                               | 16
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 20
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 64
// +----------------+----------------+----------------+----------------+
//
// operation - XENSND_OP_READ for read or XENSND_OP_WRITE for write
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_rw_req {
    pub offset: u32,
    pub length: u32,
}

//
// Request set/get volume - set/get channels' volume of the stream given:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |   operation    |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |                              offset                               | 12
// +----------------+----------------+----------------+----------------+
// |                              length                               | 16
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 20
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 64
// +----------------+----------------+----------------+----------------+
//
// operation - XENSND_OP_SET_VOLUME for volume set
// or XENSND_OP_GET_VOLUME for volume get
// Buffer passed with XENSND_OP_OPEN is used to exchange volume
// values:
//
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |                             channel[0]                            | 4
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             channel[i]                            | i*4
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                           channel[N - 1]                          | (N-1)*4
// +----------------+----------------+----------------+----------------+
//
// N = XENSND_OP_OPEN.pcm_channels
// i - uint8_t, index of a channel
// channel[i] - sint32_t, volume of i-th channel
// Volume is expressed as a signed value in steps of 0.001 dB,
// while 0 being 0 dB.
//
// Request mute/unmute - mute/unmute stream:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |   operation    |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |                              offset                               | 12
// +----------------+----------------+----------------+----------------+
// |                              length                               | 16
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 20
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 64
// +----------------+----------------+----------------+----------------+
//
// operation - XENSND_OP_MUTE for mute or XENSND_OP_UNMUTE for unmute
// Buffer passed with XENSND_OP_OPEN is used to exchange mute/unmute
// values:
//
// 0                                 octet
// +----------------+----------------+----------------+----------------+
// |                             channel[0]                            | 4
// +----------------+----------------+----------------+----------------+
// +/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             channel[i]                            | i*4
// +----------------+----------------+----------------+----------------+
// +/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                           channel[N - 1]                          | (N-1)*4
// +----------------+----------------+----------------+----------------+
//
// N = XENSND_OP_OPEN.pcm_channels
// i - uint8_t, index of a channel
// channel[i] - uint8_t, non-zero if i-th channel needs to be muted/unmuted
//
// ------------------------------------ N.B. -----------------------------------
//
// The 'struct xensnd_rw_req' is also used for XENSND_OP_SET_VOLUME,
// XENSND_OP_GET_VOLUME, XENSND_OP_MUTE, XENSND_OP_UNMUTE.
//
// Request stream running state change - trigger PCM stream running state
// to start, stop, pause or resume:
//
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |   _OP_TRIGGER  |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |      type      |                     reserved                     | 12
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 16
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 64
// +----------------+----------------+----------------+----------------+
//
// type - uint8_t, XENSND_OP_TRIGGER_XXX value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_trigger_req {
    pub type: u8,
}

//
// Request stream parameter ranges: request intervals and
// masks of supported ranges for stream configuration values.
//
// Sound device configuration for a particular stream is a limited subset
// of the multidimensional configuration available on XenStore, e.g.
// once the frame rate has been selected there is a limited supported range
// for sample rates becomes available (which might be the same set configured
// on XenStore or less). For example, selecting 96kHz sample rate may limit
// number of channels available for such configuration from 4 to 2, etc.
// Thus, each call to XENSND_OP_HW_PARAM_QUERY may reduce configuration
// space making it possible to iteratively get the final stream configuration,
// used in XENSND_OP_OPEN request.
//
// See response format for this request.
//
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                | _HW_PARAM_QUERY|    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |                     formats mask low 32-bit                       | 12
// +----------------+----------------+----------------+----------------+
// |                     formats mask high 32-bit                      | 16
// +----------------+----------------+----------------+----------------+
// |                              min rate                             | 20
// +----------------+----------------+----------------+----------------+
// |                              max rate                             | 24
// +----------------+----------------+----------------+----------------+
// |                            min channels                           | 28
// +----------------+----------------+----------------+----------------+
// |                            max channels                           | 32
// +----------------+----------------+----------------+----------------+
// |                         min buffer frames                         | 36
// +----------------+----------------+----------------+----------------+
// |                         max buffer frames                         | 40
// +----------------+----------------+----------------+----------------+
// |                         min period frames                         | 44
// +----------------+----------------+----------------+----------------+
// |                         max period frames                         | 48
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 52
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 64
// +----------------+----------------+----------------+----------------+
//
// formats - uint64_t, bit mask representing values of the parameter
// made as bitwise OR of (1 << XENSND_PCM_FORMAT_XXX) values
//
// For interval parameters:
// min - uint32_t, minimum value of the parameter
// max - uint32_t, maximum value of the parameter
//
// Frame is defined as a product of the number of channels by the
// number of octets per one sample.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_query_hw_param {
    pub formats: u64,
    pub min: u32,
    pub max: u32,
    pub rates: },
    pub min: u32,
    pub max: u32,
    pub channels: },
    pub min: u32,
    pub max: u32,
    pub buffer: },
    pub min: u32,
    pub max: u32,
    pub period: },
}

//
// ---------------------------------- Responses --------------------------------
//
// All response packets have the same length (64 octets)
//
// All response packets have common header:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |    operation   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                              status                               | 8
// +----------------+----------------+----------------+----------------+
//
// id - uint16_t, copied from the request
// operation - uint8_t, XENSND_OP_* - copied from request
// status - int32_t, response status, zero on success and -XEN_EXX on failure
//
// HW parameter query response - response for XENSND_OP_HW_PARAM_QUERY:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |    operation   |    reserved    | 4
// +----------------+----------------+----------------+----------------+
// |                              status                               | 8
// +----------------+----------------+----------------+----------------+
// |                     formats mask low 32-bit                       | 12
// +----------------+----------------+----------------+----------------+
// |                     formats mask high 32-bit                      | 16
// +----------------+----------------+----------------+----------------+
// |                              min rate                             | 20
// +----------------+----------------+----------------+----------------+
// |                              max rate                             | 24
// +----------------+----------------+----------------+----------------+
// |                            min channels                           | 28
// +----------------+----------------+----------------+----------------+
// |                            max channels                           | 32
// +----------------+----------------+----------------+----------------+
// |                         min buffer frames                         | 36
// +----------------+----------------+----------------+----------------+
// |                         max buffer frames                         | 40
// +----------------+----------------+----------------+----------------+
// |                         min period frames                         | 44
// +----------------+----------------+----------------+----------------+
// |                         max period frames                         | 48
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 52
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 64
// +----------------+----------------+----------------+----------------+
//
// Meaning of the values in this response is the same as for
// XENSND_OP_HW_PARAM_QUERY request.
//
// ----------------------------------- Events ----------------------------------
//
// Events are sent via shared page allocated by the front and propagated by
// evt-event-channel/evt-ring-ref XenStore entries
// All event packets have the same length (64 octets)
// All event packets have common header:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |      type      |   reserved     | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
//
// id - uint16_t, event id, may be used by front
// type - uint8_t, type of the event
//
// Current stream position - event from back to front when stream's
// playback/capture position has advanced:
// 0                1                 2               3        octet
// +----------------+----------------+----------------+----------------+
// |               id                |   _EVT_CUR_POS |   reserved     | 4
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 8
// +----------------+----------------+----------------+----------------+
// |                         position low 32-bit                       | 12
// +----------------+----------------+----------------+----------------+
// |                         position high 32-bit                      | 16
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 20
// +----------------+----------------+----------------+----------------+
// |/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/\/|
// +----------------+----------------+----------------+----------------+
// |                             reserved                              | 64
// +----------------+----------------+----------------+----------------+
//
// position - current value of stream's playback/capture position, octets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_cur_pos_evt {
    pub position: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_req {
    pub id: u16,
    pub operation: u8,
    pub reserved: [u8; 5],
    pub open: xensnd_open_req,
    pub rw: xensnd_rw_req,
    pub trigger: xensnd_trigger_req,
    pub hw_param: xensnd_query_hw_param,
    pub reserved: [u8; 56],
    pub op: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_resp {
    pub id: u16,
    pub operation: u8,
    pub reserved: u8,
    pub status: i32,
    pub hw_param: xensnd_query_hw_param,
    pub reserved1: [u8; 56],
    pub resp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_evt {
    pub id: u16,
    pub type: u8,
    pub reserved: [u8; 5],
    pub cur_pos: xensnd_cur_pos_evt,
    pub reserved: [u8; 56],
    pub op: },
}

//
// Back to front events delivery
//
// In order to deliver asynchronous events from back to front a shared page is
// allocated by front and its granted reference propagated to back via
// XenStore entries (evt-ring-ref/evt-event-channel).
// This page has a common header used by both front and back to synchronize
// access and control event's ring buffer, while back being a producer of the
// events and front being a consumer. The rest of the page after the header
// is used for event packets.
//
// Upon reception of an event(s) front may confirm its reception
// for either each event, group of events or none.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xensnd_event_page {
    pub in_cons: u32,
    pub in_prod: u32,
    pub reserved: [u8; 56],
}

