//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ioport.h
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
//
// ioport.h	Definitions of routines for detecting, reserving and
// allocating system resources.
//
// Authors:	Linus Torvalds
//

//
// Resources are tree-like, allowing
// nesting etc..
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource {
    pub start: resource_size_t,
    pub end: resource_size_t,
    pub name: *const c_char,
    pub flags: c_ulong,
    pub desc: c_ulong,
    pub child: *mut *mut *mut resource parent, sibling,,
}

//
// IO resources have these defined flags.
//
// PCI devices expose these flags to userspace in the "resource" sysfs file,
// so don't move them.
//
pub const IORESOURCE_BITS: c_uint = 0x000000ff	/* Bus-specific bits */;
pub const IORESOURCE_TYPE_BITS: c_uint = 0x00001f00	/* Resource type */;
pub const IORESOURCE_IO: c_uint = 0x00000100	/* PCI/ISA I/O ports */;
pub const IORESOURCE_MEM: c_uint = 0x00000200;
pub const IORESOURCE_REG: c_uint = 0x00000300	/* Register offsets */;
pub const IORESOURCE_IRQ: c_uint = 0x00000400;
pub const IORESOURCE_DMA: c_uint = 0x00000800;
pub const IORESOURCE_BUS: c_uint = 0x00001000;
pub const IORESOURCE_PREFETCH: c_uint = 0x00002000	/* No side effects */;
pub const IORESOURCE_READONLY: c_uint = 0x00004000;
pub const IORESOURCE_CACHEABLE: c_uint = 0x00008000;
pub const IORESOURCE_RANGELENGTH: c_uint = 0x00010000;
pub const IORESOURCE_SHADOWABLE: c_uint = 0x00020000;
pub const IORESOURCE_SIZEALIGN: c_uint = 0x00040000	/* size indicates alignment */;
pub const IORESOURCE_STARTALIGN: c_uint = 0x00080000	/* start field is alignment */;
pub const IORESOURCE_MEM_64: c_uint = 0x00100000;
pub const IORESOURCE_WINDOW: c_uint = 0x00200000	/* forwarded by bridge */;
pub const IORESOURCE_MUXED: c_uint = 0x00400000	/* Resource is software muxed */;
pub const IORESOURCE_EXT_TYPE_BITS: c_uint = 0x01000000	/* Resource extended types */;
pub const IORESOURCE_SYSRAM: c_uint = 0x01000000	/* System RAM (modifier) */;
// IORESOURCE_SYSRAM specific bits.
pub const IORESOURCE_SYSRAM_DRIVER_MANAGED: c_uint = 0x02000000 /* Always detected via a driver. */;
pub const IORESOURCE_SYSRAM_MERGEABLE: c_uint = 0x04000000 /* Resource can be merged. */;
pub const IORESOURCE_EXCLUSIVE: c_uint = 0x08000000	/* Userland may not map this resource */;
pub const IORESOURCE_DISABLED: c_uint = 0x10000000;
pub const IORESOURCE_UNSET: c_uint = 0x20000000	/* No address assigned yet */;
pub const IORESOURCE_AUTO: c_uint = 0x40000000;
pub const IORESOURCE_BUSY: c_uint = 0x80000000	/* Driver has marked this resource busy */;
// I/O resource extended types

// PnP IRQ specific bits (IORESOURCE_BITS)

// PnP DMA specific bits (IORESOURCE_BITS)

// PnP memory I/O specific bits (IORESOURCE_BITS)

// PnP I/O specific bits (IORESOURCE_BITS)

// PCI ROM control bits (IORESOURCE_BITS)

// PCI control bits.  Shares IORESOURCE_BITS with above PCI ROM.

//
// I/O Resource Descriptors
//
// Descriptors are used by walk_iomem_res_desc() and region_intersects()
// for searching a specific resource range in the iomem table.  Assign
// a new descriptor when a resource range supports the search interfaces.
// Otherwise, resource.desc must be set to IORES_DESC_NONE (0).
//
// Flags controlling ioremap() behavior.
//
// helpers to define resources

//
// typedef resource_alignf - Resource alignment callback
// @data:	Private data used by the callback
// @res:	Resource candidate range (an empty resource space)
// @empty_res:	Empty resource range without alignment applied
// @size:	The minimum size of the empty space
// @align:	Alignment from the constraints
//
// Callback allows calculating resource placement and alignment beyond min,
// max, and align fields in the struct resource_constraint.
//
// Return: Start address for the resource.
//
// struct resource_constraint - constraints to be met while searching empty
// resource space
// @min:		The minimum address for the memory range
// @max:		The maximum address for the memory range
// @align:		Alignment for the start address of the empty space
// @alignf:		Additional alignment constraints callback
// @alignf_data:	Data provided for @alignf callback
//
// Contains the range and alignment constraints that have to be met during
// find_resource_space(). @alignf can be NULL indicating no alignment beyond
// @align is necessary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_constraint {
    pub align: resource_size_t min, max,,
    pub alignf: resource_alignf,
    pub alignf_data: *mut c_void,
}

// PC/ISA/whatever - the normal PC address spaces: IO and memory
extern "C" {
    pub fn request_resource(root: *mut resource, new: *mut resource) -> c_int;
}
extern "C" {
    pub fn release_resource(new: *mut resource) -> c_int;
}
extern "C" {
    pub fn release_child_resources(new: *mut resource);
}
extern "C" {
    pub fn insert_resource(parent: *mut resource, new: *mut resource) -> c_int;
}
extern "C" {
    pub fn insert_resource_expand_to_fit(root: *mut resource, new: *mut resource);
}
extern "C" {
    pub fn remove_resource(old: *mut resource) -> c_int;
}
extern "C" {
    pub fn arch_remove_reservations(avail: *mut resource);
}
extern "C" {
    pub fn resource_alignment(res: *const resource) -> resource_size_t;
}
//
// resource_set_size - Calculate resource end address from size and start
// @res: Resource descriptor
// @size: Size of the resource
//
// Calculate the end address for @res based on @size.
//
// Note: The start address of @res must be set when calling this function.
// Prefer resource_set_range() if setting both the start address and @size.
//
// resource_set_range - Set resource start and end addresses
// @res: Resource descriptor
// @start: Start address for the resource
// @size: Size of the resource
//
// Set @res start address and calculate the end address based on @size.
//
// For checking if @r1 completely contains @r2 for resources that have real
// addresses but are not yet crafted into the resource tree. Normally
// resource_contains() should be used instead of this function as it checks
// also IORESOURCE_UNSET flag.
//
// True iff r1 completely contains r2
extern "C" {
    pub fn __resource_contains_unbound(_arg: r1, _arg: r2) -> return;
}
// True if any part of r1 overlaps r2
//
// Check if this resource is added to a resource tree or detached. Caller is
// responsible for not racing assignment.
//
// Convenience shorthand with allocation

// Compatibility cruft

extern "C" {
    pub fn release_mem_region_adjustable(_arg: resource_size_t, _arg: resource_size_t);
}

extern "C" {
    pub fn merge_system_ram_resource(res: *mut resource);
}

// Wrappers for managed devices
extern "C" {
    pub fn devm_release_resource(dev: *mut device, new: *mut resource);
}

extern "C" {
    pub fn iomem_map_sanity_check(addr: resource_size_t, size: c_ulong) -> c_int;
}
extern "C" {
    pub fn iomem_is_exclusive(addr: u64) -> bool;
}

