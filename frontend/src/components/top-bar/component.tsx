import { ReactElement } from "react";
import { ExternalLink, LogOut, User } from "lucide-react";

import Button from "../button/component";

import "./component.css";

interface TopBarProps {
	links: {
		text: string;
		href: string;
	}[];
	login?: "enabled" | "disabled" | "logout" | ["panel", string];
}

export default function TopBar({links, login = "enabled"}: TopBarProps): ReactElement {
	function logOut(): void {
		document.cookie = "auth=; Path=/; Expires=Thu, 01 Jan 1970 00:00:01 GMT; path=/;";

		location.assign("/");
	}

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
			{
				Array.isArray(login)
					? <Button
						type="primary"
						icon={<User />}
						href="/user"
					>
						{login[1]}
					</Button>
					: login == "logout"
						? <Button
							type="error"
							icon={<LogOut />}
							onClick={logOut}
						>
							Log Out
						</Button>
						: <Button
							type="primary"
							icon={<User />}
							href="/login"
							disabled={login == "disabled"}
						>
							Login
						</Button>
			}
		</div>
	</div>;
}
