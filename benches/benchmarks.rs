#[macro_use]
extern crate criterion;

use criterion::Criterion;

use base64::config::{MIME, STANDARD};
use base64::Base64;

fn bench_convert_to_decimal(c: &mut Criterion) {
    c.bench_function("Convert b64 to decimal", move |b| {
        let x = Base64::new_random(20, STANDARD);
        b.iter(|| x.decode_to_unsigned())
    });
}

fn bench_set_from_unsigned(c: &mut Criterion) {
    c.bench_function("Set b64 from decimal", move |b| {
        let mut x: Base64 = Base64::new_encode_unsigned(&0, STANDARD);
        b.iter(|| x.encode_unsigned(&20769187000000000000000000000000000))
    });
}

fn bench_set_from_string(c: &mut Criterion) {
    c.bench_function("Set b64 from String", move |b| {
        let mut x: Base64 = Base64::new_encode_unsigned(&0, STANDARD);
        b.iter(|| x.set_from_string(&"20769187000000000000000000000000000"))
    });
}

fn bench_create_random(c: &mut Criterion) {
    c.bench_function("Create random b64 of length 20000", move |b| {
        b.iter(|| Base64::new_random(20000, STANDARD))
    });
}

fn bench_enconde_string(c: &mut Criterion) {
    let s: &str = "Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.";
    c.bench_function("Encode String Short", move |b| {
        b.iter(|| Base64::new_encode_bytes(s.as_bytes(), MIME))
    });
}

fn bench_encode_string_long(c: &mut Criterion) {
    let s: &str = "Somebody once told me the world is gonna roll me
I ain't the sharpest tool in the shed
She was looking kind of dumb with her finger and her thumb
In the shape of an \"L\" on her forehead
Well the years start coming and they don't stop coming
Fed to the rules and I hit the ground running
Didn't make sense not to live for fun
Your brain gets smart but your head gets dumb
So much to do, so much to see
So what's wrong with taking the back streets?
You'll never know if you don't go
You'll never shine if you don't glow
Hey now, you're an all-star, get your game on, go play
Hey now, you're a rock star, get the show on, get paid
And all that glitters is gold
Only shooting stars break the mold
It's a cool place and they say it gets colder
You're bundled up now, wait till you get older
But the meteor men beg to differ
Judging by the hole in the satellite picture
The ice we skate is getting pretty thin
The water's getting warm so you might as well swim
My world's on fire, how about yours?
That's the way I like it and I never get bored
";
    c.bench_function("Encode String Long", move |b| {
        b.iter(|| Base64::new_encode_bytes(s.as_bytes(), MIME))
    });
}

fn bench_decode_string_long(c: &mut Criterion) {
    let s: &str = "Somebody once told me the world is gonna roll me
I ain't the sharpest tool in the shed
She was looking kind of dumb with her finger and her thumb
In the shape of an \"L\" on her forehead
Well the years start coming and they don't stop coming
Fed to the rules and I hit the ground running
Didn't make sense not to live for fun
Your brain gets smart but your head gets dumb
So much to do, so much to see
So what's wrong with taking the back streets?
You'll never know if you don't go
You'll never shine if you don't glow
Hey now, you're an all-star, get your game on, go play
Hey now, you're a rock star, get the show on, get paid
And all that glitters is gold
Only shooting stars break the mold
It's a cool place and they say it gets colder
You're bundled up now, wait till you get older
But the meteor men beg to differ
Judging by the hole in the satellite picture
The ice we skate is getting pretty thin
The water's getting warm so you might as well swim
My world's on fire, how about yours?
That's the way I like it and I never get bored";
    let b64 = Base64::new_encode_bytes(s.as_bytes(), MIME);
    c.bench_function("Decode String Long", move |b| {
        b.iter(|| b64.decode_to_bytes())
    });
}

criterion_group!(
    benches,
    bench_convert_to_decimal,
    bench_set_from_unsigned,
    bench_set_from_string,
    bench_create_random,
    bench_enconde_string,
    bench_encode_string_long,
    bench_decode_string_long,
);

criterion_main!(benches);
