//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/mailbox-test.c
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
// Copyright (C) 2015 ST Microelectronics
//
// Author: Lee Jones <lee.jones@linaro.org>
//

pub const MBOX_MAX_SIG_LEN: c_int = 8;
pub const MBOX_MAX_MSG_LEN: c_int = 128;
pub const MBOX_BYTES_PER_LINE: c_int = 16;

    (MBOX_MAX_MSG_LEN / MBOX_BYTES_PER_LINE))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_test_device {
    pub dev: *mut device,
    pub tx_mmio: *mut void __iomem,
    pub rx_mmio: *mut void __iomem,
    pub tx_channel: *mut mbox_chan,
    pub rx_channel: *mut mbox_chan,
    pub rx_buffer: *mut c_char,
    pub signal: *mut c_char,
    pub message: *mut c_char,
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub waitq: wait_queue_head_t,
    pub data_ready: bool,
    pub async_queue: *mut fasync_struct,
    pub root_debugfs_dir: *mut dentry,
}

    static ssize_t mbox_test_signal_write(struct file *filp,
    const char __user *userbuf,
    size_t count, loff_t *ppos)
    {
    struct mbox_test_device *tdev = filp.private_data;
    if (!tdev.tx_channel) {
    dev_err(tdev.dev, "Channel cannot do Tx\n");
    return -EINVAL;
    }
    if (count > MBOX_MAX_SIG_LEN) {
    dev_err(tdev.dev,
    "Signal length %zd greater than max allowed %d\n",
    count, MBOX_MAX_SIG_LEN);
    return -EINVAL;
    }
// Only allocate memory if we need to
    if (!tdev.signal) {
    tdev.signal = kzalloc(MBOX_MAX_SIG_LEN, GFP_KERNEL);
    if (!tdev.signal)
    return -ENOMEM;
    }
    if (copy_from_user(tdev.signal, userbuf, count)) {
    kfree(tdev.signal);
    tdev.signal = core::ptr::null_mut();
    return -EFAULT;
    }
    return count;
    }
    static const struct file_operations mbox_test_signal_ops = {
    .write	= mbox_test_signal_write,
    .open	= simple_open,
    .llseek	= generic_file_llseek,
    };
