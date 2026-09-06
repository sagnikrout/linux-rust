//! Automatically rewritten from C to Rust
//! Source: drivers/base/cpu.c
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
// CPU subsystem support
//

    static DEFINE_PER_CPU(struct device *, cpu_sys_devices);
#[no_mangle]
unsafe extern "C" fn cpu_subsys_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int cpu_subsys_match(struct device *dev, const struct device_driver *drv)
    {
// ACPI style match is the only one that may succeed.
    if (acpi_driver_match_device(dev, drv))
    return 1;
    return 0;
    }

    static void change_cpu_under_node(struct cpu *cpu,
    unsigned int from_nid, unsigned int to_nid)
    {
    let mut cpuid: c_int = cpu.dev.id;
    unregister_cpu_under_node(cpuid, from_nid);
    register_cpu_under_node(cpuid, to_nid);
    cpu.node_id = to_nid;
    }
#[no_mangle]
unsafe extern "C" fn cpu_subsys_online(dev: *mut device) -> c_int {
    static int cpu_subsys_online(struct device *dev)
    {
    struct cpu *cpu = container_of(dev, struct cpu, dev);
    let mut cpuid: c_int = dev.id;
    int from_nid, to_nid;
    int ret;
    let mut retries: c_int = 0;
    from_nid = cpu_to_node(cpuid);
    if (from_nid == NUMA_NO_NODE)
    return -ENODEV;
    retry:
    ret = cpu_device_up(dev);
//
// If -EBUSY is returned, it is likely that hotplug is temporarily
// disabled when cpu_hotplug_disable() was called. This condition is
// transient. So we retry after waiting for an exponentially
// increasing delay up to a total of at least 620ms as some PCI
// device initialization can take quite a while.
//
    if (ret == -EBUSY) {
    retries++;
    if (retries > 5)
    return ret;
    msleep(10 * (1 << retries));
    goto retry;
    }
//
// When hot adding memory to memoryless node and enabling a cpu
// on the node, node number of the cpu may internally change.
//
    to_nid = cpu_to_node(cpuid);
    if (from_nid != to_nid)
    change_cpu_under_node(cpu, from_nid, to_nid);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpu_subsys_offline(dev: *mut device) -> c_int {
    static int cpu_subsys_offline(struct device *dev)
    {
    return cpu_device_down(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn unregister_cpu(cpu: *mut cpu) {
    void unregister_cpu(struct cpu *cpu)
    {
    let mut logical_cpu: c_int = cpu.dev.id;
    set_cpu_enabled(logical_cpu, false);
    unregister_cpu_under_node(logical_cpu, cpu_to_node(logical_cpu));
    device_unregister(&cpu.dev);
    per_cpu(cpu_sys_devices, logical_cpu) = core::ptr::null_mut();
    return;
    }

    static ssize_t cpu_probe_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t count)
    {
    ssize_t cnt;
    int ret;
    ret = lock_device_hotplug_sysfs();
    if (ret)
    return ret;
    cnt = arch_cpu_probe(buf, count);
    unlock_device_hotplug();
    return cnt;
    }
    static ssize_t cpu_release_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t count)
    {
    ssize_t cnt;
    int ret;
    ret = lock_device_hotplug_sysfs();
    if (ret)
    return ret;
    cnt = arch_cpu_release(buf, count);
    unlock_device_hotplug();
    return cnt;
    }
    static DEVICE_ATTR(probe, S_IWUSR, core::ptr::null_mut(), cpu_probe_store);
    static DEVICE_ATTR(release, S_IWUSR, core::ptr::null_mut(), cpu_release_store);

    static ssize_t crash_notes_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct cpu *cpu = container_of(dev, struct cpu, dev);
    unsigned long long addr;
    int cpunum;
    cpunum = cpu.dev.id;
//
// Might be reading other cpu's data based on which cpu read thread
// has been scheduled. But cpu data (memory) is allocated once during
// boot up and this data does not change there after. Hence this
// operation should be safe. No locking required.
//
    addr = per_cpu_ptr_to_phys(per_cpu_ptr(crash_notes, cpunum));
    return sysfs_emit(buf, "%llx\n", addr);
    }
    static DEVICE_ATTR_ADMIN_RO(crash_notes);
    static ssize_t crash_notes_size_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%zu\n", sizeof(note_buf_t));
    }
    static DEVICE_ATTR_ADMIN_RO(crash_notes_size);
    static struct attribute *crash_note_cpu_attrs[] = {
    &dev_attr_crash_notes.attr,
    &dev_attr_crash_notes_size.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group crash_note_cpu_attr_group = {
    .attrs = crash_note_cpu_attrs,
    };

    static const struct attribute_group *common_cpu_attr_groups[] = {

    &crash_note_cpu_attr_group,

    core::ptr::null_mut()
    };
    static const struct attribute_group *hotplugable_cpu_attr_groups[] = {

    &crash_note_cpu_attr_group,

    core::ptr::null_mut()
    };
//
// Print cpu online, possible, present, and system maps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_attr {
    pub attr: device_attribute,
    pub map: *const *const cpumask,
}

    static ssize_t show_cpus_attr(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct cpu_attr *ca = container_of(attr, struct cpu_attr, attr);
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(ca.map));
    }

    { __ATTR(name, 0444, show_cpus_attr, core::ptr::null_mut()), map }
// Keep in sync with cpu_subsys_attrs
    static struct cpu_attr cpu_attrs[] = {
    _CPU_ATTR(online, &__cpu_online_mask),
    _CPU_ATTR(possible, &__cpu_possible_mask),
    _CPU_ATTR(present, &__cpu_present_mask),
    };
//
// Print values for NR_CPUS and offlined cpus
//
    static ssize_t print_cpus_kernel_max(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", NR_CPUS - 1);
    }
    static DEVICE_ATTR(kernel_max, 0444, print_cpus_kernel_max, core::ptr::null_mut());
// arch-optional setting to enable display of offline cpus >= nr_cpu_ids
    unsigned int total_cpus;
    static ssize_t print_cpus_offline(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut len: c_int = 0;
    cpumask_var_t offline;
// display offline cpus < nr_cpu_ids
    if (!alloc_cpumask_var(&offline, GFP_KERNEL))
    return -ENOMEM;
    cpumask_andnot(offline, cpu_possible_mask, cpu_online_mask);
    len += sysfs_emit_at(buf, len, "%*pbl", cpumask_pr_args(offline));
    free_cpumask_var(offline);
// display offline cpus >= nr_cpu_ids
    if (total_cpus && nr_cpu_ids < total_cpus) {
    len += sysfs_emit_at(buf, len, ",");
    if (nr_cpu_ids == total_cpus-1)
    len += sysfs_emit_at(buf, len, "%u", nr_cpu_ids);
    else
    len += sysfs_emit_at(buf, len, "%u-%d",
    nr_cpu_ids, total_cpus - 1);
    }
    len += sysfs_emit_at(buf, len, "\n");
    return len;
    }
    static DEVICE_ATTR(offline, 0444, print_cpus_offline, core::ptr::null_mut());
    static ssize_t print_cpus_enabled(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpu_enabled_mask));
    }
    static DEVICE_ATTR(enabled, 0444, print_cpus_enabled, core::ptr::null_mut());
    static ssize_t print_cpus_isolated(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int len;
    cpumask_var_t isolated;
    if (!alloc_cpumask_var(&isolated, GFP_KERNEL))
    return -ENOMEM;
    cpumask_andnot(isolated, cpu_possible_mask,
    housekeeping_cpumask(HK_TYPE_DOMAIN_BOOT));
    len = sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(isolated));
    free_cpumask_var(isolated);
    return len;
    }
    static DEVICE_ATTR(isolated, 0444, print_cpus_isolated, core::ptr::null_mut());
    static ssize_t housekeeping_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    const struct cpumask *hk_mask;
    hk_mask = housekeeping_cpumask(HK_TYPE_KERNEL_NOISE);
    if (housekeeping_enabled(HK_TYPE_KERNEL_NOISE))
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(hk_mask));
    return sysfs_emit(buf, "\n");
    }
    static DEVICE_ATTR_RO(housekeeping);

    static ssize_t nohz_full_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    if (cpumask_available(tick_nohz_full_mask))
    return sysfs_emit(buf, "%*pbl\n",
    cpumask_pr_args(tick_nohz_full_mask));
    return sysfs_emit(buf, "\n");
    }
    static DEVICE_ATTR_RO(nohz_full);

    static ssize_t crash_hotplug_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%d\n", crash_check_hotplug_support());
    }
    static DEVICE_ATTR_RO(crash_hotplug);

