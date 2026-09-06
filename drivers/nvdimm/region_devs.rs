//! Automatically rewritten from C to Rust
//! Source: drivers/nvdimm/region_devs.c
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

//
// For readq() and writeq() on 32-bit builds, the hi-lo, lo-hi order is
// irrelevant.
//

    static DEFINE_PER_CPU(int, flush_idx);
    static int nvdimm_map_flush(struct device *dev, struct nvdimm *nvdimm, int dimm,
    struct nd_region_data *ndrd)
    {
    int i, j;
    dev_dbg(dev, "%s: map %d flush address%s\n", nvdimm_name(nvdimm),
    nvdimm.num_flush, nvdimm.num_flush == 1 ? "" : "es");
    for (i = 0; i < (1 << ndrd.hints_shift); i++) {
    struct resource *res = &nvdimm.flush_wpq[i];
    let mut pfn: c_ulong = PHYS_PFN(res.start);
    void __iomem *flush_page;
// check if flush hints share a page
    for (j = 0; j < i; j++) {
    struct resource *res_j = &nvdimm.flush_wpq[j];
    let mut pfn_j: c_ulong = PHYS_PFN(res_j.start);
    if (pfn == pfn_j)
    break;
    }
    if (j < i)
    flush_page = (void __iomem *) ((unsigned long)
    ndrd_get_flush_wpq(ndrd, dimm, j)
    & PAGE_MASK);
    else
    flush_page = devm_nvdimm_ioremap(dev,
    PFN_PHYS(pfn), PAGE_SIZE);
    if (!flush_page)
    return -ENXIO;
    ndrd_set_flush_wpq(ndrd, dimm, i, flush_page
    + (res.start & ~PAGE_MASK));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nd_region_invalidate_memregion(nd_region: *mut nd_region) -> c_int {
    static int nd_region_invalidate_memregion(struct nd_region *nd_region)
    {
    int i, incoherent = 0;
    for (i = 0; i < nd_region.ndr_mappings; i++) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[i];
    struct nvdimm *nvdimm = nd_mapping.nvdimm;
    if (test_bit(NDD_INCOHERENT, &nvdimm.flags)) {
    incoherent++;
    break;
    }
    }
    if (!incoherent)
    return 0;
    if (!cpu_cache_has_invalidate_memregion()) {
    if (IS_ENABLED(CONFIG_NVDIMM_SECURITY_TEST)) {
    dev_warn(
    &nd_region.dev,
    "Bypassing cpu_cache_invalidate_memergion() for testing!\n");
    goto out;
    } else {
    dev_err(&nd_region.dev,
    "Failed to synchronize CPU cache state\n");
    return -ENXIO;
    }
    }
    cpu_cache_invalidate_all();
    out:
    for (i = 0; i < nd_region.ndr_mappings; i++) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[i];
    struct nvdimm *nvdimm = nd_mapping.nvdimm;
    clear_bit(NDD_INCOHERENT, &nvdimm.flags);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_flush_data(nd_region: *mut nd_region, size: *mut usize, num_flush: *mut c_int) -> c_int {
    static int get_flush_data(struct nd_region *nd_region, size_t *size, int *num_flush)
    {
    let mut flush_data_size: usize = sizeof(void *);
    let mut _num_flush: c_int = 0;
    int i;
    guard(nvdimm_bus)(&nd_region.dev);
    for (i = 0; i < nd_region.ndr_mappings; i++) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[i];
    struct nvdimm *nvdimm = nd_mapping.nvdimm;
    if (test_bit(NDD_SECURITY_OVERWRITE, &nvdimm.flags))
    return -EBUSY;
// at least one null hint slot per-dimm for the "no-hint" case
    flush_data_size += sizeof(void *);
    _num_flush = min_not_zero(_num_flush, nvdimm.num_flush);
    if (!nvdimm.num_flush)
    continue;
    flush_data_size += nvdimm.num_flush * sizeof(void *);
    }
// size = flush_data_size;
// num_flush = _num_flush;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nd_region_activate(nd_region: *mut nd_region) -> c_int {
    int nd_region_activate(struct nd_region *nd_region)
    {
    int i, j, rc, num_flush;
    struct nd_region_data *ndrd;
    struct device *dev = &nd_region.dev;
    size_t flush_data_size;
    rc = get_flush_data(nd_region, &flush_data_size, &num_flush);
    if (rc)
    return rc;
    rc = nd_region_invalidate_memregion(nd_region);
    if (rc)
    return rc;
    ndrd = devm_kzalloc(dev, sizeof(*ndrd) + flush_data_size, GFP_KERNEL);
    if (!ndrd)
    return -ENOMEM;
    dev_set_drvdata(dev, ndrd);
    if (!num_flush)
    return 0;
    ndrd.hints_shift = ilog2(num_flush);
    for (i = 0; i < nd_region.ndr_mappings; i++) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[i];
    struct nvdimm *nvdimm = nd_mapping.nvdimm;
    let mut rc: c_int = nvdimm_map_flush(&nd_region.dev, nvdimm, i, ndrd);
    if (rc)
    return rc;
    }
//
// Clear out entries that are duplicates. This should prevent the
// extra flushings.
//
    for (i = 0; i < nd_region.ndr_mappings - 1; i++) {
// ignore if NULL already
    if (!ndrd_get_flush_wpq(ndrd, i, 0))
    continue;
    for (j = i + 1; j < nd_region.ndr_mappings; j++)
    if (ndrd_get_flush_wpq(ndrd, i, 0) ==
    ndrd_get_flush_wpq(ndrd, j, 0))
    ndrd_set_flush_wpq(ndrd, j, 0, core::ptr::null_mut());
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nd_region_release(dev: *mut device) {
    static void nd_region_release(struct device *dev)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    u16 i;
    for (i = 0; i < nd_region.ndr_mappings; i++) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[i];
    struct nvdimm *nvdimm = nd_mapping.nvdimm;
    put_device(&nvdimm.dev);
    }
    for (i = 0; i < nd_region.num_lanes; i++)
    mutex_destroy(&nd_region.lane[i].lock);
    kfree(nd_region.lane);
    if (!test_bit(ND_REGION_CXL, &nd_region.flags))
    memregion_free(nd_region.id);
    kfree(nd_region);
    }
    struct nd_region *to_nd_region(struct device *dev)
    {
    struct nd_region *nd_region = container_of(dev, struct nd_region, dev);
    WARN_ON(dev.type.release != nd_region_release);
    return nd_region;
    }
    EXPORT_SYMBOL_GPL(to_nd_region);
    struct device *nd_region_dev(struct nd_region *nd_region)
    {
    if (!nd_region)
    return core::ptr::null_mut();
    return &nd_region.dev;
    }
    EXPORT_SYMBOL_GPL(nd_region_dev);
    void *nd_region_provider_data(struct nd_region *nd_region)
    {
    return nd_region.provider_data;
    }
    EXPORT_SYMBOL_GPL(nd_region_provider_data);
//
// nd_region_to_nstype() - region to an integer namespace type
// @nd_region: region-device to interrogate
//
// This is the 'nstype' attribute of a region as well, an input to the
// MODALIAS for namespace devices, and bit number for a nvdimm_bus to match
// namespace devices with namespace drivers.
//
#[no_mangle]
pub unsafe extern "C" fn nd_region_to_nstype(nd_region: *mut nd_region) -> c_int {
    int nd_region_to_nstype(struct nd_region *nd_region)
    {
    if (is_memory(&nd_region.dev)) {
    u16 i, label;
    for (i = 0, label = 0; i < nd_region.ndr_mappings; i++) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[i];
    struct nvdimm *nvdimm = nd_mapping.nvdimm;
    if (test_bit(NDD_LABELING, &nvdimm.flags))
    label++;
    }
    if (label)
    return ND_DEVICE_NAMESPACE_PMEM;
    else
    return ND_DEVICE_NAMESPACE_IO;
    }
    return 0;
    }
    EXPORT_SYMBOL(nd_region_to_nstype);
#[no_mangle]
unsafe extern "C" fn region_size(nd_region: *mut nd_region) -> c_ulonglong {
    static unsigned long long region_size(struct nd_region *nd_region)
    {
    if (is_memory(&nd_region.dev)) {
    return nd_region.ndr_size;
    } else if (nd_region.ndr_mappings == 1) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[0];
    return nd_mapping.size;
    }
    return 0;
    }
    static ssize_t size_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    return sprintf(buf, "%llu\n", region_size(nd_region));
    }
    static DEVICE_ATTR_RO(size);
    static ssize_t deep_flush_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
//
// NOTE: in the nvdimm_has_flush() error case this attribute is
// not visible.
//
    return sprintf(buf, "%d\n", nvdimm_has_flush(nd_region));
    }
    static ssize_t deep_flush_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t len)
    {
    bool flush;
    let mut rc: c_int = kstrtobool(buf, &flush);
    struct nd_region *nd_region = to_nd_region(dev);
    if (rc)
    return rc;
    if (!flush)
    return -EINVAL;
    rc = nvdimm_flush(nd_region, core::ptr::null_mut());
    if (rc)
    return rc;
    return len;
    }
    static DEVICE_ATTR_RW(deep_flush);
    static ssize_t mappings_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    return sprintf(buf, "%d\n", nd_region.ndr_mappings);
    }
    static DEVICE_ATTR_RO(mappings);
    static ssize_t nstype_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    return sprintf(buf, "%d\n", nd_region_to_nstype(nd_region));
    }
    static DEVICE_ATTR_RO(nstype);
    static ssize_t set_cookie_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    struct nd_interleave_set *nd_set = nd_region.nd_set;
    let mut rc: isize = 0;
    if (is_memory(dev) && nd_set)
