//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/iscsi_ibft.c
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
// Copyright 2007-2010 Red Hat, Inc.
// by Peter Jones <pjones@redhat.com>
// Copyright 2008 IBM, Inc.
// by Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Copyright 2008
// by Konrad Rzeszutek <ketuzsezr@darnok.org>
//
// This code exposes the iSCSI Boot Format Table to userland via sysfs.
//
// Changelog:
//
// 06 Jan 2010 - Peter Jones <pjones@redhat.com>
// New changelog entries are in the git log from now on.  Not here.
//
// 14 Mar 2008 - Konrad Rzeszutek <ketuzsezr@darnok.org>
// Updated comments and copyrights. (v0.4.9)
//
// 11 Feb 2008 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Converted to using ibft_addr. (v0.4.8)
//
// 8 Feb 2008 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Combined two functions in one: reserve_ibft_region. (v0.4.7)
//
// 30 Jan 2008 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Added logic to handle IPv6 addresses. (v0.4.6)
//
// 25 Jan 2008 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Added logic to handle badly not-to-spec iBFT. (v0.4.5)
//
// 4 Jan 2008 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Added __init to function declarations. (v0.4.4)
//
// 21 Dec 2007 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Updated kobject registration, combined unregister functions in one
// and code and style cleanup. (v0.4.3)
//
// 5 Dec 2007 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Added end-markers to enums and re-organized kobject registration. (v0.4.2)
//
// 4 Dec 2007 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Created 'device' sysfs link to the NIC and style cleanup. (v0.4.1)
//
// 28 Nov 2007 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Added sysfs-ibft documentation, moved 'find_ibft' function to
// in its own file and added text attributes for every struct field.  (v0.4)
//
// 21 Nov 2007 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Added text attributes emulating OpenFirmware /proc/device-tree naming.
// Removed binary /sysfs interface (v0.3)
//
// 29 Aug 2007 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Added functionality in setup.c to reserve iBFT region. (v0.2)
//
// 27 Aug 2007 - Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// First version exposing iBFT data via a binary /sysfs. (v0.1)
//

    MODULE_AUTHOR("Peter Jones <pjones@redhat.com> and "
    "Konrad Rzeszutek <ketuzsezr@darnok.org>");
    MODULE_DESCRIPTION("sysfs interface to BIOS iBFT information");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(IBFT_ISCSI_VERSION);
    static struct acpi_table_ibft *ibft_addr;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibft_hdr {
    pub id: u8,
    pub version: u8,
    pub length: u16,
    pub index: u8,
    pub flags: u8,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibft_control {
    pub hdr: ibft_hdr,
    pub extensions: u16,
    pub initiator_off: u16,
    pub nic0_off: u16,
    pub tgt0_off: u16,
    pub nic1_off: u16,
    pub tgt1_off: u16,
    pub expansion: [u16; ],
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibft_initiator {
    pub hdr: ibft_hdr,
    pub isns_server: [c_char; 16],
    pub slp_server: [c_char; 16],
    pub pri_radius_server: [c_char; 16],
    pub sec_radius_server: [c_char; 16],
    pub initiator_name_len: u16,
    pub initiator_name_off: u16,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibft_nic {
    pub hdr: ibft_hdr,
    pub ip_addr: [c_char; 16],
    pub subnet_mask_prefix: u8,
    pub origin: u8,
    pub gateway: [c_char; 16],
    pub primary_dns: [c_char; 16],
    pub secondary_dns: [c_char; 16],
    pub dhcp: [c_char; 16],
    pub vlan: u16,
    pub mac: [c_char; 6],
    pub pci_bdf: u16,
    pub hostname_len: u16,
    pub hostname_off: u16,
    pub __attribute__((__packed__)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibft_tgt {
    pub hdr: ibft_hdr,
    pub ip_addr: [c_char; 16],
    pub port: u16,
    pub lun: [c_char; 8],
    pub chap_type: u8,
    pub nic_assoc: u8,
    pub tgt_name_len: u16,
    pub tgt_name_off: u16,
    pub chap_name_len: u16,
    pub chap_name_off: u16,
    pub chap_secret_len: u16,
    pub chap_secret_off: u16,
    pub rev_chap_name_len: u16,
    pub rev_chap_name_off: u16,
    pub rev_chap_secret_len: u16,
    pub rev_chap_secret_off: u16,
    pub __attribute__((__packed__)): },
//
// The kobject different types and its names.
//
    enum ibft_id {
    id_reserved = 0, /* We don't support. */
    id_control = 1, /* Should show up only once and is not exported. */
    id_initiator = 2,
    id_nic = 3,
    id_target = 4,
    id_extensions = 5, /* We don't support. */
    id_end_marker,
}

//
// The kobject and attribute structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibft_kobject {
    pub header: *mut acpi_table_ibft,
    union {
    pub initiator: *mut ibft_initiator,
    pub nic: *mut ibft_nic,
    pub tgt: *mut ibft_tgt,
    pub hdr: *mut ibft_hdr,
}

    };
    static struct iscsi_boot_kset *boot_kset;
// fully null address
    static const char nulls[16];
// IPv4-mapped IPv6 ::ffff:0.0.0.0
    static const char mapped_nulls[16] = { 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xff, 0xff,
    0x00, 0x00, 0x00, 0x00 };
#[no_mangle]
unsafe extern "C" fn address_not_null(ip: *mut u8) -> c_int {
    static int address_not_null(u8 *ip)
    {
    return (memcmp(ip, nulls, 16) && memcmp(ip, mapped_nulls, 16));
    }
//
// Helper functions to parse data properly.
//
#[no_mangle]
unsafe extern "C" fn sprintf_ipaddr(buf: *mut c_char, ip: *mut u8) -> isize {
    static ssize_t sprintf_ipaddr(char *buf, u8 *ip)
    {
    char *str = buf;
    if (ip[0] == 0 && ip[1] == 0 && ip[2] == 0 && ip[3] == 0 &&
    ip[4] == 0 && ip[5] == 0 && ip[6] == 0 && ip[7] == 0 &&
    ip[8] == 0 && ip[9] == 0 && ip[10] == 0xff && ip[11] == 0xff) {
//
// IPV4
//
    str += sprintf(buf, "%pI4", ip + 12);
    } else {
//
// IPv6
//
    str += sprintf(str, "%pI6", ip);
    }
    str += sprintf(str, "\n");
    return str - buf;
    }
#[no_mangle]
unsafe extern "C" fn sprintf_string(str: *mut c_char, len: c_int, buf: *mut c_char) -> isize {
    static ssize_t sprintf_string(char *str, int len, char *buf)
    {
    return sprintf(str, "%.*s\n", len, buf);
    }
//
// Helper function to verify the IBFT header.
//
#[no_mangle]
unsafe extern "C" fn ibft_verify_hdr(t: *mut c_char, hdr: *mut ibft_hdr, id: c_int, length: c_int) -> c_int {
    static int ibft_verify_hdr(char *t, struct ibft_hdr *hdr, int id, int length)
    {
    if (hdr.id != id) {
    printk(KERN_ERR "iBFT error: We expected the %s " \
    "field header.id to have %d but " \
    "found %d instead!\n", t, id, hdr.id);
    return -ENODEV;
    }
    if (length && hdr.length != length) {
    printk(KERN_ERR "iBFT error: We expected the %s " \
    "field header.length to have %d but " \
    "found %d instead!\n", t, length, hdr.length);
    return -ENODEV;
    }
    return 0;
    }
//
// Routines for parsing the iBFT data to be human readable.
//
#[no_mangle]
unsafe extern "C" fn ibft_attr_show_initiator(data: *mut c_void, type: c_int, buf: *mut c_char) -> isize {
    static ssize_t ibft_attr_show_initiator(void *data, int type, char *buf)
    {
    struct ibft_kobject *entry = data;
    struct ibft_initiator *initiator = entry.initiator;
    void *ibft_loc = entry.header;
    char *str = buf;
    if (!initiator)
    return 0;
    switch (type) {
    case ISCSI_BOOT_INI_INDEX:
    str += sprintf(str, "%d\n", initiator.hdr.index);
    break;
    case ISCSI_BOOT_INI_FLAGS:
    str += sprintf(str, "%d\n", initiator.hdr.flags);
    break;
    case ISCSI_BOOT_INI_ISNS_SERVER:
    str += sprintf_ipaddr(str, initiator.isns_server);
    break;
    case ISCSI_BOOT_INI_SLP_SERVER:
    str += sprintf_ipaddr(str, initiator.slp_server);
    break;
    case ISCSI_BOOT_INI_PRI_RADIUS_SERVER:
    str += sprintf_ipaddr(str, initiator.pri_radius_server);
    break;
    case ISCSI_BOOT_INI_SEC_RADIUS_SERVER:
    str += sprintf_ipaddr(str, initiator.sec_radius_server);
    break;
    case ISCSI_BOOT_INI_INITIATOR_NAME:
    str += sprintf_string(str, initiator.initiator_name_len,
    (char *)ibft_loc +
    initiator.initiator_name_off);
    break;
    default:
    break;
    }
    return str - buf;
    }
#[no_mangle]
unsafe extern "C" fn ibft_attr_show_nic(data: *mut c_void, type: c_int, buf: *mut c_char) -> isize {
    static ssize_t ibft_attr_show_nic(void *data, int type, char *buf)
    {
    struct ibft_kobject *entry = data;
    struct ibft_nic *nic = entry.nic;
    void *ibft_loc = entry.header;
    char *str = buf;
    __be32 val;
    if (!nic)
    return 0;
    switch (type) {
    case ISCSI_BOOT_ETH_INDEX:
    str += sprintf(str, "%d\n", nic.hdr.index);
    break;
    case ISCSI_BOOT_ETH_FLAGS:
    str += sprintf(str, "%d\n", nic.hdr.flags);
    break;
    case ISCSI_BOOT_ETH_IP_ADDR:
    str += sprintf_ipaddr(str, nic.ip_addr);
    break;
    case ISCSI_BOOT_ETH_SUBNET_MASK:
    if (nic.subnet_mask_prefix > 32)
    val = cpu_to_be32(~0);
    else
    val = cpu_to_be32(~((1 << (32-nic.subnet_mask_prefix))-1));
    str += sprintf(str, "%pI4", &val);
    break;
    case ISCSI_BOOT_ETH_PREFIX_LEN:
    str += sprintf(str, "%d\n", nic.subnet_mask_prefix);
    break;
    case ISCSI_BOOT_ETH_ORIGIN:
    str += sprintf(str, "%d\n", nic.origin);
    break;
    case ISCSI_BOOT_ETH_GATEWAY:
    str += sprintf_ipaddr(str, nic.gateway);
    break;
    case ISCSI_BOOT_ETH_PRIMARY_DNS:
    str += sprintf_ipaddr(str, nic.primary_dns);
    break;
    case ISCSI_BOOT_ETH_SECONDARY_DNS:
    str += sprintf_ipaddr(str, nic.secondary_dns);
    break;
    case ISCSI_BOOT_ETH_DHCP:
    str += sprintf_ipaddr(str, nic.dhcp);
    break;
    case ISCSI_BOOT_ETH_VLAN:
    str += sprintf(str, "%d\n", nic.vlan);
    break;
    case ISCSI_BOOT_ETH_MAC:
    str += sprintf(str, "%pM\n", nic.mac);
    break;
    case ISCSI_BOOT_ETH_HOSTNAME:
    str += sprintf_string(str, nic.hostname_len,
    (char *)ibft_loc + nic.hostname_off);
    break;
    default:
    break;
    }
    return str - buf;
    };
#[no_mangle]
unsafe extern "C" fn ibft_attr_show_target(data: *mut c_void, type: c_int, buf: *mut c_char) -> isize {
    static ssize_t ibft_attr_show_target(void *data, int type, char *buf)
    {
    struct ibft_kobject *entry = data;
    struct ibft_tgt *tgt = entry.tgt;
    void *ibft_loc = entry.header;
    char *str = buf;
    int i;
    if (!tgt)
    return 0;
    switch (type) {
    case ISCSI_BOOT_TGT_INDEX:
    str += sprintf(str, "%d\n", tgt.hdr.index);
    break;
    case ISCSI_BOOT_TGT_FLAGS:
    str += sprintf(str, "%d\n", tgt.hdr.flags);
    break;
    case ISCSI_BOOT_TGT_IP_ADDR:
    str += sprintf_ipaddr(str, tgt.ip_addr);
    break;
    case ISCSI_BOOT_TGT_PORT:
    str += sprintf(str, "%d\n", tgt.port);
    break;
    case ISCSI_BOOT_TGT_LUN:
    for (i = 0; i < 8; i++)
    str += sprintf(str, "%x", (u8)tgt.lun[i]);
    str += sprintf(str, "\n");
    break;
    case ISCSI_BOOT_TGT_NIC_ASSOC:
    str += sprintf(str, "%d\n", tgt.nic_assoc);
    break;
    case ISCSI_BOOT_TGT_CHAP_TYPE:
    str += sprintf(str, "%d\n", tgt.chap_type);
    break;
    case ISCSI_BOOT_TGT_NAME:
    str += sprintf_string(str, tgt.tgt_name_len,
    (char *)ibft_loc + tgt.tgt_name_off);
    break;
    case ISCSI_BOOT_TGT_CHAP_NAME:
    str += sprintf_string(str, tgt.chap_name_len,
    (char *)ibft_loc + tgt.chap_name_off);
    break;
    case ISCSI_BOOT_TGT_CHAP_SECRET:
    str += sprintf_string(str, tgt.chap_secret_len,
    (char *)ibft_loc + tgt.chap_secret_off);
    break;
    case ISCSI_BOOT_TGT_REV_CHAP_NAME:
    str += sprintf_string(str, tgt.rev_chap_name_len,
    (char *)ibft_loc +
    tgt.rev_chap_name_off);
    break;
    case ISCSI_BOOT_TGT_REV_CHAP_SECRET:
    str += sprintf_string(str, tgt.rev_chap_secret_len,
    (char *)ibft_loc +
    tgt.rev_chap_secret_off);
    break;
    default:
    break;
    }
    return str - buf;
    }
#[no_mangle]
unsafe extern "C" fn ibft_attr_show_acpitbl(data: *mut c_void, type: c_int, buf: *mut c_char) -> isize {
    static ssize_t ibft_attr_show_acpitbl(void *data, int type, char *buf)
    {
    struct ibft_kobject *entry = data;
    char *str = buf;
    switch (type) {
    case ISCSI_BOOT_ACPITBL_SIGNATURE:
    str += sprintf_string(str, ACPI_NAMESEG_SIZE,
    entry.header.header.signature);
    break;
    case ISCSI_BOOT_ACPITBL_OEM_ID:
    str += sprintf_string(str, ACPI_OEM_ID_SIZE,
    entry.header.header.oem_id);
    break;
    case ISCSI_BOOT_ACPITBL_OEM_TABLE_ID:
    str += sprintf_string(str, ACPI_OEM_TABLE_ID_SIZE,
    entry.header.header.oem_table_id);
    break;
    default:
    break;
    }
    return str - buf;
    }
#[no_mangle]
unsafe extern "C" fn ibft_check_device() -> int __init {
    static int __init ibft_check_device(void)
    {
    int len;
    u8 *pos;
    let mut csum: u8 = 0;
    len = ibft_addr.header.length;
// Sanity checking of iBFT.
    if (ibft_addr.header.revision != 1) {
    printk(KERN_ERR "iBFT module supports only revision 1, " \
    "while this is %d.\n",
    ibft_addr.header.revision);
    return -ENOENT;
    }
    for (pos = (u8 *)ibft_addr; pos < (u8 *)ibft_addr + len; pos++)
    csum += *pos;
    if (csum) {
    printk(KERN_ERR "iBFT has incorrect checksum (0x%x)!\n", csum);
    return -ENOENT;
    }
    return 0;
    }
//
// Helper routiners to check to determine if the entry is valid
// in the proper iBFT structure.
//
#[no_mangle]
unsafe extern "C" fn ibft_check_nic_for(data: *mut c_void, type: c_int) -> umode_t {
    static umode_t ibft_check_nic_for(void *data, int type)
    {
    struct ibft_kobject *entry = data;
    struct ibft_nic *nic = entry.nic;
    let mut rc: umode_t = 0;
    switch (type) {
    case ISCSI_BOOT_ETH_INDEX:
    case ISCSI_BOOT_ETH_FLAGS:
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_ETH_IP_ADDR:
    if (address_not_null(nic.ip_addr))
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_ETH_PREFIX_LEN:
    case ISCSI_BOOT_ETH_SUBNET_MASK:
    if (nic.subnet_mask_prefix)
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_ETH_ORIGIN:
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_ETH_GATEWAY:
    if (address_not_null(nic.gateway))
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_ETH_PRIMARY_DNS:
    if (address_not_null(nic.primary_dns))
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_ETH_SECONDARY_DNS:
    if (address_not_null(nic.secondary_dns))
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_ETH_DHCP:
    if (address_not_null(nic.dhcp))
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_ETH_VLAN:
    case ISCSI_BOOT_ETH_MAC:
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_ETH_HOSTNAME:
    if (nic.hostname_off)
    rc = S_IRUGO;
    break;
    default:
    break;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ibft_check_tgt_for(data: *mut c_void, type: c_int) -> umode_t __init {
    static umode_t __init ibft_check_tgt_for(void *data, int type)
    {
    struct ibft_kobject *entry = data;
    struct ibft_tgt *tgt = entry.tgt;
    let mut rc: umode_t = 0;
    switch (type) {
    case ISCSI_BOOT_TGT_INDEX:
    case ISCSI_BOOT_TGT_FLAGS:
    case ISCSI_BOOT_TGT_IP_ADDR:
    case ISCSI_BOOT_TGT_PORT:
    case ISCSI_BOOT_TGT_LUN:
    case ISCSI_BOOT_TGT_NIC_ASSOC:
    case ISCSI_BOOT_TGT_CHAP_TYPE:
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_TGT_NAME:
    if (tgt.tgt_name_len)
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_TGT_CHAP_NAME:
    case ISCSI_BOOT_TGT_CHAP_SECRET:
    if (tgt.chap_name_len)
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_TGT_REV_CHAP_NAME:
    case ISCSI_BOOT_TGT_REV_CHAP_SECRET:
    if (tgt.rev_chap_name_len)
    rc = S_IRUGO;
    break;
    default:
    break;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ibft_check_initiator_for(data: *mut c_void, type: c_int) -> umode_t __init {
    static umode_t __init ibft_check_initiator_for(void *data, int type)
    {
    struct ibft_kobject *entry = data;
    struct ibft_initiator *init = entry.initiator;
    let mut rc: umode_t = 0;
    switch (type) {
    case ISCSI_BOOT_INI_INDEX:
    case ISCSI_BOOT_INI_FLAGS:
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_INI_ISNS_SERVER:
    if (address_not_null(init.isns_server))
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_INI_SLP_SERVER:
    if (address_not_null(init.slp_server))
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_INI_PRI_RADIUS_SERVER:
    if (address_not_null(init.pri_radius_server))
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_INI_SEC_RADIUS_SERVER:
    if (address_not_null(init.sec_radius_server))
    rc = S_IRUGO;
    break;
    case ISCSI_BOOT_INI_INITIATOR_NAME:
    if (init.initiator_name_len)
    rc = S_IRUGO;
    break;
    default:
    break;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ibft_check_acpitbl_for(data: *mut c_void, type: c_int) -> umode_t __init {
    static umode_t __init ibft_check_acpitbl_for(void *data, int type)
    {
    let mut rc: umode_t = 0;
    switch (type) {
    case ISCSI_BOOT_ACPITBL_SIGNATURE:
    case ISCSI_BOOT_ACPITBL_OEM_ID:
    case ISCSI_BOOT_ACPITBL_OEM_TABLE_ID:
    rc = S_IRUGO;
    break;
    default:
    break;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ibft_kobj_release(data: *mut c_void) {
    static void ibft_kobj_release(void *data)
    {
    kfree(data);
    }
//
// Helper function for ibft_register_kobjects.
//
    static int __init ibft_create_kobject(struct acpi_table_ibft *header,
    struct ibft_hdr *hdr)
    {
    struct iscsi_boot_kobj *boot_kobj = core::ptr::null_mut();
    struct ibft_kobject *ibft_kobj = core::ptr::null_mut();
    struct ibft_nic *nic = (struct ibft_nic *)hdr;
    struct pci_dev *pci_dev;
    let mut rc: c_int = 0;
    ibft_kobj = kzalloc_obj(*ibft_kobj);
    if (!ibft_kobj)
    return -ENOMEM;
    ibft_kobj.header = header;
    ibft_kobj.hdr = hdr;
    switch (hdr.id) {
    case id_initiator:
    rc = ibft_verify_hdr("initiator", hdr, id_initiator,
    sizeof(*ibft_kobj.initiator));
    if (rc)
    break;
    boot_kobj = iscsi_boot_create_initiator(boot_kset, hdr.index,
    ibft_kobj,
    ibft_attr_show_initiator,
    ibft_check_initiator_for,
    ibft_kobj_release);
    if (!boot_kobj) {
    rc = -ENOMEM;
    goto free_ibft_obj;
    }
    break;
    case id_nic:
    rc = ibft_verify_hdr("ethernet", hdr, id_nic,
    sizeof(*ibft_kobj.nic));
    if (rc)
    break;
    boot_kobj = iscsi_boot_create_ethernet(boot_kset, hdr.index,
    ibft_kobj,
    ibft_attr_show_nic,
    ibft_check_nic_for,
    ibft_kobj_release);
    if (!boot_kobj) {
    rc = -ENOMEM;
    goto free_ibft_obj;
    }
    break;
    case id_target:
    rc = ibft_verify_hdr("target", hdr, id_target,
    sizeof(*ibft_kobj.tgt));
    if (rc)
    break;
    boot_kobj = iscsi_boot_create_target(boot_kset, hdr.index,
    ibft_kobj,
    ibft_attr_show_target,
    ibft_check_tgt_for,
    ibft_kobj_release);
    if (!boot_kobj) {
    rc = -ENOMEM;
    goto free_ibft_obj;
    }
    break;
    case id_reserved:
    case id_control:
    case id_extensions:
// Fields which we don't support. Ignore them
    rc = 1;
    break;
    default:
    printk(KERN_ERR "iBFT has unknown structure type (%d). " \
    "Report this bug to %.6s!\n", hdr.id,
    header.header.oem_id);
    rc = 1;
    break;
    }
    if (rc) {
// Skip adding this kobject, but exit with non-fatal error.
    rc = 0;
    goto free_ibft_obj;
    }
    if (hdr.id == id_nic) {
//
// We don't search for the device in other domains than
// zero. This is because on x86 platforms the BIOS
// executes only devices which are in domain 0. Furthermore, the
// iBFT spec doesn't have a domain id field :-(
//
    pci_dev = pci_get_domain_bus_and_slot(0,
    (nic.pci_bdf & 0xff00) >> 8,
    (nic.pci_bdf & 0xff));
    if (pci_dev) {
    rc = sysfs_create_link(&boot_kobj.kobj,
    &pci_dev.dev.kobj, "device");
    pci_dev_put(pci_dev);
    }
    }
    return 0;
    free_ibft_obj:
    kfree(ibft_kobj);
    return rc;
    }
//
// Scan the IBFT table structure for the NIC and Target fields. When
// found add them on the passed-in list. We do not support the other
// fields at this point, so they are skipped.
//
#[no_mangle]
unsafe extern "C" fn ibft_register_kobjects(header: *mut acpi_table_ibft) -> int __init {
    static int __init ibft_register_kobjects(struct acpi_table_ibft *header)
    {
    struct ibft_control *control = core::ptr::null_mut();
    struct iscsi_boot_kobj *boot_kobj;
    struct ibft_kobject *ibft_kobj;
    void *ptr, *end;
    let mut rc: c_int = 0;
    u16 offset;
    u16 eot_offset;
    control = (void *)header + sizeof(*header);
    end = (void *)control + control.hdr.length;
    eot_offset = (void *)header + header.header.length - (void *)control;
    rc = ibft_verify_hdr("control", (struct ibft_hdr *)control, id_control, 0);
// iBFT table safety checking
    rc |= ((control.hdr.index) ? -ENODEV : 0);
    rc |= ((control.hdr.length < sizeof(*control)) ? -ENODEV : 0);
    if (rc) {
    printk(KERN_ERR "iBFT error: Control header is invalid!\n");
    return rc;
    }
    for (ptr = &control.initiator_off; ptr + sizeof(u16) <= end; ptr += sizeof(u16)) {
    offset = *(u16 *)ptr;
    if (offset && offset < header.header.length &&
    offset < eot_offset) {
    rc = ibft_create_kobject(header,
    (void *)header + offset);
    if (rc)
    break;
    }
    }
    if (rc)
    return rc;
    ibft_kobj = kzalloc_obj(*ibft_kobj);
    if (!ibft_kobj)
    return -ENOMEM;
    ibft_kobj.header = header;
    ibft_kobj.hdr = core::ptr::null_mut(); /*for ibft_unregister*/
    boot_kobj = iscsi_boot_create_acpitbl(boot_kset, 0,
    ibft_kobj,
    ibft_attr_show_acpitbl,
    ibft_check_acpitbl_for,
    ibft_kobj_release);
    if (!boot_kobj)  {
    kfree(ibft_kobj);
    rc = -ENOMEM;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn ibft_unregister() {
    static void ibft_unregister(void)
    {
    struct iscsi_boot_kobj *boot_kobj, *tmp_kobj;
    struct ibft_kobject *ibft_kobj;
    list_for_each_entry_safe(boot_kobj, tmp_kobj,
    &boot_kset.kobj_list, list) {
    ibft_kobj = boot_kobj.data;
    if (ibft_kobj.hdr && ibft_kobj.hdr.id == id_nic)
    sysfs_remove_link(&boot_kobj.kobj, "device");
    };
    }
#[no_mangle]
unsafe extern "C" fn ibft_cleanup() {
    static void ibft_cleanup(void)
    {
    if (boot_kset) {
    ibft_unregister();
    iscsi_boot_destroy_kset(boot_kset);
    }
    }
#[no_mangle]
unsafe extern "C" fn ibft_exit() -> void __exit {
    static void __exit ibft_exit(void)
    {
    ibft_cleanup();
    }

    static const struct {
    char *sign;
    } ibft_signs[] = {
//
// One spec says "IBFT", the other says "iBFT". We have to check
// for both.
//
    { ACPI_SIG_IBFT },
    { "iBFT" },
    { "BIFT" },	/* Broadcom iSCSI Offload */
    };
#[no_mangle]
unsafe extern "C" fn acpi_find_ibft_region() -> void __init {
    static void __init acpi_find_ibft_region(void)
    {
    int i;
    struct acpi_table_header *table = core::ptr::null_mut();
    if (acpi_disabled)
    return;
    for (i = 0; i < ARRAY_SIZE(ibft_signs) && !ibft_addr; i++) {
    acpi_get_table(ibft_signs[i].sign, 0, &table);
    ibft_addr = (struct acpi_table_ibft *)table;
    }
    }

#[no_mangle]
unsafe extern "C" fn acpi_find_ibft_region() -> void __init {
    static void __init acpi_find_ibft_region(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn acpi_find_isa_region() -> int __init {
    static int __init acpi_find_isa_region(void)
    {
    if (ibft_phys_addr) {
    ibft_addr = isa_bus_to_virt(ibft_phys_addr);
    return 0;
    }
    return -ENODEV;
    }

#[no_mangle]
unsafe extern "C" fn acpi_find_isa_region() -> int __init {
    static int __init acpi_find_isa_region(void)
    {
    return -ENODEV;
    }

//
// ibft_init() - creates sysfs tree entries for the iBFT data.
//
#[no_mangle]
unsafe extern "C" fn ibft_init() -> int __init {
    static int __init ibft_init(void)
    {
    let mut rc: c_int = 0;
//
#[no_mangle]
pub unsafe extern "C" fn setup_arch(_arg: )/reserve_ibft_region() -> As on UEFI systems the {
    As on UEFI systems the setup_arch()/reserve_ibft_region()
    is called before ACPI tables are parsed and it only does
    legacy finding.
//
    if (acpi_find_isa_region())
    acpi_find_ibft_region();
    if (ibft_addr) {
    pr_info("iBFT detected.\n");
    rc = ibft_check_device();
    if (rc)
    return rc;
    boot_kset = iscsi_boot_create_kset("ibft");
    if (!boot_kset)
    return -ENOMEM;
// Scan the IBFT for data and register the kobjects.
    rc = ibft_register_kobjects(ibft_addr);
    if (rc)
    goto out_free;
    } else
    printk(KERN_INFO "No iBFT detected.\n");
    return 0;
    out_free:
    ibft_cleanup();
    return rc;
    }
    module_init(ibft_init);
    module_exit(ibft_exit);
