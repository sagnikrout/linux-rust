//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gvt/firmware.c
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


//
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Zhi Wang <zhi.a.wang@intel.com>
//
// Contributors:
// Changbin Du <changbin.du@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gvt_firmware_header {
    pub magic: u64,
    pub /: *mut *mut u32 crc32; / protect the data after this field,
    pub version: u32,
    pub cfg_space_size: u64,
    pub /: *mut *mut u64 cfg_space_offset; / offset in the file,
    pub mmio_size: u64,
    pub /: *mut *mut u64 mmio_offset; / offset in the file,
    pub data: [c_uchar; ],
}

    static BIN_ATTR_SIMPLE_ADMIN_RO(gvt_firmware);
#[no_mangle]
unsafe extern "C" fn expose_firmware_sysfs(gvt: *mut intel_gvt) -> c_int {
    static int expose_firmware_sysfs(struct intel_gvt *gvt)
    {
    struct intel_gvt_device_info *info = &gvt.device_info;
    struct drm_i915_private *i915 = gvt.gt.i915;
    struct pci_dev *pdev = to_pci_dev(i915.drm.dev);
    struct gvt_firmware_header *h;
    void *firmware;
    void *p;
    unsigned long size, crc32_start;
    int ret;
    size = offsetof(struct gvt_firmware_header, data) + info.mmio_size + info.cfg_space_size;
    firmware = vzalloc(size);
    if (!firmware)
    return -ENOMEM;
    h = firmware;
    h.magic = VGT_MAGIC;
    h.version = FIRMWARE_VERSION;
    h.cfg_space_size = info.cfg_space_size;
    h.cfg_space_offset = offsetof(struct gvt_firmware_header, data);
    h.mmio_size = info.mmio_size;
    h.mmio_offset = h.cfg_space_offset + h.cfg_space_size;
    p = firmware + h.cfg_space_offset;
    memcpy(gvt.firmware.cfg_space, i915.vgpu.initial_cfg_space,
    info.cfg_space_size);
    memcpy(p, gvt.firmware.cfg_space, info.cfg_space_size);
    p = firmware + h.mmio_offset;
    memcpy(gvt.firmware.mmio, i915.vgpu.initial_mmio,
    info.mmio_size);
    memcpy(p, gvt.firmware.mmio, info.mmio_size);
    crc32_start = offsetof(struct gvt_firmware_header, version);
    h.crc32 = crc32_le(0, firmware + crc32_start, size - crc32_start);
    bin_attr_gvt_firmware.size = size;
    bin_attr_gvt_firmware.private = firmware;
    ret = device_create_bin_file(&pdev.dev, &bin_attr_gvt_firmware);
    if (ret) {
    vfree(firmware);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clean_firmware_sysfs(gvt: *mut intel_gvt) {
    static void clean_firmware_sysfs(struct intel_gvt *gvt)
    {
    struct pci_dev *pdev = to_pci_dev(gvt.gt.i915.drm.dev);
    device_remove_bin_file(&pdev.dev, &bin_attr_gvt_firmware);
    vfree(bin_attr_gvt_firmware.private);
    }
//
// intel_gvt_free_firmware - free GVT firmware
// @gvt: intel gvt device
//
#[no_mangle]
pub unsafe extern "C" fn intel_gvt_free_firmware(gvt: *mut intel_gvt) {
    void intel_gvt_free_firmware(struct intel_gvt *gvt)
    {
    if (!gvt.firmware.firmware_loaded)
    clean_firmware_sysfs(gvt);
    kfree(gvt.firmware.cfg_space);
    vfree(gvt.firmware.mmio);
    }
    static int verify_firmware(struct intel_gvt *gvt,
    const struct firmware *fw)
    {
    struct intel_gvt_device_info *info = &gvt.device_info;
    struct pci_dev *pdev = to_pci_dev(gvt.gt.i915.drm.dev);
    struct gvt_firmware_header *h;
    unsigned long id, crc32_start;
    const void *mem;
    const char *item;
    u64 file, request;
    h = (struct gvt_firmware_header *)fw.data;
    crc32_start = offsetofend(struct gvt_firmware_header, crc32);
    mem = fw.data + crc32_start;

    item = (s); file = (u64)(a); request = (u64)(b); \
    if ((a) != (b)) \
    goto invalid_firmware; \
    } while (0)
    VERIFY("magic number", h.magic, VGT_MAGIC);
    VERIFY("version", h.version, FIRMWARE_VERSION);
    VERIFY("crc32", h.crc32, crc32_le(0, mem, fw.size - crc32_start));
    VERIFY("cfg space size", h.cfg_space_size, info.cfg_space_size);
    VERIFY("mmio size", h.mmio_size, info.mmio_size);
    mem = (fw.data + h.cfg_space_offset);
    id = *(u16 *)(mem + PCI_VENDOR_ID);
    VERIFY("vendor id", id, pdev.vendor);
    id = *(u16 *)(mem + PCI_DEVICE_ID);
    VERIFY("device id", id, pdev.device);
    id = *(u8 *)(mem + PCI_REVISION_ID);
    VERIFY("revision id", id, pdev.revision);

    return 0;
    invalid_firmware:
    gvt_dbg_core("Invalid firmware: %s [file] 0x%llx [request] 0x%llx\n",
    item, file, request);
    return -EINVAL;
    }

//
// intel_gvt_load_firmware - load GVT firmware
// @gvt: intel gvt device
//
#[no_mangle]
pub unsafe extern "C" fn intel_gvt_load_firmware(gvt: *mut intel_gvt) -> c_int {
    int intel_gvt_load_firmware(struct intel_gvt *gvt)
    {
    struct intel_gvt_device_info *info = &gvt.device_info;
    struct pci_dev *pdev = to_pci_dev(gvt.gt.i915.drm.dev);
    struct intel_gvt_firmware *firmware = &gvt.firmware;
    struct gvt_firmware_header *h;
    const struct firmware *fw;
    char *path;
    void *mem;
    int ret;
    path = kmalloc(PATH_MAX, GFP_KERNEL);
    if (!path)
    return -ENOMEM;
    mem = kmalloc(info.cfg_space_size, GFP_KERNEL);
    if (!mem) {
    kfree(path);
    return -ENOMEM;
    }
    firmware.cfg_space = mem;
    mem = vmalloc(info.mmio_size);
    if (!mem) {
    kfree(path);
    kfree(firmware.cfg_space);
    return -ENOMEM;
    }
    firmware.mmio = mem;
    sprintf(path, "%s/vid_0x%04x_did_0x%04x_rid_0x%02x.golden_hw_state",
    GVT_FIRMWARE_PATH, pdev.vendor, pdev.device,
    pdev.revision);
    gvt_dbg_core("request hw state firmware %s...\n", path);
    ret = request_firmware(&fw, path, gvt.gt.i915.drm.dev);
    kfree(path);
    if (ret)
    goto expose_firmware;
    gvt_dbg_core("success.\n");
    ret = verify_firmware(gvt, fw);
    if (ret)
    goto out_free_fw;
    gvt_dbg_core("verified.\n");
    h = (struct gvt_firmware_header *)fw.data;
    memcpy(firmware.cfg_space, fw.data + h.cfg_space_offset,
    h.cfg_space_size);
    memcpy(firmware.mmio, fw.data + h.mmio_offset,
    h.mmio_size);
    release_firmware(fw);
    firmware.firmware_loaded = true;
    return 0;
    out_free_fw:
    release_firmware(fw);
    expose_firmware:
    expose_firmware_sysfs(gvt);
    return 0;
    }
