//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/reconfig.c
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
// pSeries_reconfig.c - support for dynamic reconfiguration (including PCI
// Hotplug and Dynamic Logical Partitioning on RPA platforms).
//
// Copyright (C) 2005 Nathan Lynch
// Copyright (C) 2005 IBM Corporation
//

#[no_mangle]
unsafe extern "C" fn pSeries_reconfig_add_node(path: *const c_char, proplist: *mut property) -> c_int {
    static int pSeries_reconfig_add_node(const char *path, struct property *proplist)
    {
    struct device_node *np;
    let mut err: c_int = -ENOMEM;
    np = kzalloc_obj(*np);
    if (!np)
    goto out_err;
    np.full_name = kstrdup(kbasename(path), GFP_KERNEL);
    if (!np.full_name)
    goto out_err;
    np.properties = proplist;
    of_node_set_flag(np, OF_DYNAMIC);
    of_node_init(np);
    np.parent = pseries_of_derive_parent(path);
    if (IS_ERR(np.parent)) {
    err = PTR_ERR(np.parent);
    goto out_err;
    }
    err = of_attach_node(np);
    if (err) {
    printk(KERN_ERR "Failed to add device node %s\n", path);
    goto out_err;
    }
    of_node_put(np.parent);
    return 0;
    out_err:
    if (np) {
    of_node_put(np.parent);
    kfree(np.full_name);
    kfree(np);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pSeries_reconfig_remove_node(np: *mut device_node) -> c_int {
    static int pSeries_reconfig_remove_node(struct device_node *np)
    {
    struct device_node *parent, *child;
    parent = of_get_parent(np);
    if (!parent)
    return -EINVAL;
    if ((child = of_get_next_child(np, core::ptr::null_mut()))) {
    of_node_put(child);
    of_node_put(parent);
    return -EBUSY;
    }
    of_detach_node(np);
    of_node_put(parent);
    return 0;
    }
//
// /proc/powerpc/ofdt - yucky binary interface for adding and removing
// OF device nodes.  Should be deprecated as soon as we get an
// in-kernel wrapper for the RTAS ibm,configure-connector call.
//
#[no_mangle]
unsafe extern "C" fn release_prop_list(prop: *const property) {
    static void release_prop_list(const struct property *prop)
    {
    struct property *next;
    for (; prop; prop = next) {
    next = prop.next;
    kfree(prop.name);
    kfree(prop.value);
    kfree(prop);
    }
    }
//
// parse_next_property - process the next property from raw input buffer
// @buf: input buffer, must be nul-terminated
// @end: end of the input buffer + 1, for validation
// @name: return value; set to property name in buf
// @length: return value; set to length of value
// @value: return value; set to the property value in buf
//
// Note that the caller must make copies of the name and value returned,
// this function does no allocation or copying of the data.  Return value
// is set to the next name in buf, or NULL on error.
//
    static char * parse_next_property(char *buf, char *end, char **name, int *length,
    unsigned char **value)
    {
    char *tmp;
// name = buf;
    tmp = strchr(buf, ' ');
    if (!tmp) {
    printk(KERN_ERR "property parse failed in %s at line %d\n",
    __func__, __LINE__);
    return core::ptr::null_mut();
    }
// tmp = '\0';
    if (++tmp >= end) {
    printk(KERN_ERR "property parse failed in %s at line %d\n",
    __func__, __LINE__);
    return core::ptr::null_mut();
    }
// now we're on the length
// length = -1;
// length = simple_strtoul(tmp, &tmp, 10);
    if (*length == -1) {
    printk(KERN_ERR "property parse failed in %s at line %d\n",
    __func__, __LINE__);
    return core::ptr::null_mut();
    }
    if (*tmp != ' ' || ++tmp >= end) {
    printk(KERN_ERR "property parse failed in %s at line %d\n",
    __func__, __LINE__);
    return core::ptr::null_mut();
    }
// now we're on the value
// value = tmp;
    tmp += *length;
    if (tmp > end) {
    printk(KERN_ERR "property parse failed in %s at line %d\n",
    __func__, __LINE__);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn if('\0': *mut *mut *mut tmp < end && tmp != ' ' && tmp !=) -> else {
    printk(KERN_ERR "property parse failed in %s at line %d\n",
    __func__, __LINE__);
    return core::ptr::null_mut();
    }
    tmp++;
// and now we should be on the next name, or the end
    return tmp;
    }
    static struct property *new_property(const char *name, const int length,
    const unsigned char *value, struct property *last)
    {
    struct property *new = kzalloc_obj(*new);
    if (!new)
    return core::ptr::null_mut();
    if (!(new.name = kstrdup(name, GFP_KERNEL)))
    goto cleanup;
    if (!(new.value = kmalloc(length + 1, GFP_KERNEL)))
    goto cleanup;
    memcpy(new.value, value, length);
// (((char *)new->value) + length) = 0;
    new.length = length;
    new.next = last;
    return new;
    cleanup:
    kfree(new.name);
    kfree(new.value);
    kfree(new);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn do_add_node(buf: *mut c_char, bufsize: usize) -> c_int {
    static int do_add_node(char *buf, size_t bufsize)
    {
    char *path, *end, *name;
    struct device_node *np;
    struct property *prop = core::ptr::null_mut();
    unsigned char* value;
    int length, rv = 0;
    end = buf + bufsize;
    path = buf;
    buf = strchr(buf, ' ');
    if (!buf)
    return -EINVAL;
// buf = '\0';
    buf++;
    if ((np = of_find_node_by_path(path))) {
    of_node_put(np);
    return -EINVAL;
    }
// rv = build_prop_list(tmp, bufsize - (tmp - buf), &proplist);
    while (buf < end &&
    (buf = parse_next_property(buf, end, &name, &length, &value))) {
    struct property *last = prop;
    prop = new_property(name, length, value, last);
    if (!prop) {
    rv = -ENOMEM;
    prop = last;
    goto out;
    }
    }
    if (!buf) {
    rv = -EINVAL;
    goto out;
    }
    rv = pSeries_reconfig_add_node(path, prop);
    out:
    if (rv)
    release_prop_list(prop);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn do_remove_node(buf: *mut c_char) -> c_int {
    static int do_remove_node(char *buf)
    {
    struct device_node *node;
    let mut rv: c_int = -ENODEV;
    if ((node = of_find_node_by_path(buf)))
    rv = pSeries_reconfig_remove_node(node);
    of_node_put(node);
    return rv;
    }
    static char *parse_node(char *buf, size_t bufsize, struct device_node **npp)
    {
    char *handle_str;
    phandle handle;
// npp = NULL;
    handle_str = buf;
    buf = strchr(buf, ' ');
    if (!buf)
    return core::ptr::null_mut();
// buf = '\0';
    buf++;
    handle = simple_strtoul(handle_str, core::ptr::null_mut(), 0);
// npp = of_find_node_by_phandle(handle);
    return buf;
    }
#[no_mangle]
unsafe extern "C" fn do_add_property(buf: *mut c_char, bufsize: usize) -> c_int {
    static int do_add_property(char *buf, size_t bufsize)
    {
    struct property *prop = core::ptr::null_mut();
    struct device_node *np;
    unsigned char *value;
    char *name, *end;
    int length;
    end = buf + bufsize;
    buf = parse_node(buf, bufsize, &np);
    if (!np)
    return -ENODEV;
    if (parse_next_property(buf, end, &name, &length, &value) == core::ptr::null_mut())
    return -EINVAL;
    prop = new_property(name, length, value, core::ptr::null_mut());
    if (!prop)
    return -ENOMEM;
    of_add_property(np, prop);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_remove_property(buf: *mut c_char, bufsize: usize) -> c_int {
    static int do_remove_property(char *buf, size_t bufsize)
    {
    struct device_node *np;
    char *tmp;
    buf = parse_node(buf, bufsize, &np);
    if (!np)
    return -ENODEV;
    tmp = strchr(buf,' ');
    if (tmp)
// tmp = '\0';
    if (*buf == '\0')
    return -EINVAL;
    return of_remove_property(np, of_find_property(np, buf, core::ptr::null_mut()));
    }
#[no_mangle]
unsafe extern "C" fn do_update_property(buf: *mut c_char, bufsize: usize) -> c_int {
    static int do_update_property(char *buf, size_t bufsize)
    {
    struct device_node *np;
    unsigned char *value;
    char *name, *end, *next_prop;
    int length;
    struct property *newprop;
    buf = parse_node(buf, bufsize, &np);
    end = buf + bufsize;
    if (!np)
    return -ENODEV;
    next_prop = parse_next_property(buf, end, &name, &length, &value);
    if (!next_prop)
    return -EINVAL;
    if (*name == '\0')
    return -ENODEV;
    newprop = new_property(name, length, value, core::ptr::null_mut());
    if (!newprop)
    return -ENOMEM;
    if (!strcmp(name, "slb-size") || !strcmp(name, "ibm,slb-size"))
    slb_set_size(*(int *)value);
    return of_update_property(np, newprop);
    }
//
// ofdt_write - perform operations on the Open Firmware device tree
//
// @file: not used
// @buf: command and arguments
// @count: size of the command buffer
// @off: not used
//
// Operations supported at this time are addition and removal of
// whole nodes along with their properties.  Operations on individual
// properties are not implemented (yet).
//
    static ssize_t ofdt_write(struct file *file, const char __user *buf, size_t count,
    loff_t *off)
    {
    int rv;
    char *kbuf;
    char *tmp;
    rv = security_locked_down(LOCKDOWN_DEVICE_TREE);
    if (rv)
    return rv;
    kbuf = memdup_user_nul(buf, count);
    if (IS_ERR(kbuf))
    return PTR_ERR(kbuf);
    tmp = strchr(kbuf, ' ');
    if (!tmp) {
    rv = -EINVAL;
    goto out;
    }
// tmp = '\0';
    tmp++;
    if (!strcmp(kbuf, "add_node"))
    rv = do_add_node(tmp, count - (tmp - kbuf));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(kbuf, _arg: "remove_node")) -> else {
    else if (!strcmp(kbuf, "remove_node"))
    rv = do_remove_node(tmp);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(kbuf, _arg: "add_property")) -> else {
    else if (!strcmp(kbuf, "add_property"))
    rv = do_add_property(tmp, count - (tmp - kbuf));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(kbuf, _arg: "remove_property")) -> else {
    else if (!strcmp(kbuf, "remove_property"))
    rv = do_remove_property(tmp, count - (tmp - kbuf));
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(kbuf, _arg: "update_property")) -> else {
    else if (!strcmp(kbuf, "update_property"))
    rv = do_update_property(tmp, count - (tmp - kbuf));
    else
    rv = -EINVAL;
    out:
    kfree(kbuf);
    return rv ? rv : count;
    }
    static const struct proc_ops ofdt_proc_ops = {
    .proc_write	= ofdt_write,
    .proc_lseek	= noop_llseek,
    };
// create /proc/powerpc/ofdt write-only by root
#[no_mangle]
unsafe extern "C" fn proc_ppc64_create_ofdt() -> c_int {
    static int proc_ppc64_create_ofdt(void)
    {
    struct proc_dir_entry *ent;
    ent = proc_create("powerpc/ofdt", 0200, core::ptr::null_mut(), &ofdt_proc_ops);
    if (ent)
    proc_set_size(ent, 0);
    return 0;
    }
    machine_device_initcall(pseries, proc_ppc64_create_ofdt);
