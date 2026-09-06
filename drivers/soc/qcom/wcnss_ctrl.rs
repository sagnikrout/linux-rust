//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/wcnss_ctrl.c
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
// Copyright (c) 2016, Linaro Ltd.
// Copyright (c) 2015, Sony Mobile Communications Inc.
//

pub const WCNSS_ACK_DONE_BOOTING: c_int = 1;
pub const WCNSS_ACK_COLD_BOOTING: c_int = 2;
pub const NV_FRAGMENT_SIZE: c_int = 3072;

//
// struct wcnss_ctrl - driver context
// @dev:	device handle
// @channel:	SMD channel handle
// @ack:	completion for outstanding requests
// @cbc:	completion for cbc complete indication
// @ack_status:	status of the outstanding request
// @probe_work: worker for uploading nv binary
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcnss_ctrl {
    pub dev: *mut device,
    pub channel: *mut rpmsg_endpoint,
    pub ack: completion,
    pub cbc: completion,
    pub ack_status: c_int,
    pub probe_work: work_struct,
}

// message types
    enum {
    WCNSS_VERSION_REQ = 0x01000000,
    WCNSS_VERSION_RESP,
    WCNSS_DOWNLOAD_NV_REQ,
    WCNSS_DOWNLOAD_NV_RESP,
    WCNSS_UPLOAD_CAL_REQ,
    WCNSS_UPLOAD_CAL_RESP,
    WCNSS_DOWNLOAD_CAL_REQ,
    WCNSS_DOWNLOAD_CAL_RESP,
    WCNSS_VBAT_LEVEL_IND,
    WCNSS_BUILD_VERSION_REQ,
    WCNSS_BUILD_VERSION_RESP,
    WCNSS_PM_CONFIG_REQ,
    WCNSS_CBC_COMPLETE_IND,
    };
//
// struct wcnss_msg_hdr - common packet header for requests and responses
// @type:	packet message type
// @len:	total length of the packet, including this header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcnss_msg_hdr {
    pub type: u32,
    pub len: u32,
    pub __packed: },
//
// struct wcnss_version_resp - version request response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcnss_version_resp {
    pub hdr: wcnss_msg_hdr,
    pub major: u8,
    pub minor: u8,
    pub version: u8,
    pub revision: u8,
    pub __packed: },
//
// struct wcnss_download_nv_req - firmware fragment request
// @hdr:	common packet wcnss_msg_hdr header
// @seq:	sequence number of this fragment
// @last:	boolean indicator of this being the last fragment of the binary
// @frag_size:	length of this fragment
// @fragment:	fragment data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcnss_download_nv_req {
    pub hdr: wcnss_msg_hdr,
    pub seq: u16,
    pub last: u16,
    pub frag_size: u32,
    pub __counted_by(frag_size): u8 fragment[],
    pub __packed: },
//
// struct wcnss_download_nv_resp - firmware download response
// @hdr:	common packet wcnss_msg_hdr header
// @status:	boolean to indicate success of the download
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcnss_download_nv_resp {
    pub hdr: wcnss_msg_hdr,
    pub status: u8,
    pub __packed: },
