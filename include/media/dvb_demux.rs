//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/dvb_demux.h
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
// dvb_demux.h: DVB kernel demux API
//
// Copyright (C) 2000-2001 Marcus Metzler & Ralph Metzler
// for convergence integrated media GmbH
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
// enum dvb_dmx_filter_type - type of demux feed.
//
// @DMX_TYPE_TS:	feed is in TS mode.
// @DMX_TYPE_SEC:	feed is in Section mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dvb_dmx_filter_type {
    DMX_TYPE_TS,
    DMX_TYPE_SEC,
}

//
// enum dvb_dmx_state - state machine for a demux filter.
//
// @DMX_STATE_FREE:		indicates that the filter is freed.
// @DMX_STATE_ALLOCATED:	indicates that the filter was allocated
// to be used.
// @DMX_STATE_READY:		indicates that the filter is ready
// to be used.
// @DMX_STATE_GO:		indicates that the filter is running.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dvb_dmx_state {
    DMX_STATE_FREE,
    DMX_STATE_ALLOCATED,
    DMX_STATE_READY,
    DMX_STATE_GO,
}

pub const DVB_DEMUX_MASK_MAX: c_int = 18;
pub const MAX_PID: c_uint = 0x1fff;
pub const SPEED_PKTS_INTERVAL: c_int = 50000;
//
// struct dvb_demux_filter - Describes a DVB demux section filter.
//
// @filter:		Section filter as defined by &struct dmx_section_filter.
// @maskandmode:	logical ``and`` bit mask.
// @maskandnotmode:	logical ``and not`` bit mask.
// @doneq:		flag that indicates when a filter is ready.
// @next:		pointer to the next section filter.
// @feed:		&struct dvb_demux_feed pointer.
// @index:		index of the used demux filter.
// @state:		state of the filter as described by &enum dvb_dmx_state.
// @type:		type of the filter as described
// by &enum dvb_dmx_filter_type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_demux_filter {
    pub filter: dmx_section_filter,
    pub maskandmode: [u8; DMX_MAX_FILTER_SIZE],
    pub maskandnotmode: [u8; DMX_MAX_FILTER_SIZE],
    pub doneq: bool,
    pub next: *mut dvb_demux_filter,
    pub feed: *mut dvb_demux_feed,
    pub index: c_int,
    pub state: dvb_dmx_state,
    pub type: dvb_dmx_filter_type,
// private: used only by av7110
    pub hw_handle: u16,
}

//
// struct dvb_demux_feed - describes a DVB field
//
// @feed:	a union describing a digital TV feed.
// Depending on the feed type, it can be either
// @feed.ts or @feed.sec.
// @feed.ts:	a &struct dmx_ts_feed pointer.
// For TS feed only.
// @feed.sec:	a &struct dmx_section_feed pointer.
// For section feed only.
// @cb:		a union describing digital TV callbacks.
// Depending on the feed type, it can be either
// @cb.ts or @cb.sec.
// @cb.ts:	a dmx_ts_cb() calback function pointer.
// For TS feed only.
// @cb.sec:	a dmx_section_cb() callback function pointer.
// For section feed only.
// @demux:	pointer to &struct dvb_demux.
// @priv:	private data that can optionally be used by a DVB driver.
// @type:	type of the filter, as defined by &enum dvb_dmx_filter_type.
// @state:	state of the filter as defined by &enum dvb_dmx_state.
// @pid:	PID to be filtered.
// @timeout:	feed timeout.
// @filter:	pointer to &struct dvb_demux_filter.
// @buffer_flags: Buffer flags used to report discontinuity users via DVB
// memory mapped API, as defined by &enum dmx_buffer_flags.
// @ts_type:	type of TS, as defined by &enum ts_filter_type.
// @pes_type:	type of PES, as defined by &enum dmx_ts_pes.
// @cc:		MPEG-TS packet continuity counter
// @pusi_seen:	if true, indicates that a discontinuity was detected.
// it is used to prevent feeding of garbage from previous section.
// @peslen:	length of the PES (Packet Elementary Stream).
// @list_head:	head for the list of digital TV demux feeds.
// @index:	a unique index for each feed. Can be used as hardware
// pid filter index.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_demux_feed {
    pub ts: dmx_ts_feed,
    pub sec: dmx_section_feed,
    pub feed: },
    pub ts: dmx_ts_cb,
    pub sec: dmx_section_cb,
    pub cb: },
    pub demux: *mut dvb_demux,
    pub priv: *mut c_void,
    pub type: dvb_dmx_filter_type,
    pub state: dvb_dmx_state,
    pub pid: u16,
    pub timeout: ktime_t,
    pub filter: *mut dvb_demux_filter,
    pub buffer_flags: u32,
    pub ts_type: ts_filter_type,
    pub pes_type: dmx_ts_pes,
    pub cc: c_int,
    pub pusi_seen: bool,
    pub peslen: u16,
    pub list_head: list_head,
    pub index: c_uint,
}

