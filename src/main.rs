mod viae;

fn main() {
    let vrs = viae::get_viae();

    find_path(vrs, "Roma".to_string(), "Pisae".to_string());
}

fn find_path(vrs: Vec<viae::Via>, start: String, finish: String) {
    println!("Start: {}, Finish: {}", start, finish);

    for via in &vrs {
        for urbe_in_via in &via.urbes_in_via {
            if urbe_in_via.nomen == start {
                println!("{} [{}]", via.nomen, urbe_in_via.miliarium);
            }
        }
    }
}
