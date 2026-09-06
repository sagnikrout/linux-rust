//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-smbios-smm.c
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
// SMI methods for use with dell-smbios
//
// Copyright (c) Red Hat <mjg@redhat.com>
// Copyright (c) 2014 Gabriele Mazzotta <gabriele.mzt@gmail.com>
// Copyright (c) 2014 Pali Rohár <pali@kernel.org>
// Copyright (c) 2017 Dell Inc.
//

    static int da_command_address;
    static int da_command_code;
    static struct smi_buffer smi_buf;
    static struct calling_interface_buffer *buffer;
    static struct platform_device *platform_device;
    static DEFINE_MUTEX(smm_mutex);
#[no_mangle]
unsafe extern "C" fn parse_da_table(dm: *const dmi_header) {
    static void parse_da_table(const struct dmi_header *dm)
    {
    struct calling_interface_structure *table =
    container_of(dm, struct calling_interface_structure, header);
// 4 bytes of table header, plus 7 bytes of Dell header, plus at least
// 6 bytes of entry
//
    if (dm.length < 17)
    return;
    da_command_address = table.cmdIOAddress;
    da_command_code = table.cmdIOCode;
    }
#[no_mangle]
unsafe extern "C" fn find_cmd_address(dm: *const dmi_header, dummy: *mut c_void) {
    static void find_cmd_address(const struct dmi_header *dm, void *dummy)
    {
    switch (dm.type) {
    case 0xda: /* Calling interface */
    parse_da_table(dm);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn dell_smbios_smm_call(dev: *mut device, input: *mut calling_interface_buffer) -> c_int {
    static int dell_smbios_smm_call(struct device *dev, struct calling_interface_buffer *input)
    {
    struct smi_cmd command;
    size_t size;
    size = sizeof(struct calling_interface_buffer);
    command.magic = SMI_CMD_MAGIC;
    command.command_address = da_command_address;
    command.command_code = da_command_code;
    command.ebx = smi_buf.dma;
    command.ecx = 0x42534931;
    mutex_lock(&smm_mutex);
    memcpy(buffer, input, size);
    dcdbas_smi_request(&command);
    memcpy(input, buffer, size);
    mutex_unlock(&smm_mutex);
    return 0;
    }
// When enabled this indicates that SMM won't work
#[no_mangle]
unsafe extern "C" fn test_wsmt_enabled(dev: *mut device) -> bool {
    static bool test_wsmt_enabled(struct device *dev)
    {
    struct calling_interface_token *wsmt;
// if token doesn't exist, SMM will work
    wsmt = dell_smbios_find_token(WSMT_EN_TOKEN);
    if (!wsmt)
    return false;
// If token exists, try to access over SMM but set a dummy return.
// - If WSMT disabled it will be overwritten by SMM
// - If WSMT enabled then dummy value will remain
//
    buffer.cmd_class = CLASS_TOKEN_READ;
    buffer.cmd_select = SELECT_TOKEN_STD;
    memset(buffer, 0, sizeof(struct calling_interface_buffer));
    buffer.input[0] = wsmt.location;
    buffer.output[0] = 99;
    dell_smbios_smm_call(dev, buffer);
    if (buffer.output[0] == 99)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn init_dell_smbios_smm() -> c_int {
    int init_dell_smbios_smm(void)
    {
    int ret;
//
// Allocate buffer below 4GB for SMI data--only 32-bit physical addr
// is passed to SMI handler.
//
    ret = dcdbas_smi_alloc(&smi_buf, PAGE_SIZE);
    if (ret)
    return ret;
    buffer = (void *)smi_buf.virt;
    dmi_walk(find_cmd_address, core::ptr::null_mut());
    if (test_wsmt_enabled(&platform_device.dev)) {
    pr_debug("Disabling due to WSMT enabled\n");
    ret = -ENODEV;
    goto fail_wsmt;
    }
    platform_device = platform_device_alloc("dell-smbios", 1);
    if (!platform_device) {
    ret = -ENOMEM;
    goto fail_platform_device_alloc;
    }
    ret = platform_device_add(platform_device);
    if (ret)
    goto fail_platform_device_add;
    ret = dell_smbios_register_device(&platform_device.dev, 0, &dell_smbios_smm_call);
    if (ret)
    goto fail_register;
    return 0;
    fail_register:
    platform_device_del(platform_device);
    fail_platform_device_add:
    platform_device_put(platform_device);
    fail_wsmt:
    fail_platform_device_alloc:
    dcdbas_smi_free(&smi_buf);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn exit_dell_smbios_smm() {
    void exit_dell_smbios_smm(void)
    {
    if (platform_device) {
    dell_smbios_unregister_device(&platform_device.dev);
    platform_device_unregister(platform_device);
    dcdbas_smi_free(&smi_buf);
    }
    }
