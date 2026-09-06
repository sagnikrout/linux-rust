//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/ralink/rt2x00/rt2x00crypto.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
    Copyright (C) 2004 - 2009 Ivo van Doorn <IvDoorn@gmail.com>
    <http://rt2x00.serialmonkey.com>
//
    Module: rt2x00lib
    Abstract: rt2x00 crypto specific routines.
//

#[no_mangle]
pub unsafe extern "C" fn rt2x00crypto_key_to_cipher(key: *mut ieee80211_key_conf) -> enum cipher {
    enum cipher rt2x00crypto_key_to_cipher(struct ieee80211_key_conf *key)
    {
    switch (key.cipher) {
    case WLAN_CIPHER_SUITE_WEP40:
    return CIPHER_WEP64;
    case WLAN_CIPHER_SUITE_WEP104:
    return CIPHER_WEP128;
    case WLAN_CIPHER_SUITE_TKIP:
    return CIPHER_TKIP;
    case WLAN_CIPHER_SUITE_CCMP:
    return CIPHER_AES;
    default:
    return CIPHER_NONE;
    }
    }
    void rt2x00crypto_create_tx_descriptor(struct rt2x00_dev *rt2x00dev,
    struct sk_buff *skb,
    struct txentry_desc *txdesc)
    {
    struct ieee80211_tx_info *tx_info = IEEE80211_SKB_CB(skb);
    struct ieee80211_key_conf *hw_key = tx_info.control.hw_key;
    if (!rt2x00_has_cap_hw_crypto(rt2x00dev) || !hw_key)
    return;
    __set_bit(ENTRY_TXD_ENCRYPT, &txdesc.flags);
    txdesc.cipher = rt2x00crypto_key_to_cipher(hw_key);
    if (hw_key.flags & IEEE80211_KEY_FLAG_PAIRWISE)
    __set_bit(ENTRY_TXD_ENCRYPT_PAIRWISE, &txdesc.flags);
    txdesc.key_idx = hw_key.hw_key_idx;
    txdesc.iv_offset = txdesc.header_length;
    txdesc.iv_len = hw_key.iv_len;
    if (!(hw_key.flags & IEEE80211_KEY_FLAG_GENERATE_IV))
    __set_bit(ENTRY_TXD_ENCRYPT_IV, &txdesc.flags);
    if (!(hw_key.flags & IEEE80211_KEY_FLAG_GENERATE_MMIC))
    __set_bit(ENTRY_TXD_ENCRYPT_MMIC, &txdesc.flags);
    }
    unsigned int rt2x00crypto_tx_overhead(struct rt2x00_dev *rt2x00dev,
    struct sk_buff *skb)
    {
    struct ieee80211_tx_info *tx_info = IEEE80211_SKB_CB(skb);
    struct ieee80211_key_conf *key = tx_info.control.hw_key;
    let mut overhead: c_uint = 0;
    if (!rt2x00_has_cap_hw_crypto(rt2x00dev) || !key)
    return overhead;
//
// Extend frame length to include IV/EIV/ICV/MMIC,
// note that these lengths should only be added when
// mac80211 does not generate it.
//
    overhead += key.icv_len;
    if (!(key.flags & IEEE80211_KEY_FLAG_GENERATE_IV))
    overhead += key.iv_len;
    if (!(key.flags & IEEE80211_KEY_FLAG_GENERATE_MMIC)) {
    if (key.cipher == WLAN_CIPHER_SUITE_TKIP)
    overhead += 8;
    }
    return overhead;
    }
#[no_mangle]
pub unsafe extern "C" fn rt2x00crypto_tx_copy_iv(skb: *mut sk_buff, txdesc: *mut txentry_desc) {
    void rt2x00crypto_tx_copy_iv(struct sk_buff *skb, struct txentry_desc *txdesc)
    {
    struct skb_frame_desc *skbdesc = get_skb_frame_desc(skb);
    if (unlikely(!txdesc.iv_len))
    return;
// Copy IV/EIV data
    memcpy(skbdesc.iv, skb.data + txdesc.iv_offset, txdesc.iv_len);
    }
#[no_mangle]
pub unsafe extern "C" fn rt2x00crypto_tx_remove_iv(skb: *mut sk_buff, txdesc: *mut txentry_desc) {
    void rt2x00crypto_tx_remove_iv(struct sk_buff *skb, struct txentry_desc *txdesc)
    {
    struct skb_frame_desc *skbdesc = get_skb_frame_desc(skb);
    if (unlikely(!txdesc.iv_len))
    return;
// Copy IV/EIV data
    memcpy(skbdesc.iv, skb.data + txdesc.iv_offset, txdesc.iv_len);
// Move ieee80211 header
    memmove(skb.data + txdesc.iv_len, skb.data, txdesc.iv_offset);
// Pull buffer to correct size
    skb_pull(skb, txdesc.iv_len);
    txdesc.length -= txdesc.iv_len;
// IV/EIV data has officially been stripped
    skbdesc.flags |= SKBDESC_IV_STRIPPED;
    }
#[no_mangle]
pub unsafe extern "C" fn rt2x00crypto_tx_insert_iv(skb: *mut sk_buff, header_length: c_uint) {
    void rt2x00crypto_tx_insert_iv(struct sk_buff *skb, unsigned int header_length)
    {
    struct skb_frame_desc *skbdesc = get_skb_frame_desc(skb);
    const unsigned int iv_len =
    ((!!(skbdesc.iv[0])) * 4) + ((!!(skbdesc.iv[1])) * 4);
    if (!(skbdesc.flags & SKBDESC_IV_STRIPPED))
    return;
    skb_push(skb, iv_len);
// Move ieee80211 header
    memmove(skb.data, skb.data + iv_len, header_length);
// Copy IV/EIV data
    memcpy(skb.data + header_length, skbdesc.iv, iv_len);
// IV/EIV data has returned into the frame
    skbdesc.flags &= ~SKBDESC_IV_STRIPPED;
    }
    void rt2x00crypto_rx_insert_iv(struct sk_buff *skb,
    unsigned int header_length,
    struct rxdone_entry_desc *rxdesc)
    {
    let mut payload_len: c_uint = rxdesc.size - header_length;
    let mut align: c_uint = ALIGN_SIZE(skb, header_length);
    unsigned int iv_len;
    unsigned int icv_len;
    let mut transfer: c_uint = 0;
//
// WEP64/WEP128: Provides IV & ICV
// TKIP: Provides IV/EIV & ICV
// AES: Provies IV/EIV & ICV
//
    switch (rxdesc.cipher) {
    case CIPHER_WEP64:
    case CIPHER_WEP128:
    iv_len = 4;
    icv_len = 4;
    break;
    case CIPHER_TKIP:
    iv_len = 8;
    icv_len = 4;
    break;
    case CIPHER_AES:
    iv_len = 8;
    icv_len = 8;
    break;
    default:
// Unsupport type
    return;
    }
//
// Make room for new data. There are 2 possibilities
// either the alignment is already present between
// the 802.11 header and payload. In that case we
// have to move the header less than the iv_len
// since we can use the already available l2pad bytes
// for the iv data.
// When the alignment must be added manually we must
// move the header more then iv_len since we must
// make room for the payload move as well.
//
    if (rxdesc.dev_flags & RXDONE_L2PAD) {
    skb_push(skb, iv_len - align);
    skb_put(skb, icv_len);
// Move ieee80211 header
    memmove(skb.data + transfer,
    skb.data + transfer + (iv_len - align),
    header_length);
    transfer += header_length;
    } else {
    skb_push(skb, iv_len + align);
    skb_put(skb, icv_len - align);
// Move ieee80211 header
    memmove(skb.data + transfer,
    skb.data + transfer + iv_len + align,
    header_length);
    transfer += header_length;
    }
// Copy IV/EIV data
    memcpy(skb.data + transfer, rxdesc.iv, iv_len);
    transfer += iv_len;
//
// Move payload for alignment purposes. Note that
// this is only needed when no l2 padding is present.
//
    if (!(rxdesc.dev_flags & RXDONE_L2PAD)) {
    memmove(skb.data + transfer,
    skb.data + transfer + align,
    payload_len);
    }
//
// NOTE: Always count the payload as transferred,
// even when alignment was set to zero. This is required
// for determining the correct offset for the ICV data.
//
    transfer += payload_len;
//
// Copy ICV data
// AES appends 8 bytes, we can't fill the upper
// 4 bytes, but mac80211 doesn't care about what
// we provide here anyway and strips it immediately.
//
    memcpy(skb.data + transfer, &rxdesc.icv, 4);
    transfer += icv_len;
// IV/EIV/ICV has been inserted into frame
    rxdesc.size = transfer;
    rxdesc.flags &= ~RX_FLAG_IV_STRIPPED;
    }
