//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vmalloc.h
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

// bits in flags of vmalloc's vm_struct below
pub const VM_IOREMAP: c_uint = 0x00000001	/* ioremap() and friends */;
pub const VM_ALLOC: c_uint = 0x00000002	/* vmalloc() */;
pub const VM_MAP: c_uint = 0x00000004	/* vmap()ed pages */;
pub const VM_USERMAP: c_uint = 0x00000008	/* suitable for remap_vmalloc_range */;
pub const VM_DMA_COHERENT: c_uint = 0x00000010	/* dma_alloc_coherent */;
pub const VM_UNINITIALIZED: c_uint = 0x00000020	/* vm_struct is not fully initialized */;
pub const VM_NO_GUARD: c_uint = 0x00000040      /* ***DANGEROUS*** don't add guard page */;
pub const VM_KASAN: c_uint = 0x00000080      /* has allocated kasan shadow memory */;
pub const VM_FLUSH_RESET_PERMS: c_uint = 0x00000100	/* reset direct map and flush TLB on unmap, can't be freed in atomic context */;
pub const VM_MAP_PUT_PAGES: c_uint = 0x00000200	/* put pages and free array in vfree */;
pub const VM_ALLOW_HUGE_VMAP: c_uint = 0x00000400      /* Allow for huge pages on archs with HAVE_ARCH_HUGE_VMALLOC */;

pub const VM_DEFER_KMEMLEAK: c_uint = 0x00000800	/* defer kmemleak object creation */;

pub const VM_DEFER_KMEMLEAK: c_int = 0;

pub const VM_SPARSE: c_uint = 0x00001000	/* sparse vm_area. not all pages are present. */;
// bits [20..32] reserved for arch specific ioremap internals
//
// Maximum alignment for ioremap() regions.
// Can be overridden by arch-specific value.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_struct {
    pub /: *mut *mut *mut vm_next; / Early registration of vm_areas.,
    pub /: *mut *mut llist_node llnode; / Asynchronous freeing on error paths.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmap_area {
    pub va_start: c_ulong,
    pub va_end: c_ulong,
    pub /: *mut *mut rb_node rb_node; / address sorted rbtree,
    pub /: *mut *mut list_head list; / address sorted list,
//
// The following two variables can be packed, because
// a vmap_area object can be either:
// 1) in "free" tree (root is free_vmap_area_root)
// 2) or "busy" tree (root is vmap_area_root)
//
    pub /: *mut *mut unsigned long subtree_max_size; / in "free" tree,
    pub /: *mut *mut *mut vm_vm; / in "busy" tree,
}

// archs that select HAVE_ARCH_HUGE_VMAP should override one or more of these

//
// Highlevel APIs for driver use
//
extern "C" {
    pub fn vm_unmap_ram(mem: *const c_void, count: c_uint);
}
extern "C" {
    pub fn vm_unmap_aliases();
}

extern "C" {
    pub fn vmalloc_huge_node(_arg: size, _arg: gfp_mask, _arg: NUMA_NO_NODE) -> return;
}

extern "C" {
    pub fn vfree(addr: *const c_void);
}
extern "C" {
    pub fn vfree_atomic(addr: *const c_void);
}
extern "C" {
    pub fn vunmap(addr: *const c_void);
}
//
// Lowlevel-APIs (not for driver use!)
//
// return actual size without guard page
extern "C" {
    pub fn free_vm_area(area: *mut vm_struct);
}
//
// This may not 100% tell if the area is mapped with > PAGE_SIZE
// page table entries, if for some reason the architecture indicates
// larger sizes are available but decides not to use them, nothing
// prevents that. This only indicates the size of the physical page
// allocated in the vmalloc layer.
//

// for /proc/kcore
extern "C" {
    pub fn vread_iter(iter: *mut iov_iter, addr: *const c_char, count: usize) -> c_long;
}
//
// Internals.  Don't use..
//
extern "C" {
    pub fn vm_area_add_early(vm: *mut vm_struct) -> __init void;
}
extern "C" {
    pub fn vm_area_register_early(vm: *mut vm_struct, align: usize) -> __init void;
}
extern "C" {
    pub fn register_vmap_purge_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_vmap_purge_notifier(nb: *mut notifier_block) -> c_int;
}

extern "C" {
    pub fn vunmap_range(addr: c_ulong, end: c_ulong);
}

extern "C" {
    pub fn pcpu_free_vm_areas(vms: *mut vm_struct, nr_vms: c_int);
}

extern "C" {
    pub fn vmalloc_dump_obj(object: *mut c_void) -> bool;
}

extern "C" {
    pub fn memalloc_apply_gfp_scope(gfp_mask: gfp_t) -> c_uint;
}
extern "C" {
    pub fn memalloc_restore_scope(flags: c_uint);
}