#[no_mangle]
unsafe extern "C" fn mbox_test_message_fasync(fd: c_int, filp: *mut file, on: c_int) -> c_int {
    static int mbox_test_message_fasync(int fd, struct file *filp, int on)
    {
    struct mbox_test_device *tdev = filp.private_data;
    return fasync_helper(fd, filp, on, &tdev.async_queue);
    }
    static ssize_t mbox_test_message_write(struct file *filp,
    const char __user *userbuf,
    size_t count, loff_t *ppos)
    {
    struct mbox_test_device *tdev = filp.private_data;
    char *message;
    void *data;
    int ret;
    if (!tdev.tx_channel) {
    dev_err(tdev.dev, "Channel cannot do Tx\n");
    return -EINVAL;
    }
    if (count > MBOX_MAX_MSG_LEN) {
    dev_err(tdev.dev,
    "Message length %zd greater than max allowed %d\n",
    count, MBOX_MAX_MSG_LEN);
    return -EINVAL;
    }
    message = kzalloc(MBOX_MAX_MSG_LEN, GFP_KERNEL);
    if (!message)
    return -ENOMEM;
    mutex_lock(&tdev.mutex);
    tdev.message = message;
    ret = copy_from_user(tdev.message, userbuf, count);
    if (ret) {
    ret = -EFAULT;
    goto out;
    }
//
// A separate signal is only of use if there is
// MMIO to subsequently pass the message through
//
    if (tdev.tx_mmio && tdev.signal) {
    print_hex_dump_bytes("Client: Sending: Signal: ", DUMP_PREFIX_ADDRESS,
    tdev.signal, MBOX_MAX_SIG_LEN);
    data = tdev.signal;
    } else
    data = tdev.message;
    print_hex_dump_bytes("Client: Sending: Message: ", DUMP_PREFIX_ADDRESS,
    tdev.message, MBOX_MAX_MSG_LEN);
    ret = mbox_send_message(tdev.tx_channel, data);
    if (ret < 0)
    dev_err(tdev.dev, "Failed to send message via mailbox\n");
    out:
    kfree(tdev.signal);
    kfree(tdev.message);
    tdev.signal = core::ptr::null_mut();
    mutex_unlock(&tdev.mutex);
    return ret < 0 ? ret : count;
    }
#[no_mangle]
unsafe extern "C" fn mbox_test_message_data_ready(tdev: *mut mbox_test_device) -> bool {
    static bool mbox_test_message_data_ready(struct mbox_test_device *tdev)
    {
    bool data_ready;
    unsigned long flags;
    spin_lock_irqsave(&tdev.lock, flags);
    data_ready = tdev.data_ready;
    spin_unlock_irqrestore(&tdev.lock, flags);
    return data_ready;
    }
    static ssize_t mbox_test_message_read(struct file *filp, char __user *userbuf,
    size_t count, loff_t *ppos)
    {
    struct mbox_test_device *tdev = filp.private_data;
    unsigned long flags;
    char *touser, *ptr;
    let mut l: c_int = 0;
    int ret;
    DECLARE_WAITQUEUE(wait, current);
    touser = kzalloc(MBOX_HEXDUMP_MAX_LEN + 1, GFP_KERNEL);
    if (!touser)
    return -ENOMEM;
    if (!tdev.rx_channel) {
    ret = snprintf(touser, 20, "<NO RX CAPABILITY>\n");
    ret = simple_read_from_buffer(userbuf, count, ppos,
    touser, ret);
    goto kfree_err;
    }
    add_wait_queue(&tdev.waitq, &wait);
    do {
    __set_current_state(TASK_INTERRUPTIBLE);
    if (mbox_test_message_data_ready(tdev))
    break;
    if (filp.f_flags & O_NONBLOCK) {
    ret = -EAGAIN;
    goto waitq_err;
    }
    if (signal_pending(current)) {
    ret = -ERESTARTSYS;
    goto waitq_err;
    }
    schedule();
    } while (1);
    spin_lock_irqsave(&tdev.lock, flags);
    ptr = tdev.rx_buffer;
    while (l < MBOX_HEXDUMP_MAX_LEN) {
    hex_dump_to_buffer(ptr,
    MBOX_BYTES_PER_LINE,
    MBOX_BYTES_PER_LINE, 1, touser + l,
    MBOX_HEXDUMP_LINE_LEN, true);
    ptr += MBOX_BYTES_PER_LINE;
    l += MBOX_HEXDUMP_LINE_LEN;
// (touser + (l - 1)) = '\n';
    }
// (touser + l) = '\0';
    memset(tdev.rx_buffer, 0, MBOX_MAX_MSG_LEN);
    tdev.data_ready = false;
    spin_unlock_irqrestore(&tdev.lock, flags);
    ret = simple_read_from_buffer(userbuf, count, ppos, touser, MBOX_HEXDUMP_MAX_LEN);
    waitq_err:
    __set_current_state(TASK_RUNNING);
    remove_wait_queue(&tdev.waitq, &wait);
    kfree_err:
    kfree(touser);
    return ret;
    }
    static __poll_t
    mbox_test_message_poll(struct file *filp, struct poll_table_struct *wait)
    {
    struct mbox_test_device *tdev = filp.private_data;
    poll_wait(filp, &tdev.waitq, wait);
    if (mbox_test_message_data_ready(tdev))
    return EPOLLIN | EPOLLRDNORM;
    return 0;
    }
    static const struct file_operations mbox_test_message_ops = {
    .write	= mbox_test_message_write,
    .read	= mbox_test_message_read,
    .fasync	= mbox_test_message_fasync,
    .poll	= mbox_test_message_poll,
    .open	= simple_open,
    .llseek	= generic_file_llseek,
    };
    static int mbox_test_add_debugfs(struct platform_device *pdev,
    struct mbox_test_device *tdev)
    {
    if (!debugfs_initialized())
    return 0;
    tdev.root_debugfs_dir = debugfs_create_dir(dev_name(&pdev.dev), core::ptr::null_mut());
    if (IS_ERR(tdev.root_debugfs_dir)) {
    dev_err(&pdev.dev, "Failed to create Mailbox debugfs\n");
    return -EINVAL;
    }
    debugfs_create_file("message", 0600, tdev.root_debugfs_dir,
    tdev, &mbox_test_message_ops);
    debugfs_create_file("signal", 0200, tdev.root_debugfs_dir,
    tdev, &mbox_test_signal_ops);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mbox_test_receive_message(client: *mut mbox_client, message: *mut c_void) {
    static void mbox_test_receive_message(struct mbox_client *client, void *message)
    {
    struct mbox_test_device *tdev = dev_get_drvdata(client.dev);
    unsigned long flags;
    spin_lock_irqsave(&tdev.lock, flags);
    if (tdev.rx_mmio) {
    memcpy_fromio(tdev.rx_buffer, tdev.rx_mmio, MBOX_MAX_MSG_LEN);
    print_hex_dump_bytes("Client: Received [MMIO]: ", DUMP_PREFIX_ADDRESS,
    tdev.rx_buffer, MBOX_MAX_MSG_LEN);
    } else if (message) {
    print_hex_dump_bytes("Client: Received [API]: ", DUMP_PREFIX_ADDRESS,
    message, MBOX_MAX_MSG_LEN);
    memcpy(tdev.rx_buffer, message, MBOX_MAX_MSG_LEN);
    }
    tdev.data_ready = true;
    spin_unlock_irqrestore(&tdev.lock, flags);
    wake_up_interruptible(&tdev.waitq);
    kill_fasync(&tdev.async_queue, SIGIO, POLL_IN);
    }
#[no_mangle]
unsafe extern "C" fn mbox_test_prepare_message(client: *mut mbox_client, message: *mut c_void) {
    static void mbox_test_prepare_message(struct mbox_client *client, void *message)
    {
    struct mbox_test_device *tdev = dev_get_drvdata(client.dev);
    if (tdev.tx_mmio) {
    if (tdev.signal)
    memcpy_toio(tdev.tx_mmio, tdev.message, MBOX_MAX_MSG_LEN);
    else
    memcpy_toio(tdev.tx_mmio, message, MBOX_MAX_MSG_LEN);
    }
    }
    static void mbox_test_message_sent(struct mbox_client *client,
    void *message, int r)
    {
    if (r)
    dev_warn(client.dev,
    "Client: Message could not be sent: %d\n", r);
    else
    dev_info(client.dev,
    "Client: Message sent\n");
    }
    static struct mbox_chan *
    mbox_test_request_channel(struct platform_device *pdev, const char *name)
    {
    struct mbox_client *client;
    struct mbox_chan *channel;
    client = devm_kzalloc(&pdev.dev, sizeof(*client), GFP_KERNEL);
    if (!client)
    return core::ptr::null_mut();
    client.dev		= &pdev.dev;
    client.rx_callback	= mbox_test_receive_message;
    client.tx_prepare	= mbox_test_prepare_message;
    client.tx_done		= mbox_test_message_sent;
    client.tx_block	= true;
    client.knows_txdone	= false;
    client.tx_tout		= 500;
    channel = mbox_request_channel_byname(client, name);
    if (IS_ERR(channel)) {
    dev_warn(&pdev.dev, "Failed to request %s channel\n", name);
    return core::ptr::null_mut();
    }
    return channel;
    }
    static void __iomem *mbox_test_ioremap(struct platform_device *pdev, unsigned int res_num)
    {
    struct resource *res;
    void __iomem *mmio;
    res = platform_get_resource(pdev, IORESOURCE_MEM, res_num);
    if (!res)
    return core::ptr::null_mut();
    mmio = devm_ioremap_resource(&pdev.dev, res);
    if (PTR_ERR(mmio) == -EBUSY) {
    dev_info(&pdev.dev, "trying workaround with plain ioremap\n");
    return devm_ioremap(&pdev.dev, res.start, resource_size(res));
    }
    return IS_ERR(mmio) ? core::ptr::null_mut() : mmio;
    }
#[no_mangle]
unsafe extern "C" fn mbox_test_probe(pdev: *mut platform_device) -> c_int {
    static int mbox_test_probe(struct platform_device *pdev)
    {
    struct mbox_test_device *tdev;
    int ret;
    tdev = devm_kzalloc(&pdev.dev, sizeof(*tdev), GFP_KERNEL);
    if (!tdev)
    return -ENOMEM;
    tdev.dev = &pdev.dev;
    spin_lock_init(&tdev.lock);
    mutex_init(&tdev.mutex);
    init_waitqueue_head(&tdev.waitq);
    platform_set_drvdata(pdev, tdev);
// It's okay for MMIO to be NULL
    tdev.tx_mmio = mbox_test_ioremap(pdev, 0);
// If specified, second reg entry is Rx MMIO
    tdev.rx_mmio = mbox_test_ioremap(pdev, 1);
    if (!tdev.rx_mmio)
    tdev.rx_mmio = tdev.tx_mmio;
    tdev.tx_channel = mbox_test_request_channel(pdev, "tx");
    tdev.rx_channel = mbox_test_request_channel(pdev, "rx");
    if (!tdev.tx_channel && !tdev.rx_channel)
    return -EPROBE_DEFER;
// If Rx is not specified but has Rx MMIO, then Rx = Tx
    if (!tdev.rx_channel && (tdev.rx_mmio != tdev.tx_mmio))
    tdev.rx_channel = tdev.tx_channel;
    if (tdev.rx_channel) {
    tdev.rx_buffer = devm_kzalloc(&pdev.dev,
    MBOX_MAX_MSG_LEN, GFP_KERNEL);
    if (!tdev.rx_buffer) {
    ret = -ENOMEM;
    goto err_free_chans;
    }
    }
    ret = mbox_test_add_debugfs(pdev, tdev);
    if (ret)
    goto err_free_chans;
    dev_info(&pdev.dev, "Successfully registered\n");
    return 0;
    err_free_chans:
    if (tdev.tx_channel)
    mbox_free_channel(tdev.tx_channel);
    if (tdev.rx_channel && tdev.rx_channel != tdev.tx_channel)
    mbox_free_channel(tdev.rx_channel);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mbox_test_remove(pdev: *mut platform_device) {
    static void mbox_test_remove(struct platform_device *pdev)
    {
    struct mbox_test_device *tdev = platform_get_drvdata(pdev);
    debugfs_remove_recursive(tdev.root_debugfs_dir);
    if (tdev.tx_channel)
    mbox_free_channel(tdev.tx_channel);
    if (tdev.rx_channel && tdev.rx_channel != tdev.tx_channel)
    mbox_free_channel(tdev.rx_channel);
    }
    static const struct of_device_id mbox_test_match[] = {
    { .compatible = "mailbox-test" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mbox_test_match);
    static struct platform_driver mbox_test_driver = {
    .driver = {
    .name = "mailbox_test",
    .of_match_table = mbox_test_match,
    },
    .probe = mbox_test_probe,
    .remove = mbox_test_remove,
    };
    module_platform_driver(mbox_test_driver);
    MODULE_DESCRIPTION("Generic Mailbox Testing Facility");
    MODULE_AUTHOR("Lee Jones <lee.jones@linaro.org");
    MODULE_LICENSE("GPL v2");
