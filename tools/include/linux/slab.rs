//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/slab.h
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

pub const SLAB_RECLAIM_ACCOUNT: c_uint = 0x00020000UL            /* Objects are reclaimable */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _slab_flag_bits {
    _SLAB_KMALLOC,
    _SLAB_HWCACHE_ALIGN,
    _SLAB_PANIC,
    _SLAB_TYPESAFE_BY_RCU,
    _SLAB_ACCOUNT,
    _SLAB_FLAGS_LAST_BIT
}

extern "C" {
    pub fn kfree(p: *mut c_void);
}
extern "C" {
    pub fn slab_is_available() -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slab_state {
    DOWN,
    PARTIAL,
    UP,
    FULL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache {
    pub lock: pthread_mutex_t,
    pub size: c_uint,
    pub align: c_uint,
    pub sheaf_capacity: c_uint,
    pub nr_objs: c_int,
    pub objs: *mut c_void,
    pub ): *mut *mut void (ctor)(void,
    pub non_kernel_enabled: bool,
    pub non_kernel: c_uint,
    pub nr_allocated: c_ulong,
    pub nr_tallocated: c_ulong,
    pub exec_callback: bool,
    pub ): *mut *mut void (callback)(void,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_cache_args {
//
// @align: The required alignment for the objects.
//
// %0 means no specific alignment is requested.
//
    pub align: c_uint,
//
// @sheaf_capacity: The maximum size of the sheaf.
//
    pub sheaf_capacity: c_uint,
//
// @useroffset: Usercopy region offset.
//
// %0 is a valid offset, when @usersize is non-%0
//
    pub useroffset: c_uint,
//
// @usersize: Usercopy region size.
//
// %0 means no usercopy region is specified.
//
    pub usersize: c_uint,
//
// @freeptr_offset: Custom offset for the free pointer
// in &SLAB_TYPESAFE_BY_RCU caches
//
// By default &SLAB_TYPESAFE_BY_RCU caches place the free pointer
// outside of the object. This might cause the object to grow in size.
// Cache creators that have a reason to avoid this can specify a custom
// free pointer offset in their struct where the free pointer will be
// placed.
//
// Note that placing the free pointer inside the object requires the
// caller to ensure that no fields are invalidated that are required to
// guard against object recycling (See &SLAB_TYPESAFE_BY_RCU for
// details).
//
// Using %0 as a value for @freeptr_offset is valid. If @freeptr_offset
// is specified, %use_freeptr_offset must be set %true.
//
// Note that @ctor currently isn't supported with custom free pointers
// as a @ctor requires an external free pointer.
//
    pub freeptr_offset: c_uint,
//
// @use_freeptr_offset: Whether a @freeptr_offset is used.
//
    pub use_freeptr_offset: bool,
//
// @ctor: A constructor for the objects.
//
// The constructor is invoked for each object in a newly allocated slab
// page. It is the cache user's responsibility to free object in the
// same state as after calling the constructor, or deal appropriately
// with any differences between a freshly constructed and a reallocated
// object.
//
// %NULL means no constructor.
//
    pub ): *mut *mut void (ctor)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_sheaf {
    pub barn_list: list_head,
// only used for prefilled sheafs
    pub capacity: c_uint,
}

extern "C" {
    pub fn kmalloc(_arg: size, __GFP_ZERO: gfp |) -> return;
}
extern "C" {
    pub fn kmem_cache_alloc_lru(_arg: cachep, _arg: NULL, _arg: flags) -> return;
}
extern "C" {
    pub fn kmem_cache_free(cachep: *mut kmem_cache, objp: *mut c_void);
}
// If NULL is passed for @args, use this variant with default arguments.
extern "C" {
    pub fn __kmem_cache_create_args(_arg: name, _arg: size, _arg: &kmem_default_args, _arg: flags) -> return;
}
extern "C" {
    pub fn __kmem_cache_create_args(_arg: name, _arg: size, _arg: &kmem_args, _arg: flags) -> return;
}

extern "C" {
    pub fn kmem_cache_free_bulk(cachep: *mut kmem_cache, size: usize, list: *mut c_void);
}

