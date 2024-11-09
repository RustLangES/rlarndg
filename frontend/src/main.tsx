import { ReactElement, StrictMode, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

import Landing from "./pages/landing/page";
import Documentation from "./pages/documentation/page";

import "./index.css";

interface ComponentRoute {
	route: RegExp;
	component: ReactElement;
}

interface RedirectRoute {
	route: RegExp;
	redirect: string;
}

type Route = ComponentRoute | RedirectRoute;

const routes: Route[] = [
	{
		route: /^\/$/,
		component: <Landing />
	},
	{
		route: /^\/docs$/,
		component: <Documentation />
	}
];

function App(): ReactElement {
	const [route, setRoute] = useState<ComponentRoute | null>(null);

	useEffect(() => {
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

		setRoute(route);
	});

	return <>{route?.component || ""}</>;
}

createRoot(document.getElementById("root")!)
	.render(<StrictMode><App /></StrictMode>)
