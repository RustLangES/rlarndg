import { ReactElement, useState } from "react";
import { CopyBlock, hybrid } from "react-code-blocks";

import TopBar from "../../components/top-bar/component";
import Endpoint from "../../components/endpoint/component";

import { OptionalUserProps } from "../../helpers/user";

import "./page.css";

export default function Documentation({user}: OptionalUserProps): ReactElement {
	const [apiKey, setApiKey] = useState("");

	let name = undefined;

	if (user != undefined)
		name = user.email.split("@")[0];

	return <>
		<TopBar
			links={[
				{text: "Home", href: "/"},
				{text: "Pricing", href: "/pricing"}
			]}
			login={name == undefined ? "enabled" : ["panel", name]}
		/>
		<div className="documentation-main">
			<h1>Documentation</h1>
			<p>
				This whole project is HTTP based, you can obtain
				the random numbers trough HTTP requests documented
				as follows.
			</p>
			<div className="separator" />
			<h2>Authorization</h2>
			<p>
				You can opt to pass or not an Authorization header,
				if you don't you will have a 30 seconds ratelimit for
				each request, otherwise you can buy an API key by
				logging in. All of the money goes to fund the RustLangEs
				project which intends to translate rust resources for spanish programmers.
				The requests you make with an API key will not be ratelimited.
			</p>
			<div className="separator" />
			<h2>Responses</h2>
			<p>
				When the response returns a <span className="md-highlight">200 OK</span>
				response code, it will always return an object containing the author email,
				a second based unix timestamp and the value, which may vary depending on the
				endpoint.
			</p>
			<p>
				Usually in typescript you should be able to represent the
				response in the following way
				<br />
				<br />
				<CopyBlock
					language="typescript"
					text="
	interface MarndgResponse<TValue> {
		author: string;
		timestamp: number;
		value: TValue;
	}
					"
					theme={hybrid}
				/>
				<br />
				For the error response which is <span className="md-highlight">399 &lt;</span> a
				single string is returned with the error, so you can always parse it
				as a <span className="md-highlight">string</span>.
			</p>
			<div className="separator" />
			<h2>Endpoints</h2>
			<p>
				You can test all of the available endpoints within the rest of the page,
				the endpoint guides component will help you get started.
			</p>
			<div className="separator" />
			<h3>Get random color</h3>
			<p>
				This endpoint generates a random color, based on an integer it
				performs a conversion to a hexadecimal valid color range,
				or unless requested it returns an object with the <span className="md-highlight">red</span>
				, <span className="md-highlight">green</span> and <span className="md-highlight">blue</span> fields
				respectively. the<span className="md-highlight">format</span> query parameter
				is allowed with hex or rgb values. Except <span className="md-highlight">hex</span> itself
				returns a number fitting a 24 bits integer or more, not a css
				formatted color.
			</p>
			<Endpoint
				method="GET"
				url="/api/random/color"
				responseType="json-or-text"
				query={[
					{
						name: "format",
						validity(current) {
							return current == "hex" || current == "rgb"
								? null
								: "The value must be or either `hex` or `rgb`";
						},
						autocomplete: () => "rgb",
					}
				]}
				headers={[
					{
						name: "Authorization",
						autocomplete: () => apiKey
					}
				]}
			/>
			<div className="separator" />
			<h3>Get random boolean</h3>
			<p>
				As the name mentions all this endpoint does is return a boolean, which is an
				either <span className="md-highlight">true</span> or
				<span className="md-highlight">false</span> value.
			</p>
			<Endpoint
				method="GET"
				url="/api/random/boolean"
				responseType="json-or-text"
				headers={[
					{
						name: "Authorization",
						autocomplete: () => apiKey
					}
				]}
			/>
			<div className="separator" />
			<h3>Get random signed integer</h3>
			<p>
				This endpoint gets a random <span className="md-highlight">32-bit</span> signed
				integer, which ranges from <span className="md-highlight">-2,147,483,648</span>
				to <span className="md-highlight">2,147,483,647</span> (inclusive).
			</p>
			<Endpoint
				method="GET"
				url="/api/random/signed"
				responseType="json-or-text"
				headers={[
					{
						name: "Authorization",
						autocomplete: () => apiKey
					}
				]}
			/>
			<div className="separator" />
			<h3>Get random unsigned integer</h3>
			<p>
				This endpoint gets a random <span className="md-highlight">32-bit</span> unsigned
				integer, which ranges from <span className="md-highlight">0</span>
				to <span className="md-highlight">4,294,967,295</span> (inclusive),
				which is basically the double of the signed integer.
			</p>
			<Endpoint
				method="GET"
				url="/api/random/unsigned"
				responseType="json-or-text"
				headers={[
					{
						name: "Authorization",
						autocomplete: () => apiKey
					}
				]}
			/>
			<footer>
				<p>No Copyright RustLangEs, Licensed as CC0.</p>
			</footer>
		</div>
	</>;
}
