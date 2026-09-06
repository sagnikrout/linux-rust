//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt76x2/usb_mcu.c
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
pub const MCU_FW_URB_MAX_PAYLOAD: c_uint = 0x3900;
pub const MCU_ROM_PATCH_MAX_PAYLOAD: c_int = 2048;
pub const MT76U_MCU_ILM_OFFSET: c_uint = 0x80000;
pub const MT76U_MCU_DLM_OFFSET: c_uint = 0x110000;
pub const MT76U_MCU_ROM_PATCH_OFFSET: c_uint = 0x90000;
#[no_mangle]
unsafe extern "C" fn mt76x2u_mcu_load_ivb(dev: *mut mt76x02_dev) {
    static void mt76x2u_mcu_load_ivb(struct mt76x02_dev *dev)
    {
    mt76u_vendor_request(&dev.mt76, MT_VEND_DEV_MODE,
    USB_DIR_OUT | USB_TYPE_VENDOR,
    0x12, 0, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn mt76x2u_mcu_enable_patch(dev: *mut mt76x02_dev) {
    static void mt76x2u_mcu_enable_patch(struct mt76x02_dev *dev)
    {
    struct mt76_usb *usb = &dev.mt76.usb;
    static const u8 data[] = {
    0x6f, 0xfc, 0x08, 0x01,
    0x20, 0x04, 0x00, 0x00,
    0x00, 0x09, 0x00,
    };
    memcpy(usb.data, data, sizeof(data));
    mt76u_vendor_request(&dev.mt76, MT_VEND_DEV_MODE,
    USB_DIR_OUT | USB_TYPE_CLASS,
    0x12, 0, usb.data, sizeof(data));
    }
#[no_mangle]
unsafe extern "C" fn mt76x2u_mcu_reset_wmt(dev: *mut mt76x02_dev) {
    static void mt76x2u_mcu_reset_wmt(struct mt76x02_dev *dev)
    {
    struct mt76_usb *usb = &dev.mt76.usb;
    u8 data[] = {
    0x6f, 0xfc, 0x05, 0x01,
    0x07, 0x01, 0x00, 0x04
    };
    memcpy(usb.data, data, sizeof(data));
    mt76u_vendor_request(&dev.mt76, MT_VEND_DEV_MODE,
    USB_DIR_OUT | USB_TYPE_CLASS,
    0x12, 0, usb.data, sizeof(data));
    }
#[no_mangle]
unsafe extern "C" fn mt76x2u_mcu_load_rom_patch(dev: *mut mt76x02_dev) -> c_int {
    static int mt76x2u_mcu_load_rom_patch(struct mt76x02_dev *dev)
    {
    let mut rom_protect: bool = !is_mt7612(dev);
    struct mt76x02_patch_header *hdr;
    u32 val, patch_mask, patch_reg;
    const struct firmware *fw;
    int err;
    if (rom_protect &&
    !mt76_poll_msec(dev, MT_MCU_SEMAPHORE_03, 1, 1, 600)) {
    dev_err(dev.mt76.dev,
    "could not get hardware semaphore for ROM PATCH\n");
    return -ETIMEDOUT;
    }
    if (mt76xx_rev(dev) >= MT76XX_REV_E3) {
    patch_mask = BIT(0);
    patch_reg = MT_MCU_CLOCK_CTL;
    } else {
    patch_mask = BIT(1);
    patch_reg = MT_MCU_COM_REG0;
    }
    if (rom_protect && (mt76_rr(dev, patch_reg) & patch_mask)) {
    dev_info(dev.mt76.dev, "ROM patch already applied\n");
    return 0;
    }
    err = request_firmware(&fw, MT7662_ROM_PATCH, dev.mt76.dev);
    if (err < 0)
    return err;
    if (!fw || !fw.data || fw.size <= sizeof(*hdr)) {
    dev_err(dev.mt76.dev, "failed to load firmware\n");
    err = -EIO;
    goto out;
    }
    hdr = (struct mt76x02_patch_header *)fw.data;
    dev_info(dev.mt76.dev, "ROM patch build: %.15s\n", hdr.build_time);
// enable USB_DMA_CFG
    val = MT_USB_DMA_CFG_RX_BULK_EN |
    MT_USB_DMA_CFG_TX_BULK_EN |
    FIELD_PREP(MT_USB_DMA_CFG_RX_BULK_AGG_TOUT, 0x20);
    mt76_wr(dev, MT_VEND_ADDR(CFG, MT_USB_U3DMA_CFG), val);
// vendor reset
    mt76x02u_mcu_fw_reset(dev);
    usleep_range(5000, 10000);
// enable FCE to send in-band cmd
    mt76_wr(dev, MT_FCE_PSE_CTRL, 0x1);
// FCE tx_fs_base_ptr
    mt76_wr(dev, MT_TX_CPU_FROM_FCE_BASE_PTR, 0x400230);
// FCE tx_fs_max_cnt
    mt76_wr(dev, MT_TX_CPU_FROM_FCE_MAX_COUNT, 0x1);
// FCE pdma enable
    mt76_wr(dev, MT_FCE_PDMA_GLOBAL_CONF, 0x44);
// FCE skip_fs_en
    mt76_wr(dev, MT_FCE_SKIP_FS, 0x3);
    err = mt76x02u_mcu_fw_send_data(dev, fw.data + sizeof(*hdr),
    fw.size - sizeof(*hdr),
    MCU_ROM_PATCH_MAX_PAYLOAD,
    MT76U_MCU_ROM_PATCH_OFFSET);
    if (err < 0) {
    err = -EIO;
    goto out;
    }
    mt76x2u_mcu_enable_patch(dev);
    mt76x2u_mcu_reset_wmt(dev);
    mdelay(20);
    if (!mt76_poll_msec(dev, patch_reg, patch_mask, patch_mask, 100)) {
    dev_err(dev.mt76.dev, "failed to load ROM patch\n");
    err = -ETIMEDOUT;
    }
    out:
    if (rom_protect)
    mt76_wr(dev, MT_MCU_SEMAPHORE_03, 1);
    release_firmware(fw);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mt76x2u_mcu_load_firmware(dev: *mut mt76x02_dev) -> c_int {
    static int mt76x2u_mcu_load_firmware(struct mt76x02_dev *dev)
    {
    u32 val, dlm_offset = MT76U_MCU_DLM_OFFSET;
    const struct mt76x02_fw_header *hdr;
    int err, len, ilm_len, dlm_len;
    const struct firmware *fw;
    err = request_firmware(&fw, MT7662_FIRMWARE, dev.mt76.dev);
    if (err < 0)
    return err;
    if (!fw || !fw.data || fw.size < sizeof(*hdr)) {
    err = -EINVAL;
    goto out;
    }
    hdr = (const struct mt76x02_fw_header *)fw.data;
    ilm_len = le32_to_cpu(hdr.ilm_len);
    dlm_len = le32_to_cpu(hdr.dlm_len);
    len = sizeof(*hdr) + ilm_len + dlm_len;
    if (fw.size != len) {
    err = -EINVAL;
    goto out;
    }
    val = le16_to_cpu(hdr.fw_ver);
    dev_info(dev.mt76.dev, "Firmware Version: %d.%d.%02d\n",
    (val >> 12) & 0xf, (val >> 8) & 0xf, val & 0xf);
    val = le16_to_cpu(hdr.build_ver);
    dev_info(dev.mt76.dev, "Build: %x\n", val);
    dev_info(dev.mt76.dev, "Build Time: %.16s\n", hdr.build_time);
// vendor reset
    mt76x02u_mcu_fw_reset(dev);
    usleep_range(5000, 10000);
// enable USB_DMA_CFG
    val = MT_USB_DMA_CFG_RX_BULK_EN |
    MT_USB_DMA_CFG_TX_BULK_EN |
    FIELD_PREP(MT_USB_DMA_CFG_RX_BULK_AGG_TOUT, 0x20);
    mt76_wr(dev, MT_VEND_ADDR(CFG, MT_USB_U3DMA_CFG), val);
// enable FCE to send in-band cmd
    mt76_wr(dev, MT_FCE_PSE_CTRL, 0x1);
// FCE tx_fs_base_ptr
    mt76_wr(dev, MT_TX_CPU_FROM_FCE_BASE_PTR, 0x400230);
// FCE tx_fs_max_cnt
    mt76_wr(dev, MT_TX_CPU_FROM_FCE_MAX_COUNT, 0x1);
// FCE pdma enable
    mt76_wr(dev, MT_FCE_PDMA_GLOBAL_CONF, 0x44);
// FCE skip_fs_en
    mt76_wr(dev, MT_FCE_SKIP_FS, 0x3);
// load ILM
    err = mt76x02u_mcu_fw_send_data(dev, fw.data + sizeof(*hdr),
    ilm_len, MCU_FW_URB_MAX_PAYLOAD,
    MT76U_MCU_ILM_OFFSET);
    if (err < 0) {
    err = -EIO;
    goto out;
    }
// load DLM
    if (mt76xx_rev(dev) >= MT76XX_REV_E3)
    dlm_offset += 0x800;
    err = mt76x02u_mcu_fw_send_data(dev, fw.data + sizeof(*hdr) + ilm_len,
    dlm_len, MCU_FW_URB_MAX_PAYLOAD,
    dlm_offset);
    if (err < 0) {
    err = -EIO;
    goto out;
    }
    mt76x2u_mcu_load_ivb(dev);
    if (!mt76_poll_msec(dev, MT_MCU_COM_REG0, 1, 1, 100)) {
    dev_err(dev.mt76.dev, "firmware failed to start\n");
    err = -ETIMEDOUT;
    goto out;
    }
    mt76_set(dev, MT_MCU_COM_REG0, BIT(1));
// enable FCE to send in-band cmd
    mt76_wr(dev, MT_FCE_PSE_CTRL, 0x1);
    mt76x02_set_ethtool_fwver(dev, hdr);
    dev_dbg(dev.mt76.dev, "firmware running\n");
    out:
    release_firmware(fw);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mt76x2u_mcu_fw_init(dev: *mut mt76x02_dev) -> c_int {
    int mt76x2u_mcu_fw_init(struct mt76x02_dev *dev)
    {
    int err;
    err = mt76x2u_mcu_load_rom_patch(dev);
    if (err < 0)
    return err;
    return mt76x2u_mcu_load_firmware(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn mt76x2u_mcu_init(dev: *mut mt76x02_dev) -> c_int {
    int mt76x2u_mcu_init(struct mt76x02_dev *dev)
    {
    int err;
    err = mt76x02_mcu_function_select(dev, Q_SELECT, 1);
    if (err < 0)
    return err;
    return mt76x02_mcu_set_radio_state(dev, true);
    }
