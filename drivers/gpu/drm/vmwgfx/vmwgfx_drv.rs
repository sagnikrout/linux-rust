//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_drv.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (c) 2009-2025 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//

//
// FIXME: vmwgfx_drm.h needs to be last due to dependencies.
// uapi headers should not depend on header files outside uapi/.
//

pub const VMWGFX_DRIVER_MAJOR: c_int = 2;
pub const VMWGFX_DRIVER_MINOR: c_int = 21;
pub const VMWGFX_DRIVER_PATCHLEVEL: c_int = 0;

pub const VMWGFX_NUM_DISPLAY_UNITS: c_int = 8;
pub const VMWGFX_CMD_BOUNCE_INIT_SIZE: c_int = 32768;
pub const VMWGFX_MIN_INITIAL_WIDTH: c_int = 1280;
pub const VMWGFX_MIN_INITIAL_HEIGHT: c_int = 800;
pub const VMWGFX_PCI_ID_SVGA2: c_uint = 0x0405;
pub const VMWGFX_PCI_ID_SVGA3: c_uint = 0x0406;
//
// This has to match get_count_order(SVGA_IRQFLAG_MAX)
//
pub const VMWGFX_MAX_NUM_IRQS: c_int = 6;
//
// Perhaps we should have sysfs entries for these.
//
pub const VMWGFX_NUM_GB_CONTEXT: c_int = 256;
pub const VMWGFX_NUM_GB_SHADER: c_int = 20000;
pub const VMWGFX_NUM_GB_SURFACE: c_int = 32768;

pub const VMWGFX_NUM_DXCONTEXT: c_int = 256;
pub const VMWGFX_NUM_DXQUERY: c_int = 512;

pub const VMW_RES_HT_ORDER: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_fpriv {
    pub tfile: *mut ttm_object_file,
    pub /: *mut *mut bool gb_aware; / user-space is guest-backed aware,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmwgfx_hash_item {
    pub head: hlist_node,
    pub key: c_ulong,
}

//
// struct vmw_resource - base class for hardware resources
//
// @kref: For refcounting.
// @dev_priv: Pointer to the device private for this resource. Immutable.
// @id: Device id. Protected by @dev_priv::resource_lock.
// @used_prio: Priority for this resource.
// @guest_memory_size: Guest memory buffer size. Immutable.
// @res_dirty: Resource contains data not yet in the guest memory buffer.
// Protected by resource reserved.
// @guest_memory_dirty: Guest memory buffer contains data not yet in the HW
// resource. Protected by resource reserved.
// @coherent: Emulate coherency by tracking vm accesses.
// @guest_memory_bo: The guest memory buffer if any. Protected by resource
// reserved.
// @guest_memory_offset: Offset into the guest memory buffer if any. Protected
// by resource reserved. Note that only a few resource types can have a
// @guest_memory_offset different from zero.
// @pin_count: The pin count for this resource. A pinned resource has a
// pin-count greater than zero. It is not on the resource LRU lists and its
// guest memory buffer is pinned. Hence it can't be evicted.
// @func: Method vtable for this resource. Immutable.
// @mob_node: Node for the MOB guest memory rbtree. Protected by
// @guest_memory_bo reserved.
// @lru_head: List head for the LRU list. Protected by @dev_priv::resource_lock.
// @binding_head: List head for the context binding list. Protected by
// the @dev_priv::binding_mutex
// @dirty: resource's dirty tracker
// @res_free: The resource destructor.
// @hw_destroy: Callback to destroy the resource on the device, as part of
// resource destruction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_resource {
    pub kref: kref,
    pub dev_priv: *mut vmw_private,
    pub id: c_int,
    pub used_prio: u32,
    pub guest_memory_size: c_ulong,
    pub 1: u32 res_dirty :,
    pub 1: u32 guest_memory_dirty :,
    pub 1: u32 coherent :,
    pub guest_memory_bo: *mut vmw_bo,
    pub guest_memory_offset: c_ulong,
    pub pin_count: c_ulong,
    pub func: *const vmw_res_func,
    pub mob_node: rb_node,
    pub lru_head: list_head,
    pub binding_head: list_head,
    pub dirty: *mut vmw_resource_dirty,
    pub res): *mut *mut void (res_free) (struct vmw_resource,
    pub res): *mut *mut void (hw_destroy) (struct vmw_resource,
}

