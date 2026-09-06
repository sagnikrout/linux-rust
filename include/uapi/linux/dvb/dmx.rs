//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dvb/dmx.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// dmx.h
//
// Copyright (C) 2000 Marcus Metzler <marcus@convergence.de>
// & Ralph  Metzler <ralph@convergence.de>
// for convergence integrated media GmbH
//

pub const DMX_FILTER_SIZE: c_int = 16;
//
// enum dmx_output - Output for the demux.
//
// @DMX_OUT_DECODER:
// Streaming directly to decoder.
// @DMX_OUT_TAP:
// Output going to a memory buffer (to be retrieved via the read command).
// Delivers the stream output to the demux device on which the ioctl
// is called.
// @DMX_OUT_TS_TAP:
// Output multiplexed into a new TS (to be retrieved by reading from the
// logical DVR device). Routes output to the logical DVR device
// ``/dev/dvb/adapter?/dvr?``, which delivers a TS multiplexed from all
// filters for which @DMX_OUT_TS_TAP was specified.
// @DMX_OUT_TSDEMUX_TAP:
// Like @DMX_OUT_TS_TAP but retrieved from the DMX device.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmx_output {
    DMX_OUT_DECODER,
    DMX_OUT_TAP,
    DMX_OUT_TS_TAP,
    DMX_OUT_TSDEMUX_TAP
}

//
// enum dmx_input - Input from the demux.
//
// @DMX_IN_FRONTEND:	Input from a front-end device.
// @DMX_IN_DVR:		Input from the logical DVR device.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmx_input {
    DMX_IN_FRONTEND,
    DMX_IN_DVR
}

//
// enum dmx_ts_pes - type of the PES filter.
//
// @DMX_PES_AUDIO0:	first audio PID. Also referred as @DMX_PES_AUDIO.
// @DMX_PES_VIDEO0:	first video PID. Also referred as @DMX_PES_VIDEO.
// @DMX_PES_TELETEXT0:	first teletext PID. Also referred as @DMX_PES_TELETEXT.
// @DMX_PES_SUBTITLE0:	first subtitle PID. Also referred as @DMX_PES_SUBTITLE.
// @DMX_PES_PCR0:	first Program Clock Reference PID.
// Also referred as @DMX_PES_PCR.
//
// @DMX_PES_AUDIO1:	second audio PID.
// @DMX_PES_VIDEO1:	second video PID.
// @DMX_PES_TELETEXT1:	second teletext PID.
// @DMX_PES_SUBTITLE1:	second subtitle PID.
// @DMX_PES_PCR1:	second Program Clock Reference PID.
//
// @DMX_PES_AUDIO2:	third audio PID.
// @DMX_PES_VIDEO2:	third video PID.
// @DMX_PES_TELETEXT2:	third teletext PID.
// @DMX_PES_SUBTITLE2:	third subtitle PID.
// @DMX_PES_PCR2:	third Program Clock Reference PID.
//
// @DMX_PES_AUDIO3:	fourth audio PID.
// @DMX_PES_VIDEO3:	fourth video PID.
// @DMX_PES_TELETEXT3:	fourth teletext PID.
// @DMX_PES_SUBTITLE3:	fourth subtitle PID.
// @DMX_PES_PCR3:	fourth Program Clock Reference PID.
//
// @DMX_PES_OTHER:	any other PID.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmx_ts_pes {
    DMX_PES_AUDIO0,
    DMX_PES_VIDEO0,
    DMX_PES_TELETEXT0,
    DMX_PES_SUBTITLE0,
    DMX_PES_PCR0,

    DMX_PES_AUDIO1,
    DMX_PES_VIDEO1,
    DMX_PES_TELETEXT1,
    DMX_PES_SUBTITLE1,
    DMX_PES_PCR1,

    DMX_PES_AUDIO2,
    DMX_PES_VIDEO2,
    DMX_PES_TELETEXT2,
    DMX_PES_SUBTITLE2,
    DMX_PES_PCR2,

    DMX_PES_AUDIO3,
    DMX_PES_VIDEO3,
    DMX_PES_TELETEXT3,
    DMX_PES_SUBTITLE3,
    DMX_PES_PCR3,

    DMX_PES_OTHER
}

//
// struct dmx_filter - Specifies a section header filter.
//
// @filter: bit array with bits to be matched at the section header.
// @mask: bits that are valid at the filter bit array.
// @mode: mode of match: if bit is zero, it will match if equal (positive
// match); if bit is one, it will match if the bit is negated.
//
// Note: All arrays in this struct have a size of DMX_FILTER_SIZE (16 bytes).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_filter {
    pub filter: [__u8; DMX_FILTER_SIZE],
    pub mask: [__u8; DMX_FILTER_SIZE],
    pub mode: [__u8; DMX_FILTER_SIZE],
}

//
// struct dmx_sct_filter_params - Specifies a section filter.
//
// @pid: PID to be filtered.
// @filter: section header filter, as defined by &struct dmx_filter.
// @timeout: maximum time to filter, in milliseconds.
// @flags: extra flags for the section filter.
//
// Carries the configuration for a MPEG-TS section filter.
//
// The @flags can be:
//
// - %DMX_CHECK_CRC - only deliver sections where the CRC check succeeded;
// - %DMX_ONESHOT - disable the section filter after one section
// has been delivered;
// - %DMX_IMMEDIATE_START - Start filter immediately without requiring a
// :ref:`DMX_START`.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_sct_filter_params {
    pub pid: __u16,
    pub filter: dmx_filter,
    pub timeout: __u32,
    pub flags: __u32,
