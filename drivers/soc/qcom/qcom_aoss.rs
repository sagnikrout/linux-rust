//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/qcom_aoss.c
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
// Copyright (c) 2019, Linaro Ltd
//

// Macro flag: #define CREATE_TRACE_POINTS

pub const QMP_DESC_MAGIC: c_uint = 0x0;
pub const QMP_DESC_VERSION: c_uint = 0x4;
pub const QMP_DESC_FEATURES: c_uint = 0x8;
// AOP-side offsets
pub const QMP_DESC_UCORE_LINK_STATE: c_uint = 0xc;
pub const QMP_DESC_UCORE_LINK_STATE_ACK: c_uint = 0x10;
pub const QMP_DESC_UCORE_CH_STATE: c_uint = 0x14;
pub const QMP_DESC_UCORE_CH_STATE_ACK: c_uint = 0x18;
pub const QMP_DESC_UCORE_MBOX_SIZE: c_uint = 0x1c;
pub const QMP_DESC_UCORE_MBOX_OFFSET: c_uint = 0x20;
// Linux-side offsets
pub const QMP_DESC_MCORE_LINK_STATE: c_uint = 0x24;
pub const QMP_DESC_MCORE_LINK_STATE_ACK: c_uint = 0x28;
pub const QMP_DESC_MCORE_CH_STATE: c_uint = 0x2c;
pub const QMP_DESC_MCORE_CH_STATE_ACK: c_uint = 0x30;
pub const QMP_DESC_MCORE_MBOX_SIZE: c_uint = 0x34;
pub const QMP_DESC_MCORE_MBOX_OFFSET: c_uint = 0x38;

pub const QMP_MAGIC: c_uint = 0x4d41494c /* mail */;
pub const QMP_VERSION: c_int = 1;
// 64 bytes is enough to store the requests and provides padding to 4 bytes
pub const QMP_MSG_LEN: c_int = 64;
pub const QMP_NUM_COOLING_RESOURCES: c_int = 2;
pub const QMP_DEBUGFS_FILES: c_int = 4;
    let mut qmp_cdev_max_state: static bool = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmp_cooling_device {
    pub cdev: *mut thermal_cooling_device,
    pub qmp: *mut qmp,
    pub name: *mut c_char,
    pub state: bool,
}

//
// struct qmp - driver state for QMP implementation
// @msgram: iomem referencing the message RAM used for communication
// @dev: reference to QMP device
// @mbox_client: mailbox client used to ring the doorbell on transmit
// @mbox_chan: mailbox channel used to ring the doorbell on transmit
// @offset: offset within @msgram where messages should be written
// @size: maximum size of the messages to be transmitted
// @event: wait_queue for synchronization with the IRQ
// @tx_lock: provides synchronization between multiple callers of qmp_send()
// @qdss_clk: QDSS clock hw struct
// @cooling_devs: thermal cooling devices
// @debugfs_root: directory for the developer/tester interface
// @debugfs_files: array of individual debugfs entries under debugfs_root
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmp {
    pub msgram: *mut void __iomem,
    pub dev: *mut device,
    pub mbox_client: mbox_client,
    pub mbox_chan: *mut mbox_chan,
    pub offset: usize,
    pub size: usize,
    pub event: wait_queue_head_t,
    pub tx_lock: mutex,
    pub qdss_clk: clk_hw,
    pub cooling_devs: *mut qmp_cooling_device,
    pub debugfs_root: *mut dentry,
    pub debugfs_files: [*mut dentry; QMP_DEBUGFS_FILES],
}

