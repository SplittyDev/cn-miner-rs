// Reference: https://github.com/noahdesu/xmonarch/blob/master/keccak.c

//
// Imports
//

use std::mem::size_of;

//
// Type Aliases
//

type KeccakState = [u64; 25];

//
// Constants
//

const HASH_DATA_AREA    : usize = 136;
const KECCAK_ROUNDS     : usize = 24;

//
// Macros
//

macro_rules! rotl64 {
    ($x:expr, $y:expr) => { ($x << $y) | ($x >> (64 - $y)) };
}

macro_rules! keccak {
    (theta init [$bc:expr]; $i:expr => [st:$st:expr]) => {
        $bc[$i] = $st[$i] ^ $st[$i + 5] ^ $st[$i + 10] ^ $st[$i + 15] ^ $st[$i + 20];
    };
    (theta mix [$st:expr]; [t:$t:expr;i:$i:expr]) => {
        $st[$i     ] ^= $t; $st[$i +  5] ^= $t; $st[$i + 10] ^= $t;
        $st[$i + 15] ^= $t; $st[$i + 20] ^= $t;
    };
    (theta full mix [$st:expr]; [bc:$bc:expr;t:$t:expr;i:$i:expr]) => {
        $t = $bc[($i + 4) % 5] ^ rotl64!($bc[($i + 1) % 5], 1);
        keccak!(theta mix [$st]; [t:$t;i:$i]);
    };
    (theta unroll mix [$st:expr]; [bc:$bc:expr;t:$t:expr]) => {
        keccak!(theta full mix [$st]; [bc:$bc;t:$t;i:0]); keccak!(theta full mix [$st]; [bc:$bc;t:$t;i:1]);
        keccak!(theta full mix [$st]; [bc:$bc;t:$t;i:2]); keccak!(theta full mix [$st]; [bc:$bc;t:$t;i:3]);
        keccak!(theta full mix [$st]; [bc:$bc;t:$t;i:4]);
    };
    (rho_pi [$st:expr]; [bc:$bc:expr;t:$t:ident;i:$i:expr]) => {
        $bc[0] = $st[$crate::keccak::KECCAK_PILN[$i]];
        $st[$crate::keccak::KECCAK_PILN[$i]] = rotl64!($t, $crate::keccak::KECCAK_ROTC[$i]);
        $t = $bc[0];
    };
    (rho_pi unroll5 [$st:expr]; [bc:$bc:expr;t:$t:ident;i:$i:expr]) => {
        keccak!(rho_pi [$st]; [bc:$bc;t:$t;i:$i + 0]); keccak!(rho_pi [$st]; [bc:$bc;t:$t;i:$i + 1]);
        keccak!(rho_pi [$st]; [bc:$bc;t:$t;i:$i + 2]); keccak!(rho_pi [$st]; [bc:$bc;t:$t;i:$i + 3]);
        keccak!(rho_pi [$st]; [bc:$bc;t:$t;i:$i + 4]);
    };
    (rho_pi unroll [$st:expr]; [bc:$bc:expr;t:$t:ident]) => {
        keccak!(rho_pi unroll5 [$st]; [bc:$bc;t:$t;i: 0]); keccak!(rho_pi unroll5 [$st]; [bc:$bc;t:$t;i: 5]);
        keccak!(rho_pi unroll5 [$st]; [bc:$bc;t:$t;i:10]); keccak!(rho_pi unroll5 [$st]; [bc:$bc;t:$t;i:15]);
        keccak!(rho_pi [$st]; [bc:$bc;t:$t;i:20]);
        keccak!(rho_pi [$st]; [bc:$bc;t:$t;i:21]);
        keccak!(rho_pi [$st]; [bc:$bc;t:$t;i:22]);
        #[allow(unused_assignments)] {
            // Last assignment to t is not read in the last round
            keccak!(rho_pi [$st]; [bc:$bc;t:$t;i:23]);
        }
    };
    (chi copy [$bc:expr]; $i:expr => [st:$st:expr;j:$j:expr]) => {
        $bc[$i] = $st[$j + $i];
    };
    (chi mix [$st:expr]; $i:expr => [bc:$bc:expr;j:$j:expr]) => {
        $st[$j + $i] ^= (!$bc[($i + 1) % 5]) & $bc[($i + 2) % 5];
    };
    (chi full [$bc:expr]; $i:expr => [st:$st:expr]) => {
        keccak!(chi copy [$bc]; 0 => [st:$st;j:$i]); keccak!(chi copy [$bc]; 1 => [st:$st;j:$i]);
        keccak!(chi copy [$bc]; 2 => [st:$st;j:$i]); keccak!(chi copy [$bc]; 3 => [st:$st;j:$i]);
        keccak!(chi copy [$bc]; 4 => [st:$st;j:$i]); keccak!(chi mix  [$st]; 0 => [bc:$bc;j:$i]);
        keccak!(chi mix  [$st]; 1 => [bc:$bc;j:$i]); keccak!(chi mix  [$st]; 2 => [bc:$bc;j:$i]);
        keccak!(chi mix  [$st]; 3 => [bc:$bc;j:$i]); keccak!(chi mix  [$st]; 4 => [bc:$bc;j:$i]);
    };
    (chi unroll [$bc:expr]; [st:$st:expr]) => {
        keccak!(chi full [$bc];  0 => [st:$st]); keccak!(chi full [$bc];  5 => [st:$st]);
        keccak!(chi full [$bc]; 10 => [st:$st]); keccak!(chi full [$bc]; 15 => [st:$st]);
        keccak!(chi full [$bc]; 20 => [st:$st]);
    };
}

