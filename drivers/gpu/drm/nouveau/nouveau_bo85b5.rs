//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nouveau_bo85b5.c
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
// Copyright 2007 Dave Airlied
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Dave Airlied <airlied@linux.ie>
// Ben Skeggs   <darktama@iinet.net.au>
// Jeremy Kolb  <jkolb@brandeis.edu>
//

// XXX: Fixup class to be compatible with NVIDIA's, which will allow sharing
// code with KeplerDmaCopyA.
//
    int
    nva3_bo_move_copy(struct nouveau_channel *chan, struct ttm_buffer_object *bo,
    struct ttm_resource *old_reg, struct ttm_resource *new_reg)
    {
    struct nouveau_mem *mem = nouveau_mem(old_reg);
    struct nvif_push *push = &chan.chan.push;
    let mut src_offset: u64 = mem.vma[0].addr;
    let mut dst_offset: u64 = mem.vma[1].addr;
    let mut page_count: u32 = PFN_UP(new_reg.size);
    int ret;
    page_count = PFN_UP(new_reg.size);
    while (page_count) {
    let mut line_count: c_int = (page_count > 8191) ? 8191 : page_count;
    ret = PUSH_WAIT(push, 11);
    if (ret)
    return ret;
    PUSH_NVSQ(push, NV85B5, 0x030c, upper_32_bits(src_offset),
    0x0310, lower_32_bits(src_offset),
    0x0314, upper_32_bits(dst_offset),
    0x0318, lower_32_bits(dst_offset),
    0x031c, PAGE_SIZE,
    0x0320, PAGE_SIZE,
    0x0324, PAGE_SIZE,
    0x0328, line_count);
    PUSH_NVSQ(push, NV85B5, 0x0300, 0x00000110);
    page_count -= line_count;
    src_offset += (PAGE_SIZE * line_count);
    dst_offset += (PAGE_SIZE * line_count);
    }
    return 0;
    }
