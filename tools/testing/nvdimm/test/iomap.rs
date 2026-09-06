//! Automatically rewritten from C to Rust
//! Source: tools/testing/nvdimm/test/iomap.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
//

    static LIST_HEAD(iomap_head);
    static struct iomap_ops {
    nfit_test_lookup_fn nfit_test_lookup;
    nfit_test_evaluate_dsm_fn evaluate_dsm;
    struct list_head list;
    } iomap_ops = {
    .list = LIST_HEAD_INIT(iomap_ops.list),
    };
    void nfit_test_setup(nfit_test_lookup_fn lookup,
    nfit_test_evaluate_dsm_fn evaluate)
    {
    iomap_ops.nfit_test_lookup = lookup;
    iomap_ops.evaluate_dsm = evaluate;
    list_add_rcu(&iomap_ops.list, &iomap_head);
    }
    EXPORT_SYMBOL(nfit_test_setup);
#[no_mangle]
pub unsafe extern "C" fn nfit_test_teardown() {
    void nfit_test_teardown(void)
    {
    list_del_rcu(&iomap_ops.list);
    synchronize_rcu();
    }
    EXPORT_SYMBOL(nfit_test_teardown);
    static struct nfit_test_resource *__get_nfit_res(resource_size_t resource)
    {
    struct iomap_ops *ops;
    ops = list_first_or_null_rcu(&iomap_head, typeof(*ops), list);
    if (ops)
    return ops.nfit_test_lookup(resource);
    return core::ptr::null_mut();
    }
    struct nfit_test_resource *get_nfit_res(resource_size_t resource)
    {
    struct nfit_test_resource *res;
    rcu_read_lock();
    res = __get_nfit_res(resource);
    rcu_read_unlock();
    return res;
    }
    EXPORT_SYMBOL(get_nfit_res);

    struct nfit_test_resource *nfit_res = get_nfit_res(offset);	\
    nfit_res ?							\
    (void __iomem *) nfit_res.buf + (offset)		\
    - nfit_res.res.start				\
    :								\
    fallback_fn((offset), (size)) ;				\
    })
    void __iomem *__wrap_devm_ioremap(struct device *dev,
    resource_size_t offset, unsigned long size)
    {
    struct nfit_test_resource *nfit_res = get_nfit_res(offset);
    if (nfit_res)
    return (void __iomem *) nfit_res.buf + offset
    - nfit_res.res.start;
    return devm_ioremap(dev, offset, size);
    }
    EXPORT_SYMBOL(__wrap_devm_ioremap);
    void *__wrap_devm_memremap(struct device *dev, resource_size_t offset,
    size_t size, unsigned long flags)
    {
    struct nfit_test_resource *nfit_res = get_nfit_res(offset);
    if (nfit_res)
    return nfit_res.buf + offset - nfit_res.res.start;
    return devm_memremap(dev, offset, size, flags);
    }
    EXPORT_SYMBOL(__wrap_devm_memremap);
