//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/sclp_ocf.c
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
// SCLP OCF communication parameters sysfs interface
//
// Copyright IBM Corp. 2011
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//

    static char hmc_network[OCF_LENGTH_HMC_NETWORK + 1];
    static char cpc_name[OCF_LENGTH_CPC_NAME]; /* in EBCDIC */
    static DEFINE_SPINLOCK(sclp_ocf_lock);
    static struct work_struct sclp_ocf_change_work;
    static struct kset *ocf_kset;
#[no_mangle]
unsafe extern "C" fn sclp_ocf_change_notify(work: *mut work_struct) {
    static void sclp_ocf_change_notify(struct work_struct *work)
    {
    kobject_uevent(&ocf_kset.kobj, KOBJ_CHANGE);
    }
// Handler for OCF event. Look for the CPC image name.
#[no_mangle]
unsafe extern "C" fn sclp_ocf_handler(evbuf: *mut evbuf_header) {
    static void sclp_ocf_handler(struct evbuf_header *evbuf)
    {
    struct gds_vector *v;
    struct gds_subvector *sv, *netid, *cpc;
    size_t size;
// Find the 0x9f00 block.
    v = sclp_find_gds_vector(evbuf + 1, (void *) evbuf + evbuf.length,
    0x9f00);
    if (!v)
    return;
// Find the 0x9f22 block inside the 0x9f00 block.
    v = sclp_find_gds_vector(v + 1, (void *) v + v.length, 0x9f22);
    if (!v)
    return;
// Find the 0x81 block inside the 0x9f22 block.
    sv = sclp_find_gds_subvector(v + 1, (void *) v + v.length, 0x81);
    if (!sv)
    return;
// Find the 0x01 block inside the 0x81 block.
    netid = sclp_find_gds_subvector(sv + 1, (void *) sv + sv.length, 1);
// Find the 0x02 block inside the 0x81 block.
    cpc = sclp_find_gds_subvector(sv + 1, (void *) sv + sv.length, 2);
// Copy network name and cpc name.
    spin_lock(&sclp_ocf_lock);
    if (netid) {
    size = min(OCF_LENGTH_HMC_NETWORK, (size_t) netid.length);
    memcpy(hmc_network, netid + 1, size);
    EBCASC(hmc_network, size);
    hmc_network[size] = 0;
    }
    if (cpc) {
    size = min(OCF_LENGTH_CPC_NAME, (size_t) cpc.length);
    memset(cpc_name, 0, OCF_LENGTH_CPC_NAME);
    memcpy(cpc_name, cpc + 1, size);
    }
    spin_unlock(&sclp_ocf_lock);
    schedule_work(&sclp_ocf_change_work);
    }
    static struct sclp_register sclp_ocf_event = {
    .receive_mask = EVTYP_OCF_MASK,
    .receiver_fn = sclp_ocf_handler,
    };
#[no_mangle]
pub unsafe extern "C" fn sclp_ocf_cpc_name_copy(dst: *mut c_char) {
    void sclp_ocf_cpc_name_copy(char *dst)
    {
    spin_lock_irq(&sclp_ocf_lock);
    memcpy(dst, cpc_name, OCF_LENGTH_CPC_NAME);
    spin_unlock_irq(&sclp_ocf_lock);
    }
    EXPORT_SYMBOL(sclp_ocf_cpc_name_copy);
    static ssize_t cpc_name_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *page)
    {
    char name[OCF_LENGTH_CPC_NAME + 1];
    sclp_ocf_cpc_name_copy(name);
    name[OCF_LENGTH_CPC_NAME] = 0;
    EBCASC(name, OCF_LENGTH_CPC_NAME);
    return sysfs_emit(page, "%s\n", name);
    }
    static struct kobj_attribute cpc_name_attr =
    __ATTR(cpc_name, 0444, cpc_name_show, core::ptr::null_mut());
    static ssize_t hmc_network_show(struct kobject *kobj,
    struct kobj_attribute *attr, char *page)
    {
    int rc;
    spin_lock_irq(&sclp_ocf_lock);
    rc = sysfs_emit(page, "%s\n", hmc_network);
    spin_unlock_irq(&sclp_ocf_lock);
    return rc;
    }
    static struct kobj_attribute hmc_network_attr =
    __ATTR(hmc_network, 0444, hmc_network_show, core::ptr::null_mut());
    static struct attribute *ocf_attrs[] = {
    &cpc_name_attr.attr,
    &hmc_network_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ocf_attr_group = {
    .attrs = ocf_attrs,
    };
#[no_mangle]
unsafe extern "C" fn ocf_init() -> int __init {
    static int __init ocf_init(void)
    {
    int rc;
    INIT_WORK(&sclp_ocf_change_work, sclp_ocf_change_notify);
    ocf_kset = kset_create_and_add("ocf", core::ptr::null_mut(), firmware_kobj);
    if (!ocf_kset)
    return -ENOMEM;
    rc = sysfs_create_group(&ocf_kset.kobj, &ocf_attr_group);
    if (rc) {
    kset_unregister(ocf_kset);
    return rc;
    }
    return sclp_register(&sclp_ocf_event);
    }
    device_initcall(ocf_init);