//
// Resources that are managed using ioctls.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_res_type {
    vmw_res_context,
    vmw_res_surface,
    vmw_res_stream,
    vmw_res_shader,
    vmw_res_dx_context,
    vmw_res_cotable,
    vmw_res_view,
    vmw_res_streamoutput,
    vmw_res_max
}

//
// Resources that are managed using command streams.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_cmdbuf_res_type {
    vmw_cmdbuf_res_shader,
    vmw_cmdbuf_res_view,
    vmw_cmdbuf_res_streamoutput
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_cursor_snooper {
    pub id: usize,
    pub image: *mut u32,
}

//
// struct vmw_surface_metadata - Metadata describing a surface.
//
// @flags: Device flags.
// @format: Surface SVGA3D_x format.
// @mip_levels: Mip level for each face. For GB first index is used only.
// @multisample_count: Sample count.
// @multisample_pattern: Sample patterns.
// @quality_level: Quality level.
// @autogen_filter: Filter for automatically generated mipmaps.
// @array_size: Number of array elements for a 1D/2D texture. For cubemap
// texture number of faces * array_size. This should be 0 for pre
// SM4 device.
// @buffer_byte_stride: Buffer byte stride.
// @num_sizes: Size of @sizes. For GB surface this should always be 1.
// @base_size: Surface dimension.
// @sizes: Array representing mip sizes. Legacy only.
// @scanout: Whether this surface will be used for scanout.
//
// This tracks metadata for both legacy and guest backed surface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_surface_metadata {
    pub flags: u64,
    pub format: u32,
    pub mip_levels: [u32; DRM_VMW_MAX_SURFACE_FACES],
    pub multisample_count: u32,
    pub multisample_pattern: u32,
    pub quality_level: u32,
    pub autogen_filter: u32,
    pub array_size: u32,
    pub num_sizes: u32,
    pub buffer_byte_stride: u32,
    pub base_size: drm_vmw_size,
    pub sizes: *mut drm_vmw_size,
    pub scanout: bool,
}

//
// struct vmw_surface: Resource structure for a surface.
//
// @res: The base resource for this surface.
// @metadata: Metadata for this surface resource.
// @snooper: Cursor data. Legacy surface only.
// @offsets: Legacy surface only.
// @view_list: List of views bound to this surface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_surface {
    pub res: vmw_resource,
    pub metadata: vmw_surface_metadata,
    pub snooper: vmw_cursor_snooper,
    pub offsets: *mut vmw_surface_offset,
    pub view_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_fifo_state {
    pub reserved_size: c_ulong,
    pub dynamic_buffer: *mut u32,
    pub static_buffer: *mut u32,
    pub static_buffer_size: c_ulong,
    pub using_bounce_buffer: bool,
    pub capabilities: u32,
    pub fifo_mutex: mutex,
    pub rwsem: rw_semaphore,
}

//
// struct vmw_res_cache_entry - resource information cache entry
// @handle: User-space handle of a resource.
// @res: Non-ref-counted pointer to the resource.
// @valid_handle: Whether the @handle member is valid.
// @valid: Whether the entry is valid, which also implies that the execbuf
// code holds a reference to the resource, and it's placed on the
// validation list.
//
// Used to avoid frequent repeated user-space handle lookups of the
// same resource.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_res_cache_entry {
    pub handle: u32,
    pub res: *mut vmw_resource,
// private:
    pub private: *mut c_void,
// public:
    pub valid_handle: c_ushort,
    pub valid: c_ushort,
}

//
// enum vmw_dma_map_mode - indicate how to perform TTM page dma mappings.
// @vmw_dma_alloc_coherent: Use TTM coherent pages
// @vmw_dma_map_populate: Unmap from DMA just after unpopulate
// @vmw_dma_map_bind: Unmap from DMA just before unbind
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_dma_map_mode {
    vmw_dma_alloc_coherent,
    vmw_dma_map_populate,
    vmw_dma_map_bind,
// private:
    vmw_dma_map_max
}

