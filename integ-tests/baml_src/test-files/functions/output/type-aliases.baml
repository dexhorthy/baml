type Primitive = int | string | bool | float

type List = string[]

type Graph = map<string, string[]>

type Combination = Primitive | List | Graph

function PrimitiveAlias(p: Primitive) -> Primitive {
    client "openai/gpt-4o"
    prompt r#"
      Return the given value back: {{ p }}
    "#
}

function MapAlias(m: Graph) -> Graph {
    client "openai/gpt-4o"
    prompt r#"
      Return the given Graph back:

      {{ m }}

      {{ ctx.output_format }}
    "#
}

function NestedAlias(c: Combination) -> Combination {
    client "openai/gpt-4o"
    prompt r#"
      Return the given value back:

      {{ c }}

      {{ ctx.output_format }}
    "#
}
