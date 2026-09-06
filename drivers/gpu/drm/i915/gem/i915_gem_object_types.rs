//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gem/i915_gem_object_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2016 Intel Corporation
//

//
// struct i915_lut_handle tracks the fast lookups from handle to vma used
// for execbuf. Although we use a radixtree for that mapping, in order to
// remove them as the object or context is closed, we need a secondary list
// and a translation entry (i915_lut_handle).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_lut_handle {
    pub obj_link: list_head,
    pub ctx: *mut i915_gem_context,
    pub handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_object_ops {
    pub flags: c_uint,

// Skip the shrinker management in set_pages/unset_pages

// Interface between the GEM object and its backing storage.
// get_pages() is called once prior to the use of the associated set
// of pages before to binding them into the GTT, and put_pages() is
// called after we no longer need them. As we expect there to be
// associated cost with migrating pages between the backing storage
// and making them available for the GPU (e.g. clflush), we may hold
// onto the pages after they are no longer referenced by the GPU
// in case they may be used again shortly (for example migrating the
// pages to a different memory domain within the GTT). put_pages()
// will therefore most likely be called when the object itself is
// being released or under memory pressure (where we attempt to
// reap pages for the shrinker).
//
    pub obj): *mut *mut int (get_pages)(struct drm_i915_gem_object,
    pub pages): *mut sg_table,
    pub obj): *mut *mut int (truncate)(struct drm_i915_gem_object,
//
// shrink - Perform further backend specific actions to facilate
// shrinking.
// @obj: The gem object
// @flags: Extra flags to control shrinking behaviour in the backend
//
// Possible values for @flags:
//
// I915_GEM_OBJECT_SHRINK_WRITEBACK - Try to perform writeback of the
// backing pages, if supported.
//
// I915_GEM_OBJECT_SHRINK_NO_GPU_WAIT - Don't wait for the object to
// idle.  Active objects can be considered later. The TTM backend for
// example might have aync migrations going on, which don't use any
// i915_vma to track the active GTT binding, and hence having an unbound
// object might not be enough.
//

    pub flags): *mut *mut *mut int (shrink)(struct drm_i915_gem_object obj, unsigned int,
    pub arg): *const drm_i915_gem_pread,
    pub arg): *const drm_i915_gem_pwrite,
    pub obj): *mut *mut u64 (mmap_offset)(struct drm_i915_gem_object,
    pub obj): *mut *mut void (unmap_virtual)(struct drm_i915_gem_object,
    pub obj): *mut *mut int (dmabuf_export)(struct drm_i915_gem_object,
//
// adjust_lru - notify that the madvise value was updated
// @obj: The gem object
//
// The madvise value may have been updated, or object was recently
// referenced so act accordingly (Perhaps changing an LRU list etc).
//
    pub obj): *mut *mut void (adjust_lru)(struct drm_i915_gem_object,
//
// delayed_free - Override the default delayed free implementation
//
    pub obj): *mut *mut void (delayed_free)(struct drm_i915_gem_object,
//
// migrate - Migrate object to a different region either for
// pinning or for as long as the object lock is held.
//
    pub flags): c_uint,
    pub obj): *mut *mut void (release)(struct drm_i915_gem_object,
    pub mmap_ops: *const vm_operations_struct,
    pub /: *const *const *const char name; / friendly name for debug, e.g. lockdep classes,
}

//
// enum i915_cache_level - The supported GTT caching values for system memory
// pages.
//
// These translate to some special GTT PTE bits when binding pages into some
// address space. It also determines whether an object, or rather its pages are
// coherent with the GPU, when also reading or writing through the CPU cache
// with those pages.
//
// Userspace can also control this through struct drm_i915_gem_caching.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i915_cache_level {
//
// @I915_CACHE_NONE:
//
// GPU access is not coherent with the CPU cache. If the cache is dirty
// and we need the underlying pages to be coherent with some later GPU
// access then we need to manually flush the pages.
//
// On shared LLC platforms reads and writes through the CPU cache are
// still coherent even with this setting. See also
// &drm_i915_gem_object.cache_coherent for more details. Due to this we
// should only ever use uncached for scanout surfaces, otherwise we end
// up over-flushing in some places.
//
// This is the default on non-LLC platforms.
//
    I915_CACHE_NONE = 0,
