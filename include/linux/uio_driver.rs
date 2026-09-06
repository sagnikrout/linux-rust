//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/uio_driver.h
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
// include/linux/uio_driver.h
//
// Copyright(C) 2005, Benedikt Spranger <b.spranger@linutronix.de>
// Copyright(C) 2005, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright(C) 2006, Hans J. Koch <hjk@hansjkoch.de>
// Copyright(C) 2006, Greg Kroah-Hartman <greg@kroah.com>
//
// Userspace IO driver.
//

//
// struct uio_mem - description of a UIO memory region
// @name:		name of the memory region for identification
// @addr:               address of the device's memory rounded to page
// size (phys_addr is used since addr can be
// logical, virtual, or physical & phys_addr_t
// should always be large enough to handle any of
// the address types)
// @dma_addr:		DMA handle set by dma_alloc_coherent, used with
// UIO_MEM_DMA_COHERENT only (@addr should be the
// void * returned from the same dma_alloc_coherent call)
// @offs:               offset of device memory within the page
// @size:		size of IO (multiple of page size)
// @memtype:		type of memory addr points to
// @internal_addr:	ioremap-ped version of addr, for driver internal use
// @dma_device:		device struct that was passed to dma_alloc_coherent,
// used with UIO_MEM_DMA_COHERENT only
// @map:		for use by the UIO core only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uio_mem {
    pub name: *const c_char,
    pub addr: phys_addr_t,
    pub dma_addr: dma_addr_t,
    pub offs: c_ulong,
    pub size: resource_size_t,
    pub memtype: c_int,
    pub internal_addr: *mut void __iomem,
    pub dma_device: *mut device,
    pub map: *mut uio_map,
}

pub const MAX_UIO_MAPS: c_int = 5;
//
// struct uio_port - description of a UIO port region
// @name:		name of the port region for identification
// @start:		start of port region
// @size:		size of port region
// @porttype:		type of port (see UIO_PORT_* below)
// @portio:		for use by the UIO core only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uio_port {
    pub name: *const c_char,
    pub start: c_ulong,
    pub size: c_ulong,
    pub porttype: c_int,
    pub portio: *mut uio_portio,
}

pub const MAX_UIO_PORT_REGIONS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uio_device {
    pub owner: *mut module,
    pub dev: device,
    pub minor: c_int,
    pub event: core::sync::atomic::AtomicI32,
    pub async_queue: *mut fasync_struct,
    pub wait: wait_queue_head_t,
    pub info: *mut uio_info,
    pub info_lock: mutex,
    pub map_dir: *mut kobject,
    pub portio_dir: *mut kobject,
}

//
// struct uio_info - UIO device capabilities
// @uio_dev:		the UIO device this info belongs to
// @name:		device name
// @version:		device driver version
// @mem:		list of mappable memory regions, size==0 for end of list
// @port:		list of port regions, size==0 for end of list
// @irq:		interrupt number or UIO_IRQ_CUSTOM
// @irq_flags:		flags for request_irq()
// @priv:		optional private data
// @handler:		the device's irq handler
// @mmap_prepare:	mmap_prepare operation for this uio device
// @open:		open operation for this uio device
// @release:		release operation for this uio device
// @irqcontrol:		disable/enable irqs when 0/1 is written to /dev/uioX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uio_info {
    pub uio_dev: *mut uio_device,
    pub name: *const c_char,
    pub version: *const c_char,
    pub mem: [uio_mem; MAX_UIO_MAPS],
    pub port: [uio_port; MAX_UIO_PORT_REGIONS],
    pub irq: c_long,
    pub irq_flags: c_ulong,
    pub priv: *mut c_void,
    pub dev_info): *mut *mut irqreturn_t (handler)(int irq, struct uio_info,
    pub desc): *mut *mut *mut int (mmap_prepare)(struct uio_info info, struct vm_area_desc,
    pub inode): *mut *mut *mut int (open)(struct uio_info info, struct inode,
    pub inode): *mut *mut *mut int (release)(struct uio_info info, struct inode,
    pub irq_on): *mut *mut *mut int (irqcontrol)(struct uio_info info, s32,
}

// use a define to avoid include chaining to get THIS_MODULE
//
// uio_register_device - register a new userspace IO device
// @parent:	parent device
// @info:	UIO device capabilities
//
// returns zero on success or a negative error code.
//

extern "C" {
    pub fn uio_unregister_device(info: *mut uio_info);
}
extern "C" {
    pub fn uio_event_notify(info: *mut uio_info);
}
// use a define to avoid include chaining to get THIS_MODULE
//
// devm_uio_register_device - Resource managed uio_register_device()
// @parent:	parent device
// @info:	UIO device capabilities
//
// returns zero on success or a negative error code.
//

// defines for uio_info->irq

pub const UIO_IRQ_NONE: c_int = 0;
// defines for uio_mem->memtype
pub const UIO_MEM_NONE: c_int = 0;
pub const UIO_MEM_PHYS: c_int = 1;
pub const UIO_MEM_LOGICAL: c_int = 2;
pub const UIO_MEM_VIRTUAL: c_int = 3;
pub const UIO_MEM_IOVA: c_int = 4;
//
// UIO_MEM_DMA_COHERENT exists for legacy drivers that had been getting by with
// improperly mapping DMA coherent allocations through the other modes.
// Do not use in new drivers.
//
pub const UIO_MEM_DMA_COHERENT: c_int = 5;
// defines for uio_port->porttype
pub const UIO_PORT_NONE: c_int = 0;
pub const UIO_PORT_X86: c_int = 1;
pub const UIO_PORT_GPIO: c_int = 2;
pub const UIO_PORT_OTHER: c_int = 3;
