//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv50/cursc37a.c
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
    cursc37a_update(struct nv50_wndw *wndw, u32 *interlock)
    {
    struct nvif_object *user = &wndw.wimm.base.user;
    let mut ret: c_int = nvif_chan_wait(&wndw.wimm, 1);
    if (ret == 0)
    NVIF_WR32(user, NVC37A, UPDATE, 0x00000001);
    return ret;
    }
    static int
    cursc37a_point(struct nv50_wndw *wndw, struct nv50_wndw_atom *asyw)
    {
    struct nvif_object *user = &wndw.wimm.base.user;
    let mut ret: c_int = nvif_chan_wait(&wndw.wimm, 1);
    if (ret == 0) {
    NVIF_WR32(user, NVC37A, SET_CURSOR_HOT_SPOT_POINT_OUT(0),
    NVVAL(NVC37A, SET_CURSOR_HOT_SPOT_POINT_OUT, X, asyw.point.x) |
    NVVAL(NVC37A, SET_CURSOR_HOT_SPOT_POINT_OUT, Y, asyw.point.y));
    }
    return ret;
    }
    static const struct nv50_wimm_func
    cursc37a = {
    .point = cursc37a_point,
    .update = cursc37a_update,
    };
    int
    cursc37a_new(struct nouveau_drm *drm, int head, s32 oclass,
    struct nv50_wndw **pwndw)
    {
    return curs507a_new_(&cursc37a, drm, head, oclass,
    0x00000001 << head, pwndw);
    }