//
// struct dvb_demux - represents a digital TV demux
// @dmx:		embedded &struct dmx_demux with demux capabilities
// and callbacks.
// @priv:		private data that can optionally be used by
// a DVB driver.
// @filternum:		maximum amount of DVB filters.
// @feednum:		maximum amount of DVB feeds.
// @start_feed:		callback routine to be called in order to start
// a DVB feed.
// @stop_feed:		callback routine to be called in order to stop
// a DVB feed.
// @write_to_decoder:	callback routine to be called if the feed is TS and
// it is routed to an A/V decoder, when a new TS packet
// is received.
// Used only on av7110-av.c.
// @check_crc32:	callback routine to check CRC. If not initialized,
// dvb_demux will use an internal one.
// @memcopy:		callback routine to memcopy received data.
// If not initialized, dvb_demux will default to memcpy().
// @users:		counter for the number of demux opened file descriptors.
// Currently, it is limited to 10 users.
// @filter:		pointer to &struct dvb_demux_filter.
// @feed:		pointer to &struct dvb_demux_feed.
// @frontend_list:	&struct list_head with frontends used by the demux.
// @pesfilter:		array of &struct dvb_demux_feed with the PES types
// that will be filtered.
// @pids:		list of filtered program IDs.
// @feed_list:		&struct list_head with feeds.
// @tsbuf:		temporary buffer used internally to store TS packets.
// @tsbufp:		temporary buffer index used internally.
// @mutex:		pointer to &struct mutex used to protect feed set
// logic.
// @lock:		pointer to &spinlock_t, used to protect buffer handling.
// @cnt_storage:	buffer used for TS/TEI continuity check.
// @speed_last_time:	&ktime_t used for TS speed check.
// @speed_pkts_cnt:	packets count used for TS speed check.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_demux {
    pub dmx: dmx_demux,
    pub priv: *mut c_void,
    pub filternum: c_int,
    pub feednum: c_int,
    pub feed): *mut *mut int (start_feed)(struct dvb_demux_feed,
    pub feed): *mut *mut int (stop_feed)(struct dvb_demux_feed,
    pub len): *const *const u8 buf, size_t,
    pub len): *const *const u8 buf, size_t,
    pub len): *const *const u8 src, size_t,
    pub users: c_int,
pub const MAX_DVB_DEMUX_USERS: c_int = 10;
    pub filter: *mut dvb_demux_filter,
    pub feed: *mut dvb_demux_feed,
    pub frontend_list: list_head,
    pub pesfilter: [*mut dvb_demux_feed; DMX_PES_OTHER],
    pub pids: [u16; DMX_PES_OTHER],
pub const DMX_MAX_PID: c_uint = 0x2000;
    pub feed_list: list_head,
    pub tsbuf: [u8; 204],
    pub tsbufp: c_int,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub /: *mut *mut *mut uint8_t cnt_storage; / for TS continuity check,
    pub /: *mut *mut ktime_t speed_last_time; / for TS speed check,
    pub /: *mut *mut uint32_t speed_pkts_cnt; / for TS speed check,
