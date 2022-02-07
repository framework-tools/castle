use super::type_system::Type;


#[derive(Debug, PartialEq)]
pub struct VecType {
    pub inner_type: Box<Type>,
}

impl VecType {
    pub fn new(type_: &str) -> Option<Type> {
        let mut type_inside_vec_as_str = String::new();
        let mut parsing_inside_type = false;
        let mut i = 0;
        let mut not_a_vec = false;
        loop {
            let c = type_.chars().nth(i);
            let c = c.unwrap();
            //check that first 3 characters of type_ is "Vec"
            if i == 0 { if c != 'V' { not_a_vec = true; break; } }
            if i == 1 { if c != 'e' { not_a_vec = true; break; } }
            if i == 2 { if c != 'c' { not_a_vec = true; break; } }

            //parse type inside vec
            if c == '<' {
                parsing_inside_type = true;
            } else if c == '>' { }
            else if parsing_inside_type {
                type_inside_vec_as_str.push(c);
            }

            i += 1;

            if i == type_.len() {
                break;
            }
        };

        if not_a_vec { return None } else {
            let type_inside_vec = Type::new(type_inside_vec_as_str);
            return Some(Type::VecType( VecType {
                inner_type: type_inside_vec.into()
            }))
        }
    }

    pub fn get_vec_type_struct(type_: Type) -> VecType {
        match type_ {
            Type::VecType(vec_type) => vec_type,
            _ => panic!("Type is not a VecType")
        }
    }
}