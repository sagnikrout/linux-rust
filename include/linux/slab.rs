//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/slab.h
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
// Written by Mark Hemment, 1996 (markhe@nextd.demon.co.uk).
//
// (C) SGI 2006, Christoph Lameter
// Cleaned up and restructured to ease the addition of alternative
// implementations of SLAB allocators.
// (C) Linux Foundation 2008-2013
// Unified interface for all slab allocators
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _slab_flag_bits {
    _SLAB_CONSISTENCY_CHECKS,
    _SLAB_RED_ZONE,
    _SLAB_POISON,
    _SLAB_KMALLOC,
    _SLAB_HWCACHE_ALIGN,
    _SLAB_CACHE_DMA,
    _SLAB_CACHE_DMA32,
    _SLAB_STORE_USER,
    _SLAB_PANIC,
    _SLAB_TYPESAFE_BY_RCU,
    _SLAB_TRACE,

    _SLAB_DEBUG_OBJECTS,

    _SLAB_NOLEAKTRACE,
    _SLAB_NO_MERGE,

    _SLAB_FAILSLAB,

    _SLAB_ACCOUNT,
    _SLAB_MAY_ACCOUNT,

    _SLAB_KASAN,

    _SLAB_NO_USER_FLAGS,

    _SLAB_SKIP_KFENCE,

    _SLAB_RECLAIM_ACCOUNT,

    _SLAB_OBJECT_POISON,
    _SLAB_CMPXCHG_DOUBLE,

    _SLAB_NO_OBJ_EXT,

    _SLAB_OBJ_EXT_IN_OBJ,

    _SLAB_NO_SHEAVES,
    _SLAB_FLAGS_LAST_BIT
}

//
// Flags to pass to kmem_cache_create().
// The ones marked DEBUG need CONFIG_SLUB_DEBUG enabled, otherwise are no-op
//
// DEBUG: Perform (expensive) checks on alloc/free

// DEBUG: Red zone objs in a cache

// DEBUG: Poison objects

// Indicate a kmalloc slab

//
// define SLAB_HWCACHE_ALIGN - Align objects on cache line boundaries.
//
// Sufficiently large objects are aligned on cache line boundary. For object
// size smaller than a half of cache line size, the alignment is on the half of
// cache line size. In general, if object size is smaller than 1/2^n of cache
// line size, the alignment is adjusted to 1/2^n.
//
// If explicit alignment is also requested by the respective
// &struct kmem_cache_args field, the greater of both is alignments is applied.
//

// Use GFP_DMA memory

// Use GFP_DMA32 memory

// DEBUG: Store the last owner for bug hunting

// Panic if kmem_cache_create() fails

//
// define SLAB_TYPESAFE_BY_RCU - **WARNING** READ THIS!
//
// This delays freeing the SLAB page by a grace period, it does _NOT_
// delay object freeing. This means that if you do kmem_cache_free()
// that memory location is free to be reused at any time. Thus it may
// be possible to see another object there in the same RCU grace period.
//
// This feature only ensures the memory location backing the object
// stays valid, the trick to using this is relying on an independent
// object validation pass. Something like:
//
// ::
//
// begin:
// rcu_read_lock();
// obj = lockless_lookup(key);
// if (obj) {
// if (!try_get_ref(obj)) // might fail for free objects
// rcu_read_unlock();
// goto begin;
//
// if (obj->key != key) { // not the object we expected
// put_ref(obj);
// rcu_read_unlock();
// goto begin;
// }
// rcu_read_unlock();
//
// This is useful if we need to approach a kernel structure obliquely,
// from its address obtained without the usual locking. We can lock
// the structure to stabilize it and check it's still at the given address,
// only if we can be sure that the memory has not been meanwhile reused
// for some other kind of object (which our subsystem's lock might corrupt).
//
// rcu_read_lock before reading the address, then rcu_read_unlock after
// taking the spinlock within the structure expected at that address.
//
// Note that object identity check has to be done *after* acquiring a
// reference, therefore user has to ensure proper ordering for loads.
// Similarly, when initializing objects allocated with SLAB_TYPESAFE_BY_RCU,
// the newly allocated object has to be fully initialized *before* its
// refcount gets initialized and proper ordering for stores is required.
// refcount_{add|inc}_not_zero_acquire() and refcount_set_release() are
// designed with the proper fences required for reference counting objects
// allocated with SLAB_TYPESAFE_BY_RCU.
//
// Note that it is not possible to acquire a lock within a structure
// allocated with SLAB_TYPESAFE_BY_RCU without first acquiring a reference
// as described above.  The reason is that SLAB_TYPESAFE_BY_RCU pages
// are not zeroed before being given to the slab, which means that any
// locks must be initialized after each and every kmem_struct_alloc().
// Alternatively, make the ctor passed to kmem_cache_create() initialize
// the locks at page-allocation time, as is done in __i915_request_ctor(),
// sighand_ctor(), and anon_vma_ctor().  Such a ctor permits readers
// to safely acquire those ctor-initialized locks under rcu_read_lock()
// protection.
//
// Note that SLAB_TYPESAFE_BY_RCU was originally named SLAB_DESTROY_BY_RCU.
//