//
// struct vmw_sg_table - Scatter/gather table for binding, with additional
// device-specific information.
//
// @mode: which page mapping mode to use
// @pages: Array of page pointers to the pages.
// @addrs: DMA addresses to the pages if coherent pages are used.
// @sgt: Pointer to a struct sg_table with binding information
// @num_pages: Number of @pages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_sg_table {
    pub mode: vmw_dma_map_mode,
    pub pages: *mut page,
    pub addrs: *const dma_addr_t,
    pub sgt: *mut sg_table,
    pub num_pages: c_ulong,
}

//
// struct vmw_piter - Page iterator that iterates over a list of pages
// and DMA addresses that could be either a scatter-gather list or
// arrays
//
// @pages: Array of page pointers to the pages.
// @addrs: DMA addresses to the pages if coherent pages are used.
// @iter: Scatter-gather page iterator. Current position in SG list.
// @i: Current position in arrays.
// @num_pages: Number of pages total.
// @next: Function to advance the iterator. Returns false if past the list
// of pages, true otherwise.
// @dma_address: Function to return the DMA address of the current page.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_piter {
    pub pages: *mut page,
    pub addrs: *const dma_addr_t,
    pub iter: sg_dma_page_iter,
    pub i: c_ulong,
    pub num_pages: c_ulong,
    pub ): *mut *mut bool (next)(struct vmw_piter,
    pub ): *mut *mut dma_addr_t (dma_address)(struct vmw_piter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_ttm_tt {
    pub dma_ttm: ttm_tt,
    pub dev_priv: *mut vmw_private,
    pub gmr_id: c_int,
    pub mob: *mut vmw_mob,
    pub mem_type: c_int,
    pub sgt: sg_table,
    pub vsgt: vmw_sg_table,
    pub mapped: bool,
    pub bound: bool,
}

//
// enum vmw_display_unit_type - Describes the display unit
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_display_unit_type {
    vmw_du_invalid = 0,
    vmw_du_legacy,
    vmw_du_screen_object,
    vmw_du_screen_target,
    vmw_du_max
}

//
// struct vmw_sw_context - Command submission context
// @res_ht: Pointer hash table used to find validation duplicates
// @kernel: Whether the command buffer originates from kernel code rather
// than from user-space
// @fp: If @kernel is false, points to the file of the client. Otherwise
// NULL
// @filp: DRM state for this file
// @cmd_bounce: Command bounce buffer used for command validation before
// copying to fifo space
// @cmd_bounce_size: Current command bounce buffer size
// @cur_query_bo: Current buffer object used as query result buffer
// @bo_relocations: List of buffer object relocations
// @res_relocations: List of resource relocations
// @buf_start: Pointer to start of memory where command validation takes
// place
// @res_cache: Cache of recently looked up resources
// @last_query_ctx: Last context that submitted a query
// @needs_post_query_barrier: Whether a query barrier is needed after
// command submission
// @staged_bindings: Cached per-context binding tracker
// @staged_bindings_inuse: Whether the cached per-context binding tracker
// is in use
// @staged_cmd_res: List of staged command buffer managed resources in this
// command buffer
// @ctx_list: List of context resources referenced in this command buffer
// @dx_ctx_node: Validation metadata of the current DX context
// @dx_query_mob: The MOB used for DX queries
// @dx_query_ctx: The DX context used for the last DX query
// @man: Pointer to the command buffer managed resource manager
// @ctx: The validation context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_sw_context {
    pub VMW_RES_HT_ORDER): DECLARE_HASHTABLE(res_ht,,
    pub kernel: bool,
    pub fp: *mut vmw_fpriv,
    pub filp: *mut drm_file,
    pub cmd_bounce: *mut u32,
    pub cmd_bounce_size: u32,
    pub cur_query_bo: *mut vmw_bo,
    pub bo_relocations: list_head,
    pub res_relocations: list_head,
    pub buf_start: *mut u32,
    pub res_cache: [vmw_res_cache_entry; vmw_res_max],
    pub last_query_ctx: *mut vmw_resource,
    pub needs_post_query_barrier: bool,
    pub staged_bindings: *mut vmw_ctx_binding_state,
    pub staged_bindings_inuse: bool,
    pub staged_cmd_res: list_head,
    pub ctx_list: list_head,
    pub dx_ctx_node: *mut vmw_ctx_validation_info,
    pub dx_query_mob: *mut vmw_bo,
    pub dx_query_ctx: *mut vmw_resource,
    pub man: *mut vmw_cmdbuf_res_manager,
    pub ctx: *mut vmw_validation_context,
}

