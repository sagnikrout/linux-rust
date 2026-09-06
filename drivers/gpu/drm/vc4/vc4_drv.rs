//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vc4/vc4_drv.h
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
// Copyright (C) 2015 Broadcom
//

// Don't forget to update vc4_bo.c: bo_type_names[] when adding to
// this.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc4_kernel_bo_type {
// Any kernel allocation (gem_create_object hook) before it
// gets another type set.
//
    VC4_BO_TYPE_KERNEL,
    VC4_BO_TYPE_V3D,
    VC4_BO_TYPE_V3D_SHADER,
    VC4_BO_TYPE_DUMB,
    VC4_BO_TYPE_BIN,
    VC4_BO_TYPE_RCL,
    VC4_BO_TYPE_BCL,
    VC4_BO_TYPE_KERNEL_CACHE,
    VC4_BO_TYPE_COUNT
}

// Performance monitor object. The perform lifetime is controlled by userspace
// using perfmon related ioctls. A perfmon can be attached to a submit_cl
// request, and when this is the case, HW perf counters will be activated just
// before the submit_cl is submitted to the GPU and disabled when the job is
// done. This way, only events related to a specific job will be counted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_perfmon {
    pub dev: *mut vc4_dev,
// Tracks the number of users of the perfmon, when this counter reaches
// zero the perfmon is destroyed.
//
    pub refcnt: refcount_t,
// Number of counters activated in this perfmon instance
// (should be less than DRM_VC4_MAX_PERF_COUNTERS).
//
    pub ncounters: u8,
// Events counted by the HW perf counters.
    pub events: [u8; DRM_VC4_MAX_PERF_COUNTERS],
// Storage for counter values. Counters are incremented by the HW
// perf counter values every time the perfmon is attached to a GPU job.
// This way, perfmon users don't have to retrieve the results after
// each job if they want to track events covering several submissions.
// Note that counter values can't be reset, but you can fake a reset by
// destroying the perfmon and creating a new one.
//
    pub __counted_by(ncounters): u64 counters[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc4_gen {
    VC4_GEN_4,
    VC4_GEN_5,
    VC4_GEN_6_C,
    VC4_GEN_6_D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_dev {
    pub base: drm_device,
    pub dev: *mut device,
    pub gen: vc4_gen,
    pub irq: c_uint,
    pub hvs: *mut vc4_hvs,
    pub v3d: *mut vc4_v3d,
    pub hang_state: *mut vc4_hang_state,
// The kernel-space BO cache.  Tracks buffers that have been
// unreferenced by all other users (refcounts of 0!) but not
// yet freed, so we can do cheap allocations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_bo_cache {
// Array of list heads for entries in the BO cache,
// based on number of pages, so we can do O(1) lookups
// in the cache when allocating.
//
    pub size_list: *mut list_head,
    pub size_list_size: u32,
// List of all BOs in the cache, ordered by age, so we
// can do O(1) lookups when trying to free old
// buffers.
//
    pub time_list: list_head,
    pub time_work: work_struct,
    pub time_timer: timer_list,
    pub bo_cache: },
    pub num_labels: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_label {
    pub name: *const c_char,
    pub num_allocated: u32,
    pub size_allocated: u32,
    pub bo_labels: *mut },
// Protects bo_cache and bo_labels.
    pub bo_lock: mutex,
// Purgeable BO pool. All BOs in this pool can have their memory
// reclaimed if the driver is unable to allocate new BOs. We also
// keep stats related to the purge mechanism here.
//
    pub list: list_head,
    pub num: c_uint,
    pub size: usize,
    pub purged_num: c_uint,
    pub purged_size: usize,
    pub lock: mutex,
    pub purgeable: },
    pub dma_fence_context: u64,
// Sequence number for the last job queued in bin_job_list.
// Starts at 0 (no jobs emitted).
//
    pub emit_seqno: u64,
// Sequence number for the last completed job on the GPU.
// Starts at 0 (no jobs completed).
//
    pub finished_seqno: u64,
// List of all struct vc4_exec_info for jobs to be executed in
// the binner.  The first job in the list is the one currently
// programmed into ct0ca for execution.
//
    pub bin_job_list: list_head,
// List of all struct vc4_exec_info for jobs that have
// completed binning and are ready for rendering.  The first
// job in the list is the one currently programmed into ct1ca
// for execution.
//
    pub render_job_list: list_head,
// List of the finished vc4_exec_infos waiting to be freed by
// job_done_work.
//
    pub job_done_list: list_head,
// Spinlock used to synchronize the job_list and seqno
// accesses between the IRQ handler and GEM ioctls.
//
    pub job_lock: spinlock_t,
    pub job_wait_queue: wait_queue_head_t,
    pub job_done_work: work_struct,
// Used to track the active perfmon if any. Access to this field is
// protected by job_lock.
//
    pub active_perfmon: *mut vc4_perfmon,
// The memory used for storing binner tile alloc, tile state,
// and overflow memory allocations.  This is freed when V3D
// powers down.
//
    pub bin_bo: *mut vc4_bo,
// Size of blocks allocated within bin_bo.
    pub bin_alloc_size: u32,
// Bitmask of the bin_alloc_size chunks in bin_bo that are
// used.
//
    pub bin_alloc_used: u32,
// Bitmask of the current bin_alloc used for overflow memory.
    pub bin_alloc_overflow: u32,
// Incremented when an underrun error happened after an atomic commit.
// This is particularly useful to detect when a specific modeset is too
// demanding in term of memory or HVS bandwidth which is hard to guess
// at atomic check time.
//
    pub underrun: core::sync::atomic::AtomicI32,
    pub overflow_mem_work: work_struct,
    pub power_refcount: c_int,
// Set to true when the load tracker is active.
    pub load_tracker_enabled: bool,
// Mutex controlling the power refcount.
    pub power_lock: mutex,
    pub timer: timer_list,
    pub reset_work: work_struct,
    pub hangcheck: },
    pub ctm_state_lock: drm_modeset_lock,
    pub ctm_manager: drm_private_obj,
    pub hvs_channels: drm_private_obj,
    pub load_tracker: drm_private_obj,
