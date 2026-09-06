//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/mmu.c
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

    void
    nvif_mmu_dtor(struct nvif_mmu *mmu)
    {
    if (!nvif_object_constructed(&mmu.object))
    return;
    kfree(mmu.kind);
    kfree(mmu.type);
    kfree(mmu.heap);
    nvif_object_dtor(&mmu.object);
    }
    int
    nvif_mmu_ctor(struct nvif_object *parent, const char *name, s32 oclass,
    struct nvif_mmu *mmu)
    {
    static const struct nvif_mclass mems[] = {
    { NVIF_CLASS_MEM_GF100, -1 },
    { NVIF_CLASS_MEM_NV50 , -1 },
    { NVIF_CLASS_MEM_NV04 , -1 },
    {}
    };
    struct nvif_mmu_v0 args;
    int ret, i;
    args.version = 0;
    mmu.heap = core::ptr::null_mut();
    mmu.type = core::ptr::null_mut();
    mmu.kind = core::ptr::null_mut();
    ret = nvif_object_ctor(parent, name ? name : "nvifMmu", 0, oclass,
    &args, sizeof(args), &mmu.object);
    if (ret)
    goto done;
    mmu.dmabits = args.dmabits;
    mmu.heap_nr = args.heap_nr;
    mmu.type_nr = args.type_nr;
    mmu.kind_nr = args.kind_nr;
    ret = nvif_mclass(&mmu.object, mems);
    if (ret < 0)
    goto done;
    mmu.mem = mems[ret].oclass;
    mmu.heap = kmalloc_objs(*mmu.heap, mmu.heap_nr);
    mmu.type = kmalloc_objs(*mmu.type, mmu.type_nr);
    if (ret = -ENOMEM, !mmu.heap || !mmu.type)
    goto done;
    mmu.kind = kmalloc_objs(*mmu.kind, mmu.kind_nr);
    if (!mmu.kind && mmu.kind_nr)
    goto done;
    for (i = 0; i < mmu.heap_nr; i++) {
    let mut args: nvif_mmu_heap_v0 = { .index = i };
    ret = nvif_object_mthd(&mmu.object, NVIF_MMU_V0_HEAP,
    &args, sizeof(args));
    if (ret)
    goto done;
    mmu.heap[i].size = args.size;
    }
    for (i = 0; i < mmu.type_nr; i++) {
    let mut args: nvif_mmu_type_v0 = { .index = i };
    ret = nvif_object_mthd(&mmu.object, NVIF_MMU_V0_TYPE,
    &args, sizeof(args));
    if (ret)
    goto done;
    mmu.type[i].type = 0;
    if (args.vram) mmu.type[i].type |= NVIF_MEM_VRAM;
    if (args.host) mmu.type[i].type |= NVIF_MEM_HOST;
    if (args.comp) mmu.type[i].type |= NVIF_MEM_COMP;
    if (args.disp) mmu.type[i].type |= NVIF_MEM_DISP;
    if (args.kind    ) mmu.type[i].type |= NVIF_MEM_KIND;
    if (args.mappable) mmu.type[i].type |= NVIF_MEM_MAPPABLE;
    if (args.coherent) mmu.type[i].type |= NVIF_MEM_COHERENT;
    if (args.uncached) mmu.type[i].type |= NVIF_MEM_UNCACHED;
    mmu.type[i].heap = args.heap;
    }
    if (mmu.kind_nr) {
    struct nvif_mmu_kind_v0 *kind;
    let mut argc: usize = struct_size(kind, data, mmu.kind_nr);
    if (ret = -ENOMEM, !(kind = kmalloc(argc, GFP_KERNEL)))
    goto done;
    kind.version = 0;
    kind.count = mmu.kind_nr;
    ret = nvif_object_mthd(&mmu.object, NVIF_MMU_V0_KIND,
    kind, argc);
    if (ret == 0)
    memcpy(mmu.kind, kind.data, kind.count);
    mmu.kind_inv = kind.kind_inv;
    kfree(kind);
    }
    done:
    if (ret)
    nvif_mmu_dtor(mmu);
    return ret;
    }
