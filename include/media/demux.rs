//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/demux.h
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


//
// demux.h
//
// The Kernel Digital TV Demux kABI defines a driver-internal interface for
// registering low-level, hardware specific driver to a hardware independent
// demux layer.
//
// Copyright (c) 2002 Convergence GmbH
//
// based on code:
// Copyright (c) 2000 Nokia Research Center
// Tampere, FINLAND
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

//
// Common definitions
//
// DMX_MAX_FILTER_SIZE: Maximum length (in bytes) of a section/PES filter.
//

pub const DMX_MAX_FILTER_SIZE: c_int = 18;

//
// DMX_MAX_SECFEED_SIZE: Maximum length (in bytes) of a private section feed
// filter.
//

pub const DMX_MAX_SECTION_SIZE: c_int = 4096;

//
// TS packet reception
//
// enum ts_filter_type - filter type bitmap for dmx_ts_feed.set\(\)
//
// @TS_PACKET:		Send TS packets (188 bytes) to callback (default).
// @TS_PAYLOAD_ONLY:	In case TS_PACKET is set, only send the TS payload
// (<=184 bytes per packet) to callback
// @TS_DECODER:		Send stream to built-in decoder (if present).
// @TS_DEMUX:		In case TS_PACKET is set, send the TS to the demux
// device, not to the dvr device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ts_filter_type {
    TS_PACKET = 1,
    TS_PAYLOAD_ONLY = 2,
    TS_DECODER = 4,
    TS_DEMUX = 8,
}

//
// struct dmx_ts_feed - Structure that contains a TS feed filter
//
// @is_filtering:	Set to non-zero when filtering in progress
// @parent:		pointer to struct dmx_demux
// @priv:		pointer to private data of the API client
// @set:		sets the TS filter
// @start_filtering:	starts TS filtering
// @stop_filtering:	stops TS filtering
//
// A TS feed is typically mapped to a hardware PID filter on the demux chip.
// Using this API, the client can set the filtering properties to start/stop
// filtering TS packets on a particular TS feed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_ts_feed {
    pub is_filtering: c_int,
    pub parent: *mut dmx_demux,
    pub priv: *mut c_void,
    pub timeout): ktime_t,
    pub feed): *mut *mut int (start_filtering)(struct dmx_ts_feed,
    pub feed): *mut *mut int (stop_filtering)(struct dmx_ts_feed,
}

//
// Section reception
//
// struct dmx_section_filter - Structure that describes a section filter
//
// @filter_value: Contains up to 16 bytes (128 bits) of the TS section header
// that will be matched by the section filter
// @filter_mask:  Contains a 16 bytes (128 bits) filter mask with the bits
// specified by @filter_value that will be used on the filter
// match logic.
// @filter_mode:  Contains a 16 bytes (128 bits) filter mode.
// @parent:	  Back-pointer to struct dmx_section_feed.
// @priv:	  Pointer to private data of the API client.
//
// The @filter_mask controls which bits of @filter_value are compared with
// the section headers/payload. On a binary value of 1 in filter_mask, the
// corresponding bits are compared. The filter only accepts sections that are
// equal to filter_value in all the tested bit positions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_section_filter {
    pub filter_value: [u8; DMX_MAX_FILTER_SIZE],
    pub filter_mask: [u8; DMX_MAX_FILTER_SIZE],
    pub filter_mode: [u8; DMX_MAX_FILTER_SIZE],
    pub parent: *mut dmx_section_feed,
    pub priv: *mut c_void,
}

//
// struct dmx_section_feed - Structure that contains a section feed filter
//
// @is_filtering:	Set to non-zero when filtering in progress
// @parent:		pointer to struct dmx_demux
// @priv:		pointer to private data of the API client
// @check_crc:		If non-zero, check the CRC values of filtered sections.
// @set:		sets the section filter
// @allocate_filter:	This function is used to allocate a section filter on
// the demux. It should only be called when no filtering
// is in progress on this section feed. If a filter cannot
// be allocated, the function fails with -ENOSPC.
// @release_filter:	This function releases all the resources of a
// previously allocated section filter. The function
// should not be called while filtering is in progress
// on this section feed. After calling this function,
// the caller should not try to dereference the filter
// pointer.
// @start_filtering:	starts section filtering
// @stop_filtering:	stops section filtering
//
// A TS feed is typically mapped to a hardware PID filter on the demux chip.
// Using this API, the client can set the filtering properties to start/stop
// filtering TS packets on a particular TS feed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_section_feed {
    pub is_filtering: c_int,
    pub parent: *mut dmx_demux,
    pub priv: *mut c_void,
    pub check_crc: c_int,
// private: Used internally at dvb_demux.c
    pub crc_val: u32,
    pub secbuf: *mut u8,
    pub secbuf_base: [u8; DMX_MAX_SECFEED_SIZE],
    pub tsfeedp: u16 secbufp, seclen,,
