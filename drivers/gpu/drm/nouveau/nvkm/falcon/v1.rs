//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/falcon/v1.c
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
// Copyright (c) 2016, NVIDIA CORPORATION. All rights reserved.
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
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

    void
    nvkm_falcon_v1_load_imem(struct nvkm_falcon *falcon, void *data, u32 start,
    u32 size, u16 tag, u8 port, bool secure)
    {
    let mut rem: u8 = size % 4;
    u32 reg;
    int i;
    size -= rem;
    reg = start | BIT(24) | (secure ? BIT(28) : 0);
    nvkm_falcon_wr32(falcon, 0x180 + (port * 16), reg);
    for (i = 0; i < size / 4; i++) {
// write new tag every 256B
    if ((i & 0x3f) == 0)
    nvkm_falcon_wr32(falcon, 0x188 + (port * 16), tag++);
    nvkm_falcon_wr32(falcon, 0x184 + (port * 16), ((u32 *)data)[i]);
    }
//
// If size is not a multiple of 4, mask the last work to ensure garbage
// does not get written
//
    if (rem) {
    let mut extra: u32 = ((u32 *)data)[i];
// write new tag every 256B
    if ((i & 0x3f) == 0)
    nvkm_falcon_wr32(falcon, 0x188 + (port * 16), tag++);
    nvkm_falcon_wr32(falcon, 0x184 + (port * 16),
    extra & (BIT(rem * 8) - 1));
    ++i;
    }
// code must be padded to 0x40 words
    for (; i & 0x3f; i++)
    nvkm_falcon_wr32(falcon, 0x184 + (port * 16), 0);
    }
    void
    nvkm_falcon_v1_load_dmem(struct nvkm_falcon *falcon, void *data, u32 start,
    u32 size, u8 port)
    {
    let mut rem: u8 = size % 4;
    int i;
    size -= rem;
    nvkm_falcon_wr32(falcon, 0x1c0 + (port * 8), start | (0x1 << 24));
    for (i = 0; i < size / 4; i++)
    nvkm_falcon_wr32(falcon, 0x1c4 + (port * 8), ((u32 *)data)[i]);
//
// If size is not a multiple of 4, mask the last word to ensure garbage
// does not get written
//
    if (rem) {
    let mut extra: u32 = ((u32 *)data)[i];
    nvkm_falcon_wr32(falcon, 0x1c4 + (port * 8),
    extra & (BIT(rem * 8) - 1));
    }
    }
    void
    nvkm_falcon_v1_start(struct nvkm_falcon *falcon)
    {
    let mut reg: u32 = nvkm_falcon_rd32(falcon, 0x100);
    if (reg & BIT(6))
    nvkm_falcon_wr32(falcon, 0x130, 0x2);
    else
    nvkm_falcon_wr32(falcon, 0x100, 0x2);
    }