#[no_mangle]
unsafe extern "C" fn cpu_device_release(dev: *mut device) {
    static void cpu_device_release(struct device *dev)
    {
//
// This is an empty function to prevent the driver core from spitting a
// warning at us.  Yes, I know this is directly opposite of what the
// documentation for the driver core and kobjects say, and the author
// of this code has already been publicly ridiculed for doing
// something as foolish as this.  However, at this point in time, it is
// the only way to handle the issue of statically allocated cpu
// devices.  The different architectures will have their cpu device
// code reworked to properly handle this in the near future, so this
// function will then be changed to correctly free up the memory held
// by the cpu device.
//
// Never copy this way of doing things, or you too will be made fun of
// on the linux-kernel list, you have been warned.
//
    }

    static ssize_t print_cpu_modalias(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut len: c_int = 0;
    u32 i;
    len += sysfs_emit_at(buf, len,
    "cpu:type:" CPU_FEATURE_TYPEFMT ":feature:",
    CPU_FEATURE_TYPEVAL);
    for (i = 0; i < MAX_CPU_FEATURES; i++)
    if (cpu_have_feature(i)) {
    if (len + sizeof(",XXXX\n") >= PAGE_SIZE) {
    WARN(1, "CPU features overflow page\n");
    break;
    }
    len += sysfs_emit_at(buf, len, ",%04X", i);
    }
    len += sysfs_emit_at(buf, len, "\n");
    return len;
    }
#[no_mangle]
unsafe extern "C" fn cpu_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int cpu_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    char *buf = kzalloc(PAGE_SIZE, GFP_KERNEL);
    if (buf) {
    print_cpu_modalias(core::ptr::null_mut(), core::ptr::null_mut(), buf);
    add_uevent_var(env, "MODALIAS=%s", buf);
    kfree(buf);
    }
    return 0;
    }

    const struct bus_type cpu_subsys = {
    .name = "cpu",
    .dev_name = "cpu",
    .match = cpu_subsys_match,

    .online = cpu_subsys_online,
    .offline = cpu_subsys_offline,

    .uevent = cpu_uevent,

    };
    EXPORT_SYMBOL_GPL(cpu_subsys);
