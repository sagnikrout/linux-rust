//! Automatically rewritten from C to Rust
//! Source: drivers/base/swnode.c
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
// Software nodes for the firmware node framework.
//
// Copyright (C) 2018, Intel Corporation
// Author: Heikki Krogerus <heikki.krogerus@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swnode {
    pub kobj: kobject,
    pub fwnode: fwnode_handle,
    pub node: *const software_node,
    pub id: c_int,
// hierarchy
    pub child_ids: ida,
    pub entry: list_head,
    pub children: list_head,
    pub parent: *mut swnode,
    pub allocated:1: c_uint,
    pub managed:1: c_uint,
}

    static DEFINE_IDA(swnode_root_ids);
    static struct kset *swnode_kset;

    static const struct fwnode_operations software_node_ops;
#[no_mangle]
pub unsafe extern "C" fn is_software_node(fwnode: *const fwnode_handle) -> bool {
    bool is_software_node(const struct fwnode_handle *fwnode)
    {
    return !IS_ERR_OR_NULL(fwnode) && fwnode.ops == &software_node_ops;
    }
    EXPORT_SYMBOL_GPL(is_software_node);

    ({								\
    typeof(__fwnode) __to_swnode_fwnode = __fwnode;		\
    \
    is_software_node(__to_swnode_fwnode) ?			\
    container_of(__to_swnode_fwnode,		\
    struct swnode, fwnode) : core::ptr::null_mut();	\
    })
    static inline struct swnode *dev_to_swnode(struct device *dev)
    {
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    if (!fwnode)
    return core::ptr::null_mut();
    if (!is_software_node(fwnode))
    fwnode = fwnode.secondary;
    return to_swnode(fwnode);
    }
    static struct swnode *
    software_node_to_swnode(const struct software_node *node)
    {
    struct swnode *swnode = core::ptr::null_mut();
    struct kobject *k;
    if (!node)
    return core::ptr::null_mut();
    spin_lock(&swnode_kset.list_lock);
    list_for_each_entry(k, &swnode_kset.list, entry) {
    swnode = kobj_to_swnode(k);
    if (swnode.node == node)
    break;
    swnode = core::ptr::null_mut();
    }
    spin_unlock(&swnode_kset.list_lock);
    return swnode;
    }
    const struct software_node *to_software_node(const struct fwnode_handle *fwnode)
    {
    const struct swnode *swnode = to_swnode(fwnode);
    return swnode ? swnode.node : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(to_software_node);
    struct fwnode_handle *software_node_fwnode(const struct software_node *node)
    {
    struct swnode *swnode = software_node_to_swnode(node);
    return swnode ? &swnode.fwnode : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(software_node_fwnode);
// --------------------------------------------------------------------------
// property_entry processing
    static const struct property_entry *
    property_entry_get(const struct property_entry *prop, const char *name)
    {
    if (!prop)
    return core::ptr::null_mut();
    for (; prop.name; prop++)
    if (!strcmp(name, prop.name))
    return prop;
    return core::ptr::null_mut();
    }
    static const void *property_get_pointer(const struct property_entry *prop)
    {
    if (!prop.length)
    return core::ptr::null_mut();
    return prop.is_inline ? &prop.value : prop.pointer;
    }
    static const void *property_entry_find(const struct property_entry *props,
    const char *propname, size_t length)
    {
    const struct property_entry *prop;
    const void *pointer;
    prop = property_entry_get(props, propname);
    if (!prop)
    return ERR_PTR(-EINVAL);
    pointer = property_get_pointer(prop);
    if (!pointer)
    return ERR_PTR(-ENODATA);
    if (length > prop.length)
    return ERR_PTR(-EOVERFLOW);
    return pointer;
    }
    static int
    property_entry_count_elems_of_size(const struct property_entry *props,
    const char *propname, size_t length)
    {
    const struct property_entry *prop;
    prop = property_entry_get(props, propname);
    if (!prop)
    return -EINVAL;
    return prop.length / length;
    }
    static int property_entry_read_int_array(const struct property_entry *props,
    const char *name,
    unsigned int elem_size, void *val,
    size_t nval)
    {
    const void *pointer;
    size_t length;
    if (!val)
    return property_entry_count_elems_of_size(props, name,
    elem_size);
    if (!is_power_of_2(elem_size) || elem_size > sizeof(u64))
    return -ENXIO;
    length = nval * elem_size;
    pointer = property_entry_find(props, name, length);
    if (IS_ERR(pointer))
    return PTR_ERR(pointer);
    memcpy(val, pointer, length);
    return 0;
    }
    static int property_entry_read_string_array(const struct property_entry *props,
    const char *propname,
    const char **strings, size_t nval)
    {
    const void *pointer;
    size_t length;
    int array_len;
// Find out the array length.
    array_len = property_entry_count_elems_of_size(props, propname,
    sizeof(const char *));
    if (array_len < 0)
    return array_len;
// Return how many there are if strings is NULL.
    if (!strings)
    return array_len;
    array_len = min_t(size_t, nval, array_len);
    length = array_len * sizeof(*strings);
    pointer = property_entry_find(props, propname, length);
    if (IS_ERR(pointer))
    return PTR_ERR(pointer);
    memcpy(strings, pointer, length);
    return array_len;
    }
#[no_mangle]
unsafe extern "C" fn property_entry_free_data(p: *const property_entry) {
    static void property_entry_free_data(const struct property_entry *p)
    {
    const char * const *src_str;
    size_t i, nval;
    if (p.type == DEV_PROP_STRING) {
    src_str = property_get_pointer(p);
    nval = p.length / sizeof(*src_str);
    for (i = 0; i < nval; i++)
    kfree(src_str[i]);
    }
    if (!p.is_inline)
    kfree(p.pointer);
    kfree(p.name);
    }
    static bool property_copy_string_array(const char **dst_ptr,
    const char * const *src_ptr,
    size_t nval)
    {
    int i;
    for (i = 0; i < nval; i++) {
    dst_ptr[i] = kstrdup(src_ptr[i], GFP_KERNEL);
    if (!dst_ptr[i] && src_ptr[i]) {
    while (--i >= 0)
    kfree(dst_ptr[i]);
    return false;
    }
    }
    return true;
    }
    static int property_entry_copy_data(struct property_entry *dst,
    const struct property_entry *src)
    {
    const void *pointer = property_get_pointer(src);
    void *dst_ptr;
    size_t nval;
//
// Properties with no data should not be marked as stored
// out of line.
//
    if (!src.is_inline && !src.length)
    return -ENODATA;
//
// Reference properties are never stored inline as
// they are too big.
//
    if (src.type == DEV_PROP_REF && src.is_inline)
    return -EINVAL;
    if (src.length <= sizeof(dst.value)) {
    dst_ptr = &dst.value;
    dst.is_inline = true;
    } else {
    dst_ptr = kmalloc(src.length, GFP_KERNEL);
    if (!dst_ptr)
    return -ENOMEM;
    dst.pointer = dst_ptr;
    }
    if (src.type == DEV_PROP_STRING) {
    nval = src.length / sizeof(const char *);
    if (!property_copy_string_array(dst_ptr, pointer, nval)) {
    if (!dst.is_inline)
    kfree(dst.pointer);
    return -ENOMEM;
    }
    } else {
    memcpy(dst_ptr, pointer, src.length);
    }
    dst.length = src.length;
    dst.type = src.type;
    dst.name = kstrdup(src.name, GFP_KERNEL);
    if (!dst.name) {
    property_entry_free_data(dst);
    return -ENOMEM;
    }
    return 0;
    }
//
// property_entries_dup - duplicate array of properties
// @properties: array of properties to copy
//
// This function creates a deep copy of the given NULL-terminated array
// of property entries.
//
    struct property_entry *
    property_entries_dup(const struct property_entry *properties)
    {
    struct property_entry *p;
    int i, n = 0;
    int ret;
    if (!properties)
    return core::ptr::null_mut();
    while (properties[n].name)
    n++;
    p = kzalloc_objs(*p, n + 1);
    if (!p)
    return ERR_PTR(-ENOMEM);
    for (i = 0; i < n; i++) {
    ret = property_entry_copy_data(&p[i], &properties[i]);
    if (ret) {
    while (--i >= 0)
    property_entry_free_data(&p[i]);
    kfree(p);
    return ERR_PTR(ret);
    }
    }
    return p;
    }
    EXPORT_SYMBOL_GPL(property_entries_dup);
//
// property_entries_free - free previously allocated array of properties
// @properties: array of properties to destroy
//
// This function frees given NULL-terminated array of property entries,
// along with their data.
//
#[no_mangle]
pub unsafe extern "C" fn property_entries_free(properties: *const property_entry) {
    void property_entries_free(const struct property_entry *properties)
    {
    const struct property_entry *p;
    if (!properties)
    return;
    for (p = properties; p.name; p++)
    property_entry_free_data(p);
    kfree(properties);
    }
    EXPORT_SYMBOL_GPL(property_entries_free);
// --------------------------------------------------------------------------
// fwnode operations
    static struct swnode *swnode_get(struct swnode *swnode)
    {
    kobject_get(&swnode.kobj);
    return swnode;
    }
#[no_mangle]
unsafe extern "C" fn swnode_put(swnode: *mut swnode) {
    static void swnode_put(struct swnode *swnode)
    {
    kobject_put(&swnode.kobj);
    }
    static struct fwnode_handle *software_node_get(struct fwnode_handle *fwnode)
    {
    struct swnode *swnode = swnode_get(to_swnode(fwnode));
    return &swnode.fwnode;
    }
#[no_mangle]
unsafe extern "C" fn software_node_put(fwnode: *mut fwnode_handle) {
    static void software_node_put(struct fwnode_handle *fwnode)
    {
    swnode_put(to_swnode(fwnode));
    }
    static bool software_node_property_present(const struct fwnode_handle *fwnode,
    const char *propname)
    {
    struct swnode *swnode = to_swnode(fwnode);
    return !!property_entry_get(swnode.node.properties, propname);
    }
    static int software_node_read_int_array(const struct fwnode_handle *fwnode,
    const char *propname,
    unsigned int elem_size, void *val,
    size_t nval)
    {
    struct swnode *swnode = to_swnode(fwnode);
    return property_entry_read_int_array(swnode.node.properties, propname,
    elem_size, val, nval);
    }
    static int software_node_read_string_array(const struct fwnode_handle *fwnode,
    const char *propname,
    const char **val, size_t nval)
    {
    struct swnode *swnode = to_swnode(fwnode);
    return property_entry_read_string_array(swnode.node.properties,
    propname, val, nval);
    }
    static const char *
    software_node_get_name(const struct fwnode_handle *fwnode)
    {
    const struct swnode *swnode = to_swnode(fwnode);
    return kobject_name(&swnode.kobj);
    }
    static const char *
    software_node_get_name_prefix(const struct fwnode_handle *fwnode)
    {
    struct fwnode_handle *parent;
    const char *prefix;
    parent = fwnode_get_parent(fwnode);
    if (!parent)
    return "";
// Figure out the prefix from the parents.
    while (is_software_node(parent))
    parent = fwnode_get_next_parent(parent);
    prefix = fwnode_get_name_prefix(parent);
    fwnode_handle_put(parent);
// Guess something if prefix was NULL.
    return prefix ?: "/";
    }
    static struct fwnode_handle *
    software_node_get_parent(const struct fwnode_handle *fwnode)
    {
    struct swnode *swnode = to_swnode(fwnode);
    if (!swnode || !swnode.parent)
    return core::ptr::null_mut();
    return fwnode_handle_get(&swnode.parent.fwnode);
    }
    static struct fwnode_handle *
    software_node_get_next_child(const struct fwnode_handle *fwnode,
    struct fwnode_handle *child)
    {
    struct swnode *p = to_swnode(fwnode);
    struct swnode *c = to_swnode(child);
    if (!p || list_empty(&p.children) ||
    (c && list_is_last(&c.entry, &p.children))) {
    fwnode_handle_put(child);
    return core::ptr::null_mut();
    }
    if (c)
    c = list_next_entry(c, entry);
    else
    c = list_first_entry(&p.children, struct swnode, entry);
    fwnode_handle_put(child);
    return fwnode_handle_get(&c.fwnode);
    }
    static struct fwnode_handle *
    software_node_get_named_child_node(const struct fwnode_handle *fwnode,
    const char *childname)
    {
    struct swnode *swnode = to_swnode(fwnode);
    struct swnode *child;
    if (!swnode || list_empty(&swnode.children))
    return core::ptr::null_mut();
    list_for_each_entry(child, &swnode.children, entry) {
    if (!strcmp(childname, kobject_name(&child.kobj))) {
    swnode_get(child);
    return &child.fwnode;
    }
    }
    return core::ptr::null_mut();
    }
    static int
    software_node_get_reference_args(const struct fwnode_handle *fwnode,
    const char *propname, const char *nargs_prop,
    unsigned int nargs, unsigned int index,
    struct fwnode_reference_args *args)
    {
    struct swnode *swnode = to_swnode(fwnode);
    const struct software_node_ref_args *ref_array;
    const struct software_node_ref_args *ref;
    const struct property_entry *prop;
    struct fwnode_handle *refnode;
    u32 nargs_prop_val;
    int error;
    int i;
    prop = property_entry_get(swnode.node.properties, propname);
    if (!prop)
    return -ENOENT;
    if (prop.type != DEV_PROP_REF)
    return -EINVAL;
//
// We expect that references are never stored inline, even
// single ones, as they are too big.
//
    if (prop.is_inline)
    return -EINVAL;
    if (index >= prop.length / sizeof(*ref))
    return -ENOENT;
    ref_array = prop.pointer;
    ref = &ref_array[index];
//
// A software node can reference other software nodes or firmware
// nodes (which are the abstraction layer sitting on top of them).
// This is done to ensure we can create references to static software
// nodes before they're registered with the firmware node framework.
// At the time the reference is being resolved, we expect the swnodes
// in question to already have been registered and to be backed by
// a firmware node. This is why we use the fwnode API below to read the
// relevant properties and bump the reference count.
//
    if (ref.swnode)
    refnode = software_node_fwnode(ref.swnode);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ref->fwnode) -> else {
    else if (ref.fwnode)
    refnode = ref.fwnode;
    else
    return -EINVAL;
    if (!refnode)
    return -ENOTCONN;
    if (nargs_prop) {
    error = fwnode_property_read_u32(refnode, nargs_prop, &nargs_prop_val);
    if (error)
    return error;
    nargs = nargs_prop_val;
    }
    if (nargs > NR_FWNODE_REFERENCE_ARGS)
    return -EINVAL;
    if (!args)
    return 0;
    args.fwnode = fwnode_handle_get(refnode);
    args.nargs = nargs;
    for (i = 0; i < nargs; i++)
    args.args[i] = ref.args[i];
    return 0;
    }
    static struct fwnode_handle *
    swnode_graph_find_next_port(const struct fwnode_handle *parent,
    struct fwnode_handle *port)
    {
    struct fwnode_handle *old = port;
    while ((port = software_node_get_next_child(parent, old))) {
//
// fwnode ports have naming style "port@", so we search for any
// children that follow that convention.
//
    if (!strncmp(to_swnode(port).node.name, "port@",
    strlen("port@")))
    return port;
    old = port;
    }
    return core::ptr::null_mut();
    }
    static struct fwnode_handle *
    software_node_graph_get_next_endpoint(const struct fwnode_handle *fwnode,
    struct fwnode_handle *endpoint)
    {
    struct swnode *swnode = to_swnode(fwnode);
    struct fwnode_handle *parent;
    struct fwnode_handle *port;
    if (!swnode)
    return core::ptr::null_mut();
    if (endpoint) {
    port = software_node_get_parent(endpoint);
    parent = software_node_get_parent(port);
    } else {
    parent = software_node_get_named_child_node(fwnode, "ports");
    if (!parent)
    parent = software_node_get(&swnode.fwnode);
    port = swnode_graph_find_next_port(parent, core::ptr::null_mut());
    }
    for (; port; port = swnode_graph_find_next_port(parent, port)) {
    endpoint = software_node_get_next_child(port, endpoint);
    if (endpoint) {
    fwnode_handle_put(port);
    break;
    }
    }
    fwnode_handle_put(parent);
    return endpoint;
    }
    static struct fwnode_handle *
    software_node_graph_get_remote_endpoint(const struct fwnode_handle *fwnode)
    {
    struct swnode *swnode = to_swnode(fwnode);
    const struct software_node_ref_args *ref;
    const struct property_entry *prop;
    if (!swnode)
    return core::ptr::null_mut();
    prop = property_entry_get(swnode.node.properties, "remote-endpoint");
    if (!prop || prop.type != DEV_PROP_REF || prop.is_inline)
    return core::ptr::null_mut();
    ref = prop.pointer;
    if (!ref.swnode)
    return core::ptr::null_mut();
    return software_node_get(software_node_fwnode(ref.swnode));
    }
    static struct fwnode_handle *
    software_node_graph_get_port_parent(struct fwnode_handle *fwnode)
    {
    struct swnode *swnode = to_swnode(fwnode);
    swnode = swnode.parent;
    if (swnode && !strcmp(swnode.node.name, "ports"))
    swnode = swnode.parent;
    return swnode ? software_node_get(&swnode.fwnode) : core::ptr::null_mut();
    }
    static int
    software_node_graph_parse_endpoint(const struct fwnode_handle *fwnode,
    struct fwnode_endpoint *endpoint)
    {
    struct swnode *swnode = to_swnode(fwnode);
    const char *parent_name = swnode.parent.node.name;
    int ret;
    if (strlen("port@") >= strlen(parent_name) ||
    strncmp(parent_name, "port@", strlen("port@")))
    return -EINVAL;
// Ports have naming style "port@n", we need to select the n
    ret = kstrtou32(parent_name + strlen("port@"), 10, &endpoint.port);
    if (ret)
    return ret;
    endpoint.id = swnode.id;
    endpoint.local_fwnode = fwnode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn software_node_add_links(fwnode: *mut fwnode_handle) -> c_int {
    static int software_node_add_links(struct fwnode_handle *fwnode)
    {
    const struct software_node_ref_args *ref, *ref_array;
    struct swnode *swnode = to_swnode(fwnode);
    const struct property_entry *prop;
    struct fwnode_handle *refnode;
    unsigned int count;
    if (!swnode || !swnode.node.properties)
    return 0;
//
// Unlike Device Tree, where phandles appear in many non-supplier
// contexts and a curated allowlist is required, a software node only
// carries a DEV_PROP_REF property when the author explicitly describes
// a reference to another node. Every such reference is therefore an
// intentional supplier dependency, so we create fwnode links for all
// of them.
//
    for (prop = swnode.node.properties; prop.name; prop++) {
    if (prop.type != DEV_PROP_REF || prop.is_inline)
    continue;
//
// TODO: Graph "remote-endpoint" references go both ways
// between endpoint child nodes and would create endpoint
// cycles. Let's leave it out for now until we have potential
// users.
//
    if (!strcmp(prop.name, "remote-endpoint"))
    continue;
    ref_array = prop.pointer;
    count = prop.length / sizeof(*ref_array);
    for (unsigned int i = 0; i < count; i++) {
    ref = &ref_array[i];
    if (ref.swnode)
    refnode = software_node_fwnode(ref.swnode);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ref->fwnode) -> else {
    else if (ref.fwnode)
    refnode = ref.fwnode;
    else
    continue;
// Supplier not registered yet, or self-reference.
    if (!refnode || refnode == &swnode.fwnode)
    continue;
    fwnode_link_add(&swnode.fwnode, refnode, 0);
    }
    }
    return 0;
    }
    static const struct fwnode_operations software_node_ops = {
    .get = software_node_get,
    .put = software_node_put,
    .property_present = software_node_property_present,
    .property_read_bool = software_node_property_present,
    .property_read_int_array = software_node_read_int_array,
    .property_read_string_array = software_node_read_string_array,
    .get_name = software_node_get_name,
    .get_name_prefix = software_node_get_name_prefix,
    .get_parent = software_node_get_parent,
    .get_next_child_node = software_node_get_next_child,
    .get_named_child_node = software_node_get_named_child_node,
    .get_reference_args = software_node_get_reference_args,
    .graph_get_next_endpoint = software_node_graph_get_next_endpoint,
    .graph_get_remote_endpoint = software_node_graph_get_remote_endpoint,
    .graph_get_port_parent = software_node_graph_get_port_parent,
    .graph_parse_endpoint = software_node_graph_parse_endpoint,
    .add_links = software_node_add_links,
    };
// --------------------------------------------------------------------------
//
// software_node_find_by_name - Find software node by name
// @parent: Parent of the software node
// @name: Name of the software node
//
// The function will find a node that is child of @parent and that is named
// @name. If no node is found, the function returns NULL.
//
// NOTE: you will need to drop the reference with fwnode_handle_put() after use.
//
    const struct software_node *
    software_node_find_by_name(const struct software_node *parent, const char *name)
    {
    struct swnode *swnode = core::ptr::null_mut();
    struct kobject *k;
    if (!name)
    return core::ptr::null_mut();
    spin_lock(&swnode_kset.list_lock);
    list_for_each_entry(k, &swnode_kset.list, entry) {
    swnode = kobj_to_swnode(k);
    if (parent == swnode.node.parent && swnode.node.name &&
    !strcmp(name, swnode.node.name)) {
    swnode_get(swnode);
    break;
    }
    swnode = core::ptr::null_mut();
    }
    spin_unlock(&swnode_kset.list_lock);
    return swnode ? swnode.node : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(software_node_find_by_name);
    static struct software_node *software_node_alloc(const struct property_entry *properties)
    {
    struct property_entry *props;
    struct software_node *node;
    props = property_entries_dup(properties);
    if (IS_ERR(props))
    return ERR_CAST(props);
    node = kzalloc_obj(*node);
    if (!node) {
    property_entries_free(props);
    return ERR_PTR(-ENOMEM);
    }
    node.properties = props;
    return node;
    }
#[no_mangle]
unsafe extern "C" fn software_node_free(node: *const software_node) {
    static void software_node_free(const struct software_node *node)
    {
    property_entries_free(node.properties);
    kfree(node);
    }
#[no_mangle]
unsafe extern "C" fn software_node_release(kobj: *mut kobject) {
    static void software_node_release(struct kobject *kobj)
    {
    struct swnode *swnode = kobj_to_swnode(kobj);
    fwnode_links_purge(&swnode.fwnode);
    if (swnode.parent) {
    ida_free(&swnode.parent.child_ids, swnode.id);
    list_del(&swnode.entry);
    } else {
    ida_free(&swnode_root_ids, swnode.id);
    }
    if (swnode.allocated)
    software_node_free(swnode.node);
    ida_destroy(&swnode.child_ids);
    kfree(swnode);
    }
    static const struct kobj_type software_node_type = {
    .release = software_node_release,
    .sysfs_ops = &kobj_sysfs_ops,
    };
    static struct fwnode_handle *
    swnode_register(const struct software_node *node, struct swnode *parent,
    unsigned int allocated)
    {
    struct swnode *swnode;
    int ret;
    swnode = kzalloc_obj(*swnode);
    if (!swnode)
    return ERR_PTR(-ENOMEM);
    ret = ida_alloc(parent ? &parent.child_ids : &swnode_root_ids,
    GFP_KERNEL);
    if (ret < 0) {
    kfree(swnode);
    return ERR_PTR(ret);
    }
    swnode.id = ret;
    swnode.node = node;
    swnode.parent = parent;
    swnode.kobj.kset = swnode_kset;
    fwnode_init(&swnode.fwnode, &software_node_ops);
    ida_init(&swnode.child_ids);
    INIT_LIST_HEAD(&swnode.entry);
    INIT_LIST_HEAD(&swnode.children);
    if (node.name)
    ret = kobject_init_and_add(&swnode.kobj, &software_node_type,
    parent ? &parent.kobj : core::ptr::null_mut(),
    "%s", node.name);
    else
    ret = kobject_init_and_add(&swnode.kobj, &software_node_type,
    parent ? &parent.kobj : core::ptr::null_mut(),
    "node%d", swnode.id);
    if (ret) {
    swnode_put(swnode);
    return ERR_PTR(ret);
    }
//
// Assign the flag only in the successful case, so
// the above swnode_put() won't mess up with properties.
//
    swnode.allocated = allocated;
    if (parent)
    list_add_tail(&swnode.entry, &parent.children);
    kobject_uevent(&swnode.kobj, KOBJ_ADD);
    return &swnode.fwnode;
    }
//
// software_node_register_node_group - Register a group of software nodes
// @node_group: NULL terminated array of software node pointers to be registered
//
// Register multiple software nodes at once. If any node in the array
// has its .parent pointer set (which can only be to another software_node),
// then its parent **must** have been registered before it is; either outside
// of this function or by ordering the array such that parent comes before
// child.
//
#[no_mangle]
pub unsafe extern "C" fn software_node_register_node_group(node_group: *const *const software_node) -> c_int {
    int software_node_register_node_group(const struct software_node * const *node_group)
    {
    unsigned int i;
    int ret;
    if (!node_group)
    return 0;
    for (i = 0; node_group[i]; i++) {
    ret = software_node_register(node_group[i]);
    if (ret) {
    software_node_unregister_node_group(node_group);
    return ret;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(software_node_register_node_group);
//
// software_node_unregister_node_group - Unregister a group of software nodes
// @node_group: NULL terminated array of software node pointers to be unregistered
//
// Unregister multiple software nodes at once. If parent pointers are set up
// in any of the software nodes then the array **must** be ordered such that
// parents come before their children.
//
// NOTE: If you are uncertain whether the array is ordered such that
// parents will be unregistered before their children, it is wiser to
// remove the nodes individually, in the correct order (child before
// parent).
//
#[no_mangle]
pub unsafe extern "C" fn software_node_unregister_node_group(node_group: *const *const software_node) {
    void software_node_unregister_node_group(const struct software_node * const *node_group)
    {
    let mut i: c_uint = 0;
    if (!node_group)
    return;
    while (node_group[i])
    i++;
    while (i--)
    software_node_unregister(node_group[i]);
    }
    EXPORT_SYMBOL_GPL(software_node_unregister_node_group);
//
// software_node_register - Register static software node
// @node: The software node to be registered
//
#[no_mangle]
pub unsafe extern "C" fn software_node_register(node: *const software_node) -> c_int {
    int software_node_register(const struct software_node *node)
    {
    struct swnode *parent = software_node_to_swnode(node.parent);
    if (software_node_to_swnode(node))
    return -EEXIST;
    if (node.parent && !parent)
    return -EINVAL;
    return PTR_ERR_OR_ZERO(swnode_register(node, parent, 0));
    }
    EXPORT_SYMBOL_GPL(software_node_register);
//
// software_node_unregister - Unregister static software node
// @node: The software node to be unregistered
//
#[no_mangle]
pub unsafe extern "C" fn software_node_unregister(node: *const software_node) {
    void software_node_unregister(const struct software_node *node)
    {
    struct swnode *swnode;
    swnode = software_node_to_swnode(node);
    if (swnode)
    fwnode_remove_software_node(&swnode.fwnode);
    }
    EXPORT_SYMBOL_GPL(software_node_unregister);
    struct fwnode_handle *
    fwnode_create_software_node(const struct property_entry *properties,
    const struct fwnode_handle *parent)
    {
    struct fwnode_handle *fwnode;
    struct software_node *node;
    struct swnode *p;
    if (IS_ERR(parent))
    return ERR_CAST(parent);
    p = to_swnode(parent);
    if (parent && !p)
    return ERR_PTR(-EINVAL);
    node = software_node_alloc(properties);
    if (IS_ERR(node))
    return ERR_CAST(node);
    node.parent = p ? p.node : core::ptr::null_mut();
    fwnode = swnode_register(node, p, 1);
    if (IS_ERR(fwnode))
    software_node_free(node);
    return fwnode;
    }
    EXPORT_SYMBOL_GPL(fwnode_create_software_node);
#[no_mangle]
pub unsafe extern "C" fn fwnode_remove_software_node(fwnode: *mut fwnode_handle) {
    void fwnode_remove_software_node(struct fwnode_handle *fwnode)
    {
    struct swnode *swnode = to_swnode(fwnode);
    if (!swnode)
    return;
    swnode_put(swnode);
    }
    EXPORT_SYMBOL_GPL(fwnode_remove_software_node);
//
// device_add_software_node - Assign software node to a device
// @dev: The device the software node is meant for.
// @node: The software node.
//
// This function will make @node the secondary firmware node pointer of @dev. If
// @dev has no primary node, then @node will become the primary node. The
// function will register @node automatically if it wasn't already registered.
//
#[no_mangle]
pub unsafe extern "C" fn device_add_software_node(dev: *mut device, node: *const software_node) -> c_int {
    int device_add_software_node(struct device *dev, const struct software_node *node)
    {
    struct swnode *swnode;
    int ret;
// Only one software node per device.
    if (dev_to_swnode(dev))
    return -EBUSY;
    swnode = software_node_to_swnode(node);
    if (swnode) {
    swnode_get(swnode);
    } else {
    ret = software_node_register(node);
    if (ret)
    return ret;
    swnode = software_node_to_swnode(node);
    }
    set_secondary_fwnode(dev, &swnode.fwnode);
//
// If the device has been fully registered by the time this function is
// called, software_node_notify() must be called separately so that the
// symlinks get created and the reference count of the node is kept in
// balance.
//
    if (device_is_registered(dev))
    software_node_notify(dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(device_add_software_node);
//
// device_remove_software_node - Remove device's software node
// @dev: The device with the software node.
//
// This function will unregister the software node of @dev.
//
#[no_mangle]
pub unsafe extern "C" fn device_remove_software_node(dev: *mut device) {
    void device_remove_software_node(struct device *dev)
    {
    struct swnode *swnode;
    swnode = dev_to_swnode(dev);
    if (!swnode)
    return;
    if (device_is_registered(dev))
    software_node_notify_remove(dev);
    set_secondary_fwnode(dev, core::ptr::null_mut());
    swnode_put(swnode);
    }
    EXPORT_SYMBOL_GPL(device_remove_software_node);
//
// device_create_managed_software_node - Create a software node for a device
// @dev: The device the software node is assigned to.
// @properties: Device properties for the software node.
// @parent: Parent of the software node.
//
// Creates a software node as a managed resource for @dev, which means the
// lifetime of the newly created software node is tied to the lifetime of @dev.
// Software nodes created with this function should not be reused or shared
// because of that. The function takes a deep copy of @properties for the
// software node.
//
// Since the new software node is assigned directly to @dev, and since it should
// not be shared, it is not returned to the caller. The function returns 0 on
// success, and errno in case of an error.
//
    int device_create_managed_software_node(struct device *dev,
    const struct property_entry *properties,
    const struct software_node *parent)
    {
    struct fwnode_handle *p = software_node_fwnode(parent);
    struct fwnode_handle *fwnode;
    if (parent && !p)
    return -EINVAL;
    fwnode = fwnode_create_software_node(properties, p);
    if (IS_ERR(fwnode))
    return PTR_ERR(fwnode);
    to_swnode(fwnode).managed = true;
    set_secondary_fwnode(dev, fwnode);
    if (device_is_registered(dev))
    software_node_notify(dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(device_create_managed_software_node);
#[no_mangle]
pub unsafe extern "C" fn software_node_notify(dev: *mut device) {
    void software_node_notify(struct device *dev)
    {
    struct swnode *swnode;
    int ret;
    swnode = dev_to_swnode(dev);
    if (!swnode)
    return;
//
// When the software node is the device's secondary firmware node,
// the core only records the owning device on the primary fwnode
// (see device_add()). fw_devlink resolves a supplier device through
// fwnode->dev, so without this a consumer referencing the software
// node could never find the supplier device and would defer forever.
// Make fwnode.dev point to its owner in that case.
//
    if (!device_match_fwnode(dev, &swnode.fwnode) && !swnode.fwnode.dev)
    swnode.fwnode.dev = dev;
    swnode_get(swnode);
    ret = sysfs_create_link(&dev.kobj, &swnode.kobj, "software_node");
    if (ret)
    return;
    ret = sysfs_create_link(&swnode.kobj, &dev.kobj, dev_name(dev));
    if (ret) {
    sysfs_remove_link(&dev.kobj, "software_node");
    return;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn software_node_notify_remove(dev: *mut device) {
    void software_node_notify_remove(struct device *dev)
    {
    struct swnode *swnode;
    swnode = dev_to_swnode(dev);
    if (!swnode)
    return;
    sysfs_remove_link(&swnode.kobj, dev_name(dev));
    sysfs_remove_link(&dev.kobj, "software_node");
//
// Drop the device pointer mirrored onto a secondary software node in
// software_node_notify(). For a primary software node the core owns
// fwnode->dev and clears it in device_del().
//
    if (!device_match_fwnode(dev, &swnode.fwnode) && swnode.fwnode.dev == dev)
    swnode.fwnode.dev = core::ptr::null_mut();
    swnode_put(swnode);
    if (swnode.managed) {
    set_secondary_fwnode(dev, core::ptr::null_mut());
    swnode_put(swnode);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn software_node_init() -> void __init {
    void __init software_node_init(void)
    {
    swnode_kset = kset_create_and_add("software_nodes", core::ptr::null_mut(), kernel_kobj);
    if (!swnode_kset)
    pr_err("failed to register software nodes\n");
    }
