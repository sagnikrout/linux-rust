//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/amd/pmc/mp1_stb.c
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
// AMD MP1 Smart Trace Buffer (STB) Layer
//
// Copyright (c) 2024, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Authors: Shyam Sundar S K <Shyam-sundar.S-k@amd.com>
// Sanket Goswami <Sanket.Goswami@amd.com>
//

// STB Spill to DRAM Parameters
pub const S2D_TELEMETRY_DRAMBYTES_MAX: c_uint = 0x1000000;
pub const S2D_TELEMETRY_BYTES_MAX: c_uint = 0x100000U;
pub const S2D_RSVD_RAM_SPACE: c_uint = 0x100000;
// STB Registers
pub const AMD_STB_PMI_0: c_uint = 0x03E30600;
pub const AMD_PMC_STB_DUMMY_PC: c_uint = 0xC6000007;
// STB Spill to DRAM Message Definition
pub const STB_FORCE_FLUSH_DATA: c_uint = 0xCF;
pub const FIFO_SIZE: c_int = 4096;
// STB S2D(Spill to DRAM) has different message port offset
pub const AMD_S2D_REGISTER_MESSAGE: c_uint = 0xA20;
pub const AMD_S2D_REGISTER_RESPONSE: c_uint = 0xA80;
pub const AMD_S2D_REGISTER_ARGUMENT: c_uint = 0xA88;
// STB S2D (Spill to DRAM) message port offset for 44h model
pub const AMD_GNR_REGISTER_MESSAGE: c_uint = 0x524;
pub const AMD_GNR_REGISTER_RESPONSE: c_uint = 0x570;
pub const AMD_GNR_REGISTER_ARGUMENT: c_uint = 0xA40;
    static bool enable_stb;
    module_param(enable_stb, bool, 0644);
    MODULE_PARM_DESC(enable_stb, "Enable the STB debug mechanism");
    static bool dump_custom_stb;
    module_param(dump_custom_stb, bool, 0644);
    MODULE_PARM_DESC(dump_custom_stb, "Enable to dump full STB buffer");
    enum s2d_arg {
    S2D_TELEMETRY_SIZE = 0x01,
    S2D_PHYS_ADDR_LOW,
    S2D_PHYS_ADDR_HIGH,
    S2D_NUM_SAMPLES,
    S2D_DRAM_SIZE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_stb_v2_data {
    pub size: usize,
    pub __counted_by(size): u8 data[],
}

#[no_mangle]
pub unsafe extern "C" fn amd_stb_write(dev: *mut amd_pmc_dev, data: u32) -> c_int {
    int amd_stb_write(struct amd_pmc_dev *dev, u32 data)
    {
    int err;
    err = amd_smn_write(0, AMD_STB_PMI_0, data);
    if (err) {
    dev_err(dev.dev, "failed to write data in stb: 0x%X\n", AMD_STB_PMI_0);
    return pcibios_err_to_errno(err);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn amd_stb_read(dev: *mut amd_pmc_dev, buf: *mut u32) -> c_int {
    int amd_stb_read(struct amd_pmc_dev *dev, u32 *buf)
    {
    int i, err;
    for (i = 0; i < FIFO_SIZE; i++) {
    err = amd_smn_read(0, AMD_STB_PMI_0, buf++);
    if (err) {
    dev_err(dev.dev, "error reading data from stb: 0x%X\n", AMD_STB_PMI_0);
    return pcibios_err_to_errno(err);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_stb_debugfs_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int amd_stb_debugfs_open(struct inode *inode, struct file *filp)
    {
    struct amd_pmc_dev *dev = filp.f_inode.i_private;
    let mut size: u32 = FIFO_SIZE * sizeof(u32);
    u32 *buf;
    int rc;
    buf = kzalloc(size, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    rc = amd_stb_read(dev, buf);
    if (rc) {
    kfree(buf);
    return rc;
    }
    filp.private_data = buf;
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn amd_stb_debugfs_read(filp: *mut file, buf: *mut char __user, size: usize, pos: *mut loff_t) -> isize {
    static ssize_t amd_stb_debugfs_read(struct file *filp, char __user *buf, size_t size, loff_t *pos)
    {
    if (!filp.private_data)
    return -EINVAL;
    return simple_read_from_buffer(buf, size, pos, filp.private_data,
    FIFO_SIZE * sizeof(u32));
    }
#[no_mangle]
unsafe extern "C" fn amd_stb_debugfs_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int amd_stb_debugfs_release(struct inode *inode, struct file *filp)
    {
    kfree(filp.private_data);
    return 0;
    }
    static const struct file_operations amd_stb_debugfs_fops = {
    .owner = THIS_MODULE,
    .open = amd_stb_debugfs_open,
    .read = amd_stb_debugfs_read,
    .release = amd_stb_debugfs_release,
    };
// Enhanced STB Firmware Reporting Mechanism
#[no_mangle]
unsafe extern "C" fn amd_stb_handle_efr(filp: *mut file) -> c_int {
    static int amd_stb_handle_efr(struct file *filp)
    {
    struct amd_pmc_dev *dev = filp.f_inode.i_private;
    struct amd_stb_v2_data *stb_data_arr;
    u32 fsize;
    fsize = dev.dram_size - S2D_RSVD_RAM_SPACE;
    stb_data_arr = kmalloc_flex(*stb_data_arr, data, fsize);
    if (!stb_data_arr)
    return -ENOMEM;
    stb_data_arr.size = fsize;
    memcpy_fromio(stb_data_arr.data, dev.stb_virt_addr, fsize);
    filp.private_data = stb_data_arr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_stb_debugfs_open_v2(inode: *mut inode, filp: *mut file) -> c_int {
    static int amd_stb_debugfs_open_v2(struct inode *inode, struct file *filp)
    {
    struct amd_pmc_dev *dev = filp.f_inode.i_private;
    u32 fsize, num_samples, val, stb_rdptr_offset = 0;
    struct amd_stb_v2_data *stb_data_arr;
    let mut ret: c_int = 0;
// Write dummy postcode while reading the STB buffer
    ret = amd_stb_write(dev, AMD_PMC_STB_DUMMY_PC);
    if (ret)
    dev_err(dev.dev, "error writing to STB: %d\n", ret);
// Spill to DRAM num_samples uses separate SMU message port
    dev.msg_port = MSG_PORT_S2D;
    ret = amd_pmc_send_cmd(dev, 0, &val, STB_FORCE_FLUSH_DATA, 1);
    if (ret)
    dev_dbg_once(dev.dev, "S2D force flush not supported: %d\n", ret);
//
// We have a custom stb size and the PMFW is supposed to give
// the enhanced dram size. Note that we land here only for the
// platforms that support enhanced dram size reporting.
//
    if (dump_custom_stb) {
    ret = amd_stb_handle_efr(filp);
    goto out;
    }
// Get the num_samples to calculate the last push location
    ret = amd_pmc_send_cmd(dev, S2D_NUM_SAMPLES, &num_samples, dev.stb_arg.s2d_msg_id, true);
    if (ret) {
    dev_err(dev.dev, "error: S2D_NUM_SAMPLES not supported : %d\n", ret);
    goto out;
    }
    fsize = min(num_samples, S2D_TELEMETRY_BYTES_MAX);
    stb_data_arr = kmalloc_flex(*stb_data_arr, data, fsize);
    if (!stb_data_arr) {
    ret = -ENOMEM;
    goto out;
    }
    stb_data_arr.size = fsize;
//
// Start capturing data from the last push location.
// This is for general cases, where the stb limits
// are meant for standard usage.
//
    if (num_samples > S2D_TELEMETRY_BYTES_MAX) {
// First read oldest data starting 1 behind last write till end of ringbuffer
    stb_rdptr_offset = num_samples % S2D_TELEMETRY_BYTES_MAX;
    fsize = S2D_TELEMETRY_BYTES_MAX - stb_rdptr_offset;
    memcpy_fromio(stb_data_arr.data, dev.stb_virt_addr + stb_rdptr_offset, fsize);
// Second copy the newer samples from offset 0 - last write
    memcpy_fromio(stb_data_arr.data + fsize, dev.stb_virt_addr, stb_rdptr_offset);
    } else {
    memcpy_fromio(stb_data_arr.data, dev.stb_virt_addr, fsize);
    }
    filp.private_data = stb_data_arr;
    out:
// Restore the default message port for subsequent SMU operations
    dev.msg_port = MSG_PORT_PMC;
    return ret;
    }
    static ssize_t amd_stb_debugfs_read_v2(struct file *filp, char __user *buf, size_t size,
    loff_t *pos)
    {
    struct amd_stb_v2_data *data = filp.private_data;
    return simple_read_from_buffer(buf, size, pos, data.data, data.size);
    }
#[no_mangle]
unsafe extern "C" fn amd_stb_debugfs_release_v2(inode: *mut inode, filp: *mut file) -> c_int {
    static int amd_stb_debugfs_release_v2(struct inode *inode, struct file *filp)
    {
    kfree(filp.private_data);
    return 0;
    }
    static const struct file_operations amd_stb_debugfs_fops_v2 = {
    .owner = THIS_MODULE,
    .open = amd_stb_debugfs_open_v2,
    .read = amd_stb_debugfs_read_v2,
    .release = amd_stb_debugfs_release_v2,
    };
#[no_mangle]
unsafe extern "C" fn amd_stb_update_args(dev: *mut amd_pmc_dev) {
    static void amd_stb_update_args(struct amd_pmc_dev *dev)
    {
    if (cpu_feature_enabled(X86_FEATURE_ZEN5))
    switch (boot_cpu_data.x86_model) {
    case 0x44:
    dev.stb_arg.msg = AMD_GNR_REGISTER_MESSAGE;
    dev.stb_arg.arg = AMD_GNR_REGISTER_ARGUMENT;
    dev.stb_arg.resp = AMD_GNR_REGISTER_RESPONSE;
    return;
    default:
    break;
    }
    dev.stb_arg.msg = AMD_S2D_REGISTER_MESSAGE;
    dev.stb_arg.arg = AMD_S2D_REGISTER_ARGUMENT;
    dev.stb_arg.resp = AMD_S2D_REGISTER_RESPONSE;
    }
#[no_mangle]
unsafe extern "C" fn amd_is_stb_supported(dev: *mut amd_pmc_dev) -> bool {
    static bool amd_is_stb_supported(struct amd_pmc_dev *dev)
    {
    switch (dev.cpu_id) {
    case AMD_CPU_ID_YC:
    case AMD_CPU_ID_CB:
    if (boot_cpu_data.x86_model == 0x44)
    dev.stb_arg.s2d_msg_id = 0x9B;
    else
    dev.stb_arg.s2d_msg_id = 0xBE;
    break;
    case AMD_CPU_ID_PS:
    dev.stb_arg.s2d_msg_id = 0x85;
    break;
    case PCI_DEVICE_ID_AMD_1AH_M20H_ROOT:
    case PCI_DEVICE_ID_AMD_1AH_M60H_ROOT:
    if (boot_cpu_data.x86_model == 0x70)
    dev.stb_arg.s2d_msg_id = 0xF1;
    else
    dev.stb_arg.s2d_msg_id = 0xDE;
    break;
    default:
    return false;
    }
    amd_stb_update_args(dev);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn amd_stb_s2d_init(dev: *mut amd_pmc_dev) -> c_int {
    int amd_stb_s2d_init(struct amd_pmc_dev *dev)
    {
    u32 phys_addr_low, phys_addr_hi;
    u64 stb_phys_addr;
    let mut size: u32 = 0;
    let mut ret: c_int = 0;
    if (!enable_stb)
    return 0;
    if (!amd_is_stb_supported(dev)) {
    debugfs_create_file("stb_read", 0644, dev.dbgfs_dir, dev,
    &amd_stb_debugfs_fops);
    return 0;
    }
// Spill to DRAM feature uses separate SMU message port
    dev.msg_port = MSG_PORT_S2D;
    ret = amd_pmc_send_cmd(dev, S2D_TELEMETRY_SIZE, &size, dev.stb_arg.s2d_msg_id, true);
    if (ret)
    goto out;
    if (size != S2D_TELEMETRY_BYTES_MAX) {
    ret = -EIO;
    goto out;
    }
// Get DRAM size; fall back to the default if the query fails
    if (amd_pmc_send_cmd(dev, S2D_DRAM_SIZE, &dev.dram_size, dev.stb_arg.s2d_msg_id, true) ||
    !dev.dram_size)
    dev.dram_size = S2D_TELEMETRY_DRAMBYTES_MAX;
// Get STB DRAM address
    ret = amd_pmc_send_cmd(dev, S2D_PHYS_ADDR_LOW, &phys_addr_low,
    dev.stb_arg.s2d_msg_id, true);
    if (ret)
    goto out;
    ret = amd_pmc_send_cmd(dev, S2D_PHYS_ADDR_HIGH, &phys_addr_hi,
    dev.stb_arg.s2d_msg_id, true);
    if (ret)
    goto out;
    stb_phys_addr = ((u64)phys_addr_hi << 32 | phys_addr_low);
    if (!stb_phys_addr) {
    dev_err(dev.dev, "S2D phys addr query returned invalid address\n");
    ret = -ENXIO;
    goto out;
    }
    dev.stb_virt_addr = devm_ioremap(dev.dev, stb_phys_addr, dev.dram_size);
    if (!dev.stb_virt_addr) {
    ret = -ENOMEM;
    goto out;
    }
//
// Only expose stb_read once the buffer is mapped; otherwise a read
// faults on a NULL dev->stb_virt_addr, now that a failed STB init no
// longer aborts probe.
//
    debugfs_create_file("stb_read", 0644, dev.dbgfs_dir, dev,
    &amd_stb_debugfs_fops_v2);
    out:
// Restore the default message port for subsequent SMU operations
    dev.msg_port = MSG_PORT_PMC;
    return ret;
    }
