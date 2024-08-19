/* eslint-disable @typescript-eslint/no-explicit-any */
type UnionToIntersection<U> = (U extends any ? (k: U) => void : never) extends (k: infer I) => void
	? I
	: never;

export type UnionToTuple<T> = UnionToIntersection<
	T extends any ? () => T : never
> extends () => infer R
	? [...UnionToTuple<Exclude<T, R>>, R]
	: [];