// Trace allocations and frees

// Flag to prevent checks on free

// Avoid kmemleak tracing

//
// Prevent merging with compatible kmem caches. This flag should be used
// cautiously. Valid use cases:
//
// - caches created for self-tests (e.g. kunit)
// - general caches created and used by a subsystem, only when a
// (subsystem-specific) debug option is enabled
// - performance critical caches, should be very rare and consulted with slab
// maintainers, and not used together with CONFIG_SLUB_TINY
//

// Fault injection mark

//
// define SLAB_ACCOUNT - Account allocations to memcg.
//
// All object allocations from this cache will be memcg accounted, regardless of
// __GFP_ACCOUNT being or not being passed to individual allocations.
//

//
// Ignore user specified debugging flags.
// Intended for caches created for self-tests so they have only flags
// specified in the code and other flags are ignored.
//

// The following flags affect the page allocator grouping pages by mobility
//
// define SLAB_RECLAIM_ACCOUNT - Objects are reclaimable.
//
// Use this flag for caches that have an associated shrinker. As a result, slab
// pages are allocated with __GFP_RECLAIMABLE, which affects grouping pages by
// mobility, and are accounted in SReclaimable counter in /proc/meminfo
//

// Slab caches without obj_exts array

//
// ZERO_SIZE_PTR will be returned for zero sized kmalloc requests.
//
// Dereferencing ZERO_SIZE_PTR will lead to a distinct access fault.
//
// ZERO_SIZE_PTR can be passed to kfree though in the same way that NULL can.
// Both make kfree a no-op.
//

//
// struct kmem_cache related prototypes
//
extern "C" {
    pub fn slab_is_available() -> bool;
}
//
// struct kmem_cache_args - Less common arguments for kmem_cache_create()
//
// Any uninitialized fields of the structure are interpreted as unused. The
// exception is @freeptr_offset where %0 is a valid value, so
// @use_freeptr_offset must be also set to %true in order to interpret the field
// as used. For @useroffset %0 is also valid, but only with non-%0
// @usersize.
//
// When %NULL args is passed to kmem_cache_create(), it is equivalent to all
// fields unused.
//
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
// in caches with &SLAB_TYPESAFE_BY_RCU or @ctor
//
// By default, &SLAB_TYPESAFE_BY_RCU and @ctor caches place the free
// pointer outside of the object. This might cause the object to grow
// in size. Cache creators that have a reason to avoid this can specify
// a custom free pointer offset in their data structure where the free
// pointer will be placed.
//
// For caches with &SLAB_TYPESAFE_BY_RCU, the caller must ensure that
// the free pointer does not overlay fields required to guard against
// object recycling (See &SLAB_TYPESAFE_BY_RCU for details).
//
// For caches with @ctor, the caller must ensure that the free pointer
// does not overlay fields initialized by the constructor.
//
// Currently, only caches with &SLAB_TYPESAFE_BY_RCU or @ctor
// may specify @freeptr_offset.
//
// Using %0 as a value for @freeptr_offset is valid. If @freeptr_offset
// is specified, @use_freeptr_offset must be set %true.
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
//
// @sheaf_capacity: Enable sheaves of given capacity for the cache.
//
// With a non-zero value, allocations from the cache go through caching
// arrays called sheaves. Each cpu has a main sheaf that's always
// present, and a spare sheaf that may be not present. When both become
// empty, there's an attempt to replace an empty sheaf with a full sheaf
// from the per-node barn.
//
// When no full sheaf is available, and gfp flags allow blocking, a
// sheaf is allocated and filled from slab(s) using bulk allocation.
// Otherwise the allocation falls back to the normal operation
// allocating a single object from a slab.
//
// Analogically when freeing and both percpu sheaves are full, the barn
// may replace it with an empty sheaf, unless it's over capacity. In
// that case a sheaf is bulk freed to slab pages.
//
// The sheaves do not enforce NUMA placement of objects, so allocations
// via kmem_cache_alloc_node() with a node specified other than
// NUMA_NO_NODE will bypass them.
//
// Bulk allocation and free operations also try to use the cpu sheaves
// and barn, but fallback to using slab pages directly.
//
// When slub_debug is enabled for the cache, the sheaf_capacity argument
// is ignored.
//
// %0 means no sheaves will be created.
//
    pub sheaf_capacity: c_uint,
}

