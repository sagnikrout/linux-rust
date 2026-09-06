//! Automatically rewritten from C to Rust
//! Source: drivers/pci/doe.c
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
// Data Object Exchange
// PCIe r6.0, sec 6.30 DOE
//
// Copyright (C) 2021 Huawei
// Jonathan Cameron <Jonathan.Cameron@huawei.com>
//
// Copyright (C) 2022 Intel Corporation
// Ira Weiny <ira.weiny@intel.com>
//

// Timeout of 1 second from 6.30.2 Operation, PCI Spec r6.0

pub const PCI_DOE_FLAG_CANCEL: c_int = 0;
pub const PCI_DOE_FLAG_DEAD: c_int = 1;
// Max data object length is 2^18 dwords

//
// struct pci_doe_mb - State for a single DOE mailbox
//
// This state is used to manage a single DOE mailbox capability.  All fields
// should be considered opaque to the consumers and the structure passed into
// the helpers below after being created by pci_doe_create_mb().
//
// @pdev: PCI device this mailbox belongs to
// @cap_offset: Capability offset
// @feats: Array of features supported (encoded as long values)
// @wq: Wait queue for work item
// @work_queue: Queue of pci_doe_work items
// @flags: Bit array of PCI_DOE_FLAG_* flags
// @sysfs_attrs: Array of sysfs device attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_doe_mb {
    pub pdev: *mut pci_dev,
    pub cap_offset: u16,
    pub feats: xarray,
    pub wq: wait_queue_head_t,
    pub work_queue: *mut workqueue_struct,
    pub flags: c_ulong,

    pub sysfs_attrs: *mut device_attribute,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_doe_feature {
    pub vid: u16,
    pub type: u8,
}

