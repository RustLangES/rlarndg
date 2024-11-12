import { ReactElement } from "react";

import { User } from "../../helpers/user";
import { MResponse } from "../../helpers/router";

import "./page.css";

interface UserPanelProps {
	user: User;
}

export async function userPannelMiddleware(): Promise<MResponse> {
	const user = await fetch("/api/auth/user", {
		credentials: "include"
	});

	return user.status > 399
		? MResponse.redirect("/login")
		: MResponse.next(await user.json());
}

export default function UserPanel({user}: UserPanelProps): ReactElement {
	return <></>;
}
