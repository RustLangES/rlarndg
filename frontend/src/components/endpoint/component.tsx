import { ReactElement, useState, FormEventHandler, Dispatch, SetStateAction, useEffect, Fragment } from "react";

import Button from "../button/component";

import "./component.css";

interface Parameter {
	name: string;
	required?: boolean;
	autocomplete?: () => string;
	validity?(current?: string): string | null;
	readonly?: boolean;
}

interface ValidatedParameter extends Omit<Parameter, "autocomplete" | "validity"> {
	content: string;
	error: string | null;
}

interface EndpointProps {
	method: "GET" | "POST" | "PUT" | "DELETE" | "PATCH";
	url: string;
	query?: Parameter[];
	headers?: Parameter[];
	responseType: "json" | "json-or-text" | "text";
	className?: string;
}

export default function Endpoint(props: EndpointProps): ReactElement {
	const [fieldsVisible, setFieldsVisible] = useState(false);
	const [query, setQuery] = useState<{[key: string]: string}>({});
	const [headers, setHeaders] = useState<{[key: string]: string}>({});
	const [result, setResult] = useState<string | undefined>();
	const [status, setStatus] = useState<[number, string] | undefined>();

	useEffect(() => {
		function autocomplete(
			parameters: Parameter[],
			setter: Dispatch<SetStateAction<{[key: string]: string}>>
		) {
			setter((current) => {
				const newValues = {...current};
				
				parameters.forEach(({name, autocomplete}) => {
					if (autocomplete) {
						newValues[name] = autocomplete();
					}
				});

				return newValues;
			});
		}

		if (props.query)
			autocomplete(props.query, setQuery);

		if (props.headers)
			autocomplete(props.headers, setHeaders);
	}, []);

	function clickVisibility(): void {
		setFieldsVisible((current) => !current);
	}

	function updateInputFactory
	(name: string, setter: Dispatch<SetStateAction<{[key: string]: string}>>):
	FormEventHandler<HTMLInputElement> {
		return (event) => {
			setter(current => {
				const values = {...current};
				values[name] = (event.target as HTMLInputElement).value;
				return values;
			});
		}
	}

	const queryKeys = Object.keys(query)
		.filter(key => query[key]);

	const url = (
			props.url.startsWith("http")
				? props.url
				: location.origin + props.url
		)
		+ (queryKeys.length > 0 ? "?" : "")
		+ queryKeys
			.map(key => `${encodeURIComponent(key)}=${encodeURIComponent(query[key])}`)
			.join("&");

	async function request(): Promise<void> {
		const result = await fetch(url, {
			headers,
			method: props.method,
			redirect: "follow",
			cache: "no-cache",
			credentials: "omit",
			body: null
		});

		setStatus([result.status, result.statusText]);

		setResult(
			(
				props.responseType == "json"
				|| props.responseType == "json-or-text"
				&& result.status < 399
			)
				? JSON.stringify(await result.json(), null, 4)
				: await result.text()
		);
	}

	function validityMapFactory
	(validityObject: {[key: string]: string})
	: (parameter: Parameter) => ValidatedParameter {
		return ({validity, name, ...rest}) => ({
			content: validityObject[name],
			error: validity != undefined ?
				validity(validityObject[name])
				: null,
			name,
			...rest
		})
	}

	const validatedQuery = props.query
		?.map(validityMapFactory(query));

	const validatedHeaders = props.headers
		?.map(validityMapFactory(headers));
	
	return <div className={`endpoint ${props.className ?? ""}`}>
		<div className="endpoint-main">
			<label data-method={props.method}>{props.method}</label>
			<input readOnly value={url}/>
		</div>
		<Button
			onClick={clickVisibility}
			type={!fieldsVisible ? "primary" : "secondary"}
		>
			{!fieldsVisible ? "Show more" : "Show less"}
		</Button>
		{fieldsVisible && <>
			<div className="endpoint-param">
				{props.query && <span><b>Query parameters</b></span>}
				<div className="endpoint-query">
					{validatedQuery && validatedQuery
						.map((queryValue, key) =>
							<div className="endpoint-param" key={key}>
								<label>{queryValue.name}</label>
								<input
									readOnly={queryValue.readonly}
									onInput={updateInputFactory(queryValue.name, setQuery)}
									defaultValue={queryValue.content}
								/>
								{<span>{queryValue.error ?? ""}</span>}
							</div>
						)
					}
				</div>
				{props.headers && <span><b>Request headers</b></span>}
				<div className="endpoint-headers">
					{validatedHeaders && validatedHeaders
						.map((headerValue, key) =>
							<div className="endpoint-param" key={key}>
								<label>{headerValue.name}</label>
								<input
									readOnly={headerValue.readonly}
									onInput={updateInputFactory(headerValue.name, setHeaders)}
									defaultValue={headerValue.content}
								/>
								{<span>{headerValue.error ?? ""}</span>}
							</div>
						)
					}
				</div>
			</div>
			<Button
				type="primary"
				onClick={request}
				disabled={
					validatedQuery?.some(query => query.error != null)
					|| validatedHeaders?.some(header => header.error != null)
					? true
					: false // type safety.
				}
			>
				Request
			</Button>
			{status && <span className={
				status[0] > 399
					? "status-red"
					: status[0] > 299
						? "status-yellow"
						: "status-green"
			}>
				{status[0]} {status[1]}
			</span>}
			{result &&
				<span
					role="textbox"
					className="endpoint-result"
				>
					{result
						.split("\n")
						.map((line, index) => (
							<Fragment key={index}>
								{line.replace(/[ \t]/g, "\u00A0")}
								<br />
							</Fragment>
						))
					}
				</span>
			}
		</>}
	</div>;
}
