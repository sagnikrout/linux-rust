//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/sof-client-probes-ipc4.c
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
// Author: Jyri Sarha <jyri.sarha@intel.com>
//

    enum sof_ipc4_dma_type {
    SOF_IPC4_DMA_HDA_HOST_OUTPUT = 0,
    SOF_IPC4_DMA_HDA_HOST_INPUT = 1,
    SOF_IPC4_DMA_HDA_LINK_OUTPUT = 8,
    SOF_IPC4_DMA_HDA_LINK_INPUT = 9,
    SOF_IPC4_DMA_DMIC_LINK_INPUT = 11,
    SOF_IPC4_DMA_I2S_LINK_OUTPUT = 12,
    SOF_IPC4_DMA_I2S_LINK_INPUT = 13,
    };
    enum sof_ipc4_probe_runtime_param {
    SOF_IPC4_PROBE_INJECTION_DMA = 1,
    SOF_IPC4_PROBE_INJECTION_DMA_DETACH,
    SOF_IPC4_PROBE_POINTS,
    SOF_IPC4_PROBE_POINTS_DISCONNECT,
    SOF_IPC4_PROBE_POINTS_AVAILABLE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_probe_gtw_cfg {
    pub node_id: u32,
    pub dma_buffer_size: u32,
    pub __aligned(4): } __packed,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_probe_cfg {
    pub base: sof_ipc4_base_module_cfg,
    pub gtw_cfg: sof_ipc4_probe_gtw_cfg,
    pub __aligned(4): } __packed,
    enum sof_ipc4_probe_type {
    SOF_IPC4_PROBE_TYPE_INPUT = 0,
    SOF_IPC4_PROBE_TYPE_OUTPUT,
    SOF_IPC4_PROBE_TYPE_INTERNAL
}

pub const SOF_IPC4_PROBE_TYPE_SHIFT: c_int = 24;

    >> SOF_IPC4_PROBE_TYPE_SHIFT)
