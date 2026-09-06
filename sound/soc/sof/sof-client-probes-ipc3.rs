//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/sof-client-probes-ipc3.c
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
// Copyright(c) 2019-2022 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//
// Code moved to this file by:
// Jyri Sarha <jyri.sarha@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_probe_dma {
    pub stream_tag: c_uint,
    pub dma_buffer_size: c_uint,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_probe_dma_add_params {
    pub hdr: sof_ipc_cmd_hdr,
    pub num_elems: c_uint,
    pub dma: [sof_probe_dma; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_probe_info_params {
    pub rhdr: sof_ipc_reply,
    pub num_elems: c_uint,
    union {
    pub dma): DECLARE_FLEX_ARRAY(struct sof_probe_dma,,
    pub desc): DECLARE_FLEX_ARRAY(struct sof_probe_point_desc,,
}

    } __packed;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_probe_point_add_params {
    pub hdr: sof_ipc_cmd_hdr,
    pub num_elems: c_uint,
    pub desc: [sof_probe_point_desc; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_probe_point_remove_params {
    pub hdr: sof_ipc_cmd_hdr,
    pub num_elems: c_uint,
    pub buffer_id: [c_uint; ],
    pub __packed: },
//
// ipc3_probes_init - initialize data probing
// @cdev:		SOF client device
// @stream_tag:		Extractor stream tag
// @buffer_size:	DMA buffer size to set for extractor
//
// Host chooses whether extraction is supported or not by providing
// valid stream tag to DSP. Once specified, stream described by that
// tag will be tied to DSP for extraction for the entire lifetime of
// probe.
//
// Probing is initialized only once and each INIT request must be
// matched by DEINIT call.
//
    static int ipc3_probes_init(struct sof_client_dev *cdev, u32 stream_tag,
    size_t buffer_size)
    {
    pub msg: *mut sof_ipc_probe_dma_add_params,
    pub 1): size_t size = struct_size(msg, dma,,
    pub ret: c_int,
    pub GFP_KERNEL): msg = kmalloc(size,,
    if (!msg)
    pub -ENOMEM: return,
    pub size: msg->hdr.size =,
    pub SOF_IPC_PROBE_INIT: msg->hdr.cmd = SOF_IPC_GLB_PROBE |,
    pub 1: msg->num_elems =,
    pub stream_tag: msg->dma[0].stream_tag =,
    pub buffer_size: msg->dma[0].dma_buffer_size =,
    pub msg): ret = sof_client_ipc_tx_message_no_reply(cdev,,
    pub ret: return,
    }
//
// ipc3_probes_deinit - cleanup after data probing
// @cdev:		SOF client device
//
// Host sends DEINIT request to free previously initialized probe
// on DSP side once it is no longer needed. DEINIT only when there
// are no probes connected and with all injectors detached.
//
#[no_mangle]
unsafe extern "C" fn ipc3_probes_deinit(cdev: *mut sof_client_dev) -> c_int {
    static int ipc3_probes_deinit(struct sof_client_dev *cdev)
    {
    pub msg: sof_ipc_cmd_hdr,
    pub sizeof(msg): msg.size =,
    pub SOF_IPC_PROBE_DEINIT: msg.cmd = SOF_IPC_GLB_PROBE |,
    pub &msg): return sof_client_ipc_tx_message_no_reply(cdev,,
    }
    static int ipc3_probes_info(struct sof_client_dev *cdev, unsigned int cmd,
    void **params, size_t *num_params,
    enum sof_probe_info_type type)
    {
    pub sof_client_get_ipc_max_payload_size(cdev): size_t max_msg_size =,
    pub &cdev->auxdev.dev: *mut *mut device dev =,
    pub {{{0}}}: sof_ipc_probe_info_params msg =,
    pub reply: *mut sof_ipc_probe_info_params,
    pub payload_size: size_t bytes, elem_size,,
    pub ret: c_int,
// params = NULL;
// num_params = 0;
    if (type != PROBES_INFO_ACTIVE_PROBES) {
    pub type): dev_err(dev, "%s: info type %u not supported", __func__,,
    pub -EOPNOTSUPP: return,
    }
    pub GFP_KERNEL): reply = kzalloc(max_msg_size,,
    if (!reply)
    pub -ENOMEM: return,
    pub sizeof(msg): msg.rhdr.hdr.size =,
    pub cmd: msg.rhdr.hdr.cmd = SOF_IPC_GLB_PROBE |,
    pub max_msg_size): ret = sof_client_ipc_tx_message(cdev, &msg, reply,,
    if (ret < 0 || reply.rhdr.error < 0)
    pub exit: goto,
    pub reply->rhdr.hdr.size: payload_size =,
    if (payload_size < offsetof(struct sof_ipc_probe_info_params, dma)) {
    pub -EINVAL: ret =,
    pub exit: goto,
    }
    if (!reply.num_elems)
    pub exit: goto,
    if (cmd == SOF_IPC_PROBE_DMA_INFO)
    pub sizeof(reply->dma[0]): elem_size =,
    else
    pub sizeof(reply->desc[0]): elem_size =,
    pub dma): payload_size -= offsetof(struct sof_ipc_probe_info_params,,
    if (reply.num_elems > payload_size / elem_size) {
    dev_err(dev, "%s: invalid probe info element count %u\n",
    pub reply->num_elems): __func__,,
    pub -EINVAL: ret =,
    pub exit: goto,
    }
    pub elem_size: *mut *mut bytes = reply->num_elems,
// params = kmemdup(&reply->dma[0], bytes, GFP_KERNEL);
    if (!*params) {
    pub -ENOMEM: ret =,
    pub exit: goto,
    }
// num_params = reply->num_elems;
    exit:
    pub ret: return,
    }
//
// ipc3_probes_points_info - retrieve list of probe points
// @cdev:		SOF client device
// @desc:	Returned list of active probes
// @num_desc:	Returned count of active probes
// @type:	Either PROBES_INFO_ACTIVE_PROBES or PROBES_INFO_AVAILABE_PROBES
//
// If type is PROBES_INFO_ACTIVE_PROBES, host sends PROBE_POINT_INFO
// request to obtain list of active probe points, valid for
// disconnection when given probe is no longer required.
//
// Type PROBES_INFO_AVAILABE_PROBES is not yet supported.
//
    static int ipc3_probes_points_info(struct sof_client_dev *cdev,
    struct sof_probe_point_desc **desc,
    size_t *num_desc,
    enum sof_probe_info_type type)
    {
    return ipc3_probes_info(cdev, SOF_IPC_PROBE_POINT_INFO,
    pub type): *mut *mut *mut (void )desc, num_desc,,
    }
//
// ipc3_probes_points_add - connect specified probes
// @cdev:		SOF client device
// @desc:	List of probe points to connect
// @num_desc:	Number of elements in @desc
//
// Dynamically connects to provided set of endpoints. Immediately
// after connection is established, host must be prepared to
// transfer data from or to target stream given the probing purpose.
//
// Each probe point should be removed using PROBE_POINT_REMOVE
// request when no longer needed.
//
    static int ipc3_probes_points_add(struct sof_client_dev *cdev,
    struct sof_probe_point_desc *desc,
    size_t num_desc)
    {
    pub msg: *mut sof_ipc_probe_point_add_params,
    pub num_desc): size_t size = struct_size(msg, desc,,
    pub ret: c_int,
    pub GFP_KERNEL): msg = kmalloc(size,,
    if (!msg)
    pub -ENOMEM: return,
    pub size: msg->hdr.size =,
    pub num_desc: msg->num_elems =,
    pub SOF_IPC_PROBE_POINT_ADD: msg->hdr.cmd = SOF_IPC_GLB_PROBE |,
    pub sizeof(*msg)): *mut memcpy(&msg->desc[0], desc, size -,
    pub msg): ret = sof_client_ipc_tx_message_no_reply(cdev,,
    pub ret: return,
    }
//
// ipc3_probes_points_remove - disconnect specified probes
// @cdev:		SOF client device
// @buffer_id:		List of probe points to disconnect
// @num_buffer_id:	Number of elements in @desc
//
// Removes previously connected probes from list of active probe
// points and frees all resources on DSP side.
//
    static int ipc3_probes_points_remove(struct sof_client_dev *cdev,
    unsigned int *buffer_id,
    size_t num_buffer_id)
    {
    pub msg: *mut sof_ipc_probe_point_remove_params,
    pub num_buffer_id): size_t size = struct_size(msg, buffer_id,,
    pub ret: c_int,
    pub GFP_KERNEL): msg = kmalloc(size,,
    if (!msg)
    pub -ENOMEM: return,
    pub size: msg->hdr.size =,
    pub num_buffer_id: msg->num_elems =,
    pub SOF_IPC_PROBE_POINT_REMOVE: msg->hdr.cmd = SOF_IPC_GLB_PROBE |,
    pub sizeof(*msg)): *mut memcpy(&msg->buffer_id[0], buffer_id, size -,
    pub msg): ret = sof_client_ipc_tx_message_no_reply(cdev,,
    pub ret: return,
    }
    const struct sof_probes_ipc_ops ipc3_probe_ops =  {
    .init = ipc3_probes_init,
    .deinit = ipc3_probes_deinit,
    .points_info = ipc3_probes_points_info,
    .points_add = ipc3_probes_points_add,
    .points_remove = ipc3_probes_points_remove,
}
