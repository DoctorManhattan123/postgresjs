use neon::prelude::*;

fn factorial(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n = cx.argument::<JsNumber>(0)?.value(&mut cx) as u64;
    let result = (1..=n).product::<u64>();
    Ok(cx.number(result as f64))
}

struct PostgresConnection {
    pub url: String,
    pub connections: f64,
}

pub fn connect(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let url = cx.argument::<JsString>(0)?.value(&mut cx);
    let connections = cx.argument::<JsNumber>(1)?.value(&mut cx) as f64;

    println!("Connect with {} with {} connections", url, connections);

    Ok(cx.number(0 as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let postgres_connection = PostgresConnection {
        url: "123".to_string(),
        connections: 12.0,
    };
    let object = postgres_connection.to_object(&mut cx)?;

    cx.export_value("pgConnection", object)?;
    cx.export_function("fac", factorial)?;
    cx.export_function("connect", connect)?;
    Ok(())
}

impl PostgresConnection {
    pub fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();
        let url = cx.string(&self.url);
        obj.set(cx, "url", url)?;
        let connections = cx.number(self.connections);
        obj.set(cx, "connections", connections)?;
        Ok(obj)
    }
}