#[no_mangle]
unsafe extern "C" fn nfit_test_kill(_pgmap: *mut c_void) {
    static void nfit_test_kill(void *_pgmap)
    {
    struct dev_pagemap *pgmap = _pgmap;
    WARN_ON(!pgmap);
    percpu_ref_kill(&pgmap.ref);
    wait_for_completion(&pgmap.done);
    percpu_ref_exit(&pgmap.ref);
    }
#[no_mangle]
unsafe extern "C" fn dev_pagemap_percpu_release(ref: *mut percpu_ref) {
    static void dev_pagemap_percpu_release(struct percpu_ref *ref)
    {
    struct dev_pagemap *pgmap = container_of(ref, struct dev_pagemap, ref);
    complete(&pgmap.done);
    }
    void *__wrap_devm_memremap_pages(struct device *dev, struct dev_pagemap *pgmap)
    {
    int error;
    let mut offset: resource_size_t = pgmap.range.start;
    struct nfit_test_resource *nfit_res = get_nfit_res(offset);
    if (!nfit_res)
    return devm_memremap_pages(dev, pgmap);
    init_completion(&pgmap.done);
    error = percpu_ref_init(&pgmap.ref, dev_pagemap_percpu_release, 0,
    GFP_KERNEL);
    if (error)
    return ERR_PTR(error);
    error = devm_add_action_or_reset(dev, nfit_test_kill, pgmap);
    if (error)
    return ERR_PTR(error);
    return nfit_res.buf + offset - nfit_res.res.start;
    }
    EXPORT_SYMBOL_GPL(__wrap_devm_memremap_pages);
    void *__wrap_memremap(resource_size_t offset, size_t size,
    unsigned long flags)
    {
    struct nfit_test_resource *nfit_res = get_nfit_res(offset);
    if (nfit_res)
    return nfit_res.buf + offset - nfit_res.res.start;
    return memremap(offset, size, flags);
    }
    EXPORT_SYMBOL(__wrap_memremap);
#[no_mangle]
pub unsafe extern "C" fn __wrap_devm_memunmap(dev: *mut device, addr: *mut c_void) {
    void __wrap_devm_memunmap(struct device *dev, void *addr)
    {
    struct nfit_test_resource *nfit_res = get_nfit_res((long) addr);
    if (nfit_res)
    return;
    return devm_memunmap(dev, addr);
    }
    EXPORT_SYMBOL(__wrap_devm_memunmap);
    void __iomem *__wrap_ioremap(resource_size_t offset, unsigned long size)
    {
    return __nfit_test_ioremap(offset, size, ioremap);
    }
    EXPORT_SYMBOL(__wrap_ioremap);
    void __iomem *__wrap_ioremap_wc(resource_size_t offset, unsigned long size)
    {
    return __nfit_test_ioremap(offset, size, ioremap_wc);
    }
    EXPORT_SYMBOL(__wrap_ioremap_wc);
#[no_mangle]
pub unsafe extern "C" fn __wrap_iounmap(addr: *mut volatile void __iomem) {
    void __wrap_iounmap(volatile void __iomem *addr)
    {
    struct nfit_test_resource *nfit_res = get_nfit_res((long) addr);
    if (nfit_res)
    return;
    return iounmap(addr);
    }
    EXPORT_SYMBOL(__wrap_iounmap);
#[no_mangle]
pub unsafe extern "C" fn __wrap_memunmap(addr: *mut c_void) {
    void __wrap_memunmap(void *addr)
    {
    struct nfit_test_resource *nfit_res = get_nfit_res((long) addr);
    if (nfit_res)
    return;
    return memunmap(addr);
    }
    EXPORT_SYMBOL(__wrap_memunmap);
    static bool nfit_test_release_region(struct device *dev,
    struct resource *parent, resource_size_t start,
    resource_size_t n);
#[no_mangle]
unsafe extern "C" fn nfit_devres_release(dev: *mut device, data: *mut c_void) {
    static void nfit_devres_release(struct device *dev, void *data)
    {
    struct resource *res = *((struct resource **) data);
    WARN_ON(!nfit_test_release_region(core::ptr::null_mut(), &iomem_resource, res.start,
    resource_size(res)));
    }
#[no_mangle]
unsafe extern "C" fn match(dev: *mut device, __res: *mut c_void, match_data: *mut c_void) -> c_int {
    static int match(struct device *dev, void *__res, void *match_data)
    {
    struct resource *res = *((struct resource **) __res);
    let mut start: resource_size_t = *((resource_size_t *) match_data);
    return res.start == start;
    }
    static bool nfit_test_release_region(struct device *dev,
    struct resource *parent, resource_size_t start,
    resource_size_t n)
    {
    if (parent == &iomem_resource) {
    struct nfit_test_resource *nfit_res = get_nfit_res(start);
    if (nfit_res) {
    struct nfit_test_request *req;
    struct resource *res = core::ptr::null_mut();
    if (dev) {
    devres_release(dev, nfit_devres_release, match,
    &start);
    return true;
    }
    spin_lock(&nfit_res.lock);
    list_for_each_entry(req, &nfit_res.requests, list)
    if (req.res.start == start) {
    res = &req.res;
    list_del(&req.list);
    break;
    }
    spin_unlock(&nfit_res.lock);
    WARN(!res || resource_size(res) != n,
    "%s: start: %llx n: %llx mismatch: %pr\n",
    __func__, start, n, res);
    if (res)
    kfree(req);
    return true;
    }
    }
    return false;
    }
    static struct resource *nfit_test_request_region(struct device *dev,
    struct resource *parent, resource_size_t start,
    resource_size_t n, const char *name, int flags)
    {
    struct nfit_test_resource *nfit_res;
    if (parent == &iomem_resource) {
    nfit_res = get_nfit_res(start);
    if (nfit_res) {
    struct nfit_test_request *req;
    struct resource *res = core::ptr::null_mut();
    if (start + n > nfit_res.res.start
    + resource_size(&nfit_res.res)) {
    pr_debug("%s: start: %llx n: %llx overflow: %pr\n",
    __func__, start, n,
    &nfit_res.res);
    return core::ptr::null_mut();
    }
    spin_lock(&nfit_res.lock);
    list_for_each_entry(req, &nfit_res.requests, list)
    if (start == req.res.start) {
    res = &req.res;
    break;
    }
    spin_unlock(&nfit_res.lock);
    if (res) {
    WARN(1, "%pr already busy\n", res);
    return core::ptr::null_mut();
    }
    req = kzalloc(sizeof(*req), GFP_KERNEL);
    if (!req)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&req.list);
    res = &req.res;
    res.start = start;
    res.end = start + n - 1;
    res.name = name;
    res.flags = resource_type(parent);
    res.flags |= IORESOURCE_BUSY | flags;
    spin_lock(&nfit_res.lock);
    list_add(&req.list, &nfit_res.requests);
    spin_unlock(&nfit_res.lock);
    if (dev) {
    struct resource **d;
    d = devres_alloc(nfit_devres_release,
    sizeof(struct resource *),
    GFP_KERNEL);
    if (!d)
    return core::ptr::null_mut();
// d = res;
    devres_add(dev, d);
    }
    pr_debug("%s: %pr\n", __func__, res);
    return res;
    }
    }
    if (dev)
    return __devm_request_region(dev, parent, start, n, name);
    return __request_region(parent, start, n, name, flags);
    }
    struct resource *__wrap___request_region(struct resource *parent,
    resource_size_t start, resource_size_t n, const char *name,
    int flags)
    {
    return nfit_test_request_region(core::ptr::null_mut(), parent, start, n, name, flags);
    }
    EXPORT_SYMBOL(__wrap___request_region);
#[no_mangle]
pub unsafe extern "C" fn __wrap_insert_resource(parent: *mut resource, res: *mut resource) -> c_int {
    int __wrap_insert_resource(struct resource *parent, struct resource *res)
    {
    if (get_nfit_res(res.start))
    return 0;
    return insert_resource(parent, res);
    }
    EXPORT_SYMBOL(__wrap_insert_resource);
#[no_mangle]
pub unsafe extern "C" fn __wrap_remove_resource(res: *mut resource) -> c_int {
    int __wrap_remove_resource(struct resource *res)
    {
    if (get_nfit_res(res.start))
    return 0;
    return remove_resource(res);
    }
    EXPORT_SYMBOL(__wrap_remove_resource);
    struct resource *__wrap___devm_request_region(struct device *dev,
    struct resource *parent, resource_size_t start,
    resource_size_t n, const char *name)
    {
    if (!dev)
    return core::ptr::null_mut();
    return nfit_test_request_region(dev, parent, start, n, name, 0);
    }
    EXPORT_SYMBOL(__wrap___devm_request_region);
    void __wrap___release_region(struct resource *parent, resource_size_t start,
    resource_size_t n)
    {
    if (!nfit_test_release_region(core::ptr::null_mut(), parent, start, n))
    __release_region(parent, start, n);
    }
    EXPORT_SYMBOL(__wrap___release_region);
    void __wrap___devm_release_region(struct device *dev, struct resource *parent,
    resource_size_t start, resource_size_t n)
    {
    if (!nfit_test_release_region(dev, parent, start, n))
    __devm_release_region(dev, parent, start, n);
    }
    EXPORT_SYMBOL(__wrap___devm_release_region);
    acpi_status __wrap_acpi_evaluate_object(acpi_handle handle, acpi_string path,
    struct acpi_object_list *p, struct acpi_buffer *buf)
    {
    struct nfit_test_resource *nfit_res = get_nfit_res((long) handle);
    union acpi_object **obj;
    if (!nfit_res || strcmp(path, "_FIT") || !buf)
    return acpi_evaluate_object(handle, path, p, buf);
    obj = nfit_res.buf;
    buf.length = sizeof(union acpi_object);
    buf.pointer = *obj;
    return AE_OK;
    }
    EXPORT_SYMBOL(__wrap_acpi_evaluate_object);
    union acpi_object * __wrap_acpi_evaluate_dsm(acpi_handle handle, const guid_t *guid,
    u64 rev, u64 func, union acpi_object *argv4)
    {
    union acpi_object *obj = ERR_PTR(-ENXIO);
    struct iomap_ops *ops;
    rcu_read_lock();
    ops = list_first_or_null_rcu(&iomap_head, typeof(*ops), list);
    if (ops)
    obj = ops.evaluate_dsm(handle, guid, rev, func, argv4);
    rcu_read_unlock();
    if (IS_ERR(obj))
    return acpi_evaluate_dsm(handle, guid, rev, func, argv4);
    return obj;
    }
    EXPORT_SYMBOL(__wrap_acpi_evaluate_dsm);
    MODULE_DESCRIPTION("NVDIMM unit test");
    MODULE_LICENSE("GPL v2");
