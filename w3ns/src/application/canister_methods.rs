use ic_kit::candid::{candid_method, export_service};
use ic_kit::macros::*;
use ic_kit::*;

#[query]
#[candid_method(query)]
pub fn name() -> String {
    return String::from("W3NS");
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        println!("{:?}", dir);
        let dir = dir.parent().unwrap().join("candid");
        write(dir.join("w3ns.did"), export_candid()).expect("Write failed.");
    }
}