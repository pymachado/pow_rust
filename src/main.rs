use std::ops::Add;

use::sha256::digest;

//return the time in compute the nonce
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let prevHash: String = String::from("3a7bd3e2360a3d29eea436fcfb7e44c735d117c42d1c1835420b6b9942dd4f1b");
    let chalanger:String = args[1].to_string();
    let mut nonce:u32 = 0;
    let mut raw_data:[String;3]= [prevHash, args[0].to_string(), nonce.to_string()];
    let mut hash:String = compute_sha256(&add_string(&raw_data));
    loop {
        
        if hash < chalanger {
            break;
        }
        else {
            nonce +=1;
            raw_data[2] = nonce.to_string();
            hash = compute_sha256(&add_string(&raw_data));
            //println!("hash {}, nonce {}", hash, nonce);
        }
    }
    
    println!("Hash resultant is {}", nonce);
    println!("Hash is {}", hash);

    //compute_sha256(&add_string(&raw_data));
}

fn compute_sha256(data:&String) -> String {
    digest(data.to_string())
}

fn add_string(string_vec:&[String;3]) -> String {
    let mut data:String = String::from("");
    for word in string_vec {
        data.push_str(word);
    }
    data
}

