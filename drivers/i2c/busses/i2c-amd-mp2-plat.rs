//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-amd-mp2-plat.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// AMD MP2 platform driver
//
// Setup the I2C adapters enumerated in the ACPI namespace.
// MP2 controllers have 2 separate busses, up to 2 I2C adapters may be listed.
//
// Authors: Nehal Bakulchandra Shah <Nehal-bakulchandra.shah@amd.com>
// Elie Morisse <syniurge@gmail.com>
//

//
// struct amd_i2c_dev - MP2 bus/i2c adapter context
// @common: shared context with the MP2 PCI driver
// @pdev: platform driver node
// @adap: i2c adapter
// @cmd_complete: xfer completion object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_i2c_dev {
    pub common: amd_i2c_common,
    pub pdev: *mut platform_device,
    pub adap: i2c_adapter,
    pub cmd_complete: completion,
}

    container_of(__common, struct amd_i2c_dev, common)
#[no_mangle]
unsafe extern "C" fn i2c_amd_dma_map(i2c_common: *mut amd_i2c_common) -> c_int {
    static int i2c_amd_dma_map(struct amd_i2c_common *i2c_common)
    {
    struct device *dev_pci = &i2c_common.mp2_dev.pci_dev.dev;
    struct amd_i2c_dev *i2c_dev = amd_i2c_dev_common(i2c_common);
    enum dma_data_direction dma_direction =
    i2c_common.msg.flags & I2C_M_RD ?
    DMA_FROM_DEVICE : DMA_TO_DEVICE;
    i2c_common.dma_buf = i2c_get_dma_safe_msg_buf(i2c_common.msg, 0);
    i2c_common.dma_addr = dma_map_single(dev_pci, i2c_common.dma_buf,
    i2c_common.msg.len,
    dma_direction);
    if (unlikely(dma_mapping_error(dev_pci, i2c_common.dma_addr))) {
    dev_err(&i2c_dev.pdev.dev,
    "Error while mapping dma buffer %p\n",
    i2c_common.dma_buf);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_dma_unmap(i2c_common: *mut amd_i2c_common) {
    static void i2c_amd_dma_unmap(struct amd_i2c_common *i2c_common)
    {
    struct device *dev_pci = &i2c_common.mp2_dev.pci_dev.dev;
    enum dma_data_direction dma_direction =
    i2c_common.msg.flags & I2C_M_RD ?
    DMA_FROM_DEVICE : DMA_TO_DEVICE;
    dma_unmap_single(dev_pci, i2c_common.dma_addr,
    i2c_common.msg.len, dma_direction);
    i2c_put_dma_safe_msg_buf(i2c_common.dma_buf, i2c_common.msg, true);
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_start_cmd(i2c_dev: *mut amd_i2c_dev) {
    static void i2c_amd_start_cmd(struct amd_i2c_dev *i2c_dev)
    {
    struct amd_i2c_common *i2c_common = &i2c_dev.common;
    reinit_completion(&i2c_dev.cmd_complete);
    i2c_common.cmd_success = false;
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_cmd_completion(i2c_common: *mut amd_i2c_common) {
    static void i2c_amd_cmd_completion(struct amd_i2c_common *i2c_common)
    {
    struct amd_i2c_dev *i2c_dev = amd_i2c_dev_common(i2c_common);
    union i2c_event *event = &i2c_common.eventval;
    if (event.r.status == i2c_readcomplete_event)
    dev_dbg(&i2c_dev.pdev.dev, "readdata:%*ph\n", event.r.length,
    i2c_common.msg.buf);
    complete(&i2c_dev.cmd_complete);
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_check_cmd_completion(i2c_dev: *mut amd_i2c_dev) -> c_int {
    static int i2c_amd_check_cmd_completion(struct amd_i2c_dev *i2c_dev)
    {
    struct amd_i2c_common *i2c_common = &i2c_dev.common;
    unsigned long time_left;
    time_left = wait_for_completion_timeout(&i2c_dev.cmd_complete,
    i2c_dev.adap.timeout);
    if ((i2c_common.reqcmd == i2c_read ||
    i2c_common.reqcmd == i2c_write) &&
    i2c_common.msg.len > 32)
    i2c_amd_dma_unmap(i2c_common);
    if (time_left == 0) {
    amd_mp2_rw_timeout(i2c_common);
    return -ETIMEDOUT;
    }
    amd_mp2_process_event(i2c_common);
    if (!i2c_common.cmd_success)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_enable_set(i2c_dev: *mut amd_i2c_dev, enable: bool) -> c_int {
    static int i2c_amd_enable_set(struct amd_i2c_dev *i2c_dev, bool enable)
    {
    struct amd_i2c_common *i2c_common = &i2c_dev.common;
    i2c_amd_start_cmd(i2c_dev);
    amd_mp2_bus_enable_set(i2c_common, enable);
    return i2c_amd_check_cmd_completion(i2c_dev);
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_xfer_msg(i2c_dev: *mut amd_i2c_dev, pmsg: *mut i2c_msg) -> c_int {
    static int i2c_amd_xfer_msg(struct amd_i2c_dev *i2c_dev, struct i2c_msg *pmsg)
    {
    struct amd_i2c_common *i2c_common = &i2c_dev.common;
    i2c_amd_start_cmd(i2c_dev);
    i2c_common.msg = pmsg;
    if (pmsg.len > 32)
    if (i2c_amd_dma_map(i2c_common))
    return -EIO;
    if (pmsg.flags & I2C_M_RD)
    amd_mp2_rw(i2c_common, i2c_read);
    else
    amd_mp2_rw(i2c_common, i2c_write);
    return i2c_amd_check_cmd_completion(i2c_dev);
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_xfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int i2c_amd_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs, int num)
    {
    struct amd_i2c_dev *i2c_dev = i2c_get_adapdata(adap);
    int i;
    struct i2c_msg *pmsg;
    let mut err: c_int = 0;
// the adapter might have been deleted while waiting for the bus lock
    if (unlikely(!i2c_dev.common.mp2_dev))
    return -EINVAL;
    amd_mp2_pm_runtime_get(i2c_dev.common.mp2_dev);
    for (i = 0; i < num; i++) {
    pmsg = &msgs[i];
    err = i2c_amd_xfer_msg(i2c_dev, pmsg);
    if (err)
    break;
    }
    amd_mp2_pm_runtime_put(i2c_dev.common.mp2_dev);
    return err ? err : num;
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_func(a: *mut i2c_adapter) -> u32 {
    static u32 i2c_amd_func(struct i2c_adapter *a)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm i2c_amd_algorithm = {
    .xfer = i2c_amd_xfer,
    .functionality = i2c_amd_func,
    };

#[no_mangle]
unsafe extern "C" fn i2c_amd_suspend(i2c_common: *mut amd_i2c_common) -> c_int {
    static int i2c_amd_suspend(struct amd_i2c_common *i2c_common)
    {
    struct amd_i2c_dev *i2c_dev = amd_i2c_dev_common(i2c_common);
    i2c_amd_enable_set(i2c_dev, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_resume(i2c_common: *mut amd_i2c_common) -> c_int {
    static int i2c_amd_resume(struct amd_i2c_common *i2c_common)
    {
    struct amd_i2c_dev *i2c_dev = amd_i2c_dev_common(i2c_common);
    return i2c_amd_enable_set(i2c_dev, true);
    }

    static const u32 supported_speeds[] = {
    I2C_MAX_HIGH_SPEED_MODE_FREQ,
    I2C_MAX_TURBO_MODE_FREQ,
    I2C_MAX_FAST_MODE_PLUS_FREQ,
    I2C_MAX_FAST_MODE_FREQ,
    I2C_MAX_STANDARD_MODE_FREQ,
    };
#[no_mangle]
unsafe extern "C" fn i2c_amd_get_bus_speed(pdev: *mut platform_device) -> enum speed_enum {
    static enum speed_enum i2c_amd_get_bus_speed(struct platform_device *pdev)
    {
    u32 acpi_speed;
    int i;
    acpi_speed = i2c_acpi_find_bus_speed(&pdev.dev);
// round down to the lowest standard speed
    for (i = 0; i < ARRAY_SIZE(supported_speeds); i++) {
    if (acpi_speed >= supported_speeds[i])
    break;
    }
    acpi_speed = i < ARRAY_SIZE(supported_speeds) ? supported_speeds[i] : 0;
    switch (acpi_speed) {
    case I2C_MAX_STANDARD_MODE_FREQ:
    return speed100k;
    case I2C_MAX_FAST_MODE_FREQ:
    return speed400k;
    case I2C_MAX_FAST_MODE_PLUS_FREQ:
    return speed1000k;
    case I2C_MAX_TURBO_MODE_FREQ:
    return speed1400k;
    case I2C_MAX_HIGH_SPEED_MODE_FREQ:
    return speed3400k;
    default:
    return speed400k;
    }
    }
    static const struct i2c_adapter_quirks amd_i2c_dev_quirks = {
    .max_read_len = AMD_MP2_I2C_MAX_RW_LENGTH,
    .max_write_len = AMD_MP2_I2C_MAX_RW_LENGTH,
    };
#[no_mangle]
unsafe extern "C" fn i2c_amd_probe(pdev: *mut platform_device) -> c_int {
    static int i2c_amd_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int ret;
    struct amd_i2c_dev *i2c_dev;
    struct amd_mp2_dev *mp2_dev;
    u64 uid;
    ret = acpi_dev_uid_to_integer(ACPI_COMPANION(dev), &uid);
    if (ret)
    return dev_err_probe(dev, ret, "missing UID/bus id!\n");
    if (uid >= 2)
    return dev_err_probe(dev, -EINVAL, "incorrect UID/bus id \"%llu\"!\n", uid);
    dev_dbg(dev, "bus id is %llu\n", uid);
// The ACPI namespace doesn't contain information about which MP2 PCI
// device an AMDI0011 ACPI device is related to, so assume that there's
// only one MP2 PCI device per system.
//
    mp2_dev = amd_mp2_find_device();
    if (!mp2_dev || !mp2_dev.probed)
// The MP2 PCI device should get probed later
    return -EPROBE_DEFER;
    i2c_dev = devm_kzalloc(&pdev.dev, sizeof(*i2c_dev), GFP_KERNEL);
    if (!i2c_dev)
    return -ENOMEM;
    i2c_dev.common.bus_id = uid;
    i2c_dev.common.mp2_dev = mp2_dev;
    i2c_dev.pdev = pdev;
    platform_set_drvdata(pdev, i2c_dev);
    i2c_dev.common.cmd_completion = &i2c_amd_cmd_completion;

    i2c_dev.common.suspend = &i2c_amd_suspend;
    i2c_dev.common.resume = &i2c_amd_resume;

// Register the adapter
    amd_mp2_pm_runtime_get(mp2_dev);
    i2c_dev.common.reqcmd = i2c_none;
    if (amd_mp2_register_cb(&i2c_dev.common))
    return -EINVAL;
    device_link_add(&i2c_dev.pdev.dev, &mp2_dev.pci_dev.dev,
    DL_FLAG_AUTOREMOVE_CONSUMER);
    i2c_dev.common.i2c_speed = i2c_amd_get_bus_speed(pdev);
// Setup i2c adapter description
    i2c_dev.adap.owner = THIS_MODULE;
    i2c_dev.adap.algo = &i2c_amd_algorithm;
    i2c_dev.adap.quirks = &amd_i2c_dev_quirks;
    i2c_dev.adap.dev.parent = &pdev.dev;
    i2c_dev.adap.algo_data = i2c_dev;
    i2c_dev.adap.timeout = AMD_I2C_TIMEOUT;
    ACPI_COMPANION_SET(&i2c_dev.adap.dev, ACPI_COMPANION(&pdev.dev));
    i2c_dev.adap.dev.of_node = pdev.dev.of_node;
    snprintf(i2c_dev.adap.name, sizeof(i2c_dev.adap.name),
    "AMD MP2 i2c bus %u", i2c_dev.common.bus_id);
    i2c_set_adapdata(&i2c_dev.adap, i2c_dev);
    init_completion(&i2c_dev.cmd_complete);
// Enable the bus
    if (i2c_amd_enable_set(i2c_dev, true))
    dev_err(&pdev.dev, "initial bus enable failed\n");
// Attach to the i2c layer
    ret = i2c_add_adapter(&i2c_dev.adap);
    amd_mp2_pm_runtime_put(mp2_dev);
    if (ret < 0) {
    dev_err(&pdev.dev, "i2c add adapter failed = %d\n", ret);
    amd_mp2_unregister_cb(&i2c_dev.common);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i2c_amd_remove(pdev: *mut platform_device) {
    static void i2c_amd_remove(struct platform_device *pdev)
    {
    struct amd_i2c_dev *i2c_dev = platform_get_drvdata(pdev);
    struct amd_i2c_common *i2c_common = &i2c_dev.common;
    i2c_lock_bus(&i2c_dev.adap, I2C_LOCK_ROOT_ADAPTER);
    i2c_amd_enable_set(i2c_dev, false);
    amd_mp2_unregister_cb(i2c_common);
    i2c_common.mp2_dev = core::ptr::null_mut();
    i2c_unlock_bus(&i2c_dev.adap, I2C_LOCK_ROOT_ADAPTER);
    i2c_del_adapter(&i2c_dev.adap);
    }
    static const struct acpi_device_id i2c_amd_acpi_match[] = {
    { "AMDI0011" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, i2c_amd_acpi_match);
    static struct platform_driver i2c_amd_plat_driver = {
    .probe = i2c_amd_probe,
    .remove = i2c_amd_remove,
    .driver = {
    .name = "i2c_amd_mp2",
    .acpi_match_table = ACPI_PTR(i2c_amd_acpi_match),
    },
    };
    module_platform_driver(i2c_amd_plat_driver);
    MODULE_DESCRIPTION("AMD(R) MP2 I2C Platform Driver");
    MODULE_AUTHOR("Nehal Shah <nehal-bakulchandra.shah@amd.com>");
    MODULE_AUTHOR("Elie Morisse <syniurge@gmail.com>");
    MODULE_LICENSE("Dual BSD/GPL");
