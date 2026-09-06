//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/gemini/sl3516-ce.h
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
// sl3516-ce.h - hardware cryptographic offloader for cortina/gemini SoC
//
// Copyright (C) 2021 Corentin LABBE <clabbe@baylibre.com>
//
// General notes on this driver:
// Called either Crypto Acceleration Engine Module, Security Acceleration Engine
// or IPSEC module in the datasheet, it will be called Crypto Engine for short
// in this driver.
// The CE was designed to handle IPSEC and wifi(TKIP WEP) protocol.
// It can handle AES, DES, 3DES, MD5, WEP, TKIP, SHA1, HMAC(MD5), HMAC(SHA1),
// Michael cipher/digest suites.
// It acts the same as a network hw, with both RX and TX chained descriptors.
//

pub const TQ0_TYPE_DATA: c_int = 0;

pub const ECB_AES: c_uint = 0x2;
pub const DESC_LAST: c_uint = 0x01;
pub const DESC_FIRST: c_uint = 0x02;
pub const IPSEC_ID: c_uint = 0x0000;
pub const IPSEC_STATUS_REG: c_uint = 0x00a8;
pub const IPSEC_RAND_NUM_REG: c_uint = 0x00ac;
pub const IPSEC_DMA_DEVICE_ID: c_uint = 0xff00;
pub const IPSEC_DMA_STATUS: c_uint = 0xff04;
pub const IPSEC_TXDMA_CTRL: c_uint = 0xff08;
pub const IPSEC_TXDMA_FIRST_DESC: c_uint = 0xff0c;
pub const IPSEC_TXDMA_CURR_DESC: c_uint = 0xff10;
pub const IPSEC_RXDMA_CTRL: c_uint = 0xff14;
pub const IPSEC_RXDMA_FIRST_DESC: c_uint = 0xff18;
pub const IPSEC_RXDMA_CURR_DESC: c_uint = 0xff1c;
pub const IPSEC_TXDMA_BUF_ADDR: c_uint = 0xff28;
pub const IPSEC_RXDMA_BUF_ADDR: c_uint = 0xff38;
pub const IPSEC_RXDMA_BUF_SIZE: c_uint = 0xff30;
pub const CE_ENCRYPTION: c_uint = 0x01;
pub const CE_DECRYPTION: c_uint = 0x03;
pub const MAXDESC: c_int = 6;

// the burst value is not documented in the datasheet

// the burst value is not documented in the datasheet

pub const CE_CPU: c_int = 0;
pub const CE_DMA: c_int = 1;
//
// struct sl3516_ce_descriptor - descriptor for CE operations
// @frame_ctrl:		Information for the current descriptor
// @flag_status:	For send packet, describe flag of operations.
// @buf_adr:		pointer to a send/recv buffer for data packet
// @next_desc:		control linking to other descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct descriptor {
    pub raw: u32,
//
// struct desc_frame_ctrl - Information for the current descriptor
// @buffer_size:	the size of buffer at buf_adr
// @desc_count:		Upon completion of a DMA operation, DMA
// write the number of descriptors used
// for the current frame
// @checksum:		unknown
// @authcomp:		unknown
// @perr:		Protocol error during processing this descriptor
// @derr:		Data error during processing this descriptor
// @own:		0 if owned by CPU, 1 for DMA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_frame_ctrl {
    pub :16: u32 buffer_size,
    pub :6: u32 desc_count,
    pub :6: u32 checksum,
    pub :1: u32 authcomp,
    pub :1: u32 perr,
    pub :1: u32 derr,
    pub :1: u32 own,
    pub bits: },
    pub frame_ctrl: },
    pub raw: u32,
//
// struct desc_flag_status - flag for this descriptor
// @tqflag:	list of flag describing the type of operation
// to be performed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_tx_flag_status {
    pub :10: u32 tqflag,
    pub :22: u32 unused,
    pub tx_flag: },
    pub flag_status: },
    pub buf_adr: u32,
    pub next_descriptor: u32,
//
// struct desc_next - describe chaining of descriptors
// @sof_eof:	does the descriptor is first (0x11),
// the last (0x01), middle of a chan (0x00)
// or the only one (0x11)
// @dec:	AHB bus address increase (0), decrease (1)
// @eofie:	End of frame interrupt enable
// @ndar:	Next descriptor address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_next {
    pub :2: u32 sof_eof,
    pub :1: u32 dec,
    pub :1: u32 eofie,
    pub :28: u32 ndar,
    pub bits: },
    pub next_desc: },
}

//
// struct control - The value of this register is used to set the
// operation mode of the IPSec Module.
// @process_id:		Used to identify the process. The number will be copied
// to the descriptor status of the received packet.
// @auth_check_len:	Number of 32-bit words to be checked or appended by the
// authentication module
// @auth_algorithm:
// @auth_mode:		0:append 1:Check Authentication Result
// @fcs_stream_copy:	0:enable 1:disable authentication stream copy
// @mix_key_sel:	0:use rCipherKey0-3  1:use Key Mixer
// @aesnk:		AES Key Size
// @cipher_algorithm:	choice of CBC/ECE and AES/DES/3DES
// @op_mode:		Operation Mode for the IPSec Module
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_control_header {
    pub :8: u32 process_id,
    pub :3: u32 auth_check_len,
    pub :1: u32 un1,
    pub :3: u32 auth_algorithm,
    pub :1: u32 auth_mode,
    pub :1: u32 fcs_stream_copy,
    pub :2: u32 un2,
    pub :1: u32 mix_key_sel,
    pub :4: u32 aesnk,
    pub :3: u32 cipher_algorithm,
    pub :1: u32 un3,
    pub :4: u32 op_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_control_cipher {
    pub :16: u32 algorithm_len,
    pub :16: u32 header_len,
}

//
// struct pkt_control_ecb - control packet for ECB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_control_ecb {
    pub control: pkt_control_header,
    pub cipher: pkt_control_cipher,
    pub key: [c_uchar; AES_MAX_KEY_SIZE],
}

