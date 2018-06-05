struct Urbe {
  name  : String
}

fn main() {
    let urbi = [Urbe {name:"Roma".to_string()}, Urbe {name:"Mediolanum".to_string()}];

    for urbe in &urbi {
      println!("{}", urbe.name);
    }
}
