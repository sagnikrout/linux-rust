//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hdaudio_ext.h
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

extern "C" {
    pub fn snd_hdac_ext_bus_exit(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_ext_bus_device_remove(bus: *mut hdac_bus);
}

extern "C" {
    pub fn snd_hdac_ext_bus_ppcap_enable(chip: *mut hdac_bus, enable: bool);
}
extern "C" {
    pub fn snd_hdac_ext_bus_ppcap_int_enable(chip: *mut hdac_bus, enable: bool);
}
extern "C" {
    pub fn snd_hdac_ext_bus_get_ml_capabilities(bus: *mut hdac_bus) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdac_ext_stream_type {
    HDAC_EXT_STREAM_TYPE_COUPLED = 0,
    HDAC_EXT_STREAM_TYPE_HOST,
    HDAC_EXT_STREAM_TYPE_LINK
}

//
// hdac_ext_stream: HDAC extended stream for extended HDA caps
//
// @hstream: hdac_stream
// @pphc_addr: processing pipe host stream pointer
// @pplc_addr: processing pipe link stream pointer
// @decoupled: stream host and link is decoupled
// @link_locked: link is locked
// @link_prepared: link is prepared
// @link_substream: link substream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_ext_stream {
    pub hstream: hdac_stream,
    pub pphc_addr: *mut void __iomem,
    pub pplc_addr: *mut void __iomem,
    pub pphcllpl: u32,
    pub pphcllpu: u32,
    pub pphcldpl: u32,
    pub pphcldpu: u32,
    pub pplcllpl: u32,
    pub pplcllpu: u32,
    pub decoupled:1: bool,
    pub link_locked:1: bool,
    pub link_prepared: bool,
    pub bool): *mut *mut *mut int (host_setup)(struct hdac_stream ,,
    pub link_substream: *mut snd_pcm_substream,
}

extern "C" {
    pub fn snd_hdac_ext_stream_free_all(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_ext_link_free_all(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_ext_stream_release(hext_stream: *mut hdac_ext_stream, type: c_int);
}
extern "C" {
    pub fn snd_hdac_ext_stream_start(hext_stream: *mut hdac_ext_stream);
}
extern "C" {
    pub fn snd_hdac_ext_stream_clear(hext_stream: *mut hdac_ext_stream);
}
extern "C" {
    pub fn snd_hdac_ext_stream_reset(hext_stream: *mut hdac_ext_stream);
}
extern "C" {
    pub fn snd_hdac_ext_stream_setup(hext_stream: *mut hdac_ext_stream, fmt: c_int) -> c_int;
}
extern "C" {
    pub fn snd_hdac_ext_host_stream_setup(hext_stream: *mut hdac_ext_stream, code_loading: bool) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_ext_link {
    pub bus: *mut hdac_bus,
    pub index: c_int,
    pub /: *mut *mut *mut void __iomem ml_addr; / link output stream reg pointer,
    pub /: *mut *mut u32 lcaps; / link capablities,
    pub /: *mut *mut u16 lsdiid; / link sdi identifier,
    pub id: u32,
    pub slcount: u8,
    pub ref_count: c_int,
    pub list: list_head,
}

extern "C" {
    pub fn snd_hdac_ext_bus_link_power_up(hlink: *mut hdac_ext_link) -> c_int;
}
extern "C" {
    pub fn snd_hdac_ext_bus_link_power_down(hlink: *mut hdac_ext_link) -> c_int;
}
extern "C" {
    pub fn snd_hdac_ext_bus_link_power_up_all(bus: *mut hdac_bus) -> c_int;
}
extern "C" {
    pub fn snd_hdac_ext_bus_link_power_down_all(bus: *mut hdac_bus) -> c_int;
}
extern "C" {
    pub fn snd_hdac_ext_bus_link_get(bus: *mut hdac_bus, hlink: *mut hdac_ext_link) -> c_int;
}
extern "C" {
    pub fn snd_hdac_ext_bus_link_put(bus: *mut hdac_bus, hlink: *mut hdac_ext_link) -> c_int;
}
extern "C" {
    pub fn snd_hdac_ext_bus_link_power(codec: *mut hdac_device, enable: bool);
}
// ops common to all codec drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_ext_codec_ops {
    pub dev): *mut *mut int (build_controls)(struct hdac_ext_device,
    pub dev): *mut *mut int (init)(struct hdac_ext_device,
    pub dev): *mut *mut void (free)(struct hdac_ext_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_dai_map {
    pub dai_name: *mut c_char,
    pub nid: hda_nid_t,
    pub maxbps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_ext_dma_params {
    pub format: u32,
    pub stream_tag: u8,
}

extern "C" {
    pub fn snd_hda_ext_driver_register(drv: *mut hdac_driver) -> c_int;
}
extern "C" {
    pub fn snd_hda_ext_driver_unregister(drv: *mut hdac_driver);
}
