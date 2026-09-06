//! Automatically rewritten from C to Rust
//! Source: net/nfc/nci/spi.c
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
//
// Copyright (C) 2013  Intel Corporation. All rights reserved.
//

pub const NCI_SPI_ACK_SHIFT: c_int = 6;
pub const NCI_SPI_MSB_PAYLOAD_MASK: c_uint = 0x3F;

    NCI_CMD_TIMEOUT : NCI_DATA_TIMEOUT)
pub const NCI_SPI_DIRECT_WRITE: c_uint = 0x01;
pub const NCI_SPI_DIRECT_READ: c_uint = 0x02;
pub const ACKNOWLEDGE_NONE: c_int = 0;
pub const ACKNOWLEDGE_ACK: c_int = 1;
pub const ACKNOWLEDGE_NACK: c_int = 2;
pub const CRC_INIT: c_uint = 0xFFFF;
    static int __nci_spi_send(struct nci_spi *nspi, const struct sk_buff *skb,
    int cs_change)
    {
    struct spi_message m;
    struct spi_transfer t;
    memset(&t, 0, sizeof(struct spi_transfer));
// a NULL skb means we just want the SPI chip select line to raise
    if (skb) {
    t.tx_buf = skb.data;
    t.len = skb.len;
    } else {
// still set tx_buf non NULL to make the driver happy
    t.tx_buf = &t;
    t.len = 0;
    }
    t.cs_change = cs_change;
    t.delay.value = nspi.xfer_udelay;
    t.delay.unit = SPI_DELAY_UNIT_USECS;
    t.speed_hz = nspi.xfer_speed_hz;
    spi_message_init(&m);
    spi_message_add_tail(&t, &m);
    return spi_sync(nspi.spi, &m);
    }
    int nci_spi_send(struct nci_spi *nspi,
    struct completion *write_handshake_completion,
    struct sk_buff *skb)
    {
    let mut payload_len: c_uint = skb.len;
    unsigned char *hdr;
    int ret;
    long completion_rc;
// add the NCI SPI header to the start of the buffer
    hdr = skb_push(skb, NCI_SPI_HDR_LEN);
    hdr[0] = NCI_SPI_DIRECT_WRITE;
    hdr[1] = nspi.acknowledge_mode;
    hdr[2] = payload_len >> 8;
    hdr[3] = payload_len & 0xFF;
    if (nspi.acknowledge_mode == NCI_SPI_CRC_ENABLED) {
    u16 crc;
    crc = crc_ccitt(CRC_INIT, skb.data, skb.len);
    skb_put_u8(skb, crc >> 8);
    skb_put_u8(skb, crc & 0xFF);
    }
    if (write_handshake_completion)	{
// Trick SPI driver to raise chip select
    ret = __nci_spi_send(nspi, core::ptr::null_mut(), 1);
    if (ret)
    goto done;
// wait for NFC chip hardware handshake to complete
    if (wait_for_completion_timeout(write_handshake_completion,
    msecs_to_jiffies(1000)) == 0) {
    ret = -ETIME;
    goto done;
    }
    }
    ret = __nci_spi_send(nspi, skb, 0);
    if (ret != 0 || nspi.acknowledge_mode == NCI_SPI_CRC_DISABLED)
    goto done;
    reinit_completion(&nspi.req_completion);
    completion_rc =	wait_for_completion_interruptible_timeout(
    &nspi.req_completion,
    NCI_SPI_SEND_TIMEOUT);
    if (completion_rc <= 0 || nspi.req_result == ACKNOWLEDGE_NACK)
    ret = -EIO;
    done:
    kfree_skb(skb);
    return ret;
    }
    EXPORT_SYMBOL_GPL(nci_spi_send);
// ---- Interface to NCI SPI drivers ----
//
// nci_spi_allocate_spi - allocate a new nci spi
//
// @spi: SPI device
// @acknowledge_mode: Acknowledge mode used by the NFC device
// @delay: delay between transactions in us
// @ndev: nci dev to send incoming nci frames to
//
    struct nci_spi *nci_spi_allocate_spi(struct spi_device *spi,
    u8 acknowledge_mode, unsigned int delay,
    struct nci_dev *ndev)
    {
    struct nci_spi *nspi;
    nspi = devm_kzalloc(&spi.dev, sizeof(struct nci_spi), GFP_KERNEL);
    if (!nspi)
    return core::ptr::null_mut();
    nspi.acknowledge_mode = acknowledge_mode;
    nspi.xfer_udelay = delay;
// Use controller max SPI speed by default
    nspi.xfer_speed_hz = 0;
    nspi.spi = spi;
    nspi.ndev = ndev;
    init_completion(&nspi.req_completion);
    return nspi;
    }
    EXPORT_SYMBOL_GPL(nci_spi_allocate_spi);
