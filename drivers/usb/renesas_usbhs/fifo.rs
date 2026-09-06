//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/renesas_usbhs/fifo.h
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


// SPDX-License-Identifier: GPL-1.0+
//
// Renesas USB driver
//
// Copyright (C) 2011 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_fifo {
    pub name: *mut c_char,
    pub /: *mut *mut u32 port; / xFIFO,
    pub /: *mut *mut u32 sel; / xFIFOSEL,
    pub /: *mut *mut u32 ctr; / xFIFOCTR,
    pub pipe: *mut usbhs_pipe,
    pub tx_chan: *mut dma_chan,
    pub rx_chan: *mut dma_chan,
    pub tx_slave: sh_dmae_slave,
    pub rx_slave: sh_dmae_slave,
}

pub const USBHS_MAX_NUM_DFIFO: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_fifo_info {
    pub cfifo: usbhs_fifo,
    pub dfifo: [usbhs_fifo; USBHS_MAX_NUM_DFIFO],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_pkt {
    pub node: list_head,
    pub pipe: *mut usbhs_pipe,
    pub handler: *const usbhs_pkt_handle,
    pub pkt): *mut usbhs_pkt,
    pub work: work_struct,
    pub dma: dma_addr_t,
    pub dma_result: *const dmaengine_result,
    pub buf: *mut c_void,
    pub length: c_int,
    pub trans: c_int,
    pub actual: c_int,
    pub zero: c_int,
    pub sequence: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhs_pkt_handle {
    pub is_done): *mut *mut *mut int (prepare)(struct usbhs_pkt pkt, int,
    pub is_done): *mut *mut *mut int (try_run)(struct usbhs_pkt pkt, int,
    pub is_done): *mut *mut *mut int (dma_done)(struct usbhs_pkt pkt, int,
}

//
// fifo
//
extern "C" {
    pub fn usbhs_fifo_probe(priv: *mut usbhs_priv) -> c_int;
}
extern "C" {
    pub fn usbhs_fifo_remove(priv: *mut usbhs_priv);
}
extern "C" {
    pub fn usbhs_fifo_init(priv: *mut usbhs_priv);
}
extern "C" {
    pub fn usbhs_fifo_quit(priv: *mut usbhs_priv);
}
extern "C" {
    pub fn usbhs_fifo_clear_dcp(pipe: *mut usbhs_pipe);
}
//
// packet info
//
extern "C" {
    pub fn usbhs_pkt_init(pkt: *mut usbhs_pkt);
}
extern "C" {
    pub fn usbhs_pkt_start(pipe: *mut usbhs_pipe);
}