// pass, should be precluded by region_visible */;
    else
    return -ENXIO;
//
// The cookie to show depends on which specification of the
// labels we are using. If there are not labels then default to
// the v1.1 namespace label cookie definition. To read all this
// data we need to wait for probing to settle.
//
    guard(device)(dev);
    guard(nvdimm_bus)(dev);
    wait_nvdimm_bus_probe_idle(dev);
    if (nd_region.ndr_mappings) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[0];
    struct nvdimm_drvdata *ndd = to_ndd(nd_mapping);
    if (ndd) {
    struct nd_namespace_index *nsindex;
    nsindex = to_namespace_index(ndd, ndd.ns_current);
    rc = sprintf(buf, "%#llx\n",
    nd_region_interleave_set_cookie(nd_region,
    nsindex));
    }
    }
    if (rc)
    return rc;
    return sprintf(buf, "%#llx\n", nd_set.cookie1);
    }
    static DEVICE_ATTR_RO(set_cookie);
#[no_mangle]
pub unsafe extern "C" fn nd_region_available_dpa(nd_region: *mut nd_region) -> resource_size_t {
    resource_size_t nd_region_available_dpa(struct nd_region *nd_region)
    {
    resource_size_t available;
    int i;
    WARN_ON(!is_nvdimm_bus_locked(&nd_region.dev));
    available = 0;
    for (i = 0; i < nd_region.ndr_mappings; i++) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[i];
    struct nvdimm_drvdata *ndd = to_ndd(nd_mapping);
// if a dimm is disabled the available capacity is zero
    if (!ndd)
    return 0;
    available += nd_pmem_available_dpa(nd_region, nd_mapping);
    }
    return available;
    }
