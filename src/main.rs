mod viae;

fn main() {
    let vrs = viae::get_viae();

    let start_urbe = "Roma".to_string();
    // let start_via = find_via(&vrs, start_urbe.clone()).expect("Starting urbe not found in our viae!");
    // println!("START: {} [via: {}]", start_urbe, start_via);

    let dest_urbe = "Ariminium".to_string();
    // let dest_via = find_via(&vrs, dest_urbe.clone()).expect("Destination urbe not found un our database");
    // println!("DESTINATION: {} [via: {}]", dest_urbe, dest_via);

    let path : Vec<String> = vec![];
    find_path(&vrs, start_urbe, dest_urbe, path);
}

// TODO: accept &str instead of string, but lifetimes need to be defined - one I understand them
// fn find_via(vrs: &Vec<viae::Via>, nomen: String) -> Option<(String)> {
//     for via in vrs { // "via" is a borrowed reference
//         for urbe in &via.urbes { // "urbe" is a borrowed reference
//             if urbe.nomen == nomen {
//                 return Some( via.nomen.clone() );
//                 //println!("{} [{}]", via.nomen, urbe.miliarium);
//             }
//         }
//     }
//     None
// }

fn find_path(vrs: &Vec<viae::Via>, start_urbe: String, dest_urbe: String, mut path: Vec<String>) {
    // Find all via which traverse the urbe
    // TODO: maybe use a grep() to find the via
    for via in vrs {
        let via_len = *(&via.urbes.len());
        for i in 0 .. via_len { // "urbe" is a borrowed reference
            let urbe = &via.urbes[i];
            if urbe.nomen == start_urbe {
                println!("CUR: {} [{} - {}]", via.nomen, urbe.nomen, urbe.miliarium);
                // Walk the via in both directions
                if ( i < via_len-1 ) {
                    for j in i+1 .. via_len {
                        let next_urbe = &via.urbes[j];
                        println!("NX: {} [{} - {}]", via.nomen, next_urbe.nomen, next_urbe.miliarium);
                    }
                }
                if ( i > 0 ) {
                    for j in (0 .. i).rev() {
                        let prev_urbe = &via.urbes[j];
                        println!("PV: {} [{} - {}]", via.nomen, prev_urbe.nomen, prev_urbe.miliarium);
                    }
                }
                //Vec.index()
                //return Some( via.nomen.clone() );
            }
        }
    }
    // if path.len() < 10 {
    //     path.push( start_urbe.clone() );
    //     find_path(start_urbe, start_via, path);
    // }
}