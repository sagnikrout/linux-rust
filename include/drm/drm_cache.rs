//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_cache.h
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
// Copyright 2009 Red Hat Inc.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Authors:
// Dave Airlie <airlied@redhat.com>
//

extern "C" {
    pub fn drm_clflush_pages(pages[]: *mut page, num_pages: c_ulong);
}
extern "C" {
    pub fn drm_clflush_sg(st: *mut sg_table);
}
extern "C" {
    pub fn drm_clflush_virt_range(addr: *mut c_void, length: c_ulong);
}
extern "C" {
    pub fn drm_need_swiotlb(dma_bits: c_int) -> bool;
}

//
// The DRM driver stack is designed to work with cache coherent devices
// only, but permits an optimization to be enabled in some cases, where
// for some buffers, both the CPU and the GPU use uncached mappings,
// removing the need for DMA snooping and allocation in the CPU caches.
//
// The use of uncached GPU mappings relies on the correct implementation
// of the PCIe NoSnoop TLP attribute by the platform, otherwise the GPU
// will use cached mappings nonetheless. On x86 platforms, this does not
// seem to matter, as uncached CPU mappings will snoop the caches in any
// case. However, on ARM and arm64, enabling this optimization on a
// platform where NoSnoop is ignored results in loss of coherency, which
// breaks correct operation of the device. Since we have no way of
// detecting whether NoSnoop works or not, just disable this
// optimization entirely for ARM and arm64.
//

//
// LoongArch maintains cache coherency in hardware, but its WUC attribute
// (Weak-ordered UnCached, which is similar to WC) is out of the scope of
// cache coherency machanism. This means WUC can only used for write-only
// memory regions.
//

extern "C" {
    pub fn drm_memcpy_init_early();
}
