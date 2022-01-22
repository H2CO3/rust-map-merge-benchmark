use std::ops::Deref;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use rand::prelude::*;
use rand::distributions::Standard;
use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use mapbench::{ merge, merge_consecutive };


const SMALL_ASCII_STRING_CAPACITY: usize = 8;

#[derive(Clone, Copy, Default, Eq)]
struct SmallAsciiString {
    buf: [u8; SMALL_ASCII_STRING_CAPACITY],
    len: usize,
}

impl Deref for SmallAsciiString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::str::from_utf8_unchecked(&self.buf[..self.len])
        }
    }
}

impl AsRef<str> for SmallAsciiString {
    fn as_ref(&self) -> &str {
        self
    }
}

impl Borrow<str> for SmallAsciiString {
    fn borrow(&self) -> &str {
        self
    }
}

impl PartialEq for SmallAsciiString {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl PartialOrd for SmallAsciiString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl Ord for SmallAsciiString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deref().cmp(other.deref())
    }
}

impl Distribution<SmallAsciiString> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SmallAsciiString {
        let min = SMALL_ASCII_STRING_CAPACITY / 2;
        let max = SMALL_ASCII_STRING_CAPACITY;
        let len: usize = rng.gen_range(min..=max);
        let mut buf: [u8; SMALL_ASCII_STRING_CAPACITY] = <_>::default();

        for byte in &mut buf[..len] {
            *byte = rng.gen_range(b'a'..=b'z');
        }

        SmallAsciiString { buf, len }
    }
}

fn random_string<R: ?Sized + Rng>(rng: &mut R) -> String {
    let min = SMALL_ASCII_STRING_CAPACITY / 2;
    let max = SMALL_ASCII_STRING_CAPACITY;
    let len: usize = rng.gen_range(min..=max);
    (0..len).map(|_| rng.gen_range(b'a'..=b'z') as char).collect()
}

fn random_data_string_key<R: ?Sized + Rng>(rng: &mut R, n: usize) -> BTreeMap<String, usize> {
    (0..n).map(|i| (random_string(rng), i)).collect()
}

fn random_data_smallstr_key<R: ?Sized + Rng>(rng: &mut R, n: usize) -> BTreeMap<SmallAsciiString, usize> {
    (0..n).map(|i| (rng.gen::<SmallAsciiString>(), i)).collect()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut data = random_data_string_key(&mut rng, 65536);

    c.bench_function("merge (std::String)", |b| b.iter(|| {
        merge(
            black_box(&mut data),
            |(k1, _), (k2, _)| k1.len() == k2.len(),
            |(_, v1), (_, v2)| *v1 += v2,
        )
    }));
    c.bench_function("merge_consecutive (std::String)", |b| b.iter(|| {
        merge_consecutive(
            black_box(&mut data),
            |(k1, _), (k2, _)| k1.len() == k2.len(),
            |(_, v1), (_, v2)| *v1 += *v2,
        )
    }));

    let mut data = random_data_smallstr_key(&mut rng, 65536);
    c.bench_function("merge (SmallAsciiString)", |b| b.iter(|| {
        merge(
            black_box(&mut data),
            |(k1, _), (k2, _)| k1.len() == k2.len(),
            |(_, v1), (_, v2)| *v1 += v2,
        )
    }));
    c.bench_function("merge_consecutive (SmallAsciiString)", |b| b.iter(|| {
        merge_consecutive(
            black_box(&mut data),
            |(k1, _), (k2, _)| k1.len() == k2.len(),
            |(_, v1), (_, v2)| *v1 += *v2,
        )
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
