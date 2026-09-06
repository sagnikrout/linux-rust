//! Automatically rewritten from C to Rust
//! Source: drivers/hte/hte.c
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
// Copyright (c) 2021-2022 NVIDIA Corporation
//
// Author: Dipen Patel <dipenp@nvidia.com>
//

// Global list of the HTE devices
    static DEFINE_SPINLOCK(hte_lock);
    static LIST_HEAD(hte_devices);
    enum {
    HTE_TS_REGISTERED,
    HTE_TS_REQ,
    HTE_TS_DISABLE,
    HTE_TS_QUEUE_WK,
    };
//
// struct hte_ts_info - Information related to requested timestamp.
//
// @xlated_id: Timestamp ID as understood between HTE subsys and HTE provider,
// See xlate callback API.
// @flags: Flags holding state information.
// @hte_cb_flags: Callback related flags.
// @seq: Timestamp sequence counter.
// @line_name: HTE allocated line name.
// @free_attr_name: If set, free the attr name.
// @cb: A nonsleeping callback function provided by clients.
// @tcb: A secondary sleeping callback function provided by clients.
// @dropped_ts: Dropped timestamps.
// @slock: Spin lock to synchronize between disable/enable,
// request/release APIs.
// @cb_work: callback workqueue, used when tcb is specified.
// @req_mlock: Lock during timestamp request/release APIs.
// @ts_dbg_root: Root for the debug fs.
// @gdev: HTE abstract device that this timestamp information belongs to.
// @cl_data: Client specific data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hte_ts_info {
    pub xlated_id: u32,
    pub flags: c_ulong,
    pub hte_cb_flags: c_ulong,
    pub seq: u64,
    pub line_name: *mut c_char,
    pub free_attr_name: bool,
    pub cb: hte_ts_cb_t,
    pub tcb: hte_ts_sec_cb_t,
    pub dropped_ts: core::sync::atomic::AtomicI32,
    pub slock: spinlock_t,
    pub cb_work: work_struct,
    pub req_mlock: mutex,
    pub ts_dbg_root: *mut dentry,
    pub gdev: *mut hte_device,
    pub cl_data: *mut c_void,
}

//
// struct hte_device - HTE abstract device
// @nlines: Number of entities this device supports.
// @ts_req: Total number of entities requested.
// @sdev: Device used at various debug prints.
// @dbg_root: Root directory for debug fs.
// @list: List node to store hte_device for each provider.
// @chip: HTE chip providing this HTE device.
// @owner: helps prevent removal of modules when in use.
// @ei: Timestamp information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hte_device {
    pub nlines: u32,
    pub ts_req: core::sync::atomic::AtomicI32,
    pub sdev: *mut device,
    pub dbg_root: *mut dentry,
    pub list: list_head,
    pub chip: *mut hte_chip,
    pub owner: *mut module,
    pub __counted_by(nlines): hte_ts_info ei[],
}

    static struct dentry *hte_root;
