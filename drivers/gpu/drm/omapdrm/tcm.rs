//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/omapdrm/tcm.h
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


//
// TILER container manager specification and support functions for TI
// TILER driver.
//
// Author: Lajos Molnar <molnar@ti.com>
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
//
// * Neither the name of Texas Instruments Incorporated nor the names of
// its contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
// EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// point
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_pt {
    pub x: u16,
    pub y: u16,
}

// 1d or 2d area
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm_area {
    pub /: *mut *mut bool is2d; / whether area is 1d or 2d,
    pub /: *mut *mut *mut tcm tcm; / parent,
    pub p0: tcm_pt,
    pub p1: tcm_pt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcm {
    pub /: *mut *mut u16 width, height; / container dimensions,
    pub /: *mut *mut int lut_id; / Lookup table identifier,
    pub /: *mut *mut unsigned int y_offset; / offset to use for y coordinates,
    pub lock: spinlock_t,
    pub bitmap: *mut c_ulong,
    pub map_size: usize,
// function table
    pub area): *mut tcm_area,
    pub area): *mut *mut *mut s32 (reserve_1d)(struct tcm tcm, u32 slots, struct tcm_area,
    pub area): *mut *mut *mut s32 (free)(struct tcm tcm, struct tcm_area,
    pub tcm): *mut *mut void (deinit)(struct tcm,
}

// =============================================================================
//
// NOTE:
//
// Since some basic parameter checking is done outside the TCM algorithms,
// TCM implementation do NOT have to check the following:
//
// area pointer is NULL
// width and height fits within container
// number of pages is more than the size of the container
//
// Deinitialize tiler container manager.
//
// @param tcm	Pointer to container manager.
//
// @return 0 on success, non-0 error value on error.  The call
// should free as much memory as possible and meaningful
// even on failure.  Some error codes: -ENODEV: invalid
// manager.
//
// Reserves a 2D area in the container.
//
// @param tcm		Pointer to container manager.
// @param height	Height(in pages) of area to be reserved.
// @param width		Width(in pages) of area to be reserved.
// @param align		Alignment requirement for top-left corner of area. Not
// all values may be supported by the container manager,
// but it must support 0 (1), 32 and 64.
// 0 value is equivalent to 1.
// @param offset	Offset requirement, in bytes.  This is the offset
// from a 4KiB aligned virtual address.
// @param slot_bytes	Width of slot in bytes
// @param area		Pointer to where the reserved area should be stored.
//
// @return 0 on success.  Non-0 error code on failure.  Also,
// the tcm field of the area will be set to NULL on
// failure.  Some error codes: -ENODEV: invalid manager,
// -EINVAL: invalid area, -ENOMEM: not enough space for
// allocation.
//
// perform rudimentary error checking
// align must be a 2 power
//
// Reserves a 1D area in the container.
//
// @param tcm		Pointer to container manager.
// @param slots		Number of (contiguous) slots to reserve.
// @param area		Pointer to where the reserved area should be stored.
//
// @return 0 on success.  Non-0 error code on failure.  Also,
// the tcm field of the area will be set to NULL on
// failure.  Some error codes: -ENODEV: invalid manager,
// -EINVAL: invalid area, -ENOMEM: not enough space for
// allocation.
//
// perform rudimentary error checking
//
// Free a previously reserved area from the container.
//
// @param area	Pointer to area reserved by a prior call to
// tcm_reserve_1d or tcm_reserve_2d call, whether
// it was successful or not. (Note: all fields of
// the structure must match.)
//
// @return 0 on success.  Non-0 error code on failure.  Also, the tcm
// field of the area is set to NULL on success to avoid subsequent
// freeing.  This call will succeed even if supplying
// the area from a failed reserved call.
//
// =============================================================================
//
// This method slices off the topmost 2D slice from the parent area, and stores
// it in the 'slice' parameter.  The 'parent' parameter will get modified to
// contain the remaining portion of the area.  If the whole parent area can
// fit in a 2D slice, its tcm pointer is set to NULL to mark that it is no
// longer a valid area.
//
// @param parent	Pointer to a VALID parent area that will get modified
// @param slice		Pointer to the slice area that will get modified
//
// slice = *parent;
// check if we need to slice
// set end point of slice (start always remains)
// adjust remaining area
// mark this as the last slice
// Verify if a tcm area is logically valid
// coordinate bounds
// 1D coordinate relationship + p0.x check
// 2D coordinate relationship
// see if a coordinate is within an area
// calculate area width
// calculate area height
// calculate number of slots in an area

// limit a 1D area to the first N pages
//
// Iterate through 2D slices of a valid area. Behaves
// syntactically as a for(;;) statement.
//
// @param var		Name of a local variable of type 'struct
// tcm_area *' that will get modified to
// contain each slice.
// @param area		Pointer to the VALID parent area. This
// structure will not get modified
// throughout the loop.
//

