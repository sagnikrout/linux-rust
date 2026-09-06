//! Automatically rewritten from C to Rust
//! Source: drivers/accel/qaic/qaic_control.c
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
// Copyright (c) 2019-2021, The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.

pub const QAIC_MHI_RETRY_WAIT_MS: c_int = 100;
pub const QAIC_MHI_RETRY_MAX: c_int = 20;
    static unsigned int control_resp_timeout_s = 60; /* 60 sec default */
    module_param(control_resp_timeout_s, uint, 0600);
    MODULE_PARM_DESC(control_resp_timeout_s, "Timeout for NNC responses from QSM");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct manage_msg {
    pub len: u32,
    pub count: u32,
    pub data: [u8; ],
}

//
// wire encoding structures for the manage protocol.
// All fields are little endian on the wire
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_msg_hdr {
    pub /: *mut *mut __le32 crc32; / crc of everything following this field in the message,
    pub magic_number: __le32,
    pub sequence_number: __le32,
    pub /: *mut *mut __le32 len; / length of this message,
    pub /: *mut *mut __le32 count; / number of transactions in this message,
    pub /: *mut *mut __le32 handle; / unique id to track the resources consumed,
    pub /: *mut *mut __le32 partition_id; / partition id for the request (signed),
    pub /: *mut *mut __le32 padding; / must be 0,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_msg {
    pub hdr: wire_msg_hdr,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_hdr {
    pub type: __le32,
    pub len: __le32,
    pub __packed: },
// Each message sent from driver to device are organized in a list of wrapper_msg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wrapper_msg {
    pub list: list_head,
    pub ref_count: kref,
    pub /: *mut *mut u32 len; / length of data to transfer,
    pub head: *mut wrapper_list,
    union {
    pub msg: wire_msg,
    pub trans: wire_trans_hdr,
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wrapper_list {
    pub list: list_head,
    pub /: *mut *mut spinlock_t lock; / Protects the list state during additions and removals,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_passthrough {
    pub hdr: wire_trans_hdr,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_addr_size_pair {
    pub addr: __le64,
    pub size: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_dma_xfer {
    pub hdr: wire_trans_hdr,
    pub tag: __le32,
    pub count: __le32,
    pub dma_chunk_id: __le32,
    pub padding: __le32,
    pub data: [wire_addr_size_pair; ],
    pub __packed: },
// Initiated by device to continue the DMA xfer of a large piece of data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_dma_xfer_cont {
    pub hdr: wire_trans_hdr,
    pub dma_chunk_id: __le32,
    pub padding: __le32,
    pub xferred_size: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_activate_to_dev {
    pub hdr: wire_trans_hdr,
    pub req_q_addr: __le64,
    pub rsp_q_addr: __le64,
    pub req_q_size: __le32,
    pub rsp_q_size: __le32,
    pub buf_len: __le32,
    pub /: *mut *mut __le32 options; / unused, but BIT(16) has meaning to the device,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_activate_from_dev {
    pub hdr: wire_trans_hdr,
    pub status: __le32,
    pub dbc_id: __le32,
    pub /: *mut *mut __le64 options; / unused,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_deactivate_from_dev {
    pub hdr: wire_trans_hdr,
    pub status: __le32,
    pub dbc_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_terminate_to_dev {
    pub hdr: wire_trans_hdr,
    pub handle: __le32,
    pub padding: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_terminate_from_dev {
    pub hdr: wire_trans_hdr,
    pub status: __le32,
    pub padding: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_status_to_dev {
    pub hdr: wire_trans_hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_status_from_dev {
    pub hdr: wire_trans_hdr,
    pub major: __le16,
    pub minor: __le16,
    pub status: __le32,
    pub status_flags: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_validate_part_to_dev {
    pub hdr: wire_trans_hdr,
    pub part_id: __le32,
    pub padding: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wire_trans_validate_part_from_dev {
    pub hdr: wire_trans_hdr,
    pub status: __le32,
    pub padding: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfer_queue_elem {
//
// Node in list of ongoing transfer request on control channel.
// Maintained by root device struct.
//
    pub list: list_head,
// Sequence number of this transfer request
    pub seq_num: u32,
// This is used to wait on until completion of transfer request
    pub xfer_done: completion,
// Received data from device
    pub buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_xfer {
// Node in list of DMA transfers which is used for cleanup
    pub list: list_head,
// SG table of memory used for DMA
    pub sgt: *mut sg_table,
// Array pages used for DMA
    pub page_list: *mut page,
// Number of pages used for DMA
    pub nr_pages: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_resources {
// List of all DMA transfers which is used later for cleanup
    pub dma_xfers: list_head,
// Base address of request queue which belongs to a DBC
    pub buf: *mut c_void,
//
// Base bus address of request queue which belongs to a DBC. Response
// queue base bus address can be calculated by adding size of request
// queue to base bus address of request queue.
//
    pub dma_addr: dma_addr_t,
// Total size of request queue and response queue in byte
    pub total_size: u32,
// Total number of elements that can be queued in each of request and response queue
    pub nelem: u32,
// Base address of response queue which belongs to a DBC
    pub rsp_q_base: *mut c_void,
// Status of the NNC message received
    pub status: u32,
// DBC id of the DBC received from device
    pub dbc_id: u32,
//
// DMA transfer request messages can be big in size and it may not be
// possible to send them in one shot. In such cases the messages are
// broken into chunks, this field stores ID of such chunks.
//
    pub dma_chunk_id: u32,
// Total number of bytes transferred for a DMA xfer request
    pub xferred_dma_size: u64,
// Header of transaction message received from user. Used during DMA xfer request.
    pub trans_hdr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resp_work {
    pub work: work_struct,
    pub qdev: *mut qaic_device,
    pub buf: *mut c_void,
}

//
// Since we're working with little endian messages, its useful to be able to
// increment without filling a whole line with conversions back and forth just
// to add one(1) to a message count.
//
#[no_mangle]
unsafe extern "C" fn incr_le32(val: __le32) -> __le32 {
    static __le32 incr_le32(__le32 val)
    {
    return cpu_to_le32(le32_to_cpu(val) + 1);
    }
#[no_mangle]
unsafe extern "C" fn gen_crc(msg: *mut c_void) -> u32 {
    static u32 gen_crc(void *msg)
    {
    struct wrapper_list *wrappers = msg;
    struct wrapper_msg *w;
    let mut crc: u32 = ~0;
    list_for_each_entry(w, &wrappers.list, list)
    crc = crc32(crc, &w.msg, w.len);
    return crc ^ ~0;
    }
#[no_mangle]
unsafe extern "C" fn gen_crc_stub(msg: *mut c_void) -> u32 {
    static u32 gen_crc_stub(void *msg)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn valid_crc(msg: *mut c_void) -> bool {
    static bool valid_crc(void *msg)
    {
    struct wire_msg_hdr *hdr = msg;
    bool ret;
    u32 crc;
//
// The output of this algorithm is always converted to the native
// endianness.
//
    crc = le32_to_cpu(hdr.crc32);
    hdr.crc32 = 0;
    ret = (crc32(~0, msg, le32_to_cpu(hdr.len)) ^ ~0) == crc;
    hdr.crc32 = cpu_to_le32(crc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn valid_crc_stub(msg: *mut c_void) -> bool {
    static bool valid_crc_stub(void *msg)
    {
    return true;
    }
#[no_mangle]
unsafe extern "C" fn free_wrapper(ref: *mut kref) {
    static void free_wrapper(struct kref *ref)
    {
    struct wrapper_msg *wrapper = container_of(ref, struct wrapper_msg, ref_count);
    list_del(&wrapper.list);
    kfree(wrapper);
    }
    static void save_dbc_buf(struct qaic_device *qdev, struct ioctl_resources *resources,
    struct qaic_user *usr)
    {
    let mut dbc_id: u32 = resources.dbc_id;
    if (resources.buf) {
    wait_event_interruptible(qdev.dbc[dbc_id].dbc_release, !qdev.dbc[dbc_id].in_use);
    qdev.dbc[dbc_id].req_q_base = resources.buf;
    qdev.dbc[dbc_id].rsp_q_base = resources.rsp_q_base;
    qdev.dbc[dbc_id].dma_addr = resources.dma_addr;
    qdev.dbc[dbc_id].total_size = resources.total_size;
    qdev.dbc[dbc_id].nelem = resources.nelem;
    enable_dbc(qdev, dbc_id, usr);
    qdev.dbc[dbc_id].in_use = true;
    resources.buf = core::ptr::null_mut();
    set_dbc_state(qdev, dbc_id, DBC_STATE_ASSIGNED);
    }
    }
#[no_mangle]
unsafe extern "C" fn free_dbc_buf(qdev: *mut qaic_device, resources: *mut ioctl_resources) {
    static void free_dbc_buf(struct qaic_device *qdev, struct ioctl_resources *resources)
    {
    if (resources.buf)
    dma_free_coherent(&qdev.pdev.dev, resources.total_size, resources.buf,
    resources.dma_addr);
    resources.buf = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn free_dma_xfers(qdev: *mut qaic_device, resources: *mut ioctl_resources) {
    static void free_dma_xfers(struct qaic_device *qdev, struct ioctl_resources *resources)
    {
    struct dma_xfer *xfer;
    struct dma_xfer *x;
    int i;
    list_for_each_entry_safe(xfer, x, &resources.dma_xfers, list) {
    dma_unmap_sgtable(&qdev.pdev.dev, xfer.sgt, DMA_TO_DEVICE, 0);
    sg_free_table(xfer.sgt);
    kfree(xfer.sgt);
    for (i = 0; i < xfer.nr_pages; ++i)
    put_page(xfer.page_list[i]);
    kfree(xfer.page_list);
    list_del(&xfer.list);
    kfree(xfer);
    }
    }
    static struct wrapper_msg *add_wrapper(struct wrapper_list *wrappers, u32 size)
    {
    struct wrapper_msg *w = kzalloc(size, GFP_KERNEL);
    if (!w)
    return core::ptr::null_mut();
    list_add_tail(&w.list, &wrappers.list);
    kref_init(&w.ref_count);
    w.head = wrappers;
    return w;
    }
    static int encode_passthrough(struct qaic_device *qdev, void *trans, struct wrapper_list *wrappers,
    u32 *user_len)
    {
    struct qaic_manage_trans_passthrough *in_trans = trans;
    struct wire_trans_passthrough *out_trans;
    struct wrapper_msg *trans_wrapper;
    struct wrapper_msg *wrapper;
    struct wire_msg *msg;
    u32 msg_hdr_len;
    wrapper = list_first_entry(&wrappers.list, struct wrapper_msg, list);
    msg = &wrapper.msg;
    msg_hdr_len = le32_to_cpu(msg.hdr.len);
    if (in_trans.hdr.len % 8 != 0)
    return -EINVAL;
    if (size_add(msg_hdr_len, in_trans.hdr.len) > QAIC_MANAGE_WIRE_MSG_LENGTH)
    return -ENOSPC;
    trans_wrapper = add_wrapper(wrappers,
    offsetof(struct wrapper_msg, trans) + in_trans.hdr.len);
    if (!trans_wrapper)
    return -ENOMEM;
    trans_wrapper.len = in_trans.hdr.len;
    out_trans = (struct wire_trans_passthrough *)&trans_wrapper.trans;
    memcpy(out_trans.data, in_trans.data, in_trans.hdr.len - sizeof(in_trans.hdr));
    msg.hdr.len = cpu_to_le32(msg_hdr_len + in_trans.hdr.len);
    msg.hdr.count = incr_le32(msg.hdr.count);
// user_len += in_trans->hdr.len;
    out_trans.hdr.type = cpu_to_le32(QAIC_TRANS_PASSTHROUGH_TO_DEV);
    out_trans.hdr.len = cpu_to_le32(in_trans.hdr.len);
    return 0;
    }
// returns error code for failure, 0 if enough pages alloc'd, 1 if dma_cont is needed
    static int find_and_map_user_pages(struct qaic_device *qdev,
    struct qaic_manage_trans_dma_xfer *in_trans,
    struct ioctl_resources *resources, struct dma_xfer *xfer)
    {
    u64 xfer_start_addr, remaining, end, total;
    unsigned long need_pages;
    struct page **page_list;
    unsigned long nr_pages;
    struct sg_table *sgt;
    int ret;
    int i;
    if (check_add_overflow(in_trans.addr, resources.xferred_dma_size, &xfer_start_addr))
    return -EINVAL;
    if (in_trans.size < resources.xferred_dma_size)
    return -EINVAL;
    remaining = in_trans.size - resources.xferred_dma_size;
    if (remaining == 0)
    return -EINVAL;
    if (check_add_overflow(xfer_start_addr, remaining, &end))
    return -EINVAL;
    total = remaining + offset_in_page(xfer_start_addr);
    if (total >= SIZE_MAX)
    return -EINVAL;
    need_pages = DIV_ROUND_UP(total, PAGE_SIZE);
    nr_pages = need_pages;
    while (1) {
    page_list = kmalloc_objs(*page_list, nr_pages,
    GFP_KERNEL | __GFP_NOWARN);
    if (!page_list) {
    nr_pages = nr_pages / 2;
    if (!nr_pages)
    return -ENOMEM;
    } else {
    break;
    }
    }
    ret = get_user_pages_fast(xfer_start_addr, nr_pages, 0, page_list);
    if (ret < 0)
    goto free_page_list;
    if (ret != nr_pages) {
    nr_pages = ret;
    ret = -EFAULT;
    goto put_pages;
    }
    sgt = kmalloc_obj(*sgt);
    if (!sgt) {
    ret = -ENOMEM;
    goto put_pages;
    }
    ret = sg_alloc_table_from_pages(sgt, page_list, nr_pages,
    offset_in_page(xfer_start_addr),
    remaining, GFP_KERNEL);
    if (ret) {
    ret = -ENOMEM;
    goto free_sgt;
    }
    ret = dma_map_sgtable(&qdev.pdev.dev, sgt, DMA_TO_DEVICE, 0);
    if (ret)
    goto free_table;
    xfer.sgt = sgt;
    xfer.page_list = page_list;
    xfer.nr_pages = nr_pages;
    return need_pages > nr_pages ? 1 : 0;
    free_table:
    sg_free_table(sgt);
    free_sgt:
    kfree(sgt);
    put_pages:
    for (i = 0; i < nr_pages; ++i)
    put_page(page_list[i]);
    free_page_list:
    kfree(page_list);
    return ret;
    }
// returns error code for failure, 0 if everything was encoded, 1 if dma_cont is needed
    static int encode_addr_size_pairs(struct dma_xfer *xfer, struct wrapper_list *wrappers,
    struct ioctl_resources *resources, u32 msg_hdr_len, u32 *size,
    struct wire_trans_dma_xfer **out_trans)
    {
    struct wrapper_msg *trans_wrapper;
    struct sg_table *sgt = xfer.sgt;
    struct wire_addr_size_pair *asp;
    struct scatterlist *sg;
    struct wrapper_msg *w;
    unsigned int dma_len;
    u64 dma_chunk_len;
    void *boundary;
    int nents_dma;
    int nents;
    int i;
    nents = sgt.nents;
    nents_dma = nents;
// size = QAIC_MANAGE_WIRE_MSG_LENGTH - msg_hdr_len - sizeof(**out_trans);
    for_each_sgtable_dma_sg(sgt, sg, i) {
// size -= sizeof(*asp);
// Save 1K for possible follow-up transactions.
    if (*size < SZ_1K) {
    nents_dma = i;
    break;
    }
    }
    trans_wrapper = add_wrapper(wrappers, QAIC_WRAPPER_MAX_SIZE);
    if (!trans_wrapper)
    return -ENOMEM;
// out_trans = (struct wire_trans_dma_xfer *)&trans_wrapper->trans;
    asp = (*out_trans).data;
    boundary = (void *)trans_wrapper + QAIC_WRAPPER_MAX_SIZE;
// size = 0;
    dma_len = 0;
    w = trans_wrapper;
    dma_chunk_len = 0;
    for_each_sg(sgt.sgl, sg, nents_dma, i) {
    asp.size = cpu_to_le64(dma_len);
    dma_chunk_len += dma_len;
    if (dma_len) {
    asp++;
    if ((void *)asp + sizeof(*asp) > boundary) {
    w.len = (void *)asp - (void *)&w.msg;
// size += w->len;
    w = add_wrapper(wrappers, QAIC_WRAPPER_MAX_SIZE);
    if (!w)
    return -ENOMEM;
    boundary = (void *)w + QAIC_WRAPPER_MAX_SIZE;
    asp = (struct wire_addr_size_pair *)&w.msg;
    }
    }
    asp.addr = cpu_to_le64(sg_dma_address(sg));
    dma_len = sg_dma_len(sg);
    }
// finalize the last segment
    asp.size = cpu_to_le64(dma_len);
    w.len = (void *)asp + sizeof(*asp) - (void *)&w.msg;
// size += w->len;
    dma_chunk_len += dma_len;
    resources.xferred_dma_size += dma_chunk_len;
    return nents_dma < nents ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_xfer(qdev: *mut qaic_device, xfer: *mut dma_xfer) {
    static void cleanup_xfer(struct qaic_device *qdev, struct dma_xfer *xfer)
    {
    int i;
    dma_unmap_sgtable(&qdev.pdev.dev, xfer.sgt, DMA_TO_DEVICE, 0);
    sg_free_table(xfer.sgt);
    kfree(xfer.sgt);
    for (i = 0; i < xfer.nr_pages; ++i)
    put_page(xfer.page_list[i]);
    kfree(xfer.page_list);
    }
    static int encode_dma(struct qaic_device *qdev, void *trans, struct wrapper_list *wrappers,
    u32 *user_len, struct ioctl_resources *resources, struct qaic_user *usr)
    {
    struct qaic_manage_trans_dma_xfer *in_trans = trans;
    struct wire_trans_dma_xfer *out_trans;
    struct wrapper_msg *wrapper;
    struct dma_xfer *xfer;
    struct wire_msg *msg;
    bool need_cont_dma;
    u32 msg_hdr_len;
    u32 size;
    int ret;
    wrapper = list_first_entry(&wrappers.list, struct wrapper_msg, list);
    msg = &wrapper.msg;
    msg_hdr_len = le32_to_cpu(msg.hdr.len);
// There should be enough space to hold at least one ASP entry.
    if (size_add(msg_hdr_len, sizeof(*out_trans) + sizeof(struct wire_addr_size_pair)) >
    QAIC_MANAGE_WIRE_MSG_LENGTH)
    return -ENOMEM;
    xfer = kmalloc_obj(*xfer);
    if (!xfer)
    return -ENOMEM;
    ret = find_and_map_user_pages(qdev, in_trans, resources, xfer);
    if (ret < 0)
    goto free_xfer;
    need_cont_dma = (bool)ret;
    ret = encode_addr_size_pairs(xfer, wrappers, resources, msg_hdr_len, &size, &out_trans);
    if (ret < 0)
    goto cleanup_xfer;
    need_cont_dma = need_cont_dma || (bool)ret;
    msg.hdr.len = cpu_to_le32(msg_hdr_len + size);
    msg.hdr.count = incr_le32(msg.hdr.count);
    out_trans.hdr.type = cpu_to_le32(QAIC_TRANS_DMA_XFER_TO_DEV);
    out_trans.hdr.len = cpu_to_le32(size);
    out_trans.tag = cpu_to_le32(in_trans.tag);
    out_trans.count = cpu_to_le32((size - sizeof(*out_trans)) /
    sizeof(struct wire_addr_size_pair));
// user_len += in_trans->hdr.len;
    if (resources.dma_chunk_id) {
    out_trans.dma_chunk_id = cpu_to_le32(resources.dma_chunk_id);
    } else if (need_cont_dma) {
    while (resources.dma_chunk_id == 0)
    resources.dma_chunk_id = atomic_inc_return(&usr.chunk_id);
    out_trans.dma_chunk_id = cpu_to_le32(resources.dma_chunk_id);
    }
    resources.trans_hdr = trans;
    list_add(&xfer.list, &resources.dma_xfers);
    return 0;
    cleanup_xfer:
    cleanup_xfer(qdev, xfer);
    free_xfer:
    kfree(xfer);
    return ret;
    }
    static int encode_activate(struct qaic_device *qdev, void *trans, struct wrapper_list *wrappers,
    u32 *user_len, struct ioctl_resources *resources)
    {
    struct qaic_manage_trans_activate_to_dev *in_trans = trans;
    struct wire_trans_activate_to_dev *out_trans;
    struct wrapper_msg *trans_wrapper;
    struct wrapper_msg *wrapper;
    struct wire_msg *msg;
    dma_addr_t dma_addr;
    u32 msg_hdr_len;
    void *buf;
    u32 nelem;
    u32 size;
    int ret;
    wrapper = list_first_entry(&wrappers.list, struct wrapper_msg, list);
    msg = &wrapper.msg;
    msg_hdr_len = le32_to_cpu(msg.hdr.len);
    if (size_add(msg_hdr_len, sizeof(*out_trans)) > QAIC_MANAGE_WIRE_MSG_LENGTH)
    return -ENOSPC;
    if (!in_trans.queue_size)
    return -EINVAL;
    if (in_trans.pad)
    return -EINVAL;
    nelem = in_trans.queue_size;
    if (check_mul_overflow((u32)(get_dbc_req_elem_size() + get_dbc_rsp_elem_size()),
    nelem,
    &size))
    return -EINVAL;
    if (size + QAIC_DBC_Q_GAP + QAIC_DBC_Q_BUF_ALIGN < size)
    return -EINVAL;
    size = ALIGN((size + QAIC_DBC_Q_GAP), QAIC_DBC_Q_BUF_ALIGN);
    buf = dma_alloc_coherent(&qdev.pdev.dev, size, &dma_addr, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    trans_wrapper = add_wrapper(wrappers,
    offsetof(struct wrapper_msg, trans) + sizeof(*out_trans));
    if (!trans_wrapper) {
    ret = -ENOMEM;
    goto free_dma;
    }
    trans_wrapper.len = sizeof(*out_trans);
    out_trans = (struct wire_trans_activate_to_dev *)&trans_wrapper.trans;
    out_trans.hdr.type = cpu_to_le32(QAIC_TRANS_ACTIVATE_TO_DEV);
    out_trans.hdr.len = cpu_to_le32(sizeof(*out_trans));
    out_trans.buf_len = cpu_to_le32(size);
    out_trans.req_q_addr = cpu_to_le64(dma_addr);
    out_trans.req_q_size = cpu_to_le32(nelem);
    out_trans.rsp_q_addr = cpu_to_le64(dma_addr + size - nelem * get_dbc_rsp_elem_size());
    out_trans.rsp_q_size = cpu_to_le32(nelem);
    out_trans.options = cpu_to_le32(in_trans.options);
// user_len += in_trans->hdr.len;
    msg.hdr.len = cpu_to_le32(msg_hdr_len + sizeof(*out_trans));
    msg.hdr.count = incr_le32(msg.hdr.count);
    resources.buf = buf;
    resources.dma_addr = dma_addr;
    resources.total_size = size;
    resources.nelem = nelem;
    resources.rsp_q_base = buf + size - nelem * get_dbc_rsp_elem_size();
    return 0;
    free_dma:
    dma_free_coherent(&qdev.pdev.dev, size, buf, dma_addr);
    return ret;
    }
    static int encode_deactivate(struct qaic_device *qdev, void *trans,
    u32 *user_len, struct qaic_user *usr)
    {
    struct qaic_manage_trans_deactivate *in_trans = trans;
    if (in_trans.dbc_id >= qdev.num_dbc || in_trans.pad)
    return -EINVAL;
// user_len += in_trans->hdr.len;
    return disable_dbc(qdev, in_trans.dbc_id, usr);
    }
    static int encode_status(struct qaic_device *qdev, void *trans, struct wrapper_list *wrappers,
    u32 *user_len)
    {
    struct qaic_manage_trans_status_to_dev *in_trans = trans;
    struct wire_trans_status_to_dev *out_trans;
    struct wrapper_msg *trans_wrapper;
    struct wrapper_msg *wrapper;
    struct wire_msg *msg;
    u32 msg_hdr_len;
    wrapper = list_first_entry(&wrappers.list, struct wrapper_msg, list);
    msg = &wrapper.msg;
    msg_hdr_len = le32_to_cpu(msg.hdr.len);
    if (size_add(msg_hdr_len, in_trans.hdr.len) > QAIC_MANAGE_WIRE_MSG_LENGTH)
    return -ENOSPC;
    trans_wrapper = add_wrapper(wrappers, sizeof(*trans_wrapper));
    if (!trans_wrapper)
    return -ENOMEM;
    trans_wrapper.len = sizeof(*out_trans);
    out_trans = (struct wire_trans_status_to_dev *)&trans_wrapper.trans;
    out_trans.hdr.type = cpu_to_le32(QAIC_TRANS_STATUS_TO_DEV);
    out_trans.hdr.len = cpu_to_le32(in_trans.hdr.len);
    msg.hdr.len = cpu_to_le32(msg_hdr_len + in_trans.hdr.len);
    msg.hdr.count = incr_le32(msg.hdr.count);
// user_len += in_trans->hdr.len;
    return 0;
    }
    static int encode_message(struct qaic_device *qdev, struct manage_msg *user_msg,
    struct wrapper_list *wrappers, struct ioctl_resources *resources,
    struct qaic_user *usr)
    {
    struct qaic_manage_trans_hdr *trans_hdr;
    struct wrapper_msg *wrapper;
    struct wire_msg *msg;
    let mut user_len: u32 = 0;
    int ret;
    int i;
    if (!user_msg.count ||
    user_msg.len < sizeof(*trans_hdr)) {
    ret = -EINVAL;
    goto out;
    }
    wrapper = list_first_entry(&wrappers.list, struct wrapper_msg, list);
    msg = &wrapper.msg;
    msg.hdr.len = cpu_to_le32(sizeof(msg.hdr));
    if (resources.dma_chunk_id) {
    ret = encode_dma(qdev, resources.trans_hdr, wrappers, &user_len, resources, usr);
    msg.hdr.count = cpu_to_le32(1);
    goto out;
    }
    for (i = 0; i < user_msg.count; ++i) {
    if (user_len > user_msg.len - sizeof(*trans_hdr)) {
    ret = -EINVAL;
    break;
    }
    trans_hdr = (struct qaic_manage_trans_hdr *)(user_msg.data + user_len);
    if (trans_hdr.len < sizeof(*trans_hdr) ||
    size_add(user_len, trans_hdr.len) > user_msg.len) {
    ret = -EINVAL;
    break;
    }
    switch (trans_hdr.type) {
    case QAIC_TRANS_PASSTHROUGH_FROM_USR:
    ret = encode_passthrough(qdev, trans_hdr, wrappers, &user_len);
    break;
    case QAIC_TRANS_DMA_XFER_FROM_USR:
    ret = encode_dma(qdev, trans_hdr, wrappers, &user_len, resources, usr);
    break;
    case QAIC_TRANS_ACTIVATE_FROM_USR:
    ret = encode_activate(qdev, trans_hdr, wrappers, &user_len, resources);
    break;
    case QAIC_TRANS_DEACTIVATE_FROM_USR:
    ret = encode_deactivate(qdev, trans_hdr, &user_len, usr);
    break;
    case QAIC_TRANS_STATUS_FROM_USR:
    ret = encode_status(qdev, trans_hdr, wrappers, &user_len);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    if (ret)
    goto out;
    }
    if (user_len != user_msg.len)
    ret = -EINVAL;
    out:
    if (ret) {
    free_dma_xfers(qdev, resources);
    free_dbc_buf(qdev, resources);
    return ret;
    }
    return 0;
    }
    static int decode_passthrough(struct qaic_device *qdev, void *trans, struct manage_msg *user_msg,
    u32 *msg_len)
    {
    struct qaic_manage_trans_passthrough *out_trans;
    struct wire_trans_passthrough *in_trans = trans;
    u32 len;
    out_trans = (void *)user_msg.data + user_msg.len;
    len = le32_to_cpu(in_trans.hdr.len);
    if (len % 8 != 0)
    return -EINVAL;
    if (user_msg.len + len > QAIC_MANAGE_MAX_MSG_LENGTH)
    return -ENOSPC;
    memcpy(out_trans.data, in_trans.data, len - sizeof(in_trans.hdr));
    user_msg.len += len;
// msg_len += len;
    out_trans.hdr.type = le32_to_cpu(in_trans.hdr.type);
    out_trans.hdr.len = len;
    return 0;
    }
    static int decode_activate(struct qaic_device *qdev, void *trans, struct manage_msg *user_msg,
    u32 *msg_len, struct ioctl_resources *resources, struct qaic_user *usr)
    {
    struct qaic_manage_trans_activate_from_dev *out_trans;
    struct wire_trans_activate_from_dev *in_trans = trans;
    u32 len;
    out_trans = (void *)user_msg.data + user_msg.len;
    len = le32_to_cpu(in_trans.hdr.len);
    if (user_msg.len + len > QAIC_MANAGE_MAX_MSG_LENGTH)
    return -ENOSPC;
    user_msg.len += len;
// msg_len += len;
    out_trans.hdr.type = le32_to_cpu(in_trans.hdr.type);
    out_trans.hdr.len = len;
    out_trans.status = le32_to_cpu(in_trans.status);
    out_trans.dbc_id = le32_to_cpu(in_trans.dbc_id);
    out_trans.options = le64_to_cpu(in_trans.options);
    if (!resources.buf)
// how did we get an activate response without a request?
    return -EINVAL;
    if (out_trans.dbc_id >= qdev.num_dbc)
//
// The device assigned an invalid resource, which should never
// happen. Return an error so the user can try to recover.
//
    return -ENODEV;
    if (out_trans.status)
//
// Allocating resources failed on device side. This is not an
// expected behaviour, user is expected to handle this situation.
//
    return -ECANCELED;
    resources.status = out_trans.status;
    resources.dbc_id = out_trans.dbc_id;
    save_dbc_buf(qdev, resources, usr);
    return 0;
    }
    static int decode_deactivate(struct qaic_device *qdev, void *trans, u32 *msg_len,
    struct qaic_user *usr)
    {
    struct wire_trans_deactivate_from_dev *in_trans = trans;
    let mut dbc_id: u32 = le32_to_cpu(in_trans.dbc_id);
    let mut status: u32 = le32_to_cpu(in_trans.status);
    if (dbc_id >= qdev.num_dbc)
//
// The device assigned an invalid resource, which should never
// happen. Inject an error so the user can try to recover.
//
    return -ENODEV;
    if (usr && status) {
//
// Releasing resources failed on the device side, which puts
// us in a bind since they may still be in use, so enable the
// dbc. User is expected to retry deactivation.
//
    enable_dbc(qdev, dbc_id, usr);
    return -ECANCELED;
    }
    release_dbc(qdev, dbc_id);
    set_dbc_state(qdev, dbc_id, DBC_STATE_IDLE);
// msg_len += sizeof(*in_trans);
    return 0;
    }
    static int decode_status(struct qaic_device *qdev, void *trans, struct manage_msg *user_msg,
    u32 *user_len, struct wire_msg *msg)
    {
    struct qaic_manage_trans_status_from_dev *out_trans;
    struct wire_trans_status_from_dev *in_trans = trans;
    u32 len;
    out_trans = (void *)user_msg.data + user_msg.len;
    len = le32_to_cpu(in_trans.hdr.len);
    if (user_msg.len + len > QAIC_MANAGE_MAX_MSG_LENGTH)
    return -ENOSPC;
    out_trans.hdr.type = QAIC_TRANS_STATUS_FROM_DEV;
    out_trans.hdr.len = len;
    out_trans.major = le16_to_cpu(in_trans.major);
    out_trans.minor = le16_to_cpu(in_trans.minor);
    out_trans.status_flags = le64_to_cpu(in_trans.status_flags);
    out_trans.status = le32_to_cpu(in_trans.status);
// user_len += le32_to_cpu(in_trans->hdr.len);
    user_msg.len += len;
    if (out_trans.status)
    return -ECANCELED;
    if (out_trans.status_flags & BIT(0) && !valid_crc(msg))
    return -EPIPE;
    return 0;
    }
    static int decode_message(struct qaic_device *qdev, struct manage_msg *user_msg,
    struct wire_msg *msg, struct ioctl_resources *resources,
    struct qaic_user *usr, bool orphaned_deactivate)
    {
    let mut msg_hdr_count: u32 = le32_to_cpu(msg.hdr.count);
    let mut msg_hdr_len: u32 = le32_to_cpu(msg.hdr.len);
    struct wire_trans_hdr *trans_hdr;
    let mut msg_len: u32 = 0;
    int trans_type;
    int ret;
    int i;
    if (msg_hdr_len < sizeof(*trans_hdr) ||
    msg_hdr_len > QAIC_MANAGE_MAX_MSG_LENGTH)
    return -EINVAL;
    if (user_msg) {
    user_msg.len = 0;
    user_msg.count = msg_hdr_count;
    }
    for (i = 0; i < msg_hdr_count; ++i) {
    u32 hdr_len;
    if (msg_len > msg_hdr_len - sizeof(*trans_hdr))
    return -EINVAL;
    trans_hdr = (struct wire_trans_hdr *)(msg.data + msg_len);
    hdr_len = le32_to_cpu(trans_hdr.len);
    if (hdr_len < sizeof(*trans_hdr) ||
    size_add(msg_len, hdr_len) > msg_hdr_len)
    return -EINVAL;
    trans_type = le32_to_cpu(trans_hdr.type);
//
// orphaned_deactivate is the case where a deactivate response
// is received from the device after the user owning the DBC,
// and the message requesting deactivation, has gone away.
// In this case, only process QAIC_TRANS_DEACTIVATE_FROM_DEV
// transaction and skip the others.
//
    if (orphaned_deactivate && trans_type != QAIC_TRANS_DEACTIVATE_FROM_DEV) {
    msg_len += hdr_len;
    continue;
    }
    switch (trans_type) {
    case QAIC_TRANS_PASSTHROUGH_FROM_DEV:
    ret = decode_passthrough(qdev, trans_hdr, user_msg, &msg_len);
    break;
    case QAIC_TRANS_ACTIVATE_FROM_DEV:
    ret = decode_activate(qdev, trans_hdr, user_msg, &msg_len, resources, usr);
    break;
    case QAIC_TRANS_DEACTIVATE_FROM_DEV:
    ret = decode_deactivate(qdev, trans_hdr, &msg_len, usr);
    break;
    case QAIC_TRANS_STATUS_FROM_DEV:
    ret = decode_status(qdev, trans_hdr, user_msg, &msg_len, msg);
    break;
    default:
    return -EINVAL;
    }
    if (ret)
    return ret;
    }
    if (msg_len != (msg_hdr_len - sizeof(msg.hdr)))
    return -EINVAL;
    return 0;
    }
    static void *msg_xfer(struct qaic_device *qdev, struct wrapper_list *wrappers, u32 seq_num,
    bool ignore_signal)
    {
    struct xfer_queue_elem elem;
    struct wire_msg *out_buf;
    struct wrapper_msg *w;
    let mut ret: c_long = -EAGAIN;
    let mut xfer_count: c_int = 0;
    int retry_count;
// Allow QAIC_BOOT state since we need to check control protocol version
    if (qdev.dev_state == QAIC_OFFLINE) {
    mutex_unlock(&qdev.cntl_mutex);
    return ERR_PTR(-ENODEV);
    }
// Attempt to avoid a partial commit of a message
    list_for_each_entry(w, &wrappers.list, list)
    xfer_count++;
    for (retry_count = 0; retry_count < QAIC_MHI_RETRY_MAX; retry_count++) {
    if (xfer_count <= mhi_get_free_desc_count(qdev.cntl_ch, DMA_TO_DEVICE)) {
    ret = 0;
    break;
    }
    msleep_interruptible(QAIC_MHI_RETRY_WAIT_MS);
    if (signal_pending(current))
    break;
    }
    if (ret) {
    mutex_unlock(&qdev.cntl_mutex);
    return ERR_PTR(ret);
    }
    elem.seq_num = seq_num;
    elem.buf = core::ptr::null_mut();
    init_completion(&elem.xfer_done);
    if (likely(!qdev.cntl_lost_buf)) {
//
// The max size of request to device is QAIC_MANAGE_WIRE_MSG_LENGTH.
// The max size of response from device is QAIC_MANAGE_MAX_MSG_LENGTH.
//
    out_buf = kmalloc(QAIC_MANAGE_MAX_MSG_LENGTH, GFP_KERNEL);
    if (!out_buf) {
    mutex_unlock(&qdev.cntl_mutex);
    return ERR_PTR(-ENOMEM);
    }
    ret = mhi_queue_buf(qdev.cntl_ch, DMA_FROM_DEVICE, out_buf,
    QAIC_MANAGE_MAX_MSG_LENGTH, MHI_EOT);
    if (ret) {
    mutex_unlock(&qdev.cntl_mutex);
    return ERR_PTR(ret);
    }
    } else {
//
// we lost a buffer because we queued a recv buf, but then
// queuing the corresponding tx buf failed. To try to avoid
// a memory leak, lets reclaim it and use it for this
// transaction.
//
    qdev.cntl_lost_buf = false;
    }
    list_for_each_entry(w, &wrappers.list, list) {
    kref_get(&w.ref_count);
    ret = mhi_queue_buf(qdev.cntl_ch, DMA_TO_DEVICE, &w.msg, w.len,
    list_is_last(&w.list, &wrappers.list) ? MHI_EOT : MHI_CHAIN);
    if (ret) {
    qdev.cntl_lost_buf = true;
    kref_put(&w.ref_count, free_wrapper);
    mutex_unlock(&qdev.cntl_mutex);
    return ERR_PTR(ret);
    }
    }
    list_add_tail(&elem.list, &qdev.cntl_xfer_list);
    mutex_unlock(&qdev.cntl_mutex);
    if (ignore_signal)
    ret = wait_for_completion_timeout(&elem.xfer_done, control_resp_timeout_s * HZ);
    else
    ret = wait_for_completion_interruptible_timeout(&elem.xfer_done,
    control_resp_timeout_s * HZ);
//
// not using _interruptable because we have to cleanup or we'll
// likely cause memory corruption
//
    mutex_lock(&qdev.cntl_mutex);
    if (!list_empty(&elem.list))
    list_del(&elem.list);
// resp_worker() processed the response but the wait was interrupted
#[no_mangle]
pub unsafe extern "C" fn if(-ERESTARTSYS: ret ==) -> else {
    else if (ret == -ERESTARTSYS)
    ret = 0;
    if (!ret && !elem.buf)
    ret = -ETIMEDOUT;
#[no_mangle]
pub unsafe extern "C" fn if(!elem.buf: ret > 0 &&) -> else {
    else if (ret > 0 && !elem.buf)
    ret = -EIO;
    mutex_unlock(&qdev.cntl_mutex);
    if (ret < 0) {
    kfree(elem.buf);
    return ERR_PTR(ret);
    } else if (!qdev.valid_crc(elem.buf)) {
    kfree(elem.buf);
    return ERR_PTR(-EPIPE);
    }
    return elem.buf;
    }
// Add a transaction to abort the outstanding DMA continuation
#[no_mangle]
unsafe extern "C" fn abort_dma_cont(qdev: *mut qaic_device, wrappers: *mut wrapper_list, dma_chunk_id: u32) -> c_int {
    static int abort_dma_cont(struct qaic_device *qdev, struct wrapper_list *wrappers, u32 dma_chunk_id)
    {
    struct wire_trans_dma_xfer *out_trans;
    let mut size: u32 = sizeof(*out_trans);
    struct wrapper_msg *wrapper;
    struct wrapper_msg *w;
    struct wire_msg *msg;
    wrapper = list_first_entry(&wrappers.list, struct wrapper_msg, list);
    msg = &wrapper.msg;
// Remove all but the first wrapper which has the msg header
    list_for_each_entry_safe(wrapper, w, &wrappers.list, list)
    if (!list_is_first(&wrapper.list, &wrappers.list))
    kref_put(&wrapper.ref_count, free_wrapper);
    wrapper = add_wrapper(wrappers, sizeof(*wrapper));
    if (!wrapper)
    return -ENOMEM;
    out_trans = (struct wire_trans_dma_xfer *)&wrapper.trans;
    out_trans.hdr.type = cpu_to_le32(QAIC_TRANS_DMA_XFER_TO_DEV);
    out_trans.hdr.len = cpu_to_le32(size);
    out_trans.tag = cpu_to_le32(0);
    out_trans.count = cpu_to_le32(0);
    out_trans.dma_chunk_id = cpu_to_le32(dma_chunk_id);
    msg.hdr.len = cpu_to_le32(size + sizeof(*msg));
    msg.hdr.count = cpu_to_le32(1);
    wrapper.len = size;
    return 0;
    }
    static struct wrapper_list *alloc_wrapper_list(void)
    {
    struct wrapper_list *wrappers;
    wrappers = kmalloc_obj(*wrappers);
    if (!wrappers)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&wrappers.list);
    spin_lock_init(&wrappers.lock);
    return wrappers;
    }
    static int qaic_manage_msg_xfer(struct qaic_device *qdev, struct qaic_user *usr,
    struct manage_msg *user_msg, struct ioctl_resources *resources,
    struct wire_msg **rsp)
    {
    struct wrapper_list *wrappers;
    struct wrapper_msg *wrapper;
    struct wrapper_msg *w;
    let mut all_done: bool = false;
    struct wire_msg *msg;
    int ret;
    wrappers = alloc_wrapper_list();
    if (!wrappers)
    return -ENOMEM;
    wrapper = add_wrapper(wrappers, sizeof(*wrapper));
    if (!wrapper) {
    kfree(wrappers);
    return -ENOMEM;
    }
    msg = &wrapper.msg;
    wrapper.len = sizeof(*msg);
    ret = encode_message(qdev, user_msg, wrappers, resources, usr);
    if (ret && resources.dma_chunk_id)
    ret = abort_dma_cont(qdev, wrappers, resources.dma_chunk_id);
    if (ret)
    goto encode_failed;
    ret = mutex_lock_interruptible(&qdev.cntl_mutex);
    if (ret)
    goto lock_failed;
    msg.hdr.magic_number = MANAGE_MAGIC_NUMBER;
    msg.hdr.sequence_number = cpu_to_le32(qdev.next_seq_num++);
    if (usr) {
    msg.hdr.handle = cpu_to_le32(usr.handle);
    msg.hdr.partition_id = cpu_to_le32(usr.qddev.partition_id);
    } else {
    msg.hdr.handle = 0;
    msg.hdr.partition_id = cpu_to_le32(QAIC_NO_PARTITION);
    }
    msg.hdr.padding = cpu_to_le32(0);
    msg.hdr.crc32 = cpu_to_le32(qdev.gen_crc(wrappers));
// msg_xfer releases the mutex
// rsp = msg_xfer(qdev, wrappers, qdev->next_seq_num - 1, false);
    if (IS_ERR(*rsp))
    ret = PTR_ERR(*rsp);
    lock_failed:
    free_dma_xfers(qdev, resources);
    encode_failed:
    spin_lock(&wrappers.lock);
    list_for_each_entry_safe(wrapper, w, &wrappers.list, list)
    kref_put(&wrapper.ref_count, free_wrapper);
    all_done = list_empty(&wrappers.list);
    spin_unlock(&wrappers.lock);
    if (all_done)
    kfree(wrappers);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qaic_manage(qdev: *mut qaic_device, usr: *mut qaic_user, user_msg: *mut manage_msg) -> c_int {
    static int qaic_manage(struct qaic_device *qdev, struct qaic_user *usr, struct manage_msg *user_msg)
    {
    struct wire_trans_dma_xfer_cont *dma_cont = core::ptr::null_mut();
    struct ioctl_resources resources;
    struct wire_msg *rsp = core::ptr::null_mut();
    int ret;
    memset(&resources, 0, sizeof(struct ioctl_resources));
    INIT_LIST_HEAD(&resources.dma_xfers);
    if (user_msg.len > QAIC_MANAGE_MAX_MSG_LENGTH ||
    user_msg.count > QAIC_MANAGE_MAX_MSG_LENGTH / sizeof(struct qaic_manage_trans_hdr))
    return -EINVAL;
    dma_xfer_continue:
    ret = qaic_manage_msg_xfer(qdev, usr, user_msg, &resources, &rsp);
    if (ret)
    return ret;
// dma_cont should be the only transaction if present
    if (le32_to_cpu(rsp.hdr.count) == 1) {
    dma_cont = (struct wire_trans_dma_xfer_cont *)rsp.data;
    if (le32_to_cpu(dma_cont.hdr.type) != QAIC_TRANS_DMA_XFER_CONT)
    dma_cont = core::ptr::null_mut();
    }
    if (dma_cont) {
    if (le32_to_cpu(dma_cont.dma_chunk_id) == resources.dma_chunk_id &&
    le64_to_cpu(dma_cont.xferred_size) == resources.xferred_dma_size) {
    kfree(rsp);
    goto dma_xfer_continue;
    }
    ret = -EINVAL;
    goto dma_cont_failed;
    }
    ret = decode_message(qdev, user_msg, rsp, &resources, usr, false);
    dma_cont_failed:
    free_dbc_buf(qdev, &resources);
    kfree(rsp);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_manage_ioctl(dev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int {
    int qaic_manage_ioctl(struct drm_device *dev, void *data, struct drm_file *file_priv)
    {
    struct qaic_manage_msg *user_msg = data;
    struct qaic_device *qdev;
    struct manage_msg *msg;
    struct qaic_user *usr;
    u8 __user *user_data;
    int qdev_rcu_id;
    int usr_rcu_id;
    int ret;
    if (user_msg.len > QAIC_MANAGE_MAX_MSG_LENGTH)
    return -EINVAL;
    usr = file_priv.driver_priv;
    usr_rcu_id = srcu_read_lock(&usr.qddev_lock);
    if (!usr.qddev) {
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return -ENODEV;
    }
    qdev = usr.qddev.qdev;
    qdev_rcu_id = srcu_read_lock(&qdev.dev_lock);
    if (qdev.dev_state != QAIC_ONLINE) {
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return -ENODEV;
    }
    msg = kzalloc(QAIC_MANAGE_MAX_MSG_LENGTH + sizeof(*msg), GFP_KERNEL);
    if (!msg) {
    ret = -ENOMEM;
    goto out;
    }
    msg.len = user_msg.len;
    msg.count = user_msg.count;
    user_data = u64_to_user_ptr(user_msg.data);
    if (copy_from_user(msg.data, user_data, user_msg.len)) {
    ret = -EFAULT;
    goto free_msg;
    }
    ret = qaic_manage(qdev, usr, msg);
//
// If the qaic_manage() is successful then we copy the message onto
// userspace memory but we have an exception for -ECANCELED.
// For -ECANCELED, it means that device has NACKed the message with a
// status error code which userspace would like to know.
//
    if (ret == -ECANCELED || !ret) {
    if (copy_to_user(user_data, msg.data, msg.len)) {
    ret = -EFAULT;
    } else {
    user_msg.len = msg.len;
    user_msg.count = msg.count;
    }
    }
    free_msg:
    kfree(msg);
    out:
    srcu_read_unlock(&qdev.dev_lock, qdev_rcu_id);
    srcu_read_unlock(&usr.qddev_lock, usr_rcu_id);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn get_cntl_version(qdev: *mut qaic_device, usr: *mut qaic_user, major: *mut u16, minor: *mut u16) -> c_int {
    int get_cntl_version(struct qaic_device *qdev, struct qaic_user *usr, u16 *major, u16 *minor)
    {
    struct qaic_manage_trans_status_from_dev *status_result;
    struct qaic_manage_trans_status_to_dev *status_query;
    struct manage_msg *user_msg;
    int ret;
    user_msg = kmalloc(sizeof(*user_msg) + sizeof(*status_result), GFP_KERNEL);
    if (!user_msg) {
    ret = -ENOMEM;
    goto out;
    }
    user_msg.len = sizeof(*status_query);
    user_msg.count = 1;
    status_query = (struct qaic_manage_trans_status_to_dev *)user_msg.data;
    status_query.hdr.type = QAIC_TRANS_STATUS_FROM_USR;
    status_query.hdr.len = sizeof(status_query.hdr);
    ret = qaic_manage(qdev, usr, user_msg);
    if (ret)
    goto kfree_user_msg;
    status_result = (struct qaic_manage_trans_status_from_dev *)user_msg.data;
// major = status_result->major;
// minor = status_result->minor;
    if (status_result.status_flags & BIT(0)) { /* device is using CRC */
// By default qdev->gen_crc is programmed to generate CRC
    qdev.valid_crc = valid_crc;
    } else {
// By default qdev->valid_crc is programmed to bypass CRC
    qdev.gen_crc = gen_crc_stub;
    }
    kfree_user_msg:
    kfree(user_msg);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn resp_worker(work: *mut work_struct) {
    static void resp_worker(struct work_struct *work)
    {
    struct resp_work *resp = container_of(work, struct resp_work, work);
    struct qaic_device *qdev = resp.qdev;
    struct wire_msg *msg = resp.buf;
    struct xfer_queue_elem *elem;
    struct xfer_queue_elem *i;
    let mut found: bool = false;
    mutex_lock(&qdev.cntl_mutex);
    list_for_each_entry_safe(elem, i, &qdev.cntl_xfer_list, list) {
    if (elem.seq_num == le32_to_cpu(msg.hdr.sequence_number)) {
    found = true;
    list_del_init(&elem.list);
    elem.buf = msg;
    complete_all(&elem.xfer_done);
    break;
    }
    }
    mutex_unlock(&qdev.cntl_mutex);
    if (!found) {
//
// The user might have gone away at this point without waiting
// for QAIC_TRANS_DEACTIVATE_FROM_DEV transaction coming from
// the device. If this is not handled correctly, the host will
// not know that the DBC[n] has been freed on the device.
// Due to this failure in synchronization between the device and
// the host, if another user requests to activate a network, and
// the device assigns DBC[n] again, save_dbc_buf() will hang,
// waiting for dbc[n]->in_use to be set to false, which will not
// happen unless the qaic_dev_reset_clean_local_state() gets
// called by resetting the device (or re-inserting the module).
//
// As a solution, we look for QAIC_TRANS_DEACTIVATE_FROM_DEV
// transactions in the message before disposing of it, then
// handle releasing the DBC resources.
//
// Since the user has gone away, if the device could not
// deactivate the network (status != 0), there is no way to
// enable and reassign the DBC to the user. We can put trust in
// the device that it will release all the active DBCs in
// response to the QAIC_TRANS_TERMINATE_TO_DEV transaction,
// otherwise, the user can issue an soc_reset to the device.
//
    decode_message(qdev, core::ptr::null_mut(), msg, core::ptr::null_mut(), core::ptr::null_mut(), true);
// request must have timed out, drop packet
    kfree(msg);
    }
    kfree(resp);
    }
#[no_mangle]
unsafe extern "C" fn free_wrapper_from_list(wrappers: *mut wrapper_list, wrapper: *mut wrapper_msg) {
    static void free_wrapper_from_list(struct wrapper_list *wrappers, struct wrapper_msg *wrapper)
    {
    let mut all_done: bool = false;
    spin_lock(&wrappers.lock);
    kref_put(&wrapper.ref_count, free_wrapper);
    all_done = list_empty(&wrappers.list);
    spin_unlock(&wrappers.lock);
    if (all_done)
    kfree(wrappers);
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_mhi_ul_xfer_cb(mhi_dev: *mut mhi_device, mhi_result: *mut mhi_result) {
    void qaic_mhi_ul_xfer_cb(struct mhi_device *mhi_dev, struct mhi_result *mhi_result)
    {
    struct wire_msg *msg = mhi_result.buf_addr;
    struct wrapper_msg *wrapper = container_of(msg, struct wrapper_msg, msg);
    free_wrapper_from_list(wrapper.head, wrapper);
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_mhi_dl_xfer_cb(mhi_dev: *mut mhi_device, mhi_result: *mut mhi_result) {
    void qaic_mhi_dl_xfer_cb(struct mhi_device *mhi_dev, struct mhi_result *mhi_result)
    {
    struct qaic_device *qdev = dev_get_drvdata(&mhi_dev.dev);
    struct wire_msg *msg = mhi_result.buf_addr;
    struct resp_work *resp;
    if (mhi_result.transaction_status || msg.hdr.magic_number != MANAGE_MAGIC_NUMBER) {
    kfree(msg);
    return;
    }
    resp = kmalloc_obj(*resp, GFP_ATOMIC);
    if (!resp) {
    kfree(msg);
    return;
    }
    INIT_WORK(&resp.work, resp_worker);
    resp.qdev = qdev;
    resp.buf = msg;
    queue_work(qdev.cntl_wq, &resp.work);
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_control_open(qdev: *mut qaic_device) -> c_int {
    int qaic_control_open(struct qaic_device *qdev)
    {
    if (!qdev.cntl_ch)
    return -ENODEV;
    qdev.cntl_lost_buf = false;
//
// By default qaic should assume that device has CRC enabled.
// Qaic comes to know if device has CRC enabled or disabled during the
// device status transaction, which is the first transaction performed
// on control channel.
//
// So CRC validation of first device status transaction response is
// ignored (by calling valid_crc_stub) and is done later during decoding
// if device has CRC enabled.
// Now that qaic knows whether device has CRC enabled or not it acts
// accordingly.
//
    qdev.gen_crc = gen_crc;
    qdev.valid_crc = valid_crc_stub;
    return mhi_prepare_for_transfer(qdev.cntl_ch);
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_control_close(qdev: *mut qaic_device) {
    void qaic_control_close(struct qaic_device *qdev)
    {
    mhi_unprepare_from_transfer(qdev.cntl_ch);
    }
#[no_mangle]
pub unsafe extern "C" fn qaic_release_usr(qdev: *mut qaic_device, usr: *mut qaic_user) {
    void qaic_release_usr(struct qaic_device *qdev, struct qaic_user *usr)
    {
    struct wire_trans_terminate_to_dev *trans;
    struct wrapper_list *wrappers;
    struct wrapper_msg *wrapper;
    struct wire_msg *msg;
    struct wire_msg *rsp;
    wrappers = alloc_wrapper_list();
    if (!wrappers)
    return;
    wrapper = add_wrapper(wrappers, sizeof(*wrapper) + sizeof(*msg) + sizeof(*trans));
    if (!wrapper)
    return;
    msg = &wrapper.msg;
    trans = (struct wire_trans_terminate_to_dev *)msg.data;
    trans.hdr.type = cpu_to_le32(QAIC_TRANS_TERMINATE_TO_DEV);
    trans.hdr.len = cpu_to_le32(sizeof(*trans));
    trans.handle = cpu_to_le32(usr.handle);
    mutex_lock(&qdev.cntl_mutex);
    wrapper.len = sizeof(msg.hdr) + sizeof(*trans);
    msg.hdr.magic_number = MANAGE_MAGIC_NUMBER;
    msg.hdr.sequence_number = cpu_to_le32(qdev.next_seq_num++);
    msg.hdr.len = cpu_to_le32(wrapper.len);
    msg.hdr.count = cpu_to_le32(1);
    msg.hdr.handle = cpu_to_le32(usr.handle);
    msg.hdr.padding = cpu_to_le32(0);
    msg.hdr.crc32 = cpu_to_le32(qdev.gen_crc(wrappers));
//
// msg_xfer releases the mutex
// We don't care about the return of msg_xfer since we will not do
// anything different based on what happens.
// We ignore pending signals since one will be set if the user is
// killed, and we need give the device a chance to cleanup, otherwise
// DMA may still be in progress when we return.
//
    rsp = msg_xfer(qdev, wrappers, qdev.next_seq_num - 1, true);
    if (!IS_ERR(rsp))
    kfree(rsp);
    free_wrapper_from_list(wrappers, wrapper);
    }
#[no_mangle]
pub unsafe extern "C" fn wake_all_cntl(qdev: *mut qaic_device) {
    void wake_all_cntl(struct qaic_device *qdev)
    {
    struct xfer_queue_elem *elem;
    struct xfer_queue_elem *i;
    mutex_lock(&qdev.cntl_mutex);
    list_for_each_entry_safe(elem, i, &qdev.cntl_xfer_list, list) {
    list_del_init(&elem.list);
    complete_all(&elem.xfer_done);
    }
    mutex_unlock(&qdev.cntl_mutex);
    }
