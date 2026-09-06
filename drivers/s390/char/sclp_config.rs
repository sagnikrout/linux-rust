//! Automatically rewritten from C to Rust
//! Source: drivers/s390/char/sclp_config.c
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
// Copyright IBM Corp. 2007
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_mgm_data {
    pub reserved: u8,
    pub ev_qualifier: u8,
    pub __attribute__((packed)): },
pub const OFB_DATA_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_ofb_evbuf {
    pub header: evbuf_header,
    pub cm_data: conf_mgm_data,
    pub ev_data: [c_char; OFB_DATA_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sclp_ofb_sccb {
    pub header: sccb_header,
    pub ofb_evbuf: sclp_ofb_evbuf,
    pub __packed: },
pub const EV_QUAL_CPU_CHANGE: c_int = 1;
pub const EV_QUAL_CAP_CHANGE: c_int = 3;
pub const EV_QUAL_OPEN4BUSINESS: c_int = 5;
    pub sclp_cpu_capability_work: static struct work_struct,
    pub sclp_cpu_change_work: static struct work_struct,
#[no_mangle]
unsafe extern "C" fn sclp_cpu_capability_notify(work: *mut work_struct) {
    static void sclp_cpu_capability_notify(struct work_struct *work)
    {
    pub cpu: c_int,
    pub dev: *mut device,
    pub changed\n"): pr_info("CPU capability may have,
    for_each_online_cpu(cpu) {
    pub get_cpu_device(cpu): dev =,
    pub KOBJ_CHANGE): kobject_uevent(&dev->kobj,,
    }
    }
#[no_mangle]
unsafe extern "C" fn sclp_cpu_change_notify(work: *mut work_struct) -> void __ref {
    static void __ref sclp_cpu_change_notify(struct work_struct *work)
    {
    }
#[no_mangle]
unsafe extern "C" fn sclp_conf_receiver_fn(evbuf: *mut evbuf_header) {
    static void sclp_conf_receiver_fn(struct evbuf_header *evbuf)
    {
    pub cdata: *mut conf_mgm_data,
    pub 1): *mut *mut cdata = (struct conf_mgm_data )(evbuf +,
    switch (cdata.ev_qualifier) {
    case EV_QUAL_CPU_CHANGE:
    case EV_QUAL_CAP_CHANGE:
    }
    }
    static struct sclp_register sclp_conf_register =
    {
    .send_mask    = EVTYP_CONFMGMDATA_MASK,
    .receive_mask = EVTYP_CONFMGMDATA_MASK,
    .receiver_fn  = sclp_conf_receiver_fn,
}

#[no_mangle]
unsafe extern "C" fn sclp_ofb_send_req(ev_data: *mut c_char, len: usize) -> c_int {
    static int sclp_ofb_send_req(char *ev_data, size_t len)
    {
    static DEFINE_MUTEX(send_mutex);
    struct sclp_ofb_sccb *sccb;
    int rc, response;
    if (len > OFB_DATA_MAX)
    return -EINVAL;
    sccb = (struct sclp_ofb_sccb *) get_zeroed_page(GFP_KERNEL | GFP_DMA);
    if (!sccb)
    return -ENOMEM;
// Setup SCCB for Control-Program Identification
    sccb.header.length = sizeof(struct sclp_ofb_sccb);
    sccb.ofb_evbuf.header.length = sizeof(struct sclp_ofb_evbuf);
    sccb.ofb_evbuf.header.type = EVTYP_CONFMGMDATA;
    sccb.ofb_evbuf.cm_data.ev_qualifier = EV_QUAL_OPEN4BUSINESS;
    memcpy(sccb.ofb_evbuf.ev_data, ev_data, len);
    if (!(sclp_conf_register.sclp_receive_mask & EVTYP_CONFMGMDATA_MASK))
    pr_warn("SCLP receiver did not register to receive "
    "Configuration Management Data Events.\n");
    mutex_lock(&send_mutex);
    rc = sclp_sync_request(SCLP_CMDW_WRITE_EVENT_DATA, sccb);
    mutex_unlock(&send_mutex);
    if (rc)
    goto out;
    response = sccb.header.response_code;
    if (response != 0x0020) {
    pr_err("Open for Business request failed with response code "
    "0x%04x\n", response);
    rc = -EIO;
    }
    out:
    free_page((unsigned long)sccb);
    return rc;
    }
    static ssize_t sysfs_ofb_data_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    int rc;
    rc = sclp_ofb_send_req(buf, count);
    return rc ?: count;
    }
    static const struct bin_attribute ofb_bin_attr = {
    .attr = {
    .name = "event_data",
    .mode = S_IWUSR,
    },
    .write = sysfs_ofb_data_write,
    };
#[no_mangle]
unsafe extern "C" fn sclp_ofb_setup() -> int __init {
    static int __init sclp_ofb_setup(void)
    {
    struct kset *ofb_kset;
    int rc;
    ofb_kset = kset_create_and_add("ofb", core::ptr::null_mut(), firmware_kobj);
    if (!ofb_kset)
    return -ENOMEM;
    rc = sysfs_create_bin_file(&ofb_kset.kobj, &ofb_bin_attr);
    if (rc) {
    kset_unregister(ofb_kset);
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sclp_conf_init() -> int __init {
    static int __init sclp_conf_init(void)
    {
    int rc;
    INIT_WORK(&sclp_cpu_capability_work, sclp_cpu_capability_notify);
    INIT_WORK(&sclp_cpu_change_work, sclp_cpu_change_notify);
    rc = sclp_register(&sclp_conf_register);
    if (rc)
    return rc;
    return sclp_ofb_setup();
    }
    __initcall(sclp_conf_init);
