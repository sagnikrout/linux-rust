//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/pcmcia/nsp_io.h
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
// $Id: nsp_io.h,v 1.3 2003/08/04 21:15:26 elca Exp $
//
// Basic IO
//
extern "C" {
    pub fn inb(index: base +) -> return;
}
//
// Indexed IO
//
extern "C" {
    pub fn inb(DATAREG: BaseAddr +) -> return;
}
//
// fifo func
//
// read 8 bit FIFO
// nsp_dbg(NSP_DEBUG_DATA_IO, "buf=0x%p, count=0x%lx", buf, count);
// --------------------------------------------------------------
// read 16 bit FIFO
// nsp_dbg(NSP_DEBUG_DATA_IO, "buf=0x%p, count=0x%lx*2", buf, count);
// --------------------------------------------------------------
// read 32bit FIFO
// nsp_dbg(NSP_DEBUG_DATA_IO, "buf=0x%p, count=0x%lx*4", buf, count);
// ----------------------------------------------------------
// write 8bit FIFO
// ---------------------------------------------------------
// write 16bit FIFO
// ---------------------------------------------------------
// write 32bit FIFO
// ====================================================================
extern "C" {
    pub fn readb(_arg: ptr) -> return;
}
// -----------
extern "C" {
    pub fn readb(_arg: data_ptr) -> return;
}
// read 32bit FIFO
// nsp_dbg(NSP_DEBUG_DATA_IO, "base 0x%0lx ptr 0x%p",base,ptr);
// tmp = readl(ptr);
// nsp_dbg(NSP_DEBUG_DATA_IO, "<%d,%p,%p,%lx>", i, ptr, tmp, *tmp);
// nsp_dbg(NSP_DEBUG_DATA_IO, "buf=0x%p, count=0x%lx*4", buf, count);
// nsp_dbg(NSP_DEBUG_DATA_IO, "base 0x%0lx ptr 0x%p",base,ptr);
// nsp_dbg(NSP_DEBUG_DATA_IO, "<%d,%p,%p,%lx>", i, ptr, tmp, *tmp);
// nsp_dbg(NSP_DEBUG_DATA_IO, "buf=0x%p, count=0x%lx*4", buf, count);

// end
