//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/raid_class.c
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
// raid_class.c - implementation of a simple raid visualisation class
//
// Copyright (c) 2005 - James Bottomley <James.Bottomley@steeleye.com>
//
// This class is designed to allow raid attributes to be visualised and
// manipulated in a form independent of the underlying raid.  Ultimately this
// should work for both hardware and software raids.
//

pub const RAID_NUM_ATTRS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid_internal {
    pub r: raid_template,
    pub f: *mut raid_function_template,
// The actual attributes
    pub private_attrs: [device_attribute; RAID_NUM_ATTRS],
// The array of null terminated pointers to attributes
// needed by scsi_sysfs.c
    pub 1]: *mut *mut device_attribute attrs[RAID_NUM_ATTRS +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid_component {
    pub node: list_head,
    pub dev: device,
    pub num: c_int,
}

    struct raid_template *r =					\
    container_of(tcont, struct raid_template, raid_attrs);	\
    to_raid_internal(r);						\
    })

    struct transport_container *tc =				\
    container_of(acont, struct transport_container, ac);	\
    tc_to_raid_internal(tc);					\
    })

    struct attribute_container *ac =				\
    attribute_container_classdev_to_container(dev);	\
    ac_to_raid_internal(ac);					\
    })
#[no_mangle]
unsafe extern "C" fn raid_match(cont: *mut attribute_container, dev: *mut device) -> c_int {
    static int raid_match(struct attribute_container *cont, struct device *dev)
    {
// We have to look for every subsystem that could house
// emulated RAID devices, so start with SCSI
    struct raid_internal *i = ac_to_raid_internal(cont);
    if (IS_ENABLED(CONFIG_SCSI) && scsi_is_sdev_device(dev)) {
    struct scsi_device *sdev = to_scsi_device(dev);
    if (i.f.cookie != sdev.host.hostt)
    return 0;
    return i.f.is_raid(dev);
    }
// FIXME: look at other subsystems too
    return 0;
    }
    static int raid_setup(struct transport_container *tc, struct device *dev,
    struct device *cdev)
    {
    struct raid_data *rd;
    BUG_ON(dev_get_drvdata(cdev));
    rd = kzalloc_obj(*rd);
    if (!rd)
    return -ENOMEM;
    INIT_LIST_HEAD(&rd.component_list);
    dev_set_drvdata(cdev, rd);
    return 0;
    }
    static int raid_remove(struct transport_container *tc, struct device *dev,
    struct device *cdev)
    {
    struct raid_data *rd = dev_get_drvdata(cdev);
    struct raid_component *rc, *next;
    dev_printk(KERN_ERR, dev, "RAID REMOVE\n");
    dev_set_drvdata(cdev, core::ptr::null_mut());
    list_for_each_entry_safe(rc, next, &rd.component_list, node) {
    list_del(&rc.node);
    dev_printk(KERN_ERR, rc.dev.parent, "RAID COMPONENT REMOVE\n");
    device_unregister(&rc.dev);
    }
    dev_printk(KERN_ERR, dev, "RAID REMOVE DONE\n");
    kfree(rd);
    return 0;
    }
    static DECLARE_TRANSPORT_CLASS(raid_class,
    "raid_devices",
    raid_setup,
    raid_remove,
    core::ptr::null_mut());
    static const struct {
    enum raid_state	value;
    char		*name;
    } raid_states[] = {
    { RAID_STATE_UNKNOWN, "unknown" },
    { RAID_STATE_ACTIVE, "active" },
    { RAID_STATE_DEGRADED, "degraded" },
    { RAID_STATE_RESYNCING, "resyncing" },
    { RAID_STATE_OFFLINE, "offline" },
    };
    static const char *raid_state_name(enum raid_state state)
    {
    int i;
    char *name = core::ptr::null_mut();
    for (i = 0; i < ARRAY_SIZE(raid_states); i++) {
    if (raid_states[i].value == state) {
    name = raid_states[i].name;
    break;
    }
    }
    return name;
    }
    static struct {
    enum raid_level value;
    char *name;
    } raid_levels[] = {
    { RAID_LEVEL_UNKNOWN, "unknown" },
    { RAID_LEVEL_LINEAR, "linear" },
    { RAID_LEVEL_0, "raid0" },
    { RAID_LEVEL_1, "raid1" },
    { RAID_LEVEL_10, "raid10" },
    { RAID_LEVEL_1E, "raid1e" },
    { RAID_LEVEL_3, "raid3" },
    { RAID_LEVEL_4, "raid4" },
    { RAID_LEVEL_5, "raid5" },
    { RAID_LEVEL_50, "raid50" },
    { RAID_LEVEL_6, "raid6" },
    { RAID_LEVEL_JBOD, "jbod" },
    };
    static const char *raid_level_name(enum raid_level level)
    {
    int i;
    char *name = core::ptr::null_mut();
    for (i = 0; i < ARRAY_SIZE(raid_levels); i++) {
    if (raid_levels[i].value == level) {
    name = raid_levels[i].name;
    break;
    }
    }
    return name;
    }

    static ssize_t raid_show_##attr(struct device *dev, 			\
    struct device_attribute *attr, 		\
    char *buf)				\
    {									\
    struct raid_data *rd = dev_get_drvdata(dev);			\
    code								\
    return snprintf(buf, 20, #fmt "\n", var);			\
    }

    raid_attr_show_internal(attr, %s, name,					\
    const char *name;						\
    code								\
    name = raid_##states##_name(rd.attr);				\
    )									\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR(_arg: attr, _arg: S_IRUGO, _arg: raid_show_##attr, _arg: NULL) -> static {
    static DEVICE_ATTR(attr, S_IRUGO, raid_show_##attr, core::ptr::null_mut())

    raid_attr_show_internal(attr, %d, rd.attr, code)			\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR(_arg: attr, _arg: S_IRUGO, _arg: raid_show_##attr, _arg: NULL) -> static {
    static DEVICE_ATTR(attr, S_IRUGO, raid_show_##attr, core::ptr::null_mut())

    struct raid_internal *i = device_to_raid_internal(dev);		\
    if (i.f.get_##attr)						\
    i.f.get_##attr(dev.parent);

    raid_attr_ro_state(level);
    raid_attr_ro_fn(resync);
    raid_attr_ro_state_fn(state);
    struct raid_template *
    raid_class_attach(struct raid_function_template *ft)
    {
    struct raid_internal *i = kzalloc_obj(struct raid_internal);
    let mut count: c_int = 0;
    if (unlikely(!i))
    return core::ptr::null_mut();
    i.f = ft;
    i.r.raid_attrs.ac.class = &raid_class.class;
    i.r.raid_attrs.ac.match = raid_match;
    i.r.raid_attrs.ac.attrs = &i.attrs[0];
    attribute_container_register(&i.r.raid_attrs.ac);
    i.attrs[count++] = &dev_attr_level;
    i.attrs[count++] = &dev_attr_resync;
    i.attrs[count++] = &dev_attr_state;
    i.attrs[count] = core::ptr::null_mut();
    BUG_ON(count > RAID_NUM_ATTRS);
    return &i.r;
    }
    EXPORT_SYMBOL(raid_class_attach);
    void
    raid_class_release(struct raid_template *r)
    {
    struct raid_internal *i = to_raid_internal(r);
    BUG_ON(attribute_container_unregister(&i.r.raid_attrs.ac));
    kfree(i);
    }
    EXPORT_SYMBOL(raid_class_release);
#[no_mangle]
unsafe extern "C" fn raid_init() -> __init int {
    static __init int raid_init(void)
    {
    return transport_class_register(&raid_class);
    }
#[no_mangle]
unsafe extern "C" fn raid_exit() -> __exit void {
    static __exit void raid_exit(void)
    {
    transport_class_unregister(&raid_class);
    }
    MODULE_AUTHOR("James Bottomley");
    MODULE_DESCRIPTION("RAID device class");
    MODULE_LICENSE("GPL");
    module_init(raid_init);
    module_exit(raid_exit);