//
// struct vmw_otable - Guest Memory OBject table metadata
//
// @size:           Size of the table (page-aligned).
// @page_table:     Pointer to a struct vmw_mob holding the page table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_otable {
    pub size: c_ulong,
    pub page_table: *mut vmw_mob,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_otable_batch {
    pub num_otables: unsigned,
    pub otables: *mut vmw_otable,
    pub context: *mut vmw_resource,
    pub otable_bo: *mut vmw_bo,
}

//
// enum vmw_sm_type - Graphics context capability supported by device.
// @VMW_SM_LEGACY: Pre DX context.
// @VMW_SM_4: Context support upto SM4.
// @VMW_SM_4_1: Context support upto SM4_1.
// @VMW_SM_5: Context support up to SM5.
// @VMW_SM_5_1X: Adds support for sm5_1 and gl43 extensions.
// @VMW_SM_MAX: Should be the last.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmw_sm_type {
    VMW_SM_LEGACY = 0,
    VMW_SM_4,
    VMW_SM_4_1,
    VMW_SM_5,
    VMW_SM_5_1X,
    VMW_SM_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_private {
    pub drm: drm_device,
    pub bdev: ttm_device,
    pub pci_id: u32,
    pub io_start: resource_size_t,
    pub vram_start: resource_size_t,
    pub vram_size: resource_size_t,
    pub max_primary_mem: resource_size_t,
    pub rmmio: *mut u32 __iomem,
    pub fifo_mem: *mut u32,
    pub fifo_mem_size: resource_size_t,
    pub fb_max_width: u32,
    pub fb_max_height: u32,
    pub texture_max_width: u32,
    pub texture_max_height: u32,
    pub stdu_max_width: u32,
    pub stdu_max_height: u32,
    pub initial_width: u32,
    pub initial_height: u32,
    pub capabilities: u32,
    pub capabilities2: u32,
    pub max_gmr_ids: u32,
    pub max_gmr_pages: u32,
    pub max_mob_pages: u32,
    pub max_mob_size: u32,
    pub memory_size: u32,
    pub has_gmr: bool,
    pub has_mob: bool,
    pub hw_lock: spinlock_t,
    pub assume_16bpp: bool,
    pub irqs: [u32; VMWGFX_MAX_NUM_IRQS],
    pub num_irq_vectors: u32,
    pub sm_type: vmw_sm_type,
//
// Framebuffer info.
//
    pub active_display_unit: vmw_display_unit_type,
    pub ldu_priv: *mut vmw_legacy_display,
    pub overlay_priv: *mut vmw_overlay,
    pub hotplug_mode_update_property: *mut drm_property,
    pub implicit_placement_property: *mut drm_property,
    pub cursor_lock: spinlock_t,
    pub suspend_state: *mut drm_atomic_commit,
//
// Context and surface management.
//
    pub resource_lock: spinlock_t,
    pub res_idr: [idr; vmw_res_max],
//
// A resource manager for kernel-only surfaces and
// contexts.
//
    pub tdev: *mut ttm_object_device,
//
// Fencing and IRQs.
//
    pub marker_seq: core::sync::atomic::AtomicI32,
    pub fence_queue: wait_queue_head_t,
    pub fifo_queue: wait_queue_head_t,
    pub waiter_lock: spinlock_t,
    pub /: *mut *mut int fence_queue_waiters; / Protected by waiter_lock,
    pub /: *mut *mut int goal_queue_waiters; / Protected by waiter_lock,
    pub /: *mut *mut int cmdbuf_waiters; / Protected by waiter_lock,
    pub /: *mut *mut int error_waiters; / Protected by waiter_lock,
    pub /: *mut *mut int fifo_queue_waiters; / Protected by waiter_lock,
    pub last_read_seqno: core::sync::atomic::AtomicI32,
    pub fman: *mut vmw_fence_manager,
    pub /: *mut *mut uint32_t irq_mask; / Updates protected by waiter_lock,
//
// Device state
//
    pub traces_state: u32,
    pub enable_state: u32,
    pub config_done_state: u32,
//
// Execbuf
//
// Protected by the cmdbuf mutex.
//
    pub ctx: vmw_sw_context,
    pub cmdbuf_mutex: mutex,
    pub binding_mutex: mutex,
//
// PM management.
//
    pub pm_nb: notifier_block,
    pub refuse_hibernation: bool,
    pub suspend_locked: bool,
    pub num_fifo_resources: core::sync::atomic::AtomicI32,
//
// Query processing. These members
// are protected by the cmdbuf mutex.
//
    pub dummy_query_bo: *mut vmw_bo,
    pub pinned_bo: *mut vmw_bo,
    pub query_cid: u32,
    pub query_cid_valid: u32,
    pub dummy_query_bo_pinned: bool,
//
// Surface swapping. The "surface_lru" list is protected by the
// resource lock in order to be able to destroy a surface and take
// it off the lru atomically. "used_memory_size" is currently
// protected by the cmdbuf mutex for simplicity.
//
    pub res_lru: [list_head; vmw_res_max],
    pub used_memory_size: u32,
//
// DMA mapping stuff.
//
    pub map_mode: vmw_dma_map_mode,
//
// Guest Backed stuff
//
    pub otable_batch: vmw_otable_batch,
    pub fifo: *mut vmw_fifo_state,
    pub cman: *mut vmw_cmdbuf_man,
    pub VMW_IRQTHREAD_MAX): DECLARE_BITMAP(irqthread_pending,,
    pub devcaps: *mut uint32,
    pub vkms_enabled: bool,
    pub crc_workq: *mut workqueue_struct,
//
// mksGuestStat instance-descriptor and pid arrays
//
    pub mksstat_user_pages: [*mut page; MKSSTAT_CAPACITY],
    pub mksstat_user_pids: [core::sync::atomic::AtomicI32; MKSSTAT_CAPACITY],    pub mksstat_kern_pages: [*mut page; MKSSTAT_CAPACITY],
    pub mksstat_kern_top_timer: [u8; MKSSTAT_CAPACITY],
    pub mksstat_kern_pids: [core::sync::atomic::AtomicI32; MKSSTAT_CAPACITY],
}

