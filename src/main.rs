mod viae;

static DEBUG : usize = 1;

fn main() {
    let vrs = viae::get_viae();

    let start_urbe = "Roma".to_string();

    //let dest_urbe = "Ariminium".to_string();
    let dest_urbe = "Parma".to_string();
    //let dest_urbe = "Mediolanum".to_string();

    //let traversed_urbes : Vec<String> = [""]
    find_path(&vrs, start_urbe, dest_urbe, "".to_string(), &vec!["".to_string()], 0);
}

fn find_path(
    vrs: &Vec<viae::Via>, start_urbe_nomen: String, dest_urbe_nomen: String,
    from_via: String, traversed_urbes_nomen: &Vec<String>, depth: usize
){
    if depth > 10 {
        for _dp in 0 .. depth { print!("  "); }
        println!("!!! TOO MUCH RECURSION !!!");
        return;
    }

    let steps = traversed_urbes_nomen.len();

    // Find all via which traverse the urbe
    for via in vrs {
        // Avoid walking the same via we are coming from, or it would be an endless loop
        if via.nomen == from_via {
            continue;
        }
        let via_len = via.urbes.len();

        let start_urbe_idx : usize = match get_urbe_idx_in_via(via, &start_urbe_nomen) {
            Some(idx)   => { idx }
            None        => { continue; }
        };
        let start_urbe = &via.urbes[start_urbe_idx];

        if DEBUG > 0 {
            if depth > 0 {
                for _dp in 1 .. depth { print!("  "); }
                print!("-> "); 
            }
            println!("VIA: {} [{} - {}]", via.nomen, start_urbe.nomen, start_urbe.miliarium);
        }

        let mut indent = String::new();
        for _dp in 0 .. depth { for _si in 0 .. 3 { indent.push(' '); } }

        // We now walk the via up and down relative to the start_urbe, in order to
        // find the destination or a crossing with another via (we recurse in such case)
        let mut curtraversed_bk = traversed_urbes_nomen.clone();
        let mut curtraversed_fw = traversed_urbes_nomen.clone();
        for i in 0 .. via_len {
            let urbe = &via.urbes[i];

            // We are only interested in other urbes
            if urbe.nomen == start_urbe_nomen {
                continue;
            }

            let cursteps = steps + (start_urbe_idx as isize - i as isize).abs() as usize;
            let direction = if start_urbe_idx > i { "BK" } else { "FW" };
            if DEBUG > 0 {
                println!("{}STEP {}/{}: {} [{} - {}]", indent, cursteps, direction, via.nomen.clone(), urbe.nomen, urbe.miliarium);
            }

            // If we have already been in this urbe before in this path, abandon the path
            // otherwise we will loop
            let urbe_nomen = &urbe.nomen;
            if traversed_urbes_nomen.iter().position(|ref el| *el == urbe_nomen) != None {
                if DEBUG > 0 {
                    println!("{}   already been here, abandoning path", indent);
                }
                continue;
            }

            // Push on the path we are walking.
            // Note: do this before creating the curtraversed reference, or it wouldn't be allowed by Rust
            if direction == "BK" {
                curtraversed_bk.push(urbe.nomen.clone());
            } else {
                curtraversed_fw.push(urbe.nomen.clone());
            }
            
            if urbe.nomen == dest_urbe_nomen {
                println!("{}   !!! FOUND DEST !!!\n", indent);
                break;
            } else {
                let curtraversed = if direction == "BK" { &curtraversed_bk } else { &curtraversed_fw };
                find_path(vrs, urbe.nomen.clone(), dest_urbe_nomen.clone(), via.nomen.clone(), curtraversed, depth+1);
            }
        }
    }
}

fn get_urbe_idx_in_via(via: &viae::Via, urbe_nomen: &String) -> Option<usize> {
    for i in 0 .. via.urbes.len() {
        let urbe = &via.urbes[i];

        // We are only interested in the start_urbe
        if urbe.nomen != *urbe_nomen {
            continue;
        }

        return Some(i);
    }

    None
}