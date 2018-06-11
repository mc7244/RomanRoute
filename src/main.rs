mod viae;

fn main() {
    let vrs = viae::get_viae();

    let (start_urbe, start_via) = find_urbe(&vrs, "Roma".to_string()).expect("Starting urbe not found");
    println!("START: {} [via: {} - miliarum: {}]", start_urbe.nomen, start_via.nomen, start_urbe.miliarium);

    let (dest_urbe, dest_via) = find_urbe(&vrs, "Ariminium".to_string()).expect("Destination urbe not found");
    println!("DESTINATION: {} [via: {} - miliarum: {}]", dest_urbe.nomen, dest_via.nomen, dest_urbe.miliarium);
}

// TODO: accept &str instead of string, but lifetimes need to be defined - one I understand them
fn find_urbe(vrs: &Vec<viae::Via>, nomen: String) -> Option<(&viae::Urbe, &viae::Via)> {
    for via in vrs { // "via" is a borrowed reference
        for urbe in &via.urbes { // "urbe" is a borrowed reference
            if urbe.nomen == nomen {
                return Some((urbe, via));
                //println!("{} [{}]", via.nomen, urbe.miliarium);
            }
        }
    }
    None
}

// fn find_path(start_via: &Via) {
//      println!("{} [{}]", via.nomen, urbe.miliarium);
//}