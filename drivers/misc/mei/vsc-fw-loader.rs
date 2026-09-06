//! Automatically rewritten from C to Rust
//! Source: drivers/misc/mei/vsc-fw-loader.c
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
// Copyright (c) 2023, Intel Corporation.
// Intel Visual Sensing Controller Transport Layer Linux driver
//

pub const VSC_MAGIC_NUM: c_uint = 0x49505343 /* IPSC */;
pub const VSC_MAGIC_FW: c_uint = 0x49574653 /* IWFS */;
pub const VSC_MAGIC_FILE: c_uint = 0x46564353 /* FVCS */;
pub const VSC_ADDR_BASE: c_uint = 0xE0030000;

pub const VSC_MAINSTEPPING_VERSION_A: c_int = 0;

pub const VSC_SUBSTEPPING_VERSION_0: c_int = 0;
pub const VSC_SUBSTEPPING_VERSION_1: c_int = 2;

pub const VSC_SKU_CFG_LOCATION: c_uint = 0x5001A000;

pub const VSC_ACE_IMG_CNT: c_int = 2;
pub const VSC_CSI_IMG_CNT: c_int = 4;
pub const VSC_IMG_CNT_MAX: c_int = 6;

pub const VSC_IMAGE_PATH_MAX_LEN: c_int = 64;
pub const VSC_SENSOR_NAME_MAX_LEN: c_int = 16;
// command id
    enum {
    VSC_CMD_QUERY = 0,
    VSC_CMD_DL_SET = 1,
    VSC_CMD_DL_START = 2,
    VSC_CMD_DL_CONT = 3,
    VSC_CMD_DUMP_MEM = 4,
    VSC_CMD_GET_CONT = 8,
    VSC_CMD_CAM_BOOT = 10,
    };
// command ack token
    enum {
    VSC_TOKEN_BOOTLOADER_REQ = 1,
    VSC_TOKEN_DUMP_RESP = 4,
    VSC_TOKEN_ERROR = 7,
    };
// image type
    enum {
    VSC_IMG_BOOTLOADER_TYPE = 1,
    VSC_IMG_CSI_EM7D_TYPE,
    VSC_IMG_CSI_SEM_TYPE,
    VSC_IMG_CSI_RUNTIME_TYPE,
    VSC_IMG_ACE_VISION_TYPE,
    VSC_IMG_ACE_CFG_TYPE,
    VSC_IMG_SKU_CFG_TYPE,
    };
