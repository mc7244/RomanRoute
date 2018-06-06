// struct Urbe {
//     nomen : String
// }

pub struct UrbeInVia {
    pub nomen     : String,
    pub miliarium : u32
}

pub struct Via {
    pub nomen           : String,
    pub urbes_in_via    : Vec<UrbeInVia>
}

pub fn get_viae() -> Vec<Via> {
    let viae = vec![
        Via { nomen: "Flaminia".to_string(), urbes_in_via : vec![
            UrbeInVia { nomen: "Ostia".to_string(), miliarium: 0 },
            UrbeInVia { nomen: "Roma".to_string(), miliarium: 1 },
            UrbeInVia { nomen: "Spoletum".to_string(), miliarium: 10 },
            UrbeInVia { nomen: "Ariminium".to_string(), miliarium: 20 }
        ] },
        Via { nomen: "Aemilia".to_string(), urbes_in_via : vec![
            UrbeInVia { nomen: "Ariminium".to_string(), miliarium: 0 },
            UrbeInVia { nomen: "Bononia".to_string(), miliarium: 10 },
            UrbeInVia { nomen: "Parma".to_string(), miliarium: 20 },
            UrbeInVia { nomen: "Placentia".to_string(), miliarium: 30 }
        ] },
        Via { nomen: "Aurelia".to_string(), urbes_in_via : vec![
            UrbeInVia { nomen: "Roma".to_string(), miliarium: 0 },
            UrbeInVia { nomen: "Populonium".to_string(), miliarium: 10 },
            UrbeInVia { nomen: "Pisae".to_string(), miliarium: 20 },
            UrbeInVia { nomen: "Luna".to_string(), miliarium: 30 }
        ] },
    ];

    viae
}