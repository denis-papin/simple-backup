#![cfg_attr(
feature = "cargo-clippy",
allow(unreadable_literal, decimal_literal_representation)
)]

#[cfg(test)]


use serde::{Deserialize, Serialize};
use serde_yaml::{Value, Mapping};
use std::{fs};
use std::fmt::Debug;
use unindent::unindent;
use std::path::Path;

fn test_serde<T>(thing: &T, yaml: &str)
    where
        T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug,
{
    let serialized = serde_yaml::to_string(&thing).unwrap();
    assert_eq!(yaml, serialized);

    let value = serde_yaml::to_value(&thing).unwrap();
    let serialized = serde_yaml::to_string(&value).unwrap();
    assert_eq!(yaml, serialized);

    let deserialized: T = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(*thing, deserialized);

    let value: Value = serde_yaml::from_str(yaml).unwrap();
    let deserialized: T = serde_yaml::from_value(value).unwrap();
    assert_eq!(*thing, deserialized);

    serde_yaml::from_str::<serde::de::IgnoredAny>(yaml).unwrap();
}

#[test]
fn test_vect_map() {

    let mut m1 =  Mapping::new();
    let mut m2 =  Mapping::new();
    let mut m3 =  Mapping::new();

    m1.insert(
        Value::String("projet 1".to_owned()),
        Value::String("c/a".to_owned()),
    );

    m2.insert(
        Value::String("projet 2".to_owned()),
        Value::String("bar".to_owned()),
    );

    m3.insert(
        Value::String("projet 3".to_owned()),
        Value::String("hug".to_owned()),
    );

    let thing_map = vec![m1, m2, m3];

    let yaml = unindent(
        "
        ---
        - projet 1: c/a
        - projet 2: bar
        - projet 3: hug",
    );

    test_serde(&thing_map, &yaml);
}


