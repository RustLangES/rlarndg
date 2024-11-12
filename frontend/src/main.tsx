import { ReactElement, StrictMode, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

import { Route } from "./helpers/router";
import { optionalUserMiddleware, User } from "./helpers/user";

import Landing from "./pages/landing/page";
import Documentation from "./pages/documentation/page";
import Login, { loginMiddleware } from "./pages/login/page";
import UserPanel, { userPannelMiddleware } from "./pages/user/page";

import "./index.css";

const routes: Route[] = [
	{
		route: /^\/$/,
		component: (user?: User) => <Landing user={user} />,
		middleware: [optionalUserMiddleware]
	},
	{
		route: /^\/docs$/,
		component: (user?: User) => <Documentation user={user} />,
		middleware: [optionalUserMiddleware]
	},
	{
		route: /^\/login$/,
		component: () => <Login />,
		middleware: [loginMiddleware]
	},
	{
		route: /^\/user$/,
		component: (user: User) => <UserPanel user={user} />,
		middleware: [userPannelMiddleware]
	}
];

function App(): ReactElement {
	const [route, setRoute] = useState<ReactElement | null>(null);

	useEffect(() => {
		async function ew(): Promise<void> {
			const route = routes
				.find(({route}) => route.test(location.pathname));

			if (route == undefined) {
				location.assign("/");
				return;
			}

			if ('redirect' in route) {
				location.assign(route.redirect);
				return;
			}

			let middlewareValue;

			for (const middleware of route.middleware ?? []) {
				const [next, value] = (await middleware()).apply();

				middlewareValue = value;

				if (!next) {
					return;
				}
			}

			setRoute(route.component(middlewareValue));
		}

		ew()
			.then();
	}, []);

	return <>{route || ""}</>;
}

createRoot(document.getElementById("root")!)
	.render(<StrictMode><App /></StrictMode>)