extern "C" {
    pub fn __kmem_cache_create_args(_arg: name, _arg: size, _arg: &kmem_args, _arg: flags) -> return;
}
//
// kmem_cache_create_usercopy - Create a kmem cache with a region suitable
// for copying to userspace.
// @name: A string which is used in /proc/slabinfo to identify this cache.
// @size: The size of objects to be created in this cache.
// @align: The required alignment for the objects.
// @flags: SLAB flags
// @useroffset: Usercopy region offset
// @usersize: Usercopy region size
// @ctor: A constructor for the objects, or %NULL.
//
// This is a legacy wrapper, new code should use either KMEM_CACHE_USERCOPY()
// if whitelisting a single field is sufficient, or kmem_cache_create() with
// the necessary parameters passed via the args parameter (see
// &struct kmem_cache_args)
//
// Return: a pointer to the cache on success, NULL on failure.
//
extern "C" {
    pub fn __kmem_cache_create_args(_arg: name, _arg: size, _arg: &kmem_args, _arg: flags) -> return;
}
// If NULL is passed for @args, use this variant with default arguments.
// Make sure we don't get passed garbage.
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn __kmem_cache_create_args(_arg: name, _arg: size, _arg: &kmem_default_args, _arg: flags) -> return;
}
//
// kmem_cache_create - Create a kmem cache.
// @__name: A string which is used in /proc/slabinfo to identify this cache.
// @__object_size: The size of objects to be created in this cache.
// @__args: Optional arguments, see &struct kmem_cache_args. Passing %NULL
// means defaults will be used for all the arguments.
//
// This is currently implemented as a macro using ``_Generic()`` to call
// either the new variant of the function, or a legacy one.
//
// The new variant has 4 parameters:
// ``kmem_cache_create(name, object_size, args, flags)``
//
// See __kmem_cache_create_args() which implements this.
//
// The legacy variant has 5 parameters:
// ``kmem_cache_create(name, object_size, align, flags, ctor)``
//
// The align and ctor parameters map to the respective fields of
// &struct kmem_cache_args
//
// Context: Cannot be called within a interrupt, but can be interrupted.
//
// Return: a pointer to the cache on success, NULL on failure.
//

extern "C" {
    pub fn kmem_cache_destroy(s: *mut kmem_cache);
}
extern "C" {
    pub fn kmem_cache_shrink(s: *mut kmem_cache) -> c_int;
}
//
// Please use this macro to create slab caches. Simply specify the
// name of the structure and maybe some flags that are listed above.
//
// The alignment of the struct determines object alignment. If you
// f.e. add ____cacheline_aligned_in_smp to the struct declaration
// then the objects will be properly aligned in SMP configurations.
//

//
// To whitelist a single field for copying to/from usercopy, use this
// macro instead for KMEM_CACHE() above.
//

// Macro flag: #define DECL_TOKEN_PARAM(_token)
// Macro flag: #define _PASS_TOKEN_PARAM(_token)

