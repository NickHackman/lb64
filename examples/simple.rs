//! Simple example to show encoding and decoding of bytes using the MIME compliant config constant
extern crate lb64; // Include base64 crate

use lb64::config::MIME;
use lb64::Base64;

fn main() {
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
    let b64 = Base64::new_encode_bytes(s.as_bytes(), MIME); // Create Base64 from the encoded &str
    println!("{}", b64);
    println!("{}", String::from_utf8(b64.decode_to_bytes()).unwrap()); // Decodes the Base64 into bytes then convert that into utf8
}
