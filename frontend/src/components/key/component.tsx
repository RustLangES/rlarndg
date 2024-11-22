import { ReactElement, useState } from "react";

import Button from "../button/component";

import "./component.css";

interface ApiKeyProps {
	id: number;
	number: number;
}

export default function ApiKey({id, number}: ApiKeyProps): ReactElement {
	const [shownKey, setShownKey] = useState<string | undefined>();
	const [sure, setSure] = useState(false);

	function resetKey() {
		fetch(`/api/keys/reset?id=${id}`, {method: "POST", credentials: "include"})
			.then(r => r.text())
			.then(setShownKey);

		setSure(false);
	}

	return <div className="api-key">
		<h3>Key {number}</h3>
		<input
			readOnly
			value={shownKey ?? "****************************************************"}
		/>
		<div>
			<p hidden={!sure}>Are you sure? This could be a destructive action.</p>
			{
				!sure
					? <Button type="secondary" onClick={() => setSure(true)}>Reset</Button>
					: <>
						<Button type="secondary" onClick={() => setSure(false)}>Cancel</Button>
						<Button type="error" onClick={resetKey}>Reset</Button>
					</>
			}
		</div>
	</div>;
}
