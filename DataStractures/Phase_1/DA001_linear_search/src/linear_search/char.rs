#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Cordanets {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub fn get_position() -> Vec<Cordanets> {
    vec![Cordanets { x: 0, y: 0, z: 0 }, Cordanets { x: 1, y: 34, z: 99 }, Cordanets { x: 2, y: 3, z: 4 }, Cordanets { x: 122, y: 0, z: 99 }]
}

#[derive(PartialEq, Eq)]
/// # In Rust, equality and ordering are broken down into a hierarchy. You can't have a total ordering (Ord) without first establishing basic equality (PartialEq).
/// [ Ord ]         **4. Everything can be ordered perfectly (No surprises)**
///    🔼
/// [ PartialOrd ]  **3. Things can be compared, but maybe some are unordered (like NaN)**
///    🔼
/// [ Eq ]          **2. Structural equality (A always equals A)**
///    🔼
/// [ PartialEq ]   **1. Basic equality (Can use == and !=)**
/// ## PartialOrd => allows to compair and returns Option<Ordering>
/// ```
/// //eg.
/// fn x(_a,_b)-> Option<Ordering>{...}
/// ```
/// ### BUT PartialOrd dont know the Order. hence => Ord
///
/// Ord defined Order directly.
pub enum Char {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    ZZ,
}
impl PartialOrd for Char {
    // Returns Option<Ordering>
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // compairing self with Other which is also type of self.
        Some(self.cmp(other))
    }
}

// R: Ord cannot be implimented if someting can not be partially Ordered Hence PartialOrd 🔼
impl Ord for Char {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // just some IN-LINE CLOUSER with Match {..} [(local helper fn)]
        // That change the type into something compairable eg Numbers
        let to_weight = |c: &Char| match c {
            Char::A => 1,
            Char::B => 2,
            Char::C => 3,
            Char::D => 4,
            Char::E => 5,
            Char::F => 6,
            Char::G => 7,
            Char::H => 8,
            Char::I => 9,
            Char::J => 10,
            Char::K => 11,
            Char::L => 12,
            Char::M => 13,
            Char::N => 14,
            Char::O => 15,
            Char::P => 16,
            Char::Q => 17,
            Char::R => 18,
            Char::S => 19,
            Char::T => 20,
            Char::U => 21,
            Char::V => 22,
            Char::W => 23,
            Char::X => 24,
            Char::Y => 25,
            Char::Z => 26,
            Char::ZZ => 26,
        };
        // compaire with the compairable value instead of actual value.
        to_weight(self).cmp(&to_weight(other))
    }
}
