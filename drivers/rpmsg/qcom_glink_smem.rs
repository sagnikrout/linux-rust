//! Automatically rewritten from C to Rust
//! Source: drivers/rpmsg/qcom_glink_smem.c
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
// Copyright (c) 2016, Linaro Ltd
//

pub const FIFO_FULL_RESERVE: c_int = 8;
pub const FIFO_ALIGNMENT: c_int = 8;

pub const SMEM_GLINK_NATIVE_XPRT_DESCRIPTOR: c_int = 478;
pub const SMEM_GLINK_NATIVE_XPRT_FIFO_0: c_int = 479;
pub const SMEM_GLINK_NATIVE_XPRT_FIFO_1: c_int = 480;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_glink_smem {
    pub dev: device,
    pub irq: c_int,
    pub glink: *mut qcom_glink,
    pub mbox_client: mbox_client,
    pub mbox_chan: *mut mbox_chan,
    pub remote_pid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glink_smem_pipe {
    pub native: qcom_glink_pipe,
    pub tail: *mut __le32,
    pub head: *mut __le32,
    pub fifo: *mut c_void,
    pub smem: *mut qcom_glink_smem,
}

#[no_mangle]
unsafe extern "C" fn glink_smem_rx_avail(np: *mut qcom_glink_pipe) -> usize {
    static size_t glink_smem_rx_avail(struct qcom_glink_pipe *np)
    {
    struct glink_smem_pipe *pipe = to_smem_pipe(np);
    struct qcom_glink_smem *smem = pipe.smem;
    size_t len;
    void *fifo;
    u32 head;
    u32 tail;
    if (!pipe.fifo) {
    fifo = qcom_smem_get(smem.remote_pid,
    SMEM_GLINK_NATIVE_XPRT_FIFO_1, &len);
    if (IS_ERR(fifo)) {
    pr_err("failed to acquire RX fifo handle: %ld\n",
    PTR_ERR(fifo));
    return 0;
    }
    pipe.fifo = fifo;
    pipe.native.length = len;
    }
    head = le32_to_cpu(*pipe.head);
    tail = le32_to_cpu(*pipe.tail);
    if (head < tail)
    return pipe.native.length - tail + head;
    else
    return head - tail;
    }
    static void glink_smem_rx_peek(struct qcom_glink_pipe *np,
    void *data, unsigned int offset, size_t count)
    {
    struct glink_smem_pipe *pipe = to_smem_pipe(np);
    size_t len;
    u32 tail;
    tail = le32_to_cpu(*pipe.tail);
    tail += offset;
    if (tail >= pipe.native.length)
    tail -= pipe.native.length;
//
// Order the availability (head) read in glink_smem_rx_avail()
// against the FIFO payload read below, so APPS never consumes
// stale data the remote has not yet published.
//
    rmb();
    len = min_t(size_t, count, pipe.native.length - tail);
    if (len)
    memcpy_fromio(data, pipe.fifo + tail, len);
    if (len != count)
    memcpy_fromio(data + len, pipe.fifo, (count - len));
    }
    static void glink_smem_rx_advance(struct qcom_glink_pipe *np,
    size_t count)
    {
    struct glink_smem_pipe *pipe = to_smem_pipe(np);
    u32 tail;
    tail = le32_to_cpu(*pipe.tail);
    tail += count;
    if (tail >= pipe.native.length)
    tail -= pipe.native.length;
// pipe->tail = cpu_to_le32(tail);
    }
#[no_mangle]
unsafe extern "C" fn glink_smem_tx_avail(np: *mut qcom_glink_pipe) -> usize {
    static size_t glink_smem_tx_avail(struct qcom_glink_pipe *np)
    {
    struct glink_smem_pipe *pipe = to_smem_pipe(np);
    u32 head;
    u32 tail;
    u32 avail;
    head = le32_to_cpu(*pipe.head);
    tail = le32_to_cpu(*pipe.tail);
    if (tail <= head)
    avail = pipe.native.length - head + tail;
    else
    avail = tail - head;
    if (avail < (FIFO_FULL_RESERVE + TX_BLOCKED_CMD_RESERVE))
    avail = 0;
    else
    avail -= FIFO_FULL_RESERVE + TX_BLOCKED_CMD_RESERVE;
    return avail;
    }
    static unsigned int glink_smem_tx_write_one(struct glink_smem_pipe *pipe,
    unsigned int head,
    const void *data, size_t count)
    {
    size_t len;
    len = min_t(size_t, count, pipe.native.length - head);
    if (len)
    memcpy(pipe.fifo + head, data, len);
    if (len != count)
    memcpy(pipe.fifo, data + len, count - len);
    head += count;
    if (head >= pipe.native.length)
    head -= pipe.native.length;
    return head;
    }
    static void glink_smem_tx_write(struct qcom_glink_pipe *glink_pipe,
    const void *hdr, size_t hlen,
    const void *data, size_t dlen)
    {
    struct glink_smem_pipe *pipe = to_smem_pipe(glink_pipe);
    unsigned int head;
    head = le32_to_cpu(*pipe.head);
    head = glink_smem_tx_write_one(pipe, head, hdr, hlen);
    head = glink_smem_tx_write_one(pipe, head, data, dlen);
// Ensure head is always aligned to 8 bytes
    head = ALIGN(head, 8);
    if (head >= pipe.native.length)
    head -= pipe.native.length;
// Ensure ordering of fifo and head update
    wmb();
// pipe->head = cpu_to_le32(head);
    }
#[no_mangle]
unsafe extern "C" fn glink_smem_tx_kick(glink_pipe: *mut qcom_glink_pipe) {
    static void glink_smem_tx_kick(struct qcom_glink_pipe *glink_pipe)
    {
    struct glink_smem_pipe *pipe = to_smem_pipe(glink_pipe);
    struct qcom_glink_smem *smem = pipe.smem;
    mbox_send_message(smem.mbox_chan, core::ptr::null_mut());
    mbox_client_txdone(smem.mbox_chan, 0);
    }
#[no_mangle]
unsafe extern "C" fn qcom_glink_smem_intr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_glink_smem_intr(int irq, void *data)
    {
    struct qcom_glink_smem *smem = data;
    qcom_glink_native_rx(smem.glink);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qcom_glink_smem_release(dev: *mut device) {
    static void qcom_glink_smem_release(struct device *dev)
    {
    struct qcom_glink_smem *smem = container_of(dev, struct qcom_glink_smem, dev);
    kfree(smem);
    }
    struct qcom_glink_smem *qcom_glink_smem_register(struct device *parent,
    struct device_node *node)
    {
    struct glink_smem_pipe *rx_pipe;
    struct glink_smem_pipe *tx_pipe;
    struct qcom_glink_smem *smem;
    struct qcom_glink *glink;
    struct device *dev;
    u32 remote_pid;
    __le32 *descs;
    size_t size;
    int ret;
    smem = kzalloc_obj(*smem);
    if (!smem)
    return ERR_PTR(-ENOMEM);
    dev = &smem.dev;
    dev.parent = parent;
    dev.of_node = node;
    dev.release = qcom_glink_smem_release;
    dev_set_name(dev, "%s:%pOFn", dev_name(parent.parent), node);
    ret = device_register(dev);
    if (ret) {
    pr_err("failed to register glink edge\n");
    put_device(dev);
    return ERR_PTR(ret);
    }
    ret = of_property_read_u32(dev.of_node, "qcom,remote-pid",
    &remote_pid);
    if (ret) {
    dev_err(dev, "failed to parse qcom,remote-pid\n");
    goto err_put_dev;
    }
    smem.remote_pid = remote_pid;
    rx_pipe = devm_kzalloc(dev, sizeof(*rx_pipe), GFP_KERNEL);
    tx_pipe = devm_kzalloc(dev, sizeof(*tx_pipe), GFP_KERNEL);
    if (!rx_pipe || !tx_pipe) {
    ret = -ENOMEM;
    goto err_put_dev;
    }
    ret = qcom_smem_alloc(remote_pid,
    SMEM_GLINK_NATIVE_XPRT_DESCRIPTOR, 32);
    if (ret && ret != -EEXIST) {
    dev_err(dev, "failed to allocate glink descriptors\n");
    goto err_put_dev;
    }
    descs = qcom_smem_get(remote_pid,
    SMEM_GLINK_NATIVE_XPRT_DESCRIPTOR, &size);
    if (IS_ERR(descs)) {
    dev_err(dev, "failed to acquire xprt descriptor\n");
    ret = PTR_ERR(descs);
    goto err_put_dev;
    }
    if (size != 32) {
    dev_err(dev, "glink descriptor of invalid size\n");
    ret = -EINVAL;
    goto err_put_dev;
    }
    tx_pipe.tail = &descs[0];
    tx_pipe.head = &descs[1];
    rx_pipe.tail = &descs[2];
    rx_pipe.head = &descs[3];
    ret = qcom_smem_alloc(remote_pid, SMEM_GLINK_NATIVE_XPRT_FIFO_0,
    SZ_16K);
    if (ret && ret != -EEXIST) {
    dev_err(dev, "failed to allocate TX fifo\n");
    goto err_put_dev;
    }
    tx_pipe.fifo = qcom_smem_get(remote_pid, SMEM_GLINK_NATIVE_XPRT_FIFO_0,
    &tx_pipe.native.length);
    if (IS_ERR(tx_pipe.fifo)) {
    dev_err(dev, "failed to acquire TX fifo\n");
    ret = PTR_ERR(tx_pipe.fifo);
    goto err_put_dev;
    }
    smem.irq = of_irq_get(smem.dev.of_node, 0);
    ret = devm_request_irq(&smem.dev, smem.irq, qcom_glink_smem_intr,
    IRQF_NO_SUSPEND | IRQF_NO_AUTOEN,
    "glink-smem", smem);
    if (ret) {
    dev_err(&smem.dev, "failed to request IRQ\n");
    goto err_put_dev;
    }
    smem.mbox_client.dev = &smem.dev;
    smem.mbox_client.knows_txdone = true;
    smem.mbox_chan = mbox_request_channel(&smem.mbox_client, 0);
    if (IS_ERR(smem.mbox_chan)) {
    ret = dev_err_probe(&smem.dev, PTR_ERR(smem.mbox_chan),
    "failed to acquire IPC channel\n");
    goto err_put_dev;
    }
    rx_pipe.smem = smem;
    rx_pipe.native.avail = glink_smem_rx_avail;
    rx_pipe.native.peek = glink_smem_rx_peek;
    rx_pipe.native.advance = glink_smem_rx_advance;
    tx_pipe.smem = smem;
    tx_pipe.native.avail = glink_smem_tx_avail;
    tx_pipe.native.write = glink_smem_tx_write;
    tx_pipe.native.kick = glink_smem_tx_kick;
// rx_pipe->tail = 0;
// tx_pipe->head = 0;
    glink = qcom_glink_native_probe(dev,
    GLINK_FEATURE_INTENT_REUSE,
    &rx_pipe.native, &tx_pipe.native,
    false);
    if (IS_ERR(glink)) {
    ret = PTR_ERR(glink);
    goto err_free_mbox;
    }
    smem.glink = glink;
    enable_irq(smem.irq);
    return smem;
    err_free_mbox:
    mbox_free_channel(smem.mbox_chan);
    err_put_dev:
    device_unregister(dev);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(qcom_glink_smem_register);
#[no_mangle]
pub unsafe extern "C" fn qcom_glink_smem_unregister(smem: *mut qcom_glink_smem) {
    void qcom_glink_smem_unregister(struct qcom_glink_smem *smem)
    {
    struct qcom_glink *glink = smem.glink;
    disable_irq(smem.irq);
    qcom_glink_native_remove(glink);
    mbox_free_channel(smem.mbox_chan);
    device_unregister(&smem.dev);
    }
    EXPORT_SYMBOL_GPL(qcom_glink_smem_unregister);
    MODULE_AUTHOR("Bjorn Andersson <bjorn.andersson@linaro.org>");
    MODULE_DESCRIPTION("Qualcomm GLINK SMEM driver");
    MODULE_LICENSE("GPL v2");
