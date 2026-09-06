//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/macsmc.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple SMC (System Management Controller) MFD driver
//
// Copyright The Asahi Linux Contributors
//

pub const SMC_ENDPOINT: c_uint = 0x20;
// We don't actually know the true size here but this seem reasonable
pub const SMC_SHMEM_SIZE: c_uint = 0x1000;
pub const SMC_MAX_SIZE: c_int = 255;
pub const SMC_MSG_READ_KEY: c_uint = 0x10;
pub const SMC_MSG_WRITE_KEY: c_uint = 0x11;
pub const SMC_MSG_GET_KEY_BY_INDEX: c_uint = 0x12;
pub const SMC_MSG_GET_KEY_INFO: c_uint = 0x13;
pub const SMC_MSG_INITIALIZE: c_uint = 0x17;
pub const SMC_MSG_NOTIFICATION: c_uint = 0x18;
pub const SMC_MSG_RW_KEY: c_uint = 0x20;

pub const SMC_TIMEOUT_MS: c_int = 500;
    static const struct mfd_cell apple_smc_devs[] = {
    MFD_CELL_NAME("macsmc-input"),
    MFD_CELL_NAME("macsmc-power"),
    MFD_CELL_OF("macsmc-gpio", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "apple,smc-gpio"),
    MFD_CELL_OF("macsmc-hwmon", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "apple,smc-hwmon"),
    MFD_CELL_OF("macsmc-reboot", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "apple,smc-reboot"),
    MFD_CELL_OF("macsmc-rtc", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "apple,smc-rtc"),
    };
    static int apple_smc_cmd_locked(struct apple_smc *smc, u64 cmd, u64 arg,
    u64 size, u64 wsize, u32 *ret_data)
    {
    u8 result;
    int ret;
    u64 msg;
    lockdep_assert_held(&smc.mutex);
    if (smc.boot_stage != APPLE_SMC_INITIALIZED)
    return -EIO;
    if (smc.atomic_mode)
    return -EIO;
    reinit_completion(&smc.cmd_done);
    smc.msg_id = (smc.msg_id + 1) & 0xf;
    msg = (FIELD_PREP(SMC_MSG, cmd) |
    FIELD_PREP(SMC_SIZE, size) |
    FIELD_PREP(SMC_WSIZE, wsize) |
    FIELD_PREP(SMC_ID, smc.msg_id) |
    FIELD_PREP(SMC_DATA, arg));
    ret = apple_rtkit_send_message(smc.rtk, SMC_ENDPOINT, msg, core::ptr::null_mut(), false);
    if (ret) {
    dev_err(smc.dev, "Failed to send command\n");
    return ret;
    }
    if (wait_for_completion_timeout(&smc.cmd_done, msecs_to_jiffies(SMC_TIMEOUT_MS)) <= 0) {
    dev_err(smc.dev, "Command timed out (%llx)", msg);
    return -ETIMEDOUT;
    }
    if (FIELD_GET(SMC_ID, smc.cmd_ret) != smc.msg_id) {
    dev_err(smc.dev, "Command sequence mismatch (expected %d, got %d)\n",
    smc.msg_id, (unsigned int)FIELD_GET(SMC_ID, smc.cmd_ret));
    return -EIO;
    }
    result = FIELD_GET(SMC_RESULT, smc.cmd_ret);
    if (result)
    return -EIO;
    if (ret_data)
// ret_data = FIELD_GET(SMC_DATA, smc->cmd_ret);
    return FIELD_GET(SMC_SIZE, smc.cmd_ret);
    }
    static int apple_smc_cmd(struct apple_smc *smc, u64 cmd, u64 arg,
    u64 size, u64 wsize, u32 *ret_data)
    {
    guard(mutex)(&smc.mutex);
    return apple_smc_cmd_locked(smc, cmd, arg, size, wsize, ret_data);
    }
    static int apple_smc_rw_locked(struct apple_smc *smc, smc_key key,
    const void *wbuf, size_t wsize,
    void *rbuf, size_t rsize)
    {
    u64 smc_size, smc_wsize;
    u32 rdata;
    int ret;
    u64 cmd;
    lockdep_assert_held(&smc.mutex);
    if (rsize > SMC_MAX_SIZE)
    return -EINVAL;
    if (wsize > SMC_MAX_SIZE)
    return -EINVAL;
    if (rsize && wsize) {
    cmd = SMC_MSG_RW_KEY;
    memcpy_toio(smc.shmem.iomem, wbuf, wsize);
    smc_size = rsize;
    smc_wsize = wsize;
    } else if (wsize && !rsize) {
    cmd = SMC_MSG_WRITE_KEY;
    memcpy_toio(smc.shmem.iomem, wbuf, wsize);
//
// Setting size to the length we want to write and wsize to 0
// looks silly but that's how the SMC protocol works ¯\_(ツ)_/¯
//
    smc_size = wsize;
    smc_wsize = 0;
    } else if (!wsize && rsize) {
    cmd = SMC_MSG_READ_KEY;
    smc_size = rsize;
    smc_wsize = 0;
    } else {
    return -EINVAL;
    }
    ret = apple_smc_cmd_locked(smc, cmd, key, smc_size, smc_wsize, &rdata);
    if (ret < 0)
    return ret;
    if (rsize) {
//
// Small data <= 4 bytes is returned as part of the reply
// message which is sent over the mailbox FIFO. Everything
// bigger has to be copied from SRAM which is mapped as
// Device memory.
//
    if (rsize <= 4)
    memcpy(rbuf, &rdata, rsize);
    else
    memcpy_fromio(rbuf, smc.shmem.iomem, rsize);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn apple_smc_read(smc: *mut apple_smc, key: smc_key, buf: *mut c_void, size: usize) -> c_int {
    int apple_smc_read(struct apple_smc *smc, smc_key key, void *buf, size_t size)
    {
    guard(mutex)(&smc.mutex);
    return apple_smc_rw_locked(smc, key, core::ptr::null_mut(), 0, buf, size);
    }
    EXPORT_SYMBOL(apple_smc_read);
#[no_mangle]
pub unsafe extern "C" fn apple_smc_write(smc: *mut apple_smc, key: smc_key, buf: *const c_void, size: usize) -> c_int {
    int apple_smc_write(struct apple_smc *smc, smc_key key, const void *buf, size_t size)
    {
    guard(mutex)(&smc.mutex);
    return apple_smc_rw_locked(smc, key, buf, size, core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL(apple_smc_write);
    int apple_smc_rw(struct apple_smc *smc, smc_key key, const void *wbuf, size_t wsize,
    void *rbuf, size_t rsize)
    {
    guard(mutex)(&smc.mutex);
    return apple_smc_rw_locked(smc, key, wbuf, wsize, rbuf, rsize);
    }
    EXPORT_SYMBOL(apple_smc_rw);
#[no_mangle]
pub unsafe extern "C" fn apple_smc_get_key_by_index(smc: *mut apple_smc, index: c_int, key: *mut smc_key) -> c_int {
    int apple_smc_get_key_by_index(struct apple_smc *smc, int index, smc_key *key)
    {
    int ret;
    ret = apple_smc_cmd(smc, SMC_MSG_GET_KEY_BY_INDEX, index, 0, 0, key);
// key = swab32(*key);
    return ret;
    }
    EXPORT_SYMBOL(apple_smc_get_key_by_index);
#[no_mangle]
pub unsafe extern "C" fn apple_smc_get_key_info(smc: *mut apple_smc, key: smc_key, info: *mut apple_smc_key_info) -> c_int {
    int apple_smc_get_key_info(struct apple_smc *smc, smc_key key, struct apple_smc_key_info *info)
    {
    u8 key_info[6];
    int ret;
    ret = apple_smc_cmd(smc, SMC_MSG_GET_KEY_INFO, key, 0, 0, core::ptr::null_mut());
    if (ret >= 0 && info) {
    memcpy_fromio(key_info, smc.shmem.iomem, sizeof(key_info));
    info.size = key_info[0];
    info.type_code = get_unaligned_be32(&key_info[1]);
    info.flags = key_info[5];
    }
    return ret;
    }
    EXPORT_SYMBOL(apple_smc_get_key_info);
#[no_mangle]
pub unsafe extern "C" fn apple_smc_enter_atomic(smc: *mut apple_smc) -> c_int {
    int apple_smc_enter_atomic(struct apple_smc *smc)
    {
    guard(mutex)(&smc.mutex);
//
// Disable notifications since this is called before shutdown and no
// notification handler will be able to handle the notification
// using atomic operations only. Also ignore any failure here
// because we're about to shut down or reboot anyway.
// We can't use apple_smc_write_flag here since that would try to lock
// smc->mutex again.
//
    let mut flag: u8 = 0;
    apple_smc_rw_locked(smc, SMC_KEY(NTAP), &flag, sizeof(flag), core::ptr::null_mut(), 0);
    smc.atomic_mode = true;
    return 0;
    }
    EXPORT_SYMBOL(apple_smc_enter_atomic);
#[no_mangle]
pub unsafe extern "C" fn apple_smc_write_atomic(smc: *mut apple_smc, key: smc_key, buf: *const c_void, size: usize) -> c_int {
    int apple_smc_write_atomic(struct apple_smc *smc, smc_key key, const void *buf, size_t size)
    {
    guard(spinlock_irqsave)(&smc.lock);
    u8 result;
    int ret;
    u64 msg;
    if (size > SMC_MAX_SIZE || size == 0)
    return -EINVAL;
    if (smc.boot_stage != APPLE_SMC_INITIALIZED)
    return -EIO;
    if (!smc.atomic_mode)
    return -EIO;
    memcpy_toio(smc.shmem.iomem, buf, size);
    smc.msg_id = (smc.msg_id + 1) & 0xf;
    msg = (FIELD_PREP(SMC_MSG, SMC_MSG_WRITE_KEY) |
    FIELD_PREP(SMC_SIZE, size) |
    FIELD_PREP(SMC_ID, smc.msg_id) |
    FIELD_PREP(SMC_DATA, key));
    smc.atomic_pending = true;
    ret = apple_rtkit_send_message(smc.rtk, SMC_ENDPOINT, msg, core::ptr::null_mut(), true);
    if (ret < 0) {
    dev_err(smc.dev, "Failed to send command (%d)\n", ret);
    return ret;
    }
    while (smc.atomic_pending) {
    ret = apple_rtkit_poll(smc.rtk);
    if (ret < 0) {
    dev_err(smc.dev, "RTKit poll failed (%llx)", msg);
    return ret;
    }
    udelay(100);
    }
    if (FIELD_GET(SMC_ID, smc.cmd_ret) != smc.msg_id) {
    dev_err(smc.dev, "Command sequence mismatch (expected %d, got %d)\n",
    smc.msg_id, (unsigned int)FIELD_GET(SMC_ID, smc.cmd_ret));
    return -EIO;
    }
    result = FIELD_GET(SMC_RESULT, smc.cmd_ret);
    if (result)
    return -EIO;
    return FIELD_GET(SMC_SIZE, smc.cmd_ret);
    }
    EXPORT_SYMBOL(apple_smc_write_atomic);
#[no_mangle]
unsafe extern "C" fn apple_smc_rtkit_crashed(cookie: *mut c_void, bfr: *const c_void, bfr_len: usize) {
    static void apple_smc_rtkit_crashed(void *cookie, const void *bfr, size_t bfr_len)
    {
    struct apple_smc *smc = cookie;
    smc.boot_stage = APPLE_SMC_ERROR_CRASHED;
    dev_err(smc.dev, "SMC crashed! Your system will reboot in a few seconds...\n");
    }
#[no_mangle]
unsafe extern "C" fn apple_smc_rtkit_shmem_setup(cookie: *mut c_void, bfr: *mut apple_rtkit_shmem) -> c_int {
    static int apple_smc_rtkit_shmem_setup(void *cookie, struct apple_rtkit_shmem *bfr)
    {
    struct apple_smc *smc = cookie;
    size_t bfr_end;
    if (!bfr.iova) {
    dev_err(smc.dev, "RTKit wants a RAM buffer\n");
    return -EIO;
    }
    if (check_add_overflow(bfr.iova, bfr.size - 1, &bfr_end))
    return -EFAULT;
    if (bfr.iova < smc.sram.start || bfr.iova > smc.sram.end ||
    bfr_end > smc.sram.end) {
    dev_err(smc.dev, "RTKit buffer request outside SRAM region: [0x%llx, 0x%llx]\n",
    (unsigned long long)bfr.iova,
    (unsigned long long)bfr_end);
    return -EFAULT;
    }
    bfr.iomem = smc.sram_base + (bfr.iova - smc.sram.start);
    bfr.is_mapped = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_smc_rtkit_recv_early(cookie: *mut c_void, endpoint: u8, message: u64) -> bool {
    static bool apple_smc_rtkit_recv_early(void *cookie, u8 endpoint, u64 message)
    {
    struct apple_smc *smc = cookie;
    if (endpoint != SMC_ENDPOINT) {
    dev_warn(smc.dev, "Received message for unknown endpoint 0x%x\n", endpoint);
    return false;
    }
    if (smc.boot_stage == APPLE_SMC_BOOTING) {
    int ret;
    smc.shmem.iova = message;
    smc.shmem.size = SMC_SHMEM_SIZE;
    ret = apple_smc_rtkit_shmem_setup(smc, &smc.shmem);
    if (ret < 0) {
    smc.boot_stage = APPLE_SMC_ERROR_NO_SHMEM;
    dev_err(smc.dev, "Failed to initialize shared memory (%d)\n", ret);
    } else {
    smc.boot_stage = APPLE_SMC_INITIALIZED;
    }
    complete(&smc.init_done);
    } else if (FIELD_GET(SMC_MSG, message) == SMC_MSG_NOTIFICATION) {
// Handle these in the RTKit worker thread
    return false;
    } else {
    smc.cmd_ret = message;
    if (smc.atomic_pending)
    smc.atomic_pending = false;
    else
    complete(&smc.cmd_done);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn apple_smc_rtkit_recv(cookie: *mut c_void, endpoint: u8, message: u64) {
    static void apple_smc_rtkit_recv(void *cookie, u8 endpoint, u64 message)
    {
    struct apple_smc *smc = cookie;
    if (endpoint != SMC_ENDPOINT) {
    dev_warn(smc.dev, "Received message for unknown endpoint 0x%x\n", endpoint);
    return;
    }
    if (FIELD_GET(SMC_MSG, message) != SMC_MSG_NOTIFICATION) {
    dev_warn(smc.dev, "Received unknown message from worker: 0x%llx\n", message);
    return;
    }
    blocking_notifier_call_chain(&smc.event_handlers, FIELD_GET(SMC_DATA, message), core::ptr::null_mut());
    }
    static const struct apple_rtkit_ops apple_smc_rtkit_ops = {
    .crashed = apple_smc_rtkit_crashed,
    .recv_message = apple_smc_rtkit_recv,
    .recv_message_early = apple_smc_rtkit_recv_early,
    .shmem_setup = apple_smc_rtkit_shmem_setup,
    };
#[no_mangle]
unsafe extern "C" fn apple_smc_rtkit_shutdown(data: *mut c_void) {
    static void apple_smc_rtkit_shutdown(void *data)
    {
    struct apple_smc *smc = data;
// Shut down SMC firmware, if it's not completely wedged
    if (apple_rtkit_is_running(smc.rtk))
    apple_rtkit_quiesce(smc.rtk);
    }
#[no_mangle]
unsafe extern "C" fn apple_smc_disable_notifications(data: *mut c_void) {
    static void apple_smc_disable_notifications(void *data)
    {
    struct apple_smc *smc = data;
    apple_smc_write_flag(smc, SMC_KEY(NTAP), false);
    }
#[no_mangle]
unsafe extern "C" fn apple_smc_probe(pdev: *mut platform_device) -> c_int {
    static int apple_smc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct apple_smc *smc;
    __be32 count;
    int ret;
    smc = devm_kzalloc(dev, sizeof(*smc), GFP_KERNEL);
    if (!smc)
    return -ENOMEM;
    mutex_init(&smc.mutex);
    smc.dev = &pdev.dev;
    smc.sram_base = devm_platform_get_and_ioremap_resource(pdev, 1, &smc.sram);
    if (IS_ERR(smc.sram_base))
    return dev_err_probe(dev, PTR_ERR(smc.sram_base), "Failed to map SRAM region");
    smc.rtk = devm_apple_rtkit_init(dev, smc, core::ptr::null_mut(), 0, &apple_smc_rtkit_ops);
    if (IS_ERR(smc.rtk))
    return dev_err_probe(dev, PTR_ERR(smc.rtk), "Failed to initialize RTKit");
    smc.boot_stage = APPLE_SMC_BOOTING;
    ret = apple_rtkit_wake(smc.rtk);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to wake up SMC");
    ret = devm_add_action_or_reset(dev, apple_smc_rtkit_shutdown, smc);
    if (ret)
    return ret;
    ret = apple_rtkit_start_ep(smc.rtk, SMC_ENDPOINT);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to start SMC endpoint");
    init_completion(&smc.init_done);
    init_completion(&smc.cmd_done);
    ret = apple_rtkit_send_message(smc.rtk, SMC_ENDPOINT,
    FIELD_PREP(SMC_MSG, SMC_MSG_INITIALIZE), core::ptr::null_mut(), false);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to send init message");
    if (wait_for_completion_timeout(&smc.init_done, msecs_to_jiffies(SMC_TIMEOUT_MS)) == 0) {
    dev_err(dev, "Timed out initializing SMC");
    return -ETIMEDOUT;
    }
    if (smc.boot_stage != APPLE_SMC_INITIALIZED) {
    dev_err(dev, "SMC failed to boot successfully, boot stage=%d\n", smc.boot_stage);
    return -EIO;
    }
    dev_set_drvdata(&pdev.dev, smc);
    BLOCKING_INIT_NOTIFIER_HEAD(&smc.event_handlers);
    ret = apple_smc_read(smc, SMC_KEY(#KEY), &count, sizeof(count));
    if (ret >= 0 && ret != sizeof(count))
    ret = -EINVAL;
    if (ret < 0)
    return dev_err_probe(smc.dev, ret, "Failed to get key count");
    smc.key_count = be32_to_cpu(count);
// Enable notifications
    apple_smc_write_flag(smc, SMC_KEY(NTAP), true);
    ret = devm_add_action_or_reset(dev, apple_smc_disable_notifications, smc);
    if (ret)
    return ret;
    ret = devm_mfd_add_devices(smc.dev, PLATFORM_DEVID_NONE,
    apple_smc_devs, ARRAY_SIZE(apple_smc_devs),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(smc.dev, ret, "Failed to register sub-devices");
    return 0;
    }
    static const struct of_device_id apple_smc_of_match[] = {
    { .compatible = "apple,t8103-smc" },
    { .compatible = "apple,smc" },
    {},
    };
    MODULE_DEVICE_TABLE(of, apple_smc_of_match);
    static struct platform_driver apple_smc_driver = {
    .driver = {
    .name = "macsmc",
    .of_match_table = apple_smc_of_match,
    },
    .probe = apple_smc_probe,
    };
    module_platform_driver(apple_smc_driver);
    MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
    MODULE_AUTHOR("Sven Peter <sven@kernel.org>");
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_DESCRIPTION("Apple SMC driver");
