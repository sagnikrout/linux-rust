//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fault/gp100.c
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

    void
    gp100_fault_buffer_intr(struct nvkm_fault_buffer *buffer, bool enable)
    {
    struct nvkm_device *device = buffer.fault.subdev.device;
    nvkm_mc_intr_mask(device, NVKM_SUBDEV_FAULT, 0, enable);
    }
    void
    gp100_fault_buffer_fini(struct nvkm_fault_buffer *buffer)
    {
    struct nvkm_device *device = buffer.fault.subdev.device;
    nvkm_mask(device, 0x002a70, 0x00000001, 0x00000000);
    }
    void
    gp100_fault_buffer_init(struct nvkm_fault_buffer *buffer)
    {
    struct nvkm_device *device = buffer.fault.subdev.device;
    nvkm_wr32(device, 0x002a74, upper_32_bits(buffer.addr));
    nvkm_wr32(device, 0x002a70, lower_32_bits(buffer.addr));
    nvkm_mask(device, 0x002a70, 0x00000001, 0x00000001);
    }
#[no_mangle]
pub unsafe extern "C" fn gp100_fault_buffer_pin(buffer: *mut nvkm_fault_buffer) -> u64 {
    u64 gp100_fault_buffer_pin(struct nvkm_fault_buffer *buffer)
    {
    return nvkm_memory_bar2(buffer.mem);
    }
    void
    gp100_fault_buffer_info(struct nvkm_fault_buffer *buffer)
    {
    buffer.entries = nvkm_rd32(buffer.fault.subdev.device, 0x002a78);
    buffer.get = 0x002a7c;
    buffer.put = 0x002a80;
    }
    void
    gp100_fault_intr(struct nvkm_fault *fault)
    {
    nvkm_event_ntfy(&fault.event, 0, NVKM_FAULT_BUFFER_EVENT_PENDING);
    }
    static const struct nvkm_fault_func
    gp100_fault = {
    .intr = gp100_fault_intr,
    .buffer.nr = 1,
    .buffer.entry_size = 32,
    .buffer.info = gp100_fault_buffer_info,
    .buffer.pin = gp100_fault_buffer_pin,
    .buffer.init = gp100_fault_buffer_init,
    .buffer.fini = gp100_fault_buffer_fini,
    .buffer.intr = gp100_fault_buffer_intr,
    .user = { { 0, 0, MAXWELL_FAULT_BUFFER_A }, 0 },
    };
    int
    gp100_fault_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_fault **pfault)
    {
    return nvkm_fault_new_(&gp100_fault, device, type, inst, pfault);
    }
