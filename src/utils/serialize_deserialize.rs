use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    postcode: String,
    city: String
}

pub fn serialize_deserialize_examples() {
    let person = Person {
        name: "Broussel".to_string(),
        age: 30,
        address: Address {
            city: "Granollers".to_string(),
            postcode: "08402".to_string(),
            street: "Lope de Vega, 4, 2-1".to_string()
        }
    };

    // Sérialisation en JSON
    let json = serde_json::to_string_pretty(&person).unwrap();
    println!("JSON:\n{}", json);

    // Désérialisation à partir du JSON
    let personne_deserialisee: Person = serde_json::from_str(&json).unwrap();
    println!("Personne désérialisée : {:#?}", personne_deserialisee);    
}