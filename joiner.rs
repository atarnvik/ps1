use std::os;
use std::io::File;
use std::str;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
    	let tempArg = &args;
		let fname1 = &tempArg[1];
		let fname2 = &tempArg[2];

		let path1 = Path::new(fname1.clone());
		let path2 = Path::new(fname2.clone());

		let msg_file1 = File::open(&path1);
		let msg_file2 = File::open(&path2);

		match (msg_file1, msg_file2) {
			(Some(mut msg1), Some(mut msg2)) => {
				let msg1_bytes: ~[u8] = msg1.read_to_end();
				let msg2_bytes: ~[u8] = msg2.read_to_end();
                join(msg1_bytes, msg2_bytes); 
			},
            (_, _) => fail!("Error opening message file")
		}
	}
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(msg1_bytes: &[u8], msg2_bytes: &[u8]) {
    let decrypted_bytes = xor(msg1_bytes, msg2_bytes);
    let s = str::from_utf8(decrypted_bytes);
    println(s);
}