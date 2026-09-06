//! Automatically rewritten from C to Rust
//! Source: block/blk-mq-sysfs.c
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

#[no_mangle]
unsafe extern "C" fn blk_mq_sysfs_release(kobj: *mut kobject) {
    static void blk_mq_sysfs_release(struct kobject *kobj)
    {
    struct blk_mq_ctxs *ctxs = container_of(kobj, struct blk_mq_ctxs, kobj);
    free_percpu(ctxs.queue_ctx);
    kfree(ctxs);
    }
#[no_mangle]
unsafe extern "C" fn blk_mq_ctx_sysfs_release(kobj: *mut kobject) {
    static void blk_mq_ctx_sysfs_release(struct kobject *kobj)
    {
    struct blk_mq_ctx *ctx = container_of(kobj, struct blk_mq_ctx, kobj);
// ctx->ctxs won't be released until all ctx are freed
    kobject_put(&ctx.ctxs.kobj);
    }
#[no_mangle]
unsafe extern "C" fn blk_mq_hw_sysfs_release(kobj: *mut kobject) {
    static void blk_mq_hw_sysfs_release(struct kobject *kobj)
    {
    struct blk_mq_hw_ctx *hctx = container_of(kobj, struct blk_mq_hw_ctx,
    kobj);
    sbitmap_free(&hctx.ctx_map);
    free_cpumask_var(hctx.cpumask);
    kfree(hctx.ctxs);
    kfree(hctx);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_hw_ctx_sysfs_entry {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct blk_mq_hw_ctx , char,
}

    static ssize_t blk_mq_hw_sysfs_show(struct kobject *kobj,
    struct attribute *attr, char *page)
    {
    struct blk_mq_hw_ctx_sysfs_entry *entry;
    struct blk_mq_hw_ctx *hctx;
    struct request_queue *q;
    ssize_t res;
    entry = container_of_const(attr, struct blk_mq_hw_ctx_sysfs_entry, attr);
    hctx = container_of(kobj, struct blk_mq_hw_ctx, kobj);
    q = hctx.queue;
    if (!entry.show)
    return -EIO;
    mutex_lock(&q.elevator_lock);
    res = entry.show(hctx, page);
    mutex_unlock(&q.elevator_lock);
    return res;
    }
    static ssize_t blk_mq_hw_sysfs_nr_tags_show(struct blk_mq_hw_ctx *hctx,
    char *page)
    {
    return sprintf(page, "%u\n", hctx.tags.nr_tags);
    }
    static ssize_t blk_mq_hw_sysfs_nr_reserved_tags_show(struct blk_mq_hw_ctx *hctx,
    char *page)
    {
    return sprintf(page, "%u\n", hctx.tags.nr_reserved_tags);
    }
#[no_mangle]
unsafe extern "C" fn blk_mq_hw_sysfs_cpus_show(hctx: *mut blk_mq_hw_ctx, page: *mut c_char) -> isize {
    static ssize_t blk_mq_hw_sysfs_cpus_show(struct blk_mq_hw_ctx *hctx, char *page)
    {
    let mut size: usize = PAGE_SIZE - 1;
    unsigned int i, first = 1;
    let mut ret: c_int = 0, pos = 0;
    for_each_cpu(i, hctx.cpumask) {
    if (first)
    ret = snprintf(pos + page, size - pos, "%u", i);
    else
    ret = snprintf(pos + page, size - pos, ", %u", i);
    if (ret >= size - pos)
    break;
    first = 0;
    pos += ret;
    }
    ret = snprintf(pos + page, size + 1 - pos, "\n");
    return pos + ret;
    }
    static const struct blk_mq_hw_ctx_sysfs_entry blk_mq_hw_sysfs_nr_tags = {
    .attr = {.name = "nr_tags", .mode = 0444 },
    .show = blk_mq_hw_sysfs_nr_tags_show,
    };
    static const struct blk_mq_hw_ctx_sysfs_entry blk_mq_hw_sysfs_nr_reserved_tags = {
    .attr = {.name = "nr_reserved_tags", .mode = 0444 },
    .show = blk_mq_hw_sysfs_nr_reserved_tags_show,
    };
    static const struct blk_mq_hw_ctx_sysfs_entry blk_mq_hw_sysfs_cpus = {
    .attr = {.name = "cpu_list", .mode = 0444 },
    .show = blk_mq_hw_sysfs_cpus_show,
    };
    static const struct attribute *const default_hw_ctx_attrs[] = {
    &blk_mq_hw_sysfs_nr_tags.attr,
    &blk_mq_hw_sysfs_nr_reserved_tags.attr,
    &blk_mq_hw_sysfs_cpus.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(default_hw_ctx);
    static const struct sysfs_ops blk_mq_hw_sysfs_ops = {
    .show	= blk_mq_hw_sysfs_show,
    };
    static const struct kobj_type blk_mq_ktype = {
    .release	= blk_mq_sysfs_release,
    };
    static const struct kobj_type blk_mq_ctx_ktype = {
    .release	= blk_mq_ctx_sysfs_release,
    };
    static const struct kobj_type blk_mq_hw_ktype = {
    .sysfs_ops	= &blk_mq_hw_sysfs_ops,
    .default_groups = default_hw_ctx_groups,
    .release	= blk_mq_hw_sysfs_release,
    };
#[no_mangle]
unsafe extern "C" fn blk_mq_unregister_hctx(hctx: *mut blk_mq_hw_ctx) {
    static void blk_mq_unregister_hctx(struct blk_mq_hw_ctx *hctx)
    {
    struct blk_mq_ctx *ctx;
    int i;
    if (!hctx.nr_ctx)
    return;
    hctx_for_each_ctx(hctx, ctx, i)
    if (ctx.kobj.state_in_sysfs)
    kobject_del(&ctx.kobj);
    if (hctx.kobj.state_in_sysfs)
    kobject_del(&hctx.kobj);
    }
#[no_mangle]
unsafe extern "C" fn blk_mq_register_hctx(hctx: *mut blk_mq_hw_ctx) -> c_int {
    static int blk_mq_register_hctx(struct blk_mq_hw_ctx *hctx)
    {
    struct request_queue *q = hctx.queue;
    struct blk_mq_ctx *ctx;
    int i, j, ret;
    if (!hctx.nr_ctx)
    return 0;
    ret = kobject_add(&hctx.kobj, q.mq_kobj, "%u", hctx.queue_num);
    if (ret)
    return ret;
    hctx_for_each_ctx(hctx, ctx, i) {
    ret = kobject_add(&ctx.kobj, &hctx.kobj, "cpu%u", ctx.cpu);
    if (ret)
    goto out;
    }
    return 0;
    out:
    hctx_for_each_ctx(hctx, ctx, j) {
    if (j < i)
    kobject_del(&ctx.kobj);
    }
    kobject_del(&hctx.kobj);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_hctx_kobj_init(hctx: *mut blk_mq_hw_ctx) {
    void blk_mq_hctx_kobj_init(struct blk_mq_hw_ctx *hctx)
    {
    kobject_init(&hctx.kobj, &blk_mq_hw_ktype);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_deinit(q: *mut request_queue) {
    void blk_mq_sysfs_deinit(struct request_queue *q)
    {
    struct blk_mq_ctx *ctx;
    int cpu;
    for_each_possible_cpu(cpu) {
    ctx = per_cpu_ptr(q.queue_ctx, cpu);
    kobject_put(&ctx.kobj);
    }
    kobject_put(q.mq_kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_init(q: *mut request_queue) {
    void blk_mq_sysfs_init(struct request_queue *q)
    {
    struct blk_mq_ctx *ctx;
    int cpu;
    kobject_init(q.mq_kobj, &blk_mq_ktype);
    for_each_possible_cpu(cpu) {
    ctx = per_cpu_ptr(q.queue_ctx, cpu);
    kobject_get(q.mq_kobj);
    kobject_init(&ctx.kobj, &blk_mq_ctx_ktype);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_register(disk: *mut gendisk) -> c_int {
    int blk_mq_sysfs_register(struct gendisk *disk)
    {
    struct request_queue *q = disk.queue;
    struct blk_mq_hw_ctx *hctx;
    unsigned long i, j;
    int ret;
    ret = kobject_add(q.mq_kobj, &disk_to_dev(disk).kobj, "mq");
    if (ret < 0)
    return ret;
    kobject_uevent(q.mq_kobj, KOBJ_ADD);
    mutex_lock(&q.tag_set.tag_list_lock);
    queue_for_each_hw_ctx(q, hctx, i) {
    ret = blk_mq_register_hctx(hctx);
    if (ret)
    goto out_unreg;
    }
    mutex_unlock(&q.tag_set.tag_list_lock);
    return 0;
    out_unreg:
    queue_for_each_hw_ctx(q, hctx, j) {
    if (j < i)
    blk_mq_unregister_hctx(hctx);
    }
    mutex_unlock(&q.tag_set.tag_list_lock);
    kobject_uevent(q.mq_kobj, KOBJ_REMOVE);
    kobject_del(q.mq_kobj);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_unregister(disk: *mut gendisk) {
    void blk_mq_sysfs_unregister(struct gendisk *disk)
    {
    struct request_queue *q = disk.queue;
    struct blk_mq_hw_ctx *hctx;
    unsigned long i;
    mutex_lock(&q.tag_set.tag_list_lock);
    queue_for_each_hw_ctx(q, hctx, i)
    blk_mq_unregister_hctx(hctx);
    mutex_unlock(&q.tag_set.tag_list_lock);
    kobject_uevent(q.mq_kobj, KOBJ_REMOVE);
    kobject_del(q.mq_kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_unregister_hctxs(q: *mut request_queue) {
    void blk_mq_sysfs_unregister_hctxs(struct request_queue *q)
    {
    struct blk_mq_hw_ctx *hctx;
    unsigned long i;
    if (!blk_queue_registered(q))
    return;
    queue_for_each_hw_ctx(q, hctx, i)
    blk_mq_unregister_hctx(hctx);
    }
#[no_mangle]
pub unsafe extern "C" fn blk_mq_sysfs_register_hctxs(q: *mut request_queue) -> c_int {
    int blk_mq_sysfs_register_hctxs(struct request_queue *q)
    {
    struct blk_mq_hw_ctx *hctx;
    unsigned long i;
    let mut ret: c_int = 0;
    if (!blk_queue_registered(q))
    goto out;
    queue_for_each_hw_ctx(q, hctx, i) {
    ret = blk_mq_register_hctx(hctx);
    if (ret)
    break;
    }
    out:
    return ret;
    }
