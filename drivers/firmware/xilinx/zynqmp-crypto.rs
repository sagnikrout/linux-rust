//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/xilinx/zynqmp-crypto.c
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
// Firmware layer for XilSecure APIs.
//
// Copyright (C) 2014-2022 Xilinx, Inc.
// Copyright (C) 2022-2025 Advanced Micro Devices, Inc.
//

//
// zynqmp_pm_aes_engine - Access AES hardware to encrypt/decrypt the data using
// AES-GCM core.
// @address:	Address of the AesParams structure.
// @out:	Returned output value
//
// Return:	Returns status, either success or error code.
//
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_aes_engine(address: u64, out: *mut u32) -> c_int {
    int zynqmp_pm_aes_engine(const u64 address, u32 *out)
    {
    u32 ret_payload[PAYLOAD_ARG_CNT];
    int ret;
    if (!out)
    return -EINVAL;
    ret = zynqmp_pm_invoke_fn(PM_SECURE_AES, ret_payload, 2, upper_32_bits(address),
    lower_32_bits(address));
// out = ret_payload[1];
    return ret;
    }
    EXPORT_SYMBOL_GPL(zynqmp_pm_aes_engine);
//
// zynqmp_pm_sha_hash - Access the SHA engine to calculate the hash
// @address:	Address of the data/ Address of output buffer where
// hash should be stored.
// @size:	Size of the data.
// @flags:
// BIT(0) - for initializing csudma driver and SHA3(Here address
// and size inputs can be NULL).
// BIT(1) - to call Sha3_Update API which can be called multiple
// times when data is not contiguous.
// BIT(2) - to get final hash of the whole updated data.
// Hash will be overwritten at provided address with
// 48 bytes.
//
// Return:	Returns status, either success or error code.
//
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_sha_hash(address: u64, size: u32, flags: u32) -> c_int {
    int zynqmp_pm_sha_hash(const u64 address, const u32 size, const u32 flags)
    {
    let mut lower_addr: u32 = lower_32_bits(address);
    let mut upper_addr: u32 = upper_32_bits(address);
    return zynqmp_pm_invoke_fn(PM_SECURE_SHA, core::ptr::null_mut(), 4, upper_addr, lower_addr, size, flags);
    }
    EXPORT_SYMBOL_GPL(zynqmp_pm_sha_hash);
//
// xlnx_get_crypto_dev_data() - Get crypto dev data of platform
// @feature_map:	List of available feature map of all platform
//
// Return: Returns crypto dev data, either address crypto dev or ERR PTR
//
    void *xlnx_get_crypto_dev_data(struct xlnx_feature *feature_map)
    {
    struct xlnx_feature *feature;
    u32 pm_family_code;
    int ret;
// Get the Family code and sub family code of platform
    ret = zynqmp_pm_get_family_info(&pm_family_code);
    if (ret < 0)
    return ERR_PTR(ret);
    feature = feature_map;
    for (; feature.family; feature++) {
    if (feature.family == pm_family_code) {
    ret = zynqmp_pm_feature(feature.feature_id);
    if (ret < 0)
    return ERR_PTR(ret);
    return feature.data;
    }
    }
    return ERR_PTR(-ENODEV);
    }
    EXPORT_SYMBOL_GPL(xlnx_get_crypto_dev_data);
//
// versal_pm_aes_key_write - Write AES key registers
// @keylen:	Size of the input key to be written
// @keysrc:	Key Source to be selected to which provided
// key should be updated
// @keyaddr:	Address of a buffer which should contain the key
// to be written
//
// This function provides support to write AES volatile user keys.
//
// Return: Returns status, either success or error+reason
//
    int versal_pm_aes_key_write(const u32 keylen,
    const u32 keysrc, const u64 keyaddr)
    {
    return zynqmp_pm_invoke_fn(XSECURE_API_AES_WRITE_KEY, core::ptr::null_mut(), 4,
    keylen, keysrc,
    lower_32_bits(keyaddr),
    upper_32_bits(keyaddr));
    }
    EXPORT_SYMBOL_GPL(versal_pm_aes_key_write);
//
// versal_pm_aes_key_zero - Zeroise AES User key registers
// @keysrc:	Key Source to be selected to which provided
// key should be updated
//
// This function provides support to zeroise AES volatile user keys.
//
// Return: Returns status, either success or error+reason
//
#[no_mangle]
pub unsafe extern "C" fn versal_pm_aes_key_zero(keysrc: u32) -> c_int {
    int versal_pm_aes_key_zero(const u32 keysrc)
    {
    return zynqmp_pm_invoke_fn(XSECURE_API_AES_KEY_ZERO, core::ptr::null_mut(), 1, keysrc);
    }
    EXPORT_SYMBOL_GPL(versal_pm_aes_key_zero);
//
// versal_pm_aes_op_init - Init AES operation
// @hw_req:	AES op init structure address
//
// This function provides support to init AES operation.
//
// Return: Returns status, either success or error+reason
//
#[no_mangle]
pub unsafe extern "C" fn versal_pm_aes_op_init(hw_req: u64) -> c_int {
    int versal_pm_aes_op_init(const u64 hw_req)
    {
    return zynqmp_pm_invoke_fn(XSECURE_API_AES_OP_INIT, core::ptr::null_mut(), 2,
    lower_32_bits(hw_req),
    upper_32_bits(hw_req));
    }
    EXPORT_SYMBOL_GPL(versal_pm_aes_op_init);
//
// versal_pm_aes_update_aad - AES update aad
// @aad_addr:	AES aad address
// @aad_len:	AES aad data length
//
// This function provides support to update AAD data.
//
// Return: Returns status, either success or error+reason
//
#[no_mangle]
pub unsafe extern "C" fn versal_pm_aes_update_aad(aad_addr: u64, aad_len: u32) -> c_int {
    int versal_pm_aes_update_aad(const u64 aad_addr, const u32 aad_len)
    {
    return zynqmp_pm_invoke_fn(XSECURE_API_AES_UPDATE_AAD, core::ptr::null_mut(), 3,
    lower_32_bits(aad_addr),
    upper_32_bits(aad_addr),
    aad_len);
    }
    EXPORT_SYMBOL_GPL(versal_pm_aes_update_aad);
//
// versal_pm_aes_enc_update - Access AES hardware to encrypt the data using
// AES-GCM core.
// @in_params:	Address of the AesParams structure
// @in_addr:	Address of input buffer
//
// Return:	Returns status, either success or error code.
//
#[no_mangle]
pub unsafe extern "C" fn versal_pm_aes_enc_update(in_params: u64, in_addr: u64) -> c_int {
    int versal_pm_aes_enc_update(const u64 in_params, const u64 in_addr)
    {
    return zynqmp_pm_invoke_fn(XSECURE_API_AES_ENCRYPT_UPDATE, core::ptr::null_mut(), 4,
    lower_32_bits(in_params),
    upper_32_bits(in_params),
    lower_32_bits(in_addr),
    upper_32_bits(in_addr));
    }
    EXPORT_SYMBOL_GPL(versal_pm_aes_enc_update);
//
// versal_pm_aes_enc_final - Access AES hardware to store the GCM tag
// @gcm_addr:	Address of the gcm tag
//
// Return:	Returns status, either success or error code.
//
#[no_mangle]
pub unsafe extern "C" fn versal_pm_aes_enc_final(gcm_addr: u64) -> c_int {
    int versal_pm_aes_enc_final(const u64 gcm_addr)
    {
    return zynqmp_pm_invoke_fn(XSECURE_API_AES_ENCRYPT_FINAL, core::ptr::null_mut(), 2,
    lower_32_bits(gcm_addr),
    upper_32_bits(gcm_addr));
    }
    EXPORT_SYMBOL_GPL(versal_pm_aes_enc_final);
//
// versal_pm_aes_dec_update - Access AES hardware to decrypt the data using
// AES-GCM core.
// @in_params:	Address of the AesParams structure
// @in_addr:	Address of input buffer
//
// Return:	Returns status, either success or error code.
//
#[no_mangle]
pub unsafe extern "C" fn versal_pm_aes_dec_update(in_params: u64, in_addr: u64) -> c_int {
    int versal_pm_aes_dec_update(const u64 in_params, const u64 in_addr)
    {
    return zynqmp_pm_invoke_fn(XSECURE_API_AES_DECRYPT_UPDATE, core::ptr::null_mut(), 4,
    lower_32_bits(in_params),
    upper_32_bits(in_params),
    lower_32_bits(in_addr),
    upper_32_bits(in_addr));
    }
    EXPORT_SYMBOL_GPL(versal_pm_aes_dec_update);
//
// versal_pm_aes_dec_final - Access AES hardware to get the GCM tag
// @gcm_addr:	Address of the gcm tag
//
// Return:	Returns status, either success or error code.
//
#[no_mangle]
pub unsafe extern "C" fn versal_pm_aes_dec_final(gcm_addr: u64) -> c_int {
    int versal_pm_aes_dec_final(const u64 gcm_addr)
    {
    return zynqmp_pm_invoke_fn(XSECURE_API_AES_DECRYPT_FINAL, core::ptr::null_mut(), 2,
    lower_32_bits(gcm_addr),
    upper_32_bits(gcm_addr));
    }
    EXPORT_SYMBOL_GPL(versal_pm_aes_dec_final);
//
// versal_pm_aes_init - Init AES block
//
// This function initialise AES block.
//
// Return: Returns status, either success or error+reason
//
#[no_mangle]
pub unsafe extern "C" fn versal_pm_aes_init() -> c_int {
    int versal_pm_aes_init(void)
    {
    return zynqmp_pm_invoke_fn(XSECURE_API_AES_INIT, core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL_GPL(versal_pm_aes_init);