//
// Common kmalloc functions provided by all allocators
//

//
// krealloc_node_align - reallocate memory. The contents will remain unchanged.
// @p: object to reallocate memory for.
// @new_size: how many bytes of memory are required.
// @align: desired alignment.
// @flags: the type of memory to allocate.
// @nid: NUMA node or NUMA_NO_NODE
//
// If @p is %NULL, krealloc() behaves exactly like kmalloc().  If @new_size
// is 0 and @p is not a %NULL pointer, the object pointed to is freed.
//
// Only alignments up to those guaranteed by kmalloc() will be honored. Please see
// Documentation/core-api/memory-allocation.rst for more details.
//
// If __GFP_ZERO logic is requested, callers must ensure that, starting with the
// initial memory allocation, every subsequent call to this API for the same
// memory allocation is flagged with __GFP_ZERO. Otherwise, it is possible that
// __GFP_ZERO is not fully honored by this API.
//
// When slub_debug_orig_size() is off, krealloc() only knows about the bucket
// size of an allocation (but not the exact size it was allocated with) and
// hence implements the following semantics for shrinking and growing buffers
// with __GFP_ZERO::
//
// new             bucket
// 0       size             size
// |--------|----------------|
// |  keep  |      zero      |
//
// Otherwise, the original allocation size 'orig_size' could be used to
// precisely clear the requested size, and the new size will also be stored
// as the new 'orig_size'.
//
// In any case, the contents of the object pointed to are preserved up to the
// lesser of the new and old sizes.
//
// Return: pointer to the allocated memory or %NULL in case of error
//

extern "C" {
    pub fn kfree(objp: *const c_void);
}
extern "C" {
    pub fn kfree_nolock(objp: *const c_void);
}
extern "C" {
    pub fn kfree_sensitive(objp: *const c_void);
}
extern "C" {
    pub fn ksize(objp: *const c_void) -> usize;
}

extern "C" {
    pub fn kmem_dump_obj(object: *mut c_void) -> bool;
}

//
// Some archs want to perform DMA into kmalloc caches and need a guaranteed
// alignment larger than the alignment of a 64-bit integer.
// Setting ARCH_DMA_MINALIGN in arch headers allows that.
//

//
// Setting ARCH_SLAB_MINALIGN in arch headers allows a different alignment.
// Intended for arches that get misalignment faults even for 64 bit integer
// aligned buffers.
//

//
// Arches can define this function if they want to decide the minimum slab
// alignment at runtime. The value returned by the function must be a power
// of two and >= ARCH_SLAB_MINALIGN.
//

//
// kmem_cache_alloc and friends return pointers aligned to ARCH_SLAB_MINALIGN.
// kmalloc and friends return pointers aligned to both ARCH_KMALLOC_MINALIGN
// and ARCH_SLAB_MINALIGN, but here we only assume the former alignment.
//

//
// Kmalloc array related definitions
//
// SLUB directly allocates requests fitting in to an order-1 page
// (PAGE_SIZE*2).  Larger requests are passed to the page allocator.
//

pub const KMALLOC_SHIFT_LOW: c_int = 3;

// Maximum allocatable size

// Maximum size for which we actually use a slab cache

// Maximum order allocatable via the slab allocator

//
// Kmalloc subsystem.
//

//
// This restriction comes from byte sized index implementation.
// Page size is normally 2^12 bytes and, in this case, if we want to use
// byte sized index which can represent 2^8 entries, the size of the object
// should be equal or greater to 2^12 / 2^8 = 2^4 = 16.
// If minimum size of kmalloc is less than 16, we use it as minimum object
// size and give up to use byte sized index.
//

pub const KMALLOC_PARTITION_CACHES_NR: c_int = 0;

