#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate las;

use rocket::State;
use std::fs::File;
use std::sync::Mutex;

/**
 * las::Reader internally holds a Box<dyn las::reader::PointReader>, a trait which does not implement 'Send',
 * so we can't wrap the las::Reader in a Mutex to use it as shared state with Rocket.
 *
 * Internally, the las::reader::PointReader trait (or some of its implementations at least) refer to the
 * LasZipDecompressor struct of the laz_rs crate, which is also not 'Send'
 */
type RocketState = Mutex<las::Reader>;

#[get("/test")]
fn dummy_handler(state: State<RocektState>) -> u64 {
    let reader = state.lock().unwrap();

    reader.header().number_of_points()
}

fn main() {
    let las_reader = las::Reader::from_path("test.laz").unwrap();

    rocket::ignite()
        .mount("/", routes![dummy_handler])
        .manage(Mutex::new(las_reader))
        .launch();
}
