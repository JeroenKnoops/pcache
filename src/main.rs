extern crate clap;
use clap::{crate_authors, crate_description, crate_name, crate_version, App};

fn main() {
    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();
}
