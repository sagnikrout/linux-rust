//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/vfn/tu102.c
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
// Copyright 2021 Red Hat Inc.
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

    static void
    tu102_vfn_intr_reset(struct nvkm_intr *intr, int leaf, u32 mask)
    {
    struct nvkm_vfn *vfn = container_of(intr, typeof(*vfn), intr);
    nvkm_wr32(vfn.subdev.device, vfn.addr.priv + 0x1000 + (leaf * 4), mask);
    }
    static void
    tu102_vfn_intr_allow(struct nvkm_intr *intr, int leaf, u32 mask)
    {
    struct nvkm_vfn *vfn = container_of(intr, typeof(*vfn), intr);
    nvkm_wr32(vfn.subdev.device, vfn.addr.priv + 0x1200 + (leaf * 4), mask);
    }
    static void
    tu102_vfn_intr_block(struct nvkm_intr *intr, int leaf, u32 mask)
    {
    struct nvkm_vfn *vfn = container_of(intr, typeof(*vfn), intr);
    nvkm_wr32(vfn.subdev.device, vfn.addr.priv + 0x1400 + (leaf * 4), mask);
    }
    static void
    tu102_vfn_intr_rearm(struct nvkm_intr *intr)
    {
    struct nvkm_vfn *vfn = container_of(intr, typeof(*vfn), intr);
    nvkm_wr32(vfn.subdev.device, vfn.addr.priv + 0x1608, 0x0000000f);
    }
    static void
    tu102_vfn_intr_unarm(struct nvkm_intr *intr)
    {
    struct nvkm_vfn *vfn = container_of(intr, typeof(*vfn), intr);
    nvkm_wr32(vfn.subdev.device, vfn.addr.priv + 0x1610, 0x0000000f);
    }
    static bool
    tu102_vfn_intr_pending(struct nvkm_intr *intr)
    {
    struct nvkm_vfn *vfn = container_of(intr, typeof(*vfn), intr);
    struct nvkm_device *device = vfn.subdev.device;
    let mut intr_top: u32 = nvkm_rd32(device, vfn.addr.priv + 0x1600);
    let mut pending: c_int = 0, leaf;
    for (leaf = 0; leaf < 8; leaf++) {
    if (intr_top & BIT(leaf / 2)) {
    intr.stat[leaf] = nvkm_rd32(device, vfn.addr.priv + 0x1000 + (leaf * 4));
    if (intr.stat[leaf])
    pending++;
    } else {
    intr.stat[leaf] = 0;
    }
    }
    return pending != 0;
    }
    const struct nvkm_intr_func
    tu102_vfn_intr = {
    .pending = tu102_vfn_intr_pending,
    .unarm = tu102_vfn_intr_unarm,
    .rearm = tu102_vfn_intr_rearm,
    .block = tu102_vfn_intr_block,
    .allow = tu102_vfn_intr_allow,
    .reset = tu102_vfn_intr_reset,
    };
    static const struct nvkm_vfn_func
    tu102_vfn = {
    .intr = &tu102_vfn_intr,
    .user = { 0x030000, 0x010000, { -1, -1, TURING_USERMODE_A } },
    };
    int
    tu102_vfn_new(struct nvkm_device *device,
    enum nvkm_subdev_type type, int inst, struct nvkm_vfn **pvfn)
    {
    if (nvkm_gsp_rm(device.gsp))
    return r535_vfn_new(&tu102_vfn, device, type, inst, 0xb80000, pvfn);
    return nvkm_vfn_new_(&tu102_vfn, device, type, inst, 0xb80000, pvfn);
    }