#[no_mangle]
pub unsafe extern "C" fn nd_region_allocatable_dpa(nd_region: *mut nd_region) -> resource_size_t {
    resource_size_t nd_region_allocatable_dpa(struct nd_region *nd_region)
    {
    let mut avail: resource_size_t = 0;
    int i;
    WARN_ON(!is_nvdimm_bus_locked(&nd_region.dev));
    for (i = 0; i < nd_region.ndr_mappings; i++) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[i];
    avail = min_not_zero(avail, nd_pmem_max_contiguous_dpa(
    nd_region, nd_mapping));
    }
    return avail * nd_region.ndr_mappings;
    }
    static ssize_t available_size_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
//
// Flush in-flight updates and grab a snapshot of the available
// size.  Of course, this value is potentially invalidated the
// memory nvdimm_bus_lock() is dropped, but that's userspace's
// problem to not race itself.
//
    guard(device)(dev);
    guard(nvdimm_bus)(dev);
    wait_nvdimm_bus_probe_idle(dev);
    return sprintf(buf, "%llu\n", nd_region_available_dpa(nd_region));
    }
    static DEVICE_ATTR_RO(available_size);
    static ssize_t max_available_extent_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    guard(device)(dev);
    guard(nvdimm_bus)(dev);
    wait_nvdimm_bus_probe_idle(dev);
    return sprintf(buf, "%llu\n", nd_region_allocatable_dpa(nd_region));
    }
    static DEVICE_ATTR_RO(max_available_extent);
    static ssize_t init_namespaces_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region_data *ndrd = dev_get_drvdata(dev);
    guard(nvdimm_bus)(dev);
    if (!ndrd)
    return -ENXIO;
    return sprintf(buf, "%d/%d\n", ndrd.ns_active, ndrd.ns_count);
    }
    static DEVICE_ATTR_RO(init_namespaces);
    static ssize_t namespace_seed_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    guard(nvdimm_bus)(dev);
    if (nd_region.ns_seed)
    return sprintf(buf, "%s\n", dev_name(nd_region.ns_seed));
    return sprintf(buf, "\n");
    }
    static DEVICE_ATTR_RO(namespace_seed);
    static ssize_t btt_seed_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    guard(nvdimm_bus)(dev);
    if (nd_region.btt_seed)
    return sprintf(buf, "%s\n", dev_name(nd_region.btt_seed));
    return sprintf(buf, "\n");
    }
    static DEVICE_ATTR_RO(btt_seed);
    static ssize_t pfn_seed_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    guard(nvdimm_bus)(dev);
    if (nd_region.pfn_seed)
    return sprintf(buf, "%s\n", dev_name(nd_region.pfn_seed));
    return sprintf(buf, "\n");
    }
    static DEVICE_ATTR_RO(pfn_seed);
    static ssize_t dax_seed_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    guard(nvdimm_bus)(dev);
    if (nd_region.dax_seed)
    return sprintf(buf, "%s\n", dev_name(nd_region.dax_seed));
    return sprintf(buf, "\n");
    }
    static DEVICE_ATTR_RO(dax_seed);
    static ssize_t read_only_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    return sprintf(buf, "%d\n", nd_region.ro);
    }
