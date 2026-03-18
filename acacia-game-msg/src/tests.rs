use super::*;

#[test]
fn test() {
    let msg = protocol::ReqLogin {
        provider: "1".into(),
        token: "2".into(),
        client_profile: None,
    };
    println!("{:?}", msg);
}
