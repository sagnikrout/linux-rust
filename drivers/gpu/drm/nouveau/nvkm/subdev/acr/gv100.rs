//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/acr/gv100.c
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
// Copyright 2022 Red Hat Inc.
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

    MODULE_FIRMWARE("nvidia/gv100/acr/unload_bl.bin");
    MODULE_FIRMWARE("nvidia/gv100/acr/ucode_unload.bin");
    static const struct nvkm_acr_hsf_fwif
    gv100_acr_unload_fwif[] = {
    { 0, gm200_acr_hsfw_ctor, &gp108_acr_hsfw_0, NVKM_ACR_HSF_PMU, 0, 0x00000000 },
    {}
    };
    MODULE_FIRMWARE("nvidia/gv100/acr/bl.bin");
    MODULE_FIRMWARE("nvidia/gv100/acr/ucode_load.bin");
    static const struct nvkm_acr_hsf_fwif
    gv100_acr_load_fwif[] = {
    { 0, gm200_acr_hsfw_ctor, &gp108_acr_load_0, NVKM_ACR_HSF_SEC2, 0, 0x00000010 },
    {}
    };
    static const struct nvkm_acr_func
    gv100_acr = {
    .load = gv100_acr_load_fwif,
    .unload = gv100_acr_unload_fwif,
    .wpr_parse = gp102_acr_wpr_parse,
    .wpr_layout = gp102_acr_wpr_layout,
    .wpr_alloc = gp102_acr_wpr_alloc,
    .wpr_build = gp102_acr_wpr_build,
    .wpr_patch = gp102_acr_wpr_patch,
    .wpr_check = gm200_acr_wpr_check,
    .init = gm200_acr_init,
    };
    static const struct nvkm_acr_fwif
    gv100_acr_fwif[] = {
    {  0, gp102_acr_load, &gv100_acr },
    { -1, gm200_acr_nofw, &gm200_acr },
    {}
    };
    int
    gv100_acr_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_acr **pacr)
    {
    return nvkm_acr_new_(gv100_acr_fwif, device, type, inst, pacr);
    }
