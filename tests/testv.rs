#[cfg(test)]
mod tests {
    use linka::{link, LldFlavor};

    #[test]
    fn test_via_version() {
        let res = link(LldFlavor::Coff, vec!["--version".to_string()].as_ref());
        res.debug_print();
    }
}