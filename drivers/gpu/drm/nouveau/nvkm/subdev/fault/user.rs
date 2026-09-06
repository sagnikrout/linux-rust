//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fault/user.c
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
    nvkm_ufault_uevent(struct nvkm_object *object, void *argv, u32 argc, struct nvkm_uevent *uevent)
    {
    struct nvkm_fault_buffer *buffer = nvkm_fault_buffer(object);
    union nvif_clb069_event_args *args = argv;
    if (!uevent)
    return 0;
    if (argc != sizeof(args.vn))
    return -ENOSYS;
    return nvkm_uevent_add(uevent, &buffer.fault.event, buffer.id,
    NVKM_FAULT_BUFFER_EVENT_PENDING, core::ptr::null_mut());
    }
    static int
    nvkm_ufault_map(struct nvkm_object *object, void *argv, u32 argc,
    enum nvkm_object_map *type, u64 *addr, u64 *size)
    {
    struct nvkm_fault_buffer *buffer = nvkm_fault_buffer(object);
    struct nvkm_device *device = buffer.fault.subdev.device;
// type = NVKM_OBJECT_MAP_IO;
// addr = device->func->resource_addr(device, NVKM_BAR2_INST) + buffer->addr;
// size = nvkm_memory_size(buffer->mem);
    return 0;
    }
    static int
    nvkm_ufault_fini(struct nvkm_object *object, enum nvkm_suspend_state suspend)
    {
    struct nvkm_fault_buffer *buffer = nvkm_fault_buffer(object);
    buffer.fault.func.buffer.fini(buffer);
    return 0;
    }
    static int
    nvkm_ufault_init(struct nvkm_object *object)
    {
    struct nvkm_fault_buffer *buffer = nvkm_fault_buffer(object);
    buffer.fault.func.buffer.init(buffer);
    return 0;
    }
    static void *
    nvkm_ufault_dtor(struct nvkm_object *object)
    {
    return core::ptr::null_mut();
    }
    static const struct nvkm_object_func
    nvkm_ufault = {
    .dtor = nvkm_ufault_dtor,
    .init = nvkm_ufault_init,
    .fini = nvkm_ufault_fini,
    .map = nvkm_ufault_map,
    .uevent = nvkm_ufault_uevent,
    };
    int
    nvkm_ufault_new(struct nvkm_device *device, const struct nvkm_oclass *oclass,
    void *argv, u32 argc, struct nvkm_object **pobject)
    {
    union {
    struct nvif_clb069_v0 v0;
    } *args = argv;
    struct nvkm_fault *fault = device.fault;
    struct nvkm_fault_buffer *buffer = fault.buffer[fault.func.user.rp];
    let mut ret: c_int = -ENOSYS;
    if (!(ret = nvif_unpack(ret, &argv, &argc, args.v0, 0, 0, false))) {
    args.v0.entries = buffer.entries;
    args.v0.get = buffer.get;
    args.v0.put = buffer.put;
    } else
    return ret;
    nvkm_object_ctor(&nvkm_ufault, oclass, &buffer.object);
// pobject = &buffer->object;
    return 0;
    }
