//! Automatically rewritten from C to Rust
//! Source: drivers/soc/mediatek/mtk-cmdq-helper.c
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
// Copyright (c) 2018 MediaTek Inc.

// dedicate the last GPR_R15 to assign the register address to be poll

pub const CMDQ_IMMEDIATE_VALUE: c_int = 0;
pub const CMDQ_REG_TYPE: c_int = 1;
pub const CMDQ_JUMP_RELATIVE: c_int = 0;
pub const CMDQ_JUMP_ABSOLUTE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_instruction {
    union {
    pub value: u32,
    pub mask: u32,
    struct {
    pub arg_c: u16,
    pub src_reg: u16,
}

    };
    union {
    u16 offset;
    u16 event;
    u16 reg_dst;
    };
    union {
    u8 subsys;
    struct {
    u8 sop:5;
    u8 arg_c_t:1;
    u8 src_t:1;
    u8 dst_t:1;
    };
    };
    u8 op;
    };
#[no_mangle]
pub unsafe extern "C" fn cmdq_operand_get_type(op: *mut cmdq_operand) -> u8 {
    static inline u8 cmdq_operand_get_type(struct cmdq_operand *op)
    {
    return op.reg ? CMDQ_REG_TYPE : CMDQ_IMMEDIATE_VALUE;
    }
#[no_mangle]
pub unsafe extern "C" fn cmdq_operand_get_idx_value(op: *mut cmdq_operand) -> u16 {
    static inline u16 cmdq_operand_get_idx_value(struct cmdq_operand *op)
    {
    return op.reg ? op.idx : op.value;
    }
    int cmdq_dev_get_client_reg(struct device *dev,
    struct cmdq_client_reg *client_reg, int idx)
    {
    struct of_phandle_args spec;
    struct resource res;
    int err;
    if (!client_reg)
    return -ENOENT;
    err = of_address_to_resource(dev.of_node, 0, &res);
    if (err) {
    dev_err(dev, "Missing reg in %s node\n", dev.of_node.full_name);
    return -EINVAL;
    }
    client_reg.pa_base = res.start;
    err = of_parse_phandle_with_fixed_args(dev.of_node,
    "mediatek,gce-client-reg",
    3, idx, &spec);
    if (err < 0) {
    dev_dbg(dev,
    "error %d can't parse gce-client-reg property (%d)",
    err, idx);
// make subsys invalid
    client_reg.subsys = CMDQ_SUBSYS_INVALID;
//
// All GCEs support writing register PA with mask without subsys,
// but this requires extra GCE instructions to convert the PA into
// a format that GCE can handle, which is less performance than
// directly using subsys. Therefore, when subsys is available,
// we prefer to use subsys for writing register PA.
//
    client_reg.pkt_write = cmdq_pkt_write_pa;
    client_reg.pkt_write_mask = cmdq_pkt_write_mask_pa;
    return 0;
    }
    client_reg.subsys = (u8)spec.args[0];
    client_reg.offset = (u16)spec.args[1];
    client_reg.size = (u16)spec.args[2];
    of_node_put(spec.np);
    client_reg.pkt_write = cmdq_pkt_write_subsys;
    client_reg.pkt_write_mask = cmdq_pkt_write_mask_subsys;
    return 0;
    }
    EXPORT_SYMBOL(cmdq_dev_get_client_reg);
    struct cmdq_client *cmdq_mbox_create(struct device *dev, int index)
    {
    struct cmdq_client *client;
    client = kzalloc_obj(*client);
    if (!client)
    return (struct cmdq_client *)-ENOMEM;
    client.client.dev = dev;
    client.client.tx_block = false;
    client.client.knows_txdone = true;
    client.chan = mbox_request_channel(&client.client, index);
    if (IS_ERR(client.chan)) {
    long err;
    dev_err(dev, "failed to request channel\n");
    err = PTR_ERR(client.chan);
    kfree(client);
    return ERR_PTR(err);
    }
    return client;
    }
    EXPORT_SYMBOL(cmdq_mbox_create);
#[no_mangle]
pub unsafe extern "C" fn cmdq_mbox_destroy(client: *mut cmdq_client) {
    void cmdq_mbox_destroy(struct cmdq_client *client)
    {
    mbox_free_channel(client.chan);
    kfree(client);
    }
    EXPORT_SYMBOL(cmdq_mbox_destroy);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_create(client: *mut cmdq_client, pkt: *mut cmdq_pkt, size: usize) -> c_int {
    int cmdq_pkt_create(struct cmdq_client *client, struct cmdq_pkt *pkt, size_t size)
    {
    struct device *dev;
    dma_addr_t dma_addr;
    pkt.va_base = kzalloc(size, GFP_KERNEL);
    if (!pkt.va_base)
    return -ENOMEM;
    pkt.buf_size = size;
    dev = client.chan.mbox.dev;
    dma_addr = dma_map_single(dev, pkt.va_base, pkt.buf_size,
    DMA_TO_DEVICE);
    if (dma_mapping_error(dev, dma_addr)) {
    dev_err(dev, "dma map failed, size=%u\n", (u32)(u64)size);
    kfree(pkt.va_base);
    return -ENOMEM;
    }
    pkt.pa_base = dma_addr;
    cmdq_get_mbox_priv(client.chan, &pkt.priv);
    return 0;
    }
    EXPORT_SYMBOL(cmdq_pkt_create);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_destroy(client: *mut cmdq_client, pkt: *mut cmdq_pkt) {
    void cmdq_pkt_destroy(struct cmdq_client *client, struct cmdq_pkt *pkt)
    {
    dma_unmap_single(client.chan.mbox.dev, pkt.pa_base, pkt.buf_size,
    DMA_TO_DEVICE);
    kfree(pkt.va_base);
    }
    EXPORT_SYMBOL(cmdq_pkt_destroy);
    static int cmdq_pkt_append_command(struct cmdq_pkt *pkt,
    struct cmdq_instruction inst)
    {
    struct cmdq_instruction *cmd_ptr;
    if (unlikely(pkt.cmd_buf_size + CMDQ_INST_SIZE > pkt.buf_size)) {
//
// In the case of allocated buffer size (pkt->buf_size) is used
// up, the real required size (pkt->cmdq_buf_size) is still
// increased, so that the user knows how much memory should be
// ultimately allocated after appending all commands and
// flushing the command packet. Therefor, the user can call
// cmdq_pkt_create() again with the real required buffer size.
//
    pkt.cmd_buf_size += CMDQ_INST_SIZE;
    WARN_ONCE(1, "%s: buffer size %u is too small !\n",
    __func__, (u32)pkt.buf_size);
    return -ENOMEM;
    }
    cmd_ptr = pkt.va_base + pkt.cmd_buf_size;
// cmd_ptr = inst;
    pkt.cmd_buf_size += CMDQ_INST_SIZE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmdq_pkt_mask(pkt: *mut cmdq_pkt, mask: u32) -> c_int {
    static int cmdq_pkt_mask(struct cmdq_pkt *pkt, u32 mask)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_MASK,
    .mask = ~mask
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_write(pkt: *mut cmdq_pkt, subsys: u8, offset: u16, value: u32) -> c_int {
    int cmdq_pkt_write(struct cmdq_pkt *pkt, u8 subsys, u16 offset, u32 value)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_WRITE,
    .value = value,
    .offset = offset,
    .subsys = subsys
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_write);
    int cmdq_pkt_write_pa(struct cmdq_pkt *pkt, u8 subsys /*unused*/, u32 pa_base,
    u16 offset, u32 value)
    {
    int err;
    err = cmdq_pkt_assign(pkt, CMDQ_THR_SPR_IDX0, CMDQ_ADDR_HIGH(pa_base));
    if (err < 0)
    return err;
    return cmdq_pkt_write_s_value(pkt, CMDQ_THR_SPR_IDX0, CMDQ_ADDR_LOW(offset), value);
    }
    EXPORT_SYMBOL(cmdq_pkt_write_pa);
    int cmdq_pkt_write_subsys(struct cmdq_pkt *pkt, u8 subsys, u32 pa_base /*unused*/,
    u16 offset, u32 value)
    {
    return cmdq_pkt_write(pkt, subsys, offset, value);
    }
    EXPORT_SYMBOL(cmdq_pkt_write_subsys);
    int cmdq_pkt_write_mask(struct cmdq_pkt *pkt, u8 subsys,
    u16 offset, u32 value, u32 mask)
    {
    let mut offset_mask: u16 = offset;
    int err;
    if (mask != GENMASK(31, 0)) {
    err = cmdq_pkt_mask(pkt, mask);
    if (err < 0)
    return err;
    offset_mask |= CMDQ_WRITE_ENABLE_MASK;
    }
    return cmdq_pkt_write(pkt, subsys, offset_mask, value);
    }
    EXPORT_SYMBOL(cmdq_pkt_write_mask);
    int cmdq_pkt_write_mask_pa(struct cmdq_pkt *pkt, u8 subsys /*unused*/, u32 pa_base,
    u16 offset, u32 value, u32 mask)
    {
    int err;
    err = cmdq_pkt_assign(pkt, CMDQ_THR_SPR_IDX0, CMDQ_ADDR_HIGH(pa_base));
    if (err < 0)
    return err;
    return cmdq_pkt_write_s_mask_value(pkt, CMDQ_THR_SPR_IDX0,
    CMDQ_ADDR_LOW(offset), value, mask);
    }
    EXPORT_SYMBOL(cmdq_pkt_write_mask_pa);
    int cmdq_pkt_write_mask_subsys(struct cmdq_pkt *pkt, u8 subsys, u32 pa_base /*unused*/,
    u16 offset, u32 value, u32 mask)
    {
    return cmdq_pkt_write_mask(pkt, subsys, offset, value, mask);
    }
    EXPORT_SYMBOL(cmdq_pkt_write_mask_subsys);
    int cmdq_pkt_read_s(struct cmdq_pkt *pkt, u16 high_addr_reg_idx, u16 addr_low,
    u16 reg_idx)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_READ_S,
    .dst_t = CMDQ_REG_TYPE,
    .sop = high_addr_reg_idx,
    .reg_dst = reg_idx,
    .src_reg = addr_low
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_read_s);
    int cmdq_pkt_write_s(struct cmdq_pkt *pkt, u16 high_addr_reg_idx,
    u16 addr_low, u16 src_reg_idx)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_WRITE_S,
    .src_t = CMDQ_REG_TYPE,
    .sop = high_addr_reg_idx,
    .offset = addr_low,
    .src_reg = src_reg_idx
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_write_s);
    int cmdq_pkt_write_s_mask(struct cmdq_pkt *pkt, u16 high_addr_reg_idx,
    u16 addr_low, u16 src_reg_idx, u32 mask)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_WRITE_S_MASK,
    .src_t = CMDQ_REG_TYPE,
    .sop = high_addr_reg_idx,
    .offset = addr_low,
    .src_reg = src_reg_idx,
    };
    int err;
    err = cmdq_pkt_mask(pkt, mask);
    if (err < 0)
    return err;
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_write_s_mask);
    int cmdq_pkt_write_s_value(struct cmdq_pkt *pkt, u8 high_addr_reg_idx,
    u16 addr_low, u32 value)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_WRITE_S,
    .sop = high_addr_reg_idx,
    .offset = addr_low,
    .value = value
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_write_s_value);
    int cmdq_pkt_write_s_mask_value(struct cmdq_pkt *pkt, u8 high_addr_reg_idx,
    u16 addr_low, u32 value, u32 mask)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_WRITE_S_MASK,
    .sop = high_addr_reg_idx,
    .offset = addr_low,
    .value = value
    };
    int err;
    err = cmdq_pkt_mask(pkt, mask);
    if (err < 0)
    return err;
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_write_s_mask_value);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_mem_move(pkt: *mut cmdq_pkt, src_addr: dma_addr_t, dst_addr: dma_addr_t) -> c_int {
    int cmdq_pkt_mem_move(struct cmdq_pkt *pkt, dma_addr_t src_addr, dma_addr_t dst_addr)
    {
    let mut high_addr_reg_idx: u16 = CMDQ_THR_SPR_IDX0;
    let mut value_reg_idx: u16 = CMDQ_THR_SPR_IDX1;
    int ret;
// read the value of src_addr into high_addr_reg_idx
    src_addr += pkt.priv.mminfra_offset;
    ret = cmdq_pkt_assign(pkt, high_addr_reg_idx, CMDQ_ADDR_HIGH(src_addr));
    if (ret < 0)
    return ret;
    ret = cmdq_pkt_read_s(pkt, high_addr_reg_idx, CMDQ_ADDR_LOW(src_addr), value_reg_idx);
    if (ret < 0)
    return ret;
// write the value of value_reg_idx into dst_addr
    dst_addr += pkt.priv.mminfra_offset;
    ret = cmdq_pkt_assign(pkt, high_addr_reg_idx, CMDQ_ADDR_HIGH(dst_addr));
    if (ret < 0)
    return ret;
    ret = cmdq_pkt_write_s(pkt, high_addr_reg_idx, CMDQ_ADDR_LOW(dst_addr), value_reg_idx);
    if (ret < 0)
    return ret;
    return 0;
    }
    EXPORT_SYMBOL(cmdq_pkt_mem_move);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_wfe(pkt: *mut cmdq_pkt, event: u16, clear: bool) -> c_int {
    int cmdq_pkt_wfe(struct cmdq_pkt *pkt, u16 event, bool clear)
    {
    let mut clear_option: u32 = clear ? CMDQ_WFE_UPDATE : 0;
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_WFE,
    .value = CMDQ_WFE_OPTION | clear_option,
    .event = event
    };
    if (event >= CMDQ_MAX_EVENT)
    return -EINVAL;
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_wfe);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_acquire_event(pkt: *mut cmdq_pkt, event: u16) -> c_int {
    int cmdq_pkt_acquire_event(struct cmdq_pkt *pkt, u16 event)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_WFE,
    .value = CMDQ_WFE_UPDATE | CMDQ_WFE_UPDATE_VALUE | CMDQ_WFE_WAIT,
    .event = event
    };
    if (event >= CMDQ_MAX_EVENT)
    return -EINVAL;
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_acquire_event);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_clear_event(pkt: *mut cmdq_pkt, event: u16) -> c_int {
    int cmdq_pkt_clear_event(struct cmdq_pkt *pkt, u16 event)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_WFE,
    .value = CMDQ_WFE_UPDATE,
    .event = event
    };
    if (event >= CMDQ_MAX_EVENT)
    return -EINVAL;
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_clear_event);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_set_event(pkt: *mut cmdq_pkt, event: u16) -> c_int {
    int cmdq_pkt_set_event(struct cmdq_pkt *pkt, u16 event)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_WFE,
    .value = CMDQ_WFE_UPDATE | CMDQ_WFE_UPDATE_VALUE,
    .event = event
    };
    if (event >= CMDQ_MAX_EVENT)
    return -EINVAL;
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_set_event);
    int cmdq_pkt_poll(struct cmdq_pkt *pkt, u8 subsys,
    u16 offset, u32 value)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_POLL,
    .value = value,
    .offset = offset,
    .subsys = subsys
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_poll);
    int cmdq_pkt_poll_mask(struct cmdq_pkt *pkt, u8 subsys,
    u16 offset, u32 value, u32 mask)
    {
    int err;
    err = cmdq_pkt_mask(pkt, mask);
    if (err < 0)
    return err;
    offset = offset | CMDQ_POLL_ENABLE_MASK;
    return cmdq_pkt_poll(pkt, subsys, offset, value);
    }
    EXPORT_SYMBOL(cmdq_pkt_poll_mask);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_poll_addr(pkt: *mut cmdq_pkt, addr: dma_addr_t, value: u32, mask: u32) -> c_int {
    int cmdq_pkt_poll_addr(struct cmdq_pkt *pkt, dma_addr_t addr, u32 value, u32 mask)
    {
    let mut inst: cmdq_instruction = { {0} };
    let mut use_mask: u8 = 0;
    int ret;
//
// Append an MASK instruction to set the mask for following POLL instruction
// which enables use_mask bit.
//
    if (mask != GENMASK(31, 0)) {
    ret = cmdq_pkt_mask(pkt, mask);
    if (ret < 0)
    return ret;
    use_mask = CMDQ_POLL_ENABLE_MASK;
    }
//
// POLL is an legacy operation in GCE and it does not support SPR and CMDQ_CODE_LOGIC,
// so it can not use cmdq_pkt_assign to keep polling register address to SPR.
// If user wants to poll a register address which doesn't have a subsys id,
// user needs to use GPR and CMDQ_CODE_MASK to move polling register address to GPR.
//
    inst.op = CMDQ_CODE_MASK;
    inst.dst_t = CMDQ_REG_TYPE;
    inst.sop = CMDQ_POLL_ADDR_GPR;
    inst.value = addr + pkt.priv.mminfra_offset;
    ret = cmdq_pkt_append_command(pkt, inst);
    if (ret < 0)
    return ret;
// Append POLL instruction to poll the register address assign to GPR previously.
    inst.op = CMDQ_CODE_POLL;
    inst.dst_t = CMDQ_REG_TYPE;
    inst.sop = CMDQ_POLL_ADDR_GPR;
    inst.offset = use_mask;
    inst.value = value;
    ret = cmdq_pkt_append_command(pkt, inst);
    if (ret < 0)
    return ret;
    return 0;
    }
    EXPORT_SYMBOL(cmdq_pkt_poll_addr);
    int cmdq_pkt_logic_command(struct cmdq_pkt *pkt, u16 result_reg_idx,
    struct cmdq_operand *left_operand,
    enum cmdq_logic_op s_op,
    struct cmdq_operand *right_operand)
    {
    struct cmdq_instruction inst;
    if (!left_operand || !right_operand || s_op >= CMDQ_LOGIC_MAX)
    return -EINVAL;
    inst.value = 0;
    inst.op = CMDQ_CODE_LOGIC;
    inst.dst_t = CMDQ_REG_TYPE;
    inst.src_t = cmdq_operand_get_type(left_operand);
    inst.arg_c_t = cmdq_operand_get_type(right_operand);
    inst.sop = s_op;
    inst.reg_dst = result_reg_idx;
    inst.src_reg = cmdq_operand_get_idx_value(left_operand);
    inst.arg_c = cmdq_operand_get_idx_value(right_operand);
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_logic_command);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_assign(pkt: *mut cmdq_pkt, reg_idx: u16, value: u32) -> c_int {
    int cmdq_pkt_assign(struct cmdq_pkt *pkt, u16 reg_idx, u32 value)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_LOGIC,
    .dst_t = CMDQ_REG_TYPE,
    .reg_dst = reg_idx,
    .value = value
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_assign);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_jump_abs(pkt: *mut cmdq_pkt, addr: dma_addr_t, shift_pa: u8) -> c_int {
    int cmdq_pkt_jump_abs(struct cmdq_pkt *pkt, dma_addr_t addr, u8 shift_pa)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_JUMP,
    .offset = CMDQ_JUMP_ABSOLUTE,
    .value = (addr +  pkt.priv.mminfra_offset) >> pkt.priv.shift_pa
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_jump_abs);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_jump_rel(pkt: *mut cmdq_pkt, offset: i32, shift_pa: u8) -> c_int {
    int cmdq_pkt_jump_rel(struct cmdq_pkt *pkt, s32 offset, u8 shift_pa)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_JUMP,
    .value = (u32)offset >> shift_pa
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_jump_rel);
#[no_mangle]
pub unsafe extern "C" fn cmdq_pkt_eoc(pkt: *mut cmdq_pkt) -> c_int {
    int cmdq_pkt_eoc(struct cmdq_pkt *pkt)
    {
    struct cmdq_instruction inst = {
    .op = CMDQ_CODE_EOC,
    .value = CMDQ_EOC_IRQ_EN
    };
    return cmdq_pkt_append_command(pkt, inst);
    }
    EXPORT_SYMBOL(cmdq_pkt_eoc);
    MODULE_DESCRIPTION("MediaTek Command Queue (CMDQ) driver");
    MODULE_LICENSE("GPL v2");
