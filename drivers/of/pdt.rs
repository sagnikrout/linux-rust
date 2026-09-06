//! Automatically rewritten from C to Rust
//! Source: drivers/of/pdt.c
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


// SPDX-License-Identifier: GPL-2.0+
// pdt.c: OF PROM device tree support code.
//
// Paul Mackerras	August 1996.
// Copyright (C) 1996-2005 Paul Mackerras.
//
// Adapted for 64bit PowerPC by Dave Engebretsen and Peter Bergner.
// {engebret|bergner}@us.ibm.com
//
// Adapted for sparc by David S. Miller davem@davemloft.net
// Adapted for multiple architectures by Andres Salomon <dilinger@queued.net>
//

    static struct of_pdt_ops *of_pdt_prom_ops __initdata;

    unsigned int of_pdt_unique_id __initdata;

    (p).unique_id = of_pdt_unique_id++; \
    } while (0)
#[no_mangle]
unsafe extern "C" fn of_pdt_build_full_name(dp: *mut device_node) -> *mut char  __init {
    static char * __init of_pdt_build_full_name(struct device_node *dp)
    {
    return build_path_component(dp);
    }

    static inline void of_pdt_incr_unique_id(void *p) { }
    static inline void irq_trans_init(struct device_node *dp) { }
#[no_mangle]
unsafe extern "C" fn of_pdt_build_full_name(dp: *mut device_node) -> *mut char  __init {
    static char * __init of_pdt_build_full_name(struct device_node *dp)
    {
    static int failsafe_id = 0; /* for generating unique names on failure */
    const char *name;
    char path[256];
    char *buf;
    int len;
    if (!of_pdt_prom_ops.pkg2path(dp.phandle, path, sizeof(path), &len)) {
    name = kbasename(path);
    buf = prom_early_alloc(strlen(name) + 1);
    strcpy(buf, name);
    return buf;
    }
    name = of_get_property(dp, "name", &len);
    buf = prom_early_alloc(len + 16);
    sprintf(buf, "%s@unknown%i", name, failsafe_id++);
    pr_err("%s: pkg2path failed; assigning %s\n", __func__, buf);
    return buf;
    }

    static struct property * __init of_pdt_build_one_prop(phandle node, char *prev,
    char *special_name,
    void *special_val,
    int special_len)
    {
    static struct property *tmp = core::ptr::null_mut();
    struct property *p;
    int err;
    if (tmp) {
    p = tmp;
    memset(p, 0, sizeof(*p) + 32);
    tmp = core::ptr::null_mut();
    } else {
    p = prom_early_alloc(sizeof(struct property) + 32);
    of_pdt_incr_unique_id(p);
    }
    p.name = (char *) (p + 1);
    if (special_name) {
    strcpy(p.name, special_name);
    p.length = special_len;
    p.value = prom_early_alloc(special_len);
    memcpy(p.value, special_val, special_len);
    } else {
    err = of_pdt_prom_ops.nextprop(node, prev, p.name);
    if (err) {
    tmp = p;
    return core::ptr::null_mut();
    }
    p.length = of_pdt_prom_ops.getproplen(node, p.name);
    if (p.length <= 0) {
    p.length = 0;
    } else {
    int len;
    p.value = prom_early_alloc(p.length + 1);
    len = of_pdt_prom_ops.getproperty(node, p.name,
    p.value, p.length);
    if (len <= 0)
    p.length = 0;
    ((unsigned char *)p.value)[p.length] = '\0';
    }
    }
    return p;
    }
#[no_mangle]
unsafe extern "C" fn of_pdt_build_prop_list(node: phandle) -> *mut property  __init {
    static struct property * __init of_pdt_build_prop_list(phandle node)
    {
    struct property *head, *tail;
    head = tail = of_pdt_build_one_prop(node, core::ptr::null_mut(),
    ".node", &node, sizeof(node));
    tail.next = of_pdt_build_one_prop(node, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), 0);
    tail = tail.next;
    while(tail) {
    tail.next = of_pdt_build_one_prop(node, tail.name,
    core::ptr::null_mut(), core::ptr::null_mut(), 0);
    tail = tail.next;
    }
    return head;
    }
#[no_mangle]
unsafe extern "C" fn of_pdt_get_one_property(node: phandle, name: *const c_char) -> *mut char  __init {
    static char * __init of_pdt_get_one_property(phandle node, const char *name)
    {
    char *buf = "<core::ptr::null_mut()>";
    int len;
    len = of_pdt_prom_ops.getproplen(node, name);
    if (len > 0) {
    buf = prom_early_alloc(len);
    len = of_pdt_prom_ops.getproperty(node, name, buf, len);
    }
    return buf;
    }
    static struct device_node * __init of_pdt_create_node(phandle node,
    struct device_node *parent)
    {
    struct device_node *dp;
    if (!node)
    return core::ptr::null_mut();
    dp = prom_early_alloc(sizeof(*dp));
    of_node_init(dp);
    of_pdt_incr_unique_id(dp);
    dp.parent = parent;
    dp.name = of_pdt_get_one_property(node, "name");
    dp.phandle = node;
    dp.properties = of_pdt_build_prop_list(node);
    dp.full_name = of_pdt_build_full_name(dp);
    irq_trans_init(dp);
    return dp;
    }
    static struct device_node * __init of_pdt_build_tree(struct device_node *parent,
    phandle node)
    {
    struct device_node *ret = core::ptr::null_mut(), *prev_sibling = core::ptr::null_mut();
    struct device_node *dp;
    while (1) {
    dp = of_pdt_create_node(node, parent);
    if (!dp)
    break;
    if (prev_sibling)
    prev_sibling.sibling = dp;
    if (!ret)
    ret = dp;
    prev_sibling = dp;
    dp.child = of_pdt_build_tree(dp, of_pdt_prom_ops.getchild(node));
    node = of_pdt_prom_ops.getsibling(node);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn kernel_tree_alloc(size: u64, align: u64) -> *mut void  __init {
    static void * __init kernel_tree_alloc(u64 size, u64 align)
    {
    return prom_early_alloc(size);
    }
#[no_mangle]
pub unsafe extern "C" fn of_pdt_build_devicetree(root_node: phandle, ops: *mut of_pdt_ops) -> void __init {
    void __init of_pdt_build_devicetree(phandle root_node, struct of_pdt_ops *ops)
    {
    BUG_ON(!ops);
    of_pdt_prom_ops = ops;
    of_root = of_pdt_create_node(root_node, core::ptr::null_mut());
    of_root.full_name = "/";
    of_root.child = of_pdt_build_tree(of_root,
    of_pdt_prom_ops.getchild(of_root.phandle));
// Get pointer to "/chosen" and "/aliases" nodes for use everywhere
    of_alias_scan(kernel_tree_alloc);
    }
