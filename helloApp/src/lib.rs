// A Marco Polo Game:
// If the name Marco is given, the program respond Polo
// Else, responds "Whats your name?"


pub fn marco_polo(name: str) -> String{
    if name == "Marco" {
        "Polo".to_string() 
    }
    else {
        "Whats your name?".to_string()
    }
}