//
// Functions
//

pub fn keccakf(st: &mut[u64; 25], rounds: usize) {
    let mut t: u64;
    let mut bc = [0u64; 5];
    for round in 0..rounds {
        // Theta
        keccak!(theta init [bc]; 0 => [st:st]);
        keccak!(theta init [bc]; 1 => [st:st]);
        keccak!(theta init [bc]; 2 => [st:st]);
        keccak!(theta init [bc]; 3 => [st:st]);
        keccak!(theta init [bc]; 4 => [st:st]);
        keccak!(theta unroll mix [st]; [bc:bc;t:t]);
        // Rho Pi
        t = st[1];
        keccak!(rho_pi unroll [st]; [bc:bc;t:t]);
        // Chi
        keccak!(chi unroll [bc]; [st:st]);
        // Iota
        st[0] ^= KECCAK_RNDC[round];
    }
}

pub fn keccak(input: &[u8], md: &mut[u8]) {
    let (mut in_len, md_len) = (input.len(), md.len());
    let mut st: KeccakState = [0u64; 25];
    let mut temp = [0u8; 144];
    let rsiz = match size_of::<KeccakState>() {
        sz if sz == md_len => HASH_DATA_AREA,
        _ => 200 - (2 * md_len),
    };
    let rsizw = rsiz / 8;
    let mut i_off = 0;
    while in_len >= rsiz {
        for i in 0..rsizw {
            unsafe {
                let in_ptr = input.as_ptr().offset(i_off as isize);
                let in_ptr = in_ptr as *const u64;
                st[i] ^= *in_ptr;
            }
        }
        keccakf(&mut st, KECCAK_ROUNDS);
        in_len -= rsiz;
        i_off += rsiz;
    }
    for i in 0..in_len { temp[i] = input[i]; }
    temp[in_len] = 1;
    in_len += 1;
    for i in 0..(rsiz - in_len) {
        temp[in_len + i] = 0;
    }
    temp[rsiz - 1] |= 0x80;
    for i in 0..rsizw {
        st[i] ^= unsafe {
            *(temp.as_ptr() as *const u64).offset(i as isize)
        };
    }
    keccakf(&mut st, KECCAK_ROUNDS);
    for i in 0..md_len {
        unsafe {
            let st_ptr = st.as_ptr() as *const u8;
            md[i] = *st_ptr.offset(i as isize);
        }
    }
}

//
// Algorithm Constants
//

static KECCAK_RNDC : [u64; 24] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
    0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
    0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
    0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
    0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
    0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
    0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
];

static KECCAK_ROTC : [i32; 24] = [
    1,   3,  6, 10, 15, 21, 28, 36, 45, 55,  2, 14,
    27, 41, 56,  8, 25, 43, 62, 18, 39, 61, 20, 44,
];

static KECCAK_PILN : [usize; 24] = [
    10,  7, 11, 17, 18, 3,  5, 16, 8, 21, 24, 4, 
    15, 23, 19, 13, 12, 2, 20, 14, 22, 9,  6, 1,
];