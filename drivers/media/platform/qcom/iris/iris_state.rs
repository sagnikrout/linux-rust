//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_state.h
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
//
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//
// enum iris_core_state
//
// @IRIS_CORE_DEINIT: default state.
// @IRIS_CORE_INIT:   core state with core initialized. FW loaded and
// HW brought out of reset, shared queues established
// between host driver and firmware.
// @IRIS_CORE_ERROR:  error state.
//
// -----------
// |
// V
// -----------
// +--->| DEINIT  |<---+
// |   -----------    |
// |         |        |
// |         v        |
// |   -----------    |
// |     /     \      |
// |    /       \     |
// |   /         \    |
// |  v           v   v
// -----------    -----------
// |  INIT  |--->|  ERROR  |
// -----------    -----------
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iris_core_state {
    IRIS_CORE_DEINIT,
    IRIS_CORE_INIT,
    IRIS_CORE_ERROR,
}

//
// enum iris_inst_state
//
// @IRIS_INST_INIT: video instance is opened.
// @IRIS_INST_INPUT_STREAMING: stream on is completed on output plane.
// @IRIS_INST_OUTPUT_STREAMING: stream on is completed on capture plane.
// @IRIS_INST_STREAMING: stream on is completed on both output and capture planes.
// @IRIS_INST_DEINIT: video instance is closed.
// @IRIS_INST_ERROR: error state.
// |
// V
// -------------
// +--------|     INIT    |----------+
// |         -------------           |
// |            ^   ^                |
// |           /      \              |
// |          /        \             |
// |         v          v            |
// |   -----------    -----------    |
// |   |   INPUT         OUTPUT  |   |
// |---| STREAMING     STREAMING |---|
// |   -----------    -----------    |
// |       ^            ^            |
// |         \          /            |
// |          \        /             |
// |           v      v              |
// |         -------------           |
// |--------|  STREAMING |-----------|
// |        -------------            |
// |               |                 |
// |               v                 |
// |          -----------            |
// +-------->|  DEINIT   |<----------+
// |          -----------            |
// |               |                 |
// |               v                 |
// |          ----------             |
// +-------->|   ERROR |<------------+
// ----------
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iris_inst_state {
    IRIS_INST_DEINIT,
    IRIS_INST_INIT,
    IRIS_INST_INPUT_STREAMING,
    IRIS_INST_OUTPUT_STREAMING,
    IRIS_INST_STREAMING,
    IRIS_INST_ERROR,
}

pub const IRIS_INST_SUB_STATES: c_int = 8;

//
// enum iris_inst_sub_state
//
// @IRIS_INST_SUB_FIRST_IPSC: indicates source change is received from firmware
// when output port is not yet streaming.
// @IRIS_INST_SUB_DRC: indicates source change is received from firmware
// when output port is streaming and source change event is
// sent to client.
// @IRIS_INST_SUB_DRC_LAST: indicates last buffer is received from firmware
// as part of source change.
// @IRIS_INST_SUB_DRAIN: indicates drain is in progress.
// @IRIS_INST_SUB_DRAIN_LAST: indicates last buffer is received from firmware
// as part of drain sequence.
// @IRIS_INST_SUB_INPUT_PAUSE: source change is received form firmware. This
// indicates that firmware is paused to process
// any further input frames.
// @IRIS_INST_SUB_OUTPUT_PAUSE: last buffer is received form firmware as part
// of drc sequence. This indicates that
// firmware is paused to process any further output frames.
// @IRIS_INST_SUB_LOAD_RESOURCES: indicates all the resources have been loaded by the
// firmware and it is ready for processing.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iris_inst_sub_state {
    IRIS_INST_SUB_FIRST_IPSC	= BIT(0),
    IRIS_INST_SUB_DRC		= BIT(1),
    IRIS_INST_SUB_DRC_LAST		= BIT(2),
    IRIS_INST_SUB_DRAIN		= BIT(3),
    IRIS_INST_SUB_DRAIN_LAST	= BIT(4),
    IRIS_INST_SUB_INPUT_PAUSE	= BIT(5),
    IRIS_INST_SUB_OUTPUT_PAUSE	= BIT(6),
    IRIS_INST_SUB_LOAD_RESOURCES	= BIT(7),
}

extern "C" {
    pub fn iris_inst_state_change_streamon(inst: *mut iris_inst, plane: u32) -> c_int;
}
extern "C" {
    pub fn iris_inst_state_change_streamoff(inst: *mut iris_inst, plane: u32) -> c_int;
}
extern "C" {
    pub fn iris_inst_sub_state_change_drc(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_inst_sub_state_change_drain_last(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_inst_sub_state_change_drc_last(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_inst_sub_state_change_pause(inst: *mut iris_inst, plane: u32) -> c_int;
}
extern "C" {
    pub fn iris_allow_cmd(inst: *mut iris_inst, cmd: u32) -> bool;
}
extern "C" {
    pub fn iris_drc_pending(inst: *mut iris_inst) -> bool;
}
extern "C" {
    pub fn iris_drain_pending(inst: *mut iris_inst) -> bool;
}
