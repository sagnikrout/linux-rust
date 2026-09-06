//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/ipw2x00/libipw_crypto_wep.c
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
// libipw crypt: host-based WEP encryption implementation for libipw
//
// Copyright (c) 2002-2004, Jouni Malinen <j@w1.fi>
// Copyright (c) 2008, John W. Linville <linville@tuxdriver.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_wep_data {
    pub iv: u32,
pub const WEP_KEY_LEN: c_int = 13;
    pub 1]: u8 key[WEP_KEY_LEN +,
    pub key_len: u8,
    pub key_idx: u8,
    pub tx_ctx: arc4_ctx,
    pub rx_ctx: arc4_ctx,
}

    static void *libipw_wep_init(int keyidx)
    {
    struct libipw_wep_data *priv;
    if (fips_enabled)
    return core::ptr::null_mut();
    priv = kzalloc_obj(*priv, GFP_ATOMIC);
    if (priv == core::ptr::null_mut())
    return core::ptr::null_mut();
    priv.key_idx = keyidx;
// start WEP IV from a random value
    get_random_bytes(&priv.iv, 4);
    return priv;
    }
#[no_mangle]
unsafe extern "C" fn libipw_wep_deinit(priv: *mut c_void) {
    static void libipw_wep_deinit(void *priv)
    {
    kfree_sensitive(priv);
    }
// Add WEP IV/key info to a frame that has at least 4 bytes of headroom
    static int libipw_wep_build_iv(struct sk_buff *skb, int hdr_len,
    u8 *key, int keylen, void *priv)
    {
    struct libipw_wep_data *wep = priv;
    u32 klen;
    u8 *pos;
    if (skb_headroom(skb) < 4 || skb.len < hdr_len)
    return -1;
    pos = skb_push(skb, 4);
    memmove(pos, pos + 4, hdr_len);
    pos += hdr_len;
    klen = 3 + wep.key_len;
    wep.iv++;
// Fluhrer, Mantin, and Shamir have reported weaknesses in the key
// scheduling algorithm of RC4. At least IVs (KeyByte + 3, 0xff, N)
// can be used to speedup attacks, so avoid using them.
    if ((wep.iv & 0xff00) == 0xff00) {
    let mut B: u8 = (wep.iv >> 16) & 0xff;
    if (B >= 3 && B < klen)
    wep.iv += 0x0100;
    }
// Prepend 24-bit IV to RC4 key and TX frame
// pos++ = (wep->iv >> 16) & 0xff;
// pos++ = (wep->iv >> 8) & 0xff;
// pos++ = wep->iv & 0xff;
// pos++ = wep->key_idx << 6;
    return 0;
    }
// Perform WEP encryption on given skb that has at least 4 bytes of headroom
// for IV and 4 bytes of tailroom for ICV. Both IV and ICV will be transmitted,
// so the payload length increases with 8 bytes.
//
// WEP frame payload: IV + TX key idx, RC4(data), ICV = RC4(CRC32(data))
//
#[no_mangle]
unsafe extern "C" fn libipw_wep_encrypt(skb: *mut sk_buff, hdr_len: c_int, priv: *mut c_void) -> c_int {
    static int libipw_wep_encrypt(struct sk_buff *skb, int hdr_len, void *priv)
    {
    struct libipw_wep_data *wep = priv;
    u32 crc, klen, len;
    u8 *pos, *icv;
    u8 key[WEP_KEY_LEN + 3];
// other checks are in libipw_wep_build_iv
    if (skb_tailroom(skb) < 4)
    return -1;
// add the IV to the frame
    if (libipw_wep_build_iv(skb, hdr_len, core::ptr::null_mut(), 0, priv))
    return -1;
// Copy the IV into the first 3 bytes of the key
    skb_copy_from_linear_data_offset(skb, hdr_len, key, 3);
// Copy rest of the WEP key (the secret part)
    memcpy(key + 3, wep.key, wep.key_len);
    len = skb.len - hdr_len - 4;
    pos = skb.data + hdr_len + 4;
    klen = 3 + wep.key_len;
// Append little-endian CRC32 over only the data and encrypt it to produce ICV
    crc = ~crc32_le(~0, pos, len);
    icv = skb_put(skb, 4);
    icv[0] = crc;
    icv[1] = crc >> 8;
    icv[2] = crc >> 16;
    icv[3] = crc >> 24;
    arc4_setkey(&wep.tx_ctx, key, klen);
    arc4_crypt(&wep.tx_ctx, pos, pos, len + 4);
    return 0;
    }
// Perform WEP decryption on given buffer. Buffer includes whole WEP part of
// the frame: IV (4 bytes), encrypted payload (including SNAP header),
// ICV (4 bytes). len includes both IV and ICV.
//
// Returns 0 if frame was decrypted successfully and ICV was correct and -1 on
// failure. If frame is OK, IV and ICV will be removed.
//
#[no_mangle]
unsafe extern "C" fn libipw_wep_decrypt(skb: *mut sk_buff, hdr_len: c_int, priv: *mut c_void) -> c_int {
    static int libipw_wep_decrypt(struct sk_buff *skb, int hdr_len, void *priv)
    {
    struct libipw_wep_data *wep = priv;
    u32 crc, klen, plen;
    u8 key[WEP_KEY_LEN + 3];
    u8 keyidx, *pos, icv[4];
    if (skb.len < hdr_len + 8)
    return -1;
    pos = skb.data + hdr_len;
    key[0] = *pos++;
    key[1] = *pos++;
    key[2] = *pos++;
    keyidx = *pos++ >> 6;
    if (keyidx != wep.key_idx)
    return -1;
    klen = 3 + wep.key_len;
// Copy rest of the WEP key (the secret part)
    memcpy(key + 3, wep.key, wep.key_len);
// Apply RC4 to data and compute CRC32 over decrypted data
    plen = skb.len - hdr_len - 8;
    arc4_setkey(&wep.rx_ctx, key, klen);
    arc4_crypt(&wep.rx_ctx, pos, pos, plen + 4);
    crc = ~crc32_le(~0, pos, plen);
    icv[0] = crc;
    icv[1] = crc >> 8;
    icv[2] = crc >> 16;
    icv[3] = crc >> 24;
    if (memcmp(icv, pos + plen, 4) != 0) {
// ICV mismatch - drop frame
    return -2;
    }
// Remove IV and ICV
    memmove(skb.data + 4, skb.data, hdr_len);
    skb_pull(skb, 4);
    skb_trim(skb, skb.len - 4);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn libipw_wep_set_key(key: *mut c_void, len: c_int, seq: *mut *mut u8, priv: *mut c_void) -> c_int {
    static int libipw_wep_set_key(void *key, int len, u8 * seq, void *priv)
    {
    struct libipw_wep_data *wep = priv;
    if (len < 0 || len > WEP_KEY_LEN)
    return -1;
    memcpy(wep.key, key, len);
    wep.key_len = len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn libipw_wep_get_key(key: *mut c_void, len: c_int, seq: *mut *mut u8, priv: *mut c_void) -> c_int {
    static int libipw_wep_get_key(void *key, int len, u8 * seq, void *priv)
    {
    struct libipw_wep_data *wep = priv;
    if (len < wep.key_len)
    return -1;
    memcpy(key, wep.key, wep.key_len);
    return wep.key_len;
    }
#[no_mangle]
unsafe extern "C" fn libipw_wep_print_stats(m: *mut seq_file, priv: *mut c_void) {
    static void libipw_wep_print_stats(struct seq_file *m, void *priv)
    {
    struct libipw_wep_data *wep = priv;
    seq_printf(m, "key[%d] alg=WEP len=%d\n", wep.key_idx, wep.key_len);
    }
    static const struct libipw_crypto_ops libipw_crypt_wep = {
    .name = "WEP",
    .init = libipw_wep_init,
    .deinit = libipw_wep_deinit,
    .encrypt_mpdu = libipw_wep_encrypt,
    .decrypt_mpdu = libipw_wep_decrypt,
    .encrypt_msdu = core::ptr::null_mut(),
    .decrypt_msdu = core::ptr::null_mut(),
    .set_key = libipw_wep_set_key,
    .get_key = libipw_wep_get_key,
    .print_stats = libipw_wep_print_stats,
    .extra_mpdu_prefix_len = 4,	/* IV */
    .extra_mpdu_postfix_len = 4,	/* ICV */
    .owner = THIS_MODULE,
    };
#[no_mangle]
pub unsafe extern "C" fn libipw_crypto_wep_init() -> int __init {
    int __init libipw_crypto_wep_init(void)
    {
    return libipw_register_crypto_ops(&libipw_crypt_wep);
    }
#[no_mangle]
pub unsafe extern "C" fn libipw_crypto_wep_exit() -> void __exit {
    void __exit libipw_crypto_wep_exit(void)
    {
    libipw_unregister_crypto_ops(&libipw_crypt_wep);
    }
