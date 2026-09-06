//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_gem_dma_helper.h
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
// struct drm_gem_dma_object - GEM object backed by DMA memory allocations
// @base: base GEM object
// @dma_addr: DMA address of the backing memory
// @sgt: scatter/gather table for imported PRIME buffers. The table can have
// more than one entry but they are guaranteed to have contiguous
// DMA addresses.
// @vaddr: kernel virtual address of the backing memory
// @map_noncoherent: if true, the GEM object is backed by non-coherent memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_dma_object {
    pub base: drm_gem_object,
    pub dma_addr: dma_addr_t,
    pub sgt: *mut sg_table,
// For objects with DMA memory allocated by GEM DMA
    pub vaddr: *mut c_void,
    pub map_noncoherent: bool,
}

extern "C" {
    pub fn drm_gem_dma_free(dma_obj: *mut drm_gem_dma_object);
}
extern "C" {
    pub fn drm_gem_dma_mmap(dma_obj: *mut drm_gem_dma_object, vma: *mut vm_area_struct) -> c_int;
}
//
// GEM object functions
//
// drm_gem_dma_object_free - GEM object function for drm_gem_dma_free()
// @obj: GEM object to free
//
// This function wraps drm_gem_dma_free_object(). Drivers that employ the DMA helpers
// should use it as their &drm_gem_object_funcs.free handler.
//
// drm_gem_dma_object_print_info() - Print &drm_gem_dma_object info for debugfs
// @p: DRM printer
// @indent: Tab indentation level
// @obj: GEM object
//
// This function wraps drm_gem_dma_print_info(). Drivers that employ the DMA helpers
// should use this function as their &drm_gem_object_funcs.print_info handler.
//
// drm_gem_dma_object_get_sg_table - GEM object function for drm_gem_dma_get_sg_table()
// @obj: GEM object
//
// This function wraps drm_gem_dma_get_sg_table(). Drivers that employ the DMA helpers should
// use it as their &drm_gem_object_funcs.get_sg_table handler.
//
// Returns:
// A pointer to the scatter/gather table of pinned pages or NULL on failure.
//
extern "C" {
    pub fn drm_gem_dma_get_sg_table(_arg: dma_obj) -> return;
}
//
// drm_gem_dma_object_vmap - GEM object function for drm_gem_dma_vmap()
// @obj: GEM object
// @map: Returns the kernel virtual address of the DMA GEM object's backing store.
//
// This function wraps drm_gem_dma_vmap(). Drivers that employ the DMA helpers should
// use it as their &drm_gem_object_funcs.vmap handler.
//
// Returns:
// 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn drm_gem_dma_vmap(_arg: dma_obj, _arg: map) -> return;
}
//
// drm_gem_dma_object_mmap - GEM object function for drm_gem_dma_mmap()
// @obj: GEM object
// @vma: VMA for the area to be mapped
//
// This function wraps drm_gem_dma_mmap(). Drivers that employ the dma helpers should
// use it as their &drm_gem_object_funcs.mmap handler.
//
// Returns:
// 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn drm_gem_dma_mmap(_arg: dma_obj, _arg: vma) -> return;
}
//
// Driver ops
//
// create memory region for DRM framebuffer
//
// DRM_GEM_DMA_DRIVER_OPS_WITH_DUMB_CREATE - DMA GEM driver operations
// @dumb_create_func: callback function for .dumb_create
//
// This macro provides a shortcut for setting the default GEM operations in the
// &drm_driver structure.
//
// This macro is a variant of DRM_GEM_DMA_DRIVER_OPS for drivers that
// override the default implementation of &struct rm_driver.dumb_create. Use
// DRM_GEM_DMA_DRIVER_OPS if possible. Drivers that require a virtual address
// on imported buffers should use
// DRM_GEM_DMA_DRIVER_OPS_VMAP_WITH_DUMB_CREATE() instead.
//

//
// DRM_GEM_DMA_DRIVER_OPS - DMA GEM driver operations
//
// This macro provides a shortcut for setting the default GEM operations in the
// &drm_driver structure.
//
// Drivers that come with their own implementation of
// &struct drm_driver.dumb_create should use
// DRM_GEM_DMA_DRIVER_OPS_WITH_DUMB_CREATE() instead. Use
// DRM_GEM_DMA_DRIVER_OPS if possible. Drivers that require a virtual address
// on imported buffers should use DRM_GEM_DMA_DRIVER_OPS_VMAP instead.
//

//
// DRM_GEM_DMA_DRIVER_OPS_VMAP_WITH_DUMB_CREATE - DMA GEM driver operations
// ensuring a virtual address
// on the buffer
// @dumb_create_func: callback function for .dumb_create
//
// This macro provides a shortcut for setting the default GEM operations in the
// &drm_driver structure for drivers that need the virtual address also on
// imported buffers.
//
// This macro is a variant of DRM_GEM_DMA_DRIVER_OPS_VMAP for drivers that
// override the default implementation of &struct drm_driver.dumb_create. Use
// DRM_GEM_DMA_DRIVER_OPS_VMAP if possible. Drivers that do not require a
// virtual address on imported buffers should use
// DRM_GEM_DMA_DRIVER_OPS_WITH_DUMB_CREATE() instead.
//

//
// DRM_GEM_DMA_DRIVER_OPS_VMAP - DMA GEM driver operations ensuring a virtual
// address on the buffer
//
// This macro provides a shortcut for setting the default GEM operations in the
// &drm_driver structure for drivers that need the virtual address also on
// imported buffers.
//
// Drivers that come with their own implementation of
// &struct drm_driver.dumb_create should use
// DRM_GEM_DMA_DRIVER_OPS_VMAP_WITH_DUMB_CREATE() instead. Use
// DRM_GEM_DMA_DRIVER_OPS_VMAP if possible. Drivers that do not require a
// virtual address on imported buffers should use DRM_GEM_DMA_DRIVER_OPS
// instead.
//

//
// File ops
//

// Macro flag: #define DRM_GEM_DMA_UNMAPPED_AREA_FOPS

//
// DEFINE_DRM_GEM_DMA_FOPS() - macro to generate file operations for DMA drivers
// @name: name for the generated structure
//
// This macro autogenerates a suitable &struct file_operations for DMA based
// drivers, which can be assigned to &drm_driver.fops. Note that this structure
// cannot be shared between drivers, because it contains a reference to the
// current module using THIS_MODULE.
//
// Note that the declaration is already marked as static - if you need a
// non-static version of this you're probably doing it wrong and will break the
// THIS_MODULE reference by accident.
//

