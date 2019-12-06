#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
extern "C" {
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[derive ( Copy, Clone )]
#[repr(C)]
pub struct _prefix_stats {
    pub prefix: *mut libc::c_char,
    pub prefix_len: size_t,
    pub num_gets: libc::c_int,
    pub num_sets: libc::c_int,
    pub num_deletes: libc::c_int,
    pub num_hits: libc::c_int,
    pub next: *mut PREFIX_STATS,
}
/* -*- Mode: C; tab-width: 4; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Detailed statistics management. For simple stats like total number of
 * "get" requests, we use inline code in memcached.c and friends, but when
 * stats detail mode is activated, the code here records more information.
 *
 * Author:
 *   Steven Grimm <sgrimm@facebook.com>
 */
/*
 * Stats are tracked on the basis of key prefixes. This is a simple
 * fixed-size hash of prefixes; we run the prefixes through the same
 * CRC function used by the cache hashtable.
 */
pub type PREFIX_STATS = _prefix_stats;
static mut prefix_stats: [*mut PREFIX_STATS; 256] =
    [0 as *const PREFIX_STATS as *mut PREFIX_STATS; 256];
static mut num_prefixes: libc::c_int = 0 as libc::c_int;
static mut total_prefix_size: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn stats_prefix_init() {
    memset(prefix_stats.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[*mut PREFIX_STATS; 256]>() as
               libc::c_ulong);
}
/*
 * Cleans up all our previously collected stats. NOTE: the stats lock is
 * assumed to be held when this is called.
 */
#[no_mangle]
pub unsafe extern "C" fn stats_prefix_clear() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut cur: *mut PREFIX_STATS = 0 as *mut PREFIX_STATS;
        let mut next: *mut PREFIX_STATS = 0 as *mut PREFIX_STATS;
        cur = prefix_stats[i as usize];
        while !cur.is_null() {
            next = (*cur).next;
            free((*cur).prefix as *mut libc::c_void);
            free(cur as *mut libc::c_void);
            cur = next
        }
        prefix_stats[i as usize] = 0 as *mut PREFIX_STATS;
        i += 1
    }
    num_prefixes = 0 as libc::c_int;
    total_prefix_size = 0 as libc::c_int;
}
/*
 * Records a "delete" of a key.
 */
#[no_mangle]
pub unsafe extern "C" fn stats_prefix_record_delete(mut key:
                                                        *const libc::c_char,
                                                    nkey: size_t) {
    let mut pfs: *mut PREFIX_STATS = 0 as *mut PREFIX_STATS;
    STATS_LOCK();
    pfs = stats_prefix_find(key, nkey);
    !pfs.is_null();
    STATS_UNLOCK();
}
/*
 * Records a "set" of a key.
 */
#[no_mangle]
pub unsafe extern "C" fn stats_prefix_record_set(mut key: *const libc::c_char,
                                                 nkey: size_t) {
    let mut pfs: *mut PREFIX_STATS = 0 as *mut PREFIX_STATS;
    STATS_LOCK();
    pfs = stats_prefix_find(key, nkey);
    !pfs.is_null();
    STATS_UNLOCK();
}
/*
 * Returns stats in textual form suitable for writing to a client.
 */
/*@null@*/
#[no_mangle]
pub unsafe extern "C" fn stats_prefix_dump(mut length: *mut libc::c_int)
 -> *mut libc::c_char {
    let mut format: *const libc::c_char =
        b"PREFIX %s get %llu hit %llu set %llu del %llu\r\n\x00" as *const u8
            as *const libc::c_char;
    let mut pfs: *mut PREFIX_STATS = 0 as *mut PREFIX_STATS;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut written: size_t = 0 as libc::c_int as size_t;
    let mut total_written: size_t = 0 as libc::c_int as size_t;
    /*
     * Figure out how big the buffer needs to be. This is the sum of the
     * lengths of the prefixes themselves, plus the size of one copy of
     * the per-prefix output with 20-digit values for all the counts,
     * plus space for the "END" at the end.
     */
    STATS_LOCK();
    size =
        strlen(format).wrapping_add(total_prefix_size as
                                        libc::c_ulong).wrapping_add((num_prefixes
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(strlen(format).wrapping_sub(2
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong).wrapping_add((4
                                                                                                                                                                      as
                                                                                                                                                                      libc::c_int
                                                                                                                                                                      *
                                                                                                                                                                      (20
                                                                                                                                                                           as
                                                                                                                                                                           libc::c_int
                                                                                                                                                                           -
                                                                                                                                                                           4
                                                                                                                                                                               as
                                                                                                                                                                               libc::c_int))
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_ulong))).wrapping_add(::std::mem::size_of::<[libc::c_char; 6]>()
                                                                                                                                                                                                       as
                                                                                                                                                                                                       libc::c_ulong);
    buf = malloc(size) as *mut libc::c_char;
    if buf.is_null() {
        perror(b"Can\'t allocate stats response: malloc\x00" as *const u8 as
                   *const libc::c_char);
        STATS_UNLOCK();
        return 0 as *mut libc::c_char
    }
    pos = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        pfs = prefix_stats[i as usize];
        while !pfs.is_null() {
            pos =
                (pos as libc::c_ulong).wrapping_add(written) as libc::c_int as
                    libc::c_int;
            total_written =
                (total_written as libc::c_ulong).wrapping_add(written) as
                    size_t as size_t;
            pfs = (*pfs).next
        }
        i += 1
    }
    STATS_UNLOCK();
    memcpy(buf.offset(pos as isize) as *mut libc::c_void,
           b"END\r\n\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void, 6 as libc::c_int as libc::c_ulong);
    *length = pos + 5 as libc::c_int;
    return buf;
}