// Mutex for binner bo allocation.
    pub bin_bo_lock: mutex,
// Reference count for our binner bo.
    pub bin_bo_kref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_bo {
    pub base: drm_gem_dma_object,
    pub t_format: bool,
// List entry for the BO's position in either
// vc4_exec_info->unref_list or vc4_dev->bo_cache.time_list
//
    pub unref_head: list_head,
// Time in jiffies when the BO was put in vc4->bo_cache.
    pub free_time: c_ulong,
// List entry for the BO's position in vc4_dev->bo_cache.size_list
    pub size_head: list_head,
// Struct for shader validation state, if created by
// DRM_IOCTL_VC4_CREATE_SHADER_BO.
//
    pub validated_shader: *mut vc4_validated_shader_info,
// One of enum vc4_kernel_bo_type, or VC4_BO_TYPE_COUNT + i
// for user-allocated labels.
//
    pub label: c_int,
// Count the number of active users. This is needed to determine
// whether we can move the BO to the purgeable list or not (when the BO
// is used by the GPU or the display engine we can't purge it).
//
    pub usecnt: refcount_t,
// Store purgeable/purged state here
    pub madv: u32,
    pub madv_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_fence {
    pub base: dma_fence,
    pub dev: *mut drm_device,
// vc4 seqno for signaled() test
    pub seqno: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_v3d {
    pub vc4: *mut vc4_dev,
    pub pdev: *mut platform_device,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub regset: debugfs_regset32,
}

pub const VC4_NUM_UPM_HANDLES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_upm_refcounts {
    pub refcount: refcount_t,
// Allocation size
    pub size: usize,
// Our allocation in UPM for prefetching.
    pub upm: drm_mm_node,
// Pointer back to the HVS structure
    pub hvs: *mut vc4_hvs,
}

pub const HVS_NUM_CHANNELS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_hvs {
    pub vc4: *mut vc4_dev,
    pub pdev: *mut platform_device,
    pub regs: *mut void __iomem,
    pub dlist: *mut u32 __iomem,
    pub dlist_mem_size: c_uint,
    pub core_clk: *mut clk,
    pub disp_clk: *mut clk,
    pub max_core_rate: c_ulong,
// Memory manager for CRTCs to allocate space in the display
// list.  Units are dwords.
//
    pub dlist_mm: drm_mm,
// Memory manager for the LBM memory used by HVS scaling.
    pub lbm_mm: drm_mm,
// Memory manager for the UPM memory used for prefetching.
    pub upm_mm: drm_mm,
    pub upm_handles: ida,
    pub 1]: vc4_upm_refcounts upm_refcounts[VC4_NUM_UPM_HANDLES +,
    pub mm_lock: spinlock_t,
    pub mitchell_netravali_filter: drm_mm_node,
    pub regset: debugfs_regset32,
//
// Even if HDMI0 on the RPi4 can output modes requiring a pixel
// rate higher than 297MHz, it needs some adjustments in the
// config.txt file to be able to do so and thus won't always be
// available.
//
    pub vc5_hdmi_enable_hdmi_20: bool,
//
// 4096x2160@60 requires a core overclock to work, so register
// whether that is sufficient.
//
    pub vc5_hdmi_enable_4096by2160: bool,
}

pub const HVS_NUM_CHANNELS: c_int = 3;
pub const HVS_UBM_WORD_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_hvs_state {
    pub base: drm_private_state,
    pub core_clock_rate: c_ulong,
    pub 1: unsigned in_use:,
    pub fifo_load: c_ulong,
    pub pending_commit: *mut drm_crtc_commit,
    pub fifo_state: [}; HVS_NUM_CHANNELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_plane {
    pub base: drm_plane,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc4_scaling_mode {
    VC4_SCALING_NONE,
    VC4_SCALING_TPZ,
    VC4_SCALING_PPF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_plane_state {
    pub base: drm_plane_state,
// System memory copy of the display list for this element, computed
// at atomic_check time.
//
    pub dlist: *mut u32,
    pub /: *mut *mut u32 dlist_size; / Number of dwords allocated for the display list,
    pub /: *mut *mut u32 dlist_count; / Number of used dwords in the display list.,
// Offset in the dlist to various words, for pageflip or
// cursor updates.
//
    pub pos0_offset: u32,
    pub pos2_offset: u32,
    pub ptr0_offset: [u32; DRM_FORMAT_MAX_PLANES],
    pub lbm_offset: u32,
// Offset where the plane's dlist was last stored in the
// hardware at vc4_crtc_atomic_flush() time.
//
    pub hw_dlist: *mut u32 __iomem,
// Clipped coordinates of the plane on the display.
    pub crtc_h: int crtc_x, crtc_y, crtc_w,,
// Clipped area being scanned from in the FB in u16.16 format
    pub src_y: u32 src_x,,
    pub src_h: [u32 src_w[2],; 2],
// Scaling selection for the RGB/Y plane and the Cb/Cr planes.
    pub y_scaling: [vc4_scaling_mode x_scaling[2],; 2],
    pub is_unity: bool,
    pub is_yuv: bool,
// Our allocation in LBM for temporary storage during scaling.
    pub lbm: drm_mm_node,
// The Unified Pre-Fetcher Handle
    pub upm_handle: [c_uint; DRM_FORMAT_MAX_PLANES],
// Number of lines to pre-fetch
    pub upm_buffer_lines: c_uint,
// Set when the plane has per-pixel alpha content or does not cover
// the entire screen. This is a hint to the CRTC that it might need
// to enable background color fill.
//
    pub needs_bg_fill: bool,
// Mark the dlist as initialized. Useful to avoid initializing it twice
// when async update is not possible.
//
    pub dlist_initialized: bool,
// Load of this plane on the HVS block. The load is expressed in HVS
// cycles/sec.
//
    pub hvs_load: u64,
// Memory bandwidth needed for this plane. This is expressed in
// bytes/sec.
//
    pub membus_load: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc4_encoder_type {
    VC4_ENCODER_TYPE_NONE,
    VC4_ENCODER_TYPE_HDMI0,
    VC4_ENCODER_TYPE_HDMI1,
    VC4_ENCODER_TYPE_VEC,
    VC4_ENCODER_TYPE_DSI0,
    VC4_ENCODER_TYPE_DSI1,
    VC4_ENCODER_TYPE_SMI,
    VC4_ENCODER_TYPE_DPI,
    VC4_ENCODER_TYPE_TXP0,
    VC4_ENCODER_TYPE_TXP1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_encoder {
    pub base: drm_encoder,
    pub type: vc4_encoder_type,
    pub clock_select: u32,
    pub state): *mut *mut *mut void (pre_crtc_configure)(struct drm_encoder encoder, struct drm_atomic_commit,
    pub state): *mut *mut *mut void (pre_crtc_enable)(struct drm_encoder encoder, struct drm_atomic_commit,
    pub state): *mut *mut *mut void (post_crtc_enable)(struct drm_encoder encoder, struct drm_atomic_commit,
    pub state): *mut *mut *mut void (post_crtc_disable)(struct drm_encoder encoder, struct drm_atomic_commit,
    pub state): *mut *mut *mut void (post_crtc_powerdown)(struct drm_encoder encoder, struct drm_atomic_commit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_crtc_data {
    pub name: *const c_char,
    pub debugfs_name: *const c_char,
// Bitmask of channels (FIFOs) of the HVS that the output can source from
    pub hvs_available_channels: c_uint,
// Which output of the HVS this pixelvalve sources from.
    pub hvs_output: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_txp_data {
    pub base: vc4_crtc_data,
    pub encoder_type: vc4_encoder_type,
    pub high_addr_ptr_reg: c_uint,
    pub has_byte_enable:1: c_uint,
    pub size_minus_one:1: c_uint,
    pub supports_40bit_addresses:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_pv_data {
    pub base: vc4_crtc_data,
// Depth of the PixelValve FIFO in bytes
    pub fifo_depth: c_uint,
// Number of pixels output per clock period
    pub pixels_per_clock: u8,
    pub encoder_types: [vc4_encoder_type; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_crtc {
    pub base: drm_crtc,
    pub pdev: *mut platform_device,
    pub data: *const vc4_crtc_data,
    pub regs: *mut void __iomem,
// Timestamp at start of vblank irq - unaffected by lock delays.
    pub t_vblank: ktime_t,
    pub lut_r: [u8; 256],
    pub lut_g: [u8; 256],
    pub lut_b: [u8; 256],
    pub event: *mut drm_pending_vblank_event,
    pub regset: debugfs_regset32,
//
// @feeds_txp: True if the CRTC feeds our writeback controller.
//
    pub feeds_txp: bool,
//
// @irq_lock: Spinlock protecting the resources shared between
// the atomic code and our vblank handler.
//
    pub irq_lock: spinlock_t,
//
// @current_dlist: Start offset of the display list currently
// set in the HVS for that CRTC. Protected by @irq_lock, and
// copied in vc4_hvs_update_dlist() for the CRTC interrupt
// handler to have access to that value.
//
    pub current_dlist: c_uint,
//
// @current_hvs_channel: HVS channel currently assigned to the
// CRTC. Protected by @irq_lock, and copied in
// vc4_hvs_atomic_begin() for the CRTC interrupt handler to have
// access to that value.
//
    pub current_hvs_channel: c_uint,
}

extern "C" {
    pub fn container_of_const(_arg: data, vc4_pv_data: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_crtc_state {
    pub base: drm_crtc_state,
// Dlist area for this CRTC configuration.
    pub mm: drm_mm_node,
    pub txp_armed: bool,
    pub assigned_channel: c_uint,
    pub margins: drm_connector_tv_margins,
    pub hvs_load: c_ulong,
// Transitional state below, only valid during atomic commits
    pub update_muxing: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_exec_info {
    pub dev: *mut vc4_dev,
// Sequence number for this bin/render job.
    pub seqno: u64,
    pub fence: *mut dma_fence,
// Last current addresses the hardware was processing when the
// hangcheck timer checked on us.
//
    pub last_ct1ca: uint32_t last_ct0ca,,
// Kernel-space copy of the ioctl arguments
    pub args: *mut drm_vc4_submit_cl,
// This is the array of BOs that were looked up at the start of exec.
// Command validation will use indices into this array.
//
    pub bo: *mut drm_gem_object,
    pub bo_count: u32,
// List of BOs that are being written by the RCL.  Other than
// the binner temporary storage, this is all the BOs written
// by the job.
//
    pub rcl_write_bo: [*mut drm_gem_dma_object; 4],
    pub rcl_write_bo_count: u32,
// Pointers for our position in vc4->job_list
    pub head: list_head,
// List of other BOs used in the job that need to be released
// once the job is complete.
//
    pub unref_list: list_head,
// Current unvalidated indices into @bo loaded by the non-hardware
// VC4_PACKET_GEM_HANDLES.
//
    pub bo_index: [u32; 2],
// This is the BO where we store the validated command lists, shader
// records, and uniforms.
//
    pub exec_bo: *mut drm_gem_dma_object,
//
// This tracks the per-shader-record state (packet 64) that
// determines the length of the shader record and the offset
// it's expected to be found at.  It gets read in from the
// command lists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_shader_state {
    pub addr: u32,
// Maximum vertex index referenced by any primitive using this
// shader state.
//
    pub max_index: u32,
    pub shader_state: *mut },
// How many shader states the user declared they were using.
    pub shader_state_size: u32,
// How many shader state records the validator has seen.
    pub shader_state_count: u32,
    pub found_tile_binning_mode_config_packet: bool,
    pub found_start_tile_binning_packet: bool,
    pub found_increment_semaphore_packet: bool,
    pub found_flush: bool,
    pub bin_tiles_y: uint8_t bin_tiles_x,,
// Physical address of the start of the tile alloc array
// (where each tile's binned CL will start)
//
    pub tile_alloc_offset: u32,
// Bitmask of which binner slots are freed when this job completes.
    pub bin_slots: u32,
//
// Computed addresses pointing into exec_bo where we start the
// bin thread (ct0) and render thread (ct1).
//
    pub ct0ea: uint32_t ct0ca,,
    pub ct1ea: uint32_t ct1ca,,
// Pointer to the unvalidated bin CL (if present).
    pub bin_u: *mut c_void,
// Pointers to the shader recs.  These paddr gets incremented as CL
// packets are relocated in validate_gl_shader_state, and the vaddrs
// (u and v) get incremented and size decremented as the shader recs
// themselves are validated.
//
    pub shader_rec_u: *mut c_void,
    pub shader_rec_v: *mut c_void,
    pub shader_rec_p: u32,
    pub shader_rec_size: u32,
// Pointers to the uniform data.  These pointers are incremented, and
// size decremented, as each batch of uniforms is uploaded.
//
    pub uniforms_u: *mut c_void,
    pub uniforms_v: *mut c_void,
    pub uniforms_p: u32,
    pub uniforms_size: u32,
// Pointer to a performance monitor object if the user requested it,
// NULL otherwise.
//
    pub perfmon: *mut vc4_perfmon,
// Whether the exec has taken a reference to the binner BO, which should
// happen with a VC4_PACKET_TILE_BINNING_MODE_CONFIG packet.
//
    pub bin_bo_used: bool,
}

// Per-open file private data. Any driver-specific resource that has to be
// released when the DRM file is closed should be placed here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_file {
    pub dev: *mut vc4_dev,
    pub perfmons: xarray,
    pub bin_bo_used: bool,
}

//
// struct vc4_texture_sample_info - saves the offsets into the UBO for texture
// setup parameters.
//
// This will be used at draw time to relocate the reference to the texture
// contents in p0, and validate that the offset combined with
// width/height/stride/etc. from p1 and p2/p3 doesn't sample outside the BO.
// Note that the hardware treats unprovided config parameters as 0, so not all
// of them need to be set up for every texure sample, and we'll store ~0 as
// the offset to mark the unused ones.
//
// See the VC4 3D architecture guide page 41 ("Texture and Memory Lookup Unit
// Setup") for definitions of the texture parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_texture_sample_info {
    pub is_direct: bool,
    pub p_offset: [u32; 4],
}

//
// struct vc4_validated_shader_info - information about validated shaders that
// needs to be used from command list validation.
//
// For a given shader, each time a shader state record references it, we need
// to verify that the shader doesn't read more uniforms than the shader state
// record's uniform BO pointer can provide, and we need to apply relocations
// and validate the shader state record's uniforms that define the texture
// samples.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_validated_shader_info {
    pub uniforms_size: u32,
    pub uniforms_src_size: u32,
    pub num_texture_samples: u32,
    pub texture_samples: *mut vc4_texture_sample_info,
    pub num_uniform_addr_offsets: u32,
    pub uniform_addr_offsets: *mut u32,
    pub is_threaded: bool,
}

//
// __wait_for - magic wait macro
//
// Macro to help avoid open coding check/wait/timeout patterns. Note that it's
// important that we check the condition again after having timed out, since the
// timeout could be due to preemption or similar and we've never had a chance to
// check the condition before the timeout.
//

// Guarantee COND check prior to timeout */		\

// vc4_bo.c
extern "C" {
    pub fn vc4_bo_cache_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn vc4_bo_inc_usecnt(bo: *mut vc4_bo) -> c_int;
}
extern "C" {
    pub fn vc4_bo_dec_usecnt(bo: *mut vc4_bo);
}
extern "C" {
    pub fn vc4_bo_add_to_purgeable_pool(bo: *mut vc4_bo);
}
extern "C" {
    pub fn vc4_bo_remove_from_purgeable_pool(bo: *mut vc4_bo);
}
extern "C" {
    pub fn vc4_bo_debugfs_init(minor: *mut drm_minor) -> c_int;
}
// vc4_crtc.c
extern "C" {
    pub fn vc4_crtc_disable_at_boot(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn vc4_crtc_reset(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn vc4_crtc_handle_vblank(crtc: *mut vc4_crtc);
}
extern "C" {
    pub fn vc4_crtc_send_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn vc4_crtc_late_register(crtc: *mut drm_crtc) -> c_int;
}
// vc4_debugfs.c
extern "C" {
    pub fn vc4_debugfs_init(minor: *mut drm_minor);
}

// vc4_drv.c
extern "C" {
    pub fn vc4_dumb_fixup_args(args: *mut drm_mode_create_dumb) -> c_int;
}
// vc4_dpi.c
// vc4_dsi.c
// vc4_fence.c
// vc4_gem.c
extern "C" {
    pub fn vc4_gem_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn vc4_submit_next_bin_job(dev: *mut drm_device);
}
extern "C" {
    pub fn vc4_submit_next_render_job(dev: *mut drm_device);
}
extern "C" {
    pub fn vc4_move_job_to_render(dev: *mut drm_device, exec: *mut vc4_exec_info);
}
extern "C" {
    pub fn vc4_job_handle_completed(vc4: *mut vc4_dev);
}
// vc4_hdmi.c
// vc4_vec.c
// vc4_txp.c
// vc4_irq.c
extern "C" {
    pub fn vc4_irq_enable(dev: *mut drm_device);
}
extern "C" {
    pub fn vc4_irq_disable(dev: *mut drm_device);
}
extern "C" {
    pub fn vc4_irq_install(dev: *mut drm_device, irq: c_int) -> c_int;
}
extern "C" {
    pub fn vc4_irq_uninstall(dev: *mut drm_device);
}
extern "C" {
    pub fn vc4_irq_reset(dev: *mut drm_device);
}
// vc4_hvs.c
extern "C" {
    pub fn vc4_hvs_stop_channel(hvs: *mut vc4_hvs, output: c_uint);
}
extern "C" {
    pub fn vc4_hvs_get_fifo_from_output(hvs: *mut vc4_hvs, output: c_uint) -> c_int;
}
extern "C" {
    pub fn vc4_hvs_get_fifo_frame_count(hvs: *mut vc4_hvs, fifo: c_uint) -> u8;
}
extern "C" {
    pub fn vc4_hvs_atomic_check(crtc: *mut drm_crtc, state: *mut drm_atomic_commit) -> c_int;
}
extern "C" {
    pub fn vc4_hvs_atomic_begin(crtc: *mut drm_crtc, state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn vc4_hvs_atomic_enable(crtc: *mut drm_crtc, state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn vc4_hvs_atomic_disable(crtc: *mut drm_crtc, state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn vc4_hvs_atomic_flush(crtc: *mut drm_crtc, state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn vc4_hvs_dump_state(hvs: *mut vc4_hvs);
}
extern "C" {
    pub fn vc4_hvs_unmask_underrun(hvs: *mut vc4_hvs, channel: c_int);
}
extern "C" {
    pub fn vc4_hvs_mask_underrun(hvs: *mut vc4_hvs, channel: c_int);
}
extern "C" {
    pub fn vc4_hvs_debugfs_init(minor: *mut drm_minor) -> c_int;
}
// vc4_kms.c
extern "C" {
    pub fn vc4_kms_load(dev: *mut drm_device) -> c_int;
}
// vc4_plane.c
extern "C" {
    pub fn vc4_plane_create_additional_planes(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn vc4_plane_write_dlist(plane: *mut drm_plane, dlist: *mut u32 __iomem) -> u32;
}
extern "C" {
    pub fn vc4_plane_dlist_size(state: *const drm_plane_state) -> u32;
}
// vc4_v3d.c
extern "C" {
    pub fn vc4_v3d_get_bin_slot(vc4: *mut vc4_dev) -> c_int;
}
extern "C" {
    pub fn vc4_v3d_bin_bo_get(vc4: *mut vc4_dev, used: *mut bool) -> c_int;
}
extern "C" {
    pub fn vc4_v3d_bin_bo_put(vc4: *mut vc4_dev);
}
extern "C" {
    pub fn vc4_v3d_pm_get(vc4: *mut vc4_dev) -> c_int;
}
extern "C" {
    pub fn vc4_v3d_pm_put(vc4: *mut vc4_dev);
}
extern "C" {
    pub fn vc4_v3d_debugfs_init(minor: *mut drm_minor) -> c_int;
}
// vc4_validate.c
extern "C" {
    pub fn vc4_get_rcl(dev: *mut drm_device, exec: *mut vc4_exec_info) -> c_int;
}
// vc4_validate_shader.c
// vc4_perfmon.c
extern "C" {
    pub fn vc4_perfmon_get(perfmon: *mut vc4_perfmon);
}
extern "C" {
    pub fn vc4_perfmon_put(perfmon: *mut vc4_perfmon);
}
extern "C" {
    pub fn vc4_perfmon_start(vc4: *mut vc4_dev, perfmon: *mut vc4_perfmon);
}
extern "C" {
    pub fn vc4_perfmon_open_file(vc4file: *mut vc4_file);
}
extern "C" {
    pub fn vc4_perfmon_close_file(vc4file: *mut vc4_file);
}
