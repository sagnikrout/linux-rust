//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/ops.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// Mandatory operations are verified during probing
// init
extern "C" {
    pub fn sof_ops(_arg: sdev)->probe_early(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->probe(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->shutdown(sdev) -> return;
}
// control
//
// snd_sof_dsp_run returns the core mask of the cores that are available
// after successful fw boot
//
extern "C" {
    pub fn sof_ops(_arg: sdev)->run(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->stall(sdev, _arg: core_mask) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->reset(sdev) -> return;
}
// dsp core get/put
// if current ref_count is > 0, increment it and return
// power up the core
// increment ref_count
// and update enabled_cores_mask
// decrement ref_count and return if it is > 0
// power down the core
// and update enabled_cores_mask
// pre/post fw load
extern "C" {
    pub fn sof_ops(_arg: sdev)->pre_fw_run(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->post_fw_run(sdev) -> return;
}
// parse platform specific extended manifest
extern "C" {
    pub fn sof_ops(_arg: sdev)->parse_platform_ext_manifest(sdev, _arg: hdr) -> return;
}
// misc
//
// snd_sof_dsp_get_bar_index - Maps a section type with a BAR index
//
// @sdev: sof device
// @type: section type as described by snd_sof_fw_blk_type
//
// Returns the corresponding BAR index (a positive integer) or -EINVAL
// in case there is no mapping
//
extern "C" {
    pub fn sof_ops(_arg: sdev)->get_bar_index(sdev, _arg: type) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->get_mailbox_offset(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->get_window_offset(sdev, _arg: id) -> return;
}
// power management
extern "C" {
    pub fn sof_ops(_arg: sdev)->resume(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->suspend(sdev, _arg: target_state) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->runtime_resume(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->runtime_suspend(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->runtime_idle(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->set_hw_params_upon_resume(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->set_clk(sdev, _arg: freq) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->set_power_state(sdev, _arg: target_state) -> return;
}
// debug
extern "C" {
    pub fn snd_sof_dsp_dbg_dump(sdev: *mut snd_sof_dev, msg: *const c_char, flags: u32);
}
// register IO
extern "C" {
    pub fn sof_ops(_arg: sdev)->read8(sdev, offset: sdev->bar[bar] +) -> return;
}
extern "C" {
    pub fn readb(offset: sdev->bar[bar] +) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->read(sdev, offset: sdev->bar[bar] +) -> return;
}
extern "C" {
    pub fn readl(offset: sdev->bar[bar] +) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->read64(sdev, offset: sdev->bar[bar] +) -> return;
}
extern "C" {
    pub fn readq(offset: sdev->bar[bar] +) -> return;
}
// block IO
extern "C" {
    pub fn sof_ops(_arg: sdev)->block_read(sdev, _arg: blk_type, _arg: offset, _arg: dest, _arg: bytes) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->block_write(sdev, _arg: blk_type, _arg: offset, _arg: src, _arg: bytes) -> return;
}
// mailbox IO
// ipc
extern "C" {
    pub fn sof_ops(_arg: sdev)->send_msg(sdev, _arg: msg) -> return;
}
// host PCM ops
extern "C" {
    pub fn sof_ops(_arg: sdev)->pcm_open(sdev, _arg: substream) -> return;
}
// disconnect pcm substream to a host stream
extern "C" {
    pub fn sof_ops(_arg: sdev)->pcm_close(sdev, _arg: substream) -> return;
}
// host stream hw params
// host stream hw free
extern "C" {
    pub fn sof_ops(_arg: sdev)->pcm_hw_free(sdev, _arg: substream) -> return;
}
// host stream trigger
extern "C" {
    pub fn sof_ops(_arg: sdev)->pcm_trigger(sdev, _arg: substream, _arg: cmd) -> return;
}
// Firmware loading
extern "C" {
    pub fn sof_ops(_arg: sdev)->load_firmware(sdev) -> return;
}
// host DSP message data
extern "C" {
    pub fn sof_ops(_arg: sdev)->ipc_msg_data(sdev, _arg: sps, _arg: p, _arg: sz) -> return;
}
// host side configuration of the stream's data offset in stream mailbox area
// host stream pointer
extern "C" {
    pub fn sof_ops(_arg: sdev)->pcm_pointer(sdev, _arg: substream) -> return;
}
// pcm ack
extern "C" {
    pub fn sof_ops(_arg: sdev)->pcm_ack(sdev, _arg: substream) -> return;
}
// machine driver
extern "C" {
    pub fn sof_ops(_arg: sdev)->machine_register(sdev, _arg: pdata) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->machine_select(sdev) -> return;
}
extern "C" {
    pub fn sof_ops(_arg: sdev)->is_chain_dma_supported(sdev, _arg: dai_type) -> return;
}
//
// snd_sof_dsp_register_poll_timeout - Periodically poll an address
// until a condition is met or a timeout occurs
// @op: accessor function (takes @addr as its only argument)
// @addr: Address to poll
// @val: Variable to read the value into
// @cond: Break condition (usually involving @val)
// @sleep_us: Maximum time to sleep between reads in us (0 tight-loops). Please
// read usleep_range() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout. In either
// case, the last read value at @addr is stored in @val. Must not
// be called from atomic context if sleep_us or timeout_us are used.
//
// This is modelled after the readx_poll_timeout macros in linux/iopoll.h.
//

// This is for registers bits with attribute RWC
extern "C" {
    pub fn snd_sof_dsp_panic(sdev: *mut snd_sof_dev, offset: u32, non_recoverable: bool);
}
