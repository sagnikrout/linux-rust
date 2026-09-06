//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/misc.c
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
// Miscellaneous cgroup controller
//
// Copyright 2020 Google LLC
// Author: Vipin Sharma <vipinsh@google.com>
//

// Miscellaneous res name, keep it in sync with enum misc_res_type
    static const char *const misc_res_name[] = {

// AMD SEV ASIDs resource
    "sev",
// AMD SEV-ES ASIDs resource
    "sev_es",

// Intel TDX HKIDs resource
    "tdx",

    };
// Root misc cgroup
    static struct misc_cg root_cg;
//
// Miscellaneous resources capacity for the entire machine. 0 capacity means
// resource is not initialized or not present in the host.
//
// root_cg.max and capacity are independent of each other. root_cg.max can be
// more than the actual capacity. We are using Limits resource distribution
// model of cgroup for miscellaneous controller.
//
    static u64 misc_res_capacity[MISC_CG_RES_TYPES];
//
// parent_misc() - Get the parent of the passed misc cgroup.
// @cgroup: cgroup whose parent needs to be fetched.
//
// Context: Any context.
// Return:
// * struct misc_cg* - Parent of the @cgroup.
// * %NULL - If @cgroup is null or the passed cgroup does not have a parent.
//
    static struct misc_cg *parent_misc(struct misc_cg *cgroup)
    {
    return cgroup ? css_misc(cgroup.css.parent) : core::ptr::null_mut();
    }
