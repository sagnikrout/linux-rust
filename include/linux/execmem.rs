//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/execmem.h
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
// enum execmem_type - types of executable memory ranges
//
// There are several subsystems that allocate executable memory.
// Architectures define different restrictions on placement,
// permissions, alignment and other parameters for memory that can be used
// by these subsystems.
// Types in this enum identify subsystems that allocate executable memory
// and let architectures define parameters for ranges suitable for
// allocations by each subsystem.
//
// @EXECMEM_DEFAULT: default parameters that would be used for types that
// are not explicitly defined.
// @EXECMEM_MODULE_TEXT: parameters for module text sections
// @EXECMEM_KPROBES: parameters for kprobes
// @EXECMEM_FTRACE: parameters for ftrace
// @EXECMEM_BPF: parameters for BPF
// @EXECMEM_MODULE_DATA: parameters for module data sections
// @EXECMEM_TYPE_MAX:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum execmem_type {
    EXECMEM_DEFAULT,
    EXECMEM_MODULE_TEXT = EXECMEM_DEFAULT,
    EXECMEM_KPROBES,
    EXECMEM_FTRACE,
    EXECMEM_BPF,
    EXECMEM_MODULE_DATA,
    EXECMEM_TYPE_MAX,
}

//
// enum execmem_range_flags - options for executable memory allocations
// @EXECMEM_KASAN_SHADOW:	allocate kasan shadow
// @EXECMEM_ROX_CACHE:		allocations should use ROX cache of huge pages
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum execmem_range_flags {
    EXECMEM_KASAN_SHADOW	= (1 << 0),
    EXECMEM_ROX_CACHE	= (1 << 1),
}

//
// execmem_fill_trapping_insns - set memory to contain instructions that
// will trap
// @ptr:	pointer to memory to fill
// @size:	size of the range to fill
//
// A hook for architecures to fill execmem ranges with invalid instructions.
// Architectures that use EXECMEM_ROX_CACHE must implement this.
//
extern "C" {
    pub fn execmem_fill_trapping_insns(ptr: *mut c_void, size: usize);
}
//
// execmem_restore_rox - restore read-only-execute permissions
// @ptr:	address of the region to remap
// @size:	size of the region to remap
//
// Restores read-only-execute permissions on a range [@ptr, @ptr + @size)
// after it was temporarily remapped as writable. Relies on architecture
// implementation of set_memory_rox() to restore mapping using large pages.
//
// Return: 0 on success or negative error code on failure.
//
extern "C" {
    pub fn execmem_restore_rox(ptr: *mut c_void, size: usize) -> c_int;
}

//
// struct execmem_range - definition of an address space suitable for code and
// related data allocations
// @start:	address space start
// @end:	address space end (inclusive)
// @fallback_start: start of the secondary address space range for fallback
// allocations on architectures that require it
// @fallback_end:   end of the secondary address space (inclusive)
// @pgprot:	permissions for memory in this address space
// @alignment:	alignment required for text allocations
// @flags:	options for memory allocations for this range
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct execmem_range {
    pub start: c_ulong,
    pub end: c_ulong,
    pub fallback_start: c_ulong,
    pub fallback_end: c_ulong,
    pub pgprot: pgprot_t,
    pub alignment: c_uint,
    pub flags: execmem_range_flags,
}

//
// struct execmem_info - architecture parameters for code allocations
// @ranges: array of parameter sets defining architecture specific
// parameters for executable memory allocations. The ranges that are not
// explicitly initialized by an architecture use parameters defined for
// @EXECMEM_DEFAULT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct execmem_info {
    pub ranges: [execmem_range; EXECMEM_TYPE_MAX],
}

//
// execmem_arch_setup - define parameters for allocations of executable memory
//
// A hook for architectures to define parameters for allocations of
// executable memory. These parameters should be filled into the
// @execmem_info structure.
//
// For architectures that do not implement this method a default set of
// parameters will be used
//
// Return: a structure defining architecture parameters and restrictions
// for allocations of executable memory
//
// execmem_alloc - allocate executable memory
// @type: type of the allocation
// @size: how many bytes of memory are required
//
// Allocates memory that will contain executable code, either generated or
// loaded from kernel modules.
//
// Allocates memory that will contain data coupled with executable code,
// like data sections in kernel modules.
//
// The memory will have protections defined by architecture for executable
// region of the @type.
//
// Return: a pointer to the allocated memory or %NULL
//
// execmem_alloc_rw - allocate writable executable memory
// @type: type of the allocation
// @size: how many bytes of memory are required
//
// Allocates memory that will contain executable code, either generated or
// loaded from kernel modules.
//
// Allocates memory that will contain data coupled with executable code,
// like data sections in kernel modules.
//
// Forces writable permissions on the allocated memory and the caller is
// responsible to manage the permissions afterwards.
//
// For architectures that use ROX cache the permissions will be set to R+W.
// For architectures that don't use ROX cache the default permissions for @type
// will be used as they must be writable.
//
// Return: a pointer to the allocated memory or %NULL
//
// execmem_free - free executable memory
// @ptr: pointer to the memory that should be freed
//
extern "C" {
    pub fn execmem_free(ptr: *mut c_void);
}

//
// execmem_vmap - create virtual mapping for EXECMEM_MODULE_DATA memory
// @size: size of the virtual mapping in bytes
//
// Maps virtually contiguous area in the range suitable for EXECMEM_MODULE_DATA.
//
// Return: the area descriptor on success or %NULL on failure.
//

//
// execmem_is_rox - check if execmem is read-only
// @type - the execmem type to check
//
// Return: %true if the @type is read-only, %false if it's writable
//
extern "C" {
    pub fn execmem_is_rox(type: execmem_type) -> bool;
}

extern "C" {
    pub fn execmem_init();
}