extern "C" {
    pub fn container_of(_arg: res, vmw_surface: struct, _arg: res) -> return;
}
extern "C" {
    pub fn container_of(_arg: dev, vmw_private: struct, _arg: drm) -> return;
}
extern "C" {
    pub fn container_of(_arg: bdev, vmw_private: struct, _arg: bdev) -> return;
}
//
// SVGA v3 has mmio register access and lacks fifo cmds
//
// The locking here is fine-grained, so that it is performed once
// for every read- and write operation. This is of course costly, but we
// don't perform much register access in the timing critical paths anyway.
// Instead we have the extra benefit of being sure that we don't forget
// the hw lock around register accesses.
//
// has_sm4_context - Does the device support SM4 context.
// @dev_priv: Device private.
//
// Return: Bool value if device support SM4 context or not.
//
// has_sm4_1_context - Does the device support SM4_1 context.
// @dev_priv: Device private.
//
// Return: Bool value if device support SM4_1 context or not.
//
// has_sm5_context - Does the device support SM5 context.
// @dev_priv: Device private.
//
// Return: Bool value if device support SM5 context or not.
//
// has_gl43_context - Does the device support GL43 context.
// @dev_priv: Device private.
//
// Return: Bool value if device support SM5 context or not.
//
extern "C" {
    pub fn vmw_svga_enable(dev_priv: *mut vmw_private);
}
extern "C" {
    pub fn vmw_svga_disable(dev_priv: *mut vmw_private);
}
extern "C" {
    pub fn vmwgfx_supported(vmw: *mut vmw_private) -> bool;
}
//
// GMR utilities - vmwgfx_gmr.c
//
extern "C" {
    pub fn vmw_gmr_unbind(dev_priv: *mut vmw_private, gmr_id: c_int);
}
//
// User handles
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_user_object {
    pub surface: *mut vmw_surface,
    pub buffer: *mut vmw_bo,
}