//
// struct sl3516_ce_dev - main container for all this driver information
// @base:	base address
// @clks:	clocks used
// @reset:	pointer to reset controller
// @dev:	the platform device
// @engine:	ptr to the crypto/crypto_engine
// @complete:	completion for the current task on this flow
// @status:	set to 1 by interrupt if task is done
// @dtx:	base DMA address for TX descriptors
// @tx		base address of TX descriptors
// @drx:	base DMA address for RX descriptors
// @rx		base address of RX descriptors
// @ctx		current used TX descriptor
// @crx		current used RX descriptor
// @trng	hw_random structure for RNG
// @hwrng_stat_req	number of HWRNG requests
// @hwrng_stat_bytes	total number of bytes generated by RNG
// @stat_irq	number of IRQ handled by CE
// @stat_irq_tx	number of TX IRQ handled by CE
// @stat_irq_rx	number of RX IRQ handled by CE
// @stat_req	number of requests handled by CE
// @fallbak_sg_count_tx		number of fallback due to destination SG count
// @fallbak_sg_count_rx		number of fallback due to source SG count
// @fallbak_not_same_len	number of fallback due to difference in SG length
// @dbgfs_dir:	Debugfs dentry for statistic directory
// @dbgfs_stats: Debugfs dentry for statistic counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl3516_ce_dev {
    pub base: *mut void __iomem,
    pub clks: *mut clk,
    pub reset: *mut reset_control,
    pub dev: *mut device,
    pub engine: *mut crypto_engine,
    pub complete: completion,
    pub status: c_int,
    pub dtx: dma_addr_t,
    pub tx: *mut descriptor,
    pub drx: dma_addr_t,
    pub rx: *mut descriptor,
    pub ctx: c_int,
    pub crx: c_int,
    pub trng: hwrng,
    pub hwrng_stat_req: c_ulong,
    pub hwrng_stat_bytes: c_ulong,
    pub stat_irq: c_ulong,
    pub stat_irq_tx: c_ulong,
    pub stat_irq_rx: c_ulong,
    pub stat_req: c_ulong,
    pub fallback_sg_count_tx: c_ulong,
    pub fallback_sg_count_rx: c_ulong,
    pub fallback_not_same_len: c_ulong,
    pub fallback_mod16: c_ulong,
    pub fallback_align16: c_ulong,

    pub dbgfs_dir: *mut dentry,
    pub dbgfs_stats: *mut dentry,

    pub pctrl: *mut c_void,
    pub dctrl: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sginfo {
    pub addr: u32,
    pub len: u32,
}

//
// struct sl3516_ce_cipher_req_ctx - context for a skcipher request
// @t_src:		list of mapped SGs with their size
// @t_dst:		list of mapped SGs with their size
// @op_dir:		direction (encrypt vs decrypt) for this request
// @pctrllen:		the length of the ctrl packet
// @tqflag:		the TQflag to set in data packet
// @h			pointer to the pkt_control_cipher header
// @nr_sgs:		number of source SG
// @nr_sgd:		number of destination SG
// @fallback_req:	request struct for invoking the fallback skcipher TFM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl3516_ce_cipher_req_ctx {
    pub t_src: [sginfo; MAXDESC],
    pub t_dst: [sginfo; MAXDESC],
    pub op_dir: u32,
    pub pctrllen: c_uint,
    pub tqflag: u32,
    pub h: *mut pkt_control_cipher,
    pub nr_sgs: c_int,
    pub nr_sgd: c_int,
    pub end: skcipher_request fallback_req; // keep at the,
}

//
// struct sl3516_ce_cipher_tfm_ctx - context for a skcipher TFM
// @key:		pointer to key data
// @keylen:		len of the key
// @ce:			pointer to the private data of driver handling this TFM
// @fallback_tfm:	pointer to the fallback TFM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl3516_ce_cipher_tfm_ctx {
    pub key: *mut u32,
    pub keylen: u32,
    pub ce: *mut sl3516_ce_dev,
    pub fallback_tfm: *mut crypto_skcipher,
}

//
// struct sl3516_ce_alg_template - crypto_alg template
// @type:		the CRYPTO_ALG_TYPE for this template
// @mode:		value to be used in control packet for this algorithm
// @ce:			pointer to the sl3516_ce_dev structure associated with
// this template
// @alg:		one of sub struct must be used
// @stat_req:		number of request done on this template
// @stat_fb:		number of request which has fallbacked
// @stat_bytes:		total data size done by this template
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl3516_ce_alg_template {
    pub type: u32,
    pub mode: u32,
    pub ce: *mut sl3516_ce_dev,
    pub skcipher: skcipher_engine_alg,
    pub alg: },
    pub stat_req: c_ulong,
    pub stat_fb: c_ulong,
    pub stat_bytes: c_ulong,
}

extern "C" {
    pub fn sl3516_ce_cipher_init(tfm: *mut crypto_tfm) -> c_int;
}
extern "C" {
    pub fn sl3516_ce_cipher_exit(tfm: *mut crypto_tfm);
}
extern "C" {
    pub fn sl3516_ce_skdecrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sl3516_ce_skencrypt(areq: *mut skcipher_request) -> c_int;
}
extern "C" {
    pub fn sl3516_ce_rng_register(ce: *mut sl3516_ce_dev) -> c_int;
}
extern "C" {
    pub fn sl3516_ce_rng_unregister(ce: *mut sl3516_ce_dev);
}
extern "C" {
    pub fn sl3516_ce_handle_cipher_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int;
}
