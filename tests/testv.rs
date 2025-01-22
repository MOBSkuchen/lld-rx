#[cfg(test)]
mod tests {
    use lld_rx::{link, LldFlavor};

    #[test]
    fn test_via_version() {
        let res = link(LldFlavor::Coff, vec!["--version".to_string()]);
        res.debug_print();
    }
}