//
// register_cpu - Setup a sysfs device for a CPU.
// @cpu - cpu->hotpluggable field set to 1 will generate a control file in
// sysfs for this CPU.
// @num - CPU number to use when creating the device.
//
// Initialize and register the CPU device.
//
#[no_mangle]
pub unsafe extern "C" fn register_cpu(cpu: *mut cpu, num: c_int) -> c_int {
    int register_cpu(struct cpu *cpu, int num)
    {
    int error;
    cpu.node_id = cpu_to_node(num);
    memset(&cpu.dev, 0x00, sizeof(struct device));
    cpu.dev.id = num;
    cpu.dev.bus = &cpu_subsys;
    cpu.dev.release = cpu_device_release;
    dev_assign_offline_disabled(&cpu.dev, !cpu.hotpluggable);
    dev_assign_offline(&cpu.dev, !cpu_online(num));
    cpu.dev.of_node = of_get_cpu_node(num, core::ptr::null_mut());
    cpu.dev.groups = common_cpu_attr_groups;
    if (cpu.hotpluggable)
    cpu.dev.groups = hotplugable_cpu_attr_groups;
    error = device_register(&cpu.dev);
    if (error) {
    put_device(&cpu.dev);
    return error;
    }
    per_cpu(cpu_sys_devices, num) = &cpu.dev;
    register_cpu_under_node(num, cpu_to_node(num));
    dev_pm_qos_expose_latency_limit(&cpu.dev,
    PM_QOS_RESUME_LATENCY_NO_CONSTRAINT);
    set_cpu_enabled(num, true);
    return 0;
    }
    struct device *get_cpu_device(unsigned int cpu)
    {
    if (cpu < nr_cpu_ids && cpu_possible(cpu))
    return per_cpu(cpu_sys_devices, cpu);
    else
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(get_cpu_device);
#[no_mangle]
unsafe extern "C" fn device_create_release(dev: *mut device) {
    static void device_create_release(struct device *dev)
    {
    kfree(dev);
    }
    __printf(4, 0)
    static struct device *
    __cpu_device_create(struct device *parent, void *drvdata,
    const struct attribute_group **groups,
    const char *fmt, va_list args)
    {
    struct device *dev = core::ptr::null_mut();
    let mut retval: c_int = -ENOMEM;
    dev = kzalloc_obj(*dev);
    if (!dev)
    goto error;
    device_initialize(dev);
    dev.parent = parent;
    dev.groups = groups;
    dev.release = device_create_release;
    device_set_pm_not_required(dev);
    dev_set_drvdata(dev, drvdata);
    retval = kobject_set_name_vargs(&dev.kobj, fmt, args);
    if (retval)
    goto error;
    retval = device_add(dev);
    if (retval)
    goto error;
    return dev;
    error:
    put_device(dev);
    return ERR_PTR(retval);
    }
    struct device *cpu_device_create(struct device *parent, void *drvdata,
    const struct attribute_group **groups,
    const char *fmt, ...)
    {
    va_list vargs;
    struct device *dev;
    va_start(vargs, fmt);
    dev = __cpu_device_create(parent, drvdata, groups, fmt, vargs);
    va_end(vargs);
    return dev;
    }
    EXPORT_SYMBOL_GPL(cpu_device_create);

    static DEVICE_ATTR(modalias, 0444, print_cpu_modalias, core::ptr::null_mut());

    static struct attribute *cpu_root_attrs[] = {

    &dev_attr_probe.attr,
    &dev_attr_release.attr,

    &cpu_attrs[0].attr.attr,
    &cpu_attrs[1].attr.attr,
    &cpu_attrs[2].attr.attr,
    &dev_attr_kernel_max.attr,
    &dev_attr_offline.attr,
    &dev_attr_enabled.attr,
    &dev_attr_isolated.attr,
    &dev_attr_housekeeping.attr,

    &dev_attr_nohz_full.attr,

    &dev_attr_crash_hotplug.attr,

    &dev_attr_modalias.attr,

    core::ptr::null_mut()
    };
    static const struct attribute_group cpu_root_attr_group = {
    .attrs = cpu_root_attrs,
    };
    static const struct attribute_group *cpu_root_attr_groups[] = {
    &cpu_root_attr_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn cpu_is_hotpluggable(cpu: c_uint) -> bool {
    bool cpu_is_hotpluggable(unsigned int cpu)
    {
    struct device *dev = get_cpu_device(cpu);
    return dev && container_of(dev, struct cpu, dev).hotpluggable
    && tick_nohz_cpu_hotpluggable(cpu);
    }
    EXPORT_SYMBOL_GPL(cpu_is_hotpluggable);

    DEFINE_PER_CPU(struct cpu, cpu_devices);
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_is_hotpluggable(cpu: c_int) -> bool __weak {
    bool __weak arch_cpu_is_hotpluggable(int cpu)
    {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_register_cpu(cpu: c_int) -> int __weak {
    int __weak arch_register_cpu(int cpu)
    {
    struct cpu *c = &per_cpu(cpu_devices, cpu);
    c.hotpluggable = arch_cpu_is_hotpluggable(cpu);
    return register_cpu(c, cpu);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_unregister_cpu(num: c_int) -> void __weak {
    void __weak arch_unregister_cpu(int num)
    {
    unregister_cpu(&per_cpu(cpu_devices, num));
    }

#[no_mangle]
unsafe extern "C" fn cpu_dev_register_generic() -> void __init {
    static void __init cpu_dev_register_generic(void)
    {
    int i, ret;
    if (!IS_ENABLED(CONFIG_GENERIC_CPU_DEVICES))
    return;
    for_each_present_cpu(i) {
    ret = arch_register_cpu(i);
    if (ret && ret != -EPROBE_DEFER)
    pr_warn("register_cpu %d failed (%d)\n", i, ret);
    }
    }

    static ssize_t cpu_show_not_affected(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "Not affected\n");
    }

    ssize_t cpu_show_##func(struct device *,			\
    struct device_attribute *, char *)	\
    __attribute__((weak, alias("cpu_show_not_affected")))
    CPU_SHOW_VULN_FALLBACK(meltdown);
    CPU_SHOW_VULN_FALLBACK(spectre_v1);
    CPU_SHOW_VULN_FALLBACK(spectre_v2);
    CPU_SHOW_VULN_FALLBACK(spec_store_bypass);
    CPU_SHOW_VULN_FALLBACK(l1tf);
    CPU_SHOW_VULN_FALLBACK(mds);
    CPU_SHOW_VULN_FALLBACK(tsx_async_abort);
    CPU_SHOW_VULN_FALLBACK(itlb_multihit);
    CPU_SHOW_VULN_FALLBACK(srbds);
    CPU_SHOW_VULN_FALLBACK(mmio_stale_data);
    CPU_SHOW_VULN_FALLBACK(retbleed);
    CPU_SHOW_VULN_FALLBACK(spec_rstack_overflow);
    CPU_SHOW_VULN_FALLBACK(gds);
    CPU_SHOW_VULN_FALLBACK(reg_file_data_sampling);
    CPU_SHOW_VULN_FALLBACK(ghostwrite);
    CPU_SHOW_VULN_FALLBACK(old_microcode);
    CPU_SHOW_VULN_FALLBACK(indirect_target_selection);
    CPU_SHOW_VULN_FALLBACK(tsa);
    CPU_SHOW_VULN_FALLBACK(vmscape);
    static DEVICE_ATTR(meltdown, 0444, cpu_show_meltdown, core::ptr::null_mut());
    static DEVICE_ATTR(spectre_v1, 0444, cpu_show_spectre_v1, core::ptr::null_mut());
    static DEVICE_ATTR(spectre_v2, 0444, cpu_show_spectre_v2, core::ptr::null_mut());
    static DEVICE_ATTR(spec_store_bypass, 0444, cpu_show_spec_store_bypass, core::ptr::null_mut());
    static DEVICE_ATTR(l1tf, 0444, cpu_show_l1tf, core::ptr::null_mut());
    static DEVICE_ATTR(mds, 0444, cpu_show_mds, core::ptr::null_mut());
    static DEVICE_ATTR(tsx_async_abort, 0444, cpu_show_tsx_async_abort, core::ptr::null_mut());
    static DEVICE_ATTR(itlb_multihit, 0444, cpu_show_itlb_multihit, core::ptr::null_mut());
    static DEVICE_ATTR(srbds, 0444, cpu_show_srbds, core::ptr::null_mut());
    static DEVICE_ATTR(mmio_stale_data, 0444, cpu_show_mmio_stale_data, core::ptr::null_mut());
    static DEVICE_ATTR(retbleed, 0444, cpu_show_retbleed, core::ptr::null_mut());
    static DEVICE_ATTR(spec_rstack_overflow, 0444, cpu_show_spec_rstack_overflow, core::ptr::null_mut());
    static DEVICE_ATTR(gather_data_sampling, 0444, cpu_show_gds, core::ptr::null_mut());
    static DEVICE_ATTR(reg_file_data_sampling, 0444, cpu_show_reg_file_data_sampling, core::ptr::null_mut());
    static DEVICE_ATTR(ghostwrite, 0444, cpu_show_ghostwrite, core::ptr::null_mut());
    static DEVICE_ATTR(old_microcode, 0444, cpu_show_old_microcode, core::ptr::null_mut());
    static DEVICE_ATTR(indirect_target_selection, 0444, cpu_show_indirect_target_selection, core::ptr::null_mut());
    static DEVICE_ATTR(tsa, 0444, cpu_show_tsa, core::ptr::null_mut());
    static DEVICE_ATTR(vmscape, 0444, cpu_show_vmscape, core::ptr::null_mut());
    static struct attribute *cpu_root_vulnerabilities_attrs[] = {
    &dev_attr_meltdown.attr,
    &dev_attr_spectre_v1.attr,
    &dev_attr_spectre_v2.attr,
    &dev_attr_spec_store_bypass.attr,
    &dev_attr_l1tf.attr,
    &dev_attr_mds.attr,
    &dev_attr_tsx_async_abort.attr,
    &dev_attr_itlb_multihit.attr,
    &dev_attr_srbds.attr,
    &dev_attr_mmio_stale_data.attr,
    &dev_attr_retbleed.attr,
    &dev_attr_spec_rstack_overflow.attr,
    &dev_attr_gather_data_sampling.attr,
    &dev_attr_reg_file_data_sampling.attr,
    &dev_attr_ghostwrite.attr,
    &dev_attr_old_microcode.attr,
    &dev_attr_indirect_target_selection.attr,
    &dev_attr_tsa.attr,
    &dev_attr_vmscape.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group cpu_root_vulnerabilities_group = {
    .name  = "vulnerabilities",
    .attrs = cpu_root_vulnerabilities_attrs,
    };
#[no_mangle]
unsafe extern "C" fn cpu_register_vulnerabilities() -> void __init {
    static void __init cpu_register_vulnerabilities(void)
    {
    struct device *dev = bus_get_dev_root(&cpu_subsys);
    if (dev) {
    if (sysfs_create_group(&dev.kobj, &cpu_root_vulnerabilities_group))
    pr_err("Unable to register CPU vulnerabilities\n");
    put_device(dev);
    }
    }

    static inline void cpu_register_vulnerabilities(void) { }

#[no_mangle]
pub unsafe extern "C" fn cpu_dev_init() -> void __init {
    void __init cpu_dev_init(void)
    {
    if (subsys_system_register(&cpu_subsys, cpu_root_attr_groups))
    panic("Failed to register CPU subsystem");
    cpu_dev_register_generic();
    cpu_register_vulnerabilities();
    }
