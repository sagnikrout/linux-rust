//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/comedi/comedi_isadma.h
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
// COMEDI ISA DMA support functions
// Copyright (c) 2014 H Hartley Sweeten <hsweeten@visionengravers.com>
//

//
// These are used to avoid issues when <asm/dma.h> and the DMA_MODE_
// defines are not available.
//
pub const COMEDI_ISADMA_READ: c_int = 0;
pub const COMEDI_ISADMA_WRITE: c_int = 1;
//
// struct comedi_isadma_desc - cookie for ISA DMA
// @virt_addr:	virtual address of buffer
// @hw_addr:	hardware (bus) address of buffer
// @chan:	DMA channel
// @maxsize:	allocated size of buffer (in bytes)
// @size:	transfer size (in bytes)
// @mode:	DMA_MODE_READ or DMA_MODE_WRITE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comedi_isadma_desc {
    pub virt_addr: *mut c_void,
    pub hw_addr: dma_addr_t,
    pub chan: c_uint,
    pub maxsize: c_uint,
    pub size: c_uint,
    pub mode: c_char,
}

//
// struct comedi_isadma - ISA DMA data
// @dev:	device to allocate non-coherent memory for
// @desc:	cookie for each DMA buffer
// @n_desc:	the number of cookies
// @cur_dma:	the current cookie in use
// @chan:	the first DMA channel requested
// @chan2:	the second DMA channel requested
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct comedi_isadma {
    pub dev: *mut device,
    pub n_desc: c_int,
    pub cur_dma: c_int,
    pub chan: c_uint,
    pub chan2: c_uint,
    pub __counted_by(n_desc): comedi_isadma_desc desc[],
}

extern "C" {
    pub fn comedi_isadma_program(desc: *mut comedi_isadma_desc);
}
extern "C" {
    pub fn comedi_isadma_disable(dma_chan: c_uint) -> c_uint;
}
extern "C" {
    pub fn comedi_isadma_poll(dma: *mut comedi_isadma) -> c_uint;
}
extern "C" {
    pub fn comedi_isadma_set_mode(desc: *mut comedi_isadma_desc, dma_dir: c_char);
}
extern "C" {
    pub fn comedi_isadma_free(dma: *mut comedi_isadma);
}