//
// struct pci_doe_task - represents a single query/response
//
// @feat: DOE Feature
// @request_pl: The request payload
// @request_pl_sz: Size of the request payload (bytes)
// @response_pl: The response payload
// @response_pl_sz: Size of the response payload (bytes)
// @rv: Return value.  Length of received response or error (bytes)
// @complete: Called when task is complete
// @private: Private data for the consumer
// @work: Used internally by the mailbox
// @doe_mb: Used internally by the mailbox
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_doe_task {
    pub feat: pci_doe_feature,
    pub request_pl: *const __le32,
    pub request_pl_sz: usize,
    pub response_pl: *mut __le32,
    pub response_pl_sz: usize,
    pub rv: c_int,
    pub task): *mut *mut void (complete)(struct pci_doe_task,
    pub private: *mut c_void,
// initialized by pci_doe_submit_task()
    pub work: work_struct,
    pub doe_mb: *mut pci_doe_mb,
}

    static ssize_t doe_discovery_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "0001:00\n");
    }
    static DEVICE_ATTR_RO(doe_discovery);
    static struct attribute *pci_doe_sysfs_feature_attrs[] = {
    &dev_attr_doe_discovery.attr,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn pci_doe_features_sysfs_group_visible(kobj: *mut kobject) -> bool {
    static bool pci_doe_features_sysfs_group_visible(struct kobject *kobj)
    {
    struct pci_dev *pdev = to_pci_dev(kobj_to_dev(kobj));
    return !xa_empty(&pdev.doe_mbs);
    }
    DEFINE_SIMPLE_SYSFS_GROUP_VISIBLE(pci_doe_features_sysfs)
    const struct attribute_group pci_doe_sysfs_group = {
    .name	    = "doe_features",
    .attrs	    = pci_doe_sysfs_feature_attrs,
    .is_visible = SYSFS_GROUP_VISIBLE(pci_doe_features_sysfs),
    };
    static ssize_t pci_doe_sysfs_feature_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%s\n", attr.attr.name);
    }
    static void pci_doe_sysfs_feature_remove(struct pci_dev *pdev,
    struct pci_doe_mb *doe_mb)
    {
    struct device_attribute *attrs = doe_mb.sysfs_attrs;
    struct device *dev = &pdev.dev;
    unsigned long i;
    void *entry;
    if (!attrs)
    return;
    doe_mb.sysfs_attrs = core::ptr::null_mut();
    xa_for_each(&doe_mb.feats, i, entry) {
    if (attrs[i].show)
    sysfs_remove_file_from_group(&dev.kobj, &attrs[i].attr,
    pci_doe_sysfs_group.name);
    kfree(attrs[i].attr.name);
    }
    kfree(attrs);
    }
    static int pci_doe_sysfs_feature_populate(struct pci_dev *pdev,
    struct pci_doe_mb *doe_mb)
    {
    struct device *dev = &pdev.dev;
    struct device_attribute *attrs;
    let mut num_features: c_ulong = 0;
    unsigned long vid, type;
    unsigned long i;
    void *entry;
    int ret;
    xa_for_each(&doe_mb.feats, i, entry)
    num_features++;
    attrs = kzalloc_objs(*attrs, num_features);
    if (!attrs) {
    pci_warn(pdev, "Failed allocating the device_attribute array\n");
    return -ENOMEM;
    }
    doe_mb.sysfs_attrs = attrs;
    xa_for_each(&doe_mb.feats, i, entry) {
    sysfs_attr_init(&attrs[i].attr);
    vid = xa_to_value(entry) >> 8;
    type = xa_to_value(entry) & 0xFF;
    if (vid == PCI_VENDOR_ID_PCI_SIG &&
    type == PCI_DOE_FEATURE_DISCOVERY) {
//
// DOE Discovery, manually displayed by
// `dev_attr_doe_discovery`
//
    continue;
    }
    attrs[i].attr.name = kasprintf(GFP_KERNEL,
    "%04lx:%02lx", vid, type);
    if (!attrs[i].attr.name) {
    ret = -ENOMEM;
    pci_warn(pdev, "Failed allocating the attribute name\n");
    goto fail;
    }
    attrs[i].attr.mode = 0444;
    attrs[i].show = pci_doe_sysfs_feature_show;
    ret = sysfs_add_file_to_group(&dev.kobj, &attrs[i].attr,
    pci_doe_sysfs_group.name);
    if (ret) {
    attrs[i].show = core::ptr::null_mut();
    if (ret != -EEXIST) {
    pci_warn(pdev, "Failed adding %s to sysfs group\n",
    attrs[i].attr.name);
    goto fail;
    } else
    kfree(attrs[i].attr.name);
    }
    }
    return 0;
    fail:
    pci_doe_sysfs_feature_remove(pdev, doe_mb);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_doe_sysfs_teardown(pdev: *mut pci_dev) {
    void pci_doe_sysfs_teardown(struct pci_dev *pdev)
    {
    struct pci_doe_mb *doe_mb;
    unsigned long index;
    xa_for_each(&pdev.doe_mbs, index, doe_mb)
    pci_doe_sysfs_feature_remove(pdev, doe_mb);
    }
#[no_mangle]
pub unsafe extern "C" fn pci_doe_sysfs_init(pdev: *mut pci_dev) {
    void pci_doe_sysfs_init(struct pci_dev *pdev)
    {
    struct pci_doe_mb *doe_mb;
    unsigned long index;
    int ret;
    xa_for_each(&pdev.doe_mbs, index, doe_mb) {
    ret = pci_doe_sysfs_feature_populate(pdev, doe_mb);
    if (ret)
    return;
    }
    }

#[no_mangle]
unsafe extern "C" fn pci_doe_wait(doe_mb: *mut pci_doe_mb, timeout: c_ulong) -> c_int {
    static int pci_doe_wait(struct pci_doe_mb *doe_mb, unsigned long timeout)
    {
    if (wait_event_timeout(doe_mb.wq,
    test_bit(PCI_DOE_FLAG_CANCEL, &doe_mb.flags),
    timeout))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_doe_write_ctrl(doe_mb: *mut pci_doe_mb, val: u32) {
    static void pci_doe_write_ctrl(struct pci_doe_mb *doe_mb, u32 val)
    {
    struct pci_dev *pdev = doe_mb.pdev;
    let mut offset: c_int = doe_mb.cap_offset;
    pci_write_config_dword(pdev, offset + PCI_DOE_CTRL, val);
    }
#[no_mangle]
unsafe extern "C" fn pci_doe_abort(doe_mb: *mut pci_doe_mb) -> c_int {
    static int pci_doe_abort(struct pci_doe_mb *doe_mb)
    {
    struct pci_dev *pdev = doe_mb.pdev;
    let mut offset: c_int = doe_mb.cap_offset;
    unsigned long timeout_jiffies;
    pci_dbg(pdev, "[%x] Issuing Abort\n", offset);
    timeout_jiffies = jiffies + PCI_DOE_TIMEOUT;
    pci_doe_write_ctrl(doe_mb, PCI_DOE_CTRL_ABORT);
    do {
    int rc;
    u32 val;
    rc = pci_doe_wait(doe_mb, PCI_DOE_POLL_INTERVAL);
    if (rc)
    return rc;
    pci_read_config_dword(pdev, offset + PCI_DOE_STATUS, &val);
// Abort success!
    if (!FIELD_GET(PCI_DOE_STATUS_ERROR, val) &&
    !FIELD_GET(PCI_DOE_STATUS_BUSY, val))
    return 0;
    } while (!time_after(jiffies, timeout_jiffies));
// Abort has timed out and the MB is dead
    pci_err(pdev, "[%x] ABORT timed out\n", offset);
    return -EIO;
    }
    static int pci_doe_send_req(struct pci_doe_mb *doe_mb,
    struct pci_doe_task *task)
    {
    struct pci_dev *pdev = doe_mb.pdev;
    let mut offset: c_int = doe_mb.cap_offset;
    unsigned long timeout_jiffies;
    size_t length, remainder;
    u32 val;
    int i;
//
// Check the DOE busy bit is not set. If it is set, this could indicate
// someone other than Linux (e.g. firmware) is using the mailbox. Note
// it is expected that firmware and OS will negotiate access rights via
// an, as yet to be defined, method.
//
// Wait up to one PCI_DOE_TIMEOUT period to allow the prior command to
// finish. Otherwise, simply error out as unable to field the request.
//
// PCIe r6.2 sec 6.30.3 states no interrupt is raised when the DOE Busy
// bit is cleared, so polling here is our best option for the moment.
//
    timeout_jiffies = jiffies + PCI_DOE_TIMEOUT;
    do {
    pci_read_config_dword(pdev, offset + PCI_DOE_STATUS, &val);
    } while (FIELD_GET(PCI_DOE_STATUS_BUSY, val) &&
    !time_after(jiffies, timeout_jiffies));
    if (FIELD_GET(PCI_DOE_STATUS_BUSY, val))
    return -EBUSY;
    if (FIELD_GET(PCI_DOE_STATUS_ERROR, val))
    return -EIO;
// Length is 2 DW of header + length of payload in DW
    length = 2 + DIV_ROUND_UP(task.request_pl_sz, sizeof(__le32));
    if (length > PCI_DOE_MAX_LENGTH)
    return -EIO;
    if (length == PCI_DOE_MAX_LENGTH)
    length = 0;
// Write DOE Header
    val = FIELD_PREP(PCI_DOE_DATA_OBJECT_HEADER_1_VID, task.feat.vid) |
    FIELD_PREP(PCI_DOE_DATA_OBJECT_HEADER_1_TYPE, task.feat.type);
    pci_write_config_dword(pdev, offset + PCI_DOE_WRITE, val);
    pci_write_config_dword(pdev, offset + PCI_DOE_WRITE,
    FIELD_PREP(PCI_DOE_DATA_OBJECT_HEADER_2_LENGTH,
    length));
// Write payload
    for (i = 0; i < task.request_pl_sz / sizeof(__le32); i++)
    pci_write_config_dword(pdev, offset + PCI_DOE_WRITE,
    le32_to_cpu(task.request_pl[i]));
// Write last payload dword
    remainder = task.request_pl_sz % sizeof(__le32);
    if (remainder) {
    val = 0;
    memcpy(&val, &task.request_pl[i], remainder);
    le32_to_cpus(&val);
    pci_write_config_dword(pdev, offset + PCI_DOE_WRITE, val);
    }
    pci_doe_write_ctrl(doe_mb, PCI_DOE_CTRL_GO);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_doe_data_obj_ready(doe_mb: *mut pci_doe_mb) -> bool {
    static bool pci_doe_data_obj_ready(struct pci_doe_mb *doe_mb)
    {
    struct pci_dev *pdev = doe_mb.pdev;
    let mut offset: c_int = doe_mb.cap_offset;
    u32 val;
    pci_read_config_dword(pdev, offset + PCI_DOE_STATUS, &val);
    if (FIELD_GET(PCI_DOE_STATUS_DATA_OBJECT_READY, val))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pci_doe_recv_resp(doe_mb: *mut pci_doe_mb, task: *mut pci_doe_task) -> c_int {
    static int pci_doe_recv_resp(struct pci_doe_mb *doe_mb, struct pci_doe_task *task)
    {
    size_t length, payload_length, remainder, received;
    struct pci_dev *pdev = doe_mb.pdev;
    let mut offset: c_int = doe_mb.cap_offset;
    let mut i: c_int = 0;
    u32 val;
// Read the first dword to get the feature
    pci_read_config_dword(pdev, offset + PCI_DOE_READ, &val);
    if ((FIELD_GET(PCI_DOE_DATA_OBJECT_HEADER_1_VID, val) != task.feat.vid) ||
    (FIELD_GET(PCI_DOE_DATA_OBJECT_HEADER_1_TYPE, val) != task.feat.type)) {
    dev_err_ratelimited(&pdev.dev, "[%x] expected [VID, Feature] = [%04x, %02x], got [%04x, %02x]\n",
    doe_mb.cap_offset, task.feat.vid, task.feat.type,
    FIELD_GET(PCI_DOE_DATA_OBJECT_HEADER_1_VID, val),
    FIELD_GET(PCI_DOE_DATA_OBJECT_HEADER_1_TYPE, val));
    return -EIO;
    }
    pci_write_config_dword(pdev, offset + PCI_DOE_READ, 0);
// Read the second dword to get the length
    pci_read_config_dword(pdev, offset + PCI_DOE_READ, &val);
    pci_write_config_dword(pdev, offset + PCI_DOE_READ, 0);
    length = FIELD_GET(PCI_DOE_DATA_OBJECT_HEADER_2_LENGTH, val);
// A value of 0x0 indicates max data object length
    if (!length)
    length = PCI_DOE_MAX_LENGTH;
    if (length < 2)
    return -EIO;
// First 2 dwords have already been read
    length -= 2;
    received = task.response_pl_sz;
    payload_length = DIV_ROUND_UP(task.response_pl_sz, sizeof(__le32));
    remainder = task.response_pl_sz % sizeof(__le32);
// remainder signifies number of data bytes in last payload dword
    if (!remainder)
    remainder = sizeof(__le32);
    if (length < payload_length) {
    received = length * sizeof(__le32);
    payload_length = length;
    remainder = sizeof(__le32);
    }
    if (payload_length) {
// Read all payload dwords except the last
    for (; i < payload_length - 1; i++) {
    pci_read_config_dword(pdev, offset + PCI_DOE_READ,
    &val);
    task.response_pl[i] = cpu_to_le32(val);
    pci_write_config_dword(pdev, offset + PCI_DOE_READ, 0);
    }
// Read last payload dword
    pci_read_config_dword(pdev, offset + PCI_DOE_READ, &val);
    cpu_to_le32s(&val);
    memcpy(&task.response_pl[i], &val, remainder);
// Prior to the last ack, ensure Data Object Ready
    if (!pci_doe_data_obj_ready(doe_mb))
    return -EIO;
    pci_write_config_dword(pdev, offset + PCI_DOE_READ, 0);
    i++;
    }
// Flush excess length
    for (; i < length; i++) {
    pci_read_config_dword(pdev, offset + PCI_DOE_READ, &val);
    pci_write_config_dword(pdev, offset + PCI_DOE_READ, 0);
    }
// Final error check to pick up on any since Data Object Ready
    pci_read_config_dword(pdev, offset + PCI_DOE_STATUS, &val);
    if (FIELD_GET(PCI_DOE_STATUS_ERROR, val))
    return -EIO;
    return received;
    }
#[no_mangle]
unsafe extern "C" fn signal_task_complete(task: *mut pci_doe_task, rv: c_int) {
    static void signal_task_complete(struct pci_doe_task *task, int rv)
    {
    task.rv = rv;
    destroy_work_on_stack(&task.work);
    task.complete(task);
    }
#[no_mangle]
unsafe extern "C" fn signal_task_abort(task: *mut pci_doe_task, rv: c_int) {
    static void signal_task_abort(struct pci_doe_task *task, int rv)
    {
    struct pci_doe_mb *doe_mb = task.doe_mb;
    struct pci_dev *pdev = doe_mb.pdev;
    if (pci_doe_abort(doe_mb)) {
//
// If the device can't process an abort; set the mailbox dead
// - no more submissions
//
    pci_err(pdev, "[%x] Abort failed marking mailbox dead\n",
    doe_mb.cap_offset);
    set_bit(PCI_DOE_FLAG_DEAD, &doe_mb.flags);
    }
    signal_task_complete(task, rv);
    }
#[no_mangle]
unsafe extern "C" fn doe_statemachine_work(work: *mut work_struct) {
    static void doe_statemachine_work(struct work_struct *work)
    {
    struct pci_doe_task *task = container_of(work, struct pci_doe_task,
    work);
    struct pci_doe_mb *doe_mb = task.doe_mb;
    struct pci_dev *pdev = doe_mb.pdev;
    let mut offset: c_int = doe_mb.cap_offset;
    unsigned long timeout_jiffies;
    u32 val;
    int rc;
    if (test_bit(PCI_DOE_FLAG_DEAD, &doe_mb.flags)) {
    signal_task_complete(task, -EIO);
    return;
    }
// Send request
    rc = pci_doe_send_req(doe_mb, task);
    if (rc) {
//
// The specification does not provide any guidance on how to
// resolve conflicting requests from other entities.
// Furthermore, it is likely that busy will not be detected
// most of the time.  Flag any detection of status busy with an
// error.
//
    if (rc == -EBUSY)
    dev_err_ratelimited(&pdev.dev, "[%x] busy detected; another entity is sending conflicting requests\n",
    offset);
    signal_task_abort(task, rc);
    return;
    }
    timeout_jiffies = jiffies + PCI_DOE_TIMEOUT;
// Poll for response
    retry_resp:
    pci_read_config_dword(pdev, offset + PCI_DOE_STATUS, &val);
    if (FIELD_GET(PCI_DOE_STATUS_ERROR, val)) {
    signal_task_abort(task, -EIO);
    return;
    }
    if (!FIELD_GET(PCI_DOE_STATUS_DATA_OBJECT_READY, val)) {
    if (time_after(jiffies, timeout_jiffies)) {
    signal_task_abort(task, -EIO);
    return;
    }
    rc = pci_doe_wait(doe_mb, PCI_DOE_POLL_INTERVAL);
    if (rc) {
    signal_task_abort(task, rc);
    return;
    }
    goto retry_resp;
    }
    rc  = pci_doe_recv_resp(doe_mb, task);
    if (rc < 0) {
    signal_task_abort(task, rc);
    return;
    }
    signal_task_complete(task, rc);
    }
#[no_mangle]
unsafe extern "C" fn pci_doe_task_complete(task: *mut pci_doe_task) {
    static void pci_doe_task_complete(struct pci_doe_task *task)
    {
    complete(task.private);
    }
    static int pci_doe_discovery(struct pci_doe_mb *doe_mb, u8 capver, u8 *index, u16 *vid,
    u8 *feature)
    {
    u32 request_pl = FIELD_PREP(PCI_DOE_DATA_OBJECT_DISC_REQ_3_INDEX,
// index) |
    FIELD_PREP(PCI_DOE_DATA_OBJECT_DISC_REQ_3_VER,
    (capver >= 2) ? 2 : 0);
    let mut request_pl_le: __le32 = cpu_to_le32(request_pl);
    __le32 response_pl_le;
    u32 response_pl;
    int rc;
    rc = pci_doe(doe_mb, PCI_VENDOR_ID_PCI_SIG, PCI_DOE_FEATURE_DISCOVERY,
    &request_pl_le, sizeof(request_pl_le),
    &response_pl_le, sizeof(response_pl_le));
    if (rc < 0)
    return rc;
    if (rc != sizeof(response_pl_le))
    return -EIO;
    response_pl = le32_to_cpu(response_pl_le);
// vid = FIELD_GET(PCI_DOE_DATA_OBJECT_DISC_RSP_3_VID, response_pl);
// feature = FIELD_GET(PCI_DOE_DATA_OBJECT_DISC_RSP_3_TYPE,
    response_pl);
// index = FIELD_GET(PCI_DOE_DATA_OBJECT_DISC_RSP_3_NEXT_INDEX,
    response_pl);
    return 0;
    }
    static void *pci_doe_xa_feat_entry(u16 vid, u8 type)
    {
    return xa_mk_value((vid << 8) | type);
    }
#[no_mangle]
unsafe extern "C" fn pci_doe_cache_features(doe_mb: *mut pci_doe_mb) -> c_int {
    static int pci_doe_cache_features(struct pci_doe_mb *doe_mb)
    {
    let mut index: u8 = 0;
    let mut xa_idx: u8 = 0;
    let mut hdr: u32 = 0;
    pci_read_config_dword(doe_mb.pdev, doe_mb.cap_offset, &hdr);
    do {
    int rc;
    u16 vid;
    u8 type;
    rc = pci_doe_discovery(doe_mb, PCI_EXT_CAP_VER(hdr), &index,
    &vid, &type);
    if (rc)
    return rc;
    pci_dbg(doe_mb.pdev,
    "[%x] Found feature %d vid: %x type: %x\n",
    doe_mb.cap_offset, xa_idx, vid, type);
    rc = xa_insert(&doe_mb.feats, xa_idx++,
    pci_doe_xa_feat_entry(vid, type), GFP_KERNEL);
    if (rc)
    return rc;
    } while (index);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_doe_cancel_tasks(doe_mb: *mut pci_doe_mb) {
    static void pci_doe_cancel_tasks(struct pci_doe_mb *doe_mb)
    {
// Stop all pending work items from starting
    set_bit(PCI_DOE_FLAG_DEAD, &doe_mb.flags);
// Cancel an in progress work item, if necessary
    set_bit(PCI_DOE_FLAG_CANCEL, &doe_mb.flags);
    wake_up(&doe_mb.wq);
    }
//
// pci_doe_create_mb() - Create a DOE mailbox object
//
// @pdev: PCI device to create the DOE mailbox for
// @cap_offset: Offset of the DOE mailbox
//
// Create a single mailbox object to manage the mailbox feature at the
// cap_offset specified.
//
// RETURNS: created mailbox object on success
// ERR_PTR(-errno) on failure
//
    static struct pci_doe_mb *pci_doe_create_mb(struct pci_dev *pdev,
    u16 cap_offset)
    {
    struct pci_doe_mb *doe_mb;
    int rc;
    doe_mb = kzalloc_obj(*doe_mb);
    if (!doe_mb)
    return ERR_PTR(-ENOMEM);
    doe_mb.pdev = pdev;
    doe_mb.cap_offset = cap_offset;
    init_waitqueue_head(&doe_mb.wq);
    xa_init(&doe_mb.feats);
    doe_mb.work_queue = alloc_ordered_workqueue("%s %s DOE [%x]", 0,
    dev_bus_name(&pdev.dev),
    pci_name(pdev),
    doe_mb.cap_offset);
    if (!doe_mb.work_queue) {
    pci_err(pdev, "[%x] failed to allocate work queue\n",
    doe_mb.cap_offset);
    rc = -ENOMEM;
    goto err_free;
    }
// Reset the mailbox by issuing an abort
    rc = pci_doe_abort(doe_mb);
    if (rc) {
    pci_err(pdev, "[%x] failed to reset mailbox with abort command : %d\n",
    doe_mb.cap_offset, rc);
    goto err_destroy_wq;
    }
//
// The state machine and the mailbox should be in sync now;
// Use the mailbox to query features.
//
    rc = pci_doe_cache_features(doe_mb);
    if (rc) {
    pci_err(pdev, "[%x] failed to cache features : %d\n",
    doe_mb.cap_offset, rc);
    goto err_cancel;
    }
    return doe_mb;
    err_cancel:
    pci_doe_cancel_tasks(doe_mb);
    xa_destroy(&doe_mb.feats);
    err_destroy_wq:
    destroy_workqueue(doe_mb.work_queue);
    err_free:
    kfree(doe_mb);
    return ERR_PTR(rc);
    }
//
// pci_doe_destroy_mb() - Destroy a DOE mailbox object
//
// @doe_mb: DOE mailbox
//
// Destroy all internal data structures created for the DOE mailbox.
//
#[no_mangle]
unsafe extern "C" fn pci_doe_destroy_mb(doe_mb: *mut pci_doe_mb) {
    static void pci_doe_destroy_mb(struct pci_doe_mb *doe_mb)
    {
    pci_doe_cancel_tasks(doe_mb);
    xa_destroy(&doe_mb.feats);
    destroy_workqueue(doe_mb.work_queue);
    kfree(doe_mb);
    }
//
// pci_doe_supports_feat() - Return if the DOE instance supports the given
// feature
// @doe_mb: DOE mailbox capability to query
// @vid: Feature Vendor ID
// @type: Feature type
//
// RETURNS: True if the DOE mailbox supports the feature specified
//
#[no_mangle]
unsafe extern "C" fn pci_doe_supports_feat(doe_mb: *mut pci_doe_mb, vid: u16, type: u8) -> bool {
    static bool pci_doe_supports_feat(struct pci_doe_mb *doe_mb, u16 vid, u8 type)
    {
    unsigned long index;
    void *entry;
// The discovery feature must always be supported
    if (vid == PCI_VENDOR_ID_PCI_SIG && type == PCI_DOE_FEATURE_DISCOVERY)
    return true;
    xa_for_each(&doe_mb.feats, index, entry)
    if (entry == pci_doe_xa_feat_entry(vid, type))
    return true;
    return false;
    }
//
// pci_doe_submit_task() - Submit a task to be processed by the state machine
//
// @doe_mb: DOE mailbox capability to submit to
// @task: task to be queued
//
// Submit a DOE task (request/response) to the DOE mailbox to be processed.
// Returns upon queueing the task object.  If the queue is full this function
// will sleep until there is room in the queue.
//
// task->complete will be called when the state machine is done processing this
// task.
//
// @task must be allocated on the stack.
//
// Excess data will be discarded.
//
// RETURNS: 0 when task has been successfully queued, -ERRNO on error
//
    static int pci_doe_submit_task(struct pci_doe_mb *doe_mb,
    struct pci_doe_task *task)
    {
    if (!pci_doe_supports_feat(doe_mb, task.feat.vid, task.feat.type))
    return -EINVAL;
    if (test_bit(PCI_DOE_FLAG_DEAD, &doe_mb.flags))
    return -EIO;
    task.doe_mb = doe_mb;
    INIT_WORK_ONSTACK(&task.work, doe_statemachine_work);
    queue_work(doe_mb.work_queue, &task.work);
    return 0;
    }
//
// pci_doe() - Perform Data Object Exchange
//
// @doe_mb: DOE Mailbox
// @vendor: Vendor ID
// @type: Data Object Type
// @request: Request payload
// @request_sz: Size of request payload (bytes)
// @response: Response payload
// @response_sz: Size of response payload (bytes)
//
// Submit @request to @doe_mb and store the @response.
// The DOE exchange is performed synchronously and may therefore sleep.
//
// Payloads are treated as opaque byte streams which are transmitted verbatim,
// without byte-swapping.  If payloads contain little-endian register values,
// the caller is responsible for conversion with cpu_to_le32() / le32_to_cpu().
//
// For convenience, arbitrary payload sizes are allowed even though PCIe r6.0
// sec 6.30.1 specifies the Data Object Header 2 "Length" in dwords.  The last
// (partial) dword is copied with byte granularity and padded with zeroes if
// necessary.  Callers are thus relieved of using dword-sized bounce buffers.
//
// RETURNS: Length of received response or negative errno.
// Received data in excess of @response_sz is discarded.
// The length may be smaller than @response_sz and the caller
// is responsible for checking that.
//
    int pci_doe(struct pci_doe_mb *doe_mb, u16 vendor, u8 type,
    const void *request, size_t request_sz,
    void *response, size_t response_sz)
    {
    DECLARE_COMPLETION_ONSTACK(c);
    struct pci_doe_task task = {
    .feat.vid = vendor,
    .feat.type = type,
    .request_pl = request,
    .request_pl_sz = request_sz,
    .response_pl = response,
    .response_pl_sz = response_sz,
    .complete = pci_doe_task_complete,
    .private = &c,
    };
    int rc;
    rc = pci_doe_submit_task(doe_mb, &task);
    if (rc)
    return rc;
    wait_for_completion(&c);
    return task.rv;
    }
    EXPORT_SYMBOL_GPL(pci_doe);
//
// pci_find_doe_mailbox() - Find Data Object Exchange mailbox
//
// @pdev: PCI device
// @vendor: Vendor ID
// @type: Data Object Type
//
// Find first DOE mailbox of a PCI device which supports the given feature.
//
// RETURNS: Pointer to the DOE mailbox or NULL if none was found.
//
    struct pci_doe_mb *pci_find_doe_mailbox(struct pci_dev *pdev, u16 vendor,
    u8 type)
    {
    struct pci_doe_mb *doe_mb;
    unsigned long index;
    xa_for_each(&pdev.doe_mbs, index, doe_mb)
    if (pci_doe_supports_feat(doe_mb, vendor, type))
    return doe_mb;
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(pci_find_doe_mailbox);
#[no_mangle]
pub unsafe extern "C" fn pci_doe_init(pdev: *mut pci_dev) {
    void pci_doe_init(struct pci_dev *pdev)
    {
    struct pci_doe_mb *doe_mb;
    let mut offset: u16 = 0;
    int rc;
    xa_init(&pdev.doe_mbs);
    while ((offset = pci_find_next_ext_capability(pdev, offset,
    PCI_EXT_CAP_ID_DOE))) {
    doe_mb = pci_doe_create_mb(pdev, offset);
    if (IS_ERR(doe_mb)) {
    pci_err(pdev, "[%x] failed to create mailbox: %pe\n",
    offset, doe_mb);
    continue;
    }
    rc = xa_insert(&pdev.doe_mbs, offset, doe_mb, GFP_KERNEL);
    if (rc) {
    pci_err(pdev, "[%x] failed to insert mailbox: %d\n",
    offset, rc);
    pci_doe_destroy_mb(doe_mb);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pci_doe_destroy(pdev: *mut pci_dev) {
    void pci_doe_destroy(struct pci_dev *pdev)
    {
    struct pci_doe_mb *doe_mb;
    unsigned long index;
    xa_for_each(&pdev.doe_mbs, index, doe_mb)
    pci_doe_destroy_mb(doe_mb);
    xa_destroy(&pdev.doe_mbs);
    }
#[no_mangle]
pub unsafe extern "C" fn pci_doe_disconnected(pdev: *mut pci_dev) {
    void pci_doe_disconnected(struct pci_dev *pdev)
    {
    struct pci_doe_mb *doe_mb;
    unsigned long index;
    xa_for_each(&pdev.doe_mbs, index, doe_mb)
    pci_doe_cancel_tasks(doe_mb);
    }
