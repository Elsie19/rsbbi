// Basically something I found out a while ago is that some Jews believe they cannot remove the
// tetragrammaton text when it is written or printed, so basically to make this "kosher", I have to
// log any reference of "YHWH" in Hebrew to a file, but what's funnier is that I can't type the
// tetragrammaton in here, so I'll just convert it to UTF8 hex and hehehehe.
// Source? My friend. Idk ask him if you have questions @oklopfer.

use serde_json::Value;

static TETRA: &[u8] = b"\xd7\x99\xd7\x94\xd7\x95\xd7\x94";

pub fn check_for_tetra(text: &Vec<Value>) -> bool {
    for line in text {
        if line
            .as_str()
            .expect("Could not parse line checking for the tetragrammaton")
            .contains(std::str::from_utf8(TETRA).unwrap())
        {
            return true;
        }
    }
    false
}
