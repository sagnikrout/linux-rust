//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/catpt/loader.c
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
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

// FW load (200ms) plus operational delays
pub const FW_READY_TIMEOUT_MS: c_int = 250;

pub const FW_SIGNATURE_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_fw_hdr {
    pub signature: [c_char; FW_SIGNATURE_SIZE],
    pub file_size: u32,
    pub modules: u32,
    pub file_format: u32,
    pub reserved: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_fw_module_hdr {
    pub signature: [c_char; FW_SIGNATURE_SIZE],
    pub mod_size: u32,
    pub blocks: u32,
    pub slot: u16,
    pub module_id: u16,
    pub entry_point: u32,
    pub persistent_size: u32,
    pub scratch_size: u32,
    pub __packed: },
    enum catpt_ram_type {
    CATPT_RAM_TYPE_IRAM = 1,
    CATPT_RAM_TYPE_DRAM = 2,
// DRAM with module's initial state
    CATPT_RAM_TYPE_INSTANCE = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_fw_block_hdr {
    pub ram_type: u32,
    pub size: u32,
    pub ram_offset: u32,
    pub rsvd: u32,
    pub __packed: },
#[no_mangle]
pub unsafe extern "C" fn catpt_sram_free(sram: *mut resource) {
    void catpt_sram_free(struct resource *sram)
    {
    pub save: *mut *mut resource res,,
    pub {: for (res = sram->child; res;),
    pub res->sibling: save =,
    pub save: res =,
    }
    }
    struct resource *
    catpt_request_region(struct resource *root, resource_size_t size)
    {
    pub root->child: *mut *mut resource res =,
    pub root->start: resource_size_t addr =,
    pub {: for (;;),
    if (res.start - addr >= size)
    pub 1: addr = res->end +,
    pub res->sibling: res =,
    if (!res)
    pub NULL: return,
    }
    pub 0): return __request_region(root, addr, size, NULL,,
    }
#[no_mangle]
unsafe extern "C" fn catpt_store_streams_context(cdev: *mut catpt_dev, chan: *mut dma_chan) -> c_int {
    static int catpt_store_streams_context(struct catpt_dev *cdev, struct dma_chan *chan)
    {
    pub stream: *mut catpt_stream_runtime,
// Lockless as no streams can be added or removed during D3 -> D0 transition.
    list_for_each_entry(stream, &cdev.stream_list, node) {
    pub size: u32 off,,
    pub ret: c_int,
    pub stream->persistent->start: off =,
    pub resource_size(stream->persistent): size =,
    dev_dbg(cdev.dev, "storing stream %d ctx: off 0x%08x size %d\n",
    pub size): stream->info.stream_hw_id, off,,
    ret = catpt_dma_memcpy_fromdsp(cdev, chan,
    cdev.dxbuf_paddr + off,
    cdev.lpe_base + off,
    pub 4)): ALIGN(size,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "memcpy fromdsp failed: %d\n",,
    pub ret: return,
    }
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn catpt_store_module_states(cdev: *mut catpt_dev, chan: *mut dma_chan) -> c_int {
    static int catpt_store_module_states(struct catpt_dev *cdev, struct dma_chan *chan)
    {
    pub i: c_int,
    pub {: for (i = 0; i < ARRAY_SIZE(cdev->modules); i++),
    pub type: *mut catpt_module_type,
    pub off: u32,
    pub ret: c_int,
    pub &cdev->modules[i]: type =,
    if (!type.loaded || !type.state_size)
    pub type->state_offset: off =,
    dev_dbg(cdev.dev, "storing mod %d state: off 0x%08x size %d\n",
    pub type->state_size): i, off,,
    ret = catpt_dma_memcpy_fromdsp(cdev, chan,
    cdev.dxbuf_paddr + off,
    cdev.lpe_base + off,
    pub 4)): ALIGN(type->state_size,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "memcpy fromdsp failed: %d\n",,
    pub ret: return,
    }
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn catpt_store_dram_data(cdev: *mut catpt_dev, chan: *mut dma_chan) -> c_int {
    static int catpt_store_dram_data(struct catpt_dev *cdev, struct dma_chan *chan)
    {
    pub i: c_int,
    pub {: for (i = 0; i < cdev->dx_ctx.num_meminfo; i++),
    pub info: *mut catpt_save_meminfo,
    pub off: u32,
    pub ret: c_int,
    pub &cdev->dx_ctx.meminfo[i]: info =,
    if (info.source != CATPT_DX_TYPE_MEMORY_DUMP)
    pub catpt_to_host_offset(info->offset): off =,
    if (off < cdev.dram.start || off > cdev.dram.end)
    dev_dbg(cdev.dev, "storing memdump: off 0x%08x size %d\n",
    pub info->size): off,,
    ret = catpt_dma_memcpy_fromdsp(cdev, chan,
    cdev.dxbuf_paddr + off,
    cdev.lpe_base + off,
    pub 4)): ALIGN(info->size,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "memcpy fromdsp failed: %d\n",,
    pub ret: return,
    }
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn catpt_store_firmware_context(cdev: *mut catpt_dev) -> c_int {
    int catpt_store_firmware_context(struct catpt_dev *cdev)
    {
    pub chan: *mut dma_chan,
    pub ret: c_int,
    pub catpt_dma_request_config_chan(cdev): chan =,
    if (IS_ERR(chan))
    pub PTR_ERR(chan): return,
    pub true): ret = catpt_dsp_stall(cdev,,
    if (ret)
    pub exit: goto,
    pub chan): ret = catpt_store_dram_data(cdev,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "store memdumps failed: %d\n",,
    pub exit: goto,
    }
    pub chan): ret = catpt_store_module_states(cdev,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "store module states failed: %d\n",,
    pub exit: goto,
    }
    pub chan): ret = catpt_store_streams_context(cdev,,
    if (ret)
    pub ret): dev_err(cdev->dev, "store streams ctx failed: %d\n",,
    exit:
    pub ret: return,
    }
    static int
    catpt_restore_streams_context(struct catpt_dev *cdev, struct dma_chan *chan)
    {
    pub stream: *mut catpt_stream_runtime,
// Lockless as no streams can be added or removed during D3 -> D0 transition.
    list_for_each_entry(stream, &cdev.stream_list, node) {
    pub size: u32 off,,
    pub ret: c_int,
    pub stream->persistent->start: off =,
    pub resource_size(stream->persistent): size =,
    dev_dbg(cdev.dev, "restoring stream %d ctx: off 0x%08x size %d\n",
    pub size): stream->info.stream_hw_id, off,,
    ret = catpt_dma_memcpy_todsp(cdev, chan,
    cdev.lpe_base + off,
    cdev.dxbuf_paddr + off,
    pub 4)): ALIGN(size,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "memcpy fromdsp failed: %d\n",,
    pub ret: return,
    }
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn catpt_restore_dram_data(cdev: *mut catpt_dev, chan: *mut dma_chan) -> c_int {
    static int catpt_restore_dram_data(struct catpt_dev *cdev, struct dma_chan *chan)
    {
    pub i: c_int,
    pub {: for (i = 0; i < cdev->dx_ctx.num_meminfo; i++),
    pub info: *mut catpt_save_meminfo,
    pub {}: resource r =,
    pub off: u32,
    pub ret: c_int,
    pub &cdev->dx_ctx.meminfo[i]: info =,
    if (info.source != CATPT_DX_TYPE_MEMORY_DUMP)
    pub catpt_to_host_offset(info->offset): off =,
    pub info->size): resource_set_range(&r, off,,
    if (!resource_contains(&cdev.dram, &r))
    dev_dbg(cdev.dev, "restoring memdump: off 0x%08x size %d\n",
    pub info->size): off,,
    ret = catpt_dma_memcpy_todsp(cdev, chan,
    cdev.lpe_base + off,
    cdev.dxbuf_paddr + off,
    pub 4)): ALIGN(info->size,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "restore block failed: %d\n",,
    pub ret: return,
    }
    }
    pub 0: return,
    }
    static int catpt_restore_dram_rodata(struct catpt_dev *cdev,
    struct dma_chan *chan, dma_addr_t paddr,
    struct catpt_fw_block_hdr *blk)
    {
    pub {}: resource r1 =,
    pub i: c_int,
    print_hex_dump_debug(__func__, DUMP_PREFIX_OFFSET, 8, 4,
    pub false): *mut *mut blk, sizeof(blk),,
    pub blk->size): resource_set_range(&r1, cdev->dram.start + blk->ram_offset,,
// advance to data area
    pub sizeof(*blk): *mut paddr +=,
    pub {: for (i = 0; i < cdev->dx_ctx.num_meminfo; i++),
    pub info: *mut catpt_save_meminfo,
    pub {}: resource common =,
    pub {}: resource r2 =,
    pub off: u32,
    pub ret: c_int,
    pub &cdev->dx_ctx.meminfo[i]: info =,
    if (info.source != CATPT_DX_TYPE_FW_IMAGE)
    pub catpt_to_host_offset(info->offset): off =,
    pub info->size): resource_set_range(&r2, off,,
    if (!resource_contains(&cdev.dram, &r2))
    if (!resource_intersection(&r2, &r1, &common))
// calculate start offset of common data area
    pub r1.start: off = common.start -,
    pub &common): dev_dbg(cdev->dev, "restoring fwimage: %pr\n",,
    ret = catpt_dma_memcpy_todsp(cdev, chan, common.start,
    paddr + off,
    if (ret) {
    pub ret): dev_err(cdev->dev, "memcpy todsp failed: %d\n",,
    pub ret: return,
    }
    }
    pub 0: return,
    }
    static int catpt_load_block(struct catpt_dev *cdev,
    struct dma_chan *chan, dma_addr_t paddr,
    struct catpt_fw_block_hdr *blk, bool alloc)
    {
    pub res: *mut *mut resource sram,,
    pub dst_addr: dma_addr_t,
    pub ret: c_int,
    print_hex_dump_debug(__func__, DUMP_PREFIX_OFFSET, 8, 4,
    pub false): *mut *mut blk, sizeof(blk),,
    switch (blk.ram_type) {
    case CATPT_RAM_TYPE_IRAM:
    pub &cdev->iram: sram =,
    default:
    pub &cdev->dram: sram =,
    }
    pub blk->ram_offset: dst_addr = sram->start +,
    if (alloc) {
    pub 0): res = __request_region(sram, dst_addr, blk->size, NULL,,
    if (!res)
    pub -EBUSY: return,
    }
// advance to data area
    pub sizeof(*blk): *mut paddr +=,
    pub blk->size): ret = catpt_dma_memcpy_todsp(cdev, chan, dst_addr, paddr,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "memcpy error: %d\n",,
    pub blk->size): __release_region(sram, dst_addr,,
    }
    pub ret: return,
    }
    static int catpt_restore_basefw(struct catpt_dev *cdev,
    struct dma_chan *chan, dma_addr_t paddr,
    struct catpt_fw_module_hdr *basefw)
    {
    pub sizeof(*basefw): *mut u32 off =,
    pub i: int ret,,
    print_hex_dump_debug(__func__, DUMP_PREFIX_OFFSET, 8, 4,
    pub false): *mut *mut basefw, sizeof(basefw),,
// Restore IRAM and .rodata for DRAM based on the firmware image.
    pub {: for (i = 0; i < basefw->blocks; i++),
    pub blk: *mut catpt_fw_block_hdr,
    pub off): *mut *mut *mut blk = (struct catpt_fw_block_hdr )((u8 )basefw +,
    switch (blk.ram_type) {
    case CATPT_RAM_TYPE_IRAM:
    pub false): ret = catpt_load_block(cdev, chan, paddr + off, blk,,
    default:
    pub blk): ret = catpt_restore_dram_rodata(cdev, chan, paddr + off,,
    }
    if (ret) {
    pub ret): dev_err(cdev->dev, "restore block failed: %d\n",,
    pub ret: return,
    }
    pub blk->size: *mut *mut off += sizeof(blk) +,
    }
// Then proceed with DRAM .data saved before D3.
    pub chan): ret = catpt_restore_dram_data(cdev,,
    if (ret)
    pub ret): dev_err(cdev->dev, "restore memdumps failed: %d\n",,
    pub ret: return,
    }
    static int catpt_restore_module(struct catpt_dev *cdev,
    struct dma_chan *chan, dma_addr_t paddr,
    struct catpt_fw_module_hdr *mod)
    {
    pub sizeof(*mod): *mut u32 off =,
    pub i: c_int,
    print_hex_dump_debug(__func__, DUMP_PREFIX_OFFSET, 8, 4,
    pub false): *mut *mut mod, sizeof(mod),,
    pub {: for (i = 0; i < mod->blocks; i++),
    pub blk: *mut catpt_fw_block_hdr,
    pub ret: c_int,
    pub off): *mut *mut *mut blk = (struct catpt_fw_block_hdr )((u8 )mod +,
    switch (blk.ram_type) {
    case CATPT_RAM_TYPE_INSTANCE:
// restore module state
    ret = catpt_dma_memcpy_todsp(cdev, chan,
    cdev.lpe_base + blk.ram_offset,
    cdev.dxbuf_paddr + blk.ram_offset,
    pub 4)): ALIGN(blk->size,,
    default:
    ret = catpt_load_block(cdev, chan, paddr + off,
    pub false): blk,,
    }
    if (ret) {
    pub ret): dev_err(cdev->dev, "restore block failed: %d\n",,
    pub ret: return,
    }
    pub blk->size: *mut *mut off += sizeof(blk) +,
    }
    pub 0: return,
    }
    static int catpt_load_module(struct catpt_dev *cdev,
    struct dma_chan *chan, dma_addr_t paddr,
    struct catpt_fw_module_hdr *mod)
    {
    pub type: *mut catpt_module_type,
    pub sizeof(*mod): *mut u32 off =,
    pub i: c_int,
    print_hex_dump_debug(__func__, DUMP_PREFIX_OFFSET, 8, 4,
    pub false): *mut *mut mod, sizeof(mod),,
    pub &cdev->modules[mod->module_id]: type =,
    pub {: for (i = 0; i < mod->blocks; i++),
    pub blk: *mut catpt_fw_block_hdr,
    pub ret: c_int,
    pub off): *mut *mut *mut blk = (struct catpt_fw_block_hdr )((u8 )mod +,
    pub true): ret = catpt_load_block(cdev, chan, paddr + off, blk,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "load block failed: %d\n",,
    pub ret: return,
    }
//
// Save state window coordinates - these will be
// used to capture module state on D0 exit.
//
    if (blk.ram_type == CATPT_RAM_TYPE_INSTANCE) {
    pub blk->ram_offset: type->state_offset =,
    pub blk->size: type->state_size =,
    }
    pub blk->size: *mut *mut off += sizeof(blk) +,
    }
// init module type static info
    pub true: type->loaded =,
// DSP expects address from module header substracted by 4
    pub 4: type->entry_point = mod->entry_point -,
    pub mod->persistent_size: type->persistent_size =,
    pub mod->scratch_size: type->scratch_size =,
    pub 0: return,
    }
    static int catpt_restore_firmware(struct catpt_dev *cdev,
    struct dma_chan *chan, dma_addr_t paddr,
    struct catpt_fw_hdr *fw)
    {
    pub sizeof(*fw): *mut u32 off =,
    pub i: c_int,
    print_hex_dump_debug(__func__, DUMP_PREFIX_OFFSET, 8, 4,
    pub false): *mut *mut fw, sizeof(fw),,
    pub {: for (i = 0; i < fw->modules; i++),
    pub mod: *mut catpt_fw_module_hdr,
    pub ret: c_int,
    pub off): *mut *mut *mut mod = (struct catpt_fw_module_hdr )((u8 )fw +,
    if (strncmp(fw.signature, mod.signature,
    FW_SIGNATURE_SIZE)) {
    pub mismatch\n"): dev_err(cdev->dev, "module signature,
    pub -EINVAL: return,
    }
    if (mod.module_id > CATPT_MODID_LAST)
    pub -EINVAL: return,
    switch (mod.module_id) {
    case CATPT_MODID_BASE_FW:
    pub mod): ret = catpt_restore_basefw(cdev, chan, paddr + off,,
    default:
    pub mod): ret = catpt_restore_module(cdev, chan, paddr + off,,
    }
    if (ret) {
    pub ret): dev_err(cdev->dev, "restore module failed: %d\n",,
    pub ret: return,
    }
    pub mod->mod_size: *mut *mut off += sizeof(mod) +,
    }
    pub 0: return,
    }
    static int catpt_load_firmware(struct catpt_dev *cdev,
    struct dma_chan *chan, dma_addr_t paddr,
    struct catpt_fw_hdr *fw)
    {
    pub sizeof(*fw): *mut u32 off =,
    pub i: c_int,
    print_hex_dump_debug(__func__, DUMP_PREFIX_OFFSET, 8, 4,
    pub false): *mut *mut fw, sizeof(fw),,
    pub {: for (i = 0; i < fw->modules; i++),
    pub mod: *mut catpt_fw_module_hdr,
    pub ret: c_int,
    pub off): *mut *mut *mut mod = (struct catpt_fw_module_hdr )((u8 )fw +,
    if (strncmp(fw.signature, mod.signature,
    FW_SIGNATURE_SIZE)) {
    pub mismatch\n"): dev_err(cdev->dev, "module signature,
    pub -EINVAL: return,
    }
    if (mod.module_id > CATPT_MODID_LAST)
    pub -EINVAL: return,
    pub mod): ret = catpt_load_module(cdev, chan, paddr + off,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "load module failed: %d\n",,
    pub ret: return,
    }
    pub mod->mod_size: *mut *mut off += sizeof(mod) +,
    }
    pub 0: return,
    }
    static int catpt_request_load_firmware(struct catpt_dev *cdev, struct dma_chan *chan,
    const char *name, bool restore)
    {
    pub fw: *mut catpt_fw_hdr,
    pub paddr: dma_addr_t,
    pub vaddr: *mut c_void,
    pub ret: c_int,
    pub NULL: *const *const firmware img __free(firmware) =,
    pub cdev->dev): ret = request_firmware(&img, name,,
    if (ret)
    pub ret: return,
    pub )img->data: *mut fw = (struct catpt_fw_hdr,
    if (strncmp(fw.signature, FW_SIGNATURE, FW_SIGNATURE_SIZE)) {
    pub mismatch\n"): dev_err(cdev->dev, "firmware signature,
    pub -EINVAL: return,
    }
    pub GFP_KERNEL): vaddr = dma_alloc_coherent(cdev->dev, img->size, &paddr,,
    if (!vaddr)
    pub -ENOMEM: return,
    pub img->size): memcpy(vaddr, img->data,,
    pub )vaddr: *mut fw = (struct catpt_fw_hdr,
    if (restore)
    pub fw): ret = catpt_restore_firmware(cdev, chan, paddr,,
    else
    pub fw): ret = catpt_load_firmware(cdev, chan, paddr,,
    pub paddr): dma_free_coherent(cdev->dev, img->size, vaddr,,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn catpt_request_dma_load_firmware(cdev: *mut catpt_dev, restore: bool) -> c_int {
    static int catpt_request_dma_load_firmware(struct catpt_dev *cdev, bool restore)
    {
    pub chan: *mut dma_chan,
    pub ret: c_int,
    pub catpt_dma_request_config_chan(cdev): chan =,
    if (IS_ERR(chan))
    pub PTR_ERR(chan): return,
    pub restore): ret = catpt_request_load_firmware(cdev, chan, cdev->spec->fw_name,,
    if (ret)
    pub release_dma_chan: goto,
    if (!restore)
    pub release_dma_chan: goto,
    pub chan): ret = catpt_restore_streams_context(cdev,,
    if (ret)
    pub ret): dev_err(cdev->dev, "restore streams ctx failed: %d\n",,
    release_dma_chan:
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn catpt_boot_firmware(cdev: *mut catpt_dev, restore: bool) -> c_int {
    int catpt_boot_firmware(struct catpt_dev *cdev, bool restore)
    {
    pub ret: c_int,
    pub true): catpt_dsp_stall(cdev,,
    pub restore): ret = catpt_request_dma_load_firmware(cdev,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "load binaries failed: %d\n",,
    pub ret: return,
    }
    pub false): catpt_dsp_stall(cdev,,
    ret = wait_for_completion_timeout(&cdev.fw_ready,
    if (!ret) {
    pub timeout\n"): dev_err(cdev->dev, "firmware ready,
    pub -ETIMEDOUT: return,
    }
// Wake up does not mean FW is ready, an exception could occur.
    if (!cdev.ipc.ready)
    pub -EREMOTEIO: return,
// update sram pg & clock once done booting
    pub cdev->spec->dram_mask): catpt_dsp_update_srampge(cdev, &cdev->dram,,
    pub cdev->spec->iram_mask): catpt_dsp_update_srampge(cdev, &cdev->iram,,
    pub catpt_dsp_update_lpclock(cdev): return,
    }
#[no_mangle]
pub unsafe extern "C" fn catpt_first_boot_firmware(cdev: *mut catpt_dev) -> c_int {
    int catpt_first_boot_firmware(struct catpt_dev *cdev)
    {
    pub res: *mut resource,
    pub ret: c_int,
    pub false): ret = catpt_boot_firmware(cdev,,
    if (ret) {
    pub ret): dev_err(cdev->dev, "basefw boot failed: %d\n",,
    pub ret: return,
    }
// restrict FW Core dump area
    pub 0): __request_region(&cdev->dram, 0, 0x200, NULL,,
// restrict entire area following BASE_FW - highest offset in DRAM
    pub res->sibling): for (res = cdev->dram.child; res->sibling; res =,
    __request_region(&cdev.dram, res.end + 1,
    pub 0): cdev->dram.end - res->end, NULL,,
    pub &cdev->mixer): ret = catpt_ipc_get_mixer_stream_info(cdev,,
    if (ret)
    pub CATPT_IPC_RET(ret): return,
    pub catpt_arm_stream_templates(cdev): ret =,
    if (ret) {
    pub ret): dev_err(cdev->dev, "arm templates failed: %d\n",,
    pub ret: return,
    }
// update dram pg for scratch and restricted regions
    pub cdev->spec->dram_mask): catpt_dsp_update_srampge(cdev, &cdev->dram,,
    pub 0: return,
    }