// public:
    pub check_crc): c_int,
    pub filter): *mut dmx_section_filter,
    pub filter): *mut dmx_section_filter,
    pub feed): *mut *mut int (start_filtering)(struct dmx_section_feed,
    pub feed): *mut *mut int (stop_filtering)(struct dmx_section_feed,
}

//
// typedef dmx_ts_cb - DVB demux TS filter callback function prototype
//
// @buffer1:		Pointer to the start of the filtered TS packets.
// @buffer1_length:	Length of the TS data in buffer1.
// @buffer2:		Pointer to the tail of the filtered TS packets, or NULL.
// @buffer2_length:	Length of the TS data in buffer2.
// @source:		Indicates which TS feed is the source of the callback.
// @buffer_flags:	Address where buffer flags are stored. Those are
// used to report discontinuity users via DVB
// memory mapped API, as defined by
// &enum dmx_buffer_flags.
//
// This function callback prototype, provided by the client of the demux API,
// is called from the demux code. The function is only called when filtering
// on a TS feed has been enabled using the start_filtering\(\) function at
// the &dmx_demux.
// Any TS packets that match the filter settings are copied to a circular
// buffer. The filtered TS packets are delivered to the client using this
// callback function.
// It is expected that the @buffer1 and @buffer2 callback parameters point to
// addresses within the circular buffer, but other implementations are also
// possible. Note that the called party should not try to free the memory
// the @buffer1 and @buffer2 parameters point to.
//
// When this function is called, the @buffer1 parameter typically points to
// the start of the first undelivered TS packet within a circular buffer.
// The @buffer2 buffer parameter is normally NULL, except when the received
// TS packets have crossed the last address of the circular buffer and
// "wrapped" to the beginning of the buffer. In the latter case the @buffer1
// parameter would contain an address within the circular buffer, while the
// @buffer2 parameter would contain the first address of the circular buffer.
// The number of bytes delivered with this function (i.e. @buffer1_length +
// @buffer2_length) is usually equal to the value of callback_length parameter
// given in the set() function, with one exception: if a timeout occurs before
// receiving callback_length bytes of TS data, any undelivered packets are
// immediately delivered to the client by calling this function. The timeout
// duration is controlled by the set() function in the TS Feed API.
//
// If a TS packet is received with errors that could not be fixed by the
// TS-level forward error correction (FEC), the Transport_error_indicator
// flag of the TS packet header should be set. The TS packet should not be
// discarded, as the error can possibly be corrected by a higher layer
// protocol. If the called party is slow in processing the callback, it
// is possible that the circular buffer eventually fills up. If this happens,
// the demux driver should discard any TS packets received while the buffer
// is full and return -EOVERFLOW.
//
// The type of data returned to the callback can be selected by the
// &dmx_ts_feed.@set function. The type parameter decides if the raw
// TS packet (TS_PACKET) or just the payload (TS_PACKET|TS_PAYLOAD_ONLY)
// should be returned. If additionally the TS_DECODER bit is set the stream
// will also be sent to the hardware MPEG decoder.
//
// Return:
//
// - 0, on success;
//
// - -EOVERFLOW, on buffer overflow.
//
// typedef dmx_section_cb - DVB demux TS filter callback function prototype
//
// @buffer1:		Pointer to the start of the filtered section, e.g.
// within the circular buffer of the demux driver.
// @buffer1_len:	Length of the filtered section data in @buffer1,
// including headers and CRC.
// @buffer2:		Pointer to the tail of the filtered section data,
// or NULL. Useful to handle the wrapping of a
// circular buffer.
// @buffer2_len:	Length of the filtered section data in @buffer2,
// including headers and CRC.
// @source:		Indicates which section feed is the source of the
// callback.
// @buffer_flags:	Address where buffer flags are stored. Those are
// used to report discontinuity users via DVB
// memory mapped API, as defined by
// &enum dmx_buffer_flags.
//
// This function callback prototype, provided by the client of the demux API,
// is called from the demux code. The function is only called when
// filtering of sections has been enabled using the function
// &dmx_ts_feed.@start_filtering. When the demux driver has received a
// complete section that matches at least one section filter, the client
// is notified via this callback function. Normally this function is called
// for each received section; however, it is also possible to deliver
// multiple sections with one callback, for example when the system load
// is high. If an error occurs while receiving a section, this
// function should be called with the corresponding error type set in the
// success field, whether or not there is data to deliver. The Section Feed
// implementation should maintain a circular buffer for received sections.
// However, this is not necessary if the Section Feed API is implemented as
// a client of the TS Feed API, because the TS Feed implementation then
// buffers the received data. The size of the circular buffer can be
// configured using the &dmx_ts_feed.@set function in the Section Feed API.
// If there is no room in the circular buffer when a new section is received,
// the section must be discarded. If this happens, the value of the success
// parameter should be DMX_OVERRUN_ERROR on the next callback.
//
// DVB Front-End
//
// enum dmx_frontend_source - Used to identify the type of frontend
//
// @DMX_MEMORY_FE:	The source of the demux is memory. It means that
// the MPEG-TS to be filtered comes from userspace,
// via write() syscall.
//
// @DMX_FRONTEND_0:	The source of the demux is a frontend connected
// to the demux.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmx_frontend_source {
    DMX_MEMORY_FE,
    DMX_FRONTEND_0,
}