//
// Whenever changing this, take care of that kmalloc_type() and
// create_kmalloc_caches() still work as intended.
//
// KMALLOC_NORMAL can contain only unaccounted objects whereas KMALLOC_CGROUP
// is for accounted but unreclaimable and non-dma objects. All the other
// kmem caches can have both accounted and unaccounted objects.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kmalloc_cache_type {
    KMALLOC_NORMAL = 0,

    KMALLOC_DMA = KMALLOC_NORMAL,

    KMALLOC_CGROUP = KMALLOC_NORMAL,

    KMALLOC_NO_OBJ_EXT = KMALLOC_NORMAL,

    KMALLOC_PARTITION_START = KMALLOC_NORMAL,
    KMALLOC_PARTITION_END = KMALLOC_PARTITION_START + KMALLOC_PARTITION_CACHES_NR,

    KMALLOC_RECLAIM = KMALLOC_NORMAL,

    KMALLOC_RECLAIM,

    KMALLOC_DMA,

    KMALLOC_CGROUP,

    KMALLOC_NO_OBJ_EXT,

    NR_KMALLOC_TYPES
}

//
// Define gfp bits that should not be set for KMALLOC_NORMAL.
//

//
// The most common case is KMALLOC_NORMAL, so test for it
// with a single branch for all the relevant flags.
//

// KMALLOC_PARTITION_CACHES_NR (=15) copies + the KMALLOC_NORMAL

//
// At least one of the flags has to be set. Their priorities in
// decreasing order are:
// 1) __GFP_DMA
// 2) __GFP_RECLAIMABLE
// 3) __GFP_ACCOUNT
//
// Figure out which kmalloc slab an allocation of a certain size
// belongs to.
// 0 = zero alloc
// 1 =  65 .. 96 bytes
// 2 = 129 .. 192 bytes
// n = 2^(n-1)+1 .. 2^n
//
// Note: __kmalloc_index() is compile-time optimized, and not runtime optimized;
// typical usage is via kmalloc_index() and therefore evaluated at compile-time.
// Callers where !size_is_constant should only be test modules, where runtime
// overheads of __kmalloc_index() can be tolerated.  Also see kmalloc_slab().
//
// Will never be reached. Needed because the compiler may complain

//
// kmem_cache_alloc - Allocate an object
// @cachep: The cache to allocate from.
// @flags: See kmalloc().
//
// Allocate an object from this cache.
// See kmem_cache_zalloc() for a shortcut of adding __GFP_ZERO to flags.
//
// Return: pointer to the new object or %NULL in case of error
//

//
// kmem_cache_charge - memcg charge an already allocated slab memory
// @objp: address of the slab object to memcg charge
// @gfpflags: describe the allocation context
//
// kmem_cache_charge allows charging a slab object to the current memcg,
// primarily in cases where charging at allocation time might not be possible
// because the target memcg is not known (i.e. softirq context)
//
// The objp should be pointer returned by the slab allocator functions like
// kmalloc (with __GFP_ACCOUNT in flags) or kmem_cache_alloc. The memcg charge
// behavior can be controlled through gfpflags parameter, which affects how the
// necessary internal metadata can be allocated. Including __GFP_NOFAIL denotes
// that overcharging is requested instead of failure, but is not applied for the
// internal metadata allocation.
//
// There are several cases where it will return true even if the charging was
// not done:
// More specifically:
//
// 1. For !CONFIG_MEMCG or cgroup_disable=memory systems.
// 2. Already charged slab objects.
// 3. For slab objects from KMALLOC_NORMAL caches - allocated by kmalloc()
// without __GFP_ACCOUNT
// 4. Allocating internal metadata has failed
//
// Return: true if charge was successful otherwise false.
//
extern "C" {
    pub fn kmem_cache_charge(objp: *mut c_void, gfpflags: gfp_t) -> bool;
}
extern "C" {
    pub fn kmem_cache_free(s: *mut kmem_cache, objp: *mut c_void);
}
//
// Bulk allocation and freeing operations. These are accelerated in an
// allocator specific way to avoid taking locks repeatedly or building
// metadata structures unnecessarily.
//
// Note that interrupts must be enabled when calling these functions.
//
extern "C" {
    pub fn kmem_cache_free_bulk(s: *mut kmem_cache, size: usize, p: *mut c_void);
}

extern "C" {
    pub fn kmem_cache_sheaf_size(sheaf: *mut slab_sheaf) -> c_uint;
}
//
// These macros allow declaring a kmem_buckets * parameter alongside size, which
// can be compiled out with CONFIG_SLAB_BUCKETS=n so that a large number of call
// sites don't have to pass NULL.
//