pub const SOF_IPC4_PROBE_IDX_SHIFT: c_int = 26;

    >> SOF_IPC4_PROBE_IDX_SHIFT)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_probe_point {
    pub point_id: u32,
    pub purpose: u32,
    pub stream_tag: u32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_probe_info {
    pub num_elems: c_uint,
    pub points): DECLARE_FLEX_ARRAY(struct sof_ipc4_probe_point,,
    pub __packed: },
pub const INVALID_PIPELINE_ID: c_uint = 0xFF;
    static const char *sof_probe_ipc4_type_string(u32 type)
    {
    switch (type) {
    case SOF_IPC4_PROBE_TYPE_INPUT:
    pub "input": return,
    case SOF_IPC4_PROBE_TYPE_OUTPUT:
    pub "output": return,
    case SOF_IPC4_PROBE_TYPE_INTERNAL:
    pub "internal": return,
    default:
    pub "UNKNOWN": return,
    }
    }
//
// sof_ipc4_probe_get_module_info - Get IPC4 module info for probe module
// @cdev:		SOF client device
// @return:		Pointer to IPC4 probe module info
//
// Look up the IPC4 probe module info based on the hard coded uuid and
// store the value for the future calls.
//
    static struct sof_man4_module *sof_ipc4_probe_get_module_info(struct sof_client_dev *cdev)
    {
    pub cdev->data: *mut *mut sof_probes_priv priv =,
    pub &cdev->auxdev.dev: *mut *mut device dev =,
    static const guid_t probe_uuid =
    GUID_INIT(0x7CAD0808, 0xAB10, 0xCD23,
    pub 0xEF): 0xEF, 0x45, 0x12, 0xAB, 0x34, 0xCD, 0x56,,
    if (!priv.ipc_priv) {
    struct sof_ipc4_fw_module *fw_module =
    pub &probe_uuid): sof_client_ipc4_find_module(cdev,,
    if (!fw_module) {
    pub __func__): dev_err(dev, "%s: no matching uuid found",,
    pub NULL: return,
    }
    pub &fw_module->man4_module_entry: priv->ipc_priv =,
    }
    pub )priv->ipc_priv: *mut return (struct sof_man4_module,
    }
//
// ipc4_probes_init - initialize data probing
// @cdev:		SOF client device
// @stream_tag:		Extractor stream tag
// @buffer_size:	DMA buffer size to set for extractor
// @return:		0 on success, negative error code on error
//
// Host chooses whether extraction is supported or not by providing
// valid stream tag to DSP. Once specified, stream described by that
// tag will be tied to DSP for extraction for the entire lifetime of
// probe.
//
// Probing is initialized only once and each INIT request must be
// matched by DEINIT call.
//
    static int ipc4_probes_init(struct sof_client_dev *cdev, u32 stream_tag,
    size_t buffer_size)
    {
    pub sof_ipc4_probe_get_module_info(cdev): *mut *mut sof_man4_module mentry =,
    pub msg: sof_ipc4_msg,
    pub cfg: sof_ipc4_probe_cfg,
    if (!mentry)
    pub -ENODEV: return,
    pub sizeof(cfg)): memset(&cfg, '\0',,
    cfg.gtw_cfg.node_id = SOF_IPC4_PROBE_NODE_ID_INDEX(stream_tag - 1) |
    pub buffer_size: cfg.gtw_cfg.dma_buffer_size =,
    pub mentry->id: msg.primary =,
    pub SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_MOD_INIT_INSTANCE): msg.primary |=,
    pub SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST): msg.primary |=,
    pub SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG): msg.primary |=,
    pub SOF_IPC4_MOD_EXT_DST_MOD_INSTANCE(INVALID_PIPELINE_ID): msg.extension =,
    pub SOF_IPC4_MOD_EXT_CORE_ID(0): msg.extension |=,
    pub sizeof(uint32_t)): msg.extension |= SOF_IPC4_MOD_EXT_PARAM_SIZE(sizeof(cfg) /,
    pub sizeof(cfg): msg.data_size =,
    pub &cfg: msg.data_ptr =,
    pub &msg): return sof_client_ipc_tx_message_no_reply(cdev,,
    }
//
// ipc4_probes_deinit - cleanup after data probing
// @cdev:		SOF client device
// @return:		0 on success, negative error code on error
//
// Host sends DEINIT request to free previously initialized probe
// on DSP side once it is no longer needed. DEINIT only when there
// are no probes connected and with all injectors detached.
//
#[no_mangle]
unsafe extern "C" fn ipc4_probes_deinit(cdev: *mut sof_client_dev) -> c_int {
    static int ipc4_probes_deinit(struct sof_client_dev *cdev)
    {
    pub sof_ipc4_probe_get_module_info(cdev): *mut *mut sof_man4_module mentry =,
    pub msg: sof_ipc4_msg,
    if (!mentry)
    pub -ENODEV: return,
    pub mentry->id: msg.primary =,
    pub SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_MOD_DELETE_INSTANCE): msg.primary |=,
    pub SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST): msg.primary |=,
    pub SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG): msg.primary |=,
    pub SOF_IPC4_MOD_EXT_DST_MOD_INSTANCE(INVALID_PIPELINE_ID): msg.extension =,
    pub SOF_IPC4_MOD_EXT_CORE_ID(0): msg.extension |=,
    pub 0: msg.data_size =,
    pub NULL: msg.data_ptr =,
    pub &msg): return sof_client_ipc_tx_message_no_reply(cdev,,
    }
