//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/amd_cache_disable.c
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
// AMD L3 cache_disable_{0,1} sysfs handling
// Documentation/ABI/testing/sysfs-devices-system-cpu
//

//
// L3 cache descriptors
//
#[no_mangle]
unsafe extern "C" fn amd_calc_l3_indices(nb: *mut amd_northbridge) {
    static void amd_calc_l3_indices(struct amd_northbridge *nb)
    {
    struct amd_l3_cache *l3 = &nb.l3_cache;
    unsigned int sc0, sc1, sc2, sc3;
    let mut val: u32 = 0;
    pci_read_config_dword(nb.misc, 0x1C4, &val);
// calculate subcache sizes
    l3.subcaches[0] = sc0 = !(val & BIT(0));
    l3.subcaches[1] = sc1 = !(val & BIT(4));
    if (boot_cpu_data.x86 == 0x15) {
    l3.subcaches[0] = sc0 += !(val & BIT(1));
    l3.subcaches[1] = sc1 += !(val & BIT(5));
    }
    l3.subcaches[2] = sc2 = !(val & BIT(8))  + !(val & BIT(9));
    l3.subcaches[3] = sc3 = !(val & BIT(12)) + !(val & BIT(13));
    l3.indices = (max(max3(sc0, sc1, sc2), sc3) << 10) - 1;
    }
//
// check whether a slot used for disabling an L3 index is occupied.
// @l3: L3 cache descriptor
// @slot: slot number (0..1)
//
// @returns: the disabled index if used or negative value if slot free.
//
#[no_mangle]
unsafe extern "C" fn amd_get_l3_disable_slot(nb: *mut amd_northbridge, slot: c_uint) -> c_int {
    static int amd_get_l3_disable_slot(struct amd_northbridge *nb, unsigned int slot)
    {
    let mut reg: c_uint = 0;
    pci_read_config_dword(nb.misc, 0x1BC + slot * 4, &reg);
// check whether this slot is activated already
    if (reg & (3UL << 30))
    return reg & 0xfff;
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn show_cache_disable(ci: *mut cacheinfo, buf: *mut c_char, slot: c_uint) -> isize {
    static ssize_t show_cache_disable(struct cacheinfo *ci, char *buf, unsigned int slot)
    {
    int index;
    struct amd_northbridge *nb = ci.priv;
    index = amd_get_l3_disable_slot(nb, slot);
    if (index >= 0)
    return sysfs_emit(buf, "%d\n", index);
    return sysfs_emit(buf, "FREE\n");
    }

    static ssize_t								\
    cache_disable_##slot##_show(struct device *dev,				\
    struct device_attribute *attr, char *buf)	\
    {									\
    struct cacheinfo *ci = dev_get_drvdata(dev);			\
    return show_cache_disable(ci, buf, slot);			\
    }
    SHOW_CACHE_DISABLE(0)
    SHOW_CACHE_DISABLE(1)
    static void amd_l3_disable_index(struct amd_northbridge *nb, int cpu,
    unsigned int slot, unsigned long idx)
    {
    int i;
    idx |= BIT(30);
//
// disable index in all 4 subcaches
//
    for (i = 0; i < 4; i++) {
    let mut reg: u32 = idx | (i << 20);
    if (!nb.l3_cache.subcaches[i])
    continue;
    pci_write_config_dword(nb.misc, 0x1BC + slot * 4, reg);
//
// We need to WBINVD on a core on the node containing the L3
// cache which indices we disable therefore a simple wbinvd()
// is not sufficient.
//
    wbinvd_on_cpu(cpu);
    reg |= BIT(31);
    pci_write_config_dword(nb.misc, 0x1BC + slot * 4, reg);
    }
    }
//
// disable a L3 cache index by using a disable-slot
//
// @l3:    L3 cache descriptor
// @cpu:   A CPU on the node containing the L3 cache
// @slot:  slot number (0..1)
// @index: index to disable
//
// @return: 0 on success, error status on failure
//
    static int amd_set_l3_disable_slot(struct amd_northbridge *nb, int cpu,
    unsigned int slot, unsigned long index)
    {
    let mut ret: c_int = 0;
// check if @slot is already used or the index is already disabled
    ret = amd_get_l3_disable_slot(nb, slot);
    if (ret >= 0)
    return -EEXIST;
    if (index > nb.l3_cache.indices)
    return -EINVAL;
// check whether the other slot has disabled the same index already
    if (index == amd_get_l3_disable_slot(nb, !slot))
    return -EEXIST;
    amd_l3_disable_index(nb, cpu, slot, index);
    return 0;
    }
    static ssize_t store_cache_disable(struct cacheinfo *ci, const char *buf,
    size_t count, unsigned int slot)
    {
    struct amd_northbridge *nb = ci.priv;
    let mut val: c_ulong = 0;
    int cpu, err = 0;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    cpu = cpumask_first(&ci.shared_cpu_map);
    if (kstrtoul(buf, 10, &val) < 0)
    return -EINVAL;
    err = amd_set_l3_disable_slot(nb, cpu, slot, val);
    if (err) {
    if (err == -EEXIST)
    pr_warn("L3 slot %d in use/index already disabled!\n",
    slot);
    return err;
    }
    return count;
    }

    static ssize_t								\
    cache_disable_##slot##_store(struct device *dev,			\
    struct device_attribute *attr,		\
    const char *buf, size_t count)		\
    {									\
    struct cacheinfo *ci = dev_get_drvdata(dev);			\
    return store_cache_disable(ci, buf, count, slot);		\
    }
    STORE_CACHE_DISABLE(0)
    STORE_CACHE_DISABLE(1)
    static ssize_t subcaches_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct cacheinfo *ci = dev_get_drvdata(dev);
    let mut cpu: c_int = cpumask_first(&ci.shared_cpu_map);
    return sysfs_emit(buf, "%x\n", amd_get_subcaches(cpu));
    }
    static ssize_t subcaches_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct cacheinfo *ci = dev_get_drvdata(dev);
    let mut cpu: c_int = cpumask_first(&ci.shared_cpu_map);
    unsigned long val;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    if (kstrtoul(buf, 16, &val) < 0)
    return -EINVAL;
    if (amd_set_subcaches(cpu, val))
    return -EINVAL;
    return count;
    }
    static DEVICE_ATTR_RW(cache_disable_0);
    static DEVICE_ATTR_RW(cache_disable_1);
    static DEVICE_ATTR_RW(subcaches);
    static umode_t cache_private_attrs_is_visible(struct kobject *kobj,
    struct attribute *attr, int unused)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct cacheinfo *ci = dev_get_drvdata(dev);
    let mut mode: umode_t = attr.mode;
    if (!ci.priv)
    return 0;
    if ((attr == &dev_attr_subcaches.attr) &&
    amd_nb_has_feature(AMD_NB_L3_PARTITIONING))
    return mode;
    if ((attr == &dev_attr_cache_disable_0.attr ||
    attr == &dev_attr_cache_disable_1.attr) &&
    amd_nb_has_feature(AMD_NB_L3_INDEX_DISABLE))
    return mode;
    return 0;
    }
    static struct attribute_group cache_private_group = {
    .is_visible = cache_private_attrs_is_visible,
    };
