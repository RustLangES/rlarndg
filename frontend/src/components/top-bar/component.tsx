import { ReactElement } from "react";
import { ExternalLink, User } from "lucide-react";

import Button from "../button/component";

import "./component.css";

interface TopBarProps {
	links: {
		text: string;
		href: string;
	}[];
}

export default function TopBar({links}: TopBarProps): ReactElement {
	return <div className="top-bar">
		<div className="top-bar-related">
			<div className="top-bar-logo">
				<img src="/logo.png" alt="logo" />
				<span>RlARndG</span>
			</div>
			{links.map(({text, href}, i) =>
				<a href={href} key={i}>
					{text}
				</a>
			)}
			<a href="https://github.com/RustLangES/rlarndg">
				<span>Source</span>
			</a>
		</div>
		<div className="top-bar-external">
			<a href="https://rustlang-es.org/">
				<span>RustLangEs</span>
				<ExternalLink />
			</a>
			<a href="https://discord.gg/4ng5HgmaMg">
				<span>Discord</span>
				<ExternalLink />
			</a>
			<Button
				type="primary"
				icon={<User />}
			>
				Login
			</Button>
		</div>
	</div>;
}
