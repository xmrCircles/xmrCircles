//TODO: Define the circles, structs the messages passed, join circle logic, Voting logic
//TODO: serialize structs

//The circles can have a permanent or temporary leader
enum Leader {
    Strict,
    Temporary,
    None,
}

//Nodes identify the other instances of the application and the participants in the circle
#[derive(Debug)]
struct Node {
  name: String,
  id: String,
  pgp_pubkey: String,
  onion: String
}


// A circle defines the Nodes connected to each other
#[derive(Debug)]
struct Circle {
    name: String,
    identifier: String,
    leader_type: Leader,
    leader_id: String,
    members: String[] // Contains the Node Id's
}

struct Invite{}

struct Poll{}
