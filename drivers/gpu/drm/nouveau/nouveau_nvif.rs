//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nouveau_nvif.c
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
// Copyright 2014 Red Hat Inc.
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
// Authors: Ben Skeggs <bskeggs@redhat.com>
//
// NVIF client driver - NVKM directly linked
//

    static void
    nvkm_client_unmap(void *priv, void __iomem *ptr, u32 size)
    {
    iounmap(ptr);
    }
    static void __iomem *
    nvkm_client_map(void *priv, u64 handle, u32 size)
    {
    return ioremap(handle, size);
    }
    static int
    nvkm_client_ioctl(void *priv, void *data, u32 size, void **hack)
    {
    return nvkm_ioctl(priv, data, size, hack);
    }
    static int
    nvkm_client_resume(void *priv)
    {
    struct nvkm_client *client = priv;
    return nvkm_object_init(&client.object);
    }
    static int
    nvkm_client_suspend(void *priv, bool runtime)
    {
    struct nvkm_client *client = priv;
    enum nvkm_suspend_state state;
    if (runtime)
    state = NVKM_RUNTIME_SUSPEND;
    else
    state = NVKM_SUSPEND;
    return nvkm_object_fini(&client.object, state);
    }
    static int
    nvkm_client_event(u64 token, void *repv, u32 repc)
    {
    struct nvif_object *object = (void *)(unsigned long)token;
    struct nvif_event *event = container_of(object, typeof(*event), object);
    if (event.func(event, repv, repc) == NVIF_EVENT_KEEP)
    return NVKM_EVENT_KEEP;
    return NVKM_EVENT_DROP;
    }
    static int
    nvkm_client_driver_init(const char *name, u64 device, const char *cfg,
    const char *dbg, void **ppriv)
    {
    return nvkm_client_new(name, device, cfg, dbg, nvkm_client_event,
    (struct nvkm_client **)ppriv);
    }
    const struct nvif_driver
    nvif_driver_nvkm = {
    .name = "nvkm",
    .init = nvkm_client_driver_init,
    .suspend = nvkm_client_suspend,
    .resume = nvkm_client_resume,
    .ioctl = nvkm_client_ioctl,
    .map = nvkm_client_map,
    .unmap = nvkm_client_unmap,
    };
