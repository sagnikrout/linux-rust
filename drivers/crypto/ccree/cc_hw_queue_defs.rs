//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_hw_queue_defs.h
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
// Copyright (C) 2012-2019 ARM Limited (or its affiliates).

//
// DEFINITIONS
//
pub const HW_DESC_SIZE_WORDS: c_int = 6;
// Define max. available slots in HW queue
pub const HW_QUEUE_SLOTS_MAX: c_int = 15;

//
// TYPE DEFINITIONS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_hw_desc {
    pub word: [u32; HW_DESC_SIZE_WORDS],
    pub 2]: *mut *mut u16 hword[HW_DESC_SIZE_WORDS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_axi_sec {
    AXI_SECURE = 0,
    AXI_NOT_SECURE = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_desc_direction {
    DESC_DIRECTION_ILLEGAL = -1,
    DESC_DIRECTION_ENCRYPT_ENCRYPT = 0,
    DESC_DIRECTION_DECRYPT_DECRYPT = 1,
    DESC_DIRECTION_DECRYPT_ENCRYPT = 3,
    DESC_DIRECTION_END = S32_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_dma_mode {
    DMA_MODE_NULL		= -1,
    NO_DMA			= 0,
    DMA_SRAM		= 1,
    DMA_DLLI		= 2,
    DMA_MLLI		= 3,
    DMA_MODE_END		= S32_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_flow_mode {
    FLOW_MODE_NULL		= -1,
// data flows
    BYPASS			= 0,
    DIN_AES_DOUT		= 1,
    AES_to_HASH		= 2,
    AES_and_HASH		= 3,
    DIN_DES_DOUT		= 4,
    DES_to_HASH		= 5,
    DES_and_HASH		= 6,
    DIN_HASH		= 7,
    DIN_HASH_and_BYPASS	= 8,
    AESMAC_and_BYPASS	= 9,
    AES_to_HASH_and_DOUT	= 10,
    DIN_RC4_DOUT		= 11,
    DES_to_HASH_and_DOUT	= 12,
    AES_to_AES_to_HASH_and_DOUT	= 13,
    AES_to_AES_to_HASH	= 14,
    AES_to_HASH_and_AES	= 15,
    DIN_SM4_DOUT		= 16,
    DIN_AES_AESMAC		= 17,
    HASH_to_DOUT		= 18,
// setup flows
    S_DIN_to_AES		= 32,
    S_DIN_to_AES2		= 33,
    S_DIN_to_DES		= 34,
    S_DIN_to_RC4		= 35,
    S_DIN_to_SM4		= 36,
    S_DIN_to_HASH		= 37,
    S_AES_to_DOUT		= 38,
    S_AES2_to_DOUT		= 39,
    S_SM4_to_DOUT		= 40,
    S_RC4_to_DOUT		= 41,
    S_DES_to_DOUT		= 42,
    S_HASH_to_DOUT		= 43,
    SET_FLOW_ID		= 44,
    FLOW_MODE_END = S32_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_setup_op {
    SETUP_LOAD_NOP		= 0,
    SETUP_LOAD_STATE0	= 1,
    SETUP_LOAD_STATE1	= 2,
    SETUP_LOAD_STATE2	= 3,
    SETUP_LOAD_KEY0		= 4,
    SETUP_LOAD_XEX_KEY	= 5,
    SETUP_WRITE_STATE0	= 8,
    SETUP_WRITE_STATE1	= 9,
    SETUP_WRITE_STATE2	= 10,
    SETUP_WRITE_STATE3	= 11,
    SETUP_OP_END = S32_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_hash_conf_pad {
    HASH_PADDING_DISABLED = 0,
    HASH_PADDING_ENABLED = 1,
    HASH_DIGEST_RESULT_LITTLE_ENDIAN = 2,
    HASH_CONFIG1_PADDING_RESERVE32 = S32_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_aes_mac_selector {
    AES_SK = 1,
    AES_CMAC_INIT = 2,
    AES_CMAC_SIZE0 = 3,
    AES_MAC_END = S32_MAX,
}

pub const HW_KEY_MASK_CIPHER_DO: c_uint = 0x3;
pub const HW_KEY_SHIFT_CIPHER_CFG2: c_int = 2;
// HwCryptoKey[1:0] is mapped to cipher_do[1:0]
// HwCryptoKey[2:3] is mapped to cipher_config2[1:0]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_hw_crypto_key {
    USER_KEY = 0,			/* 0x0000 */
    ROOT_KEY = 1,			/* 0x0001 */
    PROVISIONING_KEY = 2,		/* 0x0010 */ /* ==KCP */
    SESSION_KEY = 3,		/* 0x0011 */
    RESERVED_KEY = 4,		/* NA */
    PLATFORM_KEY = 5,		/* 0x0101 */
    CUSTOMER_KEY = 6,		/* 0x0110 */
    KFDE0_KEY = 7,			/* 0x0111 */
    KFDE1_KEY = 9,			/* 0x1001 */
    KFDE2_KEY = 10,			/* 0x1010 */
    KFDE3_KEY = 11,			/* 0x1011 */
    END_OF_KEYS = S32_MAX,
}

pub const CC_NUM_HW_KEY_SLOTS: c_int = 4;
pub const CC_FIRST_HW_KEY_SLOT: c_int = 0;

pub const CC_NUM_CPP_KEY_SLOTS: c_int = 8;
pub const CC_FIRST_CPP_KEY_SLOT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_hw_aes_key_size {
    AES_128_KEY = 0,
    AES_192_KEY = 1,
    AES_256_KEY = 2,
    END_OF_AES_KEYS = S32_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_hash_cipher_pad {
    DO_NOT_PAD = 0,
    DO_PAD = 1,
    HASH_CIPHER_DO_PADDING_RESERVE32 = S32_MAX,
}

pub const CC_CPP_DIN_ADDR: c_uint = 0xFF00FF00UL;
pub const CC_CPP_DIN_SIZE: c_uint = 0xFF00FFUL;
//
// Descriptor packing macros
//
// hw_desc_init() - Init a HW descriptor struct
// @pdesc: pointer to HW descriptor struct
//
// set_queue_last_ind_bit() - Indicate the end of current HW descriptors flow
// and release the HW engines.
//
// @pdesc: Pointer to HW descriptor struct
//
// set_din_type() - Set the DIN field of a HW descriptor
//
// @pdesc: Pointer to HW descriptor struct
// @dma_mode: The DMA mode: NO_DMA, SRAM, DLLI, MLLI, CONSTANT
// @addr: DIN address
// @size: Data size in bytes
// @axi_sec: AXI secure bit
//

//
// set_din_no_dma() - Set the DIN field of a HW descriptor to NO DMA mode.
// Used for NOP descriptor, register patches and other special modes.
//
// @pdesc: Pointer to HW descriptor struct
// @addr: DIN address
// @size: Data size in bytes
//
// set_cpp_crypto_key() - Setup the special CPP descriptor
//
// @pdesc: Pointer to HW descriptor struct
// @slot: Slot number
//
// set_din_sram() - Set the DIN field of a HW descriptor to SRAM mode.
// Note: No need to check SRAM alignment since host requests do not use SRAM and
// the adaptor will enforce alignment checks.
//
// @pdesc: Pointer to HW descriptor struct
// @addr: DIN address
// @size: Data size in bytes
//
// set_din_const() - Set the DIN field of a HW descriptor to CONST mode
//
// @pdesc: Pointer to HW descriptor struct
// @val: DIN const value
// @size: Data size in bytes
//
// set_din_not_last_indication() - Set the DIN not last input data indicator
//
// @pdesc: Pointer to HW descriptor struct
//
// set_dout_type() - Set the DOUT field of a HW descriptor
//
// @pdesc: Pointer to HW descriptor struct
// @dma_mode: The DMA mode: NO_DMA, SRAM, DLLI, MLLI, CONSTANT
// @addr: DOUT address
// @size: Data size in bytes
// @axi_sec: AXI secure bit
//

//
// set_dout_dlli() - Set the DOUT field of a HW descriptor to DLLI type
// The LAST INDICATION is provided by the user
//
// @pdesc: Pointer to HW descriptor struct
// @addr: DOUT address
// @size: Data size in bytes
// @axi_sec: AXI secure bit
// @last_ind: The last indication bit
//
// set_dout_mlli() - Set the DOUT field of a HW descriptor to MLLI type
// The LAST INDICATION is provided by the user
//
// @pdesc: Pointer to HW descriptor struct
// @addr: DOUT address
// @size: Data size in bytes
// @axi_sec: AXI secure bit
// @last_ind: The last indication bit
//
// set_dout_no_dma() - Set the DOUT field of a HW descriptor to NO DMA mode.
// Used for NOP descriptor, register patches and other special modes.
//
// @pdesc: pointer to HW descriptor struct
// @addr: DOUT address
// @size: Data size in bytes
// @write_enable: Enables a write operation to a register
//
// set_xor_val() - Set the word for the XOR operation.
//
// @pdesc: Pointer to HW descriptor struct
// @val: XOR data value
//
// set_xor_active() - Set the XOR indicator bit in the descriptor
//
// @pdesc: Pointer to HW descriptor struct
//
// set_aes_not_hash_mode() - Select the AES engine instead of HASH engine when
// setting up combined mode with AES XCBC MAC
//
// @pdesc: Pointer to HW descriptor struct
//
// set_aes_xor_crypto_key() - Set aes xor crypto key, which in some scenarios
// selects the SM3 engine
//
// @pdesc: Pointer to HW descriptor struct
//
// set_dout_sram() - Set the DOUT field of a HW descriptor to SRAM mode
// Note: No need to check SRAM alignment since host requests do not use SRAM and
// the adaptor will enforce alignment checks.
//
// @pdesc: Pointer to HW descriptor struct
// @addr: DOUT address
// @size: Data size in bytes
//
// set_xex_data_unit_size() - Set the data unit size for XEX mode in
// data_out_addr[15:0]
//
// @pdesc: Pointer to HW descriptor struct
// @size: Data unit size for XEX mode
//
// set_multi2_num_rounds() - Set the number of rounds for Multi2 in
// data_out_addr[15:0]
//
// @pdesc: Pointer to HW descriptor struct
// @num: Number of rounds for Multi2
//
// set_flow_mode() - Set the flow mode.
//
// @pdesc: Pointer to HW descriptor struct
// @mode: Any one of the modes defined in [CC7x-DESC]
//
// set_cipher_mode() - Set the cipher mode.
//
// @pdesc: Pointer to HW descriptor struct
// @mode: Any one of the modes defined in [CC7x-DESC]
//
// set_hash_cipher_mode() - Set the cipher mode for hash algorithms.
//
// @pdesc: Pointer to HW descriptor struct
// @cipher_mode: Any one of the modes defined in [CC7x-DESC]
// @hash_mode: specifies which hash is being handled
//
// set_cipher_config0() - Set the cipher configuration fields.
//
// @pdesc: Pointer to HW descriptor struct
// @mode: Any one of the modes defined in [CC7x-DESC]
//
// set_cipher_config1() - Set the cipher configuration fields.
//
// @pdesc: Pointer to HW descriptor struct
// @config: Padding mode
//
// set_hw_crypto_key() - Set HW key configuration fields.
//
// @pdesc: Pointer to HW descriptor struct
// @hw_key: The HW key slot asdefined in enum cc_hw_crypto_key
//
// set_bytes_swap() - Set byte order of all setup-finalize descriptors.
//
// @pdesc: Pointer to HW descriptor struct
// @config: True to enable byte swapping
//
// set_cmac_size0_mode() - Set CMAC_SIZE0 mode.
//
// @pdesc: Pointer to HW descriptor struct
//
// set_key_size() - Set key size descriptor field.
//
// @pdesc: Pointer to HW descriptor struct
// @size: Key size in bytes (NOT size code)
//
// set_key_size_aes() - Set AES key size.
//
// @pdesc: Pointer to HW descriptor struct
// @size: Key size in bytes (NOT size code)
//
// set_key_size_des() - Set DES key size.
//
// @pdesc: Pointer to HW descriptor struct
// @size: Key size in bytes (NOT size code)
//
// set_setup_mode() - Set the descriptor setup mode
//
// @pdesc: Pointer to HW descriptor struct
// @mode: Any one of the setup modes defined in [CC7x-DESC]
//
// set_cipher_do() - Set the descriptor cipher DO
//
// @pdesc: Pointer to HW descriptor struct
// @config: Any one of the cipher do defined in [CC7x-DESC]
//