//
// valid_type() - Check if @type is valid or not.
// @type: misc res type.
//
// Context: Any context.
// Return:
// * true - If valid type.
// * false - If not valid type.
//
#[no_mangle]
pub unsafe extern "C" fn valid_type(type: enum misc_res_type) -> bool {
    static inline bool valid_type(enum misc_res_type type)
    {
    return type >= 0 && type < MISC_CG_RES_TYPES;
    }
//
// misc_cg_set_capacity() - Set the capacity of the misc cgroup res.
// @type: Type of the misc res.
// @capacity: Supported capacity of the misc res on the host.
//
// If capacity is 0 then the charging a misc cgroup fails for that type.
//
// Context: Any context.
// Return:
// * %0 - Successfully registered the capacity.
// * %-EINVAL - If @type is invalid.
//
#[no_mangle]
pub unsafe extern "C" fn misc_cg_set_capacity(type: enum misc_res_type, capacity: u64) -> c_int {
    int misc_cg_set_capacity(enum misc_res_type type, u64 capacity)
    {
    if (!valid_type(type))
    return -EINVAL;
    WRITE_ONCE(misc_res_capacity[type], capacity);
    return 0;
    }
    EXPORT_SYMBOL_GPL(misc_cg_set_capacity);
//
// misc_cg_cancel_charge() - Cancel the charge from the misc cgroup.
// @type: Misc res type in misc cg to cancel the charge from.
// @cg: Misc cgroup to cancel charge from.
// @amount: Amount to cancel.
//
// Context: Any context.
//
    static void misc_cg_cancel_charge(enum misc_res_type type, struct misc_cg *cg,
    u64 amount)
    {
    WARN_ONCE(atomic64_add_negative(-amount, &cg.res[type].usage),
    "misc cgroup resource %s became less than 0",
    misc_res_name[type]);
    }
#[no_mangle]
unsafe extern "C" fn misc_cg_update_watermark(res: *mut misc_res, new_usage: u64) {
    static void misc_cg_update_watermark(struct misc_res *res, u64 new_usage)
    {
    u64 old;
    while (true) {
    old = atomic64_read(&res.watermark);
    if (new_usage <= old)
    break;
    if (atomic64_cmpxchg(&res.watermark, old, new_usage) == old)
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn misc_cg_event(type: enum misc_res_type, cg: *mut misc_cg) {
    static void misc_cg_event(enum misc_res_type type, struct misc_cg *cg)
    {
    atomic64_inc(&cg.res[type].events_local);
    cgroup_file_notify(&cg.events_local_file);
    for (; parent_misc(cg); cg = parent_misc(cg)) {
    atomic64_inc(&cg.res[type].events);
    cgroup_file_notify(&cg.events_file);
    }
    }
//
// misc_cg_try_charge() - Try charging the misc cgroup.
// @type: Misc res type to charge.
// @cg: Misc cgroup which will be charged.
// @amount: Amount to charge.
//
// Charge @amount to the misc cgroup. Caller must use the same cgroup during
// the uncharge call.
//
// Context: Any context.
// Return:
// * %0 - If successfully charged.
// * -EINVAL - If @type is invalid or misc res has 0 capacity.
// * -EBUSY - If max limit will be crossed or total usage will be more than the
// capacity.
//
#[no_mangle]
pub unsafe extern "C" fn misc_cg_try_charge(type: enum misc_res_type, cg: *mut misc_cg, amount: u64) -> c_int {
    int misc_cg_try_charge(enum misc_res_type type, struct misc_cg *cg, u64 amount)
    {
    struct misc_cg *i, *j;
    int ret;
    struct misc_res *res;
    u64 new_usage;
    if (!(valid_type(type) && cg && READ_ONCE(misc_res_capacity[type])))
    return -EINVAL;
    if (!amount)
    return 0;
    for (i = cg; i; i = parent_misc(i)) {
    res = &i.res[type];
    new_usage = atomic64_add_return(amount, &res.usage);
    if (new_usage > READ_ONCE(res.max) ||
    new_usage > READ_ONCE(misc_res_capacity[type])) {
    ret = -EBUSY;
    goto err_charge;
    }
    misc_cg_update_watermark(res, new_usage);
    }
    return 0;
    err_charge:
    misc_cg_event(type, i);
    for (j = cg; j != i; j = parent_misc(j))
    misc_cg_cancel_charge(type, j, amount);
    misc_cg_cancel_charge(type, i, amount);
    return ret;
    }
    EXPORT_SYMBOL_GPL(misc_cg_try_charge);
//
// misc_cg_uncharge() - Uncharge the misc cgroup.
// @type: Misc res type which was charged.
// @cg: Misc cgroup which will be uncharged.
// @amount: Charged amount.
//
// Context: Any context.
//
#[no_mangle]
pub unsafe extern "C" fn misc_cg_uncharge(type: enum misc_res_type, cg: *mut misc_cg, amount: u64) {
    void misc_cg_uncharge(enum misc_res_type type, struct misc_cg *cg, u64 amount)
    {
    struct misc_cg *i;
    if (!(amount && valid_type(type) && cg))
    return;
    for (i = cg; i; i = parent_misc(i))
    misc_cg_cancel_charge(type, i, amount);
    }
    EXPORT_SYMBOL_GPL(misc_cg_uncharge);
//
// misc_cg_max_show() - Show the misc cgroup max limit.
// @sf: Interface file
// @v: Arguments passed
//
// Context: Any context.
// Return: 0 to denote successful print.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_max_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int misc_cg_max_show(struct seq_file *sf, void *v)
    {
    int i;
    struct misc_cg *cg = css_misc(seq_css(sf));
    u64 max;
    for (i = 0; i < MISC_CG_RES_TYPES; i++) {
    if (READ_ONCE(misc_res_capacity[i])) {
    max = READ_ONCE(cg.res[i].max);
    if (max == MAX_NUM)
    seq_printf(sf, "%s max\n", misc_res_name[i]);
    else
    seq_printf(sf, "%s %llu\n", misc_res_name[i],
    max);
    }
    }
    return 0;
    }
//
// misc_cg_max_write() - Update the maximum limit of the cgroup.
// @of: Handler for the file.
// @buf: Data from the user. It should be either "max", 0, or a positive
// integer.
// @nbytes: Number of bytes of the data.
// @off: Offset in the file.
//
// User can pass data like:
// echo sev 23 > misc.max, OR
// echo sev max > misc.max
//
// Context: Any context.
// Return:
// * >= 0 - Number of bytes processed in the input.
// * -EINVAL - If buf is not valid.
// * -ERANGE - If number is bigger than the u64 capacity.
//
    static ssize_t misc_cg_max_write(struct kernfs_open_file *of, char *buf,
    size_t nbytes, loff_t off)
    {
    struct misc_cg *cg;
    u64 max;
    let mut ret: c_int = 0, i;
    let mut type: enum misc_res_type = MISC_CG_RES_TYPES;
    char *token;
    buf = strstrip(buf);
    token = strsep(&buf, " ");
    if (!token || !buf)
    return -EINVAL;
    for (i = 0; i < MISC_CG_RES_TYPES; i++) {
    if (!strcmp(misc_res_name[i], token)) {
    type = i;
    break;
    }
    }
    if (type == MISC_CG_RES_TYPES)
    return -EINVAL;
    if (!strcmp(MAX_STR, buf)) {
    max = MAX_NUM;
    } else {
    ret = kstrtou64(buf, 0, &max);
    if (ret)
    return ret;
    }
    cg = css_misc(of_css(of));
    if (READ_ONCE(misc_res_capacity[type]))
    WRITE_ONCE(cg.res[type].max, max);
    else
    ret = -EINVAL;
    return ret ? ret : nbytes;
    }
//
// misc_cg_current_show() - Show the current usage of the misc cgroup.
// @sf: Interface file
// @v: Arguments passed
//
// Context: Any context.
// Return: 0 to denote successful print.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_current_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int misc_cg_current_show(struct seq_file *sf, void *v)
    {
    int i;
    u64 usage;
    struct misc_cg *cg = css_misc(seq_css(sf));
    for (i = 0; i < MISC_CG_RES_TYPES; i++) {
    usage = atomic64_read(&cg.res[i].usage);
    if (READ_ONCE(misc_res_capacity[i]) || usage)
    seq_printf(sf, "%s %llu\n", misc_res_name[i], usage);
    }
    return 0;
    }
//
// misc_cg_peak_show() - Show the peak usage of the misc cgroup.
// @sf: Interface file
// @v: Arguments passed
//
// Context: Any context.
// Return: 0 to denote successful print.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_peak_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int misc_cg_peak_show(struct seq_file *sf, void *v)
    {
    int i;
    u64 watermark;
    struct misc_cg *cg = css_misc(seq_css(sf));
    for (i = 0; i < MISC_CG_RES_TYPES; i++) {
    watermark = atomic64_read(&cg.res[i].watermark);
    if (READ_ONCE(misc_res_capacity[i]) || watermark)
    seq_printf(sf, "%s %llu\n", misc_res_name[i], watermark);
    }
    return 0;
    }
//
// misc_cg_capacity_show() - Show the total capacity of misc res on the host.
// @sf: Interface file
// @v: Arguments passed
//
// Only present in the root cgroup directory.
//
// Context: Any context.
// Return: 0 to denote successful print.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_capacity_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int misc_cg_capacity_show(struct seq_file *sf, void *v)
    {
    int i;
    u64 cap;
    for (i = 0; i < MISC_CG_RES_TYPES; i++) {
    cap = READ_ONCE(misc_res_capacity[i]);
    if (cap)
    seq_printf(sf, "%s %llu\n", misc_res_name[i], cap);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __misc_events_show(sf: *mut seq_file, local: bool) -> c_int {
    static int __misc_events_show(struct seq_file *sf, bool local)
    {
    struct misc_cg *cg = css_misc(seq_css(sf));
    u64 events;
    int i;
    for (i = 0; i < MISC_CG_RES_TYPES; i++) {
    if (local)
    events = atomic64_read(&cg.res[i].events_local);
    else
    events = atomic64_read(&cg.res[i].events);
    if (READ_ONCE(misc_res_capacity[i]) || events)
    seq_printf(sf, "%s.max %llu\n", misc_res_name[i], events);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn misc_events_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int misc_events_show(struct seq_file *sf, void *v)
    {
    return __misc_events_show(sf, false);
    }
#[no_mangle]
unsafe extern "C" fn misc_events_local_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    static int misc_events_local_show(struct seq_file *sf, void *v)
    {
    return __misc_events_show(sf, true);
    }
// Misc cgroup interface files
    static struct cftype misc_cg_files[] = {
    {
    .name = "max",
    .write = misc_cg_max_write,
    .seq_show = misc_cg_max_show,
    .flags = CFTYPE_NOT_ON_ROOT,
    },
    {
    .name = "current",
    .seq_show = misc_cg_current_show,
    },
    {
    .name = "peak",
    .seq_show = misc_cg_peak_show,
    },
    {
    .name = "capacity",
    .seq_show = misc_cg_capacity_show,
    .flags = CFTYPE_ONLY_ON_ROOT,
    },
    {
    .name = "events",
    .flags = CFTYPE_NOT_ON_ROOT,
    .file_offset = offsetof(struct misc_cg, events_file),
    .seq_show = misc_events_show,
    },
    {
    .name = "events.local",
    .flags = CFTYPE_NOT_ON_ROOT,
    .file_offset = offsetof(struct misc_cg, events_local_file),
    .seq_show = misc_events_local_show,
    },
    {}
    };
//
// misc_cg_alloc() - Allocate misc cgroup.
// @parent_css: Parent cgroup.
//
// Context: Process context.
// Return:
// * struct cgroup_subsys_state* - css of the allocated cgroup.
// * ERR_PTR(-ENOMEM) - No memory available to allocate.
//
    static struct cgroup_subsys_state *
    misc_cg_alloc(struct cgroup_subsys_state *parent_css)
    {
    enum misc_res_type i;
    struct misc_cg *cg;
    if (!parent_css) {
    cg = &root_cg;
    } else {
    cg = kzalloc_obj(*cg);
    if (!cg)
    return ERR_PTR(-ENOMEM);
    }
    for (i = 0; i < MISC_CG_RES_TYPES; i++) {
    WRITE_ONCE(cg.res[i].max, MAX_NUM);
    atomic64_set(&cg.res[i].usage, 0);
    }
    return &cg.css;
    }
//
// misc_cg_free() - Free the misc cgroup.
// @css: cgroup subsys object.
//
// Context: Any context.
//
#[no_mangle]
unsafe extern "C" fn misc_cg_free(css: *mut cgroup_subsys_state) {
    static void misc_cg_free(struct cgroup_subsys_state *css)
    {
    kfree(css_misc(css));
    }
// Cgroup controller callbacks
    struct cgroup_subsys misc_cgrp_subsys = {
    .css_alloc = misc_cg_alloc,
    .css_free = misc_cg_free,
    .legacy_cftypes = misc_cg_files,
    .dfl_cftypes = misc_cg_files,
    };
