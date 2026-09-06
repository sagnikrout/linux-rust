//! Automatically rewritten from C to Rust
//! Source: drivers/virt/coco/efi_secret/efi_secret.c
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
// efi_secret module
//
// Copyright (C) 2022 IBM Corporation
// Author: Dov Murik <dovmurik@linux.ibm.com>
//
// DOC: efi_secret: Allow reading EFI confidential computing (coco) secret area
// via securityfs interface.
//
// When the module is loaded (and securityfs is mounted, typically under
// /sys/kernel/security), a "secrets/coco" directory is created in securityfs.
// In it, a file is created for each secret entry.  The name of each such file
// is the GUID of the secret entry, and its content is the secret data.
//

pub const EFI_SECRET_NUM_FILES: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_secret {
    pub secrets_dir: *mut dentry,
    pub secret_data: *mut void __iomem,
    pub secret_data_len: u64,
}

//
// Structure of the EFI secret area
//
// Offset   Length
// (bytes)  (bytes)  Usage
// -------  -------  -----
// 0       16  Secret table header GUID (must be 1e74f542-71dd-4d66-963e-ef4287ff173b)
// 16        4  Length of bytes of the entire secret area
//
// 20       16  First secret entry's GUID
// 36        4  First secret entry's length in bytes (= 16 + 4 + x)
// 40        x  First secret entry's data
//
// 40+x       16  Second secret entry's GUID
// 56+x        4  Second secret entry's length in bytes (= 16 + 4 + y)
// 60+x        y  Second secret entry's data
//
// (... and so on for additional entries)
//
// The GUID of each secret entry designates the usage of the secret data.
//
// struct secret_header - Header of entire secret area; this should be followed
// by instances of struct secret_entry.
// @guid:	Must be EFI_SECRET_TABLE_HEADER_GUID
// @len:	Length in bytes of entire secret area, including header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct secret_header {
    pub guid: efi_guid_t,
    pub len: u32,
    pub __attribute((packed)): },
//
// struct secret_entry - Holds one secret entry
// @guid:	Secret-specific GUID (or NULL_GUID if this secret entry was deleted)
// @len:	Length of secret entry, including its guid and len fields
// @data:	The secret data (full of zeros if this secret entry was deleted)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct secret_entry {
    pub guid: efi_guid_t,
    pub len: u32,
    pub data: [u8; ],
    pub __attribute((packed)): },
