use super::*;

pub const MERKLE_ROOT: [u8; 32] = [
    226, 232, 88, 169, 58, 205, 6, 218, 105, 50, 193, 122, 78, 205, 77, 52, 61, 10, 186, 180, 53,
    72, 106, 171, 236, 129, 122, 153, 38, 144, 59, 143,
];

pub fn get_merkle_root(env: &Env) -> BytesN<32> {
    BytesN::from_array(env, &MERKLE_ROOT)
}

pub const TOTAL_AMOUNT: i128 = 4905160406;

pub const ACCOUNT_1: &str = "GBOAWTUJNSI5VKE3MDGY32LJF723OCQ42XYLNJWXDHCJKRZSFV3PKKMY";
pub const ACCOUNT_2: &str = "CDVQVKOY2YSXS2IC7KN6MNASSHPAO7UN2UR2ON4OI2SKMFJNVAMDX6DP";
pub const INVALID_ACCOUNT: &str = "GDMTVHLWJTHSUDMZVVMXXH6VJHA2ZV3HNG5LYNAZ6RTWB7GISM6PGTUV";

pub fn get_account_address(env: &Env, account: &str) -> Address {
    match account {
        ACCOUNT_1 => Address::from_string(&String::from_str(&env, ACCOUNT_1)),
        ACCOUNT_2 => Address::from_string(&String::from_str(&env, ACCOUNT_2)),
        _ => panic!("Invalid account"),
    }
}

pub const ACCOUNT_1_ALLOCATION: i128 = 100000000;
pub const ACCOUNT_2_ALLOCATION: i128 = 12634851;

pub const ACCOUNT_1_MERKLE_PROOFS: [[u8; 32]; 7] = [
    [
        156, 239, 167, 103, 160, 6, 57, 237, 88, 109, 132, 190, 242, 100, 107, 147, 241, 101, 0,
        46, 162, 164, 239, 192, 244, 198, 233, 211, 164, 254, 80, 51,
    ],
    [
        140, 222, 20, 151, 8, 166, 37, 211, 169, 97, 97, 143, 183, 181, 193, 160, 157, 17, 164,
        175, 241, 131, 110, 246, 20, 222, 106, 13, 51, 121, 155, 75,
    ],
    [
        209, 134, 21, 144, 199, 151, 15, 31, 9, 31, 77, 13, 197, 68, 131, 56, 253, 66, 159, 155,
        129, 78, 7, 116, 111, 239, 71, 155, 1, 59, 154, 198,
    ],
    [
        77, 122, 27, 117, 21, 202, 140, 14, 110, 39, 203, 190, 172, 11, 199, 224, 72, 193, 168,
        156, 43, 134, 39, 76, 20, 245, 234, 82, 124, 131, 41, 172,
    ],
    [
        67, 232, 166, 151, 190, 193, 236, 250, 189, 227, 112, 37, 232, 139, 127, 53, 48, 251, 227,
        113, 25, 73, 40, 162, 0, 113, 45, 140, 94, 80, 214, 10,
    ],
    [
        79, 87, 63, 241, 170, 241, 94, 169, 41, 218, 106, 49, 251, 176, 219, 179, 217, 51, 100,
        183, 18, 238, 146, 63, 8, 39, 138, 7, 21, 72, 97, 164,
    ],
    [
        32, 33, 232, 239, 41, 200, 245, 178, 138, 84, 201, 36, 93, 157, 124, 251, 39, 145, 155,
        120, 38, 55, 58, 121, 17, 241, 29, 212, 161, 115, 179, 182,
    ],
];
pub const ACCOUNT_2_MERKLE_PROOFS: [[u8; 32]; 7] = [
    [
        42, 212, 114, 18, 113, 126, 52, 214, 123, 81, 14, 24, 168, 84, 197, 31, 151, 180, 225, 83,
        228, 248, 174, 87, 226, 50, 75, 180, 184, 190, 119, 217,
    ],
    [
        238, 146, 88, 81, 5, 51, 150, 53, 228, 233, 161, 126, 77, 245, 200, 49, 190, 253, 161, 243,
        185, 155, 72, 155, 137, 134, 95, 250, 241, 244, 162, 157,
    ],
    [
        199, 19, 140, 246, 114, 150, 193, 227, 87, 60, 39, 54, 204, 13, 213, 22, 172, 146, 32, 99,
        214, 145, 224, 62, 126, 248, 116, 17, 252, 191, 68, 243,
    ],
    [
        102, 219, 112, 79, 95, 158, 116, 11, 175, 211, 60, 173, 36, 119, 66, 172, 61, 57, 173, 243,
        171, 142, 57, 10, 205, 251, 52, 57, 131, 166, 34, 155,
    ],
    [
        112, 144, 30, 148, 185, 177, 50, 34, 81, 199, 98, 175, 181, 30, 112, 40, 121, 54, 198, 204,
        57, 117, 163, 82, 188, 8, 55, 111, 247, 11, 38, 12,
    ],
    [
        187, 145, 10, 67, 244, 178, 138, 82, 237, 240, 94, 223, 207, 96, 108, 5, 81, 75, 189, 208,
        136, 233, 222, 52, 64, 162, 181, 1, 49, 9, 154, 164,
    ],
    [
        32, 33, 232, 239, 41, 200, 245, 178, 138, 84, 201, 36, 93, 157, 124, 251, 39, 145, 155,
        120, 38, 55, 58, 121, 17, 241, 29, 212, 161, 115, 179, 182,
    ],
];

pub fn get_merkle_proofs(env: &Env, account: &str) -> Vec<BytesN<32>> {
    match account {
        ACCOUNT_1 => vec![
            env,
            BytesN::from_array(env, &ACCOUNT_1_MERKLE_PROOFS[0]),
            BytesN::from_array(env, &ACCOUNT_1_MERKLE_PROOFS[1]),
            BytesN::from_array(env, &ACCOUNT_1_MERKLE_PROOFS[2]),
            BytesN::from_array(env, &ACCOUNT_1_MERKLE_PROOFS[3]),
            BytesN::from_array(env, &ACCOUNT_1_MERKLE_PROOFS[4]),
            BytesN::from_array(env, &ACCOUNT_1_MERKLE_PROOFS[5]),
            BytesN::from_array(env, &ACCOUNT_1_MERKLE_PROOFS[6]),
        ],
        ACCOUNT_2 => vec![
            env,
            BytesN::from_array(env, &ACCOUNT_2_MERKLE_PROOFS[0]),
            BytesN::from_array(env, &ACCOUNT_2_MERKLE_PROOFS[1]),
            BytesN::from_array(env, &ACCOUNT_2_MERKLE_PROOFS[2]),
            BytesN::from_array(env, &ACCOUNT_2_MERKLE_PROOFS[3]),
            BytesN::from_array(env, &ACCOUNT_2_MERKLE_PROOFS[4]),
            BytesN::from_array(env, &ACCOUNT_2_MERKLE_PROOFS[5]),
            BytesN::from_array(env, &ACCOUNT_2_MERKLE_PROOFS[6]),
        ],
        _ => panic!("Invalid account"),
    }
}
