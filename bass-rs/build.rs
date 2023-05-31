

// some extra checking is done here
fn main() {
    // fx is not ready so error if someone tries to use it
    #[cfg(feature="bass_fx")]
    panic!("BassFX is not ready to be used!")

}