import { ReactElement } from "react";

import { OptionalUserProps } from "../../helpers/user";

import TopBar from "../../components/top-bar/component";

import "./page.css";

export default function Pricing({user}: OptionalUserProps): ReactElement {
	let name = undefined;

	if (user != undefined)
		name = user.email.split("@")[0];

	return <>
		<TopBar
			links={[
				{ text: "Home", href: "/" },
				{ text: "Documentation", href: "/docs" }
			]}
			login={name == undefined ? "enabled" : ["panel", name]}
		/>
	</>;
}