//
// struct dmx_frontend - Structure that lists the frontends associated with
// a demux
//
// @connectivity_list:	List of front-ends that can be connected to a
// particular demux;
// @source:		Type of the frontend.
//
// FIXME: this structure should likely be replaced soon by some
// media-controller based logic.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_frontend {
    pub connectivity_list: list_head,
    pub source: dmx_frontend_source,
}

//
// MPEG-2 TS Demux
//
// enum dmx_demux_caps - MPEG-2 TS Demux capabilities bitmap
//
// @DMX_TS_FILTERING:		set if TS filtering is supported;
// @DMX_SECTION_FILTERING:	set if section filtering is supported;
// @DMX_MEMORY_BASED_FILTERING:	set if write() available.
//
// Those flags are OR'ed in the &dmx_demux.capabilities field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmx_demux_caps {
    DMX_TS_FILTERING = 1,
    DMX_SECTION_FILTERING = 4,
    DMX_MEMORY_BASED_FILTERING = 8,
}

//
// Demux resource type identifier.
//
// DMX_FE_ENTRY - Casts elements in the list of registered
// front-ends from the generic type struct list_head
// to the type * struct dmx_frontend
//
// @list: list of struct dmx_frontend
//

//
// struct dmx_demux - Structure that contains the demux capabilities and
// callbacks.
//
// @capabilities: Bitfield of capability flags.
//
// @frontend: Front-end connected to the demux
//
// @priv: Pointer to private data of the API client
//
// @open: This function reserves the demux for use by the caller and, if
// necessary, initializes the demux. When the demux is no longer needed,
// the function @close should be called. It should be possible for
// multiple clients to access the demux at the same time. Thus, the
// function implementation should increment the demux usage count when
// @open is called and decrement it when @close is called.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// It returns:
// 0 on success;
// -EUSERS, if maximum usage count was reached;
// -EINVAL, on bad parameter.
//
// @close: This function reserves the demux for use by the caller and, if
// necessary, initializes the demux. When the demux is no longer needed,
// the function @close should be called. It should be possible for
// multiple clients to access the demux at the same time. Thus, the
// function implementation should increment the demux usage count when
// @open is called and decrement it when @close is called.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// It returns:
// 0 on success;
// -ENODEV, if demux was not in use (e. g. no users);
// -EINVAL, on bad parameter.
//
// @write: This function provides the demux driver with a memory buffer
// containing TS packets. Instead of receiving TS packets from the DVB
// front-end, the demux driver software will read packets from memory.
// Any clients of this demux with active TS, PES or Section filters will
// receive filtered data via the Demux callback API (see 0). The function
// returns when all the data in the buffer has been consumed by the demux.
// Demux hardware typically cannot read TS from memory. If this is the
// case, memory-based filtering has to be implemented entirely in software.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// The @buf function parameter contains a pointer to the TS data in
// kernel-space memory.
// The @count function parameter contains the length of the TS data.
// It returns:
// 0 on success;
// -ERESTARTSYS, if mutex lock was interrupted;
// -EINTR, if a signal handling is pending;
// -ENODEV, if demux was removed;
// -EINVAL, on bad parameter.
//
// @allocate_ts_feed: Allocates a new TS feed, which is used to filter the TS
// packets carrying a certain PID. The TS feed normally corresponds to a
// hardware PID filter on the demux chip.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// The @feed function parameter contains a pointer to the TS feed API and
// instance data.
// The @callback function parameter contains a pointer to the callback
// function for passing received TS packet.
// It returns:
// 0 on success;
// -ERESTARTSYS, if mutex lock was interrupted;
// -EBUSY, if no more TS feeds is available;
// -EINVAL, on bad parameter.
//
// @release_ts_feed: Releases the resources allocated with @allocate_ts_feed.
// Any filtering in progress on the TS feed should be stopped before
// calling this function.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// The @feed function parameter contains a pointer to the TS feed API and
// instance data.
// It returns:
// 0 on success;
// -EINVAL on bad parameter.
//
// @allocate_section_feed: Allocates a new section feed, i.e. a demux resource
// for filtering and receiving sections. On platforms with hardware
// support for section filtering, a section feed is directly mapped to
// the demux HW. On other platforms, TS packets are first PID filtered in
// hardware and a hardware section filter then emulated in software. The
// caller obtains an API pointer of type dmx_section_feed_t as an out
// parameter. Using this API the caller can set filtering parameters and
// start receiving sections.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// The @feed function parameter contains a pointer to the TS feed API and
// instance data.
// The @callback function parameter contains a pointer to the callback
// function for passing received TS packet.
// It returns:
// 0 on success;
// -EBUSY, if no more TS feeds is available;
// -EINVAL, on bad parameter.
//
// @release_section_feed: Releases the resources allocated with
// @allocate_section_feed, including allocated filters. Any filtering in
// progress on the section feed should be stopped before calling this
// function.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// The @feed function parameter contains a pointer to the TS feed API and
// instance data.
// It returns:
// 0 on success;
// -EINVAL, on bad parameter.
//
// @add_frontend: Registers a connectivity between a demux and a front-end,
// i.e., indicates that the demux can be connected via a call to
// @connect_frontend to use the given front-end as a TS source. The
// client of this function has to allocate dynamic or static memory for
// the frontend structure and initialize its fields before calling this
// function. This function is normally called during the driver
// initialization. The caller must not free the memory of the frontend
// struct before successfully calling @remove_frontend.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// The @frontend function parameter contains a pointer to the front-end
// instance data.
// It returns:
// 0 on success;
// -EINVAL, on bad parameter.
//
// @remove_frontend: Indicates that the given front-end, registered by a call
// to @add_frontend, can no longer be connected as a TS source by this
// demux. The function should be called when a front-end driver or a demux
// driver is removed from the system. If the front-end is in use, the
// function fails with the return value of -EBUSY. After successfully
// calling this function, the caller can free the memory of the frontend
// struct if it was dynamically allocated before the @add_frontend
// operation.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// The @frontend function parameter contains a pointer to the front-end
// instance data.
// It returns:
// 0 on success;
// -ENODEV, if the front-end was not found,
// -EINVAL, on bad parameter.
//
// @get_frontends: Provides the APIs of the front-ends that have been
// registered for this demux. Any of the front-ends obtained with this
// call can be used as a parameter for @connect_frontend. The include
// file demux.h contains the macro DMX_FE_ENTRY() for converting an
// element of the generic type struct &list_head * to the type
// struct &dmx_frontend *. The caller must not free the memory of any of
// the elements obtained via this function call.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// It returns a struct list_head pointer to the list of front-end
// interfaces, or NULL in the case of an empty list.
//
// @connect_frontend: Connects the TS output of the front-end to the input of
// the demux. A demux can only be connected to a front-end registered to
// the demux with the function @add_frontend. It may or may not be
// possible to connect multiple demuxes to the same front-end, depending
// on the capabilities of the HW platform. When not used, the front-end
// should be released by calling @disconnect_frontend.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// The @frontend function parameter contains a pointer to the front-end
// instance data.
// It returns:
// 0 on success;
// -EINVAL, on bad parameter.
//
// @disconnect_frontend: Disconnects the demux and a front-end previously
// connected by a @connect_frontend call.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// It returns:
// 0 on success;
// -EINVAL on bad parameter.
//
// @get_pes_pids: Get the PIDs for DMX_PES_AUDIO0, DMX_PES_VIDEO0,
// DMX_PES_TELETEXT0, DMX_PES_SUBTITLE0 and DMX_PES_PCR0.
// The @demux function parameter contains a pointer to the demux API and
// instance data.
// The @pids function parameter contains an array with five u16 elements
// where the PIDs will be stored.
// It returns:
// 0 on success;
// -EINVAL on bad parameter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_demux {
    pub capabilities: dmx_demux_caps,
    pub frontend: *mut dmx_frontend,
    pub priv: *mut c_void,
    pub demux): *mut *mut int (open)(struct dmx_demux,
    pub demux): *mut *mut int (close)(struct dmx_demux,
    pub count): usize,
    pub callback): dmx_ts_cb,
    pub feed): *mut dmx_ts_feed,
    pub callback): dmx_section_cb,
    pub feed): *mut dmx_section_feed,
    pub frontend): *mut dmx_frontend,
    pub frontend): *mut dmx_frontend,
    pub demux): *mut *mut *mut list_head (get_frontends)(dmx_demux,
    pub frontend): *mut dmx_frontend,
    pub demux): *mut *mut int (disconnect_frontend)(struct dmx_demux,
    pub pids): *mut *mut *mut int (get_pes_pids)(struct dmx_demux demux, u16,
// private:
//
// Only used at av7110, to read some data from firmware.
// As this was never documented, we have no clue about what's
// there, and its usage on other drivers aren't encouraged.
//
    pub base): *mut *mut u64 stc, unsigned int,
}