#[no_mangle]
unsafe extern "C" fn revalidate_read_only(dev: *mut device, data: *mut c_void) -> c_int {
    static int revalidate_read_only(struct device *dev, void *data)
    {
    nd_device_notify(dev, NVDIMM_REVALIDATE_REGION);
    return 0;
    }
    static ssize_t read_only_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t len)
    {
    bool ro;
    let mut rc: c_int = kstrtobool(buf, &ro);
    struct nd_region *nd_region = to_nd_region(dev);
    if (rc)
    return rc;
    nd_region.ro = ro;
    device_for_each_child(dev, core::ptr::null_mut(), revalidate_read_only);
    return len;
    }
    static DEVICE_ATTR_RW(read_only);
    static ssize_t align_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    return sprintf(buf, "%#lx\n", nd_region.align);
    }
    static ssize_t align_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t len)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    unsigned long val, dpa;
    u32 mappings, remainder;
    int rc;
    rc = kstrtoul(buf, 0, &val);
    if (rc)
    return rc;
//
// Ensure space-align is evenly divisible by the region
// interleave-width because the kernel typically has no facility
// to determine which DIMM(s), dimm-physical-addresses, would
// contribute to the tail capacity in system-physical-address
// space for the namespace.
//
    mappings = max_t(u32, 1, nd_region.ndr_mappings);
    dpa = div_u64_rem(val, mappings, &remainder);
    if (!is_power_of_2(dpa) || dpa < PAGE_SIZE
    || val > region_size(nd_region) || remainder)
    return -EINVAL;