#[no_mangle]
unsafe extern "C" fn secret_entry_data_len(e: *mut secret_entry) -> usize {
    static size_t secret_entry_data_len(struct secret_entry *e)
    {
    pub sizeof(*e): *mut return e->len -,
    }
    pub the_efi_secret: static struct efi_secret,
    static inline struct efi_secret *efi_secret_get(void)
    {
    pub &the_efi_secret: return,
    }
#[no_mangle]
unsafe extern "C" fn efi_secret_bin_file_show(file: *mut seq_file, data: *mut c_void) -> c_int {
    static int efi_secret_bin_file_show(struct seq_file *file, void *data)
    {
    pub file->private: *mut *mut secret_entry e =,
    if (e)
    pub secret_entry_data_len(e)): seq_write(file, e->data,,
    pub 0: return,
    }
//
// Overwrite memory content with zeroes, and ensure that dirty cache lines are
// actually written back to memory, to clear out the secret.
//
#[no_mangle]
unsafe extern "C" fn wipe_memory(addr: *mut c_void, size: usize) {
    static void wipe_memory(void *addr, size_t size)
    {
    pub size): memzero_explicit(addr,,

    pub size): clflush_cache_range(addr,,

    }
#[no_mangle]
unsafe extern "C" fn efi_secret_unlink(dir: *mut inode, dentry: *mut dentry) -> c_int {
    static int efi_secret_unlink(struct inode *dir, struct dentry *dentry)
    {
    pub d_inode(dentry): *mut *mut inode inode =,
    pub )inode->i_private: *mut *mut secret_entry e = (secret_entry,
    if (e) {
// Zero out the secret data
    pub secret_entry_data_len(e)): wipe_memory(e->data,,
    pub NULL_GUID: e->guid =,
    }
    pub NULL: inode->i_private =,
    pub dentry): return simple_unlink(inode,,
    }
    static const struct inode_operations efi_secret_dir_inode_operations = {
    .lookup         = simple_lookup,
    .unlink         = efi_secret_unlink,
}

#[no_mangle]
unsafe extern "C" fn efi_secret_map_area(dev: *mut platform_device) -> c_int {
    static int efi_secret_map_area(struct platform_device *dev)
    {
    int ret;
    struct efi_secret *s = efi_secret_get();
    struct linux_efi_coco_secret_area *secret_area;
    if (efi.coco_secret == EFI_INVALID_TABLE_ADDR) {
    dev_err(&dev.dev, "Secret area address is not available\n");
    return -EINVAL;
    }
    secret_area = memremap(efi.coco_secret, sizeof(*secret_area), MEMREMAP_WB);
    if (secret_area == core::ptr::null_mut()) {
    dev_err(&dev.dev, "Could not map secret area EFI config entry\n");
    return -ENOMEM;
    }
    if (!secret_area.base_pa || secret_area.size < sizeof(struct secret_header)) {
    dev_err(&dev.dev,
    "Invalid secret area memory location (base_pa=0x%llx size=0x%llx)\n",
    secret_area.base_pa, secret_area.size);
    ret = -EINVAL;
    goto unmap;
    }
    s.secret_data = ioremap_encrypted(secret_area.base_pa, secret_area.size);
    if (s.secret_data == core::ptr::null_mut()) {
    dev_err(&dev.dev, "Could not map secret area\n");
    ret = -ENOMEM;
    goto unmap;
    }
    s.secret_data_len = secret_area.size;
    ret = 0;
    unmap:
    memunmap(secret_area);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn efi_secret_securityfs_teardown(dev: *mut platform_device) {
    static void efi_secret_securityfs_teardown(struct platform_device *dev)
    {
    struct efi_secret *s = efi_secret_get();
    securityfs_remove(s.secrets_dir);
    s.secrets_dir = core::ptr::null_mut();
    dev_dbg(&dev.dev, "Removed securityfs entries\n");
    }
#[no_mangle]
unsafe extern "C" fn efi_secret_securityfs_setup(dev: *mut platform_device) -> c_int {
    static int efi_secret_securityfs_setup(struct platform_device *dev)
    {
    struct efi_secret *s = efi_secret_get();
    let mut ret: c_int = 0, i = 0, bytes_left;
    unsigned char *ptr;
    struct secret_header *h;
    struct secret_entry *e;
    struct dentry *dent, *dir;
    char guid_str[EFI_VARIABLE_GUID_LEN + 1];
    ptr = (void  *)s.secret_data;
    h = (struct secret_header *)ptr;
    if (efi_guidcmp(h.guid, EFI_SECRET_TABLE_HEADER_GUID)) {
//
// This is not an error: it just means that EFI defines secret
// area but it was not populated by the Guest Owner.
//
    dev_dbg(&dev.dev, "EFI secret area does not start with correct GUID\n");
    return -ENODEV;
    }
    if (h.len < sizeof(*h)) {
    dev_err(&dev.dev, "EFI secret area reported length is too small\n");
    return -EINVAL;
    }
    if (h.len > s.secret_data_len) {
    dev_err(&dev.dev, "EFI secret area reported length is too big\n");
    return -EINVAL;
    }
    s.secrets_dir = core::ptr::null_mut();
    dent = securityfs_create_dir("secrets", core::ptr::null_mut());
    if (IS_ERR(dent)) {
    dev_err(&dev.dev, "Error creating secrets securityfs directory entry err=%ld\n",
    PTR_ERR(dent));
    return PTR_ERR(dent);
    }
    s.secrets_dir = dent;
    dir = securityfs_create_dir("coco", s.secrets_dir);
    if (IS_ERR(dir)) {
    dev_err(&dev.dev, "Error creating coco securityfs directory entry err=%ld\n",
    PTR_ERR(dir));
    return PTR_ERR(dir);
    }
    d_inode(dir).i_op = &efi_secret_dir_inode_operations;
    bytes_left = h.len - sizeof(*h);
    ptr += sizeof(*h);
    while (bytes_left >= (int)sizeof(*e) && i < EFI_SECRET_NUM_FILES) {
    e = (struct secret_entry *)ptr;
    if (e.len < sizeof(*e) || e.len > (unsigned int)bytes_left) {
    dev_err(&dev.dev, "EFI secret area is corrupted\n");
    ret = -EINVAL;
    goto err_cleanup;
    }
// Skip deleted entries (which will have NULL_GUID)
    if (efi_guidcmp(e.guid, NULL_GUID)) {
    efi_guid_to_str(&e.guid, guid_str);
    dent = securityfs_create_file(guid_str, 0440, dir, (void *)e,
    &efi_secret_bin_file_fops);
    if (IS_ERR(dent)) {
    dev_err(&dev.dev, "Error creating efi_secret securityfs entry\n");
    ret = PTR_ERR(dent);
    goto err_cleanup;
    }
    i++;
    }
    ptr += e.len;
    bytes_left -= e.len;
    }
    dev_info(&dev.dev, "Created %d entries in securityfs secrets/coco\n", i);
    return 0;
    err_cleanup:
    efi_secret_securityfs_teardown(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn efi_secret_unmap_area() {
    static void efi_secret_unmap_area(void)
    {
    struct efi_secret *s = efi_secret_get();
    if (s.secret_data) {
    iounmap(s.secret_data);
    s.secret_data = core::ptr::null_mut();
    s.secret_data_len = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn efi_secret_probe(dev: *mut platform_device) -> c_int {
    static int efi_secret_probe(struct platform_device *dev)
    {
    int ret;
    ret = efi_secret_map_area(dev);
    if (ret)
    return ret;
    ret = efi_secret_securityfs_setup(dev);
    if (ret)
    goto err_unmap;
    return ret;
    err_unmap:
    efi_secret_unmap_area();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn efi_secret_remove(dev: *mut platform_device) {
    static void efi_secret_remove(struct platform_device *dev)
    {
    efi_secret_securityfs_teardown(dev);
    efi_secret_unmap_area();
    }
    static struct platform_driver efi_secret_driver = {
    .probe = efi_secret_probe,
    .remove = efi_secret_remove,
    .driver = {
    .name = "efi_secret",
    },
    };
    module_platform_driver(efi_secret_driver);
    MODULE_DESCRIPTION("Confidential computing EFI secret area access");
    MODULE_AUTHOR("IBM");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:efi_secret");