#[no_mangle]
unsafe extern "C" fn qmp_kick(qmp: *mut qmp) {
    static void qmp_kick(struct qmp *qmp)
    {
    mbox_send_message(qmp.mbox_chan, core::ptr::null_mut());
    mbox_client_txdone(qmp.mbox_chan, 0);
    }
#[no_mangle]
unsafe extern "C" fn qmp_magic_valid(qmp: *mut qmp) -> bool {
    static bool qmp_magic_valid(struct qmp *qmp)
    {
    return readl(qmp.msgram + QMP_DESC_MAGIC) == QMP_MAGIC;
    }
#[no_mangle]
unsafe extern "C" fn qmp_link_acked(qmp: *mut qmp) -> bool {
    static bool qmp_link_acked(struct qmp *qmp)
    {
    return readl(qmp.msgram + QMP_DESC_MCORE_LINK_STATE_ACK) == QMP_STATE_UP;
    }
#[no_mangle]
unsafe extern "C" fn qmp_mcore_channel_acked(qmp: *mut qmp) -> bool {
    static bool qmp_mcore_channel_acked(struct qmp *qmp)
    {
    return readl(qmp.msgram + QMP_DESC_MCORE_CH_STATE_ACK) == QMP_STATE_UP;
    }
#[no_mangle]
unsafe extern "C" fn qmp_ucore_channel_up(qmp: *mut qmp) -> bool {
    static bool qmp_ucore_channel_up(struct qmp *qmp)
    {
    return readl(qmp.msgram + QMP_DESC_UCORE_CH_STATE) == QMP_STATE_UP;
    }
#[no_mangle]
unsafe extern "C" fn qmp_open(qmp: *mut qmp) -> c_int {
    static int qmp_open(struct qmp *qmp)
    {
    int ret;
    u32 val;
    if (!qmp_magic_valid(qmp)) {
    dev_err(qmp.dev, "QMP magic doesn't match\n");
    return -EINVAL;
    }
    val = readl(qmp.msgram + QMP_DESC_VERSION);
    if (val != QMP_VERSION) {
    dev_err(qmp.dev, "unsupported QMP version %d\n", val);
    return -EINVAL;
    }
    qmp.offset = readl(qmp.msgram + QMP_DESC_MCORE_MBOX_OFFSET);
    qmp.size = readl(qmp.msgram + QMP_DESC_MCORE_MBOX_SIZE);
    if (!qmp.size) {
    dev_err(qmp.dev, "invalid mailbox size\n");
    return -EINVAL;
    }
// Ack remote core's link state
    val = readl(qmp.msgram + QMP_DESC_UCORE_LINK_STATE);
    writel(val, qmp.msgram + QMP_DESC_UCORE_LINK_STATE_ACK);
// Set local core's link state to up
    writel(QMP_STATE_UP, qmp.msgram + QMP_DESC_MCORE_LINK_STATE);
    qmp_kick(qmp);
    ret = wait_event_timeout(qmp.event, qmp_link_acked(qmp), HZ);
    if (!ret) {
    dev_err(qmp.dev, "ucore didn't ack link\n");
    goto timeout_close_link;
    }
    writel(QMP_STATE_UP, qmp.msgram + QMP_DESC_MCORE_CH_STATE);
    qmp_kick(qmp);
    ret = wait_event_timeout(qmp.event, qmp_ucore_channel_up(qmp), HZ);
    if (!ret) {
    dev_err(qmp.dev, "ucore didn't open channel\n");
    goto timeout_close_channel;
    }
// Ack remote core's channel state
    writel(QMP_STATE_UP, qmp.msgram + QMP_DESC_UCORE_CH_STATE_ACK);
    qmp_kick(qmp);
    ret = wait_event_timeout(qmp.event, qmp_mcore_channel_acked(qmp), HZ);
    if (!ret) {
    dev_err(qmp.dev, "ucore didn't ack channel\n");
    goto timeout_close_channel;
    }
    return 0;
    timeout_close_channel:
    writel(QMP_STATE_DOWN, qmp.msgram + QMP_DESC_MCORE_CH_STATE);
    timeout_close_link:
    writel(QMP_STATE_DOWN, qmp.msgram + QMP_DESC_MCORE_LINK_STATE);
    qmp_kick(qmp);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn qmp_close(qmp: *mut qmp) {
    static void qmp_close(struct qmp *qmp)
    {
    writel(QMP_STATE_DOWN, qmp.msgram + QMP_DESC_MCORE_CH_STATE);
    writel(QMP_STATE_DOWN, qmp.msgram + QMP_DESC_MCORE_LINK_STATE);
    qmp_kick(qmp);
    }
#[no_mangle]
unsafe extern "C" fn qmp_intr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qmp_intr(int irq, void *data)
    {
    struct qmp *qmp = data;
    wake_up_all(&qmp.event);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qmp_message_empty(qmp: *mut qmp) -> bool {
    static bool qmp_message_empty(struct qmp *qmp)
    {
    return readl(qmp.msgram + qmp.offset) == 0;
    }
//
// qmp_send() - send a message to the AOSS
// @qmp: qmp context
// @fmt: format string for message to be sent
// @...: arguments for the format string
//
// Transmit message to AOSS and wait for the AOSS to acknowledge the message.
// data must not be longer than the mailbox size. Access is synchronized by
// this implementation.
//
// Return: 0 on success, negative errno on failure
//
#[no_mangle]
pub unsafe extern "C" fn __printf(_arg: 2, qmp: *mut 3) qmp_send(struct qmp, fmt: *const c_char, ...) -> c_int {
    int __printf(2, 3) qmp_send(struct qmp *qmp, const char *fmt, ...)
    {
    char buf[QMP_MSG_LEN];
    long time_left;
    va_list args;
    int len;
    int ret;
    if (WARN_ON(IS_ERR_OR_NULL(qmp) || !fmt))
    return -EINVAL;
    memset(buf, 0, sizeof(buf));
    va_start(args, fmt);
    len = vsnprintf(buf, sizeof(buf), fmt, args);
    va_end(args);
    if (WARN_ON(len >= sizeof(buf)))
    return -EINVAL;
    mutex_lock(&qmp.tx_lock);
    trace_aoss_send(buf);
// The message RAM only implements 32-bit accesses
    __iowrite32_copy(qmp.msgram + qmp.offset + sizeof(u32),
    buf, sizeof(buf) / sizeof(u32));
    writel(sizeof(buf), qmp.msgram + qmp.offset);
// Read back length to confirm data written in message RAM
    readl(qmp.msgram + qmp.offset);
    qmp_kick(qmp);
    time_left = wait_event_interruptible_timeout(qmp.event,
    qmp_message_empty(qmp), HZ);
    if (!time_left) {
    dev_err(qmp.dev, "ucore did not ack channel\n");
    ret = -ETIMEDOUT;
// Clear message from buffer
    writel(0, qmp.msgram + qmp.offset);
    } else {
    ret = 0;
    }
    trace_aoss_send_done(buf, ret);
    mutex_unlock(&qmp.tx_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(qmp_send);
#[no_mangle]
unsafe extern "C" fn qmp_qdss_clk_prepare(hw: *mut clk_hw) -> c_int {
    static int qmp_qdss_clk_prepare(struct clk_hw *hw)
    {
    static const char *buf = "{class: clock, res: qdss, val: 1}";
    struct qmp *qmp = container_of(hw, struct qmp, qdss_clk);
    return qmp_send(qmp, buf);
    }
#[no_mangle]
unsafe extern "C" fn qmp_qdss_clk_unprepare(hw: *mut clk_hw) {
    static void qmp_qdss_clk_unprepare(struct clk_hw *hw)
    {
    static const char *buf = "{class: clock, res: qdss, val: 0}";
    struct qmp *qmp = container_of(hw, struct qmp, qdss_clk);
    qmp_send(qmp, buf);
    }
    static const struct clk_ops qmp_qdss_clk_ops = {
    .prepare = qmp_qdss_clk_prepare,
    .unprepare = qmp_qdss_clk_unprepare,
    };
#[no_mangle]
unsafe extern "C" fn qmp_qdss_clk_add(qmp: *mut qmp) -> c_int {
    static int qmp_qdss_clk_add(struct qmp *qmp)
    {
    static const struct clk_init_data qdss_init = {
    .ops = &qmp_qdss_clk_ops,
    .name = "qdss",
    };
    int ret;
    qmp.qdss_clk.init = &qdss_init;
    ret = clk_hw_register(qmp.dev, &qmp.qdss_clk);
    if (ret < 0) {
    dev_err(qmp.dev, "failed to register qdss clock\n");
    return ret;
    }
    ret = of_clk_add_hw_provider(qmp.dev.of_node, of_clk_hw_simple_get,
    &qmp.qdss_clk);
    if (ret < 0) {
    dev_err(qmp.dev, "unable to register of clk hw provider\n");
    clk_hw_unregister(&qmp.qdss_clk);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qmp_qdss_clk_remove(qmp: *mut qmp) {
    static void qmp_qdss_clk_remove(struct qmp *qmp)
    {
    of_clk_del_provider(qmp.dev.of_node);
    clk_hw_unregister(&qmp.qdss_clk);
    }
    static int qmp_cdev_get_max_state(struct thermal_cooling_device *cdev,
    unsigned long *state)
    {
// state = qmp_cdev_max_state;
    return 0;
    }
    static int qmp_cdev_get_cur_state(struct thermal_cooling_device *cdev,
    unsigned long *state)
    {
    struct qmp_cooling_device *qmp_cdev = cdev.devdata;
// state = qmp_cdev->state;
    return 0;
    }
    static int qmp_cdev_set_cur_state(struct thermal_cooling_device *cdev,
    unsigned long state)
    {
    struct qmp_cooling_device *qmp_cdev = cdev.devdata;
    bool cdev_state;
    int ret;
// Normalize state
    cdev_state = !!state;
    if (qmp_cdev.state == cdev_state)
    return 0;
    ret = qmp_send(qmp_cdev.qmp, "{class: volt_flr, event:zero_temp, res:%s, value:%s}",
    qmp_cdev.name, str_on_off(cdev_state));
    if (!ret)
    qmp_cdev.state = cdev_state;
    return ret;
    }
    static const struct thermal_cooling_device_ops qmp_cooling_device_ops = {
    .get_max_state = qmp_cdev_get_max_state,
    .get_cur_state = qmp_cdev_get_cur_state,
    .set_cur_state = qmp_cdev_set_cur_state,
    };
    static int qmp_cooling_device_add(struct qmp *qmp,
    struct qmp_cooling_device *qmp_cdev,
    struct device_node *node)
    {
    char *cdev_name = (char *)node.name;
    qmp_cdev.qmp = qmp;
    qmp_cdev.state = !qmp_cdev_max_state;
    qmp_cdev.name = cdev_name;
    qmp_cdev.cdev = devm_thermal_of_child_cooling_device_register
    (qmp.dev, node,
    cdev_name,
    qmp_cdev, &qmp_cooling_device_ops);
    if (IS_ERR(qmp_cdev.cdev))
    dev_err(qmp.dev, "unable to register %s cooling device\n",
    cdev_name);
    return PTR_ERR_OR_ZERO(qmp_cdev.cdev);
    }
#[no_mangle]
unsafe extern "C" fn qmp_cooling_devices_register(qmp: *mut qmp) -> c_int {
    static int qmp_cooling_devices_register(struct qmp *qmp)
    {
    struct device_node *np;
    let mut count: c_int = 0;
    int ret;
    np = qmp.dev.of_node;
    qmp.cooling_devs = devm_kcalloc(qmp.dev, QMP_NUM_COOLING_RESOURCES,
    sizeof(*qmp.cooling_devs),
    GFP_KERNEL);
    if (!qmp.cooling_devs)
    return -ENOMEM;
    for_each_available_child_of_node_scoped(np, child) {
    if (!of_property_present(child, "#cooling-cells"))
    continue;
    ret = qmp_cooling_device_add(qmp, &qmp.cooling_devs[count++],
    child);
    if (ret)
    goto unroll;
    }
    if (!count)
    devm_kfree(qmp.dev, qmp.cooling_devs);
    return 0;
    unroll:
    while (--count >= 0)
    thermal_cooling_device_unregister
    (qmp.cooling_devs[count].cdev);
    devm_kfree(qmp.dev, qmp.cooling_devs);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qmp_cooling_devices_remove(qmp: *mut qmp) {
    static void qmp_cooling_devices_remove(struct qmp *qmp)
    {
    int i;
    for (i = 0; i < QMP_NUM_COOLING_RESOURCES; i++)
    thermal_cooling_device_unregister(qmp.cooling_devs[i].cdev);
    }
//
// qmp_get() - get a qmp handle from a device
// @dev: client device pointer
//
// Return: handle to qmp device on success, ERR_PTR() on failure
//
    struct qmp *qmp_get(struct device *dev)
    {
    struct platform_device *pdev;
    struct device_node *np;
    struct qmp *qmp;
    if (!dev || !dev.of_node)
    return ERR_PTR(-EINVAL);
    np = of_parse_phandle(dev.of_node, "qcom,qmp", 0);
    if (!np)
    return ERR_PTR(-ENODEV);
    pdev = of_find_device_by_node(np);
    of_node_put(np);
    if (!pdev)
    return ERR_PTR(-EINVAL);
    qmp = platform_get_drvdata(pdev);
    if (!qmp) {
    put_device(&pdev.dev);
    return ERR_PTR(-EPROBE_DEFER);
    }
    return qmp;
    }
    EXPORT_SYMBOL_GPL(qmp_get);
//
// qmp_put() - release a qmp handle
// @qmp: qmp handle obtained from qmp_get()
//
#[no_mangle]
pub unsafe extern "C" fn qmp_put(qmp: *mut qmp) {
    void qmp_put(struct qmp *qmp)
    {
//
// Match get_device() inside of_find_device_by_node() in
// qmp_get()
//
    if (!IS_ERR_OR_NULL(qmp))
    put_device(qmp.dev);
    }
    EXPORT_SYMBOL_GPL(qmp_put);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmp_debugfs_entry {
    pub name: *const c_char,
    pub fmt: *const c_char,
    pub is_bool: bool,
    pub true_val: *const c_char,
    pub false_val: *const c_char,
}

    static const struct qmp_debugfs_entry qmp_debugfs_entries[QMP_DEBUGFS_FILES] = {
    { "ddr_frequency_mhz", "{class: ddr, res: fixed, val: %u}", false },
    { "prevent_aoss_sleep", "{class: aoss_slp, res: sleep: %s}", true, "enable", "disable" },
    { "prevent_cx_collapse", "{class: cx_mol, res: cx, val: %s}", true, "mol", "off" },
    { "prevent_ddr_collapse", "{class: ddr_mol, res: ddr, val: %s}", true, "mol", "off" },
    };
    static ssize_t qmp_debugfs_write(struct file *file, const char __user *user_buf,
    size_t count, loff_t *pos)
    {
    const struct qmp_debugfs_entry *entry = core::ptr::null_mut();
    struct qmp *qmp = file.private_data;
    char buf[QMP_MSG_LEN];
    unsigned int uint_val;
    const char *str_val;
    bool bool_val;
    int ret;
    int i;
    for (i = 0; i < ARRAY_SIZE(qmp.debugfs_files); i++) {
    if (qmp.debugfs_files[i] == file.f_path.dentry) {
    entry = &qmp_debugfs_entries[i];
    break;
    }
    }
    if (WARN_ON(!entry))
    return -EFAULT;
    if (entry.is_bool) {
    ret = kstrtobool_from_user(user_buf, count, &bool_val);
    if (ret)
    return ret;
    str_val = bool_val ? entry.true_val : entry.false_val;
    ret = snprintf(buf, sizeof(buf), entry.fmt, str_val);
    if (ret >= sizeof(buf))
    return -EINVAL;
    } else {
    ret = kstrtou32_from_user(user_buf, count, 0, &uint_val);
    if (ret)
    return ret;
    ret = snprintf(buf, sizeof(buf), entry.fmt, uint_val);
    if (ret >= sizeof(buf))
    return -EINVAL;
    }
    ret = qmp_send(qmp, buf);
    if (ret < 0)
    return ret;
    return count;
    }
    static const struct file_operations qmp_debugfs_fops = {
    .open = simple_open,
    .write = qmp_debugfs_write,
    };
#[no_mangle]
unsafe extern "C" fn qmp_debugfs_create(qmp: *mut qmp) {
    static void qmp_debugfs_create(struct qmp *qmp)
    {
    const struct qmp_debugfs_entry *entry;
    int i;
    qmp.debugfs_root = debugfs_create_dir("qcom_aoss", core::ptr::null_mut());
    for (i = 0; i < ARRAY_SIZE(qmp.debugfs_files); i++) {
    entry = &qmp_debugfs_entries[i];
    qmp.debugfs_files[i] = debugfs_create_file(entry.name, 0200,
    qmp.debugfs_root,
    qmp,
    &qmp_debugfs_fops);
    }
    }
#[no_mangle]
unsafe extern "C" fn qmp_probe(pdev: *mut platform_device) -> c_int {
    static int qmp_probe(struct platform_device *pdev)
    {
    struct qmp *qmp;
    int irq;
    int ret;
    qmp = devm_kzalloc(&pdev.dev, sizeof(*qmp), GFP_KERNEL);
    if (!qmp)
    return -ENOMEM;
    qmp.dev = &pdev.dev;
    init_waitqueue_head(&qmp.event);
    mutex_init(&qmp.tx_lock);
    qmp.msgram = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(qmp.msgram))
    return PTR_ERR(qmp.msgram);
    qmp.mbox_client.dev = &pdev.dev;
    qmp.mbox_client.knows_txdone = true;
    qmp.mbox_chan = mbox_request_channel(&qmp.mbox_client, 0);
    if (IS_ERR(qmp.mbox_chan)) {
    dev_err(&pdev.dev, "failed to acquire ipc mailbox\n");
    return PTR_ERR(qmp.mbox_chan);
    }
    irq = platform_get_irq(pdev, 0);
    ret = devm_request_irq(&pdev.dev, irq, qmp_intr, 0,
    "aoss-qmp", qmp);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to request interrupt\n");
    goto err_free_mbox;
    }
    ret = qmp_open(qmp);
    if (ret < 0)
    goto err_free_mbox;
    ret = qmp_qdss_clk_add(qmp);
    if (ret)
    goto err_close_qmp;
    ret = qmp_cooling_devices_register(qmp);
    if (ret)
    dev_err(&pdev.dev, "failed to register aoss cooling devices\n");
    platform_set_drvdata(pdev, qmp);
    qmp_debugfs_create(qmp);
    return 0;
    err_close_qmp:
    qmp_close(qmp);
    err_free_mbox:
    mbox_free_channel(qmp.mbox_chan);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qmp_remove(pdev: *mut platform_device) {
    static void qmp_remove(struct platform_device *pdev)
    {
    struct qmp *qmp = platform_get_drvdata(pdev);
    debugfs_remove_recursive(qmp.debugfs_root);
    qmp_qdss_clk_remove(qmp);
    qmp_cooling_devices_remove(qmp);
    qmp_close(qmp);
    mbox_free_channel(qmp.mbox_chan);
    }
    static const struct of_device_id qmp_dt_match[] = {
    { .compatible = "qcom,sc7180-aoss-qmp", },
    { .compatible = "qcom,sc7280-aoss-qmp", },
    { .compatible = "qcom,sdm845-aoss-qmp", },
    { .compatible = "qcom,sm8150-aoss-qmp", },
    { .compatible = "qcom,sm8250-aoss-qmp", },
    { .compatible = "qcom,sm8350-aoss-qmp", },
    { .compatible = "qcom,aoss-qmp", },
    {}
    };
    MODULE_DEVICE_TABLE(of, qmp_dt_match);
    static struct platform_driver qmp_driver = {
    .driver = {
    .name		= "qcom_aoss_qmp",
    .of_match_table	= qmp_dt_match,
    .suppress_bind_attrs = true,
    },
    .probe = qmp_probe,
    .remove = qmp_remove,
    };
    module_platform_driver(qmp_driver);
    MODULE_DESCRIPTION("Qualcomm AOSS QMP driver");
    MODULE_LICENSE("GPL v2");
