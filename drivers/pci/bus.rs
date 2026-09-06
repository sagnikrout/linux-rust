//! Automatically rewritten from C to Rust
//! Source: drivers/pci/bus.c
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
// From setup-res.c, by:
// Dave Rusling (david.rusling@reo.mts.dec.com)
// David Mosberger (davidm@cs.arizona.edu)
// David Miller (davem@redhat.com)
// Ivan Kokshaysky (ink@jurassic.park.msu.ru)
//

//
// The first PCI_BRIDGE_RESOURCE_NUM PCI bus resources (those that correspond
// to P2P or CardBus bridge windows) go in a table.  Additional ones (for
// buses below host bridges or subtractive decode bridges) go in the list.
// Use pci_bus_for_each_resource() to iterate through all the resources.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bus_resource {
    pub list: list_head,
    pub res: *mut resource,
}

    void pci_add_resource_offset(struct list_head *resources, struct resource *res,
    resource_size_t offset)
    {
    struct resource_entry *entry;
    entry = resource_list_create_entry(res, 0);
    if (!entry) {
    pr_err("PCI: can't add host bridge window %pR\n", res);
    return;
    }
    entry.offset = offset;
    resource_list_add_tail(entry, resources);
    }
    EXPORT_SYMBOL(pci_add_resource_offset);