//
// @I915_CACHE_LLC:
//
// GPU access is coherent with the CPU cache. If the cache is dirty,
// then the GPU will ensure that access remains coherent, when both
// reading and writing through the CPU cache. GPU writes can dirty the
// CPU cache.
//
// Not used for scanout surfaces.
//
// Applies to both platforms with shared LLC(HAS_LLC), and snooping
// based platforms(HAS_SNOOP).
//
// This is the default on shared LLC platforms.  The only exception is
// scanout objects, where the display engine is not coherent with the
// CPU cache. For such objects I915_CACHE_NONE or I915_CACHE_WT is
// automatically applied by the kernel in pin_for_display, if userspace
// has not done so already.
//
    I915_CACHE_LLC,
//
// @I915_CACHE_L3_LLC:
//
// Explicitly enable the Gfx L3 cache, with coherent LLC.
//
// The Gfx L3 sits between the domain specific caches, e.g
// sampler/render caches, and the larger LLC. LLC is coherent with the
// GPU, but L3 is only visible to the GPU, so likely needs to be flushed
// when the workload completes.
//
// Not used for scanout surfaces.
//
// Only exposed on some gen7 + GGTT. More recent hardware has dropped
// this explicit setting, where it should now be enabled by default.
//
    I915_CACHE_L3_LLC,
//
// @I915_CACHE_WT:
//
// Write-through. Used for scanout surfaces.
//
// The GPU can utilise the caches, while still having the display engine
// be coherent with GPU writes, as a result we don't need to flush the
// CPU caches when moving out of the render domain. This is the default
// setting chosen by the kernel, if supported by the HW, otherwise we
// fallback to I915_CACHE_NONE. On the CPU side writes through the CPU
// cache still need to be flushed, to remain coherent with the display
// engine.
//
    I915_CACHE_WT,
