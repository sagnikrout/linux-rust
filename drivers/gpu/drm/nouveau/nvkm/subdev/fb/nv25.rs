//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fb/nv25.c
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
// Copyright (C) 2010 Francisco Jerez.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

    static void
    nv25_fb_tile_comp(struct nvkm_fb *fb, int i, u32 size, u32 flags,
    struct nvkm_fb_tile *tile)
    {
    let mut tiles: u32 = DIV_ROUND_UP(size, 0x40);
    let mut tags: u32 = round_up(tiles / fb.ram.parts, 0x40);
    if (!nvkm_mm_head(&fb.tags.mm, 0, 1, tags, tags, 1, &tile.tag)) {
    if (!(flags & 2)) tile.zcomp = 0x00100000; /* Z16 */
    else              tile.zcomp = 0x00200000; /* Z24S8 */
    tile.zcomp |= tile.tag.offset;

    tile.zcomp |= 0x01000000;

    }
    }
    static const struct nvkm_fb_func
    nv25_fb = {
    .tags = nv20_fb_tags,
    .tile.regions = 8,
    .tile.init = nv20_fb_tile_init,
    .tile.comp = nv25_fb_tile_comp,
    .tile.fini = nv20_fb_tile_fini,
    .tile.prog = nv20_fb_tile_prog,
    .ram_new = nv20_ram_new,
    };
    int
    nv25_fb_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst, struct nvkm_fb **pfb)
    {
    return nvkm_fb_new_(&nv25_fb, device, type, inst, pfb);
    }
