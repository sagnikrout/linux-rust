//! Automatically rewritten from C to Rust
//! Source: drivers/base/map.c
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


// SPDX-License-Identifier: GPL-2.0
//
// linux/drivers/base/map.c
//
// (C) Copyright Al Viro 2002,2003
//
// NOTE: data structure needs to be changed.  It works, but for large dev_t
// it will be too slow.  It is isolated, though, so these changes will be
// local to that file.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kobj_map {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct probe {
    pub next: *mut probe,
    pub dev: dev_t,
    pub range: c_ulong,
    pub owner: *mut module,
    pub get: *mut kobj_probe_t,
    pub ): *mut *mut int (lock)(dev_t, void,
    pub data: *mut c_void,
    pub probes: [*mut }; 255],
    pub lock: *mut mutex,
}

    int kobj_map(struct kobj_map *domain, dev_t dev, unsigned long range,
    struct module *module, kobj_probe_t *probe,
    int (*lock)(dev_t, void *), void *data)
    {
    let mut n: c_uint = MAJOR(dev + range - 1) - MAJOR(dev) + 1;
    let mut index: c_uint = MAJOR(dev);
    unsigned int i;
    struct probe *p;
    if (n > 255)
    n = 255;
    p = kmalloc_objs(struct probe, n);
    if (p == core::ptr::null_mut())
    return -ENOMEM;
    for (i = 0; i < n; i++, p++) {
    p.owner = module;
    p.get = probe;
    p.lock = lock;
    p.dev = dev;
    p.range = range;
    p.data = data;
    }
    mutex_lock(domain.lock);
    for (i = 0, p -= n; i < n; i++, p++, index++) {
    struct probe **s = &domain.probes[index % 255];
    while (*s && (*s).range < range)
    s = &(*s).next;
    p.next = *s;
// s = p;
    }
    mutex_unlock(domain.lock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kobj_unmap(domain: *mut kobj_map, dev: dev_t, range: c_ulong) {
    void kobj_unmap(struct kobj_map *domain, dev_t dev, unsigned long range)
    {
    let mut n: c_uint = MAJOR(dev + range - 1) - MAJOR(dev) + 1;
    let mut index: c_uint = MAJOR(dev);
    unsigned int i;
    struct probe *found = core::ptr::null_mut();
    if (n > 255)
    n = 255;
    mutex_lock(domain.lock);
    for (i = 0; i < n; i++, index++) {
    struct probe **s;
    for (s = &domain.probes[index % 255]; *s; s = &(*s).next) {
    struct probe *p = *s;
    if (p.dev == dev && p.range == range) {
// s = p->next;
    if (!found)
    found = p;
    break;
    }
    }
    }
    mutex_unlock(domain.lock);
    kfree(found);
    }
    struct kobject *kobj_lookup(struct kobj_map *domain, dev_t dev, int *index)
    {
    struct kobject *kobj;
    struct probe *p;
    let mut best: c_ulong = ~0UL;
    retry:
    mutex_lock(domain.lock);
    for (p = domain.probes[MAJOR(dev) % 255]; p; p = p.next) {
    struct kobject *(*probe)(dev_t, int *, void *);
    struct module *owner;
    void *data;
    if (p.dev > dev || p.dev + p.range - 1 < dev)
    continue;
    if (p.range - 1 >= best)
    break;
    if (!try_module_get(p.owner))
    continue;
    owner = p.owner;
    data = p.data;
    probe = p.get;
    best = p.range - 1;
// index = dev - p->dev;
    if (p.lock && p.lock(dev, data) < 0) {
    module_put(owner);
    continue;
    }
    mutex_unlock(domain.lock);
    kobj = probe(dev, index, data);
// Currently ->owner protects _only_ ->probe() itself.
    module_put(owner);
    if (kobj)
    return kobj;
    goto retry;
    }
    mutex_unlock(domain.lock);
    return core::ptr::null_mut();
    }
    struct kobj_map *kobj_map_init(kobj_probe_t *base_probe, struct mutex *lock)
    {
    struct kobj_map *p = kmalloc_obj(struct kobj_map);
    struct probe *base = kzalloc_obj(*base);
    int i;
    if ((p == core::ptr::null_mut()) || (base == core::ptr::null_mut())) {
    kfree(p);
    kfree(base);
    return core::ptr::null_mut();
    }
    base.dev = 1;
    base.range = ~0;
    base.get = base_probe;
    for (i = 0; i < 255; i++)
    p.probes[i] = base;
    p.lock = lock;
    return p;
    }
