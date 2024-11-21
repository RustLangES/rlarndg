import { ReactElement, useState } from "react";

import { OptionalUserProps } from "../../helpers/user";

import TopBar from "../../components/top-bar/component";
import Button from "../../components/button/component";

import "./page.css";

export default function Pricing({user}: OptionalUserProps): ReactElement {
	const [error, setError] = useState(false);

	function onTriggerDonate() {
		const input = document.getElementById("money-input") as HTMLInputElement;

		if (!/^\d+$/.test(input.value))
			return;

		try {
			location.assign(`/api/keys/checkout?amount=${input.valueAsNumber}`);
		} catch {
			setError(true);
		}
	}

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
		<div className="pricing-wrapper">
			<div>
				<h1>Pricing</h1>
				<p>
					Since this project is for funding RustLangEs it
					doesn't really have a pricing defined for an API key
					the minimum donation is 3 dollars, but you can donate
					whatever you want, you can donate 6 dollars or
					3 dollars twice for two api key slots.
				</p>
			</div>
			<div>
				<div>
					<input
						className={`input${error ? " input-error" : ""}`}
						placeholder="3.00$"
						type="number"
						id="money-input"
					/>
					<Button
						type={user == undefined ? "secondary" : "primary"}
						disabled={user == undefined}
						onClick={onTriggerDonate}
					>
						{user == undefined ? "Login before donating" : "Donate"}
					</Button>
				</div>
				<div>
					<span className="error" hidden={!error}>The minimum amount is 3.00$.</span>
				</div>
			</div>
		</div>
	</>;
}
