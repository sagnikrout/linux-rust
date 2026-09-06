//! Automatically rewritten from C to Rust
//! Source: fs/smb/client/cifsencrypt.c
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


// SPDX-License-Identifier: LGPL-2.1
//
// Encryption and hashing operations relating to NTLM, NTLMv2.  See MS-NLMP
// for more detailed information
//
// Copyright (C) International Business Machines  Corp., 2005,2013
// Author(s): Steve French (sfrench@us.ibm.com)
//

    static size_t cifs_sig_step(void *iter_base, size_t progress, size_t len,
    void *priv, void *priv2)
    {
    struct cifs_calc_sig_ctx *ctx = priv;
    if (ctx.md5)
    md5_update(ctx.md5, iter_base, len);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ctx->hmac) -> else {
    else if (ctx.hmac)
    hmac_sha256_update(ctx.hmac, iter_base, len);
    else
    aes_cmac_update(ctx.cmac, iter_base, len);
    return 0; /* Return value is length *not* processed, i.e. 0. */
    }
#[no_mangle]
unsafe extern "C" fn cifs_sig_final(ctx: *mut cifs_calc_sig_ctx, out: *mut u8) {
    static void cifs_sig_final(struct cifs_calc_sig_ctx *ctx, u8 *out)
    {
    if (ctx.md5)
    md5_final(ctx.md5, out);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ctx->hmac) -> else {
    else if (ctx.hmac)
    hmac_sha256_final(ctx.hmac, out);
    else
    aes_cmac_final(ctx.cmac, out);
    }
//
// Pass the data from an iterator into a hash.
//
    static int cifs_sig_iter(const struct iov_iter *iter, size_t maxsize,
    struct cifs_calc_sig_ctx *ctx)
    {
    let mut tmp_iter: iov_iter = *iter;
    size_t did;
    did = iterate_and_advance_kernel(&tmp_iter, maxsize, ctx, core::ptr::null_mut(),
    cifs_sig_step);
    if (did != maxsize)
    return smb_EIO2(smb_eio_trace_sig_iter, did, maxsize);
    return 0;
    }
    int __cifs_calc_signature(struct smb_rqst *rqst, struct TCP_Server_Info *server,
    char *signature, struct cifs_calc_sig_ctx *ctx)
    {
    struct iov_iter iter;
    ssize_t rc;
    let mut size: usize = 0;
    for (int i = 0; i < rqst.rq_nvec; i++)
    size += rqst.rq_iov[i].iov_len;
    iov_iter_kvec(&iter, ITER_SOURCE, rqst.rq_iov, rqst.rq_nvec, size);
    if (iov_iter_count(&iter) <= 4)
    return smb_EIO2(smb_eio_trace_sig_data_too_small,
    iov_iter_count(&iter), 4);
    rc = cifs_sig_iter(&iter, iov_iter_count(&iter), ctx);
    if (rc < 0)
    return rc;
    rc = cifs_sig_iter(&rqst.rq_iter, iov_iter_count(&rqst.rq_iter), ctx);
    if (rc < 0)
    return rc;
    cifs_sig_final(ctx, signature);
    return 0;
    }
// Build a proper attribute value/target info pairs blob.
// Fill in netbios and dns domain name and workstation name
// and client time (total five av pairs and + one end of fields indicator.
// Allocate domain name which gets freed when session struct is deallocated.
//
    static int
    build_avpair_blob(struct cifs_ses *ses, const struct nls_table *nls_cp)
    {
    unsigned int dlen;
    let mut size: c_uint = 2 * sizeof(struct ntlmssp2_name);
    char *defdmname = "WORKGROUP";
    unsigned char *blobptr;
    struct ntlmssp2_name *attrptr;
    if (!ses.domainName) {
    ses.domainName = kstrdup(defdmname, GFP_KERNEL);
    if (!ses.domainName)
    return -ENOMEM;
    }
    dlen = strlen(ses.domainName);
//
// The length of this blob is two times the size of a
// structure (av pair) which holds name/size
// ( for NTLMSSP_AV_NB_DOMAIN_NAME followed by NTLMSSP_AV_EOL ) +
// unicode length of a netbios domain name
//
    kfree_sensitive(ses.auth_key.response);
    ses.auth_key.len = size + 2 * dlen;
    ses.auth_key.response = kzalloc(ses.auth_key.len, GFP_KERNEL);
    if (!ses.auth_key.response) {
    ses.auth_key.len = 0;
    return -ENOMEM;
    }
    blobptr = ses.auth_key.response;
    attrptr = (struct ntlmssp2_name *) blobptr;
//
// As defined in MS-NTLM 3.3.2, just this av pair field
// is sufficient as part of the temp
//
    attrptr.type = cpu_to_le16(NTLMSSP_AV_NB_DOMAIN_NAME);
    attrptr.length = cpu_to_le16(2 * dlen);
    blobptr = (unsigned char *)attrptr + sizeof(struct ntlmssp2_name);
    cifs_strtoUTF16((__le16 *)blobptr, ses.domainName, dlen, nls_cp);
    return 0;
    }

    for (av = core::ptr::null_mut(); (av = find_next_av(ses, av));)
    static struct ntlmssp2_name *find_next_av(struct cifs_ses *ses,
    struct ntlmssp2_name *av)
    {
    u16 len;
    u8 *end;
    end = (u8 *)ses.auth_key.response + ses.auth_key.len;
    if (!av) {
    if (unlikely(!ses.auth_key.response || !ses.auth_key.len))
    return core::ptr::null_mut();
    av = (void *)ses.auth_key.response;
    } else {
    av = (void *)((u8 *)av + sizeof(*av) + AV_LEN(av));
    }
    if ((u8 *)av + sizeof(*av) > end)
    return core::ptr::null_mut();
    len = AV_LEN(av);
    if (AV_TYPE(av) == NTLMSSP_AV_EOL)
    return core::ptr::null_mut();
    if ((u8 *)av + sizeof(*av) + len > end)
    return core::ptr::null_mut();
    return av;
    }
//
// Check if server has provided av pair of @type in the NTLMSSP
// CHALLENGE_MESSAGE blob.
//
#[no_mangle]
unsafe extern "C" fn find_av_name(ses: *mut cifs_ses, type: u16, name: *mut c_char, maxlen: u16) -> c_int {
    static int find_av_name(struct cifs_ses *ses, u16 type, char **name, u16 maxlen)
    {
    const struct nls_table *nlsc = ses.local_nls;
    struct ntlmssp2_name *av;
    u16 len, nlen;
    if (*name)
    return 0;
    av_for_each_entry(ses, av) {
    len = AV_LEN(av);
    if (AV_TYPE(av) != type || !len)
    continue;
    if (!IS_ALIGNED(len, sizeof(__le16))) {
    cifs_dbg(VFS | ONCE, "%s: bad length(%u) for type %u\n",
    __func__, len, type);
    continue;
    }
    nlen = len / sizeof(__le16);
    if (nlen <= maxlen) {
    ++nlen;
// name = kmalloc(nlen, GFP_KERNEL);
    if (!*name)
    return -ENOMEM;
    cifs_from_utf16(*name, AV_DATA_PTR(av), nlen,
    len, nlsc, NO_MAP_UNI_RSVD);
    break;
    }
    }
    return 0;
    }
// Server has provided av pairs/target info in the type 2 challenge
// packet and we have plucked it and stored within smb session.
// We parse that blob here to find the server given timestamp
// as part of ntlmv2 authentication (or local current time as
// default in case of failure)
//
#[no_mangle]
unsafe extern "C" fn find_timestamp(ses: *mut cifs_ses) -> __le64 {
    static __le64 find_timestamp(struct cifs_ses *ses)
    {
    struct ntlmssp2_name *av;
    struct timespec64 ts;
    av_for_each_entry(ses, av) {
    if (AV_TYPE(av) == NTLMSSP_AV_TIMESTAMP &&
    AV_LEN(av) == sizeof(u64))
    return *((__le64 *)AV_DATA_PTR(av));
    }
    ktime_get_real_ts64(&ts);
    return cpu_to_le64(cifs_UnixTimeToNT(ts));
    }
    static int calc_ntlmv2_hash(struct cifs_ses *ses, char *ntlmv2_hash,
    const struct nls_table *nls_cp)
    {
    int len;
    char nt_hash[CIFS_NTHASH_SIZE];
    struct hmac_md5_ctx hmac_ctx;
    __le16 *user;
    wchar_t *domain;
    wchar_t *server;
// calculate md4 hash of password
    E_md4hash(ses.password, nt_hash, nls_cp);
    hmac_md5_init_usingrawkey(&hmac_ctx, nt_hash, CIFS_NTHASH_SIZE);
    memzero_explicit(nt_hash, sizeof(nt_hash));
// convert ses->user_name to unicode
    len = ses.user_name ? strlen(ses.user_name) : 0;
    user = kmalloc(2 + (len * 2), GFP_KERNEL);
    if (user == core::ptr::null_mut())
    goto out_nomem;
    if (len) {
    len = cifs_strtoUTF16(user, ses.user_name, len, nls_cp);
    UniStrupr(user);
    } else {
// (u16 *)user = 0;
    }
    hmac_md5_update(&hmac_ctx, (const u8 *)user, 2 * len);
    kfree(user);
// convert ses->domainName to unicode and uppercase
    if (ses.domainName) {
    len = strlen(ses.domainName);
    domain = kmalloc(2 + (len * 2), GFP_KERNEL);
    if (domain == core::ptr::null_mut())
    goto out_nomem;
    len = cifs_strtoUTF16((__le16 *)domain, ses.domainName, len,
    nls_cp);
    hmac_md5_update(&hmac_ctx, (const u8 *)domain, 2 * len);
    kfree(domain);
    } else {
// We use ses->ip_addr if no domain name available
    len = strlen(ses.ip_addr);
    server = kmalloc(2 + (len * 2), GFP_KERNEL);
    if (server == core::ptr::null_mut())
    goto out_nomem;
    len = cifs_strtoUTF16((__le16 *)server, ses.ip_addr, len, nls_cp);
    hmac_md5_update(&hmac_ctx, (const u8 *)server, 2 * len);
    kfree(server);
    }
    hmac_md5_final(&hmac_ctx, ntlmv2_hash);
    return 0;
    out_nomem:
    memzero_explicit(&hmac_ctx, sizeof(hmac_ctx));
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn CalcNTLMv2_response(ses: *const cifs_ses, ntlmv2_hash: *mut c_char) {
    static void CalcNTLMv2_response(const struct cifs_ses *ses, char *ntlmv2_hash)
    {
    struct ntlmv2_resp *ntlmv2 = (struct ntlmv2_resp *)
    (ses.auth_key.response + CIFS_SESS_KEY_SIZE);
    unsigned int hash_len;
// The MD5 hash starts at challenge_key.key
    hash_len = ses.auth_key.len - (CIFS_SESS_KEY_SIZE +
    offsetof(struct ntlmv2_resp, challenge.key[0]));
    if (ses.server.negflavor == CIFS_NEGFLAVOR_EXTENDED)
    memcpy(ntlmv2.challenge.key, ses.ntlmssp.cryptkey, CIFS_SERVER_CHALLENGE_SIZE);
    else
    memcpy(ntlmv2.challenge.key, ses.server.cryptkey, CIFS_SERVER_CHALLENGE_SIZE);
// Note that the HMAC-MD5 value overwrites ntlmv2->challenge.key
    hmac_md5_usingrawkey(ntlmv2_hash, CIFS_HMAC_MD5_HASH_SIZE,
    ntlmv2.challenge.key, hash_len,
    ntlmv2.ntlmv2_hash);
    }
//
// Set up NTLMv2 response blob with SPN (cifs/<hostname>) appended to the
// existing list of AV pairs.
//
#[no_mangle]
unsafe extern "C" fn set_auth_key_response(ses: *mut cifs_ses) -> c_int {
    static int set_auth_key_response(struct cifs_ses *ses)
    {
    let mut baselen: usize = CIFS_SESS_KEY_SIZE + sizeof(struct ntlmv2_resp);
    size_t len, spnlen, tilen = 0, num_avs = 2 /* SPN + EOL */;
    struct TCP_Server_Info *server = ses.server;
    char *spn __free(kfree) = core::ptr::null_mut();
    struct ntlmssp2_name *av;
    char *rsp = core::ptr::null_mut();
    int rc;
    spnlen = strlen(server.hostname);
    len = sizeof("cifs/") + spnlen;
    spn = kmalloc(len, GFP_KERNEL);
    if (!spn) {
    rc = -ENOMEM;
    goto out;
    }
    spnlen = scnprintf(spn, len, "cifs/%.*s",
    (int)spnlen, server.hostname);
    av_for_each_entry(ses, av)
    tilen += sizeof(*av) + AV_LEN(av);
    len = baselen + tilen + spnlen * sizeof(__le16) + num_avs * sizeof(*av);
    rsp = kmalloc(len, GFP_KERNEL);
    if (!rsp) {
    rc = -ENOMEM;
    goto out;
    }
    memcpy(rsp + baselen, ses.auth_key.response, tilen);
    av = (void *)(rsp + baselen + tilen);
    av.type = cpu_to_le16(NTLMSSP_AV_TARGET_NAME);
    av.length = cpu_to_le16(spnlen * sizeof(__le16));
    cifs_strtoUTF16((__le16 *)av.data, spn, spnlen, ses.local_nls);
    av = (void *)((__u8 *)av + sizeof(*av) + AV_LEN(av));
    av.type = cpu_to_le16(NTLMSSP_AV_EOL);
    av.length = 0;
    rc = 0;
    ses.auth_key.len = len;
    out:
    ses.auth_key.response = rsp;
    return rc;
    }
    int
    setup_ntlmv2_rsp(struct cifs_ses *ses, const struct nls_table *nls_cp)
    {
    unsigned char *tiblob = core::ptr::null_mut(); /* target info blob */
    struct ntlmv2_resp *ntlmv2;
    char ntlmv2_hash[16];
    __le64 rsp_timestamp;
    __u64 cc;
    int rc;
    if (nls_cp == core::ptr::null_mut()) {
    cifs_dbg(VFS, "%s called with nls_cp==core::ptr::null_mut()\n", __func__);
    return -EINVAL;
    }
    if (ses.server.negflavor == CIFS_NEGFLAVOR_EXTENDED) {
    if (!ses.domainName) {
    if (ses.domainAuto) {
//
// Domain (workgroup) hasn't been specified in
// mount options, so try to find it in
// CHALLENGE_MESSAGE message and then use it as
// part of NTLMv2 authentication.
//
    rc = find_av_name(ses, NTLMSSP_AV_NB_DOMAIN_NAME,
    &ses.domainName,
    CIFS_MAX_DOMAINNAME_LEN);
    if (rc)
    goto setup_ntlmv2_rsp_ret;
    } else {
    ses.domainName = kstrdup("", GFP_KERNEL);
    if (!ses.domainName) {
    rc = -ENOMEM;
    goto setup_ntlmv2_rsp_ret;
    }
    }
    }
    rc = find_av_name(ses, NTLMSSP_AV_DNS_DOMAIN_NAME,
    &ses.dns_dom, CIFS_MAX_DOMAINNAME_LEN);
    if (rc)
    goto setup_ntlmv2_rsp_ret;
    } else {
    rc = build_avpair_blob(ses, nls_cp);
    if (rc) {
    cifs_dbg(VFS, "error %d building av pair blob\n", rc);
    goto setup_ntlmv2_rsp_ret;
    }
    }
// Must be within 5 minutes of the server (or in range +/-2h
// in case of Mac OS X), so simply carry over server timestamp
// (as Windows 7 does)
//
    rsp_timestamp = find_timestamp(ses);
    get_random_bytes(&cc, sizeof(cc));
    cifs_server_lock(ses.server);
    tiblob = ses.auth_key.response;
    rc = set_auth_key_response(ses);
    if (rc) {
    ses.auth_key.len = 0;
    goto unlock;
    }
    ntlmv2 = (struct ntlmv2_resp *)
    (ses.auth_key.response + CIFS_SESS_KEY_SIZE);
    ntlmv2.blob_signature = cpu_to_le32(0x00000101);
    ntlmv2.reserved = 0;
    ntlmv2.time = rsp_timestamp;
    ntlmv2.client_chal = cc;
    ntlmv2.reserved2 = 0;
    if (fips_enabled) {
    cifs_dbg(VFS, "NTLMv2 support is disabled due to FIPS\n");
    rc = -EOPNOTSUPP;
    goto unlock;
    }
// calculate ntlmv2_hash
    rc = calc_ntlmv2_hash(ses, ntlmv2_hash, nls_cp);
    if (rc) {
    cifs_dbg(VFS, "Could not get NTLMv2 hash, rc=%d\n", rc);
    goto unlock;
    }
// calculate first part of the client response (CR1)
    CalcNTLMv2_response(ses, ntlmv2_hash);
// now calculate the session key for NTLMv2
    hmac_md5_usingrawkey(ntlmv2_hash, CIFS_HMAC_MD5_HASH_SIZE,
    ntlmv2.ntlmv2_hash, CIFS_HMAC_MD5_HASH_SIZE,
    ses.auth_key.response);
    rc = 0;
    unlock:
    cifs_server_unlock(ses.server);
    memzero_explicit(ntlmv2_hash, sizeof(ntlmv2_hash));
    setup_ntlmv2_rsp_ret:
    kfree_sensitive(tiblob);
    return rc;
    }
    int
    calc_seckey(struct cifs_ses *ses)
    {
    unsigned char sec_key[CIFS_SESS_KEY_SIZE]; /* a nonce */
    struct arc4_ctx *ctx_arc4;
    if (fips_enabled)
    return -ENODEV;
    get_random_bytes(sec_key, CIFS_SESS_KEY_SIZE);
    ctx_arc4 = kmalloc_obj(*ctx_arc4);
    if (!ctx_arc4) {
    cifs_dbg(VFS, "Could not allocate arc4 context\n");
    return -ENOMEM;
    }
    arc4_setkey(ctx_arc4, ses.auth_key.response, CIFS_SESS_KEY_SIZE);
    arc4_crypt(ctx_arc4, ses.ntlmssp.ciphertext, sec_key,
    CIFS_CPHTXT_SIZE);
// make secondary_key/nonce as session key
    memcpy(ses.auth_key.response, sec_key, CIFS_SESS_KEY_SIZE);
// and make len as that of session key only
    ses.auth_key.len = CIFS_SESS_KEY_SIZE;
    memzero_explicit(sec_key, CIFS_SESS_KEY_SIZE);
    kfree_sensitive(ctx_arc4);
    return 0;
    }
    void
    cifs_crypto_secmech_release(struct TCP_Server_Info *server)
    {
    if (server.secmech.enc) {
    crypto_free_aead(server.secmech.enc);
    server.secmech.enc = core::ptr::null_mut();
    }
    if (server.secmech.dec) {
    crypto_free_aead(server.secmech.dec);
    server.secmech.dec = core::ptr::null_mut();
    }
    }
