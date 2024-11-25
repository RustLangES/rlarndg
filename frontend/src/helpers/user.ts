import { MResponse } from "./router";

export interface User {
	id: number;
	email: string;
}

export interface OptionalUserProps {
	user?: User;
}

export async function optionalUserMiddleware(): Promise<MResponse> {
	const user = await fetch("/api/auth/user", {
		credentials: "include"
	});

	return MResponse.next(user.status > 399 ? undefined : await user.json());
}
