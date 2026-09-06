//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/cs-etm-decoder/cs-etm-decoder.h
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
// SPDX-License-Identifier: GPL-2.0
//
// Copyright(C) 2015-2018 Linaro Limited.
//
// Author: Tor Jeremiassen <tor@ti.com>
// Author: Mathieu Poirier <mathieu.poirier@linaro.org>
//

// Macro flag: #define INCLUDE__CS_ETM_DECODER_H__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etmv3_trace_params {
    pub reg_ctrl: u32,
    pub reg_trc_id: u32,
    pub reg_ccer: u32,
    pub reg_idr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etmv4_trace_params {
    pub reg_idr0: u32,
    pub reg_idr1: u32,
    pub reg_idr2: u32,
    pub reg_idr8: u32,
    pub reg_configr: u32,
    pub reg_traceidr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_ete_trace_params {
    pub reg_idr0: u32,
    pub reg_idr1: u32,
    pub reg_idr2: u32,
    pub reg_idr8: u32,
    pub reg_configr: u32,
    pub reg_traceidr: u32,
    pub reg_devarch: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etm_trace_params {
    pub protocol: c_int,
    pub etmv3: cs_etmv3_trace_params,
    pub etmv4: cs_etmv4_trace_params,
    pub ete: cs_ete_trace_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_etm_decoder_params {
    pub operation: c_int,
    pub data): *const *const *const void (packet_printer)(char msg, void,
    pub mem_acc_cb: cs_etm_mem_cb_type,
    pub formatted: bool,
    pub fsyncs: bool,
    pub hsyncs: bool,
    pub frame_aligned: bool,
    pub data: *mut c_void,
}

//
// The following enums are indexed starting with 1 to align with the
// open source coresight trace decoder library.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs_etm_decoder_operation {
    CS_ETM_OPERATION_PRINT = 1,
    CS_ETM_OPERATION_DECODE,
    CS_ETM_OPERATION_MAX,
}

extern "C" {
    pub fn cs_etm_decoder__free(decoder: *mut cs_etm_decoder);
}
extern "C" {
    pub fn cs_etm_decoder__reset(decoder: *mut cs_etm_decoder) -> c_int;
}
