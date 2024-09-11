use string::bwt::{burrows_wheeler_transform, inv_burrows_wheeler_transform};

fn main() {
    let result = burrows_wheeler_transform("THISISATEST".to_string());
    println!("The result of bwt is: {:?}", result);
    assert_eq!(inv_burrows_wheeler_transform(result), "THISISATEST");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("CARROT".to_string())),
            "CARROT"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("TOMATO".to_string())),
            "TOMATO"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THISISATEST".to_string())),
            "THISISATEST"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THEALGORITHMS".to_string())),
            "THEALGORITHMS"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("RUST".to_string())),
            "RUST"
        );
    }

    #[test]
    fn special_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!.!.!??.=::".to_string())),
            "!.!.!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform(
                "!{}{}(((&&%%!??.=::".to_string()
            )),
            "!{}{}(((&&%%!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("//&$[]".to_string())),
            "//&$[]"
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("".to_string())),
            ""
        );
    }
}
