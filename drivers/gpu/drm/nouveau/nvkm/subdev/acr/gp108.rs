//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/acr/gp108.c
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
// Copyright 2019 Red Hat Inc.
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
    gp108_acr_hsfw_load_bld(struct nvkm_falcon_fw *fw)
    {
    struct flcn_bl_dmem_desc_v2 hsdesc = {
    .ctx_dma = FALCON_DMAIDX_VIRT,
    .code_dma_base = fw.vma.addr,
    .non_sec_code_off = fw.nmem_base,
    .non_sec_code_size = fw.nmem_size,
    .sec_code_off = fw.imem_base,
    .sec_code_size = fw.imem_size,
    .code_entry_point = 0,
    .data_dma_base = fw.vma.addr + fw.dmem_base_img,
    .data_size = fw.dmem_size,
    .argc = 0,
    .argv = 0,
    };
    flcn_bl_dmem_desc_v2_dump(fw.falcon.user, &hsdesc);
    return nvkm_falcon_pio_wr(fw.falcon, (u8 *)&hsdesc, 0, 0, DMEM, 0, sizeof(hsdesc), 0, 0);
    }
    const struct nvkm_falcon_fw_func
    gp108_acr_hsfw_0 = {
    .signature = gm200_flcn_fw_signature,
    .reset = gm200_flcn_fw_reset,
    .load = gm200_flcn_fw_load,
    .load_bld = gp108_acr_hsfw_load_bld,
    .boot = gm200_flcn_fw_boot,
    };
    MODULE_FIRMWARE("nvidia/gp108/acr/unload_bl.bin");
    MODULE_FIRMWARE("nvidia/gp108/acr/ucode_unload.bin");
    static const struct nvkm_acr_hsf_fwif
    gp108_acr_unload_fwif[] = {
    { 0, gm200_acr_hsfw_ctor, &gp108_acr_hsfw_0, NVKM_ACR_HSF_PMU, 0x1d, 0x00000010 },
    {}
    };
    const struct nvkm_falcon_fw_func
    gp108_acr_load_0 = {
    .signature = gm200_flcn_fw_signature,
    .reset = gm200_flcn_fw_reset,
    .setup = gp102_acr_load_setup,
    .load = gm200_flcn_fw_load,
    .load_bld = gp108_acr_hsfw_load_bld,
    .boot = gm200_flcn_fw_boot,
    };
    MODULE_FIRMWARE("nvidia/gp108/acr/bl.bin");
    MODULE_FIRMWARE("nvidia/gp108/acr/ucode_load.bin");
    static const struct nvkm_acr_hsf_fwif
    gp108_acr_load_fwif[] = {
    { 0, gm200_acr_hsfw_ctor, &gp108_acr_load_0, NVKM_ACR_HSF_SEC2, 0, 0x00000010 },
    {}
    };
    static const struct nvkm_acr_func
    gp108_acr = {
    .load = gp108_acr_load_fwif,
    .unload = gp108_acr_unload_fwif,
    .wpr_parse = gp102_acr_wpr_parse,
    .wpr_layout = gp102_acr_wpr_layout,
    .wpr_alloc = gp102_acr_wpr_alloc,
    .wpr_build = gp102_acr_wpr_build,
    .wpr_patch = gp102_acr_wpr_patch,
    .wpr_check = gm200_acr_wpr_check,
    .init = gm200_acr_init,
    };
    static const struct nvkm_acr_fwif
    gp108_acr_fwif[] = {
    {  0, gp102_acr_load, &gp108_acr },
    { -1, gm200_acr_nofw, &gm200_acr },
    {}
    };
    int
    gp108_acr_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_acr **pacr)
    {
    return nvkm_acr_new_(gp108_acr_fwif, device, type, inst, pacr);
    }
