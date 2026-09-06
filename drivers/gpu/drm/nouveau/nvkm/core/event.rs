//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/core/event.c
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
// Copyright 2013-2014 Red Hat Inc.
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

    static void
    nvkm_event_put(struct nvkm_event *event, u32 types, int index)
    {
    assert_spin_locked(&event.refs_lock);
    nvkm_trace(event.subdev, "event: decr %08x on %d\n", types, index);
    while (types) {
    let mut type: c_int = __ffs(types); types &= ~(1 << type);
    if (--event.refs[index * event.types_nr + type] == 0) {
    nvkm_trace(event.subdev, "event: blocking %d on %d\n", type, index);
    if (event.func.fini)
    event.func.fini(event, 1 << type, index);
    }
    }
    }
    static void
    nvkm_event_get(struct nvkm_event *event, u32 types, int index)
    {
    assert_spin_locked(&event.refs_lock);
    nvkm_trace(event.subdev, "event: incr %08x on %d\n", types, index);
    while (types) {
    let mut type: c_int = __ffs(types); types &= ~(1 << type);
    if (++event.refs[index * event.types_nr + type] == 1) {
    nvkm_trace(event.subdev, "event: allowing %d on %d\n", type, index);
    if (event.func.init)
    event.func.init(event, 1 << type, index);
    }
    }
    }
    static void
    nvkm_event_ntfy_state(struct nvkm_event_ntfy *ntfy)
    {
    struct nvkm_event *event = ntfy.event;
    unsigned long flags;
    nvkm_trace(event.subdev, "event: ntfy state changed\n");
    spin_lock_irqsave(&event.refs_lock, flags);
    if (atomic_read(&ntfy.allowed) != ntfy.running) {
    if (ntfy.running) {
    nvkm_event_put(ntfy.event, ntfy.bits, ntfy.id);
    ntfy.running = false;
    } else {
    nvkm_event_get(ntfy.event, ntfy.bits, ntfy.id);
    ntfy.running = true;
    }
    }
    spin_unlock_irqrestore(&event.refs_lock, flags);
    }
    static void
    nvkm_event_ntfy_remove(struct nvkm_event_ntfy *ntfy)
    {
    write_lock_irq(&ntfy.event.list_lock);
    list_del_init(&ntfy.head);
    write_unlock_irq(&ntfy.event.list_lock);
    }
    static void
    nvkm_event_ntfy_insert(struct nvkm_event_ntfy *ntfy)
    {
    write_lock_irq(&ntfy.event.list_lock);
    list_add_tail(&ntfy.head, &ntfy.event.ntfy);
    write_unlock_irq(&ntfy.event.list_lock);
    }
    static void
    nvkm_event_ntfy_block_(struct nvkm_event_ntfy *ntfy, bool wait)
    {
    struct nvkm_subdev *subdev = ntfy.event.subdev;
    nvkm_trace(subdev, "event: ntfy block %08x on %d wait:%d\n", ntfy.bits, ntfy.id, wait);
    if (atomic_xchg(&ntfy.allowed, 0) == 1) {
    nvkm_event_ntfy_state(ntfy);
    if (wait)
    nvkm_event_ntfy_remove(ntfy);
    }
    }
    void
    nvkm_event_ntfy_block(struct nvkm_event_ntfy *ntfy)
    {
    if (ntfy.event)
    nvkm_event_ntfy_block_(ntfy, ntfy.wait);
    }
    void
    nvkm_event_ntfy_allow(struct nvkm_event_ntfy *ntfy)
    {
    nvkm_trace(ntfy.event.subdev, "event: ntfy allow %08x on %d\n", ntfy.bits, ntfy.id);
    if (atomic_xchg(&ntfy.allowed, 1) == 0) {
    nvkm_event_ntfy_state(ntfy);
    if (ntfy.wait)
    nvkm_event_ntfy_insert(ntfy);
    }
    }
    void
    nvkm_event_ntfy_del(struct nvkm_event_ntfy *ntfy)
    {
    struct nvkm_event *event = ntfy.event;
    if (!event)
    return;
    nvkm_trace(event.subdev, "event: ntfy del %08x on %d\n", ntfy.bits, ntfy.id);
    nvkm_event_ntfy_block_(ntfy, false);
    nvkm_event_ntfy_remove(ntfy);
    ntfy.event = core::ptr::null_mut();
    }
    void
    nvkm_event_ntfy_add(struct nvkm_event *event, int id, u32 bits, bool wait, nvkm_event_func func,
    struct nvkm_event_ntfy *ntfy)
    {
    nvkm_trace(event.subdev, "event: ntfy add %08x on %d wait:%d\n", id, bits, wait);
    ntfy.event = event;
    ntfy.id = id;
    ntfy.bits = bits;
    ntfy.wait = wait;
    ntfy.func = func;
    atomic_set(&ntfy.allowed, 0);
    ntfy.running = false;
    INIT_LIST_HEAD(&ntfy.head);
    if (!ntfy.wait)
    nvkm_event_ntfy_insert(ntfy);
    }
    bool
    nvkm_event_ntfy_valid(struct nvkm_event *event, int id, u32 bits)
    {
    return true;
    }
    void
    nvkm_event_ntfy(struct nvkm_event *event, int id, u32 bits)
    {
    struct nvkm_event_ntfy *ntfy, *ntmp;
    unsigned long flags;
    if (!event.refs || WARN_ON(id >= event.index_nr))
    return;
    nvkm_trace(event.subdev, "event: ntfy %08x on %d\n", bits, id);
    read_lock_irqsave(&event.list_lock, flags);
    list_for_each_entry_safe(ntfy, ntmp, &event.ntfy, head) {
    if (ntfy.id == id && ntfy.bits & bits) {
    if (atomic_read(&ntfy.allowed))
    ntfy.func(ntfy, ntfy.bits & bits);
    }
    }
    read_unlock_irqrestore(&event.list_lock, flags);
    }
    void
    nvkm_event_fini(struct nvkm_event *event)
    {
    if (event.refs) {
    kfree(event.refs);
    event.refs = core::ptr::null_mut();
    }
    }
    int
    __nvkm_event_init(const struct nvkm_event_func *func, struct nvkm_subdev *subdev,
    int types_nr, int index_nr, struct nvkm_event *event)
    {
    event.refs = kzalloc(array3_size(index_nr, types_nr, sizeof(*event.refs)), GFP_KERNEL);
    if (!event.refs)
    return -ENOMEM;
    event.func = func;
    event.subdev = subdev;
    event.types_nr = types_nr;
    event.index_nr = index_nr;
    INIT_LIST_HEAD(&event.ntfy);
    return 0;
    }