//
// The following functions are not to be used directly and are intended only
// for internal use from kmalloc() and kmalloc_node()
// with the exception of kunit tests
//
extern "C" {
    pub fn __alloc_size(_arg: 1) -> __assume_kmalloc_alignment;
}
extern "C" {
    pub fn __alloc_size(_arg: 1) -> __assume_kmalloc_alignment;
}
extern "C" {
    pub fn __alloc_size(_arg: 3) -> __assume_kmalloc_alignment;
}
extern "C" {
    pub fn __alloc_size(_arg: 4) -> __assume_kmalloc_alignment;
}
extern "C" {
    pub fn __alloc_size(_arg: 1) -> __assume_page_alignment;
}
extern "C" {
    pub fn __alloc_size(_arg: 1) -> __assume_page_alignment;
}
extern "C" {
    pub fn __kmalloc_large_noprof(_arg: size, _arg: flags) -> return;
}
extern "C" {
    pub fn __kmalloc_noprof(_arg: PASS_TOKEN_PARAMS(size, _arg: token), _arg: flags) -> return;
}

//
// kmalloc - allocate kernel memory
// @size: how many bytes of memory are required.
// @flags: describe the allocation context
//
// kmalloc is the normal method of allocating memory
// for objects smaller than page size in the kernel.
//
// The allocated object address is aligned to at least ARCH_KMALLOC_MINALIGN
// bytes. For @size of power of two bytes, the alignment is also guaranteed
// to be at least to the size. For other sizes, the alignment is guaranteed to
// be at least the largest power-of-two divisor of @size.
//
// The @flags argument may be one of the GFP flags defined at
// include/linux/gfp_types.h and described at
// :ref:`Documentation/core-api/mm-api.rst <mm-api-gfp-flags>`
//
// The recommended usage of the @flags is described at
// :ref:`Documentation/core-api/memory-allocation.rst <memory_allocation>`
//
// Below is a brief outline of the most useful GFP flags
//
// %GFP_KERNEL
// Allocate normal kernel ram. May sleep.
//
// %GFP_NOWAIT
// Allocation will not sleep.
//
// %GFP_ATOMIC
// Allocation will not sleep.  May use emergency pools.
//
// Also it is possible to set different flags by OR'ing
// in one or more of the following additional @flags:
//
// %__GFP_ZERO
// Zero the allocated memory before returning. Also see kzalloc().
//
// %__GFP_HIGH
// This allocation has high priority and may use emergency pools.
//
// %__GFP_NOFAIL
// Indicate that this allocation is in no way allowed to fail
// (think twice before using).
//
// %__GFP_NORETRY
// If memory is not immediately available,
// then give up at once.
//
// %__GFP_NOWARN
// If allocation fails, don't issue any warnings.
//
// %__GFP_RETRY_MAYFAIL
// Try really hard to succeed the allocation but fail
// eventually.
//

//
// kmalloc_nolock - Allocate an object of given size from any context.
// @size: size to allocate
// @gfp_flags: GFP flags. Only __GFP_ACCOUNT and __GFP_ZERO allowed.  Also
// __GFP_NOWARN and __GFP_NOMEMALLOC are allowed but added internally thus not
// necessary.
// @node: node number of the target node.
//
// Return: pointer to the new object or NULL in case of error.
// NULL does not mean EBUSY or EAGAIN. It means ENOMEM.
// There is no reason to call it again and expect !NULL.
//

//
// __alloc_objs - Allocate objects of a given type using
// @KMALLOC: which size-based kmalloc wrapper to allocate with.
// @GFP: GFP flags for the allocation.
// @TYPE: type to allocate space for.
// @COUNT: how many @TYPE objects to allocate.
//
// Returns: Newly allocated pointer to (first) @TYPE of @COUNT-many
// allocated @TYPE objects, or NULL on failure.
//

