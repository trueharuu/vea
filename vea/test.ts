type A<T extends Array<string>, S extends string, P extends string = ""> = T extends [infer Q extends string]
  ? `${P}${Q}`
  : T extends [infer Q extends string, ...infer R extends Array<string>]
    ? A<R, S, `${P}${Q}${S}`>
    : P;

type c=A<["a", "b", "c"], ",">