//! Automatically rewritten from C to Rust
//! Source: drivers/peci/request.c
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
// Copyright (c) 2021 Intel Corporation

pub const PECI_GET_DIB_CMD: c_uint = 0xf7;
pub const PECI_GET_DIB_WR_LEN: c_int = 1;
pub const PECI_GET_DIB_RD_LEN: c_int = 8;
pub const PECI_GET_TEMP_CMD: c_uint = 0x01;
pub const PECI_GET_TEMP_WR_LEN: c_int = 1;
pub const PECI_GET_TEMP_RD_LEN: c_int = 2;
pub const PECI_RDPKGCFG_CMD: c_uint = 0xa1;
pub const PECI_RDPKGCFG_WR_LEN: c_int = 5;
pub const PECI_RDPKGCFG_RD_LEN_BASE: c_int = 1;
pub const PECI_WRPKGCFG_CMD: c_uint = 0xa5;
pub const PECI_WRPKGCFG_WR_LEN_BASE: c_int = 6;
pub const PECI_WRPKGCFG_RD_LEN: c_int = 1;
pub const PECI_RDIAMSR_CMD: c_uint = 0xb1;
pub const PECI_RDIAMSR_WR_LEN: c_int = 5;
pub const PECI_RDIAMSR_RD_LEN: c_int = 9;
pub const PECI_WRIAMSR_CMD: c_uint = 0xb5;
pub const PECI_RDIAMSREX_CMD: c_uint = 0xd1;
pub const PECI_RDIAMSREX_WR_LEN: c_int = 6;
pub const PECI_RDIAMSREX_RD_LEN: c_int = 9;
pub const PECI_RDPCICFG_CMD: c_uint = 0x61;
pub const PECI_RDPCICFG_WR_LEN: c_int = 6;
pub const PECI_RDPCICFG_RD_LEN: c_int = 5;
pub const PECI_RDPCICFG_RD_LEN_MAX: c_int = 24;
pub const PECI_WRPCICFG_CMD: c_uint = 0x65;
pub const PECI_RDPCICFGLOCAL_CMD: c_uint = 0xe1;
pub const PECI_RDPCICFGLOCAL_WR_LEN: c_int = 5;
pub const PECI_RDPCICFGLOCAL_RD_LEN_BASE: c_int = 1;
pub const PECI_WRPCICFGLOCAL_CMD: c_uint = 0xe5;
pub const PECI_WRPCICFGLOCAL_WR_LEN_BASE: c_int = 6;
pub const PECI_WRPCICFGLOCAL_RD_LEN: c_int = 1;
pub const PECI_ENDPTCFG_TYPE_LOCAL_PCI: c_uint = 0x03;
pub const PECI_ENDPTCFG_TYPE_PCI: c_uint = 0x04;
pub const PECI_ENDPTCFG_TYPE_MMIO: c_uint = 0x05;
pub const PECI_ENDPTCFG_ADDR_TYPE_PCI: c_uint = 0x04;
pub const PECI_ENDPTCFG_ADDR_TYPE_MMIO_D: c_uint = 0x05;
pub const PECI_ENDPTCFG_ADDR_TYPE_MMIO_Q: c_uint = 0x06;
pub const PECI_RDENDPTCFG_CMD: c_uint = 0xc1;
pub const PECI_RDENDPTCFG_PCI_WR_LEN: c_int = 12;
pub const PECI_RDENDPTCFG_MMIO_WR_LEN_BASE: c_int = 10;
pub const PECI_RDENDPTCFG_MMIO_D_WR_LEN: c_int = 14;
pub const PECI_RDENDPTCFG_MMIO_Q_WR_LEN: c_int = 18;
pub const PECI_RDENDPTCFG_RD_LEN_BASE: c_int = 1;
pub const PECI_WRENDPTCFG_CMD: c_uint = 0xc5;
pub const PECI_WRENDPTCFG_PCI_WR_LEN_BASE: c_int = 13;
pub const PECI_WRENDPTCFG_MMIO_D_WR_LEN_BASE: c_int = 15;
pub const PECI_WRENDPTCFG_MMIO_Q_WR_LEN_BASE: c_int = 19;
pub const PECI_WRENDPTCFG_RD_LEN: c_int = 1;
// Device Specific Completion Code (CC) Definition
pub const PECI_CC_SUCCESS: c_uint = 0x40;
pub const PECI_CC_NEED_RETRY: c_uint = 0x80;
pub const PECI_CC_OUT_OF_RESOURCE: c_uint = 0x81;
pub const PECI_CC_UNAVAIL_RESOURCE: c_uint = 0x82;
pub const PECI_CC_INVALID_REQ: c_uint = 0x90;
pub const PECI_CC_MCA_ERROR: c_uint = 0x91;
pub const PECI_CC_CATASTROPHIC_MCA_ERROR: c_uint = 0x93;
pub const PECI_CC_FATAL_MCA_ERROR: c_uint = 0x94;
pub const PECI_CC_PARITY_ERR_GPSB_OR_PMSB: c_uint = 0x98;
pub const PECI_CC_PARITY_ERR_GPSB_OR_PMSB_IERR: c_uint = 0x9B;
pub const PECI_CC_PARITY_ERR_GPSB_OR_PMSB_MCA: c_uint = 0x9C;