//
// __alloc_flex - Allocate an object that has a trailing flexible array
// @KMALLOC: kmalloc wrapper function to use for allocation.
// @GFP: GFP flags for the allocation.
// @TYPE: type of structure to allocate space for.
// @FAM: The name of the flexible array member of @TYPE structure.
// @COUNT: how many @FAM elements to allocate space for.
//
// Returns: Newly allocated pointer to @TYPE with @COUNT-many trailing
// @FAM elements, or NULL on failure or if @COUNT cannot be represented
// by the member of @TYPE that counts the @FAM elements (annotated via
// __counted_by()).
//

//
// kmalloc_obj - Allocate a single instance of the given type
// @VAR_OR_TYPE: Variable or type to allocate.
// @...: optional GFP flags for the allocation (GFP_KERNEL when not specified).
//
// Returns: newly allocated pointer to a @VAR_OR_TYPE on success, or NULL
// on failure.
//

//
// kmalloc_objs - Allocate an array of the given type
// @VAR_OR_TYPE: Variable or type to allocate an array of.
// @COUNT: How many elements in the array.
// @...: optional GFP flags for the allocation (GFP_KERNEL when not specified).
//
// Returns: newly allocated pointer to array of @VAR_OR_TYPE on success,
// or NULL on failure.
//

//
// kmalloc_flex - Allocate a single instance of the given flexible structure
// @VAR_OR_TYPE: Variable or type to allocate (with its flex array).
// @FAM: The name of the flexible array member of the structure.
// @COUNT: How many flexible array member elements are desired.
// @...: optional GFP flags for the allocation (GFP_KERNEL when not specified).
//
// Returns: newly allocated pointer to @VAR_OR_TYPE on success, NULL on
// failure. If @FAM has been annotated with __counted_by(), the allocation
// will immediately fail if @COUNT is larger than what the type of the
// struct's counter variable can represent.
//

// All kzalloc aliases for kmalloc_(obj|objs|flex).

// All kvmalloc aliases for kmalloc_(obj|objs|flex).

// All kvzalloc aliases for kmalloc_(obj|objs|flex).

extern "C" {
    pub fn __kmalloc_large_node_noprof(_arg: size, _arg: flags, _arg: node) -> return;
}
extern "C" {
    pub fn __kmalloc_node_noprof(_arg: PASS_KMALLOC_PARAMS(size, _arg: NULL, _arg: token), _arg: flags, _arg: node) -> return;
}

extern "C" {
    pub fn _kmalloc_noprof(_arg: bytes, _arg: flags, _arg: token) -> return;
}

//
// kmalloc_array - allocate memory for an array.
// @n: number of elements.
// @size: element size.
// @flags: the type of memory to allocate (see kmalloc).
//

extern "C" {
    pub fn krealloc_node_align_noprof(_arg: p, _arg: PASS_TOKEN_PARAMS(bytes, _arg: token), _arg: 1, _arg: flags, _arg: NUMA_NO_NODE) -> return;
}

//
// krealloc_array - reallocate memory for an array.
// @p: pointer to the memory chunk to reallocate
// @new_n: new number of elements to alloc
// @new_size: new size of a single member of the array
// @flags: the type of memory to allocate (see kmalloc)
//
// If __GFP_ZERO logic is requested, callers must ensure that, starting with the
// initial memory allocation, every subsequent call to this API for the same
// memory allocation is flagged with __GFP_ZERO. Otherwise, it is possible that
// __GFP_ZERO is not fully honored by this API.
//
// See krealloc_noprof() for further details.
//
// In any case, the contents of the object pointed to are preserved up to the
// lesser of the new and old sizes.
//

//
// kcalloc - allocate memory for an array. The memory is set to zero.
// @n: number of elements.
// @size: element size.
// @flags: the type of memory to allocate (see kmalloc).
//

//
// kmalloc_track_caller is a special version of kmalloc that records the
// calling function of the routine calling it for slab leak tracking instead
// of just the calling function (confusing, eh?).
// It's useful when the call to kmalloc comes from a widely-used standard
// allocator where we care about the real place the memory allocation
// request comes from.
//

extern "C" {
    pub fn _kmalloc_node_noprof(_arg: bytes, _arg: flags, _arg: node, _arg: token) -> return;
}
extern "C" {
    pub fn __kmalloc_node_noprof(_arg: PASS_KMALLOC_PARAMS(bytes, _arg: NULL, _arg: token), _arg: flags, _arg: node) -> return;
}