#[no_mangle]
unsafe extern "C" fn hte_subsys_dbgfs_init() -> int __init {
    static int __init hte_subsys_dbgfs_init(void)
    {
// creates /sys/kernel/debug/hte/
    hte_root = debugfs_create_dir("hte", core::ptr::null_mut());
    return 0;
    }
    subsys_initcall(hte_subsys_dbgfs_init);
#[no_mangle]
unsafe extern "C" fn hte_chip_dbgfs_init(gdev: *mut hte_device) {
    static void hte_chip_dbgfs_init(struct hte_device *gdev)
    {
    const struct hte_chip *chip = gdev.chip;
    const char *name = chip.name ? chip.name : dev_name(chip.dev);
    gdev.dbg_root = debugfs_create_dir(name, hte_root);
    debugfs_create_atomic_t("ts_requested", 0444, gdev.dbg_root,
    &gdev.ts_req);
    debugfs_create_u32("total_ts", 0444, gdev.dbg_root,
    &gdev.nlines);
    }
#[no_mangle]
unsafe extern "C" fn hte_ts_dbgfs_init(name: *const c_char, ei: *mut hte_ts_info) {
    static void hte_ts_dbgfs_init(const char *name, struct hte_ts_info *ei)
    {
    if (!ei.gdev.dbg_root || !name)
    return;
    ei.ts_dbg_root = debugfs_create_dir(name, ei.gdev.dbg_root);
    debugfs_create_atomic_t("dropped_timestamps", 0444, ei.ts_dbg_root,
    &ei.dropped_ts);
    }

#[no_mangle]
unsafe extern "C" fn hte_chip_dbgfs_init(gdev: *mut hte_device) {
    static void hte_chip_dbgfs_init(struct hte_device *gdev)
    {
    }
#[no_mangle]
unsafe extern "C" fn hte_ts_dbgfs_init(name: *const c_char, ei: *mut hte_ts_info) {
    static void hte_ts_dbgfs_init(const char *name, struct hte_ts_info *ei)
    {
    }

//
// hte_ts_put() - Release and disable timestamp for the given desc.
//
// @desc: timestamp descriptor.
//
// Context: debugfs_remove_recursive() function call may use sleeping locks,
// not suitable from atomic context.
// Returns: 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn hte_ts_put(desc: *mut hte_ts_desc) -> c_int {
    int hte_ts_put(struct hte_ts_desc *desc)
    {
    let mut ret: c_int = 0;
    unsigned long flag;
    struct hte_device *gdev;
    struct hte_ts_info *ei;
    if (!desc)
    return -EINVAL;
    ei = desc.hte_data;
    if (!ei || !ei.gdev)
    return -EINVAL;
    gdev = ei.gdev;
    mutex_lock(&ei.req_mlock);
    if (unlikely(!test_bit(HTE_TS_REQ, &ei.flags) &&
    !test_bit(HTE_TS_REGISTERED, &ei.flags))) {
    dev_info(gdev.sdev, "id:%d is not requested\n",
    desc.attr.line_id);
    ret = -EINVAL;
    goto unlock;
    }
    if (unlikely(!test_bit(HTE_TS_REQ, &ei.flags) &&
    test_bit(HTE_TS_REGISTERED, &ei.flags))) {
    dev_info(gdev.sdev, "id:%d is registered but not requested\n",
    desc.attr.line_id);
    ret = -EINVAL;
    goto unlock;
    }
    if (test_bit(HTE_TS_REQ, &ei.flags) &&
    !test_bit(HTE_TS_REGISTERED, &ei.flags)) {
    clear_bit(HTE_TS_REQ, &ei.flags);
    desc.hte_data = core::ptr::null_mut();
    ret = 0;
    goto mod_put;
    }
    ret = gdev.chip.ops.release(gdev.chip, desc, ei.xlated_id);
    if (ret) {
    dev_err(gdev.sdev, "id: %d free failed\n",
    desc.attr.line_id);
    goto unlock;
    }
    kfree(ei.line_name);
    if (ei.free_attr_name)
    kfree_const(desc.attr.name);
    debugfs_remove_recursive(ei.ts_dbg_root);
    spin_lock_irqsave(&ei.slock, flag);
    if (test_bit(HTE_TS_QUEUE_WK, &ei.flags)) {
    spin_unlock_irqrestore(&ei.slock, flag);
    flush_work(&ei.cb_work);
    spin_lock_irqsave(&ei.slock, flag);
    }
    atomic_dec(&gdev.ts_req);
    atomic_set(&ei.dropped_ts, 0);
    ei.seq = 1;
    ei.flags = 0;
    desc.hte_data = core::ptr::null_mut();
    spin_unlock_irqrestore(&ei.slock, flag);
    ei.cb = core::ptr::null_mut();
    ei.tcb = core::ptr::null_mut();
    ei.cl_data = core::ptr::null_mut();
    mod_put:
    module_put(gdev.owner);
    unlock:
    mutex_unlock(&ei.req_mlock);
    dev_dbg(gdev.sdev, "release id: %d\n", desc.attr.line_id);
    return ret;
    }
    EXPORT_SYMBOL_GPL(hte_ts_put);
#[no_mangle]
unsafe extern "C" fn hte_ts_dis_en_common(desc: *mut hte_ts_desc, en: bool) -> c_int {
    static int hte_ts_dis_en_common(struct hte_ts_desc *desc, bool en)
    {
    u32 ts_id;
    struct hte_device *gdev;
    struct hte_ts_info *ei;
    int ret;
    unsigned long flag;
    if (!desc)
    return -EINVAL;
    ei = desc.hte_data;
    if (!ei || !ei.gdev)
    return -EINVAL;
    gdev = ei.gdev;
    ts_id = desc.attr.line_id;
    mutex_lock(&ei.req_mlock);
    if (!test_bit(HTE_TS_REGISTERED, &ei.flags)) {
    dev_dbg(gdev.sdev, "id:%d is not registered", ts_id);
    ret = -EUSERS;
    goto out;
    }
    spin_lock_irqsave(&ei.slock, flag);
    if (en) {
    if (!test_bit(HTE_TS_DISABLE, &ei.flags)) {
    ret = 0;
    goto out_unlock;
    }
    spin_unlock_irqrestore(&ei.slock, flag);
    ret = gdev.chip.ops.enable(gdev.chip, ei.xlated_id);
    if (ret) {
    dev_warn(gdev.sdev, "id: %d enable failed\n",
    ts_id);
    goto out;
    }
    spin_lock_irqsave(&ei.slock, flag);
    clear_bit(HTE_TS_DISABLE, &ei.flags);
    } else {
    if (test_bit(HTE_TS_DISABLE, &ei.flags)) {
    ret = 0;
    goto out_unlock;
    }
    spin_unlock_irqrestore(&ei.slock, flag);
    ret = gdev.chip.ops.disable(gdev.chip, ei.xlated_id);
    if (ret) {
    dev_warn(gdev.sdev, "id: %d disable failed\n",
    ts_id);
    goto out;
    }
    spin_lock_irqsave(&ei.slock, flag);
    set_bit(HTE_TS_DISABLE, &ei.flags);
    }
    out_unlock:
    spin_unlock_irqrestore(&ei.slock, flag);
    out:
    mutex_unlock(&ei.req_mlock);
    return ret;
    }
//
// hte_disable_ts() - Disable timestamp on given descriptor.
//
// The API does not release any resources associated with desc.
//
// @desc: ts descriptor, this is the same as returned by the request API.
//
// Context: Holds mutex lock, not suitable from atomic context.
// Returns: 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn hte_disable_ts(desc: *mut hte_ts_desc) -> c_int {
    int hte_disable_ts(struct hte_ts_desc *desc)
    {
    return hte_ts_dis_en_common(desc, false);
    }
    EXPORT_SYMBOL_GPL(hte_disable_ts);
//
// hte_enable_ts() - Enable timestamp on given descriptor.
//
// @desc: ts descriptor, this is the same as returned by the request API.
//
// Context: Holds mutex lock, not suitable from atomic context.
// Returns: 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn hte_enable_ts(desc: *mut hte_ts_desc) -> c_int {
    int hte_enable_ts(struct hte_ts_desc *desc)
    {
    return hte_ts_dis_en_common(desc, true);
    }
    EXPORT_SYMBOL_GPL(hte_enable_ts);
#[no_mangle]
unsafe extern "C" fn hte_do_cb_work(w: *mut work_struct) {
    static void hte_do_cb_work(struct work_struct *w)
    {
    unsigned long flag;
    struct hte_ts_info *ei = container_of(w, struct hte_ts_info, cb_work);
    if (unlikely(!ei.tcb))
    return;
    ei.tcb(ei.cl_data);
    spin_lock_irqsave(&ei.slock, flag);
    clear_bit(HTE_TS_QUEUE_WK, &ei.flags);
    spin_unlock_irqrestore(&ei.slock, flag);
    }
    static int __hte_req_ts(struct hte_ts_desc *desc, hte_ts_cb_t cb,
    hte_ts_sec_cb_t tcb, void *data)
    {
    int ret;
    struct hte_device *gdev;
    struct hte_ts_info *ei = desc.hte_data;
    gdev = ei.gdev;
//
// There is a chance that multiple consumers requesting same entity,
// lock here.
//
    mutex_lock(&ei.req_mlock);
    if (test_bit(HTE_TS_REGISTERED, &ei.flags) ||
    !test_bit(HTE_TS_REQ, &ei.flags)) {
    dev_dbg(gdev.chip.dev, "id:%u req failed\n",
    desc.attr.line_id);
    ret = -EUSERS;
    goto unlock;
    }
    ei.cb = cb;
    ei.tcb = tcb;
    if (tcb)
    INIT_WORK(&ei.cb_work, hte_do_cb_work);
    ret = gdev.chip.ops.request(gdev.chip, desc, ei.xlated_id);
    if (ret < 0) {
    dev_err(gdev.chip.dev, "ts request failed\n");
    goto unlock;
    }
    ei.cl_data = data;
    ei.seq = 1;
    atomic_inc(&gdev.ts_req);
    if (desc.attr.name)
    ei.line_name = core::ptr::null_mut();
    else
    ei.line_name = kasprintf(GFP_KERNEL, "ts_%u", desc.attr.line_id);
    hte_ts_dbgfs_init(desc.attr.name == core::ptr::null_mut() ?
    ei.line_name : desc.attr.name, ei);
    set_bit(HTE_TS_REGISTERED, &ei.flags);
    dev_dbg(gdev.chip.dev, "id: %u, xlated id:%u",
    desc.attr.line_id, ei.xlated_id);
    ret = 0;
    unlock:
    mutex_unlock(&ei.req_mlock);
    return ret;
    }
    static int hte_bind_ts_info_locked(struct hte_ts_info *ei,
    struct hte_ts_desc *desc, u32 x_id)
    {
    let mut ret: c_int = 0;
    mutex_lock(&ei.req_mlock);
    if (test_bit(HTE_TS_REQ, &ei.flags)) {
    dev_dbg(ei.gdev.chip.dev, "id:%u is already requested\n",
    desc.attr.line_id);
    ret = -EUSERS;
    goto out;
    }
    set_bit(HTE_TS_REQ, &ei.flags);
    desc.hte_data = ei;
    ei.xlated_id = x_id;
    out:
    mutex_unlock(&ei.req_mlock);
    return ret;
    }
    static struct hte_device *of_node_to_htedevice(struct device_node *np)
    {
    struct hte_device *gdev;
    spin_lock(&hte_lock);
    list_for_each_entry(gdev, &hte_devices, list)
    if (gdev.chip && gdev.chip.dev &&
    device_match_of_node(gdev.chip.dev, np)) {
    spin_unlock(&hte_lock);
    return gdev;
    }
    spin_unlock(&hte_lock);
    return ERR_PTR(-ENODEV);
    }
    static struct hte_device *hte_find_dev_from_linedata(struct hte_ts_desc *desc)
    {
    struct hte_device *gdev;
    spin_lock(&hte_lock);
    list_for_each_entry(gdev, &hte_devices, list)
    if (gdev.chip && gdev.chip.match_from_linedata) {
    if (!gdev.chip.match_from_linedata(gdev.chip, desc))
    continue;
    spin_unlock(&hte_lock);
    return gdev;
    }
    spin_unlock(&hte_lock);
    return ERR_PTR(-ENODEV);
    }
//
// of_hte_req_count - Return the number of entities to timestamp.
//
// The function returns the total count of the requested entities to timestamp
// by parsing device tree.
//
// @dev: The HTE consumer.
//
// Returns: Positive number on success, -ENOENT if no entries,
// -EINVAL for other errors.
//
#[no_mangle]
pub unsafe extern "C" fn of_hte_req_count(dev: *mut device) -> c_int {
    int of_hte_req_count(struct device *dev)
    {
    int count;
    if (!dev || !dev.of_node)
    return -EINVAL;
    count = of_count_phandle_with_args(dev.of_node, "timestamps",
    "#timestamp-cells");
    return count ? count : -ENOENT;
    }
    EXPORT_SYMBOL_GPL(of_hte_req_count);
    static inline struct hte_device *hte_get_dev(struct hte_ts_desc *desc)
    {
    return hte_find_dev_from_linedata(desc);
    }
    static struct hte_device *hte_of_get_dev(struct device *dev,
    struct hte_ts_desc *desc,
    int index,
    struct of_phandle_args *args,
    bool *free_name)
    {
    int ret;
    struct device_node *np;
    char *temp;
    if (!dev.of_node)
    return ERR_PTR(-EINVAL);
    np = dev.of_node;
    if (!of_property_present(np, "timestamp-names")) {
// Let hte core construct it during request time
    desc.attr.name = core::ptr::null_mut();
    } else {
    ret = of_property_read_string_index(np, "timestamp-names",
    index, &desc.attr.name);
    if (ret) {
    pr_err("can't parse \"timestamp-names\" property\n");
    return ERR_PTR(ret);
    }
// free_name = false;
    if (desc.attr.name) {
    temp = skip_spaces(desc.attr.name);
    if (!*temp)
    desc.attr.name = core::ptr::null_mut();
    }
    }
    ret = of_parse_phandle_with_args(np, "timestamps", "#timestamp-cells",
    index, args);
    if (ret) {
    pr_err("%s(): can't parse \"timestamps\" property\n",
    __func__);
    return ERR_PTR(ret);
    }
    of_node_put(args.np);
    return of_node_to_htedevice(args.np);
    }
//
// hte_ts_get() - The function to initialize and obtain HTE desc.
//
// The function initializes the consumer provided HTE descriptor. If consumer
// has device tree node, index is used to parse the line id and other details.
// The function needs to be called before using any request APIs.
//
// @dev: HTE consumer/client device, used in case of parsing device tree node.
// @desc: Pre-allocated timestamp descriptor.
// @index: The index will be used as an index to parse line_id from the
// device tree node if node is present.
//
// Context: Holds mutex lock.
// Returns: Returns 0 on success or negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn hte_ts_get(dev: *mut device, desc: *mut hte_ts_desc, index: c_int) -> c_int {
    int hte_ts_get(struct device *dev, struct hte_ts_desc *desc, int index)
    {
    struct hte_device *gdev;
    struct hte_ts_info *ei;
    const struct fwnode_handle *fwnode;
    struct of_phandle_args args;
    u32 xlated_id;
    int ret;
    let mut free_name: bool = false;
    if (!desc)
    return -EINVAL;
    fwnode = dev ? dev_fwnode(dev) : core::ptr::null_mut();
    if (is_of_node(fwnode))
    gdev = hte_of_get_dev(dev, desc, index, &args, &free_name);
    else
    gdev = hte_get_dev(desc);
    if (IS_ERR(gdev)) {
    pr_err("%s() no hte dev found\n", __func__);
    return PTR_ERR(gdev);
    }
    if (!try_module_get(gdev.owner))
    return -ENODEV;
    if (!gdev.chip) {
    pr_err("%s(): requested id does not have provider\n",
    __func__);
    ret = -ENODEV;
    goto put;
    }
    if (is_of_node(fwnode)) {
    if (!gdev.chip.xlate_of)
    ret = -EINVAL;
    else
    ret = gdev.chip.xlate_of(gdev.chip, &args,
    desc, &xlated_id);
    } else {
    if (!gdev.chip.xlate_plat)
    ret = -EINVAL;
    else
    ret = gdev.chip.xlate_plat(gdev.chip, desc,
    &xlated_id);
    }
    if (ret < 0)
    goto put;
    ei = &gdev.ei[xlated_id];
    ret = hte_bind_ts_info_locked(ei, desc, xlated_id);
    if (ret)
    goto put;
    ei.free_attr_name = free_name;
    return 0;
    put:
    module_put(gdev.owner);
    return ret;
    }
    EXPORT_SYMBOL_GPL(hte_ts_get);
#[no_mangle]
unsafe extern "C" fn __devm_hte_release_ts(res: *mut c_void) {
    static void __devm_hte_release_ts(void *res)
    {
    hte_ts_put(res);
    }
//
// hte_request_ts_ns() - The API to request and enable hardware timestamp in
// nanoseconds.
//
// The entity is provider specific for example, GPIO lines, signals, buses
// etc...The API allocates necessary resources and enables the timestamp.
//
// @desc: Pre-allocated and initialized timestamp descriptor.
// @cb: Callback to push the timestamp data to consumer.
// @tcb: Optional callback. If its provided, subsystem initializes
// workqueue. It is called when cb returns HTE_RUN_SECOND_CB.
// @data: Client data, used during cb and tcb callbacks.
//
// Context: Holds mutex lock.
// Returns: Returns 0 on success or negative error code on failure.
//
    int hte_request_ts_ns(struct hte_ts_desc *desc, hte_ts_cb_t cb,
    hte_ts_sec_cb_t tcb, void *data)
    {
    int ret;
    struct hte_ts_info *ei;
    if (!desc || !desc.hte_data || !cb)
    return -EINVAL;
    ei = desc.hte_data;
    if (!ei || !ei.gdev)
    return -EINVAL;
    ret = __hte_req_ts(desc, cb, tcb, data);
    if (ret < 0) {
    dev_err(ei.gdev.chip.dev,
    "failed to request id: %d\n", desc.attr.line_id);
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(hte_request_ts_ns);
//
// devm_hte_request_ts_ns() - Resource managed API to request and enable
// hardware timestamp in nanoseconds.
//
// The entity is provider specific for example, GPIO lines, signals, buses
// etc...The API allocates necessary resources and enables the timestamp. It
// deallocates and disables automatically when the consumer exits.
//
// @dev: HTE consumer/client device.
// @desc: Pre-allocated and initialized timestamp descriptor.
// @cb: Callback to push the timestamp data to consumer.
// @tcb: Optional callback. If its provided, subsystem initializes
// workqueue. It is called when cb returns HTE_RUN_SECOND_CB.
// @data: Client data, used during cb and tcb callbacks.
//
// Context: Holds mutex lock.
// Returns: Returns 0 on success or negative error code on failure.
//
    int devm_hte_request_ts_ns(struct device *dev, struct hte_ts_desc *desc,
    hte_ts_cb_t cb, hte_ts_sec_cb_t tcb,
    void *data)
    {
    int err;
    if (!dev)
    return -EINVAL;
    err = hte_request_ts_ns(desc, cb, tcb, data);
    if (err)
    return err;
    err = devm_add_action_or_reset(dev, __devm_hte_release_ts, desc);
    if (err)
    return err;
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_hte_request_ts_ns);
//
// hte_init_line_attr() - Initialize line attributes.
//
// Zeroes out line attributes and initializes with provided arguments.
// The function needs to be called before calling any consumer facing
// functions.
//
// @desc: Pre-allocated timestamp descriptor.
// @line_id: line id.
// @edge_flags: edge flags related to line_id.
// @name: name of the line.
// @data: line data related to line_id.
//
// Context: Any.
// Returns: 0 on success or negative error code for the failure.
//
    int hte_init_line_attr(struct hte_ts_desc *desc, u32 line_id,
    unsigned long edge_flags, const char *name, void *data)
    {
    if (!desc)
    return -EINVAL;
    memset(&desc.attr, 0, sizeof(desc.attr));
    desc.attr.edge_flags = edge_flags;
    desc.attr.line_id = line_id;
    desc.attr.line_data = data;
    if (name) {
    name =  kstrdup_const(name, GFP_KERNEL);
    if (!name)
    return -ENOMEM;
    }
    desc.attr.name = name;
    return 0;
    }
    EXPORT_SYMBOL_GPL(hte_init_line_attr);
//
// hte_get_clk_src_info() - Get the clock source information for a ts
// descriptor.
//
// @desc: ts descriptor, same as returned from request API.
// @ci: The API fills this structure with the clock information data.
//
// Context: Any context.
// Returns: 0 on success else negative error code on failure.
//
    int hte_get_clk_src_info(const struct hte_ts_desc *desc,
    struct hte_clk_info *ci)
    {
    struct hte_chip *chip;
    struct hte_ts_info *ei;
    if (!desc || !desc.hte_data || !ci) {
    pr_debug("%s:%d\n", __func__, __LINE__);
    return -EINVAL;
    }
    ei = desc.hte_data;
    if (!ei.gdev || !ei.gdev.chip)
    return -EINVAL;
    chip = ei.gdev.chip;
    if (!chip.ops.get_clk_src_info)
    return -EOPNOTSUPP;
    return chip.ops.get_clk_src_info(chip, ci);
    }
    EXPORT_SYMBOL_GPL(hte_get_clk_src_info);
//
// hte_push_ts_ns() - Push timestamp data in nanoseconds.
//
// It is used by the provider to push timestamp data.
//
// @chip: The HTE chip, used during the registration.
// @xlated_id: entity id understood by both subsystem and provider, this is
// obtained from xlate callback during request API.
// @data: timestamp data.
//
// Returns: 0 on success or a negative error code on failure.
//
    int hte_push_ts_ns(const struct hte_chip *chip, u32 xlated_id,
    struct hte_ts_data *data)
    {
    enum hte_return ret;
    let mut st: c_int = 0;
    struct hte_ts_info *ei;
    unsigned long flag;
    if (!chip || !data || !chip.gdev)
    return -EINVAL;
    if (xlated_id >= chip.nlines)
    return -EINVAL;
    ei = &chip.gdev.ei[xlated_id];
    spin_lock_irqsave(&ei.slock, flag);
// timestamp sequence counter
    data.seq = ei.seq++;
    if (!test_bit(HTE_TS_REGISTERED, &ei.flags) ||
    test_bit(HTE_TS_DISABLE, &ei.flags)) {
    dev_dbg(chip.dev, "Unknown timestamp push\n");
    atomic_inc(&ei.dropped_ts);
    st = -EINVAL;
    goto unlock;
    }
    ret = ei.cb(data, ei.cl_data);
    if (ret == HTE_RUN_SECOND_CB && ei.tcb) {
    queue_work(system_dfl_wq, &ei.cb_work);
    set_bit(HTE_TS_QUEUE_WK, &ei.flags);
    }
    unlock:
    spin_unlock_irqrestore(&ei.slock, flag);
    return st;
    }
    EXPORT_SYMBOL_GPL(hte_push_ts_ns);
#[no_mangle]
unsafe extern "C" fn hte_register_chip(chip: *mut hte_chip) -> c_int {
    static int hte_register_chip(struct hte_chip *chip)
    {
    struct hte_device *gdev;
    u32 i;
    if (!chip || !chip.dev || !chip.dev.of_node)
    return -EINVAL;
    if (!chip.ops || !chip.ops.request || !chip.ops.release) {
    dev_err(chip.dev, "Driver needs to provide ops\n");
    return -EINVAL;
    }
    gdev = kzalloc_flex(*gdev, ei, chip.nlines);
    if (!gdev)
    return -ENOMEM;
    gdev.chip = chip;
    chip.gdev = gdev;
    gdev.nlines = chip.nlines;
    gdev.sdev = chip.dev;
    for (i = 0; i < chip.nlines; i++) {
    gdev.ei[i].gdev = gdev;
    mutex_init(&gdev.ei[i].req_mlock);
    spin_lock_init(&gdev.ei[i].slock);
    }
    if (chip.dev.driver)
    gdev.owner = chip.dev.driver.owner;
    else
    gdev.owner = THIS_MODULE;
    of_node_get(chip.dev.of_node);
    INIT_LIST_HEAD(&gdev.list);
    spin_lock(&hte_lock);
    list_add_tail(&gdev.list, &hte_devices);
    spin_unlock(&hte_lock);
    hte_chip_dbgfs_init(gdev);
    dev_dbg(chip.dev, "Added hte chip\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hte_unregister_chip(chip: *mut hte_chip) -> c_int {
    static int hte_unregister_chip(struct hte_chip *chip)
    {
    struct hte_device *gdev;
    if (!chip)
    return -EINVAL;
    gdev = chip.gdev;
    spin_lock(&hte_lock);
    list_del(&gdev.list);
    spin_unlock(&hte_lock);
    gdev.chip = core::ptr::null_mut();
    of_node_put(chip.dev.of_node);
    debugfs_remove_recursive(gdev.dbg_root);
    kfree(gdev);
    dev_dbg(chip.dev, "Removed hte chip\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn _hte_devm_unregister_chip(chip: *mut c_void) {
    static void _hte_devm_unregister_chip(void *chip)
    {
    hte_unregister_chip(chip);
    }
//
// devm_hte_register_chip() - Resource managed API to register HTE chip.
//
// It is used by the provider to register itself with the HTE subsystem.
// The unregistration is done automatically when the provider exits.
//
// @chip: the HTE chip to add to subsystem.
//
// Returns: 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn devm_hte_register_chip(chip: *mut hte_chip) -> c_int {
    int devm_hte_register_chip(struct hte_chip *chip)
    {
    int err;
    err = hte_register_chip(chip);
    if (err)
    return err;
    err = devm_add_action_or_reset(chip.dev, _hte_devm_unregister_chip,
    chip);
    if (err)
    return err;
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_hte_register_chip);
