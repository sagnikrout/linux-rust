//! Automatically rewritten from C to Rust
//! Source: kernel/dma.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0
//
// linux/kernel/dma.c: A DMA channel allocator. Inspired by linux/kernel/irq.c.
//
// Written by Hennus Bergman, 1992.
//
// 1994/12/26: Changes by Alex Nash to fix a minor bug in /proc/dma.
// In the previous version the reported device could end up being wrong,
// if a device requested a DMA channel that was already in use.
// [It also happened to remove the sizeof == sizeof(int)
// assumption introduced because of those /proc/dma patches. -- Hennus]
//

// A note on resource allocation:
//
// All drivers needing DMA channels, should allocate and release them
// through the public routines `request_dma()' and `free_dma()'.
//
// In order to avoid problems, all processes should allocate resources in
// the same sequence and release them in the reverse order.
//
// So, when allocating DMAs and IRQs, first allocate the IRQ, then the DMA.
// When releasing them, first release the DMA, then release the IRQ.
// If you don't, you may cause allocation requests to fail unnecessarily.
// This doesn't really matter now, but it will once we get real semaphores
// in the kernel.
//
// DEFINE_SPINLOCK;
//
// If our port doesn't define this it has no PC like DMA
//

// Channel n is busy iff dma_chan_busy[n].lock != 0.
// DMA0 used to be reserved for DRAM refresh, but apparently not any more...
// DMA4 is reserved for cascading.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_chan {
    pub lock: c_int,
    pub device_id: *const c_char,
}

pub static mut dma_chan: usize = 0;
//
// request_dma - request and reserve a system DMA channel
// @dmanr: DMA channel number
// @device_id: reserving device ID string, used in /proc/dma
//
#[no_mangle]
pub unsafe extern "C" fn request_dma(dmanr: c_uint, device_id: *const *const c_char) -> c_int {
    if (dmanr >= MAX_DMA_CHANNELS) {
    return -EINVAL;
    }
    if (xchg(&dma_chan_busy[dmanr].lock, 1) != 0) {
    return -EBUSY;
    }
    dma_chan_busy[dmanr].device_id = device_id;
// old flag was 0, now contains 1 to indicate busy
    return 0;
    } /* request_dma */
//
// free_dma - free a reserved system DMA channel
// @dmanr: DMA channel number
//
#[no_mangle]
pub unsafe extern "C" fn free_dma(dmanr: c_uint) {
    if (dmanr >= MAX_DMA_CHANNELS) {
    printk("Trying to free DMA%d\n", dmanr);
    return;
    }
    if (xchg(&dma_chan_busy[dmanr].lock, 0) == 0) {
    printk("Trying to free free DMA%d\n", dmanr);
    return;
    }
    } /* free_dma */

#[no_mangle]
pub unsafe extern "C" fn request_dma(dmanr: c_uint, device_id: *const c_char) -> c_int {
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn free_dma(dmanr: c_uint) {
    }

#[no_mangle]
unsafe extern "C" fn proc_dma_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    for (i = 0 ; i < MAX_DMA_CHANNELS ; i++) {
    if (dma_chan_busy[i].lock) {
    seq_printf(m, "%2d: %s\n", i,
    dma_chan_busy[i].device_id);
    }
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn proc_dma_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    seq_puts(m, "No DMA\n");
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn proc_dma_init() -> c_int {
    proc_create_single("dma", 0, core::ptr::null_mut(), proc_dma_show);
    return 0;
    }
// __initcall;

// EXPORT_SYMBOL;
// EXPORT_SYMBOL;
// EXPORT_SYMBOL;