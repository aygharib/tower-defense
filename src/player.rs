
enum Race {
    Terran,
    Protoss,
    Zerg,
}

struct Player {
    race: Race,
    minerals: u32,
    gas: u32,
}