//! Automatically rewritten from C to Rust
//! Source: drivers/nvdimm/core.c
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

    LIST_HEAD(nvdimm_bus_list);
    DEFINE_MUTEX(nvdimm_bus_list_mutex);
#[no_mangle]
pub unsafe extern "C" fn nvdimm_bus_lock(dev: *mut device) {
    void nvdimm_bus_lock(struct device *dev)
    {
    struct nvdimm_bus *nvdimm_bus = walk_to_nvdimm_bus(dev);
    if (!nvdimm_bus)
    return;
    mutex_lock(&nvdimm_bus.reconfig_mutex);
    }
    EXPORT_SYMBOL(nvdimm_bus_lock);
#[no_mangle]
pub unsafe extern "C" fn nvdimm_bus_unlock(dev: *mut device) {
    void nvdimm_bus_unlock(struct device *dev)
    {
    struct nvdimm_bus *nvdimm_bus = walk_to_nvdimm_bus(dev);
    if (!nvdimm_bus)
    return;
    mutex_unlock(&nvdimm_bus.reconfig_mutex);
    }
    EXPORT_SYMBOL(nvdimm_bus_unlock);
#[no_mangle]
pub unsafe extern "C" fn is_nvdimm_bus_locked(dev: *mut device) -> bool {
    bool is_nvdimm_bus_locked(struct device *dev)
    {
    struct nvdimm_bus *nvdimm_bus = walk_to_nvdimm_bus(dev);
    if (!nvdimm_bus)
    return false;
    return mutex_is_locked(&nvdimm_bus.reconfig_mutex);
    }
    EXPORT_SYMBOL(is_nvdimm_bus_locked);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_map {
    pub nvdimm_bus: *mut nvdimm_bus,
    pub list: list_head,
    pub offset: resource_size_t,
    pub flags: c_ulong,
    pub size: usize,
    union {
    pub mem: *mut c_void,
    pub iomem: *mut void __iomem,
}

    struct kref kref;
    };
    static struct nvdimm_map *find_nvdimm_map(struct device *dev,
    resource_size_t offset)
    {
    struct nvdimm_bus *nvdimm_bus = walk_to_nvdimm_bus(dev);
    struct nvdimm_map *nvdimm_map;
    list_for_each_entry(nvdimm_map, &nvdimm_bus.mapping_list, list)
    if (nvdimm_map.offset == offset)
    return nvdimm_map;
    return core::ptr::null_mut();
    }
    static struct nvdimm_map *alloc_nvdimm_map(struct device *dev,
    resource_size_t offset, size_t size, unsigned long flags)
    {
    struct nvdimm_bus *nvdimm_bus = walk_to_nvdimm_bus(dev);
    struct nvdimm_map *nvdimm_map;
    nvdimm_map = kzalloc_obj(*nvdimm_map);
    if (!nvdimm_map)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&nvdimm_map.list);
    nvdimm_map.nvdimm_bus = nvdimm_bus;
    nvdimm_map.offset = offset;
    nvdimm_map.flags = flags;
    nvdimm_map.size = size;
    kref_init(&nvdimm_map.kref);
    if (!request_mem_region(offset, size, dev_name(&nvdimm_bus.dev))) {
    dev_err(&nvdimm_bus.dev, "failed to request %pa + %zd for %s\n",
    &offset, size, dev_name(dev));
    goto err_request_region;
    }
    if (flags)
    nvdimm_map.mem = memremap(offset, size, flags);
    else
    nvdimm_map.iomem = ioremap(offset, size);
    if (!nvdimm_map.mem)
    goto err_map;
    dev_WARN_ONCE(dev, !is_nvdimm_bus_locked(dev), "%s: bus unlocked!",
    __func__);
    list_add(&nvdimm_map.list, &nvdimm_bus.mapping_list);
    return nvdimm_map;
    err_map:
    release_mem_region(offset, size);
    err_request_region:
    kfree(nvdimm_map);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn nvdimm_map_release(kref: *mut kref) {
    static void nvdimm_map_release(struct kref *kref)
    {
    struct nvdimm_bus *nvdimm_bus;
    struct nvdimm_map *nvdimm_map;
    nvdimm_map = container_of(kref, struct nvdimm_map, kref);
    nvdimm_bus = nvdimm_map.nvdimm_bus;
    dev_dbg(&nvdimm_bus.dev, "%pa\n", &nvdimm_map.offset);
    list_del(&nvdimm_map.list);
    if (nvdimm_map.flags)
    memunmap(nvdimm_map.mem);
    else
    iounmap(nvdimm_map.iomem);
    release_mem_region(nvdimm_map.offset, nvdimm_map.size);
    kfree(nvdimm_map);
    }
#[no_mangle]
unsafe extern "C" fn nvdimm_map_put(data: *mut c_void) {
    static void nvdimm_map_put(void *data)
    {
    struct nvdimm_map *nvdimm_map = data;
    struct nvdimm_bus *nvdimm_bus = nvdimm_map.nvdimm_bus;
    guard(nvdimm_bus)(&nvdimm_bus.dev);
    kref_put(&nvdimm_map.kref, nvdimm_map_release);
    }
//
// devm_nvdimm_memremap - map a resource that is shared across regions
// @dev: device that will own a reference to the shared mapping
// @offset: physical base address of the mapping
// @size: mapping size
// @flags: memremap flags, or, if zero, perform an ioremap instead
//
    void *devm_nvdimm_memremap(struct device *dev, resource_size_t offset,
    size_t size, unsigned long flags)
    {
    struct nvdimm_map *nvdimm_map;
    scoped_guard(nvdimm_bus, dev) {
    nvdimm_map = find_nvdimm_map(dev, offset);
    if (!nvdimm_map)
    nvdimm_map = alloc_nvdimm_map(dev, offset, size, flags);
    else
    kref_get(&nvdimm_map.kref);
    }
    if (!nvdimm_map)
    return core::ptr::null_mut();
    if (devm_add_action_or_reset(dev, nvdimm_map_put, nvdimm_map))
    return core::ptr::null_mut();
    return nvdimm_map.mem;
    }
    EXPORT_SYMBOL_GPL(devm_nvdimm_memremap);
#[no_mangle]
pub unsafe extern "C" fn nd_fletcher64(addr: *mut c_void, len: usize, le: bool) -> u64 {
    u64 nd_fletcher64(void *addr, size_t len, bool le)
    {
    u32 *buf = addr;
    let mut lo32: u32 = 0;
    let mut hi32: u64 = 0;
    int i;
    for (i = 0; i < len / sizeof(u32); i++) {
    lo32 += le ? le32_to_cpu((__le32) buf[i]) : buf[i];
    hi32 += lo32;
    }
    return hi32 << 32 | lo32;
    }
    EXPORT_SYMBOL_GPL(nd_fletcher64);
    struct nvdimm_bus_descriptor *to_nd_desc(struct nvdimm_bus *nvdimm_bus)
    {
// struct nvdimm_bus definition is private to libnvdimm
    return nvdimm_bus.nd_desc;
    }
    EXPORT_SYMBOL_GPL(to_nd_desc);
    struct device *to_nvdimm_bus_dev(struct nvdimm_bus *nvdimm_bus)
    {
// struct nvdimm_bus definition is private to libnvdimm
    return &nvdimm_bus.dev;
    }
    EXPORT_SYMBOL_GPL(to_nvdimm_bus_dev);
//
// nd_uuid_store: common implementation for writing 'uuid' sysfs attributes
// @dev: container device for the uuid property
// @uuid_out: uuid buffer to replace
// @buf: raw sysfs buffer to parse
//
// Enforce that uuids can only be changed while the device is disabled
// (driver detached)
// LOCKING: expects device_lock() is held on entry
//
    int nd_uuid_store(struct device *dev, uuid_t **uuid_out, const char *buf,
    size_t len)
    {
    uuid_t uuid;
    int rc;
    if (dev.driver)
    return -EBUSY;
    rc = uuid_parse(buf, &uuid);
    if (rc)
    return rc;
    kfree(*uuid_out);
// uuid_out = kmemdup(&uuid, sizeof(uuid), GFP_KERNEL);
    if (!(*uuid_out))
    return -ENOMEM;
    return 0;
    }
    ssize_t nd_size_select_show(unsigned long current_size,
    const unsigned long *supported, char *buf)
    {
    let mut len: isize = 0;
    int i;
    for (i = 0; supported[i]; i++)
    if (current_size == supported[i])
    len += sprintf(buf + len, "[%ld] ", supported[i]);
    else
    len += sprintf(buf + len, "%ld ", supported[i]);
    len += sprintf(buf + len, "\n");
    return len;
    }
    ssize_t nd_size_select_store(struct device *dev, const char *buf,
    unsigned long *current_size, const unsigned long *supported)
    {
    unsigned long lbasize;
    int rc, i;
    if (dev.driver)
    return -EBUSY;
    rc = kstrtoul(buf, 0, &lbasize);
    if (rc)
    return rc;
    for (i = 0; supported[i]; i++)
    if (lbasize == supported[i])
    break;
    if (supported[i]) {
// current_size = lbasize;
    return 0;
    } else {
    return -EINVAL;
    }
    }
    static ssize_t commands_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int cmd, len = 0;
    struct nvdimm_bus *nvdimm_bus = to_nvdimm_bus(dev);
    struct nvdimm_bus_descriptor *nd_desc = nvdimm_bus.nd_desc;
    for_each_set_bit(cmd, &nd_desc.cmd_mask, BITS_PER_LONG)
    len += sprintf(buf + len, "%s ", nvdimm_bus_cmd_name(cmd));
    len += sprintf(buf + len, "\n");
    return len;
    }
    static DEVICE_ATTR_RO(commands);
    static const char *nvdimm_bus_provider(struct nvdimm_bus *nvdimm_bus)
    {
    struct nvdimm_bus_descriptor *nd_desc = nvdimm_bus.nd_desc;
    struct device *parent = nvdimm_bus.dev.parent;
    if (nd_desc.provider_name)
    return nd_desc.provider_name;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: parent) -> else {
    else if (parent)
    return dev_name(parent);
    else
    return "unknown";
    }
    static ssize_t provider_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nvdimm_bus *nvdimm_bus = to_nvdimm_bus(dev);
    return sprintf(buf, "%s\n", nvdimm_bus_provider(nvdimm_bus));
    }
    static DEVICE_ATTR_RO(provider);
#[no_mangle]
unsafe extern "C" fn flush_namespaces(dev: *mut device, data: *mut c_void) -> c_int {
    static int flush_namespaces(struct device *dev, void *data)
    {
    device_lock(dev);
    device_unlock(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flush_regions_dimms(dev: *mut device, data: *mut c_void) -> c_int {
    static int flush_regions_dimms(struct device *dev, void *data)
    {
    device_lock(dev);
    device_unlock(dev);
    device_for_each_child(dev, core::ptr::null_mut(), flush_namespaces);
    return 0;
    }
    static ssize_t wait_probe_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nvdimm_bus *nvdimm_bus = to_nvdimm_bus(dev);
    struct nvdimm_bus_descriptor *nd_desc = nvdimm_bus.nd_desc;
    int rc;
    if (nd_desc.flush_probe) {
    rc = nd_desc.flush_probe(nd_desc);
    if (rc)
    return rc;
    }
    nd_synchronize();
    device_for_each_child(dev, core::ptr::null_mut(), flush_regions_dimms);
    return sprintf(buf, "1\n");
    }
    static DEVICE_ATTR_RO(wait_probe);
    static struct attribute *nvdimm_bus_attributes[] = {
    &dev_attr_commands.attr,
    &dev_attr_wait_probe.attr,
    &dev_attr_provider.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nvdimm_bus_attribute_group = {
    .attrs = nvdimm_bus_attributes,
    };
    static ssize_t capability_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nvdimm_bus *nvdimm_bus = to_nvdimm_bus(dev);
    struct nvdimm_bus_descriptor *nd_desc = nvdimm_bus.nd_desc;
    enum nvdimm_fwa_capability cap;
    if (!nd_desc.fw_ops)
    return -EOPNOTSUPP;
    cap = nd_desc.fw_ops.capability(nd_desc);
    switch (cap) {
    case NVDIMM_FWA_CAP_QUIESCE:
    return sprintf(buf, "quiesce\n");
    case NVDIMM_FWA_CAP_LIVE:
    return sprintf(buf, "live\n");
    default:
    return -EOPNOTSUPP;
    }
    }
    static DEVICE_ATTR_RO(capability);
    static ssize_t activate_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nvdimm_bus *nvdimm_bus = to_nvdimm_bus(dev);
    struct nvdimm_bus_descriptor *nd_desc = nvdimm_bus.nd_desc;
    enum nvdimm_fwa_capability cap;
    enum nvdimm_fwa_state state;
    if (!nd_desc.fw_ops)
    return -EOPNOTSUPP;
    cap = nd_desc.fw_ops.capability(nd_desc);
    state = nd_desc.fw_ops.activate_state(nd_desc);
    if (cap < NVDIMM_FWA_CAP_QUIESCE)
    return -EOPNOTSUPP;
    switch (state) {
    case NVDIMM_FWA_IDLE:
    return sprintf(buf, "idle\n");
    case NVDIMM_FWA_BUSY:
    return sprintf(buf, "busy\n");
    case NVDIMM_FWA_ARMED:
    return sprintf(buf, "armed\n");
    case NVDIMM_FWA_ARM_OVERFLOW:
    return sprintf(buf, "overflow\n");
    default:
    return -ENXIO;
    }
    }
#[no_mangle]
unsafe extern "C" fn exec_firmware_activate(data: *mut c_void) -> c_int {
    static int exec_firmware_activate(void *data)
    {
    struct nvdimm_bus_descriptor *nd_desc = data;
    return nd_desc.fw_ops.activate(nd_desc);
    }
    static ssize_t activate_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t len)
    {
    struct nvdimm_bus *nvdimm_bus = to_nvdimm_bus(dev);
    struct nvdimm_bus_descriptor *nd_desc = nvdimm_bus.nd_desc;
    enum nvdimm_fwa_state state;
    bool quiesce;
    ssize_t rc;
    if (!nd_desc.fw_ops)
    return -EOPNOTSUPP;
    if (sysfs_streq(buf, "live"))
    quiesce = false;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: "quiesce")) -> else {
    else if (sysfs_streq(buf, "quiesce"))
    quiesce = true;
    else
    return -EINVAL;
    state = nd_desc.fw_ops.activate_state(nd_desc);
    switch (state) {
    case NVDIMM_FWA_BUSY:
    rc = -EBUSY;
    break;
    case NVDIMM_FWA_ARMED:
    case NVDIMM_FWA_ARM_OVERFLOW:
    if (quiesce)
    rc = hibernate_quiet_exec(exec_firmware_activate, nd_desc);
    else
    rc = nd_desc.fw_ops.activate(nd_desc);
    break;
    case NVDIMM_FWA_IDLE:
    default:
    rc = -ENXIO;
    }
    if (rc == 0)
    rc = len;
    return rc;
    }
    static DEVICE_ATTR_ADMIN_RW(activate);
#[no_mangle]
unsafe extern "C" fn nvdimm_bus_firmware_visible(kobj: *mut kobject, a: *mut attribute, n: c_int) -> umode_t {
    static umode_t nvdimm_bus_firmware_visible(struct kobject *kobj, struct attribute *a, int n)
    {
    struct device *dev = container_of(kobj, typeof(*dev), kobj);
    struct nvdimm_bus *nvdimm_bus = to_nvdimm_bus(dev);
    struct nvdimm_bus_descriptor *nd_desc = nvdimm_bus.nd_desc;
    enum nvdimm_fwa_capability cap;
//
// Both 'activate' and 'capability' disappear when no ops
// detected, or a negative capability is indicated.
//
    if (!nd_desc.fw_ops)
    return 0;
    cap = nd_desc.fw_ops.capability(nd_desc);
    if (cap < NVDIMM_FWA_CAP_QUIESCE)
    return 0;
    return a.mode;
    }
    static struct attribute *nvdimm_bus_firmware_attributes[] = {
    &dev_attr_activate.attr,
    &dev_attr_capability.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nvdimm_bus_firmware_attribute_group = {
    .name = "firmware",
    .attrs = nvdimm_bus_firmware_attributes,
    .is_visible = nvdimm_bus_firmware_visible,
    };
    const struct attribute_group *nvdimm_bus_attribute_groups[] = {
    &nvdimm_bus_attribute_group,
    &nvdimm_bus_firmware_attribute_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn nvdimm_bus_add_badrange(nvdimm_bus: *mut nvdimm_bus, addr: u64, length: u64) -> c_int {
    int nvdimm_bus_add_badrange(struct nvdimm_bus *nvdimm_bus, u64 addr, u64 length)
    {
    return badrange_add(&nvdimm_bus.badrange, addr, length);
    }
    EXPORT_SYMBOL_GPL(nvdimm_bus_add_badrange);
#[no_mangle]
unsafe extern "C" fn libnvdimm_init() -> __init int {
    static __init int libnvdimm_init(void)
    {
    int rc;
    rc = nvdimm_bus_init();
    if (rc)
    return rc;
    rc = nvdimm_init();
    if (rc)
    goto err_dimm;
    rc = nd_region_init();
    if (rc)
    goto err_region;
    nd_label_init();
    return 0;
    err_region:
    nvdimm_exit();
    err_dimm:
    nvdimm_bus_exit();
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn libnvdimm_exit() -> __exit void {
    static __exit void libnvdimm_exit(void)
    {
    WARN_ON(!list_empty(&nvdimm_bus_list));
    nd_region_exit();
    nvdimm_exit();
    nvdimm_bus_exit();
    nvdimm_devs_exit();
    }
    MODULE_DESCRIPTION("NVDIMM (Non-Volatile Memory Device) core");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Intel Corporation");
    subsys_initcall(libnvdimm_init);
    module_exit(libnvdimm_exit);
