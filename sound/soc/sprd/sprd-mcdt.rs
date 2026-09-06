//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sprd/sprd-mcdt.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sprd_mcdt_channel_type {
    SPRD_MCDT_DAC_CHAN,
    SPRD_MCDT_ADC_CHAN,
    SPRD_MCDT_UNKNOWN_CHAN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sprd_mcdt_dma_chan {
    SPRD_MCDT_DMA_CH0,
    SPRD_MCDT_DMA_CH1,
    SPRD_MCDT_DMA_CH2,
    SPRD_MCDT_DMA_CH3,
    SPRD_MCDT_DMA_CH4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_mcdt_chan_callback {
    pub data): *mut *mut void (notify)(void,
    pub data: *mut c_void,
}

//
// struct sprd_mcdt_chan - this struct represents a single channel instance
// @mcdt: the mcdt controller
// @id: channel id
// @fifo_phys: channel fifo physical address which is used for DMA transfer
// @type: channel type
// @cb: channel fifo interrupt's callback interface to notify the fifo events
// @dma_enable: indicate if use DMA mode to transfer data
// @int_enable: indicate if use interrupt mode to notify users to read or
// write data manually
// @list: used to link into the global list
//
// Note: users should not modify any members of this structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_mcdt_chan {
    pub mcdt: *mut sprd_mcdt_dev,
    pub id: u8,
    pub fifo_phys: c_ulong,
    pub type: sprd_mcdt_channel_type,
    pub dma_chan: sprd_mcdt_dma_chan,
    pub cb: *mut sprd_mcdt_chan_callback,
    pub dma_enable: bool,
    pub int_enable: bool,
    pub list: list_head,
}

extern "C" {
    pub fn sprd_mcdt_free_chan(chan: *mut sprd_mcdt_chan);
}
extern "C" {
    pub fn sprd_mcdt_chan_write(chan: *mut sprd_mcdt_chan, tx_buf: *mut c_char, size: u32) -> c_int;
}
extern "C" {
    pub fn sprd_mcdt_chan_read(chan: *mut sprd_mcdt_chan, rx_buf: *mut c_char, size: u32) -> c_int;
}
extern "C" {
    pub fn sprd_mcdt_chan_int_disable(chan: *mut sprd_mcdt_chan);
}
extern "C" {
    pub fn sprd_mcdt_chan_dma_disable(chan: *mut sprd_mcdt_chan);
}

