//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/efibc.c
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
// efibc: control EFI bootloaders which obey LoaderEntryOneShot var
// Copyright (c) 2013-2016, Intel Corporation.
//

pub const MAX_DATA_LEN: c_int = 512;
    static int efibc_set_variable(efi_char16_t *name, efi_char16_t *value,
    unsigned long len)
    {
    efi_status_t status;
    status = efi.set_variable(name, &LINUX_EFI_LOADER_ENTRY_GUID,
    EFI_VARIABLE_NON_VOLATILE
    | EFI_VARIABLE_BOOTSERVICE_ACCESS
    | EFI_VARIABLE_RUNTIME_ACCESS,
    len * sizeof(efi_char16_t), value);
    if (status != EFI_SUCCESS) {
    pr_err("failed to set EFI variable: 0x%lx\n", status);
    return -EIO;
    }
    return 0;
    }
    static int efibc_reboot_notifier_call(struct notifier_block *notifier,
    unsigned long event, void *data)
    {
    efi_char16_t *reason = event == SYS_RESTART ? L"reboot"
    : L"shutdown";
    const u8 *str = data;
    efi_char16_t *wdata;
    unsigned long l;
    int ret;
    ret = efibc_set_variable(L"LoaderEntryRebootReason", reason,
    ucs2_strlen(reason));
    if (ret || !data)
    return NOTIFY_DONE;
    wdata = kmalloc_objs(efi_char16_t, MAX_DATA_LEN);
    if (!wdata)
    return NOTIFY_DONE;
    for (l = 0; l < MAX_DATA_LEN - 1 && str[l] != '\0'; l++)
    wdata[l] = str[l];
    wdata[l] = L'\0';
    efibc_set_variable(L"LoaderEntryOneShot", wdata, l);
    kfree(wdata);
    return NOTIFY_DONE;
    }
    static struct notifier_block efibc_reboot_notifier = {
    .notifier_call = efibc_reboot_notifier_call,
    };
#[no_mangle]
unsafe extern "C" fn efibc_init() -> int __init {
    static int __init efibc_init(void)
    {
    int ret;
    if (!efi_rt_services_supported(EFI_RT_SUPPORTED_SET_VARIABLE))
    return -ENODEV;
    ret = register_reboot_notifier(&efibc_reboot_notifier);
    if (ret)
    pr_err("unable to register reboot notifier\n");
    return ret;
    }
    module_init(efibc_init);
#[no_mangle]
unsafe extern "C" fn efibc_exit() -> void __exit {
    static void __exit efibc_exit(void)
    {
    unregister_reboot_notifier(&efibc_reboot_notifier);
    }
    module_exit(efibc_exit);
    MODULE_AUTHOR("Jeremy Compostella <jeremy.compostella@intel.com>");
    MODULE_AUTHOR("Matt Gumbel <matthew.k.gumbel@intel.com");
    MODULE_DESCRIPTION("EFI Bootloader Control");
    MODULE_LICENSE("GPL v2");
