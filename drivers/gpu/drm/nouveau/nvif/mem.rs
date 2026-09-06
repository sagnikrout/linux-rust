//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/mem.c
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
// Copyright 2017 Red Hat Inc.
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
    nvif_mem_ctor_map(struct nvif_mmu *mmu, const char *name, u8 type, u64 size,
    struct nvif_mem *mem)
    {
    int ret = nvif_mem_ctor(mmu, name, mmu.mem, NVIF_MEM_MAPPABLE | type,
    0, size, core::ptr::null_mut(), 0, mem);
    if (ret == 0) {
    ret = nvif_object_map(&mem.object, core::ptr::null_mut(), 0);
    if (ret)
    nvif_mem_dtor(mem);
    }
    return ret;
    }
    void
    nvif_mem_dtor(struct nvif_mem *mem)
    {
    nvif_object_dtor(&mem.object);
    }
    int
    nvif_mem_ctor_type(struct nvif_mmu *mmu, const char *name, s32 oclass,
    int type, u8 page, u64 size, void *argv, u32 argc,
    struct nvif_mem *mem)
    {
    struct nvif_mem_v0 *args;
    u8 stack[128];
    int ret;
    mem.object.client = core::ptr::null_mut();
    if (type < 0)
    return -EINVAL;
    if (sizeof(*args) + argc > sizeof(stack)) {
    if (!(args = kmalloc(sizeof(*args) + argc, GFP_KERNEL)))
    return -ENOMEM;
    } else {
    args = (void *)stack;
    }
    args.version = 0;
    args.type = type;
    args.page = page;
    args.size = size;
    memcpy(args.data, argv, argc);
    ret = nvif_object_ctor(&mmu.object, name ? name : "nvifMem", 0, oclass,
    args, sizeof(*args) + argc, &mem.object);
    if (ret == 0) {
    mem.type = mmu.type[type].type;
    mem.page = args.page;
    mem.addr = args.addr;
    mem.size = args.size;
    }
    if (args != (void *)stack)
    kfree(args);
    return ret;
    }
    int
    nvif_mem_ctor(struct nvif_mmu *mmu, const char *name, s32 oclass, u8 type,
    u8 page, u64 size, void *argv, u32 argc, struct nvif_mem *mem)
    {
    let mut ret: c_int = -EINVAL, i;
    mem.object.client = core::ptr::null_mut();
    for (i = 0; ret && i < mmu.type_nr; i++) {
    if ((mmu.type[i].type & type) == type) {
    ret = nvif_mem_ctor_type(mmu, name, oclass, i, page,
    size, argv, argc, mem);
    }
    }
    return ret;
    }