//
// ipc4_probes_points_info - retrieve list of probe points
// @cdev:	SOF client device
// @desc:	Returned list of active probes
// @num_desc:	Returned count of active probes
// @type:	Either PROBES_INFO_ACTIVE_PROBES or PROBES_INFO_AVAILABE_PROBES
// @return:	0 on success, negative error code on error
//
// Returns list if active probe points if type is
// PROBES_INFO_ACTIVE_PROBES, or list of all available probe points if
// type is PROBES_INFO_AVAILABE_PROBES.
//
    static int ipc4_probes_points_info(struct sof_client_dev *cdev,
    struct sof_probe_point_desc **desc,
    size_t *num_desc,
    enum sof_probe_info_type type)
    {
    pub sof_ipc4_probe_get_module_info(cdev): *mut *mut sof_man4_module mentry =,
    pub &cdev->auxdev.dev: *mut *mut device dev =,
    pub info: *mut sof_ipc4_probe_info,
    pub msg: sof_ipc4_msg,
    pub param_id: u32,
    pub ret: int i,,
    if (!mentry)
    pub -ENODEV: return,
    switch (type) {
    case PROBES_INFO_ACTIVE_PROBES:
    pub SOF_IPC4_PROBE_POINTS: param_id =,
    case PROBES_INFO_AVAILABE_PROBES:
    pub SOF_IPC4_PROBE_POINTS_AVAILABLE: param_id =,
    default:
    pub type): dev_err(dev, "%s: info type %u not supported", __func__,,
    pub -EOPNOTSUPP: return,
    }
    pub mentry->id: msg.primary =,
    pub SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST): msg.primary |=,
    pub SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG): msg.primary |=,
    pub SOF_IPC4_MOD_EXT_MSG_PARAM_ID(param_id): msg.extension =,
    pub sof_client_get_ipc_max_payload_size(cdev): msg.data_size =,
    pub GFP_KERNEL): msg.data_ptr = kzalloc(msg.data_size,,
    if (!msg.data_ptr)
    pub -ENOMEM: return,
    pub false): ret = sof_client_ipc_set_get_data(cdev, &msg,,
    if (ret) {
    pub ret: return,
    }
    pub msg.data_ptr: info =,
    if (msg.data_size < sizeof(*info) ||
    info.num_elems > (msg.data_size - sizeof(*info)) /
    sizeof(info.points[0])) {
    dev_err(dev, "%s: invalid probe info element count %u\n",
    pub info->num_elems): __func__,,
    pub -EINVAL: return,
    }
// num_desc = info->num_elems;
    pub num_desc): *mut dev_dbg(dev, "%s: got %zu probe points", __func__,,
// desc = kzalloc_objs(**desc, *num_desc);
    if (!*desc) {
    pub -ENOMEM: return,
    }
    pub {: *mut *mut for (i = 0; i < num_desc; i++),
    pub info->points[i].point_id: *mut *mut (desc)[i].buffer_id =,
    pub info->points[i].purpose: *mut *mut (desc)[i].purpose =,
    pub info->points[i].stream_tag: *mut *mut (desc)[i].stream_tag =,
    }
    pub 0: return,
    }
