//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/hyperv/hyperv_drm_proto.c
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
// Copyright 2021 Microsoft
//

// Support for VERSION_WIN7 is removed. #define is retained for reference.

pub const SYNTHVID_DEPTH_WIN8: c_int = 32;
pub const SYNTHVID_WIDTH_WIN8: c_int = 1600;
pub const SYNTHVID_HEIGHT_WIN8: c_int = 1200;

    enum pipe_msg_type {
    PIPE_MSG_INVALID,
    PIPE_MSG_DATA,
    PIPE_MSG_MAX
    };
    enum synthvid_msg_type {
    SYNTHVID_ERROR			= 0,
    SYNTHVID_VERSION_REQUEST	= 1,
    SYNTHVID_VERSION_RESPONSE	= 2,
    SYNTHVID_VRAM_LOCATION		= 3,
    SYNTHVID_VRAM_LOCATION_ACK	= 4,
    SYNTHVID_SITUATION_UPDATE	= 5,
    SYNTHVID_SITUATION_UPDATE_ACK	= 6,
    SYNTHVID_POINTER_POSITION	= 7,
    SYNTHVID_POINTER_SHAPE		= 8,
    SYNTHVID_FEATURE_CHANGE		= 9,
    SYNTHVID_DIRT			= 10,
    SYNTHVID_RESOLUTION_REQUEST	= 13,
    SYNTHVID_RESOLUTION_RESPONSE	= 14,
    SYNTHVID_MAX			= 15
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_msg_hdr {
    pub type: u32,
    pub /: *mut *mut u32 size; / size of message after this field,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvd_screen_info {
    pub width: u16,
    pub height: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_msg_hdr {
    pub type: u32,
    pub /: *mut *mut u32 size; / size of this header + payload after this field,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_version_req {
    pub version: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_version_resp {
    pub version: u32,
    pub is_accepted: u8,
    pub max_video_outputs: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_vram_location {
    pub user_ctx: u64,
    pub is_vram_gpa_specified: u8,
    pub vram_gpa: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_vram_location_ack {
    pub user_ctx: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_output_situation {
    pub active: u8,
    pub vram_offset: u32,
    pub depth_bits: u8,
    pub width_pixels: u32,
    pub height_pixels: u32,
    pub pitch_bytes: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_situation_update {
    pub user_ctx: u64,
    pub video_output_count: u8,
    pub video_output: [video_output_situation; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_situation_update_ack {
    pub user_ctx: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_pointer_position {
    pub is_visible: u8,
    pub video_output: u8,
    pub image_x: i32,
    pub image_y: i32,
    pub __packed: },
pub const SYNTHVID_CURSOR_MAX_X: c_int = 96;
pub const SYNTHVID_CURSOR_MAX_Y: c_int = 96;
pub const SYNTHVID_CURSOR_ARGB_PIXEL_SIZE: c_int = 4;

    SYNTHVID_CURSOR_MAX_Y * SYNTHVID_CURSOR_ARGB_PIXEL_SIZE)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_pointer_shape {
    pub part_idx: u8,
    pub is_argb: u8,
    pub /: *mut *mut u32 width; / SYNTHVID_CURSOR_MAX_X at most,
    pub /: *mut *mut u32 height; / SYNTHVID_CURSOR_MAX_Y at most,
    pub /: *mut *mut u32 hot_x; / hotspot relative to upper-left of pointer image,
    pub hot_y: u32,
    pub data: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_feature_change {
    pub is_dirt_needed: u8,
    pub is_ptr_pos_needed: u8,
    pub is_ptr_shape_needed: u8,
    pub is_situ_needed: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rect {
    pub /: *mut *mut s32 x1, y1; / top left corner,
    pub /: *mut *mut s32 x2, y2; / bottom right corner, exclusive,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_dirt {
    pub video_output: u8,
    pub dirt_count: u8,
    pub rect: [rect; 1],
    pub __packed: },
pub const SYNTHVID_EDID_BLOCK_SIZE: c_int = 128;
pub const SYNTHVID_MAX_RESOLUTION_COUNT: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_supported_resolution_req {
    pub maximum_resolution_count: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_supported_resolution_resp {
    pub edid_block: [u8; SYNTHVID_EDID_BLOCK_SIZE],
    pub resolution_count: u8,
    pub default_resolution_index: u8,
    pub is_standard: u8,
    pub supported_resolution: [hvd_screen_info; SYNTHVID_MAX_RESOLUTION_COUNT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthvid_msg {
    pub pipe_hdr: pipe_msg_hdr,
    pub vid_hdr: synthvid_msg_hdr,
    union {
    pub ver_req: synthvid_version_req,
    pub ver_resp: synthvid_version_resp,
    pub vram: synthvid_vram_location,
    pub vram_ack: synthvid_vram_location_ack,
    pub situ: synthvid_situation_update,
    pub situ_ack: synthvid_situation_update_ack,
    pub ptr_pos: synthvid_pointer_position,
    pub ptr_shape: synthvid_pointer_shape,
    pub feature_chg: synthvid_feature_change,
    pub dirt: synthvid_dirt,
    pub resolution_req: synthvid_supported_resolution_req,
    pub resolution_resp: synthvid_supported_resolution_resp,
}

    } __packed;
#[no_mangle]
pub unsafe extern "C" fn hv_drm_version_ge(ver1: u32, ver2: u32) -> bool {
    static inline bool hv_drm_version_ge(u32 ver1, u32 ver2)
    {
    if (SYNTHVID_VER_GET_MAJOR(ver1) > SYNTHVID_VER_GET_MAJOR(ver2) ||
    (SYNTHVID_VER_GET_MAJOR(ver1) == SYNTHVID_VER_GET_MAJOR(ver2) &&
    SYNTHVID_VER_GET_MINOR(ver1) >= SYNTHVID_VER_GET_MINOR(ver2)))
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn hv_drm_sendpacket(hdev: *mut hv_device, msg: *mut synthvid_msg) -> c_int {
    static inline int hv_drm_sendpacket(struct hv_device *hdev, struct synthvid_msg *msg)
    {
    let mut request_id: static atomic64_t = ATOMIC64_INIT(0);
    struct hv_drm_device *hv = hv_get_drvdata(hdev);
    int ret;
    msg.pipe_hdr.type = PIPE_MSG_DATA;
    msg.pipe_hdr.size = msg.vid_hdr.size;
    ret = vmbus_sendpacket(hdev.channel, msg,
    msg.vid_hdr.size + sizeof(struct pipe_msg_hdr),
    atomic64_inc_return(&request_id),
    VM_PKT_DATA_INBAND, 0);
    if (ret)
    drm_err_ratelimited(&hv.dev, "Unable to send packet via vmbus; error %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hv_drm_negotiate_version(hdev: *mut hv_device, ver: u32) -> c_int {
    static int hv_drm_negotiate_version(struct hv_device *hdev, u32 ver)
    {
    struct hv_drm_device *hv = hv_get_drvdata(hdev);
    struct synthvid_msg *msg = (struct synthvid_msg *)hv.init_buf;
    struct drm_device *dev = &hv.dev;
    unsigned long t;
    memset(msg, 0, sizeof(struct synthvid_msg));
    msg.vid_hdr.type = SYNTHVID_VERSION_REQUEST;
    msg.vid_hdr.size = sizeof(struct synthvid_msg_hdr) +
    sizeof(struct synthvid_version_req);
    msg.ver_req.version = ver;
    hv_drm_sendpacket(hdev, msg);
    t = wait_for_completion_timeout(&hv.wait, VMBUS_VSP_TIMEOUT);
    if (!t) {
    drm_err(dev, "Time out on waiting version response\n");
    return -ETIMEDOUT;
    }
    if (!msg.ver_resp.is_accepted) {
    drm_err(dev, "Version request not accepted\n");
    return -ENODEV;
    }
    hv.synthvid_version = ver;
    drm_info(dev, "Synthvid Version major %d, minor %d\n",
    SYNTHVID_VER_GET_MAJOR(ver), SYNTHVID_VER_GET_MINOR(ver));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hv_drm_update_vram_location(hdev: *mut hv_device, vram_pp: phys_addr_t) -> c_int {
    int hv_drm_update_vram_location(struct hv_device *hdev, phys_addr_t vram_pp)
    {
    struct hv_drm_device *hv = hv_get_drvdata(hdev);
    struct synthvid_msg *msg = (struct synthvid_msg *)hv.init_buf;
    struct drm_device *dev = &hv.dev;
    unsigned long t;
    memset(msg, 0, sizeof(struct synthvid_msg));
    msg.vid_hdr.type = SYNTHVID_VRAM_LOCATION;
    msg.vid_hdr.size = sizeof(struct synthvid_msg_hdr) +
    sizeof(struct synthvid_vram_location);
    msg.vram.user_ctx = vram_pp;
    msg.vram.vram_gpa = vram_pp;
    msg.vram.is_vram_gpa_specified = 1;
    hv_drm_sendpacket(hdev, msg);
    t = wait_for_completion_timeout(&hv.wait, VMBUS_VSP_TIMEOUT);
    if (!t) {
    drm_err(dev, "Time out on waiting vram location ack\n");
    return -ETIMEDOUT;
    }
    if (msg.vram_ack.user_ctx != vram_pp) {
    drm_err(dev, "Unable to set VRAM location\n");
    return -ENODEV;
    }
    return 0;
    }
    int hv_drm_update_situation(struct hv_device *hdev, u8 active, u32 bpp,
    u32 w, u32 h, u32 pitch)
    {
    struct synthvid_msg msg;
    memset(&msg, 0, sizeof(struct synthvid_msg));
    msg.vid_hdr.type = SYNTHVID_SITUATION_UPDATE;
    msg.vid_hdr.size = sizeof(struct synthvid_msg_hdr) +
    sizeof(struct synthvid_situation_update);
    msg.situ.user_ctx = 0;
    msg.situ.video_output_count = 1;
    msg.situ.video_output[0].active = active;
// vram_offset should always be 0
    msg.situ.video_output[0].vram_offset = 0;
    msg.situ.video_output[0].depth_bits = bpp;
    msg.situ.video_output[0].width_pixels = w;
    msg.situ.video_output[0].height_pixels = h;
    msg.situ.video_output[0].pitch_bytes = pitch;
    hv_drm_sendpacket(hdev, &msg);
    return 0;
    }
//
// Hyper-V supports a hardware cursor feature. It's not used by Linux VM,
// but the Hyper-V host still draws a point as an extra mouse pointer,
// which is unwanted, especially when Xorg is running.
//
// Hide the unwanted pointer, by setting msg.ptr_pos.is_visible = 1 and setting
// the msg.ptr_shape.data. Note: setting msg.ptr_pos.is_visible to 0 doesn't
// work in tests.
//
// The hv_drm_hide_hw_ptr() is also called in the handler of the
// SYNTHVID_FEATURE_CHANGE event, otherwise the host still draws an extra
// unwanted mouse pointer after the VM Connection window is closed and reopened.
//
#[no_mangle]
pub unsafe extern "C" fn hv_drm_hide_hw_ptr(hdev: *mut hv_device) -> c_int {
    int hv_drm_hide_hw_ptr(struct hv_device *hdev)
    {
    struct synthvid_msg msg;
    memset(&msg, 0, sizeof(struct synthvid_msg));
    msg.vid_hdr.type = SYNTHVID_POINTER_POSITION;
    msg.vid_hdr.size = sizeof(struct synthvid_msg_hdr) +
    sizeof(struct synthvid_pointer_position);
    msg.ptr_pos.is_visible = 1;
    msg.ptr_pos.video_output = 0;
    msg.ptr_pos.image_x = 0;
    msg.ptr_pos.image_y = 0;
    hv_drm_sendpacket(hdev, &msg);
    memset(&msg, 0, sizeof(struct synthvid_msg));
    msg.vid_hdr.type = SYNTHVID_POINTER_SHAPE;
    msg.vid_hdr.size = sizeof(struct synthvid_msg_hdr) +
    sizeof(struct synthvid_pointer_shape);
    msg.ptr_shape.part_idx = SYNTHVID_CURSOR_COMPLETE;
    msg.ptr_shape.is_argb = 1;
    msg.ptr_shape.width = 1;
    msg.ptr_shape.height = 1;
    msg.ptr_shape.hot_x = 0;
    msg.ptr_shape.hot_y = 0;
    msg.ptr_shape.data[0] = 0;
    msg.ptr_shape.data[1] = 1;
    msg.ptr_shape.data[2] = 1;
    msg.ptr_shape.data[3] = 1;
    hv_drm_sendpacket(hdev, &msg);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hv_drm_update_dirt(hdev: *mut hv_device, rect: *mut drm_rect) -> c_int {
    int hv_drm_update_dirt(struct hv_device *hdev, struct drm_rect *rect)
    {
    struct hv_drm_device *hv = hv_get_drvdata(hdev);
    struct synthvid_msg msg;
    if (!hv.dirt_needed)
    return 0;
    memset(&msg, 0, sizeof(struct synthvid_msg));
    msg.vid_hdr.type = SYNTHVID_DIRT;
    msg.vid_hdr.size = sizeof(struct synthvid_msg_hdr) +
    sizeof(struct synthvid_dirt);
    msg.dirt.video_output = 0;
    msg.dirt.dirt_count = 1;
    msg.dirt.rect[0].x1 = rect.x1;
    msg.dirt.rect[0].y1 = rect.y1;
    msg.dirt.rect[0].x2 = rect.x2;
    msg.dirt.rect[0].y2 = rect.y2;
    hv_drm_sendpacket(hdev, &msg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv_drm_get_supported_resolution(hdev: *mut hv_device) -> c_int {
    static int hv_drm_get_supported_resolution(struct hv_device *hdev)
    {
    struct hv_drm_device *hv = hv_get_drvdata(hdev);
    struct synthvid_msg *msg = (struct synthvid_msg *)hv.init_buf;
    struct drm_device *dev = &hv.dev;
    unsigned long t;
    u8 index;
    int i;
    memset(msg, 0, sizeof(struct synthvid_msg));
    msg.vid_hdr.type = SYNTHVID_RESOLUTION_REQUEST;
    msg.vid_hdr.size = sizeof(struct synthvid_msg_hdr) +
    sizeof(struct synthvid_supported_resolution_req);
    msg.resolution_req.maximum_resolution_count =
    SYNTHVID_MAX_RESOLUTION_COUNT;
    hv_drm_sendpacket(hdev, msg);
    t = wait_for_completion_timeout(&hv.wait, VMBUS_VSP_TIMEOUT);
    if (!t) {
    drm_err(dev, "Time out on waiting resolution response\n");
    return -ETIMEDOUT;
    }
    if (msg.resolution_resp.resolution_count == 0 ||
    msg.resolution_resp.resolution_count >
    SYNTHVID_MAX_RESOLUTION_COUNT) {
    drm_err(dev, "Invalid resolution count: %d\n",
    msg.resolution_resp.resolution_count);
    return -ENODEV;
    }
    index = msg.resolution_resp.default_resolution_index;
    if (index >= msg.resolution_resp.resolution_count) {
    drm_err(dev, "Invalid resolution index: %d\n", index);
    return -ENODEV;
    }
    for (i = 0; i < msg.resolution_resp.resolution_count; i++) {
    hv.screen_width_max = max_t(u32, hv.screen_width_max,
    msg.resolution_resp.supported_resolution[i].width);
    hv.screen_height_max = max_t(u32, hv.screen_height_max,
    msg.resolution_resp.supported_resolution[i].height);
    }
    hv.preferred_width =
    msg.resolution_resp.supported_resolution[index].width;
    hv.preferred_height =
    msg.resolution_resp.supported_resolution[index].height;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv_drm_receive_sub(hdev: *mut hv_device, bytes_recvd: u32) {
    static void hv_drm_receive_sub(struct hv_device *hdev, u32 bytes_recvd)
    {
    struct hv_drm_device *hv = hv_get_drvdata(hdev);
    struct synthvid_msg *msg;
    size_t hdr_size;
    size_t need;
    if (!hv)
    return;
    hdr_size = sizeof(struct pipe_msg_hdr) +
    sizeof(struct synthvid_msg_hdr);
    if (bytes_recvd < hdr_size) {
    drm_err_ratelimited(&hv.dev,
    "synthvid packet too small for header: %u\n",
    bytes_recvd);
    return;
    }
    msg = (struct synthvid_msg *)hv.recv_buf;
    need = hdr_size;
    switch (msg.vid_hdr.type) {
    case SYNTHVID_VERSION_RESPONSE:
    need += sizeof(struct synthvid_version_resp);
    break;
    case SYNTHVID_RESOLUTION_RESPONSE:
//
// The resolution response is variable length: the host
// fills resolution_count entries, not the full
// SYNTHVID_MAX_RESOLUTION_COUNT array. Require the fixed
// prefix first so resolution_count can be read, then
// demand exactly the count-sized array.
//
    need += offsetof(struct synthvid_supported_resolution_resp,
    supported_resolution);
    if (bytes_recvd < need)
    break;
    if (msg.resolution_resp.resolution_count >
    SYNTHVID_MAX_RESOLUTION_COUNT) {
    drm_err_ratelimited(&hv.dev,
    "synthvid resolution count too large: %u\n",
    msg.resolution_resp.resolution_count);
    return;
    }
    need += msg.resolution_resp.resolution_count *
    sizeof(struct hvd_screen_info);
    break;
    case SYNTHVID_VRAM_LOCATION_ACK:
    need += sizeof(struct synthvid_vram_location_ack);
    break;
    case SYNTHVID_FEATURE_CHANGE:
//
// Not a completion-driving message: validate its own payload
// and consume it here rather than falling through to the
// memcpy/complete shared by the wait-event responses.
//
    if (bytes_recvd < need +
    sizeof(struct synthvid_feature_change)) {
    drm_err_ratelimited(&hv.dev,
    "synthvid feature change packet too small: %u\n",
    bytes_recvd);
    return;
    }
    hv.dirt_needed = msg.feature_chg.is_dirt_needed;
    if (hv.dirt_needed)
    hv_drm_hide_hw_ptr(hv.hdev);
    return;
    default:
    return;
    }
//
// Shared completion path for the wait-event responses
// (VERSION_RESPONSE, RESOLUTION_RESPONSE, VRAM_LOCATION_ACK):
// require the type-specific payload before handing the buffer to
// the waiter.
//
    if (bytes_recvd < need) {
    drm_err_ratelimited(&hv.dev,
    "synthvid packet too small for type %u: %u < %zu\n",
    msg.vid_hdr.type, bytes_recvd, need);
    return;
    }
    memcpy(hv.init_buf, msg, bytes_recvd);
    complete(&hv.wait);
    }
#[no_mangle]
unsafe extern "C" fn hv_drm_receive(ctx: *mut c_void) {
    static void hv_drm_receive(void *ctx)
    {
    struct hv_device *hdev = ctx;
    struct hv_drm_device *hv = hv_get_drvdata(hdev);
    struct synthvid_msg *recv_buf;
    u32 bytes_recvd;
    u64 req_id;
    int ret;
    if (!hv)
    return;
    recv_buf = (struct synthvid_msg *)hv.recv_buf;
    do {
    ret = vmbus_recvpacket(hdev.channel, recv_buf,
    VMBUS_MAX_PACKET_SIZE,
    &bytes_recvd, &req_id);
    if (ret) {
//
// A nonzero return (e.g. -ENOBUFS for an oversized
// packet) is itself a malformed message: bytes_recvd
// then reports the required length rather than a copied
// payload, so it must not be forwarded to the
// sub-handler. Channel recovery is not attempted.
//
    drm_err_ratelimited(&hv.dev,
    "vmbus_recvpacket failed: %d (need %u)\n",
    ret, bytes_recvd);
    } else if (bytes_recvd > 0 &&
    recv_buf.pipe_hdr.type == PIPE_MSG_DATA) {
    hv_drm_receive_sub(hdev, bytes_recvd);
    }
    } while (bytes_recvd > 0 && ret == 0);
    }
#[no_mangle]
pub unsafe extern "C" fn hv_drm_connect_vsp(hdev: *mut hv_device) -> c_int {
    int hv_drm_connect_vsp(struct hv_device *hdev)
    {
    struct hv_drm_device *hv = hv_get_drvdata(hdev);
    struct drm_device *dev = &hv.dev;
    int ret;
    ret = vmbus_open(hdev.channel, VMBUS_RING_BUFSIZE, VMBUS_RING_BUFSIZE,
    core::ptr::null_mut(), 0, hv_drm_receive, hdev);
    if (ret) {
    drm_err(dev, "Unable to open vmbus channel\n");
    return ret;
    }
// Negotiate the protocol version with host
    switch (vmbus_proto_version) {
    case VERSION_WIN10:
    case VERSION_WIN10_V5:
    ret = hv_drm_negotiate_version(hdev, SYNTHVID_VERSION_WIN10);
    if (!ret)
    break;
    fallthrough;
    case VERSION_WIN8:
    case VERSION_WIN8_1:
    ret = hv_drm_negotiate_version(hdev, SYNTHVID_VERSION_WIN8);
    break;
    default:
    ret = hv_drm_negotiate_version(hdev, SYNTHVID_VERSION_WIN10);
    break;
    }
    if (ret) {
    drm_err(dev, "Synthetic video device version not accepted %d\n", ret);
    goto error;
    }
    hv.screen_depth = SYNTHVID_DEPTH_WIN8;
    if (hv_drm_version_ge(hv.synthvid_version, SYNTHVID_VERSION_WIN10)) {
    ret = hv_drm_get_supported_resolution(hdev);
    if (ret)
    drm_err(dev, "Failed to get supported resolution from host, use default\n");
    }
    if (!hv.screen_width_max) {
    hv.screen_width_max = SYNTHVID_WIDTH_WIN8;
    hv.screen_height_max = SYNTHVID_HEIGHT_WIN8;
    hv.preferred_width = SYNTHVID_WIDTH_WIN8;
    hv.preferred_height = SYNTHVID_HEIGHT_WIN8;
    }
    hv.mmio_megabytes = hdev.channel.offermsg.offer.mmio_megabytes;
    return 0;
    error:
    vmbus_close(hdev.channel);
    return ret;
    }
