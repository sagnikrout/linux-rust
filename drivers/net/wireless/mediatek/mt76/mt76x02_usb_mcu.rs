//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt76x02_usb_mcu.c
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2018 Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>
//

pub const MT_CMD_HDR_LEN: c_int = 4;
pub const MT_FCE_DMA_ADDR: c_uint = 0x0230;
pub const MT_FCE_DMA_LEN: c_uint = 0x0234;
pub const MT_TX_CPU_FROM_FCE_CPU_DESC_IDX: c_uint = 0x09a8;
    static void
    mt76x02u_multiple_mcu_reads(struct mt76_dev *dev, u8 *data, int len)
    {
    struct mt76_usb *usb = &dev.usb;
    int i;
    WARN_ON_ONCE(len / 8 != usb.mcu.rp_len);
    for (i = 0; i < usb.mcu.rp_len; i++) {
    let mut reg: u32 = get_unaligned_le32(data + 8 * i) - usb.mcu.base;
    let mut val: u32 = get_unaligned_le32(data + 8 * i + 4);
    WARN_ON_ONCE(usb.mcu.rp[i].reg != reg);
    usb.mcu.rp[i].value = val;
    }
    }
#[no_mangle]
unsafe extern "C" fn mt76x02u_mcu_wait_resp(dev: *mut mt76_dev, seq: u8) -> c_int {
    static int mt76x02u_mcu_wait_resp(struct mt76_dev *dev, u8 seq)
    {
    struct mt76_usb *usb = &dev.usb;
    u8 *data = usb.mcu.data;
    int i, len, ret;
    u32 rxfce;
    for (i = 0; i < 5; i++) {
    ret = mt76u_bulk_msg(dev, data, MCU_RESP_URB_SIZE, &len,
    300, MT_EP_IN_CMD_RESP);
    if (ret == -ETIMEDOUT)
    continue;
    if (ret)
    goto out;
    if (usb.mcu.rp)
    mt76x02u_multiple_mcu_reads(dev, data + 4, len - 8);
    rxfce = get_unaligned_le32(data);
    if (seq == FIELD_GET(MT_RX_FCE_INFO_CMD_SEQ, rxfce) &&
    FIELD_GET(MT_RX_FCE_INFO_EVT_TYPE, rxfce) == EVT_CMD_DONE)
    return 0;
    dev_err(dev.dev, "error: MCU resp evt:%lx seq:%hhx-%lx\n",
    FIELD_GET(MT_RX_FCE_INFO_EVT_TYPE, rxfce),
    seq, FIELD_GET(MT_RX_FCE_INFO_CMD_SEQ, rxfce));
    }
    out:
    dev_err(dev.dev, "error: %s failed with %d\n", __func__, ret);
    return ret;
    }
    static int
    __mt76x02u_mcu_send_msg(struct mt76_dev *dev, struct sk_buff *skb,
    int cmd, bool wait_resp)
    {
    let mut seq: u8 = 0;
    u32 info;
    int ret;
    if (test_bit(MT76_REMOVED, &dev.phy.state)) {
    ret = 0;
    goto out;
    }
    if (wait_resp) {
    seq = ++dev.mcu.msg_seq & 0xf;
    if (!seq)
    seq = ++dev.mcu.msg_seq & 0xf;
    }
    info = FIELD_PREP(MT_MCU_MSG_CMD_SEQ, seq) |
    FIELD_PREP(MT_MCU_MSG_CMD_TYPE, cmd) |
    MT_MCU_MSG_TYPE_CMD;
    ret = mt76x02u_skb_dma_info(skb, CPU_TX_PORT, info);
    if (ret)
    return ret;
    ret = mt76u_bulk_msg(dev, skb.data, skb.len, core::ptr::null_mut(), 500,
    MT_EP_OUT_INBAND_CMD);
    if (ret)
    goto out;
    if (wait_resp)
    ret = mt76x02u_mcu_wait_resp(dev, seq);
    out:
    consume_skb(skb);
    return ret;
    }
    static int
    mt76x02u_mcu_send_msg(struct mt76_dev *dev, int cmd, const void *data,
    int len, bool wait_resp)
    {
    struct sk_buff *skb;
    int err;
    skb = mt76_mcu_msg_alloc(dev, data, len);
    if (!skb)
    return -ENOMEM;
    mutex_lock(&dev.mcu.mutex);
    err = __mt76x02u_mcu_send_msg(dev, skb, cmd, wait_resp);
    mutex_unlock(&dev.mcu.mutex);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn skb_put_le32(skb: *mut sk_buff, val: u32) {
    static inline void skb_put_le32(struct sk_buff *skb, u32 val)
    {
    put_unaligned_le32(val, skb_put(skb, 4));
    }
    static int
    mt76x02u_mcu_wr_rp(struct mt76_dev *dev, u32 base,
    const struct mt76_reg_pair *data, int n)
    {
    let mut max_vals_per_cmd: c_int = MT_INBAND_PACKET_MAX_LEN / 8;
    let mut CMD_RANDOM_WRITE: c_int = 12;
    struct sk_buff *skb;
    int cnt, i, ret;
    if (!n)
    return 0;
    cnt = min(max_vals_per_cmd, n);
    skb = alloc_skb(cnt * 8 + MT_DMA_HDR_LEN + 4, GFP_KERNEL);
    if (!skb)
    return -ENOMEM;
    skb_reserve(skb, MT_DMA_HDR_LEN);
    for (i = 0; i < cnt; i++) {
    skb_put_le32(skb, base + data[i].reg);
    skb_put_le32(skb, data[i].value);
    }
    mutex_lock(&dev.mcu.mutex);
    ret = __mt76x02u_mcu_send_msg(dev, skb, CMD_RANDOM_WRITE, cnt == n);
    mutex_unlock(&dev.mcu.mutex);
    if (ret)
    return ret;
    return mt76x02u_mcu_wr_rp(dev, base, data + cnt, n - cnt);
    }
    static int
    mt76x02u_mcu_rd_rp(struct mt76_dev *dev, u32 base,
    struct mt76_reg_pair *data, int n)
    {
    let mut CMD_RANDOM_READ: c_int = 10;
    let mut max_vals_per_cmd: c_int = MT_INBAND_PACKET_MAX_LEN / 8;
    struct mt76_usb *usb = &dev.usb;
    struct sk_buff *skb;
    int cnt, i, ret;
    if (!n)
    return 0;
    cnt = min(max_vals_per_cmd, n);
    if (cnt != n)
    return -EINVAL;
    skb = alloc_skb(cnt * 8 + MT_DMA_HDR_LEN + 4, GFP_KERNEL);
    if (!skb)
    return -ENOMEM;
    skb_reserve(skb, MT_DMA_HDR_LEN);
    for (i = 0; i < cnt; i++) {
    skb_put_le32(skb, base + data[i].reg);
    skb_put_le32(skb, data[i].value);
    }
    mutex_lock(&dev.mcu.mutex);
    usb.mcu.rp = data;
    usb.mcu.rp_len = n;
    usb.mcu.base = base;
    ret = __mt76x02u_mcu_send_msg(dev, skb, CMD_RANDOM_READ, true);
    usb.mcu.rp = core::ptr::null_mut();
    mutex_unlock(&dev.mcu.mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mt76x02u_mcu_fw_reset(dev: *mut mt76x02_dev) {
    void mt76x02u_mcu_fw_reset(struct mt76x02_dev *dev)
    {
    mt76u_vendor_request(&dev.mt76, MT_VEND_DEV_MODE,
    USB_DIR_OUT | USB_TYPE_VENDOR,
    0x1, 0, core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL_GPL(mt76x02u_mcu_fw_reset);
    static int
    __mt76x02u_mcu_fw_send_data(struct mt76x02_dev *dev, u8 *data,
    const void *fw_data, int len, u32 dst_addr)
    {
    __le32 info;
    u32 val;
    int err, data_len;
    info = cpu_to_le32(FIELD_PREP(MT_MCU_MSG_PORT, CPU_TX_PORT) |
    FIELD_PREP(MT_MCU_MSG_LEN, len) |
    MT_MCU_MSG_TYPE_CMD);
    memcpy(data, &info, sizeof(info));
    memcpy(data + sizeof(info), fw_data, len);
    memset(data + sizeof(info) + len, 0, 4);
    mt76u_single_wr(&dev.mt76, MT_VEND_WRITE_FCE,
    MT_FCE_DMA_ADDR, dst_addr);
    len = roundup(len, 4);
    mt76u_single_wr(&dev.mt76, MT_VEND_WRITE_FCE,
    MT_FCE_DMA_LEN, len << 16);
    data_len = MT_CMD_HDR_LEN + len + sizeof(info);
    err = mt76u_bulk_msg(&dev.mt76, data, data_len, core::ptr::null_mut(), 1000,
    MT_EP_OUT_INBAND_CMD);
    if (err) {
    dev_err(dev.mt76.dev, "firmware upload failed: %d\n", err);
    return err;
    }
    val = mt76_rr(dev, MT_TX_CPU_FROM_FCE_CPU_DESC_IDX);
    val++;
    mt76_wr(dev, MT_TX_CPU_FROM_FCE_CPU_DESC_IDX, val);
    return 0;
    }
    int mt76x02u_mcu_fw_send_data(struct mt76x02_dev *dev, const void *data,
    int data_len, u32 max_payload, u32 offset)
    {
    int len, err = 0, pos = 0, max_len = max_payload - 8;
    u8 *buf;
    buf = kmalloc(max_payload, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    while (data_len > 0) {
    len = min_t(int, data_len, max_len);
    err = __mt76x02u_mcu_fw_send_data(dev, buf, data + pos,
    len, offset + pos);
    if (err < 0)
    break;
    data_len -= len;
    pos += len;
    usleep_range(5000, 10000);
    }
    kfree(buf);
    return err;
    }
    EXPORT_SYMBOL_GPL(mt76x02u_mcu_fw_send_data);
#[no_mangle]
pub unsafe extern "C" fn mt76x02u_init_mcu(dev: *mut mt76_dev) {
    void mt76x02u_init_mcu(struct mt76_dev *dev)
    {
    static const struct mt76_mcu_ops mt76x02u_mcu_ops = {
    .headroom = MT_CMD_HDR_LEN,
    .tailroom = 8,
    .mcu_send_msg = mt76x02u_mcu_send_msg,
    .mcu_parse_response = mt76x02_mcu_parse_response,
    .mcu_wr_rp = mt76x02u_mcu_wr_rp,
    .mcu_rd_rp = mt76x02u_mcu_rd_rp,
    };
    dev.mcu_ops = &mt76x02u_mcu_ops;
    }
    EXPORT_SYMBOL_GPL(mt76x02u_init_mcu);
    MODULE_AUTHOR("Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>");
    MODULE_DESCRIPTION("MediaTek MT76x02 MCU helpers");
    MODULE_LICENSE("Dual BSD/GPL");
