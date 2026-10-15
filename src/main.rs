#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Index of the first `needle` byte in `hay`.
pub fn find_byte(hay: &[u8], needle: u8) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        // SAFETY: we just verified AVX2 is available on this CPU.
        return unsafe { find_byte_avx2(hay, needle) };
    }
    hay.iter().position(|&b| b == needle) // scalar fallback
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn find_byte_avx2(hay: &[u8], needle: u8) -> Option<usize> {
    let n = _mm256_set1_epi8(needle as i8);
    let mut i = 0;
    while i + 32 <= hay.len() {
        // Unaligned load of 32 bytes; in bounds by the loop condition.
        let v = _mm256_loadu_si256(hay.as_ptr().add(i) as *const __m256i);
        // One bit per byte: 1 where the byte equals needle.
        let a = _mm256_cmpeq_epi8(v, n);
        println!("cmpeq {:?}", a);
        let mask = _mm256_movemask_epi8(a) as u32;
        println!("mask {:?}", mask);
        if mask != 0 {
            println!(
                "i {:?}, mask trailing_zeros {:?}",
                i,
                mask.trailing_zeros() as usize
            );
            return Some(i + mask.trailing_zeros() as usize);
        }
        i += 32;
    }
    // Tail shorter than 32 bytes.
    hay[i..].iter().position(|&b| b == needle).map(|p| i + p)
}

fn main() {
    let hay = (0..128).collect::<Vec<_>>();

    unsafe {
        let i = find_byte(&hay, 98);
        println!("{:?}", i);

        let i = find_byte_avx2(&hay, 98);
        println!("{:?}", i);
    }
}
