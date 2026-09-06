//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-io.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cx18 driver PCI memory mapped IO access routines
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//

//
// Readback and retry of MMIO access for reliability:
// The concept was suggested by Steve Toth <stoth@linuxtv.org>.
// The implementation is the fault of Andy Walls <awalls@md.metrocast.net>.
//
// *write* functions are implied to retry the mmio unless suffixed with _noretry
// *read* functions never retry the mmio (it never helps to do so)
//
// Non byteswapping memory mapped IO
extern "C" {
    pub fn __raw_readl(_arg: addr) -> return;
}
// Normal memory mapped IO
extern "C" {
    pub fn readl(_arg: addr) -> return;
}
extern "C" {
    pub fn readw(_arg: addr) -> return;
}
extern "C" {
    pub fn readb(_arg: addr) -> return;
}
extern "C" {
    pub fn cx18_memset_io(cx: *mut cx18, addr: *mut void __iomem, val: c_int, count: usize);
}
// Access "register" region of CX23418 memory mapped I/O
extern "C" {
    pub fn cx18_readl(_arg: cx, reg: cx->reg_mem +) -> return;
}
// Access "encoder memory" region of CX23418 memory mapped I/O
extern "C" {
    pub fn cx18_readl(_arg: cx, addr: cx->enc_mem +) -> return;
}
extern "C" {
    pub fn cx18_sw1_irq_enable(cx: *mut cx18, val: u32);
}
extern "C" {
    pub fn cx18_sw1_irq_disable(cx: *mut cx18, val: u32);
}
extern "C" {
    pub fn cx18_sw2_irq_enable(cx: *mut cx18, val: u32);
}
extern "C" {
    pub fn cx18_sw2_irq_disable(cx: *mut cx18, val: u32);
}
extern "C" {
    pub fn cx18_sw2_irq_disable_cpu(cx: *mut cx18, val: u32);
}
extern "C" {
    pub fn cx18_setup_page(cx: *mut cx18, addr: u32);
}
