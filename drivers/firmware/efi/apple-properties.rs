//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/apple-properties.c
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
// apple-properties.c - EFI device properties on Macs
// Copyright (C) 2016 Lukas Wunner <lukas@wunner.de>
//
// Properties are stored either as:
// u8 arrays which can be retrieved with device_property_read_u8_array() or
// booleans which can be queried with device_property_present().
//

    static bool dump_properties __initdata;
#[no_mangle]
unsafe extern "C" fn dump_properties_enable(arg: *mut c_char) -> int __init {
    static int __init dump_properties_enable(char *arg)
    {
    dump_properties = true;
    return 1;
    }
    __setup("dump_apple_properties", dump_properties_enable);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_header {
    pub len: u32,
    pub prop_count: u32,
    pub path: [efi_dev_path; ],
//
// followed by key/value pairs, each key and value preceded by u32 len,
// len includes itself, value may be empty (in which case its len is 4)
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct properties_header {
    pub len: u32,
    pub version: u32,
    pub dev_count: u32,
    pub dev_header: [dev_header; ],
}

    static void __init unmarshal_key_value_pairs(struct dev_header *dev_header,
    struct device *dev, const void *ptr,
    struct property_entry entry[])
    {
    int i;
    for (i = 0; i < dev_header.prop_count; i++) {
    let mut remaining: c_int = dev_header.len - (ptr - (void *)dev_header);
    u32 key_len, val_len, entry_len;
    const u8 *entry_data;
    char *key;
    if (sizeof(key_len) > remaining)
    break;
    key_len = *(typeof(key_len) *)ptr;
    if (key_len + sizeof(val_len) > remaining ||
    key_len < sizeof(key_len) + sizeof(efi_char16_t) ||
// (efi_char16_t *)(ptr + sizeof(key_len)) == 0) {
    dev_err(dev, "invalid property name len at %#zx\n",
    ptr - (void *)dev_header);
    break;
    }
    val_len = *(typeof(val_len) *)(ptr + key_len);
    if (key_len + val_len > remaining ||
    val_len < sizeof(val_len)) {
    dev_err(dev, "invalid property val len at %#zx\n",
    ptr - (void *)dev_header + key_len);
    break;
    }
// 4 bytes to accommodate UTF-8 code points + null byte
    key = kzalloc((key_len - sizeof(key_len)) * 4 + 1, GFP_KERNEL);
    if (!key) {
    dev_err(dev, "cannot allocate property name\n");
    break;
    }
    ucs2_as_utf8(key, ptr + sizeof(key_len),
    key_len - sizeof(key_len));
    entry_data = ptr + key_len + sizeof(val_len);
    entry_len = val_len - sizeof(val_len);
    if (entry_len)
    entry[i] = PROPERTY_ENTRY_U8_ARRAY_LEN(key, entry_data,
    entry_len);
    else
    entry[i] = PROPERTY_ENTRY_BOOL(key);
    if (dump_properties) {
    dev_info(dev, "property: %s\n", key);
    print_hex_dump(KERN_INFO, pr_fmt(), DUMP_PREFIX_OFFSET,
    16, 1, entry_data, entry_len, true);
    }
    ptr += key_len + val_len;
    }
    if (i != dev_header.prop_count) {
    dev_err(dev, "got %d device properties, expected %u\n", i,
    dev_header.prop_count);
    print_hex_dump(KERN_ERR, pr_fmt(), DUMP_PREFIX_OFFSET,
    16, 1, dev_header, dev_header.len, true);
    return;
    }
    dev_info(dev, "assigning %d device properties\n", i);
    }
#[no_mangle]
unsafe extern "C" fn unmarshal_devices(properties: *mut properties_header) -> int __init {
    static int __init unmarshal_devices(struct properties_header *properties)
    {
    let mut offset: usize = offsetof(struct properties_header, dev_header[0]);
    while (offset + sizeof(struct dev_header) < properties.len) {
    struct dev_header *dev_header = (void *)properties + offset;
    struct property_entry *entry = core::ptr::null_mut();
    const struct efi_dev_path *ptr;
    struct device *dev;
    size_t len;
    int ret, i;
    if (offset + dev_header.len > properties.len ||
    dev_header.len <= sizeof(*dev_header)) {
    pr_err("invalid len in dev_header at %#zx\n", offset);
    return -EINVAL;
    }
    ptr = dev_header.path;
    len = dev_header.len - sizeof(*dev_header);
    dev = efi_get_device_by_path(&ptr, &len);
    if (IS_ERR(dev)) {
    pr_err("device path parse error %ld at %#zx:\n",
    PTR_ERR(dev), (void *)ptr - (void *)dev_header);
    print_hex_dump(KERN_ERR, pr_fmt(), DUMP_PREFIX_OFFSET,
    16, 1, dev_header, dev_header.len, true);
    dev = core::ptr::null_mut();
    goto skip_device;
    }
    entry = kzalloc_objs(*entry, dev_header.prop_count + 1);
    if (!entry) {
    dev_err(dev, "cannot allocate properties\n");
    goto skip_device;
    }
    unmarshal_key_value_pairs(dev_header, dev, ptr, entry);
    if (!entry[0].name)
    goto skip_device;
    ret = device_create_managed_software_node(dev, entry, core::ptr::null_mut());
    if (ret)
    dev_err(dev, "error %d assigning properties\n", ret);
    for (i = 0; entry[i].name; i++)
    kfree(entry[i].name);
    skip_device:
    kfree(entry);
    put_device(dev);
    offset += dev_header.len;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map_properties() -> int __init {
    static int __init map_properties(void)
    {
    struct properties_header *properties;
    struct setup_data *data;
    u32 data_len;
    u64 pa_data;
    int ret;
    if (!x86_apple_machine)
    return 0;
    pa_data = boot_params.hdr.setup_data;
    while (pa_data) {
    data = memremap(pa_data, sizeof(*data), MEMREMAP_WB);
    if (!data) {
    pr_err("cannot map setup_data header\n");
    return -ENOMEM;
    }
    if (data.type != SETUP_APPLE_PROPERTIES) {
    pa_data = data.next;
    memunmap(data);
    continue;
    }
    data_len = data.len;
    memunmap(data);
    data = memremap(pa_data, sizeof(*data) + data_len, MEMREMAP_WB);
    if (!data) {
    pr_err("cannot map setup_data payload\n");
    return -ENOMEM;
    }
    properties = (struct properties_header *)data.data;
    if (data_len < sizeof(*properties)) {
    pr_err("truncated properties header\n");
    ret = -EINVAL;
    } else if (properties.version != 1) {
    pr_err("unsupported version:\n");
    print_hex_dump(KERN_ERR, pr_fmt(), DUMP_PREFIX_OFFSET,
    16, 1, properties, data_len, true);
    ret = -ENOTSUPP;
    } else if (properties.len != data_len) {
    pr_err("length mismatch, expected %u\n", data_len);
    print_hex_dump(KERN_ERR, pr_fmt(), DUMP_PREFIX_OFFSET,
    16, 1, properties, data_len, true);
    ret = -EINVAL;
    } else
    ret = unmarshal_devices(properties);
//
// Can only free the setup_data payload but not its header
// to avoid breaking the chain of ->next pointers.
//
    data.len = 0;
    memunmap(data);
    memblock_phys_free(pa_data + sizeof(*data), data_len);
    return ret;
    }
    return 0;
    }
    fs_initcall(map_properties);
