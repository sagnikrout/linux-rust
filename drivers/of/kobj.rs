//! Automatically rewritten from C to Rust
//! Source: drivers/of/kobj.c
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

// true when node is initialized
#[no_mangle]
unsafe extern "C" fn of_node_is_initialized(node: *const device_node) -> c_int {
    static int of_node_is_initialized(const struct device_node *node)
    {
    return node && node.kobj.state_initialized;
    }
// true when node is attached (i.e. present on sysfs)
#[no_mangle]
pub unsafe extern "C" fn of_node_is_attached(node: *const device_node) -> c_int {
    int of_node_is_attached(const struct device_node *node)
    {
    return node && node.kobj.state_in_sysfs;
    }

#[no_mangle]
unsafe extern "C" fn of_node_release(kobj: *mut kobject) {
    static void of_node_release(struct kobject *kobj)
    {
// Without CONFIG_OF_DYNAMIC, no nodes gets freed
    }

    const struct kobj_type of_node_ktype = {
    .release = of_node_release,
    };
    EXPORT_SYMBOL_IF_KUNIT(of_node_ktype);
    static ssize_t of_node_property_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t offset, size_t count)
    {
    struct property *pp = container_of(bin_attr, struct property, attr);
    return memory_read_from_buffer(buf, count, &offset, pp.value, pp.length);
    }
// always return newly allocated name, caller must free after use
    static const char *safe_name(const struct kobject *kobj, const char *orig_name)
    {
    const char *name = orig_name;
    struct kernfs_node *kn;
    let mut i: c_int = 0;
// don't be a hero. After 16 tries give up
    while (i < 16 && (kn = sysfs_get_dirent(kobj.sd, name))) {
    sysfs_put(kn);
    if (name != orig_name)
    kfree(name);
    name = kasprintf(GFP_KERNEL, "%s#%i", orig_name, ++i);
    }
    if (name == orig_name) {
    name = kstrdup(orig_name, GFP_KERNEL);
    } else {
    pr_warn("Duplicate name in %s, renamed to \"%s\"\n",
    kobject_name(kobj), name);
    }
    return name;
    }
#[no_mangle]
pub unsafe extern "C" fn __of_add_property_sysfs(np: *mut device_node, pp: *mut property) -> c_int {
    int __of_add_property_sysfs(struct device_node *np, struct property *pp)
    {
    int rc;
// Important: Don't leak passwords
    let mut secure: bool = strncmp(pp.name, "security-", 9) == 0;
    if (!IS_ENABLED(CONFIG_SYSFS))
    return 0;
    if (!of_kset || !of_node_is_attached(np))
    return 0;
    sysfs_bin_attr_init(&pp.attr);
    pp.attr.attr.name = safe_name(&np.kobj, pp.name);
    pp.attr.attr.mode = secure ? 0400 : 0444;
    pp.attr.size = secure ? 0 : pp.length;
    pp.attr.read = of_node_property_read;
    rc = sysfs_create_bin_file(&np.kobj, &pp.attr);
    WARN(rc, "error adding attribute %s to node %pOF\n", pp.name, np);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn __of_sysfs_remove_bin_file(np: *mut device_node, prop: *const property) {
    void __of_sysfs_remove_bin_file(struct device_node *np, const struct property *prop)
    {
    if (!IS_ENABLED(CONFIG_SYSFS))
    return;
    sysfs_remove_bin_file(&np.kobj, &prop.attr);
    kfree(prop.attr.attr.name);
    }
#[no_mangle]
pub unsafe extern "C" fn __of_remove_property_sysfs(np: *mut device_node, prop: *const property) {
    void __of_remove_property_sysfs(struct device_node *np, const struct property *prop)
    {
// at early boot, bail here and defer setup to of_init()
    if (of_kset && of_node_is_attached(np))
    __of_sysfs_remove_bin_file(np, prop);
    }
    void __of_update_property_sysfs(struct device_node *np, struct property *newprop,
    const struct property *oldprop)
    {
// At early boot, bail out and defer setup to of_init()
    if (!of_kset)
    return;
    if (oldprop)
    __of_sysfs_remove_bin_file(np, oldprop);
    __of_add_property_sysfs(np, newprop);
    }
#[no_mangle]
pub unsafe extern "C" fn __of_attach_node_sysfs(np: *mut device_node) -> c_int {
    int __of_attach_node_sysfs(struct device_node *np)
    {
    const char *name;
    struct kobject *parent;
    struct property *pp;
    int rc;
    if (!IS_ENABLED(CONFIG_SYSFS) || !of_kset)
    return 0;
    np.kobj.kset = of_kset;
    if (!np.parent) {
// Nodes without parents are new top level trees
    name = safe_name(&of_kset.kobj, "base");
    parent = core::ptr::null_mut();
    } else {
    name = safe_name(&np.parent.kobj, kbasename(np.full_name));
    parent = &np.parent.kobj;
    }
    if (!name)
    return -ENOMEM;
    rc = kobject_add(&np.kobj, parent, "%s", name);
    kfree(name);
    if (rc)
    return rc;
    for_each_property_of_node(np, pp)
    __of_add_property_sysfs(np, pp);
    of_node_get(np);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __of_detach_node_sysfs(np: *mut device_node) {
    void __of_detach_node_sysfs(struct device_node *np)
    {
    struct property *pp;
    BUG_ON(!of_node_is_initialized(np));
    if (!of_kset)
    return;
// only remove properties if on sysfs
    if (of_node_is_attached(np)) {
    for_each_property_of_node(np, pp)
    __of_sysfs_remove_bin_file(np, pp);
    kobject_del(&np.kobj);
    }
    of_node_put(np);
    }