pub const DMX_CHECK_CRC: c_int = 1;
pub const DMX_ONESHOT: c_int = 2;
pub const DMX_IMMEDIATE_START: c_int = 4;
}

//
// struct dmx_pes_filter_params - Specifies Packetized Elementary Stream (PES)
// filter parameters.
//
// @pid:	PID to be filtered.
// @input:	Demux input, as specified by &enum dmx_input.
// @output:	Demux output, as specified by &enum dmx_output.
// @pes_type:	Type of the pes filter, as specified by &enum dmx_pes_type.
// @flags:	Demux PES flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_pes_filter_params {
    pub pid: __u16,
    pub input: dmx_input,
    pub output: dmx_output,
    pub pes_type: dmx_ts_pes,
    pub flags: __u32,
}

//
// struct dmx_stc - Stores System Time Counter (STC) information.
//
// @num: input data: number of the STC, from 0 to N.
// @base: output: divisor for STC to get 90 kHz clock.
// @stc: output: stc in @base * 90 kHz units.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_stc {
    pub num: c_uint,
    pub base: c_uint,
    pub stc: __u64,
}

//
// enum dmx_buffer_flags - DMX memory-mapped buffer flags
//
// @DMX_BUFFER_FLAG_HAD_CRC32_DISCARD:
// Indicates that the Kernel discarded one or more frames due to wrong
// CRC32 checksum.
// @DMX_BUFFER_FLAG_TEI:
// Indicates that the Kernel has detected a Transport Error indicator
// (TEI) on a filtered pid.
// @DMX_BUFFER_PKT_COUNTER_MISMATCH:
// Indicates that the Kernel has detected a packet counter mismatch
// on a filtered pid.
// @DMX_BUFFER_FLAG_DISCONTINUITY_DETECTED:
// Indicates that the Kernel has detected one or more frame discontinuity.
// @DMX_BUFFER_FLAG_DISCONTINUITY_INDICATOR:
// Received at least one packet with a frame discontinuity indicator.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmx_buffer_flags {
    DMX_BUFFER_FLAG_HAD_CRC32_DISCARD		= 1 << 0,
    DMX_BUFFER_FLAG_TEI				= 1 << 1,
    DMX_BUFFER_PKT_COUNTER_MISMATCH			= 1 << 2,
    DMX_BUFFER_FLAG_DISCONTINUITY_DETECTED		= 1 << 3,
    DMX_BUFFER_FLAG_DISCONTINUITY_INDICATOR		= 1 << 4,
}

//
// struct dmx_buffer - dmx buffer info
//
// @index:	id number of the buffer
// @bytesused:	number of bytes occupied by data in the buffer (payload);
// @offset:	for buffers with memory == DMX_MEMORY_MMAP;
// offset from the start of the device memory for this plane,
// (or a "cookie" that should be passed to mmap() as offset)
// @length:	size in bytes of the buffer
// @flags:	bit array of buffer flags as defined by &enum dmx_buffer_flags.
// Filled only at &DMX_DQBUF.
// @count:	monotonic counter for filled buffers. Helps to identify
// data stream loses. Filled only at &DMX_DQBUF.
//
// Contains data exchanged by application and driver using one of the streaming
// I/O methods.
//
// Please notice that, for &DMX_QBUF, only @index should be filled.
// On &DMX_DQBUF calls, all fields will be filled by the Kernel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_buffer {
    pub index: __u32,
    pub bytesused: __u32,
    pub offset: __u32,
    pub length: __u32,
    pub flags: __u32,
    pub count: __u32,
}

//
// struct dmx_requestbuffers - request dmx buffer information
//
// @count:	number of requested buffers,
// @size:	size in bytes of the requested buffer
//
// Contains data used for requesting a dmx buffer.
// All reserved fields must be set to zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_requestbuffers {
    pub count: __u32,
    pub size: __u32,
}

//
// struct dmx_exportbuffer - export of dmx buffer as DMABUF file descriptor
//
// @index:	id number of the buffer
// @flags:	flags for newly created file, currently only O_CLOEXEC is
// supported, refer to manual of open syscall for more details
// @fd:		file descriptor associated with DMABUF (set by driver)
//
// Contains data used for exporting a dmx buffer as DMABUF file descriptor.
// The buffer is identified by a 'cookie' returned by DMX_QUERYBUF
// (identical to the cookie used to mmap() the buffer to userspace). All
// reserved fields must be set to zero. The field reserved0 is expected to
// become a structure 'type' allowing an alternative layout of the structure
// content. Therefore this field should not be used for any other extensions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmx_exportbuffer {
    pub index: __u32,
    pub flags: __u32,
    pub fd: __s32,
}

// This is needed for legacy userspace support
pub type dmx_output_t = dmx_output;
pub type dmx_input_t = dmx_input;
pub type dmx_pes_type_t = dmx_ts_pes;
pub type dmx_filter_t = dmx_filter;

