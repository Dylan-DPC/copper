use nom::{space, line_ending};
use nom::IResult::Done;

use ::component::geometry::Point;

use ::common_parsing::{utf8_str, point};


#[derive(Debug)]
pub struct ComponentInstance {
    pub name: String,
    pub reference: String,
    pub position: Point,
}

impl ComponentInstance {
    pub fn parse(input: &[u8]) -> Option<ComponentInstance> {
        let parse_res = component_instance(input);

        println!("Parse result: {:#?}", parse_res);

        match parse_res {
            Done(_, o) => Some(o),
            _ => None
        }
    }
}

named!(pub component_instance<ComponentInstance>, 
    do_parse!(
        tag_s!("$Comp") >> line_ending >>
        tag_s!("L") >> space >> name: utf8_str >> space >> reference: utf8_str >> line_ending >>
        tag_s!("U") >> take_until_either!("\r\n") >> line_ending >>
        tag_s!("P") >> space >> position: point >> line_ending >>
        take_until_s!("$EndComp") >>
        (ComponentInstance {
            name: name.to_owned(),
            reference: reference.to_owned(),
            position: position,
        })
    )
);



#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_COMPONENT: &'static str = r##"$Comp
L GND #PWR?
U 1 1 558C20D6
P 4950 2600
F 0 "#PWR?" H 4950 2350 50  0001 C CNN
F 1 "GND" H 4950 2450 50  0000 C CNN
F 2 "" H 4950 2600 60  0000 C CNN
F 3 "" H 4950 2600 60  0000 C CNN
	1    4950 2600
	1    0    0    -1  
$EndComp"##;

    fn parse_cmp() -> ComponentInstance {
        let (_, cmp) = component_instance(SAMPLE_COMPONENT.as_bytes()).unwrap();

        cmp
    }

    #[test]
    fn parse_component_name() {
        let cmp = parse_cmp();

        assert_eq!(cmp.name, "GND");
    }

    #[test]
    fn parse_reference() {
        let cmp = parse_cmp();

        assert_eq!(cmp.reference, "#PWR?");
    }

    #[test]
    fn parse_position() {
        let cmp = parse_cmp();

        assert_eq!(cmp.position, Point { x: 4950.0, y: 2600.0 });
    }
}