#[no_mangle]
unsafe extern "C" fn send_acknowledge(nspi: *mut nci_spi, acknowledge: u8) -> c_int {
    static int send_acknowledge(struct nci_spi *nspi, u8 acknowledge)
    {
    struct sk_buff *skb;
    unsigned char *hdr;
    u16 crc;
    int ret;
    skb = nci_skb_alloc(nspi.ndev, 0, GFP_KERNEL);
    if (!skb)
    return -ENOMEM;
// add the NCI SPI header to the start of the buffer
    hdr = skb_push(skb, NCI_SPI_HDR_LEN);
    hdr[0] = NCI_SPI_DIRECT_WRITE;
    hdr[1] = NCI_SPI_CRC_ENABLED;
    hdr[2] = acknowledge << NCI_SPI_ACK_SHIFT;
    hdr[3] = 0;
    crc = crc_ccitt(CRC_INIT, skb.data, skb.len);
    skb_put_u8(skb, crc >> 8);
    skb_put_u8(skb, crc & 0xFF);
    ret = __nci_spi_send(nspi, skb, 0);
    kfree_skb(skb);
    return ret;
    }
    static struct sk_buff *__nci_spi_read(struct nci_spi *nspi)
    {
    struct sk_buff *skb;
    struct spi_message m;
    unsigned char req[2], resp_hdr[2];
    struct spi_transfer tx, rx;
    let mut rx_len: c_ushort = 0;
    int ret;
    spi_message_init(&m);
    memset(&tx, 0, sizeof(struct spi_transfer));
    req[0] = NCI_SPI_DIRECT_READ;
    req[1] = nspi.acknowledge_mode;
    tx.tx_buf = req;
    tx.len = 2;
    tx.cs_change = 0;
    tx.speed_hz = nspi.xfer_speed_hz;
    spi_message_add_tail(&tx, &m);
    memset(&rx, 0, sizeof(struct spi_transfer));
    rx.rx_buf = resp_hdr;
    rx.len = 2;
    rx.cs_change = 1;
    rx.speed_hz = nspi.xfer_speed_hz;
    spi_message_add_tail(&rx, &m);
    ret = spi_sync(nspi.spi, &m);
    if (ret)
    return core::ptr::null_mut();
    if (nspi.acknowledge_mode == NCI_SPI_CRC_ENABLED)
    rx_len = ((resp_hdr[0] & NCI_SPI_MSB_PAYLOAD_MASK) << 8) +
    resp_hdr[1] + NCI_SPI_CRC_LEN;
    else
    rx_len = (resp_hdr[0] << 8) | resp_hdr[1];
    skb = nci_skb_alloc(nspi.ndev, rx_len, GFP_KERNEL);
    if (!skb)
    return core::ptr::null_mut();
    spi_message_init(&m);
    memset(&rx, 0, sizeof(struct spi_transfer));
    rx.rx_buf = skb_put(skb, rx_len);
    rx.len = rx_len;
    rx.cs_change = 0;
    rx.delay.value = nspi.xfer_udelay;
    rx.delay.unit = SPI_DELAY_UNIT_USECS;
    rx.speed_hz = nspi.xfer_speed_hz;
    spi_message_add_tail(&rx, &m);
    ret = spi_sync(nspi.spi, &m);
    if (ret)
    goto receive_error;
    if (nspi.acknowledge_mode == NCI_SPI_CRC_ENABLED) {
// (u8 *)skb_push(skb, 1) = resp_hdr[1];
// (u8 *)skb_push(skb, 1) = resp_hdr[0];
    }
    return skb;
    receive_error:
    kfree_skb(skb);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn nci_spi_check_crc(skb: *mut sk_buff) -> c_int {
    static int nci_spi_check_crc(struct sk_buff *skb)
    {
    u16 crc_data = (skb.data[skb.len - 2] << 8) |
    skb.data[skb.len - 1];
    int ret;
    ret = (crc_ccitt(CRC_INIT, skb.data, skb.len - NCI_SPI_CRC_LEN)
    == crc_data);
    skb_trim(skb, skb.len - NCI_SPI_CRC_LEN);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nci_spi_get_ack(skb: *mut sk_buff) -> u8 {
    static u8 nci_spi_get_ack(struct sk_buff *skb)
    {
    u8 ret;
    ret = skb.data[0] >> NCI_SPI_ACK_SHIFT;
// Remove NFCC part of the header: ACK, NACK and MSB payload len
    skb_pull(skb, 2);
    return ret;
    }
//
// nci_spi_read - read frame from NCI SPI drivers
//
// @nspi: The nci spi
// Context: can sleep
//
// This call may only be used from a context that may sleep.  The sleep
// is non-interruptible, and has no timeout.
//
// It returns an allocated skb containing the frame on success, or NULL.
//
    struct sk_buff *nci_spi_read(struct nci_spi *nspi)
    {
    struct sk_buff *skb;
// Retrieve frame from SPI
    skb = __nci_spi_read(nspi);
    if (!skb)
    goto done;
    if (nspi.acknowledge_mode == NCI_SPI_CRC_ENABLED) {
    if (!nci_spi_check_crc(skb)) {
    send_acknowledge(nspi, ACKNOWLEDGE_NACK);
    goto done;
    }
// In case of acknowledged mode: if ACK or NACK received,
// unblock completion of latest frame sent.
//
    nspi.req_result = nci_spi_get_ack(skb);
    if (nspi.req_result)
    complete(&nspi.req_completion);
    }
// If there is no payload (ACK/NACK only frame),
// free the socket buffer
//
    if (!skb.len) {
    kfree_skb(skb);
    skb = core::ptr::null_mut();
    goto done;
    }
    if (nspi.acknowledge_mode == NCI_SPI_CRC_ENABLED)
    send_acknowledge(nspi, ACKNOWLEDGE_ACK);
    done:
    return skb;
    }
    EXPORT_SYMBOL_GPL(nci_spi_read);
    MODULE_DESCRIPTION("NFC Controller Interface (NCI) SPI link layer");
    MODULE_LICENSE("GPL");
