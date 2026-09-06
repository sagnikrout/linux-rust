//! Automatically rewritten from C to Rust
//! Source: drivers/rpmsg/qcom_glink_rpm.c
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
// Copyright (c) 2016-2017, Linaro Ltd
//

pub const RPM_TOC_SIZE: c_int = 256;
pub const RPM_TOC_MAGIC: c_uint = 0x67727430 /* grt0 */;

    sizeof(struct rpm_toc_entry))
pub const RPM_TX_FIFO_ID: c_uint = 0x61703272 /* ap2r */;
pub const RPM_RX_FIFO_ID: c_uint = 0x72326170 /* r2ap */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpm_toc_entry {
    pub id: __le32,
    pub offset: __le32,
    pub size: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpm_toc {
    pub magic: __le32,
    pub count: __le32,
    pub entries: [rpm_toc_entry; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct glink_rpm_pipe {
    pub native: qcom_glink_pipe,
    pub tail: *mut void __iomem,
    pub head: *mut void __iomem,
    pub fifo: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glink_rpm {
    pub glink: *mut qcom_glink,
    pub irq: c_int,
    pub mbox_client: mbox_client,
    pub mbox_chan: *mut mbox_chan,
    pub rx_pipe: glink_rpm_pipe,
    pub tx_pipe: glink_rpm_pipe,
}

#[no_mangle]
unsafe extern "C" fn glink_rpm_rx_avail(glink_pipe: *mut qcom_glink_pipe) -> usize {
    static size_t glink_rpm_rx_avail(struct qcom_glink_pipe *glink_pipe)
    {
    struct glink_rpm_pipe *pipe = to_rpm_pipe(glink_pipe);
    unsigned int head;
    unsigned int tail;
    head = readl(pipe.head);
    tail = readl(pipe.tail);
    if (head < tail)
    return pipe.native.length - tail + head;
    else
    return head - tail;
    }
    static void glink_rpm_rx_peek(struct qcom_glink_pipe *glink_pipe,
    void *data, unsigned int offset, size_t count)
    {
    struct glink_rpm_pipe *pipe = to_rpm_pipe(glink_pipe);
    unsigned int tail;
    size_t len;
    tail = readl(pipe.tail);
    tail += offset;
    if (tail >= pipe.native.length)
    tail -= pipe.native.length;
    len = min_t(size_t, count, pipe.native.length - tail);
    if (len) {
    __ioread32_copy(data, pipe.fifo + tail,
    len / sizeof(u32));
    }
    if (len != count) {
    __ioread32_copy(data + len, pipe.fifo,
    (count - len) / sizeof(u32));
    }
    }
    static void glink_rpm_rx_advance(struct qcom_glink_pipe *glink_pipe,
    size_t count)
    {
    struct glink_rpm_pipe *pipe = to_rpm_pipe(glink_pipe);
    unsigned int tail;
    tail = readl(pipe.tail);
    tail += count;
    if (tail >= pipe.native.length)
    tail -= pipe.native.length;
    writel(tail, pipe.tail);
    }
#[no_mangle]
unsafe extern "C" fn glink_rpm_tx_avail(glink_pipe: *mut qcom_glink_pipe) -> usize {
    static size_t glink_rpm_tx_avail(struct qcom_glink_pipe *glink_pipe)
    {
    struct glink_rpm_pipe *pipe = to_rpm_pipe(glink_pipe);
    unsigned int head;
    unsigned int tail;
    head = readl(pipe.head);
    tail = readl(pipe.tail);
    if (tail <= head)
    return pipe.native.length - head + tail;
    else
    return tail - head;
    }
    static unsigned int glink_rpm_tx_write_one(struct glink_rpm_pipe *pipe,
    unsigned int head,
    const void *data, size_t count)
    {
    size_t len;
    len = min_t(size_t, count, pipe.native.length - head);
    if (len) {
    __iowrite32_copy(pipe.fifo + head, data,
    len / sizeof(u32));
    }
    if (len != count) {
    __iowrite32_copy(pipe.fifo, data + len,
    (count - len) / sizeof(u32));
    }
    head += count;
    if (head >= pipe.native.length)
    head -= pipe.native.length;
    return head;
    }
    static void glink_rpm_tx_write(struct qcom_glink_pipe *glink_pipe,
    const void *hdr, size_t hlen,
    const void *data, size_t dlen)
    {
    struct glink_rpm_pipe *pipe = to_rpm_pipe(glink_pipe);
    let mut tlen: usize = hlen + dlen;
    size_t aligned_dlen;
    unsigned int head;
    char padding[8] = {0};
    size_t pad;
// Header length comes from glink native and is always 4 byte aligned
    if (WARN(hlen % 4, "Glink Header length must be 4 bytes aligned\n"))
    return;
//
// Move the unaligned tail of the message to the padding chunk, to
// ensure word aligned accesses
//
    aligned_dlen = ALIGN_DOWN(dlen, 4);
    if (aligned_dlen != dlen)
    memcpy(padding, data + aligned_dlen, dlen - aligned_dlen);
    head = readl(pipe.head);
    head = glink_rpm_tx_write_one(pipe, head, hdr, hlen);
    head = glink_rpm_tx_write_one(pipe, head, data, aligned_dlen);
    pad = ALIGN(tlen, 8) - ALIGN_DOWN(tlen, 4);
    if (pad)
    head = glink_rpm_tx_write_one(pipe, head, padding, pad);
    writel(head, pipe.head);
    }
#[no_mangle]
unsafe extern "C" fn glink_rpm_tx_kick(glink_pipe: *mut qcom_glink_pipe) {
    static void glink_rpm_tx_kick(struct qcom_glink_pipe *glink_pipe)
    {
    struct glink_rpm_pipe *pipe = to_rpm_pipe(glink_pipe);
    struct glink_rpm *rpm = container_of(pipe, struct glink_rpm, tx_pipe);
    mbox_send_message(rpm.mbox_chan, core::ptr::null_mut());
    mbox_client_txdone(rpm.mbox_chan, 0);
    }
#[no_mangle]
unsafe extern "C" fn qcom_glink_rpm_intr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_glink_rpm_intr(int irq, void *data)
    {
    struct glink_rpm *rpm = data;
    qcom_glink_native_rx(rpm.glink);
    return IRQ_HANDLED;
    }
    static int glink_rpm_parse_toc(struct device *dev,
    void __iomem *msg_ram,
    size_t msg_ram_size,
    struct glink_rpm_pipe *rx,
    struct glink_rpm_pipe *tx)
    {
    struct rpm_toc *toc;
    int num_entries;
    unsigned int id;
    size_t offset;
    size_t size;
    void *buf;
    int i;
    buf = kzalloc(RPM_TOC_SIZE, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    __ioread32_copy(buf, msg_ram + msg_ram_size - RPM_TOC_SIZE,
    RPM_TOC_SIZE / sizeof(u32));
    toc = buf;
    if (le32_to_cpu(toc.magic) != RPM_TOC_MAGIC) {
    dev_err(dev, "RPM TOC has invalid magic\n");
    goto err_inval;
    }
    num_entries = le32_to_cpu(toc.count);
    if (num_entries > RPM_TOC_MAX_ENTRIES) {
    dev_err(dev, "Invalid number of toc entries\n");
    goto err_inval;
    }
    for (i = 0; i < num_entries; i++) {
    id = le32_to_cpu(toc.entries[i].id);
    offset = le32_to_cpu(toc.entries[i].offset);
    size = le32_to_cpu(toc.entries[i].size);
    if (offset > msg_ram_size || offset + size > msg_ram_size) {
    dev_err(dev, "TOC entry with invalid size\n");
    continue;
    }
    switch (id) {
    case RPM_RX_FIFO_ID:
    rx.native.length = size;
    rx.tail = msg_ram + offset;
    rx.head = msg_ram + offset + sizeof(u32);
    rx.fifo = msg_ram + offset + 2 * sizeof(u32);
    break;
    case RPM_TX_FIFO_ID:
    tx.native.length = size;
    tx.tail = msg_ram + offset;
    tx.head = msg_ram + offset + sizeof(u32);
    tx.fifo = msg_ram + offset + 2 * sizeof(u32);
    break;
    }
    }
    if (!rx.fifo || !tx.fifo) {
    dev_err(dev, "Unable to find rx and tx descriptors\n");
    goto err_inval;
    }
    kfree(buf);
    return 0;
    err_inval:
    kfree(buf);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn glink_rpm_probe(pdev: *mut platform_device) -> c_int {
    static int glink_rpm_probe(struct platform_device *pdev)
    {
    struct qcom_glink *glink;
    struct glink_rpm *rpm;
    struct device_node *np;
    void __iomem *msg_ram;
    size_t msg_ram_size;
    struct device *dev = &pdev.dev;
    struct resource r;
    int ret;
    rpm = devm_kzalloc(&pdev.dev, sizeof(*rpm), GFP_KERNEL);
    if (!rpm)
    return -ENOMEM;
    np = of_parse_phandle(dev.of_node, "qcom,rpm-msg-ram", 0);
    ret = of_address_to_resource(np, 0, &r);
    of_node_put(np);
    if (ret)
    return ret;
    msg_ram = devm_ioremap(dev, r.start, resource_size(&r));
    msg_ram_size = resource_size(&r);
    if (!msg_ram)
    return -ENOMEM;
    ret = glink_rpm_parse_toc(dev, msg_ram, msg_ram_size,
    &rpm.rx_pipe, &rpm.tx_pipe);
    if (ret)
    return ret;
    rpm.irq = of_irq_get(dev.of_node, 0);
    ret = devm_request_irq(dev, rpm.irq, qcom_glink_rpm_intr,
    IRQF_NO_SUSPEND | IRQF_NO_AUTOEN,
    "glink-rpm", rpm);
    if (ret) {
    dev_err(dev, "failed to request IRQ\n");
    return ret;
    }
    rpm.mbox_client.dev = dev;
    rpm.mbox_client.knows_txdone = true;
    rpm.mbox_chan = mbox_request_channel(&rpm.mbox_client, 0);
    if (IS_ERR(rpm.mbox_chan))
    return dev_err_probe(dev, PTR_ERR(rpm.mbox_chan), "failed to acquire IPC channel\n");
// Pipe specific accessors
    rpm.rx_pipe.native.avail = glink_rpm_rx_avail;
    rpm.rx_pipe.native.peek = glink_rpm_rx_peek;
    rpm.rx_pipe.native.advance = glink_rpm_rx_advance;
    rpm.tx_pipe.native.avail = glink_rpm_tx_avail;
    rpm.tx_pipe.native.write = glink_rpm_tx_write;
    rpm.tx_pipe.native.kick = glink_rpm_tx_kick;
    writel(0, rpm.tx_pipe.head);
    writel(0, rpm.rx_pipe.tail);
    glink = qcom_glink_native_probe(dev,
    0,
    &rpm.rx_pipe.native,
    &rpm.tx_pipe.native,
    true);
    if (IS_ERR(glink)) {
    mbox_free_channel(rpm.mbox_chan);
    return PTR_ERR(glink);
    }
    rpm.glink = glink;
    platform_set_drvdata(pdev, rpm);
    enable_irq(rpm.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn glink_rpm_remove(pdev: *mut platform_device) {
    static void glink_rpm_remove(struct platform_device *pdev)
    {
    struct glink_rpm *rpm = platform_get_drvdata(pdev);
    struct qcom_glink *glink = rpm.glink;
    disable_irq(rpm.irq);
    qcom_glink_native_remove(glink);
    mbox_free_channel(rpm.mbox_chan);
    }
    static const struct of_device_id glink_rpm_of_match[] = {
    { .compatible = "qcom,glink-rpm" },
    {}
    };
    MODULE_DEVICE_TABLE(of, glink_rpm_of_match);
    static struct platform_driver glink_rpm_driver = {
    .probe = glink_rpm_probe,
    .remove = glink_rpm_remove,
    .driver = {
    .name = "qcom_glink_rpm",
    .of_match_table = glink_rpm_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn glink_rpm_init() -> int __init {
    static int __init glink_rpm_init(void)
    {
    return platform_driver_register(&glink_rpm_driver);
    }
    subsys_initcall(glink_rpm_init);
#[no_mangle]
unsafe extern "C" fn glink_rpm_exit() -> void __exit {
    static void __exit glink_rpm_exit(void)
    {
    platform_driver_unregister(&glink_rpm_driver);
    }
    module_exit(glink_rpm_exit);
    MODULE_AUTHOR("Bjorn Andersson <bjorn.andersson@linaro.org>");
    MODULE_DESCRIPTION("Qualcomm GLINK RPM driver");
    MODULE_LICENSE("GPL v2");
