//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_scmi/shmem.c
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
// For transport using shared mem structure.
//
// Copyright (C) 2019-2024 ARM Ltd.
//

pub const SCMI_SHMEM_LAYOUT_OVERHEAD: c_int = 24;
//
// SCMI specification requires all parameters, message headers, return
// arguments or any protocol data to be expressed in little endian
// format only.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_shared_mem {
    pub reserved: __le32,
    pub channel_status: __le32,
    pub reserved1: [__le32; 2],
    pub flags: __le32,

    pub length: __le32,
    pub msg_header: __le32,
    pub msg_payload: [u8; ],
}

    static inline void shmem_memcpy_fromio32(void *to,
    const void __iomem *from,
    size_t count)
    {
    WARN_ON(!IS_ALIGNED((unsigned long)from, 4) ||
    !IS_ALIGNED((unsigned long)to, 4) ||
    count % 4);
    __ioread32_copy(to, from, count / 4);
    }
    static inline void shmem_memcpy_toio32(void __iomem *to,
    const void *from,
    size_t count)
    {
    WARN_ON(!IS_ALIGNED((unsigned long)to, 4) ||
    !IS_ALIGNED((unsigned long)from, 4) ||
    count % 4);
    __iowrite32_copy(to, from, count / 4);
    }
    static struct scmi_shmem_io_ops shmem_io_ops32 = {
    .fromio	= shmem_memcpy_fromio32,
    .toio	= shmem_memcpy_toio32,
    };
// Wrappers are needed for proper memcpy_{from,to}_io expansion by the
// pre-processor.
//
    static inline void shmem_memcpy_fromio(void *to,
    const void __iomem *from,
    size_t count)
    {
    memcpy_fromio(to, from, count);
    }
    static inline void shmem_memcpy_toio(void __iomem *to,
    const void *from,
    size_t count)
    {
    memcpy_toio(to, from, count);
    }
    static struct scmi_shmem_io_ops shmem_io_ops_default = {
    .fromio = shmem_memcpy_fromio,
    .toio	= shmem_memcpy_toio,
    };
    static void shmem_tx_prepare(struct scmi_shared_mem __iomem *shmem,
    struct scmi_xfer *xfer,
    struct scmi_chan_info *cinfo,
    shmem_copy_toio_t copy_toio)
    {
    ktime_t stop;
//
// Ideally channel must be free by now unless OS timeout last
// request and platform continued to process the same, wait
// until it releases the shared memory, otherwise we may endup
// overwriting its response with new message payload or vice-versa.
// Giving up anyway after twice the expected channel timeout so as
// not to bail-out on intermittent issues where the platform is
// occasionally a bit slower to answer.
//
// Note that after a timeout is detected we bail-out and carry on but
// the transport functionality is probably permanently compromised:
// this is just to ease debugging and avoid complete hangs on boot
// due to a misbehaving SCMI firmware.
//
    stop = ktime_add_ms(ktime_get(), 2 * cinfo.rx_timeout_ms);
    spin_until_cond((ioread32(&shmem.channel_status) &
    SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE) ||
    ktime_after(ktime_get(), stop));
    if (!(ioread32(&shmem.channel_status) &
    SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE)) {
    WARN_ON_ONCE(1);
    dev_err(cinfo.dev,
    "Timeout waiting for a free TX channel !\n");
    return;
    }
// Mark channel busy + clear error
    iowrite32(0x0, &shmem.channel_status);
    iowrite32(xfer.hdr.poll_completion ? 0 : SCMI_SHMEM_FLAG_INTR_ENABLED,
    &shmem.flags);
    iowrite32(sizeof(shmem.msg_header) + xfer.tx.len, &shmem.length);
    iowrite32(pack_scmi_header(&xfer.hdr), &shmem.msg_header);
    if (xfer.tx.buf)
    copy_toio(shmem.msg_payload, xfer.tx.buf, xfer.tx.len);
    }
