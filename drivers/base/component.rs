//! Automatically rewritten from C to Rust
//! Source: drivers/base/component.c
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
// Componentized device handling.
//

//
// DOC: overview
//
// The component helper allows drivers to collect a pile of sub-devices,
// including their bound drivers, into an aggregate driver. Various subsystems
// already provide functions to get hold of such components, e.g.
// of_clk_get_by_name(). The component helper can be used when such a
// subsystem-specific way to find a device is not available: The component
// helper fills the niche of aggregate drivers for specific hardware, where
// further standardization into a subsystem would not be practical. The common
// example is when a logical device (e.g. a DRM display driver) is spread around
// the SoC on various components (scanout engines, blending blocks, transcoders
// for various outputs and so on).
//
// The component helper also doesn't solve runtime dependencies, e.g. for system
// suspend and resume operations. See also :ref:`device links<device_link>`.
//
// Components are registered using component_add() and unregistered with
// component_del(), usually from the driver's probe and disconnect functions.
//
// Aggregate drivers first assemble a component match list of what they need
// using component_match_add(). This is then registered as an aggregate driver
// using component_master_add_with_match(), and unregistered using
// component_master_del().
//
    struct component;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct component_match_array {
    pub data: *mut c_void,
    pub ): *mut *mut *mut int (compare)(struct device , void,
    pub ): *mut *mut *mut int (compare_typed)(struct device , int, void,
    pub ): *mut *mut *mut void (release)(struct device , void,
    pub component: *mut component,
    pub duplicate: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct component_match {
    pub alloc: usize,
    pub num: usize,
    pub compare: *mut component_match_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggregate_device {
    pub node: list_head,
    pub bound: bool,
    pub ops: *const component_master_ops,
    pub parent: *mut device,
    pub match: *mut component_match,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct component {
    pub node: list_head,
    pub adev: *mut aggregate_device,
    pub bound: bool,
    pub ops: *const component_ops,
    pub subcomponent: c_int,
    pub dev: *mut device,
}

    static DEFINE_MUTEX(component_mutex);
    static LIST_HEAD(component_list);
    static LIST_HEAD(aggregate_devices);

    static struct dentry *component_debugfs_dir;
#[no_mangle]
unsafe extern "C" fn component_devices_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int component_devices_show(struct seq_file *s, void *data)
    {
    struct aggregate_device *m = s.private;
    struct component_match *match = m.match;
    size_t i;
    mutex_lock(&component_mutex);
    seq_printf(s, "%-50s %20s\n", "aggregate_device name", "status");
    seq_puts(s, "-----------------------------------------------------------------------\n");
    seq_printf(s, "%-50s %20s\n\n",
    dev_name(m.parent), m.bound ? "bound" : "not bound");
    seq_printf(s, "%-50s %20s\n", "device name", "status");
    seq_puts(s, "-----------------------------------------------------------------------\n");
    for (i = 0; i < match.num; i++) {
    struct component *component = match.compare[i].component;
    seq_printf(s, "%-50s %20s\n",
    component ? dev_name(component.dev) : "(unknown)",
    component ? (component.bound ? "bound" : "not bound") : "not registered");
    }
    mutex_unlock(&component_mutex);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(component_devices);
#[no_mangle]
unsafe extern "C" fn component_debug_init() -> int __init {
    static int __init component_debug_init(void)
    {
    component_debugfs_dir = debugfs_create_dir("device_component", core::ptr::null_mut());
    return 0;
    }
    core_initcall(component_debug_init);
#[no_mangle]
unsafe extern "C" fn component_debugfs_add(m: *mut aggregate_device) {
    static void component_debugfs_add(struct aggregate_device *m)
    {
    debugfs_create_file(dev_name(m.parent), 0444, component_debugfs_dir, m,
    &component_devices_fops);
    }
#[no_mangle]
unsafe extern "C" fn component_debugfs_del(m: *mut aggregate_device) {
    static void component_debugfs_del(struct aggregate_device *m)
    {
    debugfs_lookup_and_remove(dev_name(m.parent), component_debugfs_dir);
    }

#[no_mangle]
unsafe extern "C" fn component_debugfs_add(m: *mut aggregate_device) {
    static void component_debugfs_add(struct aggregate_device *m)
    { }
#[no_mangle]
unsafe extern "C" fn component_debugfs_del(m: *mut aggregate_device) {
    static void component_debugfs_del(struct aggregate_device *m)
    { }

    static struct aggregate_device *__aggregate_find(struct device *parent,
    const struct component_master_ops *ops)
    {
    struct aggregate_device *m;
    list_for_each_entry(m, &aggregate_devices, node)
    if (m.parent == parent && (!ops || m.ops == ops))
    return m;
    return core::ptr::null_mut();
    }
    static struct component *find_component(struct aggregate_device *adev,
    struct component_match_array *mc)
    {
    struct component *c;
    list_for_each_entry(c, &component_list, node) {
    if (c.adev && c.adev != adev)
    continue;
    if (mc.compare && mc.compare(c.dev, mc.data))
    return c;
    if (mc.compare_typed &&
    mc.compare_typed(c.dev, c.subcomponent, mc.data))
    return c;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn find_components(adev: *mut aggregate_device) -> c_int {
    static int find_components(struct aggregate_device *adev)
    {
    struct component_match *match = adev.match;
    size_t i;
    let mut ret: c_int = 0;
//
// Scan the array of match functions and attach
// any components which are found to this adev.
//
    for (i = 0; i < match.num; i++) {
    struct component_match_array *mc = &match.compare[i];
    struct component *c;
    dev_dbg(adev.parent, "Looking for component %zu\n", i);
    if (match.compare[i].component)
    continue;
    c = find_component(adev, mc);
    if (!c) {
    ret = -ENXIO;
    break;
    }
    dev_dbg(adev.parent, "found component %s, duplicate %u\n",
    dev_name(c.dev), !!c.adev);
// Attach this component to the adev
    match.compare[i].duplicate = !!c.adev;
    match.compare[i].component = c;
    c.adev = adev;
    }
    return ret;
    }
// Detach component from associated aggregate_device
#[no_mangle]
unsafe extern "C" fn remove_component(adev: *mut aggregate_device, c: *mut component) {
    static void remove_component(struct aggregate_device *adev, struct component *c)
    {
    size_t i;
// Detach the component from this adev.
    for (i = 0; i < adev.match.num; i++)
    if (adev.match.compare[i].component == c)
    adev.match.compare[i].component = core::ptr::null_mut();
    }
//
// Try to bring up an aggregate device.  If component is NULL, we're interested
// in this aggregate device, otherwise it's a component which must be present
// to try and bring up the aggregate device.
//
// Returns 1 for successful bringup, 0 if not ready, or -ve errno.
//
    static int try_to_bring_up_aggregate_device(struct aggregate_device *adev,
    struct component *component)
    {
    int ret;
    dev_dbg(adev.parent, "trying to bring up adev\n");
    if (find_components(adev)) {
    dev_dbg(adev.parent, "master has incomplete components\n");
    return 0;
    }
    if (component && component.adev != adev) {
    dev_dbg(adev.parent, "master is not for this component (%s)\n",
    dev_name(component.dev));
    return 0;
    }
    if (!devres_open_group(adev.parent, adev, GFP_KERNEL))
    return -ENOMEM;
// Found all components
    ret = adev.ops.bind(adev.parent);
    if (ret < 0) {
    devres_release_group(adev.parent, core::ptr::null_mut());
    if (ret != -EPROBE_DEFER)
    dev_info(adev.parent, "adev bind failed: %d\n", ret);
    return ret;
    }
    devres_close_group(adev.parent, core::ptr::null_mut());
    adev.bound = true;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn try_to_bring_up_masters(component: *mut component) -> c_int {
    static int try_to_bring_up_masters(struct component *component)
    {
    struct aggregate_device *adev;
    let mut ret: c_int = 0;
    list_for_each_entry(adev, &aggregate_devices, node) {
    if (!adev.bound) {
    ret = try_to_bring_up_aggregate_device(adev, component);
    if (ret != 0)
    break;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn take_down_aggregate_device(adev: *mut aggregate_device) {
    static void take_down_aggregate_device(struct aggregate_device *adev)
    {
    if (adev.bound) {
    adev.ops.unbind(adev.parent);
    devres_release_group(adev.parent, adev);
    adev.bound = false;
    }
    }
//
// component_compare_of - A common component compare function for of_node
// @dev: component device
// @data: @compare_data from component_match_add_release()
//
// A common compare function when compare_data is device of_node. e.g.
// component_match_add_release(masterdev, &match, component_release_of,
// component_compare_of, component_dev_of_node)
//
#[no_mangle]
pub unsafe extern "C" fn component_compare_of(dev: *mut device, data: *mut c_void) -> c_int {
    int component_compare_of(struct device *dev, void *data)
    {
    return device_match_of_node(dev, data);
    }
    EXPORT_SYMBOL_GPL(component_compare_of);
//
// component_release_of - A common component release function for of_node
// @dev: component device
// @data: @compare_data from component_match_add_release()
//
// About the example, Please see component_compare_of().
//
#[no_mangle]
pub unsafe extern "C" fn component_release_of(dev: *mut device, data: *mut c_void) {
    void component_release_of(struct device *dev, void *data)
    {
    of_node_put(data);
    }
    EXPORT_SYMBOL_GPL(component_release_of);
//
// component_compare_dev - A common component compare function for dev
// @dev: component device
// @data: @compare_data from component_match_add_release()
//
// A common compare function when compare_data is struce device. e.g.
// component_match_add(masterdev, &match, component_compare_dev, component_dev)
//
#[no_mangle]
pub unsafe extern "C" fn component_compare_dev(dev: *mut device, data: *mut c_void) -> c_int {
    int component_compare_dev(struct device *dev, void *data)
    {
    let mut dev: return = = data;
    }
    EXPORT_SYMBOL_GPL(component_compare_dev);
//
// component_compare_dev_name - A common component compare function for device name
// @dev: component device
// @data: @compare_data from component_match_add_release()
//
// A common compare function when compare_data is device name string. e.g.
// component_match_add(masterdev, &match, component_compare_dev_name,
// "component_dev_name")
//
#[no_mangle]
pub unsafe extern "C" fn component_compare_dev_name(dev: *mut device, data: *mut c_void) -> c_int {
    int component_compare_dev_name(struct device *dev, void *data)
    {
    return device_match_name(dev, data);
    }
    EXPORT_SYMBOL_GPL(component_compare_dev_name);
#[no_mangle]
unsafe extern "C" fn devm_component_match_release(parent: *mut device, res: *mut c_void) {
    static void devm_component_match_release(struct device *parent, void *res)
    {
    struct component_match *match = res;
    unsigned int i;
    for (i = 0; i < match.num; i++) {
    struct component_match_array *mc = &match.compare[i];
    if (mc.release)
    mc.release(parent, mc.data);
    }
    kfree(match.compare);
    }
#[no_mangle]
unsafe extern "C" fn component_match_realloc(match: *mut component_match, num: usize) -> c_int {
    static int component_match_realloc(struct component_match *match, size_t num)
    {
    struct component_match_array *new;
    if (match.alloc == num)
    return 0;
    new = kmalloc_objs(*new, num);
    if (!new)
    return -ENOMEM;
    if (match.compare) {
    memcpy(new, match.compare, sizeof(*new) *
    min(match.num, num));
    kfree(match.compare);
    }
    match.compare = new;
    match.alloc = num;
    return 0;
    }
    static void __component_match_add(struct device *parent,
    struct component_match **matchptr,
    void (*release)(struct device *, void *),
    int (*compare)(struct device *, void *),
    int (*compare_typed)(struct device *, int, void *),
    void *compare_data)
    {
    struct component_match *match = *matchptr;
    if (IS_ERR(match))
    return;
    if (!match) {
    match = devres_alloc(devm_component_match_release,
    sizeof(*match), GFP_KERNEL);
    if (!match) {
// matchptr = ERR_PTR(-ENOMEM);
    return;
    }
    devres_add(parent, match);
// matchptr = match;
    }
    if (match.num == match.alloc) {
    let mut new_size: usize = match.alloc + 16;
    int ret;
    ret = component_match_realloc(match, new_size);
    if (ret) {
// matchptr = ERR_PTR(ret);
    return;
    }
    }
    match.compare[match.num].compare = compare;
    match.compare[match.num].compare_typed = compare_typed;
    match.compare[match.num].release = release;
    match.compare[match.num].data = compare_data;
    match.compare[match.num].component = core::ptr::null_mut();
    match.num++;
    }
//
// component_match_add_release - add a component match entry with release callback
// @parent: parent device of the aggregate driver
// @matchptr: pointer to the list of component matches
// @release: release function for @compare_data
// @compare: compare function to match against all components
// @compare_data: opaque pointer passed to the @compare function
//
// Adds a new component match to the list stored in @matchptr, which the
// aggregate driver needs to function. The list of component matches pointed to
// by @matchptr must be initialized to NULL before adding the first match. This
// only matches against components added with component_add().
//
// The allocated match list in @matchptr is automatically released using devm
// actions, where upon @release will be called to free any references held by
// @compare_data, e.g. when @compare_data is a &device_node that must be
// released with of_node_put().
//
// See also component_match_add() and component_match_add_typed().
//
    void component_match_add_release(struct device *parent,
    struct component_match **matchptr,
    void (*release)(struct device *, void *),
    int (*compare)(struct device *, void *), void *compare_data)
    {
    __component_match_add(parent, matchptr, release, compare, core::ptr::null_mut(),
    compare_data);
    }
    EXPORT_SYMBOL(component_match_add_release);
//
// component_match_add_typed - add a component match entry for a typed component
// @parent: parent device of the aggregate driver
// @matchptr: pointer to the list of component matches
// @compare_typed: compare function to match against all typed components
// @compare_data: opaque pointer passed to the @compare function
//
// Adds a new component match to the list stored in @matchptr, which the
// aggregate driver needs to function. The list of component matches pointed to
// by @matchptr must be initialized to NULL before adding the first match. This
// only matches against components added with component_add_typed().
//
// The allocated match list in @matchptr is automatically released using devm
// actions.
//
// See also component_match_add_release() and component_match_add_typed().
//
    void component_match_add_typed(struct device *parent,
    struct component_match **matchptr,
    int (*compare_typed)(struct device *, int, void *), void *compare_data)
    {
    __component_match_add(parent, matchptr, core::ptr::null_mut(), core::ptr::null_mut(), compare_typed,
    compare_data);
    }
    EXPORT_SYMBOL(component_match_add_typed);
#[no_mangle]
unsafe extern "C" fn free_aggregate_device(adev: *mut aggregate_device) {
    static void free_aggregate_device(struct aggregate_device *adev)
    {
    struct component_match *match = adev.match;
    int i;
    component_debugfs_del(adev);
    list_del(&adev.node);
    if (match) {
    for (i = 0; i < match.num; i++) {
    struct component *c = match.compare[i].component;
    if (c)
    c.adev = core::ptr::null_mut();
    }
    }
    kfree(adev);
    }
//
// component_master_add_with_match - register an aggregate driver
// @parent: parent device of the aggregate driver
// @ops: callbacks for the aggregate driver
// @match: component match list for the aggregate driver
//
// Registers a new aggregate driver consisting of the components added to @match
// by calling one of the component_match_add() functions. Once all components in
// @match are available, it will be assembled by calling
// &component_master_ops.bind from @ops. Must be unregistered by calling
// component_master_del().
//
    int component_master_add_with_match(struct device *parent,
    const struct component_master_ops *ops,
    struct component_match *match)
    {
    struct aggregate_device *adev;
    int ret;
// Reallocate the match array for its true size
    ret = component_match_realloc(match, match.num);
    if (ret)
    return ret;
    adev = kzalloc_obj(*adev);
    if (!adev)
    return -ENOMEM;
    adev.parent = parent;
    adev.ops = ops;
    adev.match = match;
    component_debugfs_add(adev);
// Add to the list of available aggregate devices.
    mutex_lock(&component_mutex);
    list_add(&adev.node, &aggregate_devices);
    ret = try_to_bring_up_aggregate_device(adev, core::ptr::null_mut());
    if (ret < 0)
    free_aggregate_device(adev);
    mutex_unlock(&component_mutex);
    return ret < 0 ? ret : 0;
    }
    EXPORT_SYMBOL_GPL(component_master_add_with_match);
//
// component_master_del - unregister an aggregate driver
// @parent: parent device of the aggregate driver
// @ops: callbacks for the aggregate driver
//
// Unregisters an aggregate driver registered with
// component_master_add_with_match(). If necessary the aggregate driver is first
// disassembled by calling &component_master_ops.unbind from @ops.
//
    void component_master_del(struct device *parent,
    const struct component_master_ops *ops)
    {
    struct aggregate_device *adev;
    mutex_lock(&component_mutex);
    adev = __aggregate_find(parent, ops);
    if (adev) {
    take_down_aggregate_device(adev);
    free_aggregate_device(adev);
    }
    mutex_unlock(&component_mutex);
    }
    EXPORT_SYMBOL_GPL(component_master_del);
    bool component_master_is_bound(struct device *parent,
    const struct component_master_ops *ops)
    {
    struct aggregate_device *adev;
    guard(mutex)(&component_mutex);
    adev = __aggregate_find(parent, ops);
    if (!adev)
    return 0;
    return adev.bound;
    }
    EXPORT_SYMBOL_GPL(component_master_is_bound);
    static void component_unbind(struct component *component,
    struct aggregate_device *adev, void *data)
    {
    if (WARN_ON(!component.bound))
    return;
    dev_dbg(adev.parent, "unbinding %s component %p (ops %ps)\n",
    dev_name(component.dev), component, component.ops);
    if (component.ops && component.ops.unbind)
    component.ops.unbind(component.dev, adev.parent, data);
    component.bound = false;
// Release all resources claimed in the binding of this component
    devres_release_group(component.dev, component);
    }
//
// component_unbind_all - unbind all components of an aggregate driver
// @parent: parent device of the aggregate driver
// @data: opaque pointer, passed to all components
//
// Unbinds all components of the aggregate device by passing @data to their
// &component_ops.unbind functions. Should be called from
// &component_master_ops.unbind.
//
#[no_mangle]
pub unsafe extern "C" fn component_unbind_all(parent: *mut device, data: *mut c_void) {
    void component_unbind_all(struct device *parent, void *data)
    {
    struct aggregate_device *adev;
    struct component *c;
    size_t i;
    WARN_ON(!mutex_is_locked(&component_mutex));
    adev = __aggregate_find(parent, core::ptr::null_mut());
    if (!adev)
    return;
// Unbind components in reverse order
    for (i = adev.match.num; i--; )
    if (!adev.match.compare[i].duplicate) {
    c = adev.match.compare[i].component;
    component_unbind(c, adev, data);
    }
    }
    EXPORT_SYMBOL_GPL(component_unbind_all);
    static int component_bind(struct component *component, struct aggregate_device *adev,
    void *data)
    {
    int ret;
//
// Each component initialises inside its own devres group.
// This allows us to roll-back a failed component without
// affecting anything else.
//
    if (!devres_open_group(adev.parent, core::ptr::null_mut(), GFP_KERNEL))
    return -ENOMEM;
//
// Also open a group for the device itself: this allows us
// to release the resources claimed against the sub-device
// at the appropriate moment.
//
    if (!devres_open_group(component.dev, component, GFP_KERNEL)) {
    devres_release_group(adev.parent, core::ptr::null_mut());
    return -ENOMEM;
    }
    dev_dbg(adev.parent, "binding %s (ops %ps)\n",
    dev_name(component.dev), component.ops);
    ret = component.ops.bind(component.dev, adev.parent, data);
    if (!ret) {
    component.bound = true;
//
// Close the component device's group so that resources
// allocated in the binding are encapsulated for removal
// at unbind.  Remove the group on the DRM device as we
// can clean those resources up independently.
//
    devres_close_group(component.dev, core::ptr::null_mut());
    devres_remove_group(adev.parent, core::ptr::null_mut());
    dev_info(adev.parent, "bound %s (ops %ps)\n",
    dev_name(component.dev), component.ops);
    } else {
    devres_release_group(component.dev, core::ptr::null_mut());
    devres_release_group(adev.parent, core::ptr::null_mut());
    if (ret != -EPROBE_DEFER)
    dev_err(adev.parent, "failed to bind %s (ops %ps): %d\n",
    dev_name(component.dev), component.ops, ret);
    }
    return ret;
    }
//
// component_bind_all - bind all components of an aggregate driver
// @parent: parent device of the aggregate driver
// @data: opaque pointer, passed to all components
//
// Binds all components of the aggregate @dev by passing @data to their
// &component_ops.bind functions. Should be called from
// &component_master_ops.bind.
//
#[no_mangle]
pub unsafe extern "C" fn component_bind_all(parent: *mut device, data: *mut c_void) -> c_int {
    int component_bind_all(struct device *parent, void *data)
    {
    struct aggregate_device *adev;
    struct component *c;
    size_t i;
    let mut ret: c_int = 0;
    WARN_ON(!mutex_is_locked(&component_mutex));
    adev = __aggregate_find(parent, core::ptr::null_mut());
    if (!adev)
    return -EINVAL;
// Bind components in match order
    for (i = 0; i < adev.match.num; i++)
    if (!adev.match.compare[i].duplicate) {
    c = adev.match.compare[i].component;
    ret = component_bind(c, adev, data);
    if (ret)
    break;
    }
    if (ret != 0) {
    for (; i > 0; i--)
    if (!adev.match.compare[i - 1].duplicate) {
    c = adev.match.compare[i - 1].component;
    component_unbind(c, adev, data);
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(component_bind_all);
    static int __component_add(struct device *dev, const struct component_ops *ops,
    int subcomponent)
    {
    struct component *component;
    int ret;
    component = kzalloc_obj(*component);
    if (!component)
    return -ENOMEM;
    component.ops = ops;
    component.dev = dev;
    component.subcomponent = subcomponent;
    dev_dbg(dev, "adding component (ops %ps)\n", ops);
    mutex_lock(&component_mutex);
    list_add_tail(&component.node, &component_list);
    ret = try_to_bring_up_masters(component);
    if (ret < 0) {
    if (component.adev)
    remove_component(component.adev, component);
    list_del(&component.node);
    kfree(component);
    }
    mutex_unlock(&component_mutex);
    return ret < 0 ? ret : 0;
    }
//
// component_add_typed - register a component
// @dev: component device
// @ops: component callbacks
// @subcomponent: nonzero identifier for subcomponents
//
// Register a new component for @dev. Functions in @ops will be call when the
// aggregate driver is ready to bind the overall driver by calling
// component_bind_all(). See also &struct component_ops.
//
// @subcomponent must be nonzero and is used to differentiate between multiple
// components registered on the same device @dev. These components are match
// using component_match_add_typed().
//
// The component needs to be unregistered at driver unload/disconnect by
// calling component_del().
//
// See also component_add().
//
    int component_add_typed(struct device *dev, const struct component_ops *ops,
    int subcomponent)
    {
    if (WARN_ON(subcomponent == 0))
    return -EINVAL;
    return __component_add(dev, ops, subcomponent);
    }
    EXPORT_SYMBOL_GPL(component_add_typed);
//
// component_add - register a component
// @dev: component device
// @ops: component callbacks
//
// Register a new component for @dev. Functions in @ops will be called when the
// aggregate driver is ready to bind the overall driver by calling
// component_bind_all(). See also &struct component_ops.
//
// The component needs to be unregistered at driver unload/disconnect by
// calling component_del().
//
// See also component_add_typed() for a variant that allows multiple different
// components on the same device.
//
#[no_mangle]
pub unsafe extern "C" fn component_add(dev: *mut device, ops: *const component_ops) -> c_int {
    int component_add(struct device *dev, const struct component_ops *ops)
    {
    return __component_add(dev, ops, 0);
    }
    EXPORT_SYMBOL_GPL(component_add);
//
// component_del - unregister a component
// @dev: component device
// @ops: component callbacks
//
// Unregister a component added with component_add(). If the component is bound
// into an aggregate driver, this will force the entire aggregate driver, including
// all its components, to be unbound.
//
#[no_mangle]
pub unsafe extern "C" fn component_del(dev: *mut device, ops: *const component_ops) {
    void component_del(struct device *dev, const struct component_ops *ops)
    {
    struct component *c, *component = core::ptr::null_mut();
    mutex_lock(&component_mutex);
    list_for_each_entry(c, &component_list, node)
    if (c.dev == dev && c.ops == ops) {
    list_del(&c.node);
    component = c;
    break;
    }
    if (component && component.adev) {
    take_down_aggregate_device(component.adev);
    remove_component(component.adev, component);
    }
    mutex_unlock(&component_mutex);
    WARN_ON(!component);
    kfree(component);
    }
    EXPORT_SYMBOL_GPL(component_del);