// image fragments
    enum {
    VSC_IMG_BOOTLOADER_FRAG,
    VSC_IMG_CSI_SEM_FRAG,
    VSC_IMG_CSI_RUNTIME_FRAG,
    VSC_IMG_ACE_VISION_FRAG,
    VSC_IMG_ACE_CFG_FRAG,
    VSC_IMG_CSI_EM7D_FRAG,
    VSC_IMG_SKU_CFG_FRAG,
    VSC_IMG_FRAG_MAX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc_rom_cmd {
    pub magic: __le32,
    pub cmd_id: __u8,
    union {
// download start
    struct {
    pub img_type: __u8,
    pub option: __le16,
    pub img_len: __le32,
    pub img_loc: __le32,
    pub crc: __le32,
    pub res): DECLARE_FLEX_ARRAY(__u8,,
    pub dl_start: } __packed,
// download set
    struct {
    pub option: __u8,
    pub img_cnt: __le16,
    pub payload): DECLARE_FLEX_ARRAY(__le32,,
    pub dl_set: } __packed,
// download continue
    struct {
    pub end_flag: __u8,
    pub len: __le16,
// 8 is the offset of payload
    pub 8]: __u8 payload[VSC_ROM_PKG_SIZE -,
    pub dl_cont: } __packed,
// dump memory
    struct {
    pub res: __u8,
    pub len: __le16,
    pub addr: __le32,
    pub payload): DECLARE_FLEX_ARRAY(__u8,,
    pub dump_mem: } __packed,
// 5 is the offset of padding
    pub 5]: __u8 padding[VSC_ROM_PKG_SIZE -,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc_rom_cmd_ack {
    pub magic: __le32,
    pub token: __u8,
    pub type: __u8,
    pub res: [__u8; 2],
    pub payload: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc_fw_cmd {
    pub magic: __le32,
    pub cmd_id: __u8,
    union {
    struct {
    pub option: __le16,
    pub img_type: __u8,
    pub img_len: __le32,
    pub img_loc: __le32,
    pub crc: __le32,
    pub res): DECLARE_FLEX_ARRAY(__u8,,
    pub dl_start: } __packed,
    struct {
    pub option: __le16,
    pub img_cnt: __u8,
    pub payload): DECLARE_FLEX_ARRAY(__le32,,
    pub dl_set: } __packed,
    struct {
    pub addr: __le32,
    pub len: __u8,
    pub payload): DECLARE_FLEX_ARRAY(__u8,,
    pub dump_mem: } __packed,
    struct {
    pub resv: [__u8; 3],
    pub crc: __le32,
    pub payload): DECLARE_FLEX_ARRAY(__u8,,
    pub boot: } __packed,
// 5 is the offset of padding
    pub 5]: __u8 padding[VSC_FW_PKG_SIZE -,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc_img {
    pub magic: __le32,
    pub option: __le32,
    pub image_count: __le32,
    pub image_location: [__le32; VSC_IMG_CNT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc_fw_sign {
    pub magic: __le32,
    pub image_size: __le32,
    pub image: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc_image_code_data {
// fragment index
    pub frag_index: u8,
// image type
    pub image_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc_img_frag {
    pub type: u8,
    pub location: u32,
    pub data: *const u8,
    pub size: u32,
}

//
// struct vsc_fw_loader - represent vsc firmware loader
// @dev: device used to request firmware
// @tp: transport layer used with the firmware loader
// @csi: CSI image
// @ace: ACE image
// @cfg: config image
// @tx_buf: tx buffer
// @rx_buf: rx buffer
// @option: command option
// @count: total image count
// @sensor_name: camera sensor name
// @frags: image fragments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsc_fw_loader {
    pub dev: *mut device,
    pub tp: *mut vsc_tp,
    pub csi: *const firmware,
    pub ace: *const firmware,
    pub cfg: *const firmware,
    pub tx_buf: *mut c_void,
    pub rx_buf: *mut c_void,
    pub option: u16,
    pub count: u16,
    pub sensor_name: [c_char; VSC_SENSOR_NAME_MAX_LEN],
    pub frags: [vsc_img_frag; VSC_IMG_FRAG_MAX],
}

#[no_mangle]
pub unsafe extern "C" fn vsc_sum_crc(data: *mut c_void, size: usize) -> u32 {
    static inline u32 vsc_sum_crc(void *data, size_t size)
    {
    let mut crc: u32 = 0;
    size_t i;
    for (i = 0; i < size; i++)
    crc += *((u8 *)data + i);
    return crc;
    }
// get sensor name to construct image name
    static int vsc_get_sensor_name(struct vsc_fw_loader *fw_loader,
    struct device *dev)
    {
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER };
    union acpi_object obj = {
    .integer.type = ACPI_TYPE_INTEGER,
    .integer.value = 1,
    };
    struct acpi_object_list arg_list = {
    .count = 1,
    .pointer = &obj,
    };
    union acpi_object *ret_obj;
    acpi_handle handle;
    acpi_status status;
    let mut ret: c_int = 0;
    handle = ACPI_HANDLE(dev);
    if (!handle)
    return -EINVAL;
    status = acpi_evaluate_object(handle, "SID", &arg_list, &buffer);
    if (ACPI_FAILURE(status)) {
    dev_err(dev, "can't evaluate SID method: %d\n", status);
    return -ENODEV;
    }
    ret_obj = buffer.pointer;
    if (!ret_obj) {
    dev_err(dev, "can't locate ACPI buffer\n");
    return -ENODEV;
    }
    if (ret_obj.type != ACPI_TYPE_STRING) {
    dev_err(dev, "found non-string entry\n");
    ret = -ENODEV;
    goto out_free_buff;
    }
// string length excludes trailing NUL
    if (ret_obj.string.length >= sizeof(fw_loader.sensor_name)) {
    dev_err(dev, "sensor name buffer too small\n");
    ret = -EINVAL;
    goto out_free_buff;
    }
    memcpy(fw_loader.sensor_name, ret_obj.string.pointer,
    ret_obj.string.length);
    string_lower(fw_loader.sensor_name, fw_loader.sensor_name);
    out_free_buff:
    ACPI_FREE(buffer.pointer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vsc_identify_silicon(fw_loader: *mut vsc_fw_loader) -> c_int {
    static int vsc_identify_silicon(struct vsc_fw_loader *fw_loader)
    {
    struct vsc_rom_cmd_ack *ack = fw_loader.rx_buf;
    struct vsc_rom_cmd *cmd = fw_loader.tx_buf;
    u8 version, sub_version;
    int ret;
// identify stepping information
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_DUMP_MEM;
    cmd.data.dump_mem.addr = cpu_to_le32(VSC_EFUSE_ADDR);
    cmd.data.dump_mem.len = cpu_to_le16(sizeof(__le32));
    ret = vsc_tp_rom_xfer(fw_loader.tp, cmd, ack, VSC_ROM_PKG_SIZE);
    if (ret || ack.token == VSC_TOKEN_ERROR) {
    dev_err(fw_loader.dev, "CMD_DUMP_MEM error %d token %d\n", ret, ack.token);
    return ret ?: -EINVAL;
    }
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_GET_CONT;
    ret = vsc_tp_rom_xfer(fw_loader.tp, cmd, ack, VSC_ROM_PKG_SIZE);
    if (ret || ack.token != VSC_TOKEN_DUMP_RESP) {
    dev_err(fw_loader.dev, "CMD_GETCONT error %d token %d\n", ret, ack.token);
    return ret ?: -EINVAL;
    }
    version = FIELD_GET(VSC_MAINSTEPPING_VERSION_MASK, ack.payload[0]);
    sub_version = FIELD_GET(VSC_SUBSTEPPING_VERSION_MASK, ack.payload[0]);
    if (version != VSC_MAINSTEPPING_VERSION_A) {
    dev_err(fw_loader.dev, "mainstepping mismatch expected %d got %d\n",
    VSC_MAINSTEPPING_VERSION_A, version);
    return -EINVAL;
    }
    if (sub_version != VSC_SUBSTEPPING_VERSION_0 &&
    sub_version != VSC_SUBSTEPPING_VERSION_1) {
    dev_err(fw_loader.dev, "substepping %d is out of supported range %d - %d\n",
    sub_version, VSC_SUBSTEPPING_VERSION_0, VSC_SUBSTEPPING_VERSION_1);
    return -EINVAL;
    }
    dev_info(fw_loader.dev, "silicon stepping version is %u:%u\n",
    version, sub_version);
// identify strap information
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_DUMP_MEM;
    cmd.data.dump_mem.addr = cpu_to_le32(VSC_STRAP_ADDR);
    cmd.data.dump_mem.len = cpu_to_le16(sizeof(__le32));
    ret = vsc_tp_rom_xfer(fw_loader.tp, cmd, ack, VSC_ROM_PKG_SIZE);
    if (ret)
    return ret;
    if (ack.token == VSC_TOKEN_ERROR)
    return -EINVAL;
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_GET_CONT;
    ret = vsc_tp_rom_xfer(fw_loader.tp, cmd, ack, VSC_ROM_PKG_SIZE);
    if (ret)
    return ret;
    if (ack.token != VSC_TOKEN_DUMP_RESP)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vsc_identify_csi_image(fw_loader: *mut vsc_fw_loader) -> c_int {
    static int vsc_identify_csi_image(struct vsc_fw_loader *fw_loader)
    {
    const struct firmware *image;
    struct vsc_fw_sign *sign;
    struct vsc_img *img;
    unsigned int i;
    int ret;
    ret = request_firmware(&image, VSC_CSI_IMAGE_NAME, fw_loader.dev);
    if (ret)
    return ret;
    img = (struct vsc_img *)image.data;
    if (!img) {
    ret = -ENOENT;
    goto err_release_image;
    }
    if (le32_to_cpu(img.magic) != VSC_MAGIC_FILE) {
    ret = -EINVAL;
    goto err_release_image;
    }
    if (le32_to_cpu(img.image_count) != VSC_CSI_IMG_CNT) {
    ret = -EINVAL;
    goto err_release_image;
    }
    fw_loader.count += le32_to_cpu(img.image_count) - 1;
    fw_loader.option =
    FIELD_GET(VSC_BOOT_IMG_OPTION_MASK, le32_to_cpu(img.option));
    sign = (struct vsc_fw_sign *)
    (img.image_location + le32_to_cpu(img.image_count));
    for (i = 0; i < VSC_CSI_IMG_CNT; i++) {
// mapping from CSI image index to image code data
    static const struct vsc_image_code_data csi_image_map[] = {
    { VSC_IMG_BOOTLOADER_FRAG, VSC_IMG_BOOTLOADER_TYPE },
    { VSC_IMG_CSI_SEM_FRAG, VSC_IMG_CSI_SEM_TYPE },
    { VSC_IMG_CSI_RUNTIME_FRAG, VSC_IMG_CSI_RUNTIME_TYPE },
    { VSC_IMG_CSI_EM7D_FRAG, VSC_IMG_CSI_EM7D_TYPE },
    };
    struct vsc_img_frag *frag;
    if ((u8 *)sign + sizeof(*sign) > image.data + image.size) {
    ret = -EINVAL;
    goto err_release_image;
    }
    if (le32_to_cpu(sign.magic) != VSC_MAGIC_FW) {
    ret = -EINVAL;
    goto err_release_image;
    }
    if (!le32_to_cpu(img.image_location[i])) {
    ret = -EINVAL;
    goto err_release_image;
    }
    frag = &fw_loader.frags[csi_image_map[i].frag_index];
    frag.data = sign.image;
    frag.size = le32_to_cpu(sign.image_size);
    frag.location = le32_to_cpu(img.image_location[i]);
    frag.type = csi_image_map[i].image_type;
    sign = (struct vsc_fw_sign *)
    (sign.image + le32_to_cpu(sign.image_size));
    }
    fw_loader.csi = image;
    return 0;
    err_release_image:
    release_firmware(image);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vsc_identify_ace_image(fw_loader: *mut vsc_fw_loader) -> c_int {
    static int vsc_identify_ace_image(struct vsc_fw_loader *fw_loader)
    {
    char path[VSC_IMAGE_PATH_MAX_LEN];
    const struct firmware *image;
    struct vsc_fw_sign *sign;
    struct vsc_img *img;
    unsigned int i;
    int ret;
    snprintf(path, sizeof(path), VSC_ACE_IMAGE_NAME_FMT,
    fw_loader.sensor_name);
    ret = request_firmware(&image, path, fw_loader.dev);
    if (ret)
    return ret;
    img = (struct vsc_img *)image.data;
    if (!img) {
    ret = -ENOENT;
    goto err_release_image;
    }
    if (le32_to_cpu(img.magic) != VSC_MAGIC_FILE) {
    ret = -EINVAL;
    goto err_release_image;
    }
    if (le32_to_cpu(img.image_count) != VSC_ACE_IMG_CNT) {
    ret = -EINVAL;
    goto err_release_image;
    }
    fw_loader.count += le32_to_cpu(img.image_count);
    sign = (struct vsc_fw_sign *)
    (img.image_location + le32_to_cpu(img.image_count));
    for (i = 0; i < VSC_ACE_IMG_CNT; i++) {
// mapping from ACE image index to image code data
    static const struct vsc_image_code_data ace_image_map[] = {
    { VSC_IMG_ACE_VISION_FRAG, VSC_IMG_ACE_VISION_TYPE },
    { VSC_IMG_ACE_CFG_FRAG, VSC_IMG_ACE_CFG_TYPE },
    };
    struct vsc_img_frag *frag, *last_frag;
    u8 frag_index;
    if ((u8 *)sign + sizeof(*sign) > image.data + image.size) {
    ret = -EINVAL;
    goto err_release_image;
    }
    if (le32_to_cpu(sign.magic) != VSC_MAGIC_FW) {
    ret = -EINVAL;
    goto err_release_image;
    }
    frag_index = ace_image_map[i].frag_index;
    frag = &fw_loader.frags[frag_index];
    frag.data = sign.image;
    frag.size = le32_to_cpu(sign.image_size);
    frag.location = le32_to_cpu(img.image_location[i]);
    frag.type = ace_image_map[i].image_type;
    if (!frag.location) {
    last_frag = &fw_loader.frags[frag_index - 1];
    frag.location =
    ALIGN(last_frag.location + last_frag.size, SZ_4K);
    }
    sign = (struct vsc_fw_sign *)
    (sign.image + le32_to_cpu(sign.image_size));
    }
    fw_loader.ace = image;
    return 0;
    err_release_image:
    release_firmware(image);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vsc_identify_cfg_image(fw_loader: *mut vsc_fw_loader) -> c_int {
    static int vsc_identify_cfg_image(struct vsc_fw_loader *fw_loader)
    {
    struct vsc_img_frag *frag = &fw_loader.frags[VSC_IMG_SKU_CFG_FRAG];
    char path[VSC_IMAGE_PATH_MAX_LEN];
    const struct firmware *image;
    u32 size;
    int ret;
    snprintf(path, sizeof(path), VSC_CFG_IMAGE_NAME_FMT,
    fw_loader.sensor_name);
    ret = request_firmware(&image, path, fw_loader.dev);
    if (ret)
    return ret;
// identify image size
    if (image.size <= sizeof(u32) || image.size > VSC_SKU_MAX_SIZE) {
    ret = -EINVAL;
    goto err_release_image;
    }
    size = le32_to_cpu(*((__le32 *)image.data)) + sizeof(u32);
    if (image.size != size) {
    ret = -EINVAL;
    goto err_release_image;
    }
    frag.data = image.data;
    frag.size = image.size;
    frag.type = VSC_IMG_SKU_CFG_TYPE;
    frag.location = VSC_SKU_CFG_LOCATION;
    fw_loader.cfg = image;
    return 0;
    err_release_image:
    release_firmware(image);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vsc_download_bootloader(fw_loader: *mut vsc_fw_loader) -> c_int {
    static int vsc_download_bootloader(struct vsc_fw_loader *fw_loader)
    {
    struct vsc_img_frag *frag = &fw_loader.frags[VSC_IMG_BOOTLOADER_FRAG];
    struct vsc_rom_cmd_ack *ack = fw_loader.rx_buf;
    struct vsc_rom_cmd *cmd = fw_loader.tx_buf;
    u32 len, c_len;
    size_t remain;
    const u8 *p;
    int ret;
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_QUERY;
    ret = vsc_tp_rom_xfer(fw_loader.tp, cmd, ack, VSC_ROM_PKG_SIZE);
    if (ret)
    return ret;
    if (ack.token != VSC_TOKEN_DUMP_RESP &&
    ack.token != VSC_TOKEN_BOOTLOADER_REQ)
    return -EINVAL;
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_DL_START;
    cmd.data.dl_start.option = cpu_to_le16(fw_loader.option);
    cmd.data.dl_start.img_type = frag.type;
    cmd.data.dl_start.img_len = cpu_to_le32(frag.size);
    cmd.data.dl_start.img_loc = cpu_to_le32(frag.location);
    c_len = offsetof(struct vsc_rom_cmd, data.dl_start.crc);
    cmd.data.dl_start.crc = cpu_to_le32(vsc_sum_crc(cmd, c_len));
    ret = vsc_tp_rom_xfer(fw_loader.tp, cmd, core::ptr::null_mut(), VSC_ROM_PKG_SIZE);
    if (ret)
    return ret;
    p = frag.data;
    remain = frag.size;
// download image data
    while (remain > 0) {
    len = min(remain, sizeof(cmd.data.dl_cont.payload));
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_DL_CONT;
    cmd.data.dl_cont.len = cpu_to_le16(len);
    cmd.data.dl_cont.end_flag = remain == len;
    memcpy(cmd.data.dl_cont.payload, p, len);
    ret = vsc_tp_rom_xfer(fw_loader.tp, cmd, core::ptr::null_mut(), VSC_ROM_PKG_SIZE);
    if (ret)
    return ret;
    p += len;
    remain -= len;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vsc_download_firmware(fw_loader: *mut vsc_fw_loader) -> c_int {
    static int vsc_download_firmware(struct vsc_fw_loader *fw_loader)
    {
    struct vsc_fw_cmd *cmd = fw_loader.tx_buf;
    unsigned int i, index = 0;
    u32 c_len;
    int ret;
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_DL_SET;
    cmd.data.dl_set.img_cnt = cpu_to_le16(fw_loader.count);
    put_unaligned_le16(fw_loader.option, &cmd.data.dl_set.option);
    for (i = VSC_IMG_CSI_SEM_FRAG; i <= VSC_IMG_CSI_EM7D_FRAG; i++) {
    struct vsc_img_frag *frag = &fw_loader.frags[i];
    cmd.data.dl_set.payload[index++] = cpu_to_le32(frag.location);
    cmd.data.dl_set.payload[index++] = cpu_to_le32(frag.size);
    }
    c_len = offsetof(struct vsc_fw_cmd, data.dl_set.payload[index]);
    cmd.data.dl_set.payload[index] = cpu_to_le32(vsc_sum_crc(cmd, c_len));
    ret = vsc_tp_rom_xfer(fw_loader.tp, cmd, core::ptr::null_mut(), VSC_FW_PKG_SIZE);
    if (ret)
    return ret;
    for (i = VSC_IMG_CSI_SEM_FRAG; i < VSC_IMG_FRAG_MAX; i++) {
    struct vsc_img_frag *frag = &fw_loader.frags[i];
    const u8 *p;
    u32 remain;
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_DL_START;
    cmd.data.dl_start.img_type = frag.type;
    cmd.data.dl_start.img_len = cpu_to_le32(frag.size);
    cmd.data.dl_start.img_loc = cpu_to_le32(frag.location);
    put_unaligned_le16(fw_loader.option, &cmd.data.dl_start.option);
    c_len = offsetof(struct vsc_fw_cmd, data.dl_start.crc);
    cmd.data.dl_start.crc = cpu_to_le32(vsc_sum_crc(cmd, c_len));
    ret = vsc_tp_rom_xfer(fw_loader.tp, cmd, core::ptr::null_mut(), VSC_FW_PKG_SIZE);
    if (ret)
    return ret;
    p = frag.data;
    remain = frag.size;
// download image data
    while (remain > 0) {
    let mut len: u32 = min(remain, VSC_FW_PKG_SIZE);
    memcpy(fw_loader.tx_buf, p, len);
    memset(fw_loader.tx_buf + len, 0, VSC_FW_PKG_SIZE - len);
    ret = vsc_tp_rom_xfer(fw_loader.tp, fw_loader.tx_buf,
    core::ptr::null_mut(), VSC_FW_PKG_SIZE);
    if (ret)
    break;
    p += len;
    remain -= len;
    }
    }
    cmd.magic = cpu_to_le32(VSC_MAGIC_NUM);
    cmd.cmd_id = VSC_CMD_CAM_BOOT;
    c_len = offsetof(struct vsc_fw_cmd, data.dl_start.crc);
    cmd.data.boot.crc = cpu_to_le32(vsc_sum_crc(cmd, c_len));
    return vsc_tp_rom_xfer(fw_loader.tp, cmd, core::ptr::null_mut(), VSC_FW_PKG_SIZE);
    }
//
// vsc_tp_init - init vsc_tp
// @tp: vsc_tp device handle
// @dev: device node for mei vsc device
// Return: 0 in case of success, negative value in case of error
//
#[no_mangle]
pub unsafe extern "C" fn vsc_tp_init(tp: *mut vsc_tp, dev: *mut device) -> c_int {
    int vsc_tp_init(struct vsc_tp *tp, struct device *dev)
    {
    struct vsc_fw_loader *fw_loader __free(kfree) = core::ptr::null_mut();
    void *tx_buf __free(kfree) = core::ptr::null_mut();
    void *rx_buf __free(kfree) = core::ptr::null_mut();
    int ret;
    fw_loader = kzalloc_obj(*fw_loader);
    if (!fw_loader)
    return -ENOMEM;
    tx_buf = kzalloc(VSC_FW_PKG_SIZE, GFP_KERNEL);
    if (!tx_buf)
    return -ENOMEM;
    rx_buf = kzalloc(VSC_FW_PKG_SIZE, GFP_KERNEL);
    if (!rx_buf)
    return -ENOMEM;
    fw_loader.tx_buf = tx_buf;
    fw_loader.rx_buf = rx_buf;
    fw_loader.tp = tp;
    fw_loader.dev = dev;
    ret = vsc_get_sensor_name(fw_loader, dev);
    if (ret)
    return ret;
    ret = vsc_identify_silicon(fw_loader);
    if (ret)
    return ret;
    ret = vsc_identify_csi_image(fw_loader);
    if (ret)
    return ret;
    ret = vsc_identify_ace_image(fw_loader);
    if (ret)
    goto err_release_csi;
    ret = vsc_identify_cfg_image(fw_loader);
    if (ret)
    goto err_release_ace;
    ret = vsc_download_bootloader(fw_loader);
    if (!ret)
    ret = vsc_download_firmware(fw_loader);
    release_firmware(fw_loader.cfg);
    err_release_ace:
    release_firmware(fw_loader.ace);
    err_release_csi:
    release_firmware(fw_loader.csi);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(vsc_tp_init, "VSC_TP");
