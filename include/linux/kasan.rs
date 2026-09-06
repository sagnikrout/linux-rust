//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kasan.h
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

pub type kasan_vmalloc_flags_t = u32;

pub const KASAN_VMALLOC_PAGE_RANGE: c_uint = 0x1 /* Apply exsiting page range */;
pub const KASAN_VMALLOC_TLB_FLUSH: c_uint = 0x2 /* TLB flush */;

// Software KASAN implementations use shadow memory.

// This matches KASAN_TAG_INVALID.
pub const KASAN_SHADOW_INIT: c_uint = 0xFE;

pub const KASAN_SHADOW_INIT: c_int = 0;

pub const PTE_HWTABLE_PTRS: c_int = 0;

extern "C" {
    pub fn kasan_add_zero_shadow(start: *mut c_void, size: c_ulong) -> c_int;
}
extern "C" {
    pub fn kasan_remove_zero_shadow(start: *mut c_void, size: c_ulong);
}
// Enable reporting bugs after kasan_disable_current()
extern "C" {
    pub fn kasan_enable_current();
}
// Disable reporting bugs for current task
extern "C" {
    pub fn kasan_disable_current();
}

extern "C" {
    pub fn kasan_hw_tags_enabled() -> return;
}

extern "C" {
    pub fn __kasan_unpoison_range(addr: *const c_void, size: usize);
}
extern "C" {
    pub fn __kasan_poison_pages(page: *mut page, order: c_uint, init: bool);
}
extern "C" {
    pub fn __kasan_unpoison_pages(page: *mut page, order: c_uint, init: bool) -> bool;
}
extern "C" {
    pub fn __kasan_unpoison_pages(_arg: page, _arg: order, _arg: init) -> return;
}
extern "C" {
    pub fn __kasan_poison_slab(slab: *mut slab);
}
extern "C" {
    pub fn __kasan_unpoison_new_object(cache: *mut kmem_cache, object: *mut c_void);
}
//
// kasan_unpoison_new_object - Temporarily unpoison a new slab object.
// @cache: Cache the object belong to.
// @object: Pointer to the object.
//
// This function is intended for the slab allocator's internal use. It
// temporarily unpoisons an object from a newly allocated slab without doing
// anything else. The object must later be repoisoned by
// kasan_poison_new_object().
//
extern "C" {
    pub fn __kasan_poison_new_object(cache: *mut kmem_cache, object: *mut c_void);
}
//
// kasan_poison_new_object - Repoison a new slab object.
// @cache: Cache the object belong to.
// @object: Pointer to the object.
//
// This function is intended for the slab allocator's internal use. It
// repoisons an object that was previously unpoisoned by
// kasan_unpoison_new_object() without doing anything else.
//
extern "C" {
    pub fn __kasan_init_slab_obj(_arg: cache, _arg: object) -> return;
}
//
// kasan_slab_pre_free - Check whether freeing a slab object is safe.
// @object: Object to be freed.
//
// This function checks whether freeing the given object is safe. It may
// check for double-free and invalid-free bugs and report them.
//
// This function is intended only for use by the slab allocator.
//
// @Return true if freeing the object is unsafe; false otherwise.
//
extern "C" {
    pub fn __kasan_slab_pre_free(_arg: s, _arg: object, _arg: _RET_IP_) -> return;
}
//
// kasan_slab_free - Poison, initialize, and quarantine a slab object.
// @object: Object to be freed.
// @init: Whether to initialize the object.
// @still_accessible: Whether the object contents are still accessible.
//
// This function informs that a slab object has been freed and is not
// supposed to be accessed anymore, except when @still_accessible is set
// (indicating that the object is in a SLAB_TYPESAFE_BY_RCU cache and an RCU
// grace period might not have passed yet).
//
// For KASAN modes that have integrated memory initialization
// (kasan_has_integrated_init() == true), this function also initializes
// the object's memory. For other modes, the @init argument is ignored.
//
// This function might also take ownership of the object to quarantine it.
// When this happens, KASAN will defer freeing the object to a later
// stage and handle it internally until then. The return value indicates
// whether KASAN took ownership of the object.
//
// This function is intended only for use by the slab allocator.
//
// @Return true if KASAN took ownership of the object; false otherwise.
//
extern "C" {
    pub fn __kasan_kfree_large(ptr: *mut c_void, ip: c_ulong);
}
extern "C" {
    pub fn __kasan_slab_alloc(_arg: s, _arg: object, _arg: flags, _arg: init) -> return;
}
extern "C" {
    pub fn __kasan_kmalloc(_arg: s, _arg: object, _arg: size, _arg: flags) -> return;
}
extern "C" {
    pub fn __kasan_kmalloc_large(_arg: ptr, _arg: size, _arg: flags) -> return;
}
extern "C" {
    pub fn __kasan_krealloc(_arg: object, _arg: new_size, _arg: flags) -> return;
}
//
// kasan_mempool_poison_pages - Check and poison a mempool page allocation.
// @page: Pointer to the page allocation.
// @order: Order of the allocation.
//
// This function is intended for kernel subsystems that cache page allocations
// to reuse them instead of freeing them back to page_alloc (e.g. mempool).
//
// This function is similar to kasan_mempool_poison_object() but operates on
// page allocations.
//
// Before the poisoned allocation can be reused, it must be unpoisoned via
// kasan_mempool_unpoison_pages().
//
// Return: true if the allocation can be safely reused; false otherwise.
//
extern "C" {
    pub fn __kasan_mempool_poison_pages(_arg: page, _arg: order, _arg: _RET_IP_) -> return;
}
//
// kasan_mempool_unpoison_pages - Unpoison a mempool page allocation.
// @page: Pointer to the page allocation.
// @order: Order of the allocation.
//
// This function is intended for kernel subsystems that cache page allocations
// to reuse them instead of freeing them back to page_alloc (e.g. mempool).
//
// This function unpoisons a page allocation that was previously poisoned by
// kasan_mempool_poison_pages() without zeroing the allocation's memory. For
// the tag-based modes, this function assigns a new tag to the allocation.
//
extern "C" {
    pub fn __kasan_mempool_poison_object(ptr: *mut c_void, ip: c_ulong) -> bool;
}
//
// kasan_mempool_poison_object - Check and poison a mempool slab allocation.
// @ptr: Pointer to the slab allocation.
//
// This function is intended for kernel subsystems that cache slab allocations
// to reuse them instead of freeing them back to the slab allocator (e.g.
// mempool).
//
// This function poisons a slab allocation and saves a free stack trace for it
// without initializing the allocation's memory and without putting it into the
// quarantine (for the Generic mode).
//
// This function also performs checks to detect double-free and invalid-free
// bugs and reports them. The caller can use the return value of this function
// to find out if the allocation is buggy.
//
// Before the poisoned allocation can be reused, it must be unpoisoned via
// kasan_mempool_unpoison_object().
//
// This function operates on all slab allocations including large kmalloc
// allocations (i.e. the ones backed directly by the buddy allocator rather
// than kmalloc slab caches).
//
// Return: true if the allocation can be safely reused; false otherwise.
//
extern "C" {
    pub fn __kasan_mempool_poison_object(_arg: ptr, _arg: _RET_IP_) -> return;
}
extern "C" {
    pub fn __kasan_mempool_unpoison_object(ptr: *mut c_void, size: usize, ip: c_ulong);
}
//
// kasan_mempool_unpoison_object - Unpoison a mempool slab allocation.
// @ptr: Pointer to the slab allocation.
// @size: Size to be unpoisoned.
//
// This function is intended for kernel subsystems that cache slab allocations
// to reuse them instead of freeing them back to the slab allocator (e.g.
// mempool).
//
// This function unpoisons a slab allocation that was previously poisoned via
// kasan_mempool_poison_object() and saves an alloc stack trace for it without
// initializing the allocation's memory. For the tag-based modes, this function
// does not assign a new tag to the allocation and instead restores the
// original tags based on the pointer value.
//
// This function operates on all slab allocations including large kmalloc
// allocations (i.e. the ones backed directly by the buddy allocator rather
// than kmalloc slab caches).
//
// Unlike kasan_check_read/write(), kasan_check_byte() is performed even for
// the hardware tag-based mode that doesn't rely on compiler instrumentation.
//
extern "C" {
    pub fn __kasan_check_byte(addr: *const c_void, ip: c_ulong) -> bool;
}
extern "C" {
    pub fn __kasan_check_byte(_arg: addr, _arg: _RET_IP_) -> return;
}

