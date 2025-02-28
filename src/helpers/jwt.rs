use jwt::{SignWithKey, VerifyWithKey};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::{BTreeMap};
use dotenv::dotenv;

///
/// An HMAC key is created based on Sha256 and used to hash the passed user.
/// Claims are created from a map that has keys and values  and more details could be added to the claim.
pub fn _sign(user: String) -> String {
    dotenv().ok();
    let secret = std::env::var("SECRET").expect("Should have returned a secret");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).expect("Should create a hmac key");
    let mut claims:BTreeMap<String, String> = BTreeMap::new();
    claims.insert("role".to_string(), user);
    let token_string = claims.sign_with_key(&key).expect("Should have signed token with the key reference");
    return token_string;

}
///
/// Use provided token to evaluate the user and return true if it matches with the second user parameter.
/// An HMAC key is created based on Sha256 and used to verify the passed token.
pub fn _verify(token: String) -> BTreeMap<String, String> {
    dotenv().ok();
    let secret: String = std::env::var("SECRET").expect("Should have returned a secret");
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).expect("Should have created an hmac key");
    let claims: BTreeMap<String, String> = token.verify_with_key(&key).expect("Should have verified the token");
    return claims;
}