//
// wcnss_ctrl_smd_callback() - handler from SMD responses
// @rpdev:	remote processor message device pointer
// @data:	pointer to the incoming data packet
// @count:	size of the incoming data packet
// @priv:	unused
// @addr:	unused
//
// Handles any incoming packets from the remote WCNSS_CTRL service.
//
    static int wcnss_ctrl_smd_callback(struct rpmsg_device *rpdev,
    void *data,
    int count,
    void *priv,
    u32 addr)
    {
    pub dev_get_drvdata(&rpdev->dev): *mut *mut wcnss_ctrl wcnss =,
    pub nvresp: *const wcnss_download_nv_resp,
    pub version: *const wcnss_version_resp,
    pub data: *const *const wcnss_msg_hdr hdr =,
    switch (hdr.type) {
    case WCNSS_VERSION_RESP:
    if (count != sizeof(*version)) {
    dev_err(wcnss.dev,
    pub response\n"): "invalid size of version,
    }
    pub data: version =,
    dev_info(wcnss.dev, "WCNSS Version %d.%d %d.%d\n",
    version.major, version.minor,
    pub version->revision): version->version,,
    case WCNSS_DOWNLOAD_NV_RESP:
    if (count != sizeof(*nvresp)) {
    dev_err(wcnss.dev,
    pub response\n"): "invalid size of download,
    }
    pub data: nvresp =,
    pub nvresp->status: wcnss->ack_status =,
    case WCNSS_CBC_COMPLETE_IND:
    pub complete\n"): dev_dbg(wcnss->dev, "cold boot,
    default:
    pub hdr->type): dev_info(wcnss->dev, "unknown message type %d\n",,
    }
    pub 0: return,
    }
//
// wcnss_request_version() - send a version request to WCNSS
// @wcnss:	wcnss ctrl driver context
//
#[no_mangle]
unsafe extern "C" fn wcnss_request_version(wcnss: *mut wcnss_ctrl) -> c_int {
    static int wcnss_request_version(struct wcnss_ctrl *wcnss)
    {
    pub msg: wcnss_msg_hdr,
    pub ret: c_int,
    pub WCNSS_VERSION_REQ: msg.type =,
    pub sizeof(msg): msg.len =,
    pub sizeof(msg)): ret = rpmsg_send(wcnss->channel, &msg,,
    if (ret < 0)
    pub ret: return,
    pub WCNSS_CBC_TIMEOUT): ret = wait_for_completion_timeout(&wcnss->ack,,
    if (!ret) {
    pub response\n"): dev_err(wcnss->dev, "timeout waiting for version,
    pub -ETIMEDOUT: return,
    }
    pub 0: return,
    }
//
// wcnss_download_nv() - send nv binary to WCNSS
// @wcnss:	wcnss_ctrl state handle
// @expect_cbc:	indicator to caller that an cbc event is expected
//
// Returns 0 on success. Negative errno on failure.
//
#[no_mangle]
unsafe extern "C" fn wcnss_download_nv(wcnss: *mut wcnss_ctrl, expect_cbc: *mut bool) -> c_int {
    static int wcnss_download_nv(struct wcnss_ctrl *wcnss, bool *expect_cbc)
    {
    pub fw: *const firmware,
    pub wcnss->dev: *mut *mut device dev =,
    pub req: *mut wcnss_download_nv_req,
    pub NVBIN_FILE: *const *const char nvbin =,
    pub data: *const c_void,
    pub left: isize,
    pub ret: c_int,
    pub &nvbin): ret = of_property_read_string(dev->of_node, "firmware-name",,
    if (ret < 0 && ret != -EINVAL)
    pub ret: return,
    pub dev): ret = request_firmware(&fw, nvbin,,
    if (ret < 0) {
    pub ret): dev_err(dev, "Failed to load nv file %s: %d\n", nvbin,,
    pub ret: return,
    }
    pub fw->data: data =,
    pub fw->size: left =,
    pub NV_FRAGMENT_SIZE): *mut *mut req = kzalloc_flex(req, fragment,,
    if (!req) {
    pub -ENOMEM: ret =,
    pub release_fw: goto,
    }
    pub NV_FRAGMENT_SIZE: req->frag_size =,
    pub WCNSS_DOWNLOAD_NV_REQ: req->hdr.type =,
    pub NV_FRAGMENT_SIZE): req->hdr.len = struct_size(req, fragment,,
    pub 0: req->last =,
    pub 0: req->seq =,
    do {
    if (left <= NV_FRAGMENT_SIZE) {
    pub 1: req->last =,
    pub left: req->frag_size =,
    pub left: *mut *mut req->hdr.len = sizeof(req) +,
    }
    pub req->frag_size): memcpy(req->fragment, data,,
    pub req->hdr.len): ret = rpmsg_send(wcnss->channel, req,,
    if (ret < 0) {
    pub packet\n"): dev_err(dev, "failed to send smd,
    pub release_req: goto,
    }
// Increment for next fragment
    pub NV_FRAGMENT_SIZE: data +=,
    pub NV_FRAGMENT_SIZE: left -=,
    pub 0): } while (left >,
    pub WCNSS_REQUEST_TIMEOUT): ret = wait_for_completion_timeout(&wcnss->ack,,
    if (!ret) {
    pub ack\n"): dev_err(dev, "timeout waiting for nv upload,
    pub -ETIMEDOUT: ret =,
    } else {
// expect_cbc = wcnss->ack_status == WCNSS_ACK_COLD_BOOTING;
    pub 0: ret =,
    }
    release_req:
    release_fw:
    pub ret: return,
    }
//
// qcom_wcnss_open_channel() - open additional SMD channel to WCNSS
// @wcnss:	wcnss handle, retrieved from drvdata
// @name:	SMD channel name
// @cb:		callback to handle incoming data on the channel
// @priv:	private data for use in the call-back
//
    struct rpmsg_endpoint *qcom_wcnss_open_channel(void *wcnss, const char *name, rpmsg_rx_cb_t cb, void *priv)
    {
    pub chinfo: rpmsg_channel_info,
    pub wcnss: *mut *mut wcnss_ctrl _wcnss =,
    pub sizeof(chinfo.name)): strscpy(chinfo.name, name,,
    pub RPMSG_ADDR_ANY: chinfo.src =,
    pub RPMSG_ADDR_ANY: chinfo.dst =,
    pub chinfo): return rpmsg_create_ept(_wcnss->channel->rpdev, cb, priv,,
    }
#[no_mangle]
unsafe extern "C" fn wcnss_async_probe(work: *mut work_struct) {
    static void wcnss_async_probe(struct work_struct *work)
    {
    pub probe_work): *mut *mut wcnss_ctrl wcnss = container_of(work, wcnss_ctrl,,
    pub expect_cbc: bool,
    pub ret: c_int,
    pub wcnss_request_version(wcnss): ret =,
    if (ret < 0)
    pub &expect_cbc): ret = wcnss_download_nv(wcnss,,
    if (ret < 0)
// Wait for pending cold boot completion if indicated by the nv downloader
    if (expect_cbc) {
    pub WCNSS_REQUEST_TIMEOUT): ret = wait_for_completion_timeout(&wcnss->cbc,,
    if (!ret)
    pub completion\n"): dev_err(wcnss->dev, "expected cold boot,
    }
    pub wcnss->dev): of_platform_populate(wcnss->dev->of_node, NULL, NULL,,
    }
#[no_mangle]
unsafe extern "C" fn wcnss_ctrl_probe(rpdev: *mut rpmsg_device) -> c_int {
    static int wcnss_ctrl_probe(struct rpmsg_device *rpdev)
    {
    pub wcnss: *mut wcnss_ctrl,
    pub GFP_KERNEL): *mut *mut wcnss = devm_kzalloc(&rpdev->dev, sizeof(wcnss),,
    if (!wcnss)
    pub -ENOMEM: return,
    pub &rpdev->dev: wcnss->dev =,
    pub rpdev->ept: wcnss->channel =,
    pub wcnss_async_probe): INIT_WORK(&wcnss->probe_work,,
    pub wcnss): dev_set_drvdata(&rpdev->dev,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn wcnss_ctrl_remove(rpdev: *mut rpmsg_device) {
    static void wcnss_ctrl_remove(struct rpmsg_device *rpdev)
    {
    pub dev_get_drvdata(&rpdev->dev): *mut *mut wcnss_ctrl wcnss =,
    }
    static const struct of_device_id wcnss_ctrl_of_match[] = {
    { .compatible = "qcom,wcnss", },
    {}
}

    MODULE_DEVICE_TABLE(of, wcnss_ctrl_of_match);
    static struct rpmsg_driver wcnss_ctrl_driver = {
    .probe = wcnss_ctrl_probe,
    .remove = wcnss_ctrl_remove,
    .callback = wcnss_ctrl_smd_callback,
    .drv  = {
    .name  = "qcom_wcnss_ctrl",
    .of_match_table = wcnss_ctrl_of_match,
    },
    };
    module_rpmsg_driver(wcnss_ctrl_driver);
    MODULE_DESCRIPTION("Qualcomm WCNSS control driver");
    MODULE_LICENSE("GPL v2");
