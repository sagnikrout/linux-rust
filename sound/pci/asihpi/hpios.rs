//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/asihpi/hpios.h
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


// SPDX-License-Identifier: GPL-2.0-only
//

// Macro flag: #define HPI_OS_LINUX_KERNEL
// Macro flag: #define HPI_OS_DEFINED
// Macro flag: #define HPI_BUILD_KERNEL_MODE

// Macro flag: #define HPI_NO_OS_FILE_OPS
// Details of a memory area allocated with  pci_alloc_consistent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct consistent_dma_area {
    pub pdev: *mut device,
// looks like dma-mapping dma_devres ?!
    pub size: usize,
    pub vaddr: *mut c_void,
    pub dma_handle: dma_addr_t,
}

// locked_mem_handle, u32 *p_physical_addr)
// p_physical_addr = locked_mem_handle->dma_handle;
// locked_mem_handle, void **pp_virtual_addr)
// pp_virtual_addr = locked_mem_handle->vaddr;
// locked_mem_handle)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_ioctl_linux {
    pub phm: *mut void __user,
    pub phr: *mut void __user,
}

// Conflict?: H is already used by a number of drivers hid, bluetooth hci,
//

// Macro flag: #define HPI_LOCKING
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpios_spinlock {
    pub /: *mut *mut spinlock_t lock; / SEE hpios_spinlock,
    pub lock_context: c_int,
}

// The reason for all this evilness is that ALSA calls some of a drivers
// operators in atomic context, and some not.  But all our functions channel
// through the HPI_Message conduit, so we can't handle the different context
// per function
//
pub const IN_LOCK_BH: c_int = 1;
pub const IN_LOCK_IRQ: c_int = 0;
// NO bh or isr can execute on this processor,
//

// Macro flag: #define HPI_BUILD_DEBUG

// Macro flag: #define HPI_ALIST_LOCKING

// pci drvdata points to an instance of this struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_adapter {
    pub adapter: *mut hpi_adapter_obj,
    pub snd_card: *mut snd_card,
    pub irq: c_int,
    pub interrupt_mode: c_int,
    pub ): *mut *mut void (interrupt_callback) (struct hpi_adapter,
// mutex prevents contention for one card
    pub mutex: mutex,
    pub p_buffer: *mut c_char,
    pub buffer_size: usize,
}
