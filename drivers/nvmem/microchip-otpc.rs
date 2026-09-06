//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/microchip-otpc.c
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
// OTP Memory controller
//
// Copyright (C) 2022 Microchip Technology Inc. and its subsidiaries
//
// Author: Claudiu Beznea <claudiu.beznea@microchip.com>
//

//
// struct mchp_otpc - OTPC private data structure
// @base: base address
// @dev: struct device pointer
// @packets: list of packets in OTP memory
// @npackets: number of packets in OTP memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_otpc {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub packets: list_head,
    pub npackets: u32,
}

//
// struct mchp_otpc_packet - OTPC packet data structure
// @list: list head
// @id: packet ID
// @offset: packet offset (in words) in OTP memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_otpc_packet {
    pub list: list_head,
    pub id: u32,
    pub offset: u32,
}

    static struct mchp_otpc_packet *mchp_otpc_id_to_packet(struct mchp_otpc *otpc,
    u32 id)
    {
    struct mchp_otpc_packet *packet;
    if (id >= otpc.npackets)
    return core::ptr::null_mut();
    list_for_each_entry(packet, &otpc.packets, list) {
    if (packet.id == id)
    return packet;
    }
    return core::ptr::null_mut();
    }
    static int mchp_otpc_prepare_read(struct mchp_otpc *otpc,
    unsigned int offset)
    {
    u32 tmp;
// Set address.
    tmp = readl_relaxed(otpc.base + MCHP_OTPC_MR);
    tmp &= ~MCHP_OTPC_MR_ADDR;
    tmp |= FIELD_PREP(MCHP_OTPC_MR_ADDR, offset);
    writel_relaxed(tmp, otpc.base + MCHP_OTPC_MR);
// Set read.
    tmp = readl_relaxed(otpc.base + MCHP_OTPC_CR);
    tmp |= MCHP_OTPC_CR_READ;
    writel_relaxed(tmp, otpc.base + MCHP_OTPC_CR);
// Wait for packet to be transferred into temporary buffers.
    return read_poll_timeout(readl_relaxed, tmp, !(tmp & MCHP_OTPC_SR_READ),
    10000, 2000, false, otpc.base + MCHP_OTPC_SR);
    }