extern "C" {
    pub fn kasan_unpoison_task_stack(task: *mut task_struct);
}
extern "C" {
    pub fn kasan_unpoison_task_stack_below(watermark: *const c_void) -> asmlinkage void;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kasan_cache {
    pub alloc_meta_offset: c_int,
    pub free_meta_offset: c_int,
}

extern "C" {
    pub fn kasan_metadata_size(cache: *mut kmem_cache, in_object: bool) -> usize;
}
extern "C" {
    pub fn kasan_cache_shrink(cache: *mut kmem_cache);
}
extern "C" {
    pub fn kasan_cache_shutdown(cache: *mut kmem_cache);
}
extern "C" {
    pub fn kasan_record_aux_stack(ptr: *mut c_void);
}

// Tag-based KASAN modes do not use per-object metadata.
// And no cache-related metadata initialization is required.

//
// kasan_report - print a report about a bad memory access detected by KASAN
// @addr: address of the bad access
// @size: size of the bad access
// @is_write: whether the bad access is a write or a read
// @ip: instruction pointer for the accessibility check or the bad access itself
//

extern "C" {
    pub fn kasan_report_async();
}

extern "C" {
    pub fn kasan_init_generic() -> void __init;
}

extern "C" {
    pub fn kasan_init_sw_tags() -> void __init;
}

extern "C" {
    pub fn kasan_init_hw_tags_cpu();
}
extern "C" {
    pub fn kasan_init_hw_tags() -> void __init;
}

extern "C" {
    pub fn kasan_populate_early_vm_area_shadow(start: *mut c_void, size: c_ulong);
}
extern "C" {
    pub fn __kasan_populate_vmalloc(addr: c_ulong, size: c_ulong, gfp_mask: gfp_t) -> c_int;
}
extern "C" {
    pub fn __kasan_populate_vmalloc(_arg: addr, _arg: size, _arg: gfp_mask) -> return;
}

extern "C" {
    pub fn __kasan_unpoison_vmalloc(_arg: start, _arg: size, _arg: flags) -> return;
}
extern "C" {
    pub fn __kasan_poison_vmalloc(start: *const c_void, size: c_ulong);
}

//
// These functions allocate and free shadow memory for kernel modules.
// They are only required when KASAN_VMALLOC is not supported, as otherwise
// shadow memory is allocated by the generic vmalloc handlers.
//
extern "C" {
    pub fn kasan_alloc_module_shadow(addr: *mut c_void, size: usize, gfp_mask: gfp_t) -> c_int;
}
extern "C" {
    pub fn kasan_free_module_shadow(vm: *const vm_struct);
}

extern "C" {
    pub fn kasan_non_canonical_hook(addr: c_ulong);
}

