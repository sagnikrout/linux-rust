//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/display_parent_interface.h
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
// Copyright © 2025 Intel Corporation x

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_fb_pin_params {
    pub view: *const i915_gtt_view,
    pub alignment: c_uint,
    pub phys_alignment: c_uint,
    pub vtd_guard: c_uint,
    pub needs_cpu_lmem_access: bool,
    pub needs_low_address: bool,
    pub needs_physical: bool,
    pub needs_fence: bool,
}

// Keep struct definitions sorted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_bo_interface {
    pub /: *mut *mut *mut *mut bool (is_tiled)(struct drm_gem_object obj); / Optional,
    pub /: *mut *mut *mut *mut bool (is_userptr)(struct drm_gem_object obj); / Optional,
    pub /: *mut *mut *mut *mut bool (is_shmem)(struct drm_gem_object obj); / Optional,
    pub obj): *mut *mut bool (is_protected)(struct drm_gem_object,
    pub obj): *mut *mut int (key_check)(struct drm_gem_object,
    pub vma): *mut *mut *mut int (fb_mmap)(struct drm_gem_object obj, struct vm_area_struct,
    pub size): *mut *mut *mut *mut int (read_from_page)(struct drm_gem_object obj, u64 offset, void dst, int,
    pub /: *mut *mut *mut *mut *mut void (describe)(struct seq_file m, struct drm_gem_object obj); / Optional,
    pub mode_cmd): *mut *mut *mut int (framebuffer_init)(struct drm_gem_object obj, struct drm_mode_fb_cmd2,
    pub obj): *mut *mut void (framebuffer_fini)(struct drm_gem_object,
    pub user_mode_cmd): *const drm_mode_fb_cmd2,

    pub size): *mut *mut *mut *mut drm_gem_object (fbdev_create)(drm_device drm, int,
    pub obj): *mut *mut void (fbdev_destroy)(struct drm_gem_object,
    pub vma): *mut *mut *mut *mut int (fbdev_fill_info)(struct drm_gem_object obj, struct fb_info info, struct i915_vma,
    pub stride): *mut *mut u32 (fbdev_pitch_align)(u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_dpt_interface {
    pub size): *mut *mut *mut *mut intel_dpt (create)(drm_gem_object obj, size_t,
    pub dpt): *mut *mut void (destroy)(struct intel_dpt,
    pub dpt): *mut *mut void (suspend)(struct intel_dpt,
    pub dpt): *mut *mut void (resume)(struct intel_dpt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_dsb_interface {
    pub dsb_buf): *mut *mut u32 (ggtt_offset)(struct intel_dsb_buffer,
    pub val): *mut *mut *mut void (write)(struct intel_dsb_buffer dsb_buf, u32 idx, u32,
    pub idx): *mut *mut *mut u32 (read)(struct intel_dsb_buffer dsb_buf, u32,
    pub size): *mut *mut *mut void (fill)(struct intel_dsb_buffer dsb_buf, u32 idx, u32 val, size_t,
    pub size): *mut *mut *mut *mut intel_dsb_buffer (create)(drm_device drm, size_t,
    pub dsb_buf): *mut *mut void (cleanup)(struct intel_dsb_buffer,
    pub dsb_buf): *mut *mut void (flush_map)(struct intel_dsb_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_fb_pin_interface {
    pub out_fence_id): *mut c_int,
    pub fence_id): c_int,
    pub out_offset): *mut u32,
    pub ggtt_vma): *mut i915_vma,
    pub out_offset): *mut u32,
    pub map): *mut *mut *mut void (get_map)(struct i915_vma vma, struct iosys_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_frontbuffer_interface {
    pub obj): *mut *mut *mut intel_frontbuffer (get)(drm_gem_object,
    pub front): *mut *mut void (ref)(struct intel_frontbuffer,
    pub front): *mut *mut void (put)(struct intel_frontbuffer,
    pub front): *mut *mut void (flush_for_display)(struct intel_frontbuffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_hdcp_interface {
    pub msg_out_len): *mut *mut void msg_out, size_t,
    pub drm): *mut *mut bool (gsc_check_status)(struct drm_device,
    pub drm): *mut *mut *mut intel_hdcp_gsc_context (gsc_context_alloc)(drm_device,
    pub gsc_context): *mut *mut void (gsc_context_free)(struct intel_hdcp_gsc_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_initial_plane_interface {
    pub plane_config): *mut *mut *mut *mut drm_gem_object (alloc_obj)(drm_device drm, intel_initial_plane_config,
    pub vma): *mut *mut drm_framebuffer fb, i915_vma,
    pub plane_config): *mut *mut void (config_fini)(struct intel_initial_plane_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_irq_interface {
    pub drm): *mut *mut bool (enabled)(struct drm_device,
    pub drm): *mut *mut void (synchronize)(struct drm_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_overlay_interface {
    pub drm): *mut *mut bool (is_active)(struct drm_device,
    pub frontbuffer_bits): u32,
    pub load_polyphase_filter): bool,
    pub drm): *mut *mut int (overlay_off)(struct drm_device,
    pub drm): *mut *mut int (recover_from_interrupt)(struct drm_device,
    pub drm): *mut *mut int (release_old_vid)(struct drm_device,
    pub drm): *mut *mut void (reset)(struct drm_device,
    pub offset): *mut u32,
    pub vma): *mut i915_vma,
    pub handle): u32,
    pub needs_physical): bool,
    pub drm): *mut *mut void (cleanup)(struct drm_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_panic_interface {
    pub (*alloc)(void): *mut intel_panic,
    pub width)): *mut *mut unsigned int (tiling)(unsigned int x, unsigned int y, unsigned int,
    pub panic): *mut *mut void (finish)(struct intel_panic,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_pc8_interface {
    pub drm): *mut *mut void (block)(struct drm_device,
    pub drm): *mut *mut void (unblock)(struct drm_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_pcode_interface {
    pub val1): *mut *mut *mut *mut int (read)(struct drm_device drm, u32 mbox, u32 val, u32,
    pub timeout_ms): *mut *mut *mut int (write)(struct drm_device drm, u32 mbox, u32 val, int,
    pub timeout_base_ms): u32 reply_mask, u32 reply, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_rpm_interface {
    pub drm): *const *const *const ref_tracker (get)(drm_device,
    pub drm): *const *const *const ref_tracker (get_raw)(drm_device,
    pub drm): *const *const *const ref_tracker (get_if_in_use)(drm_device,
    pub drm): *const *const *const ref_tracker (get_noresume)(drm_device,
    pub wakeref): *const *const *const void (put)(struct drm_device drm, struct ref_tracker,
    pub wakeref): *const *const *const void (put_raw)(struct drm_device drm, struct ref_tracker,
    pub drm): *const *const void (put_unchecked)(struct drm_device,
    pub drm): *const *const bool (suspended)(struct drm_device,
    pub drm): *const *const void (assert_held)(struct drm_device,
    pub drm): *const *const void (assert_block)(struct drm_device,
    pub drm): *const *const void (assert_unblock)(struct drm_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_rps_interface {
    pub fence): *mut *mut void (boost_if_not_started)(struct dma_fence,
    pub interactive): *mut *mut *mut void (mark_interactive)(struct drm_device drm, bool,
    pub drm): *mut *mut void (ilk_irq_handler)(struct drm_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_stolen_interface {
    pub end): unsigned int align, u64 start, u64,
    pub /: *mut *mut *mut *mut int (insert_node)(struct intel_stolen_node node, u64 size, unsigned int align); / Optional,
    pub node): *mut *mut void (remove_node)(struct intel_stolen_node,
    pub drm): *mut *mut bool (initialized)(struct drm_device,
    pub node): *const *const bool (node_allocated)(struct intel_stolen_node,
    pub node): *const *const u64 (node_offset)(struct intel_stolen_node,
    pub /: *mut *mut *mut *mut u64 (area_address)(struct drm_device drm); / Optional,
    pub /: *mut *mut *mut *mut u64 (area_size)(struct drm_device drm); / Optional,
    pub node): *const *const u64 (node_address)(struct intel_stolen_node,
    pub node): *const *const u64 (node_size)(struct intel_stolen_node,
    pub drm): *mut *mut *mut intel_stolen_node (node_alloc)(drm_device,
    pub node): *const *const void (node_free)(struct intel_stolen_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_vlv_iosf_interface {
    pub unit_mask): *mut *mut *mut void (get)(struct drm_device drm, unsigned long,
    pub unit_mask): *mut *mut *mut void (put)(struct drm_device drm, unsigned long,
    pub addr): *mut *mut *mut u32 (read)(struct drm_device drm, enum vlv_iosf_sb_unit unit, u32,
    pub val): *mut *mut *mut int (write)(struct drm_device drm, enum vlv_iosf_sb_unit unit, u32 addr, u32,
}

//
// struct intel_display_parent_interface - services parent driver provides to display
//
// The parent, or core, driver provides a pointer to this structure to display
// driver when calling intel_display_device_probe(). The display driver uses it
// to access services provided by the parent driver. The structure may contain
// sub-struct pointers to group function pointers by functionality.
//
// All function and sub-struct pointers must be initialized and callable unless
// explicitly marked as "optional" below. The display driver will only NULL
// check the optional pointers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_parent_interface {
// @bo: BO interface
    pub bo: *const intel_display_bo_interface,
// @dpt: DPT interface. Optional.
    pub dpt: *const intel_display_dpt_interface,
// @dsb: DSB buffer interface
    pub dsb: *const intel_display_dsb_interface,
// @fb_pin: Framebuffer pin interface
    pub fb_pin: *const intel_display_fb_pin_interface,
// @frontbuffer: Frontbuffer interface
    pub frontbuffer: *const intel_display_frontbuffer_interface,
// @hdcp: HDCP GSC interface
    pub hdcp: *const intel_display_hdcp_interface,
// @initial_plane: Initial plane interface
    pub initial_plane: *const intel_display_initial_plane_interface,
// @irq: IRQ interface
    pub irq: *const intel_display_irq_interface,
// @panic: Panic interface
    pub panic: *const intel_display_panic_interface,
// @overlay: Overlay. Optional.
    pub overlay: *const intel_display_overlay_interface,
// @pc8: PC8 interface. Optional.
    pub pc8: *const intel_display_pc8_interface,
// @pcode: Pcode interface
    pub pcode: *const intel_display_pcode_interface,
// @rpm: Runtime PM functions
    pub rpm: *const intel_display_rpm_interface,
// @rps: RPS interface. Optional.
    pub rps: *const intel_display_rps_interface,
// @stolen: Stolen memory.
    pub stolen: *const intel_display_stolen_interface,
// @vlv_iosf: VLV IOSF sideband. Optional.
    pub vlv_iosf: *const intel_display_vlv_iosf_interface,
// Generic independent functions
// @fence_priority_display: Set display priority. Optional.
    pub fence): *mut *mut void (fence_priority_display)(struct dma_fence,
// @has_auxccs: Are AuxCCS formats supported by the parent. Optional.
    pub drm): *mut *mut bool (has_auxccs)(struct drm_device,
// @has_fenced_regions: Support legacy fencing? Optional.
    pub drm): *mut *mut bool (has_fenced_regions)(struct drm_device,
// @vgpu_active: Is vGPU active? Optional.
    pub drm): *mut *mut bool (vgpu_active)(struct drm_device,
}
