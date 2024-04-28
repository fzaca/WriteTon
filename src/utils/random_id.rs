use nanoid::nanoid;

const ALPHABET: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Generate a random id of 10 characters with random id
pub fn generate_random_id() -> String {
    nanoid!(7, &ALPHABET)
}
