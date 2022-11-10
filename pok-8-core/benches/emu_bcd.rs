
use pok_8_core::emu::*;

use bencher::{Bencher, benchmark_group, benchmark_main};

fn bcd_new(b: &mut Bencher) {
    
    let vx: u8 = 0xA;

    b.iter(||
        {
        let _bcd = Emu::double_dabble(&vx);
    })
}

benchmark_group!(bcd, bcd_new);
benchmark_main!(bcd);