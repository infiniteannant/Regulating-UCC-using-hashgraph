use rocket_contrib::json::{Json};
use shamir::SecretData;
use base64;
use crate::client_call::SplitSet;

///generate splits and then encrypt them with their public keys
/// store public keys to blockchain
/// return that to RTM
/// only valid RTM will be able to initiate the call sucessfully

///splits the phone number
/// out of 3 are needed to open the number
/// number : phone number
/// threshold : min needed to open number
/// returns json of base64 splits
/// 

///loop around to check until
// if contains '/' or '+'
/// send data to requested parties
/// send whole packet
#[get("/<number>/<threshold>")]
pub fn generate_splits(number: String, threshold: u8) -> Json<SplitSet> {
    let number_array = SecretData::with_secret(&number, threshold);

    let share_oap = number_array.get_share(1);
    let share_tap = number_array.get_share(2);
    let share_ir = number_array.get_share(3);
    let share_rtm = number_array.get_share(4);

    let encoding_share_rtm: String = base64::encode(&share_rtm);
    let encoding_share_ir: String = base64::encode(&share_ir);
    let encoding_share_oap: String = base64::encode(&share_oap);
    let encoding_share_tap: String = base64::encode(&share_tap);

    let split_set = SplitSet{
        share_rtm: encoding_share_rtm,
        share_oap: encoding_share_oap,
        share_ir: encoding_share_ir,
        share_tap : encoding_share_tap
    };
    Json(split_set)
}

///generate phone number using shamir secret
/// split_rtm : split rtm
/// split_ir : split ir
/// split_oap : split oap
/// returns decoded
/// # Arguments
/// *
#[get("/<split_rtm>/<split_ir>/<split_oap>")]
pub fn recover_secret(split_rtm: String, split_ir: String, split_oap: String) -> String {
    let dec = base64::decode(&split_rtm).unwrap();
    let dec2 = base64::decode(&split_ir).unwrap();
    let dec3 = base64::decode(&split_oap).unwrap();
//    println!("dec1 = {:?}", &dec);
//    println!("dec2 = {:?}", &dec2);
//    println!("dec3 = {:?}", &dec3);
    SecretData::recover_secret(3, vec![dec, dec2, dec3])
        .unwrap_or("Wrong Key used!..".to_string())
}


