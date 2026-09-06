//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv50/head917d.c
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

    static int
    head917d_dither(struct nv50_head *head, struct nv50_head_atom *asyh)
    {
    struct nvif_push *push = &nv50_disp(head.base.base.dev).core.chan.push;
    let mut i: c_int = head.base.index;
    int ret;
    if ((ret = PUSH_WAIT(push, 2)))
    return ret;
    PUSH_MTHD(push, NV917D, HEAD_SET_DITHER_CONTROL(i),
    NVVAL(NV917D, HEAD_SET_DITHER_CONTROL, ENABLE, asyh.dither.enable) |
    NVVAL(NV917D, HEAD_SET_DITHER_CONTROL, BITS, asyh.dither.bits) |
    NVVAL(NV917D, HEAD_SET_DITHER_CONTROL, MODE, asyh.dither.mode) |
    NVVAL(NV917D, HEAD_SET_DITHER_CONTROL, PHASE, 0));
    return 0;
    }
    static int
    head917d_base(struct nv50_head *head, struct nv50_head_atom *asyh)
    {
    struct nvif_push *push = &nv50_disp(head.base.base.dev).core.chan.push;
    let mut i: c_int = head.base.index;
    let mut bounds: u32 = 0;
    int ret;
    if (asyh.base.cpp) {
    switch (asyh.base.cpp) {
    case 8: bounds |= NVDEF(NV917D, HEAD_SET_BASE_CHANNEL_USAGE_BOUNDS, PIXEL_DEPTH, BPP_64); break;
    case 4: bounds |= NVDEF(NV917D, HEAD_SET_BASE_CHANNEL_USAGE_BOUNDS, PIXEL_DEPTH, BPP_32); break;
    case 2: bounds |= NVDEF(NV917D, HEAD_SET_BASE_CHANNEL_USAGE_BOUNDS, PIXEL_DEPTH, BPP_16); break;
    case 1: bounds |= NVDEF(NV917D, HEAD_SET_BASE_CHANNEL_USAGE_BOUNDS, PIXEL_DEPTH, BPP_8); break;
    default:
    WARN_ON(1);
    break;
    }
    bounds |= NVDEF(NV917D, HEAD_SET_BASE_CHANNEL_USAGE_BOUNDS, USABLE, TRUE);
    bounds |= NVDEF(NV917D, HEAD_SET_BASE_CHANNEL_USAGE_BOUNDS, BASE_LUT, USAGE_1025);
    }
    if ((ret = PUSH_WAIT(push, 2)))
    return ret;
    PUSH_MTHD(push, NV917D, HEAD_SET_BASE_CHANNEL_USAGE_BOUNDS(i), bounds);
    return 0;
    }
    static int
    head917d_curs_set(struct nv50_head *head, struct nv50_head_atom *asyh)
    {
    struct nvif_push *push = &nv50_disp(head.base.base.dev).core.chan.push;
    let mut i: c_int = head.base.index;
    int ret;
    ret = PUSH_WAIT(push, 5);
    if (ret)
    return ret;
    PUSH_MTHD(push, NV917D, HEAD_SET_CONTROL_CURSOR(i),
    NVDEF(NV917D, HEAD_SET_CONTROL_CURSOR, ENABLE, ENABLE) |
    NVVAL(NV917D, HEAD_SET_CONTROL_CURSOR, FORMAT, asyh.curs.format) |
    NVVAL(NV917D, HEAD_SET_CONTROL_CURSOR, SIZE, asyh.curs.layout) |
    NVVAL(NV917D, HEAD_SET_CONTROL_CURSOR, HOT_SPOT_X, 0) |
    NVVAL(NV917D, HEAD_SET_CONTROL_CURSOR, HOT_SPOT_Y, 0) |
    NVDEF(NV917D, HEAD_SET_CONTROL_CURSOR, COMPOSITION, ALPHA_BLEND),
    HEAD_SET_OFFSET_CURSOR(i), asyh.curs.offset >> 8);
    PUSH_MTHD(push, NV917D, HEAD_SET_CONTEXT_DMA_CURSOR(i), asyh.curs.handle);
    return 0;
    }
    int
    head917d_curs_layout(struct nv50_head *head, struct nv50_wndw_atom *asyw,
    struct nv50_head_atom *asyh)
    {
    switch (asyw.state.fb.width) {
    case  32: asyh.curs.layout = NV917D_HEAD_SET_CONTROL_CURSOR_SIZE_W32_H32; break;
    case  64: asyh.curs.layout = NV917D_HEAD_SET_CONTROL_CURSOR_SIZE_W64_H64; break;
    case 128: asyh.curs.layout = NV917D_HEAD_SET_CONTROL_CURSOR_SIZE_W128_H128; break;
    case 256: asyh.curs.layout = NV917D_HEAD_SET_CONTROL_CURSOR_SIZE_W256_H256; break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    const struct nv50_head_func
    head917d = {
    .view = head907d_view,
    .mode = head907d_mode,
    .olut = head907d_olut,
    .ilut_check = head907d_ilut_check,
    .olut_size = 1024,
    .olut_set = head907d_olut_set,
    .olut_clr = head907d_olut_clr,
    .core_calc = head507d_core_calc,
    .core_set = head907d_core_set,
    .core_clr = head907d_core_clr,
    .curs_layout = head917d_curs_layout,
    .curs_format = head507d_curs_format,
    .curs_set = head917d_curs_set,
    .curs_clr = head907d_curs_clr,
    .base = head917d_base,
    .ovly = head907d_ovly,
    .dither = head917d_dither,
    .procamp = head907d_procamp,
    .or = head907d_or,
    };
