# castle
// fn my_resolver(want: WantType) -> () {
//     if let WantType::Match(Match { patterns}) => {
//         let returned_object = IconTypes::Svg {
//             url: "https://example.com/icon.svg".into(),
//             size: 48,
//         };

//         match returned_object {
//             IconTypes::Emoji { unicode } => {
//                 if let Some(want) = patterns.get("SVG") {
//                     match want {
//                         WantType::ObjectProjection(wants) => {
//                             let mut returned_object = Value::object();
//                             returned_object.insert("url".into(), ("https://example.com/icon.svg".into(), PrimitiveValue::String));
//                             returned_object.insert("size".into(), (48, PrimitiveValue::UInt));
//                             return returned_object;
//                         },
//                         _ => panic!("Expected an object projection"),
//                     }
//                 }
//                 return Value::None
//             },
//         }
//     }
// }


Intended to be used inside of a rust web api server.

1. Define a schema string, including the resolvers
2.



```rs

const CASTLE_SCHEMA = r#"
    type User {
        first_name: String
    }

    directive authenticated()

    fn me() -> User @authenticated
"#;


enum Want {
    SingleField(Box<str>),
    Projection(HashSet<Want>)
}

async fn me(wants: HashSet<Want>, context: Context) -> Result<Serde::Value, Error> {
    let query: String = query!{
        let user = root.users.{context.user};

        return user::{
            ${...wants}
        }
    },

    client.query(query).await?
}

const castle_api = lazy_static!{
    let builder = CastleBuilder::new(schema)
    builder.add_fn("me", me)
    builder.build()
}

fn handle_request(req: Request, res: Response) {
    let context = Context {
        user: req.cookies.user_id,
    };

    let json = castle_api.query("
        me {
            first_name
        }
    ", context).await?;

    res.status(200);
    res.send(json);
}

```

```schema
type User {
    first_name: String
    last_name: String
    email: String
    profile_picture(size_px: UInt) -> Result<String, Error>
}

fn me() -> User

fn signup(
    email: String
    password: String
    first_name: String
    last_name: String
) -> ()
```


```query
me {
    first_name
    last_name
    email
    profile_picture(48)
    icon match {
        SVGIcon {

        }
        Emoji {

        }
    }
}

let json = {
    errors: null,
    data: {
        me: {
            first_name: "Albert",
            last_name: "Marashi",
            email: "albert@framework.tools",
            profile_picture: "https://url"
        }
    }
}
```

keywords = [
    as
    true,
    false,
    None
    Some
    match
]

```ts
interface Resolvers {
    async function me(): User | null;
}

type UserBasicInfo = {
    first_name: String
    last_name: String,
    profile_picture: Result<String, Error>
};

let result: {
    me: Result<UserBasicInfo, Error>
} = query`
    me {
        first_name
        last_name
        email
        profile_picture(48)
    }
`;


class Result<T, E> {
    t: T | null
    e: E | null

    private constructor(t: T, e: E) {
        this.t = t
        this.e = e
    }

    static Ok(t: T) {
        return new Result(t, null)
    }

    static Err(e: E) {
        return new Result(null, T)
    }

    bubble(): T {
        if !this.t {
            throw this.e
        }
        return this.t
    }
}

function Ok<T, E>(t: T): Result<T, E> {
    return Result.Ok(t)
}

function some_fn() -> Result<void, Error> {
    let result: Result<String, Error> = Ok("hello")

    if result.is_err() return result as Result<void, Error>

    switch result.is_ok() {

    }

    return val;
}

try {
    some_fn()
} catch (e) {

}

```


