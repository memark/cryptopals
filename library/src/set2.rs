#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn challenge_9_implement_pkcs_7_padding() {
        assert_eq!(
            pkcs7_padding("YELLOW SUBMARINE", 20),
            "YELLOW SUBMARINE\x04\x04\x04\x04"
        );
    }
}
