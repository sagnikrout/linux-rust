//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/dmaengine_pcm.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2012, Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

//
// snd_pcm_substream_to_dma_direction - Get dma_transfer_direction for a PCM
// substream
// @substream: PCM substream
//
// Return: DMA transfer direction
//
extern "C" {
    pub fn snd_dmaengine_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn snd_dmaengine_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
}
extern "C" {
    pub fn snd_dmaengine_pcm_pointer_no_residue(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
}
extern "C" {
    pub fn snd_dmaengine_pcm_close(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_dmaengine_pcm_sync_stop(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_dmaengine_pcm_close_release_chan(substream: *mut snd_pcm_substream) -> c_int;
}
//
// The DAI supports packed transfers, eg 2 16-bit samples in a 32-bit word.
// If this flag is set the dmaengine driver won't put any restriction on
// the supported sample formats and set the DMA transfer size to undefined.
// The DAI driver is responsible to disable any unsupported formats in it's
// configuration and catch corner cases that are not already handled in
// the ALSA core.
//

//
// struct snd_dmaengine_dai_dma_data - DAI DMA configuration data
// @addr: Address of the DAI data source or destination register.
// @addr_width: Width of the DAI data source or destination register.
// @maxburst: Maximum number of words(note: words, as in units of the
// src_addr_width member, not bytes) that can be send to or received from the
// DAI in one burst.
// @filter_data: Custom DMA channel filter data, this will usually be used when
// requesting the DMA channel.
// @chan_name: Custom channel name to use when requesting DMA channel.
// @fifo_size: FIFO size of the DAI controller in bytes
// @flags: PCM_DAI flags, only SND_DMAENGINE_PCM_DAI_FLAG_PACK for now
// @peripheral_config: peripheral configuration for programming peripheral
// for dmaengine transfer
// @peripheral_size: peripheral configuration buffer size
// @port_window_size: The length of the register area in words the data need
// to be accessed on the device side. It is only used for devices which is using
// an area instead of a single register to send/receive the data. Typically the
// DMA loops in this area in order to transfer the data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: dma_addr_t,
    pub addr_width: dma_slave_buswidth,
    pub maxburst: u32,
    pub filter_data: *mut c_void,
    pub chan_name: *const c_char,
    pub fifo_size: c_uint,
    pub flags: c_uint,
    pub peripheral_config: *mut c_void,
    pub peripheral_size: usize,
    pub port_window_size: u32,
}

//
// Try to request the DMA channel using compat_request_channel or
// compat_filter_fn if it couldn't be requested through devicetree.
//

//
// Don't try to request the DMA channels through devicetree. This flag only
// makes sense if SND_DMAENGINE_PCM_FLAG_COMPAT is set as well.
//

//
// The PCM is half duplex and the DMA channel is shared between capture and
// playback.
//

//
// struct snd_dmaengine_pcm_config - Configuration data for dmaengine based PCM
// @prepare_slave_config: Callback used to fill in the DMA slave_config for a
// PCM substream. Will be called from the PCM drivers hwparams callback.
// @compat_request_channel: Callback to request a DMA channel for platforms
// which do not use devicetree.
// @process: Callback used to apply processing on samples transferred from/to
// user space.
// @name: Component name. If null, dev_name will be used.
// @compat_filter_fn: Will be used as the filter function when requesting a
// channel for platforms which do not use devicetree. The filter parameter
// will be the DAI's DMA data.
// @dma_dev: If set, request DMA channel on this device rather than the DAI
// device.
// @chan_names: If set, these custom DMA channel names will be requested at
// registration time.
// @pcm_hardware: snd_pcm_hardware struct to be used for the PCM.
// @prealloc_buffer_size: Size of the preallocated audio buffer.
//
// Note: If both compat_request_channel and compat_filter_fn are set
// compat_request_channel will be used to request the channel and
// compat_filter_fn will be ignored. Otherwise the channel will be requested
// using dma_request_channel with compat_filter_fn as the filter function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dmaengine_pcm_config {
    pub slave_config): *mut dma_slave_config,
    pub substream): *mut snd_pcm_substream,
    pub bytes): c_ulong,
    pub name: *const c_char,
    pub compat_filter_fn: dma_filter_fn,
    pub dma_dev: *mut device,
    pub 1]: *const *const char chan_names[SNDRV_PCM_STREAM_LAST +,
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prealloc_buffer_size: c_uint,
}

extern "C" {
    pub fn snd_dmaengine_pcm_unregister(dev: *mut device);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmaengine_pcm {
    pub 1]: *mut *mut dma_chan chan[SNDRV_PCM_STREAM_LAST +,
    pub config: *const snd_dmaengine_pcm_config,
    pub flags: c_uint,
}
