struct Urbe {
    nomen : String
}

struct UrbeInVia {
    urbe      : Urbe,
    miliarium : u32
}

struct Via {
    nomen           : String,
    urbes_in_via    : Vec<UrbeInVia>
}

fn main() {
    let vias = vec![
        Via { nomen: "Flaminia".to_string(), urbes_in_via : vec![
            UrbeInVia { urbe: Urbe { nomen: "Ostia".to_string() }, miliarium: 0 },
            UrbeInVia { urbe: Urbe { nomen: "Roma".to_string() }, miliarium: 1 },
            UrbeInVia { urbe: Urbe { nomen: "Spoletum".to_string() }, miliarium: 10 },
            UrbeInVia { urbe: Urbe { nomen: "Ariminium".to_string() }, miliarium: 20 }
        ] },
        Via { nomen: "Aemilia".to_string(), urbes_in_via : vec![
            UrbeInVia { urbe: Urbe { nomen: "Ariminium".to_string() }, miliarium: 0 },
            UrbeInVia { urbe: Urbe { nomen: "Bononia".to_string() }, miliarium: 10 },
            UrbeInVia { urbe: Urbe { nomen: "Parma".to_string() }, miliarium: 20 },
            UrbeInVia { urbe: Urbe { nomen: "Placentia".to_string() }, miliarium: 30 }
        ] },
    ];

    for via in &vias {  // Why the &?
        println!("Via {}", via.nomen);
        for urbe_in_via in &via.urbes_in_via {
          println!("    {} [{}]", urbe_in_via.urbe.nomen, urbe_in_via.miliarium);
        }
    }
}
