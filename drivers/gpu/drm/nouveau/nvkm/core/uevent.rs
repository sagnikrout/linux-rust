//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/core/uevent.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_uevent {
    pub object: nvkm_object,
    pub parent: *mut nvkm_object,
    pub func: nvkm_uevent_func,
    pub wait: bool,
    pub ntfy: nvkm_event_ntfy,
    pub allowed: core::sync::atomic::AtomicI32,
}

    static int
    nvkm_uevent_mthd_block(struct nvkm_uevent *uevent, union nvif_event_block_args *args, u32 argc)
    {
    if (argc != sizeof(args.vn))
    return -ENOSYS;
    nvkm_event_ntfy_block(&uevent.ntfy);
    atomic_set(&uevent.allowed, 0);
    return 0;
    }
    static int
    nvkm_uevent_mthd_allow(struct nvkm_uevent *uevent, union nvif_event_allow_args *args, u32 argc)
    {
    if (argc != sizeof(args.vn))
    return -ENOSYS;
    nvkm_event_ntfy_allow(&uevent.ntfy);
    atomic_set(&uevent.allowed, 1);
    return 0;
    }
    static int
    nvkm_uevent_mthd(struct nvkm_object *object, u32 mthd, void *argv, u32 argc)
    {
    struct nvkm_uevent *uevent = nvkm_uevent(object);
    switch (mthd) {
    case NVIF_EVENT_V0_ALLOW: return nvkm_uevent_mthd_allow(uevent, argv, argc);
    case NVIF_EVENT_V0_BLOCK: return nvkm_uevent_mthd_block(uevent, argv, argc);
    default:
    break;
    }
    return -EINVAL;
    }
    static int
    nvkm_uevent_fini(struct nvkm_object *object, enum nvkm_suspend_state suspend)
    {
    struct nvkm_uevent *uevent = nvkm_uevent(object);
    nvkm_event_ntfy_block(&uevent.ntfy);
    return 0;
    }
    static int
    nvkm_uevent_init(struct nvkm_object *object)
    {
    struct nvkm_uevent *uevent = nvkm_uevent(object);
    if (atomic_read(&uevent.allowed))
    nvkm_event_ntfy_allow(&uevent.ntfy);
    return 0;
    }
    static void *
    nvkm_uevent_dtor(struct nvkm_object *object)
    {
    struct nvkm_uevent *uevent = nvkm_uevent(object);
    nvkm_event_ntfy_del(&uevent.ntfy);
    return uevent;
    }
    static const struct nvkm_object_func
    nvkm_uevent = {
    .dtor = nvkm_uevent_dtor,
    .init = nvkm_uevent_init,
    .fini = nvkm_uevent_fini,
    .mthd = nvkm_uevent_mthd,
    };
    static int
    nvkm_uevent_ntfy(struct nvkm_event_ntfy *ntfy, u32 bits)
    {
    struct nvkm_uevent *uevent = container_of(ntfy, typeof(*uevent), ntfy);
    struct nvkm_client *client = uevent.object.client;
    if (uevent.func)
    return uevent.func(uevent.parent, uevent.object.object, bits);
    return client.event(uevent.object.object, core::ptr::null_mut(), 0);
    }
    int
    nvkm_uevent_add(struct nvkm_uevent *uevent, struct nvkm_event *event, int id, u32 bits,
    nvkm_uevent_func func)
    {
    if (WARN_ON(uevent.func))
    return -EBUSY;
    nvkm_event_ntfy_add(event, id, bits, uevent.wait, nvkm_uevent_ntfy, &uevent.ntfy);
    uevent.func = func;
    return 0;
    }
    int
    nvkm_uevent_new(const struct nvkm_oclass *oclass, void *argv, u32 argc,
    struct nvkm_object **pobject)
    {
    struct nvkm_object *parent = oclass.parent;
    struct nvkm_uevent *uevent;
    union nvif_event_args *args = argv;
    if (argc < sizeof(args.v0) || args.v0.version != 0)
    return -ENOSYS;
    if (!(uevent = kzalloc_obj(*uevent)))
    return -ENOMEM;
// pobject = &uevent->object;
    nvkm_object_ctor(&nvkm_uevent, oclass, &uevent.object);
    uevent.parent = parent;
    uevent.func = core::ptr::null_mut();
    uevent.wait = args.v0.wait;
    uevent.ntfy.event = core::ptr::null_mut();
    return parent.func.uevent(parent, &args.v0.data, argc - sizeof(args.v0), uevent);
    }
