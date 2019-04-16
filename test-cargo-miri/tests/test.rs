extern crate rand;

use rand::{SeedableRng, FromEntropy, Rng, rngs::SmallRng};

#[test]
fn simple() {
    assert_eq!(4, 4);
}

// Having more than 1 test does seem to make a difference
// (i.e., this calls ptr::swap which having just one test does not).
#[test]
fn fixed_rng() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xdeadcafe);
    let x: u32 = rng.gen();
    let y: u32 = rng.gen();
    assert_ne!(x, y);
}

#[test]
#[cfg(not(target_os="macos"))] // FIXME entropy does not work on macOS
fn entropy_rng() {
    // Use this opportunity to test querying the RNG (needs an external crate, hence tested here and not in the compiletest suite)
    let mut rng = SmallRng::from_entropy();
    let _val = rng.gen::<i32>();

    // Also try per-thread RNG.
    let mut rng = rand::thread_rng();
    let _val = rng.gen::<i32>();
}

// A test that won't work on miri
#[cfg(not(miri))]
#[test]
fn does_not_work_on_miri() {
    let x = 0u8;
    assert!(&x as *const _ as usize % 4 < 4);
}
