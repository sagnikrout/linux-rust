//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/apei/ghes-nvidia.c
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
// NVIDIA GHES vendor record handler
//
// Copyright (c) 2026, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
//

    static const guid_t nvidia_sec_guid =
    GUID_INIT(0x6d5244f2, 0x2712, 0x11ec,
    0xbe, 0xa7, 0xcb, 0x3f, 0xdb, 0x95, 0xc7, 0x86);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_nvidia {
    pub signature: [c_char; 16],
    pub error_type: __le16,
    pub error_instance: __le16,
    pub severity: u8,
    pub socket: u8,
    pub number_regs: u8,
    pub reserved: u8,
    pub instance_base: __le64,
    struct {
    pub addr: __le64,
    pub val: __le64,
    pub __counted_by(number_regs): } regs[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvidia_ghes_private {
    pub nb: notifier_block,
    pub dev: *mut device,
}

    static void nvidia_ghes_print_error(struct device *dev,
    const struct cper_sec_nvidia *nvidia_err,
    size_t error_data_length, bool fatal)
    {
    const char *level = fatal ? KERN_ERR : KERN_INFO;
    size_t min_size;
    dev_printk(level, dev, "signature: %.16s\n", nvidia_err.signature);
    dev_printk(level, dev, "error_type: %u\n", le16_to_cpu(nvidia_err.error_type));
    dev_printk(level, dev, "error_instance: %u\n", le16_to_cpu(nvidia_err.error_instance));
    dev_printk(level, dev, "severity: %u\n", nvidia_err.severity);
    dev_printk(level, dev, "socket: %u\n", nvidia_err.socket);
    dev_printk(level, dev, "number_regs: %u\n", nvidia_err.number_regs);
    dev_printk(level, dev, "instance_base: 0x%016llx\n",
    le64_to_cpu(nvidia_err.instance_base));
    if (nvidia_err.number_regs == 0)
    return;
//
// Validate that all registers fit within error_data_length.
// Each register pair is two little-endian u64s.
//
    min_size = struct_size(nvidia_err, regs, nvidia_err.number_regs);
    if (error_data_length < min_size) {
    dev_err(dev, "Invalid number_regs %u (section size %zu, need %zu)\n",
    nvidia_err.number_regs, error_data_length, min_size);
    return;
    }
    for (int i = 0; i < nvidia_err.number_regs; i++)
    dev_printk(level, dev, "register[%d]: address=0x%016llx value=0x%016llx\n",
    i, le64_to_cpu(nvidia_err.regs[i].addr),
    le64_to_cpu(nvidia_err.regs[i].val));
    }
    static int nvidia_ghes_notify(struct notifier_block *nb,
    unsigned long event, void *data)
    {
    struct acpi_hest_generic_data *gdata = data;
    struct nvidia_ghes_private *priv;
    const struct cper_sec_nvidia *nvidia_err;
    guid_t sec_guid;
    import_guid(&sec_guid, gdata.section_type);
    if (!guid_equal(&sec_guid, &nvidia_sec_guid))
    return NOTIFY_DONE;
    priv = container_of(nb, struct nvidia_ghes_private, nb);
    if (acpi_hest_get_error_length(gdata) < sizeof(*nvidia_err)) {
    dev_err(priv.dev, "Section too small (%d < %zu)\n",
    acpi_hest_get_error_length(gdata), sizeof(*nvidia_err));
    return NOTIFY_OK;
    }
    nvidia_err = acpi_hest_get_payload(gdata);
    if (event >= GHES_SEV_RECOVERABLE)
    dev_err(priv.dev, "NVIDIA CPER section, error_data_length: %u\n",
    acpi_hest_get_error_length(gdata));
    else
    dev_info(priv.dev, "NVIDIA CPER section, error_data_length: %u\n",
    acpi_hest_get_error_length(gdata));
    nvidia_ghes_print_error(priv.dev, nvidia_err, acpi_hest_get_error_length(gdata),
    event >= GHES_SEV_RECOVERABLE);
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn nvidia_ghes_probe(pdev: *mut platform_device) -> c_int {
    static int nvidia_ghes_probe(struct platform_device *pdev)
    {
    struct nvidia_ghes_private *priv;
    int ret;
    priv = devm_kmalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// priv = (struct nvidia_ghes_private) {
    .nb.notifier_call = nvidia_ghes_notify,
    .dev = &pdev.dev,
    };
    ret = devm_ghes_register_vendor_record_notifier(&pdev.dev, &priv.nb);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to register NVIDIA GHES vendor record notifier\n");
    return 0;
    }
    static const struct acpi_device_id nvidia_ghes_acpi_match[] = {
    { "NVDA2012" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, nvidia_ghes_acpi_match);
    static struct platform_driver nvidia_ghes_driver = {
    .driver = {
    .name = "nvidia-ghes",
    .acpi_match_table = nvidia_ghes_acpi_match,
    },
    .probe = nvidia_ghes_probe,
    };
    module_platform_driver(nvidia_ghes_driver);
    MODULE_AUTHOR("Kai-Heng Feng <kaihengf@nvidia.com>");
    MODULE_DESCRIPTION("NVIDIA GHES vendor CPER record handler");
    MODULE_LICENSE("GPL");