//
// Shortcuts
//

extern "C" {
    pub fn _kmalloc_noprof(_arg: size, __GFP_ZERO: flags |, _arg: token) -> return;
}

//
// kzalloc - allocate memory. The memory is set to zero.
// @size: how many bytes of memory are required.
// @flags: the type of memory to allocate (see kmalloc).
//

//
// kvmalloc_node - attempt to allocate physically contiguous memory, but upon
// failure, fall back to non-contiguous (vmalloc) allocation.
// @size: size of the request.
// @flags: gfp mask for the allocation - must be compatible (superset) with GFP_KERNEL.
// @node: numa node to allocate from
//
// Only alignments up to those guaranteed by kmalloc() will be honored. Please see
// Documentation/core-api/memory-allocation.rst for more details.
//
// Uses kmalloc to get the memory but if the allocation fails then falls back
// to the vmalloc allocator. Use kvfree for freeing the memory.
//
// GFP_NOWAIT and GFP_ATOMIC are supported, the __GFP_NORETRY modifier is not.
// __GFP_RETRY_MAYFAIL is supported, and it should be used only if kmalloc is
// preferable to the vmalloc fallback, due to visible performance drawbacks.
//
// Return: pointer to the allocated memory of %NULL in case of failure
//

extern "C" {
    pub fn __kvmalloc_node_noprof(_arg: PASS_KMALLOC_PARAMS(bytes, _arg: NULL, _arg: token), _arg: 1, _arg: flags, _arg: node) -> return;
}

//
// kvrealloc_node_align - reallocate memory; contents remain unchanged
// @p: object to reallocate memory for
// @size: the size to reallocate
// @align: desired alignment
// @flags: the flags for the page level allocator
// @nid: NUMA node id
//
// If @p is %NULL, kvrealloc() behaves exactly like kvmalloc(). If @size is 0
// and @p is not a %NULL pointer, the object pointed to is freed.
//
// Only alignments up to those guaranteed by kmalloc() will be honored. Please see
// Documentation/core-api/memory-allocation.rst for more details.
//
// If __GFP_ZERO logic is requested, callers must ensure that, starting with the
// initial memory allocation, every subsequent call to this API for the same
// memory allocation is flagged with __GFP_ZERO. Otherwise, it is possible that
// __GFP_ZERO is not fully honored by this API.
//
// In any case, the contents of the object pointed to are preserved up to the
// lesser of the new and old sizes.
//
// This function must not be called concurrently with itself or kvfree() for the
// same memory allocation.
//
// Return: pointer to the allocated memory or %NULL in case of error
//

extern "C" {
    pub fn kvfree(addr: *const c_void);
}
extern "C" {
    pub fn kvfree_atomic(addr: *const c_void);
}
extern "C" {
    pub fn kvfree_sensitive(addr: *const c_void, len: usize);
}
extern "C" {
    pub fn kmem_cache_size(s: *mut kmem_cache) -> c_uint;
}

extern "C" {
    pub fn kfree_rcu_scheduler_running();
}

extern "C" {
    pub fn kvfree_rcu_barrier();
}
extern "C" {
    pub fn kvfree_rcu_barrier_on_cache(s: *mut kmem_cache);
}
//
// kmalloc_size_roundup - Report allocation bucket size for the given size
//
// @size: Number of bytes to round up from.
//
// This returns the number of bytes that would be available in a kmalloc()
// allocation of @size bytes. For example, a 126 byte request would be
// rounded up to the next sized kmalloc bucket, 128 bytes. (This is strictly
// for the general-purpose kmalloc()-based allocations, and is not for the
// pre-sized kmem_cache_alloc()-based allocations.)
//
// Use this to kmalloc() the full bucket size ahead of time instead of using
// ksize() to query the size after an allocation.
//
extern "C" {
    pub fn kmalloc_size_roundup(size: usize) -> usize;
}
extern "C" {
    pub fn kmem_cache_init_late() -> void __init;
}
extern "C" {
    pub fn kvfree_rcu_init() -> void __init;
}
