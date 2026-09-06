//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/fifo/ucgrp.c
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
pub struct nvkm_ucgrp {
    pub object: nvkm_object,
    pub cgrp: *mut nvkm_cgrp,
}

    static int
    nvkm_ucgrp_chan_new(const struct nvkm_oclass *oclass, void *argv, u32 argc,
    struct nvkm_object **pobject)
    {
    struct nvkm_cgrp *cgrp = nvkm_ucgrp(oclass.parent).cgrp;
    return nvkm_uchan_new(cgrp.runl.fifo, cgrp, oclass, argv, argc, pobject);
    }
    static int
    nvkm_ucgrp_sclass(struct nvkm_object *object, int index, struct nvkm_oclass *oclass)
    {
    struct nvkm_cgrp *cgrp = nvkm_ucgrp(object).cgrp;
    struct nvkm_fifo *fifo = cgrp.runl.fifo;
    const struct nvkm_fifo_func_chan *chan = &fifo.func.chan;
    let mut c: c_int = 0;
// *_CHANNEL_GPFIFO_*
    if (chan.user.oclass) {
    if (c++ == index) {
    oclass.base = chan.user;
    oclass.ctor = nvkm_ucgrp_chan_new;
    return 0;
    }
    }
    return -EINVAL;
    }
    static void *
    nvkm_ucgrp_dtor(struct nvkm_object *object)
    {
    struct nvkm_ucgrp *ucgrp = nvkm_ucgrp(object);
    nvkm_cgrp_unref(&ucgrp.cgrp);
    return ucgrp;
    }
    static const struct nvkm_object_func
    nvkm_ucgrp = {
    .dtor = nvkm_ucgrp_dtor,
    .sclass = nvkm_ucgrp_sclass,
    };
    int
    nvkm_ucgrp_new(struct nvkm_fifo *fifo, const struct nvkm_oclass *oclass, void *argv, u32 argc,
    struct nvkm_object **pobject)
    {
    union nvif_cgrp_args *args = argv;
    struct nvkm_runl *runl;
    struct nvkm_vmm *vmm;
    struct nvkm_ucgrp *ucgrp;
    int ret;
    if (argc < sizeof(args.v0) || args.v0.version != 0)
    return -ENOSYS;
    argc -= sizeof(args.v0);
    if (args.v0.namelen != argc)
    return -EINVAL;
// Lookup objects referenced in args.
    runl = nvkm_runl_get(fifo, args.v0.runlist, 0);
    if (!runl)
    return -EINVAL;
    vmm = nvkm_uvmm_search(oclass.client, args.v0.vmm);
    if (IS_ERR(vmm))
    return PTR_ERR(vmm);
// Allocate channel group.
    if (!(ucgrp = kzalloc_obj(*ucgrp))) {
    ret = -ENOMEM;
    goto done;
    }
    nvkm_object_ctor(&nvkm_ucgrp, oclass, &ucgrp.object);
// pobject = &ucgrp->object;
    ret = nvkm_cgrp_new(runl, args.v0.name, vmm, true, &ucgrp.cgrp);
    if (ret)
    goto done;
// Return channel group info to caller.
    args.v0.cgid = ucgrp.cgrp.id;
    done:
    nvkm_vmm_unref(&vmm);
    return ret;
    }