//
// @I915_MAX_CACHE_LEVEL:
//
// Mark the last entry in the enum. Used for defining cachelevel_to_pat
// array for cache_level to pat translation table.
//
    I915_MAX_CACHE_LEVEL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i915_map_type {
    I915_MAP_WB = 0,
    I915_MAP_WC,

    I915_MAP_FORCE_WB = I915_MAP_WB | I915_MAP_OVERRIDE,
    I915_MAP_FORCE_WC = I915_MAP_WC | I915_MAP_OVERRIDE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i915_mmap_type {
    I915_MMAP_TYPE_GTT = 0,
    I915_MMAP_TYPE_WC,
    I915_MMAP_TYPE_WB,
    I915_MMAP_TYPE_UC,
    I915_MMAP_TYPE_FIXED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_mmap_offset {
    pub vma_node: drm_vma_offset_node,
    pub obj: *mut drm_i915_gem_object,
    pub mmap_type: i915_mmap_type,
    pub offset: rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_object_page_iter {
    pub sg_pos: *mut scatterlist,
    pub /: *mut *mut unsigned int sg_idx; / in pages, but 32bit eek!,
    pub radix: radix_tree_root,
    pub /: *mut *mut mutex lock; / protects this cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_gem_object {
//
// We might have reason to revisit the below since it wastes
// a lot of space for non-ttm gem objects.
// In any case, always use the accessors for the ttm_buffer_object
// when accessing it.
//
    pub base: drm_gem_object,
    pub __do_not_access: ttm_buffer_object,
}

//
// @vma.lock: protect the list/tree of vmas
//
// @vma.list: List of VMAs backed by this object
//
// The VMA on this list are ordered by type, all GGTT vma are
// placed at the head and all ppGTT vma are placed at the tail.
// The different types of GGTT vma are unordered between
// themselves, use the @vma.tree (which has a defined order
// between all VMA) to quickly find an exact match.
//
// @vma.tree: Ordered tree of VMAs backed by this object
//
// All VMA created for this object are placed in the @vma.tree
// for fast retrieval via a binary search in
// i915_vma_instance(). They are also added to @vma.list for
// easy iteration.
//
// @lut_list: List of vma lookup entries in use for this object.
//
// If this object is closed, we need to remove all of its VMA from
// the fast lookup index in associated contexts; @lut_list provides
// this translation from object to context->handles_vma.
//
// @obj_link: Link into @i915_gem_ww_ctx.obj_list
//
// When we lock this object through i915_gem_object_lock() with a
// context, we add it to the list to ensure we can unlock everything
// when i915_gem_ww_ctx_backoff() or i915_gem_ww_ctx_fini() are called.
//
// @shared_resv_from: The object shares the resv from this vm.
//

//
// @client: @i915_drm_client which created the object
//
// @client_link: Link into @i915_drm_client.objects_list
//

//
// Whether the object is currently in the GGTT or any other supported
// fake offset mmap backed by lmem.
//

// Object is allowed to lose its contents on suspend / resume, even if pinned

// Object needs to be restored early using memcpy during resume

//
// Object is likely never accessed by the CPU. This will prioritise the BO to be
// allocated in the non-mappable portion of lmem. This is merely a hint, and if
// dealing with userspace objects the CPU fault handler is free to ignore this.
//

//
// Object is allowed to retain its initial data and will not be cleared on first
// access if used along with I915_BO_ALLOC_USER. This is mainly to keep
// preallocated framebuffer data intact while transitioning it to i915drmfb.
//

//
// @mem_flags - Mutable placement-related flags
//
// These are flags that indicate specifics of the memory region
// the object is currently in. As such they are only stable
// either under the object lock or if the object is pinned.
//

//
// @pat_index: The desired PAT index.
//
// See hardware specification for valid PAT indices for each platform.
// This field replaces the @cache_level that contains a value of enum
// i915_cache_level since PAT indices are being used by both userspace
// and kernel mode driver for caching policy control after GEN12.
// In the meantime platform specific tables are created to translate
// i915_cache_level into pat index, for more details check the macros
// defined i915/i915_pci.c, e.g. TGL_CACHELEVEL.
// For backward compatibility, this field contains values exactly match
// the entries of enum i915_cache_level for pre-GEN12 platforms (See
// LEGACY_CACHELEVEL), so that the PTE encode functions for these
// legacy platforms can stay the same.
//
// @pat_set_by_user: Indicate whether pat_index is set by user space
//
// This field is set to false by default, only set to true if the
// pat_index is set by user space. By design, user space is capable of
// managing caching behavior by setting pat_index, in which case this
// kernel mode driver should never touch the pat_index.
//
// @cache_coherent:
//
// Note: with the change above which replaced @cache_level with pat_index,
// the use of @cache_coherent is limited to the objects created by kernel
// or by userspace without pat index specified.
// Check for @pat_set_by_user to find out if an object has pat index set
// by userspace. The ioctl's to change cache settings have also been
// disabled for the objects with pat index set by userspace. Please don't
// assume @cache_coherent having the flags set as describe here. A helper
// function i915_gem_object_has_cache_level() provides one way to bypass
// the use of this field.
//
// Track whether the pages are coherent with the GPU if reading or
// writing through the CPU caches. The largely depends on the
// @cache_level setting.
//
// On platforms which don't have the shared LLC(HAS_SNOOP), like on Atom
// platforms, coherency must be explicitly requested with some special
// GTT caching bits(see enum i915_cache_level). When enabling coherency
// it does come at a performance and power cost on such platforms. On
// the flip side the kernel does not need to manually flush any buffers
// which need to be coherent with the GPU, if the object is not coherent
// i.e @cache_coherent is zero.
//
// On platforms that share the LLC with the CPU(HAS_LLC), all GT memory
// access will automatically snoop the CPU caches(even with CACHE_NONE).
// The one exception is when dealing with the display engine, like with
// scanout surfaces. To handle this the kernel will always flush the
// surface out of the CPU caches when preparing it for scanout.  Also
// note that since scanout surfaces are only ever read by the display
// engine we only need to care about flushing any writes through the CPU
// cache, reads on the other hand will always be coherent.
//
// Something strange here is why @cache_coherent is not a simple
// boolean, i.e coherent vs non-coherent. The reasoning for this is back
// to the display engine not being fully coherent. As a result scanout
// surfaces will either be marked as I915_CACHE_NONE or I915_CACHE_WT.
// In the case of seeing I915_CACHE_NONE the kernel makes the assumption
// that this is likely a scanout surface, and will set @cache_coherent
// as only I915_BO_CACHE_COHERENT_FOR_READ, on platforms with the shared
// LLC. The kernel uses this to always flush writes through the CPU
// cache as early as possible, where it can, in effect keeping
// @cache_dirty clean, so we can potentially avoid stalling when
// flushing the surface just before doing the scanout.  This does mean
// we might unnecessarily flush non-scanout objects in some places, but
// the default assumption is that all normal objects should be using
// I915_CACHE_LLC, at least on platforms with the shared LLC.
//
// Supported values:
//
// I915_BO_CACHE_COHERENT_FOR_READ:
//
// On shared LLC platforms, we use this for special scanout surfaces,
// where the display engine is not coherent with the CPU cache. As such
// we need to ensure we flush any writes before doing the scanout. As an
// optimisation we try to flush any writes as early as possible to avoid
// stalling later.
//
// Thus for scanout surfaces using I915_CACHE_NONE, on shared LLC
// platforms, we use:
//
// cache_coherent = I915_BO_CACHE_COHERENT_FOR_READ
//
// While for normal objects that are fully coherent, including special
// scanout surfaces marked as I915_CACHE_WT, we use:
//
// cache_coherent = I915_BO_CACHE_COHERENT_FOR_READ |
// I915_BO_CACHE_COHERENT_FOR_WRITE
//
// And then for objects that are not coherent at all we use:
//
// cache_coherent = 0
//
// I915_BO_CACHE_COHERENT_FOR_WRITE:
//
// When writing through the CPU cache, the GPU is still coherent. Note
// that this also implies I915_BO_CACHE_COHERENT_FOR_READ.
//

//
// @cache_dirty:
//
// Note: with the change above which replaced cache_level with pat_index,
// the use of @cache_dirty is limited to the objects created by kernel
// or by userspace without pat index specified.
// Check for @pat_set_by_user to find out if an object has pat index set
// by userspace. The ioctl's to change cache settings have also been
// disabled for the objects with pat_index set by userspace. Please don't
// assume @cache_dirty is set as describe here. Also see helper function
// i915_gem_object_has_cache_level() for possible ways to bypass the use
// of this field.
//
// Track if we are we dirty with writes through the CPU cache for this
// object. As a result reading directly from main memory might yield
// stale data.
//
// This also ties into whether the kernel is tracking the object as
// coherent with the GPU, as per @cache_coherent, as it determines if
// flushing might be needed at various points.
//
// Another part of @cache_dirty is managing flushing when first
// acquiring the pages for system memory, at this point the pages are
// considered foreign, so the default assumption is that the cache is
// dirty, for example the page zeroing done by the kernel might leave
// writes though the CPU cache, or swapping-in, while the actual data in
// main memory is potentially stale.  Note that this is a potential
// security issue when dealing with userspace objects and zeroing. Now,
// whether we actually need apply the big sledgehammer of flushing all
// the pages on acquire depends on if @cache_coherent is marked as
// I915_BO_CACHE_COHERENT_FOR_WRITE, i.e that the GPU will be coherent
// for both reads and writes though the CPU cache.
//
// Note that on shared LLC platforms we still apply the heavy flush for
// I915_CACHE_NONE objects, under the assumption that this is going to
// be used for scanout.
//
// Update: On some hardware there is now also the 'Bypass LLC' MOCS
// entry, which defeats our @cache_coherent tracking, since userspace
// can freely bypass the CPU cache when touching the pages with the GPU,
// where the kernel is completely unaware. On such platform we need
// apply the sledgehammer-on-acquire regardless of the @cache_coherent.
//
// Special care is taken on non-LLC platforms, to prevent potential
// information leak. The driver currently ensures:
//
// 1. All userspace objects, by default, have @cache_level set as
// I915_CACHE_NONE. The only exception is userptr objects, where we
// instead force I915_CACHE_LLC, but we also don't allow userspace to
// ever change the @cache_level for such objects. Another special case
// is dma-buf, which doesn't rely on @cache_dirty, but there we
// always do a forced flush when acquiring the pages, if there is a
// chance that the pages can be read directly from main memory with
// the GPU.
//
// 2. All I915_CACHE_NONE objects have @cache_dirty initially true.
//
// 3. All swapped-out objects(i.e shmem) have @cache_dirty set to
// true.
//
// 4. The @cache_dirty is never freely reset before the initial
// flush, even if userspace adjusts the @cache_level through the
// i915_gem_set_caching_ioctl.
//
// 5. All @cache_dirty objects(including swapped-in) are initially
// flushed with a synchronous call to drm_clflush_sg in
// __i915_gem_object_set_pages. The @cache_dirty can be freely reset
// at this point. All further asynchronous clfushes are never security
// critical, i.e userspace is free to race against itself.
//
// @is_dpt: Object houses a display page table (DPT)
//
// @read_domains: Read memory domains.
//
// These monitor which caches contain read/write data related to the
// object. When transitioning from one set of domains to another,
// the driver is called to ensure that caches are suitably flushed and
// invalidated.
//
// @write_domain: Corresponding unique write memory domain.
//
// Current tiling stride for the object, if it's tiled.

//
// Protects the pages and their use. Do not use directly, but
// instead go through the pin/unpin interfaces.
//
// @shrink_pin: Prevents the pages from being made visible to
// the shrinker, while the shrink_pin is non-zero. Most users
// should pretty much never have to care about this, outside of
// some special use cases.
//
// By default most objects will start out as visible to the
// shrinker(if I915_GEM_OBJECT_IS_SHRINKABLE) as soon as the
// backing pages are attached to the object, like in
// __i915_gem_object_set_pages(). They will then be removed the
// shrinker list once the pages are released.
//
// The @shrink_pin is incremented by calling
// i915_gem_object_make_unshrinkable(), which will also remove
// the object from the shrinker list, if the pin count was zero.
//
// Callers will then typically call
// i915_gem_object_make_shrinkable() or
// i915_gem_object_make_purgeable() to decrement the pin count,
// and make the pages visible again.
//
// @ttm_shrinkable: True when the object is using shmem pages
// underneath. Protected by the object lock.
//
// @unknown_state: Indicate that the object is effectively
// borked. This is write-once and set if we somehow encounter a
// fatal error when moving/clearing the pages, and we are not
// able to fallback to memcpy/memset, like on small-BAR systems.
// The GPU should also be wedged (or in the process) at this
// point.
//
// Only valid to read this after acquiring the dma-resv lock and
// waiting for all DMA_RESV_USAGE_KERNEL fences to be signalled,
// or if we otherwise know that the moving fence has signalled,
// and we are certain the pages underneath are valid for
// immediate access (under normal operation), like just prior to
// binding the object or when setting up the CPU fault handler.
// See i915_gem_object_has_unknown_state();
//
// Priority list of potential placements for this object.
//
// Memory region for this object.
//
// Memory manager resource allocated for this object. Only
// needed for the mock region.
//
// Element within memory_region->objects or region->purgeable
// if the object is marked as DONTNEED. Access is protected by
// region->obj_lock.
//
// Element within i915->mm.shrink_list or i915->mm.purge_list,
// locked by i915->mm.obj_lock.
//
// Advice: are the backing pages purgeable?
//
// This is set if the object has been written to since the
// pages were last acquired.
//
// Record which PXP key instance this object was created against (if
// any), so we can use it to determine if the encryption is valid by
// comparing against the current key instance.
//
// Record of address bit 17 of each page at last unbind.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_userptr {
    pub ptr: uintptr_t,
    pub notifier_seq: c_ulong,
    pub notifier: mmu_interval_notifier,
    pub pvec: *mut page,
    pub page_ref: c_int,
    pub userptr: },

    pub stolen: *mut drm_mm_node,
    pub bo_offset: resource_size_t,
    pub scratch: c_ulong,
    pub encode: u64,
    pub gvt_info: *mut c_void,
}

// Assert that to_intel_bo(NULL) == NULL
extern "C" {
    pub fn container_of(_arg: gem, drm_i915_gem_object: struct, _arg: base) -> return;
}
