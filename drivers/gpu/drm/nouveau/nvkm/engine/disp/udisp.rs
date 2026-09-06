//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/disp/udisp.c
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

    static int
    nvkm_udisp_sclass(struct nvkm_object *object, int index, struct nvkm_oclass *sclass)
    {
    struct nvkm_disp *disp = nvkm_udisp(object);
    if (index-- == 0) {
    sclass.base = (struct nvkm_sclass) { 0, 0, NVIF_CLASS_CONN };
    sclass.ctor = nvkm_uconn_new;
    return 0;
    }
    if (index-- == 0) {
    sclass.base = (struct nvkm_sclass) { 0, 0, NVIF_CLASS_OUTP };
    sclass.ctor = nvkm_uoutp_new;
    return 0;
    }
    if (index-- == 0) {
    sclass.base = (struct nvkm_sclass) { 0, 0, NVIF_CLASS_HEAD };
    sclass.ctor = nvkm_uhead_new;
    return 0;
    }
    if (disp.func.user[index].ctor) {
    sclass.base = disp.func.user[index].base;
    sclass.ctor = disp.func.user[index].ctor;
    return 0;
    }
    return -EINVAL;
    }
    static void *
    nvkm_udisp_dtor(struct nvkm_object *object)
    {
    struct nvkm_disp *disp = nvkm_udisp(object);
    spin_lock(&disp.client.lock);
    if (object == &disp.client.object)
    disp.client.object.func = core::ptr::null_mut();
    spin_unlock(&disp.client.lock);
    return core::ptr::null_mut();
    }
    static const struct nvkm_object_func
    nvkm_udisp = {
    .dtor = nvkm_udisp_dtor,
    .sclass = nvkm_udisp_sclass,
    };
    int
    nvkm_udisp_new(const struct nvkm_oclass *oclass, void *argv, u32 argc, struct nvkm_object **pobject)
    {
    struct nvkm_disp *disp = nvkm_disp(oclass.engine);
    struct nvkm_conn *conn;
    struct nvkm_outp *outp;
    struct nvkm_head *head;
    union nvif_disp_args *args = argv;
    if (argc != sizeof(args.v0) || args.v0.version != 0)
    return -ENOSYS;
    spin_lock(&disp.client.lock);
    if (disp.client.object.func) {
    spin_unlock(&disp.client.lock);
    return -EBUSY;
    }
    nvkm_object_ctor(&nvkm_udisp, oclass, &disp.client.object);
// pobject = &disp->client.object;
    spin_unlock(&disp.client.lock);
    args.v0.conn_mask = 0;
    list_for_each_entry(conn, &disp.conns, head)
    args.v0.conn_mask |= BIT(conn.index);
    args.v0.outp_mask = 0;
    list_for_each_entry(outp, &disp.outps, head)
    args.v0.outp_mask |= BIT(outp.index);
    args.v0.head_mask = 0;
    list_for_each_entry(head, &disp.heads, head)
    args.v0.head_mask |= BIT(head.id);
    return 0;
    }