#[no_mangle]
unsafe extern "C" fn init_amd_l3_attrs() {
    static void init_amd_l3_attrs(void)
    {
    static struct attribute **amd_l3_attrs;
    let mut n: c_int = 1;
    if (amd_l3_attrs) /* already initialized */
    return;
    if (amd_nb_has_feature(AMD_NB_L3_INDEX_DISABLE))
    n += 2;
    if (amd_nb_has_feature(AMD_NB_L3_PARTITIONING))
    n += 1;
    amd_l3_attrs = kzalloc_objs(*amd_l3_attrs, n);
    if (!amd_l3_attrs)
    return;
    n = 0;
    if (amd_nb_has_feature(AMD_NB_L3_INDEX_DISABLE)) {
    amd_l3_attrs[n++] = &dev_attr_cache_disable_0.attr;
    amd_l3_attrs[n++] = &dev_attr_cache_disable_1.attr;
    }
    if (amd_nb_has_feature(AMD_NB_L3_PARTITIONING))
    amd_l3_attrs[n++] = &dev_attr_subcaches.attr;
    cache_private_group.attrs = amd_l3_attrs;
    }
    const struct attribute_group *cache_get_priv_group(struct cacheinfo *ci)
    {
    struct amd_northbridge *nb = ci.priv;
    if (ci.level < 3 || !nb)
    return core::ptr::null_mut();
    if (nb && nb.l3_cache.indices)
    init_amd_l3_attrs();
    return &cache_private_group;
    }
    struct amd_northbridge *amd_init_l3_cache(int index)
    {
    struct amd_northbridge *nb;
    int node;
// only for L3, and not in virtualized environments
    if (index < 3)
    return core::ptr::null_mut();
    node = topology_amd_node_id(smp_processor_id());
    nb = node_to_amd_nb(node);
    if (nb && !nb.l3_cache.indices)
    amd_calc_l3_indices(nb);
    return nb;
    }
