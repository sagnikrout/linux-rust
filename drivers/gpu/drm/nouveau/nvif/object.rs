//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/object.c
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

    int
    nvif_object_ioctl(struct nvif_object *object, void *data, u32 size, void **hack)
    {
    struct nvif_client *client = object.client;
    union {
    struct nvif_ioctl_v0 v0;
    } *args = data;
    if (size >= sizeof(*args) && args.v0.version == 0) {
    if (object != &client.object)
    args.v0.object = nvif_handle(object);
    else
    args.v0.object = 0;
    } else
    return -ENOSYS;
    return client.driver.ioctl(client.object.priv, data, size, hack);
    }
    void
    nvif_object_sclass_put(struct nvif_sclass **psclass)
    {
    kfree(*psclass);
// psclass = NULL;
    }
    int
    nvif_object_sclass_get(struct nvif_object *object, struct nvif_sclass **psclass)
    {
    struct {
    struct nvif_ioctl_v0_hdr ioctl;
    struct nvif_ioctl_sclass_v0 sclass;
    } *args = core::ptr::null_mut();
    int ret, cnt = 0, i;
    u32 size;
    while (1) {
    size = sizeof(*args) + cnt * sizeof(args.sclass.oclass[0]);
    if (!(args = kmalloc(size, GFP_KERNEL)))
    return -ENOMEM;
    args.ioctl.version = 0;
    args.ioctl.type = NVIF_IOCTL_V0_SCLASS;
    args.sclass.version = 0;
    args.sclass.count = cnt;
    ret = nvif_object_ioctl(object, args, size, core::ptr::null_mut());
    if (ret == 0 && args.sclass.count <= cnt)
    break;
    cnt = args.sclass.count;
    kfree(args);
    if (ret != 0)
    return ret;
    }
// psclass = kzalloc_objs(**psclass, args->sclass.count);
    if (*psclass) {
    for (i = 0; i < args.sclass.count; i++) {
    (*psclass)[i].oclass = args.sclass.oclass[i].oclass;
    (*psclass)[i].minver = args.sclass.oclass[i].minver;
    (*psclass)[i].maxver = args.sclass.oclass[i].maxver;
    }
    ret = args.sclass.count;
    } else {
    ret = -ENOMEM;
    }
    kfree(args);
    return ret;
    }
    int
    nvif_object_mthd(struct nvif_object *object, u32 mthd, void *data, u32 size)
    {
    struct {
    struct nvif_ioctl_v0_hdr ioctl;
    struct nvif_ioctl_mthd_v0 mthd;
    } *args;
    u32 args_size;
    u8 stack[128];
    int ret;
    if (check_add_overflow(sizeof(*args), size, &args_size))
    return -ENOMEM;
    if (args_size > sizeof(stack)) {
    args = kmalloc(args_size, GFP_KERNEL);
    if (!args)
    return -ENOMEM;
    } else {
    args = (void *)stack;
    }
    args.ioctl.version = 0;
    args.ioctl.type = NVIF_IOCTL_V0_MTHD;
    args.mthd.version = 0;
    args.mthd.method = mthd;
    memcpy(args.mthd.data, data, size);
    ret = nvif_object_ioctl(object, args, args_size, core::ptr::null_mut());
    memcpy(data, args.mthd.data, size);
    if (args != (void *)stack)
    kfree(args);
    return ret;
    }
    void
    nvif_object_unmap_handle(struct nvif_object *object)
    {
    struct {
    struct nvif_ioctl_v0_hdr ioctl;
    struct nvif_ioctl_unmap unmap;
    } args = {
    .ioctl.type = NVIF_IOCTL_V0_UNMAP,
    };
    nvif_object_ioctl(object, &args, sizeof(args), core::ptr::null_mut());
    }
    int
    nvif_object_map_handle(struct nvif_object *object, void *argv, u32 argc,
    u64 *handle, u64 *length)
    {
    struct {
    struct nvif_ioctl_v0_hdr ioctl;
    struct nvif_ioctl_map_v0 map;
    } *args;
    let mut argn: u32 = sizeof(*args) + argc;
    int ret, maptype;
    if (!(args = kzalloc(argn, GFP_KERNEL)))
    return -ENOMEM;
    args.ioctl.type = NVIF_IOCTL_V0_MAP;
    memcpy(args.map.data, argv, argc);
    ret = nvif_object_ioctl(object, args, argn, core::ptr::null_mut());
// handle = args->map.handle;
// length = args->map.length;
    maptype = args.map.type;
    kfree(args);
    return ret ? ret : (maptype == NVIF_IOCTL_MAP_V0_IO);
    }
    void
    nvif_object_unmap(struct nvif_object *object)
    {
    struct nvif_client *client = object.client;
    if (object.map.ptr) {
    if (object.map.size) {
    client.driver.unmap(client, object.map.ptr,
    object.map.size);
    object.map.size = 0;
    }
    object.map.ptr = core::ptr::null_mut();
    nvif_object_unmap_handle(object);
    }
    }
    int
    nvif_object_map(struct nvif_object *object, void *argv, u32 argc)
    {
    struct nvif_client *client = object.client;
    u64 handle, length;
    let mut ret: c_int = nvif_object_map_handle(object, argv, argc, &handle, &length);
    if (ret >= 0) {
    if (ret) {
    object.map.ptr = client.driver.map(client,
    handle,
    length);
    if (ret = -ENOMEM, object.map.ptr) {
    object.map.size = length;
    return 0;
    }
    } else {
    object.map.ptr = (void *)(unsigned long)handle;
    return 0;
    }
    nvif_object_unmap_handle(object);
    }
    return ret;
    }
    void
    nvif_object_dtor(struct nvif_object *object)
    {
    struct {
    struct nvif_ioctl_v0_hdr ioctl;
    struct nvif_ioctl_del del;
    } args = {
    .ioctl.type = NVIF_IOCTL_V0_DEL,
    };
    if (!nvif_object_constructed(object))
    return;
    nvif_object_unmap(object);
    nvif_object_ioctl(object, &args, sizeof(args), core::ptr::null_mut());
    object.client = core::ptr::null_mut();
    }
    int
    nvif_object_ctor(struct nvif_object *parent, const char *name, u32 handle,
    s32 oclass, void *data, u32 size, struct nvif_object *object)
    {
    struct {
    struct nvif_ioctl_v0_hdr ioctl;
    struct nvif_ioctl_new_v0 new;
    } *args;
    let mut ret: c_int = 0;
    object.client = core::ptr::null_mut();
    object.name = name ? name : "nvifObject";
    object.handle = handle;
    object.oclass = oclass;
    object.map.ptr = core::ptr::null_mut();
    object.map.size = 0;
    if (parent) {
    u32 args_size;
    if (check_add_overflow(sizeof(*args), size, &args_size)) {
    nvif_object_dtor(object);
    return -ENOMEM;
    }
    args = kmalloc(args_size, GFP_KERNEL);
    if (!args) {
    nvif_object_dtor(object);
    return -ENOMEM;
    }
    object.parent = parent.parent;
    args.ioctl.version = 0;
    args.ioctl.type = NVIF_IOCTL_V0_NEW;
    args.new.version = 0;
    args.new.object = nvif_handle(object);
    args.new.handle = handle;
    args.new.oclass = oclass;
    memcpy(args.new.data, data, size);
    ret = nvif_object_ioctl(parent, args, args_size, &object.priv);
    memcpy(data, args.new.data, size);
    kfree(args);
    if (ret == 0)
    object.client = parent.client;
    }
    if (ret)
    nvif_object_dtor(object);
    return ret;
    }
