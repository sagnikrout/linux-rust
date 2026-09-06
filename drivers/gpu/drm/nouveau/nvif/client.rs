//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvif/client.c
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
// Copyright 2013 Red Hat Inc.
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
    nvif_client_suspend(struct nvif_client *client, bool runtime)
    {
    return client.driver.suspend(client.object.priv, runtime);
    }
    int
    nvif_client_resume(struct nvif_client *client)
    {
    return client.driver.resume(client.object.priv);
    }
    void
    nvif_client_dtor(struct nvif_client *client)
    {
    nvif_object_dtor(&client.object);
    client.driver = core::ptr::null_mut();
    }
    int
    nvif_client_ctor(struct nvif_client *parent, const char *name, struct nvif_client *client)
    {
    let mut args: nvif_client_v0 = {};
    int ret;
    strscpy_pad(args.name, name, sizeof(args.name));
    ret = nvif_object_ctor(parent != client ? &parent.object : core::ptr::null_mut(),
    name ? name : "nvifClient", 0,
    NVIF_CLASS_CLIENT, &args, sizeof(args),
    &client.object);
    if (ret)
    return ret;
    client.object.client = client;
    client.object.handle = ~0;
    client.driver = parent.driver;
    return 0;
    }
