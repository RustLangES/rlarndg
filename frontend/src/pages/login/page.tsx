import { KeyboardEvent, ReactElement, useState } from "react";

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

	async function onKeyDown(event: KeyboardEvent) {
		if (event.key === "Enter") {
			event.preventDefault();
			await login();
		}
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
				<input id="email" placeholder="you@email.tld" onKeyDown={onKeyDown} />
				<label htmlFor="password">Password</label>
				<input id="password" placeholder="password" type="password" onKeyDown={onKeyDown} />
			</div>
			<div className="login-info">
				<span className="login-error">{error}</span>
				<p>Don't have an account? <a href="/register">register</a></p>
			</div>
			<Button type="primary" onClick={login}>Login</Button>
		</div>
	</>;
}
