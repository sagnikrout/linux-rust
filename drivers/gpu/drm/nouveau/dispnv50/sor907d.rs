//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv50/sor907d.c
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
    sor907d_ctrl(struct nv50_core *core, int or, u32 ctrl,
    struct nv50_head_atom *asyh)
    {
    struct nvif_push *push = &core.chan.push;
    int ret;
    if ((ret = PUSH_WAIT(push, 2)))
    return ret;
    PUSH_MTHD(push, NV907D, SOR_SET_CONTROL(or), ctrl);
    return 0;
    }
    static void
    sor907d_get_caps(struct nv50_disp *disp, struct nouveau_encoder *outp, int or)
    {
    struct nouveau_bo *bo = disp.sync;
    let mut off: c_int = or * 2;
    outp.caps.dp_interlace =
    NVBO_RV32(bo, off, NV907D_CORE_NOTIFIER_3, CAPABILITIES_CAP_SOR0_20, DP_INTERLACE);
    }
    const struct nv50_outp_func
    sor907d = {
    .ctrl = sor907d_ctrl,
    .get_caps = sor907d_get_caps,
    };