//
// ipc4_probes_point_print - Human readable print of probe point descriptor
// @cdev:	SOF client device
// @buf:	Buffer to print to
// @size:	Available bytes in buffer
// @desc:	Describes the probe point to print
// @return:	Number of bytes printed or an error code (snprintf return value)
//
    static int ipc4_probes_point_print(struct sof_client_dev *cdev, char *buf, size_t size,
    struct sof_probe_point_desc *desc)
    {
    pub &cdev->auxdev.dev: *mut *mut device dev =,
    pub swidget: *mut snd_sof_widget,
    pub ret: c_int,
    swidget = sof_client_ipc4_find_swidget_by_id(cdev, SOF_IPC4_MOD_ID_GET(desc.buffer_id),
    if (!swidget)
    dev_err(dev, "%s: Failed to find widget for module %lu.%lu\n",
    __func__, SOF_IPC4_MOD_ID_GET(desc.buffer_id),
    ret = scnprintf(buf, size, "%#x,%#x,%#x\t%s %s buf idx %lu %s\n",
    desc.buffer_id, desc.purpose, desc.stream_tag,
    swidget ? swidget.widget.name : "<unknown>",
    sof_probe_ipc4_type_string(SOF_IPC4_PROBE_TYPE_GET(desc.buffer_id)),
    SOF_IPC4_PROBE_IDX_GET(desc.buffer_id),
    pub ""): desc->stream_tag ? "(connected)" :,
    pub ret: return,
    }
//
// ipc4_probes_points_add - connect specified probes
// @cdev:	SOF client device
// @desc:	List of probe points to connect
// @num_desc:	Number of elements in @desc
// @return:	0 on success, negative error code on error
//
// Translates the generic probe point presentation to an IPC4
// message to dynamically connect the provided set of endpoints.
//
    static int ipc4_probes_points_add(struct sof_client_dev *cdev,
    struct sof_probe_point_desc *desc,
    size_t num_desc)
    {
    pub sof_ipc4_probe_get_module_info(cdev): *mut *mut sof_man4_module mentry =,
    pub points: *mut sof_ipc4_probe_point,
    pub msg: sof_ipc4_msg,
    pub ret: int i,,
    if (!mentry)
    pub -EOPNOTSUPP: return,
// The sof_probe_point_desc and sof_ipc4_probe_point structs
// are of same size and even the integers are the same in the
// same order, and similar meaning, but since there is no
// performance issue I wrote the conversion explicitly open for
// future development.
//
    pub num_desc): *mut *mut points = kzalloc_objs(points,,
    if (!points)
    pub -ENOMEM: return,
    pub {: for (i = 0; i < num_desc; i++),
    pub desc[i].buffer_id: points[i].point_id =,
    pub desc[i].purpose: points[i].purpose =,
    pub desc[i].stream_tag: points[i].stream_tag =,
    }
    pub mentry->id: msg.primary =,
    pub SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST): msg.primary |=,
    pub SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG): msg.primary |=,
    pub SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_PROBE_POINTS): msg.extension =,
    pub num_desc: *mut *mut *mut msg.data_size = sizeof(points),
    pub points: msg.data_ptr =,
    pub true): ret = sof_client_ipc_set_get_data(cdev, &msg,,
    pub ret: return,
    }
//
// ipc4_probes_points_remove - disconnect specified probes
// @cdev:		SOF client device
// @buffer_id:		List of probe points to disconnect
// @num_buffer_id:	Number of elements in @desc
// @return:		0 on success, negative error code on error
//
// Converts the generic buffer_id to IPC4 probe_point_id and remove
// the probe points with an IPC4 for message.
//
    static int ipc4_probes_points_remove(struct sof_client_dev *cdev,
    unsigned int *buffer_id, size_t num_buffer_id)
    {
    pub sof_ipc4_probe_get_module_info(cdev): *mut *mut sof_man4_module mentry =,
    pub msg: sof_ipc4_msg,
    pub probe_point_ids: *mut u32,
    pub ret: int i,,
    if (!mentry)
    pub -ENODEV: return,
    probe_point_ids = kcalloc(num_buffer_id, sizeof(*probe_point_ids),
    if (!probe_point_ids)
    pub -ENOMEM: return,
    pub i++): for (i = 0; i < num_buffer_id;,
    pub buffer_id: [probe_point_ids[i] =; i],
    pub mentry->id: msg.primary =,
    pub SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST): msg.primary |=,
    pub SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG): msg.primary |=,
    msg.extension =
    pub sizeof(*probe_point_ids): *mut *mut msg.data_size = num_buffer_id,
    pub probe_point_ids: msg.data_ptr =,
    pub true): ret = sof_client_ipc_set_get_data(cdev, &msg,,
    pub ret: return,
    }
    const struct sof_probes_ipc_ops ipc4_probe_ops =  {
    .init = ipc4_probes_init,
    .deinit = ipc4_probes_deinit,
    .points_info = ipc4_probes_points_info,
    .point_print = ipc4_probes_point_print,
    .points_add = ipc4_probes_points_add,
    .points_remove = ipc4_probes_points_remove,
}
