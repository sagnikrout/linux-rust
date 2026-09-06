//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/nsp32_io.h
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
// Workbit NinjaSCSI-32Bi/UDE PCI/CardBus SCSI Host Bus Adapter driver
// I/O routine
//
// This software may be used and distributed according to the terms of
// the GNU General Public License.
//
extern "C" {
    pub fn inb(index: base +) -> return;
}
extern "C" {
    pub fn inw(index: base +) -> return;
}
extern "C" {
    pub fn inl(index: base +) -> return;
}
// ==============================================
extern "C" {
    pub fn readb(_arg: ptr) -> return;
}
extern "C" {
    pub fn le16_to_cpu(_arg: readw(ptr)) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: readl(ptr)) -> return;
}
// ==============================================
extern "C" {
    pub fn inb(DATA_REG_LOW: base +) -> return;
}
extern "C" {
    pub fn inw(DATA_REG_LOW: base +) -> return;
}
// ==============================================
extern "C" {
    pub fn readb(_arg: data_ptr) -> return;
}
extern "C" {
    pub fn le16_to_cpu(_arg: readw(data_ptr)) -> return;
}
// ==============================================

// end
