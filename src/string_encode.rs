pub fn decode_filename(s: &str) -> String {
    s.replace("%20", " ")
        .replace("%2C", ",")
        .replace("%7B", "{")
        .replace("%7C", "|")
        .replace("%7D", "}")
}

pub fn encode_filename(s: &str) -> String {
    s.replace(" ", "%20")
        .replace(",", "%2C")
        .replace("{", "%7B")
        .replace("|", "%7C")
        .replace("}", "%7D")
}