#[no_mangle]
unsafe extern "C" fn shmem_read_header(shmem: *mut scmi_shared_mem __iomem) -> u32 {
    static u32 shmem_read_header(struct scmi_shared_mem __iomem *shmem)
    {
    return ioread32(&shmem.msg_header);
    }
    static void shmem_fetch_response(struct scmi_shared_mem __iomem *shmem,
    struct scmi_xfer *xfer,
    shmem_copy_fromio_t copy_fromio)
    {
    let mut len: usize = ioread32(&shmem.length);
    xfer.hdr.status = ioread32(shmem.msg_payload);
// Skip the length of header and status in shmem area i.e 8 bytes
    xfer.rx.len = min_t(size_t, xfer.rx.len, len > 8 ? len - 8 : 0);
// Take a copy to the rx buffer..
    copy_fromio(xfer.rx.buf, shmem.msg_payload + 4, xfer.rx.len);
    }
    static void shmem_fetch_notification(struct scmi_shared_mem __iomem *shmem,
    size_t max_len, struct scmi_xfer *xfer,
    shmem_copy_fromio_t copy_fromio)
    {
    let mut len: usize = ioread32(&shmem.length);
// Skip only the length of header in shmem area i.e 4 bytes
    xfer.rx.len = min_t(size_t, max_len, len > 4 ? len - 4 : 0);
// Take a copy to the rx buffer..
    copy_fromio(xfer.rx.buf, shmem.msg_payload, xfer.rx.len);
    }
#[no_mangle]
unsafe extern "C" fn shmem_clear_channel(shmem: *mut scmi_shared_mem __iomem) {
    static void shmem_clear_channel(struct scmi_shared_mem __iomem *shmem)
    {
    iowrite32(SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE, &shmem.channel_status);
    }
    static bool shmem_poll_done(struct scmi_shared_mem __iomem *shmem,
    struct scmi_xfer *xfer)
    {
    u16 xfer_id;
    xfer_id = MSG_XTRACT_TOKEN(ioread32(&shmem.msg_header));
    if (xfer.hdr.seq != xfer_id)
    return false;
    return ioread32(&shmem.channel_status) &
    (SCMI_SHMEM_CHAN_STAT_CHANNEL_ERROR |
    SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE);
    }
#[no_mangle]
unsafe extern "C" fn shmem_channel_free(shmem: *mut scmi_shared_mem __iomem) -> bool {
    static bool shmem_channel_free(struct scmi_shared_mem __iomem *shmem)
    {
    return (ioread32(&shmem.channel_status) &
    SCMI_SHMEM_CHAN_STAT_CHANNEL_FREE);
    }
#[no_mangle]
unsafe extern "C" fn shmem_channel_intr_enabled(shmem: *mut scmi_shared_mem __iomem) -> bool {
    static bool shmem_channel_intr_enabled(struct scmi_shared_mem __iomem *shmem)
    {
    return ioread32(&shmem.flags) & SCMI_SHMEM_FLAG_INTR_ENABLED;
    }
    static void __iomem *shmem_setup_iomap(struct scmi_chan_info *cinfo,
    struct device *dev, bool tx,
    struct resource *res,
    struct scmi_shmem_io_ops **ops)
    {
    const char *desc = tx ? "Tx" : "Rx";
    int ret, idx = tx ? 0 : 1;
    struct device *cdev = cinfo.dev;
    let mut lres: resource = {};
    resource_size_t size;
    void __iomem *addr;
    u32 reg_io_width;
    struct device_node *shmem __free(device_node) = of_parse_phandle(cdev.of_node,
    "shmem", idx);
    if (!shmem)
    return IOMEM_ERR_PTR(-ENODEV);
    if (!of_device_is_compatible(shmem, "arm,scmi-shmem"))
    return IOMEM_ERR_PTR(-ENXIO);
// Use a local on-stack as a working area when not provided
    if (!res)
    res = &lres;
    ret = of_address_to_resource(shmem, 0, res);
    if (ret) {
    dev_err(cdev, "failed to get SCMI %s shared memory\n", desc);
    return IOMEM_ERR_PTR(ret);
    }
    size = resource_size(res);
    if (cinfo.max_msg_size + SCMI_SHMEM_LAYOUT_OVERHEAD > size) {
    dev_err(dev, "misconfigured SCMI shared memory\n");
    return IOMEM_ERR_PTR(-ENOSPC);
    }
    addr = devm_ioremap(dev, res.start, size);
    if (!addr) {
    dev_err(dev, "failed to ioremap SCMI %s shared memory\n", desc);
    return IOMEM_ERR_PTR(-EADDRNOTAVAIL);
    }
    of_property_read_u32(shmem, "reg-io-width", &reg_io_width);
    switch (reg_io_width) {
    case 4:
// ops = &shmem_io_ops32;
    break;
    default:
// ops = &shmem_io_ops_default;
    break;
    }
    return addr;
    }
    static const struct scmi_shared_mem_operations scmi_shmem_ops = {
    .tx_prepare = shmem_tx_prepare,
    .read_header = shmem_read_header,
    .fetch_response = shmem_fetch_response,
    .fetch_notification = shmem_fetch_notification,
    .clear_channel = shmem_clear_channel,
    .poll_done = shmem_poll_done,
    .channel_free = shmem_channel_free,
    .channel_intr_enabled = shmem_channel_intr_enabled,
    .setup_iomap = shmem_setup_iomap,
    };
    const struct scmi_shared_mem_operations *scmi_shared_mem_operations_get(void)
    {
    return &scmi_shmem_ops;
    }
