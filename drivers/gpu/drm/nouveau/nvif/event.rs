//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/event.c
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

    int
    nvif_event_block(struct nvif_event *event)
    {
    if (nvif_event_constructed(event)) {
    let mut ret: c_int = nvif_mthd(&event.object, NVIF_EVENT_V0_BLOCK, core::ptr::null_mut(), 0);
    NVIF_ERRON(ret, &event.object, "[BLOCK]");
    return ret;
    }
    return 0;
    }
    int
    nvif_event_allow(struct nvif_event *event)
    {
    if (nvif_event_constructed(event)) {
    let mut ret: c_int = nvif_mthd(&event.object, NVIF_EVENT_V0_ALLOW, core::ptr::null_mut(), 0);
    NVIF_ERRON(ret, &event.object, "[ALLOW]");
    return ret;
    }
    return 0;
    }
    void
    nvif_event_dtor(struct nvif_event *event)
    {
    nvif_object_dtor(&event.object);
    }
    int
    nvif_event_ctor_(struct nvif_object *parent, const char *name, u32 handle, nvif_event_func func,
    bool wait, struct nvif_event_v0 *args, u32 argc, bool warn,
    struct nvif_event *event)
    {
    struct nvif_event_v0 _args;
    int ret;
    if (!args) {
    args = &_args;
    argc = sizeof(_args);
    }
    args.version = 0;
    args.wait = wait;
    ret = nvif_object_ctor(parent, name ?: "nvifEvent", handle,
    NVIF_CLASS_EVENT, args, argc, &event.object);
    NVIF_ERRON(ret && warn, parent, "[NEW EVENT wait:%d size:%zd]",
    args.wait, argc - sizeof(*args));
    if (ret)
    return ret;
    event.func = func;
    return 0;
    }
