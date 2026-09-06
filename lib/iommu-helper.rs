//! Automatically rewritten from C to Rust
//! Source: lib/iommu-helper.c
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
// IOMMU helper functions for the free area management
//

    unsigned long iommu_area_alloc(unsigned long *map, unsigned long size,
    unsigned long start, unsigned int nr,
    unsigned long shift, unsigned long boundary_size,
    unsigned long align_mask)
    {
    unsigned long index;
// We don't want the last of the limit
    size -= 1;
    again:
    index = bitmap_find_next_zero_area(map, size, start, nr, align_mask);
    if (index < size) {
    if (iommu_is_span_boundary(index, nr, shift, boundary_size)) {
    start = ALIGN(shift + index, boundary_size) - shift;
    goto again;
    }
    bitmap_set(map, index, nr);
    return index;
    }
    return -1;
    }
