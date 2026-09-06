//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/ipw2x00/libipw_crypto_ccmp.c
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
// libipw crypt: host-based CCMP encryption implementation for libipw
//
// Copyright (c) 2003-2004, Jouni Malinen <j@w1.fi>
// Copyright (c) 2008, John W. Linville <linville@tuxdriver.com>
//

pub const AES_BLOCK_LEN: c_int = 16;
pub const CCMP_HDR_LEN: c_int = 8;
pub const CCMP_MIC_LEN: c_int = 8;
pub const CCMP_TK_LEN: c_int = 16;
pub const CCMP_PN_LEN: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libipw_ccmp_data {
    pub key: [u8; CCMP_TK_LEN],
    pub key_set: c_int,
    pub tx_pn: [u8; CCMP_PN_LEN],
    pub rx_pn: [u8; CCMP_PN_LEN],
    pub dot11RSNAStatsCCMPFormatErrors: u32,
    pub dot11RSNAStatsCCMPReplays: u32,
    pub dot11RSNAStatsCCMPDecryptErrors: u32,
    pub key_idx: c_int,
    pub tfm: *mut crypto_aead,
// scratch buffers for virt_to_page() (crypto API)
    pub AES_BLOCK_LEN]: *mut *mut u8 tx_aad[2,
    pub AES_BLOCK_LEN]: *mut *mut u8 rx_aad[2,
}

    static void *libipw_ccmp_init(int key_idx)
    {
    struct libipw_ccmp_data *priv;
    priv = kzalloc_obj(*priv, GFP_ATOMIC);
    if (priv == core::ptr::null_mut())
    goto fail;
    priv.key_idx = key_idx;
    priv.tfm = crypto_alloc_aead("ccm(aes)", 0, CRYPTO_ALG_ASYNC);
    if (IS_ERR(priv.tfm)) {
    priv.tfm = core::ptr::null_mut();
    goto fail;
    }
    return priv;
    fail:
    if (priv) {
    if (priv.tfm)
    crypto_free_aead(priv.tfm);
    kfree(priv);
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn libipw_ccmp_deinit(priv: *mut c_void) {
    static void libipw_ccmp_deinit(void *priv)
    {
    struct libipw_ccmp_data *_priv = priv;
    if (_priv && _priv.tfm)
    crypto_free_aead(_priv.tfm);
    kfree(priv);
    }
    static int ccmp_init_iv_and_aad(const struct ieee80211_hdr *hdr,
    const u8 *pn, u8 *iv, u8 *aad)
    {
    u8 *pos, qc = 0;
    size_t aad_len;
    int a4_included, qc_included;
    a4_included = ieee80211_has_a4(hdr.frame_control);
    qc_included = ieee80211_is_data_qos(hdr.frame_control);
    aad_len = 22;
    if (a4_included)
    aad_len += 6;
    if (qc_included) {
    pos = (u8 *) & hdr.addr4;
    if (a4_included)
    pos += 6;
    qc = *pos & 0x0f;
    aad_len += 2;
    }
// In CCM, the initial vectors (IV) used for CTR mode encryption and CBC
// mode authentication are not allowed to collide, yet both are derived
// from the same vector. We only set L := 1 here to indicate that the
// data size can be represented in (L+1) bytes. The CCM layer will take
// care of storing the data length in the top (L+1) bytes and setting
// and clearing the other bits as is required to derive the two IVs.
//
    iv[0] = 0x1;
// Nonce: QC | A2 | PN
    iv[1] = qc;
    memcpy(iv + 2, hdr.addr2, ETH_ALEN);
    memcpy(iv + 8, pn, CCMP_PN_LEN);
// AAD:
// FC with bits 4..6 and 11..13 masked to zero; 14 is always one
// A1 | A2 | A3
// SC with bits 4..15 (seq#) masked to zero
// A4 (if present)
// QC (if present)
//
    pos = (u8 *) hdr;
    aad[0] = pos[0] & 0x8f;
    aad[1] = pos[1] & 0xc7;
    memcpy(aad + 2, &hdr.addrs, 3 * ETH_ALEN);
    pos = (u8 *) & hdr.seq_ctrl;
    aad[20] = pos[0] & 0x0f;
    aad[21] = 0;		/* all bits masked */
    memset(aad + 22, 0, 8);
    if (a4_included)
    memcpy(aad + 22, hdr.addr4, ETH_ALEN);
    if (qc_included) {
    aad[a4_included ? 28 : 22] = qc;
// rest of QC masked
    }
    return aad_len;
    }
    static int libipw_ccmp_hdr(struct sk_buff *skb, int hdr_len,
    u8 *aeskey, int keylen, void *priv)
    {
    struct libipw_ccmp_data *key = priv;
    int i;
    u8 *pos;
    if (skb_headroom(skb) < CCMP_HDR_LEN || skb.len < hdr_len)
    return -1;
    if (aeskey != core::ptr::null_mut() && keylen >= CCMP_TK_LEN)
    memcpy(aeskey, key.key, CCMP_TK_LEN);
    pos = skb_push(skb, CCMP_HDR_LEN);
    memmove(pos, pos + CCMP_HDR_LEN, hdr_len);
    pos += hdr_len;
    i = CCMP_PN_LEN - 1;
    while (i >= 0) {
    key.tx_pn[i]++;
    if (key.tx_pn[i] != 0)
    break;
    i--;
    }
// pos++ = key->tx_pn[5];
// pos++ = key->tx_pn[4];
// pos++ = 0;
// pos++ = (key->key_idx << 6) | (1 << 5) /* Ext IV included */ ;
// pos++ = key->tx_pn[3];
// pos++ = key->tx_pn[2];
// pos++ = key->tx_pn[1];
// pos++ = key->tx_pn[0];
    return CCMP_HDR_LEN;
    }
#[no_mangle]
unsafe extern "C" fn libipw_ccmp_encrypt(skb: *mut sk_buff, hdr_len: c_int, priv: *mut c_void) -> c_int {
    static int libipw_ccmp_encrypt(struct sk_buff *skb, int hdr_len, void *priv)
    {
    struct libipw_ccmp_data *key = priv;
    struct ieee80211_hdr *hdr;
    struct aead_request *req;
    struct scatterlist sg[2];
    u8 *aad = key.tx_aad;
    u8 iv[AES_BLOCK_LEN];
    int len, data_len, aad_len;
    int ret;
    if (skb_tailroom(skb) < CCMP_MIC_LEN || skb.len < hdr_len)
    return -1;
    data_len = skb.len - hdr_len;
    len = libipw_ccmp_hdr(skb, hdr_len, core::ptr::null_mut(), 0, priv);
    if (len < 0)
    return -1;
    req = aead_request_alloc(key.tfm, GFP_ATOMIC);
    if (!req)
    return -ENOMEM;
    hdr = (struct ieee80211_hdr *)skb.data;
    aad_len = ccmp_init_iv_and_aad(hdr, key.tx_pn, iv, aad);
    skb_put(skb, CCMP_MIC_LEN);
    sg_init_table(sg, 2);
    sg_set_buf(&sg[0], aad, aad_len);
    sg_set_buf(&sg[1], skb.data + hdr_len + CCMP_HDR_LEN,
    data_len + CCMP_MIC_LEN);
    aead_request_set_callback(req, 0, core::ptr::null_mut(), core::ptr::null_mut());
    aead_request_set_ad(req, aad_len);
    aead_request_set_crypt(req, sg, sg, data_len, iv);
    ret = crypto_aead_encrypt(req);
    aead_request_free(req);
    return ret;
    }
//
// deal with seq counter wrapping correctly.
// refer to timer_after() for jiffies wrapping handling
//
#[no_mangle]
pub unsafe extern "C" fn ccmp_replay_check(pn_n: *mut u8, pn_o: *mut u8) -> c_int {
    static inline int ccmp_replay_check(u8 *pn_n, u8 *pn_o)
    {
    u32 iv32_n, iv16_n;
    u32 iv32_o, iv16_o;
    iv32_n = (pn_n[0] << 24) | (pn_n[1] << 16) | (pn_n[2] << 8) | pn_n[3];
    iv16_n = (pn_n[4] << 8) | pn_n[5];
    iv32_o = (pn_o[0] << 24) | (pn_o[1] << 16) | (pn_o[2] << 8) | pn_o[3];
    iv16_o = (pn_o[4] << 8) | pn_o[5];
    if ((s32)iv32_n - (s32)iv32_o < 0 ||
    (iv32_n == iv32_o && iv16_n <= iv16_o))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn libipw_ccmp_decrypt(skb: *mut sk_buff, hdr_len: c_int, priv: *mut c_void) -> c_int {
    static int libipw_ccmp_decrypt(struct sk_buff *skb, int hdr_len, void *priv)
    {
    struct libipw_ccmp_data *key = priv;
    u8 keyidx, *pos;
    struct ieee80211_hdr *hdr;
    struct aead_request *req;
    struct scatterlist sg[2];
    u8 *aad = key.rx_aad;
    u8 iv[AES_BLOCK_LEN];
    u8 pn[6];
    int aad_len, ret;
    let mut data_len: usize = skb.len - hdr_len - CCMP_HDR_LEN;
    if (skb.len < hdr_len + CCMP_HDR_LEN + CCMP_MIC_LEN) {
    key.dot11RSNAStatsCCMPFormatErrors++;
    return -1;
    }
    hdr = (struct ieee80211_hdr *)skb.data;
    pos = skb.data + hdr_len;
    keyidx = pos[3];
    if (!(keyidx & (1 << 5))) {
    net_dbg_ratelimited("CCMP: received packet without ExtIV flag from %pM\n",
    hdr.addr2);
    key.dot11RSNAStatsCCMPFormatErrors++;
    return -2;
    }
    keyidx >>= 6;
    if (key.key_idx != keyidx) {
    net_dbg_ratelimited("CCMP: RX tkey.key_idx=%d frame keyidx=%d\n",
    key.key_idx, keyidx);
    return -6;
    }
    if (!key.key_set) {
    net_dbg_ratelimited("CCMP: received packet from %pM with keyid=%d that does not have a configured key\n",
    hdr.addr2, keyidx);
    return -3;
    }
    pn[0] = pos[7];
    pn[1] = pos[6];
    pn[2] = pos[5];
    pn[3] = pos[4];
    pn[4] = pos[1];
    pn[5] = pos[0];
    pos += 8;
    if (ccmp_replay_check(pn, key.rx_pn)) {

    net_dbg_ratelimited("CCMP: replay detected: STA=%pM previous PN %02x%02x%02x%02x%02x%02x received PN %02x%02x%02x%02x%02x%02x\n",
    hdr.addr2,
    key.rx_pn[0], key.rx_pn[1], key.rx_pn[2],
    key.rx_pn[3], key.rx_pn[4], key.rx_pn[5],
    pn[0], pn[1], pn[2], pn[3], pn[4], pn[5]);

    key.dot11RSNAStatsCCMPReplays++;
    return -4;
    }
    req = aead_request_alloc(key.tfm, GFP_ATOMIC);
    if (!req)
    return -ENOMEM;
    aad_len = ccmp_init_iv_and_aad(hdr, pn, iv, aad);
    sg_init_table(sg, 2);
    sg_set_buf(&sg[0], aad, aad_len);
    sg_set_buf(&sg[1], pos, data_len);
    aead_request_set_callback(req, 0, core::ptr::null_mut(), core::ptr::null_mut());
    aead_request_set_ad(req, aad_len);
    aead_request_set_crypt(req, sg, sg, data_len, iv);
    ret = crypto_aead_decrypt(req);
    aead_request_free(req);
    if (ret) {
    net_dbg_ratelimited("CCMP: decrypt failed: STA=%pM (%d)\n",
    hdr.addr2, ret);
    key.dot11RSNAStatsCCMPDecryptErrors++;
    return -5;
    }
    memcpy(key.rx_pn, pn, CCMP_PN_LEN);
// Remove hdr and MIC
    memmove(skb.data + CCMP_HDR_LEN, skb.data, hdr_len);
    skb_pull(skb, CCMP_HDR_LEN);
    skb_trim(skb, skb.len - CCMP_MIC_LEN);
    return keyidx;
    }
#[no_mangle]
unsafe extern "C" fn libipw_ccmp_set_key(key: *mut c_void, len: c_int, seq: *mut *mut u8, priv: *mut c_void) -> c_int {
    static int libipw_ccmp_set_key(void *key, int len, u8 * seq, void *priv)
    {
    struct libipw_ccmp_data *data = priv;
    int keyidx;
    struct crypto_aead *tfm = data.tfm;
    keyidx = data.key_idx;
    memset(data, 0, sizeof(*data));
    data.key_idx = keyidx;
    data.tfm = tfm;
    if (len == CCMP_TK_LEN) {
    memcpy(data.key, key, CCMP_TK_LEN);
    data.key_set = 1;
    if (seq) {
    data.rx_pn[0] = seq[5];
    data.rx_pn[1] = seq[4];
    data.rx_pn[2] = seq[3];
    data.rx_pn[3] = seq[2];
    data.rx_pn[4] = seq[1];
    data.rx_pn[5] = seq[0];
    }
    if (crypto_aead_setauthsize(data.tfm, CCMP_MIC_LEN) ||
    crypto_aead_setkey(data.tfm, data.key, CCMP_TK_LEN))
    return -1;
    } else if (len == 0)
    data.key_set = 0;
    else
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn libipw_ccmp_get_key(key: *mut c_void, len: c_int, seq: *mut *mut u8, priv: *mut c_void) -> c_int {
    static int libipw_ccmp_get_key(void *key, int len, u8 * seq, void *priv)
    {
    struct libipw_ccmp_data *data = priv;
    if (len < CCMP_TK_LEN)
    return -1;
    if (!data.key_set)
    return 0;
    memcpy(key, data.key, CCMP_TK_LEN);
    if (seq) {
    seq[0] = data.tx_pn[5];
    seq[1] = data.tx_pn[4];
    seq[2] = data.tx_pn[3];
    seq[3] = data.tx_pn[2];
    seq[4] = data.tx_pn[1];
    seq[5] = data.tx_pn[0];
    }
    return CCMP_TK_LEN;
    }
#[no_mangle]
unsafe extern "C" fn libipw_ccmp_print_stats(m: *mut seq_file, priv: *mut c_void) {
    static void libipw_ccmp_print_stats(struct seq_file *m, void *priv)
    {
    struct libipw_ccmp_data *ccmp = priv;
    seq_printf(m,
    "key[%d] alg=CCMP key_set=%d "
    "tx_pn=%02x%02x%02x%02x%02x%02x "
    "rx_pn=%02x%02x%02x%02x%02x%02x "
    "format_errors=%d replays=%d decrypt_errors=%d\n",
    ccmp.key_idx, ccmp.key_set,
    ccmp.tx_pn[0], ccmp.tx_pn[1], ccmp.tx_pn[2],
    ccmp.tx_pn[3], ccmp.tx_pn[4], ccmp.tx_pn[5],
    ccmp.rx_pn[0], ccmp.rx_pn[1], ccmp.rx_pn[2],
    ccmp.rx_pn[3], ccmp.rx_pn[4], ccmp.rx_pn[5],
    ccmp.dot11RSNAStatsCCMPFormatErrors,
    ccmp.dot11RSNAStatsCCMPReplays,
    ccmp.dot11RSNAStatsCCMPDecryptErrors);
    }
    static const struct libipw_crypto_ops libipw_crypt_ccmp = {
    .name = "CCMP",
    .init = libipw_ccmp_init,
    .deinit = libipw_ccmp_deinit,
    .encrypt_mpdu = libipw_ccmp_encrypt,
    .decrypt_mpdu = libipw_ccmp_decrypt,
    .encrypt_msdu = core::ptr::null_mut(),
    .decrypt_msdu = core::ptr::null_mut(),
    .set_key = libipw_ccmp_set_key,
    .get_key = libipw_ccmp_get_key,
    .print_stats = libipw_ccmp_print_stats,
    .extra_mpdu_prefix_len = CCMP_HDR_LEN,
    .extra_mpdu_postfix_len = CCMP_MIC_LEN,
    .owner = THIS_MODULE,
    };
#[no_mangle]
pub unsafe extern "C" fn libipw_crypto_ccmp_init() -> int __init {
    int __init libipw_crypto_ccmp_init(void)
    {
    return libipw_register_crypto_ops(&libipw_crypt_ccmp);
    }
#[no_mangle]
pub unsafe extern "C" fn libipw_crypto_ccmp_exit() {
    void libipw_crypto_ccmp_exit(void)
    {
    libipw_unregister_crypto_ops(&libipw_crypt_ccmp);
    }
