//! Automatically rewritten from C to Rust
//! Source: kernel/ksysfs.c
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
// kernel/ksysfs.c - sysfs attributes in /sys/kernel, which
// are not related to any other subsystem
//
// Copyright (C) 2004 Kay Sievers <kay.sievers@vrfy.org>
//

    static struct kobj_attribute _name##_attr = __ATTR_RO(_name)

    static struct kobj_attribute _name##_attr = __ATTR_RW(_name)
// current uevent sequence number
    static ssize_t uevent_seqnum_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%llu\n", (u64)atomic64_read(&uevent_seqnum));
    }
    KERNEL_ATTR_RO(uevent_seqnum);
// cpu byteorder
    static ssize_t cpu_byteorder_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%s\n", CPU_BYTEORDER_STRING);
    }
    KERNEL_ATTR_RO(cpu_byteorder);
// address bits
    static ssize_t address_bits_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%zu\n", sizeof(void *) * 8 /* CHAR_BIT */);
    }
    KERNEL_ATTR_RO(address_bits);

// uevent helper program, used during early boot
    static ssize_t uevent_helper_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%s\n", uevent_helper);
    }
    static ssize_t uevent_helper_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    if (count+1 > UEVENT_HELPER_PATH_LEN)
    return -ENOENT;
    memcpy(uevent_helper, buf, count);
    uevent_helper[count] = '\0';
    if (count && uevent_helper[count-1] == '\n')
    uevent_helper[count-1] = '\0';
    return count;
    }
    KERNEL_ATTR_RW(uevent_helper);

    static ssize_t profiling_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", prof_on);
    }
    static ssize_t profiling_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    int ret;
    static DEFINE_MUTEX(lock);
//
// We need serialization, for profile_setup() initializes prof_on
// value and profile_init() must not reallocate prof_buffer after
// once allocated.
//
    guard(mutex)(&lock);
    if (prof_on)
    return -EEXIST;
//
// This eventually calls into get_option() which
// has a ton of callers and is not const.  It is
// easiest to cast it away here.
//
    profile_setup((char *)buf);
    ret = profile_init();
    if (ret)
    return ret;
    ret = create_proc_profile();
    if (ret)
    return ret;
    return count;
    }
    KERNEL_ATTR_RW(profiling);

    static ssize_t vmcoreinfo_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    let mut vmcore_base: phys_addr_t = paddr_vmcoreinfo_note();
    return sysfs_emit(buf, "%pa %x\n", &vmcore_base,
    (unsigned int)VMCOREINFO_NOTE_SIZE);
    }
    KERNEL_ATTR_RO(vmcoreinfo);

// whether file capabilities are enabled
    static ssize_t fscaps_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", file_caps_enabled);
    }
    KERNEL_ATTR_RO(fscaps);

    int rcu_expedited;
    static ssize_t rcu_expedited_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", READ_ONCE(rcu_expedited));
    }
    static ssize_t rcu_expedited_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    if (kstrtoint(buf, 0, &rcu_expedited))
    return -EINVAL;
    return count;
    }
    KERNEL_ATTR_RW(rcu_expedited);
    int rcu_normal;
    static ssize_t rcu_normal_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", READ_ONCE(rcu_normal));
    }
    static ssize_t rcu_normal_store(struct kobject *kobj,
    struct kobj_attribute *attr,
    const char *buf, size_t count)
    {
    if (kstrtoint(buf, 0, &rcu_normal))
    return -EINVAL;
    return count;
    }
    KERNEL_ATTR_RW(rcu_normal);

//
// Make /sys/kernel/notes give the raw contents of our kernel .notes section.
//
    extern const void __start_notes;
    extern const void __stop_notes;

    static __ro_after_init BIN_ATTR_SIMPLE_RO(notes);
    struct kobject *kernel_kobj;
    EXPORT_SYMBOL_GPL(kernel_kobj);
    static struct attribute * kernel_attrs[] = {
    &fscaps_attr.attr,
    &uevent_seqnum_attr.attr,
    &cpu_byteorder_attr.attr,
    &address_bits_attr.attr,

    &uevent_helper_attr.attr,

    &profiling_attr.attr,

    &vmcoreinfo_attr.attr,

    &rcu_expedited_attr.attr,
    &rcu_normal_attr.attr,

    core::ptr::null_mut()
    };
    static const struct attribute_group kernel_attr_group = {
    .attrs = kernel_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn ksysfs_init() -> void __init {
    void __init ksysfs_init(void)
    {
    int error;
    kernel_kobj = kobject_create_and_add("kernel", core::ptr::null_mut());
    if (!kernel_kobj) {
    error = -ENOMEM;
    goto exit;
    }
    error = sysfs_create_group(kernel_kobj, &kernel_attr_group);
    if (error)
    goto kset_exit;
    if (notes_size > 0) {
    bin_attr_notes.private = (void *)&__start_notes;
    bin_attr_notes.size = notes_size;
    error = sysfs_create_bin_file(kernel_kobj, &bin_attr_notes);
    if (error)
    goto group_exit;
    }
    return;
    group_exit:
    sysfs_remove_group(kernel_kobj, &kernel_attr_group);
    kset_exit:
    kobject_put(kernel_kobj);
    exit:
    pr_err("failed to initialize the kernel kobject: %d\n", error);
    }
