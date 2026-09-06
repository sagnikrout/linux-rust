//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/mmu/vmmgk20a.c
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
    gk20a_vmm_aper(enum nvkm_memory_target target)
    {
    switch (target) {
    case NVKM_MEM_TARGET_NCOH: return 0;
    default:
    return -EINVAL;
    }
    }
    static const struct nvkm_vmm_func
    gk20a_vmm_17 = {
    .join = gf100_vmm_join,
    .part = gf100_vmm_part,
    .aper = gf100_vmm_aper,
    .valid = gf100_vmm_valid,
    .flush = gf100_vmm_flush,
    .invalidate_pdb = gf100_vmm_invalidate_pdb,
    .page = {
    { 17, &gk104_vmm_desc_17_17[0], NVKM_VMM_PAGE_xxHC },
    { 12, &gk104_vmm_desc_17_12[0], NVKM_VMM_PAGE_xxHx },
    {}
    }
    };
    static const struct nvkm_vmm_func
    gk20a_vmm_16 = {
    .join = gf100_vmm_join,
    .part = gf100_vmm_part,
    .aper = gf100_vmm_aper,
    .valid = gf100_vmm_valid,
    .flush = gf100_vmm_flush,
    .invalidate_pdb = gf100_vmm_invalidate_pdb,
    .page = {
    { 16, &gk104_vmm_desc_16_16[0], NVKM_VMM_PAGE_xxHC },
    { 12, &gk104_vmm_desc_16_12[0], NVKM_VMM_PAGE_xxHx },
    {}
    }
    };
    int
    gk20a_vmm_new(struct nvkm_mmu *mmu, bool managed, u64 addr, u64 size,
    void *argv, u32 argc, struct lock_class_key *key,
    const char *name, struct nvkm_vmm **pvmm)
    {
    return gf100_vmm_new_(&gk20a_vmm_16, &gk20a_vmm_17, mmu, managed, addr,
    size, argv, argc, key, name, pvmm);
    }
