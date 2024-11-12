import { ReactElement, useState } from "react";

import { MResponse } from "../../helpers/router";

import TopBar from "../../components/top-bar/component";
import Button from "../../components/button/component";

import "./page.css";

export async function loginMiddleware(): Promise<MResponse> {
	const user = await fetch("/api/auth/user", {
		credentials: "include"
	});

	return user.status > 399
		? MResponse.next()
		: MResponse.redirect("/user");
}

export default function Login(): ReactElement {
	const [error, setError] = useState("");

	async function login() {
		const email = document.getElementById("email") as HTMLInputElement;
		const password = document.getElementById("password") as HTMLInputElement;

		if (email.value.length == 0 || password.value.length == 0) {
			setError("Please fill both fields.");
			return;
		}

		const result = await fetch("/api/auth/login", {
			method: "POST",
			body: JSON.stringify({
				email: email.value,
				password: password.value
			}),
			credentials: "include",
			headers: {
				"Content-Type": "application/json"
			}
		});

		if (result.status > 399) {
			setError(await result.text());
			return;
		}

		location.assign("/user");
	}

	return <>
		<TopBar
			links={[
				{ text: "Home", href: "/" },
				{ text: "Documentation", href: "/docs" },
				{ text: "Pricing", href: "/pricing" }
			]}
			login={"disabled"}
		/>
		<div className="login-container">
			<h1>Login to your account</h1>
			<div>
				<label htmlFor="email">Email</label>
				<input id="email" placeholder="you@email.tld" />
				<label htmlFor="password">Password</label>
				<input id="password" placeholder="password" type="password" />
			</div>
			<span className="login-error">{error}</span>
			<Button type="primary" onClick={login}>Login</Button>
		</div>
	</>;
}
