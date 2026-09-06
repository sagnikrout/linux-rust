//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/pmu/gm200.c
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
// Copyright 2016 Red Hat Inc.
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
// Authors: Ben Skeggs <bskeggs@redhat.com>
//

    static int
    gm200_pmu_flcn_bind_stat(struct nvkm_falcon *falcon, bool intr)
    {
    nvkm_falcon_wr32(falcon, 0x200, 0x0000030e);
    return (nvkm_falcon_rd32(falcon, 0x20c) & 0x00007000) >> 12;
    }
    void
    gm200_pmu_flcn_bind_inst(struct nvkm_falcon *falcon, int target, u64 addr)
    {
    nvkm_falcon_wr32(falcon, 0xe00, 4); /* DMAIDX_UCODE */
    nvkm_falcon_wr32(falcon, 0xe04, 0); /* DMAIDX_VIRT */
    nvkm_falcon_wr32(falcon, 0xe08, 4); /* DMAIDX_PHYS_VID */
    nvkm_falcon_wr32(falcon, 0xe0c, 5); /* DMAIDX_PHYS_SYS_COH */
    nvkm_falcon_wr32(falcon, 0xe10, 6); /* DMAIDX_PHYS_SYS_NCOH */
    nvkm_falcon_mask(falcon, 0x090, 0x00010000, 0x00010000);
    nvkm_falcon_wr32(falcon, 0x480, (1 << 30) | (target << 28) | (addr >> 12));
    }
    const struct nvkm_falcon_func
    gm200_pmu_flcn = {
    .disable = gm200_flcn_disable,
    .enable = gm200_flcn_enable,
    .reset_pmc = true,
    .reset_wait_mem_scrubbing = gm200_flcn_reset_wait_mem_scrubbing,
    .debug = 0xc08,
    .bind_inst = gm200_pmu_flcn_bind_inst,
    .bind_stat = gm200_pmu_flcn_bind_stat,
    .imem_pio = &gm200_flcn_imem_pio,
    .dmem_pio = &gm200_flcn_dmem_pio,
    .start = nvkm_falcon_v1_start,
    .cmdq = { 0x4a0, 0x4b0, 4 },
    .msgq = { 0x4c8, 0x4cc, 0 },
    };
    static const struct nvkm_pmu_func
    gm200_pmu = {
    .flcn = &gm200_pmu_flcn,
    .reset = gf100_pmu_reset,
    };
    int
    gm200_pmu_nofw(struct nvkm_pmu *pmu, int ver, const struct nvkm_pmu_fwif *fwif)
    {
    nvkm_warn(&pmu.subdev, "firmware unavailable\n");
    return 0;
    }
    static const struct nvkm_pmu_fwif
    gm200_pmu_fwif[] = {
    { -1, gm200_pmu_nofw, &gm200_pmu },
    {}
    };
    int
    gm200_pmu_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_pmu **ppmu)
    {
    return nvkm_pmu_new_(gm200_pmu_fwif, device, type, inst, ppmu);
    }