extern "C" {
    pub fn vmw_user_object_unref(uo: *mut vmw_user_object);
}
extern "C" {
    pub fn vmw_user_object_is_null(uo: *mut vmw_user_object) -> bool;
}
extern "C" {
    pub fn vmw_user_object_unmap(uo: *mut vmw_user_object);
}
extern "C" {
    pub fn vmw_user_object_is_mapped(uo: *mut vmw_user_object) -> bool;
}
//
// Resource utilities - vmwgfx_resource.c
//
extern "C" {
    pub fn vmw_resource_unreference(p_res: *mut vmw_resource);
}
extern "C" {
    pub fn vmw_resource_needs_backup(res: *const vmw_resource) -> bool;
}
extern "C" {
    pub fn vmw_query_readback_all(dx_query_mob: *mut vmw_bo) -> c_int;
}
extern "C" {
    pub fn vmw_resource_evict_all(dev_priv: *mut vmw_private);
}
extern "C" {
    pub fn vmw_resource_unbind_list(vbo: *mut vmw_bo);
}
extern "C" {
    pub fn vmw_resource_mob_attach(res: *mut vmw_resource);
}
extern "C" {
    pub fn vmw_resource_mob_detach(res: *mut vmw_resource);
}
extern "C" {
    pub fn vmw_resource_clean(res: *mut vmw_resource) -> c_int;
}
//
// vmw_resource_mob_attached - Whether a resource currently has a mob attached
// @res: The resource
//
// Return: true if the resource has a mob attached, false otherwise.
//
// GEM related functionality - vmwgfx_gem.c
//
extern "C" {
    pub fn vmw_debugfs_gem_init(vdev: *mut vmw_private);
}
//
// Misc Ioctl functionality - vmwgfx_ioctl.c
//
// Fifo utilities - vmwgfx_fifo.c
//
extern "C" {
    pub fn vmw_fifo_destroy(dev_priv: *mut vmw_private);
}
extern "C" {
    pub fn vmw_cmd_supported(vmw: *mut vmw_private) -> bool;
}
extern "C" {
    pub fn vmw_cmd_commit(dev_priv: *mut vmw_private, bytes: u32);
}
extern "C" {
    pub fn vmw_cmd_commit_flush(dev_priv: *mut vmw_private, bytes: u32);
}
extern "C" {
    pub fn vmw_cmd_send_fence(dev_priv: *mut vmw_private, seqno: *mut u32) -> c_int;
}
extern "C" {
    pub fn vmw_supports_3d(dev_priv: *mut vmw_private) -> bool;
}
extern "C" {
    pub fn vmw_fifo_ping_host(dev_priv: *mut vmw_private, reason: u32);
}
extern "C" {
    pub fn vmw_fifo_have_pitchlock(dev_priv: *mut vmw_private) -> bool;
}