//
// OTPC memory is organized into packets. Each packets contains a header and
// a payload. Header is 4 bytes long and contains the size of the payload.
// Payload size varies. The memory footprint is something as follows:
//
// Memory offset  Memory footprint     Packet ID
// -------------  ----------------     ---------
//
// 0x0            +------------+   <-- packet 0
// | header  0  |
// 0x4            +------------+
// | payload 0  |
// .            .
// .    ...     .
// .            .
// offset1        +------------+   <-- packet 1
// | header  1  |
// offset1 + 0x4  +------------+
// | payload 1  |
// .            .
// .    ...     .
// .            .
// offset2        +------------+   <-- packet 2
// .            .
// .    ...     .
// .            .
// offsetN        +------------+   <-- packet N
// | header  N  |
// offsetN + 0x4  +------------+
// | payload N  |
// .            .
// .    ...     .
// .            .
// +------------+
//
// where offset1, offset2, offsetN depends on the size of payload 0, payload 1,
// payload N-1.
//
// The access to memory is done on a per packet basis: the control registers
// need to be updated with an offset address (within a packet range) and the
// data registers will be update by controller with information contained by
// that packet. E.g. if control registers are updated with any address within
// the range [offset1, offset2) the data registers are updated by controller
// with packet 1. Header data is accessible though MCHP_OTPC_HR register.
// Payload data is accessible though MCHP_OTPC_DR and MCHP_OTPC_AR registers.
// There is no direct mapping b/w the offset requested by software and the
// offset returned by hardware.
//
// For this, the read function will return the first requested bytes in the
// packet. The user will have to be aware of the memory footprint before doing
// the read request.
//
    static int mchp_otpc_read(void *priv, unsigned int off, void *val,
    size_t bytes)
    {
    struct mchp_otpc *otpc = priv;
    struct mchp_otpc_packet *packet;
    u32 *buf = val;
    u32 offset;
    let mut len: usize = 0;
    int ret, payload_size;
//
// We reach this point with off being multiple of stride = 4 to
// be able to cross the subsystem. Inside the driver we use continuous
// unsigned integer numbers for packet id, thus divide off by 4
// before passing it to mchp_otpc_id_to_packet().
//
    packet = mchp_otpc_id_to_packet(otpc, off / 4);
    if (!packet)
    return -EINVAL;
    offset = packet.offset;
    while (len < bytes) {
    ret = mchp_otpc_prepare_read(otpc, offset);
    if (ret)
    return ret;
// Read and save header content.
// buf++ = readl_relaxed(otpc->base + MCHP_OTPC_HR);
    len += sizeof(*buf);
    offset++;
    if (len >= bytes)
    break;
// Read and save payload content.
    payload_size = FIELD_GET(MCHP_OTPC_HR_SIZE, *(buf - 1));
    writel_relaxed(0UL, otpc.base + MCHP_OTPC_AR);
    do {
// buf++ = readl_relaxed(otpc->base + MCHP_OTPC_DR);
    len += sizeof(*buf);
    offset++;
    payload_size--;
    } while (payload_size >= 0 && len < bytes);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_otpc_init_packets_list(otpc: *mut mchp_otpc, size: *mut u32) -> c_int {
    static int mchp_otpc_init_packets_list(struct mchp_otpc *otpc, u32 *size)
    {
    struct mchp_otpc_packet *packet;
    u32 word, word_pos = 0, id = 0, npackets = 0, payload_size;
    int ret;
    INIT_LIST_HEAD(&otpc.packets);
// size = 0;
    while (*size < MCHP_OTPC_SIZE) {
    ret = mchp_otpc_prepare_read(otpc, word_pos);
    if (ret)
    return ret;
    word = readl_relaxed(otpc.base + MCHP_OTPC_HR);
    payload_size = FIELD_GET(MCHP_OTPC_HR_SIZE, word);
    if (!payload_size)
    break;
    packet = devm_kzalloc(otpc.dev, sizeof(*packet), GFP_KERNEL);
    if (!packet)
    return -ENOMEM;
    packet.id = id++;
    packet.offset = word_pos;
    INIT_LIST_HEAD(&packet.list);
    list_add_tail(&packet.list, &otpc.packets);
// Count size by adding header and paload sizes.
// size += 4 * (payload_size + 1);
// Next word: this packet (header, payload) position + 1.
    word_pos += payload_size + 2;
    npackets++;
    }
    otpc.npackets = npackets;
    return 0;
    }
    static struct nvmem_config mchp_nvmem_config = {
    .name = MCHP_OTPC_NAME,
    .type = NVMEM_TYPE_OTP,
    .read_only = true,
    .word_size = 4,
    .stride = 4,
    .reg_read = mchp_otpc_read,
    };
#[no_mangle]
unsafe extern "C" fn mchp_otpc_probe(pdev: *mut platform_device) -> c_int {
    static int mchp_otpc_probe(struct platform_device *pdev)
    {
    struct nvmem_device *nvmem;
    struct mchp_otpc *otpc;
    u32 size;
    int ret;
    otpc = devm_kzalloc(&pdev.dev, sizeof(*otpc), GFP_KERNEL);
    if (!otpc)
    return -ENOMEM;
    otpc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(otpc.base))
    return PTR_ERR(otpc.base);
    otpc.dev = &pdev.dev;
    ret = mchp_otpc_init_packets_list(otpc, &size);
    if (ret)
    return ret;
    mchp_nvmem_config.dev = otpc.dev;
    mchp_nvmem_config.add_legacy_fixed_of_cells = true;
    mchp_nvmem_config.size = size;
    mchp_nvmem_config.priv = otpc;
    nvmem = devm_nvmem_register(&pdev.dev, &mchp_nvmem_config);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct of_device_id __maybe_unused mchp_otpc_ids[] = {
    { .compatible = "microchip,sama7g5-otpc", },
    { },
    };
    MODULE_DEVICE_TABLE(of, mchp_otpc_ids);
    static struct platform_driver mchp_otpc_driver = {
    .probe = mchp_otpc_probe,
    .driver = {
    .name = MCHP_OTPC_NAME,
    .of_match_table = of_match_ptr(mchp_otpc_ids),
    },
    };
    module_platform_driver(mchp_otpc_driver);
    MODULE_AUTHOR("Claudiu Beznea <claudiu.beznea@microchip.com>");
    MODULE_DESCRIPTION("Microchip SAMA7G5 OTPC driver");
    MODULE_LICENSE("GPL");
