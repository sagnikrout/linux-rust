//! Automatically rewritten from C to Rust
//! Source: drivers/char/tpm/eventlog/acpi.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2005 IBM Corporation
//
// Authors:
// Seiji Munetoh <munetoh@jp.ibm.com>
// Stefan Berger <stefanb@us.ibm.com>
// Reiner Sailer <sailer@watson.ibm.com>
// Kylene Hall <kjhall@us.ibm.com>
// Nayna Jain <nayna@linux.vnet.ibm.com>
//
// Maintained by: <tpmdd-devel@lists.sourceforge.net>
//
// Access to the event log extended by the TCG BIOS of PC platform
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tcpa {
    pub hdr: acpi_table_header,
    pub platform_class: u16,
    union {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_hdr {
    pub __packed: u32 log_max_len,
    pub __packed: u64 log_start_addr,
    pub client: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct server_hdr {
    pub reserved: u16,
    pub __packed: u64 log_max_len,
    pub __packed: u64 log_start_addr,
    pub server: },
}

    };
// Check that the given log is indeed a TPM2 log.
#[no_mangle]
unsafe extern "C" fn tpm_is_tpm2_log(bios_event_log: *mut c_void, len: u64) -> bool {
    static bool tpm_is_tpm2_log(void *bios_event_log, u64 len)
    {
    struct tcg_efi_specid_event_head *efispecid;
    struct tcg_pcr_event *event_header;
    int n;
    if (len < sizeof(*event_header))
    return false;
    len -= sizeof(*event_header);
    event_header = bios_event_log;
    if (len < sizeof(*efispecid))
    return false;
    efispecid = (struct tcg_efi_specid_event_head *)event_header.event;
    n = memcmp(efispecid.signature, TCG_SPECID_SIG,
    sizeof(TCG_SPECID_SIG));
    let mut n: return = = 0;
    }
#[no_mangle]
unsafe extern "C" fn tpm_bios_log_free(data: *mut c_void) {
    static void tpm_bios_log_free(void *data)
    {
    kvfree(data);
    }
// read binary bios log
#[no_mangle]
pub unsafe extern "C" fn tpm_read_log_acpi(chip: *mut tpm_chip) -> c_int {
    int tpm_read_log_acpi(struct tpm_chip *chip)
    {
    struct acpi_tcpa *buff;
    acpi_status status;
    void __iomem *virt;
    u64 len, start;
    struct tpm_bios_log *log;
    struct acpi_table_tpm2 *tbl;
    struct acpi_tpm2_phy *tpm2_phy;
    int format;
    int ret;
    log = &chip.log;
// Unfortuntely ACPI does not associate the event log with a specific
// TPM, like PPI. Thus all ACPI TPMs will read the same log.
//
    if (!chip.acpi_dev_handle)
    return -ENODEV;
    if (chip.flags & TPM_CHIP_FLAG_TPM2) {
    status = acpi_get_table("TPM2", 1,
    (struct acpi_table_header **)&tbl);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    if (tbl.header.length <
    sizeof(*tbl) + sizeof(struct acpi_tpm2_phy)) {
    acpi_put_table((struct acpi_table_header *)tbl);
    return -ENODEV;
    }
    tpm2_phy = (void *)tbl + sizeof(*tbl);
    len = tpm2_phy.log_area_minimum_length;
    start = tpm2_phy.log_area_start_address;
    if (!start || !len) {
    acpi_put_table((struct acpi_table_header *)tbl);
    return -ENODEV;
    }
    acpi_put_table((struct acpi_table_header *)tbl);
    format = EFI_TCG2_EVENT_LOG_FORMAT_TCG_2;
    } else {
// Find TCPA entry in RSDT (ACPI_LOGICAL_ADDRESSING)
    status = acpi_get_table(ACPI_SIG_TCPA, 1,
    (struct acpi_table_header **)&buff);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    switch (buff.platform_class) {
    case BIOS_SERVER:
    len = buff.server.log_max_len;
    start = buff.server.log_start_addr;
    break;
    case BIOS_CLIENT:
    default:
    len = buff.client.log_max_len;
    start = buff.client.log_start_addr;
    break;
    }
    acpi_put_table((struct acpi_table_header *)buff);
    format = EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2;
    }
    if (!len) {
    dev_warn(&chip.dev, "%s: TCPA log area empty\n", __func__);
    return -EIO;
    }
// malloc EventLog space
    log.bios_event_log = kvmalloc(len, GFP_KERNEL);
    if (!log.bios_event_log)
    return -ENOMEM;
    log.bios_event_log_end = log.bios_event_log + len;
    virt = acpi_os_map_iomem(start, len);
    if (!virt) {
    dev_warn(&chip.dev, "%s: Failed to map ACPI memory\n", __func__);
// try EFI log next
    ret = -ENODEV;
    goto err;
    }
    memcpy_fromio(log.bios_event_log, virt, len);
    acpi_os_unmap_iomem(virt, len);
    if (chip.flags & TPM_CHIP_FLAG_TPM2 &&
    !tpm_is_tpm2_log(log.bios_event_log, len)) {
// try EFI log next
    ret = -ENODEV;
    goto err;
    }
    ret = devm_add_action(&chip.dev, tpm_bios_log_free, log.bios_event_log);
    if (ret) {
    log.bios_event_log = core::ptr::null_mut();
    goto err;
    }
    return format;
    err:
    tpm_bios_log_free(log.bios_event_log);
    log.bios_event_log = core::ptr::null_mut();
    return ret;
    }