//
// Given that space allocation consults this value multiple
// times ensure it does not change for the duration of the
// allocation.
//
    guard(nvdimm_bus)(dev);
    nd_region.align = val;
    return len;
    }
    static DEVICE_ATTR_RW(align);
    static ssize_t region_badblocks_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    ssize_t rc;
    device_lock(dev);
    if (dev.driver)
    rc = badblocks_show(&nd_region.bb, buf, 0);
    else
    rc = -ENXIO;
    device_unlock(dev);
    return rc;
    }
    static DEVICE_ATTR(badblocks, 0444, region_badblocks_show, core::ptr::null_mut());
    static ssize_t resource_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    return sprintf(buf, "%#llx\n", nd_region.ndr_start);
    }
    static DEVICE_ATTR_ADMIN_RO(resource);
    static ssize_t persistence_domain_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    if (test_bit(ND_REGION_PERSIST_CACHE, &nd_region.flags))
    return sprintf(buf, "cpu_cache\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: test_bit(ND_REGION_PERSIST_MEMCTRL, _arg: &nd_region->flags)) -> else {
    else if (test_bit(ND_REGION_PERSIST_MEMCTRL, &nd_region.flags))
    return sprintf(buf, "memory_controller\n");
    else
    return sprintf(buf, "\n");
    }
    static DEVICE_ATTR_RO(persistence_domain);
    static struct attribute *nd_region_attributes[] = {
    &dev_attr_size.attr,
    &dev_attr_align.attr,
    &dev_attr_nstype.attr,
    &dev_attr_mappings.attr,
    &dev_attr_btt_seed.attr,
    &dev_attr_pfn_seed.attr,
    &dev_attr_dax_seed.attr,
    &dev_attr_deep_flush.attr,
    &dev_attr_read_only.attr,
    &dev_attr_set_cookie.attr,
    &dev_attr_available_size.attr,
    &dev_attr_max_available_extent.attr,
    &dev_attr_namespace_seed.attr,
    &dev_attr_init_namespaces.attr,
    &dev_attr_badblocks.attr,
    &dev_attr_resource.attr,
    &dev_attr_persistence_domain.attr,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn region_visible(kobj: *mut kobject, a: *mut attribute, n: c_int) -> umode_t {
    static umode_t region_visible(struct kobject *kobj, struct attribute *a, int n)
    {
    struct device *dev = container_of(kobj, typeof(*dev), kobj);
    struct nd_region *nd_region = to_nd_region(dev);
    struct nd_interleave_set *nd_set = nd_region.nd_set;
    let mut type: c_int = nd_region_to_nstype(nd_region);
    if (!is_memory(dev) && a == &dev_attr_pfn_seed.attr)
    return 0;
    if (!is_memory(dev) && a == &dev_attr_dax_seed.attr)
    return 0;
    if (!is_memory(dev) && a == &dev_attr_badblocks.attr)
    return 0;
    if (a == &dev_attr_resource.attr && !is_memory(dev))
    return 0;
    if (a == &dev_attr_deep_flush.attr) {
    let mut has_flush: c_int = nvdimm_has_flush(nd_region);
    if (has_flush == 1)
    return a.mode;
#[no_mangle]
pub unsafe extern "C" fn if(0: has_flush ==) -> else {
    else if (has_flush == 0)
    return 0444;
    else
    return 0;
    }
    if (a == &dev_attr_persistence_domain.attr) {
    if ((nd_region.flags & (BIT(ND_REGION_PERSIST_CACHE)
    | BIT(ND_REGION_PERSIST_MEMCTRL))) == 0)
    return 0;
    return a.mode;
    }
    if (a == &dev_attr_align.attr)
    return a.mode;
    if (a != &dev_attr_set_cookie.attr
    && a != &dev_attr_available_size.attr)
    return a.mode;
    if (type == ND_DEVICE_NAMESPACE_PMEM &&
    a == &dev_attr_available_size.attr)
    return a.mode;
#[no_mangle]
pub unsafe extern "C" fn if(nd_set: is_memory(dev) &&) -> else {
    else if (is_memory(dev) && nd_set)
    return a.mode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mappingN(dev: *mut device, buf: *mut c_char, n: c_int) -> isize {
    static ssize_t mappingN(struct device *dev, char *buf, int n)
    {
    struct nd_region *nd_region = to_nd_region(dev);
    struct nd_mapping *nd_mapping;
    struct nvdimm *nvdimm;
    if (n >= nd_region.ndr_mappings)
    return -ENXIO;
    nd_mapping = &nd_region.mapping[n];
    nvdimm = nd_mapping.nvdimm;
    return sprintf(buf, "%s,%llu,%llu,%d\n", dev_name(&nvdimm.dev),
    nd_mapping.start, nd_mapping.size,
    nd_mapping.position);
    }

    static ssize_t mapping##idx##_show(struct device *dev,		\
    struct device_attribute *attr, char *buf)	\
    {								\
    return mappingN(dev, buf, idx);				\
    }								\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: mapping##idx) -> static {
    static DEVICE_ATTR_RO(mapping##idx)
//
// 32 should be enough for a while, even in the presence of socket
// interleave a 32-way interleave set is a degenerate case.
//
    REGION_MAPPING(0);
    REGION_MAPPING(1);
    REGION_MAPPING(2);
    REGION_MAPPING(3);
    REGION_MAPPING(4);
    REGION_MAPPING(5);
    REGION_MAPPING(6);
    REGION_MAPPING(7);
    REGION_MAPPING(8);
    REGION_MAPPING(9);
    REGION_MAPPING(10);
    REGION_MAPPING(11);
    REGION_MAPPING(12);
    REGION_MAPPING(13);
    REGION_MAPPING(14);
    REGION_MAPPING(15);
    REGION_MAPPING(16);
    REGION_MAPPING(17);
    REGION_MAPPING(18);
    REGION_MAPPING(19);
    REGION_MAPPING(20);
    REGION_MAPPING(21);
    REGION_MAPPING(22);
    REGION_MAPPING(23);
    REGION_MAPPING(24);
    REGION_MAPPING(25);
    REGION_MAPPING(26);
    REGION_MAPPING(27);
    REGION_MAPPING(28);
    REGION_MAPPING(29);
    REGION_MAPPING(30);
    REGION_MAPPING(31);
#[no_mangle]
unsafe extern "C" fn mapping_visible(kobj: *mut kobject, a: *mut attribute, n: c_int) -> umode_t {
    static umode_t mapping_visible(struct kobject *kobj, struct attribute *a, int n)
    {
    struct device *dev = container_of(kobj, struct device, kobj);
    struct nd_region *nd_region = to_nd_region(dev);
    if (n < nd_region.ndr_mappings)
    return a.mode;
    return 0;
    }
    static struct attribute *mapping_attributes[] = {
    &dev_attr_mapping0.attr,
    &dev_attr_mapping1.attr,
    &dev_attr_mapping2.attr,
    &dev_attr_mapping3.attr,
    &dev_attr_mapping4.attr,
    &dev_attr_mapping5.attr,
    &dev_attr_mapping6.attr,
    &dev_attr_mapping7.attr,
    &dev_attr_mapping8.attr,
    &dev_attr_mapping9.attr,
    &dev_attr_mapping10.attr,
    &dev_attr_mapping11.attr,
    &dev_attr_mapping12.attr,
    &dev_attr_mapping13.attr,
    &dev_attr_mapping14.attr,
    &dev_attr_mapping15.attr,
    &dev_attr_mapping16.attr,
    &dev_attr_mapping17.attr,
    &dev_attr_mapping18.attr,
    &dev_attr_mapping19.attr,
    &dev_attr_mapping20.attr,
    &dev_attr_mapping21.attr,
    &dev_attr_mapping22.attr,
    &dev_attr_mapping23.attr,
    &dev_attr_mapping24.attr,
    &dev_attr_mapping25.attr,
    &dev_attr_mapping26.attr,
    &dev_attr_mapping27.attr,
    &dev_attr_mapping28.attr,
    &dev_attr_mapping29.attr,
    &dev_attr_mapping30.attr,
    &dev_attr_mapping31.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nd_mapping_attribute_group = {
    .is_visible = mapping_visible,
    .attrs = mapping_attributes,
    };
    static const struct attribute_group nd_region_attribute_group = {
    .attrs = nd_region_attributes,
    .is_visible = region_visible,
    };
    static const struct attribute_group *nd_region_attribute_groups[] = {
    &nd_device_attribute_group,
    &nd_region_attribute_group,
    &nd_numa_attribute_group,
    &nd_mapping_attribute_group,
    core::ptr::null_mut(),
    };
    static const struct device_type nd_pmem_device_type = {
    .name = "nd_pmem",
    .release = nd_region_release,
    .groups = nd_region_attribute_groups,
    };
    static const struct device_type nd_volatile_device_type = {
    .name = "nd_volatile",
    .release = nd_region_release,
    .groups = nd_region_attribute_groups,
    };
#[no_mangle]
pub unsafe extern "C" fn is_nd_pmem(dev: *const device) -> bool {
    bool is_nd_pmem(const struct device *dev)
    {
    return dev ? dev.type == &nd_pmem_device_type : false;
    }
#[no_mangle]
pub unsafe extern "C" fn is_nd_volatile(dev: *const device) -> bool {
    bool is_nd_volatile(const struct device *dev)
    {
    return dev ? dev.type == &nd_volatile_device_type : false;
    }
    u64 nd_region_interleave_set_cookie(struct nd_region *nd_region,
    struct nd_namespace_index *nsindex)
    {
    struct nd_interleave_set *nd_set = nd_region.nd_set;
    if (!nd_set)
    return 0;
    if (nsindex && __le16_to_cpu(nsindex.major) == 1
    && __le16_to_cpu(nsindex.minor) == 1)
    return nd_set.cookie1;
    return nd_set.cookie2;
    }
#[no_mangle]
pub unsafe extern "C" fn nd_region_interleave_set_altcookie(nd_region: *mut nd_region) -> u64 {
    u64 nd_region_interleave_set_altcookie(struct nd_region *nd_region)
    {
    struct nd_interleave_set *nd_set = nd_region.nd_set;
    if (nd_set)
    return nd_set.altcookie;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nd_mapping_free_labels(nd_mapping: *mut nd_mapping) {
    void nd_mapping_free_labels(struct nd_mapping *nd_mapping)
    {
    struct nd_label_ent *label_ent, *e;
    lockdep_assert_held(&nd_mapping.lock);
    list_for_each_entry_safe(label_ent, e, &nd_mapping.labels, list) {
    list_del(&label_ent.list);
    kfree(label_ent);
    }
    }
//
// When a namespace is activated create new seeds for the next
// namespace, or namespace-personality to be configured.
//
#[no_mangle]
pub unsafe extern "C" fn nd_region_advance_seeds(nd_region: *mut nd_region, dev: *mut device) {
    void nd_region_advance_seeds(struct nd_region *nd_region, struct device *dev)
    {
    guard(nvdimm_bus)(dev);
    if (nd_region.ns_seed == dev) {
    nd_region_create_ns_seed(nd_region);
    } else if (is_nd_btt(dev)) {
    struct nd_btt *nd_btt = to_nd_btt(dev);
    if (nd_region.btt_seed == dev)
    nd_region_create_btt_seed(nd_region);
    if (nd_region.ns_seed == &nd_btt.ndns.dev)
    nd_region_create_ns_seed(nd_region);
    } else if (is_nd_pfn(dev)) {
    struct nd_pfn *nd_pfn = to_nd_pfn(dev);
    if (nd_region.pfn_seed == dev)
    nd_region_create_pfn_seed(nd_region);
    if (nd_region.ns_seed == &nd_pfn.ndns.dev)
    nd_region_create_ns_seed(nd_region);
    } else if (is_nd_dax(dev)) {
    struct nd_dax *nd_dax = to_nd_dax(dev);
    if (nd_region.dax_seed == dev)
    nd_region_create_dax_seed(nd_region);
    if (nd_region.ns_seed == &nd_dax.nd_pfn.ndns.dev)
    nd_region_create_ns_seed(nd_region);
    }
    }
//
// nd_region_acquire_lane - allocate and lock a lane
// @nd_region: region id and number of lanes possible
//
// A lane correlates to a log slot in the BTT. Lanes are shared across
// CPUs using a static lane = cpu % num_lanes mapping, with a per-lane
// mutex to serialize access.
//
// Callers must be in sleepable context. The only in-tree caller is
// BTT's ->submit_bio handler (btt_read_pg / btt_write_pg).
//
#[no_mangle]
pub unsafe extern "C" fn nd_region_acquire_lane(nd_region: *mut nd_region) -> c_uint {
    unsigned int nd_region_acquire_lane(struct nd_region *nd_region)
    __acquires(&nd_region.lane[lane].lock)
    {
    unsigned int lane;
    might_sleep();
    lane = raw_smp_processor_id() % nd_region.num_lanes;
    mutex_lock(&nd_region.lane[lane].lock);
    return lane;
    }
    EXPORT_SYMBOL(nd_region_acquire_lane);
#[no_mangle]
pub unsafe extern "C" fn nd_region_release_lane(nd_region: *mut nd_region, lane: c_uint) {
    void nd_region_release_lane(struct nd_region *nd_region, unsigned int lane)
    __releases(&nd_region.lane[lane].lock)
    {
    mutex_unlock(&nd_region.lane[lane].lock);
    }
    EXPORT_SYMBOL(nd_region_release_lane);
//
// PowerPC requires this alignment for memremap_pages(). All other archs
// should be ok with SUBSECTION_SIZE (see memremap_compat_align()).
//

#[no_mangle]
unsafe extern "C" fn default_align(nd_region: *mut nd_region) -> c_ulong {
    static unsigned long default_align(struct nd_region *nd_region)
    {
    unsigned long align;
    u32 remainder;
    int mappings;
    align = MEMREMAP_COMPAT_ALIGN_MAX;
    if (nd_region.ndr_size < MEMREMAP_COMPAT_ALIGN_MAX)
    align = PAGE_SIZE;
    mappings = max_t(u16, 1, nd_region.ndr_mappings);
    div_u64_rem(align, mappings, &remainder);
    if (remainder)
    align *= mappings;
    return align;
    }
    static struct lock_class_key nvdimm_region_key;
    static struct nd_region *nd_region_create(struct nvdimm_bus *nvdimm_bus,
    struct nd_region_desc *ndr_desc,
    const struct device_type *dev_type, const char *caller)
    {
    struct nd_region *nd_region;
    struct device *dev;
    unsigned int i;
    let mut ro: c_int = 0;
    for (i = 0; i < ndr_desc.num_mappings; i++) {
    struct nd_mapping_desc *mapping = &ndr_desc.mapping[i];
    struct nvdimm *nvdimm = mapping.nvdimm;
    if ((mapping.start | mapping.size) % PAGE_SIZE) {
    dev_err(&nvdimm_bus.dev,
    "%s: %s mapping%d is not %ld aligned\n",
    caller, dev_name(&nvdimm.dev), i, PAGE_SIZE);
    return core::ptr::null_mut();
    }
    if (test_bit(NDD_UNARMED, &nvdimm.flags))
    ro = 1;
    }
    nd_region =
    kzalloc_flex(*nd_region, mapping, ndr_desc.num_mappings);
    if (!nd_region)
    return core::ptr::null_mut();
    nd_region.ndr_mappings = ndr_desc.num_mappings;
// CXL pre-assigns memregion ids before creating nvdimm regions
    if (test_bit(ND_REGION_CXL, &ndr_desc.flags)) {
    nd_region.id = ndr_desc.memregion;
    } else {
    nd_region.id = memregion_alloc(GFP_KERNEL);
    if (nd_region.id < 0)
    goto err_id;
    }
    nd_region.num_lanes = ndr_desc.num_lanes;
    if (!nd_region.num_lanes)
    goto err_percpu;
    nd_region.lane = kzalloc_objs(*nd_region.lane, nd_region.num_lanes);
    if (!nd_region.lane)
    goto err_percpu;
    for (i = 0; i < nd_region.num_lanes; i++)
    mutex_init(&nd_region.lane[i].lock);
    for (i = 0; i < ndr_desc.num_mappings; i++) {
    struct nd_mapping_desc *mapping = &ndr_desc.mapping[i];
    struct nvdimm *nvdimm = mapping.nvdimm;
    nd_region.mapping[i].nvdimm = nvdimm;
    nd_region.mapping[i].start = mapping.start;
    nd_region.mapping[i].size = mapping.size;
    nd_region.mapping[i].position = mapping.position;
    INIT_LIST_HEAD(&nd_region.mapping[i].labels);
    mutex_init(&nd_region.mapping[i].lock);
    get_device(&nvdimm.dev);
    }
    nd_region.provider_data = ndr_desc.provider_data;
    nd_region.nd_set = ndr_desc.nd_set;
    nd_region.flags = ndr_desc.flags;
    nd_region.ro = ro;
    nd_region.numa_node = ndr_desc.numa_node;
    nd_region.target_node = ndr_desc.target_node;
    ida_init(&nd_region.ns_ida);
    ida_init(&nd_region.btt_ida);
    ida_init(&nd_region.pfn_ida);
    ida_init(&nd_region.dax_ida);
    dev = &nd_region.dev;
    dev_set_name(dev, "region%d", nd_region.id);
    dev.parent = &nvdimm_bus.dev;
    dev.type = dev_type;
    dev.groups = ndr_desc.attr_groups;
    dev.of_node = ndr_desc.of_node;
    nd_region.ndr_size = resource_size(ndr_desc.res);
    nd_region.ndr_start = ndr_desc.res.start;
    nd_region.align = default_align(nd_region);
    if (ndr_desc.flush)
    nd_region.flush = ndr_desc.flush;
    else
    nd_region.flush = core::ptr::null_mut();
    device_initialize(dev);
    lockdep_set_class(&dev.mutex, &nvdimm_region_key);
    nd_device_register(dev);
    return nd_region;
    err_percpu:
    if (!test_bit(ND_REGION_CXL, &ndr_desc.flags))
    memregion_free(nd_region.id);
    err_id:
    kfree(nd_region);
    return core::ptr::null_mut();
    }
    struct nd_region *nvdimm_pmem_region_create(struct nvdimm_bus *nvdimm_bus,
    struct nd_region_desc *ndr_desc)
    {
    ndr_desc.num_lanes = ND_MAX_LANES;
    return nd_region_create(nvdimm_bus, ndr_desc, &nd_pmem_device_type,
    __func__);
    }
    EXPORT_SYMBOL_GPL(nvdimm_pmem_region_create);
    struct nd_region *nvdimm_volatile_region_create(struct nvdimm_bus *nvdimm_bus,
    struct nd_region_desc *ndr_desc)
    {
    ndr_desc.num_lanes = ND_MAX_LANES;
    return nd_region_create(nvdimm_bus, ndr_desc, &nd_volatile_device_type,
    __func__);
    }
    EXPORT_SYMBOL_GPL(nvdimm_volatile_region_create);
#[no_mangle]
pub unsafe extern "C" fn nvdimm_region_delete(nd_region: *mut nd_region) {
    void nvdimm_region_delete(struct nd_region *nd_region)
    {
    if (nd_region)
    nd_device_unregister(&nd_region.dev, ND_SYNC);
    }
    EXPORT_SYMBOL_GPL(nvdimm_region_delete);
#[no_mangle]
pub unsafe extern "C" fn nvdimm_flush(nd_region: *mut nd_region, bio: *mut bio) -> c_int {
    int nvdimm_flush(struct nd_region *nd_region, struct bio *bio)
    {
    let mut rc: c_int = 0;
    if (!nd_region.flush)
    rc = generic_nvdimm_flush(nd_region);
    else {
    rc = nd_region.flush(nd_region, bio);
    if (rc > 0)
    return rc;
    if (rc && rc != -ENOMEM)
    rc = -EIO;
    }
    return rc;
    }
//
// generic_nvdimm_flush() - flush any posted write queues between the cpu and pmem media
// @nd_region: interleaved pmem region
//
#[no_mangle]
pub unsafe extern "C" fn generic_nvdimm_flush(nd_region: *mut nd_region) -> c_int {
    int generic_nvdimm_flush(struct nd_region *nd_region)
    {
    struct nd_region_data *ndrd = dev_get_drvdata(&nd_region.dev);
    int i, idx;
//
// Try to encourage some diversity in flush hint addresses
// across cpus assuming a limited number of flush hints.
//
    idx = this_cpu_read(flush_idx);
    idx = this_cpu_add_return(flush_idx, hash_32(current.pid + idx, 8));
//
// The pmem_wmb() is needed to 'sfence' all
// previous writes such that they are architecturally visible for
// the platform buffer flush. Note that we've already arranged for pmem
// writes to avoid the cache via memcpy_flushcache().  The final
// wmb() ensures ordering for the NVDIMM flush write.
//
    pmem_wmb();
    for (i = 0; i < nd_region.ndr_mappings; i++)
    if (ndrd_get_flush_wpq(ndrd, i, 0))
    writeq(1, ndrd_get_flush_wpq(ndrd, i, idx));
    wmb();
    return 0;
    }
    EXPORT_SYMBOL_GPL(nvdimm_flush);
//
// nvdimm_has_flush - determine write flushing requirements
// @nd_region: interleaved pmem region
//
// Returns 1 if writes require flushing
// Returns 0 if writes do not require flushing
// Returns -ENXIO if flushing capability can not be determined
//
#[no_mangle]
pub unsafe extern "C" fn nvdimm_has_flush(nd_region: *mut nd_region) -> c_int {
    int nvdimm_has_flush(struct nd_region *nd_region)
    {
    int i;
// no nvdimm or pmem api == flushing capability unknown
    if (nd_region.ndr_mappings == 0
    || !IS_ENABLED(CONFIG_ARCH_HAS_PMEM_API))
    return -ENXIO;
// Test if an explicit flush function is defined
    if (test_bit(ND_REGION_ASYNC, &nd_region.flags) && nd_region.flush)
    return 1;
// Test if any flush hints for the region are available
    for (i = 0; i < nd_region.ndr_mappings; i++) {
    struct nd_mapping *nd_mapping = &nd_region.mapping[i];
    struct nvdimm *nvdimm = nd_mapping.nvdimm;
// flush hints present / available
    if (nvdimm.num_flush)
    return 1;
    }
//
// The platform defines dimm devices without hints nor explicit flush,
// assume platform persistence mechanism like ADR
//
    return 0;
    }
    EXPORT_SYMBOL_GPL(nvdimm_has_flush);
#[no_mangle]
pub unsafe extern "C" fn nvdimm_has_cache(nd_region: *mut nd_region) -> c_int {
    int nvdimm_has_cache(struct nd_region *nd_region)
    {
    return is_nd_pmem(&nd_region.dev) &&
    !test_bit(ND_REGION_PERSIST_CACHE, &nd_region.flags);
    }
    EXPORT_SYMBOL_GPL(nvdimm_has_cache);
#[no_mangle]
pub unsafe extern "C" fn is_nvdimm_sync(nd_region: *mut nd_region) -> bool {
    bool is_nvdimm_sync(struct nd_region *nd_region)
    {
    if (is_nd_volatile(&nd_region.dev))
    return true;
    return is_nd_pmem(&nd_region.dev) &&
    !test_bit(ND_REGION_ASYNC, &nd_region.flags);
    }
    EXPORT_SYMBOL_GPL(is_nvdimm_sync);
    MODULE_IMPORT_NS("DEVMEM");