#[no_mangle]
unsafe extern "C" fn peci_request_data_cc(req: *mut peci_request) -> u8 {
    static u8 peci_request_data_cc(struct peci_request *req)
    {
    return req.rx.buf[0];
    }
//
// peci_request_status() - return -errno based on PECI completion code
// @req: the PECI request that contains response data with completion code
//
// It can't be used for Ping(), GetDIB() and GetTemp() - for those commands we
// don't expect completion code in the response.
//
// Return: -errno
//
#[no_mangle]
pub unsafe extern "C" fn peci_request_status(req: *mut peci_request) -> c_int {
    int peci_request_status(struct peci_request *req)
    {
    let mut cc: u8 = peci_request_data_cc(req);
    if (cc != PECI_CC_SUCCESS)
    dev_dbg(&req.device.dev, "ret: %#02x\n", cc);
    switch (cc) {
    case PECI_CC_SUCCESS:
    return 0;
    case PECI_CC_NEED_RETRY:
    case PECI_CC_OUT_OF_RESOURCE:
    case PECI_CC_UNAVAIL_RESOURCE:
    return -EAGAIN;
    case PECI_CC_INVALID_REQ:
    return -EINVAL;
    case PECI_CC_MCA_ERROR:
    case PECI_CC_CATASTROPHIC_MCA_ERROR:
    case PECI_CC_FATAL_MCA_ERROR:
    case PECI_CC_PARITY_ERR_GPSB_OR_PMSB:
    case PECI_CC_PARITY_ERR_GPSB_OR_PMSB_IERR:
    case PECI_CC_PARITY_ERR_GPSB_OR_PMSB_MCA:
    return -EIO;
    }
    WARN_ONCE(1, "Unknown PECI completion code: %#02x\n", cc);
    return -EIO;
    }
    EXPORT_SYMBOL_NS_GPL(peci_request_status, "PECI");
#[no_mangle]
unsafe extern "C" fn peci_request_xfer(req: *mut peci_request) -> c_int {
    static int peci_request_xfer(struct peci_request *req)
    {
    struct peci_device *device = req.device;
    struct peci_controller *controller = to_peci_controller(device.dev.parent);
    int ret;
    mutex_lock(&controller.bus_lock);
    ret = controller.ops.xfer(controller, device.addr, req);
    mutex_unlock(&controller.bus_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn peci_request_xfer_retry(req: *mut peci_request) -> c_int {
    static int peci_request_xfer_retry(struct peci_request *req)
    {
    let mut wait_interval: c_long = PECI_RETRY_INTERVAL_MIN;
    struct peci_device *device = req.device;
    struct peci_controller *controller = to_peci_controller(device.dev.parent);
    let mut start: c_ulong = jiffies;
    int ret;
// Don't try to use it for ping
    if (WARN_ON(req.tx.len == 0))
    return 0;
    do {
    ret = peci_request_xfer(req);
    if (ret) {
    dev_dbg(&controller.dev, "xfer error: %d\n", ret);
    return ret;
    }
    if (peci_request_status(req) != -EAGAIN)
    return 0;
// Set the retry bit to indicate a retry attempt
    req.tx.buf[1] |= PECI_RETRY_BIT;
    if (schedule_timeout_interruptible(wait_interval))
    return -ERESTARTSYS;
    wait_interval = min_t(long, wait_interval * 2, PECI_RETRY_INTERVAL_MAX);
    } while (time_before(jiffies, start + PECI_RETRY_TIMEOUT));
    dev_dbg(&controller.dev, "request timed out\n");
    return -ETIMEDOUT;
    }
//
// peci_request_alloc() - allocate &struct peci_requests
// @device: PECI device to which request is going to be sent
// @tx_len: TX length
// @rx_len: RX length
//
// Return: A pointer to a newly allocated &struct peci_request on success or NULL otherwise.
//
    struct peci_request *peci_request_alloc(struct peci_device *device, u8 tx_len, u8 rx_len)
    {
    struct peci_request *req;
//
// TX and RX buffers are fixed length members of peci_request, this is
// just a warn for developers to make sure to expand the buffers (or
// change the allocation method) if we go over the current limit.
//
    if (WARN_ON_ONCE(tx_len > PECI_REQUEST_MAX_BUF_SIZE || rx_len > PECI_REQUEST_MAX_BUF_SIZE))
    return core::ptr::null_mut();
//
// PECI controllers that we are using now don't support DMA, this
// should be converted to DMA API once support for controllers that do
// allow it is added to avoid an extra copy.
//
    req = kzalloc_obj(*req);
    if (!req)
    return core::ptr::null_mut();
    req.device = device;
    req.tx.len = tx_len;
    req.rx.len = rx_len;
    return req;
    }
    EXPORT_SYMBOL_NS_GPL(peci_request_alloc, "PECI");
//
// peci_request_free() - free peci_request
// @req: the PECI request to be freed
//
#[no_mangle]
pub unsafe extern "C" fn peci_request_free(req: *mut peci_request) {
    void peci_request_free(struct peci_request *req)
    {
    kfree(req);
    }
    EXPORT_SYMBOL_NS_GPL(peci_request_free, "PECI");
    struct peci_request *peci_xfer_get_dib(struct peci_device *device)
    {
    struct peci_request *req;
    int ret;
    req = peci_request_alloc(device, PECI_GET_DIB_WR_LEN, PECI_GET_DIB_RD_LEN);
    if (!req)
    return ERR_PTR(-ENOMEM);
    req.tx.buf[0] = PECI_GET_DIB_CMD;
    ret = peci_request_xfer(req);
    if (ret) {
    peci_request_free(req);
    return ERR_PTR(ret);
    }
    return req;
    }
    EXPORT_SYMBOL_NS_GPL(peci_xfer_get_dib, "PECI");
    struct peci_request *peci_xfer_get_temp(struct peci_device *device)
    {
    struct peci_request *req;
    int ret;
    req = peci_request_alloc(device, PECI_GET_TEMP_WR_LEN, PECI_GET_TEMP_RD_LEN);
    if (!req)
    return ERR_PTR(-ENOMEM);
    req.tx.buf[0] = PECI_GET_TEMP_CMD;
    ret = peci_request_xfer(req);
    if (ret) {
    peci_request_free(req);
    return ERR_PTR(ret);
    }
    return req;
    }
    EXPORT_SYMBOL_NS_GPL(peci_xfer_get_temp, "PECI");
    static struct peci_request *
    __pkg_cfg_read(struct peci_device *device, u8 index, u16 param, u8 len)
    {
    struct peci_request *req;
    int ret;
    req = peci_request_alloc(device, PECI_RDPKGCFG_WR_LEN, PECI_RDPKGCFG_RD_LEN_BASE + len);
    if (!req)
    return ERR_PTR(-ENOMEM);
    req.tx.buf[0] = PECI_RDPKGCFG_CMD;
    req.tx.buf[1] = 0;
    req.tx.buf[2] = index;
    put_unaligned_le16(param, &req.tx.buf[3]);
    ret = peci_request_xfer_retry(req);
    if (ret) {
    peci_request_free(req);
    return ERR_PTR(ret);
    }
    return req;
    }
#[no_mangle]
unsafe extern "C" fn __get_pci_addr(bus: u8, dev: u8, func: u8, reg: u16) -> u32 {
    static u32 __get_pci_addr(u8 bus, u8 dev, u8 func, u16 reg)
    {
    return reg | PCI_DEVID(bus, PCI_DEVFN(dev, func)) << 12;
    }
    static struct peci_request *
    __pci_cfg_local_read(struct peci_device *device, u8 bus, u8 dev, u8 func, u16 reg, u8 len)
    {
    struct peci_request *req;
    u32 pci_addr;
    int ret;
    req = peci_request_alloc(device, PECI_RDPCICFGLOCAL_WR_LEN,
    PECI_RDPCICFGLOCAL_RD_LEN_BASE + len);
    if (!req)
    return ERR_PTR(-ENOMEM);
    pci_addr = __get_pci_addr(bus, dev, func, reg);
    req.tx.buf[0] = PECI_RDPCICFGLOCAL_CMD;
    req.tx.buf[1] = 0;
    put_unaligned_le24(pci_addr, &req.tx.buf[2]);
    ret = peci_request_xfer_retry(req);
    if (ret) {
    peci_request_free(req);
    return ERR_PTR(ret);
    }
    return req;
    }
    static struct peci_request *
    __ep_pci_cfg_read(struct peci_device *device, u8 msg_type, u8 seg,
    u8 bus, u8 dev, u8 func, u16 reg, u8 len)
    {
    struct peci_request *req;
    u32 pci_addr;
    int ret;
    req = peci_request_alloc(device, PECI_RDENDPTCFG_PCI_WR_LEN,
    PECI_RDENDPTCFG_RD_LEN_BASE + len);
    if (!req)
    return ERR_PTR(-ENOMEM);
    pci_addr = __get_pci_addr(bus, dev, func, reg);
    req.tx.buf[0] = PECI_RDENDPTCFG_CMD;
    req.tx.buf[1] = 0;
    req.tx.buf[2] = msg_type;
    req.tx.buf[3] = 0;
    req.tx.buf[4] = 0;
    req.tx.buf[5] = 0;
    req.tx.buf[6] = PECI_ENDPTCFG_ADDR_TYPE_PCI;
    req.tx.buf[7] = seg; /* PCI Segment */
    put_unaligned_le32(pci_addr, &req.tx.buf[8]);
    ret = peci_request_xfer_retry(req);
    if (ret) {
    peci_request_free(req);
    return ERR_PTR(ret);
    }
    return req;
    }
    static struct peci_request *
    __ep_mmio_read(struct peci_device *device, u8 bar, u8 addr_type, u8 seg,
    u8 bus, u8 dev, u8 func, u64 offset, u8 tx_len, u8 len)
    {
    struct peci_request *req;
    int ret;
    req = peci_request_alloc(device, tx_len, PECI_RDENDPTCFG_RD_LEN_BASE + len);
    if (!req)
    return ERR_PTR(-ENOMEM);
    req.tx.buf[0] = PECI_RDENDPTCFG_CMD;
    req.tx.buf[1] = 0;
    req.tx.buf[2] = PECI_ENDPTCFG_TYPE_MMIO;
    req.tx.buf[3] = 0; /* Endpoint ID */
    req.tx.buf[4] = 0; /* Reserved */
    req.tx.buf[5] = bar;
    req.tx.buf[6] = addr_type;
    req.tx.buf[7] = seg; /* PCI Segment */
    req.tx.buf[8] = PCI_DEVFN(dev, func);
    req.tx.buf[9] = bus; /* PCI Bus */
    if (addr_type == PECI_ENDPTCFG_ADDR_TYPE_MMIO_D)
    put_unaligned_le32(offset, &req.tx.buf[10]);
    else
    put_unaligned_le64(offset, &req.tx.buf[10]);
    ret = peci_request_xfer_retry(req);
    if (ret) {
    peci_request_free(req);
    return ERR_PTR(ret);
    }
    return req;
    }
#[no_mangle]
pub unsafe extern "C" fn peci_request_data_readb(req: *mut peci_request) -> u8 {
    u8 peci_request_data_readb(struct peci_request *req)
    {
    return req.rx.buf[1];
    }
    EXPORT_SYMBOL_NS_GPL(peci_request_data_readb, "PECI");
#[no_mangle]
pub unsafe extern "C" fn peci_request_data_readw(req: *mut peci_request) -> u16 {
    u16 peci_request_data_readw(struct peci_request *req)
    {
    return get_unaligned_le16(&req.rx.buf[1]);
    }
    EXPORT_SYMBOL_NS_GPL(peci_request_data_readw, "PECI");
#[no_mangle]
pub unsafe extern "C" fn peci_request_data_readl(req: *mut peci_request) -> u32 {
    u32 peci_request_data_readl(struct peci_request *req)
    {
    return get_unaligned_le32(&req.rx.buf[1]);
    }
    EXPORT_SYMBOL_NS_GPL(peci_request_data_readl, "PECI");
#[no_mangle]
pub unsafe extern "C" fn peci_request_data_readq(req: *mut peci_request) -> u64 {
    u64 peci_request_data_readq(struct peci_request *req)
    {
    return get_unaligned_le64(&req.rx.buf[1]);
    }
    EXPORT_SYMBOL_NS_GPL(peci_request_data_readq, "PECI");
#[no_mangle]
pub unsafe extern "C" fn peci_request_dib_read(req: *mut peci_request) -> u64 {
    u64 peci_request_dib_read(struct peci_request *req)
    {
    return get_unaligned_le64(&req.rx.buf[0]);
    }
    EXPORT_SYMBOL_NS_GPL(peci_request_dib_read, "PECI");
#[no_mangle]
pub unsafe extern "C" fn peci_request_temp_read(req: *mut peci_request) -> i16 {
    s16 peci_request_temp_read(struct peci_request *req)
    {
    return get_unaligned_le16(&req.rx.buf[0]);
    }
    EXPORT_SYMBOL_NS_GPL(peci_request_temp_read, "PECI");

    struct peci_request *peci_xfer_pkg_cfg_##x(struct peci_device *device, u8 index, u16 param) \
    { \
    return __pkg_cfg_read(device, index, param, sizeof(type)); \
    } \
    EXPORT_SYMBOL_NS_GPL(peci_xfer_pkg_cfg_##x, "PECI")
    __read_pkg_config(readb, u8);
    __read_pkg_config(readw, u16);
    __read_pkg_config(readl, u32);
    __read_pkg_config(readq, u64);

    struct peci_request * \
    peci_xfer_pci_cfg_local_##x(struct peci_device *device, u8 bus, u8 dev, u8 func, u16 reg) \
    { \
    return __pci_cfg_local_read(device, bus, dev, func, reg, sizeof(type)); \
    } \
    EXPORT_SYMBOL_NS_GPL(peci_xfer_pci_cfg_local_##x, "PECI")
    __read_pci_config_local(readb, u8);
    __read_pci_config_local(readw, u16);
    __read_pci_config_local(readl, u32);

    struct peci_request * \
    peci_xfer_ep_pci_cfg_##x(struct peci_device *device, u8 seg, u8 bus, u8 dev, u8 func, u16 reg) \
    { \
    return __ep_pci_cfg_read(device, msg_type, seg, bus, dev, func, reg, sizeof(type)); \
    } \
    EXPORT_SYMBOL_NS_GPL(peci_xfer_ep_pci_cfg_##x, "PECI")
    __read_ep_pci_config(local_readb, PECI_ENDPTCFG_TYPE_LOCAL_PCI, u8);
    __read_ep_pci_config(local_readw, PECI_ENDPTCFG_TYPE_LOCAL_PCI, u16);
    __read_ep_pci_config(local_readl, PECI_ENDPTCFG_TYPE_LOCAL_PCI, u32);
    __read_ep_pci_config(readb, PECI_ENDPTCFG_TYPE_PCI, u8);
    __read_ep_pci_config(readw, PECI_ENDPTCFG_TYPE_PCI, u16);
    __read_ep_pci_config(readl, PECI_ENDPTCFG_TYPE_PCI, u32);

    struct peci_request *peci_xfer_ep_mmio##y##_##x(struct peci_device *device, u8 bar, u8 seg, \
    u8 bus, u8 dev, u8 func, u64 offset) \
    { \
    return __ep_mmio_read(device, bar, addr_type, seg, bus, dev, func, \
    offset, PECI_RDENDPTCFG_MMIO_WR_LEN_BASE + sizeof(type1), \
    sizeof(type2)); \
    } \
    EXPORT_SYMBOL_NS_GPL(peci_xfer_ep_mmio##y##_##x, "PECI")
    __read_ep_mmio(readl, 32, PECI_ENDPTCFG_ADDR_TYPE_MMIO_D, u32, u32);
    __read_ep_mmio(readl, 64, PECI_ENDPTCFG_ADDR_TYPE_MMIO_Q, u64, u32);
