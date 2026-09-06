//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/mmu/memnv04.c
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
// Copyright 2017 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

    int
    nv04_mem_map(struct nvkm_mmu *mmu, struct nvkm_memory *memory, void *argv,
    u32 argc, u64 *paddr, u64 *psize, struct nvkm_vma **pvma)
    {
    union {
    struct nv04_mem_map_vn vn;
    } *args = argv;
    struct nvkm_device *device = mmu.subdev.device;
    let mut addr: u64 = nvkm_memory_addr(memory);
    let mut ret: c_int = -ENOSYS;
    if ((ret = nvif_unvers(ret, &argv, &argc, args.vn)))
    return ret;
// paddr = device->func->resource_addr(device, NVKM_BAR1_FB) + addr;
// psize = nvkm_memory_size(memory);
// pvma = ERR_PTR(-ENODEV);
    return 0;
    }
    int
    nv04_mem_new(struct nvkm_mmu *mmu, int type, u8 page, u64 size,
    void *argv, u32 argc, struct nvkm_memory **pmemory)
    {
    union {
    struct nv04_mem_vn vn;
    } *args = argv;
    let mut ret: c_int = -ENOSYS;
    if ((ret = nvif_unvers(ret, &argv, &argc, args.vn)))
    return ret;
    if (mmu.type[type].type & NVKM_MEM_MAPPABLE)
    type = NVKM_RAM_MM_NORMAL;
    else
    type = NVKM_RAM_MM_NOMAP;
    return nvkm_ram_get(mmu.subdev.device, type, 0x01, page,
    size, true, false, pmemory);
    }
