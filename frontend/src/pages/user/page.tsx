import { ReactElement, useEffect, useState } from "react";

import { User } from "../../helpers/user";
import { MResponse } from "../../helpers/router";

import TopBar from "../../components/top-bar/component";
import ApiKey from "../../components/key/component";

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
	const [keys, setKeys] = useState<number[] | undefined>();

	useEffect(() => {
		fetch("/api/keys/user", {
			credentials: "include"
		})
			.then(r => r.json())
			.then(setKeys)
	}, []);

	let name = user.email.split("@")[0];

	return <>
		<TopBar
			links={[
				{ text: "Home", href: "/" },
				{ text: "Documentation", href: "/docs" },
				{ text: "Pricing", href: "/pricing" }
			]}
			login={"logout"}
		/>
		<div className="user-wrapper">
			<div>
				<h1>Welcome {name}!</h1>
				<p>
					From This page you can manage your api keys,
					the API key ID is always the same, as it's
					based on the database id for the key.
				</p>
			</div>
			{
				keys != undefined
					? (
						keys.length > 0
							? keys
								.sort((a, b) => a - b)
								.map((id, idx) => <ApiKey id={id} key={idx} number={idx} />)
							: <div>
								<h2>No API keys found</h2>
								<p>
									It looks like you don't have any API key,
									there is not much you can do here... Go
									to <a href="/pricing">pricing</a> to obtain
									an API key.
								</p>
							</div>
					)
					: <h2>Loading...</h2>
			}
		</div>
	</>;
}