#[test]
fn test_config_read() {

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Config {
        pub source: Source,
        pub target: Target,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Source {
        pub paths: Vec<Mapping>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Target {
        pub path: String,
        #[serde(rename = "purge-after")]
        pub purge_after: u32,
    }

    // TODO : Read the DOKA_UT_ENV  variable to find the test files.
    let var = "C:/Users/denis/wks-tools/simple-backup/env/data/dummy.yml";

    let config_path: &Path = Path::new(&var);

    let data = fs::read_to_string(config_path).expect("Unable to read file");

    let config: Config = serde_yaml::from_str(&data).unwrap();

    // println!("config >>> {:?}", &config);

    match config.source.paths.get(1) {
        Some(x) => {
            let y = x.get(&Value::String("projet 2".to_owned()) ).unwrap();
            let z = y.as_str().unwrap();
            assert_eq!(z, "c:/toto le hero/tutu");
        },
        None => panic!("Cannot find the value!"),
    };

    let t = config.target.path.as_str();

    assert_eq!(t, "c:/toto/tutu");

}


//
//#[test]
//fn test_default() {
//    assert_eq!(Value::default(), Value::Null);
//}
//
//#[test]
//fn test_int() {
//    let thing = 256;
//    let yaml = unindent(
//        "
//        ---
//        256",
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_int_max_u64() {
//    let thing = ::std::u64::MAX;
//    let yaml = unindent(
//        "
//        ---
//        18446744073709551615",
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_int_min_i64() {
//    let thing = ::std::i64::MIN;
//    let yaml = unindent(
//        "
//        ---
//        -9223372036854775808",
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_int_max_i64() {
//    let thing = ::std::i64::MAX;
//    let yaml = unindent(
//        "
//        ---
//        9223372036854775807",
//    );
//    test_serde(&thing, &yaml);
//}
//
//serde_if_integer128! {
//    #[test]
//    fn test_i128_small() {
//        let thing: i128 = -256;
//        let yaml = unindent(
//            "
//            ---
//            -256",
//        );
//        test_serde(&thing, &yaml);
//    }
//
//    #[test]
//    fn test_u128_small() {
//        let thing: u128 = 256;
//        let yaml = unindent(
//            "
//            ---
//            256",
//        );
//        test_serde(&thing, &yaml);
//    }
//}
//
//#[test]
//fn test_float() {
//    let thing = 25.6;
//    let yaml = unindent(
//        "
//        ---
//        25.6",
//    );
//    test_serde(&thing, &yaml);
//
//    let thing = 25.;
//    let yaml = unindent(
//        "
//        ---
//        25.0",
//    );
//    test_serde(&thing, &yaml);
//
//    let thing = f64::INFINITY;
//    let yaml = unindent(
//        "
//        ---
//        .inf",
//    );
//    test_serde(&thing, &yaml);
//
//    let thing = f64::NEG_INFINITY;
//    let yaml = unindent(
//        "
//        ---
//        -.inf",
//    );
//    test_serde(&thing, &yaml);
//
//    let float: f64 = serde_yaml::from_str(&unindent(
//        "
//        ---
//        .nan",
//    ))
//        .unwrap();
//    assert!(float.is_nan());
//}
//
//#[test]
//fn test_vec() {
//    let thing = vec![1, 2, 3];
//    let yaml = unindent(
//        "
//        ---
//        - 1
//        - 2
//        - 3",
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_map() {
//    let mut thing = BTreeMap::new();
//    thing.insert(String::from("x"), 1);
//    thing.insert(String::from("y"), 2);
//    let yaml = unindent(
//        r#"
//        ---
//        x: 1
//        y: 2"#,
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_basic_struct() {
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    struct Basic {
//        x: isize,
//        y: String,
//        z: bool,
//    }
//    let thing = Basic {
//        x: -4,
//        y: String::from("hi\tquoted"),
//        z: true,
//    };
//    let yaml = unindent(
//        r#"
//        ---
//        x: -4
//        y: "hi\tquoted"
//        z: true"#,
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_nested_vec() {
//    let thing = vec![vec![1, 2, 3], vec![4, 5, 6]];
//    let yaml = unindent(
//        "
//        ---
//        - - 1
//          - 2
//          - 3
//        - - 4
//          - 5
//          - 6",
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_nested_struct() {
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    struct Outer {
//        inner: Inner,
//    }
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    struct Inner {
//        v: u16,
//    }
//    let thing = Outer {
//        inner: Inner { v: 512 },
//    };
//    let yaml = unindent(
//        r#"
//        ---
//        inner:
//          v: 512"#,
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_option() {
//    let thing = vec![Some(1), None, Some(3)];
//    let yaml = unindent(
//        "
//        ---
//        - 1
//        - ~
//        - 3",
//    );
//    test_serde(&thing, &yaml);
//}


//#[test]
//fn test_unit() {
//    let thing = vec![(), ()];
//    let yaml = unindent(
//        "
//        ---
//        - ~
//        - ~",
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_unit_variant() {
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    enum Variant {
//        First,
//        Second,
//    }
//    let thing = Variant::First;
//    let yaml = unindent(
//        r#"
//        ---
//        First"#,
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_newtype_struct() {
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    struct OriginalType {
//        v: u16,
//    }
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    struct NewType(OriginalType);
//    let thing = NewType(OriginalType { v: 1 });
//    let yaml = unindent(
//        r#"
//        ---
//        v: 1"#,
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_newtype_variant() {
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    enum Variant {
//        Size(usize),
//    }
//    let thing = Variant::Size(127);
//    let yaml = unindent(
//        r#"
//        ---
//        Size: 127"#,
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_tuple_variant() {
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    enum Variant {
//        Rgb(u8, u8, u8),
//    }
//    let thing = Variant::Rgb(32, 64, 96);
//    let yaml = unindent(
//        r#"
//        ---
//        Rgb:
//          - 32
//          - 64
//          - 96"#,
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_struct_variant() {
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    enum Variant {
//        Color { r: u8, g: u8, b: u8 },
//    }
//    let thing = Variant::Color {
//        r: 32,
//        g: 64,
//        b: 96,
//    };
//    let yaml = unindent(
//        r#"
//        ---
//        Color:
//          r: 32
//          g: 64
//          b: 96"#,
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_value() {
//    use serde_yaml::{Mapping, Number};
//
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    pub struct GenericInstructions {
//        #[serde(rename = "type")]
//        pub typ: String,
//        pub config: Value,
//    }
//    let thing = GenericInstructions {
//        typ: "primary".to_string(),
//        config: Value::Sequence(vec![
//            Value::Null,
//            Value::Bool(true),
//            Value::Number(Number::from(65535)),
//            Value::Number(Number::from(0.54321)),
//            Value::String("s".into()),
//            Value::Mapping(Mapping::new()),
//        ]),
//    };
//    let yaml = unindent(
//        r#"
//        ---
//        type: primary
//        config:
//          - ~
//          - true
//          - 65535
//          - 0.54321
//          - s
//          - {}"#,
//    );
//    test_serde(&thing, &yaml);
//}
//
//#[test]
//fn test_mapping() {
//    use serde_yaml::Mapping;
//    #[derive(Serialize, Deserialize, PartialEq, Debug)]
//    struct Data {
//        pub substructure: Mapping,
//    }
//
//    let mut thing = Data {
//        substructure: Mapping::new(),
//    };
//    thing.substructure.insert(
//        Value::String("a".to_owned()),
//        Value::String("foo".to_owned()),
//    );
//    thing.substructure.insert(
//        Value::String("b".to_owned()),
//        Value::String("bar".to_owned()),
//    );
//
//    let yaml = unindent(
//        "
//        ---
//        substructure:
//          a: foo
//          b: bar",
//    );
//
//    test_serde(&thing, &yaml);
//}