//
// vmw_fifo_caps - Get the capabilities of the FIFO command
// queue or 0 if fifo memory isn't present.
// @dev_priv: The device private context
//
// Returns: capabilities of the FIFO command or %0 if fifo memory not present
//
// vmw_is_cursor_bypass3_enabled - check Cursor Bypass 3 enabled setting
// in the FIFO.
// @dev_priv: The device private context
//
// Returns: %true iff Cursor Bypass 3 is enabled in the FIFO
//
// TTM buffer object driver - vmwgfx_ttm_buffer.c
//
// vmw_piter_next - Advance the iterator one page.
//
// @viter: Pointer to the iterator to advance.
//
// Returns: false if past the list of pages, true otherwise.
//
// vmw_piter_dma_addr - Return the DMA address of the current page.
//
// @viter: Pointer to the iterator
//
// Returns: the DMA address of the page pointed to by @viter.
//
// vmw_piter_page - Return a pointer to the current page.
//
// @viter: Pointer to the iterator
//
// Returns: the DMA address of the page pointed to by @viter.
//
// Command submission - vmwgfx_execbuf.c
//
// user_fence_rep,
extern "C" {
    pub fn vmw_execbuf_release_pinned_bo(dev_priv: *mut vmw_private);
}
// user_fence_rep,
extern "C" {
    pub fn vmw_cmd_describe(buf: *const c_void, size: *mut u32, cmd: *const c_char) -> bool;
}
//
// IRQs and wating - vmwgfx_irq.c
//
extern "C" {
    pub fn vmw_irq_install(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_irq_uninstall(dev: *mut drm_device);
}
extern "C" {
    pub fn vmw_seqno_waiter_add(dev_priv: *mut vmw_private) -> bool;
}
extern "C" {
    pub fn vmw_seqno_waiter_remove(dev_priv: *mut vmw_private) -> bool;
}
extern "C" {
    pub fn vmw_goal_waiter_add(dev_priv: *mut vmw_private) -> bool;
}
extern "C" {
    pub fn vmw_goal_waiter_remove(dev_priv: *mut vmw_private) -> bool;
}
//
// Kernel modesetting - vmwgfx_kms.c
//
extern "C" {
    pub fn vmw_kms_init(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_kms_close(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_kms_suspend(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn vmw_kms_resume(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn vmw_kms_lost_device(dev: *mut drm_device);
}
extern "C" {
    pub fn vmw_resource_pin(res: *mut vmw_resource, interruptible: bool) -> c_int;
}
extern "C" {
    pub fn vmw_resource_unpin(res: *mut vmw_resource);
}
extern "C" {
    pub fn vmw_res_type(res: *const vmw_resource) -> vmw_res_type;
}
//
// Overlay control - vmwgfx_overlay.c
//
extern "C" {
    pub fn vmw_overlay_init(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_overlay_close(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_overlay_resume_all(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_overlay_pause_all(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_overlay_claim(dev_priv: *mut vmw_private, out: *mut u32) -> c_int;
}
extern "C" {
    pub fn vmw_overlay_unref(dev_priv: *mut vmw_private, stream_id: u32) -> c_int;
}
extern "C" {
    pub fn vmw_overlay_num_overlays(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_overlay_num_free_overlays(dev_priv: *mut vmw_private) -> c_int;
}
//
// GMR Id manager
//
extern "C" {
    pub fn vmw_gmrid_man_init(dev_priv: *mut vmw_private, type: c_int) -> c_int;
}
extern "C" {
    pub fn vmw_gmrid_man_fini(dev_priv: *mut vmw_private, type: c_int);
}
//
// System memory manager
//
extern "C" {
    pub fn vmw_sys_man_init(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_sys_man_fini(dev_priv: *mut vmw_private);
}
//
// Prime - vmwgfx_prime.c
//
// MemoryOBject management -  vmwgfx_mob.c
//
extern "C" {
    pub fn vmw_mob_destroy(mob: *mut vmw_mob);
}
extern "C" {
    pub fn vmw_otables_setup(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_otables_takedown(dev_priv: *mut vmw_private);
}
//
// Context management - vmwgfx_context.c
//
// Surface management - vmwgfx_surface.c
//
// Shader management - vmwgfx_shader.c
//
// Streamoutput management
//
extern "C" {
    pub fn vmw_dx_streamoutput_set_size(res: *mut vmw_resource, size: u32);
}
//
// Command buffer managed resources - vmwgfx_cmdbuf_res.c
//
extern "C" {
    pub fn vmw_cmdbuf_res_man_destroy(man: *mut vmw_cmdbuf_res_manager);
}
extern "C" {
    pub fn vmw_cmdbuf_res_revert(list: *mut list_head);
}
extern "C" {
    pub fn vmw_cmdbuf_res_commit(list: *mut list_head);
}
//
// COTable management - vmwgfx_cotable.c
//
extern "C" {
    pub fn vmw_cotable_notify(res: *mut vmw_resource, id: c_int) -> c_int;
}
extern "C" {
    pub fn vmw_cotable_scrub(res: *mut vmw_resource, readback: bool) -> c_int;
}
//
// Command buffer managerment vmwgfx_cmdbuf.c
//
extern "C" {
    pub fn vmw_cmdbuf_set_pool_size(man: *mut vmw_cmdbuf_man, size: usize) -> c_int;
}
extern "C" {
    pub fn vmw_cmdbuf_remove_pool(man: *mut vmw_cmdbuf_man);
}
extern "C" {
    pub fn vmw_cmdbuf_man_destroy(man: *mut vmw_cmdbuf_man);
}
extern "C" {
    pub fn vmw_cmdbuf_header_free(header: *mut vmw_cmdbuf_header);
}
extern "C" {
    pub fn vmw_cmdbuf_irqthread(man: *mut vmw_cmdbuf_man);
}
// CPU blit utilities - vmwgfx_blit.c
//
// struct vmw_diff_cpy - CPU blit information structure
//
// @rect: The output bounding box rectangle.
// @line: The current line of the blit.
// @line_offset: Offset of the current line segment.
// @cpp: Bytes per pixel (granularity information).
// @do_cpy: Which memcpy function to use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_diff_cpy {
    pub rect: drm_rect,
    pub line: usize,
    pub line_offset: usize,
    pub cpp: c_int,
    pub n): usize,
}

extern "C" {
    pub fn vmw_memcpy(diff: *mut vmw_diff_cpy, dest: *mut u8, src: *const u8, n: usize);
}
// Host messaging -vmwgfx_msg.c:
extern "C" {
    pub fn vmw_disable_backdoor();
}
// Host mksGuestStats -vmwgfx_msg.c:
extern "C" {
    pub fn vmw_mksstat_get_kern_slot(pid: pid_t, dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_mksstat_remove_all(dev_priv: *mut vmw_private) -> c_int;
}
// VMW logging
//
// VMW_DEBUG_USER - Debug output for user-space debugging.
//
// @fmt: printf() like format string.
//
// This macro is for logging user-space error and debugging messages for e.g.
// command buffer execution errors due to malformed commands, invalid context,
// etc.
//

// Resource dirtying - vmwgfx_page_dirty.c
extern "C" {
    pub fn vmw_bo_is_dirty(vbo: *mut vmw_bo) -> bool;
}
extern "C" {
    pub fn vmw_bo_dirty_scan(vbo: *mut vmw_bo);
}
extern "C" {
    pub fn vmw_bo_dirty_add(vbo: *mut vmw_bo) -> c_int;
}
extern "C" {
    pub fn vmw_bo_dirty_clear(vbo: *mut vmw_bo);
}
extern "C" {
    pub fn vmw_bo_dirty_transfer_to_res(res: *mut vmw_resource);
}
extern "C" {
    pub fn vmw_bo_dirty_clear_res(res: *mut vmw_resource);
}
extern "C" {
    pub fn vmw_bo_dirty_release(vbo: *mut vmw_bo);
}
extern "C" {
    pub fn vmw_bo_vm_fault(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn vmw_bo_vm_mkwrite(vmf: *mut vm_fault) -> vm_fault_t;
}
//
// VMW_DEBUG_KMS - Debug output for kernel mode-setting
// @fmt: format string for the args
//
// This macro is for debugging vmwgfx mode-setting code.
//

//
// Inline helper functions
//
// srf = NULL;
//
// vmw_fifo_mem_read - Perform a MMIO read from the fifo memory
// @vmw: The device private structure
// @fifo_reg: The fifo register to read from
//
// This function is intended to be equivalent to ioread32() on
// memremap'd memory, but without byteswapping.
//
// Returns: the value read
//
extern "C" {
    pub fn READ_ONCE(fifo_reg): *mut *mut (vmw->fifo_mem +) -> return;
}
//
// vmw_fifo_mem_write - Perform a MMIO write to volatile memory
// @vmw: The device private structure
// @fifo_reg: The fifo register to write to
// @value: The value to write
//
// This function is intended to be equivalent to iowrite32 on
// memremap'd memory, but without byteswapping.
//
