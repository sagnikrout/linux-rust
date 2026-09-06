//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pci-label.c
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
// Export the firmware instance and label associated with a PCI device to
// sysfs
//
// Copyright (C) 2010 Dell Inc.
// by Narendra K <Narendra_K@dell.com>,
// Jordan Hargrave <Jordan_Hargrave@dell.com>
//
// PCI Firmware Specification Revision 3.1 section 4.6.7 (DSM for Naming a
// PCI or PCI Express Device Under Operating Systems) defines an instance
// number and string name. This code retrieves them and exports them to sysfs.
// If the system firmware does not provide the ACPI _DSM (Device Specific
// Method), then the SMBIOS type 41 instance number and string is exported to
// sysfs.
//
// SMBIOS defines type 41 for onboard pci devices. This code retrieves
// the instance number and string from the type 41 record and exports
// it to sysfs.
//
// Please see https://linux.dell.com/files/biosdevname/ for more
// information.
//

#[no_mangle]
unsafe extern "C" fn device_has_acpi_name(dev: *mut device) -> bool {
    static bool device_has_acpi_name(struct device *dev)
    {

    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    if (!handle)
    return false;
    return acpi_check_dsm(handle, &pci_acpi_dsm_guid, 0x2,
    1 << DSM_PCI_DEVICE_NAME);

    return false;

    }

    enum smbios_attr_enum {
    SMBIOS_ATTR_NONE = 0,
    SMBIOS_ATTR_LABEL_SHOW,
    SMBIOS_ATTR_INSTANCE_SHOW,
    };
    static size_t find_smbios_instance_string(struct pci_dev *pdev, char *buf,
    enum smbios_attr_enum attribute)
    {
    const struct dmi_device *dmi;
    struct dmi_dev_onboard *donboard;
    let mut domain_nr: c_int = pci_domain_nr(pdev.bus);
    let mut bus: c_int = pdev.bus.number;
    let mut devfn: c_int = pdev.devfn;
    dmi = core::ptr::null_mut();
    while ((dmi = dmi_find_device(DMI_DEV_TYPE_DEV_ONBOARD,
    core::ptr::null_mut(), dmi)) != core::ptr::null_mut()) {
    donboard = dmi.device_data;
    if (donboard && donboard.segment == domain_nr &&
    donboard.bus == bus &&
    donboard.devfn == devfn) {
    if (buf) {
    if (attribute == SMBIOS_ATTR_INSTANCE_SHOW)
    return sysfs_emit(buf, "%d\n",
    donboard.instance);
#[no_mangle]
pub unsafe extern "C" fn if(SMBIOS_ATTR_LABEL_SHOW: attribute ==) -> else {
    else if (attribute == SMBIOS_ATTR_LABEL_SHOW)
    return sysfs_emit(buf, "%s\n",
    dmi.name);
    }
    return strlen(dmi.name);
    }
    }
    return 0;
    }
    static ssize_t smbios_label_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    return find_smbios_instance_string(pdev, buf,
    SMBIOS_ATTR_LABEL_SHOW);
    }
    static struct device_attribute dev_attr_smbios_label = __ATTR(label, 0444,
    smbios_label_show, core::ptr::null_mut());
    static ssize_t index_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    return find_smbios_instance_string(pdev, buf,
    SMBIOS_ATTR_INSTANCE_SHOW);
    }
    static DEVICE_ATTR_RO(index);
    static struct attribute *smbios_attrs[] = {
    &dev_attr_smbios_label.attr,
    &dev_attr_index.attr,
    core::ptr::null_mut(),
    };
    static umode_t smbios_attr_is_visible(struct kobject *kobj, struct attribute *a,
    int n)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct pci_dev *pdev = to_pci_dev(dev);
    if (device_has_acpi_name(dev))
    return 0;
    if (!find_smbios_instance_string(pdev, core::ptr::null_mut(), SMBIOS_ATTR_NONE))
    return 0;
    return a.mode;
    }
    const struct attribute_group pci_dev_smbios_attr_group = {
    .attrs = smbios_attrs,
    .is_visible = smbios_attr_is_visible,
    };

    enum acpi_attr_enum {
    ACPI_ATTR_LABEL_SHOW,
    ACPI_ATTR_INDEX_SHOW,
    };
#[no_mangle]
unsafe extern "C" fn dsm_label_utf16s_to_utf8s(obj: *mut union acpi_object, buf: *mut c_char) -> c_int {
    static int dsm_label_utf16s_to_utf8s(union acpi_object *obj, char *buf)
    {
    int len;
    len = utf16s_to_utf8s((const wchar_t *)obj.buffer.pointer,
    obj.buffer.length,
    UTF16_LITTLE_ENDIAN,
    buf, PAGE_SIZE - 1);
    buf[len++] = '\n';
    return len;
    }
    static int dsm_get_label(struct device *dev, char *buf,
    enum acpi_attr_enum attr)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    union acpi_object *obj, *tmp;
    let mut len: c_int = 0;
    if (!handle)
    return -1;
    obj = acpi_evaluate_dsm(handle, &pci_acpi_dsm_guid, 0x2,
    DSM_PCI_DEVICE_NAME, core::ptr::null_mut());
    if (!obj)
    return -1;
    tmp = obj.package.elements;
    if (obj.type == ACPI_TYPE_PACKAGE && obj.package.count == 2 &&
    tmp[0].type == ACPI_TYPE_INTEGER &&
    (tmp[1].type == ACPI_TYPE_STRING ||
    tmp[1].type == ACPI_TYPE_BUFFER)) {
//
// The second string element is optional even when
// this _DSM is implemented; when not implemented,
// this entry must return a null string.
//
    if (attr == ACPI_ATTR_INDEX_SHOW) {
    len = sysfs_emit(buf, "%llu\n", tmp.integer.value);
    } else if (attr == ACPI_ATTR_LABEL_SHOW) {
    if (tmp[1].type == ACPI_TYPE_STRING)
    len = sysfs_emit(buf, "%s\n",
    tmp[1].string.pointer);
#[no_mangle]
pub unsafe extern "C" fn if(ACPI_TYPE_BUFFER: tmp[1].type ==) -> else {
    else if (tmp[1].type == ACPI_TYPE_BUFFER)
    len = dsm_label_utf16s_to_utf8s(tmp + 1, buf);
    }
    }
    ACPI_FREE(obj);
    return len > 0 ? len : -1;
    }
    static ssize_t label_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return dsm_get_label(dev, buf, ACPI_ATTR_LABEL_SHOW);
    }
    static DEVICE_ATTR_RO(label);
    static ssize_t acpi_index_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return dsm_get_label(dev, buf, ACPI_ATTR_INDEX_SHOW);
    }
    static DEVICE_ATTR_RO(acpi_index);
    static struct attribute *acpi_attrs[] = {
    &dev_attr_label.attr,
    &dev_attr_acpi_index.attr,
    core::ptr::null_mut(),
    };
    static umode_t acpi_attr_is_visible(struct kobject *kobj, struct attribute *a,
    int n)
    {
    struct device *dev = kobj_to_dev(kobj);
    if (!device_has_acpi_name(dev))
    return 0;
    return a.mode;
    }
    const struct attribute_group pci_dev_acpi_attr_group = {
    .attrs = acpi_attrs,
    .is_visible = acpi_attr_is_visible,
    };
