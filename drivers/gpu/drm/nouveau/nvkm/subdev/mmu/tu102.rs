//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/mmu/tu102.c
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
// Copyright 2018 Red Hat Inc.
// Copyright 2019 NVIDIA Corporation.
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

    const u8 *
    tu102_mmu_kind(struct nvkm_mmu *mmu, int *count, u8 *invalid)
    {
    static const u8
    kind[16] = {
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, /* 0x00 */
    0x06, 0x06, 0x02, 0x01, 0x03, 0x04, 0x05, 0x07,
    };
// count = ARRAY_SIZE(kind);
// invalid = 0x07;
    return kind;
    }
    static const struct nvkm_mmu_func
    tu102_mmu = {
    .dma_bits = 47,
    .mmu = {{ -1, -1, NVIF_CLASS_MMU_GF100}},
    .mem = {{ -1,  0, NVIF_CLASS_MEM_GF100}, gf100_mem_new, gf100_mem_map },
    .vmm = {{ -1,  0, NVIF_CLASS_VMM_GP100}, tu102_vmm_new },
    .kind = tu102_mmu_kind,
    .kind_sys = true,
    };
    int
    tu102_mmu_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_mmu **pmmu)
    {
    if (nvkm_gsp_rm(device.gsp))
    return r535_mmu_new(&tu102_mmu, device, type, inst, pmmu);
    return nvkm_mmu_new_(&tu102_mmu, device, type, inst, pmmu);
    }
