import { ReactElement } from "react";

export class MResponse {
	private constructor(
		private url?: string,
		private value?: unknown
	) {}

	public static next<T>(value?: T): MResponse {
		return new MResponse(undefined, value);
	}

	public static redirect(redirect: string): MResponse {
		return new MResponse(redirect, undefined);
	}

	public apply(): [boolean, unknown] {
		if (this.url != undefined) {
			location.assign(this.url);
			return [false, undefined];
		}

		return [true, this.value];
	}
}

export interface ComponentRoute {
	route: RegExp;
	component(values: unknown): ReactElement;
	middleware?: (() => Promise<MResponse> | MResponse)[];
}

export interface RedirectRoute {
	route: RegExp;
	redirect: string;
}

export type Route = ComponentRoute | RedirectRoute;

