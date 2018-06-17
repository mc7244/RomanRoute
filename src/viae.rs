pub struct Urbe {
    pub nomen     : String,
    pub miliarium : u32
}

pub struct Via {
    pub nomen     : String,
    pub urbes     : Vec<Urbe>
}

pub fn get_viae() -> Vec<Via> {
    let viae = vec![
        Via { nomen: "Aemilia".to_string(), urbes : vec![
            Urbe { nomen: "Ariminium".to_string(), miliarium: 0 },
            Urbe { nomen: "Bononia".to_string(), miliarium: 10 },
            Urbe { nomen: "Parma".to_string(), miliarium: 20 },
            Urbe { nomen: "Placentia".to_string(), miliarium: 30 },
        ] },
        Via { nomen: "Flaminia".to_string(), urbes : vec![
            Urbe { nomen: "Ostia".to_string(), miliarium: 0 },
            Urbe { nomen: "Roma".to_string(), miliarium: 1 },
            Urbe { nomen: "Spoletum".to_string(), miliarium: 10 },
            Urbe { nomen: "Ariminium".to_string(), miliarium: 20 }
        ] },
    ];

    // let viae = vec![
    //     Via { nomen: "Aemilia".to_string(), urbes : vec![
    //         Urbe { nomen: "Ariminium".to_string(), miliarium: 0 },
    //         Urbe { nomen: "Bononia".to_string(), miliarium: 10 },
    //         Urbe { nomen: "Parma".to_string(), miliarium: 20 },
    //         Urbe { nomen: "Placentia".to_string(), miliarium: 30 },
    //     ] },
    //     Via { nomen: "Cassia".to_string(), urbes : vec![
    //         Urbe { nomen: "Roma".to_string(), miliarium: 0 },
    //         Urbe { nomen: "Arretium".to_string(), miliarium: 10 },
    //         Urbe { nomen: "Florentia".to_string(), miliarium: 20 },
    //         Urbe { nomen: "Luna".to_string(), miliarium: 30 },
    //     ] },
    //     Via { nomen: "Claudia Augusta".to_string(), urbes : vec![
    //         Urbe { nomen: "Florentia".to_string(), miliarium: 0 },
    //         Urbe { nomen: "Bononia".to_string(), miliarium: 10 },
    //         Urbe { nomen: "Verona".to_string(), miliarium: 20 },
    //         Urbe { nomen: "Aug. Vindelicum".to_string(), miliarium: 30 },
    //         Urbe { nomen: "Cambodunum".to_string(), miliarium: 40 },
    //         Urbe { nomen: "Curia".to_string(), miliarium: 50 },
    //         Urbe { nomen: "Mediolanum".to_string(), miliarium: 60 },
    //     ] },
    //     Via { nomen: "Aurelia".to_string(), urbes : vec![
    //         Urbe { nomen: "Roma".to_string(), miliarium: 0 },
    //         Urbe { nomen: "Populonium".to_string(), miliarium: 10 },
    //         Urbe { nomen: "Pisae".to_string(), miliarium: 20 },
    //         Urbe { nomen: "Luna".to_string(), miliarium: 30 },
    //     ] },
    //     Via { nomen: "Flaminia".to_string(), urbes : vec![
    //         Urbe { nomen: "Ostia".to_string(), miliarium: 0 },
    //         Urbe { nomen: "Roma".to_string(), miliarium: 1 },
    //         Urbe { nomen: "Spoletum".to_string(), miliarium: 10 },
    //         Urbe { nomen: "Ariminium".to_string(), miliarium: 20 }
    //     ] },
    // ];

    viae
}