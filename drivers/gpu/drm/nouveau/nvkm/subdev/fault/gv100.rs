//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fault/gv100.c
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
    gv100_fault_buffer_process(struct work_struct *work)
    {
    struct nvkm_fault *fault = container_of(work, typeof(*fault), nrpfb_work);
    struct nvkm_fault_buffer *buffer = fault.buffer[0];
    struct nvkm_device *device = fault.subdev.device;
    struct nvkm_memory *mem = buffer.mem;
    let mut get: u32 = nvkm_rd32(device, buffer.get);
    let mut put: u32 = nvkm_rd32(device, buffer.put);
    if (put == get)
    return;
    nvkm_kmap(mem);
    while (get != put) {
    let mut base: u32 = get * buffer.fault.func.buffer.entry_size;
    let mut instlo: u32 = nvkm_ro32(mem, base + 0x00);
    let mut insthi: u32 = nvkm_ro32(mem, base + 0x04);
    let mut addrlo: u32 = nvkm_ro32(mem, base + 0x08);
    let mut addrhi: u32 = nvkm_ro32(mem, base + 0x0c);
    let mut timelo: u32 = nvkm_ro32(mem, base + 0x10);
    let mut timehi: u32 = nvkm_ro32(mem, base + 0x14);
    let mut info0: u32 = nvkm_ro32(mem, base + 0x18);
    let mut info1: u32 = nvkm_ro32(mem, base + 0x1c);
    struct nvkm_fault_data info;
    if (++get == buffer.entries)
    get = 0;
    nvkm_wr32(device, buffer.get, get);
    info.addr   = ((u64)addrhi << 32) | addrlo;
    info.inst   = ((u64)insthi << 32) | instlo;
    info.time   = ((u64)timehi << 32) | timelo;
    info.engine = (info0 & 0x000000ff);
    info.valid  = (info1 & 0x80000000) >> 31;
    info.gpc    = (info1 & 0x1f000000) >> 24;
    info.hub    = (info1 & 0x00100000) >> 20;
    info.access = (info1 & 0x000f0000) >> 16;
    info.client = (info1 & 0x00007f00) >> 8;
    info.reason = (info1 & 0x0000001f);
    nvkm_fifo_fault(device.fifo, &info);
    }
    nvkm_done(mem);
    }
    static void
    gv100_fault_buffer_intr(struct nvkm_fault_buffer *buffer, bool enable)
    {
    struct nvkm_device *device = buffer.fault.subdev.device;
    let mut intr: u32 = buffer.id ? 0x08000000 : 0x20000000;
    if (enable)
    nvkm_mask(device, 0x100a2c, intr, intr);
    else
    nvkm_mask(device, 0x100a34, intr, intr);
    }
    static void
    gv100_fault_buffer_fini(struct nvkm_fault_buffer *buffer)
    {
    struct nvkm_device *device = buffer.fault.subdev.device;
    let mut foff: u32 = buffer.id * 0x14;
    nvkm_mask(device, 0x100e34 + foff, 0x80000000, 0x00000000);
    }
    static void
    gv100_fault_buffer_init(struct nvkm_fault_buffer *buffer)
    {
    struct nvkm_device *device = buffer.fault.subdev.device;
    let mut foff: u32 = buffer.id * 0x14;
    nvkm_mask(device, 0x100e34 + foff, 0xc0000000, 0x40000000);
    nvkm_wr32(device, 0x100e28 + foff, upper_32_bits(buffer.addr));
    nvkm_wr32(device, 0x100e24 + foff, lower_32_bits(buffer.addr));
    nvkm_mask(device, 0x100e34 + foff, 0x80000000, 0x80000000);
    }
    static void
    gv100_fault_buffer_info(struct nvkm_fault_buffer *buffer)
    {
    struct nvkm_device *device = buffer.fault.subdev.device;
    let mut foff: u32 = buffer.id * 0x14;
    nvkm_mask(device, 0x100e34 + foff, 0x40000000, 0x40000000);
    buffer.entries = nvkm_rd32(device, 0x100e34 + foff) & 0x000fffff;
    buffer.get = 0x100e2c + foff;
    buffer.put = 0x100e30 + foff;
    }
    static int
    gv100_fault_ntfy_nrpfb(struct nvkm_event_ntfy *ntfy, u32 bits)
    {
    struct nvkm_fault *fault = container_of(ntfy, typeof(*fault), nrpfb);
    schedule_work(&fault.nrpfb_work);
    return NVKM_EVENT_KEEP;
    }
    static void
    gv100_fault_intr_fault(struct nvkm_fault *fault)
    {
    struct nvkm_subdev *subdev = &fault.subdev;
    struct nvkm_device *device = subdev.device;
    struct nvkm_fault_data info;
    let mut addrlo: u32 = nvkm_rd32(device, 0x100e4c);
    let mut addrhi: u32 = nvkm_rd32(device, 0x100e50);
    let mut info0: u32 = nvkm_rd32(device, 0x100e54);
    let mut insthi: u32 = nvkm_rd32(device, 0x100e58);
    let mut info1: u32 = nvkm_rd32(device, 0x100e5c);
    info.addr = ((u64)addrhi << 32) | addrlo;
    info.inst = ((u64)insthi << 32) | (info0 & 0xfffff000);
    info.time = 0;
    info.engine = (info0 & 0x000000ff);
    info.valid  = (info1 & 0x80000000) >> 31;
    info.gpc    = (info1 & 0x1f000000) >> 24;
    info.hub    = (info1 & 0x00100000) >> 20;
    info.access = (info1 & 0x000f0000) >> 16;
    info.client = (info1 & 0x00007f00) >> 8;
    info.reason = (info1 & 0x0000001f);
    nvkm_fifo_fault(device.fifo, &info);
    }
    static void
    gv100_fault_intr(struct nvkm_fault *fault)
    {
    struct nvkm_subdev *subdev = &fault.subdev;
    struct nvkm_device *device = subdev.device;
    let mut stat: u32 = nvkm_rd32(device, 0x100a20);
    if (stat & 0x80000000) {
    gv100_fault_intr_fault(fault);
    nvkm_wr32(device, 0x100e60, 0x80000000);
    stat &= ~0x80000000;
    }
    if (stat & 0x20000000) {
    if (fault.buffer[0]) {
    nvkm_event_ntfy(&fault.event, 0, NVKM_FAULT_BUFFER_EVENT_PENDING);
    stat &= ~0x20000000;
    }
    }
    if (stat & 0x08000000) {
    if (fault.buffer[1]) {
    nvkm_event_ntfy(&fault.event, 1, NVKM_FAULT_BUFFER_EVENT_PENDING);
    stat &= ~0x08000000;
    }
    }
    if (stat) {
    nvkm_debug(subdev, "intr %08x\n", stat);
    }
    }
    static void
    gv100_fault_fini(struct nvkm_fault *fault)
    {
    nvkm_event_ntfy_block(&fault.nrpfb);
    flush_work(&fault.nrpfb_work);
    if (fault.buffer[0])
    fault.func.buffer.fini(fault.buffer[0]);
    nvkm_mask(fault.subdev.device, 0x100a34, 0x80000000, 0x80000000);
    }
    static void
    gv100_fault_init(struct nvkm_fault *fault)
    {
    nvkm_mask(fault.subdev.device, 0x100a2c, 0x80000000, 0x80000000);
    fault.func.buffer.init(fault.buffer[0]);
    nvkm_event_ntfy_allow(&fault.nrpfb);
    }
    int
    gv100_fault_oneinit(struct nvkm_fault *fault)
    {
    nvkm_event_ntfy_add(&fault.event, 0, NVKM_FAULT_BUFFER_EVENT_PENDING, true,
    gv100_fault_ntfy_nrpfb, &fault.nrpfb);
    return 0;
    }
    static const struct nvkm_fault_func
    gv100_fault = {
    .oneinit = gv100_fault_oneinit,
    .init = gv100_fault_init,
    .fini = gv100_fault_fini,
    .intr = gv100_fault_intr,
    .buffer.nr = 2,
    .buffer.entry_size = 32,
    .buffer.info = gv100_fault_buffer_info,
    .buffer.pin = gp100_fault_buffer_pin,
    .buffer.init = gv100_fault_buffer_init,
    .buffer.fini = gv100_fault_buffer_fini,
    .buffer.intr = gv100_fault_buffer_intr,
// TODO: Figure out how to expose non-replayable fault buffer, which,
// for some reason, is where recoverable CE faults appear...
//
// It's a bit tricky, as both NVKM and SVM will need access to
// the non-replayable fault buffer.
//
    .user = { { 0, 0, VOLTA_FAULT_BUFFER_A }, 1 },
    };
    int
    gv100_fault_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_fault **pfault)
    {
    let mut ret: c_int = nvkm_fault_new_(&gv100_fault, device, type, inst, pfault);
    if (ret)
    return ret;
    INIT_WORK(&(*pfault).nrpfb_work, gv100_fault_buffer_process);
    return 0;
    }
