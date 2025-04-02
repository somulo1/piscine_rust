pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")  // Replace every space with "%20"
}