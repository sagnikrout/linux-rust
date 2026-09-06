//! Automatically rewritten from C to Rust
//! Source: crypto/twofish_generic.c
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
// Twofish for CryptoAPI
//
// Originally Twofish for GPG
// By Matthew Skala <mskala@ansuz.sooke.bc.ca>, July 26, 1998
// 256-bit key length added March 20, 1999
// Some modifications to reduce the text size by Werner Koch, April, 1998
// Ported to the kerneli patch by Marc Mutz <Marc@Mutz.com>
// Ported to CryptoAPI by Colin Slater <hoho@tacomeat.net>
//
// The original author has disclaimed all copyright interest in this
// code and thus put it in the public domain. The subsequent authors
// have put this under the GNU General Public License.
//
// This code is a "clean room" implementation, written from the paper
// _Twofish: A 128-Bit Block Cipher_ by Bruce Schneier, John Kelsey,
// Doug Whiting, David Wagner, Chris Hall, and Niels Ferguson, available
// through http://www.counterpane.com/twofish.html
//
// For background information on multiplication in finite fields, used for
// the matrix operations in the key schedule, see the book _Contemporary
// Abstract Algebra_ by Joseph A. Gallian, especially chapter 22 in the
// Third Edition.
//

// Macros to compute the g() function in the encryption and decryption
// rounds.  G1 is the straight g() function; G2 includes the 8-bit
// rotation for the high 32-bit word.

    (ctx.s[0][(a) & 0xFF]) ^ (ctx.s[1][((a) >> 8) & 0xFF]) \
    ^ (ctx.s[2][((a) >> 16) & 0xFF]) ^ (ctx.s[3][(a) >> 24])

    (ctx.s[1][(b) & 0xFF]) ^ (ctx.s[2][((b) >> 8) & 0xFF]) \
    ^ (ctx.s[3][((b) >> 16) & 0xFF]) ^ (ctx.s[0][(b) >> 24])
// Encryption and decryption Feistel rounds.  Each one calls the two g()
// macros, does the PHT, and performs the XOR and the appropriate bit
// rotations.  The parameters are the round number (used to select subkeys),
// and the four 32-bit chunks of the text.

    x = G1 (a); y = G2 (b); \
    x += y; y += x + ctx.k[2 * (n) + 1]; \
    (c) ^= x + ctx.k[2 * (n)]; \
    (c) = ror32((c), 1); \
    (d) = rol32((d), 1) ^ y

    x = G1 (a); y = G2 (b); \
    x += y; y += x; \
    (d) ^= y + ctx.k[2 * (n) + 1]; \
    (d) = ror32((d), 1); \
    (c) = rol32((c), 1); \
    (c) ^= (x + ctx.k[2 * (n)])
// Encryption and decryption cycles; each one is simply two Feistel rounds
// with the 32-bit chunks re-ordered to simulate the "swap"

    ENCROUND (2 * (n), a, b, c, d); \
    ENCROUND (2 * (n) + 1, c, d, a, b)

    DECROUND (2 * (n) + 1, c, d, a, b); \
    DECROUND (2 * (n), a, b, c, d)
// Macros to convert the input and output bytes into 32-bit words,
// and simultaneously perform the whitening step.  INPACK packs word
// number n into the variable named by x, using whitening subkey number m.
// OUTUNPACK unpacks word number n from the variable named by x, using
// whitening subkey number m.

    x = get_unaligned_le32(in + (n) * 4) ^ ctx.w[m]

    x ^= ctx.w[m]; \
    put_unaligned_le32(x, out + (n) * 4)
// Encrypt one block.  in and out may be the same.
#[no_mangle]
unsafe extern "C" fn twofish_encrypt(tfm: *mut crypto_tfm, out: *mut u8, in: *const u8) {
    static void twofish_encrypt(struct crypto_tfm *tfm, u8 *out, const u8 *in)
    {
    struct twofish_ctx *ctx = crypto_tfm_ctx(tfm);
// The four 32-bit chunks of the text.
    u32 a, b, c, d;
// Temporaries used by the round function.
    u32 x, y;
// Input whitening and packing.
    INPACK (0, a, 0);
    INPACK (1, b, 1);
    INPACK (2, c, 2);
    INPACK (3, d, 3);
// Encryption Feistel cycles.
    ENCCYCLE (0);
    ENCCYCLE (1);
    ENCCYCLE (2);
    ENCCYCLE (3);
    ENCCYCLE (4);
    ENCCYCLE (5);
    ENCCYCLE (6);
    ENCCYCLE (7);
// Output whitening and unpacking.
    OUTUNPACK (0, c, 4);
    OUTUNPACK (1, d, 5);
    OUTUNPACK (2, a, 6);
    OUTUNPACK (3, b, 7);
    }
// Decrypt one block.  in and out may be the same.
#[no_mangle]
unsafe extern "C" fn twofish_decrypt(tfm: *mut crypto_tfm, out: *mut u8, in: *const u8) {
    static void twofish_decrypt(struct crypto_tfm *tfm, u8 *out, const u8 *in)
    {
    struct twofish_ctx *ctx = crypto_tfm_ctx(tfm);
// The four 32-bit chunks of the text.
    u32 a, b, c, d;
// Temporaries used by the round function.
    u32 x, y;
// Input whitening and packing.
    INPACK (0, c, 4);
    INPACK (1, d, 5);
    INPACK (2, a, 6);
    INPACK (3, b, 7);
// Encryption Feistel cycles.
    DECCYCLE (7);
    DECCYCLE (6);
    DECCYCLE (5);
    DECCYCLE (4);
    DECCYCLE (3);
    DECCYCLE (2);
    DECCYCLE (1);
    DECCYCLE (0);
// Output whitening and unpacking.
    OUTUNPACK (0, a, 0);
    OUTUNPACK (1, b, 1);
    OUTUNPACK (2, c, 2);
    OUTUNPACK (3, d, 3);
    }
    static struct crypto_alg alg = {
    .cra_name           =   "twofish",
    .cra_driver_name    =   "twofish-generic",
    .cra_priority       =   100,
    .cra_flags          =   CRYPTO_ALG_TYPE_CIPHER,
    .cra_blocksize      =   TF_BLOCK_SIZE,
    .cra_ctxsize        =   sizeof(struct twofish_ctx),
    .cra_module         =   THIS_MODULE,
    .cra_u              =   { .cipher = {
    .cia_min_keysize    =   TF_MIN_KEY_SIZE,
    .cia_max_keysize    =   TF_MAX_KEY_SIZE,
    .cia_setkey         =   twofish_setkey,
    .cia_encrypt        =   twofish_encrypt,
    .cia_decrypt        =   twofish_decrypt } }
    };
#[no_mangle]
unsafe extern "C" fn twofish_mod_init() -> int __init {
    static int __init twofish_mod_init(void)
    {
    return crypto_register_alg(&alg);
    }
#[no_mangle]
unsafe extern "C" fn twofish_mod_fini() -> void __exit {
    static void __exit twofish_mod_fini(void)
    {
    crypto_unregister_alg(&alg);
    }
    module_init(twofish_mod_init);
    module_exit(twofish_mod_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION ("Twofish Cipher Algorithm");
    MODULE_ALIAS_CRYPTO("twofish");
    MODULE_ALIAS_CRYPTO("twofish-generic");