// private: used only on av7110
    pub playing: c_int,
    pub recording: c_int,
}

//
// dvb_dmx_init - initialize a digital TV demux struct.
//
// @demux: &struct dvb_demux to be initialized.
//
// Before being able to register a digital TV demux struct, drivers
// should call this routine. On its typical usage, some fields should
// be initialized at the driver before calling it.
//
// A typical usecase is::
//
// dvb->demux.dmx.capabilities =
// DMX_TS_FILTERING | DMX_SECTION_FILTERING |
// DMX_MEMORY_BASED_FILTERING;
// dvb->demux.priv       = dvb;
// dvb->demux.filternum  = 256;
// dvb->demux.feednum    = 256;
// dvb->demux.start_feed = driver_start_feed;
// dvb->demux.stop_feed  = driver_stop_feed;
// ret = dvb_dmx_init(&dvb->demux);
// if (ret < 0)
// return ret;
//
extern "C" {
    pub fn dvb_dmx_init(demux: *mut dvb_demux) -> c_int;
}
//
// dvb_dmx_release - releases a digital TV demux internal buffers.
//
// @demux: &struct dvb_demux to be released.
//
// The DVB core internally allocates data at @demux. This routine
// releases those data. Please notice that the struct itelf is not
// released, as it can be embedded on other structs.
//
extern "C" {
    pub fn dvb_dmx_release(demux: *mut dvb_demux);
}
//
// dvb_dmx_swfilter_packets - use dvb software filter for a buffer with
// multiple MPEG-TS packets with 188 bytes each.
//
// @demux: pointer to &struct dvb_demux
// @buf: buffer with data to be filtered
// @count: number of MPEG-TS packets with size of 188.
//
// The routine will discard a DVB packet that don't start with 0x47.
//
// Use this routine if the DVB demux fills MPEG-TS buffers that are
// already aligned.
//
// NOTE: The @buf size should have size equal to ``count * 188``.
//
// dvb_dmx_swfilter -  use dvb software filter for a buffer with
// multiple MPEG-TS packets with 188 bytes each.
//
// @demux: pointer to &struct dvb_demux
// @buf: buffer with data to be filtered
// @count: number of MPEG-TS packets with size of 188.
//
// If a DVB packet doesn't start with 0x47, it will seek for the first
// byte that starts with 0x47.
//
// Use this routine if the DVB demux fill buffers that may not start with
// a packet start mark (0x47).
//
// NOTE: The @buf size should have size equal to ``count * 188``.
//
extern "C" {
    pub fn dvb_dmx_swfilter(demux: *mut dvb_demux, buf: *const u8, count: usize);
}
//
// dvb_dmx_swfilter_204 -  use dvb software filter for a buffer with
// multiple MPEG-TS packets with 204 bytes each.
//
// @demux: pointer to &struct dvb_demux
// @buf: buffer with data to be filtered
// @count: number of MPEG-TS packets with size of 204.
//
// If a DVB packet doesn't start with 0x47, it will seek for the first
// byte that starts with 0x47.
//
// Use this routine if the DVB demux fill buffers that may not start with
// a packet start mark (0x47).
//
// NOTE: The @buf size should have size equal to ``count * 204``.
//
// dvb_dmx_swfilter_raw -  make the raw data available to userspace without
// filtering
//
// @demux: pointer to &struct dvb_demux
// @buf: buffer with data
// @count: number of packets to be passed. The actual size of each packet
// depends on the &dvb_demux->feed->cb.ts logic.
//
// Use it if the driver needs to deliver the raw payload to userspace without
// passing through the kernel demux. That is meant to support some
// delivery systems that aren't based on MPEG-TS.
//
// This function relies on &dvb_demux->feed->cb.ts to actually handle the
// buffer.
//
