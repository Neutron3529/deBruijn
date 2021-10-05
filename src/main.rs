#![feature(generic_const_exprs)]
//#![feature(new_uninit)]
fn search<const T: u32>(
    state: usize,
    mut curr_lv: u32,
    visit: &mut [bool; 1 << T],
) -> Option<Vec<u8>> {
    let mask = (1 << T) - 1;
    if visit[state] {
        None
    } else if curr_lv == mask {
        Some(Vec::with_capacity(1 << T))
    } else {
        curr_lv += 1;
        visit[state] = true;
        search::<T>(state * 2 & mask as usize, curr_lv, visit)
            .or_else(|| search::<T>(1 + state * 2 & mask as usize, curr_lv, visit))
            .or_else(|| {
                visit[state] = false;
                None
            })
    }
    .map(|mut x| {
        x.push(state as u8 & 1);
        x
    })
}
fn eval<const N: u32>()
where
    [(); 1 << N]: ,
{
    println!("searching N={}", N);
    let mut state = [false; 1 << N]; //unsafe{Box::<[bool;1<<N]>::new_zeroed().assume_init()};
    search::<N>(0, 0, &mut state).map(|x| {
        println!("solved.");
        if x.len() > 128 {
            println!(
                "res={}",
                (1..N)
                    .map(|_| '0')
                    .chain(x.into_iter().rev().map(|x| (x + b'0') as char))
                    .take(1 << N)
                    .collect::<String>()
            );
        } else {
            let z = x.into_iter().rev().fold(0u128, |s, x| x as u128 | (s << 1)) >> (N - 1);
            println!("res={0:0width$b} (0x{0:0x})", z, width = 1 << N)
        }
    });
}

use std::thread;
fn main() {
    thread::Builder::new()
        .stack_size(1024 * 1024 * 1024)
        .spawn(move || {
            eval::<1>();
            eval::<2>();
            eval::<3>();
            eval::<4>();
            eval::<5>();
            eval::<6>();
            eval::<7>();
            eval::<8>();
            eval::<9>();
            eval::<10>();
            eval::<11>();
            eval::<12>();
            eval::<13>();
            eval::<14>();
            eval::<15>();
            eval::<16>();
            eval::<17>();
            eval::<18>();
            eval::<19>();
            eval::<20>();
            eval::<21>();
            eval::<22>();
            eval::<23>();
            /*    eval::<24>();
            eval::<25>();
            eval::<26>();
            eval::<27>();
            eval::<28>();
            eval::<29>();
            eval::<30>();*/
        })
        .unwrap()
        .join()
        .unwrap();
}
