//! Automatically rewritten from C to Rust
//! Source: drivers/usb/atm/ueagle-atm.c
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-2-Clause)
//
// Copyright (c) 2003, 2004
// Damien Bergamini <damien.bergamini@free.fr>. All rights reserved.
//
// Copyright (c) 2005-2007 Matthieu Castet <castet.matthieu@free.fr>
// Copyright (c) 2005-2007 Stanislaw Gruszka <stf_xl@wp.pl>
//
// HISTORY : some part of the code was base on ueagle 1.3 BSD driver,
// Damien Bergamini agree to put his code under a DUAL GPL/BSD license.
//
// The rest of the code was rewritten from scratch.
//

//
// Debug macros
//

    do { \
    if (debug >= 1) \
    dev_dbg(&(usb_dev).dev, \
    "[ueagle-atm dbg] %s: " format, \
    __func__, ##args); \
    } while (0)

    do { \
    if (debug >= 2) \
    dev_dbg(&(usb_dev).dev, \
    "[ueagle-atm vdbg]  " format, ##args); \
    } while (0)

    dev_err(&(usb_dev).dev , "[UEAGLE-ATM] " format , ##args)

    dev_warn(&(usb_dev).dev , "[Ueagle-atm] " format, ##args)

    dev_info(&(usb_dev).dev , "[ueagle-atm] " format, ##args)
    struct intr_pkt;
// cmv's from firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uea_cmvs_v1 {
    pub address: u32,
    pub offset: u16,
    pub data: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uea_cmvs_v2 {
    pub group: u32,
    pub address: u32,
    pub offset: u32,
    pub data: u32,
    pub __packed: },
// information about currently processed cmv
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmv_dsc_e1 {
    pub function: u8,
    pub idx: u16,
    pub address: u32,
    pub offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmv_dsc_e4 {
    pub function: u16,
    pub offset: u16,
    pub address: u16,
    pub group: u16,
}

    union cmv_dsc {
    struct cmv_dsc_e1 e1;
    struct cmv_dsc_e4 e4;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uea_softc {
    pub usb_dev: *mut usb_device,
    pub usbatm: *mut usbatm_data,
    pub modem_index: c_int,
    pub driver_info: c_uint,
    pub annex: c_int,
pub const ANNEXA: c_int = 0;
pub const ANNEXB: c_int = 1;
    pub booting: c_int,
    pub reset: c_int,
    pub sync_q: wait_queue_head_t,
    pub kthread: *mut task_struct,
    pub data: u32,
    pub data1: u32,
    pub cmv_ack: c_int,
    pub cmv_dsc: union cmv_dsc,
    pub task: work_struct,
    pub pageno: u16,
    pub ovl: u16,
    pub dsp_firm: *const firmware,
    pub urb_int: *mut urb,
    pub ): *mut *mut *mut void (dispatch_cmv)(struct uea_softc , struct intr_pkt,
    pub ): *mut *mut *mut void (schedule_load_page)(struct uea_softc , struct intr_pkt,
    pub ): *mut *mut int (stat)(struct uea_softc,
    pub ): *mut *mut int (send_cmvs)(struct uea_softc,
// keep in sync with eaglectl
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uea_stats {
    struct {
    pub state: u32,
    pub flags: u32,
    pub mflags: u32,
    pub vidcpe: u32,
    pub vidco: u32,
    pub dsrate: u32,
    pub usrate: u32,
    pub dsunc: u32,
    pub usunc: u32,
    pub dscorr: u32,
    pub uscorr: u32,
    pub txflow: u32,
    pub rxflow: u32,
    pub usattenuation: u32,
    pub dsattenuation: u32,
    pub dsmargin: u32,
    pub usmargin: u32,
    pub firmid: u32,
    pub phy: },
    pub stats: },
}

//
// Elsa IDs
//
pub const ELSA_VID: c_uint = 0x05CC;
pub const ELSA_PID_PSTFIRM: c_uint = 0x3350;
pub const ELSA_PID_PREFIRM: c_uint = 0x3351;
pub const ELSA_PID_A_PREFIRM: c_uint = 0x3352;
pub const ELSA_PID_A_PSTFIRM: c_uint = 0x3353;
pub const ELSA_PID_B_PREFIRM: c_uint = 0x3362;
pub const ELSA_PID_B_PSTFIRM: c_uint = 0x3363;
//
// Devolo IDs : pots if (pid & 0x10)
//
pub const DEVOLO_VID: c_uint = 0x1039;
pub const DEVOLO_EAGLE_I_A_PID_PSTFIRM: c_uint = 0x2110;
pub const DEVOLO_EAGLE_I_A_PID_PREFIRM: c_uint = 0x2111;
pub const DEVOLO_EAGLE_I_B_PID_PSTFIRM: c_uint = 0x2100;
pub const DEVOLO_EAGLE_I_B_PID_PREFIRM: c_uint = 0x2101;
pub const DEVOLO_EAGLE_II_A_PID_PSTFIRM: c_uint = 0x2130;
pub const DEVOLO_EAGLE_II_A_PID_PREFIRM: c_uint = 0x2131;
pub const DEVOLO_EAGLE_II_B_PID_PSTFIRM: c_uint = 0x2120;
pub const DEVOLO_EAGLE_II_B_PID_PREFIRM: c_uint = 0x2121;
//
// Reference design USB IDs
//
pub const ANALOG_VID: c_uint = 0x1110;
pub const ADI930_PID_PREFIRM: c_uint = 0x9001;
pub const ADI930_PID_PSTFIRM: c_uint = 0x9000;
pub const EAGLE_I_PID_PREFIRM: c_uint = 0x9010	/* Eagle I */;
pub const EAGLE_I_PID_PSTFIRM: c_uint = 0x900F	/* Eagle I */;
pub const EAGLE_IIC_PID_PREFIRM: c_uint = 0x9024	/* Eagle IIC */;
pub const EAGLE_IIC_PID_PSTFIRM: c_uint = 0x9023	/* Eagle IIC */;
pub const EAGLE_II_PID_PREFIRM: c_uint = 0x9022	/* Eagle II */;
pub const EAGLE_II_PID_PSTFIRM: c_uint = 0x9021	/* Eagle II */;
pub const EAGLE_III_PID_PREFIRM: c_uint = 0x9032	/* Eagle III */;
pub const EAGLE_III_PID_PSTFIRM: c_uint = 0x9031	/* Eagle III */;
pub const EAGLE_IV_PID_PREFIRM: c_uint = 0x9042  /* Eagle IV */;
pub const EAGLE_IV_PID_PSTFIRM: c_uint = 0x9041  /* Eagle IV */;
//
// USR USB IDs
//
pub const USR_VID: c_uint = 0x0BAF;
pub const MILLER_A_PID_PREFIRM: c_uint = 0x00F2;
pub const MILLER_A_PID_PSTFIRM: c_uint = 0x00F1;
pub const MILLER_B_PID_PREFIRM: c_uint = 0x00FA;
pub const MILLER_B_PID_PSTFIRM: c_uint = 0x00F9;
pub const HEINEKEN_A_PID_PREFIRM: c_uint = 0x00F6;
pub const HEINEKEN_A_PID_PSTFIRM: c_uint = 0x00F5;
pub const HEINEKEN_B_PID_PREFIRM: c_uint = 0x00F8;
pub const HEINEKEN_B_PID_PSTFIRM: c_uint = 0x00F7;
pub const PREFIRM: c_int = 0;

    enum {
    ADI930 = 0,
    EAGLE_I,
    EAGLE_II,
    EAGLE_III,
    EAGLE_IV
    };
// macros for both struct usb_device_id and struct uea_softc

    (!((x).driver_info & PSTFIRM))

    ((x).driver_info & 0xf)

    ((x).annex & ANNEXB)

    ((data >> 8) & 0xf)

    ((UEA_CHIP_VERSION(sc) != EAGLE_IV) ? \
    (GET_STATUS(sc.stats.phy.state) == 2) : \
    (sc.stats.phy.state == 7))
//
// Set of macros to handle unaligned data in the firmware blob.
// The FW_GET_BYTE() macro is provided only for consistency.
//

pub const UEA_FW_NAME_MAX: c_int = 30;
pub const NB_MODEM: c_int = 4;
pub const BULK_TIMEOUT: c_int = 300;
pub const CTRL_TIMEOUT: c_int = 1000;

pub const UEA_INTR_IFACE_NO: c_int = 0;
pub const UEA_US_IFACE_NO: c_int = 1;
pub const UEA_DS_IFACE_NO: c_int = 2;
pub const FASTEST_ISO_INTF: c_int = 8;
pub const UEA_BULK_DATA_PIPE: c_uint = 0x02;
pub const UEA_IDMA_PIPE: c_uint = 0x04;
pub const UEA_INTR_PIPE: c_uint = 0x04;
pub const UEA_ISO_DATA_PIPE: c_uint = 0x08;
pub const UEA_E1_SET_BLOCK: c_uint = 0x0001;
pub const UEA_E4_SET_BLOCK: c_uint = 0x002c;
pub const UEA_SET_MODE: c_uint = 0x0003;
pub const UEA_SET_2183_DATA: c_uint = 0x0004;
pub const UEA_SET_TIMEOUT: c_uint = 0x0011;
pub const UEA_LOOPBACK_OFF: c_uint = 0x0002;
pub const UEA_LOOPBACK_ON: c_uint = 0x0003;
pub const UEA_BOOT_IDMA: c_uint = 0x0006;
pub const UEA_START_RESET: c_uint = 0x0007;
pub const UEA_END_RESET: c_uint = 0x0008;

// block information in eagle4 dsp firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_index {
    pub PageOffset: __le32,
    pub NotLastBlock: __le32,
    pub dummy: __le32,
    pub PageSize: __le32,
    pub PageAddress: __le32,
    pub dummy1: __le16,
    pub PageNumber: __le16,
    pub __packed: },

pub const E4_L1_STRING_HEADER: c_uint = 0x10;
pub const E4_MAX_PAGE_NUMBER: c_uint = 0x58;
pub const E4_NO_SWAPPAGE_HEADERS: c_uint = 0x31;
// l1_code is eagle4 dsp firmware format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l1_code {
    pub string_header: [u8; E4_L1_STRING_HEADER],
    pub page_number_to_block_index: [u8; E4_MAX_PAGE_NUMBER],
    pub page_header: [block_index; E4_NO_SWAPPAGE_HEADERS],
    pub code: [u8; ],
    pub __packed: },
// structures describing a block within a DSP page
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_info_e1 {
    pub wHdr: __le16,
    pub wAddress: __le16,
    pub wSize: __le16,
    pub wOvlOffset: __le16,
    pub /: *mut *mut __le16 wOvl; / overlay,
    pub wLast: __le16,
    pub __packed: },
pub const E1_BLOCK_INFO_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_info_e4 {
    pub wHdr: __be16,
    pub bBootPage: __u8,
    pub bPageNumber: __u8,
    pub dwSize: __be32,
    pub dwAddress: __be32,
    pub wReserved: __be16,
    pub __packed: },
pub const E4_BLOCK_INFO_SIZE: c_int = 14;
pub const UEA_BIHDR: c_uint = 0xabcd;
pub const UEA_RESERVED: c_uint = 0xffff;
// constants describing cmv type
pub const E1_PREAMBLE: c_uint = 0x535c;
pub const E1_MODEMTOHOST: c_uint = 0x01;
pub const E1_HOSTTOMODEM: c_uint = 0x10;
pub const E1_MEMACCESS: c_uint = 0x1;
pub const E1_ADSLDIRECTIVE: c_uint = 0x7;

pub const E4_MEMACCESS: c_int = 0;
pub const E4_ADSLDIRECTIVE: c_uint = 0xf;

// for MEMACCESS
pub const E1_REQUESTREAD: c_uint = 0x0;
pub const E1_REQUESTWRITE: c_uint = 0x1;
pub const E1_REPLYREAD: c_uint = 0x2;
pub const E1_REPLYWRITE: c_uint = 0x3;
pub const E4_REQUESTREAD: c_uint = 0x0;
pub const E4_REQUESTWRITE: c_uint = 0x4;

// for ADSLDIRECTIVE
pub const E1_KERNELREADY: c_uint = 0x0;
pub const E1_MODEMREADY: c_uint = 0x1;
pub const E4_KERNELREADY: c_uint = 0x0;
pub const E4_MODEMREADY: c_uint = 0x1;

    ((st) & 0xf) << 4 | ((s) & 0xf))

    (((c) & 0xff) << 24 |						\
    ((d) & 0xff) << 16 |						\
    ((a) & 0xff) << 8  |						\
    ((b) & 0xff))

pub const E4_SA_CNTL: c_int = 1;
pub const E4_SA_STAT: c_int = 2;
pub const E4_SA_INFO: c_int = 3;
pub const E4_SA_TEST: c_int = 4;
pub const E4_SA_OPTN: c_int = 5;
pub const E4_SA_RATE: c_int = 6;
pub const E4_SA_DIAG: c_int = 7;
pub const E4_SA_CNFG: c_int = 8;
// structures representing a CMV (Configuration and Management Variable)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmv_e1 {
    pub wPreamble: __le16,
    pub bDirection: __u8,
    pub bFunction: __u8,
    pub wIndex: __le16,
    pub dwSymbolicAddress: __le32,
    pub wOffsetAddress: __le16,
    pub dwData: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmv_e4 {
    pub wGroup: __be16,
    pub wFunction: __be16,
    pub wOffset: __be16,
    pub wAddress: __be16,
    pub dwData: [__be32; 6],
    pub __packed: },
// structures representing swap information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_info_e1 {
    pub bSwapPageNo: __u8,
    pub /: *mut *mut __u8 bOvl; / overlay,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_info_e4 {
    pub bSwapPageNo: __u8,
    pub __packed: },
// structures representing interrupt data

pub const INT_LOADSWAPPAGE: c_uint = 0x0001;
pub const INT_INCOMINGCMV: c_uint = 0x0002;
    union intr_data_e1 {
    struct {
    pub swapinfo: swap_info_e1,
    pub wDataSize: __le16,
    pub s1: } __packed,
    struct {
    pub cmv: cmv_e1,
    pub wDataSize: __le16,
    pub s2: } __packed,
    pub __packed: },
    union intr_data_e4 {
    struct {
    pub swapinfo: swap_info_e4,
    pub wDataSize: __le16,
    pub s1: } __packed,
    struct {
    pub cmv: cmv_e4,
    pub wDataSize: __le16,
    pub s2: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intr_pkt {
    pub bType: __u8,
    pub bNotification: __u8,
    pub wValue: __le16,
    pub wIndex: __le16,
    pub wLength: __le16,
    pub wInterrupt: __le16,
    union {
    pub e1: union intr_data_e1,
    pub e4: union intr_data_e4,
    pub u: },
    pub __packed: },
pub const E1_INTR_PKT_SIZE: c_int = 28;
pub const E4_INTR_PKT_SIZE: c_int = 64;
    pub uea_driver: static struct usb_driver,
    pub DEFINE_MUTEX(uea_mutex): static,
    static const char * const chip_name[] = {
    pub IV"}: "ADI930", "Eagle I", "Eagle II", "Eagle III", "Eagle,
    pub modem_index: static int,
    pub debug: static unsigned int,
    static unsigned int altsetting[NB_MODEM] = {
    pub FASTEST_ISO_INTF}: [0 ... (NB_MODEM - 1)] =,
    pub sync_wait: [static bool; NB_MODEM],
    pub cmv_file: [*mut static char; NB_MODEM],
    pub annex: [static int; NB_MODEM],
    pub 0644): module_param(debug, uint,,
    pub (0=off,1=on,2=verbose)"): MODULE_PARM_DESC(debug, "module debug level,
    pub 0644): module_param_array(altsetting, uint, NULL,,
    MODULE_PARM_DESC(altsetting, "alternate setting for incoming traffic: 0=bulk, "
    pub (default)"): "1=isoc slowest, ... , 8=isoc fastest,
    pub 0644): module_param_array(sync_wait, bool, NULL,,
    pub ATM"): MODULE_PARM_DESC(sync_wait, "wait the synchronisation before starting,
    pub 0644): module_param_array(cmv_file, charp, NULL,,
    MODULE_PARM_DESC(cmv_file,
    pub variables"): "file name with configuration and management,
    pub 0644): module_param_array(annex, uint, NULL,,
    MODULE_PARM_DESC(annex,
    pub b)"): "manually set annex a/b (0=auto, 1=annex a, 2=annex,

    ({ \
    int _r = wait_event_freezable_timeout(sc.sync_q, \
    pub \: (cond) || kthread_should_stop(), timeo);,
    if (kthread_should_stop()) \
    pub \: _r = -ENODEV;,
    pub \: _r;,
    })

    do { \
    if (sc.usbatm.atm_dev) \
    pub \: sc->usbatm->atm_dev->type = val;,
    } while (0)

    do { \
    if (sc.usbatm.atm_dev) \
    pub \: atm_dev_signal_change(sc->usbatm->atm_dev, val);,
    } while (0)
// Firmware loading
pub const LOAD_INTERNAL: c_uint = 0xA0;
pub const F8051_USBCS: c_uint = 0x7f92;
//
// uea_send_modem_cmd - Send a command for pre-firmware devices.
//
    static int uea_send_modem_cmd(struct usb_device *usb,
    u16 addr, u16 size, const u8 *buff)
    {
    pub -ENOMEM: int ret =,
    pub xfer_buff: *mut u8,
    pub GFP_KERNEL): xfer_buff = kmemdup(buff, size,,
    if (xfer_buff) {
    ret = usb_control_msg(usb,
    usb_sndctrlpipe(usb, 0),
    LOAD_INTERNAL,
    USB_DIR_OUT | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE, addr, 0, xfer_buff,
    pub CTRL_TIMEOUT): size,,
    }
    if (ret < 0)
    pub ret: return,
    pub -EIO: return (ret == size) ? 0 :,
    }
    static void uea_upload_pre_firmware(const struct firmware *fw_entry,
    void *context)
    {
    pub context: *mut *mut usb_interface intf =,
    pub interface_to_usbdev(intf): *mut *mut usb_device usb =,
    pub usb_get_intfdata(intf): *mut *mut completion fw_done =,
    pub pfw: *const u8,
    pub value: u8,
    pub 0: u32 crc =,
    pub size: int ret,,
    if (!fw_entry) {
    pub available\n"): uea_err(usb, "firmware is not,
    pub err: goto,
    }
    pub fw_entry->data: pfw =,
    pub fw_entry->size: size =,
    if (size < 4)
    pub err_fw_corrupted: goto,
    pub get_unaligned_le32(pfw): crc =,
    pub 4: pfw +=,
    pub 4: size -=,
    if (crc32_be(0, pfw, size) != crc)
    pub err_fw_corrupted: goto,
//
// Start to upload firmware : send reset
//
    pub 1: value =,
    pub &value): ret = uea_send_modem_cmd(usb, F8051_USBCS, sizeof(value),,
    if (ret < 0) {
    pub ret): uea_err(usb, "modem reset failed with error %d\n",,
    pub err: goto,
    }
    while (size > 3) {
    pub FW_GET_BYTE(pfw): u8 len =,
    pub 1): u16 add = get_unaligned_le16(pfw +,
    pub 3: size -= len +,
    if (size < 0)
    pub err_fw_corrupted: goto,
    pub 3): ret = uea_send_modem_cmd(usb, add, len, pfw +,
    if (ret < 0) {
    uea_err(usb, "uploading firmware data failed "
    pub ret): "with error %d\n",,
    pub err: goto,
    }
    pub 3: pfw += len +,
    }
    if (size != 0)
    pub err_fw_corrupted: goto,
//
// Tell the modem we finish : de-assert reset
//
    pub 0: value =,
    pub &value): ret = uea_send_modem_cmd(usb, F8051_USBCS, 1,,
    if (ret < 0)
    pub ret): uea_err(usb, "modem de-assert failed with error %d\n",,
    else
    pub uploaded\n"): uea_info(usb, "firmware,
    pub err: goto,
    err_fw_corrupted:
    pub corrupted\n"): uea_err(usb, "firmware is,
    err:
    }
//
// uea_load_firmware - Load usb firmware for pre-firmware devices.
//
#[no_mangle]
unsafe extern "C" fn uea_load_firmware(intf: *mut usb_interface, ver: c_uint) -> c_int {
    static int uea_load_firmware(struct usb_interface *intf, unsigned int ver)
    {
    pub ret: c_int,
    pub EAGLE_FIRMWARE: *mut *mut char fw_name =,
    pub interface_to_usbdev(intf): *mut *mut usb_device usb =,
    pub firmware\n"): uea_info(usb, "pre-firmware device, uploading,
    switch (ver) {
    case ADI930:
    pub ADI930_FIRMWARE: fw_name =,
    case EAGLE_I:
    pub EAGLE_I_FIRMWARE: fw_name =,
    case EAGLE_II:
    pub EAGLE_II_FIRMWARE: fw_name =,
    case EAGLE_III:
    pub EAGLE_III_FIRMWARE: fw_name =,
    case EAGLE_IV:
    pub EAGLE_IV_FIRMWARE: fw_name =,
    }
    ret = request_firmware_nowait(THIS_MODULE, 1, fw_name, &usb.dev,
    GFP_KERNEL, intf,
    if (ret)
    pub fw_name): uea_err(usb, "firmware %s is not available\n",,
    else
    pub fw_name): uea_info(usb, "loading firmware %s\n",,
    pub ret: return,
    }
// modem management : dsp firmware, send/read CMV, monitoring statistic
//
// Make sure that the DSP code provided is safe to use.
//
#[no_mangle]
unsafe extern "C" fn check_dsp_e1(dsp: *const u8, len: c_uint) -> c_int {
    static int check_dsp_e1(const u8 *dsp, unsigned int len)
    {
    pub blockcount: u8 pagecount,,
    pub blocksize: u16,
    pub pageoffset: u32,
    pub pp: unsigned int i, j, p,,
    pub FW_GET_BYTE(dsp): pagecount =,
    pub 1: p =,
// enough space for page offsets?
    if (p + 4 * pagecount > len)
    pub 1: return,
    pub {: for (i = 0; i < pagecount; i++),
    pub p): pageoffset = get_unaligned_le32(dsp +,
    pub 4: p +=,
    if (pageoffset == 0)
// enough space for blockcount?
    if (pageoffset >= len)
    pub 1: return,
    pub pageoffset: pp =,
    pub pp): blockcount = FW_GET_BYTE(dsp +,
    pub 1: pp +=,
    pub {: for (j = 0; j < blockcount; j++),
// enough space for block header?
    if (pp + 4 > len)
    pub 1: return,
    pub /: *mut *mut pp += 2; / skip blockaddr,
    pub pp): blocksize = get_unaligned_le16(dsp +,
    pub 2: pp +=,
// enough space for block data?
    if (pp + blocksize > len)
    pub 1: return,
    pub blocksize: pp +=,
    }
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn check_dsp_e4(dsp: *const u8, len: c_int) -> c_int {
    static int check_dsp_e4(const u8 *dsp, int len)
    {
    pub i: c_int,
    pub dsp: *mut *mut *mut l1_code p = (l1_code ),
    pub dsp: unsigned int sum = p->code -,
    if (len < sum)
    pub 1: return,
    if (strcmp("STRATIPHY ANEXA", p.string_header) != 0 &&
    strcmp("STRATIPHY ANEXB", p.string_header) != 0)
    pub 1: return,
    pub {: for (i = 0; i < E4_MAX_PAGE_NUMBER; i++),
    pub blockidx: *mut block_index,
    pub p->page_number_to_block_index[i]: u8 blockno =,
    if (blockno >= E4_NO_SWAPPAGE_HEADERS)
    do {
    pub l: u64,
    if (blockno >= E4_NO_SWAPPAGE_HEADERS)
    pub 1: return,
    pub &p->page_header[blockno++]: blockidx =,
    if ((u8 *)(blockidx + 1) - dsp  >= len)
    pub 1: return,
    if (le16_to_cpu(blockidx.PageNumber) != i)
    pub 1: return,
    pub E4_PAGE_BYTES(blockidx->PageSize): l =,
    pub l: sum +=,
    pub le32_to_cpu(blockidx->PageOffset): l +=,
    if (l > len)
    pub 1: return,
// zero is zero regardless endianness
    pub (blockidx->NotLastBlock): } while,
    }
    pub 1: return (sum == len) ? 0 :,
    }
//
// send data to the idma pipe
//
#[no_mangle]
unsafe extern "C" fn uea_idma_write(sc: *mut uea_softc, data: *const c_void, size: u32) -> c_int {
    static int uea_idma_write(struct uea_softc *sc, const void *data, u32 size)
    {
    pub -ENOMEM: int ret =,
    pub xfer_buff: *mut u8,
    pub bytes_read: c_int,
    pub GFP_KERNEL): xfer_buff = kmemdup(data, size,,
    if (!xfer_buff) {
    pub xfer_buff\n"): uea_err(INS_TO_USBDEV(sc), "can't allocate,
    pub ret: return,
    }
    ret = usb_bulk_msg(sc.usb_dev,
    usb_sndbulkpipe(sc.usb_dev, UEA_IDMA_PIPE),
    pub BULK_TIMEOUT): xfer_buff, size, &bytes_read,,
    if (ret < 0)
    pub ret: return,
    if (size != bytes_read) {
    uea_err(INS_TO_USBDEV(sc), "size != bytes_read %d %d\n", size,
    pub -EIO: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn request_dsp(sc: *mut uea_softc) -> c_int {
    static int request_dsp(struct uea_softc *sc)
    {
    pub ret: c_int,
    pub dsp_name: *mut c_char,
    if (UEA_CHIP_VERSION(sc) == EAGLE_IV) {
    if (IS_ISDN(sc))
    pub DSP4I_FIRMWARE: dsp_name =,
    else
    pub DSP4P_FIRMWARE: dsp_name =,
    } else if (UEA_CHIP_VERSION(sc) == ADI930) {
    if (IS_ISDN(sc))
    pub DSP9I_FIRMWARE: dsp_name =,
    else
    pub DSP9P_FIRMWARE: dsp_name =,
    } else {
    if (IS_ISDN(sc))
    pub DSPEI_FIRMWARE: dsp_name =,
    else
    pub DSPEP_FIRMWARE: dsp_name =,
    }
    pub &sc->usb_dev->dev): ret = request_firmware(&sc->dsp_firm, dsp_name,,
    if (ret < 0) {
    uea_err(INS_TO_USBDEV(sc),
    "requesting firmware %s failed with error %d\n",
    pub ret): dsp_name,,
    pub ret: return,
    }
    if (UEA_CHIP_VERSION(sc) == EAGLE_IV)
    pub sc->dsp_firm->size): ret = check_dsp_e4(sc->dsp_firm->data,,
    else
    pub sc->dsp_firm->size): ret = check_dsp_e1(sc->dsp_firm->data,,
    if (ret) {
    uea_err(INS_TO_USBDEV(sc), "firmware %s is corrupted\n",
    pub NULL: sc->dsp_firm =,
    pub -EILSEQ: return,
    }
    pub 0: return,
    }
//
// The uea_load_page() function must be called within a process context
//
#[no_mangle]
unsafe extern "C" fn uea_load_page_e1(work: *mut work_struct) {
    static void uea_load_page_e1(struct work_struct *work)
    {
    pub task): *mut *mut uea_softc sc = container_of(work, uea_softc,,
    pub sc->pageno: u16 pageno =,
    pub sc->ovl: u16 ovl =,
    pub bi: block_info_e1,
    pub p: *const u8,
    pub blockcount: u8 pagecount,,
    pub blocksize: u16 blockaddr,,
    pub pageoffset: u32,
    pub i: c_int,
// reload firmware when reboot start and it's loaded already
    if (ovl == 0 && pageno == 0) {
    pub NULL: sc->dsp_firm =,
    }
    if (sc.dsp_firm == core::ptr::null_mut() && request_dsp(sc) < 0)
    pub sc->dsp_firm->data: p =,
    pub FW_GET_BYTE(p): pagecount =,
    pub 1: p +=,
    if (pageno >= pagecount)
    pub bad1: goto,
    pub pageno: *mut *mut p += 4,
    pub get_unaligned_le32(p): pageoffset =,
    if (pageoffset == 0)
    pub bad1: goto,
    pub pageoffset: p = sc->dsp_firm->data +,
    pub FW_GET_BYTE(p): blockcount =,
    pub 1: p +=,
    uea_dbg(INS_TO_USBDEV(sc),
    pub pageno): "sending %u blocks for DSP page %u\n", blockcount,,
    pub cpu_to_le16(UEA_BIHDR): bi.wHdr =,
    pub cpu_to_le16(ovl): bi.wOvl =,
    pub 0x8000): bi.wOvlOffset = cpu_to_le16(ovl |,
    pub {: for (i = 0; i < blockcount; i++),
    pub get_unaligned_le16(p): blockaddr =,
    pub 2: p +=,
    pub get_unaligned_le16(p): blocksize =,
    pub 2: p +=,
    pub cpu_to_le16(blocksize): bi.wSize =,
    pub cpu_to_le16(blockaddr): bi.wAddress =,
    pub 0): bi.wLast = cpu_to_le16((i == blockcount - 1) ? 1 :,
// send block info through the IDMA pipe
    if (uea_idma_write(sc, &bi, E1_BLOCK_INFO_SIZE))
    pub bad2: goto,
// send block data through the IDMA pipe
    if (uea_idma_write(sc, p, blocksize))
    pub bad2: goto,
    pub blocksize: p +=,
    }
    bad2:
    pub i): uea_err(INS_TO_USBDEV(sc), "sending DSP block %u failed\n",,
    bad1:
    pub pageno): uea_err(INS_TO_USBDEV(sc), "invalid DSP page %u requested\n",,
    }
#[no_mangle]
unsafe extern "C" fn __uea_load_page_e4(sc: *mut uea_softc, pageno: u8, boot: c_int) {
    static void __uea_load_page_e4(struct uea_softc *sc, u8 pageno, int boot)
    {
    pub bi: block_info_e4,
    pub blockidx: *mut block_index,
    pub sc->dsp_firm->data: *mut *mut *mut l1_code p = (l1_code ),
    pub p->page_number_to_block_index[pageno]: u8 blockno =,
    pub cpu_to_be16(UEA_BIHDR): bi.wHdr =,
    pub boot: bi.bBootPage =,
    pub pageno: bi.bPageNumber =,
    pub cpu_to_be16(UEA_RESERVED): bi.wReserved =,
    do {
    pub blockoffset: *const u8,
    pub blocksize: c_uint,
    pub &p->page_header[blockno]: blockidx =,
    pub E4_PAGE_BYTES(blockidx->PageSize): blocksize =,
    blockoffset = sc.dsp_firm.data + le32_to_cpu(
    pub cpu_to_be32(blocksize): bi.dwSize =,
    pub cpu_to_be32(le32_to_cpu(blockidx->PageAddress)): bi.dwAddress =,
    uea_dbg(INS_TO_USBDEV(sc),
    "sending block %u for DSP page "
    "%u size %u address %x\n",
    blockno, pageno, blocksize,
// send block info through the IDMA pipe
    if (uea_idma_write(sc, &bi, E4_BLOCK_INFO_SIZE))
    pub bad: goto,
// send block data through the IDMA pipe
    if (uea_idma_write(sc, blockoffset, blocksize))
    pub bad: goto,
    pub (blockidx->NotLastBlock): } while,
    bad:
    pub blockno): uea_err(INS_TO_USBDEV(sc), "sending DSP block %u failed\n",,
    }
#[no_mangle]
unsafe extern "C" fn uea_load_page_e4(work: *mut work_struct) {
    static void uea_load_page_e4(struct work_struct *work)
    {
    pub task): *mut *mut uea_softc sc = container_of(work, uea_softc,,
    pub sc->pageno: u8 pageno =,
    pub i: c_int,
    pub bi: block_info_e4,
    pub p: *mut l1_code,
    pub pageno): uea_dbg(INS_TO_USBDEV(sc), "sending DSP page %u\n",,
// reload firmware when reboot start and it's loaded already
    if (pageno == 0) {
    pub NULL: sc->dsp_firm =,
    }
    if (sc.dsp_firm == core::ptr::null_mut() && request_dsp(sc) < 0)
    pub sc->dsp_firm->data: *mut *mut p = (struct l1_code ),
    if (pageno >= le16_to_cpu(p.page_header[0].PageNumber)) {
    uea_err(INS_TO_USBDEV(sc), "invalid DSP "
    pub pageno): "page %u requested\n",,
    }
    if (pageno != 0) {
    pub 0): __uea_load_page_e4(sc, pageno,,
    }
    uea_dbg(INS_TO_USBDEV(sc),
    pub p->page_header[0].PageNumber): "sending Main DSP page %u\n",,
    pub {: for (i = 0; i < le16_to_cpu(p->page_header[0].PageNumber); i++),
    if (E4_IS_BOOT_PAGE(p.page_header[i].PageSize))
    pub 1): __uea_load_page_e4(sc, i,,
    }
    pub bi\n"): uea_dbg(INS_TO_USBDEV(sc) , "sending start,
    pub cpu_to_be16(UEA_BIHDR): bi.wHdr =,
    pub 0: bi.bBootPage =,
    pub 0xff: bi.bPageNumber =,
    pub cpu_to_be16(UEA_RESERVED): bi.wReserved =,
    pub cpu_to_be32(E4_PAGE_BYTES(p->page_header[0].PageSize)): bi.dwSize =,
    pub cpu_to_be32(le32_to_cpu(p->page_header[0].PageAddress)): bi.dwAddress =,
// send block info through the IDMA pipe
    if (uea_idma_write(sc, &bi, E4_BLOCK_INFO_SIZE))
    pub failed\n"): uea_err(INS_TO_USBDEV(sc), "sending DSP start bi,
    }
#[no_mangle]
pub unsafe extern "C" fn wake_up_cmv_ack(sc: *mut uea_softc) {
    static inline void wake_up_cmv_ack(struct uea_softc *sc)
    {
    pub 1: sc->cmv_ack =,
    }
#[no_mangle]
pub unsafe extern "C" fn wait_cmv_ack(sc: *mut uea_softc) -> c_int {
    static inline int wait_cmv_ack(struct uea_softc *sc)
    {
    pub ACK_TIMEOUT): int ret = uea_wait(sc, sc->cmv_ack ,,
    pub 0: sc->cmv_ack =,
    uea_dbg(INS_TO_USBDEV(sc), "wait_event_timeout : %d ms\n",
    if (ret < 0)
    pub ret: return,
    pub 0: return (ret == 0) ? -ETIMEDOUT :,
    }
pub const UCDC_SEND_ENCAPSULATED_COMMAND: c_uint = 0x00;
    static int uea_request(struct uea_softc *sc,
    u16 value, u16 index, u16 size, const void *data)
    {
    pub xfer_buff: *mut u8,
    pub -ENOMEM: int ret =,
    pub GFP_KERNEL): xfer_buff = kmemdup(data, size,,
    if (!xfer_buff) {
    pub xfer_buff\n"): uea_err(INS_TO_USBDEV(sc), "can't allocate,
    pub ret: return,
    }
    ret = usb_control_msg(sc.usb_dev, usb_sndctrlpipe(sc.usb_dev, 0),
    UCDC_SEND_ENCAPSULATED_COMMAND,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    pub CTRL_TIMEOUT): value, index, xfer_buff, size,,
    if (ret < 0) {
    pub ret): uea_err(INS_TO_USBDEV(sc), "usb_control_msg error %d\n",,
    pub ret: return,
    }
    if (ret != size) {
    uea_err(INS_TO_USBDEV(sc),
    "usb_control_msg send only %d bytes (instead of %d)\n",
    pub size): ret,,
    pub -EIO: return,
    }
    pub 0: return,
    }
    static int uea_cmv_e1(struct uea_softc *sc,
    u8 function, u32 address, u16 offset, u32 data)
    {
    pub cmv: cmv_e1,
    pub ret: c_int,
    uea_vdbg(INS_TO_USBDEV(sc), "Function : %d-%d, Address : %c%c%c%c, "
    "offset : 0x%04x, data : 0x%08x\n",
    E1_FUNCTION_TYPE(function),
    E1_FUNCTION_SUBTYPE(function),
    E1_GETSA1(address), E1_GETSA2(address),
    E1_GETSA3(address),
    pub data): E1_GETSA4(address), offset,,
// we send a request, but we expect a reply
    pub 0x2: sc->cmv_dsc.e1.function = function |,
    pub address: sc->cmv_dsc.e1.address =,
    pub offset: sc->cmv_dsc.e1.offset =,
    pub cpu_to_le16(E1_PREAMBLE): cmv.wPreamble =,
    pub E1_HOSTTOMODEM: cmv.bDirection =,
    pub function: cmv.bFunction =,
    pub cpu_to_le16(sc->cmv_dsc.e1.idx): cmv.wIndex =,
    pub &cmv.dwSymbolicAddress): put_unaligned_le32(address,,
    pub cpu_to_le16(offset): cmv.wOffsetAddress =,
    pub &cmv.dwData): put_unaligned_le32(data >> 16 | data << 16,,
    ret = uea_request(sc, UEA_E1_SET_BLOCK, UEA_MPTX_START,
    pub &cmv): sizeof(cmv),,
    if (ret < 0)
    pub ret: return,
    pub wait_cmv_ack(sc): return,
    }
    static int uea_cmv_e4(struct uea_softc *sc,
    u16 function, u16 group, u16 address, u16 offset, u32 data)
    {
    pub cmv: cmv_e4,
    pub ret: c_int,
    pub sizeof(cmv)): memset(&cmv, 0,,
    uea_vdbg(INS_TO_USBDEV(sc), "Function : %d-%d, Group : 0x%04x, "
    "Address : 0x%04x, offset : 0x%04x, data : 0x%08x\n",
    E4_FUNCTION_TYPE(function), E4_FUNCTION_SUBTYPE(function),
    pub data): group, address, offset,,
// we send a request, but we expect a reply
    pub 4): sc->cmv_dsc.e4.function = function | (0x1 <<,
    pub offset: sc->cmv_dsc.e4.offset =,
    pub address: sc->cmv_dsc.e4.address =,
    pub group: sc->cmv_dsc.e4.group =,
    pub cpu_to_be16(function): cmv.wFunction =,
    pub cpu_to_be16(group): cmv.wGroup =,
    pub cpu_to_be16(address): cmv.wAddress =,
    pub cpu_to_be16(offset): cmv.wOffset =,
    pub cpu_to_be32(data): cmv.dwData[0] =,
    ret = uea_request(sc, UEA_E4_SET_BLOCK, UEA_MPTX_START,
    pub &cmv): sizeof(cmv),,
    if (ret < 0)
    pub ret: return,
    pub wait_cmv_ack(sc): return,
    }
    static inline int uea_read_cmv_e1(struct uea_softc *sc,
    u32 address, u16 offset, u32 *data)
    {
    int ret = uea_cmv_e1(sc, E1_MAKEFUNCTION(E1_MEMACCESS, E1_REQUESTREAD),
    pub 0): address, offset,,
    if (ret < 0)
    uea_err(INS_TO_USBDEV(sc),
    pub ret): "reading cmv failed with error %d\n",,
    else
// data = sc->data;
    pub ret: return,
    }
    static inline int uea_read_cmv_e4(struct uea_softc *sc,
    u8 size, u16 group, u16 address, u16 offset, u32 *data)
    {
    int ret = uea_cmv_e4(sc, E4_MAKEFUNCTION(E4_MEMACCESS,
    E4_REQUESTREAD, size),
    pub 0): group, address, offset,,
    if (ret < 0)
    uea_err(INS_TO_USBDEV(sc),
    pub ret): "reading cmv failed with error %d\n",,
    else {
// data = sc->data;
// size is in 16-bit word quantities
    if (size > 2)
// (data + 1) = sc->data1;
    }
    pub ret: return,
    }
    static inline int uea_write_cmv_e1(struct uea_softc *sc,
    u32 address, u16 offset, u32 data)
    {
    int ret = uea_cmv_e1(sc, E1_MAKEFUNCTION(E1_MEMACCESS, E1_REQUESTWRITE),
    pub data): address, offset,,
    if (ret < 0)
    uea_err(INS_TO_USBDEV(sc),
    pub ret): "writing cmv failed with error %d\n",,
    pub ret: return,
    }
    static inline int uea_write_cmv_e4(struct uea_softc *sc,
    u8 size, u16 group, u16 address, u16 offset, u32 data)
    {
    int ret = uea_cmv_e4(sc, E4_MAKEFUNCTION(E4_MEMACCESS,
    E4_REQUESTWRITE, size),
    pub data): group, address, offset,,
    if (ret < 0)
    uea_err(INS_TO_USBDEV(sc),
    pub ret): "writing cmv failed with error %d\n",,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn uea_set_bulk_timeout(sc: *mut uea_softc, dsrate: u32) {
    static void uea_set_bulk_timeout(struct uea_softc *sc, u32 dsrate)
    {
    pub ret: c_int,
    pub timeout: u16,
// in bulk mode the modem have problem with high rate
// changing internal timing could improve things, but the
// value is mysterious.
// ADI930 don't support it (-EPIPE error).
//
    if (UEA_CHIP_VERSION(sc) == ADI930 ||
    altsetting[sc.modem_index] > 0 ||
    sc.stats.phy.dsrate == dsrate)
// Original timing (1Mbit/s) from ADI (used in windows driver)
    pub 1: *mut *mut timeout = (dsrate <= 10241024) ? 0 :,
    pub NULL): ret = uea_request(sc, UEA_SET_TIMEOUT, timeout, 0,,
    uea_info(INS_TO_USBDEV(sc), "setting new timeout %d%s\n",
    pub ""): timeout, ret < 0 ? " failed" :,
    }
//
// Monitor the modem and update the stat
// return 0 if everything is ok
// return < 0 if an error occurs (-EAGAIN reboot needed)
//
#[no_mangle]
unsafe extern "C" fn uea_stat_e1(sc: *mut uea_softc) -> c_int {
    static int uea_stat_e1(struct uea_softc *sc)
    {
    pub data: u32,
    pub ret: c_int,
    pub sc->stats.phy.state: data =,
    pub &sc->stats.phy.state): ret = uea_read_cmv_e1(sc, E1_SA_STAT, 0,,
    if (ret < 0)
    pub ret: return,
    switch (GET_STATUS(sc.stats.phy.state)) {
    case 0:		/* not yet synchronized */
    uea_dbg(INS_TO_USBDEV(sc),
    pub synchronized\n"): "modem not yet,
    pub 0: return,
    case 1:		/* initialization */
    pub initializing\n"): uea_dbg(INS_TO_USBDEV(sc), "modem,
    pub 0: return,
    case 2:		/* operational */
    pub operational\n"): uea_vdbg(INS_TO_USBDEV(sc), "modem,
    case 3:		/* fail ... */
    uea_info(INS_TO_USBDEV(sc), "modem synchronization failed"
    pub cmv/dsp)\n"): " (may be try other,
    pub -EAGAIN: return,
    case 4 ... 6:	/* test state */
    uea_warn(INS_TO_USBDEV(sc),
    pub supported\n"): "modem in test mode - not,
    pub -EAGAIN: return,
    case 7:		/* fast-retain ... */
    pub mode\n"): uea_info(INS_TO_USBDEV(sc), "modem in fast-retain,
    pub 0: return,
    default:
    uea_err(INS_TO_USBDEV(sc), "modem invalid SW mode %d\n",
    pub -EAGAIN: return,
    }
    if (GET_STATUS(data) != 2) {
    pub NULL): uea_request(sc, UEA_SET_MODE, UEA_LOOPBACK_OFF, 0,,
    pub operational\n"): uea_info(INS_TO_USBDEV(sc), "modem,
// release the dsp firmware as it is not needed until
// the next failure
//
    pub NULL: sc->dsp_firm =,
    }
// always update it as atm layer could not be init when we switch to
// operational state
//
// wake up processes waiting for synchronization
    pub &sc->stats.phy.flags): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 2,,
    if (ret < 0)
    pub ret: return,
    pub sc->stats.phy.flags: sc->stats.phy.mflags |=,
// in case of a flags ( for example delineation LOSS (& 0x10)),
// we check the status again in order to detect the failure earlier
//
    if (sc.stats.phy.flags) {
    uea_dbg(INS_TO_USBDEV(sc), "Stat flag = 0x%x\n",
    pub 0: return,
    }
    pub &data): ret = uea_read_cmv_e1(sc, E1_SA_RATE, 0,,
    if (ret < 0)
    pub ret: return,
    pub 32): *mut *mut uea_set_bulk_timeout(sc, (data >> 16),
    pub 32: *mut *mut sc->stats.phy.dsrate = (data >> 16),
    pub 32: *mut *mut sc->stats.phy.usrate = (data & 0xffff),
    pub 424): *mut *mut UPDATE_ATM_STAT(link_rate, sc->stats.phy.dsrate  1000 /,
    pub &data): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 23,,
    if (ret < 0)
    pub ret: return,
    pub 2: sc->stats.phy.dsattenuation = (data & 0xff) /,
    pub &data): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 47,,
    if (ret < 0)
    pub ret: return,
    pub 2: sc->stats.phy.usattenuation = (data & 0xff) /,
    pub &sc->stats.phy.dsmargin): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 25,,
    if (ret < 0)
    pub ret: return,
    pub &sc->stats.phy.usmargin): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 49,,
    if (ret < 0)
    pub ret: return,
    pub &sc->stats.phy.rxflow): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 51,,
    if (ret < 0)
    pub ret: return,
    pub &sc->stats.phy.txflow): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 52,,
    if (ret < 0)
    pub ret: return,
    pub &sc->stats.phy.dsunc): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 54,,
    if (ret < 0)
    pub ret: return,
// only for atu-c
    pub &sc->stats.phy.usunc): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 58,,
    if (ret < 0)
    pub ret: return,
    pub &sc->stats.phy.dscorr): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 53,,
    if (ret < 0)
    pub ret: return,
// only for atu-c
    pub &sc->stats.phy.uscorr): ret = uea_read_cmv_e1(sc, E1_SA_DIAG, 57,,
    if (ret < 0)
    pub ret: return,
    pub &sc->stats.phy.vidco): ret = uea_read_cmv_e1(sc, E1_SA_INFO, 8,,
    if (ret < 0)
    pub ret: return,
    pub &sc->stats.phy.vidcpe): ret = uea_read_cmv_e1(sc, E1_SA_INFO, 13,,
    if (ret < 0)
    pub ret: return,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn uea_stat_e4(sc: *mut uea_softc) -> c_int {
    static int uea_stat_e4(struct uea_softc *sc)
    {
    pub data: u32,
    pub tmp_arr: [u32; 2],
    pub ret: c_int,
    pub sc->stats.phy.state: data =,
// XXX only need to be done before operationnal...
    pub &sc->stats.phy.state): ret = uea_read_cmv_e4(sc, 1, E4_SA_STAT, 0, 0,,
    if (ret < 0)
    pub ret: return,
    switch (sc.stats.phy.state) {
    case 0x0:	/* not yet synchronized */
    case 0x1:
    case 0x3:
    case 0x4:
    uea_dbg(INS_TO_USBDEV(sc), "modem not yet "
    pub 0: return,
    case 0x5:	/* initialization */
    case 0x6:
    case 0x9:
    case 0xa:
    pub initializing\n"): uea_dbg(INS_TO_USBDEV(sc), "modem,
    pub 0: return,
    case 0x2:	/* fail ... */
    uea_info(INS_TO_USBDEV(sc), "modem synchronization "
    pub cmv/dsp)\n"): "failed (may be try other,
    pub -EAGAIN: return,
    case 0x7:	/* operational */
    default:
    uea_warn(INS_TO_USBDEV(sc), "unknown state: %x\n",
    pub 0: return,
    }
    if (data != 7) {
    pub NULL): uea_request(sc, UEA_SET_MODE, UEA_LOOPBACK_OFF, 0,,
    pub operational\n"): uea_info(INS_TO_USBDEV(sc), "modem,
// release the dsp firmware as it is not needed until
// the next failure
//
    pub NULL: sc->dsp_firm =,
    }
// always update it as atm layer could not be init when we switch to
// operational state
//
// wake up processes waiting for synchronization
// TODO improve this state machine :
// we need some CMV info : what they do and their unit
// we should find the equivalent of eagle3- CMV
//
// check flags
    pub &sc->stats.phy.flags): ret = uea_read_cmv_e4(sc, 1, E4_SA_DIAG, 0, 0,,
    if (ret < 0)
    pub ret: return,
    pub sc->stats.phy.flags: sc->stats.phy.mflags |=,
// in case of a flags ( for example delineation LOSS (& 0x10)),
// we check the status again in order to detect the failure earlier
//
    if (sc.stats.phy.flags) {
    uea_dbg(INS_TO_USBDEV(sc), "Stat flag = 0x%x\n",
    if (sc.stats.phy.flags & 1) /* delineation LOSS */
    pub -EAGAIN: return,
    if (sc.stats.phy.flags & 0x4000) /* Reset Flag */
    pub -EAGAIN: return,
    pub 0: return,
    }
// rate data may be in upper or lower half of 64 bit word, strange
    pub tmp_arr): ret = uea_read_cmv_e4(sc, 4, E4_SA_RATE, 0, 0,,
    if (ret < 0)
    pub ret: return,
    pub tmp_arr: [data = (tmp_arr[0]) ? tmp_arr[0] :; 1],
    pub 1000: sc->stats.phy.usrate = data /,
    pub tmp_arr): ret = uea_read_cmv_e4(sc, 4, E4_SA_RATE, 1, 0,,
    if (ret < 0)
    pub ret: return,
    pub tmp_arr: [data = (tmp_arr[0]) ? tmp_arr[0] :; 1],
    pub 1000): uea_set_bulk_timeout(sc, data /,
    pub 1000: sc->stats.phy.dsrate = data /,
    pub 424): *mut *mut UPDATE_ATM_STAT(link_rate, sc->stats.phy.dsrate  1000 /,
    pub &data): ret = uea_read_cmv_e4(sc, 1, E4_SA_INFO, 68, 1,,
    if (ret < 0)
    pub ret: return,
    pub 10: sc->stats.phy.dsattenuation = data /,
    pub &data): ret = uea_read_cmv_e4(sc, 1, E4_SA_INFO, 69, 1,,
    if (ret < 0)
    pub ret: return,
    pub 10: sc->stats.phy.usattenuation = data /,
    pub &data): ret = uea_read_cmv_e4(sc, 1, E4_SA_INFO, 68, 3,,
    if (ret < 0)
    pub ret: return,
    pub 2: sc->stats.phy.dsmargin = data /,
    pub &data): ret = uea_read_cmv_e4(sc, 1, E4_SA_INFO, 69, 3,,
    if (ret < 0)
    pub ret: return,
    pub 10: sc->stats.phy.usmargin = data /,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn cmvs_file_name(sc: *mut uea_softc, cmv_name: *const *const c_char, ver: c_int) {
    static void cmvs_file_name(struct uea_softc *sc, char *const cmv_name, int ver)
    {
    pub "CMVxy.bin": char file_arr[] =,
    pub file: *mut c_char,
// set proper name corresponding modem version and line type
    if (cmv_file[sc.modem_index] == core::ptr::null_mut()) {
    if (UEA_CHIP_VERSION(sc) == ADI930)
    pub '9': file_arr[3] =,
#[no_mangle]
pub unsafe extern "C" fn if(EAGLE_IV: UEA_CHIP_VERSION(sc) ==) -> else {
    else if (UEA_CHIP_VERSION(sc) == EAGLE_IV)
    pub '4': file_arr[3] =,
    else
    pub 'e': file_arr[3] =,
    pub 'p': file_arr[4] = IS_ISDN(sc) ? 'i' :,
    pub file_arr: file =,
    } else
    pub cmv_file: [file =; sc->modem_index],
    snprintf(cmv_name, UEA_FW_NAME_MAX, FW_DIR "%s%s",
    pub ""): file, ver == 2 ? ".v2" :,
    }
    static int request_cmvs_old(struct uea_softc *sc,
    void **cmvs, const struct firmware **fw)
    {
    pub size: int ret,,
    pub data: *mut u8,
    pub /: *mut *mut char cmv_name[UEA_FW_NAME_MAX]; / 30 bytes stack variable,
    pub 1): cmvs_file_name(sc, cmv_name,,
    pub &sc->usb_dev->dev): ret = request_firmware(fw, cmv_name,,
    if (ret < 0) {
    uea_err(INS_TO_USBDEV(sc),
    "requesting firmware %s failed with error %d\n",
    pub ret): cmv_name,,
    pub ret: return,
    }
    pub (*fw)->data: *mut *mut data = (u8 ),
    pub (*fw)->size: *mut size =,
    if (size < 1)
    pub err_fw_corrupted: goto,
    if (size != *data * sizeof(struct uea_cmvs_v1) + 1)
    pub err_fw_corrupted: goto,
// cmvs = (void *)(data + 1);
    pub data: *mut return,
    err_fw_corrupted:
    pub cmv_name): uea_err(INS_TO_USBDEV(sc), "firmware %s is corrupted\n",,
    pub -EILSEQ: return,
    }
    static int request_cmvs(struct uea_softc *sc,
    void **cmvs, const struct firmware **fw, int *ver)
    {
    pub size: int ret,,
    pub crc: u32,
    pub data: *mut u8,
    pub /: *mut *mut char cmv_name[UEA_FW_NAME_MAX]; / 30 bytes stack variable,
    pub 2): cmvs_file_name(sc, cmv_name,,
    pub &sc->usb_dev->dev): ret = request_firmware(fw, cmv_name,,
    if (ret < 0) {
// if caller can handle old version, try to provide it
    if (*ver == 1) {
    uea_warn(INS_TO_USBDEV(sc), "requesting "
    "firmware %s failed, "
    pub cmv_name): "try to get older cmvs\n",,
    pub fw): return request_cmvs_old(sc, cmvs,,
    }
    uea_err(INS_TO_USBDEV(sc),
    "requesting firmware %s failed with error %d\n",
    pub ret): cmv_name,,
    pub ret: return,
    }
    pub (*fw)->size: *mut size =,
    pub (*fw)->data: *mut *mut data = (u8 ),
    if (size < 4 || strncmp(data, "cmv2", 4) != 0) {
    if (*ver == 1) {
    uea_warn(INS_TO_USBDEV(sc), "firmware %s is corrupted,"
    pub cmv_name): " try to get older cmvs\n",,
    pub fw): return request_cmvs_old(sc, cmvs,,
    }
    pub err_fw_corrupted: goto,
    }
// ver = 2;
    pub 4: data +=,
    pub 4: size -=,
    if (size < 5)
    pub err_fw_corrupted: goto,
    pub get_unaligned_le32(data): crc =,
    pub 4: data +=,
    pub 4: size -=,
    if (crc32_be(0, data, size) != crc)
    pub err_fw_corrupted: goto,
    if (size != *data * sizeof(struct uea_cmvs_v2) + 1)
    pub err_fw_corrupted: goto,
// cmvs = (void *) (data + 1);
    pub data: *mut return,
    err_fw_corrupted:
    pub cmv_name): uea_err(INS_TO_USBDEV(sc), "firmware %s is corrupted\n",,
    pub -EILSEQ: return,
    }
#[no_mangle]
unsafe extern "C" fn uea_send_cmvs_e1(sc: *mut uea_softc) -> c_int {
    static int uea_send_cmvs_e1(struct uea_softc *sc)
    {
    pub len: int i, ret,,
    pub cmvs_ptr: *mut c_void,
    pub cmvs_fw: *const firmware,
    pub /: *mut *mut int ver = 1; / we can handle v1 cmv firmware version;,
// Enter in R-IDLE (cmv) until instructed otherwise
    pub 1): ret = uea_write_cmv_e1(sc, E1_SA_CNTL, 0,,
    if (ret < 0)
    pub ret: return,
// Dump firmware version
    pub &sc->stats.phy.firmid): ret = uea_read_cmv_e1(sc, E1_SA_INFO, 10,,
    if (ret < 0)
    pub ret: return,
    uea_info(INS_TO_USBDEV(sc), "ATU-R firmware version : %x\n",
// get options
    pub &ver): ret = len = request_cmvs(sc, &cmvs_ptr, &cmvs_fw,,
    if (ret < 0)
    pub ret: return,
// send options
    if (ver == 1) {
    pub cmvs_ptr: *mut *mut uea_cmvs_v1 cmvs_v1 =,
    uea_warn(INS_TO_USBDEV(sc), "use deprecated cmvs version, "
    pub firmware\n"): "please update your,
    pub {: for (i = 0; i < len; i++),
    ret = uea_write_cmv_e1(sc,
    get_unaligned_le32(&cmvs_v1[i].address),
    get_unaligned_le16(&cmvs_v1[i].offset),
    if (ret < 0)
    pub out: goto,
    }
    } else if (ver == 2) {
    pub cmvs_ptr: *mut *mut uea_cmvs_v2 cmvs_v2 =,
    pub {: for (i = 0; i < len; i++),
    ret = uea_write_cmv_e1(sc,
    get_unaligned_le32(&cmvs_v2[i].address),
    (u16) get_unaligned_le32(&cmvs_v2[i].offset),
    if (ret < 0)
    pub out: goto,
    }
    } else {
// This really should not happen
    pub ver): uea_err(INS_TO_USBDEV(sc), "bad cmvs version %d\n",,
    pub out: goto,
    }
// Enter in R-ACT-REQ
    pub 2): ret = uea_write_cmv_e1(sc, E1_SA_CNTL, 0,,
    pub state\n"): uea_vdbg(INS_TO_USBDEV(sc), "Entering in R-ACT-REQ,
    uea_info(INS_TO_USBDEV(sc), "modem started, waiting "
    out:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn uea_send_cmvs_e4(sc: *mut uea_softc) -> c_int {
    static int uea_send_cmvs_e4(struct uea_softc *sc)
    {
    pub len: int i, ret,,
    pub cmvs_ptr: *mut c_void,
    pub cmvs_fw: *const firmware,
    pub /: *mut *mut int ver = 2; / we can only handle v2 cmv firmware version;,
// Enter in R-IDLE (cmv) until instructed otherwise
    pub 1): ret = uea_write_cmv_e4(sc, 1, E4_SA_CNTL, 0, 0,,
    if (ret < 0)
    pub ret: return,
// Dump firmware version
// XXX don't read the 3th byte as it is always 6
    pub &sc->stats.phy.firmid): ret = uea_read_cmv_e4(sc, 2, E4_SA_INFO, 55, 0,,
    if (ret < 0)
    pub ret: return,
    uea_info(INS_TO_USBDEV(sc), "ATU-R firmware version : %x\n",
// get options
    pub &ver): ret = len = request_cmvs(sc, &cmvs_ptr, &cmvs_fw,,
    if (ret < 0)
    pub ret: return,
// send options
    if (ver == 2) {
    pub cmvs_ptr: *mut *mut uea_cmvs_v2 cmvs_v2 =,
    pub {: for (i = 0; i < len; i++),
    ret = uea_write_cmv_e4(sc, 1,
    get_unaligned_le32(&cmvs_v2[i].group),
    get_unaligned_le32(&cmvs_v2[i].address),
    get_unaligned_le32(&cmvs_v2[i].offset),
    if (ret < 0)
    pub out: goto,
    }
    } else {
// This really should not happen
    pub ver): uea_err(INS_TO_USBDEV(sc), "bad cmvs version %d\n",,
    pub out: goto,
    }
// Enter in R-ACT-REQ
    pub 2): ret = uea_write_cmv_e4(sc, 1, E4_SA_CNTL, 0, 0,,
    pub state\n"): uea_vdbg(INS_TO_USBDEV(sc), "Entering in R-ACT-REQ,
    uea_info(INS_TO_USBDEV(sc), "modem started, waiting "
    out:
    pub ret: return,
    }
// Start boot post firmware modem:
// - send reset commands through usb control pipe
// - start workqueue for DSP loading
// - send CMV options to modem
//
#[no_mangle]
unsafe extern "C" fn uea_start_reset(sc: *mut uea_softc) -> c_int {
    static int uea_start_reset(struct uea_softc *sc)
    {
    pub /: *mut *mut u16 zero = 0; / ;-),
    pub ret: c_int,
    pub started\n"): uea_info(INS_TO_USBDEV(sc), "(re)booting,
// mask interrupt
    pub 1: sc->booting =,
// We need to set this here because, a ack timeout could have occurred,
// but before we start the reboot, the ack occurs and set this to 1.
// So we will failed to wait Ready CMV.
//
    pub 0: sc->cmv_ack =,
// reset statistics
    pub uea_stats)): memset(&sc->stats, 0, sizeof(struct,
// tell the modem that we want to boot in IDMA mode
    pub NULL): uea_request(sc, UEA_SET_MODE, UEA_LOOPBACK_ON, 0,,
    pub NULL): uea_request(sc, UEA_SET_MODE, UEA_BOOT_IDMA, 0,,
// enter reset mode
    pub NULL): uea_request(sc, UEA_SET_MODE, UEA_START_RESET, 0,,
// original driver use 200ms, but windows driver use 100ms
    pub msecs_to_jiffies(100)): ret = uea_wait(sc, 0,,
    if (ret < 0)
    pub ret: return,
// leave reset mode
    pub NULL): uea_request(sc, UEA_SET_MODE, UEA_END_RESET, 0,,
    if (UEA_CHIP_VERSION(sc) != EAGLE_IV) {
// clear tx and rx mailboxes
    pub &zero): uea_request(sc, UEA_SET_2183_DATA, UEA_MPTX_MAILBOX, 2,,
    pub &zero): uea_request(sc, UEA_SET_2183_DATA, UEA_MPRX_MAILBOX, 2,,
    pub &zero): uea_request(sc, UEA_SET_2183_DATA, UEA_SWAP_MAILBOX, 2,,
    }
    pub msecs_to_jiffies(1000)): ret = uea_wait(sc, 0,,
    if (ret < 0)
    pub ret: return,
    if (UEA_CHIP_VERSION(sc) == EAGLE_IV)
    sc.cmv_dsc.e4.function = E4_MAKEFUNCTION(E4_ADSLDIRECTIVE,
    pub 1): E4_MODEMREADY,,
    else
    sc.cmv_dsc.e1.function = E1_MAKEFUNCTION(E1_ADSLDIRECTIVE,
// demask interrupt
    pub 0: sc->booting =,
// start loading DSP
    pub 0: sc->pageno =,
    pub 0: sc->ovl =,
// wait for modem ready CMV
    pub wait_cmv_ack(sc): ret =,
    if (ret < 0)
    pub ret: return,
    pub received\n"): uea_vdbg(INS_TO_USBDEV(sc), "Ready CMV,
    pub sc->send_cmvs(sc): ret =,
    if (ret < 0)
    pub ret: return,
    pub 0: sc->reset =,
    pub ret: return,
    }
//
// In case of an error wait 1s before rebooting the modem
// if the modem don't request reboot (-EAGAIN).
// Monitor the modem every 1s.
//
#[no_mangle]
unsafe extern "C" fn uea_kthread(data: *mut c_void) -> c_int {
    static int uea_kthread(void *data)
    {
    pub data: *mut *mut uea_softc sc =,
    pub -EAGAIN: int ret =,
    while (!kthread_should_stop()) {
    if (ret < 0 || sc.reset)
    pub uea_start_reset(sc): ret =,
    if (!ret)
    pub sc->stat(sc): ret =,
    if (ret != -EAGAIN)
    pub msecs_to_jiffies(1000)): uea_wait(sc, 0,,
    }
    pub ret: return,
    }
// Load second usb firmware for ADI930 chip
#[no_mangle]
unsafe extern "C" fn load_XILINX_firmware(sc: *mut uea_softc) -> c_int {
    static int load_XILINX_firmware(struct uea_softc *sc)
    {
    pub fw_entry: *const firmware,
    pub ln: int ret, size, u,,
    pub pfw: *const u8,
    pub value: u8,
    pub FPGA930_FIRMWARE: *mut *mut char fw_name =,
    pub &sc->usb_dev->dev): ret = request_firmware(&fw_entry, fw_name,,
    if (ret) {
    uea_err(INS_TO_USBDEV(sc), "firmware %s is not available\n",
    pub err0: goto,
    }
    pub fw_entry->data: pfw =,
    pub fw_entry->size: size =,
    if (size != 0x577B) {
    uea_err(INS_TO_USBDEV(sc), "firmware %s is corrupted\n",
    pub -EILSEQ: ret =,
    pub err1: goto,
    }
    pub {: for (u = 0; u < size; u += ln),
    pub 64): ln = min(size - u,,
    pub u): ret = uea_request(sc, 0xe, 0, ln, pfw +,
    if (ret < 0) {
    uea_err(INS_TO_USBDEV(sc),
    pub ret): "elsa download data failed (%d)\n",,
    pub err1: goto,
    }
    }
// finish to send the fpga
    pub NULL): ret = uea_request(sc, 0xe, 1, 0,,
    if (ret < 0) {
    uea_err(INS_TO_USBDEV(sc),
    pub ret): "elsa download data failed (%d)\n",,
    pub err1: goto,
    }
// Tell the modem we finish : de-assert reset
    pub 0: value =,
    pub &value): ret = uea_send_modem_cmd(sc->usb_dev, 0xe, 1,,
    if (ret < 0)
    uea_err(sc.usb_dev, "elsa de-assert failed with error"
    pub ret): " %d\n",,
    err1:
    err0:
    pub ret: return,
    }
// The modem send us an ack. First with check if it right
#[no_mangle]
unsafe extern "C" fn uea_dispatch_cmv_e1(sc: *mut uea_softc, intr: *mut intr_pkt) {
    static void uea_dispatch_cmv_e1(struct uea_softc *sc, struct intr_pkt *intr)
    {
    pub &sc->cmv_dsc.e1: *mut *mut cmv_dsc_e1 dsc =,
    pub &intr->u.e1.s2.cmv: *mut *mut cmv_e1 cmv =,
    if (le16_to_cpu(cmv.wPreamble) != E1_PREAMBLE)
    pub bad1: goto,
    if (cmv.bDirection != E1_MODEMTOHOST)
    pub bad1: goto,
// FIXME : ADI930 reply wrong preamble (func = 2, sub = 2) to
// the first MEMACCESS cmv. Ignore it...
//
    if (cmv.bFunction != dsc.function) {
    if (UEA_CHIP_VERSION(sc) == ADI930
    && cmv.bFunction ==  E1_MAKEFUNCTION(2, 2)) {
    pub cpu_to_le16(dsc->idx): cmv->wIndex =,
    put_unaligned_le32(dsc.address,
    pub cpu_to_le16(dsc->offset): cmv->wOffsetAddress =,
    } else
    pub bad2: goto,
    }
    if (cmv.bFunction == E1_MAKEFUNCTION(E1_ADSLDIRECTIVE,
    E1_MODEMREADY)) {
    }
// in case of MEMACCESS
    if (le16_to_cpu(cmv.wIndex) != dsc.idx ||
    get_unaligned_le32(&cmv.dwSymbolicAddress) != dsc.address ||
    le16_to_cpu(cmv.wOffsetAddress) != dsc.offset)
    pub bad2: goto,
    pub get_unaligned_le32(&cmv->dwData): sc->data =,
    pub 16: sc->data = sc->data << 16 | sc->data >>,
    bad2:
    uea_err(INS_TO_USBDEV(sc), "unexpected cmv received, "
    "Function : %d, Subfunction : %d\n",
    E1_FUNCTION_TYPE(cmv.bFunction),
    bad1:
    uea_err(INS_TO_USBDEV(sc), "invalid cmv received, "
    "wPreamble %d, bDirection %d\n",
    pub cmv->bDirection): le16_to_cpu(cmv->wPreamble),,
    }
// The modem send us an ack. First with check if it right
#[no_mangle]
unsafe extern "C" fn uea_dispatch_cmv_e4(sc: *mut uea_softc, intr: *mut intr_pkt) {
    static void uea_dispatch_cmv_e4(struct uea_softc *sc, struct intr_pkt *intr)
    {
    pub &sc->cmv_dsc.e4: *mut *mut cmv_dsc_e4 dsc =,
    pub &intr->u.e4.s2.cmv: *mut *mut cmv_e4 cmv =,
    uea_dbg(INS_TO_USBDEV(sc), "cmv %x %x %x %x %x %x\n",
    be16_to_cpu(cmv.wGroup), be16_to_cpu(cmv.wFunction),
    be16_to_cpu(cmv.wOffset), be16_to_cpu(cmv.wAddress),
    pub be32_to_cpu(cmv->dwData[1])): be32_to_cpu(cmv->dwData[0]),,
    if (be16_to_cpu(cmv.wFunction) != dsc.function)
    pub bad2: goto,
    if (be16_to_cpu(cmv.wFunction) == E4_MAKEFUNCTION(E4_ADSLDIRECTIVE,
    E4_MODEMREADY, 1)) {
    }
// in case of MEMACCESS
    if (be16_to_cpu(cmv.wOffset) != dsc.offset ||
    be16_to_cpu(cmv.wGroup) != dsc.group ||
    be16_to_cpu(cmv.wAddress) != dsc.address)
    pub bad2: goto,
    pub be32_to_cpu(cmv->dwData[0]): sc->data =,
    pub be32_to_cpu(cmv->dwData[1]): sc->data1 =,
    bad2:
    uea_err(INS_TO_USBDEV(sc), "unexpected cmv received, "
    "Function : %d, Subfunction : %d\n",
    E4_FUNCTION_TYPE(cmv.wFunction),
    }
    static void uea_schedule_load_page_e1(struct uea_softc *sc,
    struct intr_pkt *intr)
    {
    pub intr->e1_bSwapPageNo: sc->pageno =,
    pub 4: sc->ovl = intr->e1_bOvl >> 4 | intr->e1_bOvl <<,
    }
    static void uea_schedule_load_page_e4(struct uea_softc *sc,
    struct intr_pkt *intr)
    {
    pub intr->e4_bSwapPageNo: sc->pageno =,
    }
//
// interrupt handler
//
#[no_mangle]
unsafe extern "C" fn uea_intr(urb: *mut urb) {
    static void uea_intr(struct urb *urb)
    {
    pub urb->context: *mut *mut uea_softc sc =,
    pub urb->transfer_buffer: *mut *mut intr_pkt intr =,
    pub urb->status: int status =,
    if (unlikely(status < 0)) {
    uea_err(INS_TO_USBDEV(sc), "uea_intr() failed with %d\n",
    }
// device-to-host interrupt
    if (intr.bType != 0x08 || sc.booting) {
    pub interrupt\n"): uea_err(INS_TO_USBDEV(sc), "wrong,
    pub resubmit: goto,
    }
    switch (le16_to_cpu(intr.wInterrupt)) {
    case INT_LOADSWAPPAGE:
    pub intr): sc->schedule_load_page(sc,,
    case INT_INCOMINGCMV:
    pub intr): sc->dispatch_cmv(sc,,
    default:
    uea_err(INS_TO_USBDEV(sc), "unknown interrupt %u\n",
    }
    resubmit:
    pub GFP_ATOMIC): usb_submit_urb(sc->urb_int,,
    }
//
// Start the modem : init the data and start kernel thread
//
#[no_mangle]
unsafe extern "C" fn uea_boot(sc: *mut uea_softc, intf: *mut usb_interface) -> c_int {
    static int uea_boot(struct uea_softc *sc, struct usb_interface *intf)
    {
    pub intr: *mut intr_pkt,
    pub -ENOMEM: int ret =,
    pub size: c_int,
    if (UEA_CHIP_VERSION(sc) == EAGLE_IV) {
    pub E4_INTR_PKT_SIZE: size =,
    pub uea_dispatch_cmv_e4: sc->dispatch_cmv =,
    pub uea_schedule_load_page_e4: sc->schedule_load_page =,
    pub uea_stat_e4: sc->stat =,
    pub uea_send_cmvs_e4: sc->send_cmvs =,
    pub uea_load_page_e4): INIT_WORK(&sc->task,,
    } else {
    pub E1_INTR_PKT_SIZE: size =,
    pub uea_dispatch_cmv_e1: sc->dispatch_cmv =,
    pub uea_schedule_load_page_e1: sc->schedule_load_page =,
    pub uea_stat_e1: sc->stat =,
    pub uea_send_cmvs_e1: sc->send_cmvs =,
    pub uea_load_page_e1): INIT_WORK(&sc->task,,
    }
    if (UEA_CHIP_VERSION(sc) == ADI930)
    if (intf.cur_altsetting.desc.bNumEndpoints < 1) {
    pub -ENODEV: ret =,
    pub err0: goto,
    }
    pub GFP_KERNEL): intr = kmalloc(size,,
    if (!intr)
    pub err0: goto,
    pub GFP_KERNEL): sc->urb_int = usb_alloc_urb(0,,
    if (!sc.urb_int)
    pub err1: goto,
    usb_fill_int_urb(sc.urb_int, sc.usb_dev,
    usb_rcvintpipe(sc.usb_dev, UEA_INTR_PIPE),
    intr, size, uea_intr, sc,
    pub GFP_KERNEL): ret = usb_submit_urb(sc->urb_int,,
    if (ret < 0) {
    uea_err(INS_TO_USBDEV(sc),
    pub ret): "urb submission failed with error %d\n",,
    pub err1: goto,
    }
// Create worker thread, but don't start it here.  Start it after
// all usbatm generic initialization is done.
//
    pub "ueagle-atm"): sc->kthread = kthread_create(uea_kthread, sc,,
    if (IS_ERR(sc.kthread)) {
    pub thread\n"): uea_err(INS_TO_USBDEV(sc), "failed to create,
    pub PTR_ERR(sc->kthread): ret =,
    pub err2: goto,
    }
    pub 0: return,
    err2:
    err1:
    pub NULL: sc->urb_int =,
    err0:
    pub ret: return,
    }
//
// Stop the modem : kill kernel thread and free data
//
#[no_mangle]
unsafe extern "C" fn uea_stop(sc: *mut uea_softc) {
    static void uea_stop(struct uea_softc *sc)
    {
    pub ret: c_int,
    pub kthread_stop(sc->kthread): ret =,
    pub ret): uea_dbg(INS_TO_USBDEV(sc), "kthread finish with status %d\n",,
    pub NULL): uea_request(sc, UEA_SET_MODE, UEA_LOOPBACK_ON, 0,,
// flush the work item, when no one can schedule it
    }
// syfs interface
    static struct uea_softc *dev_to_uea(struct device *dev)
    {
    pub intf: *mut usb_interface,
    pub usbatm: *mut usbatm_data,
    pub to_usb_interface(dev): intf =,
    if (!intf)
    pub NULL: return,
    pub usb_get_intfdata(intf): usbatm =,
    if (!usbatm)
    pub NULL: return,
    pub usbatm->driver_data: return,
    }
    static ssize_t stat_status_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub -ENODEV: int ret =,
    pub sc: *mut uea_softc,
    pub dev_to_uea(dev): sc =,
    if (!sc)
    pub out: goto,
    pub sc->stats.phy.state): ret = sysfs_emit(buf, "%08x\n",,
    out:
    pub ret: return,
    }
    static ssize_t stat_status_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    pub -ENODEV: int ret =,
    pub sc: *mut uea_softc,
    pub dev_to_uea(dev): sc =,
    if (!sc)
    pub out: goto,
    pub 1: sc->reset =,
    pub count: ret =,
    out:
    pub ret: return,
    }
    pub DEVICE_ATTR_RW(stat_status): static,
    static ssize_t stat_human_status_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    pub -ENODEV: int ret =,
    pub modem_state: c_int,
    pub sc: *mut uea_softc,
    pub dev_to_uea(dev): sc =,
    if (!sc)
    pub out: goto,
    if (UEA_CHIP_VERSION(sc) == EAGLE_IV) {
    switch (sc.stats.phy.state) {
    case 0x0:	/* not yet synchronized */
    case 0x1:
    case 0x3:
    case 0x4:
    pub 0: modem_state =,
    case 0x5:	/* initialization */
    case 0x6:
    case 0x9:
    case 0xa:
    pub 1: modem_state =,
    case 0x7:	/* operational */
    pub 2: modem_state =,
    case 0x2:	/* fail ... */
    pub 3: modem_state =,
    default:	/* unknown */
    pub 4: modem_state =,
    }
    } else
    pub GET_STATUS(sc->stats.phy.state): modem_state =,
    switch (modem_state) {
    case 0:
    pub booting\n"): ret = sysfs_emit(buf, "Modem is,
    case 1:
    pub initializing\n"): ret = sysfs_emit(buf, "Modem is,
    case 2:
    pub operational\n"): ret = sysfs_emit(buf, "Modem is,
    case 3:
    pub failed\n"): ret = sysfs_emit(buf, "Modem synchronization,
    default:
    pub unknown\n"): ret = sysfs_emit(buf, "Modem state is,
    }
    out:
    pub ret: return,
    }
    pub DEVICE_ATTR_RO(stat_human_status): static,
    static ssize_t stat_delin_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    pub -ENODEV: int ret =,
    pub sc: *mut uea_softc,
    pub "GOOD": *mut *mut char delin =,
    pub dev_to_uea(dev): sc =,
    if (!sc)
    pub out: goto,
    if (UEA_CHIP_VERSION(sc) == EAGLE_IV) {
    if (sc.stats.phy.flags & 0x4000)
    pub "RESET": delin =,
#[no_mangle]
pub unsafe extern "C" fn if(0x0001: sc->stats.phy.flags &) -> else {
    else if (sc.stats.phy.flags & 0x0001)
    pub "LOSS": delin =,
    } else {
    if (sc.stats.phy.flags & 0x0C00)
    pub "ERROR": delin =,
#[no_mangle]
pub unsafe extern "C" fn if(0x0030: sc->stats.phy.flags &) -> else {
    else if (sc.stats.phy.flags & 0x0030)
    pub "LOSS": delin =,
    }
    pub delin): ret = sysfs_emit(buf, "%s\n",,
    out:
    pub ret: return,
    }
    pub DEVICE_ATTR_RO(stat_delin): static,

    \
    static ssize_t stat_##name##_show(struct device *dev,		\
    struct device_attribute *attr, char *buf)	\
    {								\
    pub \: int ret = -ENODEV;,
    pub \: *mut *mut uea_softc sc;,
    \
    pub \: mutex_lock(&uea_mutex);,
    pub \: sc = dev_to_uea(dev);,
    if (!sc)						\
    pub \: goto out;,
    pub \: ret = sysfs_emit(buf, "%08x\n", sc->stats.phy.name);,
    if (reset)						\
    pub \: sc->stats.phy.name = 0;,
    out:								\
    pub \: mutex_unlock(&uea_mutex);,
    pub \: return ret;,
    }								\
    \
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: stat_##name) -> static {
    static DEVICE_ATTR_RO(stat_##name)
    pub 1): UEA_ATTR(mflags,,
    pub 0): UEA_ATTR(vidcpe,,
    pub 0): UEA_ATTR(usrate,,
    pub 0): UEA_ATTR(dsrate,,
    pub 0): UEA_ATTR(usattenuation,,
    pub 0): UEA_ATTR(dsattenuation,,
    pub 0): UEA_ATTR(usmargin,,
    pub 0): UEA_ATTR(dsmargin,,
    pub 0): UEA_ATTR(txflow,,
    pub 0): UEA_ATTR(rxflow,,
    pub 0): UEA_ATTR(uscorr,,
    pub 0): UEA_ATTR(dscorr,,
    pub 0): UEA_ATTR(usunc,,
    pub 0): UEA_ATTR(dsunc,,
    pub 0): UEA_ATTR(firmid,,
// Retrieve the device End System Identifier (MAC)
#[no_mangle]
unsafe extern "C" fn uea_getesi(sc: *mut uea_softc, esi: *mut u_char) -> c_int {
    static int uea_getesi(struct uea_softc *sc, u_char *esi)
    {
    pub 1]: *mut *mut unsigned char mac_str[2  ETH_ALEN +,
    pub i: c_int,
    if (usb_string
    (sc.usb_dev, sc.usb_dev.descriptor.iSerialNumber, mac_str,
    sizeof(mac_str)) != 2 * ETH_ALEN)
    pub 1: return,
    pub i++): for (i = 0; i < ETH_ALEN;,
    esi[i] = hex_to_bin(mac_str[2 * i]) * 16 +
    pub 1]): *mut *mut hex_to_bin(mac_str[2  i +,
    pub 0: return,
    }
// ATM stuff
#[no_mangle]
unsafe extern "C" fn uea_atm_open(usbatm: *mut usbatm_data, atm_dev: *mut atm_dev) -> c_int {
    static int uea_atm_open(struct usbatm_data *usbatm, struct atm_dev *atm_dev)
    {
    pub usbatm->driver_data: *mut *mut uea_softc sc =,
    pub atm_dev->esi): return uea_getesi(sc,,
    }
#[no_mangle]
unsafe extern "C" fn uea_heavy(usbatm: *mut usbatm_data, intf: *mut usb_interface) -> c_int {
    static int uea_heavy(struct usbatm_data *usbatm, struct usb_interface *intf)
    {
    pub usbatm->driver_data: *mut *mut uea_softc sc =,
    pub IS_OPERATIONAL(sc)): wait_event_interruptible(sc->sync_q,,
    pub 0: return,
    }
    static int claim_interface(struct usb_device *usb_dev,
    struct usbatm_data *usbatm, int ifnum)
    {
    pub ret: c_int,
    pub ifnum): *mut *mut usb_interface intf = usb_ifnum_to_if(usb_dev,,
    if (!intf) {
    pub ifnum): uea_err(usb_dev, "interface %d not found\n",,
    pub -ENODEV: return,
    }
    pub usbatm): ret = usb_driver_claim_interface(&uea_driver, intf,,
    if (ret != 0)
    uea_err(usb_dev, "can't claim interface %d, error %d\n", ifnum,
    pub ret: return,
    }
    static struct attribute *uea_attrs[] = {
    &dev_attr_stat_status.attr,
    &dev_attr_stat_mflags.attr,
    &dev_attr_stat_human_status.attr,
    &dev_attr_stat_delin.attr,
    &dev_attr_stat_vidcpe.attr,
    &dev_attr_stat_usrate.attr,
    &dev_attr_stat_dsrate.attr,
    &dev_attr_stat_usattenuation.attr,
    &dev_attr_stat_dsattenuation.attr,
    &dev_attr_stat_usmargin.attr,
    &dev_attr_stat_dsmargin.attr,
    &dev_attr_stat_txflow.attr,
    &dev_attr_stat_rxflow.attr,
    &dev_attr_stat_uscorr.attr,
    &dev_attr_stat_dscorr.attr,
    &dev_attr_stat_usunc.attr,
    &dev_attr_stat_dsunc.attr,
    &dev_attr_stat_firmid.attr,
    core::ptr::null_mut(),
}

    ATTRIBUTE_GROUPS(uea);
    static int uea_bind(struct usbatm_data *usbatm, struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *usb = interface_to_usbdev(intf);
    struct uea_softc *sc;
    int ret, ifnum = intf.altsetting.desc.bInterfaceNumber;
    unsigned int alt;
// interface 0 is for firmware/monitoring
    if (ifnum != UEA_INTR_IFACE_NO)
    return -ENODEV;
    usbatm.flags = (modem_index < NB_MODEM && sync_wait[modem_index]) ?
    0 : UDSL_SKIP_HEAVY_INIT;
// interface 1 is for outbound traffic
    ret = claim_interface(usb, usbatm, UEA_US_IFACE_NO);
    if (ret < 0)
    return ret;
// ADI930 has only 2 interfaces and inbound traffic is on interface 1
    if (UEA_CHIP_VERSION(id) != ADI930) {
// interface 2 is for inbound traffic
    ret = claim_interface(usb, usbatm, UEA_DS_IFACE_NO);
    if (ret < 0)
    return ret;
    }
    sc = kzalloc_obj(struct uea_softc);
    if (!sc)
    return -ENOMEM;
    sc.usb_dev = usb;
    usbatm.driver_data = sc;
    sc.usbatm = usbatm;
    sc.modem_index = (modem_index < NB_MODEM) ? modem_index++ : 0;
    sc.driver_info = id.driver_info;
// first try to use module parameter
    if (annex[sc.modem_index] == 1)
    sc.annex = ANNEXA;
#[no_mangle]
pub unsafe extern "C" fn if(2: annex[sc->modem_index] ==) -> else {
    else if (annex[sc.modem_index] == 2)
    sc.annex = ANNEXB;
// try to autodetect annex
#[no_mangle]
pub unsafe extern "C" fn if(AUTO_ANNEX_A: sc->driver_info &) -> else {
    else if (sc.driver_info & AUTO_ANNEX_A)
    sc.annex = ANNEXA;
#[no_mangle]
pub unsafe extern "C" fn if(AUTO_ANNEX_B: sc->driver_info &) -> else {
    else if (sc.driver_info & AUTO_ANNEX_B)
    sc.annex = ANNEXB;
    else
    sc.annex = (le16_to_cpu
    (sc.usb_dev.descriptor.bcdDevice) & 0x80) ? ANNEXB : ANNEXA;
    alt = altsetting[sc.modem_index];
// ADI930 don't support iso
    if (UEA_CHIP_VERSION(id) != ADI930 && alt > 0) {
    if (alt <= 8 &&
    usb_set_interface(usb, UEA_DS_IFACE_NO, alt) == 0) {
    uea_dbg(usb, "set alternate %u for 2 interface\n", alt);
    uea_info(usb, "using iso mode\n");
    usbatm.flags |= UDSL_USE_ISOC | UDSL_IGNORE_EILSEQ;
    } else {
    uea_err(usb, "setting alternate %u failed for "
    "2 interface, using bulk mode\n", alt);
    }
    }
    ret = uea_boot(sc, intf);
    if (ret < 0)
    goto error;
    return 0;
    error:
    kfree(sc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uea_unbind(usbatm: *mut usbatm_data, intf: *mut usb_interface) {
    static void uea_unbind(struct usbatm_data *usbatm, struct usb_interface *intf)
    {
    struct uea_softc *sc = usbatm.driver_data;
    uea_stop(sc);
    kfree(sc);
    }
    static struct usbatm_driver uea_usbatm_driver = {
    .driver_name = "ueagle-atm",
    .bind = uea_bind,
    .atm_start = uea_atm_open,
    .unbind = uea_unbind,
    .heavy_init = uea_heavy,
    .bulk_in = UEA_BULK_DATA_PIPE,
    .bulk_out = UEA_BULK_DATA_PIPE,
    .isoc_in = UEA_ISO_DATA_PIPE,
    };
#[no_mangle]
unsafe extern "C" fn uea_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int uea_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    struct usb_device *usb = interface_to_usbdev(intf);
    let mut single_iface: bool = usb.config.desc.bNumInterfaces == 1;
    int ret;
    uea_dbg(usb, "ADSL device found with vid (%#X) pid (%#X) Rev (%#X): %s\n",
    le16_to_cpu(usb.descriptor.idVendor),
    le16_to_cpu(usb.descriptor.idProduct),
    le16_to_cpu(usb.descriptor.bcdDevice),
    chip_name[UEA_CHIP_VERSION(id)]);
//
// uea_probe() decides between the pre-firmware and post-firmware case
// from the USB id and stores a different object as interface data in
// each case: a struct completion for a pre-firmware device, a struct
// usbatm_data for a post-firmware one. uea_disconnect() instead tells
// the two apart by the number of interfaces (a pre-firmware device
// exposes a single interface, ADI930 has 2 and eagle has 3). A crafted
// device advertising a pre-firmware id together with a multi-interface
// descriptor (or the other way around) makes the two disagree, so that
// usbatm_usb_disconnect() treats the small completion object as a
// struct usbatm_data and reads out of bounds. Reject such inconsistent
// descriptors so both paths make the same decision.
//
    if (UEA_IS_PREFIRM(id) != single_iface)
    return -ENODEV;
    usb_reset_device(usb);
    if (UEA_IS_PREFIRM(id)) {
    struct completion *fw_done;
// Wait for the firmware load to be done, in .disconnect()
    fw_done = kzalloc_obj(*fw_done);
    if (!fw_done)
    return -ENOMEM;
    init_completion(fw_done);
    usb_set_intfdata(intf, fw_done);
    ret = uea_load_firmware(intf, UEA_CHIP_VERSION(id));
    if (ret)
    kfree(fw_done);
    return ret;
    }
    ret = usbatm_usb_probe(intf, id, &uea_usbatm_driver);
    if (ret == 0) {
    struct usbatm_data *usbatm = usb_get_intfdata(intf);
    struct uea_softc *sc = usbatm.driver_data;
// Ensure carrier is initialized to off as early as possible
    UPDATE_ATM_SIGNAL(ATM_PHY_SIG_LOST);
// Only start the worker thread when all init is done
    wake_up_process(sc.kthread);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uea_disconnect(intf: *mut usb_interface) {
    static void uea_disconnect(struct usb_interface *intf)
    {
    struct usb_device *usb = interface_to_usbdev(intf);
    let mut ifnum: c_int = intf.altsetting.desc.bInterfaceNumber;
// ADI930 has 2 interfaces and eagle 3 interfaces.
// Pre-firmware device has one interface
//
    if (usb.config.desc.bNumInterfaces != 1 && ifnum == 0) {
    mutex_lock(&uea_mutex);
    usbatm_usb_disconnect(intf);
    mutex_unlock(&uea_mutex);
    uea_info(usb, "ADSL device removed\n");
    } else if (usb.config.desc.bNumInterfaces == 1) {
    struct completion *fw_done = usb_get_intfdata(intf);
    uea_dbg(usb, "pre-firmware device, waiting firmware upload\n");
    wait_for_completion(fw_done);
    uea_dbg(usb, "pre-firmware device, finished waiting\n");
    kfree(fw_done);
    }
    }
//
// List of supported VID/PID
//
    static const struct usb_device_id uea_ids[] = {
    {USB_DEVICE(ANALOG_VID,	ADI930_PID_PREFIRM),
    .driver_info = ADI930 | PREFIRM},
    {USB_DEVICE(ANALOG_VID,	ADI930_PID_PSTFIRM),
    .driver_info = ADI930 | PSTFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_I_PID_PREFIRM),
    .driver_info = EAGLE_I | PREFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_I_PID_PSTFIRM),
    .driver_info = EAGLE_I | PSTFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_II_PID_PREFIRM),
    .driver_info = EAGLE_II | PREFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_II_PID_PSTFIRM),
    .driver_info = EAGLE_II | PSTFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_IIC_PID_PREFIRM),
    .driver_info = EAGLE_II | PREFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_IIC_PID_PSTFIRM),
    .driver_info = EAGLE_II | PSTFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_III_PID_PREFIRM),
    .driver_info = EAGLE_III | PREFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_III_PID_PSTFIRM),
    .driver_info = EAGLE_III | PSTFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_IV_PID_PREFIRM),
    .driver_info = EAGLE_IV | PREFIRM},
    {USB_DEVICE(ANALOG_VID,	EAGLE_IV_PID_PSTFIRM),
    .driver_info = EAGLE_IV | PSTFIRM},
    {USB_DEVICE(DEVOLO_VID,	DEVOLO_EAGLE_I_A_PID_PREFIRM),
    .driver_info = EAGLE_I | PREFIRM},
    {USB_DEVICE(DEVOLO_VID,	DEVOLO_EAGLE_I_A_PID_PSTFIRM),
    .driver_info = EAGLE_I | PSTFIRM | AUTO_ANNEX_A},
    {USB_DEVICE(DEVOLO_VID,	DEVOLO_EAGLE_I_B_PID_PREFIRM),
    .driver_info = EAGLE_I | PREFIRM},
    {USB_DEVICE(DEVOLO_VID,	DEVOLO_EAGLE_I_B_PID_PSTFIRM),
    .driver_info = EAGLE_I | PSTFIRM | AUTO_ANNEX_B},
    {USB_DEVICE(DEVOLO_VID,	DEVOLO_EAGLE_II_A_PID_PREFIRM),
    .driver_info = EAGLE_II | PREFIRM},
    {USB_DEVICE(DEVOLO_VID,	DEVOLO_EAGLE_II_A_PID_PSTFIRM),
    .driver_info = EAGLE_II | PSTFIRM | AUTO_ANNEX_A},
    {USB_DEVICE(DEVOLO_VID,	DEVOLO_EAGLE_II_B_PID_PREFIRM),
    .driver_info = EAGLE_II | PREFIRM},
    {USB_DEVICE(DEVOLO_VID,	DEVOLO_EAGLE_II_B_PID_PSTFIRM),
    .driver_info = EAGLE_II | PSTFIRM | AUTO_ANNEX_B},
    {USB_DEVICE(ELSA_VID,	ELSA_PID_PREFIRM),
    .driver_info = ADI930 | PREFIRM},
    {USB_DEVICE(ELSA_VID,	ELSA_PID_PSTFIRM),
    .driver_info = ADI930 | PSTFIRM},
    {USB_DEVICE(ELSA_VID,	ELSA_PID_A_PREFIRM),
    .driver_info = ADI930 | PREFIRM},
    {USB_DEVICE(ELSA_VID,	ELSA_PID_A_PSTFIRM),
    .driver_info = ADI930 | PSTFIRM | AUTO_ANNEX_A},
    {USB_DEVICE(ELSA_VID,	ELSA_PID_B_PREFIRM),
    .driver_info = ADI930 | PREFIRM},
    {USB_DEVICE(ELSA_VID,	ELSA_PID_B_PSTFIRM),
    .driver_info = ADI930 | PSTFIRM | AUTO_ANNEX_B},
    {USB_DEVICE(USR_VID,	MILLER_A_PID_PREFIRM),
    .driver_info = EAGLE_I | PREFIRM},
    {USB_DEVICE(USR_VID,	MILLER_A_PID_PSTFIRM),
    .driver_info = EAGLE_I | PSTFIRM  | AUTO_ANNEX_A},
    {USB_DEVICE(USR_VID,	MILLER_B_PID_PREFIRM),
    .driver_info = EAGLE_I | PREFIRM},
    {USB_DEVICE(USR_VID,	MILLER_B_PID_PSTFIRM),
    .driver_info = EAGLE_I | PSTFIRM  | AUTO_ANNEX_B},
    {USB_DEVICE(USR_VID,	HEINEKEN_A_PID_PREFIRM),
    .driver_info = EAGLE_I | PREFIRM},
    {USB_DEVICE(USR_VID,	HEINEKEN_A_PID_PSTFIRM),
    .driver_info = EAGLE_I | PSTFIRM | AUTO_ANNEX_A},
    {USB_DEVICE(USR_VID,	HEINEKEN_B_PID_PREFIRM),
    .driver_info = EAGLE_I | PREFIRM},
    {USB_DEVICE(USR_VID,	HEINEKEN_B_PID_PSTFIRM),
    .driver_info = EAGLE_I | PSTFIRM | AUTO_ANNEX_B},
    {}
    };
//
// USB driver descriptor
//
    static struct usb_driver uea_driver = {
    .name = "ueagle-atm",
    .id_table = uea_ids,
    .probe = uea_probe,
    .disconnect = uea_disconnect,
    .dev_groups = uea_groups,
    };
    MODULE_DEVICE_TABLE(usb, uea_ids);
    module_usb_driver(uea_driver);
    MODULE_AUTHOR("Damien Bergamini/Matthieu Castet/Stanislaw W. Gruszka");
    MODULE_DESCRIPTION("ADI 930/Eagle USB ADSL Modem driver");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_FIRMWARE(EAGLE_FIRMWARE);
    MODULE_FIRMWARE(ADI930_FIRMWARE);
    MODULE_FIRMWARE(EAGLE_I_FIRMWARE);
    MODULE_FIRMWARE(EAGLE_II_FIRMWARE);
    MODULE_FIRMWARE(EAGLE_III_FIRMWARE);
    MODULE_FIRMWARE(EAGLE_IV_FIRMWARE);
    MODULE_FIRMWARE(DSP4I_FIRMWARE);
    MODULE_FIRMWARE(DSP4P_FIRMWARE);
    MODULE_FIRMWARE(DSP9I_FIRMWARE);
    MODULE_FIRMWARE(DSP9P_FIRMWARE);
    MODULE_FIRMWARE(DSPEI_FIRMWARE);
    MODULE_FIRMWARE(DSPEP_FIRMWARE);
    MODULE_FIRMWARE(FPGA930_FIRMWARE);
    MODULE_FIRMWARE(CMV4P_FIRMWARE);
    MODULE_FIRMWARE(CMV4PV2_FIRMWARE);
    MODULE_FIRMWARE(CMV4I_FIRMWARE);
    MODULE_FIRMWARE(CMV4IV2_FIRMWARE);
    MODULE_FIRMWARE(CMV9P_FIRMWARE);
    MODULE_FIRMWARE(CMV9PV2_FIRMWARE);
    MODULE_FIRMWARE(CMV9I_FIRMWARE);
    MODULE_FIRMWARE(CMV9IV2_FIRMWARE);
    MODULE_FIRMWARE(CMVEP_FIRMWARE);
    MODULE_FIRMWARE(CMVEPV2_FIRMWARE);
    MODULE_FIRMWARE(CMVEI_FIRMWARE);
    MODULE_FIRMWARE(CMVEIV2_FIRMWARE);
