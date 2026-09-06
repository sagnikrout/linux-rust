//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/user.c
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

    void
    nvif_user_dtor(struct nvif_device *device)
    {
    if (device.user.func) {
    nvif_object_dtor(&device.user.object);
    device.user.func = core::ptr::null_mut();
    }
    }
    int
    nvif_user_ctor(struct nvif_device *device, const char *name)
    {
    struct {
    s32 oclass;
    int version;
    const struct nvif_user_func *func;
    } users[] = {
    { BLACKWELL_USERMODE_A, -1, &nvif_userc361 },
    {    HOPPER_USERMODE_A, -1, &nvif_userc361 },
    {    AMPERE_USERMODE_A, -1, &nvif_userc361 },
    {    TURING_USERMODE_A, -1, &nvif_userc361 },
    {     VOLTA_USERMODE_A, -1, &nvif_userc361 },
    {}
    };
    int cid, ret;
    if (device.user.func)
    return 0;
    cid = nvif_mclass(&device.object, users);
    if (cid < 0)
    return cid;
    ret = nvif_object_ctor(&device.object, name ? name : "nvifUsermode",
    0, users[cid].oclass, core::ptr::null_mut(), 0,
    &device.user.object);
    if (ret)
    return ret;
    nvif_object_map(&device.user.object, core::ptr::null_mut(), 0);
    device.user.func = users[cid].func;
    return 0;
    }
