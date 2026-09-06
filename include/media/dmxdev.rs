//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/dmxdev.h
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
// dmxdev.h
//
// Copyright (C) 2000 Ralph Metzler & Marcus Metzler
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
// enum dmxdev_type - type of demux filter type.
//
// @DMXDEV_TYPE_NONE:	no filter set.
// @DMXDEV_TYPE_SEC:	section filter.
// @DMXDEV_TYPE_PES:	Program Elementary Stream (PES) filter.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmxdev_type {
    DMXDEV_TYPE_NONE,
    DMXDEV_TYPE_SEC,
    DMXDEV_TYPE_PES,
}

//
// enum dmxdev_state - state machine for the dmxdev.
//
// @DMXDEV_STATE_FREE:		indicates that the filter is freed.
// @DMXDEV_STATE_ALLOCATED:	indicates that the filter was allocated
// to be used.
// @DMXDEV_STATE_SET:		indicates that the filter parameters are set.
// @DMXDEV_STATE_GO:		indicates that the filter is running.
// @DMXDEV_STATE_DONE:		indicates that a packet was already filtered
// and the filter is now disabled.
// Set only if %DMX_ONESHOT. See
// &dmx_sct_filter_params.
// @DMXDEV_STATE_TIMEDOUT:	Indicates a timeout condition.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmxdev_state {
    DMXDEV_STATE_FREE,
    DMXDEV_STATE_ALLOCATED,
    DMXDEV_STATE_SET,
    DMXDEV_STATE_GO,
    DMXDEV_STATE_DONE,
    DMXDEV_STATE_TIMEDOUT
}

//
// struct dmxdev_feed - digital TV dmxdev feed
//
// @pid:	Program ID to be filtered
// @ts:		pointer to &struct dmx_ts_feed
// @next:	&struct list_head pointing to the next feed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmxdev_feed {
    pub pid: u16,
    pub ts: *mut dmx_ts_feed,
    pub next: list_head,
}

//
// struct dmxdev_filter - digital TV dmxdev filter
//
// @filter:	a union describing a dmxdev filter.
// Currently used only for section filters.
// @filter.sec: a &struct dmx_section_filter pointer.
// For section filter only.
// @feed:	a union describing a dmxdev feed.
// Depending on the filter type, it can be either
// @feed.ts or @feed.sec.
// @feed.ts:	a &struct list_head list.
// For TS and PES feeds.
// @feed.sec:	a &struct dmx_section_feed pointer.
// For section feed only.
// @params:	a union describing dmxdev filter parameters.
// Depending on the filter type, it can be either
// @params.sec or @params.pes.
// @params.sec:	a &struct dmx_sct_filter_params embedded struct.
// For section filter only.
// @params.pes:	a &struct dmx_pes_filter_params embedded struct.
// For PES filter only.
// @type:	type of the dmxdev filter, as defined by &enum dmxdev_type.
// @state:	state of the dmxdev filter, as defined by &enum dmxdev_state.
// @dev:	pointer to &struct dmxdev.
// @buffer:	an embedded &struct dvb_ringbuffer buffer.
// @vb2_ctx:	control struct for VB2 handler
// @mutex:	protects the access to &struct dmxdev_filter.
// @timer:	&struct timer_list embedded timer, used to check for
// feed timeouts.
// Only for section filter.
// @todo:	index for the @secheader.
// Only for section filter.
// @secheader:	buffer cache to parse the section header.
// Only for section filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmxdev_filter {
    pub sec: *mut dmx_section_filter,
    pub filter: },
// list of TS and PES feeds (struct dmxdev_feed)
    pub ts: list_head,
    pub sec: *mut dmx_section_feed,
    pub feed: },
    pub sec: dmx_sct_filter_params,
    pub pes: dmx_pes_filter_params,
    pub params: },
    pub type: dmxdev_type,
    pub state: dmxdev_state,
    pub dev: *mut dmxdev,
    pub buffer: dvb_ringbuffer,
    pub vb2_ctx: dvb_vb2_ctx,
    pub mutex: mutex,
// only for sections
    pub timer: timer_list,
    pub todo: c_int,
    pub secheader: [u8; 3],
}

//
// struct dmxdev - Describes a digital TV demux device.
//
// @dvbdev:		pointer to &struct dvb_device associated with
// the demux device node.
// @dvr_dvbdev:		pointer to &struct dvb_device associated with
// the dvr device node.
// @filter:		pointer to &struct dmxdev_filter.
// @demux:		pointer to &struct dmx_demux.
// @filternum:		number of filters.
// @capabilities:	demux capabilities as defined by &enum dmx_demux_caps.
// @may_do_mmap:	flag used to indicate if the device may do mmap.
// @exit:		flag to indicate that the demux is being released.
// @dvr_orig_fe:	pointer to &struct dmx_frontend.
// @dvr_buffer:		embedded &struct dvb_ringbuffer for DVB output.
// @dvr_vb2_ctx:	control struct for VB2 handler
// @mutex:		protects the usage of this structure.
// @lock:		protects access to &dmxdev->filter->data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmxdev {
    pub dvbdev: *mut dvb_device,
    pub dvr_dvbdev: *mut dvb_device,
    pub filter: *mut dmxdev_filter,
    pub demux: *mut dmx_demux,
    pub filternum: c_int,
    pub capabilities: c_int,
    pub may_do_mmap:1: c_uint,
    pub exit:1: c_uint,
pub const DMXDEV_CAP_DUPLEX: c_int = 1;
    pub dvr_orig_fe: *mut dmx_frontend,
    pub dvr_buffer: dvb_ringbuffer,

    pub dvr_vb2_ctx: dvb_vb2_ctx,
    pub mutex: mutex,
    pub lock: spinlock_t,
}

//
// dvb_dmxdev_init - initializes a digital TV demux and registers both demux
// and DVR devices.
//
// @dmxdev: pointer to &struct dmxdev.
// @adap: pointer to &struct dvb_adapter.
//
extern "C" {
    pub fn dvb_dmxdev_init(dmxdev: *mut dmxdev, adap: *mut dvb_adapter) -> c_int;
}
//
// dvb_dmxdev_release - releases a digital TV demux and unregisters it.
//
// @dmxdev: pointer to &struct dmxdev.
//
extern "C" {
    pub fn dvb_dmxdev_release(dmxdev: *mut dmxdev);
}