#[no_mangle]
pub unsafe extern "C" fn pci_add_resource(resources: *mut list_head, res: *mut resource) {
    void pci_add_resource(struct list_head *resources, struct resource *res)
    {
    pci_add_resource_offset(resources, res, 0);
    }
    EXPORT_SYMBOL(pci_add_resource);
#[no_mangle]
pub unsafe extern "C" fn pci_free_resource_list(resources: *mut list_head) {
    void pci_free_resource_list(struct list_head *resources)
    {
    resource_list_free(resources);
    }
    EXPORT_SYMBOL(pci_free_resource_list);
#[no_mangle]
pub unsafe extern "C" fn pci_bus_add_resource(bus: *mut pci_bus, res: *mut resource) {
    void pci_bus_add_resource(struct pci_bus *bus, struct resource *res)
    {
    struct pci_bus_resource *bus_res;
    bus_res = kzalloc_obj(struct pci_bus_resource);
    if (!bus_res) {
    dev_err(&bus.dev, "can't add %pR resource\n", res);
    return;
    }
    bus_res.res = res;
    list_add_tail(&bus_res.list, &bus.resources);
    }
    struct resource *pci_bus_resource_n(const struct pci_bus *bus, int n)
    {
    struct pci_bus_resource *bus_res;
    if (n < PCI_BRIDGE_RESOURCE_NUM)
    return bus.resource[n];
    n -= PCI_BRIDGE_RESOURCE_NUM;
    list_for_each_entry(bus_res, &bus.resources, list) {
    if (n-- == 0)
    return bus_res.res;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(pci_bus_resource_n);
#[no_mangle]
pub unsafe extern "C" fn pci_bus_remove_resource(bus: *mut pci_bus, res: *mut resource) {
    void pci_bus_remove_resource(struct pci_bus *bus, struct resource *res)
    {
    struct pci_bus_resource *bus_res, *tmp;
    int i;
    for (i = 0; i < PCI_BRIDGE_RESOURCE_NUM; i++) {
    if (bus.resource[i] == res) {
    bus.resource[i] = core::ptr::null_mut();
    return;
    }
    }
    list_for_each_entry_safe(bus_res, tmp, &bus.resources, list) {
    if (bus_res.res == res) {
    list_del(&bus_res.list);
    kfree(bus_res);
    return;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pci_bus_remove_resources(bus: *mut pci_bus) {
    void pci_bus_remove_resources(struct pci_bus *bus)
    {
    int i;
    struct pci_bus_resource *bus_res, *tmp;
    for (i = 0; i < PCI_BRIDGE_RESOURCE_NUM; i++)
    bus.resource[i] = core::ptr::null_mut();
    list_for_each_entry_safe(bus_res, tmp, &bus.resources, list) {
    list_del(&bus_res.list);
    kfree(bus_res);
    }
    }
    int devm_request_pci_bus_resources(struct device *dev,
    struct list_head *resources)
    {
    struct resource_entry *win;
    struct resource *parent, *res;
    int err;
    resource_list_for_each_entry(win, resources) {
    res = win.res;
    switch (resource_type(res)) {
    case IORESOURCE_IO:
    parent = &ioport_resource;
    break;
    case IORESOURCE_MEM:
    parent = &iomem_resource;
    break;
    default:
    continue;
    }
    err = devm_request_resource(dev, parent, res);
    if (err)
    return err;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_request_pci_bus_resources);
    let mut pci_32_bit: static struct pci_bus_region = {0, 0xffffffffULL};

    static struct pci_bus_region pci_64_bit = {0,
    (pci_bus_addr_t) 0xffffffffffffffffULL};
    static struct pci_bus_region pci_high = {(pci_bus_addr_t) 0x100000000ULL,
    (pci_bus_addr_t) 0xffffffffffffffffULL};

//
// @res contains CPU addresses.  Clip it so the corresponding bus addresses
// on @bus are entirely within @region.  This is used to control the bus
// addresses of resources we allocate, e.g., we may need a resource that
// can be mapped by a 32-bit BAR.
//
    static void pci_clip_resource_to_region(struct pci_bus *bus,
    struct resource *res,
    struct pci_bus_region *region)
    {
    struct pci_bus_region r;
    pcibios_resource_to_bus(bus, &r, res);
    if (r.start < region.start)
    r.start = region.start;
    if (r.end > region.end)
    r.end = region.end;
    if (r.end < r.start)
    res.end = res.start - 1;
    else
    pcibios_bus_to_resource(bus, res, &r);
    }
    static int pci_bus_alloc_from_region(struct pci_bus *bus, struct resource *res,
    resource_size_t size, resource_size_t align,
    resource_size_t min, unsigned long type_mask,
    resource_alignf alignf,
    void *alignf_data,
    struct pci_bus_region *region)
    {
    struct resource *r, avail;
    resource_size_t max;
    int ret;
    type_mask |= IORESOURCE_TYPE_BITS;
    pci_bus_for_each_resource(bus, r) {
    let mut min_used: resource_size_t = min;
    if (!r)
    continue;
    if (r.flags & (IORESOURCE_UNSET|IORESOURCE_DISABLED))
    continue;
// type_mask must match
    if ((res.flags ^ r.flags) & type_mask)
    continue;
// We cannot allocate a non-prefetching resource
    from a pre-fetching area */
    if ((r.flags & IORESOURCE_PREFETCH) &&
    !(res.flags & IORESOURCE_PREFETCH))
    continue;
    avail = *r;
    pci_clip_resource_to_region(bus, &avail, region);
//
// "min" is typically PCIBIOS_MIN_IO or PCIBIOS_MIN_MEM to
// protect badly documented motherboard resources, but if
// this is an already-configured bridge window, its start
// overrides "min".
//
    if (avail.start)
    min_used = avail.start;
    max = avail.end;
// Don't bother if available space isn't large enough
    if (size > max - min_used + 1)
    continue;
// Ok, try it out..
    ret = allocate_resource(r, res, size, min_used, max,
    align, alignf, alignf_data);
    if (ret == 0)
    return 0;
    }
    return -ENOMEM;
    }
//
// pci_bus_alloc_resource - allocate a resource from a parent bus
// @bus: PCI bus
// @res: resource to allocate
// @size: size of resource to allocate
// @align: alignment of resource to allocate
// @min: minimum /proc/iomem address to allocate
// @type_mask: IORESOURCE_* type flags
// @alignf: resource alignment function
// @alignf_data: data argument for resource alignment function
//
// Given the PCI bus a device resides on, the size, minimum address,
// alignment and type, try to find an acceptable resource allocation
// for a specific device resource.
//
    int pci_bus_alloc_resource(struct pci_bus *bus, struct resource *res,
    resource_size_t size, resource_size_t align,
    resource_size_t min, unsigned long type_mask,
    resource_alignf alignf,
    void *alignf_data)
    {

    int rc;
    if (res.flags & IORESOURCE_MEM_64) {
    rc = pci_bus_alloc_from_region(bus, res, size, align, min,
    type_mask, alignf, alignf_data,
    &pci_high);
    if (rc == 0)
    return 0;
    return pci_bus_alloc_from_region(bus, res, size, align, min,
    type_mask, alignf, alignf_data,
    &pci_64_bit);
    }

    return pci_bus_alloc_from_region(bus, res, size, align, min,
    type_mask, alignf, alignf_data,
    &pci_32_bit);
    }
    EXPORT_SYMBOL(pci_bus_alloc_resource);
//
// The @idx resource of @dev should be a PCI-PCI bridge window.  If this
// resource fits inside a window of an upstream bridge, do nothing.  If it
// overlaps an upstream window but extends outside it, clip the resource so
// it fits completely inside.
//
#[no_mangle]
pub unsafe extern "C" fn pci_bus_clip_resource(dev: *mut pci_dev, idx: c_int) -> bool {
    bool pci_bus_clip_resource(struct pci_dev *dev, int idx)
    {
    struct pci_bus *bus = dev.bus;
    struct resource *res = &dev.resource[idx];
    let mut orig_res: resource = *res;
    struct resource *r;
    pci_bus_for_each_resource(bus, r) {
    resource_size_t start, end;
    if (!r)
    continue;
    if (resource_type(res) != resource_type(r))
    continue;
    start = max(r.start, res.start);
    end = min(r.end, res.end);
    if (start > end)
    continue;	/* no overlap */
    if (res.start == start && res.end == end)
    return false;	/* no change */
    res.start = start;
    res.end = end;
    res.flags &= ~IORESOURCE_UNSET;
    orig_res.flags &= ~IORESOURCE_UNSET;
    pci_info(dev, "%pR clipped to %pR\n", &orig_res, res);
    return true;
    }
    return false;
    }
    void __weak pcibios_resource_survey_bus(struct pci_bus *bus) { }
    void __weak pcibios_bus_add_device(struct pci_dev *pdev) { }
//
// pci_bus_add_device - start driver for a single device
// @dev: device to add
//
// This adds add sysfs entries and start device drivers
//
#[no_mangle]
pub unsafe extern "C" fn pci_bus_add_device(dev: *mut pci_dev) {
    void pci_bus_add_device(struct pci_dev *dev)
    {
    struct device_node *dn = dev.dev.of_node;
//
// Can not put in pci_device_add yet because resources
// are not assigned yet for some devices.
//
    pcibios_bus_add_device(dev);
    pci_fixup_device(pci_fixup_final, dev);
    if (pci_is_bridge(dev))
    of_pci_make_dev_node(dev);
    pci_proc_attach_device(dev);
    pci_bridge_d3_update(dev);
// Save config space for error recoverability
    pci_save_state(dev);
//
// Enable runtime PM, which potentially allows the device to
// suspend immediately, only after the PCI state has been
// configured completely.
//
    pm_runtime_enable(&dev.dev);
    if (!dn || of_device_is_available(dn))
    pci_dev_allow_binding(dev);
    device_initial_probe(&dev.dev);
    pci_dev_assign_added(dev);
    }
    EXPORT_SYMBOL_GPL(pci_bus_add_device);
//
// pci_bus_add_devices - start driver for PCI devices
// @bus: bus to check for new devices
//
// Start driver for PCI devices and add some sysfs entries.
//
#[no_mangle]
pub unsafe extern "C" fn pci_bus_add_devices(bus: *const pci_bus) {
    void pci_bus_add_devices(const struct pci_bus *bus)
    {
    struct pci_dev *dev;
    struct pci_bus *child;
    list_for_each_entry(dev, &bus.devices, bus_list) {
// Skip already-added devices
    if (pci_dev_is_added(dev))
    continue;
    pci_bus_add_device(dev);
    }
    list_for_each_entry(dev, &bus.devices, bus_list) {
// Skip if device attach failed
    if (!pci_dev_is_added(dev))
    continue;
    child = dev.subordinate;
    if (child)
    pci_bus_add_devices(child);
    }
    }
    EXPORT_SYMBOL(pci_bus_add_devices);
    static int __pci_walk_bus(struct pci_bus *top, int (*cb)(struct pci_dev *, void *),
    void *userdata)
    {
    struct pci_dev *dev;
    let mut ret: c_int = 0;
    list_for_each_entry(dev, &top.devices, bus_list) {
    ret = cb(dev, userdata);
    if (ret)
    break;
    if (dev.subordinate) {
    ret = __pci_walk_bus(dev.subordinate, cb, userdata);
    if (ret)
    break;
    }
    }
    return ret;
    }
    static int __pci_walk_bus_reverse(struct pci_bus *top,
    int (*cb)(struct pci_dev *, void *),
    void *userdata)
    {
    struct pci_dev *dev;
    let mut ret: c_int = 0;
    list_for_each_entry_reverse(dev, &top.devices, bus_list) {
    if (dev.subordinate) {
    ret = __pci_walk_bus_reverse(dev.subordinate, cb,
    userdata);
    if (ret)
    break;
    }
    ret = cb(dev, userdata);
    if (ret)
    break;
    }
    return ret;
    }
//
// pci_walk_bus - walk devices on/under bus, calling callback.
// @top: bus whose devices should be walked
// @cb: callback to be called for each device found
// @userdata: arbitrary pointer to be passed to callback
//
// Walk the given bus, including any bridged devices
// on buses under this bus.  Call the provided callback
// on each device found.
//
// We check the return of @cb each time. If it returns anything
// other than 0, we break out.
//
#[no_mangle]
pub unsafe extern "C" fn pci_walk_bus(top: *mut pci_bus, : *mut *mut int (cb)(struct pci_dev, ): *mut c_void, userdata: *mut c_void) {
    void pci_walk_bus(struct pci_bus *top, int (*cb)(struct pci_dev *, void *), void *userdata)
    {
    down_read(&pci_bus_sem);
    __pci_walk_bus(top, cb, userdata);
    up_read(&pci_bus_sem);
    }
    EXPORT_SYMBOL_GPL(pci_walk_bus);
//
// pci_walk_bus_reverse - walk devices on/under bus, calling callback.
// @top: bus whose devices should be walked
// @cb: callback to be called for each device found
// @userdata: arbitrary pointer to be passed to callback
//
// Same semantics as pci_walk_bus(), but walks the bus in reverse order.
//
    void pci_walk_bus_reverse(struct pci_bus *top,
    int (*cb)(struct pci_dev *, void *), void *userdata)
    {
    down_read(&pci_bus_sem);
    __pci_walk_bus_reverse(top, cb, userdata);
    up_read(&pci_bus_sem);
    }
    EXPORT_SYMBOL_GPL(pci_walk_bus_reverse);
#[no_mangle]
pub unsafe extern "C" fn pci_walk_bus_locked(top: *mut pci_bus, : *mut *mut int (cb)(struct pci_dev, ): *mut c_void, userdata: *mut c_void) {
    void pci_walk_bus_locked(struct pci_bus *top, int (*cb)(struct pci_dev *, void *), void *userdata)
    {
    lockdep_assert_held(&pci_bus_sem);
    __pci_walk_bus(top, cb, userdata);
    }
    struct pci_bus *pci_bus_get(struct pci_bus *bus)
    {
    if (bus)
    get_device(&bus.dev);
    return bus;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_bus_put(bus: *mut pci_bus) {
    void pci_bus_put(struct pci_bus *bus)
    {
    if (bus)
    put_device(&bus.dev);
    }
