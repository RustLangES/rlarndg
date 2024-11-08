import { ReactElement, StrictMode, useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';

import Landing from './components/landing/page';

import './index.css';

interface ComponentRoute {
	route: RegExp;
	component: ReactElement;
}

interface RedirectRoute {
	route: RegExp;
	redirect: string;
}

type Route = ComponentRoute | RedirectRoute

const routes: Route[] = [
	{
		route: /^\/$/,
		component: <Landing />
	}
];

function App(): ReactElement {
	const [route, setRoute] = useState<ComponentRoute | null>(null);

	useEffect(() => {
		const route = routes
			.find(({route, ..._}) => route.test(location.pathname));

		if (route == undefined) {
			location.assign('/');
			return;
		}

		if ('redirect' in route) {
			location.assign(route.redirect);
			return;
		}

		setRoute(route);
	});

	return <>{route || ''}</>;
}

createRoot(document.getElementById('root')!)
	.render(<StrictMode><App /></StrictMode>)
