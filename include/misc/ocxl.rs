//! Automatically rewritten from C Header to Rust Module
//! Source: include/misc/ocxl.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017 IBM Corp.

//
// Opencapi drivers all need some common facilities, like parsing the
// device configuration space, adding a Process Element to the Shared
// Process Area, etc...
//
// The ocxl module provides a kernel API, to allow other drivers to
// reuse common code. A bit like a in-kernel library.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_afu_config {
    pub idx: u8,
    pub /: *mut *mut int dvsec_afu_control_pos; / offset of AFU control DVSEC,
    pub name: [c_char; OCXL_AFU_NAME_SZ],
    pub version_major: u8,
    pub version_minor: u8,
    pub afuc_type: u8,
    pub afum_type: u8,
    pub profile: u8,
    pub /: *mut *mut u8 global_mmio_bar; / global MMIO area,
    pub global_mmio_offset: u64,
    pub global_mmio_size: u32,
    pub /: *mut *mut u8 pp_mmio_bar; / per-process MMIO area,
    pub pp_mmio_offset: u64,
    pub pp_mmio_stride: u32,
    pub lpc_mem_offset: u64,
    pub lpc_mem_size: u64,
    pub special_purpose_mem_offset: u64,
    pub special_purpose_mem_size: u64,
    pub pasid_supported_log: u8,
    pub actag_supported: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocxl_fn_config {
    pub /: *mut *mut int dvsec_tl_pos; / offset of the Transaction Layer DVSEC,
    pub /: *mut *mut int dvsec_function_pos; / offset of the Function DVSEC,
    pub /: *mut *mut int dvsec_afu_info_pos; / offset of the AFU information DVSEC,
    pub max_pasid_log: i8,
    pub max_afu_index: i8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocxl_endian {
    OCXL_BIG_ENDIAN = 0,    /**< AFU data is big-endian */
    OCXL_LITTLE_ENDIAN = 1, /**< AFU data is little-endian */
    OCXL_HOST_ENDIAN = 2,   /**< AFU data is the same endianness as the host */
}

// These are opaque outside the ocxl driver
// Device detection & initialisation
//
// ocxl_function_open() - Open an OpenCAPI function on an OpenCAPI device
// @dev: The PCI device that contains the function
//
// Returns an opaque pointer to the function, or an error pointer (check with IS_ERR)
//
// ocxl_function_afu_list() - Get the list of AFUs associated with a PCI function device
// Returns a list of struct ocxl_afu
//
// @fn: The OpenCAPI function containing the AFUs
//
// ocxl_function_fetch_afu() - Fetch an AFU instance from an OpenCAPI function
// @fn: The OpenCAPI function to get the AFU from
// @afu_idx: The index of the AFU to get
//
// If successful, the AFU should be released with ocxl_afu_put()
//
// Returns a pointer to the AFU, or NULL on error
//
// ocxl_afu_get() - Take a reference to an AFU
// @afu: The AFU to increment the reference count on
//
extern "C" {
    pub fn ocxl_afu_get(afu: *mut ocxl_afu);
}
//
// ocxl_afu_put() - Release a reference to an AFU
// @afu: The AFU to decrement the reference count on
//
extern "C" {
    pub fn ocxl_afu_put(afu: *mut ocxl_afu);
}
//
// ocxl_function_config() - Get the configuration information for an OpenCAPI function
// @fn: The OpenCAPI function to get the config for
//
// Returns the function config, or NULL on error
//
// ocxl_function_close() - Close an OpenCAPI function
// This will free any AFUs previously retrieved from the function, and
// detach and associated contexts. The contexts must by freed by the caller.
//
// @fn: The OpenCAPI function to close
//
extern "C" {
    pub fn ocxl_function_close(fn: *mut ocxl_fn);
}
// Context allocation
//
// ocxl_context_alloc() - Allocate an OpenCAPI context
// @context: The OpenCAPI context to allocate, must be freed with ocxl_context_free
// @afu: The AFU the context belongs to
// @mapping: The mapping to unmap when the context is closed (may be NULL)
//
// ocxl_context_free() - Free an OpenCAPI context
// @ctx: The OpenCAPI context to free
//
extern "C" {
    pub fn ocxl_context_free(ctx: *mut ocxl_context);
}
//
// ocxl_context_attach() - Grant access to an MM to an OpenCAPI context
// @ctx: The OpenCAPI context to attach
// @amr: The value of the AMR register to restrict access
// @mm: The mm to attach to the context
//
// Returns 0 on success, negative on failure
//
// ocxl_context_detach() - Detach an MM from an OpenCAPI context
// @ctx: The OpenCAPI context to attach
//
// Returns 0 on success, negative on failure
//
extern "C" {
    pub fn ocxl_context_detach(ctx: *mut ocxl_context) -> c_int;
}
// AFU IRQs
//
// ocxl_afu_irq_alloc() - Allocate an IRQ associated with an AFU context
// @ctx: the AFU context
// @irq_id: out, the IRQ ID
//
// Returns 0 on success, negative on failure
//
extern "C" {
    pub fn ocxl_afu_irq_alloc(ctx: *mut ocxl_context, irq_id: *mut c_int) -> c_int;
}
//
// ocxl_afu_irq_free() - Frees an IRQ associated with an AFU context
// @ctx: the AFU context
// @irq_id: the IRQ ID
//
// Returns 0 on success, negative on failure
//
extern "C" {
    pub fn ocxl_afu_irq_free(ctx: *mut ocxl_context, irq_id: c_int) -> c_int;
}
//
// ocxl_afu_irq_get_addr() - Gets the address of the trigger page for an IRQ
// This can then be provided to an AFU which will write to that
// page to trigger the IRQ.
// @ctx: The AFU context that the IRQ is associated with
// @irq_id: The IRQ ID
//
// returns the trigger page address, or 0 if the IRQ is not valid
//
extern "C" {
    pub fn ocxl_afu_irq_get_addr(ctx: *mut ocxl_context, irq_id: c_int) -> u64;
}
//
// ocxl_irq_set_handler() - Provide a callback to be called when an IRQ is triggered
// @ctx: The AFU context that the IRQ is associated with
// @irq_id: The IRQ ID
// @handler: the callback to be called when the IRQ is triggered
// @free_private: the callback to be called when the IRQ is freed (may be NULL)
// @private: Private data to be passed to the callbacks
//
// Returns 0 on success, negative on failure
//
// AFU Metadata
//
// ocxl_afu_config() - Get a pointer to the config for an AFU
// @afu: a pointer to the AFU to get the config for
//
// Returns a pointer to the AFU config
//
// ocxl_afu_set_private() - Assign opaque hardware specific information to an OpenCAPI AFU.
// @afu: The OpenCAPI AFU
// @private: the opaque hardware specific information to assign to the driver
//
extern "C" {
    pub fn ocxl_afu_set_private(afu: *mut ocxl_afu, private: *mut c_void);
}
//
// ocxl_afu_get_private() - Fetch the hardware specific information associated with
// an external OpenCAPI AFU. This may be consumed by an external OpenCAPI driver.
// @afu: The OpenCAPI AFU
//
// Returns the opaque pointer associated with the device, or NULL if not set
//
// Global MMIO
//
// ocxl_global_mmio_read32() - Read a 32 bit value from global MMIO
// @afu: The AFU
// @offset: The Offset from the start of MMIO
// @endian: the endianness that the MMIO data is in
// @val: returns the value
//
// Returns 0 for success, negative on error
//
// ocxl_global_mmio_read64() - Read a 64 bit value from global MMIO
// @afu: The AFU
// @offset: The Offset from the start of MMIO
// @endian: the endianness that the MMIO data is in
// @val: returns the value
//
// Returns 0 for success, negative on error
//
// ocxl_global_mmio_write32() - Write a 32 bit value to global MMIO
// @afu: The AFU
// @offset: The Offset from the start of MMIO
// @endian: the endianness that the MMIO data is in
// @val: The value to write
//
// Returns 0 for success, negative on error
//
// ocxl_global_mmio_write64() - Write a 64 bit value to global MMIO
// @afu: The AFU
// @offset: The Offset from the start of MMIO
// @endian: the endianness that the MMIO data is in
// @val: The value to write
//
// Returns 0 for success, negative on error
//
// ocxl_global_mmio_set32() - Set bits in a 32 bit global MMIO register
// @afu: The AFU
// @offset: The Offset from the start of MMIO
// @endian: the endianness that the MMIO data is in
// @mask: a mask of the bits to set
//
// Returns 0 for success, negative on error
//
// ocxl_global_mmio_set64() - Set bits in a 64 bit global MMIO register
// @afu: The AFU
// @offset: The Offset from the start of MMIO
// @endian: the endianness that the MMIO data is in
// @mask: a mask of the bits to set
//
// Returns 0 for success, negative on error
//
// ocxl_global_mmio_clear32() - Set bits in a 32 bit global MMIO register
// @afu: The AFU
// @offset: The Offset from the start of MMIO
// @endian: the endianness that the MMIO data is in
// @mask: a mask of the bits to set
//
// Returns 0 for success, negative on error
//
// ocxl_global_mmio_clear64() - Set bits in a 64 bit global MMIO register
// @afu: The AFU
// @offset: The Offset from the start of MMIO
// @endian: the endianness that the MMIO data is in
// @mask: a mask of the bits to set
//
// Returns 0 for success, negative on error
//
// Functions left here are for compatibility with the cxlflash driver
//
// Read the configuration space of a function for the AFU specified by
// the index 'afu_idx'. Fills in a ocxl_afu_config structure
//
// Tell an AFU, by writing in the configuration space, the PASIDs that
// it can use. Range starts at 'pasid_base' and its size is a multiple
// of 2
//
// 'afu_control_offset' is the offset of the AFU control DVSEC which
// can be found in the function configuration
//
// Get the actag configuration for the function:
// 'base' is the first actag value that can be used.
// 'enabled' it the number of actags available, starting from base.
// 'supported' is the total number of actags desired by all the AFUs
// of the function.
//
// Tell a function, by writing in the configuration space, the actags
// it can use.
//
// 'func_offset' is the offset of the Function DVSEC that can found in
// the function configuration
//
// Tell an AFU, by writing in the configuration space, the actags it
// can use.
//
// 'afu_control_offset' is the offset of the AFU control DVSEC for the
// desired AFU. It can be found in the AFU configuration
//
// Enable/disable an AFU, by writing in the configuration space.
//
// 'afu_control_offset' is the offset of the AFU control DVSEC for the
// desired AFU. It can be found in the AFU configuration
//
// Set the Transaction Layer configuration in the configuration space.
// Only needed for function 0.
//
// It queries the host TL capabilities, find some common ground
// between the host and device, and set the Transaction Layer on both
// accordingly.
//
extern "C" {
    pub fn ocxl_config_set_TL(dev: *mut pci_dev, tl_dvsec: c_int) -> c_int;
}
//
// Request an AFU to terminate a PASID.
// Will return once the AFU has acked the request, or an error in case
// of timeout.
//
// The hardware can only terminate one PASID at a time, so caller must
// guarantee some kind of serialization.
//
// 'afu_control_offset' is the offset of the AFU control DVSEC for the
// desired AFU. It can be found in the AFU configuration
//
// Read the configuration space of a function and fill in a
// ocxl_fn_config structure with all the function details
//
// Set up the opencapi link for the function.
//
// When called for the first time for a link, it sets up the Shared
// Process Area for the link and the interrupt handler to process
// translation faults.
//
// Returns a 'link handle' that should be used for further calls for
// the link
//
// Remove the association between the function and its link.
//
extern "C" {
    pub fn ocxl_link_release(dev: *mut pci_dev, link_handle: *mut c_void);
}
//
// Add a Process Element to the Shared Process Area for a link.
// The process is defined by its PASID, pid, tid and its mm_struct.
//
// 'xsl_err_cb' is an optional callback if the driver wants to be
// notified when the translation fault interrupt handler detects an
// address error.
// 'xsl_err_data' is an argument passed to the above callback, if
// defined
//
// Remove a Process Element from the Shared Process Area for a link
//
extern "C" {
    pub fn ocxl_link_remove_pe(link_handle: *mut c_void, pasid: c_int) -> c_int;
}
//
// Allocate an AFU interrupt associated to the link.
//
// 'hw_irq' is the hardware interrupt number
//
extern "C" {
    pub fn ocxl_link_irq_alloc(link_handle: *mut c_void, hw_irq: *mut c_int) -> c_int;
}
//
// Free a previously allocated AFU interrupt
//
extern "C" {
    pub fn ocxl_link_free_irq(link_handle: *mut c_void, hw_irq: c_int);
}
