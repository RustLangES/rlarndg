import { ReactElement } from "react";

import { OptionalUserProps } from "../../helpers/user";

import TopBar from "../../components/top-bar/component";
import Button from "../../components/button/component";

import "./page.css";

export default function TransactionSuccess({user: user}: OptionalUserProps): ReactElement {
	let name: string | undefined = undefined;

	if (user != undefined)
		name = user.email.split("@")[0];

	function onTriggerLogin() {
		location.assign(name == undefined ? "/login" : "/user");
	}

	return <>
		<TopBar
			links={[
				{text: "Documentation", href: "/docs"},
				{text: "Pricing", href: "/pricing"}
			]}
			login={name == undefined ? "enabled" : ["panel", name]}
		/>
		<div className="ts-thanks-wrapper">
			<h1>Thanks for your contribution</h1>
			<p>
				You helped RustLangEs make a better place.
				You can access your API key under your account.
			</p>
			<Button type="primary" onClick={onTriggerLogin}>
				{name == undefined ? "Login" : "Go there"}
			</Button>
		</div>
	</>;
}
