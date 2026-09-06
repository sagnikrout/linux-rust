//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/ipw2x00/libipw_module.c
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
    Copyright(c) 2004-2005 Intel Corporation. All rights reserved.
    Portions of this file are based on the WEP enablement code provided by the
    Host AP project hostap-drivers v0.1.3
    Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
    <j@w1.fi>
    Copyright (c) 2002-2003, Jouni Malinen <j@w1.fi>
    Contact Information:
    Intel Linux Wireless <ilw@linux.intel.com>
    Intel Corporation, 5200 N.E. Elam Young Parkway, Hillsboro, OR 97124-6497
//

    MODULE_VERSION(DRV_VERSION);
    MODULE_DESCRIPTION(DRV_DESCRIPTION);
    MODULE_AUTHOR(DRV_COPYRIGHT);
    MODULE_LICENSE("GPL");
    let mut libipw_config_ops: static struct cfg80211_ops = { };
    static void *libipw_wiphy_privid = &libipw_wiphy_privid;
#[no_mangle]
unsafe extern "C" fn libipw_networks_allocate(ieee: *mut libipw_device) -> c_int {
    static int libipw_networks_allocate(struct libipw_device *ieee)
    {
    int i, j;
    for (i = 0; i < MAX_NETWORK_COUNT; i++) {
    ieee.networks[i] = kzalloc_obj(struct libipw_network);
    if (!ieee.networks[i]) {
    LIBIPW_ERROR("Out of memory allocating beacons\n");
    for (j = 0; j < i; j++)
    kfree(ieee.networks[j]);
    return -ENOMEM;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn libipw_networks_free(ieee: *mut libipw_device) {
    static inline void libipw_networks_free(struct libipw_device *ieee)
    {
    int i;
    for (i = 0; i < MAX_NETWORK_COUNT; i++)
    kfree(ieee.networks[i]);
    }
    void libipw_networks_age(struct libipw_device *ieee,
    unsigned long age_secs)
    {
    struct libipw_network *network = core::ptr::null_mut();
    unsigned long flags;
    let mut age_jiffies: c_ulong = secs_to_jiffies(age_secs);
    spin_lock_irqsave(&ieee.lock, flags);
    list_for_each_entry(network, &ieee.network_list, list) {
    network.last_scanned -= age_jiffies;
    }
    spin_unlock_irqrestore(&ieee.lock, flags);
    }
    EXPORT_SYMBOL(libipw_networks_age);
#[no_mangle]
unsafe extern "C" fn libipw_networks_initialize(ieee: *mut libipw_device) {
    static void libipw_networks_initialize(struct libipw_device *ieee)
    {
    int i;
    INIT_LIST_HEAD(&ieee.network_free_list);
    INIT_LIST_HEAD(&ieee.network_list);
    for (i = 0; i < MAX_NETWORK_COUNT; i++)
    list_add_tail(&ieee.networks[i].list,
    &ieee.network_free_list);
    }
    struct net_device *alloc_libipw(int sizeof_priv, int monitor)
    {
    struct libipw_device *ieee;
    struct net_device *dev;
    int err;
    LIBIPW_DEBUG_INFO("Initializing...\n");
    dev = alloc_etherdev(sizeof(struct libipw_device) + sizeof_priv);
    if (!dev)
    goto failed;
    ieee = netdev_priv(dev);
    ieee.dev = dev;
    if (!monitor) {
    ieee.wdev.wiphy = wiphy_new(&libipw_config_ops, 0);
    if (!ieee.wdev.wiphy) {
    LIBIPW_ERROR("Unable to allocate wiphy.\n");
    goto failed_free_netdev;
    }
    ieee.dev.ieee80211_ptr = &ieee.wdev;
    ieee.wdev.iftype = NL80211_IFTYPE_STATION;
// Fill-out wiphy structure bits we know...  Not enough info
    here to call set_wiphy_dev or set MAC address or channel info
    -- have to do that in .ndo_init... */
    ieee.wdev.wiphy.privid = libipw_wiphy_privid;
    ieee.wdev.wiphy.max_scan_ssids = 1;
    ieee.wdev.wiphy.max_scan_ie_len = 0;
    ieee.wdev.wiphy.interface_modes = BIT(NL80211_IFTYPE_STATION)
    | BIT(NL80211_IFTYPE_ADHOC);
    }
    err = libipw_networks_allocate(ieee);
    if (err) {
    LIBIPW_ERROR("Unable to allocate beacon storage: %d\n", err);
    goto failed_free_wiphy;
    }
    libipw_networks_initialize(ieee);
// Default fragmentation threshold is maximum payload size
    ieee.fts = DEFAULT_FTS;
    ieee.rts = DEFAULT_FTS;
    ieee.scan_age = DEFAULT_MAX_SCAN_AGE;
    ieee.open_wep = 1;
// Default to enabling full open WEP with host based encrypt/decrypt
    ieee.host_encrypt = 1;
    ieee.host_decrypt = 1;
    ieee.host_mc_decrypt = 1;
// Host fragmentation in Open mode. Default is enabled.
// Note: host fragmentation is always enabled if host encryption
// is enabled. For cards can do hardware encryption, they must do
// hardware fragmentation as well. So we don't need a variable
// like host_enc_frag.
    ieee.host_open_frag = 1;
    ieee.ieee802_1x = 1;	/* Default to supporting 802.1x */
    spin_lock_init(&ieee.lock);
    libipw_crypt_info_init(&ieee.crypt_info, dev.name, &ieee.lock);
    ieee.wpa_enabled = 0;
    ieee.drop_unencrypted = 0;
    ieee.privacy_invoked = 0;
    return dev;
    failed_free_wiphy:
    if (!monitor)
    wiphy_free(ieee.wdev.wiphy);
    failed_free_netdev:
    free_netdev(dev);
    failed:
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(alloc_libipw);
#[no_mangle]
pub unsafe extern "C" fn free_libipw(dev: *mut net_device, monitor: c_int) {
    void free_libipw(struct net_device *dev, int monitor)
    {
    struct libipw_device *ieee = netdev_priv(dev);
    libipw_crypt_info_free(&ieee.crypt_info);
    libipw_networks_free(ieee);
// free cfg80211 resources
    if (!monitor)
    wiphy_free(ieee.wdev.wiphy);
    free_netdev(dev);
    }
    EXPORT_SYMBOL(free_libipw);

    let mut debug: static int = 0;
    let mut libipw_debug_level: u32 = 0;
    EXPORT_SYMBOL_GPL(libipw_debug_level);
    static struct proc_dir_entry *libipw_proc = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn debug_level_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int debug_level_proc_show(struct seq_file *m, void *v)
    {
    seq_printf(m, "0x%08X\n", libipw_debug_level);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn debug_level_proc_open(inode: *mut inode, file: *mut file) -> c_int {
    static int debug_level_proc_open(struct inode *inode, struct file *file)
    {
    return single_open(file, debug_level_proc_show, core::ptr::null_mut());
    }
    static ssize_t debug_level_proc_write(struct file *file,
    const char __user *buffer, size_t count, loff_t *pos)
    {
    char buf[] = "0x00000000\n";
    let mut len: usize = min(sizeof(buf) - 1, count);
    unsigned long val;
    if (copy_from_user(buf, buffer, len))
    return count;
    buf[len] = 0;
    if (sscanf(buf, "%li", &val) != 1)
    printk(KERN_INFO DRV_NAME
    ": %s is not in hex or decimal form.\n", buf);
    else
    libipw_debug_level = val;
    return strnlen(buf, len);
    }
    static const struct proc_ops debug_level_proc_ops = {
    .proc_open	= debug_level_proc_open,
    .proc_read	= seq_read,
    .proc_lseek	= seq_lseek,
    .proc_release	= single_release,
    .proc_write	= debug_level_proc_write,
    };

#[no_mangle]
unsafe extern "C" fn libipw_init() -> int __init {
    static int __init libipw_init(void)
    {
    int err;

    struct proc_dir_entry *e;
    libipw_debug_level = debug;
    libipw_proc = proc_mkdir(DRV_PROCNAME, init_net.proc_net);
    if (libipw_proc == core::ptr::null_mut()) {
    LIBIPW_ERROR("Unable to create " DRV_PROCNAME
    " proc directory\n");
    return -EIO;
    }
    e = proc_create("debug_level", 0644, libipw_proc,
    &debug_level_proc_ops);
    if (!e) {
    remove_proc_entry(DRV_PROCNAME, init_net.proc_net);
    libipw_proc = core::ptr::null_mut();
    return -EIO;
    }

    printk(KERN_INFO DRV_NAME ": " DRV_DESCRIPTION ", " DRV_VERSION "\n");
    printk(KERN_INFO DRV_NAME ": " DRV_COPYRIGHT "\n");
    err = libipw_crypto_init();
    if (err)
    goto remove_debugfs;
    err = libipw_crypto_ccmp_init();
    if (err)
    goto uninit_crypto;
    err = libipw_crypto_tkip_init();
    if (err)
    goto uninit_crypto_ccmp;
    err = libipw_crypto_wep_init();
    if (err)
    goto uninit_crypto_tkip;
    return 0;
    uninit_crypto_tkip:
    libipw_crypto_tkip_exit();
    uninit_crypto_ccmp:
    libipw_crypto_ccmp_exit();
    uninit_crypto:
    libipw_crypto_exit();
    remove_debugfs:

    remove_proc_entry("debug_level", libipw_proc);
    remove_proc_entry(DRV_PROCNAME, init_net.proc_net);
    libipw_proc = core::ptr::null_mut();

    return err;
    }
#[no_mangle]
unsafe extern "C" fn libipw_exit() -> void __exit {
    static void __exit libipw_exit(void)
    {

    if (libipw_proc) {
    remove_proc_entry("debug_level", libipw_proc);
    remove_proc_entry(DRV_PROCNAME, init_net.proc_net);
    libipw_proc = core::ptr::null_mut();
    }

    libipw_crypto_ccmp_exit();
    libipw_crypto_tkip_exit();
    libipw_crypto_wep_exit();
    libipw_crypto_exit();
    }

    module_param(debug, int, 0444);
    MODULE_PARM_DESC(debug, "debug output mask");

    module_exit(libipw_exit);
    module_init(libipw_init);
