//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/sysfs.c
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


//
// sysfs.c - sysfs support
//
// (C) 2006-2007 Shaohua Li <shaohua.li@intel.com>
//
// This code is licenced under the GPL.
//

    static ssize_t show_available_governors(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut i: isize = 0;
    struct cpuidle_governor *tmp;
    mutex_lock(&cpuidle_lock);
    list_for_each_entry(tmp, &cpuidle_governors, governor_list) {
    if (i >= (ssize_t)(PAGE_SIZE - (CPUIDLE_NAME_LEN + 2)))
    goto out;
    i += sysfs_emit_at(buf, i, "%.*s ", CPUIDLE_NAME_LEN, tmp.name);
    }
    out:
    i += sysfs_emit_at(buf, i, "\n");
    mutex_unlock(&cpuidle_lock);
    return i;
    }
    static ssize_t show_current_driver(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    ssize_t ret;
    struct cpuidle_driver *drv;
    spin_lock(&cpuidle_driver_lock);
    drv = cpuidle_get_driver();
    if (drv)
    ret = sysfs_emit(buf, "%s\n", drv.name);
    else
    ret = sysfs_emit(buf, "none\n");
    spin_unlock(&cpuidle_driver_lock);
    return ret;
    }
    static ssize_t show_current_governor(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    ssize_t ret;
    mutex_lock(&cpuidle_lock);
    if (cpuidle_curr_governor)
    ret = sysfs_emit(buf, "%s\n", cpuidle_curr_governor.name);
    else
    ret = sysfs_emit(buf, "none\n");
    mutex_unlock(&cpuidle_lock);
    return ret;
    }
    static ssize_t store_current_governor(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    char gov_name[CPUIDLE_NAME_LEN + 1];
    int ret;
    struct cpuidle_governor *gov;
    ret = sscanf(buf, "%" __stringify(CPUIDLE_NAME_LEN) "s", gov_name);
    if (ret != 1)
    return -EINVAL;
    mutex_lock(&cpuidle_lock);
    ret = -EINVAL;
    list_for_each_entry(gov, &cpuidle_governors, governor_list) {
    if (!strncmp(gov.name, gov_name, CPUIDLE_NAME_LEN)) {
    ret = cpuidle_switch_governor(gov);
    break;
    }
    }
    mutex_unlock(&cpuidle_lock);
    return ret ? ret : count;
    }
    static DEVICE_ATTR(available_governors, 0444, show_available_governors, core::ptr::null_mut());
    static DEVICE_ATTR(current_driver, 0444, show_current_driver, core::ptr::null_mut());
    static DEVICE_ATTR(current_governor, 0644, show_current_governor,
    store_current_governor);
    static DEVICE_ATTR(current_governor_ro, 0444, show_current_governor, core::ptr::null_mut());
    static struct attribute *cpuidle_attrs[] = {
    &dev_attr_available_governors.attr,
    &dev_attr_current_driver.attr,
    &dev_attr_current_governor.attr,
    &dev_attr_current_governor_ro.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group cpuidle_attr_group = {
    .attrs = cpuidle_attrs,
    .name = "cpuidle",
    };
//
// cpuidle_add_interface - add CPU global sysfs attributes
//
#[no_mangle]
pub unsafe extern "C" fn cpuidle_add_interface() -> c_int {
    int cpuidle_add_interface(void)
    {
    struct device *dev_root = bus_get_dev_root(&cpu_subsys);
    int retval;
    if (!dev_root)
    return -EINVAL;
    retval = sysfs_create_group(&dev_root.kobj, &cpuidle_attr_group);
    put_device(dev_root);
    return retval;
    }
//
// cpuidle_remove_interface - remove CPU global sysfs attributes
// @dev: the target device
//
#[no_mangle]
pub unsafe extern "C" fn cpuidle_remove_interface(dev: *mut device) {
    void cpuidle_remove_interface(struct device *dev)
    {
    sysfs_remove_group(&dev.kobj, &cpuidle_attr_group);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_attr {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct cpuidle_device , char,
    pub count): *const *const *const *const ssize_t (store)(struct cpuidle_device , char , size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_device_kobj {
    pub dev: *mut cpuidle_device,
    pub kobj_unregister: completion,
    pub kobj: kobject,
}

    static inline struct cpuidle_device *to_cpuidle_device(struct kobject *kobj)
    {
    struct cpuidle_device_kobj *kdev =
    container_of(kobj, struct cpuidle_device_kobj, kobj);
    return kdev.dev;
    }
    static ssize_t cpuidle_show(struct kobject *kobj, struct attribute *attr,
    char *buf)
    {
    let mut ret: c_int = -EIO;
    struct cpuidle_device *dev = to_cpuidle_device(kobj);
    struct cpuidle_attr *cattr = attr_to_cpuidleattr(attr);
    if (cattr.show) {
    mutex_lock(&cpuidle_lock);
    ret = cattr.show(dev, buf);
    mutex_unlock(&cpuidle_lock);
    }
    return ret;
    }
    static ssize_t cpuidle_store(struct kobject *kobj, struct attribute *attr,
    const char *buf, size_t count)
    {
    let mut ret: c_int = -EIO;
    struct cpuidle_device *dev = to_cpuidle_device(kobj);
    struct cpuidle_attr *cattr = attr_to_cpuidleattr(attr);
    if (cattr.store) {
    mutex_lock(&cpuidle_lock);
    ret = cattr.store(dev, buf, count);
    mutex_unlock(&cpuidle_lock);
    }
    return ret;
    }
    static const struct sysfs_ops cpuidle_sysfs_ops = {
    .show = cpuidle_show,
    .store = cpuidle_store,
    };
#[no_mangle]
unsafe extern "C" fn cpuidle_sysfs_release(kobj: *mut kobject) {
    static void cpuidle_sysfs_release(struct kobject *kobj)
    {
    struct cpuidle_device_kobj *kdev =
    container_of(kobj, struct cpuidle_device_kobj, kobj);
    complete(&kdev.kobj_unregister);
    }
    static const struct kobj_type ktype_cpuidle = {
    .sysfs_ops = &cpuidle_sysfs_ops,
    .release = cpuidle_sysfs_release,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_state_attr {
    pub attr: attribute,
    ssize_t (*show)(struct cpuidle_state *, \
    pub ): *mut *mut cpuidle_state_usage , char,
    ssize_t (*store)(struct cpuidle_state *, \
    pub size_t): *const *const *const cpuidle_state_usage , char ,,
}

    static struct cpuidle_state_attr attr_##_name = __ATTR(_name, 0444, show, core::ptr::null_mut())

    static struct cpuidle_state_attr attr_##_name = __ATTR(_name, 0644, show, store)

    static ssize_t show_state_##_name(struct cpuidle_state *state, \
    struct cpuidle_state_usage *state_usage, char *buf) \
    { \
    return sysfs_emit(buf, "%u\n", state._name);\
    }

    static ssize_t show_state_##_name(struct cpuidle_state *state, \
    struct cpuidle_state_usage *state_usage, \
    char *buf)				\
    { \
    return sysfs_emit(buf, "%llu\n", state_usage._name);\
    }

    static ssize_t show_state_##_name(struct cpuidle_state *state, \
    struct cpuidle_state_usage *state_usage, \
    char *buf)				\
    { \
    if (state._name[0] == '\0')\
    return sysfs_emit(buf, "<null>\n");\
    return sysfs_emit(buf, "%s\n", state._name);\
    }

    static ssize_t show_state_##_name(struct cpuidle_state *state, \
    struct cpuidle_state_usage *state_usage, \
    char *buf) \
    { \
    return sysfs_emit(buf, "%llu\n", ktime_to_us(state._name##_ns)); \
    }
    define_show_state_time_function(exit_latency)
    define_show_state_time_function(target_residency)
    define_show_state_function(power_usage)
    define_show_state_ull_function(usage)
    define_show_state_ull_function(rejected)
    define_show_state_str_function(name)
    define_show_state_str_function(desc)
    define_show_state_ull_function(above)
    define_show_state_ull_function(below)
    static ssize_t show_state_time(struct cpuidle_state *state,
    struct cpuidle_state_usage *state_usage,
    char *buf)
    {
    return sysfs_emit(buf, "%llu\n", ktime_to_us(state_usage.time_ns));
    }
    static ssize_t show_state_disable(struct cpuidle_state *state,
    struct cpuidle_state_usage *state_usage,
    char *buf)
    {
    return sysfs_emit(buf, "%llu\n",
    state_usage.disable & CPUIDLE_STATE_DISABLED_BY_USER);
    }
    static ssize_t store_state_disable(struct cpuidle_state *state,
    struct cpuidle_state_usage *state_usage,
    const char *buf, size_t size)
    {
    unsigned int value;
    int err;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    err = kstrtouint(buf, 0, &value);
    if (err)
    return err;
    if (value)
    state_usage.disable |= CPUIDLE_STATE_DISABLED_BY_USER;
    else
    state_usage.disable &= ~CPUIDLE_STATE_DISABLED_BY_USER;
    return size;
    }
    static ssize_t show_state_default_status(struct cpuidle_state *state,
    struct cpuidle_state_usage *state_usage,
    char *buf)
    {
    return sysfs_emit(buf, "%s\n",
    state.flags & CPUIDLE_FLAG_OFF ? "disabled" : "enabled");
    }
    define_one_state_ro(name, show_state_name);
    define_one_state_ro(desc, show_state_desc);
    define_one_state_ro(latency, show_state_exit_latency);
    define_one_state_ro(residency, show_state_target_residency);
    define_one_state_ro(power, show_state_power_usage);
    define_one_state_ro(usage, show_state_usage);
    define_one_state_ro(rejected, show_state_rejected);
    define_one_state_ro(time, show_state_time);
    define_one_state_rw(disable, show_state_disable, store_state_disable);
    define_one_state_ro(above, show_state_above);
    define_one_state_ro(below, show_state_below);
    define_one_state_ro(default_status, show_state_default_status);
    static struct attribute *cpuidle_state_default_attrs[] = {
    &attr_name.attr,
    &attr_desc.attr,
    &attr_latency.attr,
    &attr_residency.attr,
    &attr_power.attr,
    &attr_usage.attr,
    &attr_rejected.attr,
    &attr_time.attr,
    &attr_disable.attr,
    &attr_above.attr,
    &attr_below.attr,
    &attr_default_status.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(cpuidle_state_default);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_state_kobj {
    pub state: *mut cpuidle_state,
    pub state_usage: *mut cpuidle_state_usage,
    pub kobj_unregister: completion,
    pub kobj: kobject,
    pub device: *mut cpuidle_device,
}

    static ssize_t show_state_s2idle_##_name(struct cpuidle_state *state, \
    struct cpuidle_state_usage *state_usage, \
    char *buf)				\
    { \
    return sysfs_emit(buf, "%llu\n", state_usage.s2idle_##_name);\
    }
    define_show_state_s2idle_ull_function(usage);
    define_show_state_s2idle_ull_function(time);

    static struct cpuidle_state_attr attr_s2idle_##_name = \
    __ATTR(_name, 0444, show, core::ptr::null_mut())
    define_one_state_s2idle_ro(usage, show_state_s2idle_usage);
    define_one_state_s2idle_ro(time, show_state_s2idle_time);
    static struct attribute *cpuidle_state_s2idle_attrs[] = {
    &attr_s2idle_usage.attr,
    &attr_s2idle_time.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group cpuidle_state_s2idle_group = {
    .name	= "s2idle",
    .attrs	= cpuidle_state_s2idle_attrs,
    };
#[no_mangle]
unsafe extern "C" fn cpuidle_add_s2idle_attr_group(kobj: *mut cpuidle_state_kobj) {
    static void cpuidle_add_s2idle_attr_group(struct cpuidle_state_kobj *kobj)
    {
    int ret;
    if (!kobj.state.enter_s2idle)
    return;
    ret = sysfs_create_group(&kobj.kobj, &cpuidle_state_s2idle_group);
    if (ret)
    pr_debug("%s: sysfs attribute group not created\n", __func__);
    }
#[no_mangle]
unsafe extern "C" fn cpuidle_remove_s2idle_attr_group(kobj: *mut cpuidle_state_kobj) {
    static void cpuidle_remove_s2idle_attr_group(struct cpuidle_state_kobj *kobj)
    {
    if (kobj.state.enter_s2idle)
    sysfs_remove_group(&kobj.kobj, &cpuidle_state_s2idle_group);
    }

    static inline void cpuidle_add_s2idle_attr_group(struct cpuidle_state_kobj *kobj) { }
    static inline void cpuidle_remove_s2idle_attr_group(struct cpuidle_state_kobj *kobj) { }

    static ssize_t cpuidle_state_show(struct kobject *kobj, struct attribute *attr,
    char *buf)
    {
    let mut ret: c_int = -EIO;
    struct cpuidle_state *state = kobj_to_state(kobj);
    struct cpuidle_state_usage *state_usage = kobj_to_state_usage(kobj);
    struct cpuidle_state_attr *cattr = attr_to_stateattr(attr);
    if (cattr.show)
    ret = cattr.show(state, state_usage, buf);
    return ret;
    }
    static ssize_t cpuidle_state_store(struct kobject *kobj, struct attribute *attr,
    const char *buf, size_t size)
    {
    let mut ret: c_int = -EIO;
    struct cpuidle_state *state = kobj_to_state(kobj);
    struct cpuidle_state_usage *state_usage = kobj_to_state_usage(kobj);
    struct cpuidle_state_attr *cattr = attr_to_stateattr(attr);
    struct cpuidle_device *dev = kobj_to_device(kobj);
    if (cattr.store)
    ret = cattr.store(state, state_usage, buf, size);
// reset poll time cache
    dev.poll_limit_ns = 0;
    return ret;
    }
    static const struct sysfs_ops cpuidle_state_sysfs_ops = {
    .show = cpuidle_state_show,
    .store = cpuidle_state_store,
    };
#[no_mangle]
unsafe extern "C" fn cpuidle_state_sysfs_release(kobj: *mut kobject) {
    static void cpuidle_state_sysfs_release(struct kobject *kobj)
    {
    struct cpuidle_state_kobj *state_obj = kobj_to_state_obj(kobj);
    complete(&state_obj.kobj_unregister);
    }
    static const struct kobj_type ktype_state_cpuidle = {
    .sysfs_ops = &cpuidle_state_sysfs_ops,
    .default_groups = cpuidle_state_default_groups,
    .release = cpuidle_state_sysfs_release,
    };
#[no_mangle]
pub unsafe extern "C" fn cpuidle_free_state_kobj(device: *mut cpuidle_device, i: c_int) {
    static inline void cpuidle_free_state_kobj(struct cpuidle_device *device, int i)
    {
    cpuidle_remove_s2idle_attr_group(device.kobjs[i]);
    kobject_put(&device.kobjs[i].kobj);
    wait_for_completion(&device.kobjs[i].kobj_unregister);
    kfree(device.kobjs[i]);
    device.kobjs[i] = core::ptr::null_mut();
    }
//
// cpuidle_add_state_sysfs - adds cpuidle states sysfs attributes
// @device: the target device
//
#[no_mangle]
unsafe extern "C" fn cpuidle_add_state_sysfs(device: *mut cpuidle_device) -> c_int {
    static int cpuidle_add_state_sysfs(struct cpuidle_device *device)
    {
    int i, ret = -ENOMEM;
    struct cpuidle_state_kobj *kobj;
    struct cpuidle_device_kobj *kdev = device.kobj_dev;
    struct cpuidle_driver *drv = cpuidle_get_cpu_driver(device);
// state statistics
    for (i = 0; i < drv.state_count; i++) {
    kobj = kzalloc_obj(struct cpuidle_state_kobj);
    if (!kobj) {
    ret = -ENOMEM;
    goto error_state;
    }
    kobj.state = &drv.states[i];
    kobj.state_usage = &device.states_usage[i];
    kobj.device = device;
    init_completion(&kobj.kobj_unregister);
    ret = kobject_init_and_add(&kobj.kobj, &ktype_state_cpuidle,
    &kdev.kobj, "state%d", i);
    if (ret) {
    kobject_put(&kobj.kobj);
    kfree(kobj);
    goto error_state;
    }
    cpuidle_add_s2idle_attr_group(kobj);
    kobject_uevent(&kobj.kobj, KOBJ_ADD);
    device.kobjs[i] = kobj;
    }
    return 0;
    error_state:
    for (i = i - 1; i >= 0; i--)
    cpuidle_free_state_kobj(device, i);
    return ret;
    }
//
// cpuidle_remove_state_sysfs - removes the cpuidle states sysfs attributes
// @device: the target device
//
#[no_mangle]
unsafe extern "C" fn cpuidle_remove_state_sysfs(device: *mut cpuidle_device) {
    static void cpuidle_remove_state_sysfs(struct cpuidle_device *device)
    {
    struct cpuidle_driver *drv = cpuidle_get_cpu_driver(device);
    int i;
    for (i = 0; i < drv.state_count; i++)
    cpuidle_free_state_kobj(device, i);
    }

    static struct cpuidle_driver_attr attr_driver_##_name = \
    __ATTR(_name, 0444, show, core::ptr::null_mut())
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_driver_kobj {
    pub drv: *mut cpuidle_driver,
    pub kobj_unregister: completion,
    pub kobj: kobject,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuidle_driver_attr {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct cpuidle_driver , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct cpuidle_driver , char ,,
}

#[no_mangle]
unsafe extern "C" fn show_driver_name(drv: *mut cpuidle_driver, buf: *mut c_char) -> isize {
    static ssize_t show_driver_name(struct cpuidle_driver *drv, char *buf)
    {
    ssize_t ret;
    spin_lock(&cpuidle_driver_lock);
    ret = sysfs_emit(buf, "%s\n", drv ? drv.name : "none");
    spin_unlock(&cpuidle_driver_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpuidle_driver_sysfs_release(kobj: *mut kobject) {
    static void cpuidle_driver_sysfs_release(struct kobject *kobj)
    {
    struct cpuidle_driver_kobj *driver_kobj = kobj_to_driver_kobj(kobj);
    complete(&driver_kobj.kobj_unregister);
    }
    static ssize_t cpuidle_driver_show(struct kobject *kobj, struct attribute *attr,
    char *buf)
    {
    let mut ret: c_int = -EIO;
    struct cpuidle_driver_kobj *driver_kobj = kobj_to_driver_kobj(kobj);
    struct cpuidle_driver_attr *dattr = attr_to_driver_attr(attr);
    if (dattr.show)
    ret = dattr.show(driver_kobj.drv, buf);
    return ret;
    }
    static ssize_t cpuidle_driver_store(struct kobject *kobj, struct attribute *attr,
    const char *buf, size_t size)
    {
    let mut ret: c_int = -EIO;
    struct cpuidle_driver_kobj *driver_kobj = kobj_to_driver_kobj(kobj);
    struct cpuidle_driver_attr *dattr = attr_to_driver_attr(attr);
    if (dattr.store)
    ret = dattr.store(driver_kobj.drv, buf, size);
    return ret;
    }
    define_one_driver_ro(name, show_driver_name);
    static const struct sysfs_ops cpuidle_driver_sysfs_ops = {
    .show = cpuidle_driver_show,
    .store = cpuidle_driver_store,
    };
    static struct attribute *cpuidle_driver_default_attrs[] = {
    &attr_driver_name.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(cpuidle_driver_default);
    static const struct kobj_type ktype_driver_cpuidle = {
    .sysfs_ops = &cpuidle_driver_sysfs_ops,
    .default_groups = cpuidle_driver_default_groups,
    .release = cpuidle_driver_sysfs_release,
    };
//
// cpuidle_add_driver_sysfs - adds the driver name sysfs attribute
// @dev: the target device
//
#[no_mangle]
unsafe extern "C" fn cpuidle_add_driver_sysfs(dev: *mut cpuidle_device) -> c_int {
    static int cpuidle_add_driver_sysfs(struct cpuidle_device *dev)
    {
    struct cpuidle_driver_kobj *kdrv;
    struct cpuidle_device_kobj *kdev = dev.kobj_dev;
    struct cpuidle_driver *drv = cpuidle_get_cpu_driver(dev);
    int ret;
    kdrv = kzalloc_obj(*kdrv);
    if (!kdrv)
    return -ENOMEM;
    kdrv.drv = drv;
    init_completion(&kdrv.kobj_unregister);
    ret = kobject_init_and_add(&kdrv.kobj, &ktype_driver_cpuidle,
    &kdev.kobj, "driver");
    if (ret) {
    kobject_put(&kdrv.kobj);
    kfree(kdrv);
    return ret;
    }
    kobject_uevent(&kdrv.kobj, KOBJ_ADD);
    dev.kobj_driver = kdrv;
    return ret;
    }
//
// cpuidle_remove_driver_sysfs - removes the driver name sysfs attribute
// @dev: the target device
//
#[no_mangle]
unsafe extern "C" fn cpuidle_remove_driver_sysfs(dev: *mut cpuidle_device) {
    static void cpuidle_remove_driver_sysfs(struct cpuidle_device *dev)
    {
    struct cpuidle_driver_kobj *kdrv = dev.kobj_driver;
    kobject_put(&kdrv.kobj);
    wait_for_completion(&kdrv.kobj_unregister);
    kfree(kdrv);
    }

#[no_mangle]
pub unsafe extern "C" fn cpuidle_add_driver_sysfs(dev: *mut cpuidle_device) -> c_int {
    static inline int cpuidle_add_driver_sysfs(struct cpuidle_device *dev)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuidle_remove_driver_sysfs(dev: *mut cpuidle_device) {
    static inline void cpuidle_remove_driver_sysfs(struct cpuidle_device *dev)
    {
    ;
    }

//
// cpuidle_add_device_sysfs - adds device specific sysfs attributes
// @device: the target device
//
#[no_mangle]
pub unsafe extern "C" fn cpuidle_add_device_sysfs(device: *mut cpuidle_device) -> c_int {
    int cpuidle_add_device_sysfs(struct cpuidle_device *device)
    {
    int ret;
    ret = cpuidle_add_state_sysfs(device);
    if (ret)
    return ret;
    ret = cpuidle_add_driver_sysfs(device);
    if (ret)
    cpuidle_remove_state_sysfs(device);
    return ret;
    }
//
// cpuidle_remove_device_sysfs : removes device specific sysfs attributes
// @device : the target device
//
#[no_mangle]
pub unsafe extern "C" fn cpuidle_remove_device_sysfs(device: *mut cpuidle_device) {
    void cpuidle_remove_device_sysfs(struct cpuidle_device *device)
    {
    cpuidle_remove_driver_sysfs(device);
    cpuidle_remove_state_sysfs(device);
    }
//
// cpuidle_add_sysfs - creates a sysfs instance for the target device
// @dev: the target device
//
#[no_mangle]
pub unsafe extern "C" fn cpuidle_add_sysfs(dev: *mut cpuidle_device) -> c_int {
    int cpuidle_add_sysfs(struct cpuidle_device *dev)
    {
    struct cpuidle_device_kobj *kdev;
    struct device *cpu_dev = get_cpu_device((unsigned long)dev.cpu);
    int error;
//
// Return if cpu_device is not setup for this CPU.
//
// This could happen if the arch did not set up cpu_device
// since this CPU is not in cpu_present mask and the
// driver did not send a correct CPU mask during registration.
// Without this check we would end up passing bogus
// value for &cpu_dev->kobj in kobject_init_and_add()
//
    if (!cpu_dev)
    return -ENODEV;
    kdev = kzalloc_obj(*kdev);
    if (!kdev)
    return -ENOMEM;
    kdev.dev = dev;
    init_completion(&kdev.kobj_unregister);
    error = kobject_init_and_add(&kdev.kobj, &ktype_cpuidle, &cpu_dev.kobj,
    "cpuidle");
    if (error) {
    kobject_put(&kdev.kobj);
    kfree(kdev);
    return error;
    }
    dev.kobj_dev = kdev;
    kobject_uevent(&kdev.kobj, KOBJ_ADD);
    return 0;
    }
//
// cpuidle_remove_sysfs - deletes a sysfs instance on the target device
// @dev: the target device
//
#[no_mangle]
pub unsafe extern "C" fn cpuidle_remove_sysfs(dev: *mut cpuidle_device) {
    void cpuidle_remove_sysfs(struct cpuidle_device *dev)
    {
    struct cpuidle_device_kobj *kdev = dev.kobj_dev;
    kobject_put(&kdev.kobj);
    wait_for_completion(&kdev.kobj_unregister);
    kfree(kdev);
    }
