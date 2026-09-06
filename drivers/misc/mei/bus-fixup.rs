//! Automatically rewritten from C to Rust
//! Source: drivers/misc/mei/bus-fixup.c
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
// Copyright (c) 2013-2023, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
//

    0x48, 0xa4, 0xef, 0xab, 0xba, 0x8a, 0x12, 0x06)
    let mut mei_nfc_info_guid: static uuid_le = MEI_UUID_NFC_INFO;

    0x94, 0xd4, 0x50, 0x26, 0x67, 0x23, 0x77, 0x5c)

    0x89, 0x9D, 0xA9, 0x15, 0x14, 0xCB, 0x32, 0xAB)

    0xba, 0xdf, 0xf, 0xb7, 0xed, 0x68, 0x2a, 0xeb)

    0x9D, 0x95, 0x06, 0xB1, 0x6B, 0x58, 0x8A, 0x5D)

    0x91, 0x70, 0xB7, 0xF4, 0x6D, 0x57, 0xB4, 0xAD)

    0xA5, 0x52, 0xD1, 0xC5, 0x4B, 0x62, 0x7F, 0x04)

    0xa6, 0x1b, 0xab, 0x8c, 0xbe, 0x36, 0xb1)

//
// number_of_connections - determine whether an client be on the bus
// according number of connections
// We support only clients:
// 1. with single connection
// 2. and fixed clients (max_number_of_connections == 0)
//
// @cldev: me clients device
//
#[no_mangle]
unsafe extern "C" fn number_of_connections(cldev: *mut mei_cl_device) {
    static void number_of_connections(struct mei_cl_device *cldev)
    {
    if (cldev.me_cl.props.max_number_of_connections > 1)
    cldev.do_match = 0;
    }
//
// blacklist - blacklist a client from the bus
//
// @cldev: me clients device
//
#[no_mangle]
unsafe extern "C" fn blacklist(cldev: *mut mei_cl_device) {
    static void blacklist(struct mei_cl_device *cldev)
    {
    cldev.do_match = 0;
    }
//
// whitelist - forcefully whitelist client
//
// @cldev: me clients device
//
#[no_mangle]
unsafe extern "C" fn whitelist(cldev: *mut mei_cl_device) {
    static void whitelist(struct mei_cl_device *cldev)
    {
    cldev.do_match = 1;
    }
pub const MKHI_SEND_MAX_TIMEOUT_MSEC: c_int = 4000;
pub const OSTYPE_LINUX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_os_ver {
    pub build: __le16,
    pub reserved1: __le16,
    pub os_type: u8,
    pub major: u8,
    pub minor: u8,
    pub reserved2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mkhi_fw_ver_block {
    pub minor: u16,
    pub major: u8,
    pub platform: u8,
    pub buildno: u16,
    pub hotfix: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mkhi_fw_ver {
    pub ver: [mkhi_fw_ver_block; MEI_MAX_FW_VER_BLOCKS],
    pub __packed: },

    sizeof(struct mkhi_fwcaps) + \
    sizeof(struct mei_os_ver))
#[no_mangle]
unsafe extern "C" fn mei_osver(cldev: *mut mei_cl_device) -> c_int {
    static int mei_osver(struct mei_cl_device *cldev)
    {
    pub MKHI_OSVER_BUF_LEN: size_t size =,
    pub buf: [u8; MKHI_OSVER_BUF_LEN],
    pub req: *mut mkhi_msg,
    pub fwcaps: *mut mkhi_fwcaps,
    pub os_ver: *mut mei_os_ver,
    pub MEI_CL_IO_TX_INTERNAL: unsigned int mode = MEI_CL_IO_TX_BLOCKING |,
    pub size): memset(buf, 0,,
    pub )buf: *mut req = (struct mkhi_msg,
    pub MKHI_FWCAPS_GROUP_ID: req->hdr.group_id =,
    pub MKHI_FWCAPS_SET_OS_VER_APP_RULE_CMD: req->hdr.command =,
    pub )req->data: *mut fwcaps = (struct mkhi_fwcaps,
    pub 0x0: fwcaps->id.rule_type =,
    pub MKHI_FEATURE_PTT: fwcaps->id.feature_id =,
    pub sizeof(*os_ver): *mut fwcaps->len =,
    pub )fwcaps->data: *mut os_ver = (struct mei_os_ver,
    pub OSTYPE_LINUX: os_ver->os_type =,
    pub MKHI_SEND_MAX_TIMEOUT_MSEC): return __mei_cl_send_timeout(cldev->cl, buf, size, 0, mode,,
    }

    sizeof(struct mkhi_fw_ver))

    sizeof(struct mkhi_fw_ver_block) * (__num))
#[no_mangle]
unsafe extern "C" fn mei_fwver(cldev: *mut mei_cl_device) -> c_int {
    static int mei_fwver(struct mei_cl_device *cldev)
    {
    pub buf: [u8; MKHI_FWVER_BUF_LEN],
    pub req: mkhi_msg,
    pub rsp: *mut mkhi_msg,
    pub fwver: *mut mkhi_fw_ver,
    pub i: int bytes_recv, ret,,
    pub sizeof(buf)): memset(buf, 0,,
    pub MKHI_GEN_GROUP_ID: req.hdr.group_id =,
    pub MKHI_GEN_GET_FW_VERSION_CMD: req.hdr.command =,
    ret = __mei_cl_send_timeout(cldev.cl, (u8 *)&req, sizeof(req), 0,
    pub MKHI_SEND_MAX_TIMEOUT_MSEC): MEI_CL_IO_TX_BLOCKING,,
    if (ret < 0) {
    pub ret): dev_info(&cldev->dev, "Could not send ReqFWVersion cmd ret = %d\n",,
    pub ret: return,
    }
    pub 0: ret =,
    bytes_recv = __mei_cl_recv(cldev.cl, buf, sizeof(buf), core::ptr::null_mut(), 0,
    if (bytes_recv < 0 || (size_t)bytes_recv < MKHI_FWVER_LEN(1)) {
//
// Should be at least one version block,
// error out if nothing found
//
    pub bytes_recv): dev_info(&cldev->dev, "Could not read FW version ret = %d\n",,
    pub -EIO: return,
    }
    pub )buf: *mut rsp = (struct mkhi_msg,
    pub )rsp->data: *mut fwver = (struct mkhi_fw_ver,
    pub sizeof(cldev->bus->fw_ver)): memset(cldev->bus->fw_ver, 0,,
    pub {: for (i = 0; i < MEI_MAX_FW_VER_BLOCKS; i++),
    if ((size_t)bytes_recv < MKHI_FWVER_LEN(i + 1))
    dev_dbg(&cldev.dev, "FW version%d %d:%d.%d.%d.%d\n",
    i, fwver.ver[i].platform,
    fwver.ver[i].major, fwver.ver[i].minor,
    pub fwver->ver[i].buildno): fwver->ver[i].hotfix,,
    pub fwver->ver[i].platform: cldev->bus->fw_ver[i].platform =,
    pub fwver->ver[i].major: cldev->bus->fw_ver[i].major =,
    pub fwver->ver[i].minor: cldev->bus->fw_ver[i].minor =,
    pub fwver->ver[i].hotfix: cldev->bus->fw_ver[i].hotfix =,
    pub fwver->ver[i].buildno: cldev->bus->fw_ver[i].buildno =,
    }
    pub 1: cldev->bus->fw_ver_received =,
    pub ret: return,
    }

#[no_mangle]
unsafe extern "C" fn mei_gfx_memory_ready(cldev: *mut mei_cl_device) -> c_int {
    static int mei_gfx_memory_ready(struct mei_cl_device *cldev)
    {
    pub {0}: mkhi_gfx_mem_ready req =,
    pub MEI_CL_IO_TX_BLOCKING: unsigned int mode = MEI_CL_IO_TX_INTERNAL |,
    pub MKHI_GROUP_ID_GFX: req.hdr.group_id =,
    pub MKHI_GFX_MEMORY_READY_CMD_REQ: req.hdr.command =,
    pub MKHI_GFX_MEM_READY_PXP_ALLOWED: req.flags =,
    pub command\n"): dev_dbg(&cldev->dev, "Sending memory ready,
    return __mei_cl_send_timeout(cldev.cl, (u8 *)&req, sizeof(req), 0,
    pub GFX_MEMORY_READY_TIMEOUT): mode,,
    }
#[no_mangle]
unsafe extern "C" fn mei_mkhi_fix(cldev: *mut mei_cl_device) {
    static void mei_mkhi_fix(struct mei_cl_device *cldev)
    {
    pub ret: c_int,
// No need to enable the client if nothing is needed from it
    if (!cldev.bus.fw_f_fw_ver_supported &&
    !cldev.bus.hbm_f_os_supported)
    pub mei_cldev_enable(cldev): ret =,
    if (ret)
    if (cldev.bus.fw_f_fw_ver_supported) {
    pub mei_fwver(cldev): ret =,
    if (ret < 0)
    dev_info(&cldev.dev, "FW version command failed %d\n",
    }
    if (cldev.bus.hbm_f_os_supported) {
    pub mei_osver(cldev): ret =,
    if (ret < 0)
    dev_info(&cldev.dev, "OS version command failed %d\n",
    }
    }
#[no_mangle]
unsafe extern "C" fn mei_gsc_mkhi_ver(cldev: *mut mei_cl_device) {
    static void mei_gsc_mkhi_ver(struct mei_cl_device *cldev)
    {
    pub ret: c_int,
//
// No need to enable the client if nothing is needed from it.
// No need to fill in version if it is already filled in by the fix address client.
//
    if (!cldev.bus.fw_f_fw_ver_supported || cldev.bus.fw_ver_received)
    pub mei_cldev_enable(cldev): ret =,
    if (ret)
    pub mei_fwver(cldev): ret =,
    if (ret < 0)
    pub ret): dev_info(&cldev->dev, "FW version command failed %d\n",,
    }
#[no_mangle]
unsafe extern "C" fn mei_gsc_mkhi_fix_ver(cldev: *mut mei_cl_device) {
    static void mei_gsc_mkhi_fix_ver(struct mei_cl_device *cldev)
    {
    pub ret: c_int,
// No need to enable the client if nothing is needed from it
    if (!cldev.bus.fw_f_fw_ver_supported &&
    cldev.bus.pxp_mode != MEI_DEV_PXP_INIT)
    pub mei_cldev_enable(cldev): ret =,
    if (ret)
    if (cldev.bus.pxp_mode == MEI_DEV_PXP_INIT) {
    pub mei_gfx_memory_ready(cldev): ret =,
    if (ret < 0) {
    pub ret): dev_err(&cldev->dev, "memory ready command failed %d\n",,
    } else {
    pub sent\n"): dev_dbg(&cldev->dev, "memory ready command,
    pub MEI_DEV_PXP_SETUP: cldev->bus->pxp_mode =,
    }
// we go to reset after that
    pub out: goto,
    }
    pub mei_fwver(cldev): ret =,
    if (ret < 0)
    dev_info(&cldev.dev, "FW version command failed %d\n",
    out:
    }
//
// mei_wd - wd client on the bus, change protocol version
// as the API has changed.
//
// @cldev: me clients device
//

#[no_mangle]
unsafe extern "C" fn mei_wd(cldev: *mut mei_cl_device) {
    static void mei_wd(struct mei_cl_device *cldev)
    {
    pub to_pci_dev(cldev->dev.parent): *mut *mut pci_dev pdev =,
    if (pdev.device == PCI_DEVICE_ID_INTEL_MEI_WPT_LP ||
    pdev.device == PCI_DEVICE_ID_INTEL_MEI_SPT ||
    pdev.device == PCI_DEVICE_ID_INTEL_MEI_SPT_H)
    pub 0x2: cldev->me_cl->props.protocol_version =,
    pub 1: cldev->do_match =,
    }

    static inline void mei_wd(struct mei_cl_device *cldev) {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_nfc_cmd {
    pub command: u8,
    pub status: u8,
    pub req_id: u16,
    pub reserved: u32,
    pub data_size: u16,
    pub sub_command: u8,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_nfc_reply {
    pub command: u8,
    pub status: u8,
    pub req_id: u16,
    pub reserved: u32,
    pub data_size: u16,
    pub sub_command: u8,
    pub reply_status: u8,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_nfc_if_version {
    pub radio_version_sw: [u8; 3],
    pub reserved: [u8; 3],
    pub radio_version_hw: [u8; 3],
    pub i2c_addr: u8,
    pub fw_ivn: u8,
    pub vendor_id: u8,
    pub radio_type: u8,
    pub __packed: },
pub const MEI_NFC_CMD_MAINTENANCE: c_uint = 0x00;
pub const MEI_NFC_SUBCMD_IF_VERSION: c_uint = 0x01;
// Vendors
pub const MEI_NFC_VENDOR_INSIDE: c_uint = 0x00;
pub const MEI_NFC_VENDOR_NXP: c_uint = 0x01;
// Radio types
pub const MEI_NFC_VENDOR_INSIDE_UREAD: c_uint = 0x00;
pub const MEI_NFC_VENDOR_NXP_PN544: c_uint = 0x01;
//
// mei_nfc_if_version - get NFC interface version
//
// @cl: host client (nfc info)
// @ver: NFC interface version to be filled in
//
// Return: 0 on success; < 0 otherwise
//
    static int mei_nfc_if_version(struct mei_cl *cl,
    struct mei_nfc_if_version *ver)
    {
    pub bus: *mut mei_device,
    struct mei_nfc_cmd cmd = {
    .command = MEI_NFC_CMD_MAINTENANCE,
    .data_size = 1,
    .sub_command = MEI_NFC_SUBCMD_IF_VERSION,
}

    struct mei_nfc_reply *reply = core::ptr::null_mut();
    size_t if_version_length;
    u8 vtag;
    int bytes_recv, ret;
    bus = cl.dev;
    WARN_ON(mutex_is_locked(&bus.device_lock));
    ret = __mei_cl_send(cl, (u8 *)&cmd, sizeof(cmd), 0,
    MEI_CL_IO_TX_BLOCKING);
    if (ret < 0) {
    dev_err(&bus.dev, "Could not send IF version cmd ret = %d\n", ret);
    return ret;
    }
// to be sure on the stack we alloc memory
    if_version_length = sizeof(*reply) + sizeof(*ver);
    reply = kzalloc(if_version_length, GFP_KERNEL);
    if (!reply)
    return -ENOMEM;
    ret = 0;
    bytes_recv = __mei_cl_recv(cl, (u8 *)reply, if_version_length, &vtag,
    0, 0);
    if (bytes_recv < 0 || (size_t)bytes_recv < if_version_length) {
    dev_err(&bus.dev, "Could not read IF version ret = %d\n", bytes_recv);
    ret = -EIO;
    goto err;
    }
    memcpy(ver, reply.data, sizeof(*ver));
    dev_info(&bus.dev, "NFC MEI VERSION: IVN 0x%x Vendor ID 0x%x Type 0x%x\n",
    ver.fw_ivn, ver.vendor_id, ver.radio_type);
    err:
    kfree(reply);
    return ret;
    }
//
// mei_nfc_radio_name - derive nfc radio name from the interface version
//
// @ver: NFC radio version
//
// Return: radio name string
//
    static const char *mei_nfc_radio_name(struct mei_nfc_if_version *ver)
    {
    if (ver.vendor_id == MEI_NFC_VENDOR_INSIDE) {
    if (ver.radio_type == MEI_NFC_VENDOR_INSIDE_UREAD)
    return "microread";
    }
    if (ver.vendor_id == MEI_NFC_VENDOR_NXP) {
    if (ver.radio_type == MEI_NFC_VENDOR_NXP_PN544)
    return "pn544";
    }
    return core::ptr::null_mut();
    }
//
// mei_nfc - The nfc fixup function. The function retrieves nfc radio
// name and set is as device attribute so we can load
// the proper device driver for it
//
// @cldev: me client device (nfc)
//
#[no_mangle]
unsafe extern "C" fn mei_nfc(cldev: *mut mei_cl_device) {
    static void mei_nfc(struct mei_cl_device *cldev)
    {
    struct mei_device *bus;
    struct mei_cl *cl;
    struct mei_me_client *me_cl = core::ptr::null_mut();
    struct mei_nfc_if_version ver;
    const char *radio_name = core::ptr::null_mut();
    int ret;
    bus = cldev.bus;
    mutex_lock(&bus.device_lock);
// we need to connect to INFO GUID
    cl = mei_cl_alloc_linked(bus);
    if (IS_ERR(cl)) {
    ret = PTR_ERR(cl);
    cl = core::ptr::null_mut();
    dev_err(&cldev.dev, "nfc hook alloc failed %d\n", ret);
    goto out;
    }
    me_cl = mei_me_cl_by_uuid(bus, &mei_nfc_info_guid);
    if (!me_cl) {
    ret = -ENOTTY;
    dev_err(&cldev.dev, "Cannot find nfc info %d\n", ret);
    goto out;
    }
    ret = mei_cl_connect(cl, me_cl, core::ptr::null_mut());
    if (ret < 0) {
    dev_err(&cldev.dev, "Can't connect to the NFC INFO ME ret = %d\n",
    ret);
    goto out;
    }
    mutex_unlock(&bus.device_lock);
    ret = mei_nfc_if_version(cl, &ver);
    if (ret)
    goto disconnect;
    radio_name = mei_nfc_radio_name(&ver);
    if (!radio_name) {
    ret = -ENOENT;
    dev_err(&cldev.dev, "Can't get the NFC interface version ret = %d\n",
    ret);
    goto disconnect;
    }
    dev_dbg(&cldev.dev, "nfc radio %s\n", radio_name);
    strscpy(cldev.name, radio_name, sizeof(cldev.name));
    disconnect:
    mutex_lock(&bus.device_lock);
    if (mei_cl_disconnect(cl) < 0)
    dev_err(&cldev.dev, "Can't disconnect the NFC INFO ME\n");
    mei_cl_flush_queues(cl, core::ptr::null_mut());
    out:
    mei_cl_unlink(cl);
    mutex_unlock(&bus.device_lock);
    mei_me_cl_put(me_cl);
    kfree(cl);
    if (ret)
    cldev.do_match = 0;
    dev_dbg(&cldev.dev, "end of fixup match = %d\n", cldev.do_match);
    }
//
// vt_support - enable on bus clients with vtag support
//
// @cldev: me clients device
//
#[no_mangle]
unsafe extern "C" fn vt_support(cldev: *mut mei_cl_device) {
    static void vt_support(struct mei_cl_device *cldev)
    {
    if (cldev.me_cl.props.vt_supported == 1)
    cldev.do_match = 1;
    }
//
// pxp_is_ready - enable bus client if pxp is ready
//
// @cldev: me clients device
//
#[no_mangle]
unsafe extern "C" fn pxp_is_ready(cldev: *mut mei_cl_device) {
    static void pxp_is_ready(struct mei_cl_device *cldev)
    {
    struct mei_device *bus = cldev.bus;
    switch (bus.pxp_mode) {
    case MEI_DEV_PXP_READY:
    case MEI_DEV_PXP_DEFAULT:
    cldev.do_match = 1;
    break;
    default:
    cldev.do_match = 0;
    break;
    }
    }

    static struct mei_fixup {
    const uuid_le uuid;
    void (*hook)(struct mei_cl_device *cldev);
    } mei_fixups[] = {
    MEI_FIXUP(MEI_UUID_ANY, number_of_connections),
    MEI_FIXUP(MEI_UUID_NFC_INFO, blacklist),
    MEI_FIXUP(MEI_UUID_NFC_HCI, mei_nfc),
    MEI_FIXUP(MEI_UUID_WD, mei_wd),
    MEI_FIXUP(MEI_UUID_MKHIF_FIX, mei_mkhi_fix),
    MEI_FIXUP(MEI_UUID_IGSC_MKHI_FIX, mei_gsc_mkhi_fix_ver),
    MEI_FIXUP(MEI_UUID_IGSC_MKHI, mei_gsc_mkhi_ver),
    MEI_FIXUP(MEI_UUID_HDCP, whitelist),
    MEI_FIXUP(MEI_UUID_ANY, vt_support),
    MEI_FIXUP(MEI_UUID_PAVP, pxp_is_ready),
    };
//
// mei_cl_bus_dev_fixup - run fixup handlers
//
// @cldev: me client device
//
#[no_mangle]
pub unsafe extern "C" fn mei_cl_bus_dev_fixup(cldev: *mut mei_cl_device) {
    void mei_cl_bus_dev_fixup(struct mei_cl_device *cldev)
    {
    struct mei_fixup *f;
    const uuid_le *uuid = mei_me_cl_uuid(cldev.me_cl);
    size_t i;
    for (i = 0; i < ARRAY_SIZE(mei_fixups); i++) {
    f = &mei_fixups[i];
    if (uuid_le_cmp(f.uuid, MEI_UUID_ANY) == 0 ||
    uuid_le_cmp(f.uuid, *uuid) == 0)
    f.hook(cldev);
